use std::io::Write;

use crossbeam::select;
use crossterm::event::KeyCode::{Down, PageDown, PageUp, Up};
use crossterm::event::{KeyEvent, MouseEventKind};

use {crate::*, anyhow::Result, crokey::CroKey, crossterm::event::Event};

use crate::cli::action::Action;
use crate::cli::internal::Internal;
use crate::cli::keybindings::KeyBindings;
use crate::cli::W;
use crate::executor::command_output::CommandExecInfo;
use crate::executor::execution_plan::ExecutionPlan;
use crate::executor::job_location::JobLocation;
use crate::executor::Executor;
use crate::view::View;

/// Run the mission and return the reference to the next
/// job to run, if any
pub fn run(
    w: &mut W,
    view: &mut View,
    location: JobLocation,
    execution_plan: ExecutionPlan,
    event_source: &EventSource,
    keep_files: bool,
) -> Result<Option<Action>> {
    let keybindings = KeyBindings::default();

    let executor = Executor::new(location, execution_plan, keep_files)?;

    view.execution_starts();
    let user_events = event_source.receiver();

    let mut action: Option<Action> = None;

    loop {
        select! {
            recv(user_events) -> user_event => {
                match user_event?.event {
                    #[allow(unused_mut)] //due to windows config should be mutable
                    Event::Resize(mut width, mut height) => {
                        // I don't know why but Crossterm seems to always report an
                        // underestimated size on Windows
                        #[cfg(windows)]
                        {
                            width += 1;
                            height += 1;
                        }
                        view.resize(width as usize, height as usize);
                    }
                    Event::Key(key_event @ KeyEvent { code, .. }) => {
                        debug!("key pressed: {}", CroKey::from(key_event));

                        match code {
                            Up => view.try_scroll_lines(w, -1)?,
                            Down => view.try_scroll_lines(w, 1)?,
                            PageUp => view.try_scroll_pages(w, -1)?,
                            PageDown => view.try_scroll_pages(w, 1)?,
                            _ => {
                                action = keybindings.get(key_event);
                                debug!("Action requested {:?}", action);
                            }
                        }
                    }
                    Event::Mouse(mouse_event) => {
                        debug!("mouse event: {:?}", mouse_event.kind);

                        match mouse_event.kind {
                            MouseEventKind::ScrollDown => view.try_scroll_lines(w, 1)?,
                            MouseEventKind::ScrollUp => view.try_scroll_lines(w, -1)?,
                            _ => {}
                        }
                    }
                }
                event_source.unblock(false);
            }
            // recv(watch_receiver) -> _ => {
            //     debug!("got a watcher event");
            //     // if let Err(e) = executor.start(state.new_task()) {
            //     //     debug!("error sending task: {}", e);
            //     // } else {
            //     //     state.computation_starts();
            //     // }
            // }
            recv(executor.line_receiver) -> info => {
                match info? {
                    CommandExecInfo::Line(line) => {
                        view.draw_help_line(w)?;
                        match view.write_command_output(w, line.content) {
                            Ok(_) => debug!("Output written"),
                            Err(e) => error!("Error on output: {}", e),
                        };
                    }
                    CommandExecInfo::Output(line) => {
                        view.draw_help_line(w)?;
                        match view.write_on(w, line) {
                            Ok(_) => debug!("Output written"),
                            Err(e) => error!("Error on output: {}", e),
                        };
                    }
                    CommandExecInfo::Start => {
                        info!("execution started");
                        view.draw_executing();
                    }
                    CommandExecInfo::End { status } => {
                        info!("execution finished with status: {:?}", status);
                        view.execution_stops();
                    }
                    CommandExecInfo::Error(e) => {
                        warn!("error in computation: {}", e);
                        match view.write_command_output(w, e) {
                            Ok(_) => debug!("Output written"),
                            Err(e) => error!("Error on output: {}", e),
                        };
                        view.execution_stops();
                        break;
                    }
                    CommandExecInfo::Interruption => {
                        debug!("command was interrupted (by us)");
                        break;
                    }
                }
                w.flush()?;
            }
        }
        if let Some(action) = action.take() {
            debug!("requested action: {action:?}");
            match action {
                Action::Internal(internal) => match internal {
                    Internal::Back => {
                        if !view.close_help() {
                            break;
                        }
                    }
                    Internal::Help => {
                        view.toggle_help();
                    }
                    Internal::Quit => {
                        break;
                    }
                },
            }
        }
        view.draw(w, None)?;
        w.flush()?
    }
    executor.die()?;
    Ok(action)
}

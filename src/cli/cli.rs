use std::io::{stdout, BufWriter};

use anyhow::bail;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};

use {
    crate::*,
    crossterm::{
        self, cursor,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
        QueueableCommand,
    },
    std::{fs, io::Write},
};

use crate::cli::action::Action;
use crate::cli::args::Args;
use crate::cli::keybindings::KeyBindings;
use crate::executor::execution_plan::ExecutionPlan;
use crate::executor::job_location::JobLocation;
use crate::view::View;

/// the type used by all GUI writing functions
///
/// Right now we use stderr, which has the advantage of letting
/// us output something if we want (for a calling process) but
/// as I'm not sure I'll even have something to output, I may
/// switch to stdout which would allow buffering.
//pub type W = std::io::Stderr;
pub type W = BufWriter<std::io::Stdout>;

/// return the writer used by the application
pub fn writer() -> W {
    //std::io::stderr()
    BufWriter::new(stdout())
}

pub fn run(args: &Args) -> anyhow::Result<Option<Action>> {
    let location = JobLocation::new(args)?;
    info!("mission location: {:#?}", &location);

    if location.path_to_md.is_none() {
        bail!("markdown file was not found");
    }
    let keybindings = KeyBindings::default();
    let mut view = View::new(&keybindings);

    let md_path = location.path_to_md.as_ref().unwrap();
    let file_content = fs::read_to_string(md_path)?;
    let text = minimad::clean::lines(&file_content);
    let execution_plan = ExecutionPlan::from_md_lines(text.into_iter());

    let mut w = writer();

    w.queue(EnterAlternateScreen)?;
    w.queue(cursor::Hide)?;
    w.queue(EnableMouseCapture)?;

    let event_source = EventSource::with_options(EventSourceOptions {
        discard_mouse_drag: true,
        discard_mouse_move: true,
    })?;
    view.draw_help_line(&mut w)?;

    let result = app::run(&mut w, &mut view, location, execution_plan, &event_source);

    w.flush()?;
    w.queue(cursor::Show)?;
    w.queue(DisableMouseCapture)?;

    w.queue(LeaveAlternateScreen)?;
    w.flush()?;
    result
}

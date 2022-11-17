use crate::cli::internal::Internal;
use crate::cli::keybindings::KeyBindings;
use crate::view::View;

pub struct HelpLine {
    quit: String,
    help: Option<String>,
    close_help: Option<String>,
}

impl HelpLine {
    pub fn new(keybindings: &KeyBindings) -> Self {
        let quit = keybindings
            .shortest_internal_key(Internal::Quit)
            .map(|k| format!("Hit *{k}* to quit"))
            .expect("the app to be quittable");
        let help = keybindings
            .shortest_internal_key(Internal::Help)
            .map(|k| format!("*{k}* for help"));
        let close_help = keybindings
            .shortest_internal_key(Internal::Back)
            .or_else(|| keybindings.shortest_internal_key(Internal::Help))
            .map(|k| format!("*{k}* to close this help"));
        Self {
            quit,
            help,
            close_help,
        }
    }
    pub fn markdown(&self, state: &View) -> String {
        let mut parts: Vec<&str> = vec![&self.quit];
        if state.is_help() {
            if let Some(s) = &self.close_help {
                parts.push(s);
            }
        } else {
            // if let CommandResult::Report(report) = &state.cmd_result {
            //     if report.suggest_backtrace {
            //         if let Some(s) = &self.toggle_backtrace {
            //             parts.push(s);
            //         }
            //     }
            //     // else if !report.is_success(state.mission.allow_warnings()) {
            //     //     if let Some(s) = &self.toggle_summary {
            //     //         parts.push(s);
            //     //     }
            //     // }
            // }
            // if state.wrap {
            //     if let Some(s) = &self.not_wrap {
            //         parts.push(s);
            //     }
            // } else {
            //     if let Some(s) = &self.wrap {
            //         parts.push(s);
            //     }
            // }
            if let Some(s) = &self.help {
                parts.push(s);
            }
        }
        parts.join(", ")
    }
}

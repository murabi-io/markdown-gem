use std::fmt;

/// one of the hardcoded actions that can be mapped
/// to a key or ran after a successful job
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Internal {
    Back,
    Help,
    Quit,
    ToggleBacktrace,
    ToggleRawOutput,
    ToggleSummary,
    ToggleWrap,
}

impl fmt::Display for Internal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Back => write!(f, "back to previous page or job"),
            Self::Help => write!(f, "help"),
            Self::Quit => write!(f, "quit"),
            Self::ToggleBacktrace => write!(f, "toggle backtrace"),
            Self::ToggleRawOutput => write!(f, "toggle raw output"),
            Self::ToggleSummary => write!(f, "toggle summary"),
            Self::ToggleWrap => write!(f, "toggle wrap"),
        }
    }
}

impl std::str::FromStr for Internal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "back" => Ok(Self::Back),
            "help" => Ok(Self::Help),
            "quit" => Ok(Self::Quit),
            "toggle-raw-output" => Ok(Self::ToggleRawOutput),
            "toggle-backtrace" => Ok(Self::ToggleBacktrace),
            "toggle-summary" => Ok(Self::ToggleSummary),
            "toggle-wrap" => Ok(Self::ToggleWrap),
            _ => Err(()),
        }
    }
}

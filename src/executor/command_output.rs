use std::process::ExitStatus;

use crate::executor::execution_plan::ExecutionItem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandStream {
    StdOut,
    StdErr,
}

/// a line coming either from stdout or from stderr
#[derive(Debug, Clone)]
pub struct CommandOutputLine {
    pub content: String,
    pub origin: CommandStream,
}

/// some output lines
#[derive(Debug, Clone, Default)]
pub struct CommandOutput {
    pub lines: Vec<CommandOutputLine>,
}

/// a piece of information about the execution of a command
pub enum CommandExecInfo {
    /// Command ended
    End { status: Option<ExitStatus> },

    /// Command started
    Start,

    /// Murabi killed the command
    #[allow(dead_code)]
    Interruption,

    /// Execution failed
    Error(String),

    /// Here's a line of output (coming from stderr or stdout)
    Line(CommandOutputLine),

    /// Output an MD line coming from execution plan directly
    Output(ExecutionItem),
}

use {crate::*, std::process::ExitStatus};

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

    /// Murabi killed the command
    Interruption,

    /// Execution failed
    Error(String),

    /// Here's a line of output (coming from stderr or stdout)
    Line(CommandOutputLine),
}

impl CommandOutput {
    pub fn reverse(&mut self) {
        self.lines.reverse()
    }
    pub fn push(&mut self, line: CommandOutputLine) {
        self.lines.push(line);
    }
    pub fn len(&self) -> usize {
        self.lines.len()
    }
}

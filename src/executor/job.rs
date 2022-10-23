use crate::cli::action::Action;
use crate::executor::executable::Executable;
use std::process::Command;

/// One of the possible job that murabi can run
#[derive(Debug, Clone)]
pub struct Job {
    /// The tokens making the command to execute (first one
    /// is the executable).
    /// This vector is guaranteed not empty
    /// by the PackageConfig::from_path loader
    pub command: Vec<String>,

    /// whether we need to capture stdout too (stderr is
    /// always captured)
    pub need_stdout: bool,

    /// the optional action to run when there's no
    /// error, warning or test failures
    pub on_success: Option<Action>,

    /// whether to consider that we have a success when
    /// we only have warnings. This is especially useful
    /// for "cargo run" jobs
    pub allow_warnings: bool,

    pub allow_errors: bool,

    pub executable: Executable,
}

impl Job {
    pub fn from_executable(item: &Executable) -> Option<Self> {
        item.code_chunk.as_ref().map(|c| {
            let attributes = c.attributes.clone();
            let cmd = attributes.cmd.unwrap_or(String::from(""));
            let command = vec![cmd];

            Self {
                command,
                need_stdout: attributes.stdout,
                on_success: None,
                allow_warnings: attributes.allow_warnings,
                allow_errors: attributes.allow_errors,
                executable: item.clone(),
            }
        })
    }

    pub fn get_command(&self) -> Command {
        let mut tokens = self.command.iter();
        let mut command = Command::new(
            tokens.next().unwrap(), // implies a check in the job
        );
        let (args, path) = self
            .executable
            .code_chunk
            .clone()
            .map(|c| (c.attributes.args, c.attributes.path))
            .unwrap_or((None, None));

        if path.is_some() {
            command.current_dir(path.unwrap());
        }
        if args.is_some() {
            for arg in args.unwrap() {
                command.arg(arg);
            }
        }
        command
    }
}

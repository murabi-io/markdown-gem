use crate::executor::command_output::CommandOutput;

/// data of a failed command
#[derive(Debug)]
pub struct Failure {
    pub error_code: i32,
    pub output: CommandOutput,
}

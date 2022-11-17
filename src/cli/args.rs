use anyhow::{bail, Result};
use clap::Parser;

#[derive(Debug, Parser)]
/// murabi watches your source and run code checks in background.
///
/// Documentation at <https://dystroy.org/murabi>
#[clap(author, version, about)]
pub struct Args {
    /// verbose mode
    #[clap(long = "vvv")]
    pub verbose: bool,

    /// path to the log file
    #[clap(short = 'l', long = "log")]
    pub log_file: Option<String>,

    /// path to watch (must be a rust directory or inside)
    #[clap(short = 'p', long = "path")]
    pub path: Option<String>,

    #[clap()]
    /// either a job, or a path, or both
    pub args: Vec<String>,
}

impl Args {
    /// positional arguments in murabi command are a convenience
    /// allowing to skip writing `-p`.
    /// To be used, it must be copied to `path` value.
    pub fn fix(&mut self) -> Result<()> {
        let mut args = self.args.drain(..);
        let path = match (args.next(), self.path.is_none()) {
            (Some(a), true) => Some(a),
            (Some(_), false) => bail!("Too many arguments"),
            _ => None,
        };

        self.path = path;
        Ok(())
    }
}

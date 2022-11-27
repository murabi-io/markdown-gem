use anyhow::{bail, Result};
use clap::Parser;

/// markdown-gem runs your MD files as code chunks.
///
/// Documentation at <https://github.com/murabi-io/murabi>
#[derive(Debug, Parser, Default)]
#[clap(author, version, about)]
pub struct Args {
    /// verbose mode
    #[clap(long = "vvv")]
    pub verbose: bool,

    /// path to the log file
    #[clap(short = 'l', long = "log")]
    pub log_file: Option<String>,

    /// path to MD file or directory with MD
    #[clap(short = 'p', long = "path")]
    pub path: Option<String>,

    /// if specified, gem won't delete the build file of the Code chunks
    #[clap(short = 'k', long = "keep")]
    pub keep_builds: bool,

    #[clap()]
    /// either a path to the folder, file name, or both
    pub args: Vec<String>,
}

impl Args {
    /// positional arguments in gem command are a convenience
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

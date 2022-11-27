use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::Command;
use std::{fs, io};

use uuid::Uuid;

use crate::cli::action::Action;
use crate::executor::executable::Executable;
use crate::executor::job_location::JobLocation;

static MURABI_BUILD_DIR: &str = ".mrbuild";

/// One of the possible jobs that gem can run
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
    /// if there are expected warnings
    pub allow_warnings: bool,

    /// whether to consider that we have a success when
    /// we get errors.
    pub allow_errors: bool,

    /// executable meta data, including code-chinks and
    /// the attributes
    pub executable: Executable,

    /// the location of the job
    pub location: JobLocation,

    /// the build file location after it's created
    file_path: Option<PathBuf>,

    // TODO: remove after sudo implementation
    #[allow(dead_code)]
    /// should the job run in sudo
    sudo: bool,
}

impl Job {
    pub fn new(location: &JobLocation, item: &Executable) -> Option<Self> {
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
                location: location.clone(),
                file_path: None,
                sudo: attributes.with_sudo,
            }
        })
    }

    pub fn get_command(&self) -> Command {
        let mut tokens = self.command.iter();
        // TODO: implement proper sudo handling with pass request
        let mut command = if self.sudo {
            let mut sudo = Command::new("sudo");
            sudo.arg(tokens.next().unwrap());
            sudo
        } else {
            Command::new(tokens.next().unwrap())
        };
        // let mut command = Command::new(tokens.next().unwrap());
        command.current_dir(self.location.workspace_root.clone());
        let (args, path) = self
            .executable
            .code_chunk
            .clone()
            .map(|c| (c.attributes.args, c.attributes.path))
            .unwrap_or((None, None));

        if path.is_some() {
            command.env("PATH", path.unwrap());
        }
        if args.is_some() {
            for arg in args.unwrap() {
                command.arg(arg);
            }
        }
        command
    }

    /// create and write the job file
    pub fn write_file(&mut self) -> io::Result<PathBuf> {
        let name = self
            .executable
            .code_chunk
            .as_ref()
            .map(|c| c.clone().attributes.id)
            .flatten()
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        let build_dir = self.location.clone().workspace_root;
        let mut build_dir = PathBuf::from(format!(
            "{}/{}",
            String::from(build_dir.to_string_lossy()),
            MURABI_BUILD_DIR
        ));
        if !build_dir.exists() {
            fs::create_dir(&build_dir)?;
        }

        build_dir.push(name);
        fs::write(&build_dir, &self.executable.code)?;

        self.file_path = Some(PathBuf::from(&build_dir));
        Ok(build_dir)
    }

    /// remove the job file
    pub fn remove_file(&self) -> io::Result<()> {
        if self.file_path.is_some() {
            let path = self.file_path.as_ref().unwrap();
            fs::remove_file(path)
        } else {
            Ok(())
        }
    }
}

impl Display for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

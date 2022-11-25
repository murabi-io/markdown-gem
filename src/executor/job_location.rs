use anyhow::Result;
use std::{env, fmt, path::PathBuf};

use crate::cli::args::Args;

#[derive(Clone, PartialEq, Eq)]
pub struct JobLocation {
    pub workspace_root: PathBuf,
    pub path_to_md: Option<PathBuf>,
}

impl fmt::Debug for JobLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JobLocation")
            .field("workspace_root", &self.workspace_root)
            .field("path_to_md", &self.path_to_md)
            .finish()
    }
}

impl JobLocation {
    pub fn new(args: &Args) -> Result<Self> {
        let mut workspace_root = args
            .path
            .as_ref()
            .map_or_else(|| env::current_dir().unwrap(), PathBuf::from);
        let path_to_md = match (
            workspace_root.is_dir(),
            workspace_root.is_file(),
            workspace_root.exists(),
        ) {
            // its a folder and it exists
            (true, _, true) => {
                let mut maybe_md_path = workspace_root.clone();
                maybe_md_path.push("README.md");
                Some(maybe_md_path)
            }
            // its a file and it exists
            (_, true, true) => {
                let path_to_md = Some(workspace_root.clone());
                workspace_root = env::current_dir()?;

                path_to_md
            }
            // its a file, but not a correct path?
            (_, true, false) => {
                let current_dir = env::current_dir()?;
                let mut maybe_md_path = PathBuf::from(&current_dir);
                maybe_md_path.push(&workspace_root);
                workspace_root = current_dir;
                if maybe_md_path.exists() {
                    Some(maybe_md_path)
                } else {
                    None
                }
            }
            _ => None,
        };

        Ok(Self {
            path_to_md,
            workspace_root,
        })
    }
}

/// Tests of text parsing
#[cfg(test)]
mod tests {
    use crate::cli::args::Args;
    use crate::executor::job_location::JobLocation;
    use std::env;

    #[test]
    fn test_job_location_without_path() {
        let args = Args::default();
        let location = JobLocation::new(&args).unwrap();
        assert!(location.path_to_md.is_some());
        assert!(location.path_to_md.unwrap().is_file());
        assert!(location.workspace_root.is_dir());
    }

    #[test]
    fn test_job_location_with_path_as_dir() {
        let args = Args {
            path: Some(String::from(env::current_dir().unwrap().to_string_lossy())),
            ..Args::default()
        };
        let location = JobLocation::new(&args).unwrap();
        assert!(location.path_to_md.is_some());
        assert!(location.path_to_md.unwrap().is_file());
        assert!(location.workspace_root.is_dir());
    }

    #[test]
    fn test_job_location_with_path_as_file() {
        let args = Args {
            path: Some("README.md".to_string()),
            ..Args::default()
        };
        let location = JobLocation::new(&args).unwrap();
        assert!(location.path_to_md.is_some());
        assert!(location.path_to_md.unwrap().is_file());
        assert!(location.workspace_root.is_dir());
    }
}

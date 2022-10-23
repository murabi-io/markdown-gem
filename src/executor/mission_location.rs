use {
    crate::*,
    anyhow::Result,
    std::{env, fmt, path::PathBuf},
};

use crate::cli::args::Args;

#[derive(Clone, PartialEq, Eq)]
pub struct MissionLocation {
    pub workspace_root: PathBuf,
    pub path_to_md: Option<PathBuf>,
}

impl fmt::Debug for MissionLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MissionLocation")
            .field("workspace_root", &self.workspace_root)
            .field("path_to_md", &self.path_to_md)
            .finish()
    }
}

impl MissionLocation {
    pub fn new(args: &Args) -> Result<Self> {
        let mut workspace_root = args
            .path
            .as_ref()
            .map_or_else(|| env::current_dir().unwrap(), PathBuf::from);
        let workspace_root_str = String::from(workspace_root.to_string_lossy());

        let maybe_md_path_str = format!("{}/README.md", workspace_root_str);
        let mut path_to_md = None;
        if workspace_root.is_dir() {
            let new_path = PathBuf::from(maybe_md_path_str);
            if new_path.is_file() && new_path.exists() {
                path_to_md = Some(new_path);
            }
        } else {
            workspace_root.pop();
        }

        Ok(Self {
            path_to_md,
            workspace_root,
        })
    }
}

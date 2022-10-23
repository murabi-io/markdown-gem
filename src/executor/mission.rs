use crate::cli::action::Action;
use {
    crate::*,
    anyhow::Result,
    std::{collections::HashSet, path::PathBuf, process::Command},
};

use crate::executor::job::Job;
use crate::executor::mission_location::MissionLocation;

/// the description of the mission of murabi
/// after analysis of the args, env, and surroundings
#[derive(Debug)]
pub struct Mission {
    pub location_name: String,
    pub job_name: String,
    pub workspace_root: PathBuf,
    job: Job,
    // files_to_watch: Vec<PathBuf>,
    // directories_to_watch: Vec<PathBuf>,
    // pub settings: &'s Settings,
}

impl Mission {
    pub fn new(location: &MissionLocation, job_name: String, job: Job) -> Result<Self> {
        let location_name = "".to_string(); //location.name();

        Ok(Mission {
            location_name,
            job_name,
            workspace_root: location.workspace_root.clone(),
            job,
        })
    }

    /// the action bound to success on this job
    pub fn on_success(&self) -> &Option<Action> {
        &self.job.on_success
    }

    pub fn allow_warnings(&self) -> bool {
        self.job.allow_warnings
    }
    pub fn get_command(&self) -> Command {
        Command::new("ls -l")
    }
    // /// build (and doesn't call) the external cargo command
    // pub fn get_command(&self) -> Command {
    //     let mut tokens = self.job.command.iter();
    //     let mut command = Command::new(
    //         tokens.next().unwrap(), // implies a check in the job
    //     );
    //     let mut no_default_features_done = false;
    //     let mut features_done = false;
    //     let mut last_is_features = false;
    //     let tokens = tokens.chain(&self.settings.additional_job_args);
    //     for arg in tokens {
    //         if last_is_features {
    //             if self.settings.all_features {
    //                 debug!("ignoring features given along --all-features");
    //             } else {
    //                 features_done = true;
    //                 // arg is expected there to be the list of features
    //                 match (&self.settings.features, self.settings.no_default_features) {
    //                     (Some(features), false) => {
    //                         // we take the features of both the job and the args
    //                         command.arg("--features");
    //                         command.arg(merge_features(arg, features));
    //                     }
    //                     (Some(features), true) => {
    //                         // arg add features and remove the job ones
    //                         command.arg("--features");
    //                         command.arg(&features);
    //                     }
    //                     (None, true) => {
    //                         // we pass no feature
    //                     }
    //                     (None, false) => {
    //                         // nothing to change
    //                         command.arg("--features");
    //                         command.arg(arg);
    //                     }
    //                 }
    //             }
    //             last_is_features = false;
    //         } else if arg == "--no-default-features" {
    //             no_default_features_done = true;
    //             last_is_features = false;
    //             command.arg(arg);
    //         } else if arg == "--features" {
    //             last_is_features = true;
    //         } else {
    //             command.arg(arg);
    //         }
    //     }
    //     if self.settings.no_default_features && !no_default_features_done {
    //         command.arg("--no-default-features");
    //     }
    //     if self.settings.all_features {
    //         command.arg("--all-features");
    //     }
    //     if !features_done {
    //         if let Some(features) = &self.settings.features {
    //             if self.settings.all_features {
    //                 debug!("not using features because of --all-features");
    //             } else {
    //                 command.arg("--features");
    //                 command.arg(features);
    //             }
    //         }
    //     }
    //     command.current_dir(&self.cargo_execution_directory);
    //     debug!("command: {:#?}", &command);
    //     command
    // }

    /// whether we need stdout and not just stderr
    pub fn need_stdout(&self) -> bool {
        self.job.need_stdout
    }
}

fn merge_features(a: &str, b: &str) -> String {
    let mut features = HashSet::new();
    for feature in a.split(',') {
        features.insert(feature);
    }
    for feature in b.split(',') {
        features.insert(feature);
    }
    features.iter().copied().collect::<Vec<&str>>().join(",")
}

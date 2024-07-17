use log::{error, info};

use crate::Config;


pub fn use_git(config: &Config) -> bool {
    config.git_repo.is_some()
}

macro_rules! check_active {
    ($c:ident) => {
        if !use_git($c) {
            return;
        }
    };
    ($c:ident, $re:expr) => {
        if !use_git($c) {
            return $re;
        }
    };
}

pub fn clone_repo(config: &Config) -> bool {
    check_active!(config, true);

    info!("Cloning git repo...");

    // We use std version here, due to being in startup still and in synchronos enviroment
    let err = match std::process::Command::new("git")
        .arg("clone")
        .arg(config.git_repo.as_ref().expect("we exit if inactive anyway"))
        .arg(config.folder_path.as_path())
        .spawn().map(|mut child| child.wait()) {

        Ok(Ok(status)) => {
            if status.success() {
                info!("Cloned successfully");
                return true;
            }
            
            if let Some(code) = status.code() {
                format!("command exited with code {code}")
            } else {
                format!("command terminated unsuccessfully")
            }
        },
        Ok(Err(e)) => format!("failed to wait for command exit: {e}"),
        Err(e) => format!("command failed to launch: {e}")
    };

    error!("Clone unsucessful, {err}");

    return false;
}

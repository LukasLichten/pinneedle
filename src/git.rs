use std::{error::Error, os::unix::fs::MetadataExt, path::PathBuf};

use log::{error, info};
use tokio::process::Command;

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

pub async fn get_creation_date(config: &Config, path: &PathBuf) -> Result<i64, Box<dyn Error>> {
    if use_git(config) {
        let child = Command::new("git")
            .arg("-C")
            .arg(config.folder_path.as_path())
            .arg("log")
            .arg("--pretty=format:\"%at\"")
            .arg("--reverse")
            .arg("--")
            .arg(path.as_path())
            .spawn()?;

        let out = child.wait_with_output().await?;

        if !out.status.success() {
            return Err(out.status.to_string().into());
        }

        let s = String::from_utf8(out.stdout)?;
        let stamp: i64 = match s.split('\n').next() {
            Some(item) => item.parse()?,
            None => return Err("Unable to extract anything out of stdout".into())
        };

        Ok(stamp)
    } else {
        let meta = std::fs::metadata(path.as_path())?;
        Ok(meta.ctime())
    }

}

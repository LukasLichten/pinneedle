use std::{error::Error, os::unix::fs::MetadataExt, path::PathBuf, process::Stdio};

use log::{debug, error, info};
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

        .stderr(Stdio::null()) // for some fucking reason this logs to stderr

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

        let path = path.strip_prefix(&config.folder_path)?;

        let out = Command::new("git")
            .arg("-C")
            .arg(config.folder_path.as_path())
            .arg("log")
            .arg("--pretty=format:%at")
            .arg("--reverse")
            .arg("--")
            .arg(path)

            .output().await?;

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

const UPDATER_SLEEP_DURATION: std::time::Duration = std::time::Duration::from_secs(5 * 60);

pub async fn updater(config: &'static Config) {
    async fn single_iteration(config: &Config) -> Result<(),Box<dyn Error>> {
        let mut child = Command::new("git")
            .arg("-C")
            .arg(config.folder_path.as_path())
            .arg("pull")

            .stdout(Stdio::null()) 
            // stderror we leave attached, so errors can be seen still

            .spawn()?;
        
        let status = child.wait().await?;

        if !status.success() {
            return Err(
                if let Some(code) = status.code() {
                    format!("command exited with code {code}")
                } else {
                    format!("command terminated unsuccessfully")
                }.into()
            );
        }

        Ok(())
    }
    
    loop {
        match single_iteration(config).await {
            Ok(()) => debug!("Checked Repo successfully for updates"),
            Err(e) => error!("Error occured when trying to pull repo: {e}")
        };

        tokio::time::sleep(UPDATER_SLEEP_DURATION).await;
    }
}

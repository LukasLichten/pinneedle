use std::{path::PathBuf, str::FromStr, sync::Arc};

use axum::routing::get;
use log::{error, info};
use tokio::net::TcpListener;

pub mod pages;
pub mod git;

// Defaults for enviroment variables of the same name
const PIN_LOCAL_PATH: &'static str = "./blog";
const PIN_BLOG_REPO: Option<String> = None;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Starting logger
    let log_level = log::LevelFilter::Debug;
    env_logger::builder().filter_level(log_level).init();

    info!("Reading Enviroment...");

    let config = Arc::new(Config::new());

    info!("Starting up webserver...");

    let app = axum::Router::new()
        .route("/", get(pages::index))
        // .route("/post/:id", get(pages::post))
        .route("/style.css", get(pages::serve_css_style))
        .with_state(config);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    info!("Webserver Launched");
    axum::serve(listener, app).await?;

    info!("Shutdown complete");

    Ok(())
}



#[derive(Debug)]
pub struct Config {
    pub folder_path: PathBuf,
    pub git_repo: Option<String>
}

impl Config {
    fn new() -> Self {
        // Reading folder path
        let path = match std::env::var("PIN_LOCAL_PATH") {
            Ok(p) => p,
            Err(std::env::VarError::NotPresent) => PIN_LOCAL_PATH.to_string(),
            Err(std::env::VarError::NotUnicode(_)) => {
                error!("Invalid unicode set in Enviroment Variable PIN_LOCAL_PATH");
                error!("Exiting!");
                std::process::exit(1);
            }
        };

        let folder = match PathBuf::from_str(path.as_str()) {
            Ok(f) => f,
            Err(e) => {
                error!("Unable to parse path '{path}': {e}");
                if std::env::var("PIN_LOCAL_PATH").is_ok() {
                    error!("Falling back to default...");
                    std::env::remove_var("PIN_LOCAL_PATH");
                    return Config::new();
                } else {
                    error!("Unable to parse fallback, aborting!");
                    std::process::exit(1);
                }
            }
        };

        info!("Using local folder: {}", folder.to_string_lossy());

        // Reading the git repo address
        let git_repo = match std::env::var("PIN_BLOG_REPO") {
            Ok(p) => Some(p),
            Err(std::env::VarError::NotPresent) => PIN_BLOG_REPO,
            Err(std::env::VarError::NotUnicode(_)) => {
                error!("Invalid unicode set in Enviroment Variable PIN_BLOG_REPO");
                error!("Exiting!");
                std::process::exit(2);
            }
        };

        match &git_repo {
            Some(repo) => info!("Using git repo {repo}"),
            None => info!("No git repo set, git features disabled")
        };

        


        // We only clone the repo if it doesn't exist
        if !folder.exists() {
            info!("Local folder does not exist, creating...");
            
            match std::fs::create_dir_all(folder.as_path()) {
                Ok(()) => (),
                Err(e) => {
                    error!("Failed to create local folder: {e}");
                    error!("Exiting!");
                    std::process::exit(1);
                }
            }

            let config = Config {
                folder_path: folder,
                git_repo
            };

            if !git::clone_repo(&config) {
                error!("Exiting!");
                std::process::exit(2);
            }

            config
        } else {
            Config {
                folder_path: folder,
                git_repo
            }
        }
    }
}

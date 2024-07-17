use std::{error::Error, sync::Arc};

use axum::{extract::State, http::StatusCode, response::Response};
use maud::{html, Markup, DOCTYPE};

use crate::Config;
type StateConfig = State<Arc<Config>>;

fn header(name: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { "Blog - " (name) }
        link rel="stylesheet" href="style.css";
    }
} 

pub async fn index(State(_config): StateConfig) -> Markup {
    html!{
        (header("Home"))

        p {
            "Hello World"
        }
    }
}

pub async fn serve_css_style(State(config): StateConfig) -> Result<Response, StatusCode> {
    let b = axum::body::Body::try_from(
            read_str(&config, "style.css")
                .await
                .map_err(|_| StatusCode::NOT_FOUND)?
        )
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Response::builder()
        .status(200)
        .header(axum::http::header::CONTENT_TYPE, "text/css")
        .body(b)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn read_str(config: &Config, internal_path: &str) -> Result<String, Box<dyn Error>> {
    let res = read_file(config, internal_path).await?;
    Ok(String::from_utf8(res)?)
}

async fn read_file(config: &Config, internal_path: &str) -> std::io::Result<Vec<u8>> {
    let mut path = config.folder_path.clone();
    path.push(internal_path);


    tokio::fs::read(path).await
}

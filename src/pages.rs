use std::{error::Error, sync::Arc};

use axum::{extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}};
use log::error;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use tokio::fs;

use crate::{git, Config};
type StateConfig = State<Arc<Config>>;

fn header(name: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (name) }
        link rel="stylesheet" href="/style.css";
    }
} 

pub fn error_page(code: StatusCode, reason: &str) -> Response {
    let page = html!{
        (header("Error"))

        h1 { "Error " (code.as_str())  }
        h2 { (reason) }
    };
    
    let mut res = page.into_response();
    *res.status_mut() = code;

    res
}

pub async fn index(State(config): StateConfig) -> Result<Markup,Response> {
    let mut posts_folder = config.folder_path.clone();
    posts_folder.push("posts");
    
    let mut posts = Vec::<(i64,PreEscaped<String>)>::new();

    if posts_folder.exists() {
        let mut iter = fs::read_dir(posts_folder).await
            .map_err(|e| error_page(StatusCode::INTERNAL_SERVER_ERROR, e.to_string().as_str()))?;

        while let Ok(Some(item)) = iter.next_entry().await {
            let path = item.path();
            if path.extension().map_or(false, |ex| ex.to_ascii_lowercase() == "md") {
                if let (Some(Some(stem)), Ok(timestamp)) = (path.file_stem().map(|stem| stem.to_str()), git::get_creation_date(&config, &path).await) {
                    let entry = html!{
                        a class="posts-list-link" href=(format!("post/{stem}")) { (stem) }
                    };

                    
                    posts.push((timestamp,entry));
                }
                
            }
            
        }
    } else {
        error!("No posts folder!");
    }

    posts.sort_by(|(a,_),(b,_)| b.cmp(a));

    Ok(html!{
        (header("Blog - Home"))

        h1 class="home-heading" { "Hello World" }
        a class="home-about-button" { "About" }

        ul class="posts-list" {
            @for (_,p) in posts {
                li { (p) }
            }
        }
    })
}

pub async fn post(State(config): StateConfig, Path(path): Path<String>) -> Result<Markup, Response> {
    if path.contains("..") {
        return Err(error_page(StatusCode::NOT_ACCEPTABLE, "oh, so you try to escape? Though luck"));
    }

    let markdown = match read_str(&config, format!("posts/{path}.md").as_str()).await {
        Ok(text) => text,
        Err(e) => return Err(error_page(StatusCode::NOT_FOUND, &e.to_string()))
    };

    
    let markup = markdown::to_html(markdown.as_str());

    Ok(html!{
        (header(&path))
        
        a class="post-back-button" href="/" { "← Back" }
        (PreEscaped(markup))
    })
}

pub async fn serve_css_style(State(config): StateConfig) -> Result<Response, StatusCode> {
    let b = axum::body::Body::try_from(
            read_str(&config, "style.css")
                .await
                .map_err(|_| { error!("style.css not found"); StatusCode::NOT_FOUND })?
        )
        .map_err(|_| { error!("unable to parse style.css"); StatusCode::UNPROCESSABLE_ENTITY })?;

    Response::builder()
        .status(200)
        .header(axum::http::header::CONTENT_TYPE, "text/css")
        .body(b)
        .map_err(|_| { error!("unable to send style.css response"); StatusCode::INTERNAL_SERVER_ERROR })
}

async fn read_str(config: &Config, internal_path: &str) -> Result<String, Box<dyn Error>> {
    let res = read_file(config, internal_path).await?;
    Ok(String::from_utf8(res)?)
}

async fn read_file(config: &Config, internal_path: &str) -> std::io::Result<Vec<u8>> {
    let mut path = config.folder_path.clone();
    path.push(internal_path);


    fs::read(path).await
}

use anyhow::Context;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use std::{fs, path::PathBuf};
use tower_http::services::ServeFile;

/// Serves a file from the ./assets directory or its subdirectories.
pub async fn serve_file(filename: String) -> Result<Response<Body>, anyhow::Error> {
    let assets_path = PathBuf::from("./assets");
    let mut file_path = None;

    for entry in fs::read_dir(&assets_path).context("Failed to read assets directory")? {
        let category_path = entry?.path();
        if category_path.is_dir() {
            let potential_path = category_path.join(&filename);
            if potential_path.exists() {
                file_path = Some(potential_path);
                break;
            }
        }
    }

    match file_path {
        Some(path) => {
            let mut service = ServeFile::new(path);
            let request = Request::builder()
                .body(Body::empty())
                .map_err(|e| anyhow::anyhow!(e))?;

            let response = service
                .try_call(request)
                .await
                .map_err(|e| anyhow::anyhow!(e))?;

            Ok(response.map(Body::new))
        }
        None => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("File not found"))
            .unwrap()),
    }
}

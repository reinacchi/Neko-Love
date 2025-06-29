use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use std::{fs, path::PathBuf};
use tower_http::services::ServeFile;

use crate::models::response::ApiResponse;

/// Serves a file from either ./assets/sfw or ./assets/nsfw subdirectories
pub async fn serve_file(filename: String) -> Result<Response<Body>, anyhow::Error> {
    let content_types = ["sfw", "nsfw"];
    let assets_base = PathBuf::from("./assets");

    for content_type in content_types.iter() {
        let content_path = assets_base.join(content_type);

        if let Ok(entries) = fs::read_dir(&content_path) {
            for entry in entries {
                let category_path = entry?.path();
                if category_path.is_dir() {
                    let potential_path = category_path.join(&filename);
                    if potential_path.exists() {
                        let mut service = ServeFile::new(potential_path);
                        let request = Request::builder()
                            .body(Body::empty())
                            .map_err(|e| anyhow::anyhow!(e))?;

                        let response = service
                            .try_call(request)
                            .await
                            .map_err(|e| anyhow::anyhow!(e))?;

                        return Ok(response.map(Body::new));
                    }
                }
            }
        }
    }

    let response = ApiResponse {
        id: None,
        message: "File not found.".into(),
        success: false,
        status: StatusCode::NOT_FOUND.as_u16(),
        url: None,
    };

    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(serde_json::to_string(&response)?))
        .unwrap())
}

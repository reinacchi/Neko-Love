use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json, http::StatusCode,
};
use crate::models::response::ImageResponse;
use std::sync::Arc;
use crate::services::image_service::ImageService;

/// Handler for GET /api/v4/{category}
/// Returns a random image from the specified category
pub async fn get_random_image(
    Path(category): Path<String>,
    State(image_service): State<Arc<ImageService>>,
) -> impl IntoResponse {
    match image_service.get_random_image(&category) {
        Ok((id, filename)) => {
            let response = ImageResponse {
                id: id.clone(),
                success: true,
                status: StatusCode::OK.as_u16(),
                url: image_service.build_image_url(&filename),
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            eprintln!("Error getting random image: {}", e);
            let response = ImageResponse {
                id: "".into(),
                success: false,
                status: StatusCode::NOT_FOUND.as_u16(),
                url: "".into(),
            };
            (StatusCode::NOT_FOUND, Json(response))
        }
    }
}
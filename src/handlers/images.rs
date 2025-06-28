use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
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
            Json(ImageResponse {
                id: id.clone(),
                url: image_service.build_image_url(&category, &filename),
            })
        }
        Err(e) => {
            eprintln!("Error getting random image: {}", e);
            Json(ImageResponse {
                id: "default".into(),
                url: "".into(),
            })
        }
    }
}
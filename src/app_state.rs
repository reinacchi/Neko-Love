use std::path::PathBuf;
use std::sync::Arc;
use crate::services::image_service::ImageService;
use anyhow::Result;

/// Shared application state containing the ImageService
pub type AppState = Arc<ImageService>;

/// Creates a new shared AppState with the given assets path and base URL
pub fn create_state(assets_path: PathBuf, base_url: String) -> Result<AppState> {
    Ok(Arc::new(ImageService::new(assets_path, base_url)?))
}
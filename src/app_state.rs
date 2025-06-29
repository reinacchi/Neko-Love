use bytes::Bytes;
use moka::future::Cache;
use std::path::PathBuf;
use std::sync::Arc;
use crate::services::image_service::ImageService;
use anyhow::Result;

#[derive(Clone)]
/// Shared application state containing the ImageService
pub struct AppState {
    pub cache: Arc<Cache<String, Bytes>>,
    pub image_service: Arc<ImageService>,
}

/// Creates a new shared AppState with the given assets path and base URL
pub fn create_state(assets_path: PathBuf, base_url: String) -> Result<AppState> {
    let image_service = Arc::new(ImageService::new(assets_path.clone(), base_url)?);

    let cache = Arc::new(
        Cache::builder()
            .max_capacity(1000) // limit entries
            .time_to_live(std::time::Duration::from_secs(300)) // 5 min TTL
            .build(),
    );

    Ok(AppState {
        cache,
        image_service,
    })
}
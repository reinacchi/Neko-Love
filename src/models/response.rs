use serde::Serialize;

/// The image response
#[derive(Debug, Serialize)]
pub struct ImageResponse {
    pub id: String,
    pub url: String,
}

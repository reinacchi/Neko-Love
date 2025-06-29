use serde::Serialize;

/// The image response
#[derive(Debug, Serialize)]
pub struct ImageResponse {
    pub id: String,
    pub success: bool,
    pub status: u16,
    pub url: String,
}

mod app_state;
mod handlers;
mod models;
mod services;

use axum::{routing::get, Router};
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;
use tower_http::services::ServeDir;

use crate::app_state::create_state;
use crate::handlers::images::get_random_image;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let server_addr = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3030".into());
    let assets_path = PathBuf::from("./assets");
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3030".into());
    let state = create_state(assets_path, base_url).unwrap();
    
    let app = Router::new()
        .route("/api/v4/{category}", get(get_random_image))
        .nest_service("/images", ServeDir::new("assets"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(server_addr).await.unwrap();
    println!(
        "Server running on {}",
        listener.local_addr().unwrap().to_string()
    );

    axum::serve(listener, app).await.unwrap();
}

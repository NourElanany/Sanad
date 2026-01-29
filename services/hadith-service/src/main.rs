use axum::{routing::get, Router, response::Json};
use shared::{AppConfig, ApiResponse};
use std::collections::HashMap;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();
    info!("Starting Hadith Service on port 8082");

    let app = Router::new().route("/health", get(health_check));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "hadith-service".to_string());
    Json(ApiResponse::success(status))
}
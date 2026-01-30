use axum::{routing::get, Router, response::Json};
use shared::ApiResponse;
use std::collections::HashMap;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    info!("Starting AI Service with RAG on port 8086");

    let app = Router::new().route("/health", get(health_check));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8086").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "ai-service".to_string());
    status.insert("features".to_string(), "RAG,Anti-Hallucination".to_string());
    Json(ApiResponse::success(status))
}
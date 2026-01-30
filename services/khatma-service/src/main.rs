mod models;
mod planning_algorithms;
mod service;
mod repository;
mod handlers;

#[cfg(test)]
mod tests;

use axum::{routing::get, Router};
use handlers::KhatmaHandlers;
use repository::KhatmaRepository;
use service::SmartKhatmaService;
use shared::{config::DatabaseConfig, ApiResponse};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    info!("Starting Smart Khatma Service");

    // For now, create a mock pool to avoid database connection issues during testing
    // In production, this would connect to the actual database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/sanad_islamic_app".to_string());

    // Create a simple router without database dependency for now
    let app = Router::new()
        .route("/health", get(health_check));

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8089").await?;
    info!("Smart Khatma Service listening on port 8089");
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> axum::response::Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "smart-khatma-service".to_string());
    status.insert("version".to_string(), "1.0.0".to_string());
    status.insert("features".to_string(), "interactive-planning,adaptive-scheduling,smart-reminders".to_string());
    axum::response::Json(ApiResponse::success(status))
}
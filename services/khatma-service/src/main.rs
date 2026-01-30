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

    // Load configuration
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/sanad_islamic_app".to_string());

    // Create database connection pool
    let pool = PgPool::connect(&database_url).await?;
    info!("Connected to database");

    // Run migrations
    sqlx::migrate!("../../database/migrations").run(&pool).await?;
    info!("Database migrations completed");

    // Create repository and service
    let repository = KhatmaRepository::new(pool);
    let service = Arc::new(SmartKhatmaService::new(repository));

    // Create router with all endpoints
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1/khatma", KhatmaHandlers::router(service));

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
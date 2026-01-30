mod models;
mod repository;
mod service;
mod handlers;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_runner;

use handlers::create_router;
use repository::StoryRepository;
use service::StoryService;
use shared::config::DatabaseConfig;
use sqlx::PgPool;
use std::env;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    info!("Starting Stories Service on port 8083");

    // Load database configuration
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/sanad".to_string());

    // Create database connection pool
    let pool = match PgPool::connect(&database_url).await {
        Ok(pool) => {
            info!("Successfully connected to database");
            pool
        }
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            return Err(e.into());
        }
    };

    // Run database migrations
    match sqlx::migrate!("../../database/migrations").run(&pool).await {
        Ok(_) => info!("Database migrations completed successfully"),
        Err(e) => {
            error!("Failed to run database migrations: {}", e);
            return Err(e.into());
        }
    }

    // Create repository and service
    let repository = StoryRepository::new(pool);
    let service = StoryService::new(repository);

    // Create router with all endpoints
    let app = create_router(service);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8083").await?;
    info!("Stories Service is running on http://0.0.0.0:8083");
    
    axum::serve(listener, app).await?;
    Ok(())
}
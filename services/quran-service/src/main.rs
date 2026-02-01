mod models;
mod repository;
mod service;
mod handlers;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod translation_tests;

use handlers::create_router;
use repository::QuranRepository;
use service::QuranService;
use shared::AppConfig;
use sqlx::PgPool;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load configuration
    let _config = AppConfig::load()
        .map_err(|e| format!("Failed to load config: {}", e))?;

    info!("Starting Quran Service on port 8081");

    // Connect to database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/sanad".to_string());

    let pool = PgPool::connect(&database_url).await
        .map_err(|e| {
            error!("Failed to connect to database: {}", e);
            format!("Database connection failed: {}", e)
        })?;

    info!("Connected to database successfully");

    // Run database migrations if needed
    sqlx::migrate!("../../database/migrations").run(&pool).await
        .map_err(|e| {
            error!("Failed to run migrations: {}", e);
            format!("Migration failed: {}", e)
        })?;

    info!("Database migrations completed");

    // Create repository and service
    let repository = QuranRepository::new(pool);
    let service = QuranService::new(repository);

    // Create router
    let app = create_router(service);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await?;
    info!("Quran Service listening on 0.0.0.0:8081");

    axum::serve(listener, app).await?;

    Ok(())
}
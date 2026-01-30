use axum::{
    routing::get,
    Router,
};
use hadith_service::{create_router, HadithRepository, HadithService};
use shared::AppConfig;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = AppConfig::load().map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;
    config.validate().map_err(|e| anyhow::anyhow!("Config validation failed: {}", e))?;
    
    info!("Starting Hadith Service...");
    info!("Database URL: {}", config.database.url.chars().take(20).collect::<String>() + "...");

    // Create database connection pool
    let pool = create_pool(&config.database.url, &config.database).await?;
    
    // Run database migrations
    info!("Running database migrations...");
    sqlx::migrate!("../../database/migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            warn!("Migration failed: {}", e);
            e
        })?;
    
    info!("Database migrations completed successfully");

    // Create repository and service
    let repository = HadithRepository::new(pool);
    let service = HadithService::new(repository);

    // Create router with all endpoints
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", create_router(service))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        );

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("Hadith Service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Create database connection pool
async fn create_pool(database_url: &str, config: &shared::DatabaseConfig) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(std::time::Duration::from_secs(config.connection_timeout_seconds))
        .idle_timeout(std::time::Duration::from_secs(config.idle_timeout_seconds))
        .max_lifetime(std::time::Duration::from_secs(config.max_lifetime_seconds))
        .connect(database_url)
        .await?;

    Ok(pool)
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "Hadith Service is healthy"
}
use widgets_service::{WidgetService, WidgetRepository, create_router};
use sqlx::PgPool;
use redis::Client as RedisClient;
use std::env;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::init();

    // Load configuration
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost/sanad_db".to_string());
    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    // Service URLs
    let prayer_service_url = env::var("PRAYER_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:8001".to_string());
    let quran_service_url = env::var("QURAN_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:8002".to_string());
    let khatma_service_url = env::var("KHATMA_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:8003".to_string());
    let notification_service_url = env::var("NOTIFICATION_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:8004".to_string());

    info!("Starting Widgets Service on port {}", port);

    // Connect to database
    let pool = PgPool::connect(&database_url).await?;
    info!("Connected to database");

    // Connect to Redis
    let redis_client = RedisClient::open(redis_url)?;
    info!("Connected to Redis");

    // Create repository and service
    let repository = WidgetRepository::new(pool);
    let service = WidgetService::new(
        repository,
        redis_client,
        prayer_service_url,
        quran_service_url,
        khatma_service_url,
        notification_service_url,
    );

    // Create router
    let app = create_router(service);

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Widgets Service listening on port {}", port);

    axum::serve(listener, app).await?;

    Ok(())
}
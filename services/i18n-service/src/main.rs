use i18n_service::{
    I18nService, I18nRepository, handlers::create_i18n_routes
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting I18n Service");

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/sanad_islamic_app".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    info!("Connected to database");

    // Run migrations
    sqlx::migrate!("../../database/migrations").run(&pool).await?;
    info!("Database migrations completed");

    // Initialize repository and service
    let repository = I18nRepository::new(pool);
    let translations_path = std::env::var("TRANSLATIONS_PATH")
        .unwrap_or_else(|_| "translations".to_string());
    
    let service = Arc::new(I18nService::new(repository, translations_path));
    
    // Initialize the service (load all language packs)
    if let Err(e) = service.initialize().await {
        error!("Failed to initialize I18n service: {}", e);
        return Err(e.into());
    }
    
    info!("I18n service initialized successfully");

    // Create router
    let app = create_i18n_routes(service);

    // Server configuration
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);

    info!("Starting server on {}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
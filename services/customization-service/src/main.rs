use customization_service::{SmartCustomizationService, CustomizationRepository, CustomizationHandlers};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{info, error};
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::init();

    info!("Starting Smart Customization Service...");

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost/sanad_db".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    
    // Run migrations
    sqlx::migrate!("../../database/migrations").run(&pool).await?;
    
    // Initialize repository and service
    let repository = CustomizationRepository::new(pool);
    let service = Arc::new(SmartCustomizationService::new(repository));
    
    // Create router
    let app = Router::new()
        .nest("/api/v1/customization", CustomizationHandlers::create_router(service));
    
    // Start server
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    info!("Smart Customization Service listening on {}", addr);
    
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
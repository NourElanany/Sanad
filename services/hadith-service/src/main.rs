use hadith_service::{create_router, HadithRepository, HadithService};
use sqlx::PgPool;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    info!("Starting Hadith Service on port 8082");

    // Connect to database
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost/sanad_db".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    info!("Connected to database");

    // Create repository and service
    let repository = HadithRepository::new(pool);
    let service = HadithService::new(repository);

    // Create router with all endpoints
    let app = create_router(service);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await?;
    info!("Hadith Service listening on 0.0.0.0:8082");
    
    axum::serve(listener, app).await?;
    Ok(())
}
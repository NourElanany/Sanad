mod models;
mod repository;
mod service;
mod handlers;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod property_tests;

#[cfg(test)]
mod simple_logic_test;

use handlers::{create_router, SharedNotificationService};
use repository::NotificationRepository;
use service::NotificationService;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    info!("Starting Notification Service on port 8090");

    // Load configuration
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/sanad_islamic_app".to_string());
    
    // Create database connection pool
    let pool = PgPool::connect(&database_url).await?;
    
    // Run database migrations
    sqlx::migrate!("../../database/migrations").run(&pool).await?;
    info!("Database migrations completed");

    // Create service instances
    let repository = NotificationRepository::new(pool);
    let service = Arc::new(NotificationService::new(repository));
    
    // Start background task for processing notifications
    let service_clone = Arc::clone(&service);
    tokio::spawn(async move {
        notification_processor_task(service_clone).await;
    });

    // Start background task for scheduling seasonal notifications
    let service_clone = Arc::clone(&service);
    tokio::spawn(async move {
        seasonal_scheduler_task(service_clone).await;
    });

    // Create router with service state
    let app = create_router(service);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8090").await?;
    info!("Notification service listening on 0.0.0.0:8090");
    
    axum::serve(listener, app).await?;
    Ok(())
}

/// Background task to process pending notifications
async fn notification_processor_task(service: SharedNotificationService) {
    let mut interval = interval(Duration::from_secs(30)); // Process every 30 seconds
    
    loop {
        interval.tick().await;
        
        match service.process_pending_notifications(50).await {
            Ok(count) => {
                if count > 0 {
                    info!("Processed {} pending notifications", count);
                }
            }
            Err(e) => {
                error!("Error processing pending notifications: {}", e);
            }
        }
    }
}

/// Background task to schedule seasonal notifications
async fn seasonal_scheduler_task(service: SharedNotificationService) {
    let mut interval = interval(Duration::from_secs(3600)); // Check every hour
    
    loop {
        interval.tick().await;
        
        match service.schedule_upcoming_seasonal_notifications().await {
            Ok(count) => {
                if count > 0 {
                    info!("Scheduled {} seasonal notifications", count);
                }
            }
            Err(e) => {
                error!("Error scheduling seasonal notifications: {}", e);
            }
        }
    }
}
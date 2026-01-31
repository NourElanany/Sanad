mod models;
mod calculator;
mod hijri_calendar;
mod repository;
mod service;
mod handlers;

use axum::{routing::get, Router, Extension};
use sqlx::PgPool;
use std::env;
use tracing::{info, error};
use repository::PrayerTimesRepository;
use service::PrayerTimesService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    info!("Starting Prayer Times Service on port 8084");

    // Database connection
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://sanad_user:sanad_password@localhost:5432/sanad_db".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    
    // Run migrations
    sqlx::migrate!("../../database/migrations").run(&pool).await?;
    
    // Initialize repository and service
    let repository = PrayerTimesRepository::new(pool);
    let service = PrayerTimesService::new(repository);
    
    // Build router
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/prayer-times", get(handlers::calculate_prayer_times))
        .route("/qibla", get(handlers::calculate_qibla_direction))
        .route("/hijri-conversion", get(handlers::gregorian_to_hijri))
        .route("/gregorian-conversion", get(handlers::hijri_to_gregorian))
        .route("/islamic-events", get(handlers::get_islamic_events))
        .route("/calendar/:hijri_year/:hijri_month", get(handlers::get_monthly_calendar))
        .layer(Extension(service));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8084").await?;
    info!("Prayer Times Service listening on port 8084");
    
    axum::serve(listener, app).await?;
    Ok(())
}
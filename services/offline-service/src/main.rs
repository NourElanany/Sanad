mod models;
mod storage_manager;
mod sync_manager;
mod service;
mod handlers;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod offline_property_tests;

use anyhow::Result;
use axum::Router;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::models::*;
use crate::service::{OfflineService, OfflineServiceBuilder};
use crate::handlers::OfflineHandlers;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "offline_service=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Offline Service for Sanad Islamic Application");

    // Load configuration
    let config = load_config().await?;
    
    // Create storage directory
    let storage_path = PathBuf::from("./data/offline");
    tokio::fs::create_dir_all(&storage_path).await?;

    // Build offline service
    let service = Arc::new(
        OfflineServiceBuilder::new()
            .storage_path(storage_path)
            .config(config)
            .server_url("http://localhost:8080".to_string()) // Main API server
            .build()
            .await?
    );

    // Start the service
    service.start().await?;

    // Create HTTP router
    let app = create_app(service).await;

    // Start HTTP server
    let listener = TcpListener::bind("0.0.0.0:8095").await?;
    info!("Offline service listening on: {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_app(service: Arc<OfflineService>) -> Router {
    Router::new()
        .nest("/api/offline", OfflineHandlers::create_router(service))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        )
}

async fn load_config() -> Result<OfflineConfig> {
    // In a real implementation, this would load from a config file
    // For now, we'll use default configuration with some customizations
    
    let mut config = OfflineConfig::default();
    
    // Customize for Islamic application needs
    config.max_storage_mb = 4096; // 4GB for Islamic content
    config.min_free_space_mb = 200; // 200MB minimum free space
    config.enable_compression = true;
    config.auto_cleanup = true;
    config.cleanup_interval_hours = 12; // Cleanup twice daily
    
    // Sync configuration
    config.sync_config.auto_sync = true;
    config.sync_config.sync_interval_minutes = 15; // Sync every 15 minutes
    config.sync_config.wifi_only = false; // Allow sync on cellular for critical data
    config.sync_config.max_retries = 5;
    config.sync_config.retry_delay_seconds = 30;
    config.sync_config.batch_size = 25;
    config.sync_config.connection_timeout_seconds = 60;

    // Set higher priorities for Islamic content
    config.content_priorities.insert(OfflineContentType::QuranText, StoragePriority::Essential);
    config.content_priorities.insert(OfflineContentType::BasicTafsir, StoragePriority::Essential);
    config.content_priorities.insert(OfflineContentType::PrayerTimes, StoragePriority::Essential);
    config.content_priorities.insert(OfflineContentType::UserBookmarks, StoragePriority::High);
    config.content_priorities.insert(OfflineContentType::ReadingProgress, StoragePriority::High);
    config.content_priorities.insert(OfflineContentType::PersonalNotes, StoragePriority::High);
    config.content_priorities.insert(OfflineContentType::FavoriteHadith, StoragePriority::High);

    info!("Loaded offline service configuration");
    info!("Max storage: {} MB", config.max_storage_mb);
    info!("Compression enabled: {}", config.enable_compression);
    info!("Auto sync enabled: {}", config.sync_config.auto_sync);
    info!("Sync interval: {} minutes", config.sync_config.sync_interval_minutes);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_service_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = OfflineConfig::default();
        
        let service = OfflineServiceBuilder::new()
            .storage_path(temp_dir.path().to_path_buf())
            .config(config)
            .server_url("http://localhost:8080".to_string())
            .build()
            .await;

        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_config_loading() {
        let config = load_config().await.unwrap();
        
        assert_eq!(config.max_storage_mb, 4096);
        assert!(config.enable_compression);
        assert!(config.sync_config.auto_sync);
        assert_eq!(config.sync_config.sync_interval_minutes, 15);
    }

    #[test]
    fn test_storage_priorities() {
        let config = OfflineConfig::default();
        
        assert_eq!(
            config.content_priorities.get(&OfflineContentType::QuranText),
            Some(&StoragePriority::Essential)
        );
        
        assert_eq!(
            config.content_priorities.get(&OfflineContentType::UserBookmarks),
            Some(&StoragePriority::High)
        );
    }
}
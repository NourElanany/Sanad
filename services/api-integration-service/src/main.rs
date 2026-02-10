//! API Integration Service - Main Entry Point
//!
//! This binary starts the HTTP server with all API endpoints.

use api_integration_service::{create_router, load_config_from_default_location, middleware, ApiIntegrationService};
use axum::middleware as axum_middleware;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing with structured logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Starting API Integration Service...");

    // Load configuration from YAML with environment variable overrides
    info!("Loading configuration...");
    let config = load_config_from_default_location()?;
    
    let host = config.service.host.clone();
    let port = config.service.port;
    
    info!("Configuration loaded successfully");
    info!("Service: {} on {}:{}", config.service.name, host, port);
    info!("Redis: {}", config.redis.url);
    info!("Postgres: {}", config.postgres.url);

    // Create service instance
    info!("Initializing service...");
    let service = Arc::new(ApiIntegrationService::new(config).await?);

    // Create router with all endpoints and middleware
    let app = create_router(service)
        // Add CORS middleware (outermost layer)
        .layer(middleware::create_cors_layer())
        // Add security headers
        .layer(axum_middleware::from_fn(middleware::security_headers_middleware))
        // Add timeout middleware
        .layer(axum_middleware::from_fn(middleware::timeout_middleware))
        // Add metrics collection
        .layer(axum_middleware::from_fn(middleware::metrics_middleware))
        // Add request/response logging
        .layer(axum_middleware::from_fn(middleware::logging_middleware))
        // Add request ID tracking
        .layer(axum_middleware::from_fn(middleware::request_id_middleware))
        // Add tower's trace layer for additional observability
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = format!("{}:{}", host, port);
    info!("Server listening on {}", addr);
    info!("Middleware stack configured: CORS, Security Headers, Timeout, Metrics, Logging, Request ID");
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

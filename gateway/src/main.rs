mod routes;
mod middleware;
mod proxy;
mod auth;

use axum::{
    http::{HeaderValue, Method},
    Router,
};
use shared::{AppConfig, SanadResult};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
};
use tracing::{info, error};
use std::time::Duration;

#[tokio::main]
async fn main() -> SanadResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = AppConfig::load()
        .map_err(|e| shared::SanadError::Configuration(e.to_string()))?;
    
    config.validate()
        .map_err(|e| shared::SanadError::Configuration(e))?;

    info!("Starting Sanad API Gateway on {}:{}", config.server.host, config.server.port);

    // Build the application router
    let app = create_app(config.clone()).await?;

    // Create the server address
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await
        .map_err(|e| shared::SanadError::Internal(format!("Failed to bind to {}: {}", addr, e)))?;

    info!("API Gateway listening on {}", addr);

    // Start the server
    axum::serve(listener, app).await
        .map_err(|e| shared::SanadError::Internal(format!("Server error: {}", e)))?;

    Ok(())
}

async fn create_app(config: AppConfig) -> SanadResult<Router> {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any)
        .allow_origin(
            config.security.cors_allowed_origins
                .iter()
                .map(|origin| origin.parse::<HeaderValue>().unwrap())
                .collect::<Vec<_>>()
        );

    // Build middleware stack
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(TimeoutLayer::new(Duration::from_secs(config.server.request_timeout_seconds)))
        .layer(middleware::rate_limit::RateLimitLayer::new(
            config.security.rate_limit_requests_per_minute
        ))
        .layer(middleware::request_id::RequestIdLayer::new());

    // Create service registry for proxying requests
    let service_registry = proxy::ServiceRegistry::new(&config).await?;

    // Build the router
    let app = Router::new()
        .nest("/api/v1", routes::create_routes(service_registry, config.clone()))
        .layer(middleware)
        .fallback(routes::fallback_handler);

    Ok(app)
}
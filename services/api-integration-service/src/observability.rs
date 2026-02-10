//! Observability setup for API Integration Service
//!
//! This module sets up:
//! - Prometheus metrics exporter
//! - OpenTelemetry tracing
//! - Structured logging

use anyhow::Result;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize observability stack
pub fn init_observability() -> Result<()> {
    // Initialize structured logging
    init_logging()?;
    
    // Initialize metrics
    init_metrics()?;
    
    Ok(())
}

/// Initialize structured logging with tracing
fn init_logging() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();
    
    tracing::info!("Structured logging initialized");
    Ok(())
}

/// Initialize Prometheus metrics exporter
fn init_metrics() -> Result<()> {
    // Build Prometheus exporter
    let builder = PrometheusBuilder::new();
    
    // Install the exporter
    builder
        .install()
        .map_err(|e| anyhow::anyhow!("Failed to install Prometheus exporter: {}", e))?;
    
    // Initialize metric descriptions
    shared::api_clients::metrics::init_metrics();
    
    tracing::info!("Prometheus metrics initialized");
    Ok(())
}

/// Get the Prometheus metrics endpoint handler
pub async fn metrics_handler() -> String {
    // The metrics are automatically collected by the exporter
    // Return a simple message indicating metrics are available
    "Metrics are being collected. Use a Prometheus scraper to access them.".to_string()
}

/// Initialize OpenTelemetry tracing (optional, for distributed tracing)
pub fn init_opentelemetry(endpoint: &str) -> Result<()> {
    use opentelemetry::global;
    use opentelemetry_otlp::WithExportConfig;
    use tracing_opentelemetry::OpenTelemetryLayer;
    use tracing_subscriber::layer::SubscriberExt;
    
    // Create OTLP exporter
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint)
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .map_err(|e| anyhow::anyhow!("Failed to install OpenTelemetry pipeline: {}", e))?;
    
    // Set as global tracer
    global::set_tracer_provider(tracer.provider().ok_or_else(|| anyhow::anyhow!("No tracer provider"))?);
    
    tracing::info!("OpenTelemetry tracing initialized with endpoint: {}", endpoint);
    Ok(())
}

/// Initialize full observability stack with OpenTelemetry
pub fn init_observability_with_otel(otel_endpoint: Option<&str>) -> Result<()> {
    // Initialize structured logging
    init_logging()?;
    
    // Initialize metrics
    init_metrics()?;
    
    // Initialize OpenTelemetry if endpoint provided
    if let Some(endpoint) = otel_endpoint {
        init_opentelemetry(endpoint)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logging() {
        // Should not panic
        let result = init_logging();
        assert!(result.is_ok());
    }

    #[test]
    fn test_init_metrics() {
        // Should not panic
        let result = init_metrics();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_metrics_handler() {
        // Initialize metrics first
        let _ = init_metrics();
        
        // Get metrics output
        let metrics = metrics_handler().await;
        
        // Should return some metrics text
        assert!(!metrics.is_empty());
    }
}

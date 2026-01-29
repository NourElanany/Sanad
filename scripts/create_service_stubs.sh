#!/bin/bash

# Script to create basic service stubs for all remaining services
# This creates the minimal structure needed for Docker builds

SERVICES=(
    "stories-service:8083"
    "prayer-times-service:8084"
    "calendar-service:8085"
    "ai-service:8086"
    "search-service:8087"
    "audio-analysis-service:8088"
    "khatma-service:8089"
    "notification-service:8090"
)

for service_info in "${SERVICES[@]}"; do
    IFS=':' read -r service_name port <<< "$service_info"
    
    echo "Creating $service_name on port $port..."
    
    # Create Cargo.toml
    cat > "services/$service_name/Cargo.toml" << EOF
[package]
name = "$service_name"
version = "0.1.0"
edition = "2021"

[dependencies]
shared = { path = "../../shared" }
tokio = { workspace = true }
axum = { workspace = true }
tower = { workspace = true }
tower-http = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
sqlx = { workspace = true }
redis = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }
EOF

    # Create main.rs
    cat > "services/$service_name/src/main.rs" << EOF
use axum::{routing::get, Router, response::Json};
use shared::{AppConfig, ApiResponse};
use std::collections::HashMap;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();
    info!("Starting ${service_name//-/ } on port $port");

    let app = Router::new().route("/health", get(health_check));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:$port").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "$service_name".to_string());
    Json(ApiResponse::success(status))
}
EOF

    # Create Dockerfile
    cat > "services/$service_name/Dockerfile" << EOF
# Build stage
FROM rust:1.75-slim as builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared/ ./shared/
COPY services/$service_name/ ./services/$service_name/
RUN cargo build --release --bin $service_name

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*
RUN useradd -r -s /bin/false sanad
WORKDIR /app
COPY --from=builder /app/target/release/$service_name /app/$service_name
RUN chown -R sanad:sanad /app
USER sanad
EXPOSE $port
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 CMD curl -f http://localhost:$port/health || exit 1
CMD ["./$service_name"]
EOF

    echo "Created $service_name"
done

echo "All service stubs created successfully!"
# Multi-stage Dockerfile for API Integration Service
# Optimized for production deployment with minimal image size

# ============================================================================
# Stage 1: Builder - Compile the Rust application
# ============================================================================
FROM rust:1.75-slim-bookworm as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /app

# Copy workspace configuration
COPY Cargo.toml Cargo.lock ./

# Copy shared library (dependency)
COPY shared/ ./shared/

# Copy the api-integration-service
COPY services/api-integration-service/ ./services/api-integration-service/

# Build the application in release mode
WORKDIR /app/services/api-integration-service
RUN cargo build --release --bin api-integration-service

# ============================================================================
# Stage 2: Runtime - Create minimal runtime image
# ============================================================================
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user for running the service
RUN useradd -m -u 1000 -s /bin/bash sanad

# Create necessary directories
RUN mkdir -p /app/config /app/logs && \
    chown -R sanad:sanad /app

# Set working directory
WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /app/target/release/api-integration-service /app/api-integration-service

# Copy configuration files
COPY config/api_integration_config.yaml /app/config/
COPY config/default.toml /app/config/
COPY config/production.toml /app/config/

# Copy .env.example as reference (actual .env should be mounted or use env vars)
COPY .env.example /app/.env.example

# Set ownership
RUN chown -R sanad:sanad /app

# Switch to non-root user
USER sanad

# Expose the service port
EXPOSE 8080

# Health check endpoint
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Set environment variables with defaults
ENV RUST_LOG=info \
    SERVICE_PORT=8080 \
    SERVICE_HOST=0.0.0.0 \
    ENVIRONMENT=production

# Run the service
CMD ["/app/api-integration-service"]

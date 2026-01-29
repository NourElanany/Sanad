# Build stage
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY shared/ ./shared/
COPY services/cache-service/ ./services/cache-service/

# Build the cache service
RUN cargo build --release --bin cache-service

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false sanad

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/cache-service /app/cache-service

# Copy configuration files
COPY config/ /app/config/

# Change ownership
RUN chown -R sanad:sanad /app

# Switch to app user
USER sanad

# Expose port
EXPOSE 8091

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8091/health || exit 1

# Run the service
CMD ["./cache-service"]
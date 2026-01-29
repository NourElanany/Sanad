# Build stage
FROM rust:1.75-slim as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared/ ./shared/
COPY services/quran-service/ ./services/quran-service/

RUN cargo build --release --bin quran-service

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /bin/false sanad
WORKDIR /app
COPY --from=builder /app/target/release/quran-service /app/quran-service
RUN chown -R sanad:sanad /app
USER sanad

EXPOSE 8081
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8081/health || exit 1

CMD ["./quran-service"]
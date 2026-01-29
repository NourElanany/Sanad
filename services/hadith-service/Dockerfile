# Build stage
FROM rust:1.75-slim as builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared/ ./shared/
COPY services/hadith-service/ ./services/hadith-service/
RUN cargo build --release --bin hadith-service

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*
RUN useradd -r -s /bin/false sanad
WORKDIR /app
COPY --from=builder /app/target/release/hadith-service /app/hadith-service
RUN chown -R sanad:sanad /app
USER sanad
EXPOSE 8082
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 CMD curl -f http://localhost:8082/health || exit 1
CMD ["./hadith-service"]
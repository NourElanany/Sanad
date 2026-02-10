# Docker Deployment Guide - API Integration Service

This guide explains how to build and deploy the API Integration Service using Docker.

## Table of Contents

- [Quick Start](#quick-start)
- [Building the Image](#building-the-image)
- [Running the Container](#running-the-container)
- [Configuration](#configuration)
- [Environment Variables](#environment-variables)
- [Health Checks](#health-checks)
- [Production Deployment](#production-deployment)
- [Troubleshooting](#troubleshooting)

## Quick Start

```bash
# Build the Docker image
docker build -t sanad/api-integration-service:latest -f services/api-integration-service/Dockerfile .

# Run the container
docker run -d \
  --name api-integration-service \
  -p 8080:8080 \
  -e REDIS_URL=redis://redis:6379 \
  -e DATABASE_URL=postgresql://user:pass@postgres:5432/sanad \
  sanad/api-integration-service:latest
```

## Building the Image

### Standard Build

Build from the project root directory:

```bash
docker build -t sanad/api-integration-service:latest -f services/api-integration-service/Dockerfile .
```

### Build with Version Tag

```bash
docker build -t sanad/api-integration-service:v1.0.0 -f services/api-integration-service/Dockerfile .
```

### Build Arguments

The Dockerfile supports build-time optimizations:

```bash
# Build with specific Rust version
docker build \
  --build-arg RUST_VERSION=1.75 \
  -t sanad/api-integration-service:latest \
  -f services/api-integration-service/Dockerfile .
```

## Running the Container

### Basic Run

```bash
docker run -d \
  --name api-integration-service \
  -p 8080:8080 \
  sanad/api-integration-service:latest
```

### Run with Environment Variables

```bash
docker run -d \
  --name api-integration-service \
  -p 8080:8080 \
  -e RUST_LOG=debug \
  -e SERVICE_PORT=8080 \
  -e REDIS_URL=redis://redis:6379 \
  -e DATABASE_URL=postgresql://user:pass@postgres:5432/sanad \
  -e QURAN_COM_API_KEY=your_key_here \
  -e SUNNAH_COM_API_KEY=your_key_here \
  sanad/api-integration-service:latest
```

### Run with Configuration File

Mount a custom configuration file:

```bash
docker run -d \
  --name api-integration-service \
  -p 8080:8080 \
  -v $(pwd)/config/api_integration_config.yaml:/app/config/api_integration_config.yaml:ro \
  sanad/api-integration-service:latest
```

### Run with Environment File

```bash
docker run -d \
  --name api-integration-service \
  -p 8080:8080 \
  --env-file .env.production \
  sanad/api-integration-service:latest
```

## Configuration

### Configuration Files

The container includes these configuration files by default:

- `/app/config/api_integration_config.yaml` - Main service configuration
- `/app/config/default.toml` - Default settings
- `/app/config/production.toml` - Production overrides

### Mounting Custom Configuration

```bash
docker run -d \
  --name api-integration-service \
  -p 8080:8080 \
  -v $(pwd)/config:/app/config:ro \
  sanad/api-integration-service:latest
```

### Configuration Priority

1. Environment variables (highest priority)
2. Mounted configuration files
3. Built-in configuration files (lowest priority)

## Environment Variables

### Required Variables

```bash
# Redis connection
REDIS_URL=redis://redis:6379

# PostgreSQL connection
DATABASE_URL=postgresql://user:pass@postgres:5432/sanad
```

### API Keys (Required for full functionality)

```bash
# Quran APIs
QURAN_COM_API_KEY=your_key_here

# Hadith APIs
SUNNAH_COM_API_KEY=your_key_here

# Prayer Times APIs
ISLAMIC_FINDER_API_KEY=your_key_here

# AI/NLP APIs (Optional)
HUGGING_FACE_API_KEY=your_key_here
OPENAI_API_KEY=your_key_here
```

### Optional Variables

```bash
# Service configuration
SERVICE_PORT=8080
SERVICE_HOST=0.0.0.0
ENVIRONMENT=production

# Logging
RUST_LOG=info  # Options: trace, debug, info, warn, error

# Performance tuning
REDIS_POOL_SIZE=10
DATABASE_POOL_SIZE=20
```

## Health Checks

### Built-in Health Check

The container includes a health check that runs every 30 seconds:

```bash
# Check container health status
docker inspect --format='{{.State.Health.Status}}' api-integration-service
```

### Manual Health Check

```bash
# From host
curl http://localhost:8080/health

# From inside container
docker exec api-integration-service curl -f http://localhost:8080/health
```

### Health Check Response

```json
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime": "3600s",
  "apis": {
    "quran.com": "healthy",
    "sunnah.com": "healthy",
    "aladhan": "healthy"
  }
}
```

## Production Deployment

### Using Docker Compose

See `docker-compose.yml` in the project root for a complete setup with Redis and PostgreSQL.

```bash
docker-compose up -d api-integration-service
```

### Resource Limits

Set resource limits for production:

```bash
docker run -d \
  --name api-integration-service \
  -p 8080:8080 \
  --memory="512m" \
  --memory-swap="1g" \
  --cpus="1.0" \
  --restart=unless-stopped \
  sanad/api-integration-service:latest
```

### Logging

Configure logging driver:

```bash
docker run -d \
  --name api-integration-service \
  -p 8080:8080 \
  --log-driver=json-file \
  --log-opt max-size=10m \
  --log-opt max-file=3 \
  sanad/api-integration-service:latest
```

### Secrets Management

Use Docker secrets for sensitive data:

```bash
# Create secrets
echo "your_api_key" | docker secret create quran_com_api_key -
echo "your_api_key" | docker secret create sunnah_com_api_key -

# Run with secrets
docker service create \
  --name api-integration-service \
  --secret quran_com_api_key \
  --secret sunnah_com_api_key \
  -p 8080:8080 \
  sanad/api-integration-service:latest
```

### Network Configuration

Create a dedicated network:

```bash
# Create network
docker network create sanad-network

# Run with network
docker run -d \
  --name api-integration-service \
  --network sanad-network \
  -p 8080:8080 \
  sanad/api-integration-service:latest
```

## Troubleshooting

### View Logs

```bash
# View all logs
docker logs api-integration-service

# Follow logs
docker logs -f api-integration-service

# View last 100 lines
docker logs --tail 100 api-integration-service

# View logs with timestamps
docker logs -t api-integration-service
```

### Interactive Shell

```bash
# Access container shell
docker exec -it api-integration-service /bin/bash

# Check configuration
docker exec api-integration-service cat /app/config/api_integration_config.yaml

# Check environment variables
docker exec api-integration-service env
```

### Common Issues

#### Container Exits Immediately

Check logs for errors:
```bash
docker logs api-integration-service
```

Common causes:
- Missing required environment variables
- Invalid configuration
- Cannot connect to Redis/PostgreSQL

#### Health Check Failing

```bash
# Check if service is listening
docker exec api-integration-service netstat -tlnp

# Test health endpoint
docker exec api-integration-service curl -v http://localhost:8080/health
```

#### High Memory Usage

```bash
# Check memory usage
docker stats api-integration-service

# Restart with memory limit
docker update --memory="512m" api-integration-service
```

#### Cannot Connect to External APIs

```bash
# Test network connectivity
docker exec api-integration-service curl -v https://api.quran.com/api/v4/chapters

# Check DNS resolution
docker exec api-integration-service nslookup api.quran.com
```

### Performance Monitoring

```bash
# Real-time stats
docker stats api-integration-service

# Detailed inspection
docker inspect api-integration-service

# Check resource usage
docker exec api-integration-service ps aux
```

## Image Information

### Image Size

The multi-stage build produces a minimal image:

```bash
# Check image size
docker images sanad/api-integration-service

# Expected size: ~100-150 MB
```

### Image Layers

```bash
# View image layers
docker history sanad/api-integration-service:latest
```

### Security Scanning

```bash
# Scan for vulnerabilities (requires Docker Scout or similar)
docker scout cves sanad/api-integration-service:latest
```

## Best Practices

1. **Always use specific version tags** in production, not `latest`
2. **Set resource limits** to prevent resource exhaustion
3. **Use secrets management** for API keys, never environment variables in production
4. **Enable health checks** for automatic recovery
5. **Configure log rotation** to prevent disk space issues
6. **Use read-only file systems** where possible
7. **Run as non-root user** (already configured in Dockerfile)
8. **Keep images updated** with security patches

## Additional Resources

- [Main README](./README.md)
- [Deployment Guide](./docs/DEPLOYMENT_GUIDE.md)
- [Configuration Guide](../../config/CONFIGURATION_GUIDE.md)
- [API Documentation](./docs/API_DOCUMENTATION.md)

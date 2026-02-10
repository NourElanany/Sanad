# Docker Compose Setup for Sanad Project

## Overview

This document describes the complete Docker Compose setup for the Sanad project, including the newly integrated API Integration Service that orchestrates all external Islamic APIs.

## Architecture

The docker-compose.yml file orchestrates the following services:

### Infrastructure Services

1. **PostgreSQL** (Port 5432)
   - Main database for persistent data storage
   - Includes initialization scripts from `database/init/`
   - Health checks enabled

2. **Redis Standalone** (Port 6379)
   - Simple caching and session storage
   - Backward compatibility for services not using cluster
   - LRU eviction policy with 512MB memory limit

3. **Redis Cluster** (Ports 7001-7003)
   - High-performance distributed caching
   - 3-node cluster for scalability
   - Used by api-integration-service for advanced caching
   - LRU eviction policy with 1GB per node

4. **Qdrant Vector Database** (Ports 6333-6334)
   - Vector storage for semantic search
   - Used by AI and search services

### Application Services

#### Core Services

1. **API Gateway** (Port 8080)
   - Main entry point for all client requests
   - Routes requests to appropriate microservices
   - Handles authentication and authorization

2. **API Integration Service** (Port 8092) ⭐ NEW
   - **Purpose**: Orchestrates all external Islamic APIs
   - **Features**:
     - Multi-source API integration (Quran, Hadith, Prayer Times, etc.)
     - Intelligent caching with Redis/Redis Cluster
     - Rate limiting per API
     - Automatic fallback mechanisms
     - Health monitoring of external APIs
     - Structured logging and metrics
   - **Dependencies**: PostgreSQL, Redis, Redis Cluster
   - **Resource Limits**: 1GB RAM, 1 CPU

#### Domain Services

3. **Quran Service** (Port 8081)
4. **Hadith Service** (Port 8082)
5. **Stories Service** (Port 8083)
6. **Prayer Times Service** (Port 8084)
7. **Calendar Service** (Port 8085)
8. **AI Service** (Port 8086)
9. **Search Service** (Port 8087)
10. **Audio Analysis Service** (Port 8088)
11. **Khatma Service** (Port 8089)
12. **Notification Service** (Port 8090)
13. **Cache Service** (Port 8091)

## API Integration Service Configuration

### Environment Variables

The API Integration Service requires the following environment variables:

#### Required Configuration

```bash
# Service Configuration
SERVICE_PORT=8080
SERVICE_HOST=0.0.0.0
ENVIRONMENT=production
RUST_LOG=info

# Database Connections
REDIS_URL=redis://redis:6379
DATABASE_URL=postgresql://sanad_user:sanad_password@postgres:5432/sanad

# Redis Cluster (Advanced Caching)
REDIS_CLUSTER_ENABLED=true
REDIS_CLUSTER_NODES=redis://redis-node-1:7001,redis://redis-node-2:7002,redis://redis-node-3:7003
```

#### API Keys (Optional but Recommended)

Store these in a `.env` file at the project root:

```bash
# Quran APIs
QURAN_COM_API_KEY=your_quran_com_api_key

# Hadith APIs
SUNNAH_COM_API_KEY=your_sunnah_com_api_key

# Prayer Times & Calendar APIs
ISLAMIC_FINDER_API_KEY=your_islamic_finder_api_key
ALADHAN_API_KEY=your_aladhan_api_key

# AI/NLP APIs
HUGGING_FACE_API_KEY=your_hugging_face_api_key
OPENAI_API_KEY=your_openai_api_key  # Optional
```

#### Feature Flags

```bash
# Rate Limiting
RATE_LIMIT_ENABLED=true

# Caching
CACHE_ENABLED=true
CACHE_DEFAULT_TTL=3600

# Health Monitoring
HEALTH_CHECK_INTERVAL=300

# Observability
METRICS_ENABLED=true
TRACING_ENABLED=true
LOG_FORMAT=json
```

### Volumes

The API Integration Service uses the following volumes:

- **Configuration**: `./config:/app/config:ro` (read-only)
  - Mounts configuration files from the project's config directory
  
- **Logs**: `api-integration-logs:/app/logs`
  - Persistent storage for application logs
  - Named volume for easy access and backup

### Health Checks

The service includes comprehensive health checks:

```yaml
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 40s
```

### Resource Limits

Production-ready resource constraints:

```yaml
deploy:
  resources:
    limits:
      cpus: '1.0'
      memory: 1G
    reservations:
      cpus: '0.5'
      memory: 512M
```

## Getting Started

### Prerequisites

1. Docker Engine 20.10+
2. Docker Compose 2.0+
3. At least 8GB RAM available
4. 20GB free disk space

### Quick Start

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd sanad
   ```

2. **Create environment file**:
   ```bash
   cp .env.example .env
   # Edit .env and add your API keys
   ```

3. **Start all services**:
   ```bash
   docker-compose up -d
   ```

4. **Check service health**:
   ```bash
   docker-compose ps
   ```

5. **View logs**:
   ```bash
   # All services
   docker-compose logs -f
   
   # Specific service
   docker-compose logs -f api-integration-service
   ```

### Starting Individual Services

To start only specific services with their dependencies:

```bash
# Start only API Integration Service and its dependencies
docker-compose up -d api-integration-service

# Start only infrastructure
docker-compose up -d postgres redis redis-cluster-setup qdrant
```

## Service Dependencies

The API Integration Service depends on:

1. **PostgreSQL** - Must be healthy before starting
2. **Redis** - Must be healthy before starting
3. **Redis Cluster** - Must be successfully set up before starting

Dependency chain:
```
postgres (healthy) ─┐
redis (healthy) ────┼─> api-integration-service
redis-cluster-setup ┘
```

## Networking

All services communicate through the `sanad-network` bridge network:

- Internal DNS resolution by service name
- Isolated from host network by default
- Exposed ports mapped to host as needed

### Port Mapping

| Service | Internal Port | External Port |
|---------|--------------|---------------|
| Gateway | 8080 | 8080 |
| API Integration | 8080 | 8092 |
| Quran Service | 8081 | 8081 |
| Hadith Service | 8082 | 8082 |
| Stories Service | 8083 | 8083 |
| Prayer Times | 8084 | 8084 |
| Calendar Service | 8085 | 8085 |
| AI Service | 8086 | 8086 |
| Search Service | 8087 | 8087 |
| Audio Analysis | 8088 | 8088 |
| Khatma Service | 8089 | 8089 |
| Notification | 8090 | 8090 |
| Cache Service | 8091 | 8091 |
| PostgreSQL | 5432 | 5432 |
| Redis | 6379 | 6379 |
| Redis Cluster | 7001-7003 | 7001-7003 |
| Qdrant | 6333-6334 | 6333-6334 |

## Monitoring and Observability

### Health Endpoints

Check service health:

```bash
# API Integration Service
curl http://localhost:8092/health

# Gateway
curl http://localhost:8080/health
```

### Logs

View structured JSON logs:

```bash
# Real-time logs
docker-compose logs -f api-integration-service

# Last 100 lines
docker-compose logs --tail=100 api-integration-service

# Logs from specific time
docker-compose logs --since 2024-01-01T00:00:00 api-integration-service
```

### Metrics

The API Integration Service exposes Prometheus metrics at:
- `http://localhost:8092/metrics`

### Accessing Log Files

Logs are persisted in the `api-integration-logs` volume:

```bash
# Inspect volume
docker volume inspect sanad_api-integration-logs

# Access logs directly
docker run --rm -v sanad_api-integration-logs:/logs alpine ls -la /logs
```

## Troubleshooting

### Service Won't Start

1. **Check dependencies**:
   ```bash
   docker-compose ps postgres redis
   ```

2. **View startup logs**:
   ```bash
   docker-compose logs api-integration-service
   ```

3. **Verify environment variables**:
   ```bash
   docker-compose config | grep -A 20 api-integration-service
   ```

### Connection Issues

1. **Check network**:
   ```bash
   docker network inspect sanad_sanad-network
   ```

2. **Test connectivity**:
   ```bash
   docker-compose exec api-integration-service curl http://redis:6379
   docker-compose exec api-integration-service pg_isready -h postgres
   ```

### Performance Issues

1. **Check resource usage**:
   ```bash
   docker stats api-integration-service
   ```

2. **Adjust resource limits** in docker-compose.yml if needed

3. **Check Redis memory**:
   ```bash
   docker-compose exec redis redis-cli INFO memory
   ```

### API Key Issues

1. **Verify environment variables are set**:
   ```bash
   docker-compose exec api-integration-service env | grep API_KEY
   ```

2. **Check logs for authentication errors**:
   ```bash
   docker-compose logs api-integration-service | grep -i "auth\|key"
   ```

## Maintenance

### Updating Services

```bash
# Rebuild and restart a service
docker-compose up -d --build api-integration-service

# Rebuild all services
docker-compose build
docker-compose up -d
```

### Backup and Restore

#### Database Backup

```bash
# Backup PostgreSQL
docker-compose exec postgres pg_dump -U sanad_user sanad > backup.sql

# Restore PostgreSQL
docker-compose exec -T postgres psql -U sanad_user sanad < backup.sql
```

#### Redis Backup

```bash
# Trigger Redis save
docker-compose exec redis redis-cli BGSAVE

# Copy RDB file
docker cp sanad-redis:/data/dump.rdb ./redis-backup.rdb
```

### Cleaning Up

```bash
# Stop all services
docker-compose down

# Stop and remove volumes (WARNING: deletes all data)
docker-compose down -v

# Remove unused images
docker image prune -a
```

## Production Considerations

### Security

1. **Change default passwords** in production
2. **Use secrets management** for API keys (e.g., Docker Secrets, HashiCorp Vault)
3. **Enable TLS/SSL** for external connections
4. **Restrict network access** using firewall rules
5. **Regular security updates** for base images

### Scaling

To scale services horizontally:

```bash
# Scale API Integration Service to 3 instances
docker-compose up -d --scale api-integration-service=3
```

Note: Requires load balancer configuration for proper distribution.

### High Availability

For production HA setup:

1. Use external managed databases (AWS RDS, Azure Database)
2. Deploy Redis Cluster with replicas
3. Use container orchestration (Kubernetes, Docker Swarm)
4. Implement health checks and auto-restart policies
5. Set up monitoring and alerting

### Performance Tuning

1. **Adjust Redis memory limits** based on cache hit rates
2. **Tune PostgreSQL** connection pool sizes
3. **Configure rate limits** based on API quotas
4. **Monitor and adjust** resource limits per service
5. **Enable HTTP/2** for better performance

## Additional Resources

- [API Integration Service Documentation](services/api-integration-service/README.md)
- [Deployment Guide](services/api-integration-service/docs/DEPLOYMENT_GUIDE.md)
- [API Documentation](services/api-integration-service/docs/API_DOCUMENTATION.md)
- [Configuration Guide](config/CONFIGURATION_GUIDE.md)

## Support

For issues or questions:
1. Check the troubleshooting section above
2. Review service-specific documentation
3. Check logs for error messages
4. Open an issue in the project repository

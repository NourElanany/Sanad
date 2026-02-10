# Docker Quick Start Guide

## 🚀 Quick Commands

### Start Everything
```bash
docker-compose up -d
```

### Start Only API Integration Service
```bash
docker-compose up -d api-integration-service
```

### View Logs
```bash
# All services
docker-compose logs -f

# API Integration Service only
docker-compose logs -f api-integration-service

# Last 50 lines
docker-compose logs --tail=50 api-integration-service
```

### Check Status
```bash
# All services
docker-compose ps

# Health check
curl http://localhost:8092/health
```

### Stop Everything
```bash
docker-compose down
```

### Restart a Service
```bash
docker-compose restart api-integration-service
```

### Rebuild After Code Changes
```bash
docker-compose up -d --build api-integration-service
```

## 📋 Service Ports

| Service | Port |
|---------|------|
| Gateway | 8080 |
| **API Integration** | **8092** |
| Quran | 8081 |
| Hadith | 8082 |
| Stories | 8083 |
| Prayer Times | 8084 |
| Calendar | 8085 |
| AI | 8086 |
| Search | 8087 |
| Audio Analysis | 8088 |
| Khatma | 8089 |
| Notification | 8090 |
| Cache | 8091 |
| PostgreSQL | 5432 |
| Redis | 6379 |
| Qdrant | 6333 |

## 🔑 Environment Setup

1. Copy the example environment file:
```bash
cp .env.example .env
```

2. Edit `.env` and add your API keys:
```bash
QURAN_COM_API_KEY=your_key_here
SUNNAH_COM_API_KEY=your_key_here
ISLAMIC_FINDER_API_KEY=your_key_here
HUGGING_FACE_API_KEY=your_key_here
```

## 🧪 Testing the API Integration Service

### Health Check
```bash
curl http://localhost:8092/health
```

### Get Quran Text
```bash
curl http://localhost:8092/api/v1/quran/text?surah=1&ayah=1
```

### Get Prayer Times
```bash
curl "http://localhost:8092/api/v1/prayer-times?latitude=21.4225&longitude=39.8262&date=2024-01-15"
```

### Search Hadith
```bash
curl "http://localhost:8092/api/v1/hadith/search?query=prayer&limit=5"
```

### Get Metrics
```bash
curl http://localhost:8092/metrics
```

## 🐛 Troubleshooting

### Service won't start?
```bash
# Check logs
docker-compose logs api-integration-service

# Check dependencies
docker-compose ps postgres redis
```

### Connection refused?
```bash
# Verify service is running
docker-compose ps api-integration-service

# Check if port is accessible
curl -v http://localhost:8092/health
```

### Out of memory?
```bash
# Check resource usage
docker stats api-integration-service

# Increase memory limit in docker-compose.yml
```

### API keys not working?
```bash
# Verify environment variables
docker-compose exec api-integration-service env | grep API_KEY

# Restart after updating .env
docker-compose restart api-integration-service
```

## 🔧 Development Workflow

### 1. Make code changes
Edit files in `services/api-integration-service/src/`

### 2. Rebuild and restart
```bash
docker-compose up -d --build api-integration-service
```

### 3. Watch logs
```bash
docker-compose logs -f api-integration-service
```

### 4. Test changes
```bash
curl http://localhost:8092/health
```

## 📊 Monitoring

### View real-time logs
```bash
docker-compose logs -f api-integration-service
```

### Check resource usage
```bash
docker stats api-integration-service
```

### Access Redis
```bash
docker-compose exec redis redis-cli
```

### Access PostgreSQL
```bash
docker-compose exec postgres psql -U sanad_user -d sanad
```

## 🧹 Cleanup

### Stop services (keep data)
```bash
docker-compose down
```

### Stop and remove volumes (⚠️ deletes all data)
```bash
docker-compose down -v
```

### Remove unused images
```bash
docker image prune -a
```

## 📚 More Information

- Full documentation: [DOCKER_COMPOSE_SETUP.md](DOCKER_COMPOSE_SETUP.md)
- API Integration Service: [services/api-integration-service/README.md](services/api-integration-service/README.md)
- Configuration: [config/CONFIGURATION_GUIDE.md](config/CONFIGURATION_GUIDE.md)

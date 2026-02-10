# API Integration Service - Configuration Guide

This guide provides comprehensive documentation for configuring the API Integration Service.

## Table of Contents

1. [Overview](#overview)
2. [Configuration Files](#configuration-files)
3. [Configuration Structure](#configuration-structure)
4. [Service Configuration](#service-configuration)
5. [Database Configuration](#database-configuration)
6. [API Configurations](#api-configurations)
7. [Cache Strategies](#cache-strategies)
8. [Health Monitoring](#health-monitoring)
9. [Retry Configuration](#retry-configuration)
10. [Environment Variables](#environment-variables)
11. [API Keys Management](#api-keys-management)
12. [Rate Limiting](#rate-limiting)
13. [Production Deployment](#production-deployment)
14. [Troubleshooting](#troubleshooting)

## Overview

The API Integration Service uses a dual configuration approach:

1. **YAML Configuration File** (`config/api_integration_config.yaml`): Base configuration with all settings
2. **Environment Variables** (`.env` file): Override YAML settings and provide API keys

This approach allows:
- Version-controlled base configuration
- Environment-specific overrides
- Secure API key management
- Easy deployment across environments

## Configuration Files

### Primary Configuration File

**Location**: `config/api_integration_config.yaml`

This file contains all service configuration including:
- Service settings (name, port, host)
- Database connections (Redis, PostgreSQL)
- API endpoints and rate limits
- Cache strategies
- Health monitoring settings
- Retry policies

### Environment Variables File

**Location**: `.env` (copy from `.env.example`)

This file contains:
- API keys (never commit to version control)
- Environment-specific overrides
- Secrets and credentials

### Configuration Loading Priority

The service loads configuration in this order (later overrides earlier):

1. YAML configuration file
2. Environment variables
3. Command-line arguments (if applicable)

## Configuration Structure

### Complete YAML Structure

```yaml
service:
  name: string              # Service name
  port: integer             # HTTP port
  host: string              # Bind address

redis:
  url: string               # Redis connection URL
  pool_size: integer        # Connection pool size
  connection_timeout: string # Timeout (e.g., "5s")

postgres:
  url: string               # PostgreSQL connection URL
  pool_size: integer        # Connection pool size
  connection_timeout: string # Timeout (e.g., "10s")

apis:
  quran: [ApiConfig]        # Quran API configurations
  hadith: [ApiConfig]       # Hadith API configurations
  prayer_times: [ApiConfig] # Prayer times API configurations
  tafsir: [ApiConfig]       # Tafsir API configurations
  calendar: [ApiConfig]     # Calendar API configurations
  qibla: [ApiConfig]        # Qibla API configurations
  ai: [ApiConfig]           # AI/NLP API configurations

cache:
  strategies:
    quran_text: CacheStrategy
    hadith: CacheStrategy
    prayer_times: CacheStrategy
    tafsir: CacheStrategy
    calendar: CacheStrategy
    qibla: CacheStrategy
    ai_response: CacheStrategy

health_monitor:
  check_interval: string    # Check interval (e.g., "5m")
  unhealthy_threshold: integer # Failures before unhealthy
  recovery_threshold: integer  # Successes before healthy

retry:
  max_attempts: integer     # Maximum retry attempts
  initial_delay: string     # Initial delay (e.g., "1s")
  max_delay: string         # Maximum delay (e.g., "10s")
  multiplier: float         # Backoff multiplier
```

## Service Configuration

### Basic Settings

```yaml
service:
  name: api-integration-service  # Service identifier
  port: 8080                     # HTTP port to listen on
  host: 0.0.0.0                  # Bind to all interfaces
```

**Environment Variable Overrides**:
- `SERVICE_NAME`: Override service name
- `SERVICE_PORT`: Override port number
- `SERVICE_HOST`: Override bind address

### Examples

**Development**:
```yaml
service:
  name: api-integration-service-dev
  port: 8080
  host: localhost
```

**Production**:
```yaml
service:
  name: api-integration-service
  port: 8080
  host: 0.0.0.0
```

## Database Configuration

### Redis Configuration

Redis is used for:
- Response caching
- Rate limiting counters
- Health status tracking

```yaml
redis:
  url: redis://localhost:6379
  pool_size: 10
  connection_timeout: 5s
```

**Environment Variable Overrides**:
- `REDIS_URL`: Redis connection string
- `REDIS_POOL_SIZE`: Connection pool size
- `REDIS_CONNECTION_TIMEOUT`: Connection timeout

**Production Considerations**:
- Use Redis Cluster for high availability
- Enable TLS: `rediss://host:6380`
- Use authentication: `redis://:password@host:6379`
- Consider Redis Sentinel for failover

### PostgreSQL Configuration

PostgreSQL is used for:
- Persistent storage
- API usage logs
- Health metrics history

```yaml
postgres:
  url: postgresql://user:pass@localhost:5432/sanad
  pool_size: 20
  connection_timeout: 10s
```

**Environment Variable Overrides**:
- `POSTGRES_URL` or `DATABASE_URL`: PostgreSQL connection string
- `POSTGRES_POOL_SIZE`: Connection pool size
- `POSTGRES_CONNECTION_TIMEOUT`: Connection timeout

**Production Considerations**:
- Use strong passwords
- Enable SSL: `postgresql://user:pass@host:5432/db?sslmode=require`
- Use connection pooling (PgBouncer)
- Configure read replicas for scaling

## API Configurations

### API Configuration Structure

Each API has the following configuration:

```yaml
- name: string              # API identifier (e.g., "quran.com")
  base_url: string          # API base URL
  priority: integer         # Priority (1=primary, 2=secondary, etc.)
  requires_key: boolean     # Whether API key is required
  rate_limit:
    requests_per_minute: integer
    requests_per_hour: integer
    requests_per_day: integer
  timeout: string           # Request timeout (e.g., "10s")
```

### Quran APIs

**Primary Sources** (Official and Verified):

1. **Quran.com / Quran Foundation API**
   ```yaml
   - name: quran.com
     base_url: https://api.quran.com/api/v4
     priority: 1
     requires_key: false
     rate_limit:
       requests_per_minute: 60
       requests_per_hour: 1000
       requests_per_day: 10000
     timeout: 10s
   ```
   - **Status**: ✅ Official - Quran Foundation
   - **Features**: Quran text, translations, recitations, tafsir
   - **Authentication**: Optional (higher limits with key)

2. **Tanzil.net**
   ```yaml
   - name: tanzil
     base_url: https://api.tanzil.net
     priority: 3
     requires_key: false
     rate_limit:
       requests_per_minute: 30
       requests_per_hour: 500
       requests_per_day: 5000
     timeout: 10s
   ```
   - **Status**: ✅ Official - Verified Quran text
   - **Features**: Highly accurate Quran text in Unicode

3. **AlQuran Cloud API**
   ```yaml
   - name: alquran.cloud
     base_url: https://api.alquran.cloud/v1
     priority: 2
     requires_key: false
     rate_limit:
       requests_per_minute: 30
       requests_per_hour: 500
       requests_per_day: 5000
     timeout: 10s
   ```
   - **Status**: ✅ Verified - Community trusted
   - **Features**: Quran text, translations, audio

4. **EveryAyah.com** (Audio)
   ```yaml
   - name: everyayah
     base_url: https://everyayah.com
     priority: 1
     requires_key: false
     rate_limit:
       requests_per_minute: 60
       requests_per_hour: 1000
       requests_per_day: 10000
     timeout: 15s
   ```
   - **Status**: ✅ Verified - Audio recitations
   - **Features**: Verse-by-verse audio from authentic reciters

### Hadith APIs

**Primary Sources** (Official and Verified):

1. **Sunnah.com**
   ```yaml
   - name: sunnah.com
     base_url: https://api.sunnah.com/v1
     priority: 1
     requires_key: true
     rate_limit:
       requests_per_minute: 30
       requests_per_hour: 500
       requests_per_day: 5000
     timeout: 15s
   ```
   - **Status**: ✅ Official - Authenticated chains
   - **Features**: Multiple hadith collections (Bukhari, Muslim, etc.)
   - **API Key**: Required - Get at https://sunnah.com/

2. **IslamHouse HadeethEnc.com**
   ```yaml
   - name: islamhouse
     base_url: https://api.islamhouse.com
     priority: 2
     requires_key: false
     rate_limit:
       requests_per_minute: 30
       requests_per_hour: 500
       requests_per_day: 5000
     timeout: 15s
   ```
   - **Status**: ✅ Official - Officially supervised
   - **Features**: Verified hadith translations

### Prayer Times APIs

1. **AlAdhan API**
   ```yaml
   - name: aladhan
     base_url: https://api.aladhan.com/v1
     priority: 1
     requires_key: false
     rate_limit:
       requests_per_minute: 60
       requests_per_hour: 1000
       requests_per_day: 10000
     timeout: 5s
   ```
   - **Status**: ✅ Official - Islamic Network
   - **Features**: Prayer times, Qibla, Hijri calendar
   - **Calculation Methods**: 22+ official methods

2. **Islamic Finder**
   ```yaml
   - name: islamic_finder
     base_url: https://api.islamicfinder.org
     priority: 2
     requires_key: true
     rate_limit:
       requests_per_minute: 30
       requests_per_hour: 500
       requests_per_day: 5000
     timeout: 5s
   ```
   - **Status**: ✅ Verified - Widely trusted
   - **Features**: Prayer times, Qibla, calendar

### Tafsir APIs

```yaml
tafsir:
  - name: quran.com
    base_url: https://api.quran.com/api/v4
    priority: 1
    requires_key: false
    rate_limit:
      requests_per_minute: 60
      requests_per_hour: 1000
      requests_per_day: 10000
    timeout: 10s
```
- **Status**: ✅ Official - Quran Foundation
- **Features**: Multiple tafsir sources by recognized scholars

### Calendar APIs

Same as Prayer Times APIs (AlAdhan and Islamic Finder) - they provide calendar functionality.

### Qibla APIs

Same as Prayer Times APIs (AlAdhan and Islamic Finder) - they provide Qibla direction.

### AI/NLP APIs

```yaml
ai:
  - name: huggingface
    base_url: https://api-inference.huggingface.co
    priority: 1
    requires_key: true
    rate_limit:
      requests_per_minute: 30
      requests_per_hour: 500
      requests_per_day: 5000
    timeout: 30s
```
- **Status**: ✅ Verified - For technical processing only
- **Use Case**: Arabic NLP, embeddings, semantic search
- **Important**: NOT used for Islamic rulings or fatwas
- **API Key**: Required - Get at https://huggingface.co/settings/tokens

## Cache Strategies

### Cache Strategy Structure

```yaml
cache_strategy:
  ttl: string              # Time to live (e.g., "30d", "24h", "60m")
  allow_stale: boolean     # Use expired cache as fallback
  stale_ttl: string        # How long to keep stale cache (optional)
```

### Predefined Strategies

#### Static Data (Long TTL)

**Quran Text**:
```yaml
quran_text:
  ttl: 30d
  allow_stale: true
  stale_ttl: 90d
```
- Quran text never changes
- Long TTL reduces API calls
- Stale cache as ultimate fallback

**Hadith**:
```yaml
hadith:
  ttl: 30d
  allow_stale: true
  stale_ttl: 90d
```
- Hadith content is static
- Similar strategy to Quran text

**Tafsir**:
```yaml
tafsir:
  ttl: 30d
  allow_stale: true
  stale_ttl: 90d
```
- Tafsir content rarely changes
- Long TTL appropriate

**Qibla**:
```yaml
qibla:
  ttl: 30d
  allow_stale: true
  stale_ttl: 90d
```
- Qibla direction is static per location
- Long TTL per location

#### Semi-Static Data (Medium TTL)

**Calendar**:
```yaml
calendar:
  ttl: 7d
  allow_stale: true
  stale_ttl: 30d
```
- Hijri dates may have minor adjustments
- Weekly refresh is sufficient

#### Dynamic Data (Short TTL)

**Prayer Times**:
```yaml
prayer_times:
  ttl: 1d
  allow_stale: true
  stale_ttl: 7d
```
- Prayer times change daily
- Daily refresh required
- Stale cache for emergencies

**AI Response**:
```yaml
ai_response:
  ttl: 1h
  allow_stale: false
```
- AI responses may improve over time
- Short TTL for freshness
- No stale cache (responses should be current)

### Cache Key Patterns

The service uses deterministic cache keys based on request parameters:

- **Quran**: `quran:{surah}:{ayah}:{translation}`
- **Hadith**: `hadith:{collection}:{book}:{number}`
- **Prayer Times**: `prayer:{lat}:{lon}:{date}:{method}:{madhab}`
- **Tafsir**: `tafsir:{surah}:{ayah}:{tafsir_id}:{language}`
- **Calendar**: `calendar:{date}:{direction}`
- **Qibla**: `qibla:{lat}:{lon}`
- **AI**: `ai:{hash(query)}:{language}`

## Health Monitoring

### Configuration

```yaml
health_monitor:
  check_interval: 5m        # How often to check API health
  unhealthy_threshold: 3    # Consecutive failures before unhealthy
  recovery_threshold: 2     # Consecutive successes before healthy
```

### How It Works

1. **Periodic Checks**: Every 5 minutes, the service checks each API
2. **Failure Tracking**: Consecutive failures are counted
3. **Unhealthy Marking**: After 3 consecutive failures, API is marked unhealthy
4. **Automatic Fallback**: Unhealthy APIs are bypassed automatically
5. **Recovery Detection**: After 2 consecutive successes, API is marked healthy
6. **Metrics**: Health status, response times, and success rates are tracked

### Health Check Endpoint

**Endpoint**: `GET /health`

**Response**:
```json
{
  "status": "healthy",
  "apis": {
    "quran.com": {
      "healthy": true,
      "last_check": "2024-01-15T10:30:00Z",
      "success_rate": 0.99,
      "avg_response_time_ms": 150
    },
    "sunnah.com": {
      "healthy": false,
      "last_check": "2024-01-15T10:30:00Z",
      "consecutive_failures": 5,
      "last_error": "Connection timeout"
    }
  }
}
```

## Retry Configuration

### Configuration

```yaml
retry:
  max_attempts: 3           # Maximum retry attempts
  initial_delay: 1s         # Initial delay before first retry
  max_delay: 10s            # Maximum delay between retries
  multiplier: 2.0           # Exponential backoff multiplier
```

### Retry Strategy

**Exponential Backoff**:
- Attempt 1: Immediate
- Attempt 2: Wait 1s (initial_delay)
- Attempt 3: Wait 2s (initial_delay × multiplier)
- Attempt 4: Wait 4s (initial_delay × multiplier²)
- Max delay: 10s (capped at max_delay)

**Retry Conditions**:
- Network errors (connection timeout, DNS failure)
- Server errors (500, 502, 503, 504)
- Timeout errors

**No Retry**:
- Authentication errors (invalid API key)
- Validation errors (invalid request)
- Rate limit errors (use fallback instead)

## Environment Variables

### Complete List

See `.env.example` for the complete list with descriptions.

### Critical Variables

**Required**:
- `SUNNAH_COM_API_KEY`: Required for hadith functionality
- `HUGGING_FACE_API_KEY`: Required for AI features

**Recommended**:
- `REDIS_URL`: Redis connection
- `POSTGRES_URL` or `DATABASE_URL`: PostgreSQL connection
- `LOG_LEVEL`: Logging verbosity

**Optional**:
- `QURAN_COM_API_KEY`: Higher rate limits
- `ISLAMIC_FINDER_API_KEY`: Backup for prayer times
- `OPENAI_API_KEY`: Alternative AI provider

### Environment-Specific Settings

**Development**:
```bash
ENVIRONMENT=development
LOG_LEVEL=debug
JSON_LOGGING=false
```

**Staging**:
```bash
ENVIRONMENT=staging
LOG_LEVEL=info
JSON_LOGGING=true
```

**Production**:
```bash
ENVIRONMENT=production
LOG_LEVEL=warn
JSON_LOGGING=true
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector:4317
SENTRY_DSN=https://your-dsn@sentry.io/project
```

## API Keys Management

### Obtaining API Keys

1. **Sunnah.com** (Required):
   - Visit: https://sunnah.com/
   - Contact for API access
   - Store in: `SUNNAH_COM_API_KEY`

2. **Hugging Face** (Required):
   - Visit: https://huggingface.co/settings/tokens
   - Create new token
   - Store in: `HUGGING_FACE_API_KEY`

3. **Islamic Finder** (Optional):
   - Visit: https://www.islamicfinder.org/
   - Request API access
   - Store in: `ISLAMIC_FINDER_API_KEY`

4. **OpenAI** (Optional):
   - Visit: https://platform.openai.com/api-keys
   - Create new key
   - Store in: `OPENAI_API_KEY`

### Security Best Practices

1. **Never Commit Keys**:
   - Add `.env` to `.gitignore`
   - Use `.env.example` as template only

2. **Use Secrets Manager in Production**:
   - AWS Secrets Manager
   - HashiCorp Vault
   - Azure Key Vault
   - Google Secret Manager

3. **Rotate Keys Regularly**:
   - Set rotation schedule (e.g., every 90 days)
   - Service supports hot-reload (no restart needed)

4. **Use Different Keys Per Environment**:
   - Development keys
   - Staging keys
   - Production keys

5. **Monitor Key Usage**:
   - Track API calls per key
   - Alert on unusual patterns
   - Monitor rate limit usage

### Key Injection

The service automatically injects API keys based on the API's requirements:

- **Header**: `X-API-Key: your_key`
- **Query Parameter**: `?api_key=your_key`
- **Bearer Token**: `Authorization: Bearer your_key`
- **Basic Auth**: `Authorization: Basic base64(username:key)`

## Rate Limiting

### How It Works

1. **Per-API Limits**: Each API has its own rate limits
2. **Time Windows**: Minute, hour, and day windows
3. **Redis Counters**: Counters stored in Redis with TTL
4. **Automatic Enforcement**: Requests blocked when limit reached
5. **Fallback**: Alternative APIs used when primary is rate-limited

### Configuring Rate Limits

**In YAML**:
```yaml
rate_limit:
  requests_per_minute: 60
  requests_per_hour: 1000
  requests_per_day: 10000
```

**Via Environment Variables**:
```bash
QURAN_COM_RATE_LIMIT_PER_MINUTE=100
QURAN_COM_RATE_LIMIT_PER_HOUR=2000
QURAN_COM_RATE_LIMIT_PER_DAY=20000
```

### Monitoring Rate Limits

**Metrics Endpoint**: `GET /metrics`

```
api_requests_total{api="quran.com"} 1234
api_rate_limit_exceeded_total{api="quran.com"} 5
api_rate_limit_usage_ratio{api="quran.com",window="minute"} 0.75
```

### Best Practices

1. **Conservative Limits**: Start with lower limits than API allows
2. **Monitor Usage**: Track actual usage vs limits
3. **Gradual Increase**: Increase limits gradually if needed
4. **Respect Terms**: Never exceed API terms of service
5. **Use Caching**: Reduce API calls through aggressive caching

## Production Deployment

### Pre-Deployment Checklist

- [ ] All API keys obtained and stored securely
- [ ] Configuration file reviewed and validated
- [ ] Environment variables set correctly
- [ ] Redis cluster configured and tested
- [ ] PostgreSQL configured with SSL
- [ ] Monitoring and alerting configured
- [ ] Log aggregation set up
- [ ] Backup and disaster recovery plan in place

### Docker Deployment

**Dockerfile**:
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin api-integration-service

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api-integration-service /usr/local/bin/
COPY config/ /etc/sanad/config/
EXPOSE 8080
CMD ["api-integration-service"]
```

**docker-compose.yml**:
```yaml
version: '3.8'
services:
  api-integration:
    build: .
    ports:
      - "8080:8080"
    environment:
      - REDIS_URL=redis://redis:6379
      - POSTGRES_URL=postgresql://postgres:5432/sanad
    env_file:
      - .env
    depends_on:
      - redis
      - postgres
  
  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
  
  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=sanad
      - POSTGRES_PASSWORD=secure_password
    ports:
      - "5432:5432"
```

### Kubernetes Deployment

**ConfigMap**:
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: api-integration-config
data:
  api_integration_config.yaml: |
    # Your YAML config here
```

**Secret**:
```yaml
apiVersion: v1
kind: Secret
metadata:
  name: api-keys
type: Opaque
stringData:
  SUNNAH_COM_API_KEY: your_key_here
  HUGGING_FACE_API_KEY: your_key_here
```

**Deployment**:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-integration-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: api-integration
  template:
    metadata:
      labels:
        app: api-integration
    spec:
      containers:
      - name: api-integration
        image: sanad/api-integration-service:latest
        ports:
        - containerPort: 8080
        envFrom:
        - secretRef:
            name: api-keys
        volumeMounts:
        - name: config
          mountPath: /etc/sanad/config
      volumes:
      - name: config
        configMap:
          name: api-integration-config
```

### Monitoring Setup

**Prometheus**:
```yaml
scrape_configs:
  - job_name: 'api-integration'
    static_configs:
      - targets: ['api-integration:8080']
    metrics_path: '/metrics'
```

**Grafana Dashboard**:
- API request rates
- Cache hit/miss ratios
- Error rates by API
- Response times
- Rate limit usage

**Alerts**:
- High error rate (> 5%)
- API unhealthy for > 10 minutes
- Rate limit approaching (> 80%)
- Cache miss rate high (> 50%)

## Troubleshooting

### Common Issues

#### 1. API Key Not Working

**Symptoms**:
- 401 Unauthorized errors
- "API key not found" errors

**Solutions**:
- Verify key is set in `.env` file
- Check key format (no extra spaces)
- Verify key is valid on API provider's website
- Check if key requires specific permissions

#### 2. Rate Limit Exceeded

**Symptoms**:
- 429 Too Many Requests errors
- "Rate limit exceeded" messages

**Solutions**:
- Check rate limit configuration
- Verify actual usage vs configured limits
- Enable caching to reduce API calls
- Use fallback APIs
- Contact API provider for higher limits

#### 3. Connection Timeouts

**Symptoms**:
- "Connection timeout" errors
- Slow response times

**Solutions**:
- Check network connectivity
- Verify API endpoint URLs
- Increase timeout values
- Check firewall rules
- Verify DNS resolution

#### 4. Cache Not Working

**Symptoms**:
- High API call volume
- Slow response times
- Redis connection errors

**Solutions**:
- Verify Redis is running
- Check Redis connection URL
- Verify Redis authentication
- Check Redis memory usage
- Review cache TTL settings

#### 5. Health Checks Failing

**Symptoms**:
- APIs marked as unhealthy
- Constant fallback usage

**Solutions**:
- Check API status pages
- Verify API keys are valid
- Review health check thresholds
- Check network connectivity
- Review API rate limits

### Debug Mode

Enable debug logging:

```bash
LOG_LEVEL=debug
```

This will log:
- All API requests and responses
- Cache hits and misses
- Rate limit checks
- Health check results
- Fallback decisions

### Support

For issues not covered here:

1. Check service logs
2. Review metrics dashboard
3. Check API provider status pages
4. Consult API documentation
5. Contact support team

## Appendix

### Time Duration Format

The service uses human-readable duration strings:

- `s`: seconds (e.g., `30s`)
- `m`: minutes (e.g., `5m`)
- `h`: hours (e.g., `2h`)
- `d`: days (e.g., `30d`)

Examples:
- `1s` = 1 second
- `30s` = 30 seconds
- `5m` = 5 minutes
- `1h` = 1 hour
- `24h` = 24 hours
- `7d` = 7 days
- `30d` = 30 days

### API Priority System

Priority determines fallback order:

- `1`: Primary API (tried first)
- `2`: Secondary API (tried if primary fails)
- `3`: Tertiary API (tried if secondary fails)
- etc.

Lower numbers = higher priority.

### Cache TTL Recommendations

| Data Type | Volatility | Recommended TTL | Stale TTL |
|-----------|-----------|-----------------|-----------|
| Quran Text | Static | 30d | 90d |
| Hadith | Static | 30d | 90d |
| Tafsir | Static | 30d | 90d |
| Prayer Times | Daily | 1d | 7d |
| Calendar | Weekly | 7d | 30d |
| Qibla | Static per location | 30d | 90d |
| AI Response | Dynamic | 1h | None |

### Rate Limit Recommendations

| API | Free Tier | Recommended | Notes |
|-----|-----------|-------------|-------|
| Quran.com | Unlimited | 60/min | Conservative |
| AlQuran Cloud | Unlimited | 30/min | Conservative |
| Sunnah.com | 500/day | 30/min | Requires key |
| AlAdhan | Unlimited | 60/min | Free and open |
| Hugging Face | 1000/day | 30/min | Free tier |

### Useful Commands

**Test configuration**:
```bash
cargo test --package api-integration-service --lib config::tests
```

**Validate YAML**:
```bash
yamllint config/api_integration_config.yaml
```

**Check environment variables**:
```bash
env | grep -E '(SERVICE|REDIS|POSTGRES|API_KEY)'
```

**Test Redis connection**:
```bash
redis-cli -u $REDIS_URL ping
```

**Test PostgreSQL connection**:
```bash
psql $POSTGRES_URL -c "SELECT 1"
```

---

**Last Updated**: 2024-01-15
**Version**: 1.0.0

# Configuration Directory

This directory contains configuration files for the API Integration Service.

## Files

### `api_integration_config.yaml`

The main configuration file containing all service settings:

- **Service Configuration**: Name, port, host
- **Database Configuration**: Redis and PostgreSQL connections
- **API Configurations**: All external API endpoints and settings
- **Cache Strategies**: TTL and stale cache policies
- **Health Monitoring**: Health check intervals and thresholds
- **Retry Configuration**: Retry policies and backoff settings

**Location**: `config/api_integration_config.yaml`

**Usage**:
```bash
# Service automatically loads from this location
./api-integration-service

# Or specify custom location
CONFIG_PATH=/path/to/config.yaml ./api-integration-service
```

### `CONFIGURATION_GUIDE.md`

Comprehensive documentation for all configuration options:

- Complete configuration reference
- API-specific settings and requirements
- Cache strategy explanations
- Environment variable overrides
- Production deployment guide
- Troubleshooting tips

**Read this first** before modifying configuration.

## Quick Start

### 1. Copy Environment Template

```bash
# From project root
cp .env.example .env
```

### 2. Add API Keys

Edit `.env` and add your API keys:

```bash
# Required
SUNNAH_COM_API_KEY=your_key_here
HUGGING_FACE_API_KEY=your_key_here

# Optional but recommended
ISLAMIC_FINDER_API_KEY=your_key_here
```

### 3. Review Configuration

Review `api_integration_config.yaml` and adjust if needed:

- Rate limits (if you have higher quotas)
- Cache TTL values (based on your needs)
- Health check intervals
- Retry settings

### 4. Test Configuration

```bash
# Run configuration tests
cargo test --package api-integration-service --lib config::tests

# Start service
cargo run --bin api-integration-service
```

## Configuration Priority

Settings are loaded in this order (later overrides earlier):

1. **YAML file** (`api_integration_config.yaml`)
2. **Environment variables** (`.env` file)
3. **System environment** (exported variables)

Example:
```yaml
# In YAML
service:
  port: 8080

# Override with environment variable
SERVICE_PORT=9090

# Result: Service runs on port 9090
```

## Environment-Specific Configuration

### Development

```bash
# .env.development
ENVIRONMENT=development
LOG_LEVEL=debug
REDIS_URL=redis://localhost:6379
POSTGRES_URL=postgresql://localhost:5432/sanad_dev
```

### Staging

```bash
# .env.staging
ENVIRONMENT=staging
LOG_LEVEL=info
REDIS_URL=redis://staging-redis:6379
POSTGRES_URL=postgresql://staging-db:5432/sanad_staging
```

### Production

```bash
# .env.production
ENVIRONMENT=production
LOG_LEVEL=warn
REDIS_URL=rediss://prod-redis:6380  # TLS enabled
POSTGRES_URL=postgresql://prod-db:5432/sanad?sslmode=require
JSON_LOGGING=true
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector:4317
```

## API Configuration

### Adding a New API

1. Add to appropriate category in `api_integration_config.yaml`:

```yaml
apis:
  quran:
    - name: new-quran-api
      base_url: https://api.example.com
      priority: 4  # Lower priority than existing
      requires_key: false
      rate_limit:
        requests_per_minute: 30
        requests_per_hour: 500
        requests_per_day: 5000
      timeout: 10s
```

2. If API requires a key, add to `.env`:

```bash
NEW_QURAN_API_KEY=your_key_here
```

3. Implement the API client in code (see developer guide)

### Modifying Rate Limits

**Option 1: Edit YAML** (permanent change):
```yaml
rate_limit:
  requests_per_minute: 100  # Increased from 60
  requests_per_hour: 2000   # Increased from 1000
  requests_per_day: 20000   # Increased from 10000
```

**Option 2: Environment Variable** (temporary override):
```bash
QURAN_COM_RATE_LIMIT_PER_MINUTE=100
QURAN_COM_RATE_LIMIT_PER_HOUR=2000
QURAN_COM_RATE_LIMIT_PER_DAY=20000
```

## Cache Configuration

### Adjusting Cache TTL

Edit cache strategies in `api_integration_config.yaml`:

```yaml
cache:
  strategies:
    quran_text:
      ttl: 60d        # Increased from 30d
      allow_stale: true
      stale_ttl: 180d # Increased from 90d
```

### Disabling Stale Cache

```yaml
cache:
  strategies:
    prayer_times:
      ttl: 1d
      allow_stale: false  # Disable stale cache
```

## Health Monitoring

### Adjusting Thresholds

```yaml
health_monitor:
  check_interval: 3m      # More frequent checks
  unhealthy_threshold: 5  # More tolerant of failures
  recovery_threshold: 1   # Faster recovery
```

### Disabling Health Checks

Not recommended, but possible via environment:

```bash
HEALTH_CHECK_INTERVAL=0  # Disables health checks
```

## Retry Configuration

### Adjusting Retry Behavior

```yaml
retry:
  max_attempts: 5         # More retries
  initial_delay: 500ms    # Faster initial retry
  max_delay: 30s          # Longer max delay
  multiplier: 1.5         # Slower backoff
```

## Validation

The service validates configuration on startup:

- Required fields are present
- Values are in valid ranges
- URLs are properly formatted
- Rate limits are positive
- TTL values are valid durations

**Validation errors** will prevent service startup with clear error messages.

## Security

### API Keys

- **Never commit** `.env` file to version control
- Use `.env.example` as template only
- Store production keys in secrets manager
- Rotate keys regularly (every 90 days)
- Use different keys per environment

### Database Credentials

- Use strong passwords
- Enable SSL/TLS in production
- Use connection pooling
- Limit database user permissions
- Rotate credentials regularly

### Configuration Files

- Protect `api_integration_config.yaml` with appropriate file permissions
- Don't include sensitive data in YAML (use environment variables)
- Review configuration changes in code review
- Audit configuration changes

## Monitoring

### Configuration Metrics

The service exposes metrics about configuration:

```
config_reload_total{status="success"} 5
config_reload_total{status="failure"} 0
config_validation_errors_total 0
```

### Configuration Logs

Configuration loading is logged:

```
INFO  Loading configuration from config/api_integration_config.yaml
INFO  Applied 3 environment variable overrides
INFO  Configuration validated successfully
INFO  Loaded 15 API configurations
INFO  Loaded 7 cache strategies
```

## Troubleshooting

### Configuration Not Loading

**Check file location**:
```bash
ls -la config/api_integration_config.yaml
```

**Check file permissions**:
```bash
chmod 644 config/api_integration_config.yaml
```

**Check YAML syntax**:
```bash
yamllint config/api_integration_config.yaml
```

### Environment Variables Not Applied

**Check variable names**:
```bash
env | grep SERVICE_
```

**Check .env file**:
```bash
cat .env | grep -v '^#' | grep -v '^$'
```

**Load .env manually**:
```bash
export $(cat .env | xargs)
```

### Validation Errors

**Common issues**:
- Empty required fields
- Invalid duration format (use `30d`, not `30 days`)
- Zero or negative values
- Missing API keys for required APIs

**Check logs** for specific validation error messages.

## Examples

### Minimal Configuration

```yaml
service:
  name: api-integration-service
  port: 8080
  host: 0.0.0.0

redis:
  url: redis://localhost:6379
  pool_size: 10
  connection_timeout: 5s

postgres:
  url: postgresql://localhost:5432/sanad
  pool_size: 20
  connection_timeout: 10s

apis:
  quran:
    - name: quran.com
      base_url: https://api.quran.com/api/v4
      priority: 1
      requires_key: false
      rate_limit:
        requests_per_minute: 60
        requests_per_hour: 1000
        requests_per_day: 10000
      timeout: 10s
  hadith: []
  prayer_times: []
  tafsir: []
  calendar: []
  qibla: []
  ai: []

cache:
  strategies:
    quran_text:
      ttl: 30d
      allow_stale: true
      stale_ttl: 90d
    hadith:
      ttl: 30d
      allow_stale: true
      stale_ttl: 90d
    prayer_times:
      ttl: 1d
      allow_stale: true
      stale_ttl: 7d
    tafsir:
      ttl: 30d
      allow_stale: true
      stale_ttl: 90d
    calendar:
      ttl: 7d
      allow_stale: true
      stale_ttl: 30d
    qibla:
      ttl: 30d
      allow_stale: true
      stale_ttl: 90d
    ai_response:
      ttl: 1h
      allow_stale: false

health_monitor:
  check_interval: 5m
  unhealthy_threshold: 3
  recovery_threshold: 2

retry:
  max_attempts: 3
  initial_delay: 1s
  max_delay: 10s
  multiplier: 2.0
```

### High-Availability Configuration

```yaml
# Optimized for high availability and performance

service:
  name: api-integration-service
  port: 8080
  host: 0.0.0.0

redis:
  url: redis://redis-cluster:6379
  pool_size: 50  # Larger pool for high concurrency
  connection_timeout: 3s

postgres:
  url: postgresql://db-primary:5432/sanad?sslmode=require
  pool_size: 100  # Larger pool
  connection_timeout: 5s

# Multiple APIs per category for redundancy
apis:
  quran:
    - name: quran.com
      base_url: https://api.quran.com/api/v4
      priority: 1
      requires_key: true  # Use authenticated for higher limits
      rate_limit:
        requests_per_minute: 120
        requests_per_hour: 5000
        requests_per_day: 50000
      timeout: 5s
    
    - name: alquran.cloud
      base_url: https://api.alquran.cloud/v1
      priority: 2
      requires_key: false
      rate_limit:
        requests_per_minute: 60
        requests_per_hour: 1000
        requests_per_day: 10000
      timeout: 5s
    
    - name: tanzil
      base_url: https://api.tanzil.net
      priority: 3
      requires_key: false
      rate_limit:
        requests_per_minute: 60
        requests_per_hour: 1000
        requests_per_day: 10000
      timeout: 5s

# Aggressive caching
cache:
  strategies:
    quran_text:
      ttl: 90d  # Longer TTL
      allow_stale: true
      stale_ttl: 365d  # Keep stale for 1 year
    
    prayer_times:
      ttl: 1d
      allow_stale: true
      stale_ttl: 30d  # Longer stale period

# More aggressive health monitoring
health_monitor:
  check_interval: 2m  # More frequent
  unhealthy_threshold: 2  # Faster detection
  recovery_threshold: 3  # More confident recovery

# More aggressive retries
retry:
  max_attempts: 5
  initial_delay: 500ms
  max_delay: 5s
  multiplier: 1.5
```

## Resources

- **Full Documentation**: `CONFIGURATION_GUIDE.md`
- **Environment Template**: `../.env.example`
- **API Documentation**: `../services/api-integration-service/README.md`
- **Developer Guide**: `../docs/DEVELOPER_GUIDE.md`

## Support

For configuration issues:

1. Check `CONFIGURATION_GUIDE.md` for detailed documentation
2. Review service logs for validation errors
3. Test configuration with unit tests
4. Consult API provider documentation
5. Contact development team

---

**Last Updated**: 2024-01-15
**Version**: 1.0.0

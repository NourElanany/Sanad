# API Integration Service - Deployment Guide

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Environment Variables](#environment-variables)
4. [Configuration Files](#configuration-files)
5. [Docker Deployment](#docker-deployment)
6. [Kubernetes Deployment](#kubernetes-deployment)
7. [Production Checklist](#production-checklist)
8. [Monitoring and Observability](#monitoring-and-observability)
9. [Backup and Disaster Recovery](#backup-and-disaster-recovery)
10. [Scaling Strategies](#scaling-strategies)
11. [Security Hardening](#security-hardening)
12. [Troubleshooting](#troubleshooting)

## Overview

This guide provides comprehensive instructions for deploying the API Integration Service in various environments. The service is designed to be deployed as a containerized application with support for:

- **Docker**: Single-container deployment for development and small-scale production
- **Docker Compose**: Multi-container deployment with Redis and PostgreSQL
- **Kubernetes**: Scalable production deployment with high availability
- **Cloud Platforms**: AWS, Azure, Google Cloud Platform

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Load Balancer                            │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
┌───────▼────────┐  ┌───────▼────────┐  ┌───────▼────────┐
│  API Service   │  │  API Service   │  │  API Service   │
│   Instance 1   │  │   Instance 2   │  │   Instance 3   │
└────────┬───────┘  └────────┬───────┘  └────────┬───────┘
         │                   │                   │
         └───────────────────┼───────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
┌───────▼────────┐  ┌───────▼────────┐  ┌───────▼────────┐
│  Redis Cluster │  │   PostgreSQL   │  │  External APIs │
│   (Caching)    │  │   (Storage)    │  │  (Quran, etc.) │
└────────────────┘  └────────────────┘  └────────────────┘
```

## Prerequisites

### System Requirements

**Minimum Requirements** (Development):
- CPU: 2 cores
- RAM: 4 GB
- Disk: 20 GB
- OS: Linux, macOS, or Windows with WSL2

**Recommended Requirements** (Production):
- CPU: 4+ cores
- RAM: 8+ GB
- Disk: 50+ GB SSD
- OS: Linux (Ubuntu 20.04+, Debian 11+, or RHEL 8+)

### Software Dependencies

**Required**:
- Docker 20.10+ or Kubernetes 1.21+
- Redis 6.0+ (for caching and rate limiting)
- PostgreSQL 13+ (for persistent storage)

**Optional**:
- Prometheus (for metrics)
- Grafana (for dashboards)
- Sentry (for error tracking)
- OpenTelemetry Collector (for distributed tracing)

### Network Requirements

**Outbound Access** (to external APIs):
- `api.quran.com` (HTTPS/443)
- `api.alquran.cloud` (HTTPS/443)
- `tanzil.net` (HTTPS/443)
- `everyayah.com` (HTTPS/443)
- `api.sunnah.com` (HTTPS/443)
- `api.aladhan.com` (HTTPS/443)
- `api.islamicfinder.org` (HTTPS/443)
- `api-inference.huggingface.co` (HTTPS/443)

**Inbound Access**:
- Port 8080 (HTTP API)
- Port 9090 (Metrics endpoint)


## Environment Variables

### Required Environment Variables

These variables MUST be set for the service to function:

```bash
# Service Configuration
SERVICE_PORT=8080                    # HTTP port (default: 8080)
SERVICE_HOST=0.0.0.0                 # Bind address (default: 0.0.0.0)

# Redis Configuration
REDIS_URL=redis://redis:6379         # Redis connection URL

# PostgreSQL Configuration
POSTGRES_URL=postgresql://user:password@postgres:5432/sanad
# OR
DATABASE_URL=postgresql://user:password@postgres:5432/sanad

# API Keys (Required for specific features)
SUNNAH_COM_API_KEY=your_key_here     # Required for hadith search
HUGGING_FACE_API_KEY=your_key_here   # Required for AI features
```

### Optional Environment Variables

```bash
# Service Configuration
ENVIRONMENT=production               # Environment name (development, staging, production)
LOG_LEVEL=info                       # Logging level (debug, info, warn, error)
JSON_LOGGING=true                    # Enable JSON logging (true/false)
CONFIG_PATH=/etc/sanad/config/api_integration_config.yaml  # Config file path

# API Keys (Optional - for higher rate limits or backup APIs)
QURAN_COM_API_KEY=your_key_here      # Optional - higher rate limits
ISLAMIC_FINDER_API_KEY=your_key_here # Optional - backup for prayer times
OPENAI_API_KEY=your_key_here         # Optional - alternative AI provider

# Redis Configuration (Advanced)
REDIS_POOL_SIZE=10                   # Connection pool size
REDIS_CONNECTION_TIMEOUT=5s          # Connection timeout

# PostgreSQL Configuration (Advanced)
POSTGRES_POOL_SIZE=20                # Connection pool size
POSTGRES_CONNECTION_TIMEOUT=10s      # Connection timeout

# Observability
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector:4317  # OpenTelemetry collector
SENTRY_DSN=https://your-dsn@sentry.io/project      # Sentry error tracking
PROMETHEUS_METRICS_PORT=9090         # Prometheus metrics port

# Rate Limiting (Override defaults)
QURAN_COM_RATE_LIMIT_PER_MINUTE=100
QURAN_COM_RATE_LIMIT_PER_HOUR=2000
QURAN_COM_RATE_LIMIT_PER_DAY=20000

# Health Monitoring
HEALTH_CHECK_INTERVAL=5m             # Health check interval
UNHEALTHY_THRESHOLD=3                # Failures before unhealthy
RECOVERY_THRESHOLD=2                 # Successes before healthy

# Retry Configuration
RETRY_MAX_ATTEMPTS=3                 # Maximum retry attempts
RETRY_INITIAL_DELAY=1s               # Initial retry delay
RETRY_MAX_DELAY=10s                  # Maximum retry delay
RETRY_MULTIPLIER=2.0                 # Exponential backoff multiplier
```

### Environment-Specific Configuration

**Development** (`.env.development`):
```bash
ENVIRONMENT=development
LOG_LEVEL=debug
JSON_LOGGING=false
REDIS_URL=redis://localhost:6379
POSTGRES_URL=postgresql://postgres:postgres@localhost:5432/sanad_dev
```

**Staging** (`.env.staging`):
```bash
ENVIRONMENT=staging
LOG_LEVEL=info
JSON_LOGGING=true
REDIS_URL=redis://redis-staging:6379
POSTGRES_URL=postgresql://user:password@postgres-staging:5432/sanad_staging
SENTRY_DSN=https://your-dsn@sentry.io/staging-project
```

**Production** (`.env.production`):
```bash
ENVIRONMENT=production
LOG_LEVEL=warn
JSON_LOGGING=true
REDIS_URL=rediss://redis-cluster:6380  # TLS enabled
POSTGRES_URL=postgresql://user:password@postgres-primary:5432/sanad?sslmode=require
SENTRY_DSN=https://your-dsn@sentry.io/production-project
OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-collector:4317
```

## Configuration Files

### Main Configuration File

**Location**: `config/api_integration_config.yaml`

This file contains all service configuration. See `config/CONFIGURATION_GUIDE.md` for detailed documentation.

**Key Sections**:
1. Service settings (name, port, host)
2. Database connections (Redis, PostgreSQL)
3. API configurations (endpoints, rate limits, timeouts)
4. Cache strategies (TTL, stale cache settings)
5. Health monitoring (check interval, thresholds)
6. Retry policies (max attempts, backoff strategy)

### Secrets Management

**Development**: Use `.env` file (never commit to version control)

**Production**: Use a secrets manager:

**AWS Secrets Manager**:
```bash
# Store secrets
aws secretsmanager create-secret \
  --name sanad/api-integration/api-keys \
  --secret-string '{"SUNNAH_COM_API_KEY":"key1","HUGGING_FACE_API_KEY":"key2"}'

# Retrieve in application
aws secretsmanager get-secret-value \
  --secret-id sanad/api-integration/api-keys \
  --query SecretString --output text
```

**HashiCorp Vault**:
```bash
# Store secrets
vault kv put secret/sanad/api-integration \
  SUNNAH_COM_API_KEY=key1 \
  HUGGING_FACE_API_KEY=key2

# Retrieve in application
vault kv get -field=SUNNAH_COM_API_KEY secret/sanad/api-integration
```

**Kubernetes Secrets**:
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


## Docker Deployment

### Building the Docker Image

**Dockerfile** (already provided in project root):

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

**Build the image**:
```bash
# From project root
docker build -t sanad/api-integration-service:latest .

# With specific version tag
docker build -t sanad/api-integration-service:1.0.0 .

# Multi-platform build (for ARM and x86)
docker buildx build --platform linux/amd64,linux/arm64 \
  -t sanad/api-integration-service:latest .
```

### Running with Docker

**Simple run** (development):
```bash
docker run -d \
  --name api-integration \
  -p 8080:8080 \
  -e REDIS_URL=redis://host.docker.internal:6379 \
  -e POSTGRES_URL=postgresql://postgres:postgres@host.docker.internal:5432/sanad \
  -e SUNNAH_COM_API_KEY=your_key \
  -e HUGGING_FACE_API_KEY=your_key \
  sanad/api-integration-service:latest
```

**With environment file**:
```bash
docker run -d \
  --name api-integration \
  -p 8080:8080 \
  --env-file .env.production \
  sanad/api-integration-service:latest
```

**With volume mounts** (for custom config):
```bash
docker run -d \
  --name api-integration \
  -p 8080:8080 \
  -v $(pwd)/config:/etc/sanad/config:ro \
  --env-file .env.production \
  sanad/api-integration-service:latest
```

### Docker Compose Deployment

**docker-compose.yml**:

```yaml
version: '3.8'

services:
  api-integration:
    image: sanad/api-integration-service:latest
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "8080:8080"
      - "9090:9090"  # Metrics
    environment:
      - REDIS_URL=redis://redis:6379
      - POSTGRES_URL=postgresql://postgres:${POSTGRES_PASSWORD}@postgres:5432/sanad
    env_file:
      - .env.production
    depends_on:
      redis:
        condition: service_healthy
      postgres:
        condition: service_healthy
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/api/v1/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
    restart: unless-stopped
    networks:
      - sanad-network
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis-data:/data
    command: redis-server --appendonly yes
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 3s
      retries: 3
    restart: unless-stopped
    networks:
      - sanad-network

  postgres:
    image: postgres:15-alpine
    ports:
      - "5432:5432"
    environment:
      - POSTGRES_DB=sanad
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
    volumes:
      - postgres-data:/var/lib/postgresql/data
      - ./database/init:/docker-entrypoint-initdb.d:ro
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 3s
      retries: 3
    restart: unless-stopped
    networks:
      - sanad-network

  # Optional: Prometheus for metrics
  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9091:9090"
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml:ro
      - prometheus-data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
    restart: unless-stopped
    networks:
      - sanad-network

  # Optional: Grafana for dashboards
  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=${GRAFANA_PASSWORD}
    volumes:
      - grafana-data:/var/lib/grafana
      - ./monitoring/grafana/dashboards:/etc/grafana/provisioning/dashboards:ro
      - ./monitoring/grafana/datasources:/etc/grafana/provisioning/datasources:ro
    restart: unless-stopped
    networks:
      - sanad-network

volumes:
  redis-data:
  postgres-data:
  prometheus-data:
  grafana-data:

networks:
  sanad-network:
    driver: bridge
```

**Deploy with Docker Compose**:
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f api-integration

# Stop all services
docker-compose down

# Stop and remove volumes (WARNING: deletes data)
docker-compose down -v
```

### Docker Compose for Production

**docker-compose.prod.yml**:

```yaml
version: '3.8'

services:
  api-integration:
    image: sanad/api-integration-service:1.0.0
    deploy:
      replicas: 3
      resources:
        limits:
          cpus: '2'
          memory: 4G
        reservations:
          cpus: '1'
          memory: 2G
      restart_policy:
        condition: on-failure
        delay: 5s
        max_attempts: 3
    ports:
      - "8080:8080"
    environment:
      - ENVIRONMENT=production
      - LOG_LEVEL=warn
      - JSON_LOGGING=true
    env_file:
      - .env.production
    secrets:
      - api_keys
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/api/v1/health"]
      interval: 30s
      timeout: 10s
      retries: 3
    networks:
      - sanad-network

  redis:
    image: redis:7-alpine
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 2G
    volumes:
      - redis-data:/data
    command: redis-server --appendonly yes --requirepass ${REDIS_PASSWORD}
    networks:
      - sanad-network

  postgres:
    image: postgres:15-alpine
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 4G
    environment:
      - POSTGRES_DB=sanad
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
    volumes:
      - postgres-data:/var/lib/postgresql/data
    networks:
      - sanad-network

secrets:
  api_keys:
    file: ./secrets/api_keys.txt

volumes:
  redis-data:
    driver: local
  postgres-data:
    driver: local

networks:
  sanad-network:
    driver: overlay
```

**Deploy in Swarm mode**:
```bash
# Initialize swarm
docker swarm init

# Deploy stack
docker stack deploy -c docker-compose.prod.yml sanad

# View services
docker stack services sanad

# View logs
docker service logs sanad_api-integration

# Scale service
docker service scale sanad_api-integration=5

# Remove stack
docker stack rm sanad
```


## Kubernetes Deployment

### Prerequisites

- Kubernetes cluster 1.21+ (EKS, GKE, AKS, or self-hosted)
- kubectl configured to access your cluster
- Helm 3+ (optional, for easier deployment)

### Kubernetes Manifests

#### 1. Namespace

**namespace.yaml**:
```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: sanad
  labels:
    name: sanad
    environment: production
```

#### 2. ConfigMap

**configmap.yaml**:
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: api-integration-config
  namespace: sanad
data:
  api_integration_config.yaml: |
    service:
      name: api-integration-service
      port: 8080
      host: 0.0.0.0
    
    redis:
      url: redis://redis-service:6379
      pool_size: 10
      connection_timeout: 5s
    
    postgres:
      url: postgresql://postgres:5432/sanad
      pool_size: 20
      connection_timeout: 10s
    
    # ... rest of configuration (see config/api_integration_config.yaml)
```

#### 3. Secrets

**secrets.yaml**:
```yaml
apiVersion: v1
kind: Secret
metadata:
  name: api-keys
  namespace: sanad
type: Opaque
stringData:
  SUNNAH_COM_API_KEY: "your_key_here"
  HUGGING_FACE_API_KEY: "your_key_here"
  ISLAMIC_FINDER_API_KEY: "your_key_here"
  OPENAI_API_KEY: "your_key_here"
---
apiVersion: v1
kind: Secret
metadata:
  name: database-credentials
  namespace: sanad
type: Opaque
stringData:
  POSTGRES_PASSWORD: "your_secure_password"
  REDIS_PASSWORD: "your_secure_password"
```

**Create secrets from command line** (recommended):
```bash
kubectl create secret generic api-keys \
  --from-literal=SUNNAH_COM_API_KEY=your_key \
  --from-literal=HUGGING_FACE_API_KEY=your_key \
  -n sanad

kubectl create secret generic database-credentials \
  --from-literal=POSTGRES_PASSWORD=your_password \
  --from-literal=REDIS_PASSWORD=your_password \
  -n sanad
```

#### 4. Deployment

**deployment.yaml**:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-integration-service
  namespace: sanad
  labels:
    app: api-integration
    version: v1
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  selector:
    matchLabels:
      app: api-integration
  template:
    metadata:
      labels:
        app: api-integration
        version: v1
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "9090"
        prometheus.io/path: "/metrics"
    spec:
      containers:
      - name: api-integration
        image: sanad/api-integration-service:1.0.0
        imagePullPolicy: IfNotPresent
        ports:
        - name: http
          containerPort: 8080
          protocol: TCP
        - name: metrics
          containerPort: 9090
          protocol: TCP
        env:
        - name: ENVIRONMENT
          value: "production"
        - name: LOG_LEVEL
          value: "info"
        - name: JSON_LOGGING
          value: "true"
        - name: REDIS_URL
          value: "redis://redis-service:6379"
        - name: POSTGRES_URL
          valueFrom:
            secretKeyRef:
              name: database-credentials
              key: POSTGRES_URL
        envFrom:
        - secretRef:
            name: api-keys
        volumeMounts:
        - name: config
          mountPath: /etc/sanad/config
          readOnly: true
        resources:
          requests:
            cpu: 500m
            memory: 1Gi
          limits:
            cpu: 2000m
            memory: 4Gi
        livenessProbe:
          httpGet:
            path: /api/v1/health
            port: http
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /api/v1/health
            port: http
          initialDelaySeconds: 10
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 2
        securityContext:
          runAsNonRoot: true
          runAsUser: 1000
          allowPrivilegeEscalation: false
          readOnlyRootFilesystem: true
          capabilities:
            drop:
            - ALL
      volumes:
      - name: config
        configMap:
          name: api-integration-config
      affinity:
        podAntiAffinity:
          preferredDuringSchedulingIgnoredDuringExecution:
          - weight: 100
            podAffinityTerm:
              labelSelector:
                matchExpressions:
                - key: app
                  operator: In
                  values:
                  - api-integration
              topologyKey: kubernetes.io/hostname
```

#### 5. Service

**service.yaml**:
```yaml
apiVersion: v1
kind: Service
metadata:
  name: api-integration-service
  namespace: sanad
  labels:
    app: api-integration
spec:
  type: ClusterIP
  ports:
  - name: http
    port: 80
    targetPort: http
    protocol: TCP
  - name: metrics
    port: 9090
    targetPort: metrics
    protocol: TCP
  selector:
    app: api-integration
```

#### 6. Ingress

**ingress.yaml**:
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: api-integration-ingress
  namespace: sanad
  annotations:
    kubernetes.io/ingress.class: nginx
    cert-manager.io/cluster-issuer: letsencrypt-prod
    nginx.ingress.kubernetes.io/rate-limit: "100"
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
spec:
  tls:
  - hosts:
    - api.sanad.app
    secretName: api-sanad-tls
  rules:
  - host: api.sanad.app
    http:
      paths:
      - path: /api/v1
        pathType: Prefix
        backend:
          service:
            name: api-integration-service
            port:
              number: 80
```

#### 7. HorizontalPodAutoscaler

**hpa.yaml**:
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: api-integration-hpa
  namespace: sanad
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: api-integration-service
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
      - type: Percent
        value: 50
        periodSeconds: 60
    scaleUp:
      stabilizationWindowSeconds: 0
      policies:
      - type: Percent
        value: 100
        periodSeconds: 15
      - type: Pods
        value: 2
        periodSeconds: 15
      selectPolicy: Max
```

### Deploy to Kubernetes

**Deploy all resources**:
```bash
# Create namespace
kubectl apply -f k8s/namespace.yaml

# Create secrets (use kubectl create secret instead of YAML for security)
kubectl create secret generic api-keys \
  --from-literal=SUNNAH_COM_API_KEY=$SUNNAH_KEY \
  --from-literal=HUGGING_FACE_API_KEY=$HF_KEY \
  -n sanad

# Create configmap
kubectl apply -f k8s/configmap.yaml

# Deploy application
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml
kubectl apply -f k8s/ingress.yaml
kubectl apply -f k8s/hpa.yaml

# Verify deployment
kubectl get pods -n sanad
kubectl get svc -n sanad
kubectl get ingress -n sanad
```

**Deploy with Kustomize**:

**kustomization.yaml**:
```yaml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization

namespace: sanad

resources:
  - namespace.yaml
  - configmap.yaml
  - deployment.yaml
  - service.yaml
  - ingress.yaml
  - hpa.yaml

secretGenerator:
  - name: api-keys
    envs:
      - secrets.env

images:
  - name: sanad/api-integration-service
    newTag: 1.0.0
```

```bash
kubectl apply -k k8s/
```


### Redis and PostgreSQL on Kubernetes

#### Redis StatefulSet

**redis-statefulset.yaml**:
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: redis
  namespace: sanad
spec:
  serviceName: redis-service
  replicas: 1
  selector:
    matchLabels:
      app: redis
  template:
    metadata:
      labels:
        app: redis
    spec:
      containers:
      - name: redis
        image: redis:7-alpine
        ports:
        - containerPort: 6379
          name: redis
        command:
        - redis-server
        - --appendonly
        - "yes"
        - --requirepass
        - $(REDIS_PASSWORD)
        env:
        - name: REDIS_PASSWORD
          valueFrom:
            secretKeyRef:
              name: database-credentials
              key: REDIS_PASSWORD
        volumeMounts:
        - name: redis-data
          mountPath: /data
        resources:
          requests:
            cpu: 250m
            memory: 512Mi
          limits:
            cpu: 1000m
            memory: 2Gi
  volumeClaimTemplates:
  - metadata:
      name: redis-data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 10Gi
---
apiVersion: v1
kind: Service
metadata:
  name: redis-service
  namespace: sanad
spec:
  clusterIP: None
  ports:
  - port: 6379
    targetPort: 6379
  selector:
    app: redis
```

#### PostgreSQL StatefulSet

**postgres-statefulset.yaml**:
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: postgres
  namespace: sanad
spec:
  serviceName: postgres-service
  replicas: 1
  selector:
    matchLabels:
      app: postgres
  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
      - name: postgres
        image: postgres:15-alpine
        ports:
        - containerPort: 5432
          name: postgres
        env:
        - name: POSTGRES_DB
          value: sanad
        - name: POSTGRES_USER
          value: postgres
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: database-credentials
              key: POSTGRES_PASSWORD
        - name: PGDATA
          value: /var/lib/postgresql/data/pgdata
        volumeMounts:
        - name: postgres-data
          mountPath: /var/lib/postgresql/data
        resources:
          requests:
            cpu: 500m
            memory: 1Gi
          limits:
            cpu: 2000m
            memory: 4Gi
  volumeClaimTemplates:
  - metadata:
      name: postgres-data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 50Gi
---
apiVersion: v1
kind: Service
metadata:
  name: postgres-service
  namespace: sanad
spec:
  clusterIP: None
  ports:
  - port: 5432
    targetPort: 5432
  selector:
    app: postgres
```

**Note**: For production, consider using managed services (AWS RDS, Azure Database, Google Cloud SQL) or operators (Redis Operator, PostgreSQL Operator) for better reliability and management.

## Production Checklist

### Pre-Deployment

- [ ] **API Keys Obtained**: All required API keys obtained and tested
- [ ] **Configuration Reviewed**: Configuration file reviewed and validated
- [ ] **Secrets Secured**: All secrets stored in secrets manager (not in code)
- [ ] **Environment Variables Set**: All required environment variables configured
- [ ] **Database Migrations**: Database schema created and migrations applied
- [ ] **Redis Configured**: Redis cluster configured with persistence
- [ ] **PostgreSQL Configured**: PostgreSQL configured with SSL and backups
- [ ] **Network Access**: Firewall rules configured for inbound/outbound traffic
- [ ] **DNS Configured**: Domain names configured and DNS records created
- [ ] **SSL Certificates**: SSL/TLS certificates obtained and configured
- [ ] **Load Balancer**: Load balancer configured with health checks
- [ ] **Monitoring Setup**: Prometheus, Grafana, and alerting configured
- [ ] **Logging Setup**: Centralized logging configured (ELK, Loki, CloudWatch)
- [ ] **Error Tracking**: Sentry or similar error tracking configured
- [ ] **Backup Strategy**: Backup and disaster recovery plan in place
- [ ] **Documentation**: Deployment documentation reviewed and updated

### Post-Deployment

- [ ] **Health Check**: Verify `/api/v1/health` endpoint returns healthy status
- [ ] **API Testing**: Test all API endpoints with sample requests
- [ ] **External API Connectivity**: Verify connectivity to all external APIs
- [ ] **Cache Working**: Verify Redis cache is working (check hit rate)
- [ ] **Database Working**: Verify PostgreSQL connection and queries
- [ ] **Rate Limiting**: Verify rate limiting is working correctly
- [ ] **Fallback Mechanisms**: Test fallback by simulating API failures
- [ ] **Monitoring Active**: Verify metrics are being collected
- [ ] **Alerts Working**: Test alert notifications
- [ ] **Logs Flowing**: Verify logs are being collected and searchable
- [ ] **Performance**: Run load tests to verify performance
- [ ] **Security Scan**: Run security scan on deployed containers
- [ ] **Documentation**: Update runbook with deployment details

### Security Checklist

- [ ] **API Keys Rotated**: API keys rotated from default/test keys
- [ ] **Strong Passwords**: Strong passwords for databases
- [ ] **TLS Enabled**: TLS/SSL enabled for all connections
- [ ] **Network Segmentation**: Services in private network, only API exposed
- [ ] **Least Privilege**: Service accounts have minimum required permissions
- [ ] **Secrets Encrypted**: Secrets encrypted at rest and in transit
- [ ] **Container Security**: Containers run as non-root user
- [ ] **Image Scanning**: Container images scanned for vulnerabilities
- [ ] **Rate Limiting**: Rate limiting configured to prevent abuse
- [ ] **Input Validation**: All inputs validated and sanitized
- [ ] **CORS Configured**: CORS configured appropriately
- [ ] **Security Headers**: Security headers configured (HSTS, CSP, etc.)
- [ ] **Audit Logging**: Audit logging enabled for sensitive operations
- [ ] **Incident Response**: Incident response plan documented


## Monitoring and Observability

### Prometheus Configuration

**prometheus.yml**:
```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'api-integration-service'
    static_configs:
      - targets: ['api-integration-service:9090']
    metrics_path: '/metrics'
    
  - job_name: 'redis'
    static_configs:
      - targets: ['redis-exporter:9121']
    
  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres-exporter:9187']

alerting:
  alertmanagers:
    - static_configs:
        - targets: ['alertmanager:9093']

rule_files:
  - '/etc/prometheus/alerts/*.yml'
```

### Alert Rules

**alerts/api-integration.yml**:
```yaml
groups:
  - name: api_integration_alerts
    interval: 30s
    rules:
      - alert: HighErrorRate
        expr: rate(api_requests_total{status="error"}[5m]) > 0.05
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value | humanizePercentage }} for {{ $labels.api }}"
      
      - alert: APIUnhealthy
        expr: api_health_status{is_healthy="false"} == 1
        for: 10m
        labels:
          severity: critical
        annotations:
          summary: "API {{ $labels.api }} is unhealthy"
          description: "API {{ $labels.api }} has been unhealthy for 10 minutes"
      
      - alert: HighResponseTime
        expr: histogram_quantile(0.95, rate(api_response_time_seconds_bucket[5m])) > 2
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High response time for {{ $labels.api }}"
          description: "95th percentile response time is {{ $value }}s"
      
      - alert: RateLimitApproaching
        expr: api_rate_limit_usage_ratio > 0.8
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Rate limit approaching for {{ $labels.api }}"
          description: "Rate limit usage is {{ $value | humanizePercentage }}"
      
      - alert: LowCacheHitRate
        expr: cache_hit_rate < 0.5
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Low cache hit rate"
          description: "Cache hit rate is {{ $value | humanizePercentage }}"
      
      - alert: ServiceDown
        expr: up{job="api-integration-service"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "API Integration Service is down"
          description: "Service has been down for 1 minute"
```

### Grafana Dashboards

**Dashboard JSON** (import into Grafana):

Key panels to include:
1. **Request Rate**: Requests per second by API
2. **Error Rate**: Error percentage by API
3. **Response Time**: P50, P95, P99 latencies
4. **Cache Performance**: Hit rate, miss rate, evictions
5. **API Health**: Health status of all APIs
6. **Rate Limit Usage**: Current usage vs limits
7. **Resource Usage**: CPU, memory, network
8. **Database Connections**: Active connections, pool usage

### OpenTelemetry Configuration

**otel-collector-config.yaml**:
```yaml
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: 0.0.0.0:4317
      http:
        endpoint: 0.0.0.0:4318

processors:
  batch:
    timeout: 10s
    send_batch_size: 1024
  
  memory_limiter:
    check_interval: 1s
    limit_mib: 512

exporters:
  prometheus:
    endpoint: "0.0.0.0:8889"
  
  jaeger:
    endpoint: jaeger:14250
    tls:
      insecure: true
  
  logging:
    loglevel: info

service:
  pipelines:
    traces:
      receivers: [otlp]
      processors: [memory_limiter, batch]
      exporters: [jaeger, logging]
    
    metrics:
      receivers: [otlp]
      processors: [memory_limiter, batch]
      exporters: [prometheus, logging]
```

### Logging Configuration

**Structured Logging** (JSON format):
```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "message": "API request completed",
  "request_id": "550e8400-e29b-41d4-a716-446655440000",
  "api": "quran.com",
  "endpoint": "/api/v1/quran/text",
  "method": "GET",
  "status_code": 200,
  "response_time_ms": 150,
  "cache_status": "HIT",
  "user_agent": "Mozilla/5.0...",
  "ip_address": "192.168.1.1"
}
```

**Log Aggregation** (ELK Stack):

**filebeat.yml**:
```yaml
filebeat.inputs:
  - type: container
    paths:
      - '/var/lib/docker/containers/*/*.log'
    processors:
      - add_kubernetes_metadata:
          host: ${NODE_NAME}
          matchers:
          - logs_path:
              logs_path: "/var/lib/docker/containers/"

output.elasticsearch:
  hosts: ["elasticsearch:9200"]
  index: "sanad-api-integration-%{+yyyy.MM.dd}"

setup.kibana:
  host: "kibana:5601"
```

## Backup and Disaster Recovery

### Backup Strategy

**PostgreSQL Backups**:

**Daily automated backups**:
```bash
#!/bin/bash
# backup-postgres.sh

BACKUP_DIR="/backups/postgres"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/sanad_$TIMESTAMP.sql.gz"

# Create backup
pg_dump -h postgres -U postgres sanad | gzip > $BACKUP_FILE

# Upload to S3
aws s3 cp $BACKUP_FILE s3://sanad-backups/postgres/

# Keep only last 30 days locally
find $BACKUP_DIR -name "*.sql.gz" -mtime +30 -delete

# Verify backup
gunzip -c $BACKUP_FILE | head -n 1
```

**Kubernetes CronJob for backups**:
```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: postgres-backup
  namespace: sanad
spec:
  schedule: "0 2 * * *"  # Daily at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: backup
            image: postgres:15-alpine
            command:
            - /bin/sh
            - -c
            - |
              pg_dump -h postgres-service -U postgres sanad | \
              gzip > /backup/sanad_$(date +%Y%m%d_%H%M%S).sql.gz
            env:
            - name: PGPASSWORD
              valueFrom:
                secretKeyRef:
                  name: database-credentials
                  key: POSTGRES_PASSWORD
            volumeMounts:
            - name: backup
              mountPath: /backup
          volumes:
          - name: backup
            persistentVolumeClaim:
              claimName: backup-pvc
          restartPolicy: OnFailure
```

**Redis Backups**:

Redis automatically creates RDB snapshots. Configure backup of `/data` directory:

```bash
# Backup Redis data
kubectl exec -n sanad redis-0 -- redis-cli BGSAVE
kubectl cp sanad/redis-0:/data/dump.rdb ./redis-backup-$(date +%Y%m%d).rdb
aws s3 cp ./redis-backup-*.rdb s3://sanad-backups/redis/
```

### Disaster Recovery

**Recovery Time Objective (RTO)**: 1 hour  
**Recovery Point Objective (RPO)**: 24 hours

**Recovery Procedure**:

1. **Restore PostgreSQL**:
```bash
# Download latest backup
aws s3 cp s3://sanad-backups/postgres/latest.sql.gz ./

# Restore database
gunzip -c latest.sql.gz | psql -h postgres -U postgres sanad
```

2. **Restore Redis** (if needed):
```bash
# Download latest backup
aws s3 cp s3://sanad-backups/redis/latest.rdb ./

# Copy to Redis pod
kubectl cp ./latest.rdb sanad/redis-0:/data/dump.rdb

# Restart Redis
kubectl rollout restart statefulset/redis -n sanad
```

3. **Verify Service**:
```bash
# Check health
curl https://api.sanad.app/api/v1/health

# Test endpoints
curl https://api.sanad.app/api/v1/quran/text?surah=1&ayah=1
```


## Scaling Strategies

### Horizontal Scaling

**Auto-scaling based on metrics**:

The HorizontalPodAutoscaler (HPA) automatically scales based on:
- CPU utilization (target: 70%)
- Memory utilization (target: 80%)
- Custom metrics (requests per second, error rate)

**Manual scaling**:
```bash
# Scale to 5 replicas
kubectl scale deployment api-integration-service --replicas=5 -n sanad

# Or update HPA
kubectl patch hpa api-integration-hpa -n sanad -p '{"spec":{"minReplicas":5,"maxReplicas":15}}'
```

### Vertical Scaling

**Increase resource limits**:
```yaml
resources:
  requests:
    cpu: 1000m      # Increased from 500m
    memory: 2Gi     # Increased from 1Gi
  limits:
    cpu: 4000m      # Increased from 2000m
    memory: 8Gi     # Increased from 4Gi
```

### Database Scaling

**Redis Scaling**:

**Option 1: Redis Cluster** (for high availability):
```yaml
# Use Redis Cluster with 3 masters and 3 replicas
# See Redis Operator documentation
```

**Option 2: Redis Sentinel** (for failover):
```yaml
# Configure Redis Sentinel for automatic failover
# See Redis Sentinel documentation
```

**PostgreSQL Scaling**:

**Read Replicas**:
```yaml
# Configure read replicas for read-heavy workloads
# Route read queries to replicas
# Route write queries to primary
```

**Connection Pooling**:
```yaml
# Use PgBouncer for connection pooling
# Reduces connection overhead
# Allows more concurrent connections
```

### Load Balancing

**Nginx Ingress Controller**:
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  annotations:
    nginx.ingress.kubernetes.io/load-balance: "round_robin"
    nginx.ingress.kubernetes.io/upstream-hash-by: "$request_uri"
```

**AWS Application Load Balancer**:
```yaml
apiVersion: v1
kind: Service
metadata:
  annotations:
    service.beta.kubernetes.io/aws-load-balancer-type: "nlb"
    service.beta.kubernetes.io/aws-load-balancer-cross-zone-load-balancing-enabled: "true"
spec:
  type: LoadBalancer
```

## Security Hardening

### Container Security

**Run as non-root user**:
```dockerfile
FROM debian:bookworm-slim
RUN useradd -m -u 1000 sanad
USER sanad
```

**Read-only root filesystem**:
```yaml
securityContext:
  readOnlyRootFilesystem: true
  runAsNonRoot: true
  runAsUser: 1000
  allowPrivilegeEscalation: false
  capabilities:
    drop:
    - ALL
```

**Image scanning**:
```bash
# Scan with Trivy
trivy image sanad/api-integration-service:latest

# Scan with Snyk
snyk container test sanad/api-integration-service:latest
```

### Network Security

**Network Policies**:
```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: api-integration-network-policy
  namespace: sanad
spec:
  podSelector:
    matchLabels:
      app: api-integration
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
    ports:
    - protocol: TCP
      port: 8080
  egress:
  - to:
    - podSelector:
        matchLabels:
          app: redis
    ports:
    - protocol: TCP
      port: 6379
  - to:
    - podSelector:
        matchLabels:
          app: postgres
    ports:
    - protocol: TCP
      port: 5432
  - to:
    - namespaceSelector: {}
    ports:
    - protocol: TCP
      port: 443  # HTTPS to external APIs
```

### TLS/SSL Configuration

**Enable TLS for Redis**:
```yaml
# Use Redis with TLS
REDIS_URL=rediss://redis:6380
```

**Enable SSL for PostgreSQL**:
```yaml
# Require SSL connections
POSTGRES_URL=postgresql://user:pass@host:5432/db?sslmode=require
```

**Certificate Management with cert-manager**:
```yaml
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: api-sanad-tls
  namespace: sanad
spec:
  secretName: api-sanad-tls
  issuerRef:
    name: letsencrypt-prod
    kind: ClusterIssuer
  dnsNames:
  - api.sanad.app
```

### Secrets Management

**Use External Secrets Operator**:
```yaml
apiVersion: external-secrets.io/v1beta1
kind: ExternalSecret
metadata:
  name: api-keys
  namespace: sanad
spec:
  refreshInterval: 1h
  secretStoreRef:
    name: aws-secrets-manager
    kind: SecretStore
  target:
    name: api-keys
  data:
  - secretKey: SUNNAH_COM_API_KEY
    remoteRef:
      key: sanad/api-integration/api-keys
      property: SUNNAH_COM_API_KEY
```

## Troubleshooting

### Common Issues

#### 1. Service Not Starting

**Symptoms**:
- Pods in CrashLoopBackOff state
- Health checks failing

**Diagnosis**:
```bash
# Check pod status
kubectl get pods -n sanad

# View pod logs
kubectl logs -n sanad api-integration-service-xxx

# Describe pod for events
kubectl describe pod -n sanad api-integration-service-xxx
```

**Common Causes**:
- Missing environment variables
- Invalid configuration
- Cannot connect to Redis/PostgreSQL
- API keys not set

#### 2. High Memory Usage

**Symptoms**:
- Pods being OOMKilled
- High memory usage in metrics

**Diagnosis**:
```bash
# Check resource usage
kubectl top pods -n sanad

# View memory metrics
kubectl exec -n sanad api-integration-service-xxx -- cat /proc/meminfo
```

**Solutions**:
- Increase memory limits
- Check for memory leaks
- Optimize cache size
- Reduce connection pool sizes

#### 3. External API Connectivity Issues

**Symptoms**:
- All APIs marked as unhealthy
- Timeout errors in logs

**Diagnosis**:
```bash
# Test connectivity from pod
kubectl exec -n sanad api-integration-service-xxx -- curl -v https://api.quran.com

# Check network policies
kubectl get networkpolicies -n sanad

# Check DNS resolution
kubectl exec -n sanad api-integration-service-xxx -- nslookup api.quran.com
```

**Solutions**:
- Verify network policies allow egress
- Check firewall rules
- Verify DNS resolution
- Check proxy settings

#### 4. Database Connection Issues

**Symptoms**:
- Cannot connect to PostgreSQL/Redis
- Connection timeout errors

**Diagnosis**:
```bash
# Test Redis connection
kubectl exec -n sanad redis-0 -- redis-cli ping

# Test PostgreSQL connection
kubectl exec -n sanad postgres-0 -- psql -U postgres -c "SELECT 1"

# Check service endpoints
kubectl get endpoints -n sanad
```

**Solutions**:
- Verify database pods are running
- Check connection strings
- Verify credentials
- Check network connectivity

### Debug Mode

**Enable debug logging**:
```bash
# Update deployment
kubectl set env deployment/api-integration-service LOG_LEVEL=debug -n sanad

# Or edit deployment
kubectl edit deployment api-integration-service -n sanad
# Change LOG_LEVEL to debug
```

### Performance Profiling

**CPU Profiling**:
```bash
# Get pprof data
kubectl port-forward -n sanad api-integration-service-xxx 6060:6060
curl http://localhost:6060/debug/pprof/profile?seconds=30 > cpu.prof

# Analyze with pprof
go tool pprof cpu.prof
```

**Memory Profiling**:
```bash
# Get heap profile
curl http://localhost:6060/debug/pprof/heap > heap.prof
go tool pprof heap.prof
```

### Useful Commands

```bash
# View all resources in namespace
kubectl get all -n sanad

# View events
kubectl get events -n sanad --sort-by='.lastTimestamp'

# View logs from all pods
kubectl logs -n sanad -l app=api-integration --tail=100 -f

# Execute command in pod
kubectl exec -it -n sanad api-integration-service-xxx -- /bin/sh

# Port forward for local testing
kubectl port-forward -n sanad svc/api-integration-service 8080:80

# Restart deployment
kubectl rollout restart deployment/api-integration-service -n sanad

# View rollout status
kubectl rollout status deployment/api-integration-service -n sanad

# Rollback deployment
kubectl rollout undo deployment/api-integration-service -n sanad
```

---

## Support and Resources

### Documentation
- **API Documentation**: See `API_DOCUMENTATION.md`
- **Configuration Guide**: See `config/CONFIGURATION_GUIDE.md`
- **Developer Guide**: See `DEVELOPER_GUIDE.md`

### Monitoring
- **Grafana Dashboards**: http://grafana.sanad.app
- **Prometheus**: http://prometheus.sanad.app
- **Health Endpoint**: https://api.sanad.app/api/v1/health

### Support Channels
- **GitHub Issues**: For bug reports and feature requests
- **Documentation**: For detailed guides and examples
- **On-Call**: For production incidents

---

**Last Updated**: 2024-01-15  
**Version**: 1.0.0  
**Maintained by**: Sanad DevOps Team


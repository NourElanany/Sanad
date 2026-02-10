# Kubernetes Deployment - API Integration Service

This directory contains production-ready Kubernetes manifests for deploying the API Integration Service.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Architecture](#architecture)
- [Manifests](#manifests)
- [Configuration](#configuration)
- [Deployment](#deployment)
- [Monitoring](#monitoring)
- [Scaling](#scaling)
- [Security](#security)
- [Troubleshooting](#troubleshooting)
- [Best Practices](#best-practices)

## Overview

The Kubernetes deployment includes:

- **API Integration Service**: Main application with 3+ replicas
- **Redis**: Caching and rate limiting
- **PostgreSQL**: Persistent storage
- **Horizontal Pod Autoscaler**: Automatic scaling based on CPU/memory
- **Pod Disruption Budget**: High availability guarantees
- **Network Policies**: Network security
- **Service Monitor**: Prometheus integration
- **Ingress**: External access with TLS

## Prerequisites

### Required

- Kubernetes cluster (v1.24+)
- kubectl CLI tool
- Sufficient cluster resources:
  - CPU: 2+ cores
  - Memory: 4+ GB
  - Storage: 20+ GB

### Optional

- Helm (for package management)
- Kustomize (for environment-specific configurations)
- cert-manager (for TLS certificate management)
- Prometheus Operator (for monitoring)
- Ingress Controller (nginx, traefik, or cloud provider)

## Quick Start

### 1. Create Namespace

```bash
kubectl apply -f namespace.yaml
```

### 2. Create Secrets

**IMPORTANT**: Replace placeholder values with actual API keys!

```bash
kubectl create secret generic api-integration-secrets \
  --from-literal=POSTGRES_PASSWORD='your_secure_password' \
  --from-literal=QURAN_COM_API_KEY='your_actual_key' \
  --from-literal=SUNNAH_COM_API_KEY='your_actual_key' \
  --from-literal=ISLAMIC_FINDER_API_KEY='your_actual_key' \
  --from-literal=HUGGING_FACE_API_KEY='your_actual_key' \
  --namespace=sanad
```

### 3. Deploy All Resources

```bash
# Apply all manifests
kubectl apply -f .

# Or use kustomize
kubectl apply -k .
```

### 4. Verify Deployment

```bash
# Check pod status
kubectl get pods -n sanad

# Check service status
kubectl get svc -n sanad

# Check ingress
kubectl get ingress -n sanad

# View logs
kubectl logs -f deployment/api-integration-service -n sanad
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Internet                             │
└────────────────────────┬────────────────────────────────────┘
                         │
                    ┌────▼────┐
                    │ Ingress │ (TLS, Rate Limiting)
                    └────┬────┘
                         │
              ┌──────────▼──────────┐
              │  Load Balancer      │
              │  (Service)          │
              └──────────┬──────────┘
                         │
         ┌───────────────┼───────────────┐
         │               │               │
    ┌────▼────┐    ┌────▼────┐    ┌────▼────┐
    │  Pod 1  │    │  Pod 2  │    │  Pod 3  │
    │  (API)  │    │  (API)  │    │  (API)  │
    └────┬────┘    └────┬────┘    └────┬────┘
         │               │               │
         └───────────────┼───────────────┘
                         │
              ┌──────────┴──────────┐
              │                     │
         ┌────▼────┐          ┌────▼────┐
         │  Redis  │          │Postgres │
         │ (Cache) │          │  (DB)   │
         └─────────┘          └─────────┘
```

## Manifests

### Core Resources

| File | Description |
|------|-------------|
| `namespace.yaml` | Namespace definition |
| `serviceaccount.yaml` | Service account and RBAC |
| `configmap.yaml` | Configuration data |
| `secret.yaml` | Sensitive data (API keys) |
| `deployment.yaml` | Main application deployment |
| `service.yaml` | Service definitions |

### Scaling & Availability

| File | Description |
|------|-------------|
| `hpa.yaml` | Horizontal Pod Autoscaler |
| `pdb.yaml` | Pod Disruption Budget |

### Networking

| File | Description |
|------|-------------|
| `ingress.yaml` | External access configuration |
| `networkpolicy.yaml` | Network security policies |

### Dependencies

| File | Description |
|------|-------------|
| `redis-deployment.yaml` | Redis cache deployment |
| `postgres-deployment.yaml` | PostgreSQL database |

### Monitoring

| File | Description |
|------|-------------|
| `servicemonitor.yaml` | Prometheus monitoring |

### Kustomize

| File | Description |
|------|-------------|
| `kustomization.yaml` | Base kustomize configuration |
| `overlays/development/` | Development environment |
| `overlays/production/` | Production environment |

## Configuration

### Environment Variables

Set via ConfigMap (`configmap.yaml`):

```yaml
SERVICE_PORT: "8080"
SERVICE_HOST: "0.0.0.0"
ENVIRONMENT: "production"
RUST_LOG: "info"
REDIS_URL: "redis://redis-service:6379"
DATABASE_URL: "postgresql://..."
```

### Secrets

Set via Secret (`secret.yaml` or kubectl):

```yaml
POSTGRES_PASSWORD: <base64-encoded>
QURAN_COM_API_KEY: <base64-encoded>
SUNNAH_COM_API_KEY: <base64-encoded>
ISLAMIC_FINDER_API_KEY: <base64-encoded>
HUGGING_FACE_API_KEY: <base64-encoded>
```

### API Configuration

Detailed API configuration in ConfigMap:

```yaml
apis:
  quran:
    - name: quran.com
      base_url: https://api.quran.com/api/v4
      priority: 1
      rate_limit:
        per_minute: 60
        per_hour: 1000
        per_day: 10000
```

## Deployment

### Using kubectl

```bash
# Deploy to default namespace
kubectl apply -f .

# Deploy to specific namespace
kubectl apply -f . -n sanad

# Deploy specific resource
kubectl apply -f deployment.yaml
```

### Using Kustomize

```bash
# Deploy base configuration
kubectl apply -k .

# Deploy development environment
kubectl apply -k overlays/development/

# Deploy production environment
kubectl apply -k overlays/production/
```

### Using Helm (if packaged)

```bash
# Install
helm install api-integration-service ./helm-chart

# Upgrade
helm upgrade api-integration-service ./helm-chart

# Uninstall
helm uninstall api-integration-service
```

### Rolling Updates

```bash
# Update image
kubectl set image deployment/api-integration-service \
  api-integration-service=sanad/api-integration-service:v1.1.0 \
  -n sanad

# Check rollout status
kubectl rollout status deployment/api-integration-service -n sanad

# Rollback if needed
kubectl rollout undo deployment/api-integration-service -n sanad
```

## Monitoring

### Health Checks

```bash
# Check pod health
kubectl get pods -n sanad

# Check service endpoints
kubectl get endpoints -n sanad

# Test health endpoint
kubectl port-forward svc/api-integration-service 8080:8080 -n sanad
curl http://localhost:8080/health
```

### Logs

```bash
# View logs from all pods
kubectl logs -l app=api-integration-service -n sanad

# Follow logs
kubectl logs -f deployment/api-integration-service -n sanad

# View logs from specific pod
kubectl logs api-integration-service-xxxxx -n sanad

# View previous container logs (after crash)
kubectl logs api-integration-service-xxxxx -n sanad --previous
```

### Metrics

```bash
# View resource usage
kubectl top pods -n sanad
kubectl top nodes

# View HPA status
kubectl get hpa -n sanad

# Describe HPA for details
kubectl describe hpa api-integration-service -n sanad
```

### Prometheus

If Prometheus Operator is installed:

```bash
# Check ServiceMonitor
kubectl get servicemonitor -n sanad

# Check PrometheusRule
kubectl get prometheusrule -n sanad

# Access Prometheus UI
kubectl port-forward -n monitoring svc/prometheus 9090:9090
```

## Scaling

### Manual Scaling

```bash
# Scale to 5 replicas
kubectl scale deployment api-integration-service --replicas=5 -n sanad

# Verify scaling
kubectl get pods -n sanad -w
```

### Horizontal Pod Autoscaler

HPA automatically scales based on:
- CPU utilization (target: 70%)
- Memory utilization (target: 80%)

```bash
# View HPA status
kubectl get hpa -n sanad

# Describe HPA
kubectl describe hpa api-integration-service -n sanad

# Edit HPA
kubectl edit hpa api-integration-service -n sanad
```

### Vertical Scaling

Update resource limits in `deployment.yaml`:

```yaml
resources:
  requests:
    cpu: 500m
    memory: 512Mi
  limits:
    cpu: 2000m
    memory: 1Gi
```

Then apply:

```bash
kubectl apply -f deployment.yaml
```

## Security

### RBAC

Service account with minimal permissions:
- Read ConfigMaps
- Read Secrets
- Read own Pod information

### Network Policies

Network policies restrict:
- Ingress: Only from ingress controller and monitoring
- Egress: Only to Redis, PostgreSQL, DNS, and external APIs

### Pod Security

- Runs as non-root user (UID 1000)
- Read-only root filesystem (where possible)
- Drops all capabilities
- No privilege escalation

### Secrets Management

**Best Practices:**

1. **Never commit secrets to Git**
2. **Use external secrets management:**
   - Sealed Secrets
   - External Secrets Operator
   - HashiCorp Vault
   - Cloud provider secrets (AWS Secrets Manager, Azure Key Vault, GCP Secret Manager)

3. **Rotate secrets regularly**

```bash
# Update secret
kubectl create secret generic api-integration-secrets \
  --from-literal=QURAN_COM_API_KEY='new_key' \
  --namespace=sanad \
  --dry-run=client -o yaml | kubectl apply -f -

# Restart pods to pick up new secret
kubectl rollout restart deployment/api-integration-service -n sanad
```

### TLS/SSL

Configure TLS in `ingress.yaml`:

```yaml
tls:
  - hosts:
      - api.sanad.example.com
    secretName: api-integration-tls
```

Create TLS secret:

```bash
kubectl create secret tls api-integration-tls \
  --cert=path/to/cert.pem \
  --key=path/to/key.pem \
  -n sanad
```

Or use cert-manager for automatic certificate management.

## Troubleshooting

### Pod Not Starting

```bash
# Check pod status
kubectl describe pod api-integration-service-xxxxx -n sanad

# Check events
kubectl get events -n sanad --sort-by='.lastTimestamp'

# Check logs
kubectl logs api-integration-service-xxxxx -n sanad
```

Common issues:
- Missing secrets
- Invalid configuration
- Cannot connect to Redis/PostgreSQL
- Insufficient resources

### Service Not Accessible

```bash
# Check service
kubectl describe svc api-integration-service -n sanad

# Check endpoints
kubectl get endpoints api-integration-service -n sanad

# Test from within cluster
kubectl run -it --rm debug --image=busybox --restart=Never -n sanad -- \
  wget -O- http://api-integration-service:8080/health
```

### High Memory/CPU Usage

```bash
# Check resource usage
kubectl top pods -n sanad

# Check HPA
kubectl get hpa -n sanad

# Increase resources
kubectl edit deployment api-integration-service -n sanad
```

### Database Connection Issues

```bash
# Check PostgreSQL pod
kubectl get pods -l app=postgres -n sanad

# Test connection
kubectl exec -it postgres-0 -n sanad -- psql -U sanad -d sanad

# Check secret
kubectl get secret api-integration-secrets -n sanad -o yaml
```

### Redis Connection Issues

```bash
# Check Redis pod
kubectl get pods -l app=redis -n sanad

# Test connection
kubectl exec -it redis-xxxxx -n sanad -- redis-cli ping

# Check service
kubectl get svc redis-service -n sanad
```

## Best Practices

### 1. Use Specific Image Tags

❌ Don't use `latest`:
```yaml
image: sanad/api-integration-service:latest
```

✅ Use specific versions:
```yaml
image: sanad/api-integration-service:v1.0.0
```

### 2. Set Resource Limits

Always set both requests and limits:

```yaml
resources:
  requests:
    cpu: 250m
    memory: 256Mi
  limits:
    cpu: 1000m
    memory: 512Mi
```

### 3. Use Health Checks

Configure all three probes:
- Liveness: Is the container alive?
- Readiness: Is the container ready for traffic?
- Startup: Give extra time for initial startup

### 4. Enable Monitoring

- Deploy ServiceMonitor for Prometheus
- Set up alerts for critical metrics
- Monitor logs centrally

### 5. Implement High Availability

- Run multiple replicas (3+)
- Use Pod Disruption Budget
- Spread pods across nodes (anti-affinity)

### 6. Secure Your Deployment

- Use RBAC with minimal permissions
- Enable Network Policies
- Run as non-root user
- Use secrets management
- Enable TLS

### 7. Plan for Disaster Recovery

- Backup PostgreSQL data regularly
- Document recovery procedures
- Test disaster recovery

### 8. Use GitOps

- Store manifests in Git
- Use tools like ArgoCD or Flux
- Automate deployments

## Additional Resources

- [Main README](../README.md)
- [Docker Deployment Guide](../DOCKER_DEPLOYMENT.md)
- [API Documentation](../docs/API_DOCUMENTATION.md)
- [Configuration Guide](../../../config/CONFIGURATION_GUIDE.md)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Kustomize Documentation](https://kustomize.io/)

## Support

For issues or questions:
1. Check the troubleshooting section
2. Review logs and events
3. Consult the main documentation
4. Open an issue on GitHub

## License

See the main project LICENSE file.

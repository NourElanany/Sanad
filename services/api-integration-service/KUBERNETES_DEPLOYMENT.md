# Kubernetes Deployment Guide - API Integration Service

## Overview

This document provides a comprehensive guide for deploying the API Integration Service to Kubernetes clusters. The deployment includes production-ready configurations with high availability, auto-scaling, monitoring, and security best practices.

## What's Included

### Core Components

1. **API Integration Service**
   - 3+ replicas for high availability
   - Horizontal Pod Autoscaler (HPA) for automatic scaling
   - Pod Disruption Budget (PDB) for availability guarantees
   - Health checks (liveness, readiness, startup probes)
   - Resource limits and requests

2. **Redis Cache**
   - Single replica deployment
   - Persistent storage (5Gi)
   - LRU eviction policy
   - Health checks

3. **PostgreSQL Database**
   - StatefulSet deployment
   - Persistent storage (10Gi)
   - Initialization scripts support
   - Health checks

### Networking

1. **Services**
   - ClusterIP service for internal access
   - Headless service for direct pod access
   - Load balancer integration

2. **Ingress**
   - TLS/SSL termination
   - Rate limiting
   - CORS configuration
   - Security headers
   - Multiple ingress options (external/internal)

3. **Network Policies**
   - Ingress rules (what can connect to the service)
   - Egress rules (what the service can connect to)
   - DNS, Redis, PostgreSQL, and external API access

### Security

1. **RBAC**
   - Service account with minimal permissions
   - Role for ConfigMap and Secret access
   - RoleBinding

2. **Pod Security**
   - Runs as non-root user (UID 1000)
   - Read-only root filesystem (where applicable)
   - Drops all capabilities
   - No privilege escalation
   - Security context at pod and container level

3. **Secrets Management**
   - Kubernetes Secrets for API keys
   - PostgreSQL password
   - Support for external secrets management

### Monitoring & Observability

1. **Prometheus Integration**
   - ServiceMonitor for metrics scraping
   - PrometheusRule for alerting
   - Pre-configured alerts:
     - High error rate
     - API unavailable
     - High response time
     - High memory/CPU usage
     - Cache miss rate
     - External API failures
     - Pod restarts

2. **Logging**
   - Structured logging with correlation IDs
   - Log aggregation support
   - Pod logs accessible via kubectl

### Configuration Management

1. **ConfigMap**
   - Service configuration
   - API endpoints and rate limits
   - Cache strategies
   - Health monitor settings
   - Retry configuration

2. **Secrets**
   - Database credentials
   - API keys for external services
   - TLS certificates

### Scaling & Availability

1. **Horizontal Pod Autoscaler (HPA)**
   - Min replicas: 3
   - Max replicas: 10
   - CPU target: 70%
   - Memory target: 80%
   - Scale-up/down policies

2. **Pod Disruption Budget (PDB)**
   - Minimum available: 2 pods
   - Ensures availability during updates

3. **Pod Anti-Affinity**
   - Spreads pods across nodes
   - Improves fault tolerance

## Directory Structure

```
k8s/
├── README.md                      # Detailed documentation
├── deploy.sh                      # Deployment script
├── undeploy.sh                    # Undeployment script
├── namespace.yaml                 # Namespace definition
├── serviceaccount.yaml            # RBAC configuration
├── configmap.yaml                 # Configuration data
├── secret.yaml                    # Secrets template
├── deployment.yaml                # Main application deployment
├── service.yaml                   # Service definitions
├── hpa.yaml                       # Horizontal Pod Autoscaler
├── pdb.yaml                       # Pod Disruption Budget
├── ingress.yaml                   # Ingress configuration
├── networkpolicy.yaml             # Network policies
├── redis-deployment.yaml          # Redis cache
├── postgres-deployment.yaml       # PostgreSQL database
├── servicemonitor.yaml            # Prometheus monitoring
├── kustomization.yaml             # Kustomize base
└── overlays/
    ├── development/               # Development environment
    │   ├── kustomization.yaml
    │   ├── deployment-patch.yaml
    │   └── configmap-patch.yaml
    └── production/                # Production environment
        ├── kustomization.yaml
        ├── deployment-patch.yaml
        └── hpa-patch.yaml
```

## Quick Start

### Prerequisites

- Kubernetes cluster (v1.24+)
- kubectl CLI configured
- Sufficient cluster resources (2+ CPU cores, 4+ GB RAM, 20+ GB storage)

### 1. Create Secrets

**IMPORTANT**: Replace with actual API keys!

```bash
kubectl create secret generic api-integration-secrets \
  --from-literal=POSTGRES_PASSWORD='your_secure_password' \
  --from-literal=QURAN_COM_API_KEY='your_actual_key' \
  --from-literal=SUNNAH_COM_API_KEY='your_actual_key' \
  --from-literal=ISLAMIC_FINDER_API_KEY='your_actual_key' \
  --from-literal=HUGGING_FACE_API_KEY='your_actual_key' \
  --namespace=sanad
```

### 2. Deploy Using Script

```bash
cd services/api-integration-service/k8s

# Make script executable (Linux/Mac)
chmod +x deploy.sh

# Deploy to production
./deploy.sh

# Or deploy to development
ENVIRONMENT=development ./deploy.sh
```

### 3. Deploy Manually

```bash
# Create namespace
kubectl apply -f namespace.yaml

# Create secrets (see step 1)

# Deploy all resources
kubectl apply -k .

# Or deploy specific environment
kubectl apply -k overlays/production/
```

### 4. Verify Deployment

```bash
# Check pods
kubectl get pods -n sanad

# Check services
kubectl get svc -n sanad

# View logs
kubectl logs -f deployment/api-integration-service -n sanad

# Test health endpoint
kubectl port-forward svc/api-integration-service 8080:8080 -n sanad
curl http://localhost:8080/health
```

## Environment-Specific Deployments

### Development Environment

- 1 replica
- Lower resource limits (100m CPU, 128Mi memory)
- Debug logging
- Faster startup

```bash
kubectl apply -k overlays/development/
```

### Production Environment

- 5 replicas (min)
- Higher resource limits (500m-2000m CPU, 512Mi-1Gi memory)
- Info logging
- Stricter security
- More aggressive auto-scaling

```bash
kubectl apply -k overlays/production/
```

## Configuration

### Updating Configuration

1. **Edit ConfigMap**:
   ```bash
   kubectl edit configmap api-integration-config -n sanad
   ```

2. **Restart pods to pick up changes**:
   ```bash
   kubectl rollout restart deployment/api-integration-service -n sanad
   ```

### Updating Secrets

1. **Update secret**:
   ```bash
   kubectl create secret generic api-integration-secrets \
     --from-literal=QURAN_COM_API_KEY='new_key' \
     --namespace=sanad \
     --dry-run=client -o yaml | kubectl apply -f -
   ```

2. **Restart pods**:
   ```bash
   kubectl rollout restart deployment/api-integration-service -n sanad
   ```

## Scaling

### Manual Scaling

```bash
# Scale to 5 replicas
kubectl scale deployment api-integration-service --replicas=5 -n sanad
```

### Auto-Scaling

HPA automatically scales based on CPU and memory:

```bash
# View HPA status
kubectl get hpa -n sanad

# Edit HPA
kubectl edit hpa api-integration-service -n sanad
```

## Monitoring

### View Metrics

```bash
# Resource usage
kubectl top pods -n sanad
kubectl top nodes

# HPA status
kubectl describe hpa api-integration-service -n sanad
```

### View Logs

```bash
# All pods
kubectl logs -l app=api-integration-service -n sanad

# Follow logs
kubectl logs -f deployment/api-integration-service -n sanad

# Specific pod
kubectl logs api-integration-service-xxxxx -n sanad
```

### Prometheus Alerts

If Prometheus Operator is installed, alerts are automatically configured for:
- High error rate (>5%)
- Service unavailable
- High response time (>1s)
- High memory/CPU usage
- Cache miss rate
- External API failures
- Pod restarts

## Troubleshooting

### Common Issues

1. **Pods not starting**:
   ```bash
   kubectl describe pod api-integration-service-xxxxx -n sanad
   kubectl logs api-integration-service-xxxxx -n sanad
   ```

2. **Service not accessible**:
   ```bash
   kubectl get endpoints api-integration-service -n sanad
   kubectl describe svc api-integration-service -n sanad
   ```

3. **Database connection issues**:
   ```bash
   kubectl get pods -l app=postgres -n sanad
   kubectl logs postgres-0 -n sanad
   ```

4. **Redis connection issues**:
   ```bash
   kubectl get pods -l app=redis -n sanad
   kubectl exec -it redis-xxxxx -n sanad -- redis-cli ping
   ```

### Debug Commands

```bash
# Get events
kubectl get events -n sanad --sort-by='.lastTimestamp'

# Describe deployment
kubectl describe deployment api-integration-service -n sanad

# Execute command in pod
kubectl exec -it api-integration-service-xxxxx -n sanad -- /bin/bash

# Port forward for local testing
kubectl port-forward svc/api-integration-service 8080:8080 -n sanad
```

## Undeployment

### Using Script

```bash
# Remove application only
./undeploy.sh

# Remove application and PVCs (DATA LOSS!)
DELETE_PVC=true ./undeploy.sh

# Remove everything including namespace
DELETE_NAMESPACE=true DELETE_PVC=true ./undeploy.sh
```

### Manual Undeployment

```bash
# Delete application
kubectl delete -k .

# Delete namespace (removes everything)
kubectl delete namespace sanad
```

## Best Practices

1. **Use specific image tags** (not `latest`)
2. **Set resource limits** for all containers
3. **Enable all health checks** (liveness, readiness, startup)
4. **Use secrets management** (never commit secrets)
5. **Enable monitoring** (Prometheus, logs)
6. **Implement high availability** (multiple replicas, PDB)
7. **Use network policies** for security
8. **Regular backups** of PostgreSQL data
9. **Test disaster recovery** procedures
10. **Use GitOps** for deployment automation

## Production Checklist

Before deploying to production:

- [ ] Replace all placeholder secrets with actual values
- [ ] Configure TLS certificates for Ingress
- [ ] Set up external secrets management (Vault, AWS Secrets Manager, etc.)
- [ ] Configure backup strategy for PostgreSQL
- [ ] Set up monitoring and alerting (Prometheus, Grafana)
- [ ] Configure log aggregation (ELK, Loki, etc.)
- [ ] Test disaster recovery procedures
- [ ] Review and adjust resource limits based on load testing
- [ ] Configure network policies for your environment
- [ ] Set up CI/CD pipeline for automated deployments
- [ ] Document runbooks for common operations
- [ ] Train team on Kubernetes operations

## Additional Resources

- [Kubernetes README](./k8s/README.md) - Detailed documentation
- [Docker Deployment Guide](./DOCKER_DEPLOYMENT.md)
- [API Documentation](./docs/API_DOCUMENTATION.md)
- [Configuration Guide](../../config/CONFIGURATION_GUIDE.md)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Kustomize Documentation](https://kustomize.io/)

## Support

For issues or questions:
1. Check the troubleshooting section
2. Review logs and events
3. Consult the detailed README in k8s/
4. Open an issue on GitHub

## License

See the main project LICENSE file.

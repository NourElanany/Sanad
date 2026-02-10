#!/bin/bash

# Kubernetes Deployment Script for API Integration Service
# This script deploys the API Integration Service to a Kubernetes cluster

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
NAMESPACE="${NAMESPACE:-sanad}"
ENVIRONMENT="${ENVIRONMENT:-production}"
KUBECTL="${KUBECTL:-kubectl}"

# Functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check kubectl
    if ! command -v $KUBECTL &> /dev/null; then
        log_error "kubectl not found. Please install kubectl."
        exit 1
    fi
    
    # Check cluster connection
    if ! $KUBECTL cluster-info &> /dev/null; then
        log_error "Cannot connect to Kubernetes cluster. Please check your kubeconfig."
        exit 1
    fi
    
    # Check kustomize (optional)
    if command -v kustomize &> /dev/null; then
        log_info "kustomize found: $(kustomize version --short)"
    else
        log_warn "kustomize not found. Using kubectl apply -k instead."
    fi
    
    log_info "Prerequisites check passed!"
}

create_namespace() {
    log_info "Creating namespace: $NAMESPACE"
    
    if $KUBECTL get namespace $NAMESPACE &> /dev/null; then
        log_warn "Namespace $NAMESPACE already exists"
    else
        $KUBECTL apply -f namespace.yaml
        log_info "Namespace created successfully"
    fi
}

check_secrets() {
    log_info "Checking secrets..."
    
    if $KUBECTL get secret api-integration-secrets -n $NAMESPACE &> /dev/null; then
        log_warn "Secret api-integration-secrets already exists"
        read -p "Do you want to update the secret? (y/N) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            create_secrets
        fi
    else
        log_warn "Secret api-integration-secrets not found"
        create_secrets
    fi
}

create_secrets() {
    log_info "Creating secrets..."
    
    # Prompt for secrets
    read -sp "Enter PostgreSQL password: " POSTGRES_PASSWORD
    echo
    read -sp "Enter Quran.com API key: " QURAN_COM_API_KEY
    echo
    read -sp "Enter Sunnah.com API key: " SUNNAH_COM_API_KEY
    echo
    read -sp "Enter Islamic Finder API key: " ISLAMIC_FINDER_API_KEY
    echo
    read -sp "Enter Hugging Face API key: " HUGGING_FACE_API_KEY
    echo
    read -sp "Enter OpenAI API key (optional): " OPENAI_API_KEY
    echo
    
    # Create secret
    $KUBECTL create secret generic api-integration-secrets \
        --from-literal=POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
        --from-literal=QURAN_COM_API_KEY="$QURAN_COM_API_KEY" \
        --from-literal=SUNNAH_COM_API_KEY="$SUNNAH_COM_API_KEY" \
        --from-literal=ISLAMIC_FINDER_API_KEY="$ISLAMIC_FINDER_API_KEY" \
        --from-literal=HUGGING_FACE_API_KEY="$HUGGING_FACE_API_KEY" \
        --from-literal=OPENAI_API_KEY="$OPENAI_API_KEY" \
        --namespace=$NAMESPACE \
        --dry-run=client -o yaml | $KUBECTL apply -f -
    
    log_info "Secrets created successfully"
}

deploy_dependencies() {
    log_info "Deploying dependencies (Redis, PostgreSQL)..."
    
    $KUBECTL apply -f redis-deployment.yaml -n $NAMESPACE
    $KUBECTL apply -f postgres-deployment.yaml -n $NAMESPACE
    
    log_info "Waiting for dependencies to be ready..."
    $KUBECTL wait --for=condition=ready pod -l app=redis -n $NAMESPACE --timeout=300s
    $KUBECTL wait --for=condition=ready pod -l app=postgres -n $NAMESPACE --timeout=300s
    
    log_info "Dependencies deployed successfully"
}

deploy_application() {
    log_info "Deploying API Integration Service..."
    
    if [ "$ENVIRONMENT" == "development" ]; then
        log_info "Deploying development environment..."
        $KUBECTL apply -k overlays/development/
    elif [ "$ENVIRONMENT" == "production" ]; then
        log_info "Deploying production environment..."
        $KUBECTL apply -k overlays/production/
    else
        log_info "Deploying base configuration..."
        $KUBECTL apply -k .
    fi
    
    log_info "Waiting for deployment to be ready..."
    $KUBECTL wait --for=condition=available deployment/api-integration-service -n $NAMESPACE --timeout=300s
    
    log_info "Application deployed successfully"
}

verify_deployment() {
    log_info "Verifying deployment..."
    
    # Check pods
    log_info "Pods:"
    $KUBECTL get pods -n $NAMESPACE
    
    # Check services
    log_info "Services:"
    $KUBECTL get svc -n $NAMESPACE
    
    # Check ingress
    log_info "Ingress:"
    $KUBECTL get ingress -n $NAMESPACE
    
    # Check HPA
    log_info "HPA:"
    $KUBECTL get hpa -n $NAMESPACE
    
    # Test health endpoint
    log_info "Testing health endpoint..."
    POD_NAME=$($KUBECTL get pods -n $NAMESPACE -l app=api-integration-service -o jsonpath='{.items[0].metadata.name}')
    if $KUBECTL exec -n $NAMESPACE $POD_NAME -- curl -f http://localhost:8080/health &> /dev/null; then
        log_info "Health check passed!"
    else
        log_warn "Health check failed. Check logs for details."
    fi
}

show_access_info() {
    log_info "Deployment complete!"
    echo
    log_info "Access information:"
    
    # Get service info
    SERVICE_IP=$($KUBECTL get svc api-integration-service -n $NAMESPACE -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>/dev/null || echo "pending")
    if [ "$SERVICE_IP" != "pending" ]; then
        echo "  Service IP: $SERVICE_IP"
    fi
    
    # Get ingress info
    INGRESS_HOST=$($KUBECTL get ingress api-integration-service -n $NAMESPACE -o jsonpath='{.spec.rules[0].host}' 2>/dev/null || echo "not configured")
    if [ "$INGRESS_HOST" != "not configured" ]; then
        echo "  Ingress Host: https://$INGRESS_HOST"
    fi
    
    echo
    log_info "Useful commands:"
    echo "  View logs:    $KUBECTL logs -f deployment/api-integration-service -n $NAMESPACE"
    echo "  View pods:    $KUBECTL get pods -n $NAMESPACE"
    echo "  Port forward: $KUBECTL port-forward svc/api-integration-service 8080:8080 -n $NAMESPACE"
    echo "  Scale:        $KUBECTL scale deployment api-integration-service --replicas=5 -n $NAMESPACE"
}

# Main execution
main() {
    log_info "Starting deployment of API Integration Service"
    log_info "Environment: $ENVIRONMENT"
    log_info "Namespace: $NAMESPACE"
    echo
    
    check_prerequisites
    create_namespace
    check_secrets
    deploy_dependencies
    deploy_application
    verify_deployment
    show_access_info
    
    log_info "Deployment completed successfully!"
}

# Run main function
main "$@"

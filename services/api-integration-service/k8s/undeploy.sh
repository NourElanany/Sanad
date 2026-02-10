#!/bin/bash

# Kubernetes Undeployment Script for API Integration Service
# This script removes the API Integration Service from a Kubernetes cluster

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
NAMESPACE="${NAMESPACE:-sanad}"
KUBECTL="${KUBECTL:-kubectl}"
DELETE_NAMESPACE="${DELETE_NAMESPACE:-false}"
DELETE_PVC="${DELETE_PVC:-false}"

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

confirm_deletion() {
    log_warn "This will delete the API Integration Service and its dependencies from namespace: $NAMESPACE"
    
    if [ "$DELETE_PVC" == "true" ]; then
        log_warn "PersistentVolumeClaims will also be deleted (DATA LOSS!)"
    fi
    
    if [ "$DELETE_NAMESPACE" == "true" ]; then
        log_warn "The entire namespace will be deleted"
    fi
    
    read -p "Are you sure you want to continue? (yes/NO) " -r
    echo
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        log_info "Deletion cancelled"
        exit 0
    fi
}

delete_application() {
    log_info "Deleting API Integration Service..."
    
    # Delete deployment
    $KUBECTL delete deployment api-integration-service -n $NAMESPACE --ignore-not-found=true
    
    # Delete service
    $KUBECTL delete service api-integration-service -n $NAMESPACE --ignore-not-found=true
    $KUBECTL delete service api-integration-service-headless -n $NAMESPACE --ignore-not-found=true
    
    # Delete HPA
    $KUBECTL delete hpa api-integration-service -n $NAMESPACE --ignore-not-found=true
    
    # Delete PDB
    $KUBECTL delete pdb api-integration-service -n $NAMESPACE --ignore-not-found=true
    
    # Delete ingress
    $KUBECTL delete ingress api-integration-service -n $NAMESPACE --ignore-not-found=true
    $KUBECTL delete ingress api-integration-service-internal -n $NAMESPACE --ignore-not-found=true
    
    # Delete network policy
    $KUBECTL delete networkpolicy api-integration-service -n $NAMESPACE --ignore-not-found=true
    
    # Delete service monitor
    $KUBECTL delete servicemonitor api-integration-service -n $NAMESPACE --ignore-not-found=true
    $KUBECTL delete prometheusrule api-integration-service -n $NAMESPACE --ignore-not-found=true
    
    log_info "Application deleted"
}

delete_dependencies() {
    log_info "Deleting dependencies..."
    
    # Delete Redis
    $KUBECTL delete deployment redis -n $NAMESPACE --ignore-not-found=true
    $KUBECTL delete service redis-service -n $NAMESPACE --ignore-not-found=true
    
    # Delete PostgreSQL
    $KUBECTL delete statefulset postgres -n $NAMESPACE --ignore-not-found=true
    $KUBECTL delete service postgres-service -n $NAMESPACE --ignore-not-found=true
    
    log_info "Dependencies deleted"
}

delete_config() {
    log_info "Deleting configuration..."
    
    # Delete ConfigMap
    $KUBECTL delete configmap api-integration-config -n $NAMESPACE --ignore-not-found=true
    $KUBECTL delete configmap postgres-init-scripts -n $NAMESPACE --ignore-not-found=true
    
    # Delete ServiceAccount and RBAC
    $KUBECTL delete serviceaccount api-integration-service -n $NAMESPACE --ignore-not-found=true
    $KUBECTL delete role api-integration-service -n $NAMESPACE --ignore-not-found=true
    $KUBECTL delete rolebinding api-integration-service -n $NAMESPACE --ignore-not-found=true
    
    log_info "Configuration deleted"
}

delete_secrets() {
    log_warn "Deleting secrets..."
    
    read -p "Do you want to delete secrets? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        $KUBECTL delete secret api-integration-secrets -n $NAMESPACE --ignore-not-found=true
        log_info "Secrets deleted"
    else
        log_info "Secrets preserved"
    fi
}

delete_pvcs() {
    if [ "$DELETE_PVC" == "true" ]; then
        log_warn "Deleting PersistentVolumeClaims (DATA LOSS!)..."
        
        $KUBECTL delete pvc redis-pvc -n $NAMESPACE --ignore-not-found=true
        $KUBECTL delete pvc postgres-data-postgres-0 -n $NAMESPACE --ignore-not-found=true
        
        log_info "PVCs deleted"
    else
        log_info "PVCs preserved (use DELETE_PVC=true to delete)"
    fi
}

delete_namespace_if_requested() {
    if [ "$DELETE_NAMESPACE" == "true" ]; then
        log_warn "Deleting namespace: $NAMESPACE"
        $KUBECTL delete namespace $NAMESPACE --ignore-not-found=true
        log_info "Namespace deleted"
    else
        log_info "Namespace preserved (use DELETE_NAMESPACE=true to delete)"
    fi
}

verify_deletion() {
    log_info "Verifying deletion..."
    
    # Check if any resources remain
    REMAINING_PODS=$($KUBECTL get pods -n $NAMESPACE 2>/dev/null | grep -c "api-integration-service" || true)
    
    if [ "$REMAINING_PODS" -eq 0 ]; then
        log_info "All resources deleted successfully"
    else
        log_warn "Some resources may still be terminating"
        $KUBECTL get pods -n $NAMESPACE
    fi
}

# Main execution
main() {
    log_info "Starting undeployment of API Integration Service"
    log_info "Namespace: $NAMESPACE"
    echo
    
    confirm_deletion
    delete_application
    delete_dependencies
    delete_config
    delete_secrets
    delete_pvcs
    delete_namespace_if_requested
    verify_deletion
    
    log_info "Undeployment completed!"
}

# Run main function
main "$@"

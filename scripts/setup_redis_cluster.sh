#!/bin/bash

# Script to set up and test Redis cluster for Sanad Islamic Application
# This script helps with Redis cluster setup and validation

set -e

echo "🚀 Setting up Redis Cluster for Sanad Islamic Application"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Docker is running
check_docker() {
    print_status "Checking Docker status..."
    if ! docker info > /dev/null 2>&1; then
        print_error "Docker is not running. Please start Docker and try again."
        exit 1
    fi
    print_success "Docker is running"
}

# Check if docker-compose is available
check_docker_compose() {
    print_status "Checking docker-compose availability..."
    if ! command -v docker-compose &> /dev/null; then
        print_error "docker-compose is not installed. Please install docker-compose and try again."
        exit 1
    fi
    print_success "docker-compose is available"
}

# Start Redis cluster services
start_redis_cluster() {
    print_status "Starting Redis cluster services..."
    
    # Start individual Redis nodes
    docker-compose up -d redis-node-1 redis-node-2 redis-node-3
    
    # Wait for nodes to be ready
    print_status "Waiting for Redis nodes to be ready..."
    sleep 10
    
    # Check if nodes are healthy
    for i in {1..3}; do
        port=$((7000 + i))
        if docker-compose exec redis-node-$i redis-cli -p $port ping | grep -q "PONG"; then
            print_success "Redis node $i is ready on port $port"
        else
            print_error "Redis node $i failed to start on port $port"
            exit 1
        fi
    done
}

# Create Redis cluster
create_cluster() {
    print_status "Creating Redis cluster..."
    
    # Run cluster setup
    docker-compose up redis-cluster-setup
    
    # Verify cluster status
    print_status "Verifying cluster status..."
    if docker-compose exec redis-node-1 redis-cli -p 7001 cluster info | grep -q "cluster_state:ok"; then
        print_success "Redis cluster created successfully"
    else
        print_warning "Cluster might not be fully ready. Checking individual nodes..."
        
        # Check each node
        for i in {1..3}; do
            port=$((7000 + i))
            node_info=$(docker-compose exec redis-node-$i redis-cli -p $port cluster nodes 2>/dev/null || echo "failed")
            if [[ "$node_info" != "failed" ]]; then
                print_success "Node $i cluster info retrieved"
            else
                print_error "Failed to get cluster info from node $i"
            fi
        done
    fi
}

# Test Redis cluster functionality
test_cluster() {
    print_status "Testing Redis cluster functionality..."
    
    # Test basic operations
    print_status "Testing SET operation..."
    if docker-compose exec redis-node-1 redis-cli -p 7001 -c set test_key "Hello Redis Cluster" | grep -q "OK"; then
        print_success "SET operation successful"
    else
        print_error "SET operation failed"
        return 1
    fi
    
    print_status "Testing GET operation..."
    result=$(docker-compose exec redis-node-1 redis-cli -p 7001 -c get test_key 2>/dev/null || echo "failed")
    if [[ "$result" == *"Hello Redis Cluster"* ]]; then
        print_success "GET operation successful"
    else
        print_error "GET operation failed. Got: $result"
        return 1
    fi
    
    # Test cluster-specific operations
    print_status "Testing cluster slots distribution..."
    slots_info=$(docker-compose exec redis-node-1 redis-cli -p 7001 cluster slots 2>/dev/null || echo "failed")
    if [[ "$slots_info" != "failed" ]] && [[ -n "$slots_info" ]]; then
        print_success "Cluster slots are properly distributed"
    else
        print_warning "Could not verify cluster slots distribution"
    fi
    
    # Cleanup test data
    docker-compose exec redis-node-1 redis-cli -p 7001 -c del test_key > /dev/null 2>&1 || true
}

# Test cache service integration
test_cache_service() {
    print_status "Testing cache service integration..."
    
    # Check if cache service is running
    if docker-compose ps cache-service | grep -q "Up"; then
        print_success "Cache service is running"
        
        # Test health endpoint
        print_status "Testing cache service health endpoint..."
        if curl -f http://localhost:8091/health > /dev/null 2>&1; then
            print_success "Cache service health check passed"
        else
            print_warning "Cache service health check failed (service might still be starting)"
        fi
    else
        print_warning "Cache service is not running. Start it with: docker-compose up -d cache-service"
    fi
}

# Monitor Redis cluster
monitor_cluster() {
    print_status "Redis Cluster Monitoring Information:"
    echo ""
    
    print_status "Cluster Nodes Status:"
    for i in {1..3}; do
        port=$((7000 + i))
        echo "Node $i (port $port):"
        docker-compose exec redis-node-$i redis-cli -p $port info replication | grep "role:" || echo "  Status: Not available"
        echo ""
    done
    
    print_status "Cluster Info:"
    docker-compose exec redis-node-1 redis-cli -p 7001 cluster info | head -10 || echo "Cluster info not available"
    
    print_status "Memory Usage:"
    for i in {1..3}; do
        port=$((7000 + i))
        memory=$(docker-compose exec redis-node-$i redis-cli -p $port info memory | grep "used_memory_human:" | cut -d: -f2 | tr -d '\r' || echo "N/A")
        echo "Node $i: $memory"
    done
}

# Cleanup function
cleanup_cluster() {
    print_status "Cleaning up Redis cluster..."
    
    docker-compose down redis-node-1 redis-node-2 redis-node-3 redis-cluster-setup
    
    # Remove volumes if requested
    if [[ "$1" == "--remove-volumes" ]]; then
        print_warning "Removing Redis cluster volumes..."
        docker volume rm $(docker volume ls -q | grep redis_node) 2>/dev/null || true
    fi
    
    print_success "Redis cluster cleanup completed"
}

# Performance test
performance_test() {
    print_status "Running Redis cluster performance test..."
    
    # Simple benchmark
    print_status "Running redis-benchmark on cluster..."
    docker-compose exec redis-node-1 redis-benchmark -h redis-node-1 -p 7001 -c 50 -n 10000 -d 3 -t set,get --csv | tail -5
    
    print_success "Performance test completed"
}

# Main script logic
case "${1:-setup}" in
    "setup")
        check_docker
        check_docker_compose
        start_redis_cluster
        create_cluster
        test_cluster
        test_cache_service
        print_success "Redis cluster setup completed successfully!"
        echo ""
        print_status "Next steps:"
        echo "  1. Start the cache service: docker-compose up -d cache-service"
        echo "  2. Monitor cluster: $0 monitor"
        echo "  3. Run performance test: $0 performance"
        ;;
    "test")
        test_cluster
        test_cache_service
        ;;
    "monitor")
        monitor_cluster
        ;;
    "performance")
        performance_test
        ;;
    "cleanup")
        cleanup_cluster "$2"
        ;;
    "help"|"--help"|"-h")
        echo "Redis Cluster Setup Script for Sanad Islamic Application"
        echo ""
        echo "Usage: $0 [command] [options]"
        echo ""
        echo "Commands:"
        echo "  setup      - Set up and start Redis cluster (default)"
        echo "  test       - Test cluster functionality"
        echo "  monitor    - Show cluster status and monitoring info"
        echo "  performance - Run performance benchmark"
        echo "  cleanup    - Stop and remove cluster containers"
        echo "               Use --remove-volumes to also remove data volumes"
        echo "  help       - Show this help message"
        echo ""
        echo "Examples:"
        echo "  $0                    # Set up cluster"
        echo "  $0 test              # Test cluster"
        echo "  $0 monitor           # Monitor cluster"
        echo "  $0 cleanup           # Clean up cluster"
        echo "  $0 cleanup --remove-volumes  # Clean up including data"
        ;;
    *)
        print_error "Unknown command: $1"
        echo "Use '$0 help' for usage information"
        exit 1
        ;;
esac
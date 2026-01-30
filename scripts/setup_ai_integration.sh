#!/bin/bash

# AI Service Integration Setup Script
# This script sets up the complete AI service integration for the Islamic application

set -e  # Exit on any error

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

print_header() {
    echo -e "\n${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE} $1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_header "Checking Prerequisites"
    
    local missing_deps=()
    
    # Check Docker
    if command_exists docker; then
        print_success "Docker is installed"
        docker --version
    else
        missing_deps+=("docker")
        print_error "Docker is not installed"
    fi
    
    # Check Docker Compose
    if command_exists docker-compose || docker compose version >/dev/null 2>&1; then
        print_success "Docker Compose is available"
    else
        missing_deps+=("docker-compose")
        print_error "Docker Compose is not available"
    fi
    
    # Check Rust
    if command_exists rustc; then
        print_success "Rust is installed"
        rustc --version
    else
        missing_deps+=("rust")
        print_error "Rust is not installed"
    fi
    
    # Check Cargo
    if command_exists cargo; then
        print_success "Cargo is available"
        cargo --version
    else
        missing_deps+=("cargo")
        print_error "Cargo is not available"
    fi
    
    # Check curl
    if command_exists curl; then
        print_success "curl is available"
    else
        missing_deps+=("curl")
        print_error "curl is not installed"
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Missing dependencies: ${missing_deps[*]}"
        print_status "Please install the missing dependencies and run this script again."
        exit 1
    fi
    
    print_success "All prerequisites are met!"
}

# Setup environment variables
setup_environment() {
    print_header "Setting Up Environment Variables"
    
    # Create .env file if it doesn't exist
    if [ ! -f .env ]; then
        print_status "Creating .env file..."
        cat > .env << EOF
# AI Service Configuration
HUGGING_FACE_API_KEY=your_api_key_here
QDRANT_HOST=localhost
QDRANT_PORT=6333
REDIS_URL=redis://localhost:6379
LOG_LEVEL=INFO
AI_SERVICE_CONFIG=config/ai_service_config.yaml

# Development settings
RUST_LOG=info
RUST_BACKTRACE=1
EOF
        print_success ".env file created"
    else
        print_status ".env file already exists"
    fi
    
    # Check if Hugging Face API key is set
    if [ -f .env ]; then
        source .env
    fi
    
    if [ -z "$HUGGING_FACE_API_KEY" ] || [ "$HUGGING_FACE_API_KEY" = "your_api_key_here" ]; then
        print_warning "Hugging Face API key is not set!"
        print_status "Please:"
        print_status "1. Get an API key from: https://huggingface.co/settings/tokens"
        print_status "2. Edit .env file and set HUGGING_FACE_API_KEY=your_actual_key"
        print_status "3. Re-run this script"
        
        read -p "Do you want to set the API key now? (y/n): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            read -p "Enter your Hugging Face API key: " api_key
            if [ ! -z "$api_key" ]; then
                sed -i.bak "s/HUGGING_FACE_API_KEY=your_api_key_here/HUGGING_FACE_API_KEY=$api_key/" .env
                print_success "API key updated in .env file"
            fi
        fi
    else
        print_success "Hugging Face API key is configured"
    fi
}

# Create required directories
create_directories() {
    print_header "Creating Required Directories"
    
    local dirs=(
        "config"
        "data"
        "data/qdrant"
        "data/redis"
        "logs"
        "monitoring"
        "monitoring/grafana/provisioning/dashboards"
        "monitoring/grafana/provisioning/datasources"
    )
    
    for dir in "${dirs[@]}"; do
        if [ ! -d "$dir" ]; then
            mkdir -p "$dir"
            print_success "Created directory: $dir"
        else
            print_status "Directory already exists: $dir"
        fi
    done
}

# Start Docker services
start_docker_services() {
    print_header "Starting Docker Services"
    
    print_status "Starting Qdrant and Redis..."
    
    # Use docker-compose or docker compose based on availability
    if command_exists docker-compose; then
        COMPOSE_CMD="docker-compose"
    else
        COMPOSE_CMD="docker compose"
    fi
    
    # Start core services
    $COMPOSE_CMD -f docker-compose.ai-services.yml up -d qdrant redis
    
    print_status "Waiting for services to be ready..."
    
    # Wait for Qdrant
    print_status "Waiting for Qdrant to be ready..."
    for i in {1..30}; do
        if curl -s http://localhost:6333/ >/dev/null 2>&1; then
            print_success "Qdrant is ready!"
            break
        fi
        if [ $i -eq 30 ]; then
            print_error "Qdrant failed to start within 60 seconds"
            exit 1
        fi
        sleep 2
    done
    
    # Wait for Redis
    print_status "Waiting for Redis to be ready..."
    for i in {1..15}; do
        if docker exec redis-islamic-app redis-cli ping >/dev/null 2>&1; then
            print_success "Redis is ready!"
            break
        fi
        if [ $i -eq 15 ]; then
            print_error "Redis failed to start within 30 seconds"
            exit 1
        fi
        sleep 2
    done
    
    print_success "All Docker services are running!"
}

# Test Hugging Face connection
test_hugging_face() {
    print_header "Testing Hugging Face Connection"
    
    if [ -f .env ]; then
        source .env
    fi
    
    if [ -z "$HUGGING_FACE_API_KEY" ] || [ "$HUGGING_FACE_API_KEY" = "your_api_key_here" ]; then
        print_warning "Hugging Face API key not set, skipping connection test"
        return
    fi
    
    print_status "Testing connection to Hugging Face API..."
    
    local test_model="sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2"
    local response=$(curl -s -w "%{http_code}" -o /dev/null \
        -H "Authorization: Bearer $HUGGING_FACE_API_KEY" \
        "https://api-inference.huggingface.co/models/$test_model")
    
    if [ "$response" = "200" ]; then
        print_success "Hugging Face API connection successful!"
    elif [ "$response" = "401" ]; then
        print_error "Hugging Face API authentication failed. Please check your API key."
    elif [ "$response" = "503" ]; then
        print_warning "Hugging Face model is loading. This is normal for the first request."
    else
        print_warning "Hugging Face API returned status code: $response"
    fi
}

# Build Rust project
build_project() {
    print_header "Building Rust Project"
    
    print_status "Building the project..."
    if cargo build --release; then
        print_success "Project built successfully!"
    else
        print_error "Failed to build project"
        exit 1
    fi
}

# Run tests
run_tests() {
    print_header "Running Tests"
    
    print_status "Running AI service integration tests..."
    if cargo test ai_service::integration_tests --lib; then
        print_success "Integration tests passed!"
    else
        print_warning "Some integration tests failed (this might be expected if services are not fully configured)"
    fi
    
    print_status "Running unit tests..."
    if cargo test ai_service --lib; then
        print_success "Unit tests passed!"
    else
        print_warning "Some unit tests failed"
    fi
}

# Create monitoring configuration
setup_monitoring() {
    print_header "Setting Up Monitoring (Optional)"
    
    # Create Prometheus configuration
    cat > monitoring/prometheus.yml << EOF
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  # - "first_rules.yml"
  # - "second_rules.yml"

scrape_configs:
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']
  
  - job_name: 'islamic-app'
    static_configs:
      - targets: ['host.docker.internal:8080']
    metrics_path: '/metrics'
    scrape_interval: 30s
EOF

    # Create Grafana datasource configuration
    cat > monitoring/grafana/provisioning/datasources/prometheus.yml << EOF
apiVersion: 1

datasources:
  - name: Prometheus
    type: prometheus
    access: proxy
    url: http://prometheus:9090
    isDefault: true
EOF

    print_success "Monitoring configuration created"
    print_status "To start monitoring services, run:"
    print_status "  docker-compose -f docker-compose.ai-services.yml --profile monitoring up -d"
}

# Run example
run_example() {
    print_header "Running Integration Example"
    
    print_status "Running AI service integration example..."
    if cargo run --example ai_service_integration; then
        print_success "Integration example completed successfully!"
    else
        print_warning "Integration example had some issues (check logs above)"
    fi
}

# Print summary
print_summary() {
    print_header "Setup Complete!"
    
    echo -e "${GREEN}🎉 AI Service Integration Setup Completed Successfully!${NC}\n"
    
    echo -e "${BLUE}📋 Services Status:${NC}"
    echo -e "   ✅ Qdrant Vector Database: http://localhost:6333"
    echo -e "   ✅ Redis Cache: localhost:6379"
    if [ ! -z "$HUGGING_FACE_API_KEY" ] && [ "$HUGGING_FACE_API_KEY" != "your_api_key_here" ]; then
        echo -e "   ✅ Hugging Face API: Configured"
    else
        echo -e "   ⚠️  Hugging Face API: Not configured"
    fi
    
    echo -e "\n${BLUE}🔧 Configuration Files:${NC}"
    echo -e "   📄 Main config: config/ai_service_config.yaml"
    echo -e "   📄 Environment: .env"
    echo -e "   📄 Docker: docker-compose.ai-services.yml"
    
    echo -e "\n${BLUE}🚀 Next Steps:${NC}"
    echo -e "   1. Set your Hugging Face API key in .env file (if not done)"
    echo -e "   2. Run: cargo run --example ai_service_integration"
    echo -e "   3. Start your main application with AI service enabled"
    
    echo -e "\n${BLUE}🛠️  Useful Commands:${NC}"
    echo -e "   • View logs: docker-compose -f docker-compose.ai-services.yml logs -f"
    echo -e "   • Stop services: docker-compose -f docker-compose.ai-services.yml down"
    echo -e "   • Restart services: docker-compose -f docker-compose.ai-services.yml restart"
    echo -e "   • Run tests: cargo test ai_service"
    
    echo -e "\n${BLUE}📚 Documentation:${NC}"
    echo -e "   📖 RAG Implementation: docs/RAG_IMPLEMENTATION.md"
    echo -e "   📖 Configuration Guide: config/ai_service_config.yaml"
    
    if [ ! -z "$HUGGING_FACE_API_KEY" ] && [ "$HUGGING_FACE_API_KEY" != "your_api_key_here" ]; then
        echo -e "\n${GREEN}✨ Everything is ready! You can now use the AI service integration.${NC}"
    else
        echo -e "\n${YELLOW}⚠️  Don't forget to set your Hugging Face API key in the .env file!${NC}"
    fi
}

# Main execution
main() {
    print_header "AI Service Integration Setup"
    
    check_prerequisites
    setup_environment
    create_directories
    start_docker_services
    test_hugging_face
    build_project
    run_tests
    setup_monitoring
    
    # Ask if user wants to run the example
    echo
    read -p "Do you want to run the integration example now? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        run_example
    fi
    
    print_summary
}

# Handle script interruption
trap 'print_error "Setup interrupted by user"; exit 1' INT

# Run main function
main "$@"
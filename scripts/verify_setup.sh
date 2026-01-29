#!/bin/bash

# Sanad Setup Verification Script
# This script verifies that the foundational infrastructure is properly set up

set -e

echo "🕌 Sanad Islamic Application - Setup Verification"
echo "=================================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
        return 1
    fi
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "services" ]; then
    echo -e "${RED}Error: Please run this script from the Sanad project root directory${NC}"
    exit 1
fi

echo "1. Checking Project Structure..."
echo "--------------------------------"

# Check main directories
directories=(
    "services"
    "shared"
    "gateway"
    "database"
    "config"
    "scripts"
)

for dir in "${directories[@]}"; do
    if [ -d "$dir" ]; then
        print_status 0 "Directory $dir exists"
    else
        print_status 1 "Directory $dir missing"
    fi
done

echo ""
echo "2. Checking Service Structure..."
echo "--------------------------------"

# Check service directories
services=(
    "quran-service"
    "hadith-service"
    "stories-service"
    "ai-service"
)

for service in "${services[@]}"; do
    if [ -d "services/$service" ]; then
        print_status 0 "Service $service directory exists"
        
        # Check for essential files
        if [ -f "services/$service/Cargo.toml" ]; then
            print_status 0 "  - Cargo.toml found"
        else
            print_status 1 "  - Cargo.toml missing"
        fi
        
        if [ -f "services/$service/src/main.rs" ]; then
            print_status 0 "  - main.rs found"
        else
            print_status 1 "  - main.rs missing"
        fi
        
        if [ -f "services/$service/Dockerfile" ]; then
            print_status 0 "  - Dockerfile found"
        else
            print_status 1 "  - Dockerfile missing"
        fi
    else
        print_status 1 "Service $service directory missing"
    fi
done

echo ""
echo "3. Checking Configuration Files..."
echo "----------------------------------"

# Check configuration files
config_files=(
    "Cargo.toml"
    "docker-compose.yml"
    ".env.example"
    "config/default.toml"
    "config/development.toml"
    "config/production.toml"
    "Makefile"
    "README.md"
)

for file in "${config_files[@]}"; do
    if [ -f "$file" ]; then
        print_status 0 "Configuration file $file exists"
    else
        print_status 1 "Configuration file $file missing"
    fi
done

echo ""
echo "4. Checking Database Schema..."
echo "------------------------------"

# Check database files
db_files=(
    "database/init/01_create_tables.sql"
    "database/init/02_sample_data.sql"
)

for file in "${db_files[@]}"; do
    if [ -f "$file" ]; then
        print_status 0 "Database file $file exists"
    else
        print_status 1 "Database file $file missing"
    fi
done

echo ""
echo "5. Checking Rust Dependencies..."
echo "--------------------------------"

# Check if Rust is installed
if command -v rustc &> /dev/null; then
    rust_version=$(rustc --version)
    print_status 0 "Rust is installed: $rust_version"
else
    print_status 1 "Rust is not installed"
    print_info "Install Rust from https://rustup.rs/"
fi

# Check if Cargo is available
if command -v cargo &> /dev/null; then
    cargo_version=$(cargo --version)
    print_status 0 "Cargo is available: $cargo_version"
else
    print_status 1 "Cargo is not available"
fi

echo ""
echo "6. Checking Docker Setup..."
echo "---------------------------"

# Check if Docker is installed
if command -v docker &> /dev/null; then
    docker_version=$(docker --version)
    print_status 0 "Docker is installed: $docker_version"
else
    print_status 1 "Docker is not installed"
    print_info "Install Docker from https://docs.docker.com/get-docker/"
fi

# Check if Docker Compose is available
if command -v docker-compose &> /dev/null; then
    compose_version=$(docker-compose --version)
    print_status 0 "Docker Compose is available: $compose_version"
else
    print_status 1 "Docker Compose is not available"
fi

echo ""
echo "7. Validating Cargo Workspace..."
echo "--------------------------------"

# Check if cargo check passes
if cargo check --workspace --quiet 2>/dev/null; then
    print_status 0 "Cargo workspace validation passed"
else
    print_status 1 "Cargo workspace validation failed"
    print_info "Run 'cargo check --workspace' for detailed errors"
fi

echo ""
echo "8. Checking Environment Setup..."
echo "--------------------------------"

# Check if .env file exists
if [ -f ".env" ]; then
    print_status 0 ".env file exists"
else
    print_warning ".env file not found (copy from .env.example)"
    print_info "Run: cp .env.example .env"
fi

echo ""
echo "9. Testing Docker Compose Configuration..."
echo "-----------------------------------------"

# Validate docker-compose.yml
if docker-compose config &> /dev/null; then
    print_status 0 "Docker Compose configuration is valid"
else
    print_status 1 "Docker Compose configuration has errors"
    print_info "Run 'docker-compose config' for detailed errors"
fi

echo ""
echo "10. Summary and Next Steps..."
echo "=============================="

print_info "Foundational infrastructure setup verification complete!"
echo ""
print_info "To start the application:"
echo "  1. Copy environment file: cp .env.example .env"
echo "  2. Edit .env with your API keys and configuration"
echo "  3. Start services: make docker-up"
echo "  4. Check health: make health"
echo ""
print_info "For development:"
echo "  1. Start databases: docker-compose up -d postgres redis qdrant"
echo "  2. Run gateway: cargo run --bin gateway"
echo ""
print_info "Available commands:"
echo "  - make help          # Show all available commands"
echo "  - make docker-up     # Start all services"
echo "  - make dev           # Development mode"
echo "  - make test          # Run tests"
echo "  - make build         # Build all services"
echo ""

# Check if any critical components are missing
if [ ! -f "Cargo.toml" ] || [ ! -f "docker-compose.yml" ] || [ ! -d "services/gateway" ]; then
    echo -e "${RED}❌ Critical components are missing. Please check the setup.${NC}"
    exit 1
else
    echo -e "${GREEN}✅ All foundational components are in place!${NC}"
    echo -e "${GREEN}🕌 Sanad Islamic Application is ready for development.${NC}"
fi
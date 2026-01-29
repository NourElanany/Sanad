# Sanad Islamic Application Makefile

.PHONY: help build test clean docker-build docker-up docker-down setup dev check fmt clippy

# Default target
help:
	@echo "Sanad Islamic Application - Available Commands:"
	@echo ""
	@echo "Development:"
	@echo "  setup          - Set up development environment"
	@echo "  dev            - Run in development mode"
	@echo "  build          - Build all services"
	@echo "  test           - Run all tests"
	@echo "  check          - Run cargo check on all services"
	@echo "  fmt            - Format code"
	@echo "  clippy         - Run clippy linter"
	@echo ""
	@echo "Docker:"
	@echo "  docker-build   - Build all Docker images"
	@echo "  docker-up      - Start all services with Docker Compose"
	@echo "  docker-down    - Stop all services"
	@echo "  docker-logs    - View logs from all services"
	@echo ""
	@echo "Database:"
	@echo "  db-setup       - Set up database with sample data"
	@echo "  db-migrate     - Run database migrations"
	@echo "  db-reset       - Reset database (WARNING: destroys data)"
	@echo ""
	@echo "Utilities:"
	@echo "  clean          - Clean build artifacts"
	@echo "  install-deps   - Install system dependencies"

# Development setup
setup:
	@echo "Setting up Sanad development environment..."
	cp .env.example .env
	@echo "Please edit .env file with your configuration"
	@echo "Then run 'make docker-up' to start the services"

# Development mode - start databases and run gateway locally
dev:
	@echo "Starting development environment..."
	docker-compose up -d postgres redis qdrant
	@echo "Waiting for databases to be ready..."
	sleep 10
	cargo run --bin gateway

# Build all services
build:
	@echo "Building all Sanad services..."
	cargo build --release

# Run all tests
test:
	@echo "Running all tests..."
	cargo test --workspace

# Run property-based tests specifically
test-pbt:
	@echo "Running property-based tests..."
	cargo test --workspace --features proptest

# Check all services
check:
	@echo "Checking all services..."
	cargo check --workspace

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt --all

# Run clippy
clippy:
	@echo "Running clippy..."
	cargo clippy --workspace --all-targets --all-features -- -D warnings

# Docker commands
docker-build:
	@echo "Building Docker images..."
	docker-compose build

docker-up:
	@echo "Starting all services with Docker Compose..."
	docker-compose up -d
	@echo "Services are starting up. Check status with 'docker-compose ps'"
	@echo "API Gateway will be available at http://localhost:8080"

docker-down:
	@echo "Stopping all services..."
	docker-compose down

docker-logs:
	@echo "Showing logs from all services..."
	docker-compose logs -f

docker-restart:
	@echo "Restarting all services..."
	docker-compose restart

# Database commands
db-setup:
	@echo "Setting up database..."
	docker-compose up -d postgres
	sleep 5
	@echo "Database setup complete"

db-migrate:
	@echo "Running database migrations..."
	# TODO: Add migration command when implemented
	@echo "Migrations not yet implemented"

db-reset:
	@echo "WARNING: This will destroy all data!"
	@read -p "Are you sure? [y/N] " -n 1 -r; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		docker-compose down -v; \
		docker-compose up -d postgres redis qdrant; \
		echo "Database reset complete"; \
	else \
		echo "Cancelled"; \
	fi

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	docker system prune -f

# Install system dependencies (Ubuntu/Debian)
install-deps:
	@echo "Installing system dependencies..."
	sudo apt-get update
	sudo apt-get install -y \
		curl \
		build-essential \
		pkg-config \
		libssl-dev \
		docker.io \
		docker-compose
	@echo "Installing Rust..."
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	@echo "Please restart your shell or run 'source ~/.cargo/env'"

# Health check
health:
	@echo "Checking service health..."
	@curl -s http://localhost:8080/api/v1/health | jq . || echo "Gateway not responding"
	@curl -s http://localhost:8081/health | jq . || echo "Quran service not responding"
	@curl -s http://localhost:8082/health | jq . || echo "Hadith service not responding"

# Performance test
perf-test:
	@echo "Running performance tests..."
	@echo "TODO: Implement performance testing"

# Security scan
security-scan:
	@echo "Running security scan..."
	cargo audit
	@echo "TODO: Add additional security scanning"

# Generate API documentation
docs:
	@echo "Generating API documentation..."
	cargo doc --workspace --no-deps --open

# Backup data
backup:
	@echo "Creating backup..."
	docker-compose exec postgres pg_dump -U sanad_user sanad > backup_$(shell date +%Y%m%d_%H%M%S).sql
	@echo "Backup created"

# Restore data
restore:
	@echo "Restoring from backup..."
	@echo "Usage: make restore BACKUP_FILE=backup_20240101_120000.sql"
	@if [ -z "$(BACKUP_FILE)" ]; then \
		echo "Please specify BACKUP_FILE"; \
		exit 1; \
	fi
	docker-compose exec -T postgres psql -U sanad_user sanad < $(BACKUP_FILE)

# Monitor logs in real-time
monitor:
	@echo "Monitoring all service logs..."
	docker-compose logs -f --tail=100

# Quick development cycle
quick: fmt clippy test
	@echo "Quick development cycle complete"

# Full CI pipeline
ci: fmt clippy test build
	@echo "CI pipeline complete"

# Production deployment preparation
prod-prep: clean build test security-scan
	@echo "Production preparation complete"
	@echo "Ready for deployment"
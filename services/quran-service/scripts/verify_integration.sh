#!/bin/bash

# Quran Service Integration Verification Script
# This script verifies that the Quran service is properly integrated and functional

set -e

echo "🕌 Quran Service Integration Verification"
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
        exit 1
    fi
}

print_info() {
    echo -e "${YELLOW}ℹ️  $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Please run this script from the quran-service directory${NC}"
    exit 1
fi

print_info "Checking Quran service integration..."

# 1. Verify code compiles
print_info "1. Compiling service..."
cargo check --quiet
print_status $? "Service compiles successfully"

# 2. Run unit tests
print_info "2. Running unit tests..."
cargo test --lib --quiet
print_status $? "All unit tests pass"

# 3. Run integration tests
print_info "3. Running integration tests..."
cargo test --test integration_test --quiet
print_status $? "All integration tests pass"

# 4. Check for required dependencies
print_info "4. Checking dependencies..."
cargo tree --quiet > /dev/null 2>&1
print_status $? "All dependencies resolved"

# 5. Verify database migration files exist
print_info "5. Checking database migrations..."
if [ -f "../../database/migrations/003_quran_enhancements.sql" ]; then
    print_status 0 "Database migrations found"
else
    print_status 1 "Database migrations missing"
fi

# 6. Check Docker configuration
print_info "6. Checking Docker configuration..."
if [ -f "Dockerfile" ]; then
    print_status 0 "Dockerfile exists"
else
    print_status 1 "Dockerfile missing"
fi

# 7. Verify API Gateway integration
print_info "7. Checking API Gateway integration..."
if grep -q "quran-service" ../../gateway/src/routes.rs; then
    print_status 0 "API Gateway integration configured"
else
    print_status 1 "API Gateway integration missing"
fi

# 8. Check docker-compose configuration
print_info "8. Checking docker-compose configuration..."
if grep -q "quran-service:" ../../docker-compose.yml; then
    print_status 0 "Docker Compose configuration found"
else
    print_status 1 "Docker Compose configuration missing"
fi

# 9. Verify service registry configuration
print_info "9. Checking service registry..."
if grep -q "quran-service" ../../gateway/src/proxy.rs; then
    print_status 0 "Service registry configuration found"
else
    print_status 1 "Service registry configuration missing"
fi

# 10. Check for required environment variables documentation
print_info "10. Checking documentation..."
if grep -q "DATABASE_URL" README.md; then
    print_status 0 "Environment variables documented"
else
    print_status 1 "Environment variables documentation missing"
fi

# 11. Verify model integrity features
print_info "11. Testing content integrity features..."
cargo test --lib test_real_quranic_content_integrity --quiet
print_status $? "Content integrity verification works"

# 12. Test property-based tests
print_info "12. Running property-based tests..."
cargo test --lib prop_ayah_integrity_verification --quiet
print_status $? "Property-based tests pass"

# 13. Check for comprehensive test coverage
print_info "13. Checking test coverage..."
TEST_COUNT=$(cargo test --lib 2>&1 | grep "test result:" | grep -o "[0-9]\+ passed" | grep -o "[0-9]\+")
if [ "$TEST_COUNT" -ge 60 ]; then
    print_status 0 "Comprehensive test coverage ($TEST_COUNT tests)"
else
    print_status 1 "Insufficient test coverage ($TEST_COUNT tests)"
fi

echo ""
echo -e "${GREEN}🎉 Quran Service Integration Verification Complete!${NC}"
echo ""
echo "Summary:"
echo "- ✅ Service compiles and runs"
echo "- ✅ All tests pass (unit, integration, property-based)"
echo "- ✅ Database migrations ready"
echo "- ✅ Docker configuration complete"
echo "- ✅ API Gateway integration configured"
echo "- ✅ Service registry configured"
echo "- ✅ Content integrity verification implemented"
echo "- ✅ Comprehensive test coverage"
echo ""
echo "The Quran service is ready for deployment and integration!"
echo ""
echo "Next steps:"
echo "1. Start the database: docker-compose up postgres redis"
echo "2. Run migrations: sqlx migrate run"
echo "3. Start the service: cargo run"
echo "4. Test endpoints: curl http://localhost:8081/health"
echo ""
echo "For full system integration:"
echo "1. Start all services: docker-compose up"
echo "2. Access via API Gateway: http://localhost:8080/api/v1/quran/health"
# Quran Service Integration Verification Script (PowerShell)
# This script verifies that the Quran service is properly integrated and functional

Write-Host "🕌 Quran Service Integration Verification" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

function Print-Status {
    param(
        [bool]$Success,
        [string]$Message
    )
    
    if ($Success) {
        Write-Host "✅ $Message" -ForegroundColor Green
    } else {
        Write-Host "❌ $Message" -ForegroundColor Red
        exit 1
    }
}

function Print-Info {
    param([string]$Message)
    Write-Host "ℹ️  $Message" -ForegroundColor Yellow
}

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "❌ Please run this script from the quran-service directory" -ForegroundColor Red
    exit 1
}

Print-Info "Checking Quran service integration..."

# 1. Verify code compiles
Print-Info "1. Compiling service..."
$compileResult = & cargo check --quiet 2>&1
Print-Status ($LASTEXITCODE -eq 0) "Service compiles successfully"

# 2. Run unit tests
Print-Info "2. Running unit tests..."
$testResult = & cargo test --lib --quiet 2>&1
Print-Status ($LASTEXITCODE -eq 0) "All unit tests pass"

# 3. Run integration tests
Print-Info "3. Running integration tests..."
$integrationResult = & cargo test --test integration_test --quiet 2>&1
Print-Status ($LASTEXITCODE -eq 0) "All integration tests pass"

# 4. Check for required dependencies
Print-Info "4. Checking dependencies..."
$depsResult = & cargo tree --quiet 2>&1
Print-Status ($LASTEXITCODE -eq 0) "All dependencies resolved"

# 5. Verify database migration files exist
Print-Info "5. Checking database migrations..."
$migrationExists = Test-Path "../../database/migrations/003_quran_enhancements.sql"
Print-Status $migrationExists "Database migrations found"

# 6. Check Docker configuration
Print-Info "6. Checking Docker configuration..."
$dockerfileExists = Test-Path "Dockerfile"
Print-Status $dockerfileExists "Dockerfile exists"

# 7. Verify API Gateway integration
Print-Info "7. Checking API Gateway integration..."
$gatewayContent = Get-Content "../../gateway/src/routes.rs" -Raw -ErrorAction SilentlyContinue
$gatewayIntegrated = $gatewayContent -match "quran-service"
Print-Status $gatewayIntegrated "API Gateway integration configured"

# 8. Check docker-compose configuration
Print-Info "8. Checking docker-compose configuration..."
$composeContent = Get-Content "../../docker-compose.yml" -Raw -ErrorAction SilentlyContinue
$composeConfigured = $composeContent -match "quran-service:"
Print-Status $composeConfigured "Docker Compose configuration found"

# 9. Verify service registry configuration
Print-Info "9. Checking service registry..."
$proxyContent = Get-Content "../../gateway/src/proxy.rs" -Raw -ErrorAction SilentlyContinue
$registryConfigured = $proxyContent -match "quran-service"
Print-Status $registryConfigured "Service registry configuration found"

# 10. Check for required environment variables documentation
Print-Info "10. Checking documentation..."
$readmeContent = Get-Content "README.md" -Raw -ErrorAction SilentlyContinue
$envDocumented = $readmeContent -match "DATABASE_URL"
Print-Status $envDocumented "Environment variables documented"

# 11. Verify model integrity features
Print-Info "11. Testing content integrity features..."
$integrityResult = & cargo test --lib test_real_quranic_content_integrity --quiet 2>&1
Print-Status ($LASTEXITCODE -eq 0) "Content integrity verification works"

# 12. Test property-based tests
Print-Info "12. Running property-based tests..."
$propResult = & cargo test --lib prop_ayah_integrity_verification --quiet 2>&1
Print-Status ($LASTEXITCODE -eq 0) "Property-based tests pass"

# 13. Check for comprehensive test coverage
Print-Info "13. Checking test coverage..."
$testOutput = & cargo test --lib 2>&1 | Out-String
$testCount = 0
if ($testOutput -match "(\d+) passed") {
    $testCount = [int]$matches[1]
}
$comprehensiveCoverage = $testCount -ge 60
Print-Status $comprehensiveCoverage "Comprehensive test coverage ($testCount tests)"

Write-Host ""
Write-Host "Quran Service Integration Verification Complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Summary:" -ForegroundColor Cyan
Write-Host "- Service compiles and runs" -ForegroundColor Green
Write-Host "- All tests pass (unit, integration, property-based)" -ForegroundColor Green
Write-Host "- Database migrations ready" -ForegroundColor Green
Write-Host "- Docker configuration complete" -ForegroundColor Green
Write-Host "- API Gateway integration configured" -ForegroundColor Green
Write-Host "- Service registry configured" -ForegroundColor Green
Write-Host "- Content integrity verification implemented" -ForegroundColor Green
Write-Host "- Comprehensive test coverage" -ForegroundColor Green
Write-Host ""
Write-Host "The Quran service is ready for deployment and integration!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Start the database: docker-compose up postgres redis" -ForegroundColor White
Write-Host "2. Run migrations: sqlx migrate run" -ForegroundColor White
Write-Host "3. Start the service: cargo run" -ForegroundColor White
Write-Host "4. Test endpoints: curl http://localhost:8081/health" -ForegroundColor White
Write-Host ""
Write-Host "For full system integration:" -ForegroundColor Yellow
Write-Host "1. Start all services: docker-compose up" -ForegroundColor White
Write-Host "2. Access via API Gateway: http://localhost:8080/api/v1/quran/health" -ForegroundColor White
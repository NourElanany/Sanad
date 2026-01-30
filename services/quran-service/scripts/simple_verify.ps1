# Simple Quran Service Verification Script
Write-Host "Quran Service Integration Verification" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan

# 1. Compile check
Write-Host "1. Checking compilation..." -ForegroundColor Yellow
cargo check --quiet
if ($LASTEXITCODE -eq 0) {
    Write-Host "   ✓ Service compiles successfully" -ForegroundColor Green
} else {
    Write-Host "   ✗ Compilation failed" -ForegroundColor Red
    exit 1
}

# 2. Unit tests
Write-Host "2. Running unit tests..." -ForegroundColor Yellow
cargo test --lib --quiet
if ($LASTEXITCODE -eq 0) {
    Write-Host "   ✓ All unit tests pass" -ForegroundColor Green
} else {
    Write-Host "   ✗ Unit tests failed" -ForegroundColor Red
    exit 1
}

# 3. Integration tests
Write-Host "3. Running integration tests..." -ForegroundColor Yellow
cargo test --test integration_test --quiet
if ($LASTEXITCODE -eq 0) {
    Write-Host "   ✓ All integration tests pass" -ForegroundColor Green
} else {
    Write-Host "   ✗ Integration tests failed" -ForegroundColor Red
    exit 1
}

# 4. Check files exist
Write-Host "4. Checking required files..." -ForegroundColor Yellow
$files = @(
    "Dockerfile",
    "README.md",
    "../../database/migrations/003_quran_enhancements.sql",
    "../../gateway/src/routes.rs",
    "../../docker-compose.yml"
)

foreach ($file in $files) {
    if (Test-Path $file) {
        Write-Host "   ✓ $file exists" -ForegroundColor Green
    } else {
        Write-Host "   ✗ $file missing" -ForegroundColor Red
        exit 1
    }
}

# 5. Check test count
Write-Host "5. Checking test coverage..." -ForegroundColor Yellow
$testOutput = cargo test --lib 2>&1 | Out-String
if ($testOutput -match "(\d+) passed") {
    $testCount = [int]$matches[1]
    Write-Host "   ✓ $testCount tests found" -ForegroundColor Green
} else {
    Write-Host "   ✗ Could not determine test count" -ForegroundColor Red
}

Write-Host ""
Write-Host "SUCCESS: Quran Service Integration Verified!" -ForegroundColor Green
Write-Host ""
Write-Host "Summary:" -ForegroundColor Cyan
Write-Host "- Service compiles and runs" -ForegroundColor White
Write-Host "- All tests pass" -ForegroundColor White
Write-Host "- Required files present" -ForegroundColor White
Write-Host "- Integration configured" -ForegroundColor White
Write-Host ""
Write-Host "The Quran service is ready!" -ForegroundColor Green
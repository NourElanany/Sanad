# AI Service Integration Setup Script (PowerShell)
# This script sets up the complete AI service integration for the Islamic application

param(
    [switch]$SkipTests,
    [switch]$SkipExample,
    [switch]$Monitoring
)

# Colors for output
$Red = "Red"
$Green = "Green"
$Yellow = "Yellow"
$Blue = "Blue"
$White = "White"

# Function to print colored output
function Write-Status {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor $Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor $Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor $Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor $Red
}

function Write-Header {
    param([string]$Title)
    Write-Host ""
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor $Blue
    Write-Host " $Title" -ForegroundColor $Blue
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor $Blue
    Write-Host ""
}

# Check if command exists
function Test-Command {
    param([string]$Command)
    try {
        Get-Command $Command -ErrorAction Stop | Out-Null
        return $true
    }
    catch {
        return $false
    }
}

# Check prerequisites
function Test-Prerequisites {
    Write-Header "Checking Prerequisites"
    
    $missingDeps = @()
    
    # Check Docker
    if (Test-Command "docker") {
        Write-Success "Docker is installed"
        docker --version
    }
    else {
        $missingDeps += "docker"
        Write-Error "Docker is not installed"
    }
    
    # Check Docker Compose
    if (Test-Command "docker-compose") {
        Write-Success "Docker Compose is available"
    }
    elseif ((docker compose version 2>$null) -and ($LASTEXITCODE -eq 0)) {
        Write-Success "Docker Compose (plugin) is available"
    }
    else {
        $missingDeps += "docker-compose"
        Write-Error "Docker Compose is not available"
    }
    
    # Check Rust
    if (Test-Command "rustc") {
        Write-Success "Rust is installed"
        rustc --version
    }
    else {
        $missingDeps += "rust"
        Write-Error "Rust is not installed"
    }
    
    # Check Cargo
    if (Test-Command "cargo") {
        Write-Success "Cargo is available"
        cargo --version
    }
    else {
        $missingDeps += "cargo"
        Write-Error "Cargo is not available"
    }
    
    # Check curl or Invoke-WebRequest
    if (Test-Command "curl") {
        Write-Success "curl is available"
    }
    elseif (Test-Command "Invoke-WebRequest") {
        Write-Success "PowerShell web requests are available"
    }
    else {
        $missingDeps += "curl or PowerShell"
        Write-Error "No web request capability found"
    }
    
    if ($missingDeps.Count -gt 0) {
        Write-Error "Missing dependencies: $($missingDeps -join ', ')"
        Write-Status "Please install the missing dependencies and run this script again."
        exit 1
    }
    
    Write-Success "All prerequisites are met!"
}

# Setup environment variables
function Set-Environment {
    Write-Header "Setting Up Environment Variables"
    
    # Create .env file if it doesn't exist
    if (-not (Test-Path ".env")) {
        Write-Status "Creating .env file..."
        @"
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
"@ | Out-File -FilePath ".env" -Encoding UTF8
        Write-Success ".env file created"
    }
    else {
        Write-Status ".env file already exists"
    }
    
    # Load environment variables
    if (Test-Path ".env") {
        Get-Content ".env" | ForEach-Object {
            if ($_ -match "^([^#][^=]+)=(.*)$") {
                [Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
            }
        }
    }
    
    # Check if Hugging Face API key is set
    $apiKey = [Environment]::GetEnvironmentVariable("HUGGING_FACE_API_KEY", "Process")
    if ([string]::IsNullOrEmpty($apiKey) -or $apiKey -eq "your_api_key_here") {
        Write-Warning "Hugging Face API key is not set!"
        Write-Status "Please:"
        Write-Status "1. Get an API key from: https://huggingface.co/settings/tokens"
        Write-Status "2. Edit .env file and set HUGGING_FACE_API_KEY=your_actual_key"
        Write-Status "3. Re-run this script"
        
        $response = Read-Host "Do you want to set the API key now? (y/n)"
        if ($response -eq "y" -or $response -eq "Y") {
            $newApiKey = Read-Host "Enter your Hugging Face API key"
            if (-not [string]::IsNullOrEmpty($newApiKey)) {
                (Get-Content ".env") -replace "HUGGING_FACE_API_KEY=your_api_key_here", "HUGGING_FACE_API_KEY=$newApiKey" | Set-Content ".env"
                Write-Success "API key updated in .env file"
                [Environment]::SetEnvironmentVariable("HUGGING_FACE_API_KEY", $newApiKey, "Process")
            }
        }
    }
    else {
        Write-Success "Hugging Face API key is configured"
    }
}

# Create required directories
function New-RequiredDirectories {
    Write-Header "Creating Required Directories"
    
    $dirs = @(
        "config",
        "data",
        "data/qdrant",
        "data/redis",
        "logs",
        "monitoring",
        "monitoring/grafana/provisioning/dashboards",
        "monitoring/grafana/provisioning/datasources"
    )
    
    foreach ($dir in $dirs) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
            Write-Success "Created directory: $dir"
        }
        else {
            Write-Status "Directory already exists: $dir"
        }
    }
}

# Start Docker services
function Start-DockerServices {
    Write-Header "Starting Docker Services"
    
    Write-Status "Starting Qdrant and Redis..."
    
    # Determine Docker Compose command
    $composeCmd = if (Test-Command "docker-compose") { "docker-compose" } else { "docker compose" }
    
    # Start core services
    & $composeCmd -f docker-compose.ai-services.yml up -d qdrant redis
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to start Docker services"
        exit 1
    }
    
    Write-Status "Waiting for services to be ready..."
    
    # Wait for Qdrant
    Write-Status "Waiting for Qdrant to be ready..."
    $qdrantReady = $false
    for ($i = 1; $i -le 30; $i++) {
        try {
            if (Test-Command "curl") {
                $response = curl -s http://localhost:6333/ 2>$null
                if ($LASTEXITCODE -eq 0) {
                    $qdrantReady = $true
                    break
                }
            }
            else {
                $response = Invoke-WebRequest -Uri "http://localhost:6333/" -UseBasicParsing -TimeoutSec 2 -ErrorAction SilentlyContinue
                if ($response.StatusCode -eq 200) {
                    $qdrantReady = $true
                    break
                }
            }
        }
        catch {
            # Continue waiting
        }
        
        if ($i -eq 30) {
            Write-Error "Qdrant failed to start within 60 seconds"
            exit 1
        }
        Start-Sleep -Seconds 2
    }
    
    if ($qdrantReady) {
        Write-Success "Qdrant is ready!"
    }
    
    # Wait for Redis
    Write-Status "Waiting for Redis to be ready..."
    $redisReady = $false
    for ($i = 1; $i -le 15; $i++) {
        try {
            docker exec redis-islamic-app redis-cli ping 2>$null | Out-Null
            if ($LASTEXITCODE -eq 0) {
                $redisReady = $true
                break
            }
        }
        catch {
            # Continue waiting
        }
        
        if ($i -eq 15) {
            Write-Error "Redis failed to start within 30 seconds"
            exit 1
        }
        Start-Sleep -Seconds 2
    }
    
    if ($redisReady) {
        Write-Success "Redis is ready!"
    }
    
    Write-Success "All Docker services are running!"
}

# Test Hugging Face connection
function Test-HuggingFace {
    Write-Header "Testing Hugging Face Connection"
    
    $apiKey = [Environment]::GetEnvironmentVariable("HUGGING_FACE_API_KEY", "Process")
    if ([string]::IsNullOrEmpty($apiKey) -or $apiKey -eq "your_api_key_here") {
        Write-Warning "Hugging Face API key not set, skipping connection test"
        return
    }
    
    Write-Status "Testing connection to Hugging Face API..."
    
    $testModel = "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2"
    $uri = "https://api-inference.huggingface.co/models/$testModel"
    
    try {
        if (Test-Command "curl") {
            $response = curl -s -w "%{http_code}" -o $null -H "Authorization: Bearer $apiKey" $uri
            $statusCode = $response
        }
        else {
            $headers = @{ "Authorization" = "Bearer $apiKey" }
            $response = Invoke-WebRequest -Uri $uri -Headers $headers -UseBasicParsing -ErrorAction SilentlyContinue
            $statusCode = $response.StatusCode
        }
        
        switch ($statusCode) {
            200 { Write-Success "Hugging Face API connection successful!" }
            401 { Write-Error "Hugging Face API authentication failed. Please check your API key." }
            503 { Write-Warning "Hugging Face model is loading. This is normal for the first request." }
            default { Write-Warning "Hugging Face API returned status code: $statusCode" }
        }
    }
    catch {
        Write-Warning "Could not test Hugging Face connection: $($_.Exception.Message)"
    }
}

# Build Rust project
function Build-Project {
    Write-Header "Building Rust Project"
    
    Write-Status "Building the project..."
    cargo build --release
    
    if ($LASTEXITCODE -eq 0) {
        Write-Success "Project built successfully!"
    }
    else {
        Write-Error "Failed to build project"
        exit 1
    }
}

# Run tests
function Invoke-Tests {
    Write-Header "Running Tests"
    
    if (-not $SkipTests) {
        Write-Status "Running AI service integration tests..."
        cargo test ai_service::integration_tests --lib
        
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Integration tests passed!"
        }
        else {
            Write-Warning "Some integration tests failed (this might be expected if services are not fully configured)"
        }
        
        Write-Status "Running unit tests..."
        cargo test ai_service --lib
        
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Unit tests passed!"
        }
        else {
            Write-Warning "Some unit tests failed"
        }
    }
    else {
        Write-Status "Skipping tests as requested"
    }
}

# Create monitoring configuration
function Set-Monitoring {
    Write-Header "Setting Up Monitoring (Optional)"
    
    # Create Prometheus configuration
    @"
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
"@ | Out-File -FilePath "monitoring/prometheus.yml" -Encoding UTF8

    # Create Grafana datasource configuration
    @"
apiVersion: 1

datasources:
  - name: Prometheus
    type: prometheus
    access: proxy
    url: http://prometheus:9090
    isDefault: true
"@ | Out-File -FilePath "monitoring/grafana/provisioning/datasources/prometheus.yml" -Encoding UTF8

    Write-Success "Monitoring configuration created"
    Write-Status "To start monitoring services, run:"
    Write-Status "  docker-compose -f docker-compose.ai-services.yml --profile monitoring up -d"
}

# Run example
function Invoke-Example {
    Write-Header "Running Integration Example"
    
    if (-not $SkipExample) {
        Write-Status "Running AI service integration example..."
        cargo run --example ai_service_integration
        
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Integration example completed successfully!"
        }
        else {
            Write-Warning "Integration example had some issues (check logs above)"
        }
    }
    else {
        Write-Status "Skipping example as requested"
    }
}

# Print summary
function Write-Summary {
    Write-Header "Setup Complete!"
    
    Write-Host "🎉 AI Service Integration Setup Completed Successfully!" -ForegroundColor $Green
    Write-Host ""
    
    Write-Host "📋 Services Status:" -ForegroundColor $Blue
    Write-Host "   ✅ Qdrant Vector Database: http://localhost:6333"
    Write-Host "   ✅ Redis Cache: localhost:6379"
    
    $apiKey = [Environment]::GetEnvironmentVariable("HUGGING_FACE_API_KEY", "Process")
    if (-not [string]::IsNullOrEmpty($apiKey) -and $apiKey -ne "your_api_key_here") {
        Write-Host "   ✅ Hugging Face API: Configured"
    }
    else {
        Write-Host "   ⚠️  Hugging Face API: Not configured" -ForegroundColor $Yellow
    }
    
    Write-Host ""
    Write-Host "🔧 Configuration Files:" -ForegroundColor $Blue
    Write-Host "   📄 Main config: config/ai_service_config.yaml"
    Write-Host "   📄 Environment: .env"
    Write-Host "   📄 Docker: docker-compose.ai-services.yml"
    
    Write-Host ""
    Write-Host "🚀 Next Steps:" -ForegroundColor $Blue
    Write-Host "   1. Set your Hugging Face API key in .env file (if not done)"
    Write-Host "   2. Run: cargo run --example ai_service_integration"
    Write-Host "   3. Start your main application with AI service enabled"
    
    Write-Host ""
    Write-Host "🛠️  Useful Commands:" -ForegroundColor $Blue
    Write-Host "   • View logs: docker-compose -f docker-compose.ai-services.yml logs -f"
    Write-Host "   • Stop services: docker-compose -f docker-compose.ai-services.yml down"
    Write-Host "   • Restart services: docker-compose -f docker-compose.ai-services.yml restart"
    Write-Host "   • Run tests: cargo test ai_service"
    
    Write-Host ""
    Write-Host "📚 Documentation:" -ForegroundColor $Blue
    Write-Host "   📖 RAG Implementation: docs/RAG_IMPLEMENTATION.md"
    Write-Host "   📖 Configuration Guide: config/ai_service_config.yaml"
    
    if (-not [string]::IsNullOrEmpty($apiKey) -and $apiKey -ne "your_api_key_here") {
        Write-Host ""
        Write-Host "✨ Everything is ready! You can now use the AI service integration." -ForegroundColor $Green
    }
    else {
        Write-Host ""
        Write-Host "⚠️  Don't forget to set your Hugging Face API key in the .env file!" -ForegroundColor $Yellow
    }
}

# Main execution
function Main {
    Write-Header "AI Service Integration Setup"
    
    try {
        Test-Prerequisites
        Set-Environment
        New-RequiredDirectories
        Start-DockerServices
        Test-HuggingFace
        Build-Project
        Invoke-Tests
        Set-Monitoring
        
        # Ask if user wants to run the example
        if (-not $SkipExample) {
            Write-Host ""
            $response = Read-Host "Do you want to run the integration example now? (y/n)"
            if ($response -eq "y" -or $response -eq "Y") {
                Invoke-Example
            }
        }
        
        Write-Summary
    }
    catch {
        Write-Error "Setup failed: $($_.Exception.Message)"
        exit 1
    }
}

# Handle script interruption
trap {
    Write-Error "Setup interrupted by user"
    exit 1
}

# Run main function
Main
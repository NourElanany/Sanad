# Build script for API Integration Service Docker image (PowerShell)

param(
    [string]$Version = "latest",
    [string]$ImageName = "sanad/api-integration-service",
    [switch]$NoCache,
    [switch]$Help
)

# Colors for output
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

# Show help
if ($Help) {
    Write-Host "Usage: .\build-docker.ps1 [OPTIONS]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -Version VERSION         Set image version tag (default: latest)"
    Write-Host "  -ImageName NAME          Set image name (default: sanad/api-integration-service)"
    Write-Host "  -NoCache                 Build without using cache"
    Write-Host "  -Help                    Show this help message"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  .\build-docker.ps1                    # Build with default settings"
    Write-Host "  .\build-docker.ps1 -Version v1.0.0    # Build with version tag"
    Write-Host "  .\build-docker.ps1 -NoCache           # Build without cache"
    exit 0
}

# Configuration
$Dockerfile = "services/api-integration-service/Dockerfile"
$BuildContext = "."

# Check if we're in the project root
if (-not (Test-Path "Cargo.toml")) {
    Write-ColorOutput Red "Error: Must be run from project root directory"
    exit 1
}

# Check if Dockerfile exists
if (-not (Test-Path $Dockerfile)) {
    Write-ColorOutput Red "Error: Dockerfile not found at $Dockerfile"
    exit 1
}

Write-ColorOutput Green "Building Docker image..."
Write-Host "Image name: $ImageName:$Version" -ForegroundColor Yellow
Write-Host "Dockerfile: $Dockerfile" -ForegroundColor Yellow
Write-Host ""

# Build command
$buildArgs = @(
    "build",
    "-t", "$ImageName:$Version",
    "-f", $Dockerfile,
    $BuildContext
)

if ($NoCache) {
    $buildArgs += "--no-cache"
}

# Build the image
& docker $buildArgs

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-ColorOutput Green "✓ Build successful!"
    Write-Host ""
    Write-Host "Image: $ImageName:$Version"
    
    # Show image size
    $imageInfo = docker images "$ImageName:$Version" --format "{{.Size}}"
    Write-Host "Size: $imageInfo"
    
    Write-Host ""
    Write-Host "To run the container:"
    Write-Host "  docker run -d -p 8080:8080 $ImageName:$Version"
    Write-Host ""
    Write-Host "To push to registry:"
    Write-Host "  docker push $ImageName:$Version"
} else {
    Write-ColorOutput Red "✗ Build failed"
    exit 1
}

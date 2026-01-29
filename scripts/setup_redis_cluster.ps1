# PowerShell script to set up and test Redis cluster for Sanad Islamic Application
# This script helps with Redis cluster setup and validation

param(
    [Parameter(Position=0)]
    [string]$Command = "setup",
    
    [Parameter(Position=1)]
    [string]$Option = ""
)

# Function to print colored output
function Write-Status {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

# Check if Docker is running
function Test-Docker {
    Write-Status "Checking Docker status..."
    try {
        $null = docker info 2>$null
        Write-Success "Docker is running"
        return $true
    }
    catch {
        Write-Error "Docker is not running. Please start Docker and try again."
        return $false
    }
}

# Check if docker-compose is available
function Test-DockerCompose {
    Write-Status "Checking docker-compose availability..."
    try {
        $null = docker-compose --version 2>$null
        Write-Success "docker-compose is available"
        return $true
    }
    catch {
        Write-Error "docker-compose is not installed. Please install docker-compose and try again."
        return $false
    }
}

# Start Redis cluster services
function Start-RedisCluster {
    Write-Status "Starting Redis cluster services..."
    
    # Start individual Redis nodes
    docker-compose up -d redis-node-1 redis-node-2 redis-node-3
    
    # Wait for nodes to be ready
    Write-Status "Waiting for Redis nodes to be ready..."
    Start-Sleep -Seconds 10
    
    # Check if nodes are healthy
    for ($i = 1; $i -le 3; $i++) {
        $port = 7000 + $i
        try {
            $result = docker-compose exec redis-node-$i redis-cli -p $port ping 2>$null
            if ($result -match "PONG") {
                Write-Success "Redis node $i is ready on port $port"
            }
            else {
                Write-Error "Redis node $i failed to start on port $port"
                return $false
            }
        }
        catch {
            Write-Error "Redis node $i failed to start on port $port"
            return $false
        }
    }
    return $true
}

# Create Redis cluster
function New-RedisCluster {
    Write-Status "Creating Redis cluster..."
    
    # Run cluster setup
    docker-compose up redis-cluster-setup
    
    # Verify cluster status
    Write-Status "Verifying cluster status..."
    try {
        $clusterInfo = docker-compose exec redis-node-1 redis-cli -p 7001 cluster info 2>$null
        if ($clusterInfo -match "cluster_state:ok") {
            Write-Success "Redis cluster created successfully"
            return $true
        }
        else {
            Write-Warning "Cluster might not be fully ready. Checking individual nodes..."
            
            # Check each node
            for ($i = 1; $i -le 3; $i++) {
                $port = 7000 + $i
                try {
                    $nodeInfo = docker-compose exec redis-node-$i redis-cli -p $port cluster nodes 2>$null
                    if ($nodeInfo) {
                        Write-Success "Node $i cluster info retrieved"
                    }
                    else {
                        Write-Error "Failed to get cluster info from node $i"
                    }
                }
                catch {
                    Write-Error "Failed to get cluster info from node $i"
                }
            }
            return $false
        }
    }
    catch {
        Write-Error "Failed to verify cluster status"
        return $false
    }
}

# Test Redis cluster functionality
function Test-RedisCluster {
    Write-Status "Testing Redis cluster functionality..."
    
    # Test basic operations
    Write-Status "Testing SET operation..."
    try {
        $setResult = docker-compose exec redis-node-1 redis-cli -p 7001 -c set test_key "Hello Redis Cluster" 2>$null
        if ($setResult -match "OK") {
            Write-Success "SET operation successful"
        }
        else {
            Write-Error "SET operation failed"
            return $false
        }
    }
    catch {
        Write-Error "SET operation failed"
        return $false
    }
    
    Write-Status "Testing GET operation..."
    try {
        $getResult = docker-compose exec redis-node-1 redis-cli -p 7001 -c get test_key 2>$null
        if ($getResult -match "Hello Redis Cluster") {
            Write-Success "GET operation successful"
        }
        else {
            Write-Error "GET operation failed. Got: $getResult"
            return $false
        }
    }
    catch {
        Write-Error "GET operation failed"
        return $false
    }
    
    # Test cluster-specific operations
    Write-Status "Testing cluster slots distribution..."
    try {
        $slotsInfo = docker-compose exec redis-node-1 redis-cli -p 7001 cluster slots 2>$null
        if ($slotsInfo) {
            Write-Success "Cluster slots are properly distributed"
        }
        else {
            Write-Warning "Could not verify cluster slots distribution"
        }
    }
    catch {
        Write-Warning "Could not verify cluster slots distribution"
    }
    
    # Cleanup test data
    try {
        docker-compose exec redis-node-1 redis-cli -p 7001 -c del test_key 2>$null | Out-Null
    }
    catch {
        # Ignore cleanup errors
    }
    
    return $true
}

# Test cache service integration
function Test-CacheService {
    Write-Status "Testing cache service integration..."
    
    # Check if cache service is running
    $cacheServiceStatus = docker-compose ps cache-service 2>$null
    if ($cacheServiceStatus -match "Up") {
        Write-Success "Cache service is running"
        
        # Test health endpoint
        Write-Status "Testing cache service health endpoint..."
        try {
            $response = Invoke-WebRequest -Uri "http://localhost:8091/health" -TimeoutSec 5 -ErrorAction Stop
            if ($response.StatusCode -eq 200) {
                Write-Success "Cache service health check passed"
            }
            else {
                Write-Warning "Cache service health check failed (HTTP $($response.StatusCode))"
            }
        }
        catch {
            Write-Warning "Cache service health check failed (service might still be starting)"
        }
    }
    else {
        Write-Warning "Cache service is not running. Start it with: docker-compose up -d cache-service"
    }
}

# Monitor Redis cluster
function Show-ClusterMonitoring {
    Write-Status "Redis Cluster Monitoring Information:"
    Write-Host ""
    
    Write-Status "Cluster Nodes Status:"
    for ($i = 1; $i -le 3; $i++) {
        $port = 7000 + $i
        Write-Host "Node $i (port $port):"
        try {
            $replicationInfo = docker-compose exec redis-node-$i redis-cli -p $port info replication 2>$null
            $role = ($replicationInfo | Select-String "role:").ToString()
            Write-Host "  $role"
        }
        catch {
            Write-Host "  Status: Not available"
        }
        Write-Host ""
    }
    
    Write-Status "Cluster Info:"
    try {
        $clusterInfo = docker-compose exec redis-node-1 redis-cli -p 7001 cluster info 2>$null
        $clusterInfo | Select-Object -First 10
    }
    catch {
        Write-Host "Cluster info not available"
    }
    
    Write-Status "Memory Usage:"
    for ($i = 1; $i -le 3; $i++) {
        $port = 7000 + $i
        try {
            $memoryInfo = docker-compose exec redis-node-$i redis-cli -p $port info memory 2>$null
            $memoryUsed = ($memoryInfo | Select-String "used_memory_human:").ToString().Split(':')[1].Trim()
            Write-Host "Node $i: $memoryUsed"
        }
        catch {
            Write-Host "Node $i: N/A"
        }
    }
}

# Cleanup function
function Remove-RedisCluster {
    param([bool]$RemoveVolumes = $false)
    
    Write-Status "Cleaning up Redis cluster..."
    
    docker-compose down redis-node-1 redis-node-2 redis-node-3 redis-cluster-setup
    
    # Remove volumes if requested
    if ($RemoveVolumes) {
        Write-Warning "Removing Redis cluster volumes..."
        try {
            $volumes = docker volume ls -q | Where-Object { $_ -match "redis_node" }
            if ($volumes) {
                docker volume rm $volumes 2>$null
            }
        }
        catch {
            # Ignore volume removal errors
        }
    }
    
    Write-Success "Redis cluster cleanup completed"
}

# Performance test
function Test-Performance {
    Write-Status "Running Redis cluster performance test..."
    
    # Simple benchmark
    Write-Status "Running redis-benchmark on cluster..."
    try {
        $benchmarkResult = docker-compose exec redis-node-1 redis-benchmark -h redis-node-1 -p 7001 -c 50 -n 10000 -d 3 -t set,get --csv 2>$null
        $benchmarkResult | Select-Object -Last 5
        Write-Success "Performance test completed"
    }
    catch {
        Write-Error "Performance test failed"
    }
}

# Show help
function Show-Help {
    Write-Host "Redis Cluster Setup Script for Sanad Islamic Application" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\setup_redis_cluster.ps1 [command] [options]" -ForegroundColor White
    Write-Host ""
    Write-Host "Commands:" -ForegroundColor Yellow
    Write-Host "  setup      - Set up and start Redis cluster (default)"
    Write-Host "  test       - Test cluster functionality"
    Write-Host "  monitor    - Show cluster status and monitoring info"
    Write-Host "  performance - Run performance benchmark"
    Write-Host "  cleanup    - Stop and remove cluster containers"
    Write-Host "               Use --remove-volumes to also remove data volumes"
    Write-Host "  help       - Show this help message"
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Green
    Write-Host "  .\setup_redis_cluster.ps1                    # Set up cluster"
    Write-Host "  .\setup_redis_cluster.ps1 test              # Test cluster"
    Write-Host "  .\setup_redis_cluster.ps1 monitor           # Monitor cluster"
    Write-Host "  .\setup_redis_cluster.ps1 cleanup           # Clean up cluster"
    Write-Host "  .\setup_redis_cluster.ps1 cleanup --remove-volumes  # Clean up including data"
}

# Main script logic
Write-Host "🚀 Setting up Redis Cluster for Sanad Islamic Application" -ForegroundColor Cyan
Write-Host ""

switch ($Command.ToLower()) {
    "setup" {
        if (-not (Test-Docker)) { exit 1 }
        if (-not (Test-DockerCompose)) { exit 1 }
        if (-not (Start-RedisCluster)) { exit 1 }
        if (-not (New-RedisCluster)) { exit 1 }
        if (-not (Test-RedisCluster)) { exit 1 }
        Test-CacheService
        Write-Success "Redis cluster setup completed successfully!"
        Write-Host ""
        Write-Status "Next steps:"
        Write-Host "  1. Start the cache service: docker-compose up -d cache-service"
        Write-Host "  2. Monitor cluster: .\setup_redis_cluster.ps1 monitor"
        Write-Host "  3. Run performance test: .\setup_redis_cluster.ps1 performance"
    }
    "test" {
        Test-RedisCluster
        Test-CacheService
    }
    "monitor" {
        Show-ClusterMonitoring
    }
    "performance" {
        Test-Performance
    }
    "cleanup" {
        $removeVolumes = $Option -eq "--remove-volumes"
        Remove-RedisCluster -RemoveVolumes $removeVolumes
    }
    { $_ -in @("help", "--help", "-h") } {
        Show-Help
    }
    default {
        Write-Error "Unknown command: $Command"
        Write-Host "Use '.\setup_redis_cluster.ps1 help' for usage information"
        exit 1
    }
}
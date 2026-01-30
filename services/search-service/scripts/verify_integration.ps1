#!/usr/bin/env pwsh

# Advanced Semantic Search Service Integration Test
# Tests the complete search service functionality

Write-Host "🔍 Advanced Semantic Search Service Integration Test" -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan

# Configuration
$SERVICE_PORT = 8087
$BASE_URL = "http://localhost:$SERVICE_PORT"
$MAX_RETRIES = 30
$RETRY_DELAY = 2

# Test results tracking
$script:TestResults = @{
    Passed = 0
    Failed = 0
    Details = @()
}

function Add-TestResult {
    param(
        [string]$TestName,
        [bool]$Success,
        [string]$Details = ""
    )
    
    if ($Success) {
        $script:TestResults.Passed++
        Write-Host "✅ $TestName" -ForegroundColor Green
    } else {
        $script:TestResults.Failed++
        Write-Host "❌ $TestName" -ForegroundColor Red
        if ($Details) {
            Write-Host "   Details: $Details" -ForegroundColor Yellow
        }
    }
    
    $script:TestResults.Details += @{
        Name = $TestName
        Success = $Success
        Details = $Details
    }
}

function Test-ServiceHealth {
    try {
        Write-Host "`n🏥 Testing Service Health..." -ForegroundColor Yellow
        
        $response = Invoke-RestMethod -Uri "$BASE_URL/health" -Method Get -TimeoutSec 10
        
        $healthOk = $response.success -eq $true -and 
                   $response.data.status -eq "healthy" -and
                   $response.data.service -eq "semantic-search-service"
        
        Add-TestResult "Health Check" $healthOk
        
        if ($healthOk) {
            Write-Host "   Service: $($response.data.service)" -ForegroundColor Gray
            Write-Host "   Status: $($response.data.status)" -ForegroundColor Gray
            Write-Host "   Features: $($response.data.features)" -ForegroundColor Gray
            Write-Host "   Endpoints: $($response.data.endpoints)" -ForegroundColor Gray
        }
        
        return $healthOk
    }
    catch {
        Add-TestResult "Health Check" $false $_.Exception.Message
        return $false
    }
}

function Test-IndexSampleData {
    try {
        Write-Host "`n📚 Testing Sample Data Indexing..." -ForegroundColor Yellow
        
        $response = Invoke-RestMethod -Uri "$BASE_URL/index/sample" -Method Post -TimeoutSec 30
        
        $indexOk = $response.success -eq $true -and 
                  $response.data.successful_count -gt 0
        
        Add-TestResult "Sample Data Indexing" $indexOk
        
        if ($indexOk) {
            Write-Host "   Total Documents: $($response.data.total_documents)" -ForegroundColor Gray
            Write-Host "   Successful: $($response.data.successful_count)" -ForegroundColor Gray
            Write-Host "   Failed: $($response.data.failed_count)" -ForegroundColor Gray
            Write-Host "   Processing Time: $($response.data.processing_time_ms)ms" -ForegroundColor Gray
        }
        
        return $indexOk
    }
    catch {
        Add-TestResult "Sample Data Indexing" $false $_.Exception.Message
        return $false
    }
}

function Test-IndexStats {
    try {
        Write-Host "`n📊 Testing Index Statistics..." -ForegroundColor Yellow
        
        $response = Invoke-RestMethod -Uri "$BASE_URL/index/stats" -Method Get -TimeoutSec 10
        
        $statsOk = $response.success -eq $true -and 
                  $response.data.total_documents -gt 0
        
        Add-TestResult "Index Statistics" $statsOk
        
        if ($statsOk) {
            Write-Host "   Total Documents: $($response.data.total_documents)" -ForegroundColor Gray
            Write-Host "   Embedding Model: $($response.data.embedding_model)" -ForegroundColor Gray
            Write-Host "   Vector Dimensions: $($response.data.vector_dimensions)" -ForegroundColor Gray
        }
        
        return $statsOk
    }
    catch {
        Add-TestResult "Index Statistics" $false $_.Exception.Message
        return $false
    }
}

function Test-SemanticSearch {
    try {
        Write-Host "`n🔍 Testing Semantic Search..." -ForegroundColor Yellow
        
        # Test Arabic search
        $arabicQuery = "بسم الله"
        $response = Invoke-RestMethod -Uri "$BASE_URL/search/semantic?query=$([System.Web.HttpUtility]::UrlEncode($arabicQuery))&limit=5" -Method Get -TimeoutSec 15
        
        $searchOk = $response.success -eq $true -and 
                   $response.data.results.Count -gt 0
        
        Add-TestResult "Arabic Semantic Search" $searchOk
        
        if ($searchOk) {
            Write-Host "   Query: $arabicQuery" -ForegroundColor Gray
            Write-Host "   Results Found: $($response.data.results.Count)" -ForegroundColor Gray
            Write-Host "   Search Time: $($response.data.search_time_ms)ms" -ForegroundColor Gray
            Write-Host "   Embedding Time: $($response.data.query_embedding_time_ms)ms" -ForegroundColor Gray
            
            # Show first result
            if ($response.data.results.Count -gt 0) {
                $firstResult = $response.data.results[0]
                Write-Host "   Top Result:" -ForegroundColor Gray
                Write-Host "     - Content Type: $($firstResult.document.content_type)" -ForegroundColor Gray
                Write-Host "     - Similarity: $($firstResult.similarity_score)" -ForegroundColor Gray
                Write-Host "     - Text Preview: $($firstResult.document.text.Substring(0, [Math]::Min(50, $firstResult.document.text.Length)))..." -ForegroundColor Gray
            }
        }
        
        return $searchOk
    }
    catch {
        Add-TestResult "Arabic Semantic Search" $false $_.Exception.Message
        return $false
    }
}

function Test-SearchWithFilters {
    try {
        Write-Host "`n🎯 Testing Search with Filters..." -ForegroundColor Yellow
        
        # Test content type filtering
        $query = "الله"
        $response = Invoke-RestMethod -Uri "$BASE_URL/search/semantic?query=$([System.Web.HttpUtility]::UrlEncode($query))&content_types=quran&limit=3" -Method Get -TimeoutSec 15
        
        $filterOk = $response.success -eq $true
        
        Add-TestResult "Content Type Filtering" $filterOk
        
        if ($filterOk) {
            Write-Host "   Query: $query" -ForegroundColor Gray
            Write-Host "   Filter: content_types=quran" -ForegroundColor Gray
            Write-Host "   Results Found: $($response.data.results.Count)" -ForegroundColor Gray
            
            # Verify all results are Quran content
            $allQuran = $true
            foreach ($result in $response.data.results) {
                if ($result.document.content_type -ne "quran") {
                    $allQuran = $false
                    break
                }
            }
            Add-TestResult "Filter Accuracy" $allQuran
        }
        
        return $filterOk
    }
    catch {
        Add-TestResult "Content Type Filtering" $false $_.Exception.Message
        return $false
    }
}

function Test-QuerySuggestions {
    try {
        Write-Host "`n💡 Testing Query Suggestions..." -ForegroundColor Yellow
        
        $query = "صلاة"
        $response = Invoke-RestMethod -Uri "$BASE_URL/search/suggestions?query=$([System.Web.HttpUtility]::UrlEncode($query))&limit=5" -Method Get -TimeoutSec 10
        
        $suggestionsOk = $response.success -eq $true
        
        Add-TestResult "Query Suggestions" $suggestionsOk
        
        if ($suggestionsOk) {
            Write-Host "   Query: $query" -ForegroundColor Gray
            Write-Host "   Suggestions Found: $($response.data.Count)" -ForegroundColor Gray
            
            if ($response.data.Count -gt 0) {
                Write-Host "   Sample Suggestions:" -ForegroundColor Gray
                for ($i = 0; $i -lt [Math]::Min(3, $response.data.Count); $i++) {
                    $suggestion = $response.data[$i]
                    Write-Host "     - $($suggestion.suggested_query) (Score: $($suggestion.similarity_score))" -ForegroundColor Gray
                }
            }
        }
        
        return $suggestionsOk
    }
    catch {
        Add-TestResult "Query Suggestions" $suggestionsOk $_.Exception.Message
        return $false
    }
}

function Test-SimilarDocuments {
    try {
        Write-Host "`n🔗 Testing Similar Documents..." -ForegroundColor Yellow
        
        # Use a known document ID (this would be from indexed sample data)
        $documentId = "quran_001_001"
        $response = Invoke-RestMethod -Uri "$BASE_URL/search/similar?document_id=$documentId&limit=3" -Method Get -TimeoutSec 10
        
        $similarOk = $response.success -eq $true
        
        Add-TestResult "Similar Documents Search" $similarOk
        
        if ($similarOk) {
            Write-Host "   Document ID: $documentId" -ForegroundColor Gray
            Write-Host "   Similar Documents Found: $($response.data.Count)" -ForegroundColor Gray
            
            if ($response.data.Count -gt 0) {
                Write-Host "   Top Similar Document:" -ForegroundColor Gray
                $topSimilar = $response.data[0]
                Write-Host "     - ID: $($topSimilar.document.id)" -ForegroundColor Gray
                Write-Host "     - Similarity: $($topSimilar.similarity_score)" -ForegroundColor Gray
                Write-Host "     - Content Type: $($topSimilar.document.content_type)" -ForegroundColor Gray
            }
        }
        
        return $similarOk
    }
    catch {
        Add-TestResult "Similar Documents Search" $false $_.Exception.Message
        return $false
    }
}

function Test-IndexValidation {
    try {
        Write-Host "`n✅ Testing Index Validation..." -ForegroundColor Yellow
        
        $response = Invoke-RestMethod -Uri "$BASE_URL/index/validate" -Method Get -TimeoutSec 15
        
        $validationOk = $response.success -eq $true -and 
                       $response.data.is_valid -eq $true
        
        Add-TestResult "Index Validation" $validationOk
        
        if ($validationOk) {
            Write-Host "   Index Valid: $($response.data.is_valid)" -ForegroundColor Gray
            Write-Host "   Total Documents: $($response.data.total_documents)" -ForegroundColor Gray
            Write-Host "   Issues Found: $($response.data.issues.Count)" -ForegroundColor Gray
        }
        
        return $validationOk
    }
    catch {
        Add-TestResult "Index Validation" $false $_.Exception.Message
        return $false
    }
}

function Wait-ForService {
    Write-Host "`n⏳ Waiting for service to be ready..." -ForegroundColor Yellow
    
    for ($i = 1; $i -le $MAX_RETRIES; $i++) {
        try {
            $response = Invoke-RestMethod -Uri "$BASE_URL/health" -Method Get -TimeoutSec 5
            if ($response.success -eq $true) {
                Write-Host "✅ Service is ready!" -ForegroundColor Green
                return $true
            }
        }
        catch {
            Write-Host "   Attempt $i/$MAX_RETRIES - Service not ready yet..." -ForegroundColor Gray
        }
        
        if ($i -lt $MAX_RETRIES) {
            Start-Sleep -Seconds $RETRY_DELAY
        }
    }
    
    Write-Host "❌ Service failed to start within timeout" -ForegroundColor Red
    return $false
}

function Show-TestSummary {
    Write-Host "`n" -NoNewline
    Write-Host "📋 Test Summary" -ForegroundColor Cyan
    Write-Host "===============" -ForegroundColor Cyan
    Write-Host "✅ Passed: $($script:TestResults.Passed)" -ForegroundColor Green
    Write-Host "❌ Failed: $($script:TestResults.Failed)" -ForegroundColor Red
    Write-Host "📊 Total:  $($script:TestResults.Passed + $script:TestResults.Failed)" -ForegroundColor Blue
    
    $successRate = if (($script:TestResults.Passed + $script:TestResults.Failed) -gt 0) {
        [math]::Round(($script:TestResults.Passed / ($script:TestResults.Passed + $script:TestResults.Failed)) * 100, 1)
    } else { 0 }
    
    Write-Host "🎯 Success Rate: $successRate%" -ForegroundColor $(if ($successRate -ge 80) { "Green" } elseif ($successRate -ge 60) { "Yellow" } else { "Red" })
    
    if ($script:TestResults.Failed -gt 0) {
        Write-Host "`n❌ Failed Tests:" -ForegroundColor Red
        foreach ($result in $script:TestResults.Details) {
            if (-not $result.Success) {
                Write-Host "   - $($result.Name)" -ForegroundColor Red
                if ($result.Details) {
                    Write-Host "     $($result.Details)" -ForegroundColor Gray
                }
            }
        }
    }
}

# Main execution
try {
    # Check if service is running
    if (-not (Wait-ForService)) {
        Write-Host "❌ Cannot connect to search service at $BASE_URL" -ForegroundColor Red
        Write-Host "   Please ensure the service is running on port $SERVICE_PORT" -ForegroundColor Yellow
        exit 1
    }
    
    # Run all tests
    $healthOk = Test-ServiceHealth
    
    if ($healthOk) {
        $indexOk = Test-IndexSampleData
        
        if ($indexOk) {
            # Wait a moment for indexing to complete
            Start-Sleep -Seconds 2
            
            Test-IndexStats
            Test-SemanticSearch
            Test-SearchWithFilters
            Test-QuerySuggestions
            Test-SimilarDocuments
            Test-IndexValidation
        } else {
            Write-Host "⚠️  Skipping search tests due to indexing failure" -ForegroundColor Yellow
        }
    } else {
        Write-Host "⚠️  Skipping all tests due to health check failure" -ForegroundColor Yellow
    }
    
    Show-TestSummary
    
    # Exit with appropriate code
    if ($script:TestResults.Failed -eq 0) {
        Write-Host "`n🎉 All tests passed! Advanced Semantic Search Service is working correctly." -ForegroundColor Green
        exit 0
    } else {
        Write-Host "`n⚠️  Some tests failed. Please check the service implementation." -ForegroundColor Yellow
        exit 1
    }
}
catch {
    Write-Host "`n💥 Test execution failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}
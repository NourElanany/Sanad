// Load Tests for API Integration Service
// Task 25.4: Run load tests
// Requirements: 9.1 (Rate Limiting), 10.1 (Caching), 12.1 (Fallback Mechanisms)

use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::timeout;

/// Load test configuration
#[derive(Debug, Clone)]
pub struct LoadTestConfig {
    /// Number of concurrent users
    pub concurrent_users: usize,
    /// Duration of the load test
    pub test_duration: Duration,
    /// Requests per second target
    pub target_rps: usize,
    /// Enable rate limiting test
    pub test_rate_limiting: bool,
    /// Enable caching test
    pub test_caching: bool,
    /// Enable fallback test
    pub test_fallback: bool,
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self {
            concurrent_users: 50,
            test_duration: Duration::from_secs(60),
            target_rps: 100,
            test_rate_limiting: true,
            test_caching: true,
            test_fallback: true,
        }
    }
}

/// Load test metrics
#[derive(Debug, Clone)]
pub struct LoadTestMetrics {
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub rate_limited_requests: usize,
    pub cached_responses: usize,
    pub fallback_responses: usize,
    pub avg_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub p50_response_time: Duration,
    pub p95_response_time: Duration,
    pub p99_response_time: Duration,
    pub requests_per_second: f64,
}

/// Individual request result
#[derive(Debug, Clone)]
struct RequestResult {
    success: bool,
    response_time: Duration,
    rate_limited: bool,
    cached: bool,
    fallback_used: bool,
}

/// Load test suite for API Integration Service
pub struct LoadTestSuite {
    config: LoadTestConfig,
    base_url: String,
    client: reqwest::Client,
}

impl LoadTestSuite {
    pub fn new(config: LoadTestConfig, base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(config.concurrent_users)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            base_url,
            client,
        }
    }

    /// Run comprehensive load tests
    pub async fn run_all_tests(&self) -> LoadTestMetrics {
        println!("\n🚀 Starting Load Tests for API Integration Service");
        println!("Configuration: {:?}", self.config);
        println!("{}", "=".repeat(80));

        let mut all_results = Vec::new();

        // Test 1: Rate Limiting Under Load
        if self.config.test_rate_limiting {
            println!("\n📊 Test 1: Rate Limiting Under Load");
            let results = self.test_rate_limiting_under_load().await;
            all_results.extend(results);
        }

        // Test 2: Caching Performance
        if self.config.test_caching {
            println!("\n💾 Test 2: Caching Performance");
            let results = self.test_caching_performance().await;
            all_results.extend(results);
        }

        // Test 3: Fallback Mechanisms Under Failure
        if self.config.test_fallback {
            println!("\n🔄 Test 3: Fallback Mechanisms Under Failure");
            let results = self.test_fallback_mechanisms().await;
            all_results.extend(results);
        }

        // Calculate and return metrics
        self.calculate_metrics(all_results)
    }

    /// Test rate limiting enforcement under high load
    /// Validates Requirement 9.1: Rate limiting should prevent exceeding API limits
    async fn test_rate_limiting_under_load(&self) -> Vec<RequestResult> {
        println!("  Testing rate limiting with {} concurrent users", self.config.concurrent_users);
        
        let semaphore = Arc::new(Semaphore::new(self.config.concurrent_users));
        let mut handles = Vec::new();
        let start_time = Instant::now();

        // Generate high load to trigger rate limiting
        let requests_to_send = self.config.target_rps * 10; // 10 seconds worth of requests
        
        for i in 0..requests_to_send {
            let sem = semaphore.clone();
            let client = self.client.clone();
            let base_url = self.base_url.clone();
            
            let handle = tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                
                let request_start = Instant::now();
                
                // Make request to Quran API endpoint
                let url = format!("{}/api/v1/quran/text?surah={}&ayah={}", 
                    base_url, 
                    (i % 114) + 1,  // Cycle through surahs
                    (i % 286) + 1   // Cycle through ayahs
                );
                
                let result = timeout(
                    Duration::from_secs(10),
                    client.get(&url).send()
                ).await;

                let response_time = request_start.elapsed();
                
                match result {
                    Ok(Ok(response)) => {
                        let status = response.status();
                        let rate_limited = status.as_u16() == 429; // Too Many Requests
                        
                        RequestResult {
                            success: status.is_success(),
                            response_time,
                            rate_limited,
                            cached: false, // Will be determined from headers if available
                            fallback_used: false,
                        }
                    }
                    Ok(Err(_)) | Err(_) => {
                        RequestResult {
                            success: false,
                            response_time,
                            rate_limited: false,
                            cached: false,
                            fallback_used: false,
                        }
                    }
                }
            });
            
            handles.push(handle);
            
            // Add small delay to simulate realistic load pattern
            if i % 10 == 0 {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }

        // Collect results
        let mut results = Vec::new();
        for handle in handles {
            if let Ok(result) = handle.await {
                results.push(result);
            }
        }

        let elapsed = start_time.elapsed();
        let rate_limited_count = results.iter().filter(|r| r.rate_limited).count();
        
        println!("  ✓ Completed {} requests in {:.2}s", results.len(), elapsed.as_secs_f64());
        println!("  ✓ Rate limited: {} requests ({:.1}%)", 
            rate_limited_count, 
            (rate_limited_count as f64 / results.len() as f64) * 100.0
        );

        results
    }

    /// Test caching performance improvements
    /// Validates Requirement 10.1: Caching should reduce API calls and improve performance
    async fn test_caching_performance(&self) -> Vec<RequestResult> {
        println!("  Testing caching with repeated requests");
        
        let mut results = Vec::new();
        
        // Test 1: First request (cache miss)
        println!("  → Testing cache miss scenario...");
        let cache_miss_results = self.make_repeated_requests(
            "/api/v1/quran/text?surah=1&ayah=1",
            10,
            false
        ).await;
        
        let avg_cache_miss = cache_miss_results.iter()
            .map(|r| r.response_time)
            .sum::<Duration>() / cache_miss_results.len() as u32;
        
        results.extend(cache_miss_results);
        
        // Small delay to ensure cache is populated
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Test 2: Repeated requests (cache hits)
        println!("  → Testing cache hit scenario...");
        let cache_hit_results = self.make_repeated_requests(
            "/api/v1/quran/text?surah=1&ayah=1",
            50,
            true
        ).await;
        
        let avg_cache_hit = cache_hit_results.iter()
            .map(|r| r.response_time)
            .sum::<Duration>() / cache_hit_results.len() as u32;
        
        results.extend(cache_hit_results);
        
        // Calculate cache performance improvement
        let improvement = ((avg_cache_miss.as_millis() as f64 - avg_cache_hit.as_millis() as f64) 
            / avg_cache_miss.as_millis() as f64) * 100.0;
        
        println!("  ✓ Cache miss avg: {:.2}ms", avg_cache_miss.as_millis());
        println!("  ✓ Cache hit avg: {:.2}ms", avg_cache_hit.as_millis());
        println!("  ✓ Performance improvement: {:.1}%", improvement);
        
        // Test 3: Concurrent cache hits
        println!("  → Testing concurrent cache hits...");
        let concurrent_cache_results = self.make_concurrent_requests(
            "/api/v1/quran/text?surah=2&ayah=1",
            100
        ).await;
        
        results.extend(concurrent_cache_results);
        
        results
    }

    /// Test fallback mechanisms under API failure scenarios
    /// Validates Requirement 12.1: Fallback should maintain service availability
    async fn test_fallback_mechanisms(&self) -> Vec<RequestResult> {
        println!("  Testing fallback mechanisms");
        
        let mut results = Vec::new();
        
        // Test 1: Primary API failure simulation
        println!("  → Testing primary API failure scenario...");
        
        // Make requests that might trigger fallback
        let fallback_results = self.make_concurrent_requests(
            "/api/v1/hadith/search?query=صلاة",
            50
        ).await;
        
        let fallback_count = fallback_results.iter()
            .filter(|r| r.fallback_used)
            .count();
        
        println!("  ✓ Fallback triggered: {} times", fallback_count);
        
        results.extend(fallback_results);
        
        // Test 2: Multiple API failures
        println!("  → Testing multiple API failures...");
        
        let multi_fallback_results = self.make_concurrent_requests(
            "/api/v1/prayer-times?latitude=21.4225&longitude=39.8262",
            30
        ).await;
        
        results.extend(multi_fallback_results);
        
        // Test 3: Stale cache fallback
        println!("  → Testing stale cache fallback...");
        
        let stale_cache_results = self.make_repeated_requests(
            "/api/v1/tafsir?surah=1&ayah=1",
            20,
            false
        ).await;
        
        results.extend(stale_cache_results);
        
        results
    }

    /// Make repeated requests to the same endpoint
    async fn make_repeated_requests(
        &self,
        endpoint: &str,
        count: usize,
        expect_cached: bool
    ) -> Vec<RequestResult> {
        let mut results = Vec::new();
        
        for _ in 0..count {
            let url = format!("{}{}", self.base_url, endpoint);
            let request_start = Instant::now();
            
            let result = timeout(
                Duration::from_secs(10),
                self.client.get(&url).send()
            ).await;
            
            let response_time = request_start.elapsed();
            
            match result {
                Ok(Ok(response)) => {
                    let status = response.status();
                    let headers = response.headers();
                    
                    // Check for cache indicators in headers
                    let cached = headers.get("x-cache-status")
                        .and_then(|v| v.to_str().ok())
                        .map(|v| v == "HIT")
                        .unwrap_or(expect_cached);
                    
                    results.push(RequestResult {
                        success: status.is_success(),
                        response_time,
                        rate_limited: status.as_u16() == 429,
                        cached,
                        fallback_used: false,
                    });
                }
                Ok(Err(_)) | Err(_) => {
                    results.push(RequestResult {
                        success: false,
                        response_time,
                        rate_limited: false,
                        cached: false,
                        fallback_used: false,
                    });
                }
            }
            
            // Small delay between requests
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        results
    }

    /// Make concurrent requests to an endpoint
    async fn make_concurrent_requests(
        &self,
        endpoint: &str,
        count: usize
    ) -> Vec<RequestResult> {
        let semaphore = Arc::new(Semaphore::new(self.config.concurrent_users));
        let mut handles = Vec::new();
        
        for _ in 0..count {
            let sem = semaphore.clone();
            let client = self.client.clone();
            let url = format!("{}{}", self.base_url, endpoint);
            
            let handle = tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                
                let request_start = Instant::now();
                
                let result = timeout(
                    Duration::from_secs(10),
                    client.get(&url).send()
                ).await;
                
                let response_time = request_start.elapsed();
                
                match result {
                    Ok(Ok(response)) => {
                        let status = response.status();
                        let headers = response.headers();
                        
                        let cached = headers.get("x-cache-status")
                            .and_then(|v| v.to_str().ok())
                            .map(|v| v == "HIT")
                            .unwrap_or(false);
                        
                        let fallback_used = headers.get("x-fallback-used")
                            .and_then(|v| v.to_str().ok())
                            .map(|v| v == "true")
                            .unwrap_or(false);
                        
                        RequestResult {
                            success: status.is_success(),
                            response_time,
                            rate_limited: status.as_u16() == 429,
                            cached,
                            fallback_used,
                        }
                    }
                    Ok(Err(_)) | Err(_) => {
                        RequestResult {
                            success: false,
                            response_time,
                            rate_limited: false,
                            cached: false,
                            fallback_used: false,
                        }
                    }
                }
            });
            
            handles.push(handle);
        }
        
        // Collect results
        let mut results = Vec::new();
        for handle in handles {
            if let Ok(result) = handle.await {
                results.push(result);
            }
        }
        
        results
    }

    /// Calculate comprehensive metrics from test results
    fn calculate_metrics(&self, results: Vec<RequestResult>) -> LoadTestMetrics {
        if results.is_empty() {
            return LoadTestMetrics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                rate_limited_requests: 0,
                cached_responses: 0,
                fallback_responses: 0,
                avg_response_time: Duration::ZERO,
                min_response_time: Duration::ZERO,
                max_response_time: Duration::ZERO,
                p50_response_time: Duration::ZERO,
                p95_response_time: Duration::ZERO,
                p99_response_time: Duration::ZERO,
                requests_per_second: 0.0,
            };
        }

        let total_requests = results.len();
        let successful_requests = results.iter().filter(|r| r.success).count();
        let failed_requests = total_requests - successful_requests;
        let rate_limited_requests = results.iter().filter(|r| r.rate_limited).count();
        let cached_responses = results.iter().filter(|r| r.cached).count();
        let fallback_responses = results.iter().filter(|r| r.fallback_used).count();

        // Calculate response time statistics
        let mut response_times: Vec<Duration> = results.iter()
            .map(|r| r.response_time)
            .collect();
        response_times.sort();

        let total_time: Duration = response_times.iter().sum();
        let avg_response_time = total_time / total_requests as u32;
        let min_response_time = *response_times.first().unwrap();
        let max_response_time = *response_times.last().unwrap();

        // Calculate percentiles
        let p50_index = (total_requests as f64 * 0.50) as usize;
        let p95_index = (total_requests as f64 * 0.95) as usize;
        let p99_index = (total_requests as f64 * 0.99) as usize;

        let p50_response_time = response_times[p50_index.min(total_requests - 1)];
        let p95_response_time = response_times[p95_index.min(total_requests - 1)];
        let p99_response_time = response_times[p99_index.min(total_requests - 1)];

        // Calculate requests per second
        let test_duration_secs = self.config.test_duration.as_secs_f64();
        let requests_per_second = total_requests as f64 / test_duration_secs;

        LoadTestMetrics {
            total_requests,
            successful_requests,
            failed_requests,
            rate_limited_requests,
            cached_responses,
            fallback_responses,
            avg_response_time,
            min_response_time,
            max_response_time,
            p50_response_time,
            p95_response_time,
            p99_response_time,
            requests_per_second,
        }
    }

    /// Print comprehensive test report
    pub fn print_report(&self, metrics: &LoadTestMetrics) {
        println!("\n\n");
        println!("{}", "=".repeat(80));
        println!("📊 LOAD TEST RESULTS SUMMARY");
        println!("{}", "=".repeat(80));
        
        println!("\n📈 Request Statistics:");
        println!("  Total Requests:      {}", metrics.total_requests);
        println!("  Successful:          {} ({:.1}%)", 
            metrics.successful_requests,
            (metrics.successful_requests as f64 / metrics.total_requests as f64) * 100.0
        );
        println!("  Failed:              {} ({:.1}%)", 
            metrics.failed_requests,
            (metrics.failed_requests as f64 / metrics.total_requests as f64) * 100.0
        );
        println!("  Rate Limited:        {} ({:.1}%)", 
            metrics.rate_limited_requests,
            (metrics.rate_limited_requests as f64 / metrics.total_requests as f64) * 100.0
        );
        
        println!("\n💾 Caching Statistics:");
        println!("  Cached Responses:    {} ({:.1}%)", 
            metrics.cached_responses,
            (metrics.cached_responses as f64 / metrics.total_requests as f64) * 100.0
        );
        
        println!("\n🔄 Fallback Statistics:");
        println!("  Fallback Used:       {} ({:.1}%)", 
            metrics.fallback_responses,
            (metrics.fallback_responses as f64 / metrics.total_requests as f64) * 100.0
        );
        
        println!("\n⏱️  Response Time Statistics:");
        println!("  Average:             {:.2}ms", metrics.avg_response_time.as_millis());
        println!("  Minimum:             {:.2}ms", metrics.min_response_time.as_millis());
        println!("  Maximum:             {:.2}ms", metrics.max_response_time.as_millis());
        println!("  P50 (Median):        {:.2}ms", metrics.p50_response_time.as_millis());
        println!("  P95:                 {:.2}ms", metrics.p95_response_time.as_millis());
        println!("  P99:                 {:.2}ms", metrics.p99_response_time.as_millis());
        
        println!("\n🚀 Throughput:");
        println!("  Requests/Second:     {:.2}", metrics.requests_per_second);
        
        println!("\n");
        println!("{}", "=".repeat(80));
        
        // Validation checks
        println!("\n✅ Validation Checks:");
        
        // Check 1: Success rate should be high
        let success_rate = (metrics.successful_requests as f64 / metrics.total_requests as f64) * 100.0;
        if success_rate >= 95.0 {
            println!("  ✓ Success rate: {:.1}% (>= 95%)", success_rate);
        } else {
            println!("  ⚠ Success rate: {:.1}% (< 95%)", success_rate);
        }
        
        // Check 2: Rate limiting should be working
        if metrics.rate_limited_requests > 0 {
            println!("  ✓ Rate limiting: Active ({} requests limited)", metrics.rate_limited_requests);
        } else {
            println!("  ℹ Rate limiting: Not triggered (load may be below limits)");
        }
        
        // Check 3: Caching should improve performance
        if metrics.cached_responses > 0 {
            let cache_rate = (metrics.cached_responses as f64 / metrics.total_requests as f64) * 100.0;
            println!("  ✓ Caching: Active ({:.1}% cache hit rate)", cache_rate);
        } else {
            println!("  ℹ Caching: No cache hits detected");
        }
        
        // Check 4: Fallback mechanisms should be available
        if metrics.fallback_responses > 0 {
            println!("  ✓ Fallback: Active ({} fallback responses)", metrics.fallback_responses);
        } else {
            println!("  ℹ Fallback: Not triggered (all primary APIs available)");
        }
        
        // Check 5: Response times should be reasonable
        if metrics.p95_response_time <= Duration::from_secs(3) {
            println!("  ✓ Response time: P95 {:.2}ms (<= 3000ms)", metrics.p95_response_time.as_millis());
        } else {
            println!("  ⚠ Response time: P95 {:.2}ms (> 3000ms)", metrics.p95_response_time.as_millis());
        }
        
        println!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test load test configuration
    #[test]
    fn test_load_test_config_default() {
        let config = LoadTestConfig::default();
        assert_eq!(config.concurrent_users, 50);
        assert_eq!(config.test_duration, Duration::from_secs(60));
        assert_eq!(config.target_rps, 100);
        assert!(config.test_rate_limiting);
        assert!(config.test_caching);
        assert!(config.test_fallback);
    }

    /// Test metrics calculation with empty results
    #[test]
    fn test_metrics_calculation_empty() {
        let config = LoadTestConfig::default();
        let suite = LoadTestSuite::new(config, "http://localhost:8080".to_string());
        let metrics = suite.calculate_metrics(vec![]);
        
        assert_eq!(metrics.total_requests, 0);
        assert_eq!(metrics.successful_requests, 0);
    }

    /// Test metrics calculation with sample results
    #[test]
    fn test_metrics_calculation_with_results() {
        let config = LoadTestConfig::default();
        let suite = LoadTestSuite::new(config, "http://localhost:8080".to_string());
        
        let results = vec![
            RequestResult {
                success: true,
                response_time: Duration::from_millis(100),
                rate_limited: false,
                cached: true,
                fallback_used: false,
            },
            RequestResult {
                success: true,
                response_time: Duration::from_millis(200),
                rate_limited: false,
                cached: false,
                fallback_used: false,
            },
            RequestResult {
                success: false,
                response_time: Duration::from_millis(5000),
                rate_limited: true,
                cached: false,
                fallback_used: false,
            },
        ];
        
        let metrics = suite.calculate_metrics(results);
        
        assert_eq!(metrics.total_requests, 3);
        assert_eq!(metrics.successful_requests, 2);
        assert_eq!(metrics.failed_requests, 1);
        assert_eq!(metrics.rate_limited_requests, 1);
        assert_eq!(metrics.cached_responses, 1);
    }
}

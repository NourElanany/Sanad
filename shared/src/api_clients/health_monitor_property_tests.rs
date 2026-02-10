//! Property-based tests for Health Monitor
//!
//! **Property 23: Periodic Health Checks**
//! **Validates: Requirements 13.1**

use super::*;
use crate::api_clients::traits::ApiClient;
use async_trait::async_trait;
use proptest::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

// Mock API client for testing
struct MockHealthCheckClient {
    name: String,
    healthy: Arc<AtomicBool>,
    check_count: Arc<AtomicU32>,
}

#[async_trait]
impl ApiClient for MockHealthCheckClient {
    type Request = ();
    type Response = ();
    
    fn api_name(&self) -> &str {
        &self.name
    }
    
    fn priority(&self) -> u8 {
        1
    }
    
    async fn is_healthy(&self) -> bool {
        self.check_count.fetch_add(1, Ordering::SeqCst);
        self.healthy.load(Ordering::SeqCst)
    }
    
    async fn request(&self, _req: Self::Request) -> Result<Self::Response, ApiError> {
        Ok(())
    }
}

proptest! {
    #![proptest_config(ProptestConfig {cases: 100, .. ProptestConfig::default()})]
    
    /// **Property 23: Periodic Health Checks**
    /// 
    /// Health checks should be performed periodically for all registered APIs.
    /// Each API should be checked at the configured interval.
    /// 
    /// **Validates: Requirements 13.1**
    #[test]
    fn prop_periodic_health_checks_execute(
        initial_healthy in prop::bool::ANY,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Create health monitor with short interval for testing
            let config = HealthMonitorConfig {
                check_interval: Duration::from_millis(100),
                unhealthy_threshold: 3,
                recovery_threshold: 2,
            };
            let monitor = Arc::new(HealthMonitor::new(config));
            
            // Create mock client
            let healthy = Arc::new(AtomicBool::new(initial_healthy));
            let check_count = Arc::new(AtomicU32::new(0));
            let client = Arc::new(MockHealthCheckClient {
                name: "test-api".to_string(),
                healthy: healthy.clone(),
                check_count: check_count.clone(),
            });
            
            // Register the API
            monitor.register_api("test-api").await;
            
            // Start monitoring
            let monitor_clone = monitor.clone();
            let clients: Vec<Arc<dyn ApiClient<Request = (), Response = ()>>> = vec![client];
            monitor_clone.start_monitoring(clients).await;
            
            // Wait for multiple check intervals
            sleep(Duration::from_millis(350)).await;
            
            // Stop monitoring
            monitor.stop_monitoring().await;
            
            // Verify checks were performed
            let checks = check_count.load(Ordering::SeqCst);
            prop_assert!(checks >= 2, "Expected at least 2 health checks, got {}", checks);
            
            // Verify status was updated
            let status = monitor.get_status("test-api").await;
            prop_assert!(status.is_some(), "API status should be available");
            
            let status = status.unwrap();
            prop_assert!(status.total_checks >= 2, "Total checks should be at least 2");
            prop_assert_eq!(status.is_healthy, initial_healthy || status.consecutive_successes >= 2);
        });
    }
    
    /// **Property 22: Primary API Recovery Detection**
    /// 
    /// When an unhealthy API recovers (consecutive successes >= recovery_threshold),
    /// it should be automatically marked as healthy again.
    /// 
    /// **Validates: Requirements 12.5**
    #[test]
    fn prop_recovery_detection(
        initial_failures in 3u32..10u32,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let monitor = HealthMonitor::with_defaults();
            
            // Register API
            monitor.register_api("test-api").await;
            
            // Simulate failures to mark as unhealthy
            {
                let mut status_map = monitor.status_map.write().await;
                let status = status_map.get_mut("test-api").unwrap();
                for _ in 0..initial_failures {
                    status.record_failure();
                }
                prop_assert!(!status.is_healthy, "API should be unhealthy after {} failures", initial_failures);
            }
            
            // Simulate recovery with 2 successes
            {
                let mut status_map = monitor.status_map.write().await;
                let status = status_map.get_mut("test-api").unwrap();
                status.record_success(Duration::from_millis(100));
                status.record_success(Duration::from_millis(100));
                prop_assert!(status.is_healthy, "API should recover after 2 consecutive successes");
                prop_assert_eq!(status.consecutive_successes, 2);
                prop_assert_eq!(status.consecutive_failures, 0);
            }
        });
    }
    
    /// **Property 25: Automatic Fallback on Unhealthy Status**
    /// 
    /// When an API is marked as unhealthy (consecutive_failures >= unhealthy_threshold),
    /// the health monitor should report it as unhealthy, triggering fallback mechanisms.
    /// 
    /// **Validates: Requirements 13.3**
    #[test]
    fn prop_automatic_fallback_on_unhealthy(
        failure_count in 3u32..10u32,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let config = HealthMonitorConfig {
                check_interval: Duration::from_secs(60),
                unhealthy_threshold: 3,
                recovery_threshold: 2,
            };
            let monitor = HealthMonitor::new(config);
            
            // Register API
            monitor.register_api("test-api").await;
            
            // Simulate failures
            {
                let mut status_map = monitor.status_map.write().await;
                let status = status_map.get_mut("test-api").unwrap();
                for _ in 0..failure_count {
                    status.record_failure();
                }
            }
            
            // Check if API is reported as unhealthy
            let is_healthy = monitor.is_api_healthy("test-api").await;
            
            if failure_count >= 3 {
                prop_assert!(!is_healthy, "API should be unhealthy after {} failures", failure_count);
            }
            
            // Verify status details
            let status = monitor.get_status("test-api").await.unwrap();
            prop_assert_eq!(status.consecutive_failures, failure_count);
            prop_assert_eq!(status.total_failures, failure_count as u64);
            
            if failure_count >= 3 {
                prop_assert!(!status.is_healthy);
            }
        });
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_health_status_tracks_metrics() {
        let mut status = ApiHealthStatus::new("test-api");
        
        // Record some successes and failures
        status.record_success(Duration::from_millis(100));
        status.record_success(Duration::from_millis(150));
        status.record_failure();
        status.record_success(Duration::from_millis(120));
        
        assert_eq!(status.total_checks, 4);
        assert_eq!(status.total_successes, 3);
        assert_eq!(status.total_failures, 1);
        assert_eq!(status.success_rate, 0.75);
        assert!(status.is_healthy);
    }
    
    #[tokio::test]
    async fn test_health_monitor_tracks_multiple_apis() {
        let monitor = HealthMonitor::with_defaults();
        
        monitor.register_api("api1").await;
        monitor.register_api("api2").await;
        monitor.register_api("api3").await;
        
        let all_status = monitor.get_all_status().await;
        assert_eq!(all_status.len(), 3);
        assert!(all_status.contains_key("api1"));
        assert!(all_status.contains_key("api2"));
        assert!(all_status.contains_key("api3"));
    }
    
    #[tokio::test]
    async fn test_overall_health_calculation() {
        let monitor = HealthMonitor::with_defaults();
        
        monitor.register_api("api1").await;
        monitor.register_api("api2").await;
        monitor.register_api("api3").await;
        
        // Mark one API as unhealthy
        {
            let mut status_map = monitor.status_map.write().await;
            let status = status_map.get_mut("api2").unwrap();
            status.record_failure();
            status.record_failure();
            status.record_failure();
        }
        
        let overall = monitor.get_overall_health().await;
        assert_eq!(overall.total_count, 3);
        assert_eq!(overall.healthy_count, 2);
        assert_eq!(overall.unhealthy_count, 1);
        assert!(!overall.is_healthy); // System is unhealthy if any API is unhealthy
        assert!((overall.health_percentage - 66.67).abs() < 0.1);
    }
}

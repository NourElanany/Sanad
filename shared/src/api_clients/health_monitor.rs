//! Health monitoring system for API clients
//! 
//! Tracks the health status of all API clients, performs periodic health checks,
//! and provides metrics for monitoring and alerting.

use crate::api_clients::error::ApiError;
use crate::api_clients::traits::ApiClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use tokio::time::sleep;

/// Health status for an API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiHealthStatus {
    pub api_name: String,
    pub is_healthy: bool,
    pub last_check: SystemTime,
    pub last_success: Option<SystemTime>,
    pub last_failure: Option<SystemTime>,
    pub success_rate: f64,
    pub avg_response_time_ms: u64,
    pub consecutive_failures: u32,
    pub consecutive_successes: u32,
    pub total_checks: u64,
    pub total_successes: u64,
    pub total_failures: u64,
}

impl ApiHealthStatus {
    /// Create a new health status for an API
    pub fn new(api_name: impl Into<String>) -> Self {
        Self {
            api_name: api_name.into(),
            is_healthy: true,
            last_check: SystemTime::now(),
            last_success: None,
            last_failure: None,
            success_rate: 1.0,
            avg_response_time_ms: 0,
            consecutive_failures: 0,
            consecutive_successes: 0,
            total_checks: 0,
            total_successes: 0,
            total_failures: 0,
        }
    }
    
    /// Update status after a successful health check
    pub fn record_success(&mut self, response_time: Duration) {
        self.last_check = SystemTime::now();
        self.last_success = Some(SystemTime::now());
        self.consecutive_successes += 1;
        self.consecutive_failures = 0;
        self.total_checks += 1;
        self.total_successes += 1;
        
        // Update average response time (exponential moving average)
        let new_time_ms = response_time.as_millis() as u64;
        if self.avg_response_time_ms == 0 {
            self.avg_response_time_ms = new_time_ms;
        } else {
            // EMA with alpha = 0.3
            self.avg_response_time_ms = ((self.avg_response_time_ms as f64 * 0.7) + (new_time_ms as f64 * 0.3)) as u64;
        }
        
        // Update success rate
        self.success_rate = self.total_successes as f64 / self.total_checks as f64;
        
        // Mark as healthy if we have consecutive successes
        if self.consecutive_successes >= 2 {
            self.is_healthy = true;
        }
    }
    
    /// Update status after a failed health check
    pub fn record_failure(&mut self) {
        self.last_check = SystemTime::now();
        self.last_failure = Some(SystemTime::now());
        self.consecutive_failures += 1;
        self.consecutive_successes = 0;
        self.total_checks += 1;
        self.total_failures += 1;
        
        // Update success rate
        self.success_rate = self.total_successes as f64 / self.total_checks as f64;
        
        // Mark as unhealthy if we have consecutive failures
        if self.consecutive_failures >= 3 {
            self.is_healthy = false;
        }
    }
}

/// Health monitor configuration
#[derive(Debug, Clone)]
pub struct HealthMonitorConfig {
    /// Interval between health checks
    pub check_interval: Duration,
    /// Number of consecutive failures before marking as unhealthy
    pub unhealthy_threshold: u32,
    /// Number of consecutive successes before marking as healthy
    pub recovery_threshold: u32,
}

impl Default for HealthMonitorConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(5 * 60), // 5 minutes
            unhealthy_threshold: 3,
            recovery_threshold: 2,
        }
    }
}

/// Health monitor for tracking API health
pub struct HealthMonitor {
    /// Health status for each API
    status_map: Arc<RwLock<HashMap<String, ApiHealthStatus>>>,
    /// Configuration
    config: HealthMonitorConfig,
    /// Whether monitoring is running
    is_running: Arc<RwLock<bool>>,
}

impl HealthMonitor {
    /// Create a new health monitor
    pub fn new(config: HealthMonitorConfig) -> Self {
        Self {
            status_map: Arc::new(RwLock::new(HashMap::new())),
            config,
            is_running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Create a health monitor with default configuration
    pub fn with_defaults() -> Self {
        Self::new(HealthMonitorConfig::default())
    }
    
    /// Start monitoring in the background
    /// 
    /// This spawns a background task that periodically checks all registered APIs
    pub async fn start_monitoring<Req, Res>(
        self: Arc<Self>,
        clients: Vec<Arc<dyn ApiClient<Request = Req, Response = Res>>>,
    ) where
        Req: Send + Sync + 'static,
        Res: Send + Sync + 'static,
    {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            log::warn!("Health monitoring is already running");
            return;
        }
        *is_running = true;
        drop(is_running);
        
        let monitor = self.clone();
        tokio::spawn(async move {
            log::info!("Starting health monitoring with interval: {:?}", monitor.config.check_interval);
            
            loop {
                // Check if we should stop
                {
                    let is_running = monitor.is_running.read().await;
                    if !*is_running {
                        log::info!("Stopping health monitoring");
                        break;
                    }
                }
                
                // Check all APIs
                for client in &clients {
                    monitor.check_api(client.as_ref()).await;
                }
                
                // Wait for next check
                sleep(monitor.config.check_interval).await;
            }
        });
    }
    
    /// Stop monitoring
    pub async fn stop_monitoring(&self) {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
    }
    
    /// Check the health of a single API
    pub async fn check_api<Req, Res>(&self, client: &dyn ApiClient<Request = Req, Response = Res>) -> bool
    where
        Req: Send + Sync,
        Res: Send + Sync,
    {
        let api_name = client.api_name();
        let start = Instant::now();
        
        let is_healthy = client.is_healthy().await;
        let duration = start.elapsed();
        
        let mut status_map = self.status_map.write().await;
        let status = status_map
            .entry(api_name.to_string())
            .or_insert_with(|| ApiHealthStatus::new(api_name));
        
        if is_healthy {
            status.record_success(duration);
            log::debug!("API {} is healthy (response time: {:?})", api_name, duration);
        } else {
            status.record_failure();
            log::warn!("API {} is unhealthy (consecutive failures: {})", api_name, status.consecutive_failures);
            
            // Alert if newly unhealthy
            if status.consecutive_failures == self.config.unhealthy_threshold {
                log::error!("API {} marked as UNHEALTHY after {} consecutive failures", api_name, self.config.unhealthy_threshold);
            }
        }
        
        is_healthy
    }
    
    /// Get the health status of a specific API
    pub async fn get_status(&self, api_name: &str) -> Option<ApiHealthStatus> {
        let status_map = self.status_map.read().await;
        status_map.get(api_name).cloned()
    }
    
    /// Get the health status of all APIs
    pub async fn get_all_status(&self) -> HashMap<String, ApiHealthStatus> {
        let status_map = self.status_map.read().await;
        status_map.clone()
    }
    
    /// Check if a specific API is healthy
    pub async fn is_api_healthy(&self, api_name: &str) -> bool {
        let status_map = self.status_map.read().await;
        status_map
            .get(api_name)
            .map(|s| s.is_healthy)
            .unwrap_or(true) // Assume healthy if not yet checked
    }
    
    /// Get overall system health
    pub async fn get_overall_health(&self) -> OverallHealth {
        let status_map = self.status_map.read().await;
        
        if status_map.is_empty() {
            return OverallHealth {
                is_healthy: true,
                healthy_count: 0,
                unhealthy_count: 0,
                total_count: 0,
                health_percentage: 100.0,
            };
        }
        
        let total_count = status_map.len();
        let healthy_count = status_map.values().filter(|s| s.is_healthy).count();
        let unhealthy_count = total_count - healthy_count;
        let health_percentage = (healthy_count as f64 / total_count as f64) * 100.0;
        
        OverallHealth {
            is_healthy: unhealthy_count == 0,
            healthy_count,
            unhealthy_count,
            total_count,
            health_percentage,
        }
    }
    
    /// Register an API for monitoring (initializes its status)
    pub async fn register_api(&self, api_name: impl Into<String>) {
        let mut status_map = self.status_map.write().await;
        let api_name = api_name.into();
        status_map.entry(api_name.clone())
            .or_insert_with(|| ApiHealthStatus::new(api_name));
    }
    
    /// Get the configuration
    pub fn config(&self) -> &HealthMonitorConfig {
        &self.config
    }
}

/// Overall system health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallHealth {
    pub is_healthy: bool,
    pub healthy_count: usize,
    pub unhealthy_count: usize,
    pub total_count: usize,
    pub health_percentage: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_clients::traits::ApiClient;
    use async_trait::async_trait;
    
    struct MockApiClient {
        name: String,
        healthy: bool,
    }
    
    #[async_trait]
    impl ApiClient for MockApiClient {
        type Request = ();
        type Response = ();
        
        fn api_name(&self) -> &str {
            &self.name
        }
        
        fn priority(&self) -> u8 {
            1
        }
        
        async fn is_healthy(&self) -> bool {
            self.healthy
        }
        
        async fn request(&self, _req: Self::Request) -> Result<Self::Response, ApiError> {
            Ok(())
        }
    }
    
    #[test]
    fn test_health_status_new() {
        let status = ApiHealthStatus::new("test-api");
        assert_eq!(status.api_name, "test-api");
        assert!(status.is_healthy);
        assert_eq!(status.consecutive_failures, 0);
        assert_eq!(status.success_rate, 1.0);
    }
    
    #[test]
    fn test_health_status_record_success() {
        let mut status = ApiHealthStatus::new("test-api");
        status.record_success(Duration::from_millis(100));
        
        assert_eq!(status.consecutive_successes, 1);
        assert_eq!(status.consecutive_failures, 0);
        assert_eq!(status.total_successes, 1);
        assert_eq!(status.avg_response_time_ms, 100);
        assert!(status.is_healthy);
    }
    
    #[test]
    fn test_health_status_record_failure() {
        let mut status = ApiHealthStatus::new("test-api");
        status.record_failure();
        
        assert_eq!(status.consecutive_failures, 1);
        assert_eq!(status.consecutive_successes, 0);
        assert_eq!(status.total_failures, 1);
        assert!(status.is_healthy); // Still healthy after 1 failure
        
        // Mark as unhealthy after 3 failures
        status.record_failure();
        status.record_failure();
        assert!(!status.is_healthy);
        assert_eq!(status.consecutive_failures, 3);
    }
    
    #[test]
    fn test_health_status_recovery() {
        let mut status = ApiHealthStatus::new("test-api");
        
        // Mark as unhealthy
        status.record_failure();
        status.record_failure();
        status.record_failure();
        assert!(!status.is_healthy);
        
        // Recover with 2 successes
        status.record_success(Duration::from_millis(100));
        status.record_success(Duration::from_millis(100));
        assert!(status.is_healthy);
        assert_eq!(status.consecutive_successes, 2);
        assert_eq!(status.consecutive_failures, 0);
    }
    
    #[tokio::test]
    async fn test_health_monitor_check_api() {
        let monitor = HealthMonitor::with_defaults();
        
        let client = MockApiClient {
            name: "test-api".to_string(),
            healthy: true,
        };
        
        let is_healthy = monitor.check_api(&client).await;
        assert!(is_healthy);
        
        let status = monitor.get_status("test-api").await;
        assert!(status.is_some());
        let status = status.unwrap();
        assert!(status.is_healthy);
        assert_eq!(status.total_checks, 1);
    }
    
    #[tokio::test]
    async fn test_health_monitor_overall_health() {
        let monitor = HealthMonitor::with_defaults();
        
        monitor.register_api("api1").await;
        monitor.register_api("api2").await;
        monitor.register_api("api3").await;
        
        let overall = monitor.get_overall_health().await;
        assert_eq!(overall.total_count, 3);
        assert_eq!(overall.healthy_count, 3);
        assert_eq!(overall.unhealthy_count, 0);
        assert!(overall.is_healthy);
        assert_eq!(overall.health_percentage, 100.0);
    }
    
    #[tokio::test]
    async fn test_health_monitor_register_api() {
        let monitor = HealthMonitor::with_defaults();
        
        monitor.register_api("test-api").await;
        
        let status = monitor.get_status("test-api").await;
        assert!(status.is_some());
        assert_eq!(status.unwrap().api_name, "test-api");
    }
}

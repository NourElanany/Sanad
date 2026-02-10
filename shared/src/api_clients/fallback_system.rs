//! Fallback system for API requests with priority-based selection

use crate::api_clients::cache_manager::CacheManager;
use crate::api_clients::error::ApiError;
use crate::api_clients::traits::ApiClient;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::SystemTime;

#[cfg(test)]
#[path = "fallback_system_property_tests.rs"]
mod property_tests;

/// Fallback event information
#[derive(Debug, Clone)]
pub struct FallbackEvent {
    pub timestamp: SystemTime,
    pub primary_api: String,
    pub fallback_api: Option<String>,
    pub reason: FallbackReason,
    pub request_id: String,
}

/// Reason for fallback
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FallbackReason {
    PrimaryFailed,
    PrimaryUnhealthy,
    AllApisFailed,
    UsingStaleCache,
    UsingLocalCalculation,
}

impl std::fmt::Display for FallbackReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FallbackReason::PrimaryFailed => write!(f, "Primary API failed"),
            FallbackReason::PrimaryUnhealthy => write!(f, "Primary API unhealthy"),
            FallbackReason::AllApisFailed => write!(f, "All APIs failed"),
            FallbackReason::UsingStaleCache => write!(f, "Using stale cache"),
            FallbackReason::UsingLocalCalculation => write!(f, "Using local calculation"),
        }
    }
}

/// Fallback system for managing API failures
pub struct FallbackSystem {
    cache_manager: Option<Arc<CacheManager>>,
    log_events: bool,
}

impl FallbackSystem {
    /// Create a new fallback system
    pub fn new(cache_manager: Option<Arc<CacheManager>>) -> Self {
        Self {
            cache_manager,
            log_events: true,
        }
    }
    
    /// Create a fallback system without event logging (for testing)
    pub fn without_logging(cache_manager: Option<Arc<CacheManager>>) -> Self {
        Self {
            cache_manager,
            log_events: false,
        }
    }
    
    /// Execute request with fallback logic
    /// 
    /// Tries APIs in priority order. If all fail, attempts to serve from stale cache.
    /// 
    /// # Arguments
    /// * `clients` - List of API clients to try in priority order
    /// * `request_fn` - Async function that takes a client and returns a result
    /// * `cache_key` - Optional cache key for stale cache fallback
    /// * `request_id` - Unique request identifier for logging
    pub async fn execute_with_fallback<T, Res, F, Fut>(
        &self,
        clients: &[Arc<T>],
        request_fn: F,
        cache_key: Option<&str>,
        request_id: String,
    ) -> Result<(Res, Option<FallbackEvent>), ApiError>
    where
        T: ApiClient + ?Sized,
        Res: Clone + Send + Sync + serde::Serialize + serde::de::DeserializeOwned,
        F: Fn(Arc<T>) -> Fut,
        Fut: std::future::Future<Output = Result<Res, ApiError>>,
    {
        if clients.is_empty() {
            return Err(ApiError::Configuration("No API clients configured".to_string()));
        }
        
        let primary_api = clients[0].api_name().to_string();
        let mut last_error = None;
        
        // Try each API in priority order
        for (index, client) in clients.iter().enumerate() {
            // Check if API is healthy
            if !client.is_healthy().await {
                log::warn!("Skipping unhealthy API: {}", client.api_name());
                
                if index == 0 {
                    // Primary API is unhealthy
                    let event = FallbackEvent {
                        timestamp: SystemTime::now(),
                        primary_api: primary_api.clone(),
                        fallback_api: clients.get(1).map(|c| c.api_name().to_string()),
                        reason: FallbackReason::PrimaryUnhealthy,
                        request_id: request_id.clone(),
                    };
                    self.log_event(&event);
                }
                continue;
            }
            
            // Try the API
            match request_fn(client.clone()).await {
                Ok(response) => {
                    // Success!
                    if index > 0 {
                        // Used fallback API
                        let event = FallbackEvent {
                            timestamp: SystemTime::now(),
                            primary_api: primary_api.clone(),
                            fallback_api: Some(client.api_name().to_string()),
                            reason: FallbackReason::PrimaryFailed,
                            request_id: request_id.clone(),
                        };
                        self.log_event(&event);
                        return Ok((response, Some(event)));
                    }
                    return Ok((response, None));
                }
                Err(error) => {
                    log::warn!("API {} failed: {}", client.api_name(), error);
                    last_error = Some(error);
                }
            }
        }
        
        // All APIs failed, try stale cache
        if let Some(cache_key) = cache_key {
            if let Some(cache_manager) = &self.cache_manager {
                if let Ok(Some(cached)) = cache_manager.get_expired::<Res>(cache_key).await {
                    log::warn!("Serving stale cache for key: {}", cache_key);
                    let event = FallbackEvent {
                        timestamp: SystemTime::now(),
                        primary_api: primary_api.clone(),
                        fallback_api: None,
                        reason: FallbackReason::UsingStaleCache,
                        request_id: request_id.clone(),
                    };
                    self.log_event(&event);
                    return Ok((cached, Some(event)));
                }
            }
        }
        
        // Everything failed
        let event = FallbackEvent {
            timestamp: SystemTime::now(),
            primary_api: primary_api.clone(),
            fallback_api: None,
            reason: FallbackReason::AllApisFailed,
            request_id: request_id.clone(),
        };
        self.log_event(&event);
        
        Err(last_error.unwrap_or(ApiError::AllApisFailed))
    }
    
    /// Try to serve from stale cache
    pub async fn try_stale_cache<T>(
        &self,
        cache_key: &str,
        primary_api: &str,
        request_id: String,
    ) -> Result<(T, FallbackEvent), ApiError>
    where
        T: serde::de::DeserializeOwned,
    {
        if let Some(cache_manager) = &self.cache_manager {
            if let Ok(Some(cached)) = cache_manager.get_expired::<T>(cache_key).await {
                let event = FallbackEvent {
                    timestamp: SystemTime::now(),
                    primary_api: primary_api.to_string(),
                    fallback_api: None,
                    reason: FallbackReason::UsingStaleCache,
                    request_id,
                };
                self.log_event(&event);
                return Ok((cached, event));
            }
        }
        
        Err(ApiError::CacheError("No stale cache available".to_string()))
    }
    
    /// Log fallback event
    fn log_event(&self, event: &FallbackEvent) {
        if !self.log_events {
            return;
        }
        
        match &event.reason {
            FallbackReason::PrimaryFailed | FallbackReason::PrimaryUnhealthy => {
                if let Some(fallback_api) = &event.fallback_api {
                    log::warn!(
                        "[Fallback] {} - Switching from {} to {} (request_id: {})",
                        event.reason,
                        event.primary_api,
                        fallback_api,
                        event.request_id
                    );
                }
            }
            FallbackReason::UsingStaleCache => {
                log::warn!(
                    "[Fallback] Using stale cache for {} (request_id: {})",
                    event.primary_api,
                    event.request_id
                );
            }
            FallbackReason::UsingLocalCalculation => {
                log::info!(
                    "[Fallback] Using local calculation for {} (request_id: {})",
                    event.primary_api,
                    event.request_id
                );
            }
            FallbackReason::AllApisFailed => {
                log::error!(
                    "[Fallback] All APIs failed for {} (request_id: {})",
                    event.primary_api,
                    event.request_id
                );
            }
        }
    }
    
    /// Create a fallback event for local calculation
    pub fn create_local_calculation_event(
        &self,
        primary_api: &str,
        request_id: String,
    ) -> FallbackEvent {
        let event = FallbackEvent {
            timestamp: SystemTime::now(),
            primary_api: primary_api.to_string(),
            fallback_api: None,
            reason: FallbackReason::UsingLocalCalculation,
            request_id,
        };
        self.log_event(&event);
        event
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_clients::RateLimitConfig;
    use async_trait::async_trait;
    
    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    struct TestResponse {
        data: String,
    }
    
    #[derive(Debug)]
    struct MockApiClient {
        name: String,
        priority: u8,
        healthy: bool,
        should_fail: bool,
    }
    
    #[async_trait]
    impl ApiClient for MockApiClient {
        fn api_name(&self) -> &str {
            &self.name
        }
        
        fn priority(&self) -> u8 {
            self.priority
        }
        
        async fn is_healthy(&self) -> bool {
            self.healthy
        }
        
        fn rate_limit(&self) -> RateLimitConfig {
            RateLimitConfig {
                requests_per_minute: 60,
                requests_per_hour: 1000,
                requests_per_day: 10000,
            }
        }
    }
    
    impl MockApiClient {
        async fn make_request(&self) -> Result<TestResponse, ApiError> {
            if self.should_fail {
                Err(ApiError::Network("Mock failure".to_string()))
            } else {
                Ok(TestResponse {
                    data: format!("Response from {}", self.name),
                })
            }
        }
    }
    
    #[tokio::test]
    async fn test_fallback_primary_success() {
        let fallback = FallbackSystem::without_logging(None);
        
        let clients: Vec<Arc<MockApiClient>> = vec![
            Arc::new(MockApiClient {
                name: "primary".to_string(),
                priority: 1,
                healthy: true,
                should_fail: false,
            }),
        ];
        
        let result = fallback.execute_with_fallback(
            &clients,
            |client: Arc<MockApiClient>| async move { client.make_request().await },
            None,
            "req-1".to_string()
        ).await;
        
        assert!(result.is_ok());
        let (response, event) = result.unwrap();
        assert_eq!(response.data, "Response from primary");
        assert!(event.is_none()); // No fallback needed
    }
    
    #[tokio::test]
    async fn test_fallback_to_secondary() {
        let fallback = FallbackSystem::without_logging(None);
        
        let clients: Vec<Arc<MockApiClient>> = vec![
            Arc::new(MockApiClient {
                name: "primary".to_string(),
                priority: 1,
                healthy: true,
                should_fail: true,
            }),
            Arc::new(MockApiClient {
                name: "secondary".to_string(),
                priority: 2,
                healthy: true,
                should_fail: false,
            }),
        ];
        
        let result = fallback.execute_with_fallback(
            &clients,
            |client: Arc<MockApiClient>| async move { client.make_request().await },
            None,
            "req-1".to_string()
        ).await;
        
        assert!(result.is_ok());
        let (response, event) = result.unwrap();
        assert_eq!(response.data, "Response from secondary");
        assert!(event.is_some());
        
        let event = event.unwrap();
        assert_eq!(event.reason, FallbackReason::PrimaryFailed);
        assert_eq!(event.fallback_api, Some("secondary".to_string()));
    }
    
    #[tokio::test]
    async fn test_fallback_skip_unhealthy() {
        let fallback = FallbackSystem::without_logging(None);
        
        let clients: Vec<Arc<MockApiClient>> = vec![
            Arc::new(MockApiClient {
                name: "primary".to_string(),
                priority: 1,
                healthy: false, // Unhealthy
                should_fail: false,
            }),
            Arc::new(MockApiClient {
                name: "secondary".to_string(),
                priority: 2,
                healthy: true,
                should_fail: false,
            }),
        ];
        
        let result = fallback.execute_with_fallback(
            &clients,
            |client: Arc<MockApiClient>| async move { client.make_request().await },
            None,
            "req-1".to_string()
        ).await;
        
        assert!(result.is_ok());
        let (response, event) = result.unwrap();
        assert_eq!(response.data, "Response from secondary");
        assert!(event.is_some());
        
        let event = event.unwrap();
        assert_eq!(event.reason, FallbackReason::PrimaryUnhealthy);
    }
    
    #[tokio::test]
    async fn test_fallback_all_fail() {
        let fallback = FallbackSystem::without_logging(None);
        
        let clients: Vec<Arc<MockApiClient>> = vec![
            Arc::new(MockApiClient {
                name: "primary".to_string(),
                priority: 1,
                healthy: true,
                should_fail: true,
            }),
            Arc::new(MockApiClient {
                name: "secondary".to_string(),
                priority: 2,
                healthy: true,
                should_fail: true,
            }),
        ];
        
        let result = fallback.execute_with_fallback(
            &clients,
            |client: Arc<MockApiClient>| async move { client.make_request().await },
            None,
            "req-1".to_string()
        ).await;
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_fallback_reason_display() {
        assert_eq!(FallbackReason::PrimaryFailed.to_string(), "Primary API failed");
        assert_eq!(FallbackReason::PrimaryUnhealthy.to_string(), "Primary API unhealthy");
        assert_eq!(FallbackReason::AllApisFailed.to_string(), "All APIs failed");
        assert_eq!(FallbackReason::UsingStaleCache.to_string(), "Using stale cache");
        assert_eq!(FallbackReason::UsingLocalCalculation.to_string(), "Using local calculation");
    }
    
    #[test]
    fn test_create_local_calculation_event() {
        let fallback = FallbackSystem::without_logging(None);
        let event = fallback.create_local_calculation_event("test-api", "req-1".to_string());
        
        assert_eq!(event.primary_api, "test-api");
        assert_eq!(event.reason, FallbackReason::UsingLocalCalculation);
        assert_eq!(event.request_id, "req-1");
        assert!(event.fallback_api.is_none());
    }
}

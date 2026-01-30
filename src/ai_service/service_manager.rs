use super::*;
use crate::ai_service::{
    config::{AIServiceConfig, ModelSpecialization},
    hugging_face_client::{HuggingFaceClient, HuggingFaceConfig},
    vector_database::{VectorDatabaseClient, VectorDatabaseConfig, DistanceMetric},
    integration_service::{IntegrationService, IntegrationConfig, CacheConfig, FallbackConfig, RateLimitConfig},
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use serde::{Serialize, Deserialize};

/// Service manager for AI service integration
pub struct AIServiceManager {
    config: AIServiceConfig,
    integration_service: Arc<RwLock<Option<IntegrationService>>>,
    health_status: Arc<RwLock<ServiceHealth>>,
    metrics: Arc<RwLock<ServiceMetrics>>,
    circuit_breaker: Arc<RwLock<CircuitBreaker>>,
}

/// Overall service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub overall_status: HealthStatus,
    pub hugging_face_status: HealthStatus,
    pub vector_db_status: HealthStatus,
    pub cache_status: HealthStatus,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub uptime_seconds: u64,
    pub error_details: Vec<String>,
}

/// Health status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Service metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub cache_hit_rate: f64,
    pub fallback_usage_rate: f64,
    pub last_reset: chrono::DateTime<chrono::Utc>,
}

/// Circuit breaker for handling service failures
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    pub state: CircuitBreakerState,
    pub failure_count: u32,
    pub last_failure_time: Option<Instant>,
    pub half_open_calls: u32,
}

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    Closed,   // Normal operation
    Open,     // Failing, reject requests
    HalfOpen, // Testing if service recovered
}

/// Service initialization result
#[derive(Debug)]
pub struct InitializationResult {
    pub success: bool,
    pub services_initialized: Vec<String>,
    pub services_failed: Vec<String>,
    pub warnings: Vec<String>,
    pub initialization_time_ms: u64,
}

impl AIServiceManager {
    /// Create a new service manager
    pub fn new(config: AIServiceConfig) -> Result<Self> {
        // Validate configuration
        config.validate().map_err(|e| AIServiceError::ConfigurationError(e))?;

        info!("Creating AI Service Manager with configuration");

        Ok(Self {
            config,
            integration_service: Arc::new(RwLock::new(None)),
            health_status: Arc::new(RwLock::new(ServiceHealth::new())),
            metrics: Arc::new(RwLock::new(ServiceMetrics::new())),
            circuit_breaker: Arc::new(RwLock::new(CircuitBreaker::new())),
        })
    }

    /// Initialize all services
    pub async fn initialize(&self) -> Result<InitializationResult> {
        let start_time = Instant::now();
        let mut result = InitializationResult {
            success: false,
            services_initialized: Vec::new(),
            services_failed: Vec::new(),
            warnings: Vec::new(),
            initialization_time_ms: 0,
        };

        info!("Initializing AI services...");

        // Initialize Hugging Face client
        match self.initialize_hugging_face().await {
            Ok(_) => {
                result.services_initialized.push("Hugging Face".to_string());
                info!("Hugging Face client initialized successfully");
            }
            Err(e) => {
                result.services_failed.push(format!("Hugging Face: {}", e));
                error!("Failed to initialize Hugging Face client: {}", e);
            }
        }

        // Initialize Vector Database
        match self.initialize_vector_database().await {
            Ok(_) => {
                result.services_initialized.push("Vector Database".to_string());
                info!("Vector Database client initialized successfully");
            }
            Err(e) => {
                result.services_failed.push(format!("Vector Database: {}", e));
                error!("Failed to initialize Vector Database: {}", e);
            }
        }

        // Initialize Integration Service if both core services are available
        if result.services_failed.is_empty() {
            match self.initialize_integration_service().await {
                Ok(_) => {
                    result.services_initialized.push("Integration Service".to_string());
                    info!("Integration Service initialized successfully");
                    result.success = true;
                }
                Err(e) => {
                    result.services_failed.push(format!("Integration Service: {}", e));
                    error!("Failed to initialize Integration Service: {}", e);
                }
            }
        } else {
            result.warnings.push("Integration Service not initialized due to dependency failures".to_string());
        }

        // Start health monitoring
        if result.success {
            self.start_health_monitoring().await;
            result.services_initialized.push("Health Monitoring".to_string());
        }

        result.initialization_time_ms = start_time.elapsed().as_millis() as u64;

        // Update health status
        let mut health = self.health_status.write().await;
        health.overall_status = if result.success {
            HealthStatus::Healthy
        } else if !result.services_initialized.is_empty() {
            HealthStatus::Degraded
        } else {
            HealthStatus::Unhealthy
        };
        health.last_check = chrono::Utc::now();

        info!("AI Service initialization completed in {}ms", result.initialization_time_ms);
        Ok(result)
    }

    /// Initialize Hugging Face client
    async fn initialize_hugging_face(&self) -> Result<()> {
        let hf_config = HuggingFaceConfig {
            api_key: self.config.hugging_face.api_key.clone(),
            base_url: self.config.hugging_face.base_url.clone(),
            timeout_seconds: self.config.hugging_face.timeout_seconds,
            max_retries: self.config.hugging_face.max_retries,
            requests_per_minute: self.config.hugging_face.requests_per_minute,
            default_model: self.config.hugging_face.default_model.clone(),
            embedding_model: self.config.hugging_face.embedding_model.clone(),
        };

        let client = HuggingFaceClient::new(hf_config)?;

        // Test connection with a simple model check
        match client.check_model_status(&self.config.hugging_face.default_model).await {
            Ok(true) => {
                info!("Hugging Face model {} is ready", self.config.hugging_face.default_model);
            }
            Ok(false) => {
                warn!("Hugging Face model {} is loading", self.config.hugging_face.default_model);
                // Wait for model to be ready
                client.wait_for_model(&self.config.hugging_face.default_model, 120).await?;
            }
            Err(e) => {
                warn!("Could not verify Hugging Face model status: {}", e);
                // Continue anyway - the model might work when actually called
            }
        }

        Ok(())
    }

    /// Initialize Vector Database client
    async fn initialize_vector_database(&self) -> Result<()> {
        let distance_metric = match self.config.vector_database.distance_metric.as_str() {
            "Cosine" => DistanceMetric::Cosine,
            "Euclidean" => DistanceMetric::Euclidean,
            "Dot" => DistanceMetric::Dot,
            _ => DistanceMetric::Cosine,
        };

        let vdb_config = VectorDatabaseConfig {
            host: self.config.vector_database.host.clone(),
            port: self.config.vector_database.port,
            collection_name: self.config.vector_database.collection_name.clone(),
            vector_size: self.config.vector_database.vector_size,
            distance_metric,
            timeout_seconds: self.config.vector_database.timeout_seconds,
            max_retries: self.config.vector_database.max_retries,
            batch_size: self.config.vector_database.batch_size,
        };

        let client = VectorDatabaseClient::new(vdb_config).await?;

        // Test connection by getting collection stats
        match client.get_collection_stats().await {
            Ok(stats) => {
                info!("Vector Database connected. Collection has {} points", stats.total_points);
            }
            Err(e) => {
                warn!("Could not get Vector Database stats: {}", e);
                // Continue anyway - the database might be empty but functional
            }
        }

        Ok(())
    }

    /// Initialize Integration Service
    async fn initialize_integration_service(&self) -> Result<()> {
        let integration_config = self.build_integration_config();
        let service = IntegrationService::new(integration_config).await?;

        // Test the integration service
        let health_status = service.health_check().await?;
        info!("Integration Service health check: {}", health_status.overall_status);

        // Store the service
        let mut integration_service = self.integration_service.write().await;
        *integration_service = Some(service);

        Ok(())
    }

    /// Build integration configuration from main config
    fn build_integration_config(&self) -> IntegrationConfig {
        let distance_metric = match self.config.vector_database.distance_metric.as_str() {
            "Cosine" => DistanceMetric::Cosine,
            "Euclidean" => DistanceMetric::Euclidean,
            "Dot" => DistanceMetric::Dot,
            _ => DistanceMetric::Cosine,
        };

        IntegrationConfig {
            hugging_face: HuggingFaceConfig {
                api_key: self.config.hugging_face.api_key.clone(),
                base_url: self.config.hugging_face.base_url.clone(),
                timeout_seconds: self.config.hugging_face.timeout_seconds,
                max_retries: self.config.hugging_face.max_retries,
                requests_per_minute: self.config.hugging_face.requests_per_minute,
                default_model: self.config.hugging_face.default_model.clone(),
                embedding_model: self.config.hugging_face.embedding_model.clone(),
            },
            vector_database: VectorDatabaseConfig {
                host: self.config.vector_database.host.clone(),
                port: self.config.vector_database.port,
                collection_name: self.config.vector_database.collection_name.clone(),
                vector_size: self.config.vector_database.vector_size,
                distance_metric,
                timeout_seconds: self.config.vector_database.timeout_seconds,
                max_retries: self.config.vector_database.max_retries,
                batch_size: self.config.vector_database.batch_size,
            },
            cache: CacheConfig {
                enable_query_cache: self.config.cache.enable_query_cache,
                enable_response_cache: self.config.cache.enable_response_cache,
                enable_embedding_cache: self.config.cache.enable_embedding_cache,
                query_cache_ttl: Duration::from_secs(self.config.cache.query_cache_ttl_seconds),
                response_cache_ttl: Duration::from_secs(self.config.cache.response_cache_ttl_seconds),
                embedding_cache_ttl: Duration::from_secs(self.config.cache.embedding_cache_ttl_seconds),
                max_cache_size: self.config.cache.max_cache_size,
                redis_url: self.config.cache.redis.as_ref().map(|r| r.url.clone()),
            },
            fallback: FallbackConfig {
                enable_fallback: self.config.fallback.enable_fallback,
                fallback_models: self.config.fallback.fallback_models.clone(),
                max_fallback_attempts: self.config.fallback.max_fallback_attempts,
                fallback_delay: Duration::from_secs(self.config.fallback.fallback_delay_seconds),
                enable_offline_mode: self.config.fallback.enable_offline_mode,
                offline_responses: self.config.fallback.offline_responses.clone(),
            },
            rate_limiting: RateLimitConfig {
                requests_per_minute: self.config.rate_limiting.requests_per_minute,
                requests_per_hour: self.config.rate_limiting.requests_per_hour,
                burst_limit: self.config.rate_limiting.burst_limit,
                enable_adaptive_rate_limiting: self.config.rate_limiting.enable_adaptive_rate_limiting,
            },
        }
    }

    /// Start health monitoring background task
    async fn start_health_monitoring(&self) {
        let health_status = Arc::clone(&self.health_status);
        let integration_service = Arc::clone(&self.integration_service);
        let interval = Duration::from_secs(self.config.monitoring.health_check_interval_seconds);

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                let service_guard = integration_service.read().await;
                if let Some(service) = service_guard.as_ref() {
                    match service.health_check().await {
                        Ok(status) => {
                            let mut health = health_status.write().await;
                            health.hugging_face_status = match status.hugging_face_status.as_str() {
                                "healthy" => HealthStatus::Healthy,
                                "degraded" | "loading" => HealthStatus::Degraded,
                                "unhealthy" => HealthStatus::Unhealthy,
                                _ => HealthStatus::Unknown,
                            };
                            health.vector_db_status = match status.vector_db_status.as_str() {
                                "healthy" => HealthStatus::Healthy,
                                "degraded" => HealthStatus::Degraded,
                                "unhealthy" => HealthStatus::Unhealthy,
                                _ => HealthStatus::Unknown,
                            };
                            health.cache_status = match status.cache_status.as_str() {
                                "healthy" => HealthStatus::Healthy,
                                "degraded" => HealthStatus::Degraded,
                                "unhealthy" => HealthStatus::Unhealthy,
                                _ => HealthStatus::Unknown,
                            };
                            
                            // Determine overall status
                            health.overall_status = if health.hugging_face_status == HealthStatus::Healthy 
                                && health.vector_db_status == HealthStatus::Healthy 
                                && health.cache_status == HealthStatus::Healthy {
                                HealthStatus::Healthy
                            } else if health.hugging_face_status == HealthStatus::Unhealthy 
                                || health.vector_db_status == HealthStatus::Unhealthy {
                                HealthStatus::Unhealthy
                            } else {
                                HealthStatus::Degraded
                            };
                            
                            health.last_check = chrono::Utc::now();
                            health.error_details = status.error_details;
                        }
                        Err(e) => {
                            error!("Health check failed: {}", e);
                            let mut health = health_status.write().await;
                            health.overall_status = HealthStatus::Unhealthy;
                            health.error_details = vec![format!("Health check error: {}", e)];
                        }
                    }
                }
            }
        });

        info!("Health monitoring started with interval of {}s", interval.as_secs());
    }

    /// Get current health status
    pub async fn get_health_status(&self) -> ServiceHealth {
        self.health_status.read().await.clone()
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> ServiceMetrics {
        self.metrics.read().await.clone()
    }

    /// Get integration service (if available)
    pub async fn get_integration_service(&self) -> Option<IntegrationService> {
        self.integration_service.read().await.clone()
    }

    /// Shutdown all services gracefully
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down AI services...");

        // Clear integration service
        let mut integration_service = self.integration_service.write().await;
        *integration_service = None;

        // Update health status
        let mut health = self.health_status.write().await;
        health.overall_status = HealthStatus::Unhealthy;
        health.last_check = chrono::Utc::now();

        info!("AI services shutdown completed");
        Ok(())
    }

    /// Record request metrics
    pub async fn record_request(&self, success: bool, response_time_ms: u64, cache_hit: bool, fallback_used: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.total_requests += 1;
        
        if success {
            metrics.successful_requests += 1;
        } else {
            metrics.failed_requests += 1;
        }

        // Update average response time (simple moving average)
        let total_successful = metrics.successful_requests;
        if total_successful > 0 {
            metrics.average_response_time_ms = 
                (metrics.average_response_time_ms * (total_successful - 1) as f64 + response_time_ms as f64) / total_successful as f64;
        }

        // Update cache hit rate
        if cache_hit {
            let cache_hits = (metrics.cache_hit_rate * (metrics.total_requests - 1) as f64) + 1.0;
            metrics.cache_hit_rate = cache_hits / metrics.total_requests as f64;
        } else {
            let cache_hits = metrics.cache_hit_rate * (metrics.total_requests - 1) as f64;
            metrics.cache_hit_rate = cache_hits / metrics.total_requests as f64;
        }

        // Update fallback usage rate
        if fallback_used {
            let fallback_uses = (metrics.fallback_usage_rate * (metrics.total_requests - 1) as f64) + 1.0;
            metrics.fallback_usage_rate = fallback_uses / metrics.total_requests as f64;
        } else {
            let fallback_uses = metrics.fallback_usage_rate * (metrics.total_requests - 1) as f64;
            metrics.fallback_usage_rate = fallback_uses / metrics.total_requests as f64;
        }
    }
}

impl ServiceHealth {
    fn new() -> Self {
        Self {
            overall_status: HealthStatus::Unknown,
            hugging_face_status: HealthStatus::Unknown,
            vector_db_status: HealthStatus::Unknown,
            cache_status: HealthStatus::Unknown,
            last_check: chrono::Utc::now(),
            uptime_seconds: 0,
            error_details: Vec::new(),
        }
    }
}

impl ServiceMetrics {
    fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            cache_hit_rate: 0.0,
            fallback_usage_rate: 0.0,
            last_reset: chrono::Utc::now(),
        }
    }
}

impl CircuitBreaker {
    fn new() -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            last_failure_time: None,
            half_open_calls: 0,
        }
    }

    /// Check if request should be allowed
    pub fn should_allow_request(&mut self, config: &crate::ai_service::config::CircuitBreakerConfig) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    if last_failure.elapsed() > Duration::from_secs(config.recovery_timeout_seconds) {
                        self.state = CircuitBreakerState::HalfOpen;
                        self.half_open_calls = 0;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => {
                self.half_open_calls < config.half_open_max_calls
            }
        }
    }

    /// Record request result
    pub fn record_result(&mut self, success: bool, config: &crate::ai_service::config::CircuitBreakerConfig) {
        match self.state {
            CircuitBreakerState::Closed => {
                if success {
                    self.failure_count = 0;
                } else {
                    self.failure_count += 1;
                    self.last_failure_time = Some(Instant::now());
                    
                    if self.failure_count >= config.failure_threshold {
                        self.state = CircuitBreakerState::Open;
                    }
                }
            }
            CircuitBreakerState::HalfOpen => {
                if success {
                    self.state = CircuitBreakerState::Closed;
                    self.failure_count = 0;
                    self.half_open_calls = 0;
                } else {
                    self.state = CircuitBreakerState::Open;
                    self.failure_count += 1;
                    self.last_failure_time = Some(Instant::now());
                }
            }
            CircuitBreakerState::Open => {
                // Should not reach here if should_allow_request is used correctly
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_manager_creation() {
        let config = AIServiceConfig::default();
        let manager = AIServiceManager::new(config);
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_health_status() {
        let config = AIServiceConfig::default();
        let manager = AIServiceManager::new(config).unwrap();
        
        let health = manager.get_health_status().await;
        assert_eq!(health.overall_status, HealthStatus::Unknown);
    }

    #[tokio::test]
    async fn test_metrics_recording() {
        let config = AIServiceConfig::default();
        let manager = AIServiceManager::new(config).unwrap();
        
        manager.record_request(true, 100, false, false).await;
        manager.record_request(false, 200, true, false).await;
        
        let metrics = manager.get_metrics().await;
        assert_eq!(metrics.total_requests, 2);
        assert_eq!(metrics.successful_requests, 1);
        assert_eq!(metrics.failed_requests, 1);
        assert_eq!(metrics.cache_hit_rate, 0.5);
    }

    #[test]
    fn test_circuit_breaker() {
        let config = crate::ai_service::config::CircuitBreakerConfig {
            failure_threshold: 3,
            recovery_timeout_seconds: 60,
            half_open_max_calls: 2,
        };
        
        let mut circuit_breaker = CircuitBreaker::new();
        
        // Initially closed
        assert!(circuit_breaker.should_allow_request(&config));
        
        // Record failures
        circuit_breaker.record_result(false, &config);
        circuit_breaker.record_result(false, &config);
        circuit_breaker.record_result(false, &config);
        
        // Should be open now
        assert_eq!(circuit_breaker.state, CircuitBreakerState::Open);
        assert!(!circuit_breaker.should_allow_request(&config));
    }
}
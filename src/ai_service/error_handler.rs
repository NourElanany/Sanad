use super::*;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::time::sleep;
use tracing::{error, warn, info, debug};
use serde::{Serialize, Deserialize};

/// Comprehensive error handler for AI service integration
pub struct ErrorHandler {
    config: ErrorHandlerConfig,
    retry_policies: HashMap<ErrorType, RetryPolicy>,
    circuit_breakers: HashMap<String, CircuitBreaker>,
    fallback_strategies: HashMap<ErrorType, FallbackStrategy>,
    error_metrics: ErrorMetrics,
}

/// Error handler configuration
#[derive(Debug, Clone)]
pub struct ErrorHandlerConfig {
    pub max_retry_attempts: u32,
    pub base_retry_delay: Duration,
    pub max_retry_delay: Duration,
    pub circuit_breaker_threshold: u32,
    pub circuit_breaker_timeout: Duration,
    pub enable_fallback: bool,
    pub enable_metrics: bool,
}

/// Types of errors that can occur
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ErrorType {
    NetworkError,
    AuthenticationError,
    RateLimitError,
    ModelLoadingError,
    VectorDatabaseError,
    CacheError,
    ConfigurationError,
    ValidationError,
    TimeoutError,
    ServiceUnavailableError,
    UnknownError,
}

/// Retry policy for different error types
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub jitter: bool,
}

/// Circuit breaker for service protection
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    pub state: CircuitBreakerState,
    pub failure_count: u32,
    pub success_count: u32,
    pub last_failure_time: Option<Instant>,
    pub last_success_time: Option<Instant>,
}

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    Closed,   // Normal operation
    Open,     // Failing, reject requests
    HalfOpen, // Testing recovery
}

/// Fallback strategies for different error types
#[derive(Debug, Clone)]
pub enum FallbackStrategy {
    RetryWithDifferentModel,
    UseCache,
    UseOfflineResponse,
    DegradeService,
    FailGracefully,
}

/// Error metrics tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    pub total_errors: u64,
    pub errors_by_type: HashMap<String, u64>,
    pub retry_attempts: u64,
    pub successful_retries: u64,
    pub fallback_activations: u64,
    pub circuit_breaker_trips: u64,
    pub last_reset: chrono::DateTime<chrono::Utc>,
}

/// Error context for better error handling
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub service: String,
    pub user_id: Option<String>,
    pub request_id: String,
    pub timestamp: Instant,
    pub metadata: HashMap<String, String>,
}

/// Recovery action to take after an error
#[derive(Debug, Clone)]
pub enum RecoveryAction {
    Retry {
        delay: Duration,
        attempt: u32,
    },
    Fallback {
        strategy: FallbackStrategy,
    },
    Fail {
        reason: String,
    },
    CircuitBreak {
        service: String,
        duration: Duration,
    },
}

impl ErrorHandler {
    /// Create a new error handler
    pub fn new(config: ErrorHandlerConfig) -> Self {
        let mut retry_policies = HashMap::new();
        
        // Configure retry policies for different error types
        retry_policies.insert(ErrorType::NetworkError, RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_millis(1000),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter: true,
        });
        
        retry_policies.insert(ErrorType::RateLimitError, RetryPolicy {
            max_attempts: 5,
            base_delay: Duration::from_secs(60),
            max_delay: Duration::from_secs(300),
            backoff_multiplier: 1.5,
            jitter: true,
        });
        
        retry_policies.insert(ErrorType::ModelLoadingError, RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_secs(10),
            max_delay: Duration::from_secs(120),
            backoff_multiplier: 2.0,
            jitter: false,
        });
        
        retry_policies.insert(ErrorType::VectorDatabaseError, RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            jitter: true,
        });
        
        retry_policies.insert(ErrorType::TimeoutError, RetryPolicy {
            max_attempts: 2,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
            jitter: false,
        });
        
        // Configure fallback strategies
        let mut fallback_strategies = HashMap::new();
        fallback_strategies.insert(ErrorType::ModelLoadingError, FallbackStrategy::RetryWithDifferentModel);
        fallback_strategies.insert(ErrorType::NetworkError, FallbackStrategy::UseCache);
        fallback_strategies.insert(ErrorType::ServiceUnavailableError, FallbackStrategy::UseOfflineResponse);
        fallback_strategies.insert(ErrorType::RateLimitError, FallbackStrategy::DegradeService);
        fallback_strategies.insert(ErrorType::AuthenticationError, FallbackStrategy::FailGracefully);
        
        Self {
            config,
            retry_policies,
            circuit_breakers: HashMap::new(),
            fallback_strategies,
            error_metrics: ErrorMetrics::new(),
        }
    }

    /// Handle an error and determine the recovery action
    pub async fn handle_error(
        &mut self,
        error: &AIServiceError,
        context: &ErrorContext,
        attempt: u32,
    ) -> RecoveryAction {
        let error_type = self.classify_error(error);
        
        // Update metrics
        self.update_error_metrics(&error_type);
        
        // Log the error
        self.log_error(error, &error_type, context, attempt);
        
        // Check circuit breaker
        if let Some(action) = self.check_circuit_breaker(&context.service, &error_type) {
            return action;
        }
        
        // Determine recovery action based on error type and attempt count
        self.determine_recovery_action(&error_type, context, attempt).await
    }

    /// Classify error into error type
    fn classify_error(&self, error: &AIServiceError) -> ErrorType {
        match error {
            AIServiceError::ExternalAPIError(msg) => {
                if msg.contains("rate limit") || msg.contains("429") {
                    ErrorType::RateLimitError
                } else if msg.contains("timeout") {
                    ErrorType::TimeoutError
                } else if msg.contains("authentication") || msg.contains("401") || msg.contains("403") {
                    ErrorType::AuthenticationError
                } else if msg.contains("503") || msg.contains("service unavailable") {
                    ErrorType::ServiceUnavailableError
                } else if msg.contains("model") && msg.contains("loading") {
                    ErrorType::ModelLoadingError
                } else {
                    ErrorType::NetworkError
                }
            }
            AIServiceError::DatabaseError(_) => ErrorType::VectorDatabaseError,
            AIServiceError::CacheError(_) => ErrorType::CacheError,
            AIServiceError::ConfigurationError(_) => ErrorType::ConfigurationError,
            AIServiceError::QuestionProcessingError(_) => ErrorType::ValidationError,
            AIServiceError::RateLimitExceeded(_) => ErrorType::RateLimitError,
            AIServiceError::ServiceUnavailable(_) => ErrorType::ServiceUnavailableError,
            _ => ErrorType::UnknownError,
        }
    }

    /// Determine the appropriate recovery action
    async fn determine_recovery_action(
        &mut self,
        error_type: &ErrorType,
        context: &ErrorContext,
        attempt: u32,
    ) -> RecoveryAction {
        // Check if we should retry
        if let Some(retry_policy) = self.retry_policies.get(error_type) {
            if attempt < retry_policy.max_attempts {
                let delay = self.calculate_retry_delay(retry_policy, attempt);
                return RecoveryAction::Retry { delay, attempt: attempt + 1 };
            }
        }
        
        // If retries exhausted, try fallback
        if self.config.enable_fallback {
            if let Some(strategy) = self.fallback_strategies.get(error_type) {
                return RecoveryAction::Fallback { strategy: strategy.clone() };
            }
        }
        
        // If no fallback available, fail gracefully
        RecoveryAction::Fail {
            reason: format!("Max retries exceeded for error type: {:?}", error_type),
        }
    }

    /// Calculate retry delay with exponential backoff and jitter
    fn calculate_retry_delay(&self, policy: &RetryPolicy, attempt: u32) -> Duration {
        let base_delay_ms = policy.base_delay.as_millis() as f64;
        let delay_ms = base_delay_ms * policy.backoff_multiplier.powi(attempt as i32 - 1);
        
        let delay_ms = delay_ms.min(policy.max_delay.as_millis() as f64);
        
        let final_delay_ms = if policy.jitter {
            // Add random jitter (±25%)
            let jitter_factor = 0.75 + (rand::random::<f64>() * 0.5);
            delay_ms * jitter_factor
        } else {
            delay_ms
        };
        
        Duration::from_millis(final_delay_ms as u64)
    }

    /// Check circuit breaker status
    fn check_circuit_breaker(&mut self, service: &str, error_type: &ErrorType) -> Option<RecoveryAction> {
        let circuit_breaker = self.circuit_breakers
            .entry(service.to_string())
            .or_insert_with(|| CircuitBreaker::new());
        
        match circuit_breaker.state {
            CircuitBreakerState::Open => {
                if let Some(last_failure) = circuit_breaker.last_failure_time {
                    if last_failure.elapsed() > self.config.circuit_breaker_timeout {
                        // Try to recover
                        circuit_breaker.state = CircuitBreakerState::HalfOpen;
                        circuit_breaker.success_count = 0;
                        info!("Circuit breaker for {} moved to half-open state", service);
                        None
                    } else {
                        // Still in open state
                        Some(RecoveryAction::CircuitBreak {
                            service: service.to_string(),
                            duration: self.config.circuit_breaker_timeout - last_failure.elapsed(),
                        })
                    }
                } else {
                    None
                }
            }
            CircuitBreakerState::HalfOpen => {
                // Allow limited requests in half-open state
                None
            }
            CircuitBreakerState::Closed => {
                // Normal operation
                None
            }
        }
    }

    /// Record error result for circuit breaker
    pub fn record_result(&mut self, service: &str, success: bool) {
        let circuit_breaker = self.circuit_breakers
            .entry(service.to_string())
            .or_insert_with(|| CircuitBreaker::new());
        
        if success {
            circuit_breaker.success_count += 1;
            circuit_breaker.last_success_time = Some(Instant::now());
            
            match circuit_breaker.state {
                CircuitBreakerState::HalfOpen => {
                    if circuit_breaker.success_count >= 3 {
                        circuit_breaker.state = CircuitBreakerState::Closed;
                        circuit_breaker.failure_count = 0;
                        info!("Circuit breaker for {} recovered to closed state", service);
                    }
                }
                _ => {
                    circuit_breaker.failure_count = 0;
                }
            }
        } else {
            circuit_breaker.failure_count += 1;
            circuit_breaker.last_failure_time = Some(Instant::now());
            
            if circuit_breaker.failure_count >= self.config.circuit_breaker_threshold {
                circuit_breaker.state = CircuitBreakerState::Open;
                self.error_metrics.circuit_breaker_trips += 1;
                error!("Circuit breaker for {} tripped due to {} failures", service, circuit_breaker.failure_count);
            }
        }
    }

    /// Execute recovery action
    pub async fn execute_recovery_action(
        &mut self,
        action: RecoveryAction,
        context: &ErrorContext,
    ) -> Result<Option<String>> {
        match action {
            RecoveryAction::Retry { delay, attempt } => {
                info!("Retrying operation '{}' after {:?} (attempt {})", context.operation, delay, attempt);
                sleep(delay).await;
                self.error_metrics.retry_attempts += 1;
                Ok(None)
            }
            
            RecoveryAction::Fallback { strategy } => {
                info!("Executing fallback strategy: {:?}", strategy);
                self.error_metrics.fallback_activations += 1;
                self.execute_fallback_strategy(strategy, context).await
            }
            
            RecoveryAction::Fail { reason } => {
                error!("Operation '{}' failed: {}", context.operation, reason);
                Err(AIServiceError::ServiceUnavailable(reason))
            }
            
            RecoveryAction::CircuitBreak { service, duration } => {
                warn!("Circuit breaker for {} is open, rejecting request for {:?}", service, duration);
                Err(AIServiceError::ServiceUnavailable(
                    format!("Service {} is temporarily unavailable", service)
                ))
            }
        }
    }

    /// Execute fallback strategy
    async fn execute_fallback_strategy(
        &self,
        strategy: FallbackStrategy,
        context: &ErrorContext,
    ) -> Result<Option<String>> {
        match strategy {
            FallbackStrategy::RetryWithDifferentModel => {
                info!("Fallback: Will retry with different model");
                Ok(Some("retry_different_model".to_string()))
            }
            
            FallbackStrategy::UseCache => {
                info!("Fallback: Using cached response if available");
                Ok(Some("use_cache".to_string()))
            }
            
            FallbackStrategy::UseOfflineResponse => {
                info!("Fallback: Using offline response");
                let offline_response = "عذراً، الخدمة غير متاحة حالياً. يرجى المحاولة لاحقاً أو استشارة العلماء المختصين.";
                Ok(Some(offline_response.to_string()))
            }
            
            FallbackStrategy::DegradeService => {
                info!("Fallback: Degrading service quality");
                Ok(Some("service_degraded".to_string()))
            }
            
            FallbackStrategy::FailGracefully => {
                info!("Fallback: Failing gracefully");
                Err(AIServiceError::ServiceUnavailable(
                    "Service is temporarily unavailable".to_string()
                ))
            }
        }
    }

    /// Update error metrics
    fn update_error_metrics(&mut self, error_type: &ErrorType) {
        self.error_metrics.total_errors += 1;
        let error_type_str = format!("{:?}", error_type);
        *self.error_metrics.errors_by_type.entry(error_type_str).or_insert(0) += 1;
    }

    /// Log error with appropriate level
    fn log_error(
        &self,
        error: &AIServiceError,
        error_type: &ErrorType,
        context: &ErrorContext,
        attempt: u32,
    ) {
        let log_message = format!(
            "Error in operation '{}' (attempt {}): {} - Error type: {:?}",
            context.operation, attempt, error, error_type
        );
        
        match error_type {
            ErrorType::AuthenticationError | ErrorType::ConfigurationError => {
                error!("{}", log_message);
            }
            ErrorType::NetworkError | ErrorType::TimeoutError | ErrorType::RateLimitError => {
                warn!("{}", log_message);
            }
            _ => {
                info!("{}", log_message);
            }
        }
    }

    /// Get error metrics
    pub fn get_metrics(&self) -> &ErrorMetrics {
        &self.error_metrics
    }

    /// Reset error metrics
    pub fn reset_metrics(&mut self) {
        self.error_metrics = ErrorMetrics::new();
    }

    /// Get circuit breaker status for a service
    pub fn get_circuit_breaker_status(&self, service: &str) -> Option<&CircuitBreaker> {
        self.circuit_breakers.get(service)
    }
}

impl Default for ErrorHandlerConfig {
    fn default() -> Self {
        Self {
            max_retry_attempts: 3,
            base_retry_delay: Duration::from_millis(1000),
            max_retry_delay: Duration::from_secs(60),
            circuit_breaker_threshold: 5,
            circuit_breaker_timeout: Duration::from_secs(60),
            enable_fallback: true,
            enable_metrics: true,
        }
    }
}

impl CircuitBreaker {
    fn new() -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
            last_success_time: None,
        }
    }
}

impl ErrorMetrics {
    fn new() -> Self {
        Self {
            total_errors: 0,
            errors_by_type: HashMap::new(),
            retry_attempts: 0,
            successful_retries: 0,
            fallback_activations: 0,
            circuit_breaker_trips: 0,
            last_reset: chrono::Utc::now(),
        }
    }
}

impl ErrorContext {
    pub fn new(operation: String, service: String) -> Self {
        Self {
            operation,
            service,
            user_id: None,
            request_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Instant::now(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error_classification() {
        let handler = ErrorHandler::new(ErrorHandlerConfig::default());
        
        let rate_limit_error = AIServiceError::ExternalAPIError("rate limit exceeded".to_string());
        assert_eq!(handler.classify_error(&rate_limit_error), ErrorType::RateLimitError);
        
        let auth_error = AIServiceError::ExternalAPIError("authentication failed".to_string());
        assert_eq!(handler.classify_error(&auth_error), ErrorType::AuthenticationError);
        
        let db_error = AIServiceError::DatabaseError("connection failed".to_string());
        assert_eq!(handler.classify_error(&db_error), ErrorType::VectorDatabaseError);
    }

    #[tokio::test]
    async fn test_retry_delay_calculation() {
        let handler = ErrorHandler::new(ErrorHandlerConfig::default());
        let policy = RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_millis(1000),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            jitter: false,
        };
        
        let delay1 = handler.calculate_retry_delay(&policy, 1);
        let delay2 = handler.calculate_retry_delay(&policy, 2);
        
        assert_eq!(delay1, Duration::from_millis(1000));
        assert_eq!(delay2, Duration::from_millis(2000));
    }

    #[tokio::test]
    async fn test_circuit_breaker() {
        let mut handler = ErrorHandler::new(ErrorHandlerConfig::default());
        let service = "test_service";
        
        // Record failures to trip circuit breaker
        for _ in 0..5 {
            handler.record_result(service, false);
        }
        
        let circuit_breaker = handler.get_circuit_breaker_status(service).unwrap();
        assert_eq!(circuit_breaker.state, CircuitBreakerState::Open);
    }

    #[test]
    fn test_error_context_creation() {
        let context = ErrorContext::new("test_operation".to_string(), "test_service".to_string())
            .with_user_id("user123".to_string())
            .with_metadata("key".to_string(), "value".to_string());
        
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.service, "test_service");
        assert_eq!(context.user_id, Some("user123".to_string()));
        assert_eq!(context.metadata.get("key"), Some(&"value".to_string()));
    }
}
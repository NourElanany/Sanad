//! Retry mechanism with exponential backoff for API requests

use crate::api_clients::error::ApiError;
use std::time::Duration;
use tokio::time::sleep;

#[cfg(test)]
#[path = "retry_mechanism_property_tests.rs"]
mod property_tests;

/// Retry strategy configuration
#[derive(Debug, Clone)]
pub struct RetryStrategy {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(10),
            multiplier: 2.0,
        }
    }
}

impl RetryStrategy {
    /// Create a new retry strategy
    pub fn new(max_attempts: u32, initial_delay: Duration, max_delay: Duration, multiplier: f64) -> Self {
        Self {
            max_attempts,
            initial_delay,
            max_delay,
            multiplier,
        }
    }
    
    /// Calculate delay for a given attempt number (0-indexed)
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::from_secs(0);
        }
        
        let delay_secs = self.initial_delay.as_secs_f64() * self.multiplier.powi((attempt - 1) as i32);
        let delay_secs = delay_secs.min(self.max_delay.as_secs_f64());
        Duration::from_secs_f64(delay_secs)
    }
    
    /// Get all delays for the retry strategy
    pub fn get_delays(&self) -> Vec<Duration> {
        (0..self.max_attempts)
            .map(|attempt| self.calculate_delay(attempt))
            .collect()
    }
}

/// Retry mechanism for executing operations with exponential backoff
pub struct RetryMechanism {
    strategy: RetryStrategy,
}

impl RetryMechanism {
    /// Create a new retry mechanism with default strategy
    pub fn new() -> Self {
        Self {
            strategy: RetryStrategy::default(),
        }
    }
    
    /// Create a new retry mechanism with custom strategy
    pub fn with_strategy(strategy: RetryStrategy) -> Self {
        Self { strategy }
    }
    
    /// Execute an operation with retry logic
    /// 
    /// The operation will be retried up to max_attempts times with exponential backoff
    /// between attempts. Only retryable errors will trigger retries.
    pub async fn execute<F, Fut, T>(&self, mut operation: F) -> Result<T, ApiError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, ApiError>>,
    {
        let mut last_error = None;
        
        for attempt in 0..self.strategy.max_attempts {
            // Wait before retry (except for first attempt)
            if attempt > 0 {
                let delay = self.strategy.calculate_delay(attempt);
                log::debug!("Retry attempt {} after {:?}", attempt + 1, delay);
                sleep(delay).await;
            }
            
            match operation().await {
                Ok(result) => {
                    if attempt > 0 {
                        log::info!("Operation succeeded after {} retries", attempt);
                    }
                    return Ok(result);
                }
                Err(error) => {
                    log::warn!("Attempt {} failed: {}", attempt + 1, error);
                    
                    // Check if error is retryable
                    if !self.is_retryable(&error) {
                        log::debug!("Error is not retryable, failing immediately");
                        return Err(error);
                    }
                    
                    last_error = Some(error);
                }
            }
        }
        
        // All attempts failed
        log::error!("All {} retry attempts failed", self.strategy.max_attempts);
        Err(last_error.unwrap_or(ApiError::AllApisFailed))
    }
    
    /// Check if an error is retryable
    fn is_retryable(&self, error: &ApiError) -> bool {
        matches!(
            error,
            ApiError::Network(_) 
            | ApiError::Timeout 
            | ApiError::ApiError(_, _)
            | ApiError::Http(_)
        )
    }
    
    /// Get the retry strategy
    pub fn strategy(&self) -> &RetryStrategy {
        &self.strategy
    }
}

impl Default for RetryMechanism {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    
    #[test]
    fn test_retry_strategy_default() {
        let strategy = RetryStrategy::default();
        assert_eq!(strategy.max_attempts, 3);
        assert_eq!(strategy.initial_delay, Duration::from_secs(1));
        assert_eq!(strategy.max_delay, Duration::from_secs(10));
        assert_eq!(strategy.multiplier, 2.0);
    }
    
    #[test]
    fn test_calculate_delay_exponential() {
        let strategy = RetryStrategy::default();
        
        // First attempt has no delay
        assert_eq!(strategy.calculate_delay(0), Duration::from_secs(0));
        
        // Subsequent attempts follow exponential backoff
        assert_eq!(strategy.calculate_delay(1), Duration::from_secs(1));  // 1 * 2^0
        assert_eq!(strategy.calculate_delay(2), Duration::from_secs(2));  // 1 * 2^1
        assert_eq!(strategy.calculate_delay(3), Duration::from_secs(4));  // 1 * 2^2
    }
    
    #[test]
    fn test_calculate_delay_max_cap() {
        let strategy = RetryStrategy {
            max_attempts: 10,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(5),
            multiplier: 2.0,
        };
        
        // Should cap at max_delay
        assert_eq!(strategy.calculate_delay(5), Duration::from_secs(5));
        assert_eq!(strategy.calculate_delay(10), Duration::from_secs(5));
    }
    
    #[test]
    fn test_get_delays() {
        let strategy = RetryStrategy {
            max_attempts: 4,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(10),
            multiplier: 2.0,
        };
        
        let delays = strategy.get_delays();
        assert_eq!(delays.len(), 4);
        assert_eq!(delays[0], Duration::from_secs(0));
        assert_eq!(delays[1], Duration::from_secs(1));
        assert_eq!(delays[2], Duration::from_secs(2));
        assert_eq!(delays[3], Duration::from_secs(4));
    }
    
    #[tokio::test]
    async fn test_retry_mechanism_success_first_attempt() {
        let retry = RetryMechanism::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        
        let result = retry.execute(|| {
            let counter = counter_clone.clone();
            async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Ok::<_, ApiError>(42)
            }
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
    
    #[tokio::test]
    async fn test_retry_mechanism_success_after_retries() {
        let retry = RetryMechanism::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        
        let result = retry.execute(|| {
            let counter = counter_clone.clone();
            async move {
                let count = counter.fetch_add(1, Ordering::SeqCst);
                if count < 2 {
                    Err(ApiError::Network("Temporary failure".to_string()))
                } else {
                    Ok::<_, ApiError>(42)
                }
            }
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
    
    #[tokio::test]
    async fn test_retry_mechanism_all_attempts_fail() {
        let retry = RetryMechanism::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        
        let result = retry.execute(|| {
            let counter = counter_clone.clone();
            async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Err::<i32, _>(ApiError::Network("Persistent failure".to_string()))
            }
        }).await;
        
        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
    
    #[tokio::test]
    async fn test_retry_mechanism_non_retryable_error() {
        let retry = RetryMechanism::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        
        let result = retry.execute(|| {
            let counter = counter_clone.clone();
            async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Err::<i32, _>(ApiError::Authentication("Invalid credentials".to_string()))
            }
        }).await;
        
        assert!(result.is_err());
        // Should fail immediately without retries
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
    
    #[test]
    fn test_is_retryable() {
        let retry = RetryMechanism::new();
        
        // Retryable errors
        assert!(retry.is_retryable(&ApiError::Network("test".to_string())));
        assert!(retry.is_retryable(&ApiError::Timeout));
        assert!(retry.is_retryable(&ApiError::ApiError("api".to_string(), "error".to_string())));
        
        // Non-retryable errors
        assert!(!retry.is_retryable(&ApiError::Authentication("test".to_string())));
        assert!(!retry.is_retryable(&ApiError::Validation("test".to_string())));
        assert!(!retry.is_retryable(&ApiError::RateLimitExceeded("test".to_string())));
    }
}

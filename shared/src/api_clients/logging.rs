//! Structured logging utilities for API clients
//!
//! This module provides utilities for logging API calls with:
//! - Correlation IDs for request tracking
//! - Timing information for performance monitoring
//! - Structured fields for observability
//! - Automatic metrics recording
//! - Distributed tracing integration

use std::time::Instant;
use tracing::{error, info, warn};
use crate::api_clients::{metrics, tracing as api_tracing};

/// Get the current correlation ID from the request context
///
/// This function attempts to retrieve the correlation ID from the task-local
/// request context. If no context is available, it returns None.
pub fn current_correlation_id() -> Option<String> {
    // This will be implemented by the service layer to access the request context
    // For now, return None as a default
    None
}

/// Log an API call with timing and structured fields
///
/// # Arguments
/// * `api_name` - Name of the API being called
/// * `operation` - Operation being performed (e.g., "get_ayah", "search_hadith")
/// * `request_id` - Optional correlation ID for request tracking
/// * `params` - Additional parameters to log (as key-value pairs)
///
/// # Returns
/// A `ApiCallLogger` that will log the result when dropped
pub fn log_api_call(
    api_name: &str,
    operation: &str,
    request_id: Option<&str>,
) -> ApiCallLogger {
    ApiCallLogger::new(api_name, operation, request_id)
}

/// Logger for API calls that tracks timing and logs results
pub struct ApiCallLogger {
    api_name: String,
    operation: String,
    request_id: Option<String>,
    start_time: Instant,
    tracer: Option<api_tracing::ApiCallTracer>,
}

impl ApiCallLogger {
    /// Create a new API call logger
    pub fn new(api_name: &str, operation: &str, request_id: Option<&str>) -> Self {
        let request_id_str = request_id.map(|s| s.to_string());
        
        // Create tracer for distributed tracing
        let tracer = Some(api_tracing::ApiCallTracer::new(api_name, operation, request_id));
        
        // Log the start of the API call
        if let Some(ref rid) = request_id_str {
            info!(
                request_id = %rid,
                api_name = %api_name,
                operation = %operation,
                "Starting API call"
            );
        } else {
            info!(
                api_name = %api_name,
                operation = %operation,
                "Starting API call"
            );
        }

        Self {
            api_name: api_name.to_string(),
            operation: operation.to_string(),
            request_id: request_id_str,
            start_time: Instant::now(),
            tracer,
        }
    }

    /// Log a successful API call
    pub fn success<T>(self, result: &T) -> T
    where
        T: Clone,
    {
        let duration = self.start_time.elapsed();
        
        // Record metrics
        metrics::record_api_call(&self.api_name, &self.operation);
        metrics::record_api_success(&self.api_name, &self.operation, duration);
        
        // Record tracing
        if let Some(tracer) = self.tracer {
            tracer.success();
        }
        
        if let Some(ref request_id) = self.request_id {
            info!(
                request_id = %request_id,
                api_name = %self.api_name,
                operation = %self.operation,
                duration_ms = %duration.as_millis(),
                status = "success",
                "API call completed successfully"
            );
        } else {
            info!(
                api_name = %self.api_name,
                operation = %self.operation,
                duration_ms = %duration.as_millis(),
                status = "success",
                "API call completed successfully"
            );
        }

        result.clone()
    }

    /// Log a failed API call
    pub fn failure<E>(self, error: &E)
    where
        E: std::fmt::Display,
    {
        let duration = self.start_time.elapsed();
        
        // Record metrics with error category
        metrics::record_api_call(&self.api_name, &self.operation);
        metrics::record_api_failure(&self.api_name, &self.operation, duration, "unknown");
        
        // Record tracing
        if let Some(tracer) = self.tracer {
            tracer.failure(&error.to_string());
        }
        
        if let Some(ref request_id) = self.request_id {
            error!(
                request_id = %request_id,
                api_name = %self.api_name,
                operation = %self.operation,
                duration_ms = %duration.as_millis(),
                status = "failure",
                error = %error,
                "API call failed"
            );
        } else {
            error!(
                api_name = %self.api_name,
                operation = %self.operation,
                duration_ms = %duration.as_millis(),
                status = "failure",
                error = %error,
                "API call failed"
            );
        }
    }

    /// Log a cached response (no actual API call made)
    pub fn cached<T>(self, result: &T) -> T
    where
        T: Clone,
    {
        let duration = self.start_time.elapsed();
        
        // Record cache hit metric
        metrics::record_cache_hit(&self.operation);
        
        // Record tracing
        if let Some(tracer) = self.tracer {
            tracer.cached();
        }
        
        if let Some(ref request_id) = self.request_id {
            info!(
                request_id = %request_id,
                api_name = %self.api_name,
                operation = %self.operation,
                duration_ms = %duration.as_millis(),
                status = "cached",
                "Returned cached response"
            );
        } else {
            info!(
                api_name = %self.api_name,
                operation = %self.operation,
                duration_ms = %duration.as_millis(),
                status = "cached",
                "Returned cached response"
            );
        }

        result.clone()
    }

    /// Log a fallback to another API
    pub fn fallback(self, fallback_api: &str, reason: &str) {
        let duration = self.start_time.elapsed();
        
        // Record fallback metric
        metrics::record_api_fallback(&self.api_name, fallback_api, reason);
        
        if let Some(ref request_id) = self.request_id {
            warn!(
                request_id = %request_id,
                api_name = %self.api_name,
                operation = %self.operation,
                duration_ms = %duration.as_millis(),
                status = "fallback",
                fallback_api = %fallback_api,
                reason = %reason,
                "Falling back to alternative API"
            );
        } else {
            warn!(
                api_name = %self.api_name,
                operation = %self.operation,
                duration_ms = %duration.as_millis(),
                status = "fallback",
                fallback_api = %fallback_api,
                reason = %reason,
                "Falling back to alternative API"
            );
        }
    }
}

/// Extension trait for Result to easily log API call results
pub trait LogApiResult<T, E> {
    /// Log the result of an API call
    fn log_result(self, logger: ApiCallLogger) -> Result<T, E>;
}

impl<T, E> LogApiResult<T, E> for Result<T, E>
where
    T: Clone,
    E: std::fmt::Display,
{
    fn log_result(self, logger: ApiCallLogger) -> Result<T, E> {
        match &self {
            Ok(result) => {
                logger.success(result);
            }
            Err(error) => {
                logger.failure(error);
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_call_logger_creation() {
        let logger = log_api_call("test_api", "test_operation", Some("test-request-id"));
        assert_eq!(logger.api_name, "test_api");
        assert_eq!(logger.operation, "test_operation");
        assert_eq!(logger.request_id, Some("test-request-id".to_string()));
    }

    #[test]
    fn test_api_call_logger_without_request_id() {
        let logger = log_api_call("test_api", "test_operation", None);
        assert_eq!(logger.api_name, "test_api");
        assert_eq!(logger.operation, "test_operation");
        assert_eq!(logger.request_id, None);
    }

    #[test]
    fn test_success_logging() {
        let logger = log_api_call("test_api", "test_operation", Some("test-request-id"));
        let result = "test result";
        let logged_result = logger.success(&result);
        assert_eq!(logged_result, result);
    }

    #[test]
    fn test_failure_logging() {
        let logger = log_api_call("test_api", "test_operation", Some("test-request-id"));
        let error = "test error";
        logger.failure(&error);
        // Just ensure it doesn't panic
    }

    #[test]
    fn test_cached_logging() {
        let logger = log_api_call("test_api", "test_operation", Some("test-request-id"));
        let result = "cached result";
        let logged_result = logger.cached(&result);
        assert_eq!(logged_result, result);
    }

    #[test]
    fn test_fallback_logging() {
        let logger = log_api_call("test_api", "test_operation", Some("test-request-id"));
        logger.fallback("fallback_api", "primary API failed");
        // Just ensure it doesn't panic
    }

    #[test]
    fn test_log_result_success() {
        let logger = log_api_call("test_api", "test_operation", Some("test-request-id"));
        let result: Result<String, String> = Ok("success".to_string());
        let logged = result.log_result(logger);
        assert!(logged.is_ok());
    }

    #[test]
    fn test_log_result_failure() {
        let logger = log_api_call("test_api", "test_operation", Some("test-request-id"));
        let result: Result<String, String> = Err("error".to_string());
        let logged = result.log_result(logger);
        assert!(logged.is_err());
    }
}

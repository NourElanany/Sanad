//! OpenTelemetry tracing for API clients
//!
//! This module provides distributed tracing capabilities for:
//! - Request flows across services
//! - API calls to external services
//! - Cache operations
//! - Rate limiting checks

use std::time::Instant;
use tracing::{info, span, Level, Span};

/// Create a span for an API call
pub fn api_call_span(api_name: &str, operation: &str, request_id: Option<&str>) -> Span {
    if let Some(rid) = request_id {
        span!(
            Level::INFO,
            "api_call",
            api_name = %api_name,
            operation = %operation,
            request_id = %rid
        )
    } else {
        span!(
            Level::INFO,
            "api_call",
            api_name = %api_name,
            operation = %operation
        )
    }
}

/// Create a span for a cache operation
pub fn cache_operation_span(operation: &str, key: &str) -> Span {
    span!(
        Level::DEBUG,
        "cache_operation",
        operation = %operation,
        key = %key
    )
}

/// Create a span for a rate limit check
pub fn rate_limit_span(api_name: &str) -> Span {
    span!(
        Level::DEBUG,
        "rate_limit_check",
        api_name = %api_name
    )
}

/// Create a span for a fallback operation
pub fn fallback_span(from_api: &str, to_api: &str, reason: &str) -> Span {
    span!(
        Level::WARN,
        "api_fallback",
        from_api = %from_api,
        to_api = %to_api,
        reason = %reason
    )
}

/// Create a span for error handling
pub fn error_handling_span(error_category: &str) -> Span {
    span!(
        Level::ERROR,
        "error_handling",
        category = %error_category
    )
}

/// Trace an API call with automatic timing
pub struct ApiCallTracer {
    span: Span,
    start_time: Instant,
}

impl ApiCallTracer {
    /// Create a new API call tracer
    pub fn new(api_name: &str, operation: &str, request_id: Option<&str>) -> Self {
        let span = api_call_span(api_name, operation, request_id);
        
        Self {
            span,
            start_time: Instant::now(),
        }
    }

    /// Record success and close the span
    pub fn success(self) {
        let duration = self.start_time.elapsed();
        let _enter = self.span.enter();
        info!(
            duration_ms = %duration.as_millis(),
            status = "success",
            "API call completed"
        );
    }

    /// Record failure and close the span
    pub fn failure(self, error: &str) {
        let duration = self.start_time.elapsed();
        let _enter = self.span.enter();
        tracing::error!(
            duration_ms = %duration.as_millis(),
            status = "failure",
            error = %error,
            "API call failed"
        );
    }

    /// Record cached response
    pub fn cached(self) {
        let duration = self.start_time.elapsed();
        let _enter = self.span.enter();
        info!(
            duration_ms = %duration.as_millis(),
            status = "cached",
            "Returned cached response"
        );
    }

    /// Get a reference to the span for manual operations
    pub fn span(&self) -> &Span {
        &self.span
    }
}

/// Trace a cache operation
pub struct CacheTracer {
    span: Span,
    start_time: Instant,
}

impl CacheTracer {
    /// Create a new cache tracer
    pub fn new(operation: &str, key: &str) -> Self {
        let span = cache_operation_span(operation, key);
        
        Self {
            span,
            start_time: Instant::now(),
        }
    }

    /// Record hit
    pub fn hit(self) {
        let duration = self.start_time.elapsed();
        let _enter = self.span.enter();
        tracing::debug!(
            duration_ms = %duration.as_millis(),
            result = "hit",
            "Cache operation completed"
        );
    }

    /// Record miss
    pub fn miss(self) {
        let duration = self.start_time.elapsed();
        let _enter = self.span.enter();
        tracing::debug!(
            duration_ms = %duration.as_millis(),
            result = "miss",
            "Cache operation completed"
        );
    }

    /// Record set
    pub fn set(self) {
        let duration = self.start_time.elapsed();
        let _enter = self.span.enter();
        tracing::debug!(
            duration_ms = %duration.as_millis(),
            result = "set",
            "Cache operation completed"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_call_span_creation() {
        let span = api_call_span("test_api", "test_operation", Some("test-request-id"));
        assert_eq!(span.metadata().unwrap().name(), "api_call");
    }

    #[test]
    fn test_api_call_span_without_request_id() {
        let span = api_call_span("test_api", "test_operation", None);
        assert_eq!(span.metadata().unwrap().name(), "api_call");
    }

    #[test]
    fn test_cache_operation_span() {
        let span = cache_operation_span("get", "test_key");
        assert_eq!(span.metadata().unwrap().name(), "cache_operation");
    }

    #[test]
    fn test_rate_limit_span() {
        let span = rate_limit_span("test_api");
        assert_eq!(span.metadata().unwrap().name(), "rate_limit_check");
    }

    #[test]
    fn test_fallback_span() {
        let span = fallback_span("primary_api", "secondary_api", "timeout");
        assert_eq!(span.metadata().unwrap().name(), "api_fallback");
    }

    #[test]
    fn test_error_handling_span() {
        let span = error_handling_span("network_error");
        assert_eq!(span.metadata().unwrap().name(), "error_handling");
    }

    #[test]
    fn test_api_call_tracer() {
        let tracer = ApiCallTracer::new("test_api", "test_operation", Some("test-request-id"));
        tracer.success();
        // Should not panic
    }

    #[test]
    fn test_api_call_tracer_failure() {
        let tracer = ApiCallTracer::new("test_api", "test_operation", None);
        tracer.failure("test error");
        // Should not panic
    }

    #[test]
    fn test_api_call_tracer_cached() {
        let tracer = ApiCallTracer::new("test_api", "test_operation", None);
        tracer.cached();
        // Should not panic
    }

    #[test]
    fn test_cache_tracer() {
        let tracer = CacheTracer::new("get", "test_key");
        tracer.hit();
        // Should not panic
    }

    #[test]
    fn test_cache_tracer_miss() {
        let tracer = CacheTracer::new("get", "test_key");
        tracer.miss();
        // Should not panic
    }

    #[test]
    fn test_cache_tracer_set() {
        let tracer = CacheTracer::new("set", "test_key");
        tracer.set();
        // Should not panic
    }
}

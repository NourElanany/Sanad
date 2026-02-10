//! Prometheus metrics for API clients
//!
//! This module provides metrics collection for:
//! - API call counts (success/failure)
//! - Cache hit/miss rates
//! - Response times
//! - Rate limit usage
//! - Error rates by category

use std::time::Duration;

/// Initialize all metrics with descriptions
pub fn init_metrics() {
    // API call metrics
    metrics::describe_counter!(
        "api_calls_total",
        "Total number of API calls made"
    );
    metrics::describe_counter!(
        "api_calls_success_total",
        "Total number of successful API calls"
    );
    metrics::describe_counter!(
        "api_calls_failure_total",
        "Total number of failed API calls"
    );
    metrics::describe_histogram!(
        "api_call_duration_seconds",
        "Duration of API calls in seconds"
    );
    
    // Cache metrics
    metrics::describe_counter!(
        "cache_hits_total",
        "Total number of cache hits"
    );
    metrics::describe_counter!(
        "cache_misses_total",
        "Total number of cache misses"
    );
    metrics::describe_counter!(
        "cache_sets_total",
        "Total number of cache sets"
    );
    metrics::describe_counter!(
        "cache_evictions_total",
        "Total number of cache evictions"
    );
    
    // Rate limit metrics
    metrics::describe_gauge!(
        "rate_limit_remaining",
        "Remaining requests in current rate limit window"
    );
    metrics::describe_counter!(
        "rate_limit_exceeded_total",
        "Total number of times rate limit was exceeded"
    );
    
    // Error metrics
    metrics::describe_counter!(
        "api_errors_total",
        "Total number of API errors by category"
    );
    
    // Fallback metrics
    metrics::describe_counter!(
        "api_fallbacks_total",
        "Total number of API fallbacks"
    );
    metrics::describe_counter!(
        "stale_cache_served_total",
        "Total number of times stale cache was served"
    );
}

/// Record an API call
pub fn record_api_call(api_name: &str, operation: &str) {
    metrics::increment_counter!("api_calls_total", "api" => api_name.to_string(), "operation" => operation.to_string());
}

/// Record a successful API call
pub fn record_api_success(api_name: &str, operation: &str, duration: Duration) {
    metrics::increment_counter!("api_calls_success_total", "api" => api_name.to_string(), "operation" => operation.to_string());
    metrics::histogram!("api_call_duration_seconds", duration.as_secs_f64(), "api" => api_name.to_string(), "operation" => operation.to_string(), "status" => "success");
}

/// Record a failed API call
pub fn record_api_failure(api_name: &str, operation: &str, duration: Duration, error_category: &str) {
    metrics::increment_counter!("api_calls_failure_total", "api" => api_name.to_string(), "operation" => operation.to_string());
    metrics::histogram!("api_call_duration_seconds", duration.as_secs_f64(), "api" => api_name.to_string(), "operation" => operation.to_string(), "status" => "failure");
    metrics::increment_counter!("api_errors_total", "api" => api_name.to_string(), "category" => error_category.to_string());
}

/// Record a cache hit
pub fn record_cache_hit(cache_type: &str) {
    metrics::increment_counter!("cache_hits_total", "type" => cache_type.to_string());
}

/// Record a cache miss
pub fn record_cache_miss(cache_type: &str) {
    metrics::increment_counter!("cache_misses_total", "type" => cache_type.to_string());
}

/// Record a cache set operation
pub fn record_cache_set(cache_type: &str) {
    metrics::increment_counter!("cache_sets_total", "type" => cache_type.to_string());
}

/// Record a cache eviction
pub fn record_cache_eviction(cache_type: &str) {
    metrics::increment_counter!("cache_evictions_total", "type" => cache_type.to_string());
}

/// Update rate limit remaining count
pub fn update_rate_limit_remaining(api_name: &str, window: &str, remaining: u32) {
    metrics::gauge!("rate_limit_remaining", remaining as f64, "api" => api_name.to_string(), "window" => window.to_string());
}

/// Record rate limit exceeded
pub fn record_rate_limit_exceeded(api_name: &str, window: &str) {
    metrics::increment_counter!("rate_limit_exceeded_total", "api" => api_name.to_string(), "window" => window.to_string());
}

/// Record an API fallback
pub fn record_api_fallback(from_api: &str, to_api: &str, reason: &str) {
    metrics::increment_counter!("api_fallbacks_total", "from" => from_api.to_string(), "to" => to_api.to_string(), "reason" => reason.to_string());
}

/// Record serving stale cache
pub fn record_stale_cache_served(cache_type: &str) {
    metrics::increment_counter!("stale_cache_served_total", "type" => cache_type.to_string());
}

/// Calculate cache hit rate
pub fn calculate_cache_hit_rate(hits: u64, misses: u64) -> f64 {
    let total = hits + misses;
    if total == 0 {
        0.0
    } else {
        (hits as f64 / total as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_hit_rate_calculation() {
        assert_eq!(calculate_cache_hit_rate(80, 20), 80.0);
        assert_eq!(calculate_cache_hit_rate(0, 100), 0.0);
        assert_eq!(calculate_cache_hit_rate(100, 0), 100.0);
        assert_eq!(calculate_cache_hit_rate(0, 0), 0.0);
    }

    #[test]
    fn test_cache_hit_rate_precision() {
        let rate = calculate_cache_hit_rate(1, 3);
        assert!((rate - 25.0).abs() < 0.01);
    }

    #[test]
    fn test_metrics_recording() {
        // Initialize metrics
        init_metrics();
        
        // Record various metrics
        record_api_call("test_api", "test_operation");
        record_api_success("test_api", "test_operation", Duration::from_millis(100));
        record_cache_hit("quran_text");
        record_cache_miss("hadith");
        update_rate_limit_remaining("test_api", "minute", 50);
        
        // These should not panic
    }

    #[test]
    fn test_error_recording() {
        init_metrics();
        
        record_api_failure(
            "test_api",
            "test_operation",
            Duration::from_millis(500),
            "network_error"
        );
        
        // Should not panic
    }

    #[test]
    fn test_fallback_recording() {
        init_metrics();
        
        record_api_fallback("primary_api", "secondary_api", "timeout");
        record_stale_cache_served("prayer_times");
        
        // Should not panic
    }
}

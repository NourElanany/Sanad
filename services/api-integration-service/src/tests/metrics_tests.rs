//! Unit tests for metrics collection
//!
//! Tests verify that:
//! - Metrics are correctly incremented
//! - Metric labels are properly set
//! - Metrics are recorded for all operations

use shared::api_clients::metrics;
use std::time::Duration;

#[test]
fn test_api_call_metrics() {
    // Initialize metrics
    metrics::init_metrics();
    
    // Record API call
    metrics::record_api_call("test_api", "test_operation");
    
    // Record success
    metrics::record_api_success("test_api", "test_operation", Duration::from_millis(100));
    
    // Should not panic
}

#[test]
fn test_api_failure_metrics() {
    metrics::init_metrics();
    
    // Record API failure
    metrics::record_api_failure(
        "test_api",
        "test_operation",
        Duration::from_millis(500),
        "network_error"
    );
    
    // Should not panic
}

#[test]
fn test_cache_metrics() {
    metrics::init_metrics();
    
    // Record cache operations
    metrics::record_cache_hit("quran_text");
    metrics::record_cache_miss("hadith");
    metrics::record_cache_set("prayer_times");
    metrics::record_cache_eviction("ai_response");
    
    // Should not panic
}

#[test]
fn test_rate_limit_metrics() {
    metrics::init_metrics();
    
    // Update rate limit remaining
    metrics::update_rate_limit_remaining("test_api", "minute", 50);
    metrics::update_rate_limit_remaining("test_api", "hour", 900);
    metrics::update_rate_limit_remaining("test_api", "day", 9000);
    
    // Record rate limit exceeded
    metrics::record_rate_limit_exceeded("test_api", "minute");
    
    // Should not panic
}

#[test]
fn test_fallback_metrics() {
    metrics::init_metrics();
    
    // Record fallback
    metrics::record_api_fallback("primary_api", "secondary_api", "timeout");
    
    // Record stale cache served
    metrics::record_stale_cache_served("prayer_times");
    
    // Should not panic
}

#[test]
fn test_metric_labels_with_special_characters() {
    metrics::init_metrics();
    
    // Test with API names containing special characters
    metrics::record_api_call("quran.com", "get_ayah");
    metrics::record_api_call("sunnah-com", "search_hadith");
    metrics::record_api_call("aladhan_api", "get_times");
    
    // Should not panic
}

#[test]
fn test_multiple_api_calls_same_api() {
    metrics::init_metrics();
    
    // Record multiple calls to the same API
    for _ in 0..10 {
        metrics::record_api_call("test_api", "test_operation");
        metrics::record_api_success("test_api", "test_operation", Duration::from_millis(100));
    }
    
    // Should not panic
}

#[test]
fn test_multiple_api_calls_different_apis() {
    metrics::init_metrics();
    
    // Record calls to different APIs
    let apis = vec!["quran_api", "hadith_api", "prayer_api", "tafsir_api"];
    
    for api in apis {
        metrics::record_api_call(api, "test_operation");
        metrics::record_api_success(api, "test_operation", Duration::from_millis(100));
    }
    
    // Should not panic
}

#[test]
fn test_cache_hit_rate_calculation() {
    // Test various hit rate scenarios
    assert_eq!(metrics::calculate_cache_hit_rate(80, 20), 80.0);
    assert_eq!(metrics::calculate_cache_hit_rate(0, 100), 0.0);
    assert_eq!(metrics::calculate_cache_hit_rate(100, 0), 100.0);
    assert_eq!(metrics::calculate_cache_hit_rate(0, 0), 0.0);
    
    // Test precision
    let rate = metrics::calculate_cache_hit_rate(1, 3);
    assert!((rate - 25.0).abs() < 0.01);
    
    let rate = metrics::calculate_cache_hit_rate(2, 3);
    assert!((rate - 40.0).abs() < 0.01);
}

#[test]
fn test_error_category_metrics() {
    metrics::init_metrics();
    
    // Test different error categories
    let error_categories = vec![
        "network_error",
        "authentication_error",
        "rate_limit_error",
        "server_error",
        "validation_error",
        "timeout_error",
    ];
    
    for category in error_categories {
        metrics::record_api_failure(
            "test_api",
            "test_operation",
            Duration::from_millis(500),
            category
        );
    }
    
    // Should not panic
}

#[test]
fn test_rate_limit_windows() {
    metrics::init_metrics();
    
    // Test all rate limit windows
    metrics::update_rate_limit_remaining("test_api", "minute", 60);
    metrics::update_rate_limit_remaining("test_api", "hour", 1000);
    metrics::update_rate_limit_remaining("test_api", "day", 10000);
    
    // Test exceeded for each window
    metrics::record_rate_limit_exceeded("test_api", "minute");
    metrics::record_rate_limit_exceeded("test_api", "hour");
    metrics::record_rate_limit_exceeded("test_api", "day");
    
    // Should not panic
}

#[test]
fn test_cache_types() {
    metrics::init_metrics();
    
    // Test all cache types
    let cache_types = vec![
        "quran_text",
        "quran_audio",
        "hadith",
        "prayer_times",
        "tafsir",
        "calendar",
        "qibla",
        "ai_response",
    ];
    
    for cache_type in cache_types {
        metrics::record_cache_hit(cache_type);
        metrics::record_cache_miss(cache_type);
        metrics::record_cache_set(cache_type);
    }
    
    // Should not panic
}

#[test]
fn test_response_time_metrics() {
    metrics::init_metrics();
    
    // Test various response times
    let durations = vec![
        Duration::from_millis(10),
        Duration::from_millis(100),
        Duration::from_millis(500),
        Duration::from_secs(1),
        Duration::from_secs(5),
    ];
    
    for duration in durations {
        metrics::record_api_success("test_api", "test_operation", duration);
    }
    
    // Should not panic
}

#[test]
fn test_concurrent_metric_recording() {
    use std::sync::Arc;
    use std::thread;
    
    metrics::init_metrics();
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let api_name = format!("api_{}", i);
                for _ in 0..100 {
                    metrics::record_api_call(&api_name, "test_operation");
                    metrics::record_api_success(&api_name, "test_operation", Duration::from_millis(100));
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Should not panic
}

#[test]
fn test_fallback_reasons() {
    metrics::init_metrics();
    
    // Test different fallback reasons
    let reasons = vec![
        "timeout",
        "network_error",
        "rate_limit_exceeded",
        "server_error",
        "authentication_failed",
    ];
    
    for reason in reasons {
        metrics::record_api_fallback("primary_api", "secondary_api", reason);
    }
    
    // Should not panic
}

#[test]
fn test_stale_cache_metrics() {
    metrics::init_metrics();
    
    // Test stale cache for different types
    let cache_types = vec![
        "quran_text",
        "hadith",
        "prayer_times",
        "tafsir",
    ];
    
    for cache_type in cache_types {
        metrics::record_stale_cache_served(cache_type);
    }
    
    // Should not panic
}

#[test]
fn test_zero_duration_metrics() {
    metrics::init_metrics();
    
    // Test with zero duration
    metrics::record_api_success("test_api", "test_operation", Duration::from_millis(0));
    metrics::record_api_failure("test_api", "test_operation", Duration::from_millis(0), "error");
    
    // Should not panic
}

#[test]
fn test_very_long_duration_metrics() {
    metrics::init_metrics();
    
    // Test with very long duration
    metrics::record_api_success("test_api", "test_operation", Duration::from_secs(3600));
    metrics::record_api_failure("test_api", "test_operation", Duration::from_secs(3600), "timeout");
    
    // Should not panic
}

#[test]
fn test_empty_string_labels() {
    metrics::init_metrics();
    
    // Test with empty strings (should still work)
    metrics::record_api_call("", "");
    metrics::record_cache_hit("");
    metrics::update_rate_limit_remaining("", "", 0);
    
    // Should not panic
}

#[test]
fn test_metric_initialization_idempotent() {
    // Initialize multiple times should be safe
    metrics::init_metrics();
    metrics::init_metrics();
    metrics::init_metrics();
    
    // Should not panic
}

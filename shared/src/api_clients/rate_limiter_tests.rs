//! Unit tests for Rate Limiter edge cases
//! 
//! Tests boundary conditions, concurrent requests, and error scenarios

#[cfg(test)]
mod unit_tests {
    use crate::api_clients::{RateLimiter, RateLimitConfig};
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::task::JoinSet;

    // Helper to create a test rate limiter
    async fn create_test_limiter(
        api_name: &str,
        per_minute: u32,
        per_hour: u32,
        per_day: u32,
    ) -> RateLimiter {
        let mut configs = HashMap::new();
        configs.insert(
            api_name.to_string(),
            RateLimitConfig {
                requests_per_minute: per_minute,
                requests_per_hour: per_hour,
                requests_per_day: per_day,
            },
        );

        RateLimiter::new("redis://127.0.0.1:6379", configs)
            .await
            .expect("Failed to create rate limiter")
    }

    #[tokio::test]
    async fn test_exactly_at_limit() {
        let api_name = format!("test_exact_limit_{}", rand::random::<u32>());
        let limit = 10;
        let limiter = create_test_limiter(&api_name, limit, 1000, 10000).await;
        
        // Reset to ensure clean state
        limiter.reset(&api_name).await.unwrap();
        
        // Make exactly 'limit' requests
        for i in 0..limit {
            let allowed = limiter.check_and_increment(&api_name).await.unwrap();
            assert!(allowed, "Request {} should be allowed", i + 1);
        }
        
        // The next request should be denied
        let allowed = limiter.check_and_increment(&api_name).await.unwrap();
        assert!(!allowed, "Request {} should be denied (limit reached)", limit + 1);
        
        // Verify usage
        let usage = limiter.get_usage(&api_name).await.unwrap();
        assert_eq!(usage.minute_count, limit);
    }

    #[tokio::test]
    async fn test_concurrent_requests() {
        let api_name = format!("test_concurrent_{}", rand::random::<u32>());
        let limit = 20;
        let concurrent_requests = 50;
        let limiter = Arc::new(create_test_limiter(&api_name, limit, 1000, 10000).await);
        
        // Reset to ensure clean state
        limiter.reset(&api_name).await.unwrap();
        
        // Spawn concurrent tasks
        let mut tasks = JoinSet::new();
        for _ in 0..concurrent_requests {
            let limiter_clone = Arc::clone(&limiter);
            let api_name_clone = api_name.clone();
            tasks.spawn(async move {
                limiter_clone.check_and_increment(&api_name_clone).await.unwrap()
            });
        }
        
        // Collect results
        let mut allowed_count = 0;
        let mut denied_count = 0;
        while let Some(result) = tasks.join_next().await {
            if result.unwrap() {
                allowed_count += 1;
            } else {
                denied_count += 1;
            }
        }
        
        // Verify that we didn't exceed the limit
        assert!(
            allowed_count <= limit,
            "Allowed {} requests but limit was {}",
            allowed_count,
            limit
        );
        
        // Verify that some requests were denied
        assert!(
            denied_count > 0,
            "Expected some requests to be denied with {} concurrent requests and limit {}",
            concurrent_requests,
            limit
        );
        
        // Verify total
        assert_eq!(allowed_count + denied_count, concurrent_requests);
    }

    #[tokio::test]
    async fn test_check_without_increment() {
        let api_name = format!("test_check_only_{}", rand::random::<u32>());
        let limiter = create_test_limiter(&api_name, 10, 1000, 10000).await;
        
        // Reset to ensure clean state
        limiter.reset(&api_name).await.unwrap();
        
        // Multiple checks without increment should not affect the counter
        for _ in 0..5 {
            let allowed = limiter.check(&api_name).await.unwrap();
            assert!(allowed, "Check should always return true when counter is 0");
        }
        
        let usage = limiter.get_usage(&api_name).await.unwrap();
        assert_eq!(usage.minute_count, 0, "Counter should remain 0 after checks");
    }

    #[tokio::test]
    async fn test_increment_without_check() {
        let api_name = format!("test_increment_only_{}", rand::random::<u32>());
        let limiter = create_test_limiter(&api_name, 10, 1000, 10000).await;
        
        // Reset to ensure clean state
        limiter.reset(&api_name).await.unwrap();
        
        // Increment multiple times
        for _ in 0..5 {
            limiter.increment(&api_name).await.unwrap();
        }
        
        let usage = limiter.get_usage(&api_name).await.unwrap();
        assert_eq!(usage.minute_count, 5, "Counter should be 5 after 5 increments");
    }

    #[tokio::test]
    async fn test_unknown_api_error() {
        let limiter = create_test_limiter("known_api", 10, 1000, 10000).await;
        
        // Try to check an unknown API
        let result = limiter.check("unknown_api").await;
        assert!(result.is_err(), "Should return error for unknown API");
        
        match result {
            Err(crate::api_clients::ApiError::UnknownApi(name)) => {
                assert_eq!(name, "unknown_api");
            }
            _ => panic!("Expected UnknownApi error"),
        }
    }

    #[tokio::test]
    async fn test_check_and_increment_or_error() {
        let api_name = format!("test_or_error_{}", rand::random::<u32>());
        let limit = 5;
        let limiter = create_test_limiter(&api_name, limit, 1000, 10000).await;
        
        // Reset to ensure clean state
        limiter.reset(&api_name).await.unwrap();
        
        // First 'limit' requests should succeed
        for i in 0..limit {
            let result = limiter.check_and_increment_or_error(&api_name).await;
            assert!(result.is_ok(), "Request {} should succeed", i + 1);
        }
        
        // Next request should return an error
        let result = limiter.check_and_increment_or_error(&api_name).await;
        assert!(result.is_err(), "Request should fail when limit exceeded");
        
        match result {
            Err(crate::api_clients::ApiError::RateLimitExceeded(msg)) => {
                assert!(msg.contains(&api_name), "Error message should contain API name");
            }
            _ => panic!("Expected RateLimitExceeded error"),
        }
    }

    #[tokio::test]
    async fn test_time_until_reset() {
        let api_name = format!("test_reset_time_{}", rand::random::<u32>());
        let limit = 5;
        let limiter = create_test_limiter(&api_name, limit, 1000, 10000).await;
        
        // Reset to ensure clean state
        limiter.reset(&api_name).await.unwrap();
        
        // When not at limit, should return None
        let reset_time = limiter.time_until_reset(&api_name).await.unwrap();
        assert!(reset_time.is_none(), "Should return None when not at limit");
        
        // Fill up the limit
        for _ in 0..limit {
            limiter.check_and_increment(&api_name).await.unwrap();
        }
        
        // Now should return Some duration
        let reset_time = limiter.time_until_reset(&api_name).await.unwrap();
        assert!(reset_time.is_some(), "Should return Some when at limit");
        
        let duration = reset_time.unwrap();
        assert!(duration.as_secs() > 0, "Reset time should be positive");
        assert!(duration.as_secs() <= 60, "Reset time should be within a minute");
    }

    #[tokio::test]
    async fn test_is_approaching_limit() {
        let api_name = format!("test_approaching_{}", rand::random::<u32>());
        let limit = 10;
        let limiter = create_test_limiter(&api_name, limit, 1000, 10000).await;
        
        // Reset to ensure clean state
        limiter.reset(&api_name).await.unwrap();
        
        // At 0%, should not be approaching
        let approaching = limiter.is_approaching_limit(&api_name, 0.8).await.unwrap();
        assert!(!approaching, "Should not be approaching at 0%");
        
        // Make 7 requests (70%)
        for _ in 0..7 {
            limiter.check_and_increment(&api_name).await.unwrap();
        }
        
        // At 70%, should not be approaching 80% threshold
        let approaching = limiter.is_approaching_limit(&api_name, 0.8).await.unwrap();
        assert!(!approaching, "Should not be approaching at 70%");
        
        // Make 2 more requests (90%)
        for _ in 0..2 {
            limiter.check_and_increment(&api_name).await.unwrap();
        }
        
        // At 90%, should be approaching 80% threshold
        let approaching = limiter.is_approaching_limit(&api_name, 0.8).await.unwrap();
        assert!(approaching, "Should be approaching at 90%");
    }

    #[tokio::test]
    async fn test_multiple_time_windows() {
        let api_name = format!("test_multi_window_{}", rand::random::<u32>());
        let minute_limit = 5;
        let hour_limit = 10;
        let limiter = create_test_limiter(&api_name, minute_limit, hour_limit, 10000).await;
        
        // Reset to ensure clean state
        limiter.reset(&api_name).await.unwrap();
        
        // Make requests up to minute limit
        for _ in 0..minute_limit {
            let allowed = limiter.check_and_increment(&api_name).await.unwrap();
            assert!(allowed, "Should be allowed within minute limit");
        }
        
        // Next request should be denied due to minute limit
        let allowed = limiter.check_and_increment(&api_name).await.unwrap();
        assert!(!allowed, "Should be denied when minute limit reached");
        
        // Verify both counters are updated
        let usage = limiter.get_usage(&api_name).await.unwrap();
        assert_eq!(usage.minute_count, minute_limit);
        assert_eq!(usage.hour_count, minute_limit);
    }

    #[tokio::test]
    async fn test_reset_functionality() {
        let api_name = format!("test_reset_func_{}", rand::random::<u32>());
        let limit = 5;
        let limiter = create_test_limiter(&api_name, limit, 1000, 10000).await;
        
        // Reset to ensure clean state
        limiter.reset(&api_name).await.unwrap();
        
        // Make some requests
        for _ in 0..3 {
            limiter.check_and_increment(&api_name).await.unwrap();
        }
        
        let usage_before = limiter.get_usage(&api_name).await.unwrap();
        assert_eq!(usage_before.minute_count, 3);
        
        // Reset
        limiter.reset(&api_name).await.unwrap();
        
        // Verify all counters are 0
        let usage_after = limiter.get_usage(&api_name).await.unwrap();
        assert_eq!(usage_after.minute_count, 0);
        assert_eq!(usage_after.hour_count, 0);
        assert_eq!(usage_after.day_count, 0);
        
        // Should be able to make requests again
        let allowed = limiter.check_and_increment(&api_name).await.unwrap();
        assert!(allowed, "Should be allowed after reset");
    }

    #[tokio::test]
    async fn test_usage_statistics() {
        let api_name = format!("test_usage_stats_{}", rand::random::<u32>());
        let limit = 10;
        let limiter = create_test_limiter(&api_name, limit, limit * 10, limit * 100).await;
        
        // Reset to ensure clean state
        limiter.reset(&api_name).await.unwrap();
        
        // Make 7 requests (70%)
        for _ in 0..7 {
            limiter.check_and_increment(&api_name).await.unwrap();
        }
        
        let usage = limiter.get_usage(&api_name).await.unwrap();
        
        // Verify counts
        assert_eq!(usage.minute_count, 7);
        assert_eq!(usage.hour_count, 7);
        assert_eq!(usage.day_count, 7);
        
        // Verify limits
        assert_eq!(usage.minute_limit, limit);
        assert_eq!(usage.hour_limit, limit * 10);
        assert_eq!(usage.day_limit, limit * 100);
        
        // Verify not exceeded
        assert!(!usage.is_exceeded());
        
        // Verify percentage
        assert_eq!(usage.max_usage_percentage(), 70.0);
    }

    #[tokio::test]
    async fn test_zero_limit() {
        let api_name = format!("test_zero_limit_{}", rand::random::<u32>());
        let limiter = create_test_limiter(&api_name, 0, 1000, 10000).await;
        
        // Reset to ensure clean state
        limiter.reset(&api_name).await.unwrap();
        
        // With zero limit, first request should be denied
        let allowed = limiter.check_and_increment(&api_name).await.unwrap();
        assert!(!allowed, "Should be denied with zero limit");
    }

    #[tokio::test]
    async fn test_very_high_limit() {
        let api_name = format!("test_high_limit_{}", rand::random::<u32>());
        let limit = 1_000_000;
        let limiter = create_test_limiter(&api_name, limit, limit, limit).await;
        
        // Reset to ensure clean state
        limiter.reset(&api_name).await.unwrap();
        
        // Should be able to make many requests
        for _ in 0..100 {
            let allowed = limiter.check_and_increment(&api_name).await.unwrap();
            assert!(allowed, "Should be allowed with very high limit");
        }
        
        let usage = limiter.get_usage(&api_name).await.unwrap();
        assert_eq!(usage.minute_count, 100);
        assert!(!usage.is_exceeded());
    }
}

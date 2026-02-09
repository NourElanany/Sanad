//! Property-based tests for Rate Limiter
//! 
//! Feature: official-apis-integration
//! Property 13: Rate Limit Enforcement
//! 
//! **Validates: Requirements 9.2, 9.3, 9.5**

#[cfg(test)]
mod property_tests {
    use crate::api_clients::{RateLimiter, RateLimitConfig};
    use proptest::prelude::*;
    use std::collections::HashMap;
    use tokio::runtime::Runtime;

    // Helper to create a test rate limiter with Redis
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

        // Use a test Redis instance (assumes Redis is running on localhost:6379)
        // In CI/CD, this would use a containerized Redis instance
        RateLimiter::new("redis://127.0.0.1:6379", configs)
            .await
            .expect("Failed to create rate limiter")
    }

    // Property 13: Rate Limit Enforcement
    // For any API with configured rate limits, the number of requests sent within
    // any time window (minute, hour, day) should never exceed the configured limit
    // for that window.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn property_rate_limit_enforcement_minute(
            requests in 1u32..200,
            limit in 10u32..100,
        ) {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let api_name = format!("test_api_minute_{}", rand::random::<u32>());
                let limiter = create_test_limiter(&api_name, limit, 1000, 10000).await;
                
                // Reset counters to ensure clean state
                limiter.reset(&api_name).await.unwrap();
                
                let mut allowed_count = 0;
                let mut denied_count = 0;
                
                // Try to make 'requests' number of requests
                for _ in 0..requests {
                    let allowed = limiter.check_and_increment(&api_name).await.unwrap();
                    if allowed {
                        allowed_count += 1;
                    } else {
                        denied_count += 1;
                    }
                }
                
                // Property: allowed_count should never exceed the limit
                prop_assert!(
                    allowed_count <= limit,
                    "Allowed {} requests but limit was {}",
                    allowed_count,
                    limit
                );
                
                // Property: if we tried more requests than the limit, some should be denied
                if requests > limit {
                    prop_assert!(
                        denied_count > 0,
                        "Expected some requests to be denied when {} > {}",
                        requests,
                        limit
                    );
                }
                
                // Property: total requests = allowed + denied
                prop_assert_eq!(
                    allowed_count + denied_count,
                    requests,
                    "Total requests should equal allowed + denied"
                );
                
                // Verify usage statistics
                let usage = limiter.get_usage(&api_name).await.unwrap();
                prop_assert_eq!(
                    usage.minute_count,
                    allowed_count,
                    "Usage count should match allowed count"
                );
                
                Ok(())
            })?;
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn property_rate_limit_enforcement_multiple_windows(
            minute_limit in 5u32..20,
            hour_limit in 50u32..100,
            requests in 1u32..150,
        ) {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let api_name = format!("test_api_multi_{}", rand::random::<u32>());
                let limiter = create_test_limiter(&api_name, minute_limit, hour_limit, 10000).await;
                
                // Reset counters
                limiter.reset(&api_name).await.unwrap();
                
                let mut allowed_count = 0;
                
                // Make requests
                for _ in 0..requests {
                    if limiter.check_and_increment(&api_name).await.unwrap() {
                        allowed_count += 1;
                    }
                }
                
                // Property: allowed count should not exceed the smallest limit
                let min_limit = minute_limit.min(hour_limit);
                prop_assert!(
                    allowed_count <= min_limit,
                    "Allowed {} requests but smallest limit was {}",
                    allowed_count,
                    min_limit
                );
                
                // Verify usage
                let usage = limiter.get_usage(&api_name).await.unwrap();
                
                // Property: minute count should not exceed minute limit
                prop_assert!(
                    usage.minute_count <= minute_limit,
                    "Minute count {} exceeded limit {}",
                    usage.minute_count,
                    minute_limit
                );
                
                // Property: hour count should not exceed hour limit
                prop_assert!(
                    usage.hour_count <= hour_limit,
                    "Hour count {} exceeded limit {}",
                    usage.hour_count,
                    hour_limit
                );
                
                Ok(())
            })?;
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]
        
        #[test]
        fn property_rate_limit_idempotent_check(
            limit in 10u32..50,
            check_count in 1u32..20,
        ) {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let api_name = format!("test_api_idempotent_{}", rand::random::<u32>());
                let limiter = create_test_limiter(&api_name, limit, 1000, 10000).await;
                
                // Reset counters
                limiter.reset(&api_name).await.unwrap();
                
                // Property: Multiple checks without increment should not change the counter
                for _ in 0..check_count {
                    let allowed = limiter.check(&api_name).await.unwrap();
                    prop_assert!(allowed, "First check should always be allowed");
                }
                
                let usage_before = limiter.get_usage(&api_name).await.unwrap();
                prop_assert_eq!(
                    usage_before.minute_count,
                    0,
                    "Counter should remain 0 after checks without increment"
                );
                
                // Now increment once
                limiter.increment(&api_name).await.unwrap();
                
                let usage_after = limiter.get_usage(&api_name).await.unwrap();
                prop_assert_eq!(
                    usage_after.minute_count,
                    1,
                    "Counter should be 1 after single increment"
                );
                
                Ok(())
            })?;
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]
        
        #[test]
        fn property_rate_limit_usage_percentage(
            limit in 10u32..100,
            requests in 1u32..150,
        ) {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let api_name = format!("test_api_percentage_{}", rand::random::<u32>());
                let limiter = create_test_limiter(&api_name, limit, limit * 10, limit * 100).await;
                
                // Reset counters
                limiter.reset(&api_name).await.unwrap();
                
                // Make requests
                let mut allowed_count = 0;
                for _ in 0..requests {
                    if limiter.check_and_increment(&api_name).await.unwrap() {
                        allowed_count += 1;
                    }
                }
                
                let usage = limiter.get_usage(&api_name).await.unwrap();
                let percentage = usage.max_usage_percentage();
                
                // Property: percentage should be between 0 and 100
                prop_assert!(
                    percentage >= 0.0 && percentage <= 100.0,
                    "Usage percentage {} should be between 0 and 100",
                    percentage
                );
                
                // Property: if we hit the limit, percentage should be 100
                if allowed_count >= limit {
                    prop_assert!(
                        percentage >= 100.0,
                        "Usage percentage should be 100 when limit is reached"
                    );
                }
                
                // Property: if we made no requests, percentage should be 0
                if requests == 0 || allowed_count == 0 {
                    prop_assert_eq!(
                        percentage,
                        0.0,
                        "Usage percentage should be 0 when no requests made"
                    );
                }
                
                Ok(())
            })?;
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]
        
        #[test]
        fn property_rate_limit_reset_clears_counters(
            limit in 10u32..50,
            initial_requests in 1u32..30,
        ) {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let api_name = format!("test_api_reset_{}", rand::random::<u32>());
                let limiter = create_test_limiter(&api_name, limit, 1000, 10000).await;
                
                // Reset to ensure clean state
                limiter.reset(&api_name).await.unwrap();
                
                // Make some requests
                for _ in 0..initial_requests.min(limit) {
                    limiter.check_and_increment(&api_name).await.unwrap();
                }
                
                let usage_before = limiter.get_usage(&api_name).await.unwrap();
                prop_assert!(
                    usage_before.minute_count > 0,
                    "Should have some requests before reset"
                );
                
                // Reset the limiter
                limiter.reset(&api_name).await.unwrap();
                
                // Property: After reset, all counters should be 0
                let usage_after = limiter.get_usage(&api_name).await.unwrap();
                prop_assert_eq!(
                    usage_after.minute_count,
                    0,
                    "Minute count should be 0 after reset"
                );
                prop_assert_eq!(
                    usage_after.hour_count,
                    0,
                    "Hour count should be 0 after reset"
                );
                prop_assert_eq!(
                    usage_after.day_count,
                    0,
                    "Day count should be 0 after reset"
                );
                
                // Property: After reset, we should be able to make requests again
                let allowed = limiter.check(&api_name).await.unwrap();
                prop_assert!(allowed, "Should be allowed to make requests after reset");
                
                Ok(())
            })?;
        }
    }
}

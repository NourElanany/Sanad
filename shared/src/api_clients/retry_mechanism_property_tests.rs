//! Property-based tests for retry mechanism
//! 
//! Feature: official-apis-integration
//! Property 18: Retry with Exponential Backoff
//! Validates: Requirements 11.2

#[cfg(test)]
mod property_tests {
    use crate::api_clients::error::ApiError;
    use crate::api_clients::retry_mechanism::{RetryMechanism, RetryStrategy};
    use proptest::prelude::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, Instant};
    
    proptest! {
        #![proptest_config(ProptestConfig {cases: 100, .. ProptestConfig::default()})]
        
        /// Property 18: Retry with Exponential Backoff
        /// 
        /// For any network error, the Retry_Mechanism should attempt up to 3 retries, 
        /// and the delay between retries should increase exponentially (e.g., 1s, 2s, 4s).
        /// 
        /// **Validates: Requirements 11.2**
        #[test]
        fn property_retry_with_exponential_backoff(
            max_attempts in 1u32..=5,
            initial_delay_ms in 100u64..=1000,
            multiplier in 1.5f64..=3.0,
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let initial_delay = Duration::from_millis(initial_delay_ms);
                let max_delay = Duration::from_secs(10);
                
                let strategy = RetryStrategy::new(max_attempts, initial_delay, max_delay, multiplier);
                let retry = RetryMechanism::with_strategy(strategy.clone());
                
                let counter = Arc::new(AtomicU32::new(0));
                let counter_clone = counter.clone();
                let timestamps = Arc::new(tokio::sync::Mutex::new(Vec::new()));
                let timestamps_clone = timestamps.clone();
                
                // Operation that always fails with network error
                let result = retry.execute(|| {
                    let counter = counter_clone.clone();
                    let timestamps = timestamps_clone.clone();
                    async move {
                        let now = Instant::now();
                        timestamps.lock().await.push(now);
                        counter.fetch_add(1, Ordering::SeqCst);
                        Err::<(), _>(ApiError::Network("Test failure".to_string()))
                    }
                }).await;
                
                // Should fail after all attempts
                prop_assert!(result.is_err());
                
                // Should have attempted exactly max_attempts times
                prop_assert_eq!(counter.load(Ordering::SeqCst), max_attempts);
                
                // Verify exponential backoff delays
                let timestamps = timestamps.lock().await;
                if timestamps.len() > 1 {
                    for i in 1..timestamps.len() {
                        let delay = timestamps[i].duration_since(timestamps[i-1]);
                        let expected_delay = strategy.calculate_delay(i as u32);
                        
                        // Allow some tolerance for timing (±200ms)
                        let tolerance = Duration::from_millis(200);
                        let lower_bound = expected_delay.saturating_sub(tolerance);
                        let upper_bound = expected_delay + tolerance;
                        
                        prop_assert!(
                            delay >= lower_bound && delay <= upper_bound,
                            "Delay {} is not within expected range [{:?}, {:?}] (expected {:?})",
                            i, lower_bound, upper_bound, expected_delay
                        );
                    }
                }
                
                Ok(())
            })?;
        }
        
        /// Property: Retry attempts are exactly max_attempts for retryable errors
        #[test]
        fn property_retry_attempts_count(max_attempts in 1u32..=5) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let strategy = RetryStrategy {
                    max_attempts,
                    initial_delay: Duration::from_millis(10),
                    max_delay: Duration::from_secs(1),
                    multiplier: 2.0,
                };
                let retry = RetryMechanism::with_strategy(strategy);
                
                let counter = Arc::new(AtomicU32::new(0));
                let counter_clone = counter.clone();
                
                let _ = retry.execute(|| {
                    let counter = counter_clone.clone();
                    async move {
                        counter.fetch_add(1, Ordering::SeqCst);
                        Err::<(), _>(ApiError::Network("Test".to_string()))
                    }
                }).await;
                
                prop_assert_eq!(counter.load(Ordering::SeqCst), max_attempts);
                Ok(())
            })?;
        }
        
        /// Property: Non-retryable errors fail immediately without retries
        #[test]
        fn property_non_retryable_no_retry(
            error in prop_oneof![
                any::<String>().prop_map(ApiError::Authentication),
                any::<String>().prop_map(ApiError::Validation),
                any::<String>().prop_map(ApiError::RateLimitExceeded),
            ]
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let retry = RetryMechanism::new();
                let counter = Arc::new(AtomicU32::new(0));
                let counter_clone = counter.clone();
                let error_clone = error.clone();
                
                let result = retry.execute(|| {
                    let counter = counter_clone.clone();
                    let error = error_clone.clone();
                    async move {
                        counter.fetch_add(1, Ordering::SeqCst);
                        Err::<(), _>(error)
                    }
                }).await;
                
                prop_assert!(result.is_err());
                // Should only attempt once (no retries)
                prop_assert_eq!(counter.load(Ordering::SeqCst), 1);
                Ok(())
            })?;
        }
        
        /// Property: Success on any attempt stops retrying
        #[test]
        fn property_success_stops_retry(
            success_on_attempt in 1u32..=3,
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let retry = RetryMechanism::new();
                let counter = Arc::new(AtomicU32::new(0));
                let counter_clone = counter.clone();
                
                let result = retry.execute(|| {
                    let counter = counter_clone.clone();
                    async move {
                        let count = counter.fetch_add(1, Ordering::SeqCst) + 1;
                        if count >= success_on_attempt {
                            Ok(42)
                        } else {
                            Err(ApiError::Network("Temporary failure".to_string()))
                        }
                    }
                }).await;
                
                prop_assert!(result.is_ok());
                prop_assert_eq!(result.unwrap(), 42);
                // Should stop after success
                prop_assert_eq!(counter.load(Ordering::SeqCst), success_on_attempt);
                Ok(())
            })?;
        }
        
        /// Property: Delay calculation is monotonically increasing (up to max)
        #[test]
        fn property_delay_monotonic_increasing(
            initial_delay_ms in 100u64..=1000,
            multiplier in 1.5f64..=3.0,
            max_delay_secs in 5u64..=20,
        ) {
            let strategy = RetryStrategy {
                max_attempts: 10,
                initial_delay: Duration::from_millis(initial_delay_ms),
                max_delay: Duration::from_secs(max_delay_secs),
                multiplier,
            };
            
            let delays = strategy.get_delays();
            
            // First delay should be 0
            prop_assert_eq!(delays[0], Duration::from_secs(0));
            
            // Subsequent delays should be increasing or capped at max
            for i in 1..delays.len() {
                prop_assert!(
                    delays[i] >= delays[i-1],
                    "Delay at attempt {} ({:?}) is less than previous ({:?})",
                    i, delays[i], delays[i-1]
                );
                
                // Should never exceed max_delay
                prop_assert!(
                    delays[i] <= strategy.max_delay,
                    "Delay {:?} exceeds max_delay {:?}",
                    delays[i], strategy.max_delay
                );
            }
        }
        
        /// Property: Exponential growth follows formula (until max)
        #[test]
        fn property_exponential_formula(
            initial_delay_ms in 100u64..=500,
            multiplier in 2.0f64..=3.0,
        ) {
            let strategy = RetryStrategy {
                max_attempts: 5,
                initial_delay: Duration::from_millis(initial_delay_ms),
                max_delay: Duration::from_secs(100), // High enough to not cap
                multiplier,
            };
            
            for attempt in 1..strategy.max_attempts {
                let delay = strategy.calculate_delay(attempt);
                let expected_secs = (initial_delay_ms as f64 / 1000.0) * multiplier.powi((attempt - 1) as i32);
                let expected = Duration::from_secs_f64(expected_secs);
                
                // Allow small floating point tolerance
                let diff = if delay > expected {
                    delay - expected
                } else {
                    expected - delay
                };
                
                prop_assert!(
                    diff < Duration::from_millis(10),
                    "Delay {:?} doesn't match expected {:?} for attempt {}",
                    delay, expected, attempt
                );
            }
        }
        
        /// Property: Max delay cap is respected
        #[test]
        fn property_max_delay_cap(
            initial_delay_ms in 100u64..=1000,
            max_delay_secs in 1u64..=5,
            multiplier in 2.0f64..=4.0,
        ) {
            let strategy = RetryStrategy {
                max_attempts: 20,
                initial_delay: Duration::from_millis(initial_delay_ms),
                max_delay: Duration::from_secs(max_delay_secs),
                multiplier,
            };
            
            let delays = strategy.get_delays();
            
            for (i, delay) in delays.iter().enumerate() {
                prop_assert!(
                    *delay <= strategy.max_delay,
                    "Delay at attempt {} ({:?}) exceeds max_delay ({:?})",
                    i, delay, strategy.max_delay
                );
            }
        }
    }
}

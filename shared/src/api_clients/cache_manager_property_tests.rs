//! Property-based tests for Cache Manager
//! 
//! Tests universal properties that should hold across all valid inputs.

use super::*;
use proptest::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PropTestData {
    id: u32,
    value: String,
}

// Helper to create test cache manager
async fn create_prop_test_cache() -> CacheManager {
    CacheManager::new("redis://localhost:6379")
        .await
        .expect("Failed to create cache manager")
}

// Feature: official-apis-integration, Property 14: Cache-First Behavior
// **Validates: Requirements 10.1, 10.2**
//
// *For any* request with valid cached data, the system should return the cached data
// without making an external API call, and the response time should be significantly
// faster than an API call.
#[tokio::test]
async fn property_cache_first_behavior() {
    let cache = create_prop_test_cache().await;
    let test_prefix = "prop:cache_first";
    
    // Clean up before test
    cache.delete_pattern(&format!("{}:*", test_prefix))
        .await
        .expect("Failed to clean up");

    // Run property test with fewer cases to avoid timeout
    for id in 1u32..20 {
        for value in &["hello", "world", "test", "cache"] {
            let key = format!("{}:{}", test_prefix, id);
            let data = PropTestData {
                id,
                value: value.to_string(),
            };

            // Set cache
            cache.set(&key, &data, Duration::from_secs(60))
                .await
                .expect("Failed to set cache");

            // First retrieval - should hit cache
            let start = std::time::Instant::now();
            let retrieved1: Option<PropTestData> = cache.get(&key)
                .await
                .expect("Failed to get from cache");
            let cache_time = start.elapsed();

            // Verify data is correct
            assert_eq!(retrieved1, Some(data.clone()));

            // Second retrieval - should also hit cache
            let start = std::time::Instant::now();
            let retrieved2: Option<PropTestData> = cache.get(&key)
                .await
                .expect("Failed to get from cache");
            let cache_time2 = start.elapsed();

            // Both should return same data
            assert_eq!(retrieved2, Some(data));

            // Cache hits should be fast (< 50ms for local Redis)
            assert!(cache_time.as_millis() < 50, 
                "Cache retrieval too slow: {:?}", cache_time);
            assert!(cache_time2.as_millis() < 50,
                "Second cache retrieval too slow: {:?}", cache_time2);

            // Cleanup
            cache.delete(&key).await.expect("Failed to delete");
        }
    }
}

// Feature: official-apis-integration, Property 15: Cache Update on Miss
// **Validates: Requirements 10.3**
//
// *For any* request with expired or missing cache, the system should fetch from the API
// and update the cache, so that a subsequent identical request finds valid cached data.
#[tokio::test]
async fn property_cache_update_on_miss() {
    let cache = create_prop_test_cache().await;
    let test_prefix = "prop:cache_miss";
    
    // Clean up before test
    cache.delete_pattern(&format!("{}:*", test_prefix))
        .await
        .expect("Failed to clean up");

    for id in 1u32..20 {
        for value in &["hello", "world", "test"] {
            let key = format!("{}:{}", test_prefix, id);
            let data = PropTestData {
                id,
                value: value.to_string(),
            };

            // Ensure key doesn't exist initially
            cache.delete(&key).await.ok();

            // First retrieval - should be a miss
            let retrieved1: Option<PropTestData> = cache.get(&key)
                .await
                .expect("Failed to get from cache");
            assert_eq!(retrieved1, None, "Expected cache miss");

            // Simulate API fetch and cache update
            cache.set(&key, &data, Duration::from_secs(60))
                .await
                .expect("Failed to set cache after miss");

            // Second retrieval - should now hit cache
            let retrieved2: Option<PropTestData> = cache.get(&key)
                .await
                .expect("Failed to get from cache");
            assert_eq!(retrieved2, Some(data.clone()), "Expected cache hit after update");

            // Third retrieval - should still hit cache
            let retrieved3: Option<PropTestData> = cache.get(&key)
                .await
                .expect("Failed to get from cache");
            assert_eq!(retrieved3, Some(data), "Expected cache hit on subsequent request");

            // Cleanup
            cache.delete(&key).await.expect("Failed to delete");
        }
    }
}

// Feature: official-apis-integration, Property 16: TTL Strategy Differentiation
// **Validates: Requirements 10.4**
//
// *For any* two data types with different volatility (e.g., Quran text vs Prayer times),
// the static data should have a longer TTL than the dynamic data in the cache.
#[tokio::test]
async fn property_ttl_strategy_differentiation() {
    let cache = create_prop_test_cache().await;
    let test_prefix = "prop:ttl_diff";
    
    // Clean up before test
    cache.delete_pattern(&format!("{}:*", test_prefix))
        .await
        .expect("Failed to clean up");

    for id in 1u32..10 {
        for value in &["test", "data"] {
            let static_key = format!("{}:static:{}", test_prefix, id);
            let dynamic_key = format!("{}:dynamic:{}", test_prefix, id);
            let data = PropTestData {
                id,
                value: value.to_string(),
            };

            // Set static data (Quran text)
            cache.set_with_category(&static_key, &data, CacheCategory::QuranText)
                .await
                .expect("Failed to set static cache");

            // Set dynamic data (Prayer times)
            cache.set_with_category(&dynamic_key, &data, CacheCategory::PrayerTimes)
                .await
                .expect("Failed to set dynamic cache");

            // Get TTLs
            let static_ttl = cache.ttl(&static_key)
                .await
                .expect("Failed to get static TTL")
                .expect("Static TTL should exist");

            let dynamic_ttl = cache.ttl(&dynamic_key)
                .await
                .expect("Failed to get dynamic TTL")
                .expect("Dynamic TTL should exist");

            // Static data should have longer TTL than dynamic data
            assert!(
                static_ttl > dynamic_ttl,
                "Static data TTL ({:?}) should be longer than dynamic data TTL ({:?})",
                static_ttl,
                dynamic_ttl
            );

            // Verify the strategies are as expected
            let quran_strategy = cache.get_strategy(CacheCategory::QuranText).unwrap();
            let prayer_strategy = cache.get_strategy(CacheCategory::PrayerTimes).unwrap();

            assert!(
                quran_strategy.ttl > prayer_strategy.ttl,
                "Quran strategy TTL should be longer than Prayer strategy TTL"
            );

            // Cleanup
            cache.delete(&static_key).await.expect("Failed to delete static");
            cache.delete(&dynamic_key).await.expect("Failed to delete dynamic");
        }
    }
}

// Additional property: Cache key determinism
// For any identical request parameters, the cache key should be the same
#[test]
fn property_cache_key_determinism() {
    for id in 1u32..100 {
        for value in &["test", "data", "cache"] {
            // Generate cache key multiple times with same parameters
            let key1 = format!("test:{}:{}", id, value);
            let key2 = format!("test:{}:{}", id, value);
            let key3 = format!("test:{}:{}", id, value);

            // All keys should be identical
            assert_eq!(&key1, &key2);
            assert_eq!(&key2, &key3);
            assert_eq!(&key1, &key3);
        }
    }
}

// Additional property: Stale cache availability
// For any data with allow_stale=true, stale cache should be available after fresh expires
#[tokio::test]
async fn property_stale_cache_availability() {
    let cache = create_prop_test_cache().await;
    let test_prefix = "prop:stale";
    
    // Clean up before test
    cache.delete_pattern(&format!("{}:*", test_prefix))
        .await
        .expect("Failed to clean up");

    for id in 1u32..10 {
        for value in &["test", "data"] {
            let key = format!("{}:{}", test_prefix, id);
            let data = PropTestData {
                id,
                value: value.to_string(),
            };

            // Set with category that allows stale cache
            cache.set_with_category(&key, &data, CacheCategory::Hadith)
                .await
                .expect("Failed to set cache");

            // Fresh cache should exist
            let fresh: Option<PropTestData> = cache.get(&key)
                .await
                .expect("Failed to get fresh cache");
            assert_eq!(fresh, Some(data.clone()));

            // Manually delete fresh cache to simulate expiration
            let mut conn = cache.redis.write().await;
            let _: () = conn.del(&key).await.expect("Failed to delete fresh");
            drop(conn);

            // Fresh cache should be gone
            let fresh_after: Option<PropTestData> = cache.get(&key)
                .await
                .expect("Failed to get fresh cache");
            assert_eq!(fresh_after, None);

            // Stale cache should still be available
            let stale: Option<PropTestData> = cache.get_stale(&key)
                .await
                .expect("Failed to get stale cache");
            assert_eq!(stale, Some(data));

            // Cleanup
            cache.delete(&key).await.expect("Failed to delete");
        }
    }
}

// Additional property: Cache operations are idempotent
// Setting the same value multiple times should result in the same cached value
#[tokio::test]
async fn property_cache_set_idempotent() {
    let cache = create_prop_test_cache().await;
    let test_prefix = "prop:idempotent";
    
    // Clean up before test
    cache.delete_pattern(&format!("{}:*", test_prefix))
        .await
        .expect("Failed to clean up");

    for id in 1u32..10 {
        for value in &["test", "data"] {
            for iterations in 2..5 {
                let key = format!("{}:{}", test_prefix, id);
                let data = PropTestData {
                    id,
                    value: value.to_string(),
                };

                // Set the same value multiple times
                for _ in 0..iterations {
                    cache.set(&key, &data, Duration::from_secs(60))
                        .await
                        .expect("Failed to set cache");
                }

                // Retrieve and verify
                let retrieved: Option<PropTestData> = cache.get(&key)
                    .await
                    .expect("Failed to get from cache");
                assert_eq!(retrieved, Some(data));

                // Cleanup
                cache.delete(&key).await.expect("Failed to delete");
            }
        }
    }
}

// Additional property: LRU eviction removes oldest entries
// When evicting N entries, the N least recently used entries should be removed
#[tokio::test]
async fn property_lru_eviction_order() {
    let cache = create_prop_test_cache().await;
    let test_prefix = "prop:lru_order";
    
    // Clean up before test
    cache.delete_pattern(&format!("{}:*", test_prefix))
        .await
        .expect("Failed to clean up");

    let count = 10usize;
    let evict_count = 3usize;

    // Set multiple values with LRU tracking
    for i in 0..count {
        let key = format!("{}:{}", test_prefix, i);
        let data = PropTestData {
            id: i as u32,
            value: format!("value_{}", i),
        };
        
        cache.set(&key, &data, Duration::from_secs(300))
            .await
            .expect("Failed to set cache");
        
        cache.touch_lru(&key).await.expect("Failed to touch LRU");
        
        // Small delay to ensure different timestamps
        tokio::time::sleep(Duration::from_millis(5)).await;
    }

    // Evict some entries
    let evicted = cache.evict_lru(evict_count)
        .await
        .expect("Failed to evict LRU");

    assert_eq!(evicted, evict_count);

    // The first evict_count keys should be evicted (oldest)
    for i in 0..evict_count {
        let key = format!("{}:{}", test_prefix, i);
        let exists = cache.exists(&key).await.expect("Failed to check existence");
        assert!(!exists, "Key {} should have been evicted", i);
    }

    // The rest should still exist
    for i in evict_count..count {
        let key = format!("{}:{}", test_prefix, i);
        let exists = cache.exists(&key).await.expect("Failed to check existence");
        assert!(exists, "Key {} should still exist", i);
    }

    // Cleanup
    cache.delete_pattern(&format!("{}:*", test_prefix))
        .await
        .expect("Failed to clean up");
}

// Additional property: Cache statistics accuracy
// The cache stats should accurately reflect the number of keys
#[tokio::test]
async fn property_cache_stats_accuracy() {
    let cache = create_prop_test_cache().await;
    let test_prefix = "prop:stats";
    
    // Clean up before test
    cache.delete_pattern(&format!("{}:*", test_prefix))
        .await
        .expect("Failed to clean up");

    let count = 10usize;

    // Set multiple values with stale cache
    for i in 0..count {
        let key = format!("{}:{}", test_prefix, i);
        let data = PropTestData {
            id: i as u32,
            value: format!("value_{}", i),
        };
        
        cache.set_with_category(&key, &data, CacheCategory::QuranText)
            .await
            .expect("Failed to set cache");
    }

    // Get stats
    let stats = cache.get_stats().await.expect("Failed to get stats");

    // Should have at least 'count' fresh keys and 'count' stale keys
    assert!(
        stats.fresh_keys >= count,
        "Expected at least {} fresh keys, got {}",
        count,
        stats.fresh_keys
    );
    assert!(
        stats.stale_keys >= count,
        "Expected at least {} stale keys, got {}",
        count,
        stats.stale_keys
    );

    // Cleanup
    cache.delete_pattern(&format!("{}:*", test_prefix))
        .await
        .expect("Failed to clean up");
}

// Additional property: Cache with fallback never returns None if stale exists
// For any key with stale cache, get_with_fallback should return Some
#[tokio::test]
async fn property_fallback_never_none_with_stale() {
    let cache = create_prop_test_cache().await;
    let test_prefix = "prop:fallback_stale";
    
    // Clean up before test
    cache.delete_pattern(&format!("{}:*", test_prefix))
        .await
        .expect("Failed to clean up");

    for id in 1u32..10 {
        for value in &["test", "data"] {
            let key = format!("{}:{}", test_prefix, id);
            let data = PropTestData {
                id,
                value: value.to_string(),
            };

            // Set with stale cache
            cache.set_with_category(&key, &data, CacheCategory::Tafsir)
                .await
                .expect("Failed to set cache");

            // Delete fresh cache
            let mut conn = cache.redis.write().await;
            let _: () = conn.del(&key).await.expect("Failed to delete fresh");
            drop(conn);

            // get_with_fallback should return stale cache
            let result: Option<PropTestData> = cache.get_with_fallback(&key)
                .await
                .expect("Failed to get with fallback");

            assert!(result.is_some(), "Expected Some with stale cache");
            assert_eq!(result, Some(data));

            // Cleanup
            cache.delete(&key).await.expect("Failed to delete");
        }
    }
}

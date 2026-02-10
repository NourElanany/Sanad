//! Unit tests for Cache Manager
//! 
//! Tests cache hit/miss scenarios, TTL expiration, stale cache retrieval,
//! and LRU eviction.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestData {
    id: u32,
    name: String,
    value: f64,
}

impl TestData {
    fn new(id: u32, name: &str, value: f64) -> Self {
        Self {
            id,
            name: name.to_string(),
            value,
        }
    }
}

// Helper function to create a test cache manager
async fn create_test_cache_manager() -> CacheManager {
    // Use a test Redis instance (assumes Redis is running on localhost:6379)
    // In CI/CD, this should use a test container
    CacheManager::new("redis://localhost:6379")
        .await
        .expect("Failed to create cache manager")
}

#[tokio::test]
async fn test_cache_set_and_get() {
    let cache = create_test_cache_manager().await;
    let key = "test:cache:set_get";
    let data = TestData::new(1, "test", 42.0);

    // Set value
    cache.set(key, &data, Duration::from_secs(60))
        .await
        .expect("Failed to set cache");

    // Get value
    let retrieved: Option<TestData> = cache.get(key)
        .await
        .expect("Failed to get from cache");

    assert_eq!(retrieved, Some(data));

    // Cleanup
    cache.delete(key).await.expect("Failed to delete");
}

#[tokio::test]
async fn test_cache_miss() {
    let cache = create_test_cache_manager().await;
    let key = "test:cache:miss:nonexistent";

    // Try to get non-existent key
    let retrieved: Option<TestData> = cache.get(key)
        .await
        .expect("Failed to get from cache");

    assert_eq!(retrieved, None);
}

#[tokio::test]
async fn test_cache_with_category() {
    let cache = create_test_cache_manager().await;
    let key = "test:cache:category:quran";
    let data = TestData::new(2, "quran_text", 100.0);

    // Set with category (QuranText has 30-day TTL)
    cache.set_with_category(key, &data, CacheCategory::QuranText)
        .await
        .expect("Failed to set with category");

    // Get value
    let retrieved: Option<TestData> = cache.get(key)
        .await
        .expect("Failed to get from cache");

    assert_eq!(retrieved, Some(data.clone()));

    // Check that stale cache was also set
    let stale_key = format!("{}:stale", key);
    let stale_retrieved: Option<TestData> = cache.get(&stale_key)
        .await
        .expect("Failed to get stale cache");

    assert_eq!(stale_retrieved, Some(data));

    // Cleanup
    cache.delete(key).await.expect("Failed to delete");
}

#[tokio::test]
async fn test_cache_ttl_expiration() {
    let cache = create_test_cache_manager().await;
    let key = "test:cache:ttl:expiration";
    let data = TestData::new(3, "expires_soon", 50.0);

    // Set with very short TTL (2 seconds)
    cache.set(key, &data, Duration::from_secs(2))
        .await
        .expect("Failed to set cache");

    // Should exist immediately
    let retrieved: Option<TestData> = cache.get(key)
        .await
        .expect("Failed to get from cache");
    assert_eq!(retrieved, Some(data));

    // Wait for expiration
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Should be expired now
    let retrieved: Option<TestData> = cache.get(key)
        .await
        .expect("Failed to get from cache");
    assert_eq!(retrieved, None);
}

#[tokio::test]
async fn test_stale_cache_retrieval() {
    let cache = create_test_cache_manager().await;
    let key = "test:cache:stale:retrieval";
    let data = TestData::new(4, "stale_data", 75.0);

    // Set with category that allows stale cache
    cache.set_with_category(key, &data, CacheCategory::PrayerTimes)
        .await
        .expect("Failed to set with category");

    // Manually delete the fresh cache to simulate expiration
    let mut conn = cache.redis.write().await;
    let _: () = conn.del(key).await.expect("Failed to delete fresh cache");
    drop(conn);

    // Fresh cache should be gone
    let fresh: Option<TestData> = cache.get(key)
        .await
        .expect("Failed to get from cache");
    assert_eq!(fresh, None);

    // But stale cache should still be available
    let stale: Option<TestData> = cache.get_stale(key)
        .await
        .expect("Failed to get stale cache");
    assert_eq!(stale, Some(data));

    // Cleanup
    cache.delete(key).await.expect("Failed to delete");
}

#[tokio::test]
async fn test_get_with_fallback() {
    let cache = create_test_cache_manager().await;
    let key = "test:cache:fallback";
    let data = TestData::new(5, "fallback_data", 88.0);

    // Set with category
    cache.set_with_category(key, &data, CacheCategory::Hadith)
        .await
        .expect("Failed to set with category");

    // Get with fallback should return fresh cache
    let retrieved: Option<TestData> = cache.get_with_fallback(key)
        .await
        .expect("Failed to get with fallback");
    assert_eq!(retrieved, Some(data.clone()));

    // Delete fresh cache
    let mut conn = cache.redis.write().await;
    let _: () = conn.del(key).await.expect("Failed to delete fresh cache");
    drop(conn);

    // Get with fallback should now return stale cache
    let retrieved: Option<TestData> = cache.get_with_fallback(key)
        .await
        .expect("Failed to get with fallback");
    assert_eq!(retrieved, Some(data));

    // Cleanup
    cache.delete(key).await.expect("Failed to delete");
}

#[tokio::test]
async fn test_cache_delete() {
    let cache = create_test_cache_manager().await;
    let key = "test:cache:delete";
    let data = TestData::new(6, "to_delete", 99.0);

    // Set value
    cache.set_with_category(key, &data, CacheCategory::Tafsir)
        .await
        .expect("Failed to set cache");

    // Verify it exists
    assert!(cache.exists(key).await.expect("Failed to check existence"));

    // Delete
    cache.delete(key).await.expect("Failed to delete");

    // Verify it's gone
    assert!(!cache.exists(key).await.expect("Failed to check existence"));

    // Verify stale is also gone
    let stale_key = format!("{}:stale", key);
    assert!(!cache.exists(&stale_key).await.expect("Failed to check stale existence"));
}

#[tokio::test]
async fn test_cache_delete_pattern() {
    let cache = create_test_cache_manager().await;
    let prefix = "test:cache:pattern";

    // Set multiple values with same prefix
    for i in 1..=5 {
        let key = format!("{}:{}", prefix, i);
        let data = TestData::new(i, &format!("pattern_{}", i), i as f64);
        cache.set(&key, &data, Duration::from_secs(60))
            .await
            .expect("Failed to set cache");
    }

    // Delete all with pattern
    let pattern = format!("{}:*", prefix);
    let deleted = cache.delete_pattern(&pattern)
        .await
        .expect("Failed to delete pattern");

    assert_eq!(deleted, 5);

    // Verify all are gone
    for i in 1..=5 {
        let key = format!("{}:{}", prefix, i);
        assert!(!cache.exists(&key).await.expect("Failed to check existence"));
    }
}

#[tokio::test]
async fn test_cache_exists() {
    let cache = create_test_cache_manager().await;
    let key = "test:cache:exists";
    let data = TestData::new(7, "exists_test", 123.0);

    // Should not exist initially
    assert!(!cache.exists(key).await.expect("Failed to check existence"));

    // Set value
    cache.set(key, &data, Duration::from_secs(60))
        .await
        .expect("Failed to set cache");

    // Should exist now
    assert!(cache.exists(key).await.expect("Failed to check existence"));

    // Cleanup
    cache.delete(key).await.expect("Failed to delete");
}

#[tokio::test]
async fn test_cache_ttl() {
    let cache = create_test_cache_manager().await;
    let key = "test:cache:ttl";
    let data = TestData::new(8, "ttl_test", 456.0);
    let ttl = Duration::from_secs(300); // 5 minutes

    // Set with TTL
    cache.set(key, &data, ttl)
        .await
        .expect("Failed to set cache");

    // Get TTL
    let remaining_ttl = cache.ttl(key)
        .await
        .expect("Failed to get TTL");

    assert!(remaining_ttl.is_some());
    let remaining = remaining_ttl.unwrap();
    
    // Should be close to 300 seconds (allow some variance)
    assert!(remaining.as_secs() >= 295 && remaining.as_secs() <= 300);

    // Cleanup
    cache.delete(key).await.expect("Failed to delete");
}

#[tokio::test]
async fn test_cache_stats() {
    let cache = create_test_cache_manager().await;
    let prefix = "test:cache:stats";

    // Clean up any existing test keys
    cache.delete_pattern(&format!("{}:*", prefix))
        .await
        .expect("Failed to clean up");

    // Set some values with different categories
    for i in 1..=3 {
        let key = format!("{}:{}", prefix, i);
        let data = TestData::new(i, &format!("stats_{}", i), i as f64);
        cache.set_with_category(&key, &data, CacheCategory::QuranText)
            .await
            .expect("Failed to set cache");
    }

    // Get stats
    let stats = cache.get_stats().await.expect("Failed to get stats");

    // Should have at least 3 fresh keys and 3 stale keys
    assert!(stats.fresh_keys >= 3);
    assert!(stats.stale_keys >= 3);

    // Cleanup
    cache.delete_pattern(&format!("{}:*", prefix))
        .await
        .expect("Failed to clean up");
}

#[tokio::test]
async fn test_different_ttl_strategies() {
    let cache = create_test_cache_manager().await;

    // Test that different categories have different TTLs
    let quran_strategy = cache.get_strategy(CacheCategory::QuranText).unwrap();
    let prayer_strategy = cache.get_strategy(CacheCategory::PrayerTimes).unwrap();
    let ai_strategy = cache.get_strategy(CacheCategory::AiResponse).unwrap();

    // QuranText should have longer TTL than PrayerTimes
    assert!(quran_strategy.ttl > prayer_strategy.ttl);

    // PrayerTimes should have longer TTL than AiResponse
    assert!(prayer_strategy.ttl > ai_strategy.ttl);

    // QuranText and Hadith should allow stale cache
    assert!(quran_strategy.allow_stale);

    // AiResponse should not allow stale cache
    assert!(!ai_strategy.allow_stale);
}

#[tokio::test]
async fn test_lru_eviction() {
    let cache = create_test_cache_manager().await;
    let prefix = "test:cache:lru";

    // Clean up any existing test keys
    cache.delete_pattern(&format!("{}:*", prefix))
        .await
        .expect("Failed to clean up");

    // Set multiple values
    for i in 1..=10 {
        let key = format!("{}:{}", prefix, i);
        let data = TestData::new(i, &format!("lru_{}", i), i as f64);
        cache.set(&key, &data, Duration::from_secs(300))
            .await
            .expect("Failed to set cache");
        
        // Update LRU tracking
        cache.touch_lru(&key).await.expect("Failed to touch LRU");
        
        // Small delay to ensure different timestamps
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    // Evict 3 least recently used entries
    let evicted = cache.evict_lru(3)
        .await
        .expect("Failed to evict LRU");

    assert_eq!(evicted, 3);

    // The first 3 keys should be evicted
    for i in 1..=3 {
        let key = format!("{}:{}", prefix, i);
        assert!(!cache.exists(&key).await.expect("Failed to check existence"));
    }

    // The rest should still exist
    for i in 4..=10 {
        let key = format!("{}:{}", prefix, i);
        assert!(cache.exists(&key).await.expect("Failed to check existence"));
    }

    // Cleanup
    cache.delete_pattern(&format!("{}:*", prefix))
        .await
        .expect("Failed to clean up");
}

#[tokio::test]
async fn test_cache_with_no_stale() {
    let cache = create_test_cache_manager().await;
    let key = "test:cache:no_stale";
    let data = TestData::new(9, "no_stale", 777.0);

    // Set with category that doesn't allow stale (AiResponse)
    cache.set_with_category(key, &data, CacheCategory::AiResponse)
        .await
        .expect("Failed to set with category");

    // Fresh cache should exist
    let fresh: Option<TestData> = cache.get(key)
        .await
        .expect("Failed to get from cache");
    assert_eq!(fresh, Some(data));

    // Stale cache should NOT exist
    let stale_key = format!("{}:stale", key);
    assert!(!cache.exists(&stale_key).await.expect("Failed to check stale existence"));

    // Cleanup
    cache.delete(key).await.expect("Failed to delete");
}

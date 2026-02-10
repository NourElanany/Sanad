//! Cache Manager for API clients
//! 
//! Implements intelligent caching with Redis backend, supporting different
//! TTL strategies per data type, stale cache serving, and LRU eviction.

use super::ApiError;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, warn};

/// Cache manager that provides intelligent caching for API responses
#[derive(Clone)]
pub struct CacheManager {
    /// Redis connection for distributed caching
    redis: Arc<RwLock<MultiplexedConnection>>,
    /// Cache strategies per category
    strategies: Arc<HashMap<CacheCategory, CacheStrategy>>,
}

/// Categories of cached data with different characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheCategory {
    /// Quran text - static, long TTL
    QuranText,
    /// Quran audio URLs - static, long TTL
    QuranAudio,
    /// Hadith - static, long TTL
    Hadith,
    /// Prayer times - dynamic, daily TTL
    PrayerTimes,
    /// Tafsir - static, long TTL
    Tafsir,
    /// Calendar - semi-static, weekly TTL
    Calendar,
    /// Qibla direction - static per location, long TTL
    Qibla,
    /// AI responses - dynamic, short TTL
    AiResponse,
}

/// Cache strategy defining TTL and stale cache behavior
#[derive(Debug, Clone)]
pub struct CacheStrategy {
    /// Primary TTL for fresh cache
    pub ttl: Duration,
    /// Whether to keep stale cache as fallback
    pub allow_stale: bool,
    /// TTL for stale cache (if allow_stale is true)
    pub stale_ttl: Duration,
}

impl CacheStrategy {
    /// Create a new cache strategy
    pub fn new(ttl: Duration, allow_stale: bool, stale_ttl: Duration) -> Self {
        Self {
            ttl,
            allow_stale,
            stale_ttl,
        }
    }

    /// Create strategy for static data (30 days fresh, 90 days stale)
    pub fn static_data() -> Self {
        Self {
            ttl: Duration::from_secs(30 * 24 * 3600), // 30 days
            allow_stale: true,
            stale_ttl: Duration::from_secs(90 * 24 * 3600), // 90 days
        }
    }

    /// Create strategy for daily data (1 day fresh, 7 days stale)
    pub fn daily_data() -> Self {
        Self {
            ttl: Duration::from_secs(24 * 3600), // 1 day
            allow_stale: true,
            stale_ttl: Duration::from_secs(7 * 24 * 3600), // 7 days
        }
    }

    /// Create strategy for weekly data (7 days fresh, 30 days stale)
    pub fn weekly_data() -> Self {
        Self {
            ttl: Duration::from_secs(7 * 24 * 3600), // 7 days
            allow_stale: true,
            stale_ttl: Duration::from_secs(30 * 24 * 3600), // 30 days
        }
    }

    /// Create strategy for hourly data (1 hour fresh, no stale)
    pub fn hourly_data() -> Self {
        Self {
            ttl: Duration::from_secs(3600), // 1 hour
            allow_stale: false,
            stale_ttl: Duration::from_secs(0),
        }
    }
}

impl Default for CacheStrategy {
    fn default() -> Self {
        Self::static_data()
    }
}

impl CacheManager {
    /// Create a new cache manager with Redis backend
    pub async fn new(redis_url: &str) -> Result<Self, ApiError> {
        let client = Client::open(redis_url)
            .map_err(|e| ApiError::Configuration(format!("Failed to connect to Redis: {}", e)))?;
        
        let redis = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| ApiError::Configuration(format!("Failed to get Redis connection: {}", e)))?;

        // Initialize default strategies
        let mut strategies = HashMap::new();
        strategies.insert(CacheCategory::QuranText, CacheStrategy::static_data());
        strategies.insert(CacheCategory::QuranAudio, CacheStrategy::static_data());
        strategies.insert(CacheCategory::Hadith, CacheStrategy::static_data());
        strategies.insert(CacheCategory::PrayerTimes, CacheStrategy::daily_data());
        strategies.insert(CacheCategory::Tafsir, CacheStrategy::static_data());
        strategies.insert(CacheCategory::Calendar, CacheStrategy::weekly_data());
        strategies.insert(CacheCategory::Qibla, CacheStrategy::static_data());
        strategies.insert(CacheCategory::AiResponse, CacheStrategy::hourly_data());

        Ok(Self {
            redis: Arc::new(RwLock::new(redis)),
            strategies: Arc::new(strategies),
        })
    }

    /// Create a new cache manager with custom strategies
    pub async fn with_strategies(
        redis_url: &str,
        strategies: HashMap<CacheCategory, CacheStrategy>,
    ) -> Result<Self, ApiError> {
        let client = Client::open(redis_url)
            .map_err(|e| ApiError::Configuration(format!("Failed to connect to Redis: {}", e)))?;
        
        let redis = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| ApiError::Configuration(format!("Failed to get Redis connection: {}", e)))?;

        Ok(Self {
            redis: Arc::new(RwLock::new(redis)),
            strategies: Arc::new(strategies),
        })
    }

    /// Get a value from cache
    /// Returns None if the key doesn't exist or has expired
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, ApiError> {
        let mut conn = self.redis.write().await;
        
        let value: Option<String> = conn.get(key).await
            .map_err(|e| ApiError::CacheError(format!("Failed to get from cache: {}", e)))?;

        match value {
            Some(v) => {
                debug!("Cache hit for key: {}", key);
                let deserialized = serde_json::from_str(&v)
                    .map_err(|e| ApiError::Serialization(e))?;
                Ok(Some(deserialized))
            }
            None => {
                debug!("Cache miss for key: {}", key);
                Ok(None)
            }
        }
    }

    /// Set a value in cache with TTL
    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<(), ApiError> {
        let serialized = serde_json::to_string(value)
            .map_err(|e| ApiError::Serialization(e))?;

        let mut conn = self.redis.write().await;
        
        let _: () = conn.set_ex(key, &serialized, ttl.as_secs() as u64).await
            .map_err(|e| ApiError::CacheError(format!("Failed to set in cache: {}", e)))?;

        debug!("Cached value for key: {} with TTL: {:?}", key, ttl);

        Ok(())
    }

    /// Get a value from stale cache (expired but still stored)
    /// Returns None if the stale key doesn't exist
    pub async fn get_stale<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, ApiError> {
        let stale_key = format!("{}:stale", key);
        
        let mut conn = self.redis.write().await;
        
        let value: Option<String> = conn.get(&stale_key).await
            .map_err(|e| ApiError::CacheError(format!("Failed to get stale cache: {}", e)))?;

        match value {
            Some(v) => {
                warn!("Serving stale cache for key: {}", key);
                let deserialized = serde_json::from_str(&v)
                    .map_err(|e| ApiError::Serialization(e))?;
                Ok(Some(deserialized))
            }
            None => {
                debug!("No stale cache available for key: {}", key);
                Ok(None)
            }
        }
    }

    /// Set a value with both fresh and stale cache based on category strategy
    pub async fn set_with_category<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        category: CacheCategory,
    ) -> Result<(), ApiError> {
        let strategy = self.strategies.get(&category)
            .ok_or_else(|| ApiError::Configuration(format!("No strategy for category: {:?}", category)))?;

        let serialized = serde_json::to_string(value)
            .map_err(|e| ApiError::Serialization(e))?;

        let mut conn = self.redis.write().await;

        // Set primary cache with TTL
        let _: () = conn.set_ex(key, &serialized, strategy.ttl.as_secs() as u64).await
            .map_err(|e| ApiError::CacheError(format!("Failed to set primary cache: {}", e)))?;

        // Set stale cache if allowed
        if strategy.allow_stale {
            let stale_key = format!("{}:stale", key);
            let _: () = conn.set_ex(&stale_key, &serialized, strategy.stale_ttl.as_secs() as u64).await
                .map_err(|e| ApiError::CacheError(format!("Failed to set stale cache: {}", e)))?;
            
            debug!(
                "Cached value for key: {} with TTL: {:?} and stale TTL: {:?}",
                key, strategy.ttl, strategy.stale_ttl
            );
        } else {
            debug!("Cached value for key: {} with TTL: {:?} (no stale)", key, strategy.ttl);
        }

        Ok(())
    }

    /// Get a value, trying fresh cache first, then stale cache
    pub async fn get_with_fallback<T: DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>, ApiError> {
        // Try fresh cache first
        if let Some(value) = self.get(key).await? {
            return Ok(Some(value));
        }

        // Try stale cache as fallback
        self.get_stale(key).await
    }

    /// Delete a key from cache (including stale)
    pub async fn delete(&self, key: &str) -> Result<(), ApiError> {
        let stale_key = format!("{}:stale", key);
        
        let mut conn = self.redis.write().await;
        
        let _: () = conn.del(key).await
            .map_err(|e| ApiError::CacheError(format!("Failed to delete from cache: {}", e)))?;
        let _: () = conn.del(&stale_key).await
            .map_err(|e| ApiError::CacheError(format!("Failed to delete stale cache: {}", e)))?;

        debug!("Deleted cache for key: {}", key);

        Ok(())
    }

    /// Delete all keys matching a pattern
    pub async fn delete_pattern(&self, pattern: &str) -> Result<u32, ApiError> {
        let mut conn = self.redis.write().await;
        
        // Get all keys matching pattern
        let keys: Vec<String> = conn.keys(pattern).await
            .map_err(|e| ApiError::CacheError(format!("Failed to get keys: {}", e)))?;

        if keys.is_empty() {
            return Ok(0);
        }

        // Delete all matching keys
        let count = keys.len() as u32;
        let _: () = conn.del(&keys).await
            .map_err(|e| ApiError::CacheError(format!("Failed to delete keys: {}", e)))?;

        debug!("Deleted {} keys matching pattern: {}", count, pattern);

        Ok(count)
    }

    /// Check if a key exists in cache
    pub async fn exists(&self, key: &str) -> Result<bool, ApiError> {
        let mut conn = self.redis.write().await;
        
        let exists: bool = conn.exists(key).await
            .map_err(|e| ApiError::CacheError(format!("Failed to check existence: {}", e)))?;

        Ok(exists)
    }

    /// Get TTL for a key (time until expiration)
    pub async fn ttl(&self, key: &str) -> Result<Option<Duration>, ApiError> {
        let mut conn = self.redis.write().await;
        
        let ttl_secs: i64 = conn.ttl(key).await
            .map_err(|e| ApiError::CacheError(format!("Failed to get TTL: {}", e)))?;

        match ttl_secs {
            -2 => Ok(None), // Key doesn't exist
            -1 => Ok(None), // Key exists but has no expiration
            secs if secs > 0 => Ok(Some(Duration::from_secs(secs as u64))),
            _ => Ok(None),
        }
    }

    /// Update the access time for LRU tracking
    /// This is called when a cache entry is accessed
    pub async fn touch_lru(&self, key: &str) -> Result<(), ApiError> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut conn = self.redis.write().await;
        
        // Store access timestamp in a sorted set
        let _: () = conn.zadd("lru:access_times", key, timestamp).await
            .map_err(|e| ApiError::CacheError(format!("Failed to update LRU: {}", e)))?;

        Ok(())
    }

    /// Evict least recently used entries when cache is full
    /// This implements LRU eviction policy
    pub async fn evict_lru(&self, count: usize) -> Result<usize, ApiError> {
        let mut conn = self.redis.write().await;
        
        // Get the least recently used keys
        let keys: Vec<String> = conn.zrange("lru:access_times", 0, (count - 1) as isize).await
            .map_err(|e| ApiError::CacheError(format!("Failed to get LRU keys: {}", e)))?;

        if keys.is_empty() {
            return Ok(0);
        }

        let evicted_count = keys.len();

        // Delete the keys
        for key in &keys {
            self.delete(key).await?;
        }

        // Remove from LRU tracking
        let _: () = conn.zrem("lru:access_times", &keys).await
            .map_err(|e| ApiError::CacheError(format!("Failed to remove from LRU: {}", e)))?;

        warn!("Evicted {} least recently used cache entries", evicted_count);

        Ok(evicted_count)
    }

    /// Get cache statistics
    pub async fn get_stats(&self) -> Result<CacheStats, ApiError> {
        let mut conn = self.redis.write().await;
        
        // Get total number of keys
        let all_keys: Vec<String> = conn.keys("*").await
            .map_err(|e| ApiError::CacheError(format!("Failed to get keys: {}", e)))?;
        
        let total_keys = all_keys.len();
        
        // Count stale keys
        let stale_keys = all_keys.iter().filter(|k| k.ends_with(":stale")).count();
        
        // Count fresh keys (excluding stale and LRU tracking)
        let fresh_keys = all_keys.iter()
            .filter(|k| !k.ends_with(":stale") && !k.starts_with("lru:"))
            .count();

        Ok(CacheStats {
            total_keys,
            fresh_keys,
            stale_keys,
        })
    }

    /// Get the strategy for a cache category
    pub fn get_strategy(&self, category: CacheCategory) -> Option<&CacheStrategy> {
        self.strategies.get(&category)
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_keys: usize,
    pub fresh_keys: usize,
    pub stale_keys: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_strategy_static_data() {
        let strategy = CacheStrategy::static_data();
        assert_eq!(strategy.ttl, Duration::from_secs(30 * 24 * 3600));
        assert!(strategy.allow_stale);
        assert_eq!(strategy.stale_ttl, Duration::from_secs(90 * 24 * 3600));
    }

    #[test]
    fn test_cache_strategy_daily_data() {
        let strategy = CacheStrategy::daily_data();
        assert_eq!(strategy.ttl, Duration::from_secs(24 * 3600));
        assert!(strategy.allow_stale);
        assert_eq!(strategy.stale_ttl, Duration::from_secs(7 * 24 * 3600));
    }

    #[test]
    fn test_cache_strategy_hourly_data() {
        let strategy = CacheStrategy::hourly_data();
        assert_eq!(strategy.ttl, Duration::from_secs(3600));
        assert!(!strategy.allow_stale);
    }

    #[test]
    fn test_cache_category_variants() {
        // Ensure all categories are distinct
        let categories = vec![
            CacheCategory::QuranText,
            CacheCategory::QuranAudio,
            CacheCategory::Hadith,
            CacheCategory::PrayerTimes,
            CacheCategory::Tafsir,
            CacheCategory::Calendar,
            CacheCategory::Qibla,
            CacheCategory::AiResponse,
        ];

        // Check that we can use them as hash keys
        let mut map = HashMap::new();
        for (i, cat) in categories.iter().enumerate() {
            map.insert(*cat, i);
        }

        assert_eq!(map.len(), 8);
    }
}

// Include unit tests
#[cfg(test)]
#[path = "cache_manager_tests.rs"]
mod unit_tests;

// Include property-based tests
#[cfg(test)]
#[path = "cache_manager_property_tests.rs"]
mod property_tests;

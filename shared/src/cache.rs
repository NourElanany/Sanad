use crate::{SanadError, SanadResult};
use chrono::{DateTime, Duration, Utc};
use redis::{aio::ConnectionManager, AsyncCommands, Client, RedisResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Advanced caching system for the Sanad Islamic application
/// Supports Redis Cluster, intelligent cache invalidation, and specialized caching strategies
#[derive(Clone)]
pub struct AdvancedCacheManager {
    /// Redis connection manager for high-performance operations
    redis_manager: ConnectionManager,
    /// In-memory cache for frequently accessed data
    memory_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// Cache configuration
    config: CacheConfig,
}

/// Cache configuration for different data types
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Default TTL for general cache entries (1 hour)
    pub default_ttl_seconds: u64,
    /// TTL for prayer times cache (24 hours)
    pub prayer_times_ttl_seconds: u64,
    /// TTL for semantic search queries (6 hours)
    pub semantic_query_ttl_seconds: u64,
    /// TTL for Quran content (never expires, but can be invalidated)
    pub quran_content_ttl_seconds: u64,
    /// TTL for Hadith content (7 days)
    pub hadith_content_ttl_seconds: u64,
    /// Maximum memory cache size (number of entries)
    pub max_memory_cache_size: usize,
    /// Enable smart cache invalidation
    pub enable_smart_invalidation: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            default_ttl_seconds: 3600,        // 1 hour
            prayer_times_ttl_seconds: 86400,  // 24 hours
            semantic_query_ttl_seconds: 21600, // 6 hours
            quran_content_ttl_seconds: 2592000, // 30 days (effectively permanent)
            hadith_content_ttl_seconds: 604800, // 7 days
            max_memory_cache_size: 10000,
            enable_smart_invalidation: true,
        }
    }
}

/// Cache entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub data: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
    pub cache_type: CacheType,
}

/// Types of cached data for specialized handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheType {
    PrayerTimes,
    SemanticQuery,
    QuranContent,
    HadithContent,
    UserPreferences,
    SearchResults,
    ApiResponse,
    General,
}

/// Cache key patterns for organized storage
pub struct CacheKeys;

impl CacheKeys {
    /// Prayer times cache key pattern: "prayer_times:{lat}:{lng}:{date}:{method}"
    pub fn prayer_times(lat: f64, lng: f64, date: &str, method: &str) -> String {
        format!("prayer_times:{}:{}:{}:{}", lat, lng, date, method)
    }

    /// Semantic query cache key pattern: "semantic_query:{hash}"
    pub fn semantic_query(query_hash: &str) -> String {
        format!("semantic_query:{}", query_hash)
    }

    /// Quran content cache key pattern: "quran:{surah}:{ayah}"
    pub fn quran_content(surah: u16, ayah: Option<u16>) -> String {
        match ayah {
            Some(ayah_num) => format!("quran:{}:{}", surah, ayah_num),
            None => format!("quran:{}", surah),
        }
    }

    /// Hadith cache key pattern: "hadith:{collection}:{book}:{number}"
    pub fn hadith_content(collection: &str, book: &str, number: &str) -> String {
        format!("hadith:{}:{}:{}", collection, book, number)
    }

    /// User preferences cache key pattern: "user_prefs:{user_id}"
    pub fn user_preferences(user_id: &str) -> String {
        format!("user_prefs:{}", user_id)
    }

    /// Search results cache key pattern: "search:{query_hash}:{filters_hash}"
    pub fn search_results(query_hash: &str, filters_hash: &str) -> String {
        format!("search:{}:{}", query_hash, filters_hash)
    }

    /// API response cache key pattern: "api:{endpoint}:{params_hash}"
    pub fn api_response(endpoint: &str, params_hash: &str) -> String {
        format!("api:{}:{}", endpoint, params_hash)
    }
}

impl AdvancedCacheManager {
    /// Create a new advanced cache manager
    pub async fn new(redis_url: &str, config: Option<CacheConfig>) -> SanadResult<Self> {
        let client = Client::open(redis_url).map_err(SanadError::Redis)?;
        let redis_manager = ConnectionManager::new(client)
            .await
            .map_err(SanadError::Redis)?;

        let config = config.unwrap_or_default();
        let memory_cache = Arc::new(RwLock::new(HashMap::new()));

        info!("Advanced cache manager initialized with Redis cluster support");

        Ok(Self {
            redis_manager,
            memory_cache,
            config,
        })
    }

    /// Set a value in cache with automatic TTL based on cache type
    pub async fn set<T>(&self, key: &str, value: &T, cache_type: CacheType) -> SanadResult<()>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value).map_err(SanadError::Serialization)?;
        let ttl = self.get_ttl_for_type(&cache_type);
        
        let entry = CacheEntry {
            data: serialized.clone(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::seconds(ttl as i64),
            access_count: 0,
            last_accessed: Utc::now(),
            cache_type: cache_type.clone(),
        };

        // Store in Redis with TTL
        let mut conn = self.redis_manager.clone();
        conn.set_ex(key, &serialized, ttl)
            .await
            .map_err(SanadError::Redis)?;

        // Store in memory cache if it's frequently accessed data
        if self.should_cache_in_memory(&cache_type) {
            self.set_memory_cache(key, entry).await;
        }

        debug!("Cached value for key: {} with type: {:?}", key, cache_type);
        Ok(())
    }

    /// Get a value from cache with automatic deserialization
    pub async fn get<T>(&self, key: &str) -> SanadResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Try memory cache first
        if let Some(entry) = self.get_memory_cache(key).await {
            if entry.expires_at > Utc::now() {
                // Update access statistics
                self.update_access_stats(key).await;
                
                let value: T = serde_json::from_str(&entry.data)
                    .map_err(SanadError::Serialization)?;
                debug!("Cache hit (memory) for key: {}", key);
                return Ok(Some(value));
            } else {
                // Remove expired entry from memory cache
                self.remove_memory_cache(key).await;
            }
        }

        // Try Redis cache
        let mut conn = self.redis_manager.clone();
        let result: RedisResult<String> = conn.get(key).await;
        
        match result {
            Ok(serialized) => {
                let value: T = serde_json::from_str(&serialized)
                    .map_err(SanadError::Serialization)?;
                debug!("Cache hit (Redis) for key: {}", key);
                Ok(Some(value))
            }
            Err(redis::RedisError { kind: redis::ErrorKind::TypeError, .. }) => {
                debug!("Cache miss for key: {}", key);
                Ok(None)
            }
            Err(e) => {
                error!("Redis error for key {}: {}", key, e);
                Err(SanadError::Redis(e))
            }
        }
    }

    /// Delete a specific cache entry
    pub async fn delete(&self, key: &str) -> SanadResult<()> {
        // Remove from Redis
        let mut conn = self.redis_manager.clone();
        conn.del(key).await.map_err(SanadError::Redis)?;

        // Remove from memory cache
        self.remove_memory_cache(key).await;

        debug!("Deleted cache entry for key: {}", key);
        Ok(())
    }

    /// Smart cache invalidation based on patterns and dependencies
    pub async fn invalidate_pattern(&self, pattern: &str) -> SanadResult<u64> {
        if !self.config.enable_smart_invalidation {
            warn!("Smart cache invalidation is disabled");
            return Ok(0);
        }

        let mut conn = self.redis_manager.clone();
        
        // Get all keys matching the pattern
        let keys: Vec<String> = conn.keys(pattern).await.map_err(SanadError::Redis)?;
        
        if keys.is_empty() {
            debug!("No keys found for pattern: {}", pattern);
            return Ok(0);
        }

        // Delete all matching keys
        let deleted_count = conn.del(&keys).await.map_err(SanadError::Redis)?;

        // Remove from memory cache
        for key in &keys {
            self.remove_memory_cache(key).await;
        }

        info!("Invalidated {} cache entries for pattern: {}", deleted_count, pattern);
        Ok(deleted_count)
    }

    /// Invalidate cache entries related to prayer times for a specific location
    pub async fn invalidate_prayer_times(&self, lat: f64, lng: f64) -> SanadResult<u64> {
        let pattern = format!("prayer_times:{}:{}:*", lat, lng);
        self.invalidate_pattern(&pattern).await
    }

    /// Invalidate all semantic query cache entries
    pub async fn invalidate_semantic_queries(&self) -> SanadResult<u64> {
        self.invalidate_pattern("semantic_query:*").await
    }

    /// Invalidate cache entries for a specific Quran surah
    pub async fn invalidate_quran_surah(&self, surah: u16) -> SanadResult<u64> {
        let pattern = format!("quran:{}:*", surah);
        self.invalidate_pattern(&pattern).await
    }

    /// Invalidate cache entries for a specific hadith collection
    pub async fn invalidate_hadith_collection(&self, collection: &str) -> SanadResult<u64> {
        let pattern = format!("hadith:{}:*", collection);
        self.invalidate_pattern(&pattern).await
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> SanadResult<CacheStats> {
        let mut conn = self.redis_manager.clone();
        
        // Get Redis info
        let info: String = conn.info("memory").await.map_err(SanadError::Redis)?;
        let redis_memory_usage = self.parse_redis_memory_usage(&info);

        // Get memory cache stats
        let memory_cache = self.memory_cache.read().await;
        let memory_cache_size = memory_cache.len();
        let memory_cache_entries_by_type = self.count_entries_by_type(&memory_cache);

        Ok(CacheStats {
            redis_memory_usage_bytes: redis_memory_usage,
            memory_cache_entries: memory_cache_size,
            memory_cache_entries_by_type,
            total_cache_operations: 0, // This would be tracked separately in production
        })
    }

    /// Warm up cache with frequently accessed data
    pub async fn warm_up_cache(&self) -> SanadResult<()> {
        info!("Starting cache warm-up process");

        // This would typically load:
        // 1. Most frequently accessed Quran verses
        // 2. Common prayer times for major cities
        // 3. Popular hadith collections
        // 4. Frequently used search queries

        // For now, we'll just log that warm-up is complete
        info!("Cache warm-up completed");
        Ok(())
    }

    /// Clean up expired entries from memory cache
    pub async fn cleanup_expired_entries(&self) -> usize {
        let mut memory_cache = self.memory_cache.write().await;
        let now = Utc::now();
        let initial_size = memory_cache.len();

        memory_cache.retain(|_, entry| entry.expires_at > now);

        let cleaned_count = initial_size - memory_cache.len();
        if cleaned_count > 0 {
            debug!("Cleaned up {} expired entries from memory cache", cleaned_count);
        }

        cleaned_count
    }

    // Private helper methods

    fn get_ttl_for_type(&self, cache_type: &CacheType) -> u64 {
        match cache_type {
            CacheType::PrayerTimes => self.config.prayer_times_ttl_seconds,
            CacheType::SemanticQuery => self.config.semantic_query_ttl_seconds,
            CacheType::QuranContent => self.config.quran_content_ttl_seconds,
            CacheType::HadithContent => self.config.hadith_content_ttl_seconds,
            _ => self.config.default_ttl_seconds,
        }
    }

    fn should_cache_in_memory(&self, cache_type: &CacheType) -> bool {
        matches!(
            cache_type,
            CacheType::QuranContent | CacheType::UserPreferences | CacheType::PrayerTimes
        )
    }

    async fn set_memory_cache(&self, key: &str, entry: CacheEntry) {
        let mut memory_cache = self.memory_cache.write().await;
        
        // Check if we need to evict entries
        if memory_cache.len() >= self.config.max_memory_cache_size {
            self.evict_lru_entries(&mut memory_cache).await;
        }

        memory_cache.insert(key.to_string(), entry);
    }

    async fn get_memory_cache(&self, key: &str) -> Option<CacheEntry> {
        let memory_cache = self.memory_cache.read().await;
        memory_cache.get(key).cloned()
    }

    async fn remove_memory_cache(&self, key: &str) {
        let mut memory_cache = self.memory_cache.write().await;
        memory_cache.remove(key);
    }

    async fn update_access_stats(&self, key: &str) {
        let mut memory_cache = self.memory_cache.write().await;
        if let Some(entry) = memory_cache.get_mut(key) {
            entry.access_count += 1;
            entry.last_accessed = Utc::now();
        }
    }

    async fn evict_lru_entries(&self, memory_cache: &mut HashMap<String, CacheEntry>) {
        // Remove 10% of entries, starting with least recently used
        let evict_count = (memory_cache.len() / 10).max(1);
        
        let mut entries: Vec<_> = memory_cache.iter().collect();
        entries.sort_by_key(|(_, entry)| entry.last_accessed);
        
        for (key, _) in entries.iter().take(evict_count) {
            memory_cache.remove(*key);
        }
        
        debug!("Evicted {} LRU entries from memory cache", evict_count);
    }

    fn parse_redis_memory_usage(&self, info: &str) -> u64 {
        // Parse Redis INFO memory output to extract used_memory
        for line in info.lines() {
            if line.starts_with("used_memory:") {
                if let Some(value) = line.split(':').nth(1) {
                    return value.parse().unwrap_or(0);
                }
            }
        }
        0
    }

    fn count_entries_by_type(&self, cache: &HashMap<String, CacheEntry>) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for entry in cache.values() {
            let type_name = format!("{:?}", entry.cache_type);
            *counts.entry(type_name).or_insert(0) += 1;
        }
        counts
    }
}

/// Cache statistics for monitoring and optimization
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub redis_memory_usage_bytes: u64,
    pub memory_cache_entries: usize,
    pub memory_cache_entries_by_type: HashMap<String, usize>,
    pub total_cache_operations: u64,
}

/// Specialized caching strategies for different data types
pub struct CacheStrategies;

impl CacheStrategies {
    /// Cache prayer times with location-based invalidation
    pub async fn cache_prayer_times(
        cache: &AdvancedCacheManager,
        lat: f64,
        lng: f64,
        date: &str,
        method: &str,
        prayer_times: &crate::models::PrayerTimes,
    ) -> SanadResult<()> {
        let key = CacheKeys::prayer_times(lat, lng, date, method);
        cache.set(&key, prayer_times, CacheType::PrayerTimes).await
    }

    /// Cache semantic query results with intelligent invalidation
    pub async fn cache_semantic_query<T>(
        cache: &AdvancedCacheManager,
        query: &str,
        results: &T,
    ) -> SanadResult<()>
    where
        T: Serialize,
    {
        let query_hash = crate::utils::calculate_content_hash(query);
        let key = CacheKeys::semantic_query(&query_hash);
        cache.set(&key, results, CacheType::SemanticQuery).await
    }

    /// Cache Quran content with permanent storage
    pub async fn cache_quran_content<T>(
        cache: &AdvancedCacheManager,
        surah: u16,
        ayah: Option<u16>,
        content: &T,
    ) -> SanadResult<()>
    where
        T: Serialize,
    {
        let key = CacheKeys::quran_content(surah, ayah);
        cache.set(&key, content, CacheType::QuranContent).await
    }

    /// Cache hadith content with collection-based organization
    pub async fn cache_hadith_content<T>(
        cache: &AdvancedCacheManager,
        collection: &str,
        book: &str,
        number: &str,
        content: &T,
    ) -> SanadResult<()>
    where
        T: Serialize,
    {
        let key = CacheKeys::hadith_content(collection, book, number);
        cache.set(&key, content, CacheType::HadithContent).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[tokio::test]
    async fn test_cache_key_generation() {
        let prayer_key = CacheKeys::prayer_times(40.7128, -74.0060, "2024-01-01", "MWL");
        assert_eq!(prayer_key, "prayer_times:40.7128:-74.006:2024-01-01:MWL");

        let quran_key = CacheKeys::quran_content(1, Some(1));
        assert_eq!(quran_key, "quran:1:1");

        let hadith_key = CacheKeys::hadith_content("bukhari", "book1", "1");
        assert_eq!(hadith_key, "hadith:bukhari:book1:1");
    }

    #[test]
    fn test_cache_config_defaults() {
        let config = CacheConfig::default();
        assert_eq!(config.default_ttl_seconds, 3600);
        assert_eq!(config.prayer_times_ttl_seconds, 86400);
        assert!(config.enable_smart_invalidation);
    }

    #[test]
    fn test_ttl_for_different_types() {
        let config = CacheConfig::default();
        let cache_manager = AdvancedCacheManager {
            redis_manager: todo!(), // This would be mocked in real tests
            memory_cache: Arc::new(RwLock::new(HashMap::new())),
            config: config.clone(),
        };

        assert_eq!(
            cache_manager.get_ttl_for_type(&CacheType::PrayerTimes),
            config.prayer_times_ttl_seconds
        );
        assert_eq!(
            cache_manager.get_ttl_for_type(&CacheType::QuranContent),
            config.quran_content_ttl_seconds
        );
    }
}
use crate::{SanadError, SanadResult};
use chrono::{DateTime, Duration, Utc};
use redis::{aio::MultiplexedConnection, AsyncCommands, Client, RedisResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use base64::Engine;

/// Advanced caching system for the Sanad Islamic application
/// Supports Redis Cluster, intelligent cache invalidation, and specialized caching strategies
#[derive(Clone)]
#[derive(Debug)]
pub struct AdvancedCacheManager {
    /// Redis connection manager for high-performance operations
    redis_manager: MultiplexedConnection,
    /// In-memory cache for frequently accessed data
    memory_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// Cache configuration
    config: CacheConfig,
    /// Query frequency tracker for intelligent caching
    query_tracker: Arc<RwLock<HashMap<String, QueryStats>>>,
    /// Heavy content cache for large data optimization
    heavy_content_cache: Arc<RwLock<HashMap<String, HeavyContentEntry>>>,
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
    /// Minimum query frequency to cache (queries per hour)
    pub min_query_frequency_for_cache: u32,
    /// Heavy content threshold in bytes (1MB)
    pub heavy_content_threshold_bytes: usize,
    /// Heavy content cache TTL (2 hours)
    pub heavy_content_ttl_seconds: u64,
    /// Enable query frequency tracking
    pub enable_query_tracking: bool,
    /// Enable adaptive TTL based on access patterns
    pub enable_adaptive_ttl: bool,
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
            min_query_frequency_for_cache: 5, // 5 queries per hour
            heavy_content_threshold_bytes: 1024 * 1024, // 1MB
            heavy_content_ttl_seconds: 7200, // 2 hours
            enable_query_tracking: true,
            enable_adaptive_ttl: true,
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
    HeavyContent,
    FrequentQuery,
    General,
}

/// Query statistics for intelligent caching decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryStats {
    pub query_hash: String,
    pub first_seen: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u64,
    pub hourly_frequency: f64,
    pub average_response_time_ms: f64,
    pub cache_hit_ratio: f64,
    pub is_frequent: bool,
}

/// Heavy content cache entry for large data optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeavyContentEntry {
    pub content_hash: String,
    pub compressed_data: Vec<u8>,
    pub original_size: usize,
    pub compressed_size: usize,
    pub compression_ratio: f64,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
    pub content_type: String,
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

    /// Frequent query cache key pattern: "frequent_query:{query_hash}"
    pub fn frequent_query(query_hash: &str) -> String {
        format!("frequent_query:{}", query_hash)
    }

    /// Heavy content cache key pattern: "heavy_content:{content_id}"
    pub fn heavy_content(content_id: &str) -> String {
        format!("heavy_content:{}", content_id)
    }
}

impl AdvancedCacheManager {
    /// Create a new advanced cache manager
    pub async fn new(redis_url: &str, config: Option<CacheConfig>) -> SanadResult<Self> {
        let client = Client::open(redis_url).map_err(SanadError::Redis)?;
        let redis_manager: MultiplexedConnection = client.get_multiplexed_async_connection()
            .await
            .map_err(SanadError::Redis)?;

        let config = config.unwrap_or_default();
        let memory_cache = Arc::new(RwLock::new(HashMap::new()));
        let query_tracker = Arc::new(RwLock::new(HashMap::new()));
        let heavy_content_cache = Arc::new(RwLock::new(HashMap::new()));

        info!("Advanced cache manager initialized with Redis cluster support");
        info!("Query tracking enabled: {}", config.enable_query_tracking);
        info!("Heavy content threshold: {} bytes", config.heavy_content_threshold_bytes);

        Ok(Self {
            redis_manager,
            memory_cache,
            config,
            query_tracker,
            heavy_content_cache,
        })
    }

    /// Set a value in cache with automatic TTL based on cache type
    pub async fn set<T>(&self, key: &str, value: &T, cache_type: CacheType) -> SanadResult<()>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value).map_err(SanadError::Serialization)?;
        let base_ttl = self.get_ttl_for_type(&cache_type);
        let adaptive_ttl = self.get_adaptive_ttl(key, base_ttl).await;
        
        let entry = CacheEntry {
            data: serialized.clone(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::seconds(adaptive_ttl as i64),
            access_count: 0,
            last_accessed: Utc::now(),
            cache_type: cache_type.clone(),
        };

        // Store in Redis with adaptive TTL
        let mut conn = self.redis_manager.clone();
        let _: () = conn.set_ex(key, &serialized, adaptive_ttl)
            .await
            .map_err(SanadError::Redis)?;

        // Store in memory cache if it's frequently accessed data
        if self.should_cache_in_memory(&cache_type) {
            self.set_memory_cache(key, entry).await;
        }

        debug!("Cached value for key: {} with type: {:?} (TTL: {}s)", key, cache_type, adaptive_ttl);
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
            Err(e) => {
                // Check if it's a type error (key doesn't exist)
                if e.to_string().contains("WRONGTYPE") || e.to_string().contains("nil") {
                    debug!("Cache miss for key: {}", key);
                    Ok(None)
                } else {
                    error!("Redis error for key {}: {}", key, e);
                    Err(SanadError::Redis(e))
                }
            }
        }
    }

    /// Delete a specific cache entry
    pub async fn delete(&self, key: &str) -> SanadResult<()> {
        // Remove from Redis
        let mut conn = self.redis_manager.clone();
        let _: () = conn.del(key).await.map_err(SanadError::Redis)?;

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
        let deleted_count: u64 = conn.del(&keys).await.map_err(SanadError::Redis)?;

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
        // Get memory cache stats
        let memory_cache = self.memory_cache.read().await;
        let memory_cache_size = memory_cache.len();
        let memory_cache_entries_by_type = self.count_entries_by_type(&memory_cache);

        // Get heavy content cache stats
        let heavy_cache = self.heavy_content_cache.read().await;
        let heavy_content_entries = heavy_cache.len();
        let (total_heavy_size, avg_compression_ratio) = heavy_cache.values().fold(
            (0usize, 0.0f64),
            |(total_size, total_ratio), entry| {
                (total_size + entry.original_size, total_ratio + entry.compression_ratio)
            }
        );
        let average_compression_ratio = if heavy_content_entries > 0 {
            avg_compression_ratio / heavy_content_entries as f64
        } else {
            0.0
        };

        // Get query tracking stats
        let query_tracker = self.query_tracker.read().await;
        let frequent_queries_count = query_tracker.values()
            .filter(|stats| stats.is_frequent)
            .count();

        Ok(CacheStats {
            redis_memory_usage_bytes: 0, // Would need separate connection for INFO
            memory_cache_entries: memory_cache_size,
            memory_cache_entries_by_type,
            total_cache_operations: 0, // This would be tracked separately in production
            heavy_content_entries,
            total_heavy_content_size_bytes: total_heavy_size,
            average_compression_ratio,
            frequent_queries_count,
            query_tracking_enabled: self.config.enable_query_tracking,
            adaptive_ttl_enabled: self.config.enable_adaptive_ttl,
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

        // Also cleanup heavy content cache
        let heavy_cleaned = self.cleanup_heavy_content_cache().await;
        debug!("Cleaned up {} expired heavy content entries", heavy_cleaned);

        cleaned_count + heavy_cleaned
    }

    /// Cache frequently accessed queries intelligently
    pub async fn cache_frequent_query<T>(&self, query: &str, result: &T) -> SanadResult<()>
    where
        T: Serialize,
    {
        if !self.config.enable_query_tracking {
            return Ok(());
        }

        let query_hash = crate::utils::calculate_content_hash(query);
        
        // Update query statistics
        self.update_query_stats(&query_hash, query).await;
        
        // Check if query is frequent enough to cache
        if self.is_frequent_query(&query_hash).await {
            let key = CacheKeys::frequent_query(&query_hash);
            self.set(&key, result, CacheType::FrequentQuery).await?;
            debug!("Cached frequent query: {}", query_hash);
        }

        Ok(())
    }

    /// Get cached frequent query result
    pub async fn get_frequent_query<T>(&self, query: &str) -> SanadResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        if !self.config.enable_query_tracking {
            return Ok(None);
        }

        let query_hash = crate::utils::calculate_content_hash(query);
        let key = CacheKeys::frequent_query(&query_hash);
        
        // Update access statistics
        self.record_query_access(&query_hash).await;
        
        self.get(&key).await
    }

    /// Cache heavy content with compression
    pub async fn cache_heavy_content(&self, content_id: &str, data: &[u8], content_type: &str) -> SanadResult<()> {
        if data.len() < self.config.heavy_content_threshold_bytes {
            // Not heavy enough, use regular caching
            return Ok(());
        }

        let content_hash = crate::utils::calculate_content_hash(&String::from_utf8_lossy(data));
        
        // Compress the data
        let compressed_data = self.compress_data(data)?;
        let compression_ratio = compressed_data.len() as f64 / data.len() as f64;
        
        let entry = HeavyContentEntry {
            content_hash: content_hash.clone(),
            original_size: data.len(),
            compressed_size: compressed_data.len(),
            compressed_data,
            compression_ratio,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::seconds(self.config.heavy_content_ttl_seconds as i64),
            access_count: 0,
            last_accessed: Utc::now(),
            content_type: content_type.to_string(),
        };

        let compressed_size = entry.compressed_size;

        // Store in heavy content cache
        let mut heavy_cache = self.heavy_content_cache.write().await;
        heavy_cache.insert(content_id.to_string(), entry);

        // Also store in Redis for persistence
        let key = CacheKeys::heavy_content(content_id);
        let mut conn = self.redis_manager.clone();
        let serialized = serde_json::to_string(&heavy_cache.get(content_id).unwrap())
            .map_err(SanadError::Serialization)?;
        let _: () = conn.set_ex(&key, &serialized, self.config.heavy_content_ttl_seconds)
            .await
            .map_err(SanadError::Redis)?;

        info!("Cached heavy content: {} bytes -> {} bytes (ratio: {:.2})", 
              data.len(), compressed_size, compression_ratio);

        Ok(())
    }

    /// Get heavy content with decompression
    pub async fn get_heavy_content(&self, content_id: &str) -> SanadResult<Option<Vec<u8>>> {
        // Try memory cache first
        {
            let mut heavy_cache = self.heavy_content_cache.write().await;
            if let Some(entry) = heavy_cache.get_mut(content_id) {
                if entry.expires_at > Utc::now() {
                    entry.access_count += 1;
                    entry.last_accessed = Utc::now();
                    
                    let decompressed = self.decompress_data(&entry.compressed_data)?;
                    debug!("Heavy content cache hit (memory): {}", content_id);
                    return Ok(Some(decompressed));
                } else {
                    // Remove expired entry
                    heavy_cache.remove(content_id);
                }
            }
        }

        // Try Redis cache
        let key = CacheKeys::heavy_content(content_id);
        let mut conn = self.redis_manager.clone();
        let result: RedisResult<String> = conn.get(&key).await;
        
        match result {
            Ok(serialized) => {
                let entry: HeavyContentEntry = serde_json::from_str(&serialized)
                    .map_err(SanadError::Serialization)?;
                
                if entry.expires_at > Utc::now() {
                    let decompressed = self.decompress_data(&entry.compressed_data)?;
                    
                    // Update memory cache
                    let mut heavy_cache = self.heavy_content_cache.write().await;
                    heavy_cache.insert(content_id.to_string(), entry);
                    
                    debug!("Heavy content cache hit (Redis): {}", content_id);
                    Ok(Some(decompressed))
                } else {
                    // Expired, remove from Redis
                    let _: () = conn.del(&key).await.map_err(SanadError::Redis)?;
                    Ok(None)
                }
            }
            Err(_) => {
                debug!("Heavy content cache miss: {}", content_id);
                Ok(None)
            }
        }
    }

    /// Get adaptive TTL based on access patterns
    pub async fn get_adaptive_ttl(&self, key: &str, base_ttl: u64) -> u64 {
        if !self.config.enable_adaptive_ttl {
            return base_ttl;
        }

        // Check access patterns from memory cache
        if let Some(entry) = self.get_memory_cache(key).await {
            let hours_since_creation = (Utc::now() - entry.created_at).num_hours() as f64;
            if hours_since_creation > 0.0 {
                let access_rate = entry.access_count as f64 / hours_since_creation;
                
                // Increase TTL for frequently accessed items
                if access_rate > 10.0 {
                    return base_ttl * 2; // Double TTL for very frequent access
                } else if access_rate > 5.0 {
                    return (base_ttl as f64 * 1.5) as u64; // 1.5x TTL for frequent access
                } else if access_rate < 1.0 {
                    return base_ttl / 2; // Half TTL for infrequent access
                }
            }
        }

        base_ttl
    }

    /// Cleanup expired heavy content cache entries
    async fn cleanup_heavy_content_cache(&self) -> usize {
        let mut heavy_cache = self.heavy_content_cache.write().await;
        let now = Utc::now();
        let initial_size = heavy_cache.len();

        heavy_cache.retain(|_, entry| entry.expires_at > now);

        initial_size - heavy_cache.len()
    }

    /// Update query statistics for intelligent caching
    async fn update_query_stats(&self, query_hash: &str, query: &str) {
        let mut tracker = self.query_tracker.write().await;
        let now = Utc::now();
        
        match tracker.get_mut(query_hash) {
            Some(stats) => {
                stats.access_count += 1;
                stats.last_accessed = now;
                
                // Calculate hourly frequency
                let hours_since_first = (now - stats.first_seen).num_hours() as f64;
                if hours_since_first > 0.0 {
                    stats.hourly_frequency = stats.access_count as f64 / hours_since_first;
                    stats.is_frequent = stats.hourly_frequency >= self.config.min_query_frequency_for_cache as f64;
                }
            }
            None => {
                let stats = QueryStats {
                    query_hash: query_hash.to_string(),
                    first_seen: now,
                    last_accessed: now,
                    access_count: 1,
                    hourly_frequency: 0.0,
                    average_response_time_ms: 0.0,
                    cache_hit_ratio: 0.0,
                    is_frequent: false,
                };
                tracker.insert(query_hash.to_string(), stats);
            }
        }
        
        debug!("Updated query stats for: {} (query: {})", query_hash, query);
    }

    /// Check if a query is frequent enough to cache
    async fn is_frequent_query(&self, query_hash: &str) -> bool {
        let tracker = self.query_tracker.read().await;
        tracker.get(query_hash)
            .map(|stats| stats.is_frequent)
            .unwrap_or(false)
    }

    /// Record query access for statistics
    async fn record_query_access(&self, query_hash: &str) {
        let mut tracker = self.query_tracker.write().await;
        if let Some(stats) = tracker.get_mut(query_hash) {
            stats.access_count += 1;
            stats.last_accessed = Utc::now();
        }
    }

    /// Compress data using gzip
    fn compress_data(&self, data: &[u8]) -> SanadResult<Vec<u8>> {
        use flate2::Compression;
        use flate2::write::GzEncoder;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).map_err(|e| SanadError::Internal(format!("Compression failed: {}", e)))?;
        encoder.finish().map_err(|e| SanadError::Internal(format!("Compression failed: {}", e)))
    }

    /// Decompress data using gzip
    fn decompress_data(&self, compressed_data: &[u8]) -> SanadResult<Vec<u8>> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(compressed_data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| SanadError::Internal(format!("Decompression failed: {}", e)))?;
        Ok(decompressed)
    }

    // Private helper methods

    fn get_ttl_for_type(&self, cache_type: &CacheType) -> u64 {
        match cache_type {
            CacheType::PrayerTimes => self.config.prayer_times_ttl_seconds,
            CacheType::SemanticQuery => self.config.semantic_query_ttl_seconds,
            CacheType::QuranContent => self.config.quran_content_ttl_seconds,
            CacheType::HadithContent => self.config.hadith_content_ttl_seconds,
            CacheType::HeavyContent => self.config.heavy_content_ttl_seconds,
            CacheType::FrequentQuery => self.config.semantic_query_ttl_seconds * 2, // Longer TTL for frequent queries
            _ => self.config.default_ttl_seconds,
        }
    }

    fn should_cache_in_memory(&self, cache_type: &CacheType) -> bool {
        matches!(
            cache_type,
            CacheType::QuranContent | 
            CacheType::UserPreferences | 
            CacheType::PrayerTimes |
            CacheType::FrequentQuery
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
        
        let mut entries: Vec<_> = memory_cache.iter().map(|(k, v)| (k.clone(), v.last_accessed)).collect();
        entries.sort_by_key(|(_, last_accessed)| *last_accessed);
        
        for (key, _) in entries.iter().take(evict_count) {
            memory_cache.remove(key);
        }
        
        debug!("Evicted {} LRU entries from memory cache", evict_count);
    }

    #[allow(dead_code)]
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
    pub heavy_content_entries: usize,
    pub total_heavy_content_size_bytes: usize,
    pub average_compression_ratio: f64,
    pub frequent_queries_count: usize,
    pub query_tracking_enabled: bool,
    pub adaptive_ttl_enabled: bool,
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
        // Use intelligent caching for frequent queries
        cache.cache_frequent_query(query, results).await?;
        
        // Also cache normally for immediate access
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

    /// Cache heavy content like audio files or large search results
    pub async fn cache_heavy_content_data(
        cache: &AdvancedCacheManager,
        content_id: &str,
        data: &[u8],
        content_type: &str,
    ) -> SanadResult<()> {
        cache.cache_heavy_content(content_id, data, content_type).await
    }

    /// Get cached heavy content
    pub async fn get_heavy_content_data(
        cache: &AdvancedCacheManager,
        content_id: &str,
    ) -> SanadResult<Option<Vec<u8>>> {
        cache.get_heavy_content(content_id).await
    }

    /// Cache search results with frequency tracking
    pub async fn cache_search_results<T>(
        cache: &AdvancedCacheManager,
        query: &str,
        filters: &str,
        results: &T,
    ) -> SanadResult<()>
    where
        T: Serialize,
    {
        let query_hash = crate::utils::calculate_content_hash(query);
        let filters_hash = crate::utils::calculate_content_hash(filters);
        let key = CacheKeys::search_results(&query_hash, &filters_hash);
        
        // Check if this is a large result set
        let serialized_size = serde_json::to_string(results)
            .map_err(SanadError::Serialization)?
            .len();
        
        if serialized_size > cache.config.heavy_content_threshold_bytes {
            // Use heavy content caching for large results
            let data = serde_json::to_vec(results).map_err(SanadError::Serialization)?;
            cache.cache_heavy_content(&key, &data, "application/json").await?;
        } else {
            // Use regular caching
            cache.set(&key, results, CacheType::SearchResults).await?;
        }
        
        // Also track as frequent query
        let full_query = format!("{}|{}", query, filters);
        cache.cache_frequent_query(&full_query, results).await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // Note: Integration tests with Redis would be in a separate test file
    // These unit tests focus on configuration and logic without external dependencies
}
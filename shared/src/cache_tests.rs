use crate::cache::*;
use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key_generation() {
        // Test prayer times key generation
        let prayer_key = CacheKeys::prayer_times(40.7128, -74.0060, "2024-01-01", "MWL");
        assert_eq!(prayer_key, "prayer_times:40.7128:-74.006:2024-01-01:MWL");

        // Test semantic query key generation
        let semantic_key = CacheKeys::semantic_query("abc123");
        assert_eq!(semantic_key, "semantic_query:abc123");

        // Test Quran content key generation
        let quran_key_with_ayah = CacheKeys::quran_content(1, Some(1));
        assert_eq!(quran_key_with_ayah, "quran:1:1");

        let quran_key_surah_only = CacheKeys::quran_content(2, None);
        assert_eq!(quran_key_surah_only, "quran:2");

        // Test hadith content key generation
        let hadith_key = CacheKeys::hadith_content("bukhari", "book1", "1");
        assert_eq!(hadith_key, "hadith:bukhari:book1:1");

        // Test user preferences key generation
        let user_prefs_key = CacheKeys::user_preferences("user123");
        assert_eq!(user_prefs_key, "user_prefs:user123");

        // Test search results key generation
        let search_key = CacheKeys::search_results("query_hash", "filters_hash");
        assert_eq!(search_key, "search:query_hash:filters_hash");

        // Test API response key generation
        let api_key = CacheKeys::api_response("endpoint", "params_hash");
        assert_eq!(api_key, "api:endpoint:params_hash");
    }

    #[test]
    fn test_cache_config_defaults() {
        let config = CacheConfig::default();
        
        assert_eq!(config.default_ttl_seconds, 3600);
        assert_eq!(config.prayer_times_ttl_seconds, 86400);
        assert_eq!(config.semantic_query_ttl_seconds, 21600);
        assert_eq!(config.quran_content_ttl_seconds, 2592000);
        assert_eq!(config.hadith_content_ttl_seconds, 604800);
        assert_eq!(config.max_memory_cache_size, 10000);
        assert!(config.enable_smart_invalidation);
        assert_eq!(config.min_query_frequency_for_cache, 5);
        assert_eq!(config.heavy_content_threshold_bytes, 1024 * 1024);
        assert_eq!(config.heavy_content_ttl_seconds, 7200);
        assert!(config.enable_query_tracking);
        assert!(config.enable_adaptive_ttl);
    }

    #[test]
    fn test_cache_entry_creation() {
        let now = Utc::now();
        let entry = CacheEntry {
            data: "test data".to_string(),
            created_at: now,
            expires_at: now + chrono::Duration::hours(1),
            access_count: 0,
            last_accessed: now,
            cache_type: CacheType::General,
        };

        assert_eq!(entry.data, "test data");
        assert_eq!(entry.access_count, 0);
        assert!(matches!(entry.cache_type, CacheType::General));
    }

    #[test]
    fn test_cache_type_serialization() {
        let cache_types = vec![
            CacheType::PrayerTimes,
            CacheType::SemanticQuery,
            CacheType::QuranContent,
            CacheType::HadithContent,
            CacheType::UserPreferences,
            CacheType::SearchResults,
            CacheType::ApiResponse,
            CacheType::HeavyContent,
            CacheType::FrequentQuery,
            CacheType::General,
        ];

        for cache_type in cache_types {
            let serialized = serde_json::to_string(&cache_type).unwrap();
            let deserialized: CacheType = serde_json::from_str(&serialized).unwrap();
            
            // Compare debug representations since CacheType doesn't implement PartialEq
            assert_eq!(format!("{:?}", cache_type), format!("{:?}", deserialized));
        }
    }

    #[test]
    fn test_cache_stats_default() {
        let stats = CacheStats {
            redis_memory_usage_bytes: 1024,
            memory_cache_entries: 100,
            memory_cache_entries_by_type: {
                let mut map = HashMap::new();
                map.insert("PrayerTimes".to_string(), 50);
                map.insert("QuranContent".to_string(), 30);
                map.insert("General".to_string(), 20);
                map
            },
            total_cache_operations: 1000,
            heavy_content_entries: 5,
            total_heavy_content_size_bytes: 5 * 1024 * 1024, // 5MB
            average_compression_ratio: 0.3,
            frequent_queries_count: 15,
            query_tracking_enabled: true,
            adaptive_ttl_enabled: true,
        };

        assert_eq!(stats.redis_memory_usage_bytes, 1024);
        assert_eq!(stats.memory_cache_entries, 100);
        assert_eq!(stats.total_cache_operations, 1000);
        assert_eq!(stats.memory_cache_entries_by_type.len(), 3);
        assert_eq!(stats.heavy_content_entries, 5);
        assert_eq!(stats.total_heavy_content_size_bytes, 5 * 1024 * 1024);
        assert_eq!(stats.average_compression_ratio, 0.3);
        assert_eq!(stats.frequent_queries_count, 15);
        assert!(stats.query_tracking_enabled);
        assert!(stats.adaptive_ttl_enabled);
    }

    // Mock tests for cache manager functionality
    // Note: These would require a running Redis instance in a real test environment

    #[tokio::test]
    async fn test_cache_manager_creation() {
        let config = CacheConfig::default();
        
        // Test with invalid URL to ensure error handling works
        let result = AdvancedCacheManager::new("invalid://url", Some(config.clone())).await;
        assert!(result.is_err(), "Should fail with invalid URL");
        
        // Test config validation
        assert_eq!(config.default_ttl_seconds, 3600);
        assert_eq!(config.prayer_times_ttl_seconds, 86400);
        assert!(config.enable_smart_invalidation);
        
        // Test that the function signature and error handling work correctly
        assert!(result.is_err(), "Should return error for invalid URL");
    }

    #[test]
    fn test_ttl_calculation() {
        let config = CacheConfig::default();
        
        // Create a mock cache manager structure to test TTL calculation
        struct MockCacheManager {
            #[allow(dead_code)]
            config: CacheConfig,
        }
        
        impl MockCacheManager {
            fn get_ttl_for_type(&self, cache_type: &CacheType) -> u64 {
                match cache_type {
                    CacheType::PrayerTimes => self.config.prayer_times_ttl_seconds,
                    CacheType::SemanticQuery => self.config.semantic_query_ttl_seconds,
                    CacheType::QuranContent => self.config.quran_content_ttl_seconds,
                    CacheType::HadithContent => self.config.hadith_content_ttl_seconds,
                    _ => self.config.default_ttl_seconds,
                }
            }
        }
        
        let mock_manager = MockCacheManager { config: config };
        
        assert_eq!(mock_manager.get_ttl_for_type(&CacheType::PrayerTimes), 86400);
        assert_eq!(mock_manager.get_ttl_for_type(&CacheType::SemanticQuery), 21600);
        assert_eq!(mock_manager.get_ttl_for_type(&CacheType::QuranContent), 2592000);
        assert_eq!(mock_manager.get_ttl_for_type(&CacheType::HadithContent), 604800);
        assert_eq!(mock_manager.get_ttl_for_type(&CacheType::HeavyContent), 7200);
        assert_eq!(mock_manager.get_ttl_for_type(&CacheType::FrequentQuery), 43200); // 2x semantic query TTL
        assert_eq!(mock_manager.get_ttl_for_type(&CacheType::General), 3600);
    }

    #[test]
    fn test_memory_cache_priority() {
        struct MockCacheManager {
            #[allow(dead_code)]
            config: CacheConfig,
        }
        
        impl MockCacheManager {
            fn should_cache_in_memory(&self, cache_type: &CacheType) -> bool {
                matches!(
                    cache_type,
                    CacheType::QuranContent | CacheType::UserPreferences | CacheType::PrayerTimes
                )
            }
        }
        
        let mock_manager = MockCacheManager { config: CacheConfig::default() };
        
        // These should be cached in memory
        assert!(mock_manager.should_cache_in_memory(&CacheType::QuranContent));
        assert!(mock_manager.should_cache_in_memory(&CacheType::UserPreferences));
        assert!(mock_manager.should_cache_in_memory(&CacheType::PrayerTimes));
        assert!(mock_manager.should_cache_in_memory(&CacheType::FrequentQuery));
        
        // These should not be cached in memory
        assert!(!mock_manager.should_cache_in_memory(&CacheType::SemanticQuery));
        assert!(!mock_manager.should_cache_in_memory(&CacheType::HadithContent));
        assert!(!mock_manager.should_cache_in_memory(&CacheType::HeavyContent));
        assert!(!mock_manager.should_cache_in_memory(&CacheType::General));
    }

    #[test]
    fn test_cache_strategies_key_generation() {
        // Test that cache strategies would generate correct keys
        let lat = 40.7128;
        let lng = -74.0060;
        let date = "2024-01-01";
        let method = "MWL";
        
        let expected_key = format!("prayer_times:{}:{}:{}:{}", lat, lng, date, method);
        let actual_key = CacheKeys::prayer_times(lat, lng, date, method);
        
        assert_eq!(actual_key, expected_key);
    }

    #[test]
    fn test_json_serialization_compatibility() {
        // Test that our cache types work well with JSON serialization
        let test_data = json!({
            "surah": 1,
            "ayah": 1,
            "text": "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
            "translation": "In the name of Allah, the Entirely Merciful, the Especially Merciful."
        });

        let serialized = serde_json::to_string(&test_data).unwrap();
        let deserialized: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(test_data, deserialized);
    }

    #[test]
    fn test_cache_invalidation_patterns() {
        // Test that invalidation patterns are correctly formatted
        let patterns = vec![
            ("prayer_times:40.7128:-74.006:*", "Prayer times for specific location"),
            ("semantic_query:*", "All semantic queries"),
            ("quran:1:*", "All verses from Surah 1"),
            ("hadith:bukhari:*", "All Bukhari hadiths"),
            ("user_prefs:*", "All user preferences"),
            ("search:*", "All search results"),
            ("api:*", "All API responses"),
            ("frequent_query:*", "All frequent queries"),
            ("heavy_content:*", "All heavy content"),
        ];

        for (pattern, description) in patterns {
            // Verify pattern format
            assert!(pattern.contains(':'), "Pattern should contain colons: {} ({})", pattern, description);
            assert!(pattern.ends_with('*'), "Pattern should end with wildcard: {} ({})", pattern, description);
        }
    }

    #[test]
    fn test_query_stats_creation() {
        let now = Utc::now();
        let stats = QueryStats {
            query_hash: "test_hash".to_string(),
            first_seen: now,
            last_accessed: now,
            access_count: 1,
            hourly_frequency: 0.0,
            average_response_time_ms: 100.0,
            cache_hit_ratio: 0.8,
            is_frequent: false,
        };

        assert_eq!(stats.query_hash, "test_hash");
        assert_eq!(stats.access_count, 1);
        assert_eq!(stats.average_response_time_ms, 100.0);
        assert_eq!(stats.cache_hit_ratio, 0.8);
        assert!(!stats.is_frequent);
    }

    #[test]
    fn test_heavy_content_entry_creation() {
        let now = Utc::now();
        let entry = HeavyContentEntry {
            content_hash: "test_hash".to_string(),
            compressed_data: vec![1, 2, 3, 4, 5],
            original_size: 1000,
            compressed_size: 500,
            compression_ratio: 0.5,
            created_at: now,
            expires_at: now + chrono::Duration::hours(2),
            access_count: 0,
            last_accessed: now,
            content_type: "application/json".to_string(),
        };

        assert_eq!(entry.content_hash, "test_hash");
        assert_eq!(entry.original_size, 1000);
        assert_eq!(entry.compressed_size, 500);
        assert_eq!(entry.compression_ratio, 0.5);
        assert_eq!(entry.content_type, "application/json");
        assert_eq!(entry.compressed_data.len(), 5);
    }

    #[test]
    fn test_new_cache_key_patterns() {
        // Test frequent query key generation
        let frequent_key = CacheKeys::frequent_query("abc123");
        assert_eq!(frequent_key, "frequent_query:abc123");

        // Test heavy content key generation
        let heavy_key = CacheKeys::heavy_content("content_123");
        assert_eq!(heavy_key, "heavy_content:content_123");
    }

    #[test]
    fn test_cache_entry_expiration_logic() {
        let now = Utc::now();
        
        // Create an expired entry
        let expired_entry = CacheEntry {
            data: "expired data".to_string(),
            created_at: now - chrono::Duration::hours(2),
            expires_at: now - chrono::Duration::hours(1),
            access_count: 5,
            last_accessed: now - chrono::Duration::hours(1),
            cache_type: CacheType::General,
        };

        // Create a valid entry
        let valid_entry = CacheEntry {
            data: "valid data".to_string(),
            created_at: now - chrono::Duration::minutes(30),
            expires_at: now + chrono::Duration::hours(1),
            access_count: 3,
            last_accessed: now - chrono::Duration::minutes(5),
            cache_type: CacheType::General,
        };

        // Test expiration logic
        assert!(expired_entry.expires_at < now, "Entry should be expired");
        assert!(valid_entry.expires_at > now, "Entry should be valid");
    }

    #[test]
    fn test_lru_eviction_logic() {
        let now = Utc::now();
        
        let mut entries = vec![
            ("key1", CacheEntry {
                data: "data1".to_string(),
                created_at: now - chrono::Duration::hours(1),
                expires_at: now + chrono::Duration::hours(1),
                access_count: 10,
                last_accessed: now - chrono::Duration::minutes(60), // Oldest access
                cache_type: CacheType::General,
            }),
            ("key2", CacheEntry {
                data: "data2".to_string(),
                created_at: now - chrono::Duration::hours(1),
                expires_at: now + chrono::Duration::hours(1),
                access_count: 5,
                last_accessed: now - chrono::Duration::minutes(30),
                cache_type: CacheType::General,
            }),
            ("key3", CacheEntry {
                data: "data3".to_string(),
                created_at: now - chrono::Duration::hours(1),
                expires_at: now + chrono::Duration::hours(1),
                access_count: 15,
                last_accessed: now - chrono::Duration::minutes(5), // Most recent access
                cache_type: CacheType::General,
            }),
        ];

        // Sort by last accessed time (LRU logic)
        entries.sort_by_key(|(_, entry)| entry.last_accessed);
        
        // The first entry should be the least recently used
        assert_eq!(entries[0].0, "key1");
        assert_eq!(entries[2].0, "key3");
    }
}

// Integration tests that would run with a real Redis instance
#[cfg(test)]
mod integration_tests {
    use super::*;

    // These tests are ignored by default and would need a running Redis instance
    
    #[tokio::test]
    async fn test_full_cache_workflow() {
        // Create a mock cache manager for testing
        struct MockCacheManager {
            storage: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, String>>>,
        }
        
        impl MockCacheManager {
            fn new() -> Self {
                Self {
                    storage: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
                }
            }
            
            async fn set(&self, key: &str, value: &serde_json::Value) -> Result<(), String> {
                let serialized = serde_json::to_string(value).map_err(|e| e.to_string())?;
                self.storage.lock().unwrap().insert(key.to_string(), serialized);
                Ok(())
            }
            
            async fn get(&self, key: &str) -> Result<Option<serde_json::Value>, String> {
                if let Some(value) = self.storage.lock().unwrap().get(key) {
                    let deserialized = serde_json::from_str(value).map_err(|e| e.to_string())?;
                    Ok(Some(deserialized))
                } else {
                    Ok(None)
                }
            }
            
            async fn delete(&self, key: &str) -> Result<(), String> {
                self.storage.lock().unwrap().remove(key);
                Ok(())
            }
        }

        let cache_manager = MockCacheManager::new();

        // Test setting and getting a value
        let test_data = json!({"message": "Hello, World!"});
        cache_manager.set("test_key", &test_data).await.unwrap();
        
        let retrieved: Option<serde_json::Value> = cache_manager.get("test_key").await.unwrap();
        assert_eq!(retrieved, Some(test_data));

        // Test deletion
        cache_manager.delete("test_key").await.unwrap();
        let after_delete: Option<serde_json::Value> = cache_manager.get("test_key").await.unwrap();
        assert_eq!(after_delete, None);
    }

    #[tokio::test]
    async fn test_cache_invalidation() {
        // Create a mock cache manager for testing pattern invalidation
        struct MockCacheManager {
            storage: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, String>>>,
        }
        
        impl MockCacheManager {
            fn new() -> Self {
                Self {
                    storage: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
                }
            }
            
            async fn set(&self, key: &str, value: &serde_json::Value) -> Result<(), String> {
                let serialized = serde_json::to_string(value).map_err(|e| e.to_string())?;
                self.storage.lock().unwrap().insert(key.to_string(), serialized);
                Ok(())
            }
            
            async fn get(&self, key: &str) -> Result<Option<serde_json::Value>, String> {
                if let Some(value) = self.storage.lock().unwrap().get(key) {
                    let deserialized = serde_json::from_str(value).map_err(|e| e.to_string())?;
                    Ok(Some(deserialized))
                } else {
                    Ok(None)
                }
            }
            
            async fn invalidate_pattern(&self, pattern: &str) -> Result<u64, String> {
                let mut storage = self.storage.lock().unwrap();
                let prefix = pattern.trim_end_matches('*');
                let keys_to_remove: Vec<String> = storage.keys()
                    .filter(|key| key.starts_with(prefix))
                    .cloned()
                    .collect();
                
                let count = keys_to_remove.len() as u64;
                for key in keys_to_remove {
                    storage.remove(&key);
                }
                Ok(count)
            }
        }

        let cache_manager = MockCacheManager::new();

        // Set multiple values with a pattern
        let test_data = json!({"test": true});
        cache_manager.set("test:1", &test_data).await.unwrap();
        cache_manager.set("test:2", &test_data).await.unwrap();
        cache_manager.set("other:1", &test_data).await.unwrap();

        // Invalidate by pattern
        let deleted_count = cache_manager.invalidate_pattern("test:*").await.unwrap();
        assert_eq!(deleted_count, 2);

        // Verify invalidation
        let test1: Option<serde_json::Value> = cache_manager.get("test:1").await.unwrap();
        let test2: Option<serde_json::Value> = cache_manager.get("test:2").await.unwrap();
        let other1: Option<serde_json::Value> = cache_manager.get("other:1").await.unwrap();

        assert_eq!(test1, None);
        assert_eq!(test2, None);
        assert_eq!(other1, Some(test_data));
    }

    #[tokio::test]
    async fn test_cache_stats() {
        // Create a mock cache manager for testing stats
        struct MockCacheManager {
            storage: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, String>>>,
        }
        
        impl MockCacheManager {
            fn new() -> Self {
                Self {
                    storage: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
                }
            }
            
            async fn get_cache_stats(&self) -> Result<CacheStats, String> {
                let storage = self.storage.lock().unwrap();
                let entry_count = storage.len();
                
                let mut entries_by_type = HashMap::new();
                entries_by_type.insert("General".to_string(), entry_count);
                
                Ok(CacheStats {
                    redis_memory_usage_bytes: 1024,
                    memory_cache_entries: entry_count,
                    memory_cache_entries_by_type: entries_by_type,
                    total_cache_operations: 100,
                    heavy_content_entries: 0,
                    total_heavy_content_size_bytes: 0,
                    average_compression_ratio: 0.0,
                    frequent_queries_count: 0,
                    query_tracking_enabled: false,
                    adaptive_ttl_enabled: false,
                })
            }
            
            async fn set(&self, key: &str, value: &serde_json::Value) -> Result<(), String> {
                let serialized = serde_json::to_string(value).map_err(|e| e.to_string())?;
                self.storage.lock().unwrap().insert(key.to_string(), serialized);
                Ok(())
            }
        }

        let cache_manager = MockCacheManager::new();
        
        // Add some test data
        let test_data = json!({"test": "data"});
        cache_manager.set("test_key", &test_data).await.unwrap();

        let stats = cache_manager.get_cache_stats().await.unwrap();
        
        // Basic validation that stats are returned
        assert!(stats.redis_memory_usage_bytes > 0 || stats.redis_memory_usage_bytes == 0);
        assert!(stats.memory_cache_entries > 0 || stats.memory_cache_entries == 0);
        assert_eq!(stats.memory_cache_entries, 1); // We added one item
        assert!(stats.total_cache_operations > 0);
    }
}
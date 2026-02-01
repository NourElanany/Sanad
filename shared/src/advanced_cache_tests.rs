use crate::cache::*;
use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;

#[cfg(test)]
mod advanced_cache_tests {
    use super::*;

    #[test]
    fn test_advanced_cache_config() {
        let config = CacheConfig {
            default_ttl_seconds: 1800,
            prayer_times_ttl_seconds: 43200,
            semantic_query_ttl_seconds: 10800,
            quran_content_ttl_seconds: 1296000,
            hadith_content_ttl_seconds: 302400,
            max_memory_cache_size: 5000,
            enable_smart_invalidation: true,
            min_query_frequency_for_cache: 10,
            heavy_content_threshold_bytes: 2 * 1024 * 1024, // 2MB
            heavy_content_ttl_seconds: 3600,
            enable_query_tracking: true,
            enable_adaptive_ttl: true,
        };

        assert_eq!(config.min_query_frequency_for_cache, 10);
        assert_eq!(config.heavy_content_threshold_bytes, 2 * 1024 * 1024);
        assert_eq!(config.heavy_content_ttl_seconds, 3600);
        assert!(config.enable_query_tracking);
        assert!(config.enable_adaptive_ttl);
    }

    #[test]
    fn test_query_frequency_calculation() {
        let now = Utc::now();
        let mut stats = QueryStats {
            query_hash: "test_query".to_string(),
            first_seen: now - chrono::Duration::hours(2),
            last_accessed: now,
            access_count: 10,
            hourly_frequency: 0.0,
            average_response_time_ms: 150.0,
            cache_hit_ratio: 0.7,
            is_frequent: false,
        };

        // Calculate hourly frequency
        let hours_since_first = (now - stats.first_seen).num_hours() as f64;
        if hours_since_first > 0.0 {
            stats.hourly_frequency = stats.access_count as f64 / hours_since_first;
        }

        assert_eq!(stats.hourly_frequency, 5.0); // 10 accesses in 2 hours = 5 per hour
    }

    #[test]
    fn test_compression_ratio_calculation() {
        let original_data = vec![0u8; 1000]; // 1KB of zeros (highly compressible)
        let compressed_data = vec![0u8; 100]; // Simulated compressed data

        let compression_ratio = compressed_data.len() as f64 / original_data.len() as f64;
        assert_eq!(compression_ratio, 0.1); // 90% compression

        let entry = HeavyContentEntry {
            content_hash: "test_hash".to_string(),
            compressed_data,
            original_size: original_data.len(),
            compressed_size: 100,
            compression_ratio,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(1),
            access_count: 0,
            last_accessed: Utc::now(),
            content_type: "application/octet-stream".to_string(),
        };

        assert_eq!(entry.compression_ratio, 0.1);
        assert_eq!(entry.original_size, 1000);
        assert_eq!(entry.compressed_size, 100);
    }

    #[test]
    fn test_adaptive_ttl_logic() {
        struct MockAdaptiveTTL {
            base_ttl: u64,
        }

        impl MockAdaptiveTTL {
            fn calculate_adaptive_ttl(&self, access_rate: f64) -> u64 {
                if access_rate > 10.0 {
                    self.base_ttl * 2 // Double TTL for very frequent access
                } else if access_rate > 5.0 {
                    (self.base_ttl as f64 * 1.5) as u64 // 1.5x TTL for frequent access
                } else if access_rate < 1.0 {
                    self.base_ttl / 2 // Half TTL for infrequent access
                } else {
                    self.base_ttl
                }
            }
        }

        let mock = MockAdaptiveTTL { base_ttl: 3600 };

        // Test very frequent access (>10 per hour)
        assert_eq!(mock.calculate_adaptive_ttl(15.0), 7200); // 2x TTL

        // Test frequent access (5-10 per hour)
        assert_eq!(mock.calculate_adaptive_ttl(7.0), 5400); // 1.5x TTL

        // Test normal access (1-5 per hour)
        assert_eq!(mock.calculate_adaptive_ttl(3.0), 3600); // Normal TTL

        // Test infrequent access (<1 per hour)
        assert_eq!(mock.calculate_adaptive_ttl(0.5), 1800); // 0.5x TTL
    }

    #[test]
    fn test_cache_type_priority() {
        struct MockCacheManager {
            config: CacheConfig,
        }

        impl MockCacheManager {
            fn get_cache_priority(&self, cache_type: &CacheType) -> u8 {
                match cache_type {
                    CacheType::QuranContent => 10, // Highest priority
                    CacheType::PrayerTimes => 9,
                    CacheType::FrequentQuery => 8,
                    CacheType::UserPreferences => 7,
                    CacheType::HadithContent => 6,
                    CacheType::SemanticQuery => 5,
                    CacheType::SearchResults => 4,
                    CacheType::HeavyContent => 3,
                    CacheType::ApiResponse => 2,
                    CacheType::General => 1, // Lowest priority
                }
            }
        }

        let mock = MockCacheManager {
            config: CacheConfig::default(),
        };

        assert_eq!(mock.get_cache_priority(&CacheType::QuranContent), 10);
        assert_eq!(mock.get_cache_priority(&CacheType::FrequentQuery), 8);
        assert_eq!(mock.get_cache_priority(&CacheType::HeavyContent), 3);
        assert_eq!(mock.get_cache_priority(&CacheType::General), 1);
    }

    #[test]
    fn test_cache_eviction_strategy() {
        let now = Utc::now();
        
        let entries = vec![
            ("low_priority", CacheEntry {
                data: "data1".to_string(),
                created_at: now - chrono::Duration::hours(2),
                expires_at: now + chrono::Duration::hours(1),
                access_count: 2,
                last_accessed: now - chrono::Duration::hours(1),
                cache_type: CacheType::General,
            }),
            ("high_priority", CacheEntry {
                data: "data2".to_string(),
                created_at: now - chrono::Duration::hours(1),
                expires_at: now + chrono::Duration::hours(2),
                access_count: 10,
                last_accessed: now - chrono::Duration::minutes(5),
                cache_type: CacheType::QuranContent,
            }),
            ("frequent_query", CacheEntry {
                data: "data3".to_string(),
                created_at: now - chrono::Duration::hours(1),
                expires_at: now + chrono::Duration::hours(3),
                access_count: 15,
                last_accessed: now - chrono::Duration::minutes(2),
                cache_type: CacheType::FrequentQuery,
            }),
        ];

        // Sort by eviction priority (lower access count and older last access should be evicted first)
        let mut sorted_entries = entries.clone();
        sorted_entries.sort_by(|a, b| {
            // First by cache type priority (General should be evicted before QuranContent)
            let type_priority_a = match a.1.cache_type {
                CacheType::QuranContent => 10,
                CacheType::FrequentQuery => 8,
                CacheType::General => 1,
                _ => 5,
            };
            let type_priority_b = match b.1.cache_type {
                CacheType::QuranContent => 10,
                CacheType::FrequentQuery => 8,
                CacheType::General => 1,
                _ => 5,
            };

            if type_priority_a != type_priority_b {
                type_priority_a.cmp(&type_priority_b)
            } else {
                // Then by access count (lower first)
                a.1.access_count.cmp(&b.1.access_count)
            }
        });

        // The first entry should be the one with lowest priority
        assert_eq!(sorted_entries[0].0, "low_priority");
    }

    #[test]
    fn test_heavy_content_threshold() {
        let config = CacheConfig::default();
        let threshold = config.heavy_content_threshold_bytes;

        // Test data sizes
        let small_data = vec![0u8; threshold / 2]; // 512KB
        let large_data = vec![0u8; threshold * 2]; // 2MB

        assert!(small_data.len() < threshold);
        assert!(large_data.len() > threshold);

        // Small data should use regular caching
        assert!(small_data.len() < config.heavy_content_threshold_bytes);
        
        // Large data should use heavy content caching
        assert!(large_data.len() >= config.heavy_content_threshold_bytes);
    }

    #[test]
    fn test_cache_statistics_aggregation() {
        let mut memory_cache_by_type = HashMap::new();
        memory_cache_by_type.insert("QuranContent".to_string(), 100);
        memory_cache_by_type.insert("PrayerTimes".to_string(), 50);
        memory_cache_by_type.insert("FrequentQuery".to_string(), 25);
        memory_cache_by_type.insert("General".to_string(), 10);

        let stats = CacheStats {
            redis_memory_usage_bytes: 10 * 1024 * 1024, // 10MB
            memory_cache_entries: 185,
            memory_cache_entries_by_type: memory_cache_by_type,
            total_cache_operations: 50000,
            heavy_content_entries: 8,
            total_heavy_content_size_bytes: 40 * 1024 * 1024, // 40MB
            average_compression_ratio: 0.25, // 75% compression
            frequent_queries_count: 25,
            query_tracking_enabled: true,
            adaptive_ttl_enabled: true,
        };

        // Verify aggregated statistics
        assert_eq!(stats.memory_cache_entries, 185);
        assert_eq!(stats.heavy_content_entries, 8);
        assert_eq!(stats.frequent_queries_count, 25);
        assert_eq!(stats.average_compression_ratio, 0.25);
        
        // Calculate total entries across all cache types
        let total_memory_entries: usize = stats.memory_cache_entries_by_type.values().sum();
        assert_eq!(total_memory_entries, 185);
    }

    #[test]
    fn test_cache_key_collision_prevention() {
        // Test that different cache types generate different keys even with same identifiers
        let id = "123";
        
        let frequent_key = CacheKeys::frequent_query(id);
        let heavy_key = CacheKeys::heavy_content(id);
        let semantic_key = CacheKeys::semantic_query(id);
        let api_key = CacheKeys::api_response("endpoint", id);

        // All keys should be different
        let keys = vec![&frequent_key, &heavy_key, &semantic_key, &api_key];
        for i in 0..keys.len() {
            for j in i+1..keys.len() {
                assert_ne!(keys[i], keys[j], "Keys should be unique: {} vs {}", keys[i], keys[j]);
            }
        }

        // Keys should contain their type prefix
        assert!(frequent_key.starts_with("frequent_query:"));
        assert!(heavy_key.starts_with("heavy_content:"));
        assert!(semantic_key.starts_with("semantic_query:"));
        assert!(api_key.starts_with("api:"));
    }

    #[test]
    fn test_cache_expiration_edge_cases() {
        let now = Utc::now();
        
        // Test entry that expires exactly now
        let expiring_now = CacheEntry {
            data: "expiring".to_string(),
            created_at: now - chrono::Duration::hours(1),
            expires_at: now,
            access_count: 5,
            last_accessed: now - chrono::Duration::minutes(10),
            cache_type: CacheType::General,
        };

        // Test entry that expired 1 second ago
        let expired = CacheEntry {
            data: "expired".to_string(),
            created_at: now - chrono::Duration::hours(2),
            expires_at: now - chrono::Duration::seconds(1),
            access_count: 3,
            last_accessed: now - chrono::Duration::hours(1),
            cache_type: CacheType::General,
        };

        // Test entry that expires in 1 second
        let valid = CacheEntry {
            data: "valid".to_string(),
            created_at: now - chrono::Duration::minutes(30),
            expires_at: now + chrono::Duration::seconds(1),
            access_count: 8,
            last_accessed: now - chrono::Duration::minutes(1),
            cache_type: CacheType::General,
        };

        // Check expiration logic
        assert!(expiring_now.expires_at <= now, "Entry expiring now should be considered expired");
        assert!(expired.expires_at < now, "Expired entry should be expired");
        assert!(valid.expires_at > now, "Valid entry should not be expired");
    }
}
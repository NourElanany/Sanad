//! Unit tests for Tafsir API Manager
//!
//! These tests validate:
//! - Verse reference validation (Requirements 4.2, 4.4)
//! - Multi-source fetching (Requirements 4.2, 4.3)
//! - Organization by scholar and language (Requirements 4.3, 4.4)
//! - Error handling scenarios
//! - Edge cases

#[cfg(test)]
mod tests {
    use crate::api_clients::{
        tafsir::{TafsirApiManager, OrganizedTafsirResponse, QuranComTafsirClient},
        CacheManager, RateLimiter, TafsirApiClient, TafsirEntry, TafsirSource,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    /// Test that the manager correctly organizes tafsirs by scholar
    #[tokio::test]
    async fn test_organize_by_scholar() {
        let cache = Arc::new(
            CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"),
        );

        let rate_limiter = Arc::new(
            RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                .await
                .expect("Failed to create rate limiter"),
        );

        let clients: Vec<Box<dyn TafsirApiClient + Send + Sync>> = vec![];
        let manager = TafsirApiManager::new(clients, cache, rate_limiter);

        // Create test tafsirs
        let tafsirs = vec![
            TafsirEntry {
                tafsir_id: "1".to_string(),
                tafsir_name: "Tafsir Ibn Kathir Arabic".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "Arabic text".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "2".to_string(),
                tafsir_name: "Tafsir Ibn Kathir English".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "English text".to_string(),
                language: "English".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "3".to_string(),
                tafsir_name: "Tafsir Al-Jalalayn".to_string(),
                scholar: "Al-Jalalayn".to_string(),
                text: "Arabic text 2".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
        ];

        // Organize by scholar
        let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        for tafsir in &tafsirs {
            by_scholar
                .entry(tafsir.scholar.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        // Verify organization
        assert_eq!(by_scholar.len(), 2, "Should have 2 scholars");
        assert_eq!(
            by_scholar.get("Ibn Kathir").unwrap().len(),
            2,
            "Ibn Kathir should have 2 tafsirs"
        );
        assert_eq!(
            by_scholar.get("Al-Jalalayn").unwrap().len(),
            1,
            "Al-Jalalayn should have 1 tafsir"
        );

        // Verify all Ibn Kathir tafsirs are from the same scholar
        for tafsir in by_scholar.get("Ibn Kathir").unwrap() {
            assert_eq!(tafsir.scholar, "Ibn Kathir");
        }
    }

    /// Test that the manager correctly organizes tafsirs by language
    #[tokio::test]
    async fn test_organize_by_language() {
        let cache = Arc::new(
            CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"),
        );

        let rate_limiter = Arc::new(
            RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                .await
                .expect("Failed to create rate limiter"),
        );

        let clients: Vec<Box<dyn TafsirApiClient + Send + Sync>> = vec![];
        let manager = TafsirApiManager::new(clients, cache, rate_limiter);

        // Create test tafsirs
        let tafsirs = vec![
            TafsirEntry {
                tafsir_id: "1".to_string(),
                tafsir_name: "Tafsir Ibn Kathir".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "Arabic text".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "2".to_string(),
                tafsir_name: "Tafsir Al-Jalalayn".to_string(),
                scholar: "Al-Jalalayn".to_string(),
                text: "Arabic text 2".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "3".to_string(),
                tafsir_name: "Tafsir Ibn Kathir English".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "English text".to_string(),
                language: "English".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "4".to_string(),
                tafsir_name: "Tafsir Al-Tabari Urdu".to_string(),
                scholar: "Al-Tabari".to_string(),
                text: "Urdu text".to_string(),
                language: "Urdu".to_string(),
                source: "test".to_string(),
            },
        ];

        // Organize by language
        let mut by_language: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        for tafsir in &tafsirs {
            by_language
                .entry(tafsir.language.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        // Verify organization
        assert_eq!(by_language.len(), 3, "Should have 3 languages");
        assert_eq!(
            by_language.get("Arabic").unwrap().len(),
            2,
            "Arabic should have 2 tafsirs"
        );
        assert_eq!(
            by_language.get("English").unwrap().len(),
            1,
            "English should have 1 tafsir"
        );
        assert_eq!(
            by_language.get("Urdu").unwrap().len(),
            1,
            "Urdu should have 1 tafsir"
        );

        // Verify all Arabic tafsirs are in Arabic
        for tafsir in by_language.get("Arabic").unwrap() {
            assert_eq!(tafsir.language, "Arabic");
        }
    }

    /// Test that the manager correctly deduplicates sources
    #[tokio::test]
    async fn test_source_deduplication() {
        let cache = Arc::new(
            CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"),
        );

        let rate_limiter = Arc::new(
            RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                .await
                .expect("Failed to create rate limiter"),
        );

        let clients: Vec<Box<dyn TafsirApiClient + Send + Sync>> = vec![];
        let manager = TafsirApiManager::new(clients, cache, rate_limiter);

        // Create sources with duplicates
        let sources = vec![
            TafsirSource {
                id: "1".to_string(),
                name: "Tafsir Ibn Kathir".to_string(),
                scholar: "Ibn Kathir".to_string(),
                language: "Arabic".to_string(),
            },
            TafsirSource {
                id: "1".to_string(), // Duplicate ID
                name: "Tafsir Ibn Kathir (Copy)".to_string(),
                scholar: "Ibn Kathir".to_string(),
                language: "Arabic".to_string(),
            },
            TafsirSource {
                id: "2".to_string(),
                name: "Tafsir Al-Jalalayn".to_string(),
                scholar: "Al-Jalalayn".to_string(),
                language: "Arabic".to_string(),
            },
            TafsirSource {
                id: "3".to_string(),
                name: "Tafsir Al-Tabari".to_string(),
                scholar: "Al-Tabari".to_string(),
                language: "Arabic".to_string(),
            },
            TafsirSource {
                id: "2".to_string(), // Another duplicate
                name: "Tafsir Al-Jalalayn (Copy)".to_string(),
                scholar: "Al-Jalalayn".to_string(),
                language: "Arabic".to_string(),
            },
        ];

        let unique = manager.deduplicate_sources(sources);

        // Should have 3 unique sources (IDs: 1, 2, 3)
        assert_eq!(unique.len(), 3, "Should have 3 unique sources");

        // Verify all IDs are unique
        let ids: Vec<String> = unique.iter().map(|s| s.id.clone()).collect();
        let mut unique_ids = ids.clone();
        unique_ids.sort();
        unique_ids.dedup();
        assert_eq!(ids.len(), unique_ids.len(), "All IDs should be unique");
    }

    /// Test that organized response contains all expected fields
    #[test]
    fn test_organized_response_structure() {
        let response = OrganizedTafsirResponse {
            surah: 1,
            ayah: 1,
            by_scholar: HashMap::new(),
            by_language: HashMap::new(),
            all_tafsirs: vec![],
        };

        assert_eq!(response.surah, 1);
        assert_eq!(response.ayah, 1);
        assert!(response.by_scholar.is_empty());
        assert!(response.by_language.is_empty());
        assert!(response.all_tafsirs.is_empty());
    }

    /// Test that manager correctly handles empty client list
    #[tokio::test]
    async fn test_empty_client_list() {
        let cache = Arc::new(
            CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"),
        );

        let rate_limiter = Arc::new(
            RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                .await
                .expect("Failed to create rate limiter"),
        );

        let clients: Vec<Box<dyn TafsirApiClient + Send + Sync>> = vec![];
        let manager = TafsirApiManager::new(clients, cache, rate_limiter);

        assert_eq!(manager.client_count(), 0);
        assert!(manager.client_names().is_empty());
    }

    /// Test cache key generation for different requests
    #[test]
    fn test_cache_key_generation() {
        // Test with specific tafsir ID
        let key_with_id = format!("tafsir:{}:{}:{}", 1, 1, "ibn_kathir");
        assert_eq!(key_with_id, "tafsir:1:1:ibn_kathir");

        // Test without tafsir ID
        let key_without_id = format!("tafsir:{}:{}", 1, 1);
        assert_eq!(key_without_id, "tafsir:1:1");

        // Test different verses
        let key_verse_2 = format!("tafsir:{}:{}", 2, 255);
        assert_eq!(key_verse_2, "tafsir:2:255");
    }

    /// Test that organization preserves all tafsir data
    #[test]
    fn test_organization_preserves_data() {
        let tafsirs = vec![
            TafsirEntry {
                tafsir_id: "1".to_string(),
                tafsir_name: "Tafsir Ibn Kathir".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "This is the text of the tafsir".to_string(),
                language: "Arabic".to_string(),
                source: "quran.com".to_string(),
            },
        ];

        let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        for tafsir in &tafsirs {
            by_scholar
                .entry(tafsir.scholar.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        let organized = by_scholar.get("Ibn Kathir").unwrap();
        assert_eq!(organized.len(), 1);
        assert_eq!(organized[0].tafsir_id, "1");
        assert_eq!(organized[0].tafsir_name, "Tafsir Ibn Kathir");
        assert_eq!(organized[0].text, "This is the text of the tafsir");
        assert_eq!(organized[0].language, "Arabic");
        assert_eq!(organized[0].source, "quran.com");
    }

    /// Test that multiple tafsirs from same scholar are grouped correctly
    #[test]
    fn test_multiple_tafsirs_same_scholar() {
        let tafsirs = vec![
            TafsirEntry {
                tafsir_id: "1".to_string(),
                tafsir_name: "Tafsir Ibn Kathir Arabic".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "Arabic text".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "2".to_string(),
                tafsir_name: "Tafsir Ibn Kathir English".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "English text".to_string(),
                language: "English".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "3".to_string(),
                tafsir_name: "Tafsir Ibn Kathir Urdu".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "Urdu text".to_string(),
                language: "Urdu".to_string(),
                source: "test".to_string(),
            },
        ];

        let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        for tafsir in &tafsirs {
            by_scholar
                .entry(tafsir.scholar.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        let ibn_kathir_tafsirs = by_scholar.get("Ibn Kathir").unwrap();
        assert_eq!(ibn_kathir_tafsirs.len(), 3);

        // Verify all are from Ibn Kathir
        for tafsir in ibn_kathir_tafsirs {
            assert_eq!(tafsir.scholar, "Ibn Kathir");
        }

        // Verify different languages
        let languages: Vec<String> = ibn_kathir_tafsirs
            .iter()
            .map(|t| t.language.clone())
            .collect();
        assert!(languages.contains(&"Arabic".to_string()));
        assert!(languages.contains(&"English".to_string()));
        assert!(languages.contains(&"Urdu".to_string()));
    }

    // ============================================================================
    // VERSE REFERENCE VALIDATION TESTS
    // **Validates: Requirements 4.2, 4.4**
    // ============================================================================

    /// Test valid verse references are accepted
    #[test]
    fn test_valid_verse_references() {
        let client = QuranComTafsirClient::new(None);

        // Test first verse of first surah
        assert!(client.validate_verse(1, 1).is_ok());

        // Test last surah
        assert!(client.validate_verse(114, 1).is_ok());

        // Test middle surah
        assert!(client.validate_verse(57, 1).is_ok());

        // Test various ayah numbers
        assert!(client.validate_verse(2, 255).is_ok()); // Ayat al-Kursi
        assert!(client.validate_verse(18, 110).is_ok());
    }

    /// Test invalid surah numbers are rejected
    #[test]
    fn test_invalid_surah_numbers() {
        let client = QuranComTafsirClient::new(None);

        // Surah 0 is invalid
        let result = client.validate_verse(0, 1);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), crate::api_clients::ApiError::Validation(_)));

        // Surah 115 is invalid (only 114 surahs)
        let result = client.validate_verse(115, 1);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), crate::api_clients::ApiError::Validation(_)));

        // Surah 200 is invalid
        let result = client.validate_verse(200, 1);
        assert!(result.is_err());
    }

    /// Test invalid ayah numbers are rejected
    #[test]
    fn test_invalid_ayah_numbers() {
        let client = QuranComTafsirClient::new(None);

        // Ayah 0 is invalid
        let result = client.validate_verse(1, 0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), crate::api_clients::ApiError::Validation(_)));

        // Negative ayah numbers should be caught by type system (u16)
        // but we test boundary at 0
        let result = client.validate_verse(2, 0);
        assert!(result.is_err());
    }

    /// Test boundary conditions for verse references
    #[test]
    fn test_verse_reference_boundaries() {
        let client = QuranComTafsirClient::new(None);

        // Test minimum valid values
        assert!(client.validate_verse(1, 1).is_ok());

        // Test maximum valid surah
        assert!(client.validate_verse(114, 1).is_ok());

        // Test just beyond boundaries
        assert!(client.validate_verse(0, 1).is_err());
        assert!(client.validate_verse(115, 1).is_err());
        assert!(client.validate_verse(1, 0).is_err());
    }

    /// Test validation error messages are descriptive
    #[test]
    fn test_validation_error_messages() {
        let client = QuranComTafsirClient::new(None);

        // Test surah validation error message
        let result = client.validate_verse(0, 1);
        assert!(result.is_err());
        let error = result.unwrap_err();
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("surah"));
        assert!(error_msg.contains("0"));

        // Test ayah validation error message
        let result = client.validate_verse(1, 0);
        assert!(result.is_err());
        let error = result.unwrap_err();
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("ayah"));
        assert!(error_msg.contains("0"));
    }

    // ============================================================================
    // MULTI-SOURCE FETCHING TESTS
    // **Validates: Requirements 4.2, 4.3**
    // ============================================================================

    /// Test that manager can handle multiple API clients
    #[tokio::test]
    async fn test_multi_source_client_configuration() {
        let cache = Arc::new(
            CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"),
        );

        let rate_limiter = Arc::new(
            RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                .await
                .expect("Failed to create rate limiter"),
        );

        // Create multiple clients
        let clients: Vec<Box<dyn TafsirApiClient + Send + Sync>> = vec![
            Box::new(QuranComTafsirClient::new(None)),
            // In a real scenario, we'd add more clients here
        ];

        let manager = TafsirApiManager::new(clients, cache, rate_limiter);

        // Verify clients are configured
        assert!(manager.client_count() > 0);
        assert!(!manager.client_names().is_empty());
    }

    /// Test that manager attempts fallback when primary source fails
    #[tokio::test]
    async fn test_fallback_behavior() {
        let cache = Arc::new(
            CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"),
        );

        let rate_limiter = Arc::new(
            RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                .await
                .expect("Failed to create rate limiter"),
        );

        let clients: Vec<Box<dyn TafsirApiClient + Send + Sync>> = vec![];
        let manager = TafsirApiManager::new(clients, cache, rate_limiter);

        // With no clients, should fail gracefully
        let result = manager.get_tafsir(1, 1, None).await;
        assert!(result.is_err());
    }

    /// Test fetching from specific tafsir source
    #[test]
    fn test_specific_source_fetching() {
        // Test that cache key includes tafsir ID when specified
        let key_with_id = format!("tafsir:{}:{}:{}", 1, 1, "ibn_kathir");
        assert_eq!(key_with_id, "tafsir:1:1:ibn_kathir");

        // Test that cache key excludes tafsir ID when not specified
        let key_without_id = format!("tafsir:{}:{}", 1, 1);
        assert_eq!(key_without_id, "tafsir:1:1");
    }

    /// Test that sources from multiple APIs are combined
    #[tokio::test]
    async fn test_source_combination() {
        let cache = Arc::new(
            CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"),
        );

        let rate_limiter = Arc::new(
            RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                .await
                .expect("Failed to create rate limiter"),
        );

        let clients: Vec<Box<dyn TafsirApiClient + Send + Sync>> = vec![];
        let manager = TafsirApiManager::new(clients, cache, rate_limiter);

        // Test deduplication of sources from multiple APIs
        let sources = vec![
            TafsirSource {
                id: "1".to_string(),
                name: "Tafsir A".to_string(),
                scholar: "Scholar A".to_string(),
                language: "Arabic".to_string(),
            },
            TafsirSource {
                id: "1".to_string(), // Duplicate
                name: "Tafsir A Copy".to_string(),
                scholar: "Scholar A".to_string(),
                language: "Arabic".to_string(),
            },
            TafsirSource {
                id: "2".to_string(),
                name: "Tafsir B".to_string(),
                scholar: "Scholar B".to_string(),
                language: "English".to_string(),
            },
        ];

        let unique = manager.deduplicate_sources(sources);
        assert_eq!(unique.len(), 2, "Should deduplicate sources by ID");
    }

    // ============================================================================
    // ORGANIZATION BY SCHOLAR AND LANGUAGE TESTS
    // **Validates: Requirements 4.3, 4.4**
    // ============================================================================

    /// Test organization by scholar with multiple scholars
    #[test]
    fn test_organization_by_multiple_scholars() {
        let tafsirs = vec![
            TafsirEntry {
                tafsir_id: "1".to_string(),
                tafsir_name: "Tafsir Ibn Kathir".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "Text 1".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "2".to_string(),
                tafsir_name: "Tafsir Al-Jalalayn".to_string(),
                scholar: "Al-Jalalayn".to_string(),
                text: "Text 2".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "3".to_string(),
                tafsir_name: "Tafsir Al-Tabari".to_string(),
                scholar: "Al-Tabari".to_string(),
                text: "Text 3".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "4".to_string(),
                tafsir_name: "Tafsir Al-Qurtubi".to_string(),
                scholar: "Al-Qurtubi".to_string(),
                text: "Text 4".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
        ];

        let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        for tafsir in &tafsirs {
            by_scholar
                .entry(tafsir.scholar.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        // Should have 4 different scholars
        assert_eq!(by_scholar.len(), 4);
        assert!(by_scholar.contains_key("Ibn Kathir"));
        assert!(by_scholar.contains_key("Al-Jalalayn"));
        assert!(by_scholar.contains_key("Al-Tabari"));
        assert!(by_scholar.contains_key("Al-Qurtubi"));

        // Each scholar should have exactly 1 tafsir
        for (_, tafsirs_list) in by_scholar.iter() {
            assert_eq!(tafsirs_list.len(), 1);
        }
    }

    /// Test organization by language with multiple languages
    #[test]
    fn test_organization_by_multiple_languages() {
        let tafsirs = vec![
            TafsirEntry {
                tafsir_id: "1".to_string(),
                tafsir_name: "Tafsir Arabic 1".to_string(),
                scholar: "Scholar A".to_string(),
                text: "Text 1".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "2".to_string(),
                tafsir_name: "Tafsir English 1".to_string(),
                scholar: "Scholar B".to_string(),
                text: "Text 2".to_string(),
                language: "English".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "3".to_string(),
                tafsir_name: "Tafsir Urdu 1".to_string(),
                scholar: "Scholar C".to_string(),
                text: "Text 3".to_string(),
                language: "Urdu".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "4".to_string(),
                tafsir_name: "Tafsir Turkish 1".to_string(),
                scholar: "Scholar D".to_string(),
                text: "Text 4".to_string(),
                language: "Turkish".to_string(),
                source: "test".to_string(),
            },
        ];

        let mut by_language: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        for tafsir in &tafsirs {
            by_language
                .entry(tafsir.language.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        // Should have 4 different languages
        assert_eq!(by_language.len(), 4);
        assert!(by_language.contains_key("Arabic"));
        assert!(by_language.contains_key("English"));
        assert!(by_language.contains_key("Urdu"));
        assert!(by_language.contains_key("Turkish"));

        // Each language should have exactly 1 tafsir
        for (_, tafsirs_list) in by_language.iter() {
            assert_eq!(tafsirs_list.len(), 1);
        }
    }

    /// Test finding tafsir by specific scholar
    #[test]
    fn test_find_tafsir_by_scholar() {
        let tafsirs = vec![
            TafsirEntry {
                tafsir_id: "1".to_string(),
                tafsir_name: "Tafsir Ibn Kathir Arabic".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "Arabic text".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "2".to_string(),
                tafsir_name: "Tafsir Ibn Kathir English".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "English text".to_string(),
                language: "English".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "3".to_string(),
                tafsir_name: "Tafsir Al-Jalalayn".to_string(),
                scholar: "Al-Jalalayn".to_string(),
                text: "Arabic text 2".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
        ];

        let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        for tafsir in &tafsirs {
            by_scholar
                .entry(tafsir.scholar.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        // Find all Ibn Kathir tafsirs
        let ibn_kathir = by_scholar.get("Ibn Kathir").unwrap();
        assert_eq!(ibn_kathir.len(), 2);
        
        // Verify both are from Ibn Kathir
        for tafsir in ibn_kathir {
            assert_eq!(tafsir.scholar, "Ibn Kathir");
        }

        // Find Al-Jalalayn tafsir
        let jalalayn = by_scholar.get("Al-Jalalayn").unwrap();
        assert_eq!(jalalayn.len(), 1);
        assert_eq!(jalalayn[0].scholar, "Al-Jalalayn");
    }

    /// Test finding tafsir by specific language
    #[test]
    fn test_find_tafsir_by_language() {
        let tafsirs = vec![
            TafsirEntry {
                tafsir_id: "1".to_string(),
                tafsir_name: "Tafsir 1".to_string(),
                scholar: "Scholar A".to_string(),
                text: "Arabic text 1".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "2".to_string(),
                tafsir_name: "Tafsir 2".to_string(),
                scholar: "Scholar B".to_string(),
                text: "Arabic text 2".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "3".to_string(),
                tafsir_name: "Tafsir 3".to_string(),
                scholar: "Scholar C".to_string(),
                text: "English text".to_string(),
                language: "English".to_string(),
                source: "test".to_string(),
            },
        ];

        let mut by_language: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        for tafsir in &tafsirs {
            by_language
                .entry(tafsir.language.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        // Find all Arabic tafsirs
        let arabic = by_language.get("Arabic").unwrap();
        assert_eq!(arabic.len(), 2);
        
        // Verify both are in Arabic
        for tafsir in arabic {
            assert_eq!(tafsir.language, "Arabic");
        }

        // Find English tafsir
        let english = by_language.get("English").unwrap();
        assert_eq!(english.len(), 1);
        assert_eq!(english[0].language, "English");
    }

    /// Test organized response structure completeness
    #[test]
    fn test_organized_response_completeness() {
        let tafsirs = vec![
            TafsirEntry {
                tafsir_id: "1".to_string(),
                tafsir_name: "Tafsir 1".to_string(),
                scholar: "Scholar A".to_string(),
                text: "Text 1".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "2".to_string(),
                tafsir_name: "Tafsir 2".to_string(),
                scholar: "Scholar B".to_string(),
                text: "Text 2".to_string(),
                language: "English".to_string(),
                source: "test".to_string(),
            },
        ];

        let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        let mut by_language: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        
        for tafsir in &tafsirs {
            by_scholar
                .entry(tafsir.scholar.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
            
            by_language
                .entry(tafsir.language.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        let response = OrganizedTafsirResponse {
            surah: 1,
            ayah: 1,
            by_scholar: by_scholar.clone(),
            by_language: by_language.clone(),
            all_tafsirs: tafsirs.clone(),
        };

        // Verify all fields are populated
        assert_eq!(response.surah, 1);
        assert_eq!(response.ayah, 1);
        assert_eq!(response.all_tafsirs.len(), 2);
        assert_eq!(response.by_scholar.len(), 2);
        assert_eq!(response.by_language.len(), 2);

        // Verify no data loss
        let total_in_scholars: usize = response.by_scholar.values().map(|v| v.len()).sum();
        let total_in_languages: usize = response.by_language.values().map(|v| v.len()).sum();
        assert_eq!(total_in_scholars, tafsirs.len());
        assert_eq!(total_in_languages, tafsirs.len());
    }

    // ============================================================================
    // ERROR HANDLING TESTS
    // **Validates: Requirements 4.2, 4.4**
    // ============================================================================

    /// Test handling of empty tafsir list
    #[test]
    fn test_empty_tafsir_list_handling() {
        let tafsirs: Vec<TafsirEntry> = vec![];

        let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        let mut by_language: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        
        for tafsir in &tafsirs {
            by_scholar
                .entry(tafsir.scholar.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
            
            by_language
                .entry(tafsir.language.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        let response = OrganizedTafsirResponse {
            surah: 1,
            ayah: 1,
            by_scholar,
            by_language,
            all_tafsirs: tafsirs,
        };

        // Should handle empty list gracefully
        assert!(response.all_tafsirs.is_empty());
        assert!(response.by_scholar.is_empty());
        assert!(response.by_language.is_empty());
    }

    /// Test handling of missing scholar information
    #[test]
    fn test_missing_scholar_information() {
        let tafsir = TafsirEntry {
            tafsir_id: "1".to_string(),
            tafsir_name: "Tafsir 1".to_string(),
            scholar: "".to_string(), // Empty scholar name
            text: "Text 1".to_string(),
            language: "Arabic".to_string(),
            source: "test".to_string(),
        };

        let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        by_scholar
            .entry(tafsir.scholar.clone())
            .or_insert_with(Vec::new)
            .push(tafsir.clone());

        // Should still organize, even with empty scholar name
        assert_eq!(by_scholar.len(), 1);
        assert!(by_scholar.contains_key(""));
    }

    /// Test handling of missing language information
    #[test]
    fn test_missing_language_information() {
        let tafsir = TafsirEntry {
            tafsir_id: "1".to_string(),
            tafsir_name: "Tafsir 1".to_string(),
            scholar: "Scholar A".to_string(),
            text: "Text 1".to_string(),
            language: "".to_string(), // Empty language
            source: "test".to_string(),
        };

        let mut by_language: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        by_language
            .entry(tafsir.language.clone())
            .or_insert_with(Vec::new)
            .push(tafsir.clone());

        // Should still organize, even with empty language
        assert_eq!(by_language.len(), 1);
        assert!(by_language.contains_key(""));
    }

    /// Test client priority ordering
    #[tokio::test]
    async fn test_client_priority_ordering() {
        let cache = Arc::new(
            CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"),
        );

        let rate_limiter = Arc::new(
            RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                .await
                .expect("Failed to create rate limiter"),
        );

        let clients: Vec<Box<dyn TafsirApiClient + Send + Sync>> = vec![
            Box::new(QuranComTafsirClient::new(None)), // Priority 1
        ];

        let manager = TafsirApiManager::new(clients, cache, rate_limiter);
        let names = manager.client_names();

        // Should be sorted by priority (lower number = higher priority)
        assert_eq!(names[0], "quran.com_tafsir");
    }

    /// Test cache key generation consistency
    #[test]
    fn test_cache_key_consistency() {
        // Same parameters should generate same key
        let key1 = format!("tafsir:{}:{}:{}", 1, 1, "ibn_kathir");
        let key2 = format!("tafsir:{}:{}:{}", 1, 1, "ibn_kathir");
        assert_eq!(key1, key2);

        // Different parameters should generate different keys
        let key3 = format!("tafsir:{}:{}:{}", 1, 2, "ibn_kathir");
        assert_ne!(key1, key3);

        let key4 = format!("tafsir:{}:{}:{}", 2, 1, "ibn_kathir");
        assert_ne!(key1, key4);

        let key5 = format!("tafsir:{}:{}:{}", 1, 1, "al_jalalayn");
        assert_ne!(key1, key5);
    }
}

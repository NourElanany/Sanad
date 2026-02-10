//! Unit tests for Hadith API clients
//!
//! These tests verify specific examples and edge cases for hadith API functionality.

#[cfg(test)]
mod tests {
    use crate::api_clients::hadith::{
        AladhanHadithClient, HadithApiClientImpl, HadithApiManager, SunnahComClient,
    };
    use crate::api_clients::{ApiClient, CacheManager, HadithApiClient, HadithResult, RateLimiter};
    use std::collections::HashMap;
    use std::sync::Arc;

    // Helper to create a test manager
    async fn create_test_manager() -> HadithApiManager {
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

        let clients: Vec<Box<dyn HadithApiClient + Send + Sync>> = vec![
            Box::new(SunnahComClient::new("test_key".to_string())),
            Box::new(HadithApiClientImpl::default()),
            Box::new(AladhanHadithClient::new()),
        ];

        HadithApiManager::new(clients, cache, rate_limiter)
    }

    // ========================================================================
    // SunnahComClient Tests
    // ========================================================================

    #[test]
    fn test_sunnah_com_client_creation() {
        let client = SunnahComClient::new("test_api_key".to_string());
        assert_eq!(client.api_name(), "sunnah.com");
        assert_eq!(client.priority(), 1);
    }

    #[test]
    fn test_sunnah_com_rate_limits() {
        let client = SunnahComClient::new("test_key".to_string());
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 30);
        assert_eq!(config.requests_per_hour, 500);
        assert_eq!(config.requests_per_day, 5000);
    }

    #[tokio::test]
    async fn test_sunnah_com_empty_query() {
        let client = SunnahComClient::new("test_key".to_string());
        let result = client.search("", 10).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sunnah_com_invalid_limit_zero() {
        let client = SunnahComClient::new("test_key".to_string());
        let result = client.search("prophet", 0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sunnah_com_invalid_limit_too_large() {
        let client = SunnahComClient::new("test_key".to_string());
        let result = client.search("prophet", 101).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sunnah_com_empty_hadith_id() {
        let client = SunnahComClient::new("test_key".to_string());
        let result = client.get_by_id("").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sunnah_com_empty_collection() {
        let client = SunnahComClient::new("test_key".to_string());
        let result = client.get_by_collection("", 10).await;
        assert!(result.is_err());
    }

    // ========================================================================
    // HadithApiClient Tests
    // ========================================================================

    #[test]
    fn test_hadith_api_client_creation() {
        let client = HadithApiClientImpl::default();
        assert_eq!(client.api_name(), "hadith.api");
        assert_eq!(client.priority(), 2);
    }

    #[test]
    fn test_hadith_api_rate_limits() {
        let client = HadithApiClientImpl::default();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 30);
        assert_eq!(config.requests_per_hour, 500);
        assert_eq!(config.requests_per_day, 5000);
    }

    #[tokio::test]
    async fn test_hadith_api_empty_query() {
        let client = HadithApiClientImpl::default();
        let result: Result<Vec<HadithResult>, _> = client.search("", 10).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_hadith_api_invalid_limit() {
        let client = HadithApiClientImpl::default();
        
        let result: Result<Vec<HadithResult>, _> = client.search("test", 0).await;
        assert!(result.is_err());
        
        let result: Result<Vec<HadithResult>, _> = client.search("test", 101).await;
        assert!(result.is_err());
    }

    // ========================================================================
    // AladhanHadithClient Tests
    // ========================================================================

    #[test]
    fn test_aladhan_client_creation() {
        let client = AladhanHadithClient::new();
        assert_eq!(client.api_name(), "aladhan.hadith");
        assert_eq!(client.priority(), 3);
    }

    #[test]
    fn test_aladhan_default_creation() {
        let client = AladhanHadithClient::default();
        assert_eq!(client.api_name(), "aladhan.hadith");
    }

    #[test]
    fn test_aladhan_rate_limits() {
        let client = AladhanHadithClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.requests_per_hour, 1000);
        assert_eq!(config.requests_per_day, 10000);
    }

    #[tokio::test]
    async fn test_aladhan_empty_query() {
        let client = AladhanHadithClient::new();
        let result = client.search("", 10).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_aladhan_get_by_id_not_supported() {
        let client = AladhanHadithClient::new();
        let result = client.get_by_id("123").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_aladhan_get_by_collection_not_supported() {
        let client = AladhanHadithClient::new();
        let result = client.get_by_collection("bukhari", 10).await;
        assert!(result.is_err());
    }

    // ========================================================================
    // HadithApiManager Tests
    // ========================================================================

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = create_test_manager().await;
        assert_eq!(manager.client_count(), 3);
    }

    #[tokio::test]
    async fn test_manager_client_priority_order() {
        let manager = create_test_manager().await;
        let names = manager.client_names();
        
        // Should be sorted by priority
        assert_eq!(names[0], "sunnah.com");      // Priority 1
        assert_eq!(names[1], "hadith.api");      // Priority 2
        assert_eq!(names[2], "aladhan.hadith");  // Priority 3
    }

    #[tokio::test]
    async fn test_manager_deduplication_basic() {
        let manager = create_test_manager().await;
        
        // Create duplicate results
        let results = vec![
            HadithResult {
                id: "1".to_string(),
                collection: "Bukhari".to_string(),
                book: "Book 1".to_string(),
                hadith_number: "1".to_string(),
                text_arabic: "نص الحديث الأول".to_string(),
                text_translation: Some("First hadith text".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Abu Hurairah".to_string(),
                source: "source1".to_string(),
            },
            HadithResult {
                id: "2".to_string(),
                collection: "Bukhari".to_string(),
                book: "Book 1".to_string(),
                hadith_number: "1".to_string(),
                text_arabic: "نص الحديث الأول".to_string(), // Duplicate
                text_translation: Some("First hadith text".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Abu Hurairah".to_string(),
                source: "source2".to_string(),
            },
            HadithResult {
                id: "3".to_string(),
                collection: "Muslim".to_string(),
                book: "Book 2".to_string(),
                hadith_number: "2".to_string(),
                text_arabic: "نص الحديث الثاني".to_string(), // Different
                text_translation: Some("Second hadith text".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Aisha".to_string(),
                source: "source1".to_string(),
            },
        ];
        
        let deduplicated = manager.deduplicate_results(results);
        assert_eq!(deduplicated.len(), 2, "Should have 2 unique results");
    }

    #[tokio::test]
    async fn test_manager_deduplication_all_unique() {
        let manager = create_test_manager().await;
        
        // Create all unique results
        let results = vec![
            HadithResult {
                id: "1".to_string(),
                collection: "Bukhari".to_string(),
                book: "Book 1".to_string(),
                hadith_number: "1".to_string(),
                text_arabic: "نص الحديث الأول".to_string(),
                text_translation: Some("First hadith".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Abu Hurairah".to_string(),
                source: "source1".to_string(),
            },
            HadithResult {
                id: "2".to_string(),
                collection: "Muslim".to_string(),
                book: "Book 2".to_string(),
                hadith_number: "2".to_string(),
                text_arabic: "نص الحديث الثاني".to_string(),
                text_translation: Some("Second hadith".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Aisha".to_string(),
                source: "source2".to_string(),
            },
            HadithResult {
                id: "3".to_string(),
                collection: "Tirmidhi".to_string(),
                book: "Book 3".to_string(),
                hadith_number: "3".to_string(),
                text_arabic: "نص الحديث الثالث".to_string(),
                text_translation: Some("Third hadith".to_string()),
                grade: Some("Hasan".to_string()),
                narrator: "Ibn Abbas".to_string(),
                source: "source3".to_string(),
            },
        ];
        
        let deduplicated = manager.deduplicate_results(results.clone());
        assert_eq!(deduplicated.len(), results.len(), "All unique should remain");
    }

    #[tokio::test]
    async fn test_manager_deduplication_all_duplicates() {
        let manager = create_test_manager().await;
        
        // Create all duplicate results (same text and number)
        let results = vec![
            HadithResult {
                id: "1".to_string(),
                collection: "Bukhari".to_string(),
                book: "Book 1".to_string(),
                hadith_number: "1".to_string(),
                text_arabic: "نفس النص".to_string(),
                text_translation: Some("Same text".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Narrator".to_string(),
                source: "source1".to_string(),
            },
            HadithResult {
                id: "2".to_string(),
                collection: "Bukhari".to_string(),
                book: "Book 1".to_string(),
                hadith_number: "1".to_string(),
                text_arabic: "نفس النص".to_string(),
                text_translation: Some("Same text".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Narrator".to_string(),
                source: "source2".to_string(),
            },
            HadithResult {
                id: "3".to_string(),
                collection: "Bukhari".to_string(),
                book: "Book 1".to_string(),
                hadith_number: "1".to_string(),
                text_arabic: "نفس النص".to_string(),
                text_translation: Some("Same text".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Narrator".to_string(),
                source: "source3".to_string(),
            },
        ];
        
        let deduplicated = manager.deduplicate_results(results);
        assert_eq!(deduplicated.len(), 1, "All duplicates should become one");
    }

    #[tokio::test]
    async fn test_manager_deduplication_empty_input() {
        let manager = create_test_manager().await;
        let results = Vec::new();
        let deduplicated = manager.deduplicate_results(results);
        assert_eq!(deduplicated.len(), 0, "Empty input should return empty");
    }

    #[tokio::test]
    async fn test_manager_deduplication_single_result() {
        let manager = create_test_manager().await;
        
        let results = vec![HadithResult {
            id: "1".to_string(),
            collection: "Bukhari".to_string(),
            book: "Book 1".to_string(),
            hadith_number: "1".to_string(),
            text_arabic: "نص الحديث".to_string(),
            text_translation: Some("Hadith text".to_string()),
            grade: Some("Sahih".to_string()),
            narrator: "Narrator".to_string(),
            source: "source1".to_string(),
        }];
        
        let deduplicated = manager.deduplicate_results(results.clone());
        assert_eq!(deduplicated.len(), 1, "Single result should remain");
        assert_eq!(deduplicated[0].id, results[0].id);
    }

    #[tokio::test]
    async fn test_manager_deduplication_different_numbers_same_text() {
        let manager = create_test_manager().await;
        
        // Same text but different hadith numbers should NOT be duplicates
        let results = vec![
            HadithResult {
                id: "1".to_string(),
                collection: "Bukhari".to_string(),
                book: "Book 1".to_string(),
                hadith_number: "1".to_string(),
                text_arabic: "نفس النص".to_string(),
                text_translation: Some("Same text".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Narrator".to_string(),
                source: "source1".to_string(),
            },
            HadithResult {
                id: "2".to_string(),
                collection: "Bukhari".to_string(),
                book: "Book 1".to_string(),
                hadith_number: "2".to_string(), // Different number
                text_arabic: "نفس النص".to_string(),
                text_translation: Some("Same text".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Narrator".to_string(),
                source: "source2".to_string(),
            },
        ];
        
        let deduplicated = manager.deduplicate_results(results);
        assert_eq!(
            deduplicated.len(),
            2,
            "Different hadith numbers should not be duplicates"
        );
    }

    #[tokio::test]
    async fn test_manager_deduplication_preserves_first_occurrence() {
        let manager = create_test_manager().await;
        
        let results = vec![
            HadithResult {
                id: "first".to_string(),
                collection: "Bukhari".to_string(),
                book: "Book 1".to_string(),
                hadith_number: "1".to_string(),
                text_arabic: "نص الحديث".to_string(),
                text_translation: Some("Hadith text".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Narrator".to_string(),
                source: "source1".to_string(),
            },
            HadithResult {
                id: "second".to_string(),
                collection: "Bukhari".to_string(),
                book: "Book 1".to_string(),
                hadith_number: "1".to_string(),
                text_arabic: "نص الحديث".to_string(), // Duplicate
                text_translation: Some("Hadith text".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Narrator".to_string(),
                source: "source2".to_string(),
            },
        ];
        
        let deduplicated = manager.deduplicate_results(results);
        assert_eq!(deduplicated.len(), 1);
        assert_eq!(deduplicated[0].id, "first", "Should preserve first occurrence");
    }
}

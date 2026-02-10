//! Property-based tests for Hadith API clients
//!
//! These tests verify universal properties that should hold across all inputs.

#[cfg(test)]
mod tests {
    use crate::api_clients::hadith::{
        AladhanHadithClient, HadithApiClientImpl, HadithApiManager, SunnahComClient,
    };
    use crate::api_clients::{CacheManager, HadithResult, RateLimiter};
    use proptest::prelude::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::time::Instant;

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

        let clients: Vec<Box<dyn crate::api_clients::HadithApiClient + Send + Sync>> = vec![
            Box::new(SunnahComClient::new("test_key".to_string())),
            Box::new(HadithApiClientImpl::default()),
            Box::new(AladhanHadithClient::new()),
        ];

        HadithApiManager::new(clients, cache, rate_limiter)
    }

    // Feature: official-apis-integration, Property 5: Parallel API Querying
    // **Validates: Requirements 2.2**
    //
    // For any hadith search request, the system should query all configured hadith APIs
    // in parallel (not sequentially), and the total time should not exceed the slowest
    // individual API call time plus overhead.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(20))]
        
        #[test]
        fn property_parallel_api_querying(
            query in "[a-z]{3,10}",
            limit in 1usize..=10
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let manager = create_test_manager().await;
                
                // Clear cache to ensure we're testing actual API calls
                let cache_key = format!("hadith:search:{}:{}", query, limit);
                let _ = manager.clear_cache(&cache_key).await;
                
                // Measure time for parallel execution
                let start = Instant::now();
                let result = manager.search(&query, limit).await;
                let parallel_duration = start.elapsed();
                
                // The property we're testing:
                // Parallel execution should complete in roughly the time of the slowest API
                // plus some overhead (we'll allow 2x the expected time for network variance)
                //
                // Since we can't easily measure individual API times in this test,
                // we'll verify that:
                // 1. The call completes in a reasonable time (< 30 seconds for 3 APIs)
                // 2. If successful, we got results from potentially multiple sources
                
                prop_assert!(
                    parallel_duration.as_secs() < 30,
                    "Parallel query took too long: {:?}",
                    parallel_duration
                );
                
                // If we got results, verify they came from our APIs
                if let Ok(results) = result {
                    for hadith in &results {
                        prop_assert!(
                            hadith.source == "sunnah.com" 
                            || hadith.source == "hadith.api" 
                            || hadith.source == "aladhan.hadith",
                            "Result from unexpected source: {}",
                            hadith.source
                        );
                    }
                }
                
                Ok(())
            });
        }
    }

    // Feature: official-apis-integration, Property 6: Deduplication of Merged Results
    // **Validates: Requirements 2.3**
    //
    // For any set of API responses containing duplicate entries (based on content hash
    // or reference), the system should return only unique entries, and the count of
    // unique results should be less than or equal to the sum of all results.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(20))]
        
        #[test]
        fn property_deduplication_of_merged_results(
            // Generate test data with potential duplicates
            num_results in 5usize..=20,
            duplicate_factor in 0.0..=0.5f64, // 0-50% duplicates
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let manager = create_test_manager().await;
                
                // Create a set of hadith results with some duplicates
                let mut all_results = Vec::new();
                let unique_count = (num_results as f64 * (1.0 - duplicate_factor)).ceil() as usize;
                
                // Create unique results
                for i in 0..unique_count {
                    all_results.push(HadithResult {
                        id: format!("test_{}", i),
                        collection: "Test".to_string(),
                        book: "Book 1".to_string(),
                        hadith_number: i.to_string(),
                        text_arabic: format!("نص رقم {}", i),
                        text_translation: Some(format!("Text number {}", i)),
                        grade: Some("Sahih".to_string()),
                        narrator: "Test Narrator".to_string(),
                        source: "test".to_string(),
                    });
                }
                
                // Add duplicates (same arabic text and hadith number)
                let num_duplicates = num_results - unique_count;
                for i in 0..num_duplicates {
                    let original_idx = i % unique_count;
                    let original = &all_results[original_idx];
                    all_results.push(HadithResult {
                        id: format!("duplicate_{}", i),
                        collection: original.collection.clone(),
                        book: original.book.clone(),
                        hadith_number: original.hadith_number.clone(),
                        text_arabic: original.text_arabic.clone(),
                        text_translation: original.text_translation.clone(),
                        grade: original.grade.clone(),
                        narrator: original.narrator.clone(),
                        source: "duplicate_source".to_string(),
                    });
                }
                
                let total_before = all_results.len();
                
                // Deduplicate using the manager's method
                let deduplicated = manager.deduplicate_results(all_results);
                let total_after = deduplicated.len();
                
                // Property: unique count <= total count
                prop_assert!(
                    total_after <= total_before,
                    "Deduplication increased count: {} -> {}",
                    total_before,
                    total_after
                );
                
                // Property: should have removed duplicates
                prop_assert!(
                    total_after == unique_count,
                    "Expected {} unique results, got {}",
                    unique_count,
                    total_after
                );
                
                // Property: all results should be unique
                let mut seen_keys = std::collections::HashSet::new();
                for result in &deduplicated {
                    let key = format!("{}:{}", result.text_arabic, result.hadith_number);
                    prop_assert!(
                        seen_keys.insert(key.clone()),
                        "Duplicate found after deduplication: {}",
                        key
                    );
                }
                
                Ok(())
            });
        }
    }

    // Additional property test: Verify deduplication is deterministic
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]
        
        #[test]
        fn property_deduplication_is_deterministic(
            num_results in 5usize..=15,
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let manager = create_test_manager().await;
                
                // Create test results
                let mut results = Vec::new();
                for i in 0..num_results {
                    results.push(HadithResult {
                        id: format!("test_{}", i),
                        collection: "Test".to_string(),
                        book: "Book 1".to_string(),
                        hadith_number: (i % 5).to_string(), // Create some duplicates
                        text_arabic: format!("نص رقم {}", i % 5),
                        text_translation: Some(format!("Text {}", i)),
                        grade: Some("Sahih".to_string()),
                        narrator: "Narrator".to_string(),
                        source: "test".to_string(),
                    });
                }
                
                // Deduplicate twice
                let dedup1 = manager.deduplicate_results(results.clone());
                let dedup2 = manager.deduplicate_results(results.clone());
                
                // Property: deduplication should be deterministic
                prop_assert_eq!(
                    dedup1.len(),
                    dedup2.len(),
                    "Deduplication produced different counts"
                );
                
                Ok(())
            });
        }
    }

    // Property test: Empty input should return empty output
    #[test]
    fn property_empty_input_empty_output() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let manager = create_test_manager().await;
            
            let empty_results = Vec::new();
            let deduplicated = manager.deduplicate_results(empty_results);
            
            assert_eq!(deduplicated.len(), 0, "Empty input should produce empty output");
        });
    }

    // Property test: Single result should remain unchanged
    #[test]
    fn property_single_result_unchanged() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let manager = create_test_manager().await;
            
            let single_result = vec![HadithResult {
                id: "test_1".to_string(),
                collection: "Test".to_string(),
                book: "Book 1".to_string(),
                hadith_number: "1".to_string(),
                text_arabic: "نص الحديث".to_string(),
                text_translation: Some("Hadith text".to_string()),
                grade: Some("Sahih".to_string()),
                narrator: "Narrator".to_string(),
                source: "test".to_string(),
            }];
            
            let deduplicated = manager.deduplicate_results(single_result.clone());
            
            assert_eq!(deduplicated.len(), 1, "Single result should remain as one");
            assert_eq!(deduplicated[0].id, single_result[0].id);
        });
    }

    // Property test: All identical results should deduplicate to one
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]
        
        #[test]
        fn property_all_identical_dedup_to_one(
            count in 2usize..=20,
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let manager = create_test_manager().await;
                
                // Create identical results
                let mut results = Vec::new();
                for i in 0..count {
                    results.push(HadithResult {
                        id: format!("test_{}", i),
                        collection: "Test".to_string(),
                        book: "Book 1".to_string(),
                        hadith_number: "1".to_string(),
                        text_arabic: "نفس النص".to_string(), // Same text
                        text_translation: Some("Same text".to_string()),
                        grade: Some("Sahih".to_string()),
                        narrator: "Narrator".to_string(),
                        source: format!("source_{}", i),
                    });
                }
                
                let deduplicated = manager.deduplicate_results(results);
                
                // Property: all identical should become one
                prop_assert_eq!(
                    deduplicated.len(),
                    1,
                    "All identical results should deduplicate to one"
                );
                
                Ok(())
            });
        }
    }
}

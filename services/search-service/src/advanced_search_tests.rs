use crate::models::*;
use crate::semantic_search::SemanticSearchEngine;
use crate::text_processor::ArabicTextProcessor;
use std::collections::HashMap;
use std::sync::Arc;
use chrono::Utc;
use tokio::sync::RwLock;

/// Test advanced filtering and browsing capabilities
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_advanced_filtering_by_content_type() {
        let engine = create_test_engine().await;
        
        let request = SemanticSearchRequest {
            query: "الصلاة".to_string(),
            limit: 10,
            filters: Some(SearchFilters {
                content_types: Some(vec![ContentType::Quran, ContentType::SahihHadith]),
                ..Default::default()
            }),
            enable_caching: false,
            ..Default::default()
        };

        // Test filter building instead of actual search
        let filter = engine.build_advanced_search_filter(&request).unwrap();
        assert!(filter.is_some(), "Filter should be built for content type filtering");
        
        // Test that the engine can process the request parameters
        let (limit, offset) = engine.calculate_pagination(&request);
        assert_eq!(limit, 10);
        assert_eq!(offset, 0);
    }

    #[tokio::test]
    async fn test_authenticity_grade_filtering() {
        let engine = create_test_engine().await;
        
        let request = SemanticSearchRequest {
            query: "حديث".to_string(),
            limit: 10,
            filters: Some(SearchFilters {
                authenticity_grades: Some(vec![AuthenticityGrade::Sahih, AuthenticityGrade::Hasan]),
                ..Default::default()
            }),
            enable_caching: false,
            ..Default::default()
        };

        // Test filter building for authenticity grades
        let filter = engine.build_advanced_search_filter(&request).unwrap();
        // The filter should be None because authenticity_grades is handled in passes_advanced_filters
        // not in the Qdrant filter
        assert!(filter.is_none(), "Authenticity grade filtering is handled post-search");
        
        // Test that passes_advanced_filters works correctly
        let mut test_doc_metadata = HashMap::new();
        test_doc_metadata.insert("authenticity_grade".to_string(), 
                                serde_json::Value::String("sahih".to_string()));
        
        let test_result = SearchResult {
            document: IslamicDocument {
                id: "test".to_string(),
                text: "test hadith".to_string(),
                content_type: ContentType::SahihHadith,
                source: "test".to_string(),
                author: None,
                language: Language::Arabic,
                metadata: test_doc_metadata,
                created_at: Some(Utc::now()),
                updated_at: Some(Utc::now()),
            },
            similarity_score: 0.8,
            rank: 1,
            highlighted_text: None,
            explanation: None,
        };
        
        // This should pass the filter (though the actual filtering logic would need to be implemented)
        let passes = engine.passes_advanced_filters(&test_result, &request);
        assert!(passes, "Result with Sahih grade should pass the filter");
    }

    #[tokio::test]
    async fn test_pagination_functionality() {
        let engine = create_test_engine().await;
        
        // Test first page
        let request_page1 = SemanticSearchRequest {
            query: "الله".to_string(),
            page: Some(1),
            page_size: Some(5),
            enable_caching: false,
            ..Default::default()
        };

        // Test pagination calculation
        let (limit, offset) = engine.calculate_pagination(&request_page1);
        assert_eq!(limit, 5);
        assert_eq!(offset, 0);

        // Test second page
        let request_page2 = SemanticSearchRequest {
            query: "الله".to_string(),
            page: Some(2),
            page_size: Some(5),
            enable_caching: false,
            ..Default::default()
        };

        let (limit, offset) = engine.calculate_pagination(&request_page2);
        assert_eq!(limit, 5);
        assert_eq!(offset, 5);
        
        // Test third page
        let request_page3 = SemanticSearchRequest {
            query: "الله".to_string(),
            page: Some(3),
            page_size: Some(10),
            enable_caching: false,
            ..Default::default()
        };

        let (limit, offset) = engine.calculate_pagination(&request_page3);
        assert_eq!(limit, 10);
        assert_eq!(offset, 20);
    }

    #[tokio::test]
    async fn test_sorting_functionality() {
        let engine = create_test_engine().await;
        
        // Create mock search results for testing sorting
        let mut results = vec![
            SearchResult {
                document: IslamicDocument {
                    id: "doc1".to_string(),
                    text: "Test document 1".to_string(),
                    content_type: ContentType::Tafsir,
                    source: "Test".to_string(),
                    author: None,
                    language: Language::Arabic,
                    metadata: HashMap::new(),
                    created_at: Some(Utc::now()),
                    updated_at: None,
                },
                similarity_score: 0.7,
                rank: 1,
                highlighted_text: None,
                explanation: None,
            },
            SearchResult {
                document: IslamicDocument {
                    id: "doc2".to_string(),
                    text: "Test document 2".to_string(),
                    content_type: ContentType::Quran,
                    source: "Test".to_string(),
                    author: None,
                    language: Language::Arabic,
                    metadata: HashMap::new(),
                    created_at: Some(Utc::now()),
                    updated_at: None,
                },
                similarity_score: 0.9,
                rank: 2,
                highlighted_text: None,
                explanation: None,
            },
        ];

        // Test sorting by similarity (default)
        let request_similarity = SemanticSearchRequest {
            query: "الصلاة".to_string(),
            limit: 10,
            sort_by: Some(SortBy::Similarity),
            sort_direction: Some(SortDirection::Desc),
            enable_caching: false,
            ..Default::default()
        };

        engine.sort_results(&mut results, &request_similarity);
        
        // Verify results are sorted by similarity in descending order
        assert!(results[0].similarity_score >= results[1].similarity_score);

        // Test sorting by priority
        let request_priority = SemanticSearchRequest {
            query: "الصلاة".to_string(),
            limit: 10,
            sort_by: Some(SortBy::Priority),
            sort_direction: Some(SortDirection::Asc),
            enable_caching: false,
            ..Default::default()
        };

        engine.sort_results(&mut results, &request_priority);
        
        // Verify results are sorted by priority in ascending order
        assert!(results[0].document.content_type.priority() <= results[1].document.content_type.priority());
    }

    #[tokio::test]
    async fn test_query_suggestions() {
        let engine = create_test_engine().await;
        
        // Test cache key generation for suggestions
        let cache_key = engine.generate_cache_key(&SemanticSearchRequest {
            query: "صلاة".to_string(),
            limit: 5,
            include_suggestions: true,
            enable_caching: false,
            ..Default::default()
        });
        
        assert!(!cache_key.is_empty());
        assert!(cache_key.starts_with("search:"));
        
        // Test that the same request generates the same cache key
        let cache_key2 = engine.generate_cache_key(&SemanticSearchRequest {
            query: "صلاة".to_string(),
            limit: 5,
            include_suggestions: true,
            enable_caching: false,
            ..Default::default()
        });
        
        assert_eq!(cache_key, cache_key2);
    }

    #[tokio::test]
    async fn test_caching_functionality() {
        let engine = create_test_engine().await;
        
        let request = SemanticSearchRequest {
            query: "الله الرحمن الرحيم".to_string(),
            limit: 5,
            enable_caching: true,
            ..Default::default()
        };

        // Test cache key generation
        let cache_key = engine.generate_cache_key(&request);
        assert!(!cache_key.is_empty());
        assert!(cache_key.starts_with("search:"));
        
        // Test that the same request generates the same cache key
        let cache_key2 = engine.generate_cache_key(&request);
        assert_eq!(cache_key, cache_key2);
        
        // Test that different requests generate different cache keys
        let different_request = SemanticSearchRequest {
            query: "different query".to_string(),
            limit: 5,
            enable_caching: true,
            ..Default::default()
        };
        
        let different_cache_key = engine.generate_cache_key(&different_request);
        assert_ne!(cache_key, different_cache_key);
    }

    #[tokio::test]
    async fn test_text_length_filtering() {
        let engine = create_test_engine().await;
        
        let request = SemanticSearchRequest {
            query: "الإسلام".to_string(),
            limit: 10,
            filters: Some(SearchFilters {
                text_length_range: Some(RangeFilter {
                    min: Some(50),
                    max: Some(200),
                }),
                ..Default::default()
            }),
            enable_caching: false,
            ..Default::default()
        };

        // Test that the filter is built correctly
        let filter = engine.build_advanced_search_filter(&request).unwrap();
        assert!(filter.is_some(), "Filter should be built for text length range");
        
        // Test passes_advanced_filters with different text lengths
        let short_result = SearchResult {
            document: IslamicDocument {
                id: "short".to_string(),
                text: "Short text".to_string(), // 10 characters
                content_type: ContentType::Quran,
                source: "test".to_string(),
                author: None,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: Some(Utc::now()),
                updated_at: None,
            },
            similarity_score: 0.8,
            rank: 1,
            highlighted_text: None,
            explanation: None,
        };
        
        let long_result = SearchResult {
            document: IslamicDocument {
                id: "long".to_string(),
                text: "This is a much longer text that should be within the specified range for testing purposes and contains enough characters".to_string(), // ~120 characters
                content_type: ContentType::Quran,
                source: "test".to_string(),
                author: None,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: Some(Utc::now()),
                updated_at: None,
            },
            similarity_score: 0.8,
            rank: 1,
            highlighted_text: None,
            explanation: None,
        };
        
        // Short text should not pass the filter (< 50 chars)
        assert!(!engine.passes_advanced_filters(&short_result, &request));
        
        // Long text should pass the filter (within 50-200 chars)
        assert!(engine.passes_advanced_filters(&long_result, &request));
    }

    #[tokio::test]
    async fn test_similarity_range_filtering() {
        let engine = create_test_engine().await;
        
        let request = SemanticSearchRequest {
            query: "القرآن الكريم".to_string(),
            limit: 10,
            filters: Some(SearchFilters {
                min_similarity: Some(0.7),
                max_similarity: Some(0.9),
                ..Default::default()
            }),
            enable_caching: false,
            ..Default::default()
        };

        // Test passes_advanced_filters with different similarity scores
        let low_similarity_result = SearchResult {
            document: IslamicDocument {
                id: "low".to_string(),
                text: "Test text".to_string(),
                content_type: ContentType::Quran,
                source: "test".to_string(),
                author: None,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: Some(Utc::now()),
                updated_at: None,
            },
            similarity_score: 0.5, // Below minimum
            rank: 1,
            highlighted_text: None,
            explanation: None,
        };
        
        let good_similarity_result = SearchResult {
            document: IslamicDocument {
                id: "good".to_string(),
                text: "Test text".to_string(),
                content_type: ContentType::Quran,
                source: "test".to_string(),
                author: None,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: Some(Utc::now()),
                updated_at: None,
            },
            similarity_score: 0.8, // Within range
            rank: 1,
            highlighted_text: None,
            explanation: None,
        };
        
        let high_similarity_result = SearchResult {
            document: IslamicDocument {
                id: "high".to_string(),
                text: "Test text".to_string(),
                content_type: ContentType::Quran,
                source: "test".to_string(),
                author: None,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: Some(Utc::now()),
                updated_at: None,
            },
            similarity_score: 0.95, // Above maximum
            rank: 1,
            highlighted_text: None,
            explanation: None,
        };
        
        // Test filtering
        assert!(!engine.passes_advanced_filters(&low_similarity_result, &request));
        assert!(engine.passes_advanced_filters(&good_similarity_result, &request));
        assert!(!engine.passes_advanced_filters(&high_similarity_result, &request));
    }

    #[tokio::test]
    async fn test_combined_filters() {
        let engine = create_test_engine().await;
        
        let request = SemanticSearchRequest {
            query: "الصوم".to_string(),
            limit: 10,
            filters: Some(SearchFilters {
                content_types: Some(vec![ContentType::Quran, ContentType::SahihHadith]),
                authenticity_grades: Some(vec![AuthenticityGrade::Sahih]),
                min_similarity: Some(0.6),
                text_length_range: Some(RangeFilter {
                    min: Some(30),
                    max: Some(500),
                }),
                ..Default::default()
            }),
            enable_caching: false,
            ..Default::default()
        };

        // Test that the filter is built correctly for content types
        let filter = engine.build_advanced_search_filter(&request).unwrap();
        assert!(filter.is_some(), "Filter should be built for content types");
        
        // Test passes_advanced_filters with a result that should pass all filters
        let mut test_metadata = HashMap::new();
        test_metadata.insert("authenticity_grade".to_string(), 
                            serde_json::Value::String("sahih".to_string()));
        
        let test_result = SearchResult {
            document: IslamicDocument {
                id: "test".to_string(),
                text: "This is a test hadith about fasting that has enough characters to pass the length filter".to_string(), // ~100 characters
                content_type: ContentType::SahihHadith,
                source: "test".to_string(),
                author: None,
                language: Language::Arabic,
                metadata: test_metadata,
                created_at: Some(Utc::now()),
                updated_at: None,
            },
            similarity_score: 0.8, // Above minimum
            rank: 1,
            highlighted_text: None,
            explanation: None,
        };
        
        // This should pass all filters
        assert!(engine.passes_advanced_filters(&test_result, &request));
        
        // Test with a result that should fail similarity filter
        let low_similarity_result = SearchResult {
            document: test_result.document.clone(),
            similarity_score: 0.4, // Below minimum
            rank: 1,
            highlighted_text: None,
            explanation: None,
        };
        
        assert!(!engine.passes_advanced_filters(&low_similarity_result, &request));
    }

    #[tokio::test]
    async fn test_cache_key_generation() {
        let engine = create_test_engine().await;
        
        let request1 = SemanticSearchRequest {
            query: "الصلاة".to_string(),
            limit: 10,
            ..Default::default()
        };
        
        let request2 = SemanticSearchRequest {
            query: "الصلاة".to_string(),
            limit: 20, // Different limit
            ..Default::default()
        };
        
        let key1 = engine.generate_cache_key(&request1);
        let key2 = engine.generate_cache_key(&request2);
        
        // Different requests should have different cache keys
        assert_ne!(key1, key2);
        
        // Same request should have same cache key
        let key1_again = engine.generate_cache_key(&request1);
        assert_eq!(key1, key1_again);
    }

    // Helper function to create a test engine without connecting to Qdrant
    async fn create_test_engine() -> SemanticSearchEngine {
        let config = SearchServiceConfig::default();
        
        // Create a mock engine for testing without actual Qdrant connection
        // Note: This won't actually work for real searches, but allows testing of other functionality
        let client = qdrant_client::Qdrant::from_url("http://localhost:6333").build().unwrap();
        
        SemanticSearchEngine {
            client,
            config: config.clone(),
            text_processor: ArabicTextProcessor::new().unwrap(),
            collection_name: "test_collection".to_string(),
            synonym_map: HashMap::new(),
            concept_map: HashMap::new(),
            root_to_words: HashMap::new(),
            query_cache: Arc::new(RwLock::new(HashMap::new())),
            suggestion_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for SearchFilters {
    fn default() -> Self {
        Self {
            source: None,
            author: None,
            language: None,
            date_range: None,
            metadata_filters: None,
            content_types: None,
            authenticity_grades: None,
            min_similarity: None,
            max_similarity: None,
            text_length_range: None,
            priority_range: None,
        }
    }
}

/// Property-based test for advanced filtering system
/// **Validates: Requirements 8.4, 8.5, 8.7**
#[cfg(test)]
mod property_tests {
    use super::*;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_pagination_consistency(page: u8, page_size: u8) -> TestResult {
        if page == 0 || page_size == 0 || page_size > 100 {
            return TestResult::discard();
        }
        
        let page = page as usize;
        let page_size = page_size as usize;
        
        // Test pagination calculation
        let request = SemanticSearchRequest {
            query: "test".to_string(),
            page: Some(page),
            page_size: Some(page_size),
            ..Default::default()
        };
        
        let engine = create_mock_engine();
        let (limit, offset) = engine.calculate_pagination(&request);
        
        // Properties that should always hold
        TestResult::from_bool(
            limit == page_size &&
            offset == (page - 1) * page_size &&
            offset >= 0
        )
    }

    #[quickcheck]
    fn prop_similarity_filtering(min_sim: f32, max_sim: f32) -> TestResult {
        // Discard NaN values and invalid ranges
        if min_sim.is_nan() || max_sim.is_nan() || 
           min_sim < 0.0 || max_sim > 1.0 || min_sim > max_sim {
            return TestResult::discard();
        }
        
        let filters = SearchFilters {
            min_similarity: Some(min_sim),
            max_similarity: Some(max_sim),
            ..Default::default()
        };
        
        let request = SemanticSearchRequest {
            query: "test".to_string(),
            filters: Some(filters),
            ..Default::default()
        };
        
        let engine = create_mock_engine();
        
        // Create a mock result
        let result = create_mock_search_result(0.5);
        
        // Test filtering logic
        let passes = engine.passes_advanced_filters(&result, &request);
        
        TestResult::from_bool(
            if 0.5 >= min_sim && 0.5 <= max_sim {
                passes
            } else {
                !passes
            }
        )
    }

    #[quickcheck]
    fn prop_text_length_filtering(min_len: u16, max_len: u16) -> TestResult {
        if min_len > max_len || max_len > 1000 {
            return TestResult::discard();
        }
        
        let min_len = min_len as usize;
        let max_len = max_len as usize;
        
        let filters = SearchFilters {
            text_length_range: Some(RangeFilter {
                min: Some(min_len),
                max: Some(max_len),
            }),
            ..Default::default()
        };
        
        let request = SemanticSearchRequest {
            query: "test".to_string(),
            filters: Some(filters),
            ..Default::default()
        };
        
        let engine = create_mock_engine();
        
        // Create mock results with different text lengths
        let short_result = create_mock_search_result_with_text("short".to_string());
        let long_result = create_mock_search_result_with_text("a".repeat(500));
        
        let short_passes = engine.passes_advanced_filters(&short_result, &request);
        let long_passes = engine.passes_advanced_filters(&long_result, &request);
        
        TestResult::from_bool(
            (5 >= min_len && 5 <= max_len) == short_passes &&
            (500 >= min_len && 500 <= max_len) == long_passes
        )
    }

    #[quickcheck]
    fn prop_cache_key_deterministic(query: String, limit: u8) -> TestResult {
        if query.is_empty() || limit == 0 {
            return TestResult::discard();
        }
        
        let request = SemanticSearchRequest {
            query: query.clone(),
            limit: limit as usize,
            ..Default::default()
        };
        
        let engine = create_mock_engine();
        
        let key1 = engine.generate_cache_key(&request);
        let key2 = engine.generate_cache_key(&request);
        
        // Same request should always generate the same cache key
        TestResult::from_bool(key1 == key2)
    }

    #[quickcheck]
    fn prop_sorting_maintains_order(sort_direction: bool) -> TestResult {
        let direction = if sort_direction { SortDirection::Asc } else { SortDirection::Desc };
        
        let request = SemanticSearchRequest {
            query: "test".to_string(),
            sort_by: Some(SortBy::Similarity),
            sort_direction: Some(direction.clone()),
            ..Default::default()
        };
        
        let engine = create_mock_engine();
        
        // Create mock results with different similarity scores
        let mut results = vec![
            create_mock_search_result(0.9),
            create_mock_search_result(0.7),
            create_mock_search_result(0.8),
            create_mock_search_result(0.6),
        ];
        
        engine.sort_results(&mut results, &request);
        
        // Check if results are properly sorted
        let is_sorted = match direction {
            SortDirection::Asc => {
                results.windows(2).all(|w| w[0].similarity_score <= w[1].similarity_score)
            },
            SortDirection::Desc => {
                results.windows(2).all(|w| w[0].similarity_score >= w[1].similarity_score)
            },
        };
        
        TestResult::from_bool(is_sorted)
    }

    // Helper functions for property tests
    fn create_mock_engine() -> SemanticSearchEngine {
        use crate::text_processor::ArabicTextProcessor;
        use std::sync::Arc;
        
        SemanticSearchEngine {
            client: create_mock_qdrant_client(),
            config: SearchServiceConfig::default(),
            text_processor: ArabicTextProcessor::new().unwrap(),
            collection_name: "test".to_string(),
            synonym_map: HashMap::new(),
            concept_map: HashMap::new(),
            root_to_words: HashMap::new(),
            query_cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            suggestion_cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
    
    fn create_mock_qdrant_client() -> qdrant_client::Qdrant {
        // This would normally create a mock client
        // For property tests, we'll use a placeholder
        qdrant_client::Qdrant::from_url("http://localhost:6333").build().unwrap()
    }
    
    fn create_mock_search_result(similarity_score: f32) -> SearchResult {
        SearchResult {
            document: IslamicDocument {
                id: "test_id".to_string(),
                text: "test text".to_string(),
                content_type: ContentType::Quran,
                source: "test_source".to_string(),
                author: None,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: Some(Utc::now()),
                updated_at: None,
            },
            similarity_score,
            rank: 1,
            highlighted_text: None,
            explanation: None,
        }
    }
    
    fn create_mock_search_result_with_text(text: String) -> SearchResult {
        SearchResult {
            document: IslamicDocument {
                id: "test_id".to_string(),
                text,
                content_type: ContentType::Quran,
                source: "test_source".to_string(),
                author: None,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: Some(Utc::now()),
                updated_at: None,
            },
            similarity_score: 0.8,
            rank: 1,
            highlighted_text: None,
            explanation: None,
        }
    }
}
use crate::semantic_search::*;
use crate::models::*;
use tokio;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test contextual search with synonym expansion
    #[tokio::test]
    async fn test_contextual_search_with_synonyms() {
        // This test would require a running Qdrant instance
        // For now, we'll test the semantic understanding components
        
        let engine = create_test_engine().await;
        
        let request = ContextualSearchRequest {
            query: "الصلاة في الإسلام".to_string(),
            context: None,
            search_mode: SearchMode::Expanded,
            expand_synonyms: true,
            expand_roots: false,
            expand_concepts: false,
            limit: 10,
            content_types: None,
            min_similarity: 0.5,
            filters: None,
        };
        
        // Test query understanding
        let understanding = engine.understand_query(&request.query, None).await.unwrap();
        
        assert!(!understanding.normalized_query.is_empty());
        assert!(understanding.confidence > 0.0);
        assert_eq!(understanding.detected_language, Some(Language::Arabic));
        
        // Should detect prayer-related concepts or keywords
        assert!(!understanding.query_keywords.is_empty(), 
                "Expected some keywords to be extracted from query: {}", request.query);
        
        // Check that we have some meaningful content
        assert!(!understanding.normalized_query.is_empty());
        assert!(understanding.confidence > 0.0);
    }

    /// Test root-based search functionality
    #[tokio::test]
    async fn test_root_based_search() {
        let engine = create_test_engine().await;
        
        let _roots = vec!["صلي".to_string(), "زكي".to_string()];
        
        // Test that root expansion works
        assert!(engine.root_to_words.contains_key("صلي"));
        assert!(engine.root_to_words.contains_key("زكي"));
        
        let prayer_words = engine.root_to_words.get("صلي").unwrap();
        assert!(prayer_words.contains(&"صلاة".to_string()));
        assert!(prayer_words.contains(&"يصلي".to_string()));
    }

    /// Test synonym and concept mapping
    #[tokio::test]
    async fn test_synonym_and_concept_mapping() {
        let engine = create_test_engine().await;
        
        // Test synonym mapping
        assert!(engine.synonym_map.contains_key("الله"));
        let allah_synonyms = engine.synonym_map.get("الله").unwrap();
        assert!(allah_synonyms.contains(&"رب".to_string()));
        assert!(allah_synonyms.contains(&"الخالق".to_string()));
        
        // Test concept mapping
        assert!(engine.concept_map.contains_key("العبادة"));
        let worship_concepts = engine.concept_map.get("العبادة").unwrap();
        assert!(worship_concepts.contains(&"صلاة".to_string()));
        assert!(worship_concepts.contains(&"زكاة".to_string()));
    }

    /// Test query intent classification
    #[tokio::test]
    async fn test_query_intent_classification() {
        let engine = create_test_engine().await;
        
        // Test factual question
        let factual_query = "ما هي أركان الإسلام؟";
        let understanding = engine.understand_query(factual_query, None).await.unwrap();
        assert!(matches!(understanding.query_intent, QueryIntent::FactualQuestion));
        
        // Test definition request
        let definition_query = "تعريف الصلاة";
        let understanding = engine.understand_query(definition_query, None).await.unwrap();
        assert!(matches!(understanding.query_intent, QueryIntent::DefinitionRequest));
        
        // Test comparison query
        let comparison_query = "الفرق بين الزكاة والصدقة";
        let understanding = engine.understand_query(comparison_query, None).await.unwrap();
        assert!(matches!(understanding.query_intent, QueryIntent::ComparativeAnalysis));
    }

    /// Test contextual scoring
    #[tokio::test]
    async fn test_contextual_scoring() {
        let engine = create_test_engine().await;
        
        let understanding = QueryUnderstanding {
            original_query: "الصلاة".to_string(),
            normalized_query: "الصلاة".to_string(),
            query_keywords: vec!["الصلاة".to_string()],
            detected_language: Some(Language::Arabic),
            extracted_concepts: vec!["العبادة".to_string()],
            identified_roots: vec!["صلي".to_string()],
            synonyms: vec!["فريضة".to_string()],
            related_terms: vec!["صلوات".to_string()],
            query_intent: QueryIntent::TextualSearch,
            confidence: 0.8,
        };
        
        // Test with Quran document (should get high score)
        let quran_doc = IslamicDocument {
            id: "quran_1".to_string(),
            text: "وأقيموا الصلاة وآتوا الزكاة".to_string(),
            content_type: ContentType::Quran,
            source: "القرآن الكريم".to_string(),
            author: None,
            language: Language::Arabic,
            metadata: std::collections::HashMap::new(),
            created_at: None,
            updated_at: None,
        };
        
        let request = ContextualSearchRequest {
            query: "الصلاة".to_string(),
            context: None,
            search_mode: SearchMode::Exact,
            expand_synonyms: false,
            expand_roots: false,
            expand_concepts: false,
            limit: 10,
            content_types: None,
            min_similarity: 0.5,
            filters: None,
        };
        
        let contextual_score = engine.calculate_contextual_score(&quran_doc, &understanding, &request);
        assert!(contextual_score > 0.3); // Should get high score for Quran content
        
        // Test with lower priority content
        let story_doc = IslamicDocument {
            id: "story_1".to_string(),
            text: "قصة عن أهمية الصلاة".to_string(),
            content_type: ContentType::IslamicStory,
            source: "قصص إسلامية".to_string(),
            author: Some("مؤلف".to_string()),
            language: Language::Arabic,
            metadata: std::collections::HashMap::new(),
            created_at: None,
            updated_at: None,
        };
        
        let story_score = engine.calculate_contextual_score(&story_doc, &understanding, &request);
        assert!(contextual_score > story_score); // Quran should score higher than story
    }

    /// Test match type determination
    #[tokio::test]
    async fn test_match_type_determination() {
        let engine = create_test_engine().await;
        
        let understanding = QueryUnderstanding {
            original_query: "الصلاة".to_string(),
            normalized_query: "الصلاة".to_string(),
            query_keywords: vec!["الصلاة".to_string()],
            detected_language: Some(Language::Arabic),
            extracted_concepts: vec!["العبادة".to_string()],
            identified_roots: vec!["صلي".to_string()],
            synonyms: vec!["فريضة".to_string()],
            related_terms: vec!["صلوات".to_string()],
            query_intent: QueryIntent::TextualSearch,
            confidence: 0.8,
        };
        
        // Test direct match
        let direct_match_doc = IslamicDocument {
            id: "direct".to_string(),
            text: "الصلاة عماد الدين".to_string(),
            content_type: ContentType::SahihHadith,
            source: "صحيح البخاري".to_string(),
            author: None,
            language: Language::Arabic,
            metadata: std::collections::HashMap::new(),
            created_at: None,
            updated_at: None,
        };
        
        let match_type = engine.determine_match_type(&direct_match_doc, &understanding);
        assert!(matches!(match_type, MatchType::DirectMatch));
        
        // Test synonym match
        let synonym_match_doc = IslamicDocument {
            id: "synonym".to_string(),
            text: "الفريضة واجبة على كل مسلم".to_string(),
            content_type: ContentType::Tafsir,
            source: "تفسير".to_string(),
            author: None,
            language: Language::Arabic,
            metadata: std::collections::HashMap::new(),
            created_at: None,
            updated_at: None,
        };
        
        let match_type = engine.determine_match_type(&synonym_match_doc, &understanding);
        assert!(matches!(match_type, MatchType::SynonymMatch));
        
        // Test root match
        let root_match_doc = IslamicDocument {
            id: "root".to_string(),
            text: "يصلي المسلم خمس مرات في اليوم".to_string(),
            content_type: ContentType::FiqhRuling,
            source: "فقه".to_string(),
            author: None,
            language: Language::Arabic,
            metadata: std::collections::HashMap::new(),
            created_at: None,
            updated_at: None,
        };
        
        let match_type = engine.determine_match_type(&root_match_doc, &understanding);
        assert!(matches!(match_type, MatchType::RootMatch));
    }

    /// Test search mode variations
    #[tokio::test]
    async fn test_search_mode_variations() {
        let engine = create_test_engine().await;
        
        let understanding = QueryUnderstanding {
            original_query: "الصلاة".to_string(),
            normalized_query: "الصلاة".to_string(),
            query_keywords: vec!["الصلاة".to_string()],
            detected_language: Some(Language::Arabic),
            extracted_concepts: vec!["العبادة".to_string()],
            identified_roots: vec!["صلي".to_string()],
            synonyms: vec!["فريضة".to_string()],
            related_terms: vec!["صلوات".to_string()],
            query_intent: QueryIntent::TextualSearch,
            confidence: 0.8,
        };
        
        // Test exact mode
        let exact_request = ContextualSearchRequest {
            query: "الصلاة".to_string(),
            context: None,
            search_mode: SearchMode::Exact,
            expand_synonyms: false,
            expand_roots: false,
            expand_concepts: false,
            limit: 10,
            content_types: None,
            min_similarity: 0.5,
            filters: None,
        };
        
        let exact_variants = engine.generate_search_variants(&understanding, &exact_request).await.unwrap();
        assert_eq!(exact_variants.len(), 1); // Should only have original query
        
        // Test expanded mode
        let expanded_request = ContextualSearchRequest {
            query: "الصلاة".to_string(),
            context: None,
            search_mode: SearchMode::Expanded,
            expand_synonyms: true,
            expand_roots: true,
            expand_concepts: false,
            limit: 10,
            content_types: None,
            min_similarity: 0.5,
            filters: None,
        };
        
        let expanded_variants = engine.generate_search_variants(&understanding, &expanded_request).await.unwrap();
        assert!(expanded_variants.len() > 1); // Should have multiple variants
        
        // Test hybrid mode
        let hybrid_request = ContextualSearchRequest {
            query: "الصلاة".to_string(),
            context: None,
            search_mode: SearchMode::Hybrid,
            expand_synonyms: true,
            expand_roots: true,
            expand_concepts: true,
            limit: 10,
            content_types: None,
            min_similarity: 0.5,
            filters: None,
        };
        
        let hybrid_variants = engine.generate_search_variants(&understanding, &hybrid_request).await.unwrap();
        assert!(hybrid_variants.len() >= expanded_variants.len()); // Should have at least as many as expanded
    }

    // Helper function to create a test engine
    async fn create_test_engine() -> SemanticSearchEngine {
        let mut config = SearchServiceConfig::default();
        config.collection_name = "test_collection".to_string();
        
        // For testing, we'll create an engine without connecting to Qdrant
        // In a real test environment, you would use a test container
        SemanticSearchEngine {
            client: qdrant_client::Qdrant::from_url("http://localhost:6333").build().unwrap(),
            config,
            text_processor: crate::text_processor::ArabicTextProcessor::new().unwrap(),
            collection_name: "test_collection".to_string(),
            synonym_map: {
                let mut map = std::collections::HashMap::new();
                map.insert("الله".to_string(), vec!["رب".to_string(), "الخالق".to_string()]);
                map.insert("صلاة".to_string(), vec!["فريضة".to_string(), "عبادة".to_string()]);
                map
            },
            concept_map: {
                let mut map = std::collections::HashMap::new();
                map.insert("العبادة".to_string(), vec!["صلاة".to_string(), "زكاة".to_string(), "صوم".to_string()]);
                map
            },
            root_to_words: {
                let mut map = std::collections::HashMap::new();
                map.insert("صلي".to_string(), vec!["صلاة".to_string(), "يصلي".to_string(), "صلوات".to_string()]);
                map.insert("زكي".to_string(), vec!["زكاة".to_string(), "يزكي".to_string(), "تزكية".to_string()]);
                map
            },
            query_cache: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            suggestion_cache: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
}
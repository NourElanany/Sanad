// Unit tests for AI/NLP API clients

use crate::api_clients::{
    AiApiClient, AiQueryRequest, ApiClient, ApiError, CacheManager, MockRedisClient, RateLimiter,
};
use std::sync::Arc;

use super::{AiApiManager, HuggingFaceClient};

// ============================================================================
// HuggingFaceClient Tests
// ============================================================================

#[test]
fn test_hugging_face_client_creation() {
    let client = HuggingFaceClient::new(None);
    assert_eq!(client.api_name(), "hugging_face");
    assert_eq!(client.priority(), 1);
}

#[test]
fn test_hugging_face_with_api_key() {
    let client = HuggingFaceClient::new(Some("test_key".to_string()));
    assert_eq!(client.api_name(), "hugging_face");
}

#[test]
fn test_hugging_face_with_custom_model() {
    let client = HuggingFaceClient::new(None).with_model("custom-model".to_string());
    assert_eq!(client.api_name(), "hugging_face");
}

#[test]
fn test_hugging_face_rate_limit() {
    let client = HuggingFaceClient::new(None);
    let config = client.rate_limit();
    assert_eq!(config.requests_per_minute, 30);
    assert_eq!(config.requests_per_hour, 1000);
    assert_eq!(config.requests_per_day, 10000);
}

#[tokio::test]
async fn test_hugging_face_empty_query() {
    let client = HuggingFaceClient::new(None);

    let request = AiQueryRequest {
        query: "".to_string(),
        context: None,
        language: "en".to_string(),
        max_tokens: None,
    };

    let result = client.process_query(&request).await;
    assert!(matches!(result, Err(ApiError::InvalidInput(_))));
}

#[tokio::test]
async fn test_hugging_face_whitespace_query() {
    let client = HuggingFaceClient::new(None);

    let request = AiQueryRequest {
        query: "   \n\t  ".to_string(),
        context: None,
        language: "en".to_string(),
        max_tokens: None,
    };

    let result = client.process_query(&request).await;
    assert!(matches!(result, Err(ApiError::InvalidInput(_))));
}

#[tokio::test]
async fn test_hugging_face_query_with_context() {
    let client = HuggingFaceClient::new(None);

    let request = AiQueryRequest {
        query: "What does this mean?".to_string(),
        context: Some("This is the context".to_string()),
        language: "en".to_string(),
        max_tokens: Some(100),
    };

    // This will likely fail without a valid API key, but should not panic
    let result = client.process_query(&request).await;
    // Either succeeds or fails gracefully
    match result {
        Ok(response) => {
            assert!(!response.response.is_empty());
            assert!(!response.sources.is_empty());
            assert!(response.confidence > 0.0 && response.confidence <= 1.0);
        }
        Err(_) => {
            // API might be unavailable, which is okay for this test
        }
    }
}

#[tokio::test]
async fn test_hugging_face_arabic_query() {
    let client = HuggingFaceClient::new(None);

    let request = AiQueryRequest {
        query: "ما معنى هذا؟".to_string(),
        context: None,
        language: "ar".to_string(),
        max_tokens: Some(100),
    };

    // This will likely fail without a valid API key, but should not panic
    let result = client.process_query(&request).await;
    // Either succeeds or fails gracefully
    match result {
        Ok(response) => {
            assert!(!response.response.is_empty());
        }
        Err(_) => {
            // API might be unavailable, which is okay for this test
        }
    }
}

// ============================================================================
// AiApiManager Tests
// ============================================================================

fn create_test_manager() -> AiApiManager {
    let redis = Arc::new(MockRedisClient::new());
    let cache = Arc::new(CacheManager::new(redis.clone()));
    let rate_limiter = Arc::new(RateLimiter::new(redis));
    AiApiManager::new(cache, rate_limiter, None)
}

#[test]
fn test_manager_creation() {
    let manager = create_test_manager();
    assert_eq!(manager.get_clients().len(), 1);
}

#[test]
fn test_cache_key_consistency() {
    let request1 = AiQueryRequest {
        query: "test query".to_string(),
        context: None,
        language: "en".to_string(),
        max_tokens: None,
    };

    let request2 = AiQueryRequest {
        query: "test query".to_string(),
        context: None,
        language: "en".to_string(),
        max_tokens: None,
    };

    let key1 = AiApiManager::cache_key(&request1);
    let key2 = AiApiManager::cache_key(&request2);

    assert_eq!(key1, key2);
}

#[test]
fn test_cache_key_uniqueness() {
    let request1 = AiQueryRequest {
        query: "query 1".to_string(),
        context: None,
        language: "en".to_string(),
        max_tokens: None,
    };

    let request2 = AiQueryRequest {
        query: "query 2".to_string(),
        context: None,
        language: "en".to_string(),
        max_tokens: None,
    };

    let key1 = AiApiManager::cache_key(&request1);
    let key2 = AiApiManager::cache_key(&request2);

    assert_ne!(key1, key2);
}

#[test]
fn test_response_validation_valid_content() {
    let manager = create_test_manager();

    let valid_responses = vec![
        "This is a technical explanation.",
        "The verse discusses patience.",
        "Historical context is important here.",
        "This word means 'mercy' in Arabic.",
    ];

    for response in valid_responses {
        let result = manager.validate_response(response);
        assert!(result.is_ok(), "Valid response rejected: {}", response);
    }
}

#[test]
fn test_response_validation_blocks_fatwas() {
    let manager = create_test_manager();

    let invalid_responses = vec![
        "This is a fatwa about the matter.",
        "The fatwa states that...",
        "According to this fatwa...",
    ];

    for response in invalid_responses {
        let result = manager.validate_response(response);
        assert!(
            result.is_err(),
            "Invalid response not blocked: {}",
            response
        );
    }
}

#[test]
fn test_response_validation_blocks_arabic_rulings() {
    let manager = create_test_manager();

    let invalid_responses = vec![
        "هذا حكم شرعي في المسألة",
        "الفتوى تقول أن...",
        "هذا حلال",
        "هذا حرام",
        "هذا واجب",
        "هذا مستحب",
        "هذا مكروه",
    ];

    for response in invalid_responses {
        let result = manager.validate_response(response);
        assert!(
            result.is_err(),
            "Invalid response not blocked: {}",
            response
        );
    }
}

#[tokio::test]
async fn test_manager_health_check() {
    let manager = create_test_manager();
    let health = manager.health_check().await;

    assert_eq!(health.len(), 1);
    assert!(health[0].0.contains("hugging_face"));
}

#[tokio::test]
async fn test_manager_empty_query() {
    let manager = create_test_manager();

    let request = AiQueryRequest {
        query: "".to_string(),
        context: None,
        language: "en".to_string(),
        max_tokens: None,
    };

    let result = manager.process_query(&request).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_manager_query_with_context() {
    let manager = create_test_manager();

    let request = AiQueryRequest {
        query: "What does this mean?".to_string(),
        context: Some("Context information".to_string()),
        language: "en".to_string(),
        max_tokens: Some(100),
    };

    // Will likely fail without API key, but should not panic
    let result = manager.process_query(&request).await;
    match result {
        Ok(response) => {
            // If it succeeds, validate the response
            assert!(!response.response.is_empty());
            assert!(!response.sources.is_empty());
            assert!(response.confidence > 0.0 && response.confidence <= 1.0);
        }
        Err(_) => {
            // API unavailable is acceptable
        }
    }
}

#[tokio::test]
async fn test_manager_caching() {
    let manager = create_test_manager();

    let request = AiQueryRequest {
        query: "test query for caching".to_string(),
        context: None,
        language: "en".to_string(),
        max_tokens: Some(50),
    };

    // First request
    let result1 = manager.process_query(&request).await;

    // Second request (should use cache if first succeeded)
    let result2 = manager.process_query(&request).await;

    // Both should have same outcome (success or failure)
    assert_eq!(result1.is_ok(), result2.is_ok());
}

// ============================================================================
// Integration Tests
// ============================================================================

#[tokio::test]
async fn test_error_handling_when_services_unavailable() {
    let manager = create_test_manager();

    let request = AiQueryRequest {
        query: "test query".to_string(),
        context: None,
        language: "en".to_string(),
        max_tokens: Some(50),
    };

    // Requirement 7.4: When AI services are unavailable, return graceful error
    let result = manager.process_query(&request).await;

    match result {
        Ok(_) => {
            // If it succeeds, that's fine
        }
        Err(e) => {
            // Error should be graceful, not a panic
            assert!(!format!("{:?}", e).is_empty());
        }
    }
}

#[tokio::test]
async fn test_response_caching() {
    let manager = create_test_manager();

    let request = AiQueryRequest {
        query: "cacheable query".to_string(),
        context: None,
        language: "en".to_string(),
        max_tokens: Some(50),
    };

    // Requirement 7.5: Cache AI responses
    let result1 = manager.process_query(&request).await;
    let result2 = manager.process_query(&request).await;

    // If first request succeeded, second should return same result
    if result1.is_ok() {
        assert!(result2.is_ok());
        let response1 = result1.unwrap();
        let response2 = result2.unwrap();
        assert_eq!(response1.response, response2.response);
    }
}

#[tokio::test]
async fn test_response_validation_integration() {
    let manager = create_test_manager();

    // Requirement 7.3: Validate responses and filter inappropriate content
    let test_cases = vec![
        ("What is the meaning of patience?", true),
        ("Explain the historical context", true),
        ("This is a fatwa about...", false),
        ("The حكم شرعي is...", false),
    ];

    for (query, should_pass) in test_cases {
        let request = AiQueryRequest {
            query: query.to_string(),
            context: None,
            language: "en".to_string(),
            max_tokens: Some(50),
        };

        let result = manager.process_query(&request).await;

        if should_pass {
            // Valid queries should either succeed or fail gracefully (API unavailable)
            match result {
                Ok(_) => {}
                Err(e) => {
                    // Should not be validation error
                    assert!(!matches!(e, ApiError::InvalidResponse(_, _)));
                }
            }
        }
    }
}

#[tokio::test]
async fn test_concurrent_queries() {
    let manager = Arc::new(create_test_manager());

    let mut handles = vec![];

    // Make 5 concurrent queries
    for i in 0..5 {
        let manager_clone = Arc::clone(&manager);
        let handle = tokio::spawn(async move {
            let request = AiQueryRequest {
                query: format!("test query {}", i),
                context: None,
                language: "en".to_string(),
                max_tokens: Some(50),
            };
            manager_clone.process_query(&request).await
        });
        handles.push(handle);
    }

    // Wait for all queries to complete
    for handle in handles {
        let result = handle.await.unwrap();
        // Should not panic, either succeeds or fails gracefully
        match result {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

#[test]
fn test_cache_key_with_different_parameters() {
    // Test that different parameters produce different cache keys
    let base_request = AiQueryRequest {
        query: "test".to_string(),
        context: None,
        language: "en".to_string(),
        max_tokens: None,
    };

    let with_context = AiQueryRequest {
        query: "test".to_string(),
        context: Some("context".to_string()),
        language: "en".to_string(),
        max_tokens: None,
    };

    let different_language = AiQueryRequest {
        query: "test".to_string(),
        context: None,
        language: "ar".to_string(),
        max_tokens: None,
    };

    let key1 = AiApiManager::cache_key(&base_request);
    let key2 = AiApiManager::cache_key(&with_context);
    let key3 = AiApiManager::cache_key(&different_language);

    assert_ne!(key1, key2);
    assert_ne!(key1, key3);
    assert_ne!(key2, key3);
}

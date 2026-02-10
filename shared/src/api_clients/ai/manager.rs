// AI API Manager
//
// Manages AI/NLP API clients with fallback support
// NOTE: AI services are used ONLY for technical language processing

use crate::api_clients::{
    AiApiClient, AiQueryRequest, AiQueryResponse, ApiClient, ApiError, CacheCategory,
    CacheManager, RateLimiter,
};
use std::sync::Arc;

use super::HuggingFaceClient;

/// Manager for AI/NLP API clients
///
/// Handles fallback between multiple AI services and response validation
pub struct AiApiManager {
    clients: Vec<Box<dyn AiApiClient + Send + Sync>>,
    cache: Arc<CacheManager>,
    rate_limiter: Arc<RateLimiter>,
}

impl AiApiManager {
    /// Create a new AI API manager
    pub fn new(
        cache: Arc<CacheManager>,
        rate_limiter: Arc<RateLimiter>,
        hugging_face_api_key: Option<String>,
    ) -> Self {
        let clients: Vec<Box<dyn AiApiClient + Send + Sync>> = vec![
            Box::new(HuggingFaceClient::new(hugging_face_api_key)),
        ];

        Self {
            clients,
            cache,
            rate_limiter,
        }
    }

    /// Create a manager with custom clients (for testing)
    pub fn with_clients(
        clients: Vec<Box<dyn AiApiClient + Send + Sync>>,
        cache: Arc<CacheManager>,
        rate_limiter: Arc<RateLimiter>,
    ) -> Self {
        Self {
            clients,
            cache,
            rate_limiter,
        }
    }

    /// Generate cache key for AI query
    fn cache_key(request: &AiQueryRequest) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        request.query.hash(&mut hasher);
        if let Some(ref context) = request.context {
            context.hash(&mut hasher);
        }
        request.language.hash(&mut hasher);

        format!("ai_query:{:x}", hasher.finish())
    }

    /// Validate AI response for inappropriate content
    fn validate_response(&self, response: &str) -> Result<(), ApiError> {
        // Check for religious ruling keywords that should not be in AI responses
        let forbidden_keywords = vec![
            "fatwa",
            "حكم شرعي",
            "فتوى",
            "حلال",
            "حرام",
            "واجب",
            "مستحب",
            "مكروه",
        ];

        for keyword in forbidden_keywords {
            if response.to_lowercase().contains(&keyword.to_lowercase()) {
                log::warn!("AI response contained forbidden keyword: {}", keyword);
                return Err(ApiError::InvalidResponse(
                    "ai_manager".to_string(),
                    format!("Response contained inappropriate religious content: {}", keyword),
                ));
            }
        }

        Ok(())
    }

    /// Process an AI query
    ///
    /// This method:
    /// 1. Checks cache first
    /// 2. Tries each AI client in priority order
    /// 3. Validates responses for inappropriate content
    /// 4. Caches successful responses
    pub async fn process_query(
        &self,
        request: &AiQueryRequest,
    ) -> Result<AiQueryResponse, ApiError> {
        let cache_key = Self::cache_key(request);

        // 1. Check cache
        if let Ok(Some(cached)) = self
            .cache
            .get::<AiQueryResponse>(&cache_key, CacheCategory::AiResponse)
            .await
        {
            log::debug!("AI query cache hit");
            return Ok(cached);
        }

        log::debug!("AI query cache miss");

        // 2. Try each client in priority order
        let mut last_error = None;

        for client in &self.clients {
            // Check if client is healthy
            if !client.is_healthy().await {
                log::warn!("AI API {} is unhealthy, skipping", client.api_name());
                continue;
            }

            // Check rate limit
            match self.rate_limiter.check_and_increment(client.api_name()).await {
                Ok(allowed) => {
                    if !allowed {
                        log::warn!("Rate limit exceeded for AI API {}", client.api_name());
                        continue;
                    }
                }
                Err(e) => {
                    log::error!("Rate limiter error for {}: {}", client.api_name(), e);
                    continue;
                }
            }

            // Try to process query
            match client.process_query(request).await {
                Ok(response) => {
                    // Validate response
                    if let Err(e) = self.validate_response(&response.response) {
                        log::warn!(
                            "AI API {} returned invalid response: {}",
                            client.api_name(),
                            e
                        );
                        last_error = Some(e);
                        continue;
                    }

                    log::info!(
                        "Successfully processed AI query with {} (confidence: {})",
                        client.api_name(),
                        response.confidence
                    );

                    // Cache the response
                    if let Err(e) = self
                        .cache
                        .set(&cache_key, &response, CacheCategory::AiResponse)
                        .await
                    {
                        log::warn!("Failed to cache AI response: {}", e);
                    }

                    return Ok(response);
                }
                Err(e) => {
                    log::warn!("AI API {} failed: {}", client.api_name(), e);
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // 3. All clients failed
        Err(last_error.unwrap_or_else(|| {
            ApiError::AllApisFailed("All AI APIs failed or unavailable".to_string())
        }))
    }

    /// Get all available AI API clients
    pub fn get_clients(&self) -> &[Box<dyn AiApiClient + Send + Sync>] {
        &self.clients
    }

    /// Check health of all AI APIs
    pub async fn health_check(&self) -> Vec<(String, bool)> {
        let mut results = Vec::new();
        for client in &self.clients {
            let is_healthy = client.is_healthy().await;
            results.push((client.api_name().to_string(), is_healthy));
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_clients::MockRedisClient;

    fn create_test_manager() -> AiApiManager {
        let redis = Arc::new(MockRedisClient::new());
        let cache = Arc::new(CacheManager::new(redis.clone()));
        let rate_limiter = Arc::new(RateLimiter::new(redis));
        AiApiManager::new(cache, rate_limiter, None)
    }

    #[test]
    fn test_manager_creation() {
        let manager = create_test_manager();
        assert_eq!(manager.clients.len(), 1);
    }

    #[test]
    fn test_cache_key_generation() {
        let request1 = AiQueryRequest {
            query: "What is the meaning of this verse?".to_string(),
            context: None,
            language: "en".to_string(),
            max_tokens: None,
        };

        let request2 = AiQueryRequest {
            query: "What is the meaning of this verse?".to_string(),
            context: None,
            language: "en".to_string(),
            max_tokens: None,
        };

        let key1 = AiApiManager::cache_key(&request1);
        let key2 = AiApiManager::cache_key(&request2);
        assert_eq!(key1, key2);

        // Different query should have different key
        let request3 = AiQueryRequest {
            query: "Different query".to_string(),
            context: None,
            language: "en".to_string(),
            max_tokens: None,
        };
        let key3 = AiApiManager::cache_key(&request3);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_response_validation_allows_valid_content() {
        let manager = create_test_manager();

        let valid_responses = vec![
            "This verse talks about patience and perseverance.",
            "The historical context of this verse is important.",
            "This is a technical explanation of the Arabic grammar.",
        ];

        for response in valid_responses {
            let result = manager.validate_response(response);
            assert!(result.is_ok(), "Valid response rejected: {}", response);
        }
    }

    #[test]
    fn test_response_validation_blocks_religious_rulings() {
        let manager = create_test_manager();

        let invalid_responses = vec![
            "This is a fatwa about the matter.",
            "According to Islamic law, this is حلال.",
            "The حكم شرعي for this is...",
            "This action is حرام.",
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
    async fn test_health_check() {
        let manager = create_test_manager();
        let health = manager.health_check().await;

        assert_eq!(health.len(), 1);
        assert!(health[0].0.contains("hugging_face"));
    }

    #[tokio::test]
    async fn test_empty_query_rejected() {
        let manager = create_test_manager();

        let request = AiQueryRequest {
            query: "".to_string(),
            context: None,
            language: "en".to_string(),
            max_tokens: None,
        };

        let result = manager.process_query(request).await;
        // Should fail due to validation in the client
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cache_key_includes_context() {
        let request1 = AiQueryRequest {
            query: "test query".to_string(),
            context: Some("context A".to_string()),
            language: "en".to_string(),
            max_tokens: None,
        };

        let request2 = AiQueryRequest {
            query: "test query".to_string(),
            context: Some("context B".to_string()),
            language: "en".to_string(),
            max_tokens: None,
        };

        let key1 = AiApiManager::cache_key(&request1);
        let key2 = AiApiManager::cache_key(&request2);

        // Different context should produce different cache keys
        assert_ne!(key1, key2);
    }

    #[tokio::test]
    async fn test_cache_key_includes_language() {
        let request1 = AiQueryRequest {
            query: "test query".to_string(),
            context: None,
            language: "en".to_string(),
            max_tokens: None,
        };

        let request2 = AiQueryRequest {
            query: "test query".to_string(),
            context: None,
            language: "ar".to_string(),
            max_tokens: None,
        };

        let key1 = AiApiManager::cache_key(&request1);
        let key2 = AiApiManager::cache_key(&request2);

        // Different language should produce different cache keys
        assert_ne!(key1, key2);
    }
}

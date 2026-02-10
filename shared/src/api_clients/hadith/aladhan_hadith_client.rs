//! Aladhan Hadith API Client
//!
//! Hadith API from Aladhan (Islamic Network)
//! API Documentation: https://aladhan.com/

use crate::api_clients::{
    ApiClient, ApiError, HadithApiClient, HadithResult, RateLimitConfig,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

/// Aladhan Hadith API client
#[derive(Debug, Clone)]
pub struct AladhanHadithClient {
    base_url: String,
    client: Client,
}

impl AladhanHadithClient {
    /// Create a new Aladhan Hadith API client
    pub fn new() -> Self {
        Self {
            base_url: "https://api.aladhan.com/v1".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Create a client with custom base URL (for testing)
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }
}

impl Default for AladhanHadithClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ApiClient for AladhanHadithClient {
    fn api_name(&self) -> &str {
        "aladhan.hadith"
    }

    fn priority(&self) -> u8 {
        3 // Tertiary API
    }

    async fn is_healthy(&self) -> bool {
        // Simple health check - try to get a random hadith
        let url = format!("{}/hadithOfTheDay", self.base_url);
        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(e) => {
                tracing::warn!("Aladhan Hadith health check failed: {}", e);
                false
            }
        }
    }

    fn rate_limit(&self) -> RateLimitConfig {
        RateLimitConfig {
            requests_per_minute: 60,
            requests_per_hour: 1000,
            requests_per_day: 10000,
        }
    }
}

#[async_trait]
impl HadithApiClient for AladhanHadithClient {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<HadithResult>, ApiError> {
        if query.trim().is_empty() {
            return Err(ApiError::Validation(
                "Search query cannot be empty".to_string(),
            ));
        }

        if limit == 0 || limit > 100 {
            return Err(ApiError::Validation(format!(
                "Invalid limit: {}. Must be between 1 and 100",
                limit
            )));
        }

        // Note: Aladhan doesn't have a direct search endpoint
        // We'll use the hadith of the day endpoint as a fallback
        // In a real implementation, this would need to be enhanced
        let url = format!("{}/hadithOfTheDay", self.base_url);
        let response = self.client.get(&url).send().await.map_err(|e| {
            ApiError::Network(format!("Failed to get hadith from Aladhan: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!(
                    "HTTP {}: {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            ));
        }

        let hadith_response: AladhanHadithResponse =
            response.json().await.map_err(|e| {
                ApiError::InvalidResponse(
                    self.api_name().to_string(),
                    format!("Failed to parse hadith response: {}", e),
                )
            })?;

        if hadith_response.code != 200 {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("API returned code: {}", hadith_response.code),
            ));
        }

        let h = hadith_response.data;
        let result = HadithResult {
            id: format!("aladhan_{}", h.number),
            collection: h.collection.clone(),
            book: h.book.clone(),
            hadith_number: h.number.to_string(),
            text_arabic: h.text_arabic,
            text_translation: Some(h.text_english),
            grade: h.grade,
            narrator: h.narrator,
            source: self.api_name().to_string(),
        };

        // Return single result (Aladhan limitation)
        Ok(vec![result])
    }

    async fn get_by_id(&self, id: &str) -> Result<HadithResult, ApiError> {
        if id.trim().is_empty() {
            return Err(ApiError::Validation("Hadith ID cannot be empty".to_string()));
        }

        // Aladhan doesn't support get by ID directly
        // Return error indicating this limitation
        Err(ApiError::ApiError(
            self.api_name().to_string(),
            "Aladhan API does not support get by ID".to_string(),
        ))
    }

    async fn get_by_collection(
        &self,
        collection: &str,
        limit: usize,
    ) -> Result<Vec<HadithResult>, ApiError> {
        if collection.trim().is_empty() {
            return Err(ApiError::Validation(
                "Collection name cannot be empty".to_string(),
            ));
        }

        if limit == 0 || limit > 100 {
            return Err(ApiError::Validation(format!(
                "Invalid limit: {}. Must be between 1 and 100",
                limit
            )));
        }

        // Aladhan doesn't support get by collection directly
        // Return error indicating this limitation
        Err(ApiError::ApiError(
            self.api_name().to_string(),
            "Aladhan API does not support get by collection".to_string(),
        ))
    }
}

// ============================================================================
// Response structures for Aladhan Hadith API
// ============================================================================

#[derive(Debug, Deserialize)]
struct AladhanHadithResponse {
    code: u16,
    status: String,
    data: AladhanHadith,
}

#[derive(Debug, Deserialize)]
struct AladhanHadith {
    number: u32,
    #[serde(rename = "hadithArabic")]
    text_arabic: String,
    #[serde(rename = "hadithEnglish")]
    text_english: String,
    collection: String,
    book: String,
    grade: Option<String>,
    narrator: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = AladhanHadithClient::new();
        assert_eq!(client.api_name(), "aladhan.hadith");
        assert_eq!(client.priority(), 3);
    }

    #[test]
    fn test_default_creation() {
        let client = AladhanHadithClient::default();
        assert_eq!(client.api_name(), "aladhan.hadith");
    }

    #[test]
    fn test_rate_limit_config() {
        let client = AladhanHadithClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.requests_per_hour, 1000);
        assert_eq!(config.requests_per_day, 10000);
    }

    #[tokio::test]
    async fn test_empty_search_query() {
        let client = AladhanHadithClient::new();
        let result = client.search("", 10).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_invalid_limit() {
        let client = AladhanHadithClient::new();
        
        // Test limit 0
        let result = client.search("test", 0).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
        
        // Test limit > 100
        let result = client.search("test", 101).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_empty_hadith_id() {
        let client = AladhanHadithClient::new();
        let result = client.get_by_id("").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_get_by_id_not_supported() {
        let client = AladhanHadithClient::new();
        let result = client.get_by_id("123").await;
        assert!(result.is_err());
        // Should return ApiError indicating not supported
        assert!(matches!(result.unwrap_err(), ApiError::ApiError(_, _)));
    }

    #[tokio::test]
    async fn test_empty_collection_name() {
        let client = AladhanHadithClient::new();
        let result = client.get_by_collection("", 10).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_get_by_collection_not_supported() {
        let client = AladhanHadithClient::new();
        let result = client.get_by_collection("bukhari", 10).await;
        assert!(result.is_err());
        // Should return ApiError indicating not supported
        assert!(matches!(result.unwrap_err(), ApiError::ApiError(_, _)));
    }
}

//! Hadith API Client
//!
//! Generic Hadith API client for accessing hadith collections
//! from various sources.

use crate::api_clients::{
    ApiClient, ApiError, HadithApiClient, HadithResult, RateLimitConfig,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

/// Generic Hadith API client
#[derive(Debug, Clone)]
pub struct HadithApiClientImpl {
    base_url: String,
    client: Client,
    api_key: Option<String>,
}

impl HadithApiClientImpl {
    /// Create a new Hadith API client
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        Self {
            base_url,
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .expect("Failed to create HTTP client"),
            api_key,
        }
    }

    /// Create a default client pointing to a public hadith API
    pub fn default() -> Self {
        Self::new(
            "https://api.hadith.gading.dev".to_string(),
            None,
        )
    }
}

#[async_trait]
impl ApiClient for HadithApiClientImpl {
    fn api_name(&self) -> &str {
        "hadith.api"
    }

    fn priority(&self) -> u8 {
        2 // Secondary API
    }

    async fn is_healthy(&self) -> bool {
        // Simple health check - try to get books list
        let url = format!("{}/books", self.base_url);
        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(e) => {
                tracing::warn!("Hadith API health check failed: {}", e);
                false
            }
        }
    }

    fn rate_limit(&self) -> RateLimitConfig {
        RateLimitConfig {
            requests_per_minute: 30,
            requests_per_hour: 500,
            requests_per_day: 5000,
        }
    }
}

#[async_trait]
impl HadithApiClient for HadithApiClientImpl {
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

        let url = format!("{}/hadiths", self.base_url);
        let mut request = self.client.get(&url).query(&[
            ("query", query),
            ("limit", &limit.to_string()),
        ]);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await.map_err(|e| {
            ApiError::Network(format!("Failed to search hadith: {}", e))
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

        let search_response: HadithApiSearchResponse =
            response.json().await.map_err(|e| {
                ApiError::InvalidResponse(
                    self.api_name().to_string(),
                    format!("Failed to parse search response: {}", e),
                )
            })?;

        let results = search_response
            .data
            .into_iter()
            .map(|h| HadithResult {
                id: format!("hadith_api_{}", h.number),
                collection: h.name.clone(),
                book: "".to_string(), // Not provided by this API
                hadith_number: h.number.to_string(),
                text_arabic: h.arab,
                text_translation: Some(h.id),
                grade: None, // Not provided by this API
                narrator: "".to_string(), // Not provided by this API
                source: self.api_name().to_string(),
            })
            .collect();

        Ok(results)
    }

    async fn get_by_id(&self, id: &str) -> Result<HadithResult, ApiError> {
        if id.trim().is_empty() {
            return Err(ApiError::Validation("Hadith ID cannot be empty".to_string()));
        }

        // Extract the actual ID if it has our prefix
        let actual_id = id.strip_prefix("hadith_api_").unwrap_or(id);

        let url = format!("{}/hadiths/{}", self.base_url, actual_id);
        let mut request = self.client.get(&url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await.map_err(|e| {
            ApiError::Network(format!("Failed to get hadith: {}", e))
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

        let hadith_response: HadithApiHadithResponse =
            response.json().await.map_err(|e| {
                ApiError::InvalidResponse(
                    self.api_name().to_string(),
                    format!("Failed to parse hadith response: {}", e),
                )
            })?;

        let h = hadith_response.data;
        Ok(HadithResult {
            id: format!("hadith_api_{}", h.number),
            collection: h.name.clone(),
            book: "".to_string(),
            hadith_number: h.number.to_string(),
            text_arabic: h.arab,
            text_translation: Some(h.id),
            grade: None,
            narrator: "".to_string(),
            source: self.api_name().to_string(),
        })
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

        let url = format!("{}/books/{}", self.base_url, collection);
        let mut request = self.client.get(&url).query(&[("limit", &limit.to_string())]);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await.map_err(|e| {
            ApiError::Network(format!("Failed to get collection: {}", e))
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

        let collection_response: HadithApiSearchResponse =
            response.json().await.map_err(|e| {
                ApiError::InvalidResponse(
                    self.api_name().to_string(),
                    format!("Failed to parse collection response: {}", e),
                )
            })?;

        let results = collection_response
            .data
            .into_iter()
            .map(|h| HadithResult {
                id: format!("hadith_api_{}", h.number),
                collection: h.name.clone(),
                book: "".to_string(),
                hadith_number: h.number.to_string(),
                text_arabic: h.arab,
                text_translation: Some(h.id),
                grade: None,
                narrator: "".to_string(),
                source: self.api_name().to_string(),
            })
            .collect();

        Ok(results)
    }
}

// ============================================================================
// Response structures for Hadith API
// ============================================================================

#[derive(Debug, Deserialize)]
struct HadithApiSearchResponse {
    data: Vec<HadithApiHadith>,
}

#[derive(Debug, Deserialize)]
struct HadithApiHadithResponse {
    data: HadithApiHadith,
}

#[derive(Debug, Deserialize)]
struct HadithApiHadith {
    number: u32,
    arab: String,
    id: String, // Indonesian translation
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = HadithApiClientImpl::default();
        assert_eq!(client.api_name(), "hadith.api");
        assert_eq!(client.priority(), 2);
    }

    #[test]
    fn test_rate_limit_config() {
        let client = HadithApiClientImpl::default();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 30);
        assert_eq!(config.requests_per_hour, 500);
        assert_eq!(config.requests_per_day, 5000);
    }

    #[tokio::test]
    async fn test_empty_search_query() {
        let client = HadithApiClientImpl::default();
        let result = client.search("", 10).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_invalid_limit() {
        let client = HadithApiClientImpl::default();
        
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
        let client = HadithApiClientImpl::default();
        let result = client.get_by_id("").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_empty_collection_name() {
        let client = HadithApiClientImpl::default();
        let result = client.get_by_collection("", 10).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }
}

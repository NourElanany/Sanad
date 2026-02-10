//! Sunnah.com API Client
//!
//! Official API from Sunnah.com providing authenticated hadith collections
//! with proper chains of narration.
//! API Documentation: https://sunnah.com/

use crate::api_clients::{
    ApiClient, ApiError, HadithApiClient, HadithResult, RateLimitConfig,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

/// Sunnah.com API client
#[derive(Debug, Clone)]
pub struct SunnahComClient {
    base_url: String,
    client: Client,
    api_key: String,
}

impl SunnahComClient {
    /// Create a new Sunnah.com API client
    pub fn new(api_key: String) -> Self {
        Self {
            base_url: "https://api.sunnah.com/v1".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .expect("Failed to create HTTP client"),
            api_key,
        }
    }

    /// Create a client with custom base URL (for testing)
    pub fn with_base_url(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .expect("Failed to create HTTP client"),
            api_key,
        }
    }

    /// Helper to compute content hash for deduplication
    fn compute_hash(text: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

#[async_trait]
impl ApiClient for SunnahComClient {
    fn api_name(&self) -> &str {
        "sunnah.com"
    }

    fn priority(&self) -> u8 {
        1 // Primary API for hadith
    }

    async fn is_healthy(&self) -> bool {
        // Simple health check - try to search for a common term
        match self.search("prophet", 1).await {
            Ok(_) => true,
            Err(e) => {
                tracing::warn!("Sunnah.com health check failed: {}", e);
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
impl HadithApiClient for SunnahComClient {
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
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .query(&[("q", query), ("limit", &limit.to_string())])
            .send()
            .await
            .map_err(|e| {
                ApiError::Network(format!("Failed to search hadith on Sunnah.com: {}", e))
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

        let search_response: SunnahComSearchResponse =
            response.json().await.map_err(|e| {
                ApiError::InvalidResponse(
                    self.api_name().to_string(),
                    format!("Failed to parse search response: {}", e),
                )
            })?;

        let results = search_response
            .hadiths
            .into_iter()
            .map(|h| HadithResult {
                id: format!("sunnah_com_{}", h.id),
                collection: h.collection,
                book: h.book,
                hadith_number: h.hadith_number,
                text_arabic: h.text_arabic,
                text_translation: Some(h.text_english),
                grade: h.grade,
                narrator: h.narrator,
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
        let actual_id = id.strip_prefix("sunnah_com_").unwrap_or(id);

        let url = format!("{}/hadiths/{}", self.base_url, actual_id);
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await
            .map_err(|e| {
                ApiError::Network(format!("Failed to get hadith from Sunnah.com: {}", e))
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

        let hadith_response: SunnahComHadithResponse =
            response.json().await.map_err(|e| {
                ApiError::InvalidResponse(
                    self.api_name().to_string(),
                    format!("Failed to parse hadith response: {}", e),
                )
            })?;

        let h = hadith_response.hadith;
        Ok(HadithResult {
            id: format!("sunnah_com_{}", h.id),
            collection: h.collection,
            book: h.book,
            hadith_number: h.hadith_number,
            text_arabic: h.text_arabic,
            text_translation: Some(h.text_english),
            grade: h.grade,
            narrator: h.narrator,
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

        let url = format!("{}/collections/{}/hadiths", self.base_url, collection);
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .query(&[("limit", &limit.to_string())])
            .send()
            .await
            .map_err(|e| {
                ApiError::Network(format!(
                    "Failed to get collection from Sunnah.com: {}",
                    e
                ))
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

        let collection_response: SunnahComSearchResponse =
            response.json().await.map_err(|e| {
                ApiError::InvalidResponse(
                    self.api_name().to_string(),
                    format!("Failed to parse collection response: {}", e),
                )
            })?;

        let results = collection_response
            .hadiths
            .into_iter()
            .map(|h| HadithResult {
                id: format!("sunnah_com_{}", h.id),
                collection: h.collection,
                book: h.book,
                hadith_number: h.hadith_number,
                text_arabic: h.text_arabic,
                text_translation: Some(h.text_english),
                grade: h.grade,
                narrator: h.narrator,
                source: self.api_name().to_string(),
            })
            .collect();

        Ok(results)
    }
}

// ============================================================================
// Response structures for Sunnah.com API
// ============================================================================

#[derive(Debug, Deserialize)]
struct SunnahComSearchResponse {
    hadiths: Vec<SunnahComHadith>,
}

#[derive(Debug, Deserialize)]
struct SunnahComHadithResponse {
    hadith: SunnahComHadith,
}

#[derive(Debug, Deserialize)]
struct SunnahComHadith {
    id: String,
    collection: String,
    book: String,
    #[serde(rename = "hadithNumber")]
    hadith_number: String,
    #[serde(rename = "arabicText")]
    text_arabic: String,
    #[serde(rename = "englishText")]
    text_english: String,
    grade: Option<String>,
    narrator: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = SunnahComClient::new("test_key".to_string());
        assert_eq!(client.api_name(), "sunnah.com");
        assert_eq!(client.priority(), 1);
    }

    #[test]
    fn test_rate_limit_config() {
        let client = SunnahComClient::new("test_key".to_string());
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 30);
        assert_eq!(config.requests_per_hour, 500);
        assert_eq!(config.requests_per_day, 5000);
    }

    #[tokio::test]
    async fn test_empty_search_query() {
        let client = SunnahComClient::new("test_key".to_string());
        let result = client.search("", 10).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_invalid_limit() {
        let client = SunnahComClient::new("test_key".to_string());
        
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
        let client = SunnahComClient::new("test_key".to_string());
        let result = client.get_by_id("").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_empty_collection_name() {
        let client = SunnahComClient::new("test_key".to_string());
        let result = client.get_by_collection("", 10).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }
}

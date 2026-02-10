//! Quran.com / Quran Foundation API Client
//!
//! Official API from Quran Foundation providing Quran text, translations, and recitations.
//! API Documentation: https://api-docs.quran.foundation/

use crate::api_clients::{
    ApiClient, ApiError, AyahData, PageData, QuranApiClient, RateLimitConfig, SurahData,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Quran.com API client
#[derive(Debug, Clone)]
pub struct QuranComClient {
    base_url: String,
    client: Client,
    api_key: Option<String>,
}

impl QuranComClient {
    /// Create a new Quran.com API client
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            base_url: "https://api.quran.com/api/v4".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            api_key,
        }
    }

    /// Create a client with custom base URL (for testing)
    pub fn with_base_url(base_url: String, api_key: Option<String>) -> Self {
        Self {
            base_url,
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            api_key,
        }
    }
}

#[async_trait]
impl ApiClient for QuranComClient {
    fn api_name(&self) -> &str {
        "quran.com"
    }

    fn priority(&self) -> u8 {
        1 // Primary API
    }

    async fn is_healthy(&self) -> bool {
        // Simple health check - try to get a single verse
        match self.get_ayah(1, 1).await {
            Ok(_) => true,
            Err(e) => {
                tracing::warn!("Quran.com health check failed: {}", e);
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
impl QuranApiClient for QuranComClient {
    async fn get_surah(&self, surah_number: u8) -> Result<SurahData, ApiError> {
        if surah_number < 1 || surah_number > 114 {
            return Err(ApiError::Validation(format!(
                "Invalid surah number: {}. Must be between 1 and 114",
                surah_number
            )));
        }

        let url = format!("{}/chapters/{}", self.base_url, surah_number);
        let mut request = self.client.get(&url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await.map_err(|e| {
            ApiError::Network(format!("Failed to fetch surah from Quran.com: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default()),
            ));
        }

        let chapter_response: QuranComChapterResponse = response.json().await.map_err(|e| {
            ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("Failed to parse chapter response: {}", e),
            )
        })?;

        // Now fetch all verses for this surah
        let verses_url = format!("{}/verses/by_chapter/{}", self.base_url, surah_number);
        let mut verses_request = self.client.get(&verses_url);

        if let Some(ref key) = self.api_key {
            verses_request = verses_request.header("Authorization", format!("Bearer {}", key));
        }

        let verses_response = verses_request.send().await.map_err(|e| {
            ApiError::Network(format!("Failed to fetch verses from Quran.com: {}", e))
        })?;

        if !verses_response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("HTTP {}: {}", verses_response.status(), verses_response.text().await.unwrap_or_default()),
            ));
        }

        let verses_data: QuranComVersesResponse = verses_response.json().await.map_err(|e| {
            ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("Failed to parse verses response: {}", e),
            )
        })?;

        let ayahs = verses_data
            .verses
            .into_iter()
            .map(|v| AyahData {
                surah: surah_number,
                ayah: v.verse_number,
                text_arabic: v.text_uthmani,
                text_translation: None, // Translations require separate API call
            })
            .collect();

        Ok(SurahData {
            number: surah_number,
            name_arabic: chapter_response.chapter.name_arabic,
            name_english: chapter_response.chapter.name_simple,
            ayahs,
        })
    }

    async fn get_ayah(&self, surah: u8, ayah: u16) -> Result<AyahData, ApiError> {
        if surah < 1 || surah > 114 {
            return Err(ApiError::Validation(format!(
                "Invalid surah number: {}. Must be between 1 and 114",
                surah
            )));
        }

        if ayah < 1 {
            return Err(ApiError::Validation(format!(
                "Invalid ayah number: {}. Must be at least 1",
                ayah
            )));
        }

        let url = format!("{}/verses/by_key/{}:{}", self.base_url, surah, ayah);
        let mut request = self.client.get(&url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await.map_err(|e| {
            ApiError::Network(format!("Failed to fetch ayah from Quran.com: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default()),
            ));
        }

        let verse_response: QuranComVerseResponse = response.json().await.map_err(|e| {
            ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("Failed to parse verse response: {}", e),
            )
        })?;

        Ok(AyahData {
            surah,
            ayah,
            text_arabic: verse_response.verse.text_uthmani,
            text_translation: None,
        })
    }

    async fn get_page(&self, page: u16) -> Result<PageData, ApiError> {
        if page < 1 || page > 604 {
            return Err(ApiError::Validation(format!(
                "Invalid page number: {}. Must be between 1 and 604",
                page
            )));
        }

        let url = format!("{}/verses/by_page/{}", self.base_url, page);
        let mut request = self.client.get(&url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await.map_err(|e| {
            ApiError::Network(format!("Failed to fetch page from Quran.com: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default()),
            ));
        }

        let page_response: QuranComVersesResponse = response.json().await.map_err(|e| {
            ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("Failed to parse page response: {}", e),
            )
        })?;

        let ayahs = page_response
            .verses
            .into_iter()
            .map(|v| AyahData {
                surah: v.chapter_id,
                ayah: v.verse_number,
                text_arabic: v.text_uthmani,
                text_translation: None,
            })
            .collect();

        Ok(PageData {
            page_number: page,
            ayahs,
        })
    }
}

// ============================================================================
// Response structures for Quran.com API
// ============================================================================

#[derive(Debug, Deserialize)]
struct QuranComChapterResponse {
    chapter: QuranComChapter,
}

#[derive(Debug, Deserialize)]
struct QuranComChapter {
    name_simple: String,
    name_arabic: String,
}

#[derive(Debug, Deserialize)]
struct QuranComVerseResponse {
    verse: QuranComVerse,
}

#[derive(Debug, Deserialize)]
struct QuranComVersesResponse {
    verses: Vec<QuranComVerse>,
}

#[derive(Debug, Deserialize)]
struct QuranComVerse {
    verse_number: u16,
    chapter_id: u8,
    text_uthmani: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = QuranComClient::new(None);
        assert_eq!(client.api_name(), "quran.com");
        assert_eq!(client.priority(), 1);
    }

    #[test]
    fn test_rate_limit_config() {
        let client = QuranComClient::new(None);
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.requests_per_hour, 1000);
        assert_eq!(config.requests_per_day, 10000);
    }

    #[tokio::test]
    async fn test_invalid_surah_number() {
        let client = QuranComClient::new(None);
        
        // Test surah number 0
        let result = client.get_surah(0).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
        
        // Test surah number 115
        let result = client.get_surah(115).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_invalid_ayah_number() {
        let client = QuranComClient::new(None);
        
        // Test ayah number 0
        let result = client.get_ayah(1, 0).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_invalid_page_number() {
        let client = QuranComClient::new(None);
        
        // Test page number 0
        let result = client.get_page(0).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
        
        // Test page number 605
        let result = client.get_page(605).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }
}

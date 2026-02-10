//! AlQuran Cloud API Client
//!
//! Community-trusted API providing Quran text, translations, and audio recitations.
//! API Documentation: https://alquran.cloud/api

use crate::api_clients::{
    ApiClient, ApiError, AyahData, PageData, QuranApiClient, RateLimitConfig, SurahData,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// AlQuran Cloud API client
#[derive(Debug, Clone)]
pub struct AlquranCloudClient {
    base_url: String,
    client: Client,
    edition: String, // Default: quran-uthmani (Uthmanic script)
}

impl AlquranCloudClient {
    /// Create a new AlQuran Cloud API client with default edition (quran-uthmani)
    pub fn new() -> Self {
        Self {
            base_url: "https://api.alquran.cloud/v1".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            edition: "quran-uthmani".to_string(),
        }
    }

    /// Create a client with custom edition
    pub fn with_edition(edition: String) -> Self {
        Self {
            base_url: "https://api.alquran.cloud/v1".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            edition,
        }
    }

    /// Create a client with custom base URL (for testing)
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            edition: "quran-uthmani".to_string(),
        }
    }
}

impl Default for AlquranCloudClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ApiClient for AlquranCloudClient {
    fn api_name(&self) -> &str {
        "alquran.cloud"
    }

    fn priority(&self) -> u8 {
        2 // Secondary API
    }

    async fn is_healthy(&self) -> bool {
        // Simple health check - try to get a single verse
        match self.get_ayah(1, 1).await {
            Ok(_) => true,
            Err(e) => {
                tracing::warn!("AlQuran Cloud health check failed: {}", e);
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
impl QuranApiClient for AlquranCloudClient {
    async fn get_surah(&self, surah_number: u8) -> Result<SurahData, ApiError> {
        if surah_number < 1 || surah_number > 114 {
            return Err(ApiError::Validation(format!(
                "Invalid surah number: {}. Must be between 1 and 114",
                surah_number
            )));
        }

        let url = format!("{}/surah/{}/{}", self.base_url, surah_number, self.edition);
        let response = self.client.get(&url).send().await.map_err(|e| {
            ApiError::Network(format!("Failed to fetch surah from AlQuran Cloud: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default()),
            ));
        }

        let surah_response: AlquranCloudSurahResponse = response.json().await.map_err(|e| {
            ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("Failed to parse surah response: {}", e),
            )
        })?;

        if surah_response.code != 200 {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("API returned code {}: {}", surah_response.code, surah_response.status),
            ));
        }

        let ayahs = surah_response
            .data
            .ayahs
            .into_iter()
            .map(|a| AyahData {
                surah: surah_number,
                ayah: a.number_in_surah,
                text_arabic: a.text,
                text_translation: None,
            })
            .collect();

        Ok(SurahData {
            number: surah_number,
            name_arabic: surah_response.data.name,
            name_english: surah_response.data.english_name,
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

        let url = format!("{}/ayah/{}:{}/{}", self.base_url, surah, ayah, self.edition);
        let response = self.client.get(&url).send().await.map_err(|e| {
            ApiError::Network(format!("Failed to fetch ayah from AlQuran Cloud: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default()),
            ));
        }

        let ayah_response: AlquranCloudAyahResponse = response.json().await.map_err(|e| {
            ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("Failed to parse ayah response: {}", e),
            )
        })?;

        if ayah_response.code != 200 {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("API returned code {}: {}", ayah_response.code, ayah_response.status),
            ));
        }

        Ok(AyahData {
            surah,
            ayah,
            text_arabic: ayah_response.data.text,
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

        let url = format!("{}/page/{}/{}", self.base_url, page, self.edition);
        let response = self.client.get(&url).send().await.map_err(|e| {
            ApiError::Network(format!("Failed to fetch page from AlQuran Cloud: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("HTTP {}: {}", response.status(), response.text().await.unwrap_or_default()),
            ));
        }

        let page_response: AlquranCloudPageResponse = response.json().await.map_err(|e| {
            ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("Failed to parse page response: {}", e),
            )
        })?;

        if page_response.code != 200 {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("API returned code {}: {}", page_response.code, page_response.status),
            ));
        }

        let ayahs = page_response
            .data
            .ayahs
            .into_iter()
            .map(|a| AyahData {
                surah: a.surah.number,
                ayah: a.number_in_surah,
                text_arabic: a.text,
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
// Response structures for AlQuran Cloud API
// ============================================================================

#[derive(Debug, Deserialize)]
struct AlquranCloudSurahResponse {
    code: u16,
    status: String,
    data: AlquranCloudSurah,
}

#[derive(Debug, Deserialize)]
struct AlquranCloudSurah {
    number: u8,
    name: String,
    #[serde(rename = "englishName")]
    english_name: String,
    ayahs: Vec<AlquranCloudAyah>,
}

#[derive(Debug, Deserialize)]
struct AlquranCloudAyahResponse {
    code: u16,
    status: String,
    data: AlquranCloudAyah,
}

#[derive(Debug, Deserialize)]
struct AlquranCloudPageResponse {
    code: u16,
    status: String,
    data: AlquranCloudPage,
}

#[derive(Debug, Deserialize)]
struct AlquranCloudPage {
    number: u16,
    ayahs: Vec<AlquranCloudPageAyah>,
}

#[derive(Debug, Deserialize)]
struct AlquranCloudAyah {
    number: u16,
    #[serde(rename = "numberInSurah")]
    number_in_surah: u16,
    text: String,
}

#[derive(Debug, Deserialize)]
struct AlquranCloudPageAyah {
    number: u16,
    #[serde(rename = "numberInSurah")]
    number_in_surah: u16,
    text: String,
    surah: AlquranCloudSurahInfo,
}

#[derive(Debug, Deserialize)]
struct AlquranCloudSurahInfo {
    number: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = AlquranCloudClient::new();
        assert_eq!(client.api_name(), "alquran.cloud");
        assert_eq!(client.priority(), 2);
        assert_eq!(client.edition, "quran-uthmani");
    }

    #[test]
    fn test_client_with_custom_edition() {
        let client = AlquranCloudClient::with_edition("en.sahih".to_string());
        assert_eq!(client.edition, "en.sahih");
    }

    #[test]
    fn test_rate_limit_config() {
        let client = AlquranCloudClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 30);
        assert_eq!(config.requests_per_hour, 500);
        assert_eq!(config.requests_per_day, 5000);
    }

    #[tokio::test]
    async fn test_invalid_surah_number() {
        let client = AlquranCloudClient::new();
        
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
        let client = AlquranCloudClient::new();
        
        // Test ayah number 0
        let result = client.get_ayah(1, 0).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_invalid_page_number() {
        let client = AlquranCloudClient::new();
        
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

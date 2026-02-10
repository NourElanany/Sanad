//! Tanzil.net API Client
//!
//! Official API providing highly verified precise Quran text in Unicode.
//! Tanzil is an international Quranic project focused on accuracy.
//! Website: https://tanzil.net/

use crate::api_clients::{
    ApiClient, ApiError, AyahData, PageData, QuranApiClient, RateLimitConfig, SurahData,
};
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

/// Tanzil.net API client
/// 
/// Note: Tanzil provides downloadable Quran text files rather than a REST API.
/// This client fetches the text file and parses it for the requested data.
#[derive(Debug, Clone)]
pub struct TanzilClient {
    base_url: String,
    client: Client,
    text_type: String, // Default: "uthmani" (Uthmanic script)
}

impl TanzilClient {
    /// Create a new Tanzil API client with default text type (uthmani)
    pub fn new() -> Self {
        Self {
            base_url: "https://tanzil.net/trans/".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .expect("Failed to create HTTP client"),
            text_type: "uthmani".to_string(),
        }
    }

    /// Create a client with custom text type
    pub fn with_text_type(text_type: String) -> Self {
        Self {
            base_url: "https://tanzil.net/trans/".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .expect("Failed to create HTTP client"),
            text_type,
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
            text_type: "uthmani".to_string(),
        }
    }

    /// Get the Quran text URL for downloading
    fn get_quran_text_url(&self) -> String {
        format!("{}quran-{}.txt", self.base_url, self.text_type)
    }

    /// Parse a Tanzil text line (format: surah|ayah|text)
    pub fn parse_line(&self, line: &str) -> Option<(u8, u16, String)> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 3 {
            let surah = parts[0].parse::<u8>().ok()?;
            let ayah = parts[1].parse::<u16>().ok()?;
            let text = parts[2].to_string();
            Some((surah, ayah, text))
        } else {
            None
        }
    }
}

impl Default for TanzilClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ApiClient for TanzilClient {
    fn api_name(&self) -> &str {
        "tanzil.net"
    }

    fn priority(&self) -> u8 {
        3 // Tertiary API
    }

    async fn is_healthy(&self) -> bool {
        // Simple health check - try to fetch a single verse
        match self.get_ayah(1, 1).await {
            Ok(_) => true,
            Err(e) => {
                tracing::warn!("Tanzil health check failed: {}", e);
                false
            }
        }
    }

    fn rate_limit(&self) -> RateLimitConfig {
        RateLimitConfig {
            requests_per_minute: 20,
            requests_per_hour: 300,
            requests_per_day: 3000,
        }
    }
}

#[async_trait]
impl QuranApiClient for TanzilClient {
    async fn get_surah(&self, surah_number: u8) -> Result<SurahData, ApiError> {
        if surah_number < 1 || surah_number > 114 {
            return Err(ApiError::Validation(format!(
                "Invalid surah number: {}. Must be between 1 and 114",
                surah_number
            )));
        }

        // For Tanzil, we need to fetch the entire Quran text and filter
        // In a production system, this should be cached
        let url = self.get_quran_text_url();
        let response = self.client.get(&url).send().await.map_err(|e| {
            ApiError::Network(format!("Failed to fetch Quran text from Tanzil: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("HTTP {}", response.status()),
            ));
        }

        let text = response.text().await.map_err(|e| {
            ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("Failed to read response text: {}", e),
            )
        })?;

        let mut ayahs = Vec::new();
        for line in text.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue; // Skip comments and empty lines
            }

            if let Some((surah, ayah, text)) = self.parse_line(line) {
                if surah == surah_number {
                    ayahs.push(AyahData {
                        surah,
                        ayah,
                        text_arabic: text,
                        text_translation: None,
                    });
                }
            }
        }

        if ayahs.is_empty() {
            return Err(ApiError::NotFound);
        }

        // Get surah names (hardcoded for now, could be fetched from metadata)
        let (name_arabic, name_english) = get_surah_names(surah_number);

        Ok(SurahData {
            number: surah_number,
            name_arabic: name_arabic.to_string(),
            name_english: name_english.to_string(),
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

        // Fetch the entire Quran text and find the specific ayah
        let url = self.get_quran_text_url();
        let response = self.client.get(&url).send().await.map_err(|e| {
            ApiError::Network(format!("Failed to fetch Quran text from Tanzil: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("HTTP {}", response.status()),
            ));
        }

        let text = response.text().await.map_err(|e| {
            ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("Failed to read response text: {}", e),
            )
        })?;

        for line in text.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }

            if let Some((s, a, t)) = self.parse_line(line) {
                if s == surah && a == ayah {
                    return Ok(AyahData {
                        surah,
                        ayah,
                        text_arabic: t,
                        text_translation: None,
                    });
                }
            }
        }

        Err(ApiError::NotFound)
    }

    async fn get_page(&self, page: u16) -> Result<PageData, ApiError> {
        if page < 1 || page > 604 {
            return Err(ApiError::Validation(format!(
                "Invalid page number: {}. Must be between 1 and 604",
                page
            )));
        }

        // Tanzil doesn't provide page-based access directly
        // This would require a mapping of verses to pages
        // For now, return an error indicating this feature is not supported
        Err(ApiError::ApiError(
            self.api_name().to_string(),
            "Page-based access not supported by Tanzil API".to_string(),
        ))
    }
}

/// Get surah names (Arabic and English)
/// This is a simplified version - in production, this should come from a database or config
fn get_surah_names(surah_number: u8) -> (&'static str, &'static str) {
    match surah_number {
        1 => ("الفاتحة", "Al-Fatihah"),
        2 => ("البقرة", "Al-Baqarah"),
        3 => ("آل عمران", "Ali 'Imran"),
        4 => ("النساء", "An-Nisa"),
        5 => ("المائدة", "Al-Ma'idah"),
        // Add more surahs as needed...
        _ => ("", "Unknown"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = TanzilClient::new();
        assert_eq!(client.api_name(), "tanzil.net");
        assert_eq!(client.priority(), 3);
        assert_eq!(client.text_type, "uthmani");
    }

    #[test]
    fn test_client_with_custom_text_type() {
        let client = TanzilClient::with_text_type("simple".to_string());
        assert_eq!(client.text_type, "simple");
    }

    #[test]
    fn test_rate_limit_config() {
        let client = TanzilClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 20);
        assert_eq!(config.requests_per_hour, 300);
        assert_eq!(config.requests_per_day, 3000);
    }

    #[test]
    fn test_parse_line() {
        let client = TanzilClient::new();
        
        let line = "1|1|بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
        let result = client.parse_line(line);
        assert!(result.is_some());
        
        let (surah, ayah, text) = result.unwrap();
        assert_eq!(surah, 1);
        assert_eq!(ayah, 1);
        assert_eq!(text, "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ");
    }

    #[test]
    fn test_parse_invalid_line() {
        let client = TanzilClient::new();
        
        let line = "invalid line";
        let result = client.parse_line(line);
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_invalid_surah_number() {
        let client = TanzilClient::new();
        
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
        let client = TanzilClient::new();
        
        // Test ayah number 0
        let result = client.get_ayah(1, 0).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_invalid_page_number() {
        let client = TanzilClient::new();
        
        // Test page number 0
        let result = client.get_page(0).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
        
        // Test page number 605
        let result = client.get_page(605).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[test]
    fn test_get_surah_names() {
        let (arabic, english) = get_surah_names(1);
        assert_eq!(arabic, "الفاتحة");
        assert_eq!(english, "Al-Fatihah");
        
        let (arabic, english) = get_surah_names(2);
        assert_eq!(arabic, "البقرة");
        assert_eq!(english, "Al-Baqarah");
    }
}

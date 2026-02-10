//! EveryAyah.com API Client
//!
//! Provides verse-by-verse audio recitations from authentic reciters.
//! Website: https://everyayah.com/

use crate::api_clients::{
    ApiClient, ApiError, AyahData, PageData, QuranApiClient, RateLimitConfig, SurahData,
};
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

/// EveryAyah.com API client
/// 
/// This client provides audio recitation URLs for Quran verses.
/// It doesn't provide text, so text-based methods will return errors.
#[derive(Debug, Clone)]
pub struct EveryayahClient {
    base_url: String,
    client: Client,
    reciter: String, // Default reciter subdirectory
}

impl EveryayahClient {
    /// Create a new EveryAyah client with default reciter (Abdul_Basit_Murattal_192kbps)
    pub fn new() -> Self {
        Self {
            base_url: "https://everyayah.com/data".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            reciter: "Abdul_Basit_Murattal_192kbps".to_string(),
        }
    }

    /// Create a client with custom reciter
    pub fn with_reciter(reciter: String) -> Self {
        Self {
            base_url: "https://everyayah.com/data".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            reciter,
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
            reciter: "Abdul_Basit_Murattal_192kbps".to_string(),
        }
    }

    /// Get the audio URL for a specific ayah
    /// Format: {base_url}/{reciter}/{surah:03d}{ayah:03d}.mp3
    pub fn get_audio_url(&self, surah: u8, ayah: u16) -> String {
        format!(
            "{}/{}/{:03}{:03}.mp3",
            self.base_url, self.reciter, surah, ayah
        )
    }

    /// Check if an audio file exists by making a HEAD request
    async fn audio_exists(&self, url: &str) -> bool {
        match self.client.head(url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
}

impl Default for EveryayahClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ApiClient for EveryayahClient {
    fn api_name(&self) -> &str {
        "everyayah.com"
    }

    fn priority(&self) -> u8 {
        1 // Primary for audio
    }

    async fn is_healthy(&self) -> bool {
        // Check if we can access an audio file
        let url = self.get_audio_url(1, 1);
        self.audio_exists(&url).await
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
impl QuranApiClient for EveryayahClient {
    async fn get_surah(&self, surah_number: u8) -> Result<SurahData, ApiError> {
        if surah_number < 1 || surah_number > 114 {
            return Err(ApiError::Validation(format!(
                "Invalid surah number: {}. Must be between 1 and 114",
                surah_number
            )));
        }

        // EveryAyah doesn't provide text, only audio URLs
        // We return an error indicating this limitation
        Err(ApiError::ApiError(
            self.api_name().to_string(),
            "EveryAyah only provides audio recitations, not text. Use get_audio_url() instead.".to_string(),
        ))
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

        // Check if the audio file exists
        let audio_url = self.get_audio_url(surah, ayah);
        if !self.audio_exists(&audio_url).await {
            return Err(ApiError::NotFound);
        }

        // Return minimal AyahData with audio URL in a note
        // In a real implementation, this would be combined with text from another API
        Ok(AyahData {
            surah,
            ayah,
            text_arabic: format!("Audio available at: {}", audio_url),
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

        // EveryAyah doesn't provide page-based access
        Err(ApiError::ApiError(
            self.api_name().to_string(),
            "EveryAyah only provides verse-by-verse audio, not page-based access.".to_string(),
        ))
    }
}

/// Available reciters on EveryAyah.com
/// This is a subset of popular reciters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reciter {
    AbdulBasitMurattal,
    AbdulBasitMujawwad,
    MisharyRashidAlafasy,
    MaherAlMuaiqly,
    SaadAlGhamdi,
    AhmedAjmy,
    HusaryMujawwad,
    MinshawMurattal,
}

impl Reciter {
    /// Get the directory name for this reciter
    pub fn directory_name(&self) -> &'static str {
        match self {
            Reciter::AbdulBasitMurattal => "Abdul_Basit_Murattal_192kbps",
            Reciter::AbdulBasitMujawwad => "Abdul_Basit_Mujawwad_128kbps",
            Reciter::MisharyRashidAlafasy => "Alafasy_128kbps",
            Reciter::MaherAlMuaiqly => "MaherAlMuaiqly128kbps",
            Reciter::SaadAlGhamdi => "Ghamadi_40kbps",
            Reciter::AhmedAjmy => "Ahmed_ibn_Ali_al-Ajamy_128kbps",
            Reciter::HusaryMujawwad => "Husary_128kbps",
            Reciter::MinshawMurattal => "Minshawy_Murattal_128kbps",
        }
    }

    /// Get the display name for this reciter
    pub fn display_name(&self) -> &'static str {
        match self {
            Reciter::AbdulBasitMurattal => "Abdul Basit (Murattal)",
            Reciter::AbdulBasitMujawwad => "Abdul Basit (Mujawwad)",
            Reciter::MisharyRashidAlafasy => "Mishary Rashid Alafasy",
            Reciter::MaherAlMuaiqly => "Maher Al Muaiqly",
            Reciter::SaadAlGhamdi => "Saad Al Ghamdi",
            Reciter::AhmedAjmy => "Ahmed Ajmy",
            Reciter::HusaryMujawwad => "Mahmoud Khalil Al-Husary (Mujawwad)",
            Reciter::MinshawMurattal => "Mohamed Siddiq El-Minshawi (Murattal)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = EveryayahClient::new();
        assert_eq!(client.api_name(), "everyayah.com");
        assert_eq!(client.priority(), 1);
        assert_eq!(client.reciter, "Abdul_Basit_Murattal_192kbps");
    }

    #[test]
    fn test_client_with_custom_reciter() {
        let client = EveryayahClient::with_reciter("Alafasy_128kbps".to_string());
        assert_eq!(client.reciter, "Alafasy_128kbps");
    }

    #[test]
    fn test_rate_limit_config() {
        let client = EveryayahClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.requests_per_hour, 1000);
        assert_eq!(config.requests_per_day, 10000);
    }

    #[test]
    fn test_get_audio_url() {
        let client = EveryayahClient::new();
        
        // Test Al-Fatihah, verse 1
        let url = client.get_audio_url(1, 1);
        assert_eq!(
            url,
            "https://everyayah.com/data/Abdul_Basit_Murattal_192kbps/001001.mp3"
        );
        
        // Test Al-Baqarah, verse 255 (Ayat al-Kursi)
        let url = client.get_audio_url(2, 255);
        assert_eq!(
            url,
            "https://everyayah.com/data/Abdul_Basit_Murattal_192kbps/002255.mp3"
        );
    }

    #[test]
    fn test_reciter_directory_names() {
        assert_eq!(
            Reciter::AbdulBasitMurattal.directory_name(),
            "Abdul_Basit_Murattal_192kbps"
        );
        assert_eq!(
            Reciter::MisharyRashidAlafasy.directory_name(),
            "Alafasy_128kbps"
        );
    }

    #[test]
    fn test_reciter_display_names() {
        assert_eq!(
            Reciter::AbdulBasitMurattal.display_name(),
            "Abdul Basit (Murattal)"
        );
        assert_eq!(
            Reciter::MisharyRashidAlafasy.display_name(),
            "Mishary Rashid Alafasy"
        );
    }

    #[tokio::test]
    async fn test_invalid_surah_number() {
        let client = EveryayahClient::new();
        
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
        let client = EveryayahClient::new();
        
        // Test ayah number 0
        let result = client.get_ayah(1, 0).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_invalid_page_number() {
        let client = EveryayahClient::new();
        
        // Test page number 0
        let result = client.get_page(0).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
        
        // Test page number 605
        let result = client.get_page(605).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_get_surah_returns_error() {
        let client = EveryayahClient::new();
        
        // EveryAyah doesn't provide text
        let result = client.get_surah(1).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::ApiError(_, _)));
    }
}

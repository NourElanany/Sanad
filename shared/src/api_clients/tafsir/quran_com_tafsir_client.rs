//! Quran.com Tafsir API Client
//!
//! Official Tafsir API from Quran Foundation providing interpretations from recognized scholars.
//! API Documentation: https://api-docs.quran.foundation/

use crate::api_clients::{
    ApiClient, ApiError, RateLimitConfig, TafsirApiClient, TafsirEntry, TafsirSource,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

/// Quran.com Tafsir API client
#[derive(Debug, Clone)]
pub struct QuranComTafsirClient {
    base_url: String,
    client: Client,
    api_key: Option<String>,
}

impl QuranComTafsirClient {
    /// Create a new Quran.com Tafsir API client
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            base_url: "https://api.quran.com/api/v4".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(15))
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
                .timeout(Duration::from_secs(15))
                .build()
                .expect("Failed to create HTTP client"),
            api_key,
        }
    }

    /// Validate surah and ayah numbers
    pub fn validate_verse(&self, surah: u8, ayah: u16) -> Result<(), ApiError> {
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

        Ok(())
    }
}

#[async_trait]
impl ApiClient for QuranComTafsirClient {
    fn api_name(&self) -> &str {
        "quran.com_tafsir"
    }

    fn priority(&self) -> u8 {
        1 // Primary tafsir API
    }

    async fn is_healthy(&self) -> bool {
        // Simple health check - try to list tafsir sources
        match self.list_tafsir_sources().await {
            Ok(_) => true,
            Err(e) => {
                tracing::warn!("Quran.com Tafsir health check failed: {}", e);
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
impl TafsirApiClient for QuranComTafsirClient {
    async fn get_tafsir(
        &self,
        surah: u8,
        ayah: u16,
        tafsir_id: Option<&str>,
    ) -> Result<Vec<TafsirEntry>, ApiError> {
        self.validate_verse(surah, ayah)?;

        let mut tafsir_entries = Vec::new();

        if let Some(id) = tafsir_id {
            // Fetch specific tafsir
            let entry = self.fetch_single_tafsir(surah, ayah, id).await?;
            tafsir_entries.push(entry);
        } else {
            // Fetch all available tafsirs for this verse
            let sources = self.list_tafsir_sources().await?;
            
            for source in sources {
                match self.fetch_single_tafsir(surah, ayah, &source.id).await {
                    Ok(entry) => tafsir_entries.push(entry),
                    Err(e) => {
                        tracing::warn!(
                            "Failed to fetch tafsir {} for {}:{}: {}",
                            source.id,
                            surah,
                            ayah,
                            e
                        );
                        // Continue with other tafsirs even if one fails
                    }
                }
            }
        }

        if tafsir_entries.is_empty() {
            return Err(ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("No tafsir found for verse {}:{}", surah, ayah),
            ));
        }

        Ok(tafsir_entries)
    }

    async fn list_tafsir_sources(&self) -> Result<Vec<TafsirSource>, ApiError> {
        let url = format!("{}/resources/tafsirs", self.base_url);
        let mut request = self.client.get(&url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await.map_err(|e| {
            ApiError::Network(format!("Failed to fetch tafsir sources from Quran.com: {}", e))
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

        let sources_response: QuranComTafsirSourcesResponse =
            response.json().await.map_err(|e| {
                ApiError::InvalidResponse(
                    self.api_name().to_string(),
                    format!("Failed to parse tafsir sources response: {}", e),
                )
            })?;

        let sources = sources_response
            .tafsirs
            .into_iter()
            .map(|t| TafsirSource {
                id: t.id.to_string(),
                name: t.name,
                scholar: t.author_name,
                language: t.language_name,
            })
            .collect();

        Ok(sources)
    }
}

impl QuranComTafsirClient {
    /// Fetch a single tafsir for a specific verse
    async fn fetch_single_tafsir(
        &self,
        surah: u8,
        ayah: u16,
        tafsir_id: &str,
    ) -> Result<TafsirEntry, ApiError> {
        let url = format!(
            "{}/tafsirs/{}/by_ayah/{}:{}",
            self.base_url, tafsir_id, surah, ayah
        );
        let mut request = self.client.get(&url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await.map_err(|e| {
            ApiError::Network(format!(
                "Failed to fetch tafsir {} for {}:{} from Quran.com: {}",
                tafsir_id, surah, ayah, e
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

        let tafsir_response: QuranComTafsirResponse = response.json().await.map_err(|e| {
            ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("Failed to parse tafsir response: {}", e),
            )
        })?;

        // Get tafsir source info for metadata
        let sources = self.list_tafsir_sources().await?;
        let source_info = sources
            .iter()
            .find(|s| s.id == tafsir_id)
            .ok_or_else(|| {
                ApiError::InvalidResponse(
                    self.api_name().to_string(),
                    format!("Tafsir source {} not found in sources list", tafsir_id),
                )
            })?;

        Ok(TafsirEntry {
            tafsir_id: tafsir_id.to_string(),
            tafsir_name: source_info.name.clone(),
            scholar: source_info.scholar.clone(),
            text: tafsir_response.tafsir.text,
            language: source_info.language.clone(),
            source: self.api_name().to_string(),
        })
    }
}

// ============================================================================
// Response structures for Quran.com Tafsir API
// ============================================================================

#[derive(Debug, Deserialize)]
struct QuranComTafsirSourcesResponse {
    tafsirs: Vec<QuranComTafsirSourceInfo>,
}

#[derive(Debug, Deserialize)]
struct QuranComTafsirSourceInfo {
    id: u32,
    name: String,
    author_name: String,
    language_name: String,
}

#[derive(Debug, Deserialize)]
struct QuranComTafsirResponse {
    tafsir: QuranComTafsirData,
}

#[derive(Debug, Deserialize)]
struct QuranComTafsirData {
    text: String,
    resource_id: u32,
    verse_key: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = QuranComTafsirClient::new(None);
        assert_eq!(client.api_name(), "quran.com_tafsir");
        assert_eq!(client.priority(), 1);
    }

    #[test]
    fn test_rate_limit_config() {
        let client = QuranComTafsirClient::new(None);
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.requests_per_hour, 1000);
        assert_eq!(config.requests_per_day, 10000);
    }

    #[test]
    fn test_validate_verse() {
        let client = QuranComTafsirClient::new(None);

        // Valid verse
        assert!(client.validate_verse(1, 1).is_ok());
        assert!(client.validate_verse(114, 1).is_ok());

        // Invalid surah number
        assert!(client.validate_verse(0, 1).is_err());
        assert!(client.validate_verse(115, 1).is_err());

        // Invalid ayah number
        assert!(client.validate_verse(1, 0).is_err());
    }

    #[tokio::test]
    async fn test_invalid_surah_number() {
        let client = QuranComTafsirClient::new(None);

        // Test surah number 0
        let result = client.get_tafsir(0, 1, None).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));

        // Test surah number 115
        let result = client.get_tafsir(115, 1, None).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_invalid_ayah_number() {
        let client = QuranComTafsirClient::new(None);

        // Test ayah number 0
        let result = client.get_tafsir(1, 0, None).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }
}

//! Example of using structured logging in API clients
//!
//! This file demonstrates how to integrate the logging utilities
//! into API client implementations for comprehensive observability.

use crate::api_clients::{
    ApiClient, ApiError, AyahData, QuranApiClient, RateLimitConfig,
    log_api_call, LogApiResult,
};
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

/// Example Quran API client with structured logging
#[derive(Debug, Clone)]
pub struct LoggingQuranClient {
    base_url: String,
    client: Client,
    api_key: Option<String>,
}

impl LoggingQuranClient {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            base_url: "https://api.example.com/v1".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            api_key,
        }
    }
}

#[async_trait]
impl ApiClient for LoggingQuranClient {
    fn api_name(&self) -> &str {
        "example.com"
    }

    fn priority(&self) -> u8 {
        1
    }

    async fn is_healthy(&self) -> bool {
        // Health check with logging
        let logger = log_api_call(self.api_name(), "health_check", None);
        
        match self.get_ayah(1, 1).await {
            Ok(_) => {
                logger.success(&true);
                true
            }
            Err(e) => {
                logger.failure(&e);
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
impl QuranApiClient for LoggingQuranClient {
    async fn get_ayah(&self, surah: u8, ayah: u16) -> Result<AyahData, ApiError> {
        // Get correlation ID from current context if available
        let correlation_id = crate::api_clients::logging::current_correlation_id();
        
        // Create logger for this API call
        let logger = log_api_call(
            self.api_name(),
            "get_ayah",
            correlation_id.as_deref(),
        );

        // Validate input
        if surah < 1 || surah > 114 {
            let error = ApiError::Validation(format!(
                "Invalid surah number: {}. Must be between 1 and 114",
                surah
            ));
            logger.failure(&error);
            return Err(error);
        }

        if ayah < 1 {
            let error = ApiError::Validation(format!(
                "Invalid ayah number: {}. Must be at least 1",
                ayah
            ));
            logger.failure(&error);
            return Err(error);
        }

        // Make API request
        let url = format!("{}/verses/{}:{}", self.base_url, surah, ayah);
        let mut request = self.client.get(&url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        // Add correlation ID to request headers if available
        if let Some(ref cid) = correlation_id {
            request = request.header("X-Correlation-ID", cid);
        }

        // Send request and log result
        let result = request
            .send()
            .await
            .map_err(|e| ApiError::Network(format!("Failed to fetch ayah: {}", e)))
            .and_then(|response| {
                if !response.status().is_success() {
                    Err(ApiError::ApiError(
                        self.api_name().to_string(),
                        format!("HTTP {}", response.status()),
                    ))
                } else {
                    Ok(response)
                }
            })
            .and_then(|response| async move {
                // Parse response
                Ok(AyahData {
                    surah,
                    ayah,
                    text_arabic: "Example text".to_string(),
                    text_translation: None,
                })
            }.await)
            .log_result(logger);

        result
    }

    async fn get_surah(&self, surah_number: u8) -> Result<crate::api_clients::SurahData, ApiError> {
        // Similar implementation with logging
        let correlation_id = crate::api_clients::logging::current_correlation_id();
        let logger = log_api_call(
            self.api_name(),
            "get_surah",
            correlation_id.as_deref(),
        );

        // Implementation would go here...
        let error = ApiError::NotImplemented("get_surah not implemented in example".to_string());
        logger.failure(&error);
        Err(error)
    }

    async fn get_page(&self, page: u16) -> Result<crate::api_clients::PageData, ApiError> {
        // Similar implementation with logging
        let correlation_id = crate::api_clients::logging::current_correlation_id();
        let logger = log_api_call(
            self.api_name(),
            "get_page",
            correlation_id.as_deref(),
        );

        // Implementation would go here...
        let error = ApiError::NotImplemented("get_page not implemented in example".to_string());
        logger.failure(&error);
        Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = LoggingQuranClient::new(None);
        assert_eq!(client.api_name(), "example.com");
        assert_eq!(client.priority(), 1);
    }
}

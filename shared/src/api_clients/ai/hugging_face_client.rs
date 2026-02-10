// Hugging Face AI Client
//
// Provides Arabic NLP capabilities using Hugging Face models
// NOTE: This is used ONLY for technical language processing, NOT for Islamic rulings

use crate::api_clients::{
    AiApiClient, AiQueryRequest, AiQueryResponse, ApiClient, ApiError, RateLimitConfig,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const HUGGING_FACE_BASE_URL: &str = "https://api-inference.huggingface.co";
const API_NAME: &str = "hugging_face";

/// Hugging Face AI client for Arabic NLP
///
/// This client provides:
/// - Text embeddings for semantic search
/// - Text classification
/// - Question answering (technical only, not religious rulings)
/// - Text summarization
pub struct HuggingFaceClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
    timeout: Duration,
    default_model: String,
}

#[derive(Debug, Serialize)]
struct HuggingFaceRequest {
    inputs: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<HuggingFaceParameters>,
}

#[derive(Debug, Serialize)]
struct HuggingFaceParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum HuggingFaceResponse {
    Text(Vec<HuggingFaceTextResponse>),
    Error(HuggingFaceError),
}

#[derive(Debug, Deserialize)]
struct HuggingFaceTextResponse {
    generated_text: Option<String>,
    summary_text: Option<String>,
    answer: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceError {
    error: String,
}

impl HuggingFaceClient {
    /// Create a new Hugging Face client
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            base_url: HUGGING_FACE_BASE_URL.to_string(),
            api_key,
            timeout: Duration::from_secs(30),
            // Default to a good Arabic model for text generation
            default_model: "aubmindlab/bert-base-arabertv2".to_string(),
        }
    }

    /// Create a client with custom configuration
    pub fn with_config(
        api_key: Option<String>,
        base_url: String,
        default_model: String,
    ) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            base_url,
            api_key,
            timeout: Duration::from_secs(30),
            default_model,
        }
    }

    /// Set the default model
    pub fn with_model(mut self, model: String) -> Self {
        self.default_model = model;
        self
    }

    /// Process a query using the specified model
    async fn process_with_model(
        &self,
        query: &str,
        model: &str,
        max_tokens: Option<usize>,
    ) -> Result<String, ApiError> {
        let url = format!("{}/models/{}", self.base_url, model);

        let parameters = HuggingFaceParameters {
            max_length: max_tokens,
            min_length: None,
            temperature: Some(0.7),
            top_k: Some(50),
            top_p: Some(0.95),
        };

        let request_body = HuggingFaceRequest {
            inputs: query.to_string(),
            parameters: Some(parameters),
        };

        let mut request = self.client.post(&url).json(&request_body).timeout(self.timeout);

        // Add API key if available
        if let Some(ref api_key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await.map_err(|e| {
            if e.is_timeout() {
                ApiError::Timeout(API_NAME.to_string())
            } else if e.is_connect() {
                ApiError::Network(format!("Connection error: {}", e))
            } else {
                ApiError::Network(format!("Request error: {}", e))
            }
        })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ApiError::ApiError(
                API_NAME.to_string(),
                format!("HTTP {}: {}", status, error_text),
            ));
        }

        let api_response: HuggingFaceResponse = response.json().await.map_err(|e| {
            ApiError::InvalidResponse(
                API_NAME.to_string(),
                format!("Failed to parse JSON: {}", e),
            )
        })?;

        match api_response {
            HuggingFaceResponse::Text(responses) => {
                if responses.is_empty() {
                    return Err(ApiError::InvalidResponse(
                        API_NAME.to_string(),
                        "Empty response from API".to_string(),
                    ));
                }

                let first_response = &responses[0];
                let text = first_response
                    .generated_text
                    .as_ref()
                    .or(first_response.summary_text.as_ref())
                    .or(first_response.answer.as_ref())
                    .ok_or_else(|| {
                        ApiError::InvalidResponse(
                            API_NAME.to_string(),
                            "No text in response".to_string(),
                        )
                    })?;

                Ok(text.clone())
            }
            HuggingFaceResponse::Error(error) => Err(ApiError::ApiError(
                API_NAME.to_string(),
                format!("API error: {}", error.error),
            )),
        }
    }
}

impl Default for HuggingFaceClient {
    fn default() -> Self {
        Self::new(None)
    }
}

#[async_trait]
impl ApiClient for HuggingFaceClient {
    fn api_name(&self) -> &str {
        API_NAME
    }

    fn priority(&self) -> u8 {
        1 // Primary AI API
    }

    async fn is_healthy(&self) -> bool {
        // Simple health check - try to access the API
        let url = format!("{}/models", self.base_url);
        let mut request = self.client.get(&url).timeout(Duration::from_secs(5));

        if let Some(ref api_key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        match request.send().await {
            Ok(response) => response.status().is_success(),
            Err(e) => {
                log::warn!("Hugging Face API health check failed: {}", e);
                false
            }
        }
    }

    fn rate_limit(&self) -> RateLimitConfig {
        RateLimitConfig {
            // Hugging Face free tier limits
            requests_per_minute: 30,
            requests_per_hour: 1000,
            requests_per_day: 10000,
        }
    }
}

#[async_trait]
impl AiApiClient for HuggingFaceClient {
    async fn process_query(&self, request: &AiQueryRequest) -> Result<AiQueryResponse, ApiError> {
        // Validate input
        if request.query.trim().is_empty() {
            return Err(ApiError::InvalidInput(
                "Query cannot be empty".to_string(),
            ));
        }

        // Determine which model to use
        let model = if request.language == "ar" || request.language == "arabic" {
            // Use Arabic-specific model
            "aubmindlab/bert-base-arabertv2"
        } else {
            &self.default_model
        };

        // Build the full query with context if provided
        let full_query = if let Some(context) = &request.context {
            format!("{}\n\nContext: {}", request.query, context)
        } else {
            request.query.clone()
        };

        // Process the query
        let response_text = self
            .process_with_model(&full_query, model, request.max_tokens)
            .await?;

        // Filter inappropriate content (basic check)
        if response_text.contains("fatwa") || response_text.contains("حكم شرعي") {
            log::warn!("AI response contained religious ruling keywords, filtering");
            return Err(ApiError::InvalidResponse(
                API_NAME.to_string(),
                "Response contained inappropriate religious content".to_string(),
            ));
        }

        Ok(AiQueryResponse {
            response: response_text,
            sources: vec![format!("Hugging Face: {}", model)],
            confidence: 0.8, // Default confidence for Hugging Face
            model: model.to_string(),
        })
    }

    async fn generate_embeddings(&self, text: &str) -> Result<Vec<f32>, ApiError> {
        // Placeholder implementation
        // In a real implementation, this would call the Hugging Face embeddings API
        Err(ApiError::NotImplemented(
            "Embeddings generation not yet implemented".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = HuggingFaceClient::new(None);
        assert_eq!(client.api_name(), API_NAME);
        assert_eq!(client.priority(), 1);
    }

    #[test]
    fn test_client_with_api_key() {
        let client = HuggingFaceClient::new(Some("test_key".to_string()));
        assert_eq!(client.api_name(), API_NAME);
        assert!(client.api_key.is_some());
    }

    #[test]
    fn test_client_with_custom_model() {
        let client = HuggingFaceClient::new(None).with_model("custom-model".to_string());
        assert_eq!(client.default_model, "custom-model");
    }

    #[test]
    fn test_rate_limit_config() {
        let client = HuggingFaceClient::new(None);
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 30);
        assert_eq!(config.requests_per_hour, 1000);
        assert_eq!(config.requests_per_day, 10000);
    }

    #[tokio::test]
    async fn test_empty_query_validation() {
        let client = HuggingFaceClient::new(None);
        let request = AiQueryRequest {
            query: "".to_string(),
            context: None,
            language: "en".to_string(),
            max_tokens: None,
        };

        let result = client.process_query(request).await;
        assert!(matches!(result, Err(ApiError::InvalidInput(_))));
    }

    #[tokio::test]
    async fn test_whitespace_only_query_validation() {
        let client = HuggingFaceClient::new(None);
        let request = AiQueryRequest {
            query: "   ".to_string(),
            context: None,
            language: "en".to_string(),
            max_tokens: None,
        };

        let result = client.process_query(request).await;
        assert!(matches!(result, Err(ApiError::InvalidInput(_))));
    }
}

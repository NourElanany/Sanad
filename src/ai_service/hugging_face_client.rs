use super::*;
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{info, warn, error, debug};

/// Hugging Face API client for Islamic AI models
#[derive(Clone)]
pub struct HuggingFaceClient {
    client: Client,
    api_key: String,
    base_url: String,
    timeout: Duration,
    max_retries: u32,
    rate_limiter: RateLimiter,
}

/// Rate limiter for API requests
#[derive(Clone)]
struct RateLimiter {
    requests_per_minute: u32,
    last_reset: std::sync::Arc<std::sync::Mutex<Instant>>,
    current_count: std::sync::Arc<std::sync::Mutex<u32>>,
}

/// Request structure for Hugging Face text generation
#[derive(Debug, Serialize)]
pub struct TextGenerationRequest {
    pub inputs: String,
    pub parameters: GenerationParameters,
    pub options: RequestOptions,
}

/// Generation parameters for controlling model output
#[derive(Debug, Serialize)]
pub struct GenerationParameters {
    pub max_new_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub repetition_penalty: Option<f32>,
    pub do_sample: Option<bool>,
    pub return_full_text: Option<bool>,
    pub stop_sequences: Option<Vec<String>>,
}

/// Request options
#[derive(Debug, Serialize)]
pub struct RequestOptions {
    pub wait_for_model: bool,
    pub use_cache: bool,
}

/// Response from Hugging Face text generation
#[derive(Debug, Deserialize)]
pub struct TextGenerationResponse {
    pub generated_text: String,
    pub details: Option<GenerationDetails>,
}

/// Generation details from the model
#[derive(Debug, Deserialize)]
pub struct GenerationDetails {
    pub finish_reason: Option<String>,
    pub generated_tokens: Option<u32>,
    pub seed: Option<u64>,
    pub prefill: Option<Vec<PrefillToken>>,
    pub tokens: Option<Vec<Token>>,
}

#[derive(Debug, Deserialize)]
pub struct PrefillToken {
    pub id: u32,
    pub text: String,
    pub logprob: f32,
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub id: u32,
    pub text: String,
    pub logprob: f32,
    pub special: bool,
}

/// Embedding request for Hugging Face
#[derive(Debug, Serialize)]
pub struct EmbeddingRequest {
    pub inputs: Vec<String>,
    pub options: RequestOptions,
}

/// Embedding response from Hugging Face
#[derive(Debug, Deserialize)]
pub struct EmbeddingResponse(pub Vec<Vec<f32>>);

/// Configuration for Hugging Face client
#[derive(Debug, Clone)]
pub struct HuggingFaceConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub requests_per_minute: u32,
    pub default_model: String,
    pub embedding_model: String,
}

impl Default for HuggingFaceConfig {
    fn default() -> Self {
        Self {
            api_key: std::env::var("HUGGING_FACE_API_KEY").unwrap_or_default(),
            base_url: "https://api-inference.huggingface.co".to_string(),
            timeout_seconds: 30,
            max_retries: 3,
            requests_per_minute: 60,
            default_model: "microsoft/DialoGPT-medium".to_string(),
            embedding_model: "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2".to_string(),
        }
    }
}

impl HuggingFaceClient {
    /// Create a new Hugging Face client
    pub fn new(config: HuggingFaceConfig) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(AIServiceError::ConfigurationError(
                "Hugging Face API key is required".to_string()
            ));
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.api_key))
                .map_err(|e| AIServiceError::ConfigurationError(format!("Invalid API key format: {}", e)))?
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .default_headers(headers)
            .build()
            .map_err(|e| AIServiceError::ExternalAPIError(format!("Failed to create HTTP client: {}", e)))?;

        let rate_limiter = RateLimiter::new(config.requests_per_minute);

        Ok(Self {
            client,
            api_key: config.api_key,
            base_url: config.base_url,
            timeout: Duration::from_secs(config.timeout_seconds),
            max_retries: config.max_retries,
            rate_limiter,
        })
    }

    /// Generate text using Islamic AI model
    pub async fn generate_text(&self, request: TextGenerationRequest, model: &str) -> Result<TextGenerationResponse> {
        self.rate_limiter.wait_if_needed().await;

        let url = format!("{}/models/{}", self.base_url, model);
        let mut last_error = None;

        for attempt in 1..=self.max_retries {
            match self.make_generation_request(&url, &request).await {
                Ok(response) => {
                    debug!("Text generation successful on attempt {}", attempt);
                    return Ok(response);
                }
                Err(e) => {
                    warn!("Text generation attempt {} failed: {}", attempt, e);
                    last_error = Some(e);
                    
                    if attempt < self.max_retries {
                        let delay = Duration::from_millis(1000 * attempt as u64);
                        sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| 
            AIServiceError::ExternalAPIError("All retry attempts failed".to_string())
        ))
    }

    /// Generate embeddings for texts
    pub async fn generate_embeddings(&self, texts: Vec<String>, model: &str) -> Result<Vec<Vec<f32>>> {
        self.rate_limiter.wait_if_needed().await;

        let request = EmbeddingRequest {
            inputs: texts,
            options: RequestOptions {
                wait_for_model: true,
                use_cache: true,
            },
        };

        let url = format!("{}/models/{}", self.base_url, model);
        let mut last_error = None;

        for attempt in 1..=self.max_retries {
            match self.make_embedding_request(&url, &request).await {
                Ok(response) => {
                    debug!("Embedding generation successful on attempt {}", attempt);
                    return Ok(response.0);
                }
                Err(e) => {
                    warn!("Embedding generation attempt {} failed: {}", attempt, e);
                    last_error = Some(e);
                    
                    if attempt < self.max_retries {
                        let delay = Duration::from_millis(1000 * attempt as u64);
                        sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| 
            AIServiceError::ExternalAPIError("All retry attempts failed".to_string())
        ))
    }

    /// Generate Islamic response with context
    pub async fn generate_islamic_response(
        &self,
        context: &str,
        question: &str,
        sources: &[IslamicSource],
        model: &str,
    ) -> Result<String> {
        let prompt = self.build_islamic_prompt(context, question, sources);
        
        let request = TextGenerationRequest {
            inputs: prompt,
            parameters: GenerationParameters {
                max_new_tokens: Some(1000),
                temperature: Some(0.3), // Lower temperature for more focused responses
                top_p: Some(0.9),
                top_k: Some(50),
                repetition_penalty: Some(1.1),
                do_sample: Some(true),
                return_full_text: Some(false),
                stop_sequences: Some(vec![
                    "\n\n---".to_string(),
                    "المصادر:".to_string(),
                    "Sources:".to_string(),
                ]),
            },
            options: RequestOptions {
                wait_for_model: true,
                use_cache: false, // Don't cache for Islamic content to ensure freshness
            },
        };

        let response = self.generate_text(request, model).await?;
        Ok(response.generated_text)
    }

    /// Check if model is available and ready
    pub async fn check_model_status(&self, model: &str) -> Result<bool> {
        let url = format!("{}/models/{}", self.base_url, model);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(true)
                } else if response.status().as_u16() == 503 {
                    // Model is loading
                    Ok(false)
                } else {
                    Err(AIServiceError::ExternalAPIError(
                        format!("Model check failed with status: {}", response.status())
                    ))
                }
            }
            Err(e) => Err(AIServiceError::ExternalAPIError(
                format!("Failed to check model status: {}", e)
            ))
        }
    }

    /// Wait for model to be ready
    pub async fn wait_for_model(&self, model: &str, max_wait_seconds: u64) -> Result<()> {
        let start_time = Instant::now();
        let max_wait = Duration::from_secs(max_wait_seconds);

        while start_time.elapsed() < max_wait {
            match self.check_model_status(model).await {
                Ok(true) => {
                    info!("Model {} is ready", model);
                    return Ok(());
                }
                Ok(false) => {
                    debug!("Model {} is still loading, waiting...", model);
                    sleep(Duration::from_secs(5)).await;
                }
                Err(e) => {
                    warn!("Error checking model status: {}", e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }

        Err(AIServiceError::ExternalAPIError(
            format!("Model {} did not become ready within {} seconds", model, max_wait_seconds)
        ))
    }

    /// Build Islamic-specific prompt with context and sources
    fn build_islamic_prompt(&self, context: &str, question: &str, sources: &[IslamicSource]) -> String {
        let mut prompt = String::new();
        
        // System instruction
        prompt.push_str("أنت مساعد ذكي متخصص في الشؤون الإسلامية. أجب على الأسئلة بناءً على المصادر الإسلامية الموثوقة المرفقة فقط.\n\n");
        
        // Add context if provided
        if !context.is_empty() {
            prompt.push_str("السياق:\n");
            prompt.push_str(context);
            prompt.push_str("\n\n");
        }
        
        // Add sources
        if !sources.is_empty() {
            prompt.push_str("المصادر الموثوقة:\n");
            for (i, source) in sources.iter().enumerate() {
                prompt.push_str(&format!("{}. {} - {}\n", i + 1, source.reference, source.text));
                if let Some(author) = &source.author {
                    prompt.push_str(&format!("   المؤلف: {}\n", author));
                }
                prompt.push_str(&format!("   درجة الموثوقية: {:?}\n", source.authenticity));
                prompt.push('\n');
            }
        }
        
        // Add instructions
        prompt.push_str("تعليمات مهمة:\n");
        prompt.push_str("- أجب بناءً على المصادر المرفقة فقط\n");
        prompt.push_str("- لا تختلق آيات أو أحاديث\n");
        prompt.push_str("- إذا لم تجد إجابة في المصادر، قل ذلك صراحة\n");
        prompt.push_str("- اذكر المصادر في نهاية الإجابة\n");
        prompt.push_str("- استخدم لغة واضحة ومفهومة\n\n");
        
        // Add the question
        prompt.push_str("السؤال: ");
        prompt.push_str(question);
        prompt.push_str("\n\nالإجابة: ");
        
        prompt
    }

    /// Make HTTP request for text generation
    async fn make_generation_request(&self, url: &str, request: &TextGenerationRequest) -> Result<TextGenerationResponse> {
        let response = self.client
            .post(url)
            .json(request)
            .send()
            .await
            .map_err(|e| AIServiceError::ExternalAPIError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIServiceError::ExternalAPIError(
                format!("API request failed with status {}: {}", status, error_text)
            ));
        }

        let response_text = response.text().await
            .map_err(|e| AIServiceError::ExternalAPIError(format!("Failed to read response: {}", e)))?;

        // Handle both single response and array response formats
        if response_text.trim().starts_with('[') {
            let responses: Vec<TextGenerationResponse> = serde_json::from_str(&response_text)
                .map_err(|e| AIServiceError::ExternalAPIError(format!("Failed to parse response: {}", e)))?;
            
            responses.into_iter().next()
                .ok_or_else(|| AIServiceError::ExternalAPIError("Empty response array".to_string()))
        } else {
            serde_json::from_str(&response_text)
                .map_err(|e| AIServiceError::ExternalAPIError(format!("Failed to parse response: {}", e)))
        }
    }

    /// Make HTTP request for embeddings
    async fn make_embedding_request(&self, url: &str, request: &EmbeddingRequest) -> Result<EmbeddingResponse> {
        let response = self.client
            .post(url)
            .json(request)
            .send()
            .await
            .map_err(|e| AIServiceError::ExternalAPIError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIServiceError::ExternalAPIError(
                format!("API request failed with status {}: {}", status, error_text)
            ));
        }

        response.json::<EmbeddingResponse>().await
            .map_err(|e| AIServiceError::ExternalAPIError(format!("Failed to parse response: {}", e)))
    }
}

impl RateLimiter {
    fn new(requests_per_minute: u32) -> Self {
        Self {
            requests_per_minute,
            last_reset: std::sync::Arc::new(std::sync::Mutex::new(Instant::now())),
            current_count: std::sync::Arc::new(std::sync::Mutex::new(0)),
        }
    }

    async fn wait_if_needed(&self) {
        let now = Instant::now();
        
        let (should_wait, wait_duration) = {
            let mut last_reset = self.last_reset.lock().unwrap();
            let mut current_count = self.current_count.lock().unwrap();
            
            // Reset counter if a minute has passed
            if now.duration_since(*last_reset) >= Duration::from_secs(60) {
                *last_reset = now;
                *current_count = 0;
            }
            
            if *current_count >= self.requests_per_minute {
                let time_until_reset = Duration::from_secs(60) - now.duration_since(*last_reset);
                (true, time_until_reset)
            } else {
                *current_count += 1;
                (false, Duration::from_secs(0))
            }
        };
        
        if should_wait {
            warn!("Rate limit reached, waiting {:?}", wait_duration);
            sleep(wait_duration).await;
            
            // Reset after waiting
            let mut last_reset = self.last_reset.lock().unwrap();
            let mut current_count = self.current_count.lock().unwrap();
            *last_reset = Instant::now();
            *current_count = 1;
        }
    }
}

impl Default for GenerationParameters {
    fn default() -> Self {
        Self {
            max_new_tokens: Some(500),
            temperature: Some(0.7),
            top_p: Some(0.9),
            top_k: Some(50),
            repetition_penalty: Some(1.1),
            do_sample: Some(true),
            return_full_text: Some(false),
            stop_sequences: None,
        }
    }
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            wait_for_model: true,
            use_cache: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hugging_face_client_creation() {
        let config = HuggingFaceConfig {
            api_key: "test_key".to_string(),
            ..Default::default()
        };
        
        let client = HuggingFaceClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_islamic_prompt_building() {
        let config = HuggingFaceConfig {
            api_key: "test_key".to_string(),
            ..Default::default()
        };
        let client = HuggingFaceClient::new(config).unwrap();
        
        let sources = vec![
            IslamicSource {
                id: "test1".to_string(),
                content_type: SourceType::Quran,
                text: "بسم الله الرحمن الرحيم".to_string(),
                reference: "الفاتحة: 1".to_string(),
                author: None,
                authenticity: AuthenticityLevel::Verified,
                language: Language::Arabic,
                metadata: std::collections::HashMap::new(),
                created_at: chrono::Utc::now(),
            }
        ];
        
        let prompt = client.build_islamic_prompt(
            "سياق السؤال",
            "ما معنى البسملة؟",
            &sources
        );
        
        assert!(prompt.contains("أنت مساعد ذكي متخصص في الشؤون الإسلامية"));
        assert!(prompt.contains("بسم الله الرحمن الرحيم"));
        assert!(prompt.contains("ما معنى البسملة؟"));
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let rate_limiter = RateLimiter::new(2); // 2 requests per minute
        
        // First two requests should go through immediately
        rate_limiter.wait_if_needed().await;
        rate_limiter.wait_if_needed().await;
        
        // Third request should be rate limited (but we won't wait in test)
        let start = Instant::now();
        rate_limiter.wait_if_needed().await;
        let elapsed = start.elapsed();
        
        // Should have waited some time (close to a minute)
        assert!(elapsed > Duration::from_secs(50));
    }
}
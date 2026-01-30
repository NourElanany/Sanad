use super::*;
use crate::ai_service::{
    hugging_face_client::{HuggingFaceClient, HuggingFaceConfig, TextGenerationRequest, GenerationParameters, RequestOptions},
    vector_database::{VectorDatabaseClient, VectorDatabaseConfig, VectorSearchRequest, VectorFilter, VectorDocument, VectorPayload},
};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{info, warn, error, debug};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Integration service that combines Hugging Face and Vector Database
/// with caching, rate limiting, and fallback strategies
#[derive(Clone)]
pub struct IntegrationService {
    hf_client: HuggingFaceClient,
    vector_db: VectorDatabaseClient,
    cache: CacheManager,
    config: IntegrationConfig,
    fallback_handler: FallbackHandler,
}

/// Configuration for the integration service
#[derive(Debug, Clone)]
pub struct IntegrationConfig {
    pub hugging_face: HuggingFaceConfig,
    pub vector_database: VectorDatabaseConfig,
    pub cache: CacheConfig,
    pub fallback: FallbackConfig,
    pub rate_limiting: RateLimitConfig,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub enable_query_cache: bool,
    pub enable_response_cache: bool,
    pub enable_embedding_cache: bool,
    pub query_cache_ttl: Duration,
    pub response_cache_ttl: Duration,
    pub embedding_cache_ttl: Duration,
    pub max_cache_size: usize,
    pub redis_url: Option<String>,
}

/// Fallback configuration
#[derive(Debug, Clone)]
pub struct FallbackConfig {
    pub enable_fallback: bool,
    pub fallback_models: Vec<String>,
    pub max_fallback_attempts: u32,
    pub fallback_delay: Duration,
    pub enable_offline_mode: bool,
    pub offline_responses: HashMap<String, String>,
}

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub burst_limit: u32,
    pub enable_adaptive_rate_limiting: bool,
}

/// Request for RAG processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGProcessingRequest {
    pub question: String,
    pub context: Option<String>,
    pub max_sources: Option<usize>,
    pub similarity_threshold: Option<f32>,
    pub preferred_source_types: Option<Vec<String>>,
    pub language: Option<String>,
    pub user_id: Option<String>,
}

/// Response from RAG processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGProcessingResponse {
    pub answer: String,
    pub confidence: f32,
    pub sources: Vec<RetrievedSource>,
    pub processing_time_ms: u64,
    pub cache_hit: bool,
    pub model_used: String,
    pub fallback_used: bool,
    pub warnings: Vec<String>,
}

/// Retrieved source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedSource {
    pub id: String,
    pub text: String,
    pub reference: String,
    pub source_type: String,
    pub similarity_score: f32,
    pub authenticity: String,
    pub author: Option<String>,
}

/// Cache manager for storing queries, responses, and embeddings
pub struct CacheManager {
    config: CacheConfig,
    query_cache: HashMap<String, CachedQuery>,
    response_cache: HashMap<String, CachedResponse>,
    embedding_cache: HashMap<String, CachedEmbedding>,
    // In production, this would use Redis or another distributed cache
}

/// Cached query result
#[derive(Debug, Clone)]
struct CachedQuery {
    sources: Vec<RetrievedSource>,
    timestamp: DateTime<Utc>,
    hit_count: u32,
}

/// Cached response
#[derive(Debug, Clone)]
struct CachedResponse {
    response: String,
    confidence: f32,
    timestamp: DateTime<Utc>,
    hit_count: u32,
}

/// Cached embedding
#[derive(Debug, Clone)]
struct CachedEmbedding {
    embedding: Vec<f32>,
    timestamp: DateTime<Utc>,
    hit_count: u32,
}

/// Fallback handler for managing service failures
pub struct FallbackHandler {
    config: FallbackConfig,
    failure_counts: HashMap<String, u32>,
    last_failure_times: HashMap<String, DateTime<Utc>>,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            hugging_face: HuggingFaceConfig::default(),
            vector_database: VectorDatabaseConfig::default(),
            cache: CacheConfig::default(),
            fallback: FallbackConfig::default(),
            rate_limiting: RateLimitConfig::default(),
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enable_query_cache: true,
            enable_response_cache: true,
            enable_embedding_cache: true,
            query_cache_ttl: Duration::from_secs(3600), // 1 hour
            response_cache_ttl: Duration::from_secs(1800), // 30 minutes
            embedding_cache_ttl: Duration::from_secs(7200), // 2 hours
            max_cache_size: 10000,
            redis_url: None,
        }
    }
}

impl Default for FallbackConfig {
    fn default() -> Self {
        let mut offline_responses = HashMap::new();
        offline_responses.insert(
            "default".to_string(),
            "عذراً، الخدمة غير متاحة حالياً. يرجى المحاولة لاحقاً أو استشارة العلماء المختصين.".to_string()
        );
        
        Self {
            enable_fallback: true,
            fallback_models: vec![
                "aubmindlab/bert-base-arabertv02".to_string(),
                "CAMeL-Lab/bert-base-arabic-camelbert-mix".to_string(),
            ],
            max_fallback_attempts: 3,
            fallback_delay: Duration::from_secs(2),
            enable_offline_mode: true,
            offline_responses,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            requests_per_hour: 1000,
            burst_limit: 10,
            enable_adaptive_rate_limiting: true,
        }
    }
}

impl IntegrationService {
    /// Create a new integration service
    pub async fn new(config: IntegrationConfig) -> Result<Self> {
        info!("Initializing Integration Service...");
        
        // Initialize Hugging Face client
        let hf_client = HuggingFaceClient::new(config.hugging_face.clone())?;
        
        // Initialize Vector Database client
        let vector_db = VectorDatabaseClient::new(config.vector_database.clone()).await?;
        
        // Initialize cache manager
        let cache = CacheManager::new(config.cache.clone());
        
        // Initialize fallback handler
        let fallback_handler = FallbackHandler::new(config.fallback.clone());
        
        info!("Integration Service initialized successfully");
        
        Ok(Self {
            hf_client,
            vector_db,
            cache,
            config,
            fallback_handler,
        })
    }
    
    /// Process a RAG request with full integration
    pub async fn process_rag_request(&mut self, request: RAGProcessingRequest) -> Result<RAGProcessingResponse> {
        let start_time = Instant::now();
        let mut warnings = Vec::new();
        let mut cache_hit = false;
        let mut fallback_used = false;
        
        debug!("Processing RAG request: {}", request.question);
        
        // 1. Check cache for existing response
        if self.cache.config.enable_response_cache {
            let cache_key = self.generate_cache_key(&request);
            if let Some(cached_response) = self.cache.get_cached_response(&cache_key) {
                info!("Cache hit for question: {}", request.question);
                cache_hit = true;
                
                return Ok(RAGProcessingResponse {
                    answer: cached_response.response,
                    confidence: cached_response.confidence,
                    sources: Vec::new(), // Sources not cached for simplicity
                    processing_time_ms: start_time.elapsed().as_millis() as u64,
                    cache_hit,
                    model_used: "cached".to_string(),
                    fallback_used: false,
                    warnings: vec!["Response served from cache".to_string()],
                });
            }
        }
        
        // 2. Generate embedding for the question
        let question_embedding = match self.generate_question_embedding(&request.question).await {
            Ok(embedding) => embedding,
            Err(e) => {
                warn!("Failed to generate embedding: {}", e);
                if self.config.fallback.enable_offline_mode {
                    return self.handle_offline_fallback(&request).await;
                }
                return Err(e);
            }
        };
        
        // 3. Search for relevant sources in vector database
        let sources = match self.search_relevant_sources(&question_embedding, &request).await {
            Ok(sources) => sources,
            Err(e) => {
                warn!("Vector search failed: {}", e);
                warnings.push("Vector search failed, using fallback".to_string());
                Vec::new() // Continue with empty sources
            }
        };
        
        // 4. Generate response using Hugging Face
        let (answer, confidence, model_used) = match self.generate_response_with_fallback(&request, &sources).await {
            Ok((answer, confidence, model)) => (answer, confidence, model),
            Err(e) => {
                error!("All response generation attempts failed: {}", e);
                if self.config.fallback.enable_offline_mode {
                    let offline_response = self.handle_offline_fallback(&request).await?;
                    return Ok(offline_response);
                }
                return Err(e);
            }
        };
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let response = RAGProcessingResponse {
            answer: answer.clone(),
            confidence,
            sources,
            processing_time_ms: processing_time,
            cache_hit,
            model_used: model_used.clone(),
            fallback_used,
            warnings,
        };
        
        // 5. Cache the response
        if self.cache.config.enable_response_cache {
            let cache_key = self.generate_cache_key(&request);
            self.cache.cache_response(&cache_key, &answer, confidence);
        }
        
        info!("RAG request processed successfully in {}ms", processing_time);
        Ok(response)
    }
    
    /// Generate embedding for a question with caching
    async fn generate_question_embedding(&mut self, question: &str) -> Result<Vec<f32>> {
        // Check cache first
        if self.cache.config.enable_embedding_cache {
            if let Some(cached_embedding) = self.cache.get_cached_embedding(question) {
                debug!("Using cached embedding for question");
                return Ok(cached_embedding.embedding);
            }
        }
        
        // Generate new embedding
        let embedding = self.hf_client
            .generate_embeddings(vec![question.to_string()], &self.config.hugging_face.embedding_model)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| AIServiceError::ExternalAPIError("No embedding returned".to_string()))?;
        
        // Cache the embedding
        if self.cache.config.enable_embedding_cache {
            self.cache.cache_embedding(question, &embedding);
        }
        
        Ok(embedding)
    }
    
    /// Search for relevant sources in vector database
    async fn search_relevant_sources(
        &self,
        query_embedding: &[f32],
        request: &RAGProcessingRequest,
    ) -> Result<Vec<RetrievedSource>> {
        let search_request = VectorSearchRequest {
            query_vector: query_embedding.to_vec(),
            limit: request.max_sources.unwrap_or(10),
            score_threshold: request.similarity_threshold,
            filter: self.build_vector_filter(request),
            with_payload: true,
            with_vectors: false,
        };
        
        let search_results = self.vector_db.search(search_request).await?;
        
        let mut sources = Vec::new();
        for result in search_results {
            let source = RetrievedSource {
                id: result.id,
                text: result.payload.get("text")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                reference: result.payload.get("reference")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                source_type: result.payload.get("content_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                similarity_score: result.score,
                authenticity: result.payload.get("authenticity")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                author: result.payload.get("author")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
            };
            sources.push(source);
        }
        
        Ok(sources)
    }
    
    /// Generate response with fallback strategies
    async fn generate_response_with_fallback(
        &self,
        request: &RAGProcessingRequest,
        sources: &[RetrievedSource],
    ) -> Result<(String, f32, String)> {
        let primary_model = &self.config.hugging_face.default_model;
        
        // Try primary model first
        match self.generate_response_with_model(request, sources, primary_model).await {
            Ok((answer, confidence)) => {
                return Ok((answer, confidence, primary_model.clone()));
            }
            Err(e) => {
                warn!("Primary model failed: {}", e);
            }
        }
        
        // Try fallback models
        if self.config.fallback.enable_fallback {
            for (i, fallback_model) in self.config.fallback.fallback_models.iter().enumerate() {
                if i > 0 {
                    sleep(self.config.fallback.fallback_delay).await;
                }
                
                match self.generate_response_with_model(request, sources, fallback_model).await {
                    Ok((answer, confidence)) => {
                        warn!("Using fallback model: {}", fallback_model);
                        return Ok((answer, confidence * 0.9, fallback_model.clone())); // Reduce confidence for fallback
                    }
                    Err(e) => {
                        warn!("Fallback model {} failed: {}", fallback_model, e);
                    }
                }
            }
        }
        
        Err(AIServiceError::ServiceUnavailable(
            "All models failed to generate response".to_string()
        ))
    }
    
    /// Generate response using a specific model
    async fn generate_response_with_model(
        &self,
        request: &RAGProcessingRequest,
        sources: &[RetrievedSource],
        model: &str,
    ) -> Result<(String, f32)> {
        let context = self.build_context_for_generation(request, sources);
        
        let generation_request = TextGenerationRequest {
            inputs: context,
            parameters: GenerationParameters {
                max_new_tokens: Some(1000),
                temperature: Some(0.3),
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
                use_cache: false,
            },
        };
        
        let response = self.hf_client.generate_text(generation_request, model).await?;
        
        // Calculate confidence based on response quality
        let confidence = self.calculate_response_confidence(&response.generated_text, sources);
        
        Ok((response.generated_text, confidence))
    }
    
    /// Build context for text generation
    fn build_context_for_generation(&self, request: &RAGProcessingRequest, sources: &[RetrievedSource]) -> String {
        let mut context = String::new();
        
        // System instruction
        context.push_str("أنت مساعد ذكي متخصص في الشؤون الإسلامية. أجب على الأسئلة بناءً على المصادر الإسلامية الموثوقة المرفقة فقط.\n\n");
        
        // Add user context if provided
        if let Some(user_context) = &request.context {
            context.push_str("السياق:\n");
            context.push_str(user_context);
            context.push_str("\n\n");
        }
        
        // Add sources
        if !sources.is_empty() {
            context.push_str("المصادر الموثوقة:\n");
            for (i, source) in sources.iter().enumerate() {
                context.push_str(&format!("{}. {} - {}\n", i + 1, source.reference, source.text));
                if let Some(author) = &source.author {
                    context.push_str(&format!("   المؤلف: {}\n", author));
                }
                context.push_str(&format!("   درجة الموثوقية: {}\n", source.authenticity));
                context.push_str(&format!("   نوع المصدر: {}\n", source.source_type));
                context.push('\n');
            }
        }
        
        // Add instructions
        context.push_str("تعليمات مهمة:\n");
        context.push_str("- أجب بناءً على المصادر المرفقة فقط\n");
        context.push_str("- لا تختلق آيات أو أحاديث\n");
        context.push_str("- إذا لم تجد إجابة في المصادر، قل ذلك صراحة\n");
        context.push_str("- اذكر المصادر في نهاية الإجابة\n");
        context.push_str("- استخدم لغة واضحة ومفهومة\n\n");
        
        // Add the question
        context.push_str("السؤال: ");
        context.push_str(&request.question);
        context.push_str("\n\nالإجابة: ");
        
        context
    }
    
    /// Calculate confidence score for a response
    fn calculate_response_confidence(&self, response: &str, sources: &[RetrievedSource]) -> f32 {
        let mut confidence = 0.5; // Base confidence
        
        // Increase confidence based on response length (reasonable responses)
        if response.len() > 50 && response.len() < 2000 {
            confidence += 0.1;
        }
        
        // Increase confidence if sources are cited
        if response.contains("المصدر") || response.contains("المرجع") {
            confidence += 0.2;
        }
        
        // Increase confidence based on source quality
        let avg_source_score = if !sources.is_empty() {
            sources.iter().map(|s| s.similarity_score).sum::<f32>() / sources.len() as f32
        } else {
            0.0
        };
        confidence += avg_source_score * 0.3;
        
        // Decrease confidence for very short or very long responses
        if response.len() < 20 || response.len() > 3000 {
            confidence -= 0.2;
        }
        
        // Ensure confidence is within bounds
        confidence.max(0.0).min(1.0)
    }
    
    /// Handle offline fallback when services are unavailable
    async fn handle_offline_fallback(&self, request: &RAGProcessingRequest) -> Result<RAGProcessingResponse> {
        let fallback_response = self.config.fallback.offline_responses
            .get("default")
            .cloned()
            .unwrap_or_else(|| "عذراً، الخدمة غير متاحة حالياً.".to_string());
        
        Ok(RAGProcessingResponse {
            answer: fallback_response,
            confidence: 0.1, // Very low confidence for offline responses
            sources: Vec::new(),
            processing_time_ms: 0,
            cache_hit: false,
            model_used: "offline_fallback".to_string(),
            fallback_used: true,
            warnings: vec!["Service unavailable - offline response provided".to_string()],
        })
    }
    
    /// Build vector filter from request parameters
    fn build_vector_filter(&self, request: &RAGProcessingRequest) -> Option<VectorFilter> {
        let mut filter = VectorFilter {
            content_types: request.preferred_source_types.clone(),
            sources: None,
            authors: None,
            languages: request.language.as_ref().map(|l| vec![l.clone()]),
            authenticity_levels: None,
            date_range: None,
            metadata_filters: None,
        };
        
        // Only return filter if it has meaningful constraints
        if filter.content_types.is_some() || filter.languages.is_some() {
            Some(filter)
        } else {
            None
        }
    }
    
    /// Generate cache key for a request
    fn generate_cache_key(&self, request: &RAGProcessingRequest) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        request.question.hash(&mut hasher);
        request.context.hash(&mut hasher);
        request.max_sources.hash(&mut hasher);
        request.preferred_source_types.hash(&mut hasher);
        request.language.hash(&mut hasher);
        
        format!("rag_request_{:x}", hasher.finish())
    }
    
    /// Index new content in the vector database
    pub async fn index_content(&mut self, content: IslamicSource) -> Result<()> {
        // Generate embedding for the content
        let embedding = self.hf_client
            .generate_embeddings(vec![content.text.clone()], &self.config.hugging_face.embedding_model)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| AIServiceError::ExternalAPIError("No embedding returned".to_string()))?;
        
        // Create vector document
        let mut metadata = HashMap::new();
        for (key, value) in &content.metadata {
            metadata.insert(key.clone(), serde_json::Value::String(value.clone()));
        }
        
        let payload = VectorPayload {
            text: content.text.clone(),
            content_type: format!("{:?}", content.content_type),
            source: content.reference.clone(),
            author: content.author.clone(),
            language: format!("{:?}", content.language),
            authenticity: format!("{:?}", content.authenticity),
            reference: content.reference.clone(),
            created_at: Some(content.created_at.timestamp()),
            updated_at: Some(Utc::now().timestamp()),
            metadata,
            keywords: Vec::new(), // Would be extracted in production
            concepts: Vec::new(),  // Would be extracted in production
            text_length: content.text.len(),
            word_count: content.text.split_whitespace().count(),
        };
        
        let document = VectorDocument {
            id: content.id,
            vector: embedding,
            payload,
        };
        
        self.vector_db.index_document(document).await?;
        
        Ok(())
    }
    
    /// Get service health status
    pub async fn health_check(&self) -> Result<ServiceHealthStatus> {
        let mut status = ServiceHealthStatus {
            overall_status: "healthy".to_string(),
            hugging_face_status: "unknown".to_string(),
            vector_db_status: "unknown".to_string(),
            cache_status: "unknown".to_string(),
            last_check: Utc::now(),
            error_details: Vec::new(),
        };
        
        // Check Hugging Face API
        match self.hf_client.check_model_status(&self.config.hugging_face.default_model).await {
            Ok(true) => status.hugging_face_status = "healthy".to_string(),
            Ok(false) => status.hugging_face_status = "loading".to_string(),
            Err(e) => {
                status.hugging_face_status = "unhealthy".to_string();
                status.error_details.push(format!("Hugging Face: {}", e));
            }
        }
        
        // Check Vector Database
        match self.vector_db.get_collection_stats().await {
            Ok(_) => status.vector_db_status = "healthy".to_string(),
            Err(e) => {
                status.vector_db_status = "unhealthy".to_string();
                status.error_details.push(format!("Vector DB: {}", e));
            }
        }
        
        // Check Cache
        status.cache_status = if self.cache.is_healthy() {
            "healthy".to_string()
        } else {
            "degraded".to_string()
        };
        
        // Determine overall status
        if status.hugging_face_status == "unhealthy" || status.vector_db_status == "unhealthy" {
            status.overall_status = "unhealthy".to_string();
        } else if status.hugging_face_status == "loading" || status.cache_status == "degraded" {
            status.overall_status = "degraded".to_string();
        }
        
        Ok(status)
    }
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealthStatus {
    pub overall_status: String,
    pub hugging_face_status: String,
    pub vector_db_status: String,
    pub cache_status: String,
    pub last_check: DateTime<Utc>,
    pub error_details: Vec<String>,
}

impl CacheManager {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            config,
            query_cache: HashMap::new(),
            response_cache: HashMap::new(),
            embedding_cache: HashMap::new(),
        }
    }
    
    pub fn get_cached_response(&self, key: &str) -> Option<&CachedResponse> {
        if let Some(cached) = self.response_cache.get(key) {
            if Utc::now().signed_duration_since(cached.timestamp) < chrono::Duration::from_std(self.config.response_cache_ttl).unwrap() {
                return Some(cached);
            }
        }
        None
    }
    
    pub fn cache_response(&mut self, key: &str, response: &str, confidence: f32) {
        if self.response_cache.len() >= self.config.max_cache_size {
            // Simple LRU eviction - remove oldest entry
            if let Some(oldest_key) = self.response_cache.keys().next().cloned() {
                self.response_cache.remove(&oldest_key);
            }
        }
        
        self.response_cache.insert(key.to_string(), CachedResponse {
            response: response.to_string(),
            confidence,
            timestamp: Utc::now(),
            hit_count: 0,
        });
    }
    
    pub fn get_cached_embedding(&self, text: &str) -> Option<&CachedEmbedding> {
        if let Some(cached) = self.embedding_cache.get(text) {
            if Utc::now().signed_duration_since(cached.timestamp) < chrono::Duration::from_std(self.config.embedding_cache_ttl).unwrap() {
                return Some(cached);
            }
        }
        None
    }
    
    pub fn cache_embedding(&mut self, text: &str, embedding: &[f32]) {
        if self.embedding_cache.len() >= self.config.max_cache_size {
            // Simple LRU eviction
            if let Some(oldest_key) = self.embedding_cache.keys().next().cloned() {
                self.embedding_cache.remove(&oldest_key);
            }
        }
        
        self.embedding_cache.insert(text.to_string(), CachedEmbedding {
            embedding: embedding.to_vec(),
            timestamp: Utc::now(),
            hit_count: 0,
        });
    }
    
    pub fn is_healthy(&self) -> bool {
        // Simple health check - in production would check Redis connection
        true
    }
}

impl FallbackHandler {
    pub fn new(config: FallbackConfig) -> Self {
        Self {
            config,
            failure_counts: HashMap::new(),
            last_failure_times: HashMap::new(),
        }
    }
    
    pub fn record_failure(&mut self, service: &str) {
        let count = self.failure_counts.entry(service.to_string()).or_insert(0);
        *count += 1;
        self.last_failure_times.insert(service.to_string(), Utc::now());
    }
    
    pub fn should_use_fallback(&self, service: &str) -> bool {
        if let Some(&count) = self.failure_counts.get(service) {
            count >= 3 // Use fallback after 3 failures
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_integration_service_creation() {
        let config = IntegrationConfig::default();
        
        // This would require running services in a real test
        // let service = IntegrationService::new(config).await;
        // assert!(service.is_ok());
    }

    #[test]
    fn test_cache_key_generation() {
        let config = IntegrationConfig::default();
        let service = IntegrationService {
            hf_client: HuggingFaceClient::new(config.hugging_face.clone()).unwrap(),
            vector_db: VectorDatabaseClient::new(config.vector_database.clone()).await.unwrap(),
            cache: CacheManager::new(config.cache.clone()),
            config: config.clone(),
            fallback_handler: FallbackHandler::new(config.fallback.clone()),
        };
        
        let request = RAGProcessingRequest {
            question: "ما هي أركان الإسلام؟".to_string(),
            context: None,
            max_sources: Some(5),
            similarity_threshold: None,
            preferred_source_types: None,
            language: Some("Arabic".to_string()),
            user_id: None,
        };
        
        let key1 = service.generate_cache_key(&request);
        let key2 = service.generate_cache_key(&request);
        
        assert_eq!(key1, key2);
        assert!(key1.starts_with("rag_request_"));
    }

    #[test]
    fn test_confidence_calculation() {
        let config = IntegrationConfig::default();
        let service = IntegrationService {
            hf_client: HuggingFaceClient::new(config.hugging_face.clone()).unwrap(),
            vector_db: VectorDatabaseClient::new(config.vector_database.clone()).await.unwrap(),
            cache: CacheManager::new(config.cache.clone()),
            config: config.clone(),
            fallback_handler: FallbackHandler::new(config.fallback.clone()),
        };
        
        let sources = vec![
            RetrievedSource {
                id: "test1".to_string(),
                text: "Test source".to_string(),
                reference: "Test:1".to_string(),
                source_type: "quran".to_string(),
                similarity_score: 0.9,
                authenticity: "verified".to_string(),
                author: None,
            }
        ];
        
        let response = "هذه إجابة تحتوي على المصدر المذكور أعلاه.";
        let confidence = service.calculate_response_confidence(response, &sources);
        
        assert!(confidence > 0.5);
        assert!(confidence <= 1.0);
    }
}
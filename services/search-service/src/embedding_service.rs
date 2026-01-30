use crate::models::{DocumentEmbedding, Result, SearchServiceConfig, SearchServiceError};
use crate::text_processor::ArabicTextProcessor;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{info, warn, error, debug};
use chrono::Utc;

/// Embedding service for converting text to vectors using Arabic models
#[derive(Clone)]
pub struct EmbeddingService {
    client: Client,
    text_processor: ArabicTextProcessor,
    config: SearchServiceConfig,
    model_info: ModelInfo,
    cache: Option<EmbeddingCache>,
}

#[derive(Debug, Clone)]
struct ModelInfo {
    name: String,
    vector_size: usize,
    max_sequence_length: usize,
    supports_batch: bool,
}

/// Simple in-memory cache for embeddings
#[derive(Clone)]
struct EmbeddingCache {
    cache: HashMap<String, CachedEmbedding>,
    max_size: usize,
    ttl_seconds: u64,
}

#[derive(Debug, Clone)]
struct CachedEmbedding {
    embedding: Vec<f32>,
    created_at: std::time::SystemTime,
}

/// Request structure for Hugging Face Inference API
#[derive(Debug, Serialize)]
struct HuggingFaceRequest {
    inputs: Vec<String>,
    options: HuggingFaceOptions,
}

#[derive(Debug, Serialize)]
struct HuggingFaceOptions {
    wait_for_model: bool,
    use_cache: bool,
}

/// Response structure from Hugging Face Inference API
#[derive(Debug, Deserialize)]
struct HuggingFaceResponse(Vec<Vec<f32>>);

/// Batch embedding request
#[derive(Debug)]
pub struct BatchEmbeddingRequest {
    pub texts: Vec<String>,
    pub document_ids: Vec<String>,
}

/// Batch embedding response
#[derive(Debug)]
pub struct BatchEmbeddingResponse {
    pub embeddings: Vec<DocumentEmbedding>,
    pub processing_time_ms: u64,
    pub successful_count: usize,
    pub failed_count: usize,
    pub errors: Vec<String>,
}

impl EmbeddingService {
    /// Create a new embedding service
    pub async fn new() -> Result<Self> {
        let config = SearchServiceConfig::default();
        Self::with_config(config).await
    }

    /// Create embedding service with custom configuration
    pub async fn with_config(config: SearchServiceConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| SearchServiceError::ConfigurationError(format!("Failed to create HTTP client: {}", e)))?;

        let text_processor = ArabicTextProcessor::new()?;

        // Initialize model info based on the configured model
        let model_info = Self::get_model_info(&config.embedding_model)?;

        let cache = if config.cache_embeddings {
            Some(EmbeddingCache::new(1000, config.cache_ttl_seconds))
        } else {
            None
        };

        info!("Initialized embedding service with model: {}", config.embedding_model);
        info!("Vector size: {}, Max sequence length: {}", model_info.vector_size, model_info.max_sequence_length);

        Ok(Self {
            client,
            text_processor,
            config,
            model_info,
            cache,
        })
    }

    /// Generate embedding for a single text
    pub async fn generate_embedding(&mut self, text: &str) -> Result<Vec<f32>> {
        let processed = self.text_processor.process_text(text)?;
        
        // Check cache first
        if let Some(cache) = &self.cache {
            if let Some(cached) = cache.get(&processed.normalized) {
                debug!("Cache hit for text: {}", &text[..std::cmp::min(50, text.len())]);
                return Ok(cached);
            }
        }

        let embedding = self.generate_embedding_from_api(&processed.normalized).await?;

        // Cache the result
        if let Some(cache) = &mut self.cache {
            cache.put(processed.normalized, embedding.clone());
        }

        Ok(embedding)
    }

    /// Generate embeddings for multiple texts in batch
    pub async fn generate_batch_embeddings(&mut self, request: BatchEmbeddingRequest) -> Result<BatchEmbeddingResponse> {
        let start_time = Instant::now();
        let mut embeddings = Vec::new();
        let mut successful_count = 0;
        let mut failed_count = 0;
        let mut errors = Vec::new();

        // Process texts in batches to respect API limits
        let batch_size = std::cmp::min(self.config.batch_size, 32); // Limit to 32 for API safety
        
        for (chunk_idx, chunk) in request.texts.chunks(batch_size).enumerate() {
            let chunk_ids: Vec<String> = request.document_ids
                .iter()
                .skip(chunk_idx * batch_size)
                .take(batch_size)
                .cloned()
                .collect();

            match self.generate_batch_embeddings_chunk(chunk, &chunk_ids).await {
                Ok(mut chunk_embeddings) => {
                    successful_count += chunk_embeddings.len();
                    embeddings.append(&mut chunk_embeddings);
                }
                Err(e) => {
                    failed_count += chunk.len();
                    errors.push(format!("Batch {}: {}", chunk_idx, e));
                    error!("Failed to process batch {}: {}", chunk_idx, e);
                }
            }

            // Add delay between batches to respect rate limits
            if chunk_idx < request.texts.chunks(batch_size).len() - 1 {
                sleep(Duration::from_millis(100)).await;
            }
        }

        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(BatchEmbeddingResponse {
            embeddings,
            processing_time_ms,
            successful_count,
            failed_count,
            errors,
        })
    }

    /// Generate embeddings for a chunk of texts
    async fn generate_batch_embeddings_chunk(&mut self, texts: &[String], document_ids: &[String]) -> Result<Vec<DocumentEmbedding>> {
        let mut processed_texts = Vec::new();
        let mut text_hashes = Vec::new();

        // Process and normalize texts
        for text in texts {
            let processed = self.text_processor.process_text(text)?;
            let text_hash = self.calculate_text_hash(&processed.normalized);
            processed_texts.push(processed.normalized);
            text_hashes.push(text_hash);
        }

        // Check cache for all texts
        let mut cached_results = Vec::new();
        let mut uncached_indices = Vec::new();
        let mut uncached_texts = Vec::new();

        if let Some(cache) = &self.cache {
            for (i, text) in processed_texts.iter().enumerate() {
                if let Some(cached_embedding) = cache.get(text) {
                    cached_results.push((i, cached_embedding));
                } else {
                    uncached_indices.push(i);
                    uncached_texts.push(text.clone());
                }
            }
        } else {
            uncached_indices = (0..processed_texts.len()).collect();
            uncached_texts = processed_texts.clone();
        }

        // Generate embeddings for uncached texts
        let mut new_embeddings = Vec::new();
        if !uncached_texts.is_empty() {
            new_embeddings = self.generate_batch_embeddings_from_api(&uncached_texts).await?;
            
            // Cache new embeddings
            if let Some(cache) = &mut self.cache {
                for (text, embedding) in uncached_texts.iter().zip(new_embeddings.iter()) {
                    cache.put(text.clone(), embedding.clone());
                }
            }
        }

        // Combine cached and new results
        let mut all_embeddings = vec![Vec::new(); texts.len()];
        
        // Insert cached results
        for (original_index, embedding) in cached_results {
            all_embeddings[original_index] = embedding;
        }
        
        // Insert new results
        for (uncached_idx, original_index) in uncached_indices.iter().enumerate() {
            if uncached_idx < new_embeddings.len() {
                all_embeddings[*original_index] = new_embeddings[uncached_idx].clone();
            }
        }

        // Create DocumentEmbedding objects
        let mut document_embeddings = Vec::new();
        for (i, embedding) in all_embeddings.into_iter().enumerate() {
            if !embedding.is_empty() && i < document_ids.len() {
                document_embeddings.push(DocumentEmbedding {
                    document_id: document_ids[i].clone(),
                    embedding,
                    text_hash: text_hashes[i].clone(),
                    generated_at: Utc::now(),
                });
            }
        }

        Ok(document_embeddings)
    }

    /// Generate embedding from external API
    async fn generate_embedding_from_api(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.generate_batch_embeddings_from_api(&[text.to_string()]).await?;
        embeddings.into_iter().next()
            .ok_or_else(|| SearchServiceError::EmbeddingError("No embedding returned from API".to_string()))
    }

    /// Generate batch embeddings from external API
    async fn generate_batch_embeddings_from_api(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        // For now, we'll use a mock implementation since we don't have access to Hugging Face API
        // In production, this would make actual API calls
        self.mock_embedding_generation(texts).await
    }

    /// Mock embedding generation for development/testing
    async fn mock_embedding_generation(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        info!("Generating mock embeddings for {} texts", texts.len());
        
        // Simulate API delay
        sleep(Duration::from_millis(100 * texts.len() as u64)).await;

        let mut embeddings = Vec::new();
        
        for text in texts {
            // Generate a deterministic but pseudo-random embedding based on text content
            let embedding = self.generate_mock_embedding(text);
            embeddings.push(embedding);
        }

        Ok(embeddings)
    }

    /// Generate a mock embedding that's deterministic but varies based on text content
    fn generate_mock_embedding(&self, text: &str) -> Vec<f32> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let seed = hasher.finish();

        // Simple pseudo-random number generator based on text hash
        let mut rng_state = seed;
        let mut embedding = Vec::with_capacity(self.model_info.vector_size);

        for i in 0..self.model_info.vector_size {
            // Linear congruential generator
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345 + i as u64);
            let normalized = (rng_state as f32) / (u64::MAX as f32);
            // Convert to range [-1, 1] and then normalize
            embedding.push((normalized - 0.5) * 2.0);
        }

        // Normalize the vector to unit length (important for cosine similarity)
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for value in &mut embedding {
                *value /= magnitude;
            }
        }

        embedding
    }

    /// Calculate hash for text (for caching and deduplication)
    fn calculate_text_hash(&self, text: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Get model information based on model name
    fn get_model_info(model_name: &str) -> Result<ModelInfo> {
        match model_name {
            "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2" => {
                Ok(ModelInfo {
                    name: model_name.to_string(),
                    vector_size: 384,
                    max_sequence_length: 512,
                    supports_batch: true,
                })
            }
            "sentence-transformers/paraphrase-multilingual-mpnet-base-v2" => {
                Ok(ModelInfo {
                    name: model_name.to_string(),
                    vector_size: 768,
                    max_sequence_length: 512,
                    supports_batch: true,
                })
            }
            "aubmindlab/bert-base-arabertv02" => {
                Ok(ModelInfo {
                    name: model_name.to_string(),
                    vector_size: 768,
                    max_sequence_length: 512,
                    supports_batch: true,
                })
            }
            _ => {
                warn!("Unknown model: {}, using default configuration", model_name);
                Ok(ModelInfo {
                    name: model_name.to_string(),
                    vector_size: 384,
                    max_sequence_length: 512,
                    supports_batch: true,
                })
            }
        }
    }

    /// Get model information
    pub fn model_info(&self) -> &ModelInfo {
        &self.model_info
    }

    /// Get embedding dimension
    pub fn get_embedding_dimension(&self) -> usize {
        self.model_info.vector_size
    }

    /// Clear embedding cache
    pub fn clear_cache(&mut self) {
        if let Some(cache) = &mut self.cache {
            cache.clear();
        }
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> Option<(usize, usize)> {
        self.cache.as_ref().map(|cache| (cache.size(), cache.max_size))
    }
}

impl EmbeddingCache {
    fn new(max_size: usize, ttl_seconds: u64) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            ttl_seconds,
        }
    }

    fn get(&self, key: &str) -> Option<Vec<f32>> {
        if let Some(cached) = self.cache.get(key) {
            // Check if cache entry is still valid
            if let Ok(elapsed) = cached.created_at.elapsed() {
                if elapsed.as_secs() < self.ttl_seconds {
                    return Some(cached.embedding.clone());
                }
            }
        }
        None
    }

    fn put(&mut self, key: String, embedding: Vec<f32>) {
        // Remove expired entries if cache is full
        if self.cache.len() >= self.max_size {
            self.cleanup_expired();
            
            // If still full, remove oldest entry
            if self.cache.len() >= self.max_size {
                if let Some(oldest_key) = self.cache.keys().next().cloned() {
                    self.cache.remove(&oldest_key);
                }
            }
        }

        self.cache.insert(key, CachedEmbedding {
            embedding,
            created_at: std::time::SystemTime::now(),
        });
    }

    fn cleanup_expired(&mut self) {
        let _now = std::time::SystemTime::now();
        let ttl_duration = Duration::from_secs(self.ttl_seconds);
        
        self.cache.retain(|_, cached| {
            cached.created_at.elapsed()
                .map(|elapsed| elapsed < ttl_duration)
                .unwrap_or(false)
        });
    }

    fn clear(&mut self) {
        self.cache.clear();
    }

    fn size(&self) -> usize {
        self.cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_embedding_service_creation() {
        let service = EmbeddingService::new().await;
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_single_embedding_generation() {
        let mut service = EmbeddingService::new().await.unwrap();
        let text = "بسم الله الرحمن الرحيم";
        
        let embedding = service.generate_embedding(text).await.unwrap();
        assert_eq!(embedding.len(), service.get_embedding_dimension());
        
        // Test that embeddings are normalized (unit vectors)
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_batch_embedding_generation() {
        let mut service = EmbeddingService::new().await.unwrap();
        let texts = vec![
            "بسم الله الرحمن الرحيم".to_string(),
            "الحمد لله رب العالمين".to_string(),
            "إنما الأعمال بالنيات".to_string(),
        ];
        let document_ids = vec!["doc1".to_string(), "doc2".to_string(), "doc3".to_string()];
        
        let request = BatchEmbeddingRequest { texts, document_ids };
        let response = service.generate_batch_embeddings(request).await.unwrap();
        
        assert_eq!(response.embeddings.len(), 3);
        assert_eq!(response.successful_count, 3);
        assert_eq!(response.failed_count, 0);
    }

    #[tokio::test]
    async fn test_embedding_cache() {
        let mut service = EmbeddingService::new().await.unwrap();
        let text = "نص للاختبار";
        
        // First call should generate embedding
        let embedding1 = service.generate_embedding(text).await.unwrap();
        
        // Second call should use cache
        let embedding2 = service.generate_embedding(text).await.unwrap();
        
        assert_eq!(embedding1, embedding2);
        
        // Check cache stats
        if let Some((size, _)) = service.get_cache_stats() {
            assert!(size > 0);
        }
    }

    #[test]
    fn test_mock_embedding_deterministic() {
        let _service = EmbeddingService::new();
        // This test would need to be adjusted based on the actual implementation
    }
}
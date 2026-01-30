use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::collections::HashMap;

/// Main configuration for the AI service integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIServiceConfig {
    pub hugging_face: HuggingFaceConfig,
    pub vector_database: VectorDatabaseConfig,
    pub cache: CacheConfig,
    pub fallback: FallbackConfig,
    pub rate_limiting: RateLimitConfig,
    pub monitoring: MonitoringConfig,
}

/// Hugging Face API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuggingFaceConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub requests_per_minute: u32,
    pub default_model: String,
    pub embedding_model: String,
    pub islamic_models: Vec<IslamicModel>,
}

/// Islamic-specific model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicModel {
    pub name: String,
    pub model_id: String,
    pub specialization: ModelSpecialization,
    pub language: String,
    pub priority: u8,
    pub max_tokens: u32,
    pub temperature: f32,
}

/// Model specialization types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelSpecialization {
    General,
    Quran,
    Hadith,
    Fiqh,
    Tafsir,
    Arabic,
}

/// Vector database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDatabaseConfig {
    pub host: String,
    pub port: u16,
    pub collection_name: String,
    pub vector_size: usize,
    pub distance_metric: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub batch_size: usize,
    pub index_settings: IndexSettings,
}

/// Vector database index settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSettings {
    pub hnsw_ef_construct: u32,
    pub hnsw_m: u32,
    pub quantization_enabled: bool,
    pub on_disk_payload: bool,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enable_query_cache: bool,
    pub enable_response_cache: bool,
    pub enable_embedding_cache: bool,
    pub query_cache_ttl_seconds: u64,
    pub response_cache_ttl_seconds: u64,
    pub embedding_cache_ttl_seconds: u64,
    pub max_cache_size: usize,
    pub redis: Option<RedisConfig>,
    pub local_cache: LocalCacheConfig,
}

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
    pub timeout_seconds: u64,
    pub cluster_mode: bool,
    pub cluster_nodes: Vec<String>,
}

/// Local cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalCacheConfig {
    pub max_memory_mb: usize,
    pub eviction_policy: String,
    pub compression_enabled: bool,
}

/// Fallback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackConfig {
    pub enable_fallback: bool,
    pub fallback_models: Vec<String>,
    pub max_fallback_attempts: u32,
    pub fallback_delay_seconds: u64,
    pub enable_offline_mode: bool,
    pub offline_responses: HashMap<String, String>,
    pub circuit_breaker: CircuitBreakerConfig,
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub recovery_timeout_seconds: u64,
    pub half_open_max_calls: u32,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub burst_limit: u32,
    pub enable_adaptive_rate_limiting: bool,
    pub per_user_limits: HashMap<String, UserRateLimit>,
}

/// Per-user rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRateLimit {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub priority: u8,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enable_metrics: bool,
    pub enable_tracing: bool,
    pub metrics_endpoint: String,
    pub log_level: String,
    pub health_check_interval_seconds: u64,
}

impl Default for AIServiceConfig {
    fn default() -> Self {
        Self {
            hugging_face: HuggingFaceConfig::default(),
            vector_database: VectorDatabaseConfig::default(),
            cache: CacheConfig::default(),
            fallback: FallbackConfig::default(),
            rate_limiting: RateLimitConfig::default(),
            monitoring: MonitoringConfig::default(),
        }
    }
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
            islamic_models: vec![
                IslamicModel {
                    name: "Arabic Islamic General".to_string(),
                    model_id: "aubmindlab/bert-base-arabertv02".to_string(),
                    specialization: ModelSpecialization::General,
                    language: "Arabic".to_string(),
                    priority: 1,
                    max_tokens: 1000,
                    temperature: 0.3,
                },
                IslamicModel {
                    name: "Arabic BERT CamelBERT".to_string(),
                    model_id: "CAMeL-Lab/bert-base-arabic-camelbert-mix".to_string(),
                    specialization: ModelSpecialization::Arabic,
                    language: "Arabic".to_string(),
                    priority: 2,
                    max_tokens: 1000,
                    temperature: 0.3,
                },
            ],
        }
    }
}

impl Default for VectorDatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 6333,
            collection_name: "islamic_sources".to_string(),
            vector_size: 384,
            distance_metric: "Cosine".to_string(),
            timeout_seconds: 30,
            max_retries: 3,
            batch_size: 100,
            index_settings: IndexSettings {
                hnsw_ef_construct: 200,
                hnsw_m: 16,
                quantization_enabled: false,
                on_disk_payload: true,
            },
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enable_query_cache: true,
            enable_response_cache: true,
            enable_embedding_cache: true,
            query_cache_ttl_seconds: 3600,
            response_cache_ttl_seconds: 1800,
            embedding_cache_ttl_seconds: 7200,
            max_cache_size: 10000,
            redis: None,
            local_cache: LocalCacheConfig {
                max_memory_mb: 512,
                eviction_policy: "LRU".to_string(),
                compression_enabled: true,
            },
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
        offline_responses.insert(
            "network_error".to_string(),
            "حدث خطأ في الاتصال بالشبكة. يرجى التحقق من اتصالك بالإنترنت والمحاولة مرة أخرى.".to_string()
        );
        offline_responses.insert(
            "service_unavailable".to_string(),
            "الخدمة غير متاحة مؤقتاً. نعمل على حل المشكلة. يرجى المحاولة لاحقاً.".to_string()
        );

        Self {
            enable_fallback: true,
            fallback_models: vec![
                "aubmindlab/bert-base-arabertv02".to_string(),
                "CAMeL-Lab/bert-base-arabic-camelbert-mix".to_string(),
            ],
            max_fallback_attempts: 3,
            fallback_delay_seconds: 2,
            enable_offline_mode: true,
            offline_responses,
            circuit_breaker: CircuitBreakerConfig {
                failure_threshold: 5,
                recovery_timeout_seconds: 60,
                half_open_max_calls: 3,
            },
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
            per_user_limits: HashMap::new(),
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            enable_tracing: true,
            metrics_endpoint: "/metrics".to_string(),
            log_level: "INFO".to_string(),
            health_check_interval_seconds: 30,
        }
    }
}

impl AIServiceConfig {
    /// Load configuration from file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: AIServiceConfig = if path.ends_with(".yaml") || path.ends_with(".yml") {
            serde_yaml::from_str(&content)?
        } else if path.ends_with(".json") {
            serde_json::from_str(&content)?
        } else {
            return Err("Unsupported configuration file format".into());
        };
        Ok(config)
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        // Override with environment variables
        if let Ok(api_key) = std::env::var("HUGGING_FACE_API_KEY") {
            config.hugging_face.api_key = api_key;
        }
        
        if let Ok(qdrant_host) = std::env::var("QDRANT_HOST") {
            config.vector_database.host = qdrant_host;
        }
        
        if let Ok(qdrant_port) = std::env::var("QDRANT_PORT") {
            if let Ok(port) = qdrant_port.parse() {
                config.vector_database.port = port;
            }
        }
        
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            config.cache.redis = Some(RedisConfig {
                url: redis_url,
                pool_size: 10,
                timeout_seconds: 5,
                cluster_mode: false,
                cluster_nodes: vec![],
            });
        }
        
        config
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.hugging_face.api_key.is_empty() {
            return Err("Hugging Face API key is required".to_string());
        }
        
        if self.vector_database.host.is_empty() {
            return Err("Vector database host is required".to_string());
        }
        
        if self.vector_database.vector_size == 0 {
            return Err("Vector size must be greater than 0".to_string());
        }
        
        if self.rate_limiting.requests_per_minute == 0 {
            return Err("Rate limit must be greater than 0".to_string());
        }
        
        Ok(())
    }

    /// Get the best Islamic model for a given specialization
    pub fn get_best_model(&self, specialization: ModelSpecialization) -> Option<&IslamicModel> {
        self.hugging_face
            .islamic_models
            .iter()
            .filter(|model| matches!(model.specialization, specialization) || matches!(model.specialization, ModelSpecialization::General))
            .min_by_key(|model| model.priority)
    }

    /// Get fallback models in priority order
    pub fn get_fallback_models(&self) -> &[String] {
        &self.fallback.fallback_models
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AIServiceConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        let mut config = AIServiceConfig::default();
        config.hugging_face.api_key = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_best_model_selection() {
        let config = AIServiceConfig::default();
        let model = config.get_best_model(ModelSpecialization::General);
        assert!(model.is_some());
        assert_eq!(model.unwrap().priority, 1);
    }

    #[test]
    fn test_env_config_loading() {
        std::env::set_var("HUGGING_FACE_API_KEY", "test_key");
        std::env::set_var("QDRANT_HOST", "test_host");
        std::env::set_var("QDRANT_PORT", "9999");
        
        let config = AIServiceConfig::from_env();
        assert_eq!(config.hugging_face.api_key, "test_key");
        assert_eq!(config.vector_database.host, "test_host");
        assert_eq!(config.vector_database.port, 9999);
        
        // Clean up
        std::env::remove_var("HUGGING_FACE_API_KEY");
        std::env::remove_var("QDRANT_HOST");
        std::env::remove_var("QDRANT_PORT");
    }
}
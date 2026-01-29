use serde::{Deserialize, Serialize};
use std::env;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub qdrant: QdrantConfig,
    pub external_apis: ExternalApisConfig,
    pub security: SecurityConfig,
    pub logging: LoggingConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub max_connections: Option<usize>,
    pub request_timeout_seconds: u64,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub max_lifetime_seconds: u64,
}

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
    pub connection_timeout_seconds: u64,
    pub default_ttl_seconds: u64,
    pub cluster_enabled: bool,
    pub cluster_nodes: Vec<String>,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
}

/// Qdrant vector database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QdrantConfig {
    pub url: String,
    pub api_key: Option<String>,
    pub collection_name: String,
    pub vector_size: usize,
    pub distance_metric: String,
}

/// External APIs configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalApisConfig {
    pub hugging_face: HuggingFaceConfig,
    pub geo_location: GeoLocationConfig,
    pub embedding_service: EmbeddingServiceConfig,
}

/// Hugging Face API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuggingFaceConfig {
    pub api_key: String,
    pub base_url: String,
    pub model_name: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

/// Geo location service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocationConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout_seconds: u64,
}

/// Embedding service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingServiceConfig {
    pub model_name: String,
    pub api_url: String,
    pub api_key: Option<String>,
    pub batch_size: usize,
    pub timeout_seconds: u64,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_expiration_hours: u64,
    pub password_salt_rounds: u32,
    pub rate_limit_requests_per_minute: u32,
    pub cors_allowed_origins: Vec<String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub file_path: Option<String>,
    pub max_file_size_mb: Option<u64>,
    pub max_files: Option<u32>,
}

impl AppConfig {
    /// Load configuration from environment variables and config files
    pub fn load() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::builder()
            // Start with default values
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("server.request_timeout_seconds", 30)?
            .set_default("database.max_connections", 10)?
            .set_default("database.min_connections", 1)?
            .set_default("database.connection_timeout_seconds", 30)?
            .set_default("database.idle_timeout_seconds", 600)?
            .set_default("database.max_lifetime_seconds", 3600)?
            .set_default("redis.pool_size", 10)?
            .set_default("redis.connection_timeout_seconds", 5)?
            .set_default("redis.default_ttl_seconds", 3600)?
            .set_default("redis.cluster_enabled", false)?
            .set_default("redis.max_retries", 3)?
            .set_default("redis.retry_delay_ms", 100)?
            .set_default("qdrant.collection_name", "islamic_content")?
            .set_default("qdrant.vector_size", 384)?
            .set_default("qdrant.distance_metric", "Cosine")?
            .set_default("external_apis.hugging_face.base_url", "https://api-inference.huggingface.co")?
            .set_default("external_apis.hugging_face.timeout_seconds", 30)?
            .set_default("external_apis.hugging_face.max_retries", 3)?
            .set_default("external_apis.embedding_service.batch_size", 32)?
            .set_default("external_apis.embedding_service.timeout_seconds", 30)?
            .set_default("security.jwt_expiration_hours", 24)?
            .set_default("security.password_salt_rounds", 12)?
            .set_default("security.rate_limit_requests_per_minute", 60)?
            .set_default("logging.level", "info")?
            .set_default("logging.format", "json")?;

        // Add configuration file if it exists
        if let Ok(config_path) = env::var("CONFIG_PATH") {
            settings = settings.add_source(config::File::with_name(&config_path));
        } else {
            // Try default config file locations
            settings = settings
                .add_source(config::File::with_name("config/default").required(false))
                .add_source(config::File::with_name("config/local").required(false));
        }

        // Add environment variables (with prefix SANAD_)
        settings = settings.add_source(
            config::Environment::with_prefix("SANAD")
                .separator("_")
                .try_parsing(true),
        );

        settings.build()?.try_deserialize()
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.database.url.is_empty() {
            return Err("Database URL is required".to_string());
        }

        if self.redis.url.is_empty() {
            return Err("Redis URL is required".to_string());
        }

        if self.qdrant.url.is_empty() {
            return Err("Qdrant URL is required".to_string());
        }

        if self.external_apis.hugging_face.api_key.is_empty() {
            return Err("Hugging Face API key is required".to_string());
        }

        if self.security.jwt_secret.is_empty() {
            return Err("JWT secret is required".to_string());
        }

        Ok(())
    }
}
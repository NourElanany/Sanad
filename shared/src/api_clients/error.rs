//! Error types for API clients

use thiserror::Error;

/// API client error types
#[derive(Debug, Error, Clone)]
pub enum ApiError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("HTTP error: {0}")]
    Http(String),
    
    #[error("Rate limit exceeded for API: {0}")]
    RateLimitExceeded(String),
    
    #[error("API key not found: {0}")]
    ApiKeyNotFound(String),
    
    #[error("API key inactive: {0}")]
    ApiKeyInactive(String),
    
    #[error("API key expired: {0}")]
    ApiKeyExpired(String),
    
    #[error("Invalid response from API {0}: {1}")]
    InvalidResponse(String, String),
    
    #[error("API {0} returned error: {1}")]
    ApiError(String, String),
    
    #[error("All APIs failed for request")]
    AllApisFailed,
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Unknown API: {0}")]
    UnknownApi(String),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Timeout")]
    Timeout,
    
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

/// Result type for API operations
pub type ApiResult<T> = Result<T, ApiError>;

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::Http(err.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::Serialization(err.to_string())
    }
}

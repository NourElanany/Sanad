use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::ApiResponse;

/// Common error types for the Islamic application
#[derive(Error, Debug)]
pub enum SanadError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Islamic content integrity error: {0}")]
    ContentIntegrity(String),

    #[error("Prayer time calculation error: {0}")]
    PrayerTimeCalculation(String),

    #[error("Audio processing error: {0}")]
    AudioProcessing(String),

    #[error("AI service error: {0}")]
    AiService(String),

    #[error("Vector database error: {0}")]
    VectorDatabase(String),

    #[error("External API error: {service}: {message}")]
    ExternalApi { service: String, message: String },

    #[error("Rate limit exceeded for service: {0}")]
    RateLimit(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

/// Result type alias for convenience
pub type SanadResult<T> = Result<T, SanadError>;

/// Application error type for HTTP handlers
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Service error: {0}")]
    Service(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Authorization(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", err)),
            AppError::Service(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Service error: {}", err)),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ApiResponse::<()>::error(error_message));
        (status, body).into_response()
    }
}

impl SanadError {
    /// Get HTTP status code for the error
    pub fn status_code(&self) -> u16 {
        match self {
            SanadError::Authentication(_) => 401,
            SanadError::Authorization(_) => 403,
            SanadError::NotFound(_) => 404,
            SanadError::Conflict(_) => 409,
            SanadError::Validation(_) => 400,
            SanadError::RateLimit(_) => 429,
            SanadError::ServiceUnavailable(_) => 503,
            _ => 500,
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            SanadError::ServiceUnavailable(_) | SanadError::RateLimit(_)
        )
    }
}
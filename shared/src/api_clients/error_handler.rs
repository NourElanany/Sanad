//! Error handling system with categorization and user-friendly messages

use crate::api_clients::error::ApiError;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

#[cfg(test)]
#[path = "error_handler_property_tests.rs"]
mod property_tests;

/// Error category for classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorCategory {
    Network,
    Authentication,
    RateLimit,
    ServerError,
    Validation,
    Timeout,
    Unknown,
}

/// Error response with detailed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_code: String,
    pub error_message: String,
    pub error_category: ErrorCategory,
    pub timestamp: SystemTime,
    pub request_id: String,
    pub retry_after: Option<Duration>,
    pub fallback_used: Option<String>,
}

/// Error handler for categorizing and formatting errors
pub struct ErrorHandler {
    service_name: String,
}

impl ErrorHandler {
    /// Create a new error handler
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }
    
    /// Categorize an API error
    pub fn categorize(&self, error: &ApiError) -> ErrorCategory {
        match error {
            ApiError::Network(_) | ApiError::Http(_) => ErrorCategory::Network,
            ApiError::Authentication(_) 
            | ApiError::ApiKeyNotFound(_) 
            | ApiError::ApiKeyInactive(_) 
            | ApiError::ApiKeyExpired(_) => ErrorCategory::Authentication,
            ApiError::RateLimitExceeded(_) => ErrorCategory::RateLimit,
            ApiError::ApiError(_, _) | ApiError::AllApisFailed => ErrorCategory::ServerError,
            ApiError::Validation(_) | ApiError::InvalidResponse(_, _) => ErrorCategory::Validation,
            ApiError::Timeout => ErrorCategory::Timeout,
            _ => ErrorCategory::Unknown,
        }
    }
    
    /// Create a user-friendly error response
    pub fn create_response(
        &self,
        error: &ApiError,
        request_id: Option<String>,
        fallback_used: Option<String>,
    ) -> ErrorResponse {
        let category = self.categorize(error);
        let error_code = self.generate_error_code(&category);
        let error_message = self.generate_user_message(error, &category);
        let retry_after = self.calculate_retry_after(&category);
        
        ErrorResponse {
            error_code,
            error_message,
            error_category: category,
            timestamp: SystemTime::now(),
            request_id: request_id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            retry_after,
            fallback_used,
        }
    }
    
    /// Generate error code based on category
    fn generate_error_code(&self, category: &ErrorCategory) -> String {
        match category {
            ErrorCategory::Network => "ERR_NETWORK".to_string(),
            ErrorCategory::Authentication => "ERR_AUTH".to_string(),
            ErrorCategory::RateLimit => "ERR_RATE_LIMIT".to_string(),
            ErrorCategory::ServerError => "ERR_SERVER".to_string(),
            ErrorCategory::Validation => "ERR_VALIDATION".to_string(),
            ErrorCategory::Timeout => "ERR_TIMEOUT".to_string(),
            ErrorCategory::Unknown => "ERR_UNKNOWN".to_string(),
        }
    }
    
    /// Generate user-friendly error message
    fn generate_user_message(&self, error: &ApiError, category: &ErrorCategory) -> String {
        match category {
            ErrorCategory::Network => {
                "Unable to connect to the service. Please check your internet connection and try again.".to_string()
            }
            ErrorCategory::Authentication => {
                "Authentication failed. Please contact support if this issue persists.".to_string()
            }
            ErrorCategory::RateLimit => {
                "Too many requests. Please wait a moment and try again.".to_string()
            }
            ErrorCategory::ServerError => {
                "The service is temporarily unavailable. Please try again later.".to_string()
            }
            ErrorCategory::Validation => {
                format!("Invalid request: {}", self.extract_validation_message(error))
            }
            ErrorCategory::Timeout => {
                "The request took too long to complete. Please try again.".to_string()
            }
            ErrorCategory::Unknown => {
                "An unexpected error occurred. Please try again or contact support.".to_string()
            }
        }
    }
    
    /// Extract validation message from error
    fn extract_validation_message(&self, error: &ApiError) -> String {
        match error {
            ApiError::Validation(msg) => msg.clone(),
            ApiError::InvalidResponse(_, msg) => msg.clone(),
            _ => "Invalid data provided".to_string(),
        }
    }
    
    /// Calculate retry after duration based on error category
    fn calculate_retry_after(&self, category: &ErrorCategory) -> Option<Duration> {
        match category {
            ErrorCategory::RateLimit => Some(Duration::from_secs(60)),
            ErrorCategory::ServerError => Some(Duration::from_secs(30)),
            ErrorCategory::Network => Some(Duration::from_secs(5)),
            ErrorCategory::Timeout => Some(Duration::from_secs(10)),
            _ => None,
        }
    }
    
    /// Check if error is retryable
    pub fn is_retryable(&self, error: &ApiError) -> bool {
        matches!(
            self.categorize(error),
            ErrorCategory::Network | ErrorCategory::ServerError | ErrorCategory::Timeout
        )
    }
    
    /// Log error with appropriate level
    pub fn log_error(&self, error: &ApiError, context: &str) {
        let category = self.categorize(error);
        match category {
            ErrorCategory::Authentication => {
                log::error!("[{}] Authentication error in {}: {}", self.service_name, context, error);
            }
            ErrorCategory::RateLimit => {
                log::warn!("[{}] Rate limit exceeded in {}: {}", self.service_name, context, error);
            }
            ErrorCategory::ServerError => {
                log::error!("[{}] Server error in {}: {}", self.service_name, context, error);
            }
            ErrorCategory::Network => {
                log::warn!("[{}] Network error in {}: {}", self.service_name, context, error);
            }
            ErrorCategory::Validation => {
                log::warn!("[{}] Validation error in {}: {}", self.service_name, context, error);
            }
            ErrorCategory::Timeout => {
                log::warn!("[{}] Timeout in {}: {}", self.service_name, context, error);
            }
            ErrorCategory::Unknown => {
                log::error!("[{}] Unknown error in {}: {}", self.service_name, context, error);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_categorize_network_error() {
        let handler = ErrorHandler::new("test-service");
        let error = ApiError::Network("Connection refused".to_string());
        assert_eq!(handler.categorize(&error), ErrorCategory::Network);
    }
    
    #[test]
    fn test_categorize_authentication_error() {
        let handler = ErrorHandler::new("test-service");
        let error = ApiError::ApiKeyNotFound("test-api".to_string());
        assert_eq!(handler.categorize(&error), ErrorCategory::Authentication);
    }
    
    #[test]
    fn test_categorize_rate_limit_error() {
        let handler = ErrorHandler::new("test-service");
        let error = ApiError::RateLimitExceeded("test-api".to_string());
        assert_eq!(handler.categorize(&error), ErrorCategory::RateLimit);
    }
    
    #[test]
    fn test_categorize_server_error() {
        let handler = ErrorHandler::new("test-service");
        let error = ApiError::AllApisFailed;
        assert_eq!(handler.categorize(&error), ErrorCategory::ServerError);
    }
    
    #[test]
    fn test_categorize_validation_error() {
        let handler = ErrorHandler::new("test-service");
        let error = ApiError::Validation("Invalid input".to_string());
        assert_eq!(handler.categorize(&error), ErrorCategory::Validation);
    }
    
    #[test]
    fn test_categorize_timeout_error() {
        let handler = ErrorHandler::new("test-service");
        let error = ApiError::Timeout;
        assert_eq!(handler.categorize(&error), ErrorCategory::Timeout);
    }
    
    #[test]
    fn test_create_response() {
        let handler = ErrorHandler::new("test-service");
        let error = ApiError::Network("Connection failed".to_string());
        let response = handler.create_response(&error, None, None);
        
        assert_eq!(response.error_code, "ERR_NETWORK");
        assert_eq!(response.error_category, ErrorCategory::Network);
        assert!(response.retry_after.is_some());
    }
    
    #[test]
    fn test_is_retryable() {
        let handler = ErrorHandler::new("test-service");
        
        assert!(handler.is_retryable(&ApiError::Network("test".to_string())));
        assert!(handler.is_retryable(&ApiError::Timeout));
        assert!(handler.is_retryable(&ApiError::AllApisFailed));
        assert!(!handler.is_retryable(&ApiError::Authentication("test".to_string())));
        assert!(!handler.is_retryable(&ApiError::Validation("test".to_string())));
    }
    
    #[test]
    fn test_error_code_generation() {
        let handler = ErrorHandler::new("test-service");
        
        assert_eq!(handler.generate_error_code(&ErrorCategory::Network), "ERR_NETWORK");
        assert_eq!(handler.generate_error_code(&ErrorCategory::Authentication), "ERR_AUTH");
        assert_eq!(handler.generate_error_code(&ErrorCategory::RateLimit), "ERR_RATE_LIMIT");
        assert_eq!(handler.generate_error_code(&ErrorCategory::ServerError), "ERR_SERVER");
        assert_eq!(handler.generate_error_code(&ErrorCategory::Validation), "ERR_VALIDATION");
        assert_eq!(handler.generate_error_code(&ErrorCategory::Timeout), "ERR_TIMEOUT");
        assert_eq!(handler.generate_error_code(&ErrorCategory::Unknown), "ERR_UNKNOWN");
    }
    
    #[test]
    fn test_retry_after_calculation() {
        let handler = ErrorHandler::new("test-service");
        
        assert_eq!(
            handler.calculate_retry_after(&ErrorCategory::RateLimit),
            Some(Duration::from_secs(60))
        );
        assert_eq!(
            handler.calculate_retry_after(&ErrorCategory::ServerError),
            Some(Duration::from_secs(30))
        );
        assert_eq!(
            handler.calculate_retry_after(&ErrorCategory::Network),
            Some(Duration::from_secs(5))
        );
        assert_eq!(
            handler.calculate_retry_after(&ErrorCategory::Timeout),
            Some(Duration::from_secs(10))
        );
        assert_eq!(handler.calculate_retry_after(&ErrorCategory::Authentication), None);
    }
}

//! Property-based tests for error handler
//! 
//! Feature: official-apis-integration
//! Property 17: Error Categorization
//! Validates: Requirements 11.1

#[cfg(test)]
mod property_tests {
    use crate::api_clients::error::ApiError;
    use crate::api_clients::error_handler::{ErrorCategory, ErrorHandler};
    use proptest::prelude::*;
    
    // Strategy to generate various API errors
    fn api_error_strategy() -> impl Strategy<Value = ApiError> {
        prop_oneof![
            // Network errors
            any::<String>().prop_map(ApiError::Network),
            
            // Authentication errors
            any::<String>().prop_map(ApiError::Authentication),
            any::<String>().prop_map(ApiError::ApiKeyNotFound),
            any::<String>().prop_map(ApiError::ApiKeyInactive),
            any::<String>().prop_map(ApiError::ApiKeyExpired),
            
            // Rate limit errors
            any::<String>().prop_map(ApiError::RateLimitExceeded),
            
            // Server errors
            (any::<String>(), any::<String>()).prop_map(|(api, msg)| ApiError::ApiError(api, msg)),
            Just(ApiError::AllApisFailed),
            
            // Validation errors
            any::<String>().prop_map(ApiError::Validation),
            (any::<String>(), any::<String>()).prop_map(|(api, msg)| ApiError::InvalidResponse(api, msg)),
            
            // Timeout errors
            Just(ApiError::Timeout),
            
            // Other errors
            Just(ApiError::NotFound),
            any::<String>().prop_map(ApiError::Configuration),
        ]
    }
    
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        /// Property 17: Error Categorization
        /// 
        /// For any API error, the Error_Handler should categorize it into one of the 
        /// defined categories (network, authentication, rate limit, server error, 
        /// validation, timeout, unknown), and the category should be included in the 
        /// error response.
        /// 
        /// **Validates: Requirements 11.1**
        #[test]
        fn property_error_categorization(error in api_error_strategy()) {
            let handler = ErrorHandler::new("test-service");
            
            // Categorize the error
            let category = handler.categorize(&error);
            
            // Verify category is one of the defined types
            prop_assert!(matches!(
                category,
                ErrorCategory::Network
                | ErrorCategory::Authentication
                | ErrorCategory::RateLimit
                | ErrorCategory::ServerError
                | ErrorCategory::Validation
                | ErrorCategory::Timeout
                | ErrorCategory::Unknown
            ));
            
            // Create error response
            let response = handler.create_response(&error, None, None);
            
            // Verify category is included in response
            prop_assert_eq!(response.error_category, category);
            
            // Verify error code matches category
            let expected_code = match category {
                ErrorCategory::Network => "ERR_NETWORK",
                ErrorCategory::Authentication => "ERR_AUTH",
                ErrorCategory::RateLimit => "ERR_RATE_LIMIT",
                ErrorCategory::ServerError => "ERR_SERVER",
                ErrorCategory::Validation => "ERR_VALIDATION",
                ErrorCategory::Timeout => "ERR_TIMEOUT",
                ErrorCategory::Unknown => "ERR_UNKNOWN",
            };
            prop_assert_eq!(response.error_code, expected_code);
            
            // Verify error message is user-friendly (not empty)
            prop_assert!(!response.error_message.is_empty());
            
            // Verify request_id is present
            prop_assert!(!response.request_id.is_empty());
        }
        
        /// Property: Consistent categorization
        /// 
        /// For any error, categorizing it multiple times should always produce 
        /// the same category.
        #[test]
        fn property_consistent_categorization(error in api_error_strategy()) {
            let handler = ErrorHandler::new("test-service");
            
            let category1 = handler.categorize(&error);
            let category2 = handler.categorize(&error);
            let category3 = handler.categorize(&error);
            
            prop_assert_eq!(category1, category2);
            prop_assert_eq!(category2, category3);
        }
        
        /// Property: Network errors are categorized as Network
        #[test]
        fn property_network_error_categorization(msg in any::<String>()) {
            let handler = ErrorHandler::new("test-service");
            let error = ApiError::Network(msg);
            
            let category = handler.categorize(&error);
            prop_assert_eq!(category, ErrorCategory::Network);
        }
        
        /// Property: Authentication errors are categorized as Authentication
        #[test]
        fn property_auth_error_categorization(
            error in prop_oneof![
                any::<String>().prop_map(ApiError::Authentication),
                any::<String>().prop_map(ApiError::ApiKeyNotFound),
                any::<String>().prop_map(ApiError::ApiKeyInactive),
                any::<String>().prop_map(ApiError::ApiKeyExpired),
            ]
        ) {
            let handler = ErrorHandler::new("test-service");
            let category = handler.categorize(&error);
            prop_assert_eq!(category, ErrorCategory::Authentication);
        }
        
        /// Property: Rate limit errors are categorized as RateLimit
        #[test]
        fn property_rate_limit_error_categorization(api_name in any::<String>()) {
            let handler = ErrorHandler::new("test-service");
            let error = ApiError::RateLimitExceeded(api_name);
            
            let category = handler.categorize(&error);
            prop_assert_eq!(category, ErrorCategory::RateLimit);
        }
        
        /// Property: Validation errors are categorized as Validation
        #[test]
        fn property_validation_error_categorization(
            error in prop_oneof![
                any::<String>().prop_map(ApiError::Validation),
                (any::<String>(), any::<String>()).prop_map(|(api, msg)| ApiError::InvalidResponse(api, msg)),
            ]
        ) {
            let handler = ErrorHandler::new("test-service");
            let category = handler.categorize(&error);
            prop_assert_eq!(category, ErrorCategory::Validation);
        }
        
        /// Property: Timeout errors are categorized as Timeout
        #[test]
        fn property_timeout_error_categorization() {
            let handler = ErrorHandler::new("test-service");
            let error = ApiError::Timeout;
            
            let category = handler.categorize(&error);
            prop_assert_eq!(category, ErrorCategory::Timeout);
        }
        
        /// Property: Retryable errors have retry_after duration
        #[test]
        fn property_retryable_errors_have_retry_after(
            error in prop_oneof![
                any::<String>().prop_map(ApiError::Network),
                Just(ApiError::Timeout),
                Just(ApiError::AllApisFailed),
                any::<String>().prop_map(ApiError::RateLimitExceeded),
            ]
        ) {
            let handler = ErrorHandler::new("test-service");
            let response = handler.create_response(&error, None, None);
            
            // Retryable errors should have retry_after
            if handler.is_retryable(&error) || matches!(error, ApiError::RateLimitExceeded(_)) {
                prop_assert!(response.retry_after.is_some());
            }
        }
        
        /// Property: Non-retryable errors don't have retry_after
        #[test]
        fn property_non_retryable_errors_no_retry_after(
            error in prop_oneof![
                any::<String>().prop_map(ApiError::Authentication),
                any::<String>().prop_map(ApiError::Validation),
            ]
        ) {
            let handler = ErrorHandler::new("test-service");
            let response = handler.create_response(&error, None, None);
            
            // Non-retryable errors should not have retry_after
            prop_assert!(response.retry_after.is_none());
        }
    }
}

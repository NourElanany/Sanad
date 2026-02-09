//! API Key Manager
//! 
//! Provides secure management of API keys with support for:
//! - Loading keys from environment variables
//! - Loading keys from secrets manager (optional)
//! - Multiple key types (Header, QueryParam, Bearer, Basic)
//! - Key validation and expiration checking
//! - Hot-reloading of keys without service restart

use super::{ApiError, ApiKey, ApiKeyType};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

/// API Key Manager for secure key storage and injection
#[derive(Debug, Clone)]
pub struct ApiKeyManager {
    keys: Arc<RwLock<HashMap<String, ApiKey>>>,
    secrets_client: Option<Arc<dyn SecretsClient>>,
}

/// Trait for secrets manager clients (AWS Secrets Manager, HashiCorp Vault, etc.)
pub trait SecretsClient: Send + Sync + std::fmt::Debug {
    /// Load API keys from the secrets manager
    fn load_secrets(&self) -> Result<HashMap<String, ApiKey>, ApiError>;
}

impl ApiKeyManager {
    /// Create a new API Key Manager
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
            secrets_client: None,
        }
    }

    /// Create a new API Key Manager with a secrets client
    pub fn with_secrets_client(secrets_client: Arc<dyn SecretsClient>) -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
            secrets_client: Some(secrets_client),
        }
    }

    /// Load API keys from environment variables
    /// 
    /// Expected environment variable format:
    /// - `{API_NAME}_API_KEY` - The API key value
    /// - `{API_NAME}_KEY_TYPE` - The key type (header, query, bearer, basic)
    /// - `{API_NAME}_KEY_HEADER` - Header name (if type is header)
    /// - `{API_NAME}_KEY_PARAM` - Query param name (if type is query)
    /// - `{API_NAME}_KEY_USERNAME` - Username (if type is basic)
    /// 
    /// Example:
    /// ```
    /// QURAN_COM_API_KEY=abc123
    /// QURAN_COM_KEY_TYPE=header
    /// QURAN_COM_KEY_HEADER=X-API-Key
    /// ```
    pub fn load_from_env(&mut self) -> Result<(), ApiError> {
        let api_names = vec![
            "QURAN_COM",
            "SUNNAH_COM",
            "ALADHAN",
            "ISLAMIC_FINDER",
            "HUGGING_FACE",
            "OPENAI",
        ];

        for api_name in api_names {
            // Try to load the API key
            let key_var = format!("{}_API_KEY", api_name);
            if let Ok(key_value) = std::env::var(&key_var) {
                // Get key type (default to header)
                let type_var = format!("{}_KEY_TYPE", api_name);
                let key_type_str = std::env::var(&type_var).unwrap_or_else(|_| "header".to_string());

                let key_type = match key_type_str.to_lowercase().as_str() {
                    "header" => {
                        let header_var = format!("{}_KEY_HEADER", api_name);
                        let header_name = std::env::var(&header_var)
                            .unwrap_or_else(|_| "X-API-Key".to_string());
                        ApiKeyType::Header(header_name)
                    }
                    "query" => {
                        let param_var = format!("{}_KEY_PARAM", api_name);
                        let param_name = std::env::var(&param_var)
                            .unwrap_or_else(|_| "api_key".to_string());
                        ApiKeyType::QueryParam(param_name)
                    }
                    "bearer" => ApiKeyType::Bearer,
                    "basic" => {
                        let username_var = format!("{}_KEY_USERNAME", api_name);
                        let username = std::env::var(&username_var)
                            .unwrap_or_else(|_| "api".to_string());
                        ApiKeyType::Basic(username)
                    }
                    _ => ApiKeyType::Header("X-API-Key".to_string()),
                };

                let api_key = ApiKey::new(
                    api_name.to_lowercase().replace('_', "."),
                    key_value,
                    key_type,
                );

                let mut keys = self.keys.write().unwrap();
                keys.insert(api_key.api_name.clone(), api_key);
            }
        }

        Ok(())
    }

    /// Load API keys from secrets manager
    pub async fn load_from_secrets_manager(&mut self) -> Result<(), ApiError> {
        if let Some(client) = &self.secrets_client {
            let secrets = client.load_secrets()?;
            let mut keys = self.keys.write().unwrap();
            for (name, key) in secrets {
                keys.insert(name, key);
            }
        }
        Ok(())
    }

    /// Get an API key by name
    pub fn get_key(&self, api_name: &str) -> Result<ApiKey, ApiError> {
        let keys = self.keys.read().unwrap();
        keys.get(api_name)
            .cloned()
            .ok_or_else(|| ApiError::ApiKeyNotFound(api_name.to_string()))
    }

    /// Add or update an API key
    pub fn set_key(&mut self, api_key: ApiKey) {
        let mut keys = self.keys.write().unwrap();
        keys.insert(api_key.api_name.clone(), api_key);
    }

    /// Remove an API key
    pub fn remove_key(&mut self, api_name: &str) -> Option<ApiKey> {
        let mut keys = self.keys.write().unwrap();
        keys.remove(api_name)
    }

    /// Check if an API key exists
    pub fn has_key(&self, api_name: &str) -> bool {
        let keys = self.keys.read().unwrap();
        keys.contains_key(api_name)
    }

    /// Get all API key names
    pub fn list_keys(&self) -> Vec<String> {
        let keys = self.keys.read().unwrap();
        keys.keys().cloned().collect()
    }

    /// Inject API key into an HTTP request
    /// 
    /// This method modifies the request to include the API key based on the key type:
    /// - Header: Adds the key as a request header
    /// - QueryParam: Adds the key as a query parameter
    /// - Bearer: Adds as Authorization: Bearer <token>
    /// - Basic: Adds as Authorization: Basic <base64(username:key)>
    pub fn inject_key(
        &self,
        api_name: &str,
        request: &mut reqwest::Request,
    ) -> Result<(), ApiError> {
        let key = self.get_key(api_name)?;

        // Validate the key
        if !key.is_active {
            return Err(ApiError::ApiKeyInactive(api_name.to_string()));
        }

        if let Some(expires_at) = key.expires_at {
            if SystemTime::now() > expires_at {
                return Err(ApiError::ApiKeyExpired(api_name.to_string()));
            }
        }

        // Inject based on key type
        match key.key_type {
            ApiKeyType::Header(ref header_name) => {
                let header_name = reqwest::header::HeaderName::from_bytes(header_name.as_bytes())
                    .map_err(|e| {
                        ApiError::Configuration(format!("Invalid header name: {}", e))
                    })?;
                let header_value = reqwest::header::HeaderValue::from_str(&key.key)
                    .map_err(|e| {
                        ApiError::Configuration(format!("Invalid header value: {}", e))
                    })?;
                request.headers_mut().insert(header_name, header_value);
            }
            ApiKeyType::QueryParam(ref param_name) => {
                let url = request.url_mut();
                url.query_pairs_mut().append_pair(param_name, &key.key);
            }
            ApiKeyType::Bearer => {
                let header_value = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", key.key))
                    .map_err(|e| {
                        ApiError::Configuration(format!("Invalid bearer token: {}", e))
                    })?;
                request.headers_mut().insert(reqwest::header::AUTHORIZATION, header_value);
            }
            ApiKeyType::Basic(ref username) => {
                use base64::{Engine as _, engine::general_purpose};
                let credentials = general_purpose::STANDARD.encode(format!("{}:{}", username, key.key));
                let header_value = reqwest::header::HeaderValue::from_str(&format!("Basic {}", credentials))
                    .map_err(|e| {
                        ApiError::Configuration(format!("Invalid basic auth: {}", e))
                    })?;
                request.headers_mut().insert(reqwest::header::AUTHORIZATION, header_value);
            }
        }

        Ok(())
    }

    /// Reload keys from environment variables and secrets manager
    /// This allows hot-reloading of keys without service restart
    pub async fn reload_keys(&mut self) -> Result<(), ApiError> {
        self.load_from_env()?;
        self.load_from_secrets_manager().await?;
        Ok(())
    }
}

impl Default for ApiKeyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_api_key_manager() {
        let manager = ApiKeyManager::new();
        assert_eq!(manager.list_keys().len(), 0);
    }

    #[test]
    fn test_add_and_get_key() {
        let mut manager = ApiKeyManager::new();
        let key = ApiKey::new(
            "test.api".to_string(),
            "secret123".to_string(),
            ApiKeyType::Header("X-API-Key".to_string()),
        );

        manager.set_key(key.clone());
        assert!(manager.has_key("test.api"));

        let retrieved = manager.get_key("test.api").unwrap();
        assert_eq!(retrieved.api_name, "test.api");
        assert_eq!(retrieved.key, "secret123");
    }

    #[test]
    fn test_remove_key() {
        let mut manager = ApiKeyManager::new();
        let key = ApiKey::new(
            "test.api".to_string(),
            "secret123".to_string(),
            ApiKeyType::Bearer,
        );

        manager.set_key(key);
        assert!(manager.has_key("test.api"));

        let removed = manager.remove_key("test.api");
        assert!(removed.is_some());
        assert!(!manager.has_key("test.api"));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let manager = ApiKeyManager::new();
        let result = manager.get_key("nonexistent");
        assert!(result.is_err());
        match result {
            Err(ApiError::ApiKeyNotFound(name)) => assert_eq!(name, "nonexistent"),
            _ => panic!("Expected ApiKeyNotFound error"),
        }
    }

    #[test]
    fn test_list_keys() {
        let mut manager = ApiKeyManager::new();
        
        manager.set_key(ApiKey::new(
            "api1".to_string(),
            "key1".to_string(),
            ApiKeyType::Bearer,
        ));
        manager.set_key(ApiKey::new(
            "api2".to_string(),
            "key2".to_string(),
            ApiKeyType::Bearer,
        ));

        let keys = manager.list_keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"api1".to_string()));
        assert!(keys.contains(&"api2".to_string()));
    }

    #[test]
    fn test_inject_key_inactive() {
        let mut manager = ApiKeyManager::new();
        let mut key = ApiKey::new(
            "test.api".to_string(),
            "secret123".to_string(),
            ApiKeyType::Bearer,
        );
        key.is_active = false;
        manager.set_key(key);

        let client = reqwest::Client::new();
        let mut request = client.get("https://example.com").build().unwrap();

        let result = manager.inject_key("test.api", &mut request);
        assert!(result.is_err());
        match result {
            Err(ApiError::ApiKeyInactive(name)) => assert_eq!(name, "test.api"),
            _ => panic!("Expected ApiKeyInactive error"),
        }
    }

    #[test]
    fn test_inject_key_expired() {
        let mut manager = ApiKeyManager::new();
        let mut key = ApiKey::new(
            "test.api".to_string(),
            "secret123".to_string(),
            ApiKeyType::Bearer,
        );
        // Set expiration to the past
        key.expires_at = Some(SystemTime::UNIX_EPOCH);
        manager.set_key(key);

        let client = reqwest::Client::new();
        let mut request = client.get("https://example.com").build().unwrap();

        let result = manager.inject_key("test.api", &mut request);
        assert!(result.is_err());
        match result {
            Err(ApiError::ApiKeyExpired(name)) => assert_eq!(name, "test.api"),
            _ => panic!("Expected ApiKeyExpired error"),
        }
    }

    // Unit tests for key rotation
    #[test]
    fn test_key_rotation_update_existing() {
        let mut manager = ApiKeyManager::new();
        
        // Add initial key
        let key1 = ApiKey::new(
            "test.api".to_string(),
            "old_key_123".to_string(),
            ApiKeyType::Bearer,
        );
        manager.set_key(key1);

        // Verify initial key
        let retrieved1 = manager.get_key("test.api").unwrap();
        assert_eq!(retrieved1.key, "old_key_123");

        // Rotate key (update with new key)
        let key2 = ApiKey::new(
            "test.api".to_string(),
            "new_key_456".to_string(),
            ApiKeyType::Bearer,
        );
        manager.set_key(key2);

        // Verify new key
        let retrieved2 = manager.get_key("test.api").unwrap();
        assert_eq!(retrieved2.key, "new_key_456");
        assert_ne!(retrieved2.key, "old_key_123");
    }

    #[test]
    fn test_key_rotation_preserves_other_keys() {
        let mut manager = ApiKeyManager::new();
        
        // Add multiple keys
        manager.set_key(ApiKey::new(
            "api1".to_string(),
            "key1".to_string(),
            ApiKeyType::Bearer,
        ));
        manager.set_key(ApiKey::new(
            "api2".to_string(),
            "key2".to_string(),
            ApiKeyType::Bearer,
        ));
        manager.set_key(ApiKey::new(
            "api3".to_string(),
            "key3".to_string(),
            ApiKeyType::Bearer,
        ));

        // Rotate one key
        manager.set_key(ApiKey::new(
            "api2".to_string(),
            "new_key2".to_string(),
            ApiKeyType::Bearer,
        ));

        // Verify rotated key changed
        assert_eq!(manager.get_key("api2").unwrap().key, "new_key2");

        // Verify other keys unchanged
        assert_eq!(manager.get_key("api1").unwrap().key, "key1");
        assert_eq!(manager.get_key("api3").unwrap().key, "key3");
    }

    #[test]
    fn test_key_rotation_with_different_type() {
        let mut manager = ApiKeyManager::new();
        
        // Add key with Bearer type
        manager.set_key(ApiKey::new(
            "test.api".to_string(),
            "key123".to_string(),
            ApiKeyType::Bearer,
        ));

        // Rotate to Header type
        manager.set_key(ApiKey::new(
            "test.api".to_string(),
            "key456".to_string(),
            ApiKeyType::Header("X-API-Key".to_string()),
        ));

        // Verify new key and type
        let retrieved = manager.get_key("test.api").unwrap();
        assert_eq!(retrieved.key, "key456");
        match retrieved.key_type {
            ApiKeyType::Header(ref name) => assert_eq!(name, "X-API-Key"),
            _ => panic!("Expected Header key type"),
        }
    }

    #[test]
    fn test_key_rotation_injection_uses_new_key() {
        let mut manager = ApiKeyManager::new();
        let client = reqwest::Client::new();
        
        // Add initial key
        manager.set_key(ApiKey::new(
            "test.api".to_string(),
            "old_key".to_string(),
            ApiKeyType::Bearer,
        ));

        // Inject old key
        let mut request1 = client.get("https://example.com").build().unwrap();
        manager.inject_key("test.api", &mut request1).unwrap();
        let auth1 = request1.headers().get("Authorization").unwrap().to_str().unwrap();
        assert_eq!(auth1, "Bearer old_key");

        // Rotate key
        manager.set_key(ApiKey::new(
            "test.api".to_string(),
            "new_key".to_string(),
            ApiKeyType::Bearer,
        ));

        // Inject new key
        let mut request2 = client.get("https://example.com").build().unwrap();
        manager.inject_key("test.api", &mut request2).unwrap();
        let auth2 = request2.headers().get("Authorization").unwrap().to_str().unwrap();
        assert_eq!(auth2, "Bearer new_key");
    }

    #[test]
    fn test_key_rotation_deactivate_old_key() {
        let mut manager = ApiKeyManager::new();
        
        // Add active key
        let mut key = ApiKey::new(
            "test.api".to_string(),
            "key123".to_string(),
            ApiKeyType::Bearer,
        );
        manager.set_key(key.clone());

        // Verify key is active
        assert!(manager.get_key("test.api").unwrap().is_active);

        // Deactivate key (simulating rotation by deactivating old key)
        key.is_active = false;
        manager.set_key(key);

        // Verify key is now inactive
        assert!(!manager.get_key("test.api").unwrap().is_active);

        // Verify injection fails for inactive key
        let client = reqwest::Client::new();
        let mut request = client.get("https://example.com").build().unwrap();
        let result = manager.inject_key("test.api", &mut request);
        assert!(result.is_err());
    }

    #[test]
    fn test_reload_keys_from_env() {
        // Set environment variables for a known API name
        std::env::set_var("QURAN_COM_API_KEY", "test_key_123");
        std::env::set_var("QURAN_COM_KEY_TYPE", "bearer");

        // Create manager and load keys
        let mut manager = ApiKeyManager::new();
        manager.load_from_env().unwrap();

        // Verify key was loaded
        assert!(manager.has_key("quran.com"));
        let key = manager.get_key("quran.com").unwrap();
        assert_eq!(key.key, "test_key_123");
        match key.key_type {
            ApiKeyType::Bearer => {},
            _ => panic!("Expected Bearer key type"),
        }

        // Clean up
        std::env::remove_var("QURAN_COM_API_KEY");
        std::env::remove_var("QURAN_COM_KEY_TYPE");
    }

    #[test]
    fn test_load_from_env_with_header_type() {
        // Set environment variables for a known API name
        std::env::set_var("SUNNAH_COM_API_KEY", "custom_key_456");
        std::env::set_var("SUNNAH_COM_KEY_TYPE", "header");
        std::env::set_var("SUNNAH_COM_KEY_HEADER", "X-Custom-Key");

        // Create manager and load keys
        let mut manager = ApiKeyManager::new();
        manager.load_from_env().unwrap();

        // Verify key was loaded
        assert!(manager.has_key("sunnah.com"));
        let key = manager.get_key("sunnah.com").unwrap();
        assert_eq!(key.key, "custom_key_456");
        match key.key_type {
            ApiKeyType::Header(ref name) => assert_eq!(name, "X-Custom-Key"),
            _ => panic!("Expected Header key type"),
        }

        // Clean up
        std::env::remove_var("SUNNAH_COM_API_KEY");
        std::env::remove_var("SUNNAH_COM_KEY_TYPE");
        std::env::remove_var("SUNNAH_COM_KEY_HEADER");
    }

    // Property-based tests
    use proptest::prelude::*;

    // Strategy for generating API key types
    fn api_key_type_strategy() -> impl Strategy<Value = ApiKeyType> {
        prop_oneof![
            Just(ApiKeyType::Header("X-API-Key".to_string())),
            Just(ApiKeyType::Header("Authorization".to_string())),
            Just(ApiKeyType::QueryParam("api_key".to_string())),
            Just(ApiKeyType::QueryParam("key".to_string())),
            Just(ApiKeyType::Bearer),
            Just(ApiKeyType::Basic("api".to_string())),
            Just(ApiKeyType::Basic("user".to_string())),
        ]
    }

    // Strategy for generating valid API names
    fn api_name_strategy() -> impl Strategy<Value = String> {
        "[a-z]{3,10}\\.[a-z]{3,10}".prop_map(|s| s.to_string())
    }

    // Strategy for generating API keys (alphanumeric strings)
    fn api_key_value_strategy() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9]{16,64}".prop_map(|s| s.to_string())
    }

    // Feature: official-apis-integration, Property 11: API Key Injection
    // **Validates: Requirements 8.2**
    //
    // For any API request, the API_Key_Manager should inject the appropriate API key
    // in the correct format (header, query param, or bearer token) based on the API's
    // requirements, and the request should contain the key before being sent.
    proptest! {
        #[test]
        fn property_api_key_injection(
            api_name in api_name_strategy(),
            key_value in api_key_value_strategy(),
            key_type in api_key_type_strategy(),
        ) {
            // Setup
            let mut manager = ApiKeyManager::new();
            let api_key = ApiKey::new(api_name.clone(), key_value.clone(), key_type.clone());
            manager.set_key(api_key);

            // Create a test request
            let client = reqwest::Client::new();
            let mut request = client.get("https://example.com/api/test").build().unwrap();

            // Act: Inject the key
            let result = manager.inject_key(&api_name, &mut request);

            // Assert: Injection should succeed
            prop_assert!(result.is_ok(), "Key injection should succeed");

            // Assert: Key should be present in the request based on type
            match key_type {
                ApiKeyType::Header(header_name) => {
                    let headers = request.headers();
                    prop_assert!(
                        headers.contains_key(&header_name),
                        "Request should contain header: {}",
                        header_name
                    );
                    let header_value = headers.get(&header_name).unwrap().to_str().unwrap();
                    prop_assert_eq!(
                        header_value,
                        &key_value,
                        "Header value should match the API key"
                    );
                }
                ApiKeyType::QueryParam(param_name) => {
                    let url = request.url();
                    let query_pairs: Vec<(String, String)> = url
                        .query_pairs()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    
                    let has_param = query_pairs.iter().any(|(k, v)| k == &param_name && v == &key_value);
                    prop_assert!(
                        has_param,
                        "Request should contain query parameter: {}={}",
                        param_name,
                        key_value
                    );
                }
                ApiKeyType::Bearer => {
                    let headers = request.headers();
                    prop_assert!(
                        headers.contains_key("Authorization"),
                        "Request should contain Authorization header"
                    );
                    let auth_value = headers.get("Authorization").unwrap().to_str().unwrap();
                    let expected = format!("Bearer {}", key_value);
                    prop_assert_eq!(
                        auth_value,
                        &expected,
                        "Authorization header should be Bearer token"
                    );
                }
                ApiKeyType::Basic(username) => {
                    use base64::{Engine as _, engine::general_purpose};
                    let headers = request.headers();
                    prop_assert!(
                        headers.contains_key("Authorization"),
                        "Request should contain Authorization header"
                    );
                    let auth_value = headers.get("Authorization").unwrap().to_str().unwrap();
                    let credentials = general_purpose::STANDARD.encode(format!("{}:{}", username, key_value));
                    let expected = format!("Basic {}", credentials);
                    prop_assert_eq!(
                        auth_value,
                        &expected,
                        "Authorization header should be Basic auth"
                    );
                }
            }
        }
    }

    // Additional property: Key injection should fail for inactive keys
    proptest! {
        #[test]
        fn property_inactive_key_injection_fails(
            api_name in api_name_strategy(),
            key_value in api_key_value_strategy(),
            key_type in api_key_type_strategy(),
        ) {
            // Setup
            let mut manager = ApiKeyManager::new();
            let mut api_key = ApiKey::new(api_name.clone(), key_value.clone(), key_type.clone());
            api_key.is_active = false;  // Mark as inactive
            manager.set_key(api_key);

            // Create a test request
            let client = reqwest::Client::new();
            let mut request = client.get("https://example.com/api/test").build().unwrap();

            // Act: Try to inject the key
            let result = manager.inject_key(&api_name, &mut request);

            // Assert: Injection should fail
            prop_assert!(result.is_err(), "Inactive key injection should fail");
        }
    }

    // Additional property: Key injection should fail for expired keys
    proptest! {
        #[test]
        fn property_expired_key_injection_fails(
            api_name in api_name_strategy(),
            key_value in api_key_value_strategy(),
            key_type in api_key_type_strategy(),
        ) {
            // Setup
            let mut manager = ApiKeyManager::new();
            let mut api_key = ApiKey::new(api_name.clone(), key_value.clone(), key_type.clone());
            api_key.expires_at = Some(SystemTime::UNIX_EPOCH);  // Expired
            manager.set_key(api_key);

            // Create a test request
            let client = reqwest::Client::new();
            let mut request = client.get("https://example.com/api/test").build().unwrap();

            // Act: Try to inject the key
            let result = manager.inject_key(&api_name, &mut request);

            // Assert: Injection should fail
            prop_assert!(result.is_err(), "Expired key injection should fail");
        }
    }

    // Additional property: Key injection should fail for non-existent keys
    proptest! {
        #[test]
        fn property_nonexistent_key_injection_fails(
            api_name in api_name_strategy(),
        ) {
            // Setup
            let manager = ApiKeyManager::new();  // Empty manager

            // Create a test request
            let client = reqwest::Client::new();
            let mut request = client.get("https://example.com/api/test").build().unwrap();

            // Act: Try to inject a non-existent key
            let result = manager.inject_key(&api_name, &mut request);

            // Assert: Injection should fail
            prop_assert!(result.is_err(), "Non-existent key injection should fail");
        }
    }

    // Feature: official-apis-integration, Property 12: API Key Confidentiality
    // **Validates: Requirements 8.4**
    //
    // For any log entry or error message, it should not contain actual API key values
    // (only masked versions like "key_***"), ensuring keys are never exposed in logs.
    proptest! {
        #[test]
        fn property_api_key_confidentiality(
            api_name in api_name_strategy(),
            key_value in api_key_value_strategy(),
            key_type in api_key_type_strategy(),
        ) {
            // Setup
            let api_key = ApiKey::new(api_name.clone(), key_value.clone(), key_type.clone());

            // Act: Get the string representation (used in logs)
            let key_string = format!("{}", api_key);
            let debug_string = format!("{:?}", api_key);
            let masked = api_key.masked_key();

            // Assert: The full key value should NOT appear in string representations
            prop_assert!(
                !key_string.contains(&key_value),
                "String representation should not contain full key value"
            );
            prop_assert!(
                !debug_string.contains(&key_value),
                "Debug representation should not contain full key value"
            );

            // Assert: The masked version should contain asterisks
            prop_assert!(
                masked.contains("***"),
                "Masked key should contain asterisks"
            );

            // Assert: For keys longer than 8 chars, masked version should show first and last 4
            if key_value.len() > 8 {
                let first_4 = &key_value[..4];
                let last_4 = &key_value[key_value.len()-4..];
                prop_assert!(
                    masked.contains(first_4),
                    "Masked key should contain first 4 characters"
                );
                prop_assert!(
                    masked.contains(last_4),
                    "Masked key should contain last 4 characters"
                );
            }
        }
    }

    // Additional property: Error messages should not expose API keys
    proptest! {
        #[test]
        fn property_error_messages_dont_expose_keys(
            api_name in api_name_strategy(),
            key_value in api_key_value_strategy(),
        ) {
            // Setup
            let mut manager = ApiKeyManager::new();
            let mut api_key = ApiKey::new(
                api_name.clone(),
                key_value.clone(),
                ApiKeyType::Bearer
            );
            api_key.is_active = false;
            manager.set_key(api_key);

            // Create a test request
            let client = reqwest::Client::new();
            let mut request = client.get("https://example.com/api/test").build().unwrap();

            // Act: Try to inject the inactive key
            let result = manager.inject_key(&api_name, &mut request);

            // Assert: Error message should not contain the actual key value
            if let Err(e) = result {
                let error_msg = format!("{}", e);
                let error_debug = format!("{:?}", e);
                prop_assert!(
                    !error_msg.contains(&key_value),
                    "Error message should not contain full key value"
                );
                prop_assert!(
                    !error_debug.contains(&key_value),
                    "Error debug should not contain full key value"
                );
            }
        }
    }
}


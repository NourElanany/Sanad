//! Shared API Client Traits and Implementations
//! 
//! This module provides a unified interface for all external API integrations
//! with support for fallback mechanisms, rate limiting, and health monitoring.

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod api_key_manager;
pub mod cache_manager;
pub mod calendar;
pub mod error;
pub mod hadith;
pub mod prayer;
pub mod quran;
pub mod rate_limiter;
pub mod tafsir;
pub mod traits;

// Re-export main types
pub use api_key_manager::{ApiKeyManager, SecretsClient};
pub use cache_manager::{CacheCategory, CacheManager, CacheStats, CacheStrategy};
pub use error::ApiError;
pub use rate_limiter::{RateLimiter, RateLimitUsage};
pub use traits::*;

/// Rate limit configuration for an API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub requests_per_day: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            requests_per_hour: 1000,
            requests_per_day: 10000,
        }
    }
}

/// API key type for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiKeyType {
    /// Header-based authentication (e.g., "X-API-Key")
    Header(String),
    /// Query parameter authentication (e.g., "api_key")
    QueryParam(String),
    /// Bearer token authentication
    Bearer,
    /// Basic authentication with username
    Basic(String),
}

/// API key information
#[derive(Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub api_name: String,
    pub key: String,
    pub key_type: ApiKeyType,
    pub created_at: std::time::SystemTime,
    pub expires_at: Option<std::time::SystemTime>,
    pub is_active: bool,
}

impl std::fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiKey")
            .field("api_name", &self.api_name)
            .field("key", &self.masked_key())
            .field("key_type", &self.key_type)
            .field("created_at", &self.created_at)
            .field("expires_at", &self.expires_at)
            .field("is_active", &self.is_active)
            .finish()
    }
}

impl ApiKey {
    /// Create a new API key
    pub fn new(api_name: String, key: String, key_type: ApiKeyType) -> Self {
        Self {
            api_name,
            key,
            key_type,
            created_at: std::time::SystemTime::now(),
            expires_at: None,
            is_active: true,
        }
    }

    /// Check if the key is valid (active and not expired)
    pub fn is_valid(&self) -> bool {
        if !self.is_active {
            return false;
        }

        if let Some(expires_at) = self.expires_at {
            if std::time::SystemTime::now() > expires_at {
                return false;
            }
        }

        true
    }

    /// Get a masked version of the key for logging
    /// Shows only the first 4 and last 4 characters
    pub fn masked_key(&self) -> String {
        if self.key.len() <= 8 {
            "***".to_string()
        } else {
            format!("{}***{}", &self.key[..4], &self.key[self.key.len()-4..])
        }
    }
}

impl std::fmt::Display for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ApiKey(api_name={}, key={}, active={})",
            self.api_name,
            self.masked_key(),
            self.is_active
        )
    }
}

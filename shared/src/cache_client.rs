use crate::{SanadError, SanadResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, error, warn};

/// Client for interacting with the cache service
#[derive(Clone)]
pub struct CacheClient {
    client: Client,
    base_url: String,
}

/// Request to set a cache value
#[derive(Debug, Serialize)]
pub struct SetCacheRequest {
    pub key: String,
    pub value: serde_json::Value,
    pub cache_type: Option<String>,
    pub ttl_seconds: Option<u64>,
}

/// Request to get multiple cache values
#[derive(Debug, Serialize)]
pub struct GetMultipleRequest {
    pub keys: Vec<String>,
}

/// Response for multiple cache values
#[derive(Debug, Default, Deserialize)]
pub struct GetMultipleResponse {
    pub values: HashMap<String, Option<serde_json::Value>>,
}

/// Request to invalidate cache by pattern
#[derive(Debug, Serialize)]
pub struct InvalidatePatternRequest {
    pub pattern: String,
}

/// Response for cache invalidation
#[derive(Debug, Default, Deserialize)]
pub struct InvalidateResponse {
    pub deleted_count: u64,
}

/// Cache statistics
#[derive(Debug, Deserialize)]
pub struct CacheStats {
    pub redis_memory_usage_bytes: u64,
    pub memory_cache_entries: usize,
    pub memory_cache_entries_by_type: HashMap<String, usize>,
    pub total_cache_operations: u64,
}

impl CacheClient {
    /// Create a new cache client
    pub fn new(cache_service_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: cache_service_url.trim_end_matches('/').to_string(),
        }
    }

    /// Get a value from cache
    pub async fn get<T>(&self, key: &str) -> SanadResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/cache/{}", self.base_url, key);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<Option<serde_json::Value>> = response
                .json()
                .await
                .map_err(SanadError::HttpClient)?;

            if let Some(value) = api_response.data.flatten() {
                let typed_value: T = serde_json::from_value(value)
                    .map_err(SanadError::Serialization)?;
                debug!("Cache hit for key: {}", key);
                Ok(Some(typed_value))
            } else {
                debug!("Cache miss for key: {}", key);
                Ok(None)
            }
        } else if response.status() == 404 {
            debug!("Cache miss for key: {}", key);
            Ok(None)
        } else {
            error!("Cache service error for key {}: {}", key, response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Set a value in cache
    pub async fn set<T>(&self, key: &str, value: &T, cache_type: Option<&str>, ttl_seconds: Option<u64>) -> SanadResult<()>
    where
        T: Serialize,
    {
        let url = format!("{}/cache/{}", self.base_url, key);
        let json_value = serde_json::to_value(value).map_err(SanadError::Serialization)?;
        
        let request = SetCacheRequest {
            key: key.to_string(),
            value: json_value,
            cache_type: cache_type.map(|s| s.to_string()),
            ttl_seconds,
        };

        let response = self.client
            .put(&url)
            .json(&request)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            debug!("Successfully cached value for key: {}", key);
            Ok(())
        } else {
            error!("Failed to cache value for key {}: {}", key, response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Delete a value from cache
    pub async fn delete(&self, key: &str) -> SanadResult<()> {
        let url = format!("{}/cache/{}", self.base_url, key);
        
        let response = self.client
            .delete(&url)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            debug!("Successfully deleted cache value for key: {}", key);
            Ok(())
        } else {
            error!("Failed to delete cache value for key {}: {}", key, response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Get multiple values from cache
    pub async fn get_multiple(&self, keys: Vec<String>) -> SanadResult<HashMap<String, Option<serde_json::Value>>> {
        let url = format!("{}/cache/multi", self.base_url);
        let request = GetMultipleRequest { keys };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<GetMultipleResponse> = response
                .json()
                .await
                .map_err(SanadError::HttpClient)?;

            Ok(api_response.data.unwrap_or_default().values)
        } else {
            error!("Failed to get multiple cache values: {}", response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Delete multiple values from cache
    pub async fn delete_multiple(&self, keys: Vec<String>) -> SanadResult<()> {
        let url = format!("{}/cache/multi", self.base_url);
        let request = GetMultipleRequest { keys };

        let response = self.client
            .delete(&url)
            .json(&request)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            debug!("Successfully deleted multiple cache values");
            Ok(())
        } else {
            error!("Failed to delete multiple cache values: {}", response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Invalidate cache entries by pattern
    pub async fn invalidate_pattern(&self, pattern: &str) -> SanadResult<u64> {
        let url = format!("{}/cache/invalidate/pattern", self.base_url);
        let request = InvalidatePatternRequest {
            pattern: pattern.to_string(),
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<InvalidateResponse> = response
                .json()
                .await
                .map_err(SanadError::HttpClient)?;

            let deleted_count = api_response.data.unwrap_or_default().deleted_count;
            debug!("Invalidated {} cache entries for pattern: {}", deleted_count, pattern);
            Ok(deleted_count)
        } else {
            error!("Failed to invalidate cache pattern {}: {}", pattern, response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Invalidate prayer times cache for a location
    pub async fn invalidate_prayer_times(&self, lat: f64, lng: f64) -> SanadResult<u64> {
        let url = format!("{}/cache/invalidate/prayer-times/{}/{}", self.base_url, lat, lng);

        let response = self.client
            .delete(&url)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<InvalidateResponse> = response
                .json()
                .await
                .map_err(SanadError::HttpClient)?;

            let deleted_count = api_response.data.unwrap_or_default().deleted_count;
            debug!("Invalidated {} prayer times cache entries for location: {}, {}", deleted_count, lat, lng);
            Ok(deleted_count)
        } else {
            error!("Failed to invalidate prayer times cache for {}, {}: {}", lat, lng, response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Invalidate all semantic query cache entries
    pub async fn invalidate_semantic_queries(&self) -> SanadResult<u64> {
        let url = format!("{}/cache/invalidate/semantic-queries", self.base_url);

        let response = self.client
            .delete(&url)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<InvalidateResponse> = response
                .json()
                .await
                .map_err(SanadError::HttpClient)?;

            let deleted_count = api_response.data.unwrap_or_default().deleted_count;
            debug!("Invalidated {} semantic query cache entries", deleted_count);
            Ok(deleted_count)
        } else {
            error!("Failed to invalidate semantic queries cache: {}", response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Invalidate Quran surah cache
    pub async fn invalidate_quran_surah(&self, surah: u16) -> SanadResult<u64> {
        let url = format!("{}/cache/invalidate/quran/{}", self.base_url, surah);

        let response = self.client
            .delete(&url)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<InvalidateResponse> = response
                .json()
                .await
                .map_err(SanadError::HttpClient)?;

            let deleted_count = api_response.data.unwrap_or_default().deleted_count;
            debug!("Invalidated {} Quran surah {} cache entries", deleted_count, surah);
            Ok(deleted_count)
        } else {
            error!("Failed to invalidate Quran surah {} cache: {}", surah, response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Invalidate hadith collection cache
    pub async fn invalidate_hadith_collection(&self, collection: &str) -> SanadResult<u64> {
        let url = format!("{}/cache/invalidate/hadith/{}", self.base_url, collection);

        let response = self.client
            .delete(&url)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<InvalidateResponse> = response
                .json()
                .await
                .map_err(SanadError::HttpClient)?;

            let deleted_count = api_response.data.unwrap_or_default().deleted_count;
            debug!("Invalidated {} hadith collection {} cache entries", deleted_count, collection);
            Ok(deleted_count)
        } else {
            error!("Failed to invalidate hadith collection {} cache: {}", collection, response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Get cache statistics
    pub async fn get_stats(&self) -> SanadResult<CacheStats> {
        let url = format!("{}/cache/stats", self.base_url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<CacheStats> = response
                .json()
                .await
                .map_err(SanadError::HttpClient)?;

            Ok(api_response.data.unwrap_or_default())
        } else {
            error!("Failed to get cache stats: {}", response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Trigger cache cleanup
    pub async fn cleanup(&self) -> SanadResult<usize> {
        let url = format!("{}/cache/cleanup", self.base_url);

        let response = self.client
            .post(&url)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<usize> = response
                .json()
                .await
                .map_err(SanadError::HttpClient)?;

            let cleaned_count = api_response.data.unwrap_or_default();
            debug!("Cleaned up {} expired cache entries", cleaned_count);
            Ok(cleaned_count)
        } else {
            error!("Failed to cleanup cache: {}", response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Trigger cache warmup
    pub async fn warmup(&self) -> SanadResult<()> {
        let url = format!("{}/cache/warmup", self.base_url);

        let response = self.client
            .post(&url)
            .send()
            .await
            .map_err(SanadError::HttpClient)?;

        if response.status().is_success() {
            debug!("Successfully triggered cache warmup");
            Ok(())
        } else {
            error!("Failed to warmup cache: {}", response.status());
            Err(SanadError::ExternalApi {
                service: "cache-service".to_string(),
                message: format!("HTTP {}", response.status()),
            })
        }
    }

    /// Check if cache service is healthy
    pub async fn health_check(&self) -> SanadResult<bool> {
        let url = format!("{}/health", self.base_url);

        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => {
                warn!("Cache service health check failed: {}", e);
                Ok(false)
            }
        }
    }
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            redis_memory_usage_bytes: 0,
            memory_cache_entries: 0,
            memory_cache_entries_by_type: HashMap::new(),
            total_cache_operations: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_client_creation() {
        let client = CacheClient::new("http://localhost:8091");
        assert_eq!(client.base_url, "http://localhost:8091");
    }

    #[test]
    fn test_cache_client_url_trimming() {
        let client = CacheClient::new("http://localhost:8091/");
        assert_eq!(client.base_url, "http://localhost:8091");
    }
}
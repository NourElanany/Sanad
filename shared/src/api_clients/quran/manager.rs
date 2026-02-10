//! Quran API Manager
//!
//! Manages multiple Quran API clients with fallback logic, caching, and rate limiting.

use crate::api_clients::{
    ApiError, AyahData, CacheManager, PageData, QuranApiClient, RateLimiter,
    SurahData,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Quran API Manager
/// 
/// Coordinates multiple Quran API clients with:
/// - Priority-based fallback
/// - Intelligent caching
/// - Rate limiting
/// - Health monitoring
pub struct QuranApiManager {
    clients: Vec<Box<dyn QuranApiClient + Send + Sync>>,
    cache: Arc<CacheManager>,
    rate_limiter: Arc<RateLimiter>,
}

impl QuranApiManager {
    /// Create a new Quran API Manager
    pub fn new(
        clients: Vec<Box<dyn QuranApiClient + Send + Sync>>,
        cache: Arc<CacheManager>,
        rate_limiter: Arc<RateLimiter>,
    ) -> Self {
        // Sort clients by priority (lower number = higher priority)
        let mut sorted_clients = clients;
        sorted_clients.sort_by_key(|c| c.priority());

        Self {
            clients: sorted_clients,
            cache,
            rate_limiter,
        }
    }

    /// Get a surah with fallback logic
    pub async fn get_surah(&self, surah_number: u8) -> Result<SurahData, ApiError> {
        let cache_key = format!("quran:surah:{}", surah_number);

        // 1. Check cache first
        if let Ok(Some(cached)) = self.cache.get::<SurahData>(&cache_key).await {
            tracing::debug!("Cache hit for surah {}", surah_number);
            return Ok(cached);
        }

        // 2. Try each API client in priority order
        let mut last_error = None;
        for client in &self.clients {
            // Check if API is healthy
            if !client.is_healthy().await {
                tracing::warn!("API {} is unhealthy, skipping", client.api_name());
                continue;
            }

            // Check rate limit
            if !self
                .rate_limiter
                .check_and_increment(client.api_name())
                .await
                .unwrap_or(false)
            {
                tracing::warn!("Rate limit exceeded for {}, trying next API", client.api_name());
                continue;
            }

            // Try to fetch from this API
            match client.get_surah(surah_number).await {
                Ok(surah) => {
                    tracing::info!("Successfully fetched surah {} from {}", surah_number, client.api_name());
                    
                    // Cache the result (30 days TTL for static Quran text)
                    if let Err(e) = self
                        .cache
                        .set(&cache_key, &surah, Duration::from_secs(30 * 24 * 60 * 60))
                        .await
                    {
                        tracing::warn!("Failed to cache surah: {}", e);
                    }
                    
                    return Ok(surah);
                }
                Err(e) => {
                    tracing::warn!("API {} failed for surah {}: {}", client.api_name(), surah_number, e);
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // 3. All APIs failed, try expired cache as last resort
        if let Ok(Some(cached)) = self.cache.get_stale::<SurahData>(&cache_key).await {
            tracing::warn!("All APIs failed, serving expired cache for surah {}", surah_number);
            return Ok(cached);
        }

        // 4. Everything failed
        Err(last_error.unwrap_or(ApiError::AllApisFailed))
    }

    /// Get an ayah with fallback logic
    pub async fn get_ayah(&self, surah: u8, ayah: u16) -> Result<AyahData, ApiError> {
        let cache_key = format!("quran:ayah:{}:{}", surah, ayah);

        // 1. Check cache first
        if let Ok(Some(cached)) = self.cache.get::<AyahData>(&cache_key).await {
            tracing::debug!("Cache hit for ayah {}:{}", surah, ayah);
            return Ok(cached);
        }

        // 2. Try each API client in priority order
        let mut last_error = None;
        for client in &self.clients {
            // Check if API is healthy
            if !client.is_healthy().await {
                tracing::warn!("API {} is unhealthy, skipping", client.api_name());
                continue;
            }

            // Check rate limit
            if !self
                .rate_limiter
                .check_and_increment(client.api_name())
                .await
                .unwrap_or(false)
            {
                tracing::warn!("Rate limit exceeded for {}, trying next API", client.api_name());
                continue;
            }

            // Try to fetch from this API
            match client.get_ayah(surah, ayah).await {
                Ok(ayah_data) => {
                    tracing::info!("Successfully fetched ayah {}:{} from {}", surah, ayah, client.api_name());
                    
                    // Cache the result (30 days TTL for static Quran text)
                    if let Err(e) = self
                        .cache
                        .set(&cache_key, &ayah_data, Duration::from_secs(30 * 24 * 60 * 60))
                        .await
                    {
                        tracing::warn!("Failed to cache ayah: {}", e);
                    }
                    
                    return Ok(ayah_data);
                }
                Err(e) => {
                    tracing::warn!("API {} failed for ayah {}:{}: {}", client.api_name(), surah, ayah, e);
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // 3. All APIs failed, try expired cache as last resort
        if let Ok(Some(cached)) = self.cache.get_stale::<AyahData>(&cache_key).await {
            tracing::warn!("All APIs failed, serving expired cache for ayah {}:{}", surah, ayah);
            return Ok(cached);
        }

        // 4. Everything failed
        Err(last_error.unwrap_or(ApiError::AllApisFailed))
    }

    /// Get a page with fallback logic
    pub async fn get_page(&self, page: u16) -> Result<PageData, ApiError> {
        let cache_key = format!("quran:page:{}", page);

        // 1. Check cache first
        if let Ok(Some(cached)) = self.cache.get::<PageData>(&cache_key).await {
            tracing::debug!("Cache hit for page {}", page);
            return Ok(cached);
        }

        // 2. Try each API client in priority order
        let mut last_error = None;
        for client in &self.clients {
            // Check if API is healthy
            if !client.is_healthy().await {
                tracing::warn!("API {} is unhealthy, skipping", client.api_name());
                continue;
            }

            // Check rate limit
            if !self
                .rate_limiter
                .check_and_increment(client.api_name())
                .await
                .unwrap_or(false)
            {
                tracing::warn!("Rate limit exceeded for {}, trying next API", client.api_name());
                continue;
            }

            // Try to fetch from this API
            match client.get_page(page).await {
                Ok(page_data) => {
                    tracing::info!("Successfully fetched page {} from {}", page, client.api_name());
                    
                    // Cache the result (30 days TTL for static Quran text)
                    if let Err(e) = self
                        .cache
                        .set(&cache_key, &page_data, Duration::from_secs(30 * 24 * 60 * 60))
                        .await
                    {
                        tracing::warn!("Failed to cache page: {}", e);
                    }
                    
                    return Ok(page_data);
                }
                Err(e) => {
                    tracing::warn!("API {} failed for page {}: {}", client.api_name(), page, e);
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // 3. All APIs failed, try expired cache as last resort
        if let Ok(Some(cached)) = self.cache.get_stale::<PageData>(&cache_key).await {
            tracing::warn!("All APIs failed, serving expired cache for page {}", page);
            return Ok(cached);
        }

        // 4. Everything failed
        Err(last_error.unwrap_or(ApiError::AllApisFailed))
    }

    /// Get the number of configured clients
    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    /// Get the names of all configured clients in priority order
    pub fn client_names(&self) -> Vec<String> {
        self.clients.iter().map(|c| c.api_name().to_string()).collect()
    }

    /// Clear cache for a specific key (for testing)
    #[cfg(test)]
    pub async fn clear_cache(&self, key: &str) -> Result<(), ApiError> {
        self.cache.delete(key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_clients::quran::{AlquranCloudClient, QuranComClient, TanzilClient};
    use crate::api_clients::{CacheCategory, CacheStrategy};

    async fn create_test_manager() -> QuranApiManager {
        // Create mock Redis client for testing
        let cache = Arc::new(CacheManager::new("redis://127.0.0.1:6379/")
            .await
            .expect("Failed to create cache manager"));
        
        let rate_limiter = Arc::new(RateLimiter::new(
            "redis://127.0.0.1:6379/",
            HashMap::new()
        )
            .await
            .expect("Failed to create rate limiter"));

        let clients: Vec<Box<dyn QuranApiClient + Send + Sync>> = vec![
            Box::new(QuranComClient::new(None)),
            Box::new(AlquranCloudClient::new()),
            Box::new(TanzilClient::new()),
        ];

        QuranApiManager::new(clients, cache, rate_limiter)
    }

    #[test]
    fn test_manager_creation() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let cache = Arc::new(CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"));
            
            let rate_limiter = Arc::new(RateLimiter::new(
                "redis://127.0.0.1:6379/",
                HashMap::new()
            )
                .await
                .expect("Failed to create rate limiter"));

            let clients: Vec<Box<dyn QuranApiClient + Send + Sync>> = vec![
                Box::new(QuranComClient::new(None)),
                Box::new(AlquranCloudClient::new()),
            ];

            let manager = QuranApiManager::new(clients, cache, rate_limiter);
            assert_eq!(manager.client_count(), 2);
        });
    }

    #[test]
    fn test_clients_sorted_by_priority() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let cache = Arc::new(CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"));
            
            let rate_limiter = Arc::new(RateLimiter::new(
                "redis://127.0.0.1:6379/",
                HashMap::new()
            )
                .await
                .expect("Failed to create rate limiter"));

            // Add clients in reverse priority order
            let clients: Vec<Box<dyn QuranApiClient + Send + Sync>> = vec![
                Box::new(TanzilClient::new()),        // Priority 3
                Box::new(AlquranCloudClient::new()),  // Priority 2
                Box::new(QuranComClient::new(None)),  // Priority 1
            ];

            let manager = QuranApiManager::new(clients, cache, rate_limiter);
            let names = manager.client_names();
            
            // Should be sorted by priority: quran.com (1), alquran.cloud (2), tanzil.net (3)
            assert_eq!(names[0], "quran.com");
            assert_eq!(names[1], "alquran.cloud");
            assert_eq!(names[2], "tanzil.net");
        });
    }

    #[tokio::test]
    async fn test_invalid_surah_number() {
        let manager = create_test_manager().await;
        
        // Test surah number 0
        let result = manager.get_surah(0).await;
        assert!(result.is_err());
        
        // Test surah number 115
        let result = manager.get_surah(115).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_ayah_number() {
        let manager = create_test_manager().await;
        
        // Test ayah number 0
        let result = manager.get_ayah(1, 0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_page_number() {
        let manager = create_test_manager().await;
        
        // Test page number 0
        let result = manager.get_page(0).await;
        assert!(result.is_err());
        
        // Test page number 605
        let result = manager.get_page(605).await;
        assert!(result.is_err());
    }
}

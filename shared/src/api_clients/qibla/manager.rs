// Qibla API Manager
//
// Manages multiple Qibla API clients with fallback support

use crate::api_clients::{
    ApiClient, ApiError, CacheCategory, CacheManager, QiblaApiClient, QiblaResponse, RateLimiter,
};
use std::sync::Arc;
use std::time::Duration;

use super::{AladhanQiblaClient, IslamicFinderQiblaClient};

/// Manager for Qibla API clients
///
/// Handles fallback between multiple Qibla APIs and local calculation
pub struct QiblaApiManager {
    clients: Vec<Box<dyn QiblaApiClient + Send + Sync>>,
    cache: Arc<CacheManager>,
    rate_limiter: Arc<RateLimiter>,
}

impl QiblaApiManager {
    /// Create a new Qibla API manager
    pub fn new(cache: Arc<CacheManager>, rate_limiter: Arc<RateLimiter>) -> Self {
        let clients: Vec<Box<dyn QiblaApiClient + Send + Sync>> = vec![
            Box::new(AladhanQiblaClient::new()),
            Box::new(IslamicFinderQiblaClient::new()), // Local calculation fallback
        ];

        Self {
            clients,
            cache,
            rate_limiter,
        }
    }

    /// Create a manager with custom clients (for testing)
    pub fn with_clients(
        clients: Vec<Box<dyn QiblaApiClient + Send + Sync>>,
        cache: Arc<CacheManager>,
        rate_limiter: Arc<RateLimiter>,
    ) -> Self {
        Self {
            clients,
            cache,
            rate_limiter,
        }
    }

    /// Generate cache key for Qibla request
    pub fn cache_key(latitude: f64, longitude: f64) -> String {
        // Round to 4 decimal places (~11 meters precision) for cache key
        format!("qibla:{}:{}", 
            (latitude * 10000.0).round() / 10000.0,
            (longitude * 10000.0).round() / 10000.0
        )
    }

    /// Get Qibla direction for a location
    ///
    /// This method:
    /// 1. Checks cache first
    /// 2. Tries primary API (Aladhan)
    /// 3. Falls back to local calculation if API fails
    /// 4. Caches successful responses
    pub async fn get_direction(&self, latitude: f64, longitude: f64) -> Result<QiblaResponse, ApiError> {
        let cache_key = Self::cache_key(latitude, longitude);

        // 1. Check cache
        if let Ok(Some(cached)) = self.cache.get::<QiblaResponse>(&cache_key).await {
            log::debug!("Qibla cache hit for location ({}, {})", latitude, longitude);
            return Ok(cached);
        }

        log::debug!("Qibla cache miss for location ({}, {})", latitude, longitude);

        // 2. Try each client in priority order
        let mut last_error = None;
        
        for client in &self.clients {
            // Check if client is healthy
            if !client.is_healthy().await {
                log::warn!("Qibla API {} is unhealthy, skipping", client.api_name());
                continue;
            }

            // Check rate limit
            match self.rate_limiter.check_and_increment(client.api_name()).await {
                Ok(allowed) => {
                    if !allowed {
                        log::warn!("Rate limit exceeded for Qibla API {}", client.api_name());
                        continue;
                    }
                }
                Err(e) => {
                    log::error!("Rate limiter error for {}: {}", client.api_name(), e);
                    continue;
                }
            }

            // Try to get Qibla direction
            match client.get_direction(latitude, longitude).await {
                Ok(response) => {
                    log::info!(
                        "Successfully got Qibla direction from {} for location ({}, {}): {} degrees",
                        client.api_name(),
                        latitude,
                        longitude,
                        response.direction
                    );

                    // Cache the response
                    let ttl = self
                        .cache
                        .get_strategy(CacheCategory::Qibla)
                        .map(|s| s.ttl)
                        .unwrap_or(Duration::from_secs(86400)); // Default 24 hours
                    
                    if let Err(e) = self.cache.set(&cache_key, &response, ttl).await {
                        log::warn!("Failed to cache Qibla response: {}", e);
                    }

                    return Ok(response);
                }
                Err(e) => {
                    log::warn!(
                        "Qibla API {} failed for location ({}, {}): {}",
                        client.api_name(),
                        latitude,
                        longitude,
                        e
                    );
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // 3. All clients failed, try expired cache
        if let Ok(Some(cached)) = self.cache.get_expired::<QiblaResponse>(&cache_key).await {
            log::warn!("Serving expired cache for Qibla request at ({}, {})", latitude, longitude);
            return Ok(cached);
        }

        // 4. Everything failed
        Err(last_error.unwrap_or(ApiError::AllApisFailed))
    }

    /// Get all available Qibla API clients
    pub fn get_clients(&self) -> &[Box<dyn QiblaApiClient + Send + Sync>] {
        &self.clients
    }

    /// Check health of all Qibla APIs
    pub async fn health_check(&self) -> Vec<(String, bool)> {
        let mut results = Vec::new();
        for client in &self.clients {
            let is_healthy = client.is_healthy().await;
            results.push((client.api_name().to_string(), is_healthy));
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_test_manager() -> Result<QiblaApiManager, ApiError> {
        let cache = Arc::new(CacheManager::new("redis://localhost:6379").await?);
        let rate_limiter = Arc::new(RateLimiter::new("redis://localhost:6379", Default::default()).await?);
        Ok(QiblaApiManager::new(cache, rate_limiter))
    }

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = create_test_manager().await.unwrap();
        assert_eq!(manager.clients.len(), 2);
    }

    #[test]
    fn test_cache_key_generation() {
        let key1 = QiblaApiManager::cache_key(40.7128, -74.0060);
        let key2 = QiblaApiManager::cache_key(40.7128, -74.0060);
        assert_eq!(key1, key2);

        // Test rounding
        let key3 = QiblaApiManager::cache_key(40.71281234, -74.00601234);
        let key4 = QiblaApiManager::cache_key(40.71284567, -74.00604567);
        assert_eq!(key3, key4); // Should be same after rounding to 4 decimal places
    }

    #[tokio::test]
    async fn test_get_direction() {
        let manager = create_test_manager().await.unwrap();

        // Test getting Qibla direction
        let result = manager.get_direction(40.7128, -74.0060).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.direction >= 0.0 && response.direction <= 360.0);
        assert!(response.distance_km > 0.0);
        assert!(!response.source.is_empty());
    }

    #[tokio::test]
    async fn test_caching() {
        let manager = create_test_manager();

        // First request - should hit API
        let result1 = manager.get_direction(51.5074, -0.1278).await;
        assert!(result1.is_ok());

        // Second request - should hit cache
        let result2 = manager.get_direction(51.5074, -0.1278).await;
        assert!(result2.is_ok());

        // Results should be identical
        let response1 = result1.unwrap();
        let response2 = result2.unwrap();
        assert_eq!(response1.direction, response2.direction);
        assert_eq!(response1.distance_km, response2.distance_km);
    }

    #[tokio::test]
    async fn test_fallback_to_local_calculation() {
        let manager = create_test_manager();

        // Even if primary API fails, local calculation should work
        let result = manager.get_direction(35.6762, 139.6503).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.direction >= 0.0 && response.direction <= 360.0);
    }

    #[tokio::test]
    async fn test_invalid_coordinates() {
        let manager = create_test_manager();

        // Invalid latitude
        let result = manager.get_direction(91.0, 0.0).await;
        assert!(result.is_err());

        // Invalid longitude
        let result = manager.get_direction(0.0, 181.0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_health_check() {
        let manager = create_test_manager();
        let health = manager.health_check().await;

        assert_eq!(health.len(), 2);
        
        // Local calculation should always be healthy
        let local_health = health.iter().find(|(name, _)| name.contains("islamic_finder"));
        assert!(local_health.is_some());
        assert!(local_health.unwrap().1);
    }

    #[tokio::test]
    async fn test_multiple_locations() {
        let manager = create_test_manager();

        let test_locations = vec![
            (40.7128, -74.0060),  // New York
            (51.5074, -0.1278),   // London
            (35.6762, 139.6503),  // Tokyo
            (-33.8688, 151.2093), // Sydney
        ];

        for (lat, lon) in test_locations {
            let result = manager.get_direction(lat, lon).await;
            assert!(result.is_ok(), "Failed for location ({}, {})", lat, lon);

            let response = result.unwrap();
            assert!(
                response.direction >= 0.0 && response.direction <= 360.0,
                "Invalid direction {} for location ({}, {})",
                response.direction,
                lat,
                lon
            );
            assert!(response.distance_km > 0.0);
        }
    }
}

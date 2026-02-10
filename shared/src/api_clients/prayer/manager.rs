//! Prayer Times API Manager
//!
//! Manages multiple Prayer Times API clients with fallback logic, caching, and rate limiting.

use crate::api_clients::{
    ApiError, CacheManager, PrayerTimesApiClient, PrayerTimesRequest, PrayerTimesResponse,
    RateLimiter,
};
use std::sync::Arc;
use std::time::Duration;

/// Prayer Times API Manager
/// 
/// Coordinates multiple Prayer Times API clients with:
/// - Priority-based fallback
/// - Intelligent caching (daily TTL)
/// - Rate limiting
/// - Health monitoring
/// - Local calculation as last resort
pub struct PrayerTimesApiManager {
    clients: Vec<Box<dyn PrayerTimesApiClient + Send + Sync>>,
    cache: Arc<CacheManager>,
    rate_limiter: Arc<RateLimiter>,
}

impl PrayerTimesApiManager {
    /// Create a new Prayer Times API Manager
    pub fn new(
        clients: Vec<Box<dyn PrayerTimesApiClient + Send + Sync>>,
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

    /// Get prayer times with fallback logic
    pub async fn get_times(&self, request: &PrayerTimesRequest) -> Result<PrayerTimesResponse, ApiError> {
        let cache_key = format!(
            "prayer:{}:{}:{}:{:?}:{:?}",
            request.latitude,
            request.longitude,
            request.date,
            request.calculation_method,
            request.madhab
        );

        // 1. Check cache first
        if let Ok(Some(cached)) = self.cache.get::<PrayerTimesResponse>(&cache_key).await {
            tracing::debug!("Cache hit for prayer times on {}", request.date);
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
            match client.get_times(request).await {
                Ok(times) => {
                    tracing::info!(
                        "Successfully fetched prayer times for {} from {}",
                        request.date,
                        client.api_name()
                    );

                    // Validate chronological ordering
                    if let Err(e) = Self::validate_chronological_order(&times) {
                        tracing::warn!("Prayer times validation failed: {}", e);
                        last_error = Some(e);
                        continue;
                    }
                    
                    // Cache the result (1 day TTL for prayer times)
                    if let Err(e) = self
                        .cache
                        .set(&cache_key, &times, Duration::from_secs(24 * 60 * 60))
                        .await
                    {
                        tracing::warn!("Failed to cache prayer times: {}", e);
                    }
                    
                    return Ok(times);
                }
                Err(e) => {
                    tracing::warn!(
                        "API {} failed for prayer times on {}: {}",
                        client.api_name(),
                        request.date,
                        e
                    );
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // 3. All APIs failed, try expired cache as last resort
        if let Ok(Some(cached)) = self.cache.get_stale::<PrayerTimesResponse>(&cache_key).await {
            tracing::warn!("All APIs failed, serving expired cache for prayer times on {}", request.date);
            return Ok(cached);
        }

        // 4. Try local calculation as absolute last resort
        match Self::calculate_locally(request) {
            Ok(times) => {
                tracing::warn!("All APIs and cache failed, using local calculation for {}", request.date);
                return Ok(times);
            }
            Err(e) => {
                tracing::error!("Local calculation also failed: {}", e);
            }
        }

        // 5. Everything failed
        Err(last_error.unwrap_or(ApiError::AllApisFailed))
    }

    /// Get prayer times for a date range
    pub async fn get_times_range(
        &self,
        request: &PrayerTimesRequest,
        days: u32,
    ) -> Result<Vec<PrayerTimesResponse>, ApiError> {
        let mut results = Vec::new();
        let mut current_date = request.date;

        for _ in 0..days {
            let mut day_request = request.clone();
            day_request.date = current_date;

            match self.get_times(&day_request).await {
                Ok(times) => results.push(times),
                Err(e) => {
                    tracing::warn!("Failed to get prayer times for {}: {}", current_date, e);
                    // Continue with next day instead of failing completely
                }
            }

            current_date = current_date
                .succ_opt()
                .ok_or_else(|| ApiError::Validation("Date overflow".to_string()))?;
        }

        if results.is_empty() {
            return Err(ApiError::AllApisFailed);
        }

        Ok(results)
    }

    /// Validate that prayer times are in chronological order
    /// Property 7: Prayer Times Chronological Ordering
    fn validate_chronological_order(times: &PrayerTimesResponse) -> Result<(), ApiError> {
        // Fajr < Sunrise < Dhuhr < Asr < Maghrib < Isha
        if times.fajr >= times.sunrise {
            return Err(ApiError::Validation(format!(
                "Fajr ({}) must be before Sunrise ({})",
                times.fajr, times.sunrise
            )));
        }
        if times.sunrise >= times.dhuhr {
            return Err(ApiError::Validation(format!(
                "Sunrise ({}) must be before Dhuhr ({})",
                times.sunrise, times.dhuhr
            )));
        }
        if times.dhuhr >= times.asr {
            return Err(ApiError::Validation(format!(
                "Dhuhr ({}) must be before Asr ({})",
                times.dhuhr, times.asr
            )));
        }
        if times.asr >= times.maghrib {
            return Err(ApiError::Validation(format!(
                "Asr ({}) must be before Maghrib ({})",
                times.asr, times.maghrib
            )));
        }
        if times.maghrib >= times.isha {
            return Err(ApiError::Validation(format!(
                "Maghrib ({}) must be before Isha ({})",
                times.maghrib, times.isha
            )));
        }

        Ok(())
    }

    /// Calculate prayer times locally using astronomical formulas
    /// This is a placeholder for local calculation
    /// In production, you would use a library like salah-rs or implement
    /// the full astronomical calculations
    fn calculate_locally(_request: &PrayerTimesRequest) -> Result<PrayerTimesResponse, ApiError> {
        // This is a placeholder for local calculation
        // In production, you would use a library like salah-rs or implement
        // the full astronomical calculations
        
        // For now, return an error to indicate local calculation is not yet implemented
        Err(ApiError::NotImplemented(
            "Local prayer times calculation not yet implemented".to_string()
        ))
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
    use crate::api_clients::prayer::{AladhanPrayerClient, IslamicFinderPrayerClient};
    use crate::api_clients::{CalculationMethod, Madhab};
    use chrono::NaiveDate;
    use std::collections::HashMap;

    async fn create_test_manager() -> PrayerTimesApiManager {
        let cache = Arc::new(CacheManager::new("redis://127.0.0.1:6379/")
            .await
            .expect("Failed to create cache manager"));
        
        let rate_limiter = Arc::new(RateLimiter::new(
            "redis://127.0.0.1:6379/",
            HashMap::new()
        )
            .await
            .expect("Failed to create rate limiter"));

        let clients: Vec<Box<dyn PrayerTimesApiClient + Send + Sync>> = vec![
            Box::new(AladhanPrayerClient::new()),
            Box::new(IslamicFinderPrayerClient::new()),
        ];

        PrayerTimesApiManager::new(clients, cache, rate_limiter)
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

            let clients: Vec<Box<dyn PrayerTimesApiClient + Send + Sync>> = vec![
                Box::new(AladhanPrayerClient::new()),
                Box::new(IslamicFinderPrayerClient::new()),
            ];

            let manager = PrayerTimesApiManager::new(clients, cache, rate_limiter);
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
            let clients: Vec<Box<dyn PrayerTimesApiClient + Send + Sync>> = vec![
                Box::new(IslamicFinderPrayerClient::new()),  // Priority 2
                Box::new(AladhanPrayerClient::new()),        // Priority 1
            ];

            let manager = PrayerTimesApiManager::new(clients, cache, rate_limiter);
            let names = manager.client_names();
            
            // Should be sorted by priority: aladhan (1), islamic_finder (2)
            assert_eq!(names[0], "aladhan");
            assert_eq!(names[1], "islamic_finder");
        });
    }

    #[test]
    fn test_chronological_validation_valid() {
        use chrono::NaiveTime;

        let times = PrayerTimesResponse {
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            fajr: NaiveTime::from_hms_opt(5, 30, 0).unwrap(),
            sunrise: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
            dhuhr: NaiveTime::from_hms_opt(12, 30, 0).unwrap(),
            asr: NaiveTime::from_hms_opt(15, 30, 0).unwrap(),
            maghrib: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            isha: NaiveTime::from_hms_opt(19, 30, 0).unwrap(),
            source: "test".to_string(),
        };

        assert!(PrayerTimesApiManager::validate_chronological_order(&times).is_ok());
    }

    #[test]
    fn test_chronological_validation_invalid() {
        use chrono::NaiveTime;

        // Fajr after Sunrise - invalid
        let times = PrayerTimesResponse {
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            fajr: NaiveTime::from_hms_opt(7, 30, 0).unwrap(),
            sunrise: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
            dhuhr: NaiveTime::from_hms_opt(12, 30, 0).unwrap(),
            asr: NaiveTime::from_hms_opt(15, 30, 0).unwrap(),
            maghrib: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            isha: NaiveTime::from_hms_opt(19, 30, 0).unwrap(),
            source: "test".to_string(),
        };

        assert!(PrayerTimesApiManager::validate_chronological_order(&times).is_err());
    }

    #[tokio::test]
    async fn test_invalid_request() {
        let manager = create_test_manager().await;
        
        // Invalid coordinates
        let request = PrayerTimesRequest {
            latitude: 91.0,
            longitude: 0.0,
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            calculation_method: CalculationMethod::MWL,
            madhab: Madhab::Shafi,
        };
        
        let result = manager.get_times(&request).await;
        assert!(result.is_err());
    }
}

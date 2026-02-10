//! Calendar API Manager
//!
//! Manages multiple Calendar API clients with fallback logic, caching, and rate limiting.

use crate::api_clients::{
    ApiError, CacheManager, CalendarApiClient, HijriDate, IslamicEvent, RateLimiter,
};
use chrono::NaiveDate;
use std::sync::Arc;
use std::time::Duration;

/// Calendar API Manager
/// 
/// Coordinates multiple Calendar API clients with:
/// - Priority-based fallback
/// - Intelligent caching (weekly TTL for calendar data)
/// - Rate limiting
/// - Health monitoring
pub struct CalendarApiManager {
    clients: Vec<Box<dyn CalendarApiClient + Send + Sync>>,
    cache: Arc<CacheManager>,
    rate_limiter: Arc<RateLimiter>,
}

impl CalendarApiManager {
    /// Create a new Calendar API Manager
    pub fn new(
        clients: Vec<Box<dyn CalendarApiClient + Send + Sync>>,
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

    /// Convert Gregorian date to Hijri with fallback logic
    pub async fn gregorian_to_hijri(&self, date: NaiveDate) -> Result<HijriDate, ApiError> {
        let cache_key = format!("calendar:g2h:{}", date);

        // 1. Check cache first
        if let Ok(Some(cached)) = self.cache.get::<HijriDate>(&cache_key).await {
            tracing::debug!("Cache hit for Gregorian to Hijri conversion: {}", date);
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
            match client.gregorian_to_hijri(date).await {
                Ok(hijri) => {
                    tracing::info!(
                        "Successfully converted {} to Hijri from {}",
                        date,
                        client.api_name()
                    );
                    
                    // Cache the result (7 days TTL for calendar conversions)
                    if let Err(e) = self
                        .cache
                        .set(&cache_key, &hijri, Duration::from_secs(7 * 24 * 60 * 60))
                        .await
                    {
                        tracing::warn!("Failed to cache Hijri date: {}", e);
                    }
                    
                    return Ok(hijri);
                }
                Err(e) => {
                    tracing::warn!(
                        "API {} failed for Gregorian to Hijri conversion: {}",
                        client.api_name(),
                        e
                    );
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // 3. All APIs failed, try expired cache as last resort
        if let Ok(Some(cached)) = self.cache.get_stale::<HijriDate>(&cache_key).await {
            tracing::warn!("All APIs failed, serving expired cache for Gregorian to Hijri: {}", date);
            return Ok(cached);
        }

        // 4. Everything failed
        Err(last_error.unwrap_or(ApiError::AllApisFailed))
    }

    /// Convert Hijri date to Gregorian with fallback logic
    pub async fn hijri_to_gregorian(&self, hijri: &HijriDate) -> Result<NaiveDate, ApiError> {
        let cache_key = format!("calendar:h2g:{}:{}:{}", hijri.year, hijri.month, hijri.day);

        // 1. Check cache first
        if let Ok(Some(cached)) = self.cache.get::<NaiveDate>(&cache_key).await {
            tracing::debug!("Cache hit for Hijri to Gregorian conversion: {}/{}/{}", 
                hijri.year, hijri.month, hijri.day);
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
            match client.hijri_to_gregorian(hijri).await {
                Ok(gregorian) => {
                    tracing::info!(
                        "Successfully converted Hijri {}/{}/{} to Gregorian from {}",
                        hijri.year, hijri.month, hijri.day,
                        client.api_name()
                    );
                    
                    // Cache the result (7 days TTL for calendar conversions)
                    if let Err(e) = self
                        .cache
                        .set(&cache_key, &gregorian, Duration::from_secs(7 * 24 * 60 * 60))
                        .await
                    {
                        tracing::warn!("Failed to cache Gregorian date: {}", e);
                    }
                    
                    return Ok(gregorian);
                }
                Err(e) => {
                    tracing::warn!(
                        "API {} failed for Hijri to Gregorian conversion: {}",
                        client.api_name(),
                        e
                    );
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // 3. All APIs failed, try expired cache as last resort
        if let Ok(Some(cached)) = self.cache.get_stale::<NaiveDate>(&cache_key).await {
            tracing::warn!("All APIs failed, serving expired cache for Hijri to Gregorian: {}/{}/{}", 
                hijri.year, hijri.month, hijri.day);
            return Ok(cached);
        }

        // 4. Everything failed
        Err(last_error.unwrap_or(ApiError::AllApisFailed))
    }

    /// Get Islamic events for a date range with fallback logic
    pub async fn get_events(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<IslamicEvent>, ApiError> {
        let cache_key = format!("calendar:events:{}:{}", start, end);

        // 1. Check cache first
        if let Ok(Some(cached)) = self.cache.get::<Vec<IslamicEvent>>(&cache_key).await {
            tracing::debug!("Cache hit for Islamic events: {} to {}", start, end);
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
            match client.get_events(start, end).await {
                Ok(events) => {
                    tracing::info!(
                        "Successfully fetched {} Islamic events from {} to {} from {}",
                        events.len(),
                        start,
                        end,
                        client.api_name()
                    );
                    
                    // Cache the result (7 days TTL for events)
                    if let Err(e) = self
                        .cache
                        .set(&cache_key, &events, Duration::from_secs(7 * 24 * 60 * 60))
                        .await
                    {
                        tracing::warn!("Failed to cache Islamic events: {}", e);
                    }
                    
                    return Ok(events);
                }
                Err(e) => {
                    tracing::warn!(
                        "API {} failed for Islamic events: {}",
                        client.api_name(),
                        e
                    );
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // 3. All APIs failed, try expired cache as last resort
        if let Ok(Some(cached)) = self.cache.get_stale::<Vec<IslamicEvent>>(&cache_key).await {
            tracing::warn!("All APIs failed, serving expired cache for Islamic events: {} to {}", start, end);
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
    use crate::api_clients::calendar::{AladhanCalendarClient, IslamicFinderCalendarClient};
    use std::collections::HashMap;

    async fn create_test_manager() -> CalendarApiManager {
        let cache = Arc::new(CacheManager::new("redis://127.0.0.1:6379/")
            .await
            .expect("Failed to create cache manager"));
        
        let rate_limiter = Arc::new(RateLimiter::new(
            "redis://127.0.0.1:6379/",
            HashMap::new()
        )
            .await
            .expect("Failed to create rate limiter"));

        let clients: Vec<Box<dyn CalendarApiClient + Send + Sync>> = vec![
            Box::new(AladhanCalendarClient::new()),
            Box::new(IslamicFinderCalendarClient::new()),
        ];

        CalendarApiManager::new(clients, cache, rate_limiter)
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

            let clients: Vec<Box<dyn CalendarApiClient + Send + Sync>> = vec![
                Box::new(AladhanCalendarClient::new()),
                Box::new(IslamicFinderCalendarClient::new()),
            ];

            let manager = CalendarApiManager::new(clients, cache, rate_limiter);
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
            let clients: Vec<Box<dyn CalendarApiClient + Send + Sync>> = vec![
                Box::new(IslamicFinderCalendarClient::new()),  // Priority 2
                Box::new(AladhanCalendarClient::new()),        // Priority 1
            ];

            let manager = CalendarApiManager::new(clients, cache, rate_limiter);
            let names = manager.client_names();
            
            // Should be sorted by priority: aladhan_calendar (1), islamic_finder_calendar (2)
            assert_eq!(names[0], "aladhan_calendar");
            assert_eq!(names[1], "islamic_finder_calendar");
        });
    }

    #[tokio::test]
    async fn test_invalid_date_range() {
        let manager = create_test_manager().await;
        
        let start = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        
        let result = manager.get_events(start, end).await;
        assert!(result.is_err());
    }
}

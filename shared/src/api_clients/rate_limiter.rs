//! Rate Limiter for API clients
//! 
//! Implements rate limiting using Redis to track request counts across
//! multiple time windows (minute, hour, day) for each API.

use super::{ApiError, RateLimitConfig};
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, warn, error};

/// Rate limiter that enforces API rate limits using Redis
#[derive(Clone)]
pub struct RateLimiter {
    /// Redis connection for distributed rate limiting
    redis: Arc<RwLock<MultiplexedConnection>>,
    /// Rate limit configurations per API
    configs: Arc<HashMap<String, RateLimitConfig>>,
}

impl RateLimiter {
    /// Create a new rate limiter with Redis backend
    pub async fn new(
        redis_url: &str,
        configs: HashMap<String, RateLimitConfig>,
    ) -> Result<Self, ApiError> {
        let client = Client::open(redis_url)
            .map_err(|e| ApiError::Configuration(format!("Failed to connect to Redis: {}", e)))?;
        
        let redis = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| ApiError::Configuration(format!("Failed to get Redis connection: {}", e)))?;

        Ok(Self {
            redis: Arc::new(RwLock::new(redis)),
            configs: Arc::new(configs),
        })
    }

    /// Check if a request is allowed for the given API
    /// Returns true if the request is within rate limits, false otherwise
    pub async fn check(&self, api_name: &str) -> Result<bool, ApiError> {
        let config = self.configs.get(api_name)
            .ok_or_else(|| ApiError::UnknownApi(api_name.to_string()))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Calculate time window boundaries
        let minute = now / 60;
        let hour = now / 3600;
        let day = now / 86400;

        let minute_key = format!("ratelimit:{}:minute:{}", api_name, minute);
        let hour_key = format!("ratelimit:{}:hour:{}", api_name, hour);
        let day_key = format!("ratelimit:{}:day:{}", api_name, day);

        // Get current counts from Redis
        let mut conn = self.redis.write().await;
        
        let minute_count: u32 = conn.get(&minute_key).await.unwrap_or(0);
        let hour_count: u32 = conn.get(&hour_key).await.unwrap_or(0);
        let day_count: u32 = conn.get(&day_key).await.unwrap_or(0);

        // Check all time windows
        if minute_count >= config.requests_per_minute {
            debug!(
                "Rate limit exceeded for {} (minute): {}/{}",
                api_name, minute_count, config.requests_per_minute
            );
            return Ok(false);
        }
        
        if hour_count >= config.requests_per_hour {
            debug!(
                "Rate limit exceeded for {} (hour): {}/{}",
                api_name, hour_count, config.requests_per_hour
            );
            return Ok(false);
        }
        
        if day_count >= config.requests_per_day {
            debug!(
                "Rate limit exceeded for {} (day): {}/{}",
                api_name, day_count, config.requests_per_day
            );
            return Ok(false);
        }

        // Log warnings when approaching limits (80% threshold)
        if minute_count as f64 >= config.requests_per_minute as f64 * 0.8 {
            warn!(
                "Approaching rate limit for {} (minute): {}/{}",
                api_name, minute_count, config.requests_per_minute
            );
        }
        
        if hour_count as f64 >= config.requests_per_hour as f64 * 0.8 {
            warn!(
                "Approaching rate limit for {} (hour): {}/{}",
                api_name, hour_count, config.requests_per_hour
            );
        }
        
        if day_count as f64 >= config.requests_per_day as f64 * 0.8 {
            warn!(
                "Approaching rate limit for {} (day): {}/{}",
                api_name, day_count, config.requests_per_day
            );
        }

        Ok(true)
    }

    /// Increment the request counter for the given API
    /// Should be called after a successful rate limit check
    pub async fn increment(&self, api_name: &str) -> Result<(), ApiError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Calculate time window boundaries
        let minute = now / 60;
        let hour = now / 3600;
        let day = now / 86400;

        let minute_key = format!("ratelimit:{}:minute:{}", api_name, minute);
        let hour_key = format!("ratelimit:{}:hour:{}", api_name, hour);
        let day_key = format!("ratelimit:{}:day:{}", api_name, day);

        let mut conn = self.redis.write().await;

        // Increment all counters with appropriate TTL
        // Minute counter expires after 2 minutes (to handle edge cases)
        let _: () = conn.incr(&minute_key, 1).await
            .map_err(|e| ApiError::CacheError(format!("Failed to increment minute counter: {}", e)))?;
        let _: () = conn.expire(&minute_key, 120).await
            .map_err(|e| ApiError::CacheError(format!("Failed to set minute TTL: {}", e)))?;

        // Hour counter expires after 2 hours
        let _: () = conn.incr(&hour_key, 1).await
            .map_err(|e| ApiError::CacheError(format!("Failed to increment hour counter: {}", e)))?;
        let _: () = conn.expire(&hour_key, 7200).await
            .map_err(|e| ApiError::CacheError(format!("Failed to set hour TTL: {}", e)))?;

        // Day counter expires after 2 days
        let _: () = conn.incr(&day_key, 1).await
            .map_err(|e| ApiError::CacheError(format!("Failed to increment day counter: {}", e)))?;
        let _: () = conn.expire(&day_key, 172800).await
            .map_err(|e| ApiError::CacheError(format!("Failed to set day TTL: {}", e)))?;

        debug!("Incremented rate limit counters for {}", api_name);

        Ok(())
    }

    /// Check and increment in a single operation (atomic)
    /// Returns true if the request was allowed and counter was incremented
    pub async fn check_and_increment(&self, api_name: &str) -> Result<bool, ApiError> {
        let allowed = self.check(api_name).await?;
        
        if allowed {
            self.increment(api_name).await?;
        } else {
            // Log rate limit exceeded
            error!(
                "Rate limit exceeded for API: {}. Request denied.",
                api_name
            );
        }
        
        Ok(allowed)
    }

    /// Check and increment, or return an error if rate limit is exceeded
    /// This is useful when you want to fail fast rather than silently deny
    pub async fn check_and_increment_or_error(&self, api_name: &str) -> Result<(), ApiError> {
        let allowed = self.check(api_name).await?;
        
        if !allowed {
            let usage = self.get_usage(api_name).await?;
            return Err(ApiError::RateLimitExceeded(format!(
                "{} (minute: {}/{}, hour: {}/{}, day: {}/{})",
                api_name,
                usage.minute_count, usage.minute_limit,
                usage.hour_count, usage.hour_limit,
                usage.day_count, usage.day_limit
            )));
        }
        
        self.increment(api_name).await?;
        Ok(())
    }

    /// Get time until rate limit resets for the most restrictive window
    /// Returns None if no limits are currently exceeded
    pub async fn time_until_reset(&self, api_name: &str) -> Result<Option<Duration>, ApiError> {
        let config = self.configs.get(api_name)
            .ok_or_else(|| ApiError::UnknownApi(api_name.to_string()))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let minute = now / 60;
        let hour = now / 3600;
        let day = now / 86400;

        let minute_key = format!("ratelimit:{}:minute:{}", api_name, minute);
        let hour_key = format!("ratelimit:{}:hour:{}", api_name, hour);
        let day_key = format!("ratelimit:{}:day:{}", api_name, day);

        let mut conn = self.redis.write().await;
        
        let minute_count: u32 = conn.get(&minute_key).await.unwrap_or(0);
        let hour_count: u32 = conn.get(&hour_key).await.unwrap_or(0);
        let day_count: u32 = conn.get(&day_key).await.unwrap_or(0);

        // Check which limit is exceeded and calculate time until reset
        if minute_count >= config.requests_per_minute {
            let next_minute = (minute + 1) * 60;
            let seconds_until_reset = next_minute - now;
            return Ok(Some(Duration::from_secs(seconds_until_reset)));
        }
        
        if hour_count >= config.requests_per_hour {
            let next_hour = (hour + 1) * 3600;
            let seconds_until_reset = next_hour - now;
            return Ok(Some(Duration::from_secs(seconds_until_reset)));
        }
        
        if day_count >= config.requests_per_day {
            let next_day = (day + 1) * 86400;
            let seconds_until_reset = next_day - now;
            return Ok(Some(Duration::from_secs(seconds_until_reset)));
        }

        Ok(None)
    }

    /// Wait until rate limit resets, then proceed
    /// This implements a simple queuing mechanism
    pub async fn wait_for_capacity(&self, api_name: &str) -> Result<(), ApiError> {
        loop {
            let allowed = self.check(api_name).await?;
            
            if allowed {
                self.increment(api_name).await?;
                return Ok(());
            }
            
            // Get time until reset
            if let Some(wait_time) = self.time_until_reset(api_name).await? {
                warn!(
                    "Rate limit exceeded for {}. Waiting {:?} until reset...",
                    api_name, wait_time
                );
                
                // Wait until reset (with a small buffer)
                tokio::time::sleep(wait_time + Duration::from_secs(1)).await;
            } else {
                // This shouldn't happen, but if it does, wait a bit and retry
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }

    /// Check if we're approaching the rate limit (within threshold percentage)
    /// Returns true if usage is >= threshold (e.g., 0.8 for 80%)
    pub async fn is_approaching_limit(&self, api_name: &str, threshold: f64) -> Result<bool, ApiError> {
        let usage = self.get_usage(api_name).await?;
        let percentage = usage.max_usage_percentage();
        Ok(percentage >= threshold * 100.0)
    }

    /// Get current usage statistics for an API
    pub async fn get_usage(&self, api_name: &str) -> Result<RateLimitUsage, ApiError> {
        let config = self.configs.get(api_name)
            .ok_or_else(|| ApiError::UnknownApi(api_name.to_string()))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let minute = now / 60;
        let hour = now / 3600;
        let day = now / 86400;

        let minute_key = format!("ratelimit:{}:minute:{}", api_name, minute);
        let hour_key = format!("ratelimit:{}:hour:{}", api_name, hour);
        let day_key = format!("ratelimit:{}:day:{}", api_name, day);

        let mut conn = self.redis.write().await;
        
        let minute_count: u32 = conn.get(&minute_key).await.unwrap_or(0);
        let hour_count: u32 = conn.get(&hour_key).await.unwrap_or(0);
        let day_count: u32 = conn.get(&day_key).await.unwrap_or(0);

        Ok(RateLimitUsage {
            api_name: api_name.to_string(),
            minute_count,
            minute_limit: config.requests_per_minute,
            hour_count,
            hour_limit: config.requests_per_hour,
            day_count,
            day_limit: config.requests_per_day,
        })
    }

    /// Reset rate limit counters for an API (useful for testing)
    pub async fn reset(&self, api_name: &str) -> Result<(), ApiError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let minute = now / 60;
        let hour = now / 3600;
        let day = now / 86400;

        let minute_key = format!("ratelimit:{}:minute:{}", api_name, minute);
        let hour_key = format!("ratelimit:{}:hour:{}", api_name, hour);
        let day_key = format!("ratelimit:{}:day:{}", api_name, day);

        let mut conn = self.redis.write().await;
        
        let _: () = conn.del(&minute_key).await
            .map_err(|e| ApiError::CacheError(format!("Failed to delete minute counter: {}", e)))?;
        let _: () = conn.del(&hour_key).await
            .map_err(|e| ApiError::CacheError(format!("Failed to delete hour counter: {}", e)))?;
        let _: () = conn.del(&day_key).await
            .map_err(|e| ApiError::CacheError(format!("Failed to delete day counter: {}", e)))?;

        debug!("Reset rate limit counters for {}", api_name);

        Ok(())
    }
}

/// Rate limit usage statistics
#[derive(Debug, Clone)]
pub struct RateLimitUsage {
    pub api_name: String,
    pub minute_count: u32,
    pub minute_limit: u32,
    pub hour_count: u32,
    pub hour_limit: u32,
    pub day_count: u32,
    pub day_limit: u32,
}

impl RateLimitUsage {
    /// Check if any limit is exceeded
    pub fn is_exceeded(&self) -> bool {
        self.minute_count >= self.minute_limit
            || self.hour_count >= self.hour_limit
            || self.day_count >= self.day_limit
    }

    /// Get the percentage of limit used (highest across all windows)
    pub fn max_usage_percentage(&self) -> f64 {
        let minute_pct = (self.minute_count as f64 / self.minute_limit as f64) * 100.0;
        let hour_pct = (self.hour_count as f64 / self.hour_limit as f64) * 100.0;
        let day_pct = (self.day_count as f64 / self.day_limit as f64) * 100.0;

        minute_pct.max(hour_pct).max(day_pct)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_usage_is_exceeded() {
        let usage = RateLimitUsage {
            api_name: "test_api".to_string(),
            minute_count: 50,
            minute_limit: 60,
            hour_count: 900,
            hour_limit: 1000,
            day_count: 10000,
            day_limit: 10000,
        };

        assert!(usage.is_exceeded()); // day limit reached
    }

    #[test]
    fn test_rate_limit_usage_not_exceeded() {
        let usage = RateLimitUsage {
            api_name: "test_api".to_string(),
            minute_count: 50,
            minute_limit: 60,
            hour_count: 900,
            hour_limit: 1000,
            day_count: 9000,
            day_limit: 10000,
        };

        assert!(!usage.is_exceeded());
    }

    #[test]
    fn test_max_usage_percentage() {
        let usage = RateLimitUsage {
            api_name: "test_api".to_string(),
            minute_count: 30,
            minute_limit: 60,  // 50%
            hour_count: 800,
            hour_limit: 1000,  // 80%
            day_count: 7000,
            day_limit: 10000,  // 70%
        };

        assert_eq!(usage.max_usage_percentage(), 80.0);
    }
}

// Include property-based tests
#[cfg(test)]
#[path = "rate_limiter_property_tests.rs"]
mod property_tests;

// Include unit tests
#[cfg(test)]
#[path = "rate_limiter_tests.rs"]
mod unit_tests;

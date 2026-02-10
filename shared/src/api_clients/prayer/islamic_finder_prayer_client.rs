//! Islamic Finder Prayer Times API Client
//!
//! Widely trusted Islamic resource for prayer times calculations.
//! API Documentation: https://www.islamicfinder.org/

use crate::api_clients::{
    ApiClient, ApiError, CalculationMethod, Madhab, PrayerTimesApiClient, PrayerTimesRequest,
    PrayerTimesResponse, RateLimitConfig,
};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveTime, Timelike};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

/// Islamic Finder Prayer Times API client
#[derive(Debug, Clone)]
pub struct IslamicFinderPrayerClient {
    base_url: String,
    client: Client,
}

impl IslamicFinderPrayerClient {
    /// Create a new Islamic Finder Prayer Times API client
    pub fn new() -> Self {
        Self {
            base_url: "https://api.islamicfinder.org/v1".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Create a client with custom base URL (for testing)
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Convert CalculationMethod to Islamic Finder method number
    fn method_to_number(method: CalculationMethod) -> u8 {
        match method {
            CalculationMethod::MWL => 3,      // Muslim World League
            CalculationMethod::ISNA => 2,     // Islamic Society of North America
            CalculationMethod::Egypt => 5,    // Egyptian General Authority of Survey
            CalculationMethod::Makkah => 4,   // Umm Al-Qura University, Makkah
            CalculationMethod::Karachi => 1,  // University of Islamic Sciences, Karachi
            CalculationMethod::Tehran => 7,   // Institute of Geophysics, University of Tehran
            CalculationMethod::Jafari => 0,   // Shia Ithna-Ashari
        }
    }

    /// Convert Madhab to Islamic Finder school number
    fn madhab_to_number(madhab: Madhab) -> u8 {
        match madhab {
            Madhab::Shafi => 0,  // Shafi (standard)
            Madhab::Hanafi => 1, // Hanafi
        }
    }

    /// Parse Unix timestamp to NaiveTime
    fn timestamp_to_time(timestamp: i64) -> Result<NaiveTime, ApiError> {
        let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
            .ok_or_else(|| {
                ApiError::InvalidResponse(
                    "islamic_finder".to_string(),
                    format!("Invalid timestamp: {}", timestamp),
                )
            })?;
        Ok(datetime.naive_utc().time())
    }
}

impl Default for IslamicFinderPrayerClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ApiClient for IslamicFinderPrayerClient {
    fn api_name(&self) -> &str {
        "islamic_finder"
    }

    fn priority(&self) -> u8 {
        2 // Secondary API
    }

    async fn is_healthy(&self) -> bool {
        // Simple health check - try to get prayer times for a known location
        let test_request = PrayerTimesRequest {
            latitude: 21.4225,  // Mecca
            longitude: 39.8262,
            date: chrono::Utc::now().naive_utc().date(),
            calculation_method: CalculationMethod::Makkah,
            madhab: Madhab::Shafi,
        };

        match self.get_times(&test_request).await {
            Ok(_) => true,
            Err(e) => {
                tracing::warn!("Islamic Finder health check failed: {}", e);
                false
            }
        }
    }

    fn rate_limit(&self) -> RateLimitConfig {
        RateLimitConfig {
            requests_per_minute: 30,
            requests_per_hour: 500,
            requests_per_day: 5000,
        }
    }
}

#[async_trait]
impl PrayerTimesApiClient for IslamicFinderPrayerClient {
    async fn get_times(&self, request: &PrayerTimesRequest) -> Result<PrayerTimesResponse, ApiError> {
        // Validate coordinates
        if request.latitude < -90.0 || request.latitude > 90.0 {
            return Err(ApiError::Validation(format!(
                "Invalid latitude: {}. Must be between -90 and 90",
                request.latitude
            )));
        }
        if request.longitude < -180.0 || request.longitude > 180.0 {
            return Err(ApiError::Validation(format!(
                "Invalid longitude: {}. Must be between -180 and 180",
                request.longitude
            )));
        }

        let method = Self::method_to_number(request.calculation_method);
        let school = Self::madhab_to_number(request.madhab);

        // Note: Islamic Finder API format may vary - this is a simplified implementation
        // In production, you would need to verify the actual API endpoint and parameters
        let url = format!("{}/prayer-times", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .query(&[
                ("latitude", request.latitude.to_string()),
                ("longitude", request.longitude.to_string()),
                ("method", method.to_string()),
                ("school", school.to_string()),
                ("date", request.date.format("%Y-%m-%d").to_string()),
            ])
            .send()
            .await
            .map_err(|e| {
                ApiError::Network(format!("Failed to fetch prayer times from Islamic Finder: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!(
                    "HTTP {}: {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            ));
        }

        let prayer_response: IslamicFinderPrayerResponse = response.json().await.map_err(|e| {
            ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("Failed to parse prayer times response: {}", e),
            )
        })?;

        // Parse times from response
        // Note: The actual response format may differ - adjust based on real API
        let timings = &prayer_response.results;

        Ok(PrayerTimesResponse {
            date: request.date,
            fajr: Self::timestamp_to_time(timings.fajr)?,
            sunrise: Self::timestamp_to_time(timings.sunrise)?,
            dhuhr: Self::timestamp_to_time(timings.dhuhr)?,
            asr: Self::timestamp_to_time(timings.asr)?,
            maghrib: Self::timestamp_to_time(timings.maghrib)?,
            isha: Self::timestamp_to_time(timings.isha)?,
            source: self.api_name().to_string(),
        })
    }

    async fn get_times_range(
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
                    return Err(e);
                }
            }

            current_date = current_date
                .succ_opt()
                .ok_or_else(|| ApiError::Validation("Date overflow".to_string()))?;
        }

        Ok(results)
    }
}

// ============================================================================
// Response structures for Islamic Finder API
// Note: These structures are simplified and may need adjustment based on actual API
// ============================================================================

#[derive(Debug, Deserialize)]
struct IslamicFinderPrayerResponse {
    results: IslamicFinderTimings,
}

#[derive(Debug, Deserialize)]
struct IslamicFinderTimings {
    fajr: i64,    // Unix timestamp
    sunrise: i64,
    dhuhr: i64,
    asr: i64,
    maghrib: i64,
    isha: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = IslamicFinderPrayerClient::new();
        assert_eq!(client.api_name(), "islamic_finder");
        assert_eq!(client.priority(), 2);
    }

    #[test]
    fn test_rate_limit_config() {
        let client = IslamicFinderPrayerClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 30);
        assert_eq!(config.requests_per_hour, 500);
        assert_eq!(config.requests_per_day, 5000);
    }

    #[test]
    fn test_method_conversion() {
        assert_eq!(IslamicFinderPrayerClient::method_to_number(CalculationMethod::MWL), 3);
        assert_eq!(IslamicFinderPrayerClient::method_to_number(CalculationMethod::ISNA), 2);
        assert_eq!(IslamicFinderPrayerClient::method_to_number(CalculationMethod::Egypt), 5);
        assert_eq!(IslamicFinderPrayerClient::method_to_number(CalculationMethod::Makkah), 4);
        assert_eq!(IslamicFinderPrayerClient::method_to_number(CalculationMethod::Karachi), 1);
        assert_eq!(IslamicFinderPrayerClient::method_to_number(CalculationMethod::Tehran), 7);
        assert_eq!(IslamicFinderPrayerClient::method_to_number(CalculationMethod::Jafari), 0);
    }

    #[test]
    fn test_madhab_conversion() {
        assert_eq!(IslamicFinderPrayerClient::madhab_to_number(Madhab::Shafi), 0);
        assert_eq!(IslamicFinderPrayerClient::madhab_to_number(Madhab::Hanafi), 1);
    }

    #[test]
    fn test_timestamp_to_time() {
        // Test with a known timestamp: 2024-01-01 05:50:00 UTC
        let timestamp = 1704088200;
        let time = IslamicFinderPrayerClient::timestamp_to_time(timestamp).unwrap();
        assert_eq!(time.hour(), 5);
        assert_eq!(time.minute(), 50);
    }

    #[tokio::test]
    async fn test_invalid_coordinates() {
        let client = IslamicFinderPrayerClient::new();

        // Invalid latitude
        let request = PrayerTimesRequest {
            latitude: 91.0,
            longitude: 0.0,
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            calculation_method: CalculationMethod::MWL,
            madhab: Madhab::Shafi,
        };
        let result = client.get_times(&request).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));

        // Invalid longitude
        let request = PrayerTimesRequest {
            latitude: 0.0,
            longitude: 181.0,
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            calculation_method: CalculationMethod::MWL,
            madhab: Madhab::Shafi,
        };
        let result = client.get_times(&request).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }
}

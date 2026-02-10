//! Aladhan Prayer Times API Client
//!
//! Official Islamic Network API for prayer times calculations.
//! API Documentation: https://aladhan.com/prayer-times-api

use crate::api_clients::{
    ApiClient, ApiError, CalculationMethod, Madhab, PrayerTimesApiClient, PrayerTimesRequest,
    PrayerTimesResponse, RateLimitConfig,
};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveTime, Timelike};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

/// Aladhan Prayer Times API client
#[derive(Debug, Clone)]
pub struct AladhanPrayerClient {
    base_url: String,
    client: Client,
}

impl AladhanPrayerClient {
    /// Create a new Aladhan Prayer Times API client
    pub fn new() -> Self {
        Self {
            base_url: "https://api.aladhan.com/v1".to_string(),
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

    /// Convert CalculationMethod to Aladhan method number
    fn method_to_number(method: CalculationMethod) -> u8 {
        match method {
            CalculationMethod::MWL => 3,      // Muslim World League
            CalculationMethod::ISNA => 2,     // Islamic Society of North America
            CalculationMethod::Egypt => 5,    // Egyptian General Authority of Survey
            CalculationMethod::Makkah => 4,   // Umm Al-Qura University, Makkah
            CalculationMethod::Karachi => 1,  // University of Islamic Sciences, Karachi
            CalculationMethod::Tehran => 7,   // Institute of Geophysics, University of Tehran
            CalculationMethod::Jafari => 0,   // Shia Ithna-Ashari, Leva Institute, Qum
        }
    }

    /// Convert Madhab to Aladhan school number
    fn madhab_to_number(madhab: Madhab) -> u8 {
        match madhab {
            Madhab::Shafi => 0,  // Shafi, Maliki, Hanbali (standard)
            Madhab::Hanafi => 1, // Hanafi
        }
    }

    /// Parse time string in format "HH:MM" to NaiveTime
    fn parse_time(time_str: &str) -> Result<NaiveTime, ApiError> {
        NaiveTime::parse_from_str(time_str, "%H:%M").map_err(|e| {
            ApiError::InvalidResponse(
                "aladhan".to_string(),
                format!("Failed to parse time '{}': {}", time_str, e),
            )
        })
    }
}

impl Default for AladhanPrayerClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ApiClient for AladhanPrayerClient {
    fn api_name(&self) -> &str {
        "aladhan"
    }

    fn priority(&self) -> u8 {
        1 // Primary API
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
                tracing::warn!("Aladhan health check failed: {}", e);
                false
            }
        }
    }

    fn rate_limit(&self) -> RateLimitConfig {
        RateLimitConfig {
            requests_per_minute: 60,
            requests_per_hour: 1000,
            requests_per_day: 10000,
        }
    }
}

#[async_trait]
impl PrayerTimesApiClient for AladhanPrayerClient {
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
        let date_str = request.date.format("%d-%m-%Y").to_string();

        let url = format!("{}/timings/{}", self.base_url, date_str);
        
        let response = self
            .client
            .get(&url)
            .query(&[
                ("latitude", request.latitude.to_string()),
                ("longitude", request.longitude.to_string()),
                ("method", method.to_string()),
                ("school", school.to_string()),
            ])
            .send()
            .await
            .map_err(|e| {
                ApiError::Network(format!("Failed to fetch prayer times from Aladhan: {}", e))
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

        let prayer_response: AladhanPrayerResponse = response.json().await.map_err(|e| {
            ApiError::InvalidResponse(
                self.api_name().to_string(),
                format!("Failed to parse prayer times response: {}", e),
            )
        })?;

        if prayer_response.code != 200 {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!(
                    "API returned code {}: {}",
                    prayer_response.code, prayer_response.status
                ),
            ));
        }

        let timings = &prayer_response.data.timings;

        Ok(PrayerTimesResponse {
            date: request.date,
            fajr: Self::parse_time(&timings.fajr)?,
            sunrise: Self::parse_time(&timings.sunrise)?,
            dhuhr: Self::parse_time(&timings.dhuhr)?,
            asr: Self::parse_time(&timings.asr)?,
            maghrib: Self::parse_time(&timings.maghrib)?,
            isha: Self::parse_time(&timings.isha)?,
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
// Response structures for Aladhan API
// ============================================================================

#[derive(Debug, Deserialize)]
struct AladhanPrayerResponse {
    code: u16,
    status: String,
    data: AladhanPrayerData,
}

#[derive(Debug, Deserialize)]
struct AladhanPrayerData {
    timings: AladhanTimings,
}

#[derive(Debug, Deserialize)]
struct AladhanTimings {
    #[serde(rename = "Fajr")]
    fajr: String,
    #[serde(rename = "Sunrise")]
    sunrise: String,
    #[serde(rename = "Dhuhr")]
    dhuhr: String,
    #[serde(rename = "Asr")]
    asr: String,
    #[serde(rename = "Maghrib")]
    maghrib: String,
    #[serde(rename = "Isha")]
    isha: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = AladhanPrayerClient::new();
        assert_eq!(client.api_name(), "aladhan");
        assert_eq!(client.priority(), 1);
    }

    #[test]
    fn test_rate_limit_config() {
        let client = AladhanPrayerClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.requests_per_hour, 1000);
        assert_eq!(config.requests_per_day, 10000);
    }

    #[test]
    fn test_method_conversion() {
        assert_eq!(AladhanPrayerClient::method_to_number(CalculationMethod::MWL), 3);
        assert_eq!(AladhanPrayerClient::method_to_number(CalculationMethod::ISNA), 2);
        assert_eq!(AladhanPrayerClient::method_to_number(CalculationMethod::Egypt), 5);
        assert_eq!(AladhanPrayerClient::method_to_number(CalculationMethod::Makkah), 4);
        assert_eq!(AladhanPrayerClient::method_to_number(CalculationMethod::Karachi), 1);
        assert_eq!(AladhanPrayerClient::method_to_number(CalculationMethod::Tehran), 7);
        assert_eq!(AladhanPrayerClient::method_to_number(CalculationMethod::Jafari), 0);
    }

    #[test]
    fn test_madhab_conversion() {
        assert_eq!(AladhanPrayerClient::madhab_to_number(Madhab::Shafi), 0);
        assert_eq!(AladhanPrayerClient::madhab_to_number(Madhab::Hanafi), 1);
    }

    #[test]
    fn test_time_parsing() {
        let time = AladhanPrayerClient::parse_time("05:30").unwrap();
        assert_eq!(time.hour(), 5);
        assert_eq!(time.minute(), 30);

        let time = AladhanPrayerClient::parse_time("23:59").unwrap();
        assert_eq!(time.hour(), 23);
        assert_eq!(time.minute(), 59);
    }

    #[test]
    fn test_invalid_time_parsing() {
        let result = AladhanPrayerClient::parse_time("25:00");
        assert!(result.is_err());

        let result = AladhanPrayerClient::parse_time("invalid");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_coordinates() {
        let client = AladhanPrayerClient::new();

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

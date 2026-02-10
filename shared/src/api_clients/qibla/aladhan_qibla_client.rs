// Aladhan Qibla API Client
//
// Official API: https://aladhan.com/qibla-api
// Provides Qibla direction calculation based on geographical coordinates

use crate::api_clients::{
    ApiClient, ApiError, QiblaApiClient, QiblaResponse, RateLimitConfig,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const ALADHAN_BASE_URL: &str = "https://api.aladhan.com/v1";
const API_NAME: &str = "aladhan_qibla";

/// Aladhan Qibla API client
///
/// Provides Qibla direction calculation using the Aladhan API
pub struct AladhanQiblaClient {
    client: Client,
    base_url: String,
    timeout: Duration,
}

#[derive(Debug, Deserialize)]
struct AladhanQiblaApiResponse {
    code: i32,
    status: String,
    data: AladhanQiblaData,
}

#[derive(Debug, Deserialize)]
struct AladhanQiblaData {
    latitude: f64,
    longitude: f64,
    direction: f64,
}

impl AladhanQiblaClient {
    /// Create a new Aladhan Qibla client
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            base_url: ALADHAN_BASE_URL.to_string(),
            timeout: Duration::from_secs(10),
        }
    }

    /// Create a new client with custom base URL (for testing)
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            base_url,
            timeout: Duration::from_secs(10),
        }
    }

    /// Calculate distance to Mecca using Haversine formula
    fn calculate_distance_to_mecca(latitude: f64, longitude: f64) -> f64 {
        const MECCA_LAT: f64 = 21.4225;
        const MECCA_LON: f64 = 39.8262;
        const EARTH_RADIUS_KM: f64 = 6371.0;

        let lat1_rad = latitude.to_radians();
        let lat2_rad = MECCA_LAT.to_radians();
        let delta_lat = (MECCA_LAT - latitude).to_radians();
        let delta_lon = (MECCA_LON - longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS_KM * c
    }
}

impl Default for AladhanQiblaClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ApiClient for AladhanQiblaClient {
    fn api_name(&self) -> &str {
        API_NAME
    }

    fn priority(&self) -> u8 {
        1 // Primary Qibla API
    }

    async fn is_healthy(&self) -> bool {
        // Simple health check - try to get Qibla for Mecca itself
        match self.get_direction(21.4225, 39.8262).await {
            Ok(_) => true,
            Err(e) => {
                log::warn!("Aladhan Qibla API health check failed: {}", e);
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
impl QiblaApiClient for AladhanQiblaClient {
    async fn get_direction(&self, latitude: f64, longitude: f64) -> Result<QiblaResponse, ApiError> {
        // Validate coordinates
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(ApiError::InvalidInput(format!(
                "Invalid latitude: {}. Must be between -90 and 90",
                latitude
            )));
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(ApiError::InvalidInput(format!(
                "Invalid longitude: {}. Must be between -180 and 180",
                longitude
            )));
        }

        let url = format!("{}/qibla/{}/{}", self.base_url, latitude, longitude);

        let response = self
            .client
            .get(&url)
            .timeout(self.timeout)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    ApiError::Timeout(API_NAME.to_string())
                } else if e.is_connect() {
                    ApiError::Network(format!("Connection error: {}", e))
                } else {
                    ApiError::Network(format!("Request error: {}", e))
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ApiError::ApiError(
                API_NAME.to_string(),
                format!("HTTP {}: {}", status, error_text),
            ));
        }

        let api_response: AladhanQiblaApiResponse = response.json().await.map_err(|e| {
            ApiError::InvalidResponse(
                API_NAME.to_string(),
                format!("Failed to parse JSON: {}", e),
            )
        })?;

        // Validate response
        if api_response.code != 200 || api_response.status != "OK" {
            return Err(ApiError::ApiError(
                API_NAME.to_string(),
                format!(
                    "API returned error: code={}, status={}",
                    api_response.code, api_response.status
                ),
            ));
        }

        let direction = api_response.data.direction;

        // Validate direction is in valid range
        if !(0.0..=360.0).contains(&direction) {
            return Err(ApiError::InvalidResponse(
                API_NAME.to_string(),
                format!("Invalid direction: {}. Must be between 0 and 360", direction),
            ));
        }

        // Calculate distance to Mecca
        let distance_km = Self::calculate_distance_to_mecca(latitude, longitude);

        Ok(QiblaResponse {
            direction,
            distance_km,
            source: API_NAME.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = AladhanQiblaClient::new();
        assert_eq!(client.api_name(), API_NAME);
        assert_eq!(client.priority(), 1);
    }

    #[test]
    fn test_rate_limit_config() {
        let client = AladhanQiblaClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.requests_per_hour, 1000);
        assert_eq!(config.requests_per_day, 10000);
    }

    #[test]
    fn test_distance_calculation() {
        // Test distance from New York to Mecca (approximately 9,700 km)
        let distance = AladhanQiblaClient::calculate_distance_to_mecca(40.7128, -74.0060);
        assert!(distance > 9500.0 && distance < 10000.0);

        // Test distance from Mecca to Mecca (should be 0)
        let distance = AladhanQiblaClient::calculate_distance_to_mecca(21.4225, 39.8262);
        assert!(distance < 1.0); // Within 1 km due to rounding

        // Test distance from London to Mecca (approximately 4,300 km)
        let distance = AladhanQiblaClient::calculate_distance_to_mecca(51.5074, -0.1278);
        assert!(distance > 4100.0 && distance < 4500.0);
    }

    #[tokio::test]
    async fn test_invalid_coordinates() {
        let client = AladhanQiblaClient::new();

        // Invalid latitude (> 90)
        let result = client.get_direction(91.0, 0.0).await;
        assert!(matches!(result, Err(ApiError::InvalidInput(_))));

        // Invalid latitude (< -90)
        let result = client.get_direction(-91.0, 0.0).await;
        assert!(matches!(result, Err(ApiError::InvalidInput(_))));

        // Invalid longitude (> 180)
        let result = client.get_direction(0.0, 181.0).await;
        assert!(matches!(result, Err(ApiError::InvalidInput(_))));

        // Invalid longitude (< -180)
        let result = client.get_direction(0.0, -181.0).await;
        assert!(matches!(result, Err(ApiError::InvalidInput(_))));
    }
}

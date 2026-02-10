// Islamic Finder Qibla API Client
//
// Provides Qibla direction calculation using local astronomical formulas
// This serves as a fallback when external APIs are unavailable

use crate::api_clients::{
    ApiClient, ApiError, QiblaApiClient, QiblaResponse, RateLimitConfig,
};
use async_trait::async_trait;
use std::time::Duration;

const API_NAME: &str = "islamic_finder_qibla";

// Mecca coordinates
const MECCA_LAT: f64 = 21.4225;
const MECCA_LON: f64 = 39.8262;
const EARTH_RADIUS_KM: f64 = 6371.0;

/// Islamic Finder Qibla client using local calculation
///
/// This client calculates Qibla direction using astronomical formulas
/// without making external API calls. It serves as a reliable fallback.
pub struct IslamicFinderQiblaClient {
    timeout: Duration,
}

impl IslamicFinderQiblaClient {
    /// Create a new Islamic Finder Qibla client
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(1), // Local calculation is fast
        }
    }

    /// Calculate Qibla direction using the great circle formula
    ///
    /// This uses the formula:
    /// qibla = atan2(sin(Δλ), cos(φ1)·tan(φ2) - sin(φ1)·cos(Δλ))
    ///
    /// Where:
    /// - φ1, λ1 = latitude and longitude of the observer
    /// - φ2, λ2 = latitude and longitude of Mecca
    /// - Δλ = λ2 - λ1
    fn calculate_qibla_direction(latitude: f64, longitude: f64) -> f64 {
        let lat1_rad = latitude.to_radians();
        let lon1_rad = longitude.to_radians();
        let lat2_rad = MECCA_LAT.to_radians();
        let lon2_rad = MECCA_LON.to_radians();

        let delta_lon = lon2_rad - lon1_rad;

        let y = delta_lon.sin();
        let x = lat1_rad.cos() * lat2_rad.tan() - lat1_rad.sin() * delta_lon.cos();

        let bearing_rad = y.atan2(x);
        let bearing_deg = bearing_rad.to_degrees();

        // Normalize to 0-360 range
        (bearing_deg + 360.0) % 360.0
    }

    /// Calculate distance to Mecca using Haversine formula
    fn calculate_distance_to_mecca(latitude: f64, longitude: f64) -> f64 {
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

impl Default for IslamicFinderQiblaClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ApiClient for IslamicFinderQiblaClient {
    fn api_name(&self) -> &str {
        API_NAME
    }

    fn priority(&self) -> u8 {
        2 // Secondary (fallback) Qibla API
    }

    async fn is_healthy(&self) -> bool {
        // Local calculation is always available
        true
    }

    fn rate_limit(&self) -> RateLimitConfig {
        RateLimitConfig {
            // No rate limits for local calculation
            requests_per_minute: u32::MAX,
            requests_per_hour: u32::MAX,
            requests_per_day: u32::MAX,
        }
    }
}

#[async_trait]
impl QiblaApiClient for IslamicFinderQiblaClient {
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

        // Calculate Qibla direction
        let direction = Self::calculate_qibla_direction(latitude, longitude);

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
        let client = IslamicFinderQiblaClient::new();
        assert_eq!(client.api_name(), API_NAME);
        assert_eq!(client.priority(), 2);
    }

    #[test]
    fn test_rate_limit_config() {
        let client = IslamicFinderQiblaClient::new();
        let config = client.rate_limit();
        // Local calculation has no rate limits
        assert_eq!(config.requests_per_minute, u32::MAX);
        assert_eq!(config.requests_per_hour, u32::MAX);
        assert_eq!(config.requests_per_day, u32::MAX);
    }

    #[tokio::test]
    async fn test_is_always_healthy() {
        let client = IslamicFinderQiblaClient::new();
        assert!(client.is_healthy().await);
    }

    #[test]
    fn test_qibla_direction_calculation() {
        // Test from New York (should be approximately 58 degrees)
        let direction = IslamicFinderQiblaClient::calculate_qibla_direction(40.7128, -74.0060);
        assert!(direction > 50.0 && direction < 65.0);

        // Test from London (should be approximately 119 degrees)
        let direction = IslamicFinderQiblaClient::calculate_qibla_direction(51.5074, -0.1278);
        assert!(direction > 110.0 && direction < 125.0);

        // Test from Tokyo (should be approximately 293 degrees)
        let direction = IslamicFinderQiblaClient::calculate_qibla_direction(35.6762, 139.6503);
        assert!(direction > 285.0 && direction < 300.0);

        // Test from Sydney (should be approximately 277 degrees)
        let direction = IslamicFinderQiblaClient::calculate_qibla_direction(-33.8688, 151.2093);
        assert!(direction > 270.0 && direction < 285.0);

        // Test from Mecca itself (direction is undefined, but should be valid)
        let direction = IslamicFinderQiblaClient::calculate_qibla_direction(MECCA_LAT, MECCA_LON);
        assert!(direction >= 0.0 && direction <= 360.0);
    }

    #[test]
    fn test_distance_calculation() {
        // Test distance from New York to Mecca (approximately 9,700 km)
        let distance = IslamicFinderQiblaClient::calculate_distance_to_mecca(40.7128, -74.0060);
        assert!(distance > 9500.0 && distance < 10000.0);

        // Test distance from Mecca to Mecca (should be 0)
        let distance = IslamicFinderQiblaClient::calculate_distance_to_mecca(MECCA_LAT, MECCA_LON);
        assert!(distance < 1.0); // Within 1 km due to rounding

        // Test distance from London to Mecca (approximately 4,300 km)
        let distance = IslamicFinderQiblaClient::calculate_distance_to_mecca(51.5074, -0.1278);
        assert!(distance > 4100.0 && distance < 4500.0);
    }

    #[tokio::test]
    async fn test_get_direction() {
        let client = IslamicFinderQiblaClient::new();

        // Test from New York
        let result = client.get_direction(40.7128, -74.0060).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.direction >= 0.0 && response.direction <= 360.0);
        assert!(response.distance_km > 9500.0 && response.distance_km < 10000.0);
        assert_eq!(response.source, API_NAME);

        // Test from London
        let result = client.get_direction(51.5074, -0.1278).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.direction >= 0.0 && response.direction <= 360.0);
        assert!(response.distance_km > 4100.0 && response.distance_km < 4500.0);
    }

    #[tokio::test]
    async fn test_invalid_coordinates() {
        let client = IslamicFinderQiblaClient::new();

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

    #[tokio::test]
    async fn test_direction_normalization() {
        let client = IslamicFinderQiblaClient::new();

        // Test various locations to ensure direction is always 0-360
        let test_locations = vec![
            (0.0, 0.0),
            (90.0, 0.0),
            (-90.0, 0.0),
            (0.0, 180.0),
            (0.0, -180.0),
            (45.0, 90.0),
            (-45.0, -90.0),
        ];

        for (lat, lon) in test_locations {
            let result = client.get_direction(lat, lon).await;
            assert!(result.is_ok());
            let response = result.unwrap();
            assert!(
                response.direction >= 0.0 && response.direction <= 360.0,
                "Direction {} out of range for location ({}, {})",
                response.direction,
                lat,
                lon
            );
        }
    }
}

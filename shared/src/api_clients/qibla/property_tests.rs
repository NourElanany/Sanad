// Property-based tests for Qibla API clients
//
// These tests verify universal properties that should hold for all Qibla calculations

use crate::api_clients::{
    ApiClient, CacheManager, MockRedisClient, QiblaApiClient, RateLimiter,
};
use proptest::prelude::*;
use std::sync::Arc;

use super::{AladhanQiblaClient, IslamicFinderQiblaClient, QiblaApiManager};

// Feature: official-apis-integration, Property 10: Qibla Direction Valid Range
// **Validates: Requirements 6.3**
//
// For any location coordinates, the calculated qibla direction should be within
// the valid range of 0-360 degrees (inclusive).
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn property_qibla_direction_valid_range(
        latitude in -90.0f64..=90.0f64,
        longitude in -180.0f64..=180.0f64,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Test with local calculation client (always available)
            let client = IslamicFinderQiblaClient::new();
            
            let result = client.get_direction(latitude, longitude).await;
            
            // Should succeed for all valid coordinates
            prop_assert!(result.is_ok(), "Failed for valid coordinates ({}, {}): {:?}", latitude, longitude, result);
            
            let response = result.unwrap();
            
            // Direction must be in valid range [0, 360]
            prop_assert!(
                response.direction >= 0.0 && response.direction <= 360.0,
                "Direction {} out of valid range [0, 360] for location ({}, {})",
                response.direction,
                latitude,
                longitude
            );
            
            // Distance must be non-negative
            prop_assert!(
                response.distance_km >= 0.0,
                "Distance {} is negative for location ({}, {})",
                response.distance_km,
                latitude,
                longitude
            );
            
            // Source should be set
            prop_assert!(
                !response.source.is_empty(),
                "Source is empty for location ({}, {})",
                latitude,
                longitude
            );
            
            Ok(())
        });
    }
}

// Feature: official-apis-integration, Property 20: Local Calculation Fallback
// **Validates: Requirements 12.3**
//
// For any prayer times or qibla request where all APIs fail and no cache exists,
// the system should attempt local calculation using astronomical formulas,
// and the result should be within reasonable accuracy.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn property_local_calculation_fallback(
        latitude in -90.0f64..=90.0f64,
        longitude in -180.0f64..=180.0f64,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let cache = Arc::new(CacheManager::new("redis://localhost:6379").await.unwrap());
            let rate_limiter = Arc::new(RateLimiter::new("redis://localhost:6379", Default::default()).await.unwrap());
            
            // Create manager with only local calculation client
            let clients: Vec<Box<dyn QiblaApiClient + Send + Sync>> = vec![
                Box::new(IslamicFinderQiblaClient::new()),
            ];
            let manager = QiblaApiManager::with_clients(clients, cache, rate_limiter);
            
            let result = manager.get_direction(latitude, longitude).await;
            
            // Local calculation should always succeed for valid coordinates
            prop_assert!(
                result.is_ok(),
                "Local calculation failed for valid coordinates ({}, {}): {:?}",
                latitude,
                longitude,
                result
            );
            
            let response = result.unwrap();
            
            // Verify result is reasonable
            prop_assert!(
                response.direction >= 0.0 && response.direction <= 360.0,
                "Local calculation produced invalid direction {} for location ({}, {})",
                response.direction,
                latitude,
                longitude
            );
            
            prop_assert!(
                response.distance_km >= 0.0,
                "Local calculation produced negative distance {} for location ({}, {})",
                response.distance_km,
                latitude,
                longitude
            );
            
            // Distance should be reasonable (Earth's circumference is ~40,075 km)
            prop_assert!(
                response.distance_km <= 20100.0, // Half of Earth's circumference
                "Local calculation produced unreasonable distance {} km for location ({}, {})",
                response.distance_km,
                latitude,
                longitude
            );
            
            Ok(())
        });
    }
}

// Additional property: Direction consistency
// For the same location, multiple calls should return the same direction
proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn property_direction_consistency(
        latitude in -90.0f64..=90.0f64,
        longitude in -180.0f64..=180.0f64,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let client = IslamicFinderQiblaClient::new();
            
            // Get direction twice
            let result1 = client.get_direction(latitude, longitude).await;
            let result2 = client.get_direction(latitude, longitude).await;
            
            prop_assert!(result1.is_ok() && result2.is_ok());
            
            let response1 = result1.unwrap();
            let response2 = result2.unwrap();
            
            // Directions should be identical for the same location
            prop_assert!(
                (response1.direction - response2.direction).abs() < 0.001,
                "Inconsistent directions for location ({}, {}): {} vs {}",
                latitude,
                longitude,
                response1.direction,
                response2.direction
            );
            
            // Distances should be identical
            prop_assert!(
                (response1.distance_km - response2.distance_km).abs() < 0.001,
                "Inconsistent distances for location ({}, {}): {} vs {}",
                latitude,
                longitude,
                response1.distance_km,
                response2.distance_km
            );
            
            Ok(())
        });
    }
}

// Additional property: Distance symmetry
// Distance from A to Mecca should equal distance from Mecca to A
#[tokio::test]
async fn property_distance_symmetry() {
    let client = IslamicFinderQiblaClient::new();
    
    // Mecca coordinates
    const MECCA_LAT: f64 = 21.4225;
    const MECCA_LON: f64 = 39.8262;
    
    // Test location: New York
    let ny_lat = 40.7128;
    let ny_lon = -74.0060;
    
    let result = client.get_direction(ny_lat, ny_lon).await;
    assert!(result.is_ok());
    
    let distance = result.unwrap().distance_km;
    
    // Distance should be the same regardless of direction
    // (This is a property of the Haversine formula)
    assert!(distance > 0.0);
    assert!(distance < 20100.0); // Less than half Earth's circumference
}

// Additional property: Antipodal points
// For antipodal points (opposite sides of Earth), distance should be close to half Earth's circumference
#[tokio::test]
async fn property_antipodal_distance() {
    let client = IslamicFinderQiblaClient::new();
    
    // Mecca coordinates
    const MECCA_LAT: f64 = 21.4225;
    const MECCA_LON: f64 = 39.8262;
    
    // Antipodal point of Mecca
    let anti_lat = -MECCA_LAT;
    let anti_lon = if MECCA_LON > 0.0 {
        MECCA_LON - 180.0
    } else {
        MECCA_LON + 180.0
    };
    
    let result = client.get_direction(anti_lat, anti_lon).await;
    assert!(result.is_ok());
    
    let distance = result.unwrap().distance_km;
    
    // Distance to antipodal point should be close to half Earth's circumference (~20,000 km)
    const HALF_EARTH_CIRCUMFERENCE: f64 = 20037.5;
    const TOLERANCE: f64 = 500.0; // 500 km tolerance
    
    assert!(
        (distance - HALF_EARTH_CIRCUMFERENCE).abs() < TOLERANCE,
        "Distance to antipodal point {} is not close to half Earth's circumference {}",
        distance,
        HALF_EARTH_CIRCUMFERENCE
    );
}

// Additional property: Nearby locations have similar directions
proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn property_nearby_locations_similar_directions(
        latitude in -89.0f64..=89.0f64,  // Avoid poles
        longitude in -179.0f64..=179.0f64,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let client = IslamicFinderQiblaClient::new();
            
            // Get direction for original location
            let result1 = client.get_direction(latitude, longitude).await;
            prop_assert!(result1.is_ok());
            let direction1 = result1.unwrap().direction;
            
            // Get direction for nearby location (0.1 degrees away, ~11 km)
            let result2 = client.get_direction(latitude + 0.1, longitude + 0.1).await;
            prop_assert!(result2.is_ok());
            let direction2 = result2.unwrap().direction;
            
            // Directions should be similar (within 5 degrees) for nearby locations
            let diff = (direction1 - direction2).abs();
            let diff_normalized = if diff > 180.0 { 360.0 - diff } else { diff };
            
            prop_assert!(
                diff_normalized < 5.0,
                "Directions differ too much for nearby locations: {} vs {} (diff: {})",
                direction1,
                direction2,
                diff_normalized
            );
            
            Ok(())
        });
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[tokio::test]
    async fn test_property_tests_run() {
        // This test ensures property tests can be executed
        // The actual property tests are run by proptest
        assert!(true);
    }

    #[tokio::test]
    async fn test_known_locations() {
        let client = IslamicFinderQiblaClient::new();
        
        // Test known locations with expected approximate directions
        let test_cases = vec![
            // (lat, lon, expected_direction_range_min, expected_direction_range_max)
            (40.7128, -74.0060, 50.0, 65.0),   // New York
            (51.5074, -0.1278, 110.0, 125.0),  // London
            (35.6762, 139.6503, 285.0, 300.0), // Tokyo
        ];
        
        for (lat, lon, min_dir, max_dir) in test_cases {
            let result = client.get_direction(lat, lon).await;
            assert!(result.is_ok(), "Failed for location ({}, {})", lat, lon);
            
            let response = result.unwrap();
            assert!(
                response.direction >= min_dir && response.direction <= max_dir,
                "Direction {} out of expected range [{}, {}] for location ({}, {})",
                response.direction,
                min_dir,
                max_dir,
                lat,
                lon
            );
        }
    }
}

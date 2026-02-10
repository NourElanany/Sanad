// Unit tests for Qibla API clients

use crate::api_clients::{
    ApiClient, ApiError, CacheManager, MockRedisClient, QiblaApiClient, RateLimiter,
};
use std::sync::Arc;

use super::{AladhanQiblaClient, IslamicFinderQiblaClient, QiblaApiManager};

// ============================================================================
// AladhanQiblaClient Tests
// ============================================================================

#[test]
fn test_aladhan_client_creation() {
    let client = AladhanQiblaClient::new();
    assert_eq!(client.api_name(), "aladhan_qibla");
    assert_eq!(client.priority(), 1);
}

#[test]
fn test_aladhan_rate_limit() {
    let client = AladhanQiblaClient::new();
    let config = client.rate_limit();
    assert_eq!(config.requests_per_minute, 60);
    assert_eq!(config.requests_per_hour, 1000);
    assert_eq!(config.requests_per_day, 10000);
}

#[tokio::test]
async fn test_aladhan_invalid_latitude() {
    let client = AladhanQiblaClient::new();

    // Latitude > 90
    let result = client.get_direction(91.0, 0.0).await;
    assert!(matches!(result, Err(ApiError::Validation(_))));

    // Latitude < -90
    let result = client.get_direction(-91.0, 0.0).await;
    assert!(matches!(result, Err(ApiError::Validation(_))));
}

#[tokio::test]
async fn test_aladhan_invalid_longitude() {
    let client = AladhanQiblaClient::new();

    // Longitude > 180
    let result = client.get_direction(0.0, 181.0).await;
    assert!(matches!(result, Err(ApiError::Validation(_))));

    // Longitude < -180
    let result = client.get_direction(0.0, -181.0).await;
    assert!(matches!(result, Err(ApiError::Validation(_))));
}

#[tokio::test]
async fn test_aladhan_boundary_coordinates() {
    let client = AladhanQiblaClient::new();

    // Test boundary values
    let test_cases = vec![
        (90.0, 0.0),    // North pole
        (-90.0, 0.0),   // South pole
        (0.0, 180.0),   // International date line
        (0.0, -180.0),  // International date line
        (0.0, 0.0),     // Equator/Prime meridian
    ];

    for (lat, lon) in test_cases {
        let result = client.get_direction(lat, lon).await;
        // May succeed or fail depending on API availability, but should not panic
        match result {
            Ok(response) => {
                assert!(response.direction >= 0.0 && response.direction <= 360.0);
                assert!(response.distance_km >= 0.0);
            }
            Err(_) => {
                // API might be unavailable, which is okay for this test
            }
        }
    }
}

// ============================================================================
// IslamicFinderQiblaClient Tests
// ============================================================================

#[test]
fn test_islamic_finder_client_creation() {
    let client = IslamicFinderQiblaClient::new();
    assert_eq!(client.api_name(), "islamic_finder_qibla");
    assert_eq!(client.priority(), 2);
}

#[test]
fn test_islamic_finder_rate_limit() {
    let client = IslamicFinderQiblaClient::new();
    let config = client.rate_limit();
    // Local calculation has no rate limits
    assert_eq!(config.requests_per_minute, u32::MAX);
    assert_eq!(config.requests_per_hour, u32::MAX);
    assert_eq!(config.requests_per_day, u32::MAX);
}

#[tokio::test]
async fn test_islamic_finder_always_healthy() {
    let client = IslamicFinderQiblaClient::new();
    assert!(client.is_healthy().await);
}

#[tokio::test]
async fn test_islamic_finder_direction_calculation() {
    let client = IslamicFinderQiblaClient::new();

    // Test known locations with expected approximate directions
    let test_cases = vec![
        // (lat, lon, expected_min, expected_max, description)
        (40.7128, -74.0060, 50.0, 65.0, "New York"),
        (51.5074, -0.1278, 110.0, 125.0, "London"),
        (35.6762, 139.6503, 285.0, 300.0, "Tokyo"),
        (-33.8688, 151.2093, 270.0, 285.0, "Sydney"),
    ];

    for (lat, lon, min_dir, max_dir, desc) in test_cases {
        let result = client.get_direction(lat, lon).await;
        assert!(result.is_ok(), "Failed for {}", desc);

        let response = result.unwrap();
        assert!(
            response.direction >= min_dir && response.direction <= max_dir,
            "Direction {} out of expected range [{}, {}] for {}",
            response.direction,
            min_dir,
            max_dir,
            desc
        );
        assert!(response.distance_km > 0.0, "Distance should be positive for {}", desc);
        assert_eq!(response.source, "islamic_finder_qibla");
    }
}

#[tokio::test]
async fn test_islamic_finder_distance_calculation() {
    let client = IslamicFinderQiblaClient::new();

    // Test known distances
    let test_cases = vec![
        // (lat, lon, expected_min_km, expected_max_km, description)
        (40.7128, -74.0060, 9500.0, 10000.0, "New York to Mecca"),
        (51.5074, -0.1278, 4100.0, 4500.0, "London to Mecca"),
        (21.4225, 39.8262, 0.0, 1.0, "Mecca to Mecca"),
    ];

    for (lat, lon, min_dist, max_dist, desc) in test_cases {
        let result = client.get_direction(lat, lon).await;
        assert!(result.is_ok(), "Failed for {}", desc);

        let response = result.unwrap();
        assert!(
            response.distance_km >= min_dist && response.distance_km <= max_dist,
            "Distance {} out of expected range [{}, {}] for {}",
            response.distance_km,
            min_dist,
            max_dist,
            desc
        );
    }
}

#[tokio::test]
async fn test_islamic_finder_invalid_coordinates() {
    let client = IslamicFinderQiblaClient::new();

    // Invalid latitude
    let result = client.get_direction(91.0, 0.0).await;
    assert!(matches!(result, Err(ApiError::Validation(_))));

    let result = client.get_direction(-91.0, 0.0).await;
    assert!(matches!(result, Err(ApiError::Validation(_))));

    // Invalid longitude
    let result = client.get_direction(0.0, 181.0).await;
    assert!(matches!(result, Err(ApiError::Validation(_))));

    let result = client.get_direction(0.0, -181.0).await;
    assert!(matches!(result, Err(ApiError::Validation(_))));
}

#[tokio::test]
async fn test_islamic_finder_direction_range() {
    let client = IslamicFinderQiblaClient::new();

    // Test various locations to ensure direction is always 0-360
    let test_locations = vec![
        (0.0, 0.0),
        (45.0, 45.0),
        (-45.0, -45.0),
        (89.0, 179.0),
        (-89.0, -179.0),
    ];

    for (lat, lon) in test_locations {
        let result = client.get_direction(lat, lon).await;
        assert!(result.is_ok(), "Failed for location ({}, {})", lat, lon);

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

// ============================================================================
// QiblaApiManager Tests
// ============================================================================

fn create_test_manager() -> QiblaApiManager {
    let redis = Arc::new(MockRedisClient::new());
    let cache = Arc::new(CacheManager::new(redis.clone()));
    let rate_limiter = Arc::new(RateLimiter::new(redis));
    QiblaApiManager::new(cache, rate_limiter)
}

#[test]
fn test_manager_creation() {
    let manager = create_test_manager();
    assert_eq!(manager.get_clients().len(), 2);
}

#[test]
fn test_cache_key_generation() {
    let key1 = QiblaApiManager::cache_key(40.7128, -74.0060);
    let key2 = QiblaApiManager::cache_key(40.7128, -74.0060);
    assert_eq!(key1, key2);

    // Test that different locations have different keys
    let key3 = QiblaApiManager::cache_key(51.5074, -0.1278);
    assert_ne!(key1, key3);

    // Test rounding (locations within ~11 meters should have same key)
    let key4 = QiblaApiManager::cache_key(40.71281234, -74.00601234);
    let key5 = QiblaApiManager::cache_key(40.71284567, -74.00604567);
    assert_eq!(key4, key5);
}

#[tokio::test]
async fn test_manager_get_direction() {
    let manager = create_test_manager();

    let result = manager.get_direction(40.7128, -74.0060).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.direction >= 0.0 && response.direction <= 360.0);
    assert!(response.distance_km > 0.0);
    assert!(!response.source.is_empty());
}

#[tokio::test]
async fn test_manager_caching() {
    let manager = create_test_manager();

    // First request
    let result1 = manager.get_direction(51.5074, -0.1278).await;
    assert!(result1.is_ok());
    let response1 = result1.unwrap();

    // Second request (should hit cache)
    let result2 = manager.get_direction(51.5074, -0.1278).await;
    assert!(result2.is_ok());
    let response2 = result2.unwrap();

    // Results should be identical
    assert_eq!(response1.direction, response2.direction);
    assert_eq!(response1.distance_km, response2.distance_km);
}

#[tokio::test]
async fn test_manager_fallback_to_local() {
    let manager = create_test_manager();

    // Even if primary API fails, local calculation should work
    let result = manager.get_direction(35.6762, 139.6503).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.direction >= 0.0 && response.direction <= 360.0);
    assert!(response.distance_km > 0.0);
}

#[tokio::test]
async fn test_manager_invalid_coordinates() {
    let manager = create_test_manager();

    // Invalid latitude
    let result = manager.get_direction(91.0, 0.0).await;
    assert!(result.is_err());

    // Invalid longitude
    let result = manager.get_direction(0.0, 181.0).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_manager_health_check() {
    let manager = create_test_manager();
    let health = manager.health_check().await;

    assert_eq!(health.len(), 2);

    // Check that we have both clients
    let has_aladhan = health.iter().any(|(name, _)| name.contains("aladhan"));
    let has_islamic_finder = health.iter().any(|(name, _)| name.contains("islamic_finder"));
    assert!(has_aladhan);
    assert!(has_islamic_finder);

    // Local calculation should always be healthy
    let local_health = health.iter().find(|(name, _)| name.contains("islamic_finder"));
    assert!(local_health.is_some());
    assert!(local_health.unwrap().1);
}

#[tokio::test]
async fn test_manager_multiple_locations() {
    let manager = create_test_manager();

    let test_locations = vec![
        (40.7128, -74.0060),  // New York
        (51.5074, -0.1278),   // London
        (35.6762, 139.6503),  // Tokyo
        (-33.8688, 151.2093), // Sydney
        (0.0, 0.0),           // Null Island
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
        assert!(response.distance_km >= 0.0);
    }
}

#[tokio::test]
async fn test_manager_concurrent_requests() {
    let manager = Arc::new(create_test_manager());

    let mut handles = vec![];

    // Make 10 concurrent requests
    for i in 0..10 {
        let manager_clone = Arc::clone(&manager);
        let handle = tokio::spawn(async move {
            let lat = 40.0 + (i as f64) * 0.1;
            let lon = -74.0 + (i as f64) * 0.1;
            manager_clone.get_direction(lat, lon).await
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

#[tokio::test]
async fn test_direction_range_validation() {
    let manager = create_test_manager();

    // Test that all responses have valid direction range
    let test_locations = vec![
        (40.7128, -74.0060),
        (51.5074, -0.1278),
        (35.6762, 139.6503),
        (-33.8688, 151.2093),
        (0.0, 0.0),
        (21.4225, 39.8262), // Mecca itself
    ];

    for (lat, lon) in test_locations {
        let result = manager.get_direction(lat, lon).await;
        assert!(result.is_ok(), "Failed for location ({}, {})", lat, lon);

        let response = result.unwrap();

        // Requirement 6.3: Direction must be in valid range [0, 360]
        assert!(
            response.direction >= 0.0 && response.direction <= 360.0,
            "Direction {} out of valid range for location ({}, {})",
            response.direction,
            lat,
            lon
        );
    }
}

#[tokio::test]
async fn test_fallback_to_local_calculation() {
    let redis = Arc::new(MockRedisClient::new());
    let cache = Arc::new(CacheManager::new(redis.clone()));
    let rate_limiter = Arc::new(RateLimiter::new(redis));

    // Create manager with only local calculation
    let clients: Vec<Box<dyn QiblaApiClient + Send + Sync>> = vec![
        Box::new(IslamicFinderQiblaClient::new()),
    ];
    let manager = QiblaApiManager::with_clients(clients, cache, rate_limiter);

    // Requirement 6.4: Local calculation should work as fallback
    let result = manager.get_direction(40.7128, -74.0060).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.direction >= 0.0 && response.direction <= 360.0);
    assert!(response.distance_km > 0.0);
    assert_eq!(response.source, "islamic_finder_qibla");
}

#[tokio::test]
async fn test_cache_with_location_based_keys() {
    let manager = create_test_manager();

    // Requirement 6.5: Cache with location-based keys
    let lat = 40.7128;
    let lon = -74.0060;

    // First request
    let result1 = manager.get_direction(lat, lon).await;
    assert!(result1.is_ok());

    // Second request with same location
    let result2 = manager.get_direction(lat, lon).await;
    assert!(result2.is_ok());

    // Should get same result (from cache)
    let response1 = result1.unwrap();
    let response2 = result2.unwrap();
    assert_eq!(response1.direction, response2.direction);
    assert_eq!(response1.distance_km, response2.distance_km);
}

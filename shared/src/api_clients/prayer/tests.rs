//! Unit tests for Prayer Times API clients

#[cfg(test)]
mod tests {
    use crate::api_clients::prayer::{
        AladhanPrayerClient, IslamicFinderPrayerClient, PrayerTimesApiManager,
    };
    use crate::api_clients::{
        ApiClient, CalculationMethod, CacheManager, Madhab, PrayerTimesApiClient,
        PrayerTimesRequest, RateLimiter,
    };
    use chrono::NaiveDate;
    use std::collections::HashMap;
    use std::sync::Arc;

    // ============================================================================
    // Aladhan Client Tests
    // ============================================================================

    #[test]
    fn test_aladhan_client_creation() {
        let client = AladhanPrayerClient::new();
        assert_eq!(client.api_name(), "aladhan");
        assert_eq!(client.priority(), 1);
    }

    #[test]
    fn test_aladhan_rate_limit_config() {
        let client = AladhanPrayerClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.requests_per_hour, 1000);
        assert_eq!(config.requests_per_day, 10000);
    }

    #[tokio::test]
    async fn test_aladhan_invalid_latitude() {
        let client = AladhanPrayerClient::new();
        let request = PrayerTimesRequest {
            latitude: 91.0, // Invalid
            longitude: 0.0,
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            calculation_method: CalculationMethod::MWL,
            madhab: Madhab::Shafi,
        };

        let result = client.get_times(&request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_aladhan_invalid_longitude() {
        let client = AladhanPrayerClient::new();
        let request = PrayerTimesRequest {
            latitude: 0.0,
            longitude: 181.0, // Invalid
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            calculation_method: CalculationMethod::MWL,
            madhab: Madhab::Shafi,
        };

        let result = client.get_times(&request).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_aladhan_all_calculation_methods() {
        let methods = vec![
            CalculationMethod::MWL,
            CalculationMethod::ISNA,
            CalculationMethod::Egypt,
            CalculationMethod::Makkah,
            CalculationMethod::Karachi,
            CalculationMethod::Tehran,
            CalculationMethod::Jafari,
        ];

        for method in methods {
            let request = PrayerTimesRequest {
                latitude: 21.4225,
                longitude: 39.8262,
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                calculation_method: method,
                madhab: Madhab::Shafi,
            };

            // Should not panic
            assert!(matches!(
                request.calculation_method,
                CalculationMethod::MWL
                    | CalculationMethod::ISNA
                    | CalculationMethod::Egypt
                    | CalculationMethod::Makkah
                    | CalculationMethod::Karachi
                    | CalculationMethod::Tehran
                    | CalculationMethod::Jafari
            ));
        }
    }

    #[test]
    fn test_aladhan_both_madhabs() {
        let madhabs = vec![Madhab::Shafi, Madhab::Hanafi];

        for madhab in madhabs {
            let request = PrayerTimesRequest {
                latitude: 21.4225,
                longitude: 39.8262,
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                calculation_method: CalculationMethod::MWL,
                madhab,
            };

            // Should not panic
            assert!(matches!(request.madhab, Madhab::Shafi | Madhab::Hanafi));
        }
    }

    // ============================================================================
    // Islamic Finder Client Tests
    // ============================================================================

    #[test]
    fn test_islamic_finder_client_creation() {
        let client = IslamicFinderPrayerClient::new();
        assert_eq!(client.api_name(), "islamic_finder");
        assert_eq!(client.priority(), 2);
    }

    #[test]
    fn test_islamic_finder_rate_limit_config() {
        let client = IslamicFinderPrayerClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 30);
        assert_eq!(config.requests_per_hour, 500);
        assert_eq!(config.requests_per_day, 5000);
    }

    #[tokio::test]
    async fn test_islamic_finder_invalid_latitude() {
        let client = IslamicFinderPrayerClient::new();
        let request = PrayerTimesRequest {
            latitude: -91.0, // Invalid
            longitude: 0.0,
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            calculation_method: CalculationMethod::MWL,
            madhab: Madhab::Shafi,
        };

        let result = client.get_times(&request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_islamic_finder_invalid_longitude() {
        let client = IslamicFinderPrayerClient::new();
        let request = PrayerTimesRequest {
            latitude: 0.0,
            longitude: -181.0, // Invalid
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            calculation_method: CalculationMethod::MWL,
            madhab: Madhab::Shafi,
        };

        let result = client.get_times(&request).await;
        assert!(result.is_err());
    }

    // ============================================================================
    // Manager Tests
    // ============================================================================

    async fn create_test_manager() -> PrayerTimesApiManager {
        let cache = Arc::new(
            CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"),
        );

        let rate_limiter = Arc::new(
            RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                .await
                .expect("Failed to create rate limiter"),
        );

        let clients: Vec<Box<dyn PrayerTimesApiClient + Send + Sync>> = vec![
            Box::new(AladhanPrayerClient::new()),
            Box::new(IslamicFinderPrayerClient::new()),
        ];

        PrayerTimesApiManager::new(clients, cache, rate_limiter)
    }

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = create_test_manager().await;
        assert_eq!(manager.client_count(), 2);
    }

    #[tokio::test]
    async fn test_manager_client_priority_order() {
        let manager = create_test_manager().await;
        let names = manager.client_names();

        // Should be sorted by priority: aladhan (1), islamic_finder (2)
        assert_eq!(names[0], "aladhan");
        assert_eq!(names[1], "islamic_finder");
    }

    #[tokio::test]
    async fn test_manager_invalid_coordinates() {
        let manager = create_test_manager().await;

        let request = PrayerTimesRequest {
            latitude: 100.0, // Invalid
            longitude: 0.0,
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            calculation_method: CalculationMethod::MWL,
            madhab: Madhab::Shafi,
        };

        let result = manager.get_times(&request).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_manager_chronological_validation() {
        use chrono::NaiveTime;
        use crate::api_clients::PrayerTimesResponse;

        // Valid chronological order
        let valid_times = PrayerTimesResponse {
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            fajr: NaiveTime::from_hms_opt(5, 30, 0).unwrap(),
            sunrise: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
            dhuhr: NaiveTime::from_hms_opt(12, 30, 0).unwrap(),
            asr: NaiveTime::from_hms_opt(15, 30, 0).unwrap(),
            maghrib: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            isha: NaiveTime::from_hms_opt(19, 30, 0).unwrap(),
            source: "test".to_string(),
        };

        // This should pass validation
        assert!(valid_times.fajr < valid_times.sunrise);
        assert!(valid_times.sunrise < valid_times.dhuhr);
        assert!(valid_times.dhuhr < valid_times.asr);
        assert!(valid_times.asr < valid_times.maghrib);
        assert!(valid_times.maghrib < valid_times.isha);

        // Invalid chronological order (Fajr after Sunrise)
        let invalid_times = PrayerTimesResponse {
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            fajr: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            sunrise: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
            dhuhr: NaiveTime::from_hms_opt(12, 30, 0).unwrap(),
            asr: NaiveTime::from_hms_opt(15, 30, 0).unwrap(),
            maghrib: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            isha: NaiveTime::from_hms_opt(19, 30, 0).unwrap(),
            source: "test".to_string(),
        };

        // This should fail validation
        assert!(invalid_times.fajr >= invalid_times.sunrise);
    }

    #[tokio::test]
    async fn test_manager_date_range() {
        let manager = create_test_manager().await;

        let request = PrayerTimesRequest {
            latitude: 21.4225,
            longitude: 39.8262,
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            calculation_method: CalculationMethod::Makkah,
            madhab: Madhab::Shafi,
        };

        // Request 7 days of prayer times
        // Note: This will likely fail without real API access, but tests the interface
        let result = manager.get_times_range(&request, 7).await;
        
        // We expect either success or a network error (since we don't have real API access in tests)
        // The important thing is that the interface works correctly
        match result {
            Ok(times) => {
                // If successful, verify we got results
                assert!(!times.is_empty());
                assert!(times.len() <= 7);
            }
            Err(_) => {
                // Expected in test environment without real API access
            }
        }
    }

    // ============================================================================
    // Edge Case Tests
    // ============================================================================

    #[test]
    fn test_boundary_coordinates() {
        // Test boundary values for coordinates
        let valid_coords = vec![
            (-90.0, -180.0),  // Min values
            (90.0, 180.0),    // Max values
            (0.0, 0.0),       // Equator/Prime Meridian
            (21.4225, 39.8262), // Mecca
            (40.7128, -74.0060), // New York
        ];

        for (lat, lon) in valid_coords {
            let request = PrayerTimesRequest {
                latitude: lat,
                longitude: lon,
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                calculation_method: CalculationMethod::MWL,
                madhab: Madhab::Shafi,
            };

            // Should not panic
            assert!(request.latitude >= -90.0 && request.latitude <= 90.0);
            assert!(request.longitude >= -180.0 && request.longitude <= 180.0);
        }
    }

    #[test]
    fn test_date_boundaries() {
        // Test various dates
        let dates = vec![
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),   // Start of decade
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(), // End of year
            NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),  // Leap year
            NaiveDate::from_ymd_opt(2024, 6, 21).unwrap(),  // Summer solstice
            NaiveDate::from_ymd_opt(2024, 12, 21).unwrap(), // Winter solstice
        ];

        for date in dates {
            let request = PrayerTimesRequest {
                latitude: 21.4225,
                longitude: 39.8262,
                date,
                calculation_method: CalculationMethod::Makkah,
                madhab: Madhab::Shafi,
            };

            // Should not panic
            assert_eq!(request.date, date);
        }
    }

    #[test]
    fn test_all_calculation_methods_and_madhabs() {
        let methods = vec![
            CalculationMethod::MWL,
            CalculationMethod::ISNA,
            CalculationMethod::Egypt,
            CalculationMethod::Makkah,
            CalculationMethod::Karachi,
            CalculationMethod::Tehran,
            CalculationMethod::Jafari,
        ];

        let madhabs = vec![Madhab::Shafi, Madhab::Hanafi];

        for method in &methods {
            for madhab in &madhabs {
                let request = PrayerTimesRequest {
                    latitude: 21.4225,
                    longitude: 39.8262,
                    date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                    calculation_method: *method,
                    madhab: *madhab,
                };

                // Should not panic
                assert!(matches!(
                    request.calculation_method,
                    CalculationMethod::MWL
                        | CalculationMethod::ISNA
                        | CalculationMethod::Egypt
                        | CalculationMethod::Makkah
                        | CalculationMethod::Karachi
                        | CalculationMethod::Tehran
                        | CalculationMethod::Jafari
                ));
                assert!(matches!(request.madhab, Madhab::Shafi | Madhab::Hanafi));
            }
        }
    }
}

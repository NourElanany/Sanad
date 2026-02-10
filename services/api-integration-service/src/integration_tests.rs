//! Integration Tests for API Integration Service
//!
//! These tests verify end-to-end functionality with actual API managers
//! and components working together.

#[cfg(test)]
mod integration_tests {
    use crate::models::*;
    use crate::service::ApiIntegrationService;
    use chrono::NaiveDate;

    /// Helper to create a test configuration
    fn create_test_config() -> ServiceConfig {
        ServiceConfig {
            service: ServiceInfo {
                name: "test-integration-service".to_string(),
                port: 8080,
                host: "localhost".to_string(),
            },
            redis: RedisConfig {
                url: std::env::var("REDIS_URL")
                    .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
                pool_size: 10,
                connection_timeout: "5s".to_string(),
            },
            postgres: PostgresConfig {
                url: std::env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgresql://localhost:5432/test".to_string()),
                pool_size: 20,
                connection_timeout: "10s".to_string(),
            },
            apis: ApiConfigs {
                quran: vec![
                    ApiConfig {
                        name: "quran.com".to_string(),
                        base_url: "https://api.quran.com/api/v4".to_string(),
                        priority: 1,
                        requires_key: Some(false),
                        rate_limit: RateLimitConfig {
                            requests_per_minute: 60,
                            requests_per_hour: 1000,
                            requests_per_day: 10000,
                        },
                        timeout: "10s".to_string(),
                    },
                ],
                hadith: vec![
                    ApiConfig {
                        name: "sunnah.com".to_string(),
                        base_url: "https://api.sunnah.com/v1".to_string(),
                        priority: 1,
                        requires_key: Some(true),
                        rate_limit: RateLimitConfig {
                            requests_per_minute: 30,
                            requests_per_hour: 500,
                            requests_per_day: 5000,
                        },
                        timeout: "15s".to_string(),
                    },
                ],
                prayer_times: vec![
                    ApiConfig {
                        name: "aladhan".to_string(),
                        base_url: "https://api.aladhan.com/v1".to_string(),
                        priority: 1,
                        requires_key: Some(false),
                        rate_limit: RateLimitConfig {
                            requests_per_minute: 60,
                            requests_per_hour: 1000,
                            requests_per_day: 10000,
                        },
                        timeout: "5s".to_string(),
                    },
                ],
                tafsir: vec![
                    ApiConfig {
                        name: "quran.com".to_string(),
                        base_url: "https://api.quran.com/api/v4".to_string(),
                        priority: 1,
                        requires_key: Some(false),
                        rate_limit: RateLimitConfig {
                            requests_per_minute: 60,
                            requests_per_hour: 1000,
                            requests_per_day: 10000,
                        },
                        timeout: "10s".to_string(),
                    },
                ],
                calendar: vec![
                    ApiConfig {
                        name: "aladhan".to_string(),
                        base_url: "https://api.aladhan.com/v1".to_string(),
                        priority: 1,
                        requires_key: Some(false),
                        rate_limit: RateLimitConfig {
                            requests_per_minute: 60,
                            requests_per_hour: 1000,
                            requests_per_day: 10000,
                        },
                        timeout: "5s".to_string(),
                    },
                ],
                qibla: vec![
                    ApiConfig {
                        name: "aladhan".to_string(),
                        base_url: "https://api.aladhan.com/v1".to_string(),
                        priority: 1,
                        requires_key: Some(false),
                        rate_limit: RateLimitConfig {
                            requests_per_minute: 60,
                            requests_per_hour: 1000,
                            requests_per_day: 10000,
                        },
                        timeout: "5s".to_string(),
                    },
                ],
                ai: vec![],
            },
            cache: CacheConfig {
                strategies: CacheStrategies {
                    quran_text: CacheStrategy {
                        ttl: "30d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("90d".to_string()),
                    },
                    hadith: CacheStrategy {
                        ttl: "30d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("90d".to_string()),
                    },
                    prayer_times: CacheStrategy {
                        ttl: "1d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("7d".to_string()),
                    },
                    tafsir: CacheStrategy {
                        ttl: "30d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("90d".to_string()),
                    },
                    calendar: CacheStrategy {
                        ttl: "7d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("30d".to_string()),
                    },
                    qibla: CacheStrategy {
                        ttl: "30d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("90d".to_string()),
                    },
                    ai_response: CacheStrategy {
                        ttl: "1h".to_string(),
                        allow_stale: false,
                        stale_ttl: None,
                    },
                },
            },
            health_monitor: HealthMonitorConfig {
                check_interval: "5m".to_string(),
                unhealthy_threshold: 3,
                recovery_threshold: 2,
            },
            retry: RetryConfig {
                max_attempts: 3,
                initial_delay: "1s".to_string(),
                max_delay: "10s".to_string(),
                multiplier: 2.0,
            },
        }
    }

    #[tokio::test]
    #[ignore] // Requires Redis to be running
    async fn test_service_initialization() {
        let config = create_test_config();
        let service = ApiIntegrationService::new(config).await;
        
        assert!(service.is_ok(), "Service should initialize successfully");
    }

    #[tokio::test]
    #[ignore] // Requires Redis to be running
    async fn test_health_check() {
        let config = create_test_config();
        let service = ApiIntegrationService::new(config).await
            .expect("Service should initialize");
        
        let health = service.health_check().await;
        
        // Health check should return a status
        assert!(
            matches!(
                health.overall_status,
                ServiceStatus::Healthy | ServiceStatus::Degraded | ServiceStatus::Unhealthy
            ),
            "Health check should return a valid status"
        );
    }

    #[tokio::test]
    #[ignore] // Requires Redis and actual API access
    async fn test_quran_text_retrieval_with_caching() {
        let config = create_test_config();
        let service = ApiIntegrationService::new(config).await
            .expect("Service should initialize");
        
        let request = QuranTextRequest {
            surah: 1,
            ayah: Some(1),
            translation: None,
            reciter: None,
        };
        
        // First request - should hit API
        let start = std::time::Instant::now();
        let result1 = service.get_quran_text(request.clone()).await;
        let duration1 = start.elapsed();
        
        assert!(result1.is_ok(), "First request should succeed");
        
        // Second request - should hit cache
        let start = std::time::Instant::now();
        let result2 = service.get_quran_text(request.clone()).await;
        let duration2 = start.elapsed();
        
        assert!(result2.is_ok(), "Second request should succeed");
        
        // Cached request should be faster
        assert!(
            duration2 < duration1,
            "Cached request should be faster than API request"
        );
        
        // Results should be identical
        let response1 = result1.unwrap();
        let response2 = result2.unwrap();
        assert_eq!(response1.text_arabic, response2.text_arabic);
    }

    #[tokio::test]
    #[ignore] // Requires Redis and actual API access
    async fn test_prayer_times_retrieval() {
        let config = create_test_config();
        let service = ApiIntegrationService::new(config).await
            .expect("Service should initialize");
        
        let request = PrayerTimesRequest {
            latitude: 21.4225,  // Mecca
            longitude: 39.8262,
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            calculation_method: CalculationMethod::Makkah,
            madhab: Madhab::Shafi,
        };
        
        let result = service.get_prayer_times(request).await;
        
        assert!(result.is_ok(), "Prayer times request should succeed");
        
        let response = result.unwrap();
        
        // Verify prayer times are in chronological order
        assert!(response.fajr < response.sunrise);
        assert!(response.sunrise < response.dhuhr);
        assert!(response.dhuhr < response.asr);
        assert!(response.asr < response.maghrib);
        assert!(response.maghrib < response.isha);
    }

    #[tokio::test]
    #[ignore] // Requires Redis and actual API access
    async fn test_qibla_direction_retrieval() {
        let config = create_test_config();
        let service = ApiIntegrationService::new(config).await
            .expect("Service should initialize");
        
        let request = QiblaRequest {
            latitude: 40.7128,  // New York
            longitude: -74.0060,
        };
        
        let result = service.get_qibla_direction(request).await;
        
        assert!(result.is_ok(), "Qibla direction request should succeed");
        
        let response = result.unwrap();
        
        // Verify direction is within valid range (0-360 degrees)
        assert!(
            response.direction >= 0.0 && response.direction <= 360.0,
            "Qibla direction should be between 0 and 360 degrees"
        );
        
        // Verify distance is positive
        assert!(response.distance_km > 0.0, "Distance should be positive");
    }

    #[tokio::test]
    #[ignore] // Requires Redis and actual API access
    async fn test_date_conversion() {
        let config = create_test_config();
        let service = ApiIntegrationService::new(config).await
            .expect("Service should initialize");
        
        let request = DateConversionRequest {
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            direction: ConversionDirection::GregorianToHijri,
        };
        
        let result = service.convert_date(request).await;
        
        assert!(result.is_ok(), "Date conversion should succeed");
        
        let response = result.unwrap();
        
        // Verify Hijri date fields are valid
        assert!(response.hijri.year > 1400, "Hijri year should be reasonable");
        assert!(response.hijri.month >= 1 && response.hijri.month <= 12);
        assert!(response.hijri.day >= 1 && response.hijri.day <= 30);
        assert!(!response.hijri.month_name_ar.is_empty());
        assert!(!response.hijri.month_name_en.is_empty());
    }

    #[tokio::test]
    #[ignore] // Requires Redis
    async fn test_rate_limiting_integration() {
        let config = create_test_config();
        let service = ApiIntegrationService::new(config).await
            .expect("Service should initialize");
        
        // Make multiple requests rapidly
        let request = QuranTextRequest {
            surah: 1,
            ayah: Some(1),
            translation: None,
            reciter: None,
        };
        
        let mut success_count = 0;
        let mut rate_limit_count = 0;
        
        for _ in 0..10 {
            match service.get_quran_text(request.clone()).await {
                Ok(_) => success_count += 1,
                Err(e) => {
                    if e.to_string().contains("rate limit") {
                        rate_limit_count += 1;
                    }
                }
            }
        }
        
        // At least some requests should succeed
        assert!(success_count > 0, "Some requests should succeed");
        
        // Note: Rate limiting behavior depends on configuration and timing
        println!("Success: {}, Rate limited: {}", success_count, rate_limit_count);
    }

    #[tokio::test]
    #[ignore] // Requires Redis
    async fn test_fallback_mechanism() {
        // This test would require mocking API failures
        // For now, we just verify the service can handle errors gracefully
        let config = create_test_config();
        let service = ApiIntegrationService::new(config).await
            .expect("Service should initialize");
        
        // Request with invalid data to trigger fallback
        let request = QuranTextRequest {
            surah: 255,  // Invalid surah number
            ayah: Some(1),
            translation: None,
            reciter: None,
        };
        
        let result = service.get_quran_text(request).await;
        
        // Should either succeed with fallback or return a proper error
        match result {
            Ok(_) => println!("Request succeeded (possibly via fallback)"),
            Err(e) => println!("Request failed with error: {}", e),
        }
    }
}

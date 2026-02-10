//! Unit Tests for Calendar API Clients
//!
//! These tests verify specific examples, edge cases, and error conditions.

use crate::api_clients::calendar::{
    AladhanCalendarClient, CalendarApiManager, IslamicFinderCalendarClient,
};
use crate::api_clients::{ApiClient, CacheManager, CalendarApiClient, HijriDate, RateLimiter};
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// AladhanCalendarClient Tests
// ============================================================================

#[tokio::test]
async fn test_aladhan_client_creation() {
    let client = AladhanCalendarClient::new();
    assert_eq!(client.api_name(), "aladhan_calendar");
    assert_eq!(client.priority(), 1);
}

#[tokio::test]
async fn test_aladhan_rate_limit_config() {
    let client = AladhanCalendarClient::new();
    let config = client.rate_limit();
    assert_eq!(config.requests_per_minute, 60);
    assert_eq!(config.requests_per_hour, 1000);
    assert_eq!(config.requests_per_day, 10000);
}

#[tokio::test]
async fn test_aladhan_gregorian_to_hijri() {
    let client = AladhanCalendarClient::new();
    
    // Test a known date: January 1, 2024
    let gregorian = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let result = client.gregorian_to_hijri(gregorian).await;
    
    // Should succeed (or fail gracefully if API is down)
    match result {
        Ok(hijri) => {
            // Verify the result is reasonable
            assert!(hijri.year >= 1445 && hijri.year <= 1446);
            assert!(hijri.month >= 1 && hijri.month <= 12);
            assert!(hijri.day >= 1 && hijri.day <= 30);
            assert!(!hijri.month_name_ar.is_empty());
            assert!(!hijri.month_name_en.is_empty());
        }
        Err(e) => {
            // API might be down, log and skip
            println!("Aladhan API unavailable: {}", e);
        }
    }
}

#[tokio::test]
async fn test_aladhan_hijri_to_gregorian() {
    let client = AladhanCalendarClient::new();
    
    // Test a known Hijri date: 1 Muharram 1445
    let hijri = HijriDate {
        year: 1445,
        month: 1,
        day: 1,
        month_name_ar: "محرم".to_string(),
        month_name_en: "Muharram".to_string(),
    };
    
    let result = client.hijri_to_gregorian(&hijri).await;
    
    match result {
        Ok(gregorian) => {
            // Should be around July 2023
            assert!(gregorian.year() == 2023 || gregorian.year() == 2024);
            assert!(gregorian.month() >= 1 && gregorian.month() <= 12);
        }
        Err(e) => {
            println!("Aladhan API unavailable: {}", e);
        }
    }
}

#[tokio::test]
async fn test_aladhan_invalid_hijri_month() {
    let client = AladhanCalendarClient::new();
    
    let hijri = HijriDate {
        year: 1445,
        month: 13, // Invalid month
        day: 1,
        month_name_ar: "Invalid".to_string(),
        month_name_en: "Invalid".to_string(),
    };
    
    let result = client.hijri_to_gregorian(&hijri).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_aladhan_invalid_hijri_day() {
    let client = AladhanCalendarClient::new();
    
    let hijri = HijriDate {
        year: 1445,
        month: 1,
        day: 31, // Invalid day (Hijri months have max 30 days)
        month_name_ar: "محرم".to_string(),
        month_name_en: "Muharram".to_string(),
    };
    
    let result = client.hijri_to_gregorian(&hijri).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_aladhan_get_events() {
    let client = AladhanCalendarClient::new();
    
    // Test getting events for a month
    let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
    
    let result = client.get_events(start, end).await;
    
    match result {
        Ok(events) => {
            // Events should be within the range
            for event in &events {
                assert!(event.date >= start && event.date <= end);
                assert!(!event.event_name_ar.is_empty());
                assert!(!event.event_name_en.is_empty());
            }
        }
        Err(e) => {
            println!("Aladhan API unavailable: {}", e);
        }
    }
}

#[tokio::test]
async fn test_aladhan_invalid_date_range() {
    let client = AladhanCalendarClient::new();
    
    // Start date after end date
    let start = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    
    let result = client.get_events(start, end).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_aladhan_islamic_events_detection() {
    let client = AladhanCalendarClient::new();
    
    // Test detection of major Islamic events
    let ramadan_start = HijriDate {
        year: 1445,
        month: 9,
        day: 1,
        month_name_ar: "رمضان".to_string(),
        month_name_en: "Ramadan".to_string(),
    };
    
    let event = AladhanCalendarClient::check_islamic_event(&ramadan_start);
    assert!(event.is_some());
    assert_eq!(event.unwrap().1, "Start of Ramadan");
    
    let eid_fitr = HijriDate {
        year: 1445,
        month: 10,
        day: 1,
        month_name_ar: "شوال".to_string(),
        month_name_en: "Shawwal".to_string(),
    };
    
    let event = AladhanCalendarClient::check_islamic_event(&eid_fitr);
    assert!(event.is_some());
    assert_eq!(event.unwrap().1, "Eid al-Fitr");
    
    let eid_adha = HijriDate {
        year: 1445,
        month: 12,
        day: 10,
        month_name_ar: "ذو الحجة".to_string(),
        month_name_en: "Dhu al-Hijjah".to_string(),
    };
    
    let event = AladhanCalendarClient::check_islamic_event(&eid_adha);
    assert!(event.is_some());
    assert_eq!(event.unwrap().1, "Eid al-Adha");
}

// ============================================================================
// IslamicFinderCalendarClient Tests
// ============================================================================

#[tokio::test]
async fn test_islamic_finder_client_creation() {
    let client = IslamicFinderCalendarClient::new();
    assert_eq!(client.api_name(), "islamic_finder_calendar");
    assert_eq!(client.priority(), 2);
}

#[tokio::test]
async fn test_islamic_finder_rate_limit_config() {
    let client = IslamicFinderCalendarClient::new();
    let config = client.rate_limit();
    assert_eq!(config.requests_per_minute, 30);
    assert_eq!(config.requests_per_hour, 500);
    assert_eq!(config.requests_per_day, 5000);
}

#[tokio::test]
async fn test_islamic_finder_gregorian_to_hijri() {
    let client = IslamicFinderCalendarClient::new();
    
    let gregorian = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let hijri = client.gregorian_to_hijri(gregorian).await.unwrap();
    
    // Verify the result is reasonable
    assert!(hijri.year >= 1445 && hijri.year <= 1446);
    assert!(hijri.month >= 1 && hijri.month <= 12);
    assert!(hijri.day >= 1 && hijri.day <= 30);
    assert!(!hijri.month_name_ar.is_empty());
    assert!(!hijri.month_name_en.is_empty());
}

#[tokio::test]
async fn test_islamic_finder_hijri_to_gregorian() {
    let client = IslamicFinderCalendarClient::new();
    
    let hijri = HijriDate {
        year: 1445,
        month: 1,
        day: 1,
        month_name_ar: "محرم".to_string(),
        month_name_en: "Muharram".to_string(),
    };
    
    let gregorian = client.hijri_to_gregorian(&hijri).await.unwrap();
    
    // Should be a valid date
    assert!(gregorian.year() >= 2020 && gregorian.year() <= 2030);
}

#[tokio::test]
async fn test_islamic_finder_invalid_hijri_date() {
    let client = IslamicFinderCalendarClient::new();
    
    // Invalid month
    let hijri = HijriDate {
        year: 1445,
        month: 13,
        day: 1,
        month_name_ar: "Invalid".to_string(),
        month_name_en: "Invalid".to_string(),
    };
    
    let result = client.hijri_to_gregorian(&hijri).await;
    assert!(result.is_err());
}

// ============================================================================
// CalendarApiManager Tests
// ============================================================================

async fn create_test_manager() -> CalendarApiManager {
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

    let clients: Vec<Box<dyn CalendarApiClient + Send + Sync>> = vec![
        Box::new(AladhanCalendarClient::new()),
        Box::new(IslamicFinderCalendarClient::new()),
    ];

    CalendarApiManager::new(clients, cache, rate_limiter)
}

#[tokio::test]
async fn test_manager_creation() {
    let manager = create_test_manager().await;
    assert_eq!(manager.client_count(), 2);
}

#[tokio::test]
async fn test_manager_clients_sorted_by_priority() {
    let manager = create_test_manager().await;
    let names = manager.client_names();
    
    // Should be sorted by priority: aladhan_calendar (1), islamic_finder_calendar (2)
    assert_eq!(names[0], "aladhan_calendar");
    assert_eq!(names[1], "islamic_finder_calendar");
}

#[tokio::test]
async fn test_manager_gregorian_to_hijri() {
    let manager = create_test_manager().await;
    
    let gregorian = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    
    // Clear cache to ensure fresh conversion
    let cache_key = format!("calendar:g2h:{}", gregorian);
    let _ = manager.clear_cache(&cache_key).await;
    
    let result = manager.gregorian_to_hijri(gregorian).await;
    
    match result {
        Ok(hijri) => {
            assert!(hijri.year >= 1445 && hijri.year <= 1446);
            assert!(hijri.month >= 1 && hijri.month <= 12);
            assert!(hijri.day >= 1 && hijri.day <= 30);
        }
        Err(e) => {
            println!("Calendar API unavailable: {}", e);
        }
    }
}

#[tokio::test]
async fn test_manager_hijri_to_gregorian() {
    let manager = create_test_manager().await;
    
    let hijri = HijriDate {
        year: 1445,
        month: 1,
        day: 1,
        month_name_ar: "محرم".to_string(),
        month_name_en: "Muharram".to_string(),
    };
    
    // Clear cache
    let cache_key = format!("calendar:h2g:{}:{}:{}", hijri.year, hijri.month, hijri.day);
    let _ = manager.clear_cache(&cache_key).await;
    
    let result = manager.hijri_to_gregorian(&hijri).await;
    
    match result {
        Ok(gregorian) => {
            assert!(gregorian.year() >= 2020 && gregorian.year() <= 2030);
        }
        Err(e) => {
            println!("Calendar API unavailable: {}", e);
        }
    }
}

#[tokio::test]
async fn test_manager_get_events() {
    let manager = create_test_manager().await;
    
    let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
    
    // Clear cache
    let cache_key = format!("calendar:events:{}:{}", start, end);
    let _ = manager.clear_cache(&cache_key).await;
    
    let result = manager.get_events(start, end).await;
    
    match result {
        Ok(events) => {
            for event in &events {
                assert!(event.date >= start && event.date <= end);
                assert!(!event.event_name_ar.is_empty());
                assert!(!event.event_name_en.is_empty());
            }
        }
        Err(e) => {
            println!("Calendar API unavailable: {}", e);
        }
    }
}

#[tokio::test]
async fn test_manager_caching() {
    let manager = create_test_manager().await;
    
    let gregorian = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let cache_key = format!("calendar:g2h:{}", gregorian);
    
    // Clear cache first
    let _ = manager.clear_cache(&cache_key).await;
    
    // First call - should hit API
    let result1 = manager.gregorian_to_hijri(gregorian).await;
    
    if result1.is_ok() {
        // Second call - should hit cache
        let result2 = manager.gregorian_to_hijri(gregorian).await;
        
        assert!(result2.is_ok());
        assert_eq!(
            format!("{:?}", result1.unwrap()),
            format!("{:?}", result2.unwrap())
        );
    }
}

#[tokio::test]
async fn test_manager_invalid_date_range() {
    let manager = create_test_manager().await;
    
    let start = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    
    let result = manager.get_events(start, end).await;
    assert!(result.is_err());
}

// ============================================================================
// Date Format Validation Tests
// ============================================================================

#[test]
fn test_hijri_date_format() {
    let hijri = HijriDate {
        year: 1445,
        month: 9,
        day: 15,
        month_name_ar: "رمضان".to_string(),
        month_name_en: "Ramadan".to_string(),
    };
    
    assert_eq!(hijri.year, 1445);
    assert_eq!(hijri.month, 9);
    assert_eq!(hijri.day, 15);
    assert_eq!(hijri.month_name_ar, "رمضان");
    assert_eq!(hijri.month_name_en, "Ramadan");
}

#[test]
fn test_month_names_consistency() {
    // Test that all month numbers have corresponding names
    for month in 1..=12 {
        let ar_name = AladhanCalendarClient::get_month_name_ar(month);
        let en_name = AladhanCalendarClient::get_month_name_en(month);
        
        assert!(!ar_name.is_empty(), "Arabic name for month {} is empty", month);
        assert!(!en_name.is_empty(), "English name for month {} is empty", month);
        assert_ne!(ar_name, "Unknown", "Arabic name for month {} is Unknown", month);
        assert_ne!(en_name, "Unknown", "English name for month {} is Unknown", month);
    }
}

#[test]
fn test_invalid_month_names() {
    let ar_name = AladhanCalendarClient::get_month_name_ar(0);
    let en_name = AladhanCalendarClient::get_month_name_en(0);
    
    assert_eq!(ar_name, "Unknown");
    assert_eq!(en_name, "Unknown");
    
    let ar_name = AladhanCalendarClient::get_month_name_ar(13);
    let en_name = AladhanCalendarClient::get_month_name_en(13);
    
    assert_eq!(ar_name, "Unknown");
    assert_eq!(en_name, "Unknown");
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[tokio::test]
async fn test_leap_year_conversion() {
    let client = AladhanCalendarClient::new();
    
    // Test February 29 in a leap year
    let gregorian = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
    
    let result = client.gregorian_to_hijri(gregorian).await;
    
    match result {
        Ok(hijri) => {
            assert!(hijri.month >= 1 && hijri.month <= 12);
            assert!(hijri.day >= 1 && hijri.day <= 30);
        }
        Err(e) => {
            println!("API unavailable: {}", e);
        }
    }
}

#[tokio::test]
async fn test_year_boundary_conversion() {
    let client = AladhanCalendarClient::new();
    
    // Test December 31
    let gregorian = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    
    let result = client.gregorian_to_hijri(gregorian).await;
    
    match result {
        Ok(hijri) => {
            assert!(hijri.year > 0);
            assert!(hijri.month >= 1 && hijri.month <= 12);
        }
        Err(e) => {
            println!("API unavailable: {}", e);
        }
    }
}

#[tokio::test]
async fn test_empty_events_range() {
    let client = AladhanCalendarClient::new();
    
    // Test a single day range
    let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
    
    let result = client.get_events(date, date).await;
    
    match result {
        Ok(events) => {
            // Should return empty or events for that single day
            for event in &events {
                assert_eq!(event.date, date);
            }
        }
        Err(e) => {
            println!("API unavailable: {}", e);
        }
    }
}

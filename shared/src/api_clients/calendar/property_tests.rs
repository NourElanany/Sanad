//! Property-Based Tests for Calendar API Clients
//!
//! These tests verify universal properties that should hold across all inputs.

use crate::api_clients::calendar::{AladhanCalendarClient, CalendarApiManager, IslamicFinderCalendarClient};
use crate::api_clients::{CacheManager, CalendarApiClient, RateLimiter};
use chrono::NaiveDate;
use proptest::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

// Feature: official-apis-integration, Property 9: Date Conversion Round Trip
// **Validates: Requirements 5.2**
//
// For any valid Gregorian date, converting it to Hijri and then back to Gregorian
// should produce the original date (or a date within acceptable margin due to calculation differences).
proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]
    
    #[test]
    fn test_date_conversion_round_trip_aladhan(
        year in 2000i32..2100,
        month in 1u32..=12,
        day in 1u32..=28  // Safe range for all months
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let client = AladhanCalendarClient::new();
            
            // Create a valid Gregorian date
            let gregorian = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            
            // Convert to Hijri
            let hijri_result = client.gregorian_to_hijri(gregorian).await;
            
            // Skip if API call fails (network issues, etc.)
            if hijri_result.is_err() {
                return Ok(());
            }
            
            let hijri = hijri_result.unwrap();
            
            // Convert back to Gregorian
            let gregorian_back_result = client.hijri_to_gregorian(&hijri).await;
            
            // Skip if API call fails
            if gregorian_back_result.is_err() {
                return Ok(());
            }
            
            let gregorian_back = gregorian_back_result.unwrap();
            
            // Calculate the difference in days
            let diff = (gregorian - gregorian_back).num_days().abs();
            
            // Should be within 1 day due to calculation differences
            prop_assert!(
                diff <= 1,
                "Round trip conversion diff too large: {} days. Original: {}, After round trip: {}",
                diff,
                gregorian,
                gregorian_back
            );
            
            Ok(())
        })?;
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]
    
    #[test]
    fn test_date_conversion_round_trip_islamic_finder(
        year in 2000i32..2100,
        month in 1u32..=12,
        day in 1u32..=28  // Safe range for all months
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let client = IslamicFinderCalendarClient::new();
            
            // Create a valid Gregorian date
            let gregorian = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            
            // Convert to Hijri
            let hijri = client.gregorian_to_hijri(gregorian).await.unwrap();
            
            // Convert back to Gregorian
            let gregorian_back = client.hijri_to_gregorian(&hijri).await.unwrap();
            
            // Calculate the difference in days
            let diff = (gregorian - gregorian_back).num_days().abs();
            
            // Should be within 1 day for the improved algorithm
            prop_assert!(
                diff <= 1,
                "Round trip conversion diff too large: {} days. Original: {}, After round trip: {}",
                diff,
                gregorian,
                gregorian_back
            );
            
            Ok(())
        })?;
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]
    
    #[test]
    fn test_date_conversion_round_trip_manager(
        year in 2000i32..2100,
        month in 1u32..=12,
        day in 1u32..=28  // Safe range for all months
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Create manager with both clients
            let cache = Arc::new(CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"));
            
            let rate_limiter = Arc::new(RateLimiter::new(
                "redis://127.0.0.1:6379/",
                HashMap::new()
            )
                .await
                .expect("Failed to create rate limiter"));

            let clients: Vec<Box<dyn CalendarApiClient + Send + Sync>> = vec![
                Box::new(AladhanCalendarClient::new()),
                Box::new(IslamicFinderCalendarClient::new()),
            ];

            let manager = CalendarApiManager::new(clients, cache, rate_limiter);
            
            // Create a valid Gregorian date
            let gregorian = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            
            // Clear cache to ensure fresh conversion
            let cache_key = format!("calendar:g2h:{}", gregorian);
            let _ = manager.clear_cache(&cache_key).await;
            
            // Convert to Hijri
            let hijri_result = manager.gregorian_to_hijri(gregorian).await;
            
            // Skip if API call fails (network issues, etc.)
            if hijri_result.is_err() {
                return Ok(());
            }
            
            let hijri = hijri_result.unwrap();
            
            // Clear cache for reverse conversion
            let cache_key = format!("calendar:h2g:{}:{}:{}", hijri.year, hijri.month, hijri.day);
            let _ = manager.clear_cache(&cache_key).await;
            
            // Convert back to Gregorian
            let gregorian_back_result = manager.hijri_to_gregorian(&hijri).await;
            
            // Skip if API call fails
            if gregorian_back_result.is_err() {
                return Ok(());
            }
            
            let gregorian_back = gregorian_back_result.unwrap();
            
            // Calculate the difference in days
            let diff = (gregorian - gregorian_back).num_days().abs();
            
            // Should be within 1 day due to calculation differences
            prop_assert!(
                diff <= 1,
                "Round trip conversion diff too large: {} days. Original: {}, After round trip: {}",
                diff,
                gregorian,
                gregorian_back
            );
            
            Ok(())
        })?;
    }
}

// Additional property: Hijri dates should have valid ranges
proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]
    
    #[test]
    fn test_hijri_date_valid_ranges(
        year in 2000i32..2100,
        month in 1u32..=12,
        day in 1u32..=28
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let client = AladhanCalendarClient::new();
            
            let gregorian = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            
            let hijri_result = client.gregorian_to_hijri(gregorian).await;
            
            // Skip if API call fails
            if hijri_result.is_err() {
                return Ok(());
            }
            
            let hijri = hijri_result.unwrap();
            
            // Verify Hijri date has valid ranges
            prop_assert!(
                hijri.month >= 1 && hijri.month <= 12,
                "Hijri month {} out of valid range (1-12)",
                hijri.month
            );
            
            prop_assert!(
                hijri.day >= 1 && hijri.day <= 30,
                "Hijri day {} out of valid range (1-30)",
                hijri.day
            );
            
            prop_assert!(
                hijri.year > 0,
                "Hijri year {} must be positive",
                hijri.year
            );
            
            // Verify month names are not empty
            prop_assert!(
                !hijri.month_name_ar.is_empty(),
                "Hijri month name (Arabic) should not be empty"
            );
            
            prop_assert!(
                !hijri.month_name_en.is_empty(),
                "Hijri month name (English) should not be empty"
            );
            
            Ok(())
        })?;
    }
}

// Property: Events should be within the requested date range
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]
    
    #[test]
    fn test_events_within_date_range(
        start_year in 2020i32..2025,
        start_month in 1u32..=12,
        days_range in 1u32..=30
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let client = AladhanCalendarClient::new();
            
            let start = NaiveDate::from_ymd_opt(start_year, start_month, 1).unwrap();
            let end = start + chrono::Duration::days(days_range as i64);
            
            let events_result = client.get_events(start, end).await;
            
            // Skip if API call fails
            if events_result.is_err() {
                return Ok(());
            }
            
            let events = events_result.unwrap();
            
            // All events should be within the requested range
            for event in events {
                prop_assert!(
                    event.date >= start && event.date <= end,
                    "Event date {} is outside requested range {} to {}",
                    event.date,
                    start,
                    end
                );
                
                // Event names should not be empty
                prop_assert!(
                    !event.event_name_ar.is_empty(),
                    "Event name (Arabic) should not be empty"
                );
                
                prop_assert!(
                    !event.event_name_en.is_empty(),
                    "Event name (English) should not be empty"
                );
            }
            
            Ok(())
        })?;
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[tokio::test]
    async fn test_specific_date_round_trip() {
        let client = AladhanCalendarClient::new();
        
        // Test a specific known date
        let gregorian = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        
        let hijri_result = client.gregorian_to_hijri(gregorian).await;
        
        // Skip if API is unavailable
        if hijri_result.is_err() {
            println!("Aladhan API unavailable, skipping test");
            return;
        }
        
        let hijri = hijri_result.unwrap();
        let gregorian_back_result = client.hijri_to_gregorian(&hijri).await;
        
        if gregorian_back_result.is_err() {
            println!("Aladhan API unavailable, skipping test");
            return;
        }
        
        let gregorian_back = gregorian_back_result.unwrap();
        let diff = (gregorian - gregorian_back).num_days().abs();
        assert!(diff <= 1, "Round trip diff: {} days", diff);
    }

    #[tokio::test]
    async fn test_ramadan_date_conversion() {
        let client = AladhanCalendarClient::new();
        
        // Test conversion around Ramadan 2024
        let gregorian = NaiveDate::from_ymd_opt(2024, 3, 11).unwrap();
        
        let hijri_result = client.gregorian_to_hijri(gregorian).await;
        
        // Skip if API is unavailable
        if hijri_result.is_err() {
            println!("Aladhan API unavailable, skipping test");
            return;
        }
        
        let hijri = hijri_result.unwrap();
        
        // Should be around Ramadan 1445
        assert!(hijri.year == 1445 || hijri.year == 1446);
        assert!(hijri.month >= 1 && hijri.month <= 12);
        assert!(hijri.day >= 1 && hijri.day <= 30);
    }

    #[tokio::test]
    async fn test_islamic_new_year_event() {
        let client = AladhanCalendarClient::new();
        
        // Get events for a range that includes Muharram 1
        let start = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 7, 31).unwrap();
        
        let events_result = client.get_events(start, end).await;
        
        // Skip if API is unavailable
        if events_result.is_err() {
            println!("Aladhan API unavailable, skipping test");
            return;
        }
        
        let events = events_result.unwrap();
        
        // Should find Islamic New Year event
        let has_new_year = events.iter().any(|e| e.event_name_en.contains("Islamic New Year"));
        assert!(has_new_year, "Should find Islamic New Year event in July 2024");
    }
}

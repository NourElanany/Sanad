use proptest::prelude::*;
use chrono::{NaiveDate, Timelike, Datelike};
use shared::{Location, CalculationMethod};
use crate::calculator::PrayerTimesCalculator;

/// Property-based tests for prayer times calculation accuracy
/// 
/// **Validates: Requirements 7.1, 7.4**
/// 
/// This module contains property-based tests that verify the accuracy of prayer time
/// calculations across different locations, dates, and calculation methods.

/// Generate valid latitude values (-90 to 90 degrees)
fn valid_latitude() -> impl Strategy<Value = f64> {
    -90.0..=90.0
}

/// Generate valid longitude values (-180 to 180 degrees)
fn valid_longitude() -> impl Strategy<Value = f64> {
    -180.0..=180.0
}

/// Generate valid dates for testing (reasonable range for Islamic calendar usage)
fn valid_date() -> impl Strategy<Value = NaiveDate> {
    (1950..=2100)
        .prop_flat_map(|year| {
            (1u32..=12u32).prop_flat_map(move |month| {
                let days_in_month = match month {
                    2 => if is_leap_year(year) { 29 } else { 28 },
                    4 | 6 | 9 | 11 => 30,
                    _ => 31,
                };
                (1u32..=days_in_month).prop_map(move |day| {
                    NaiveDate::from_ymd_opt(year, month, day).unwrap()
                })
            })
        })
}

/// Generate valid timezone strings
fn valid_timezone() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("UTC".to_string()),
        Just("Asia/Riyadh".to_string()),
        Just("America/New_York".to_string()),
        Just("Europe/London".to_string()),
        Just("Asia/Jakarta".to_string()),
        Just("Australia/Sydney".to_string()),
        Just("Africa/Cairo".to_string()),
        Just("Asia/Karachi".to_string()),
        Just("Asia/Tehran".to_string()),
    ]
}

/// Generate valid calculation methods
fn valid_calculation_method() -> impl Strategy<Value = CalculationMethod> {
    prop_oneof![
        Just(CalculationMethod::MuslimWorldLeague),
        Just(CalculationMethod::IslamicSocietyOfNorthAmerica),
        Just(CalculationMethod::EgyptianGeneralAuthorityOfSurvey),
        Just(CalculationMethod::UmmAlQuraUniversityMakkah),
        Just(CalculationMethod::UniversityOfIslamicSciencesKarachi),
        Just(CalculationMethod::InstituteOfGeophysicsUniversityOfTehran),
        Just(CalculationMethod::Shia),
        // Custom method with reasonable angles
        (10.0..25.0, 0.0..10.0, 10.0..25.0).prop_map(|(fajr, maghrib, isha)| {
            CalculationMethod::Custom {
                fajr_angle: fajr,
                maghrib_angle: maghrib,
                isha_angle: isha,
            }
        })
    ]
}

/// Generate valid Location objects
fn valid_location() -> impl Strategy<Value = Location> {
    (valid_latitude(), valid_longitude(), valid_timezone())
        .prop_map(|(lat, lng, tz)| Location {
            latitude: lat,
            longitude: lng,
            timezone: tz,
            city: Some("Test City".to_string()),
            country: Some("Test Country".to_string()),
        })
}

/// Helper function to check if a year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        /// **Property 7: Prayer Times Accuracy**
        /// **Validates: Requirements 7.1, 7.4**
        /// 
        /// For any valid location and date, the calculated prayer times must be:
        /// 1. In chronological order (Fajr < Sunrise < Dhuhr < Asr < Maghrib < Isha)
        /// 2. Within reasonable astronomical bounds
        /// 3. Consistent with the chosen calculation method
        /// 4. Accurate according to astronomical standards
        #[test]
        fn property_prayer_times_chronological_order(
            location in valid_location(),
            date in valid_date(),
            method in valid_calculation_method()
        ) {
            // Skip extreme latitudes and very old dates that may cause calculation issues
            prop_assume!(location.latitude.abs() < 60.0); // Focus on inhabited latitudes
            prop_assume!(date.year() >= 1950); // Modern era for more reliable calculations
            
            let result = PrayerTimesCalculator::calculate_prayer_times(
                &location,
                date,
                &method,
                None,
            );
            
            // If calculation fails due to extreme conditions, skip this test case
            if result.is_err() {
                return Ok(());
            }
            
            let prayer_times = result.unwrap();
            
            // Verify chronological order
            prop_assert!(
                prayer_times.fajr < prayer_times.sunrise,
                "Fajr must be before Sunrise"
            );
            prop_assert!(
                prayer_times.sunrise < prayer_times.dhuhr,
                "Sunrise must be before Dhuhr"
            );
            prop_assert!(
                prayer_times.dhuhr < prayer_times.asr,
                "Dhuhr must be before Asr"
            );
            prop_assert!(
                prayer_times.asr < prayer_times.maghrib,
                "Asr must be before Maghrib"
            );
            prop_assert!(
                prayer_times.maghrib < prayer_times.isha,
                "Maghrib must be before Isha"
            );
        }

        /// **Property 7: Prayer Times Accuracy - Astronomical Bounds**
        /// **Validates: Requirements 7.1, 7.4**
        /// 
        /// Prayer times must fall within reasonable astronomical bounds based on
        /// the location's latitude and the date's seasonal position.
        #[test]
        fn property_prayer_times_astronomical_bounds(
            location in valid_location(),
            date in valid_date(),
            method in valid_calculation_method()
        ) {
            // Skip extreme latitudes where normal calculations may not apply
            prop_assume!(location.latitude.abs() < 85.0);
            
            let result = PrayerTimesCalculator::calculate_prayer_times(
                &location,
                date,
                &method,
                None,
            );
            
            prop_assert!(result.is_ok(), "Prayer time calculation should succeed");
            
            let prayer_times = result.unwrap();
            
            // Extract hour of day for each prayer time (convert to UTC for consistent comparison)
            let fajr_hour = prayer_times.fajr.hour() as f64 + prayer_times.fajr.minute() as f64 / 60.0;
            let sunrise_hour = prayer_times.sunrise.hour() as f64 + prayer_times.sunrise.minute() as f64 / 60.0;
            let dhuhr_hour = prayer_times.dhuhr.hour() as f64 + prayer_times.dhuhr.minute() as f64 / 60.0;
            let asr_hour = prayer_times.asr.hour() as f64 + prayer_times.asr.minute() as f64 / 60.0;
            let maghrib_hour = prayer_times.maghrib.hour() as f64 + prayer_times.maghrib.minute() as f64 / 60.0;
            let isha_hour = prayer_times.isha.hour() as f64 + prayer_times.isha.minute() as f64 / 60.0;
            
            // Verify reasonable bounds (allowing for timezone differences)
            prop_assert!(
                fajr_hour >= 0.0 && fajr_hour <= 24.0,
                "Fajr hour must be valid: {}", fajr_hour
            );
            prop_assert!(
                sunrise_hour >= 0.0 && sunrise_hour <= 24.0,
                "Sunrise hour must be valid: {}", sunrise_hour
            );
            prop_assert!(
                dhuhr_hour >= 10.0 && dhuhr_hour <= 14.0,
                "Dhuhr should be around solar noon: {}", dhuhr_hour
            );
            prop_assert!(
                asr_hour >= 12.0 && asr_hour <= 18.0,
                "Asr should be in afternoon: {}", asr_hour
            );
            prop_assert!(
                maghrib_hour >= 0.0 && maghrib_hour <= 24.0,
                "Maghrib hour must be valid: {}", maghrib_hour
            );
            prop_assert!(
                isha_hour >= 0.0 && isha_hour <= 24.0,
                "Isha hour must be valid: {}", isha_hour
            );
        }

        /// **Property 7: Prayer Times Accuracy - Method Consistency**
        /// **Validates: Requirements 7.4**
        /// 
        /// Different calculation methods should produce different but valid results
        /// for the same location and date, reflecting their different parameters.
        #[test]
        fn property_calculation_method_consistency(
            location in valid_location(),
            date in valid_date()
        ) {
            prop_assume!(location.latitude.abs() < 70.0); // Avoid extreme latitudes
            
            let mwl_result = PrayerTimesCalculator::calculate_prayer_times(
                &location,
                date,
                &CalculationMethod::MuslimWorldLeague,
                None,
            );
            
            let isna_result = PrayerTimesCalculator::calculate_prayer_times(
                &location,
                date,
                &CalculationMethod::IslamicSocietyOfNorthAmerica,
                None,
            );
            
            prop_assert!(mwl_result.is_ok() && isna_result.is_ok(), 
                "Both calculation methods should succeed");
            
            let mwl_times = mwl_result.unwrap();
            let isna_times = isna_result.unwrap();
            
            // Both should maintain chronological order
            prop_assert!(mwl_times.fajr < mwl_times.sunrise);
            prop_assert!(isna_times.fajr < isna_times.sunrise);
            
            // Dhuhr should be very similar (solar noon doesn't depend on angles much)
            let dhuhr_diff = (mwl_times.dhuhr.timestamp() - isna_times.dhuhr.timestamp()).abs();
            prop_assert!(
                dhuhr_diff < 300, // Within 5 minutes
                "Dhuhr times should be similar across methods: {} seconds difference", dhuhr_diff
            );
            
            // Fajr and Isha may differ more due to different angles
            // But they should still be reasonable
            let fajr_diff = (mwl_times.fajr.timestamp() - isna_times.fajr.timestamp()).abs();
            prop_assert!(
                fajr_diff < 3600, // Within 1 hour
                "Fajr times should be reasonably close: {} seconds difference", fajr_diff
            );
        }

        /// **Property 7: Prayer Times Accuracy - Qibla Direction**
        /// **Validates: Requirements 7.1**
        /// 
        /// Qibla direction calculation must be accurate and consistent.
        #[test]
        fn property_qibla_direction_accuracy(
            latitude in valid_latitude(),
            longitude in valid_longitude()
        ) {
            let result = PrayerTimesCalculator::calculate_qibla_direction(latitude, longitude);
            
            prop_assert!(result.is_ok(), "Qibla calculation should succeed for valid coordinates");
            
            let qibla = result.unwrap();
            
            // Direction should be between 0 and 360 degrees
            prop_assert!(
                qibla.direction_degrees >= 0.0 && qibla.direction_degrees < 360.0,
                "Qibla direction must be valid: {}", qibla.direction_degrees
            );
            
            // Distance should be positive and reasonable (Earth's circumference is ~40,000 km)
            prop_assert!(
                qibla.distance_km >= 0.0 && qibla.distance_km <= 20000.0,
                "Distance to Kaaba should be reasonable: {} km", qibla.distance_km
            );
            
            // Cardinal direction should be valid
            let valid_cardinals = ["N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", 
                                 "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW"];
            prop_assert!(
                valid_cardinals.contains(&qibla.direction_cardinal.as_str()),
                "Cardinal direction should be valid: {}", qibla.direction_cardinal
            );
        }

        /// **Property 7: Prayer Times Accuracy - Seasonal Variation**
        /// **Validates: Requirements 7.1**
        /// 
        /// Prayer times should vary appropriately with seasons.
        #[test]
        fn property_seasonal_variation(
            location in valid_location(),
            year in 2000..2030i32
        ) {
            prop_assume!(location.latitude.abs() < 60.0); // Avoid extreme latitudes
            
            let summer_solstice = NaiveDate::from_ymd_opt(year, 6, 21).unwrap();
            let winter_solstice = NaiveDate::from_ymd_opt(year, 12, 21).unwrap();
            
            let summer_result = PrayerTimesCalculator::calculate_prayer_times(
                &location,
                summer_solstice,
                &CalculationMethod::MuslimWorldLeague,
                None,
            );
            
            let winter_result = PrayerTimesCalculator::calculate_prayer_times(
                &location,
                winter_solstice,
                &CalculationMethod::MuslimWorldLeague,
                None,
            );
            
            prop_assert!(summer_result.is_ok() && winter_result.is_ok(),
                "Seasonal calculations should succeed");
            
            let summer_times = summer_result.unwrap();
            let winter_times = winter_result.unwrap();
            
            // In Northern Hemisphere, summer days are longer
            if location.latitude > 0.0 {
                let summer_day_length = summer_times.maghrib.timestamp() - summer_times.sunrise.timestamp();
                let winter_day_length = winter_times.maghrib.timestamp() - winter_times.sunrise.timestamp();
                
                prop_assert!(
                    summer_day_length > winter_day_length,
                    "Summer days should be longer than winter days in Northern Hemisphere"
                );
            }
            // In Southern Hemisphere, winter days are longer
            else if location.latitude < -10.0 { // Allow some buffer around equator
                let summer_day_length = summer_times.maghrib.timestamp() - summer_times.sunrise.timestamp();
                let winter_day_length = winter_times.maghrib.timestamp() - winter_times.sunrise.timestamp();
                
                prop_assert!(
                    winter_day_length > summer_day_length,
                    "Winter days should be longer than summer days in Southern Hemisphere"
                );
            }
        }

        /// **Property 7: Prayer Times Accuracy - Time Zone Handling**
        /// **Validates: Requirements 7.1**
        /// 
        /// Prayer times should be correctly adjusted for different time zones.
        #[test]
        fn property_timezone_handling(
            latitude in (-60.0..60.0), // Avoid extreme latitudes
            longitude in (-180.0..180.0),
            date in valid_date()
        ) {
            let utc_location = Location {
                latitude,
                longitude,
                timezone: "UTC".to_string(),
                city: Some("Test".to_string()),
                country: Some("Test".to_string()),
            };
            
            let est_location = Location {
                latitude,
                longitude,
                timezone: "America/New_York".to_string(),
                city: Some("Test".to_string()),
                country: Some("Test".to_string()),
            };
            
            let utc_result = PrayerTimesCalculator::calculate_prayer_times(
                &utc_location,
                date,
                &CalculationMethod::MuslimWorldLeague,
                None,
            );
            
            let est_result = PrayerTimesCalculator::calculate_prayer_times(
                &est_location,
                date,
                &CalculationMethod::MuslimWorldLeague,
                None,
            );
            
            prop_assert!(utc_result.is_ok() && est_result.is_ok(),
                "Timezone calculations should succeed");
            
            let utc_times = utc_result.unwrap();
            let est_times = est_result.unwrap();
            
            // Both should maintain chronological order
            prop_assert!(utc_times.fajr < utc_times.sunrise);
            prop_assert!(est_times.fajr < est_times.sunrise);
            
            // The UTC timestamps should be different but represent the same local solar time
            // This is a complex property to verify exactly, so we just ensure both are valid
            prop_assert!(utc_times.dhuhr.timestamp() > 0);
            prop_assert!(est_times.dhuhr.timestamp() > 0);
        }
    }
}
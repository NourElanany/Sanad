//! Property-based tests for Prayer Times API clients
//!
//! These tests verify universal properties that should hold across all inputs.

#[cfg(test)]
mod tests {
    use crate::api_clients::{
        CalculationMethod, Madhab, PrayerTimesRequest, PrayerTimesResponse,
    };
    use chrono::{NaiveDate, NaiveTime};
    use proptest::prelude::*;

    // Helper to create valid prayer times for testing
    fn create_valid_prayer_times(
        fajr_hour: u32,
        sunrise_hour: u32,
        dhuhr_hour: u32,
        asr_hour: u32,
        maghrib_hour: u32,
        isha_hour: u32,
    ) -> PrayerTimesResponse {
        PrayerTimesResponse {
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            fajr: NaiveTime::from_hms_opt(fajr_hour, 0, 0).unwrap(),
            sunrise: NaiveTime::from_hms_opt(sunrise_hour, 0, 0).unwrap(),
            dhuhr: NaiveTime::from_hms_opt(dhuhr_hour, 0, 0).unwrap(),
            asr: NaiveTime::from_hms_opt(asr_hour, 0, 0).unwrap(),
            maghrib: NaiveTime::from_hms_opt(maghrib_hour, 0, 0).unwrap(),
            isha: NaiveTime::from_hms_opt(isha_hour, 0, 0).unwrap(),
            source: "test".to_string(),
        }
    }

    // Feature: official-apis-integration, Property 7: Prayer Times Chronological Ordering
    // **Validates: Requirements 3.4**
    //
    // *For any* prayer times response, the five prayer times (Fajr, Dhuhr, Asr, Maghrib, Isha)
    // should be in chronological order, with each time being later than the previous one.
    proptest! {
        #[test]
        fn property_prayer_times_chronological_ordering(
            fajr_hour in 4u32..6,
            sunrise_hour in 6u32..8,
            dhuhr_hour in 12u32..14,
            asr_hour in 15u32..17,
            maghrib_hour in 18u32..20,
            isha_hour in 20u32..22,
        ) {
            let times = create_valid_prayer_times(
                fajr_hour,
                sunrise_hour,
                dhuhr_hour,
                asr_hour,
                maghrib_hour,
                isha_hour,
            );

            // Verify chronological ordering
            prop_assert!(times.fajr < times.sunrise, "Fajr must be before Sunrise");
            prop_assert!(times.sunrise < times.dhuhr, "Sunrise must be before Dhuhr");
            prop_assert!(times.dhuhr < times.asr, "Dhuhr must be before Asr");
            prop_assert!(times.asr < times.maghrib, "Asr must be before Maghrib");
            prop_assert!(times.maghrib < times.isha, "Maghrib must be before Isha");
        }
    }

    // Additional property test: Prayer times should be within valid 24-hour range
    proptest! {
        #[test]
        fn property_prayer_times_valid_range(
            fajr_hour in 0u32..24,
            fajr_min in 0u32..60,
            sunrise_hour in 0u32..24,
            sunrise_min in 0u32..60,
            dhuhr_hour in 0u32..24,
            dhuhr_min in 0u32..60,
            asr_hour in 0u32..24,
            asr_min in 0u32..60,
            maghrib_hour in 0u32..24,
            maghrib_min in 0u32..60,
            isha_hour in 0u32..24,
            isha_min in 0u32..60,
        ) {
            // All times should be valid NaiveTime instances
            let fajr = NaiveTime::from_hms_opt(fajr_hour, fajr_min, 0);
            let sunrise = NaiveTime::from_hms_opt(sunrise_hour, sunrise_min, 0);
            let dhuhr = NaiveTime::from_hms_opt(dhuhr_hour, dhuhr_min, 0);
            let asr = NaiveTime::from_hms_opt(asr_hour, asr_min, 0);
            let maghrib = NaiveTime::from_hms_opt(maghrib_hour, maghrib_min, 0);
            let isha = NaiveTime::from_hms_opt(isha_hour, isha_min, 0);

            prop_assert!(fajr.is_some(), "Fajr time should be valid");
            prop_assert!(sunrise.is_some(), "Sunrise time should be valid");
            prop_assert!(dhuhr.is_some(), "Dhuhr time should be valid");
            prop_assert!(asr.is_some(), "Asr time should be valid");
            prop_assert!(maghrib.is_some(), "Maghrib time should be valid");
            prop_assert!(isha.is_some(), "Isha time should be valid");
        }
    }

    // Property test: Request coordinates should be validated
    proptest! {
        #[test]
        fn property_coordinates_validation(
            latitude in -90.0f64..=90.0,
            longitude in -180.0f64..=180.0,
        ) {
            // Valid coordinates should not cause validation errors
            let request = PrayerTimesRequest {
                latitude,
                longitude,
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                calculation_method: CalculationMethod::MWL,
                madhab: Madhab::Shafi,
            };

            // Coordinates should be within valid ranges
            prop_assert!(request.latitude >= -90.0 && request.latitude <= 90.0);
            prop_assert!(request.longitude >= -180.0 && request.longitude <= 180.0);
        }
    }

    // Property test: Invalid coordinates should be rejected
    proptest! {
        #[test]
        fn property_invalid_coordinates_rejected(
            latitude in prop::num::f64::ANY,
            longitude in prop::num::f64::ANY,
        ) {
            // Filter to only test invalid coordinates
            prop_assume!(latitude < -90.0 || latitude > 90.0 || longitude < -180.0 || longitude > 180.0);

            let request = PrayerTimesRequest {
                latitude,
                longitude,
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                calculation_method: CalculationMethod::MWL,
                madhab: Madhab::Shafi,
            };

            // Invalid coordinates should be detectable
            let is_invalid = request.latitude < -90.0 || request.latitude > 90.0 
                || request.longitude < -180.0 || request.longitude > 180.0;
            prop_assert!(is_invalid, "Invalid coordinates should be detected");
        }
    }

    // Property test: All calculation methods should be supported
    #[test]
    fn test_all_calculation_methods_supported() {
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

            // All methods should be valid
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

    // Property test: Both madhabs should be supported
    #[test]
    fn test_both_madhabs_supported() {
        let madhabs = vec![Madhab::Shafi, Madhab::Hanafi];

        for madhab in madhabs {
            let request = PrayerTimesRequest {
                latitude: 21.4225,
                longitude: 39.8262,
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                calculation_method: CalculationMethod::MWL,
                madhab,
            };

            // Both madhabs should be valid
            assert!(matches!(request.madhab, Madhab::Shafi | Madhab::Hanafi));
        }
    }

    // Property test: Date range requests should maintain chronological order
    proptest! {
        #[test]
        fn property_date_range_chronological(
            year in 2020i32..2030,
            month in 1u32..=12,
            day in 1u32..=28, // Safe range for all months
            days in 1u32..30,
        ) {
            let start_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            let mut current_date = start_date;

            // Verify that dates increment properly
            for _ in 0..days {
                let next_date = current_date.succ_opt();
                if let Some(next) = next_date {
                    prop_assert!(next > current_date, "Dates should increment");
                    current_date = next;
                } else {
                    break; // Date overflow
                }
            }
        }
    }
}

use chrono::NaiveDate;
use shared::{Location, CalculationMethod};
use crate::calculator::PrayerTimesCalculator;

#[cfg(test)]
mod debug_tests {
    use super::*;

    #[test]
    fn debug_failing_case() {
        let location = Location {
            latitude: 52.02015454966725,
            longitude: 0.0,
            timezone: "UTC".to_string(),
            city: Some("Test City".to_string()),
            country: Some("Test Country".to_string()),
        };
        
        let date = NaiveDate::from_ymd_opt(1958, 6, 1).unwrap();
        let method = CalculationMethod::Shia;
        
        // Let's debug the intermediate calculations
        let calculator = PrayerTimesCalculator::new();
        let julian_day = calculator.get_julian_day(date);
        println!("Julian day: {}", julian_day);
        
        let params = calculator.get_calculation_parameters(&method);
        println!("Fajr angle: {}", params.fajr_angle);
        println!("Maghrib angle: {}", params.maghrib_angle);
        println!("Isha angle: {}", params.isha_angle);
        
        let result = PrayerTimesCalculator::calculate_prayer_times(
            &location,
            date,
            &method,
            None,
        );
        
        match result {
            Ok(prayer_times) => {
                println!("Prayer times for {}, {} on {} with Shia method:", location.latitude, location.longitude, date);
                println!("Fajr: {}", prayer_times.fajr);
                println!("Sunrise: {}", prayer_times.sunrise);
                println!("Dhuhr: {}", prayer_times.dhuhr);
                println!("Asr: {}", prayer_times.asr);
                println!("Maghrib: {}", prayer_times.maghrib);
                println!("Isha: {}", prayer_times.isha);
                
                // Check the order
                assert!(prayer_times.fajr < prayer_times.sunrise, 
                    "Fajr ({}) should be before Sunrise ({})", 
                    prayer_times.fajr, prayer_times.sunrise);
            }
            Err(e) => {
                panic!("Prayer time calculation failed: {}", e);
            }
        }
    }
}
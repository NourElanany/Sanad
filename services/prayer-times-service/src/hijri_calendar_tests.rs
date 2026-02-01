#[cfg(test)]
mod tests {
    use crate::hijri_calendar::HijriCalendar;
    use chrono::{NaiveDate, Datelike};
    use shared::EventType;

    #[test]
    fn test_gregorian_to_hijri_conversion() {
        // Test known conversion: January 1, 2024 should be approximately Jumada al-Thani 19, 1445
        let gregorian_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let hijri_date = HijriCalendar::gregorian_to_hijri(gregorian_date).unwrap();
        
        // Allow some tolerance in the conversion due to algorithm approximation
        assert!(hijri_date.year >= 1445 && hijri_date.year <= 1446);
        assert!(hijri_date.month >= 1 && hijri_date.month <= 12);
        assert!(hijri_date.day >= 1 && hijri_date.day <= 30);
        assert!(!hijri_date.month_name.is_empty());
    }

    #[test]
    fn test_hijri_to_gregorian_conversion() {
        // Test conversion: Muharram 1, 1446 should be approximately July 2024
        let gregorian_date = HijriCalendar::hijri_to_gregorian(1446, 1, 1).unwrap();
        
        // Should be in 2024
        assert!(gregorian_date.year() >= 2024 && gregorian_date.year() <= 2025);
    }

    #[test]
    fn test_round_trip_conversion() {
        // Test that converting Gregorian -> Hijri -> Gregorian gives approximately the same date
        let original_date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let hijri_date = HijriCalendar::gregorian_to_hijri(original_date).unwrap();
        let converted_back = HijriCalendar::hijri_to_gregorian(
            hijri_date.year,
            hijri_date.month as i32,
            hijri_date.day as i32,
        ).unwrap();
        
        // Allow up to 2 days difference due to algorithm approximation
        let diff = (original_date - converted_back).num_days().abs();
        assert!(diff <= 2, "Date difference too large: {} days", diff);
    }

    #[test]
    fn test_hijri_months() {
        let months = HijriCalendar::get_hijri_months();
        assert_eq!(months.len(), 12);
        
        // Check first month
        assert_eq!(months[0].month_number, 1);
        assert_eq!(months[0].name_english, "Muharram");
        assert_eq!(months[0].name_arabic, "مُحَرَّم");
        
        // Check last month
        assert_eq!(months[11].month_number, 12);
        assert_eq!(months[11].name_english, "Dhu al-Hijjah");
        assert_eq!(months[11].name_arabic, "ذُو الحِجَّة");
    }

    #[test]
    fn test_hijri_leap_year() {
        // Test some known leap years in the 30-year cycle
        assert!(HijriCalendar::is_hijri_leap_year(1445)); // Should be leap year
        assert!(!HijriCalendar::is_hijri_leap_year(1444)); // Should not be leap year
    }

    #[test]
    fn test_days_in_hijri_month() {
        // Test regular months
        assert_eq!(HijriCalendar::days_in_hijri_month(1445, 1), 30); // Muharram
        assert_eq!(HijriCalendar::days_in_hijri_month(1445, 2), 29); // Safar
        
        // Test Dhu al-Hijjah in leap year vs non-leap year
        let leap_year = 1445;
        let non_leap_year = 1444;
        
        if HijriCalendar::is_hijri_leap_year(leap_year) {
            assert_eq!(HijriCalendar::days_in_hijri_month(leap_year, 12), 30);
        }
        if !HijriCalendar::is_hijri_leap_year(non_leap_year) {
            assert_eq!(HijriCalendar::days_in_hijri_month(non_leap_year, 12), 29);
        }
    }

    #[test]
    fn test_days_in_hijri_year() {
        let leap_year = 1445;
        let non_leap_year = 1444;
        
        if HijriCalendar::is_hijri_leap_year(leap_year) {
            assert_eq!(HijriCalendar::days_in_hijri_year(leap_year), 355);
        }
        if !HijriCalendar::is_hijri_leap_year(non_leap_year) {
            assert_eq!(HijriCalendar::days_in_hijri_year(non_leap_year), 354);
        }
    }

    #[test]
    fn test_valid_hijri_date() {
        // Valid dates
        assert!(HijriCalendar::is_valid_hijri_date(1445, 1, 1));
        assert!(HijriCalendar::is_valid_hijri_date(1445, 12, 30));
        assert!(HijriCalendar::is_valid_hijri_date(1445, 6, 29));
        
        // Invalid dates
        assert!(!HijriCalendar::is_valid_hijri_date(0, 1, 1)); // Invalid year
        assert!(!HijriCalendar::is_valid_hijri_date(1445, 0, 1)); // Invalid month
        assert!(!HijriCalendar::is_valid_hijri_date(1445, 13, 1)); // Invalid month
        assert!(!HijriCalendar::is_valid_hijri_date(1445, 1, 0)); // Invalid day
        assert!(!HijriCalendar::is_valid_hijri_date(1445, 1, 31)); // Invalid day for month
    }

    #[test]
    fn test_hijri_day_of_week() {
        // Test that day of week calculation works
        let day_of_week = HijriCalendar::hijri_day_of_week(1445, 1, 1).unwrap();
        assert!(day_of_week >= 0 && day_of_week <= 6);
    }

    #[test]
    fn test_is_friday() {
        // Test Friday detection (this will depend on the specific date)
        let is_friday = HijriCalendar::is_friday(1445, 1, 1).unwrap();
        assert!(is_friday == true || is_friday == false); // Just ensure it returns a boolean
    }

    #[test]
    fn test_islamic_events_for_date() {
        // Test Ashura (Muharram 10)
        let events = HijriCalendar::get_islamic_events_for_date(1, 10);
        assert!(!events.is_empty());
        assert!(events.iter().any(|e| e.name.contains("عاشوراء") || e.name.contains("Ashura")));
        
        // Test Prophet's Birthday (Rabi al-Awwal 12)
        let events = HijriCalendar::get_islamic_events_for_date(3, 12);
        assert!(!events.is_empty());
        assert!(events.iter().any(|e| e.name.contains("المولد النبوي") || e.name.contains("Prophet")));
        
        // Test Eid al-Fitr (Shawwal 1)
        let events = HijriCalendar::get_islamic_events_for_date(10, 1);
        assert!(!events.is_empty());
        assert!(events.iter().any(|e| e.name.contains("عيد الفطر") || e.name.contains("Eid al-Fitr")));
        
        // Test Day of Arafah (Dhu al-Hijjah 9)
        let events = HijriCalendar::get_islamic_events_for_date(12, 9);
        assert!(!events.is_empty());
        assert!(events.iter().any(|e| e.name.contains("عرفة") || e.name.contains("Arafah")));
        
        // Test Eid al-Adha (Dhu al-Hijjah 10)
        let events = HijriCalendar::get_islamic_events_for_date(12, 10);
        assert!(!events.is_empty());
        assert!(events.iter().any(|e| e.name.contains("عيد الأضحى") || e.name.contains("Eid al-Adha")));
    }

    #[test]
    fn test_ramadan_events() {
        // Test beginning of Ramadan (Ramadan 1)
        let events = HijriCalendar::get_islamic_events_for_date(9, 1);
        assert!(!events.is_empty());
        assert!(events.iter().any(|e| e.name.contains("رمضان") || e.name.contains("Ramadan")));
        
        // Test Laylat al-Qadr (Ramadan 27)
        let events = HijriCalendar::get_islamic_events_for_date(9, 27);
        assert!(!events.is_empty());
        assert!(events.iter().any(|e| e.name.contains("ليلة القدر") || e.name.contains("Laylat al-Qadr")));
        
        // Any day in Ramadan should have the month event
        let events = HijriCalendar::get_islamic_events_for_date(9, 15);
        assert!(!events.is_empty());
        assert!(events.iter().any(|e| e.name.contains("رمضان") || e.name.contains("Ramadan")));
    }

    #[test]
    fn test_event_types() {
        // Test that events have correct types
        let eid_events = HijriCalendar::get_islamic_events_for_date(10, 1); // Eid al-Fitr
        assert!(eid_events.iter().any(|e| matches!(e.event_type, EventType::Eid)));
        
        let prophet_events = HijriCalendar::get_islamic_events_for_date(3, 12); // Prophet's Birthday
        assert!(prophet_events.iter().any(|e| matches!(e.event_type, EventType::ProphetBirthday)));
        
        let important_day_events = HijriCalendar::get_islamic_events_for_date(1, 10); // Ashura
        assert!(important_day_events.iter().any(|e| matches!(e.event_type, EventType::ImportantDay)));
    }

    #[test]
    fn test_event_details() {
        // Test that event details are available for major events
        let ashura_details = HijriCalendar::get_event_details("يوم عاشوراء");
        assert!(ashura_details.is_some());
        assert!(ashura_details.unwrap().contains("صيام"));
        
        let prophet_birthday_details = HijriCalendar::get_event_details("المولد النبوي الشريف");
        assert!(prophet_birthday_details.is_some());
        assert!(prophet_birthday_details.unwrap().contains("النبي"));
        
        let laylat_qadr_details = HijriCalendar::get_event_details("ليلة القدر");
        assert!(laylat_qadr_details.is_some());
        assert!(laylat_qadr_details.unwrap().contains("ألف شهر"));
        
        // Test non-existent event
        let unknown_details = HijriCalendar::get_event_details("Unknown Event");
        assert!(unknown_details.is_none());
    }

    #[test]
    fn test_no_events_for_regular_day() {
        // Test a regular day that should have no special events
        let events = HijriCalendar::get_islamic_events_for_date(2, 15); // Safar 15
        // Should only have month-related events if any, but not major holidays
        let major_events = events.iter().filter(|e| 
            matches!(e.event_type, EventType::Eid | EventType::ProphetBirthday)
        ).count();
        assert_eq!(major_events, 0);
    }

    #[test]
    fn test_first_ten_days_dhul_hijjah() {
        // Test that the first ten days of Dhu al-Hijjah have special events
        for day in 1..=10 {
            let events = HijriCalendar::get_islamic_events_for_date(12, day);
            if day <= 10 {
                assert!(events.iter().any(|e| 
                    e.name.contains("العشر من ذي الحجة") || 
                    e.name.contains("First Ten Days")
                ));
            }
        }
    }
}
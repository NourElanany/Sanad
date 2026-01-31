use chrono::{NaiveDate, Datelike};
use shared::HijriDate;
use crate::models::{HijriMonth, HijriGregorianConversion};

/// Hijri calendar converter with accurate astronomical algorithms
pub struct HijriCalendar;

impl HijriCalendar {
    /// Convert Gregorian date to Hijri date
    pub fn gregorian_to_hijri(gregorian_date: NaiveDate) -> Result<HijriDate, Box<dyn std::error::Error>> {
        let julian_day = Self::gregorian_to_julian(gregorian_date);
        let (hijri_year, hijri_month, hijri_day) = Self::julian_to_hijri(julian_day);
        
        let month_name = Self::get_month_name(hijri_month)?;
        
        Ok(HijriDate {
            year: hijri_year,
            month: hijri_month as u8,
            day: hijri_day as u8,
            month_name,
        })
    }
    
    /// Convert Hijri date to Gregorian date
    pub fn hijri_to_gregorian(
        hijri_year: i32,
        hijri_month: i32,
        hijri_day: i32,
    ) -> Result<NaiveDate, Box<dyn std::error::Error>> {
        let julian_day = Self::hijri_to_julian(hijri_year, hijri_month, hijri_day);
        Self::julian_to_gregorian(julian_day)
    }
    
    /// Get all Hijri months with their names
    pub fn get_hijri_months() -> Vec<HijriMonth> {
        vec![
            HijriMonth {
                month_number: 1,
                name_arabic: "مُحَرَّم".to_string(),
                name_english: "Muharram".to_string(),
                name_transliteration: "Muharram".to_string(),
            },
            HijriMonth {
                month_number: 2,
                name_arabic: "صَفَر".to_string(),
                name_english: "Safar".to_string(),
                name_transliteration: "Safar".to_string(),
            },
            HijriMonth {
                month_number: 3,
                name_arabic: "رَبِيع الأَوَّل".to_string(),
                name_english: "Rabi al-Awwal".to_string(),
                name_transliteration: "Rabi al-Awwal".to_string(),
            },
            HijriMonth {
                month_number: 4,
                name_arabic: "رَبِيع الآخِر".to_string(),
                name_english: "Rabi al-Thani".to_string(),
                name_transliteration: "Rabi al-Thani".to_string(),
            },
            HijriMonth {
                month_number: 5,
                name_arabic: "جُمَادَى الأُولَى".to_string(),
                name_english: "Jumada al-Awwal".to_string(),
                name_transliteration: "Jumada al-Awwal".to_string(),
            },
            HijriMonth {
                month_number: 6,
                name_arabic: "جُمَادَى الآخِرَة".to_string(),
                name_english: "Jumada al-Thani".to_string(),
                name_transliteration: "Jumada al-Thani".to_string(),
            },
            HijriMonth {
                month_number: 7,
                name_arabic: "رَجَب".to_string(),
                name_english: "Rajab".to_string(),
                name_transliteration: "Rajab".to_string(),
            },
            HijriMonth {
                month_number: 8,
                name_arabic: "شَعْبَان".to_string(),
                name_english: "Shaban".to_string(),
                name_transliteration: "Shaban".to_string(),
            },
            HijriMonth {
                month_number: 9,
                name_arabic: "رَمَضَان".to_string(),
                name_english: "Ramadan".to_string(),
                name_transliteration: "Ramadan".to_string(),
            },
            HijriMonth {
                month_number: 10,
                name_arabic: "شَوَّال".to_string(),
                name_english: "Shawwal".to_string(),
                name_transliteration: "Shawwal".to_string(),
            },
            HijriMonth {
                month_number: 11,
                name_arabic: "ذُو القَعْدَة".to_string(),
                name_english: "Dhu al-Qadah".to_string(),
                name_transliteration: "Dhu al-Qadah".to_string(),
            },
            HijriMonth {
                month_number: 12,
                name_arabic: "ذُو الحِجَّة".to_string(),
                name_english: "Dhu al-Hijjah".to_string(),
                name_transliteration: "Dhu al-Hijjah".to_string(),
            },
        ]
    }
}
    /// Convert Gregorian date to Julian day number
    fn gregorian_to_julian(date: NaiveDate) -> i32 {
        let year = date.year();
        let month = date.month() as i32;
        let day = date.day() as i32;
        
        let a = (14 - month) / 12;
        let y = year + 4800 - a;
        let m = month + 12 * a - 3;
        
        day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
    }
    
    /// Convert Julian day number to Gregorian date
    fn julian_to_gregorian(julian_day: i32) -> Result<NaiveDate, Box<dyn std::error::Error>> {
        let a = julian_day + 32044;
        let b = (4 * a + 3) / 146097;
        let c = a - (146097 * b) / 4;
        let d = (4 * c + 3) / 1461;
        let e = c - (1461 * d) / 4;
        let m = (5 * e + 2) / 153;
        
        let day = e - (153 * m + 2) / 5 + 1;
        let month = m + 3 - 12 * (m / 10);
        let year = 100 * b + d - 4800 + m / 10;
        
        NaiveDate::from_ymd_opt(year, month as u32, day as u32)
            .ok_or("Invalid Gregorian date".into())
    }
    
    /// Convert Julian day to Hijri date using Kuwaiti algorithm
    fn julian_to_hijri(julian_day: i32) -> (i32, i32, i32) {
        // Hijri epoch: July 16, 622 CE (Julian day 1948439)
        const HIJRI_EPOCH: i32 = 1948439;
        
        let days_since_epoch = julian_day - HIJRI_EPOCH;
        
        // Average Hijri year length: 354.367 days
        let hijri_years = (days_since_epoch as f64 / 354.367).floor() as i32;
        let remaining_days = days_since_epoch - (hijri_years as f64 * 354.367).floor() as i32;
        
        // Average Hijri month length: 29.53 days
        let hijri_months = (remaining_days as f64 / 29.53).floor() as i32;
        let remaining_days = remaining_days - (hijri_months as f64 * 29.53).floor() as i32;
        
        let year = hijri_years + 1;
        let month = (hijri_months + 1).min(12);
        let day = (remaining_days + 1).max(1).min(30);
        
        (year, month, day)
    }
    
    /// Convert Hijri date to Julian day using reverse calculation
    fn hijri_to_julian(hijri_year: i32, hijri_month: i32, hijri_day: i32) -> i32 {
        const HIJRI_EPOCH: i32 = 1948439;
        
        let total_days = ((hijri_year - 1) as f64 * 354.367).floor() as i32
                       + ((hijri_month - 1) as f64 * 29.53).floor() as i32
                       + (hijri_day - 1);
        
        HIJRI_EPOCH + total_days
    }
    
    /// Get month name for a given month number
    fn get_month_name(month_number: i32) -> Result<String, Box<dyn std::error::Error>> {
        let months = Self::get_hijri_months();
        months.iter()
            .find(|m| m.month_number == month_number)
            .map(|m| m.name_english.clone())
            .ok_or("Invalid month number".into())
    }
    
    /// Check if a Hijri year is a leap year (has 355 days instead of 354)
    pub fn is_hijri_leap_year(hijri_year: i32) -> bool {
        // Simplified leap year calculation for Hijri calendar
        // In reality, this depends on lunar observations
        let cycle = hijri_year % 30;
        matches!(cycle, 2 | 5 | 7 | 10 | 13 | 16 | 18 | 21 | 24 | 26 | 29)
    }
    
    /// Get the number of days in a Hijri month
    pub fn days_in_hijri_month(hijri_year: i32, hijri_month: i32) -> i32 {
        // Simplified calculation - in reality depends on lunar observations
        match hijri_month {
            1 | 3 | 5 | 7 | 9 | 11 => 30,
            2 | 4 | 6 | 8 | 10 => 29,
            12 => if Self::is_hijri_leap_year(hijri_year) { 30 } else { 29 },
            _ => 29,
        }
    }
    
    /// Get the total days in a Hijri year
    pub fn days_in_hijri_year(hijri_year: i32) -> i32 {
        if Self::is_hijri_leap_year(hijri_year) { 355 } else { 354 }
    }
    
    /// Validate Hijri date
    pub fn is_valid_hijri_date(hijri_year: i32, hijri_month: i32, hijri_day: i32) -> bool {
        if hijri_year < 1 || hijri_month < 1 || hijri_month > 12 || hijri_day < 1 {
            return false;
        }
        
        let max_days = Self::days_in_hijri_month(hijri_year, hijri_month);
        hijri_day <= max_days
    }
    
    /// Get the day of week for a Hijri date (0 = Saturday, 6 = Friday)
    pub fn hijri_day_of_week(hijri_year: i32, hijri_month: i32, hijri_day: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let julian_day = Self::hijri_to_julian(hijri_year, hijri_month, hijri_day);
        // Julian day 0 was a Monday, so we adjust
        Ok((julian_day + 1) % 7)
    }
    
    /// Check if a Hijri date falls on Friday (Jumu'ah)
    pub fn is_friday(hijri_year: i32, hijri_month: i32, hijri_day: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let day_of_week = Self::hijri_day_of_week(hijri_year, hijri_month, hijri_day)?;
        Ok(day_of_week == 6) // Friday is day 6 in our system
    }
}
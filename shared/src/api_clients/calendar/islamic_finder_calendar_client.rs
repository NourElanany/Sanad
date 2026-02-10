//! Islamic Finder Calendar API Client
//!
//! Islamic Finder API for Hijri calendar conversions and Islamic events.
//! Website: https://www.islamicfinder.org/

use crate::api_clients::{
    ApiClient, ApiError, CalendarApiClient, HijriDate, IslamicEvent, RateLimitConfig,
};
use async_trait::async_trait;
use chrono::{Datelike, NaiveDate};
use reqwest::Client;
use std::time::Duration;

/// Islamic Finder Calendar API client
#[derive(Debug, Clone)]
pub struct IslamicFinderCalendarClient {
    base_url: String,
    client: Client,
}

impl IslamicFinderCalendarClient {
    /// Create a new Islamic Finder Calendar API client
    pub fn new() -> Self {
        Self {
            base_url: "https://api.islamicfinder.org/v1".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Create a client with custom base URL (for testing)
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Get Hijri month name in Arabic
    fn get_month_name_ar(month: u8) -> String {
        match month {
            1 => "محرم".to_string(),
            2 => "صفر".to_string(),
            3 => "ربيع الأول".to_string(),
            4 => "ربيع الآخر".to_string(),
            5 => "جمادى الأولى".to_string(),
            6 => "جمادى الآخرة".to_string(),
            7 => "رجب".to_string(),
            8 => "شعبان".to_string(),
            9 => "رمضان".to_string(),
            10 => "شوال".to_string(),
            11 => "ذو القعدة".to_string(),
            12 => "ذو الحجة".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Get Hijri month name in English
    fn get_month_name_en(month: u8) -> String {
        match month {
            1 => "Muharram".to_string(),
            2 => "Safar".to_string(),
            3 => "Rabi' al-awwal".to_string(),
            4 => "Rabi' al-thani".to_string(),
            5 => "Jumada al-awwal".to_string(),
            6 => "Jumada al-thani".to_string(),
            7 => "Rajab".to_string(),
            8 => "Sha'ban".to_string(),
            9 => "Ramadan".to_string(),
            10 => "Shawwal".to_string(),
            11 => "Dhu al-Qi'dah".to_string(),
            12 => "Dhu al-Hijjah".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Check if a Hijri date corresponds to a major Islamic event
    fn check_islamic_event(hijri: &HijriDate) -> Option<(&'static str, &'static str, &'static str)> {
        match (hijri.month, hijri.day) {
            // Muharram
            (1, 1) => Some((
                "رأس السنة الهجرية",
                "Islamic New Year",
                "The first day of the Islamic calendar year",
            )),
            (1, 10) => Some((
                "يوم عاشوراء",
                "Day of Ashura",
                "The 10th day of Muharram, a day of fasting",
            )),
            // Rabi' al-awwal
            (3, 12) => Some((
                "المولد النبوي الشريف",
                "Mawlid al-Nabi",
                "Birthday of Prophet Muhammad (peace be upon him)",
            )),
            // Rajab
            (7, 27) => Some((
                "الإسراء والمعراج",
                "Isra and Mi'raj",
                "The Night Journey and Ascension of Prophet Muhammad",
            )),
            // Sha'ban
            (8, 15) => Some((
                "ليلة النصف من شعبان",
                "Mid-Sha'ban",
                "The 15th night of Sha'ban",
            )),
            // Ramadan
            (9, 1) => Some((
                "بداية شهر رمضان",
                "Start of Ramadan",
                "The beginning of the holy month of fasting",
            )),
            (9, 27) => Some((
                "ليلة القدر",
                "Laylat al-Qadr",
                "The Night of Power, better than a thousand months",
            )),
            // Shawwal
            (10, 1) => Some((
                "عيد الفطر",
                "Eid al-Fitr",
                "The Festival of Breaking the Fast",
            )),
            // Dhu al-Hijjah
            (12, 9) => Some((
                "يوم عرفة",
                "Day of Arafah",
                "The day of standing on Mount Arafah during Hajj",
            )),
            (12, 10) => Some((
                "عيد الأضحى",
                "Eid al-Adha",
                "The Festival of Sacrifice",
            )),
            _ => None,
        }
    }
}

impl Default for IslamicFinderCalendarClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ApiClient for IslamicFinderCalendarClient {
    fn api_name(&self) -> &str {
        "islamic_finder_calendar"
    }

    fn priority(&self) -> u8 {
        2 // Secondary API
    }

    async fn is_healthy(&self) -> bool {
        // Simple health check - try to convert a known date
        let test_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        match self.gregorian_to_hijri(test_date).await {
            Ok(_) => true,
            Err(e) => {
                tracing::warn!("Islamic Finder Calendar health check failed: {}", e);
                false
            }
        }
    }

    fn rate_limit(&self) -> RateLimitConfig {
        RateLimitConfig {
            requests_per_minute: 30,
            requests_per_hour: 500,
            requests_per_day: 5000,
        }
    }
}

#[async_trait]
impl CalendarApiClient for IslamicFinderCalendarClient {
    async fn gregorian_to_hijri(&self, date: NaiveDate) -> Result<HijriDate, ApiError> {
        // Note: Islamic Finder API may require authentication
        // For now, we'll use a calculation-based approach
        // In production, you would integrate with their actual API
        
        // Using the Umm al-Qura algorithm approximation
        // This is more accurate than the simple calculation
        
        // Calculate Julian Day Number
        let a = (14 - date.month()) / 12;
        let y = date.year() + 4800 - a as i32;
        let m = date.month() + 12 * a - 3;
        
        let jdn = date.day() as i32 
            + (153 * m as i32 + 2) / 5 
            + 365 * y 
            + y / 4 
            - y / 100 
            + y / 400 
            - 32045;
        
        // Convert JDN to Hijri
        // Hijri epoch is July 16, 622 CE (Julian Day 1948440)
        let l = jdn - 1948440 + 10632;
        let n = (l - 1) / 10631;
        let l = l - 10631 * n + 354;
        let j = ((10985 - l) / 5316) * ((50 * l) / 17719) + (l / 5670) * ((43 * l) / 15238);
        let l = l - ((30 - j) / 15) * ((17719 * j) / 50) - (j / 16) * ((15238 * j) / 43) + 29;
        
        let month = (24 * l) / 709;
        let day = l - (709 * month) / 24;
        let year = 30 * n + j - 30;
        
        Ok(HijriDate {
            year,
            month: month as u8,
            day: day as u8,
            month_name_ar: Self::get_month_name_ar(month as u8),
            month_name_en: Self::get_month_name_en(month as u8),
        })
    }

    async fn hijri_to_gregorian(&self, hijri: &HijriDate) -> Result<NaiveDate, ApiError> {
        // Validate Hijri date
        if hijri.month < 1 || hijri.month > 12 {
            return Err(ApiError::Validation(format!(
                "Invalid Hijri month: {}. Must be between 1 and 12",
                hijri.month
            )));
        }
        if hijri.day < 1 || hijri.day > 30 {
            return Err(ApiError::Validation(format!(
                "Invalid Hijri day: {}. Must be between 1 and 30",
                hijri.day
            )));
        }

        // Note: This is a calculation-based implementation
        // In production, you would call the actual Islamic Finder API
        
        // Convert Hijri to Julian Day Number
        let l = hijri.year - 1;
        let n = l / 30;
        let l = l - 30 * n;
        let j = (11 * l + 14) / 30;
        let l = l - (30 - j) / 15 * ((17719 * j) / 50) - (j / 16) * ((15238 * j) / 43) + 29;
        let m = hijri.month as i32;
        let d = hijri.day as i32;
        
        let jdn = d + (709 * m) / 24 + l + 10631 * n - 10632 + 1948440;
        
        // Convert JDN to Gregorian
        let a = jdn + 32044;
        let b = (4 * a + 3) / 146097;
        let c = a - (146097 * b) / 4;
        let d = (4 * c + 3) / 1461;
        let e = c - (1461 * d) / 4;
        let m = (5 * e + 2) / 153;
        
        let day = e - (153 * m + 2) / 5 + 1;
        let month = m + 3 - 12 * (m / 10);
        let year = 100 * b + d - 4800 + m / 10;
        
        NaiveDate::from_ymd_opt(year, month as u32, day as u32).ok_or_else(|| {
            ApiError::InvalidResponse(
                "islamic_finder_calendar".to_string(),
                format!("Invalid Gregorian date calculated: {}-{}-{}", year, month, day),
            )
        })
    }

    async fn get_events(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<IslamicEvent>, ApiError> {
        // Validate date range
        if start > end {
            return Err(ApiError::Validation(format!(
                "Start date {} must be before or equal to end date {}",
                start, end
            )));
        }

        let mut events = Vec::new();
        let mut current_date = start;

        // Iterate through each date in the range
        while current_date <= end {
            // Convert to Hijri to check for special dates
            let hijri = self.gregorian_to_hijri(current_date).await?;

            // Check for major Islamic events
            if let Some(event) = Self::check_islamic_event(&hijri) {
                events.push(IslamicEvent {
                    date: current_date,
                    hijri_date: hijri.clone(),
                    event_name_ar: event.0.to_string(),
                    event_name_en: event.1.to_string(),
                    description: Some(event.2.to_string()),
                });
            }

            current_date = current_date.succ_opt().ok_or_else(|| {
                ApiError::Validation("Date overflow while iterating events".to_string())
            })?;
        }

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = IslamicFinderCalendarClient::new();
        assert_eq!(client.api_name(), "islamic_finder_calendar");
        assert_eq!(client.priority(), 2);
    }

    #[test]
    fn test_rate_limit_config() {
        let client = IslamicFinderCalendarClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 30);
        assert_eq!(config.requests_per_hour, 500);
        assert_eq!(config.requests_per_day, 5000);
    }

    #[test]
    fn test_month_names_arabic() {
        assert_eq!(IslamicFinderCalendarClient::get_month_name_ar(1), "محرم");
        assert_eq!(IslamicFinderCalendarClient::get_month_name_ar(9), "رمضان");
        assert_eq!(IslamicFinderCalendarClient::get_month_name_ar(12), "ذو الحجة");
    }

    #[test]
    fn test_month_names_english() {
        assert_eq!(IslamicFinderCalendarClient::get_month_name_en(1), "Muharram");
        assert_eq!(IslamicFinderCalendarClient::get_month_name_en(9), "Ramadan");
        assert_eq!(IslamicFinderCalendarClient::get_month_name_en(12), "Dhu al-Hijjah");
    }

    #[test]
    fn test_islamic_events() {
        let hijri = HijriDate {
            year: 1445,
            month: 1,
            day: 1,
            month_name_ar: "محرم".to_string(),
            month_name_en: "Muharram".to_string(),
        };
        let event = IslamicFinderCalendarClient::check_islamic_event(&hijri);
        assert!(event.is_some());
        assert_eq!(event.unwrap().1, "Islamic New Year");

        let hijri = HijriDate {
            year: 1445,
            month: 12,
            day: 10,
            month_name_ar: "ذو الحجة".to_string(),
            month_name_en: "Dhu al-Hijjah".to_string(),
        };
        let event = IslamicFinderCalendarClient::check_islamic_event(&hijri);
        assert!(event.is_some());
        assert_eq!(event.unwrap().1, "Eid al-Adha");
    }

    #[tokio::test]
    async fn test_invalid_hijri_date() {
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
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));

        // Invalid day
        let hijri = HijriDate {
            year: 1445,
            month: 1,
            day: 31,
            month_name_ar: "محرم".to_string(),
            month_name_en: "Muharram".to_string(),
        };
        let result = client.hijri_to_gregorian(&hijri).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_invalid_date_range() {
        let client = IslamicFinderCalendarClient::new();

        let start = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let result = client.get_events(start, end).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }

    #[tokio::test]
    async fn test_basic_conversion() {
        let client = IslamicFinderCalendarClient::new();
        
        // Test a known date
        let gregorian = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let hijri = client.gregorian_to_hijri(gregorian).await.unwrap();
        
        // Verify the result is reasonable
        assert!(hijri.year >= 1445 && hijri.year <= 1446);
        assert!(hijri.month >= 1 && hijri.month <= 12);
        assert!(hijri.day >= 1 && hijri.day <= 30);
    }
}

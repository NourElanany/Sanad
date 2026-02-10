//! Aladhan Calendar API Client
//!
//! Official Islamic Network API for Hijri calendar conversions and Islamic events.
//! API Documentation: https://aladhan.com/hijri-gregorian-calendar-api

use crate::api_clients::{
    ApiClient, ApiError, CalendarApiClient, HijriDate, IslamicEvent, RateLimitConfig,
};
use async_trait::async_trait;
use chrono::NaiveDate;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

/// Aladhan Calendar API client
#[derive(Debug, Clone)]
pub struct AladhanCalendarClient {
    base_url: String,
    client: Client,
}

impl AladhanCalendarClient {
    /// Create a new Aladhan Calendar API client
    pub fn new() -> Self {
        Self {
            base_url: "https://api.aladhan.com/v1".to_string(),
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
    pub fn get_month_name_ar(month: u8) -> String {
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
    pub fn get_month_name_en(month: u8) -> String {
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

    /// Parse Hijri date from Aladhan response
    fn parse_hijri_date(hijri: &AladhanHijriDate) -> Result<HijriDate, ApiError> {
        let month: u8 = hijri.month.number.parse().map_err(|e| {
            ApiError::InvalidResponse(
                "aladhan".to_string(),
                format!("Failed to parse Hijri month: {}", e),
            )
        })?;

        let day: u8 = hijri.day.parse().map_err(|e| {
            ApiError::InvalidResponse(
                "aladhan".to_string(),
                format!("Failed to parse Hijri day: {}", e),
            )
        })?;

        let year: i32 = hijri.year.parse().map_err(|e| {
            ApiError::InvalidResponse(
                "aladhan".to_string(),
                format!("Failed to parse Hijri year: {}", e),
            )
        })?;

        Ok(HijriDate {
            year,
            month,
            day,
            month_name_ar: Self::get_month_name_ar(month),
            month_name_en: Self::get_month_name_en(month),
        })
    }

    /// Parse Gregorian date from Aladhan response
    fn parse_gregorian_date(greg: &AladhanGregorianDate) -> Result<NaiveDate, ApiError> {
        let day: u32 = greg.day.parse().map_err(|e| {
            ApiError::InvalidResponse(
                "aladhan".to_string(),
                format!("Failed to parse Gregorian day: {}", e),
            )
        })?;

        let month: u32 = greg.month.number.parse().map_err(|e| {
            ApiError::InvalidResponse(
                "aladhan".to_string(),
                format!("Failed to parse Gregorian month: {}", e),
            )
        })?;

        let year: i32 = greg.year.parse().map_err(|e| {
            ApiError::InvalidResponse(
                "aladhan".to_string(),
                format!("Failed to parse Gregorian year: {}", e),
            )
        })?;

        NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
            ApiError::InvalidResponse(
                "aladhan".to_string(),
                format!("Invalid Gregorian date: {}-{}-{}", year, month, day),
            )
        })
    }
}

impl Default for AladhanCalendarClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ApiClient for AladhanCalendarClient {
    fn api_name(&self) -> &str {
        "aladhan_calendar"
    }

    fn priority(&self) -> u8 {
        1 // Primary API
    }

    async fn is_healthy(&self) -> bool {
        // Simple health check - try to convert a known date
        let test_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        match self.gregorian_to_hijri(test_date).await {
            Ok(_) => true,
            Err(e) => {
                tracing::warn!("Aladhan Calendar health check failed: {}", e);
                false
            }
        }
    }

    fn rate_limit(&self) -> RateLimitConfig {
        RateLimitConfig {
            requests_per_minute: 60,
            requests_per_hour: 1000,
            requests_per_day: 10000,
        }
    }
}

#[async_trait]
impl CalendarApiClient for AladhanCalendarClient {
    async fn gregorian_to_hijri(&self, date: NaiveDate) -> Result<HijriDate, ApiError> {
        let url = format!(
            "{}/gToH/{}-{}-{}",
            self.base_url,
            date.format("%d"),
            date.format("%m"),
            date.format("%Y")
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            ApiError::Network(format!(
                "Failed to fetch Hijri date from Aladhan: {}",
                e
            ))
        })?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!(
                    "HTTP {}: {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            ));
        }

        let calendar_response: AladhanCalendarResponse =
            response.json().await.map_err(|e| {
                ApiError::InvalidResponse(
                    self.api_name().to_string(),
                    format!("Failed to parse calendar response: {}", e),
                )
            })?;

        if calendar_response.code != 200 {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!(
                    "API returned code {}: {}",
                    calendar_response.code, calendar_response.status
                ),
            ));
        }

        Self::parse_hijri_date(&calendar_response.data.hijri)
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

        let url = format!(
            "{}/hToG/{}-{}-{}",
            self.base_url, hijri.day, hijri.month, hijri.year
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            ApiError::Network(format!(
                "Failed to fetch Gregorian date from Aladhan: {}",
                e
            ))
        })?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!(
                    "HTTP {}: {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            ));
        }

        let calendar_response: AladhanCalendarResponse =
            response.json().await.map_err(|e| {
                ApiError::InvalidResponse(
                    self.api_name().to_string(),
                    format!("Failed to parse calendar response: {}", e),
                )
            })?;

        if calendar_response.code != 200 {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!(
                    "API returned code {}: {}",
                    calendar_response.code, calendar_response.status
                ),
            ));
        }

        Self::parse_gregorian_date(&calendar_response.data.gregorian)
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

impl AladhanCalendarClient {
    /// Check if a Hijri date corresponds to a major Islamic event
    pub fn check_islamic_event(hijri: &HijriDate) -> Option<(&'static str, &'static str, &'static str)> {
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

// ============================================================================
// Response structures for Aladhan Calendar API
// ============================================================================

#[derive(Debug, Deserialize)]
struct AladhanCalendarResponse {
    code: u16,
    status: String,
    data: AladhanCalendarData,
}

#[derive(Debug, Deserialize)]
struct AladhanCalendarData {
    hijri: AladhanHijriDate,
    gregorian: AladhanGregorianDate,
}

#[derive(Debug, Deserialize)]
struct AladhanHijriDate {
    #[serde(deserialize_with = "deserialize_string_or_number")]
    day: String,
    month: AladhanMonth,
    #[serde(deserialize_with = "deserialize_string_or_number")]
    year: String,
}

#[derive(Debug, Deserialize)]
struct AladhanGregorianDate {
    #[serde(deserialize_with = "deserialize_string_or_number")]
    day: String,
    month: AladhanMonth,
    #[serde(deserialize_with = "deserialize_string_or_number")]
    year: String,
}

#[derive(Debug, Deserialize)]
struct AladhanMonth {
    #[serde(deserialize_with = "deserialize_string_or_number")]
    number: String,
}

// Helper function to deserialize either a string or a number as a string
fn deserialize_string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct StringOrNumber;

    impl<'de> Visitor<'de> for StringOrNumber {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or a number")
        }

        fn visit_str<E>(self, value: &str) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_i64<E>(self, value: i64) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    deserializer.deserialize_any(StringOrNumber)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = AladhanCalendarClient::new();
        assert_eq!(client.api_name(), "aladhan_calendar");
        assert_eq!(client.priority(), 1);
    }

    #[test]
    fn test_rate_limit_config() {
        let client = AladhanCalendarClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.requests_per_hour, 1000);
        assert_eq!(config.requests_per_day, 10000);
    }

    #[test]
    fn test_month_names_arabic() {
        assert_eq!(AladhanCalendarClient::get_month_name_ar(1), "محرم");
        assert_eq!(AladhanCalendarClient::get_month_name_ar(9), "رمضان");
        assert_eq!(AladhanCalendarClient::get_month_name_ar(12), "ذو الحجة");
    }

    #[test]
    fn test_month_names_english() {
        assert_eq!(AladhanCalendarClient::get_month_name_en(1), "Muharram");
        assert_eq!(AladhanCalendarClient::get_month_name_en(9), "Ramadan");
        assert_eq!(AladhanCalendarClient::get_month_name_en(12), "Dhu al-Hijjah");
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
        let event = AladhanCalendarClient::check_islamic_event(&hijri);
        assert!(event.is_some());
        assert_eq!(event.unwrap().1, "Islamic New Year");

        let hijri = HijriDate {
            year: 1445,
            month: 9,
            day: 1,
            month_name_ar: "رمضان".to_string(),
            month_name_en: "Ramadan".to_string(),
        };
        let event = AladhanCalendarClient::check_islamic_event(&hijri);
        assert!(event.is_some());
        assert_eq!(event.unwrap().1, "Start of Ramadan");

        let hijri = HijriDate {
            year: 1445,
            month: 10,
            day: 1,
            month_name_ar: "شوال".to_string(),
            month_name_en: "Shawwal".to_string(),
        };
        let event = AladhanCalendarClient::check_islamic_event(&hijri);
        assert!(event.is_some());
        assert_eq!(event.unwrap().1, "Eid al-Fitr");
    }

    #[tokio::test]
    async fn test_invalid_hijri_date() {
        let client = AladhanCalendarClient::new();

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
        let client = AladhanCalendarClient::new();

        let start = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let result = client.get_events(start, end).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::Validation(_)));
    }
}

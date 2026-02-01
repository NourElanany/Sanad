use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use shared::{Location, CalculationMethod, PrayerTimes, HijriDate, IslamicEvent, EventType};

/// Prayer calculation settings for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerCalculationSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub location_id: Option<Uuid>,
    pub calculation_method: CalculationMethod,
    pub fajr_angle: Option<f64>,
    pub maghrib_angle: Option<f64>,
    pub isha_angle: Option<f64>,
    pub fajr_adjustment: i32,
    pub dhuhr_adjustment: i32,
    pub asr_adjustment: i32,
    pub maghrib_adjustment: i32,
    pub isha_adjustment: i32,
    pub asr_method: i32, // 1 = Shafi/Maliki/Hanbali, 2 = Hanafi
    pub high_latitude_adjustment: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Daily prayer times with calculation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyPrayerTimes {
    pub id: Uuid,
    pub location_id: Uuid,
    pub calculation_method: CalculationMethod,
    pub date: NaiveDate,
    pub fajr_time: DateTime<Utc>,
    pub sunrise_time: DateTime<Utc>,
    pub dhuhr_time: DateTime<Utc>,
    pub asr_time: DateTime<Utc>,
    pub maghrib_time: DateTime<Utc>,
    pub isha_time: DateTime<Utc>,
    pub qibla_direction: f64,
    pub fajr_angle: Option<f64>,
    pub maghrib_angle: Option<f64>,
    pub isha_angle: Option<f64>,
    pub asr_method: i32,
    pub created_at: DateTime<Utc>,
}

/// Hijri month information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HijriMonth {
    pub month_number: i32,
    pub name_arabic: String,
    pub name_english: String,
    pub name_transliteration: String,
}

/// Islamic event with Hijri date information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicEventDetails {
    pub id: Uuid,
    pub name_arabic: String,
    pub name_english: String,
    pub description_arabic: Option<String>,
    pub description_english: Option<String>,
    pub hijri_month: Option<i32>,
    pub hijri_day: Option<i32>,
    pub hijri_end_month: Option<i32>,
    pub hijri_end_day: Option<i32>,
    pub event_type: String,
    pub importance_level: i32,
    pub notification_enabled: bool,
    pub special_calculation: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Hijri-Gregorian date conversion entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HijriGregorianConversion {
    pub id: Uuid,
    pub gregorian_date: NaiveDate,
    pub hijri_year: i32,
    pub hijri_month: i32,
    pub hijri_day: i32,
    pub julian_day_number: i32,
    pub created_at: DateTime<Utc>,
}

/// User prayer preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrayerPreferences {
    pub id: Uuid,
    pub user_id: Uuid,
    pub fajr_notification_enabled: bool,
    pub fajr_notification_minutes: i32,
    pub dhuhr_notification_enabled: bool,
    pub dhuhr_notification_minutes: i32,
    pub asr_notification_enabled: bool,
    pub asr_notification_minutes: i32,
    pub maghrib_notification_enabled: bool,
    pub maghrib_notification_minutes: i32,
    pub isha_notification_enabled: bool,
    pub isha_notification_minutes: i32,
    pub sunrise_notification_enabled: bool,
    pub sunrise_notification_minutes: i32,
    pub graduated_notifications_enabled: bool,
    pub graduated_intervals: Vec<i32>,
    pub show_qibla_direction: bool,
    pub qibla_compass_style: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Prayer time history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerTimeHistory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub prayer_name: String,
    pub scheduled_time: DateTime<Utc>,
    pub actual_prayer_time: Option<DateTime<Utc>>,
    pub location_id: Option<Uuid>,
    pub prayer_completed: bool,
    pub completion_method: Option<String>,
    pub prayed_in_congregation: bool,
    pub mosque_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Request to calculate prayer times
#[derive(Debug, Deserialize)]
pub struct PrayerTimesRequest {
    pub location: Location,
    pub date: NaiveDate,
    pub calculation_method: Option<CalculationMethod>,
    pub custom_angles: Option<CustomAngles>,
    pub adjustments: Option<PrayerAdjustments>,
}

/// Custom angles for prayer calculation
#[derive(Debug, Deserialize)]
pub struct CustomAngles {
    pub fajr_angle: f64,
    pub maghrib_angle: f64,
    pub isha_angle: f64,
}

/// Prayer time adjustments in minutes
#[derive(Debug, Deserialize, Serialize)]
pub struct PrayerAdjustments {
    pub fajr: i32,
    pub dhuhr: i32,
    pub asr: i32,
    pub maghrib: i32,
    pub isha: i32,
}

/// Qibla direction request
#[derive(Debug, Deserialize)]
pub struct QiblaRequest {
    pub latitude: f64,
    pub longitude: f64,
}

/// Qibla direction response
#[derive(Debug, Serialize)]
pub struct QiblaDirection {
    pub direction_degrees: f64,
    pub direction_cardinal: String,
    pub distance_km: f64,
}

/// Hijri date conversion request
#[derive(Debug, Deserialize)]
pub struct HijriConversionRequest {
    pub date: NaiveDate,
}

/// Gregorian date conversion request
#[derive(Debug, Deserialize)]
pub struct GregorianConversionRequest {
    pub hijri_year: i32,
    pub hijri_month: i32,
    pub hijri_day: i32,
}

/// Islamic events request
#[derive(Debug, Deserialize)]
pub struct IslamicEventsRequest {
    pub hijri_month: Option<i32>,
    pub hijri_year: Option<i32>,
    pub hijri_day: Option<i32>,
    pub importance_level: Option<i32>,
    pub event_type: Option<String>,
}

/// Prayer times calculation result with metadata
#[derive(Debug, Serialize)]
pub struct PrayerTimesResponse {
    pub prayer_times: PrayerTimes,
    pub qibla_direction: QiblaDirection,
    pub calculation_metadata: CalculationMetadata,
    pub islamic_events: Vec<IslamicEventDetails>,
}

/// Calculation metadata for transparency
#[derive(Debug, Serialize)]
pub struct CalculationMetadata {
    pub method_used: CalculationMethod,
    pub angles_used: AnglesUsed,
    pub adjustments_applied: PrayerAdjustments,
    pub high_latitude_method: Option<String>,
    pub calculation_timestamp: DateTime<Utc>,
}

/// Angles used in calculation
#[derive(Debug, Serialize)]
pub struct AnglesUsed {
    pub fajr_angle: f64,
    pub maghrib_angle: f64,
    pub isha_angle: f64,
    pub asr_method: i32,
}

/// Monthly Islamic calendar response
#[derive(Debug, Serialize)]
pub struct MonthlyCalendarResponse {
    pub hijri_month: HijriMonth,
    pub hijri_year: i32,
    pub days: Vec<CalendarDay>,
    pub events: Vec<IslamicEventDetails>,
}

/// Single day in Islamic calendar
#[derive(Debug, Serialize)]
pub struct CalendarDay {
    pub hijri_date: HijriDate,
    pub gregorian_date: NaiveDate,
    pub day_of_week: String,
    pub events: Vec<IslamicEventDetails>,
    pub is_friday: bool,
    pub is_weekend: bool,
}

/// Prayer notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerNotificationSettings {
    pub prayer_name: String,
    pub enabled: bool,
    pub minutes_before: i32,
    pub graduated_enabled: bool,
    pub graduated_intervals: Vec<i32>,
}

/// Notification schedule request
#[derive(Debug, Deserialize)]
pub struct NotificationScheduleRequest {
    pub user_id: Uuid,
    pub location: Location,
    pub preferences: Vec<PrayerNotificationSettings>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

/// Scheduled notification
#[derive(Debug, Serialize)]
pub struct ScheduledNotification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub prayer_name: String,
    pub prayer_time: DateTime<Utc>,
    pub notification_time: DateTime<Utc>,
    pub message_arabic: String,
    pub message_english: String,
    pub is_graduated: bool,
    pub minutes_before: i32,
}

impl Default for PrayerAdjustments {
    fn default() -> Self {
        Self {
            fajr: 0,
            dhuhr: 0,
            asr: 0,
            maghrib: 0,
            isha: 0,
        }
    }
}

impl QiblaDirection {
    pub fn new(direction_degrees: f64, distance_km: f64) -> Self {
        let direction_cardinal = Self::degrees_to_cardinal(direction_degrees);
        Self {
            direction_degrees,
            direction_cardinal,
            distance_km,
        }
    }

    fn degrees_to_cardinal(degrees: f64) -> String {
        let normalized = ((degrees % 360.0) + 360.0) % 360.0;
        match normalized {
            d if d >= 348.75 || d < 11.25 => "N".to_string(),
            d if d >= 11.25 && d < 33.75 => "NNE".to_string(),
            d if d >= 33.75 && d < 56.25 => "NE".to_string(),
            d if d >= 56.25 && d < 78.75 => "ENE".to_string(),
            d if d >= 78.75 && d < 101.25 => "E".to_string(),
            d if d >= 101.25 && d < 123.75 => "ESE".to_string(),
            d if d >= 123.75 && d < 146.25 => "SE".to_string(),
            d if d >= 146.25 && d < 168.75 => "SSE".to_string(),
            d if d >= 168.75 && d < 191.25 => "S".to_string(),
            d if d >= 191.25 && d < 213.75 => "SSW".to_string(),
            d if d >= 213.75 && d < 236.25 => "SW".to_string(),
            d if d >= 236.25 && d < 258.75 => "WSW".to_string(),
            d if d >= 258.75 && d < 281.25 => "W".to_string(),
            d if d >= 281.25 && d < 303.75 => "WNW".to_string(),
            d if d >= 303.75 && d < 326.25 => "NW".to_string(),
            _ => "NNW".to_string(),
        }
    }
}
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveTime, NaiveDate};

/// Notification types for the Islamic app
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "notification_type", rename_all = "snake_case")]
pub enum NotificationType {
    PrayerReminder,
    PrayerGraduated,
    SunnahReminder,
    NaflReminder,
    DhikrReminder,
    SeasonalReminder,
    IslamicEvent,
    KhatmaReminder,
    DailyVerse,
}

/// Notification priority levels
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "notification_priority", rename_all = "snake_case")]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Notification delivery status
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "notification_status", rename_all = "snake_case")]
pub enum NotificationStatus {
    Pending,
    Sent,
    Delivered,
    Read,
    Dismissed,
    Failed,
}

/// Prayer names for graduated notifications
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "prayer_name", rename_all = "snake_case")]
pub enum PrayerName {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}

/// Islamic seasons and special periods
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "islamic_season", rename_all = "snake_case")]
pub enum IslamicSeason {
    Ramadan,
    DhulHijjah,
    Muharram,
    Rajab,
    Shaban,
    LaylatAlQadr,
    Ashura,
    Mawlid,
    IsraMiraj,
}

/// Dhikr categories for time-appropriate reminders
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "dhikr_category", rename_all = "snake_case")]
pub enum DhikrCategory {
    Morning,
    Evening,
    AfterPrayer,
    BeforeSleep,
    AfterWudu,
    Travel,
    General,
}

/// Main notification model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub body: String,
    pub priority: NotificationPriority,
    pub status: NotificationStatus,
    
    // Scheduling information
    pub scheduled_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    
    // Metadata for different notification types
    pub metadata: serde_json::Value,
    
    // Expiration and retry logic
    pub expires_at: Option<DateTime<Utc>>,
    pub retry_count: i32,
    pub max_retries: i32,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Prayer time notifications with graduated reminders
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PrayerNotification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub prayer_name: PrayerName,
    pub prayer_time: DateTime<Utc>,
    
    // Graduated notification settings
    pub enable_graduated: bool,
    pub reminder_intervals: Vec<i32>, // minutes before prayer
    
    // Location context for prayer times
    pub latitude: Option<rust_decimal::Decimal>,
    pub longitude: Option<rust_decimal::Decimal>,
    pub timezone: Option<String>,
    
    // Notification preferences
    pub enable_adhan: bool,
    pub enable_vibration: bool,
    pub custom_message: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Sunnah and Nafl reminders
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SunnahReminder {
    pub id: Uuid,
    pub user_id: Uuid,
    
    // Sunnah details
    pub sunnah_name: String,
    pub sunnah_description: Option<String>,
    pub sunnah_reference: Option<String>, // Hadith or Quran reference
    
    // Timing and frequency
    pub reminder_time: NaiveTime,
    pub frequency: String, // daily, weekly, monthly
    pub days_of_week: Option<Vec<i32>>, // 0=Sunday, 1=Monday, etc.
    
    // Notification settings
    pub is_active: bool,
    pub priority: NotificationPriority,
    pub custom_message: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Islamic seasonal reminders
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SeasonalReminder {
    pub id: Uuid,
    pub user_id: Uuid,
    
    // Season information
    pub season: IslamicSeason,
    pub event_name: String,
    pub event_description: Option<String>,
    
    // Timing (can be Hijri-based)
    pub hijri_month: Option<i32>, // 1-12
    pub hijri_day: Option<i32>,   // 1-30
    pub gregorian_date: Option<NaiveDate>, // For fixed Gregorian dates
    
    // Notification settings
    pub days_before_notification: i32,
    pub is_active: bool,
    pub priority: NotificationPriority,
    
    // Content
    pub reminder_message: Option<String>,
    pub recommended_actions: Option<Vec<String>>,
    pub related_verses: Option<Vec<String>>,
    pub related_hadiths: Option<Vec<String>>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Dhikr reminders for time-appropriate notifications
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DhikrReminder {
    pub id: Uuid,
    pub user_id: Uuid,
    
    // Dhikr information
    pub dhikr_category: DhikrCategory,
    pub dhikr_text_arabic: String,
    pub dhikr_text_transliteration: Option<String>,
    pub dhikr_text_translation: Option<String>,
    pub dhikr_reference: Option<String>, // Source reference
    
    // Timing settings
    pub trigger_time: Option<NaiveTime>, // For fixed time dhikr (morning/evening)
    pub trigger_after_prayer: Option<PrayerName>, // For post-prayer dhikr
    pub trigger_condition: Option<String>, // Custom conditions
    
    // Notification preferences
    pub is_active: bool,
    pub frequency: String,
    pub priority: NotificationPriority,
    
    // Repetition and tracking
    pub recommended_repetitions: i32,
    pub track_completion: bool,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User notification preferences
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserNotificationPreferences {
    pub id: Uuid,
    pub user_id: Uuid,
    
    // Global notification settings
    pub notifications_enabled: bool,
    pub quiet_hours_start: NaiveTime,
    pub quiet_hours_end: NaiveTime,
    
    // Prayer notification preferences
    pub prayer_notifications_enabled: bool,
    pub prayer_graduated_enabled: bool,
    pub prayer_reminder_intervals: Vec<i32>,
    
    // Sunnah and Nafl preferences
    pub sunnah_reminders_enabled: bool,
    pub nafl_reminders_enabled: bool,
    
    // Dhikr preferences
    pub dhikr_reminders_enabled: bool,
    pub morning_dhikr_time: NaiveTime,
    pub evening_dhikr_time: NaiveTime,
    
    // Seasonal preferences
    pub seasonal_reminders_enabled: bool,
    pub ramadan_reminders_enabled: bool,
    pub hajj_reminders_enabled: bool,
    
    // Delivery preferences
    pub push_notifications: bool,
    pub email_notifications: bool,
    pub sms_notifications: bool,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Notification delivery log for tracking and analytics
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NotificationDeliveryLog {
    pub id: Uuid,
    pub notification_id: Uuid,
    pub user_id: Uuid,
    
    // Delivery details
    pub delivery_method: String, // push, email, sms
    pub delivery_status: NotificationStatus,
    pub delivery_attempt: i32,
    
    // Response tracking
    pub opened_at: Option<DateTime<Utc>>,
    pub clicked_at: Option<DateTime<Utc>>,
    pub dismissed_at: Option<DateTime<Utc>>,
    
    // Error information
    pub error_message: Option<String>,
    pub error_code: Option<String>,
    
    pub created_at: DateTime<Utc>,
}

/// Pre-defined dhikr content for common times
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DefaultDhikrContent {
    pub id: Uuid,
    pub category: DhikrCategory,
    pub title: String,
    pub arabic_text: String,
    pub transliteration: Option<String>,
    pub translation_en: Option<String>,
    pub translation_ar: Option<String>,
    pub reference: Option<String>,
    pub repetitions: i32,
    pub order_index: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// Request models for API endpoints

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub body: String,
    pub priority: Option<NotificationPriority>,
    pub scheduled_at: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePrayerNotificationRequest {
    pub user_id: Uuid,
    pub prayer_name: PrayerName,
    pub prayer_time: DateTime<Utc>,
    pub enable_graduated: Option<bool>,
    pub reminder_intervals: Option<Vec<i32>>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
    pub enable_adhan: Option<bool>,
    pub enable_vibration: Option<bool>,
    pub custom_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSunnahReminderRequest {
    pub user_id: Uuid,
    pub sunnah_name: String,
    pub sunnah_description: Option<String>,
    pub sunnah_reference: Option<String>,
    pub reminder_time: NaiveTime,
    pub frequency: Option<String>,
    pub days_of_week: Option<Vec<i32>>,
    pub priority: Option<NotificationPriority>,
    pub custom_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSeasonalReminderRequest {
    pub user_id: Uuid,
    pub season: IslamicSeason,
    pub event_name: String,
    pub event_description: Option<String>,
    pub hijri_month: Option<i32>,
    pub hijri_day: Option<i32>,
    pub gregorian_date: Option<NaiveDate>,
    pub days_before_notification: Option<i32>,
    pub priority: Option<NotificationPriority>,
    pub reminder_message: Option<String>,
    pub recommended_actions: Option<Vec<String>>,
    pub related_verses: Option<Vec<String>>,
    pub related_hadiths: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDhikrReminderRequest {
    pub user_id: Uuid,
    pub dhikr_category: DhikrCategory,
    pub dhikr_text_arabic: String,
    pub dhikr_text_transliteration: Option<String>,
    pub dhikr_text_translation: Option<String>,
    pub dhikr_reference: Option<String>,
    pub trigger_time: Option<NaiveTime>,
    pub trigger_after_prayer: Option<PrayerName>,
    pub trigger_condition: Option<String>,
    pub frequency: Option<String>,
    pub priority: Option<NotificationPriority>,
    pub recommended_repetitions: Option<i32>,
    pub track_completion: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNotificationPreferencesRequest {
    pub notifications_enabled: Option<bool>,
    pub quiet_hours_start: Option<NaiveTime>,
    pub quiet_hours_end: Option<NaiveTime>,
    pub prayer_notifications_enabled: Option<bool>,
    pub prayer_graduated_enabled: Option<bool>,
    pub prayer_reminder_intervals: Option<Vec<i32>>,
    pub sunnah_reminders_enabled: Option<bool>,
    pub nafl_reminders_enabled: Option<bool>,
    pub dhikr_reminders_enabled: Option<bool>,
    pub morning_dhikr_time: Option<NaiveTime>,
    pub evening_dhikr_time: Option<NaiveTime>,
    pub seasonal_reminders_enabled: Option<bool>,
    pub ramadan_reminders_enabled: Option<bool>,
    pub hajj_reminders_enabled: Option<bool>,
    pub push_notifications: Option<bool>,
    pub email_notifications: Option<bool>,
    pub sms_notifications: Option<bool>,
}

/// Response models

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub body: String,
    pub priority: NotificationPriority,
    pub status: NotificationStatus,
    pub scheduled_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<NotificationResponse>,
    pub total_count: i64,
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationStatsResponse {
    pub total_notifications: i64,
    pub pending_notifications: i64,
    pub sent_notifications: i64,
    pub delivered_notifications: i64,
    pub read_notifications: i64,
    pub failed_notifications: i64,
}

impl From<Notification> for NotificationResponse {
    fn from(notification: Notification) -> Self {
        Self {
            id: notification.id,
            notification_type: notification.notification_type,
            title: notification.title,
            body: notification.body,
            priority: notification.priority,
            status: notification.status,
            scheduled_at: notification.scheduled_at,
            sent_at: notification.sent_at,
            metadata: notification.metadata,
            created_at: notification.created_at,
        }
    }
}
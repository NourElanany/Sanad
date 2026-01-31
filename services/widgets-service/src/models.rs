use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Widget types available in the Islamic app
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(type_name = "widget_type", rename_all = "snake_case"))]
pub enum WidgetType {
    NextPrayerTime,
    VerseOfTheDay,
    KhatmaProgress,
    IslamicCalendar,
    DhikrReminder,
    QuickStats,
    RecentActivity,
    Notifications,
}

/// Widget size options for responsive layout
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(type_name = "widget_size", rename_all = "snake_case"))]
pub enum WidgetSize {
    Small,   // 1x1 grid
    Medium,  // 2x1 or 1x2 grid
    Large,   // 2x2 grid
    Wide,    // 3x1 or 4x1 grid
    Tall,    // 1x3 or 1x4 grid
}

/// Widget position and layout information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetLayout {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub size: WidgetSize,
}

/// Widget configuration and preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(sqlx::FromRow))]
pub struct Widget {
    pub id: Uuid,
    pub user_id: Uuid,
    pub widget_type: WidgetType,
    pub title: String,
    pub is_enabled: bool,
    pub layout: serde_json::Value, // WidgetLayout as JSON
    pub configuration: serde_json::Value, // Widget-specific config
    pub refresh_interval_minutes: i32,
    pub last_updated: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Next prayer time widget data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextPrayerTimeWidget {
    pub prayer_name: String,
    pub prayer_name_arabic: String,
    pub prayer_time: DateTime<Utc>,
    pub time_remaining: String, // e.g., "2h 30m"
    pub time_remaining_minutes: i32,
    pub location: Option<String>,
    pub qibla_direction: Option<f64>, // degrees from north
    pub is_prayer_time: bool, // true if it's currently prayer time
    pub next_prayer_after_current: Option<NextPrayerInfo>,
}

/// Information about the prayer after the current next prayer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextPrayerInfo {
    pub prayer_name: String,
    pub prayer_name_arabic: String,
    pub prayer_time: DateTime<Utc>,
}

/// Verse of the day widget data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerseOfTheDayWidget {
    pub surah_number: u8,
    pub surah_name: String,
    pub surah_name_arabic: String,
    pub ayah_number: u16,
    pub ayah_text_arabic: String,
    pub ayah_text_transliteration: Option<String>,
    pub ayah_text_translation: Option<String>,
    pub tafsir_brief: Option<String>,
    pub tafsir_source: Option<String>,
    pub audio_url: Option<String>,
    pub share_url: String,
    pub date: DateTime<Utc>,
}

/// Khatma progress widget data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KhatmaProgressWidget {
    pub khatma_id: Option<Uuid>,
    pub is_active: bool,
    pub progress_percentage: f64, // 0.0 to 100.0
    pub current_surah: Option<String>,
    pub current_surah_arabic: Option<String>,
    pub current_ayah: Option<u16>,
    pub target_completion_date: Option<DateTime<Utc>>,
    pub days_remaining: Option<i32>,
    pub daily_target_pages: Option<f64>,
    pub pages_read_today: f64,
    pub streak_days: u32,
    pub total_pages_read: u32,
    pub estimated_completion_date: Option<DateTime<Utc>>,
    pub is_on_track: bool,
    pub next_reading_suggestion: Option<ReadingSuggestion>,
}

/// Reading suggestion for Khatma progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingSuggestion {
    pub surah_start: u8,
    pub ayah_start: u16,
    pub surah_end: u8,
    pub ayah_end: u16,
    pub estimated_minutes: i32,
    pub suggested_time: Option<DateTime<Utc>>,
}

/// Islamic calendar widget data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicCalendarWidget {
    pub hijri_date: HijriDate,
    pub gregorian_date: DateTime<Utc>,
    pub islamic_events_today: Vec<IslamicEvent>,
    pub upcoming_events: Vec<IslamicEvent>,
    pub current_islamic_month_info: IslamicMonthInfo,
}

/// Hijri date representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HijriDate {
    pub day: u8,
    pub month: u8,
    pub year: u16,
    pub month_name_arabic: String,
    pub month_name_english: String,
    pub day_name_arabic: String,
    pub day_name_english: String,
}

/// Islamic event information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicEvent {
    pub name: String,
    pub name_arabic: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub hijri_date: HijriDate,
    pub event_type: IslamicEventType,
    pub significance: EventSignificance,
}

/// Types of Islamic events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IslamicEventType {
    Eid,
    HolyNight,
    HolyMonth,
    ProphetBirthday,
    HistoricalEvent,
    SeasonalObservance,
}

/// Significance levels for events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventSignificance {
    Major,
    Moderate,
    Minor,
}

/// Islamic month information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicMonthInfo {
    pub month_number: u8,
    pub month_name_arabic: String,
    pub month_name_english: String,
    pub significance: Option<String>,
    pub recommended_actions: Vec<String>,
    pub special_days: Vec<u8>, // days of the month that are special
}

/// Dhikr reminder widget data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DhikrReminderWidget {
    pub dhikr_text_arabic: String,
    pub dhikr_text_transliteration: Option<String>,
    pub dhikr_text_translation: Option<String>,
    pub dhikr_category: DhikrCategory,
    pub repetitions: i32,
    pub completed_today: i32,
    pub source_reference: Option<String>,
    pub audio_url: Option<String>,
    pub next_dhikr_time: Option<DateTime<Utc>>,
}

/// Dhikr categories for time-appropriate reminders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(type_name = "dhikr_category", rename_all = "snake_case"))]
pub enum DhikrCategory {
    Morning,
    Evening,
    AfterPrayer,
    BeforeSleep,
    AfterWudu,
    Travel,
    General,
}

/// Quick stats widget data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickStatsWidget {
    pub prayers_completed_today: u8,
    pub prayers_total_today: u8,
    pub quran_pages_read_today: f64,
    pub dhikr_completed_today: i32,
    pub current_streak_days: u32,
    pub total_khatmas_completed: u32,
    pub monthly_reading_goal_progress: f64, // percentage
    pub weekly_consistency_score: f64, // 0.0 to 1.0
}

/// Recent activity widget data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentActivityWidget {
    pub recent_activities: Vec<ActivityItem>,
    pub activity_summary: ActivitySummary,
}

/// Individual activity item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityItem {
    pub activity_type: ActivityType,
    pub description: String,
    pub description_arabic: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub duration_minutes: Option<i32>,
    pub metadata: HashMap<String, String>,
}

/// Types of user activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    QuranReading,
    PrayerCompleted,
    DhikrCompleted,
    KhatmaProgress,
    AudioRecitation,
    SearchQuery,
    BookmarkAdded,
}

/// Summary of recent activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivitySummary {
    pub total_activities_today: u32,
    pub most_active_hour: Option<u8>,
    pub primary_activity_type: Option<ActivityType>,
    pub productivity_score: f64, // 0.0 to 1.0
}

/// Notifications widget data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsWidget {
    pub unread_count: u32,
    pub recent_notifications: Vec<NotificationItem>,
    pub priority_notifications: Vec<NotificationItem>,
}

/// Individual notification item for widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationItem {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub notification_type: String,
    pub priority: String,
    pub timestamp: DateTime<Utc>,
    pub is_read: bool,
    pub action_url: Option<String>,
}

/// Widget dashboard layout for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(sqlx::FromRow))]
pub struct WidgetDashboard {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub is_default: bool,
    pub layout_config: serde_json::Value, // Grid layout configuration
    pub widgets: serde_json::Value, // Widget IDs in order as JSON
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Widget data response containing all widget information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetDataResponse {
    pub widget_id: Uuid,
    pub widget_type: WidgetType,
    pub title: String,
    pub layout: WidgetLayout,
    pub data: WidgetData,
    pub last_updated: DateTime<Utc>,
    pub refresh_interval_minutes: i32,
}

/// Union type for all widget data types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WidgetData {
    NextPrayerTime(NextPrayerTimeWidget),
    VerseOfTheDay(VerseOfTheDayWidget),
    KhatmaProgress(KhatmaProgressWidget),
    IslamicCalendar(IslamicCalendarWidget),
    DhikrReminder(DhikrReminderWidget),
    QuickStats(QuickStatsWidget),
    RecentActivity(RecentActivityWidget),
    Notifications(NotificationsWidget),
}

/// Request models for API endpoints

#[derive(Debug, Deserialize)]
pub struct CreateWidgetRequest {
    pub widget_type: WidgetType,
    pub title: Option<String>,
    pub layout: WidgetLayout,
    pub configuration: Option<serde_json::Value>,
    pub refresh_interval_minutes: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWidgetRequest {
    pub title: Option<String>,
    pub is_enabled: Option<bool>,
    pub layout: Option<WidgetLayout>,
    pub configuration: Option<serde_json::Value>,
    pub refresh_interval_minutes: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDashboardRequest {
    pub name: String,
    pub is_default: Option<bool>,
    pub layout_config: Option<serde_json::Value>,
    pub widget_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDashboardRequest {
    pub name: Option<String>,
    pub is_default: Option<bool>,
    pub layout_config: Option<serde_json::Value>,
    pub widget_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize)]
pub struct WidgetConfigurationRequest {
    pub prayer_calculation_method: Option<String>,
    pub preferred_tafsir_source: Option<String>,
    pub language_preference: Option<String>,
    pub location_latitude: Option<f64>,
    pub location_longitude: Option<f64>,
    pub timezone: Option<String>,
}

/// Response models

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub dashboard: WidgetDashboard,
    pub widgets: Vec<WidgetDataResponse>,
}

#[derive(Debug, Serialize)]
pub struct WidgetListResponse {
    pub widgets: Vec<WidgetDataResponse>,
    pub total_count: usize,
}

#[derive(Debug, Serialize)]
pub struct AvailableWidgetsResponse {
    pub available_widgets: Vec<WidgetTypeInfo>,
}

#[derive(Debug, Serialize)]
pub struct WidgetTypeInfo {
    pub widget_type: WidgetType,
    pub name: String,
    pub name_arabic: String,
    pub description: String,
    pub description_arabic: String,
    pub default_size: WidgetSize,
    pub configurable_options: Vec<String>,
    pub refresh_interval_minutes: i32,
}

/// Error types for widget service
#[derive(Debug, thiserror::Error)]
pub enum WidgetError {
    #[error("Widget not found: {widget_id}")]
    WidgetNotFound { widget_id: Uuid },
    
    #[error("Dashboard not found: {dashboard_id}")]
    DashboardNotFound { dashboard_id: Uuid },
    
    #[error("Invalid widget configuration: {message}")]
    InvalidConfiguration { message: String },
    
    #[error("Widget data fetch failed: {widget_type:?} - {message}")]
    DataFetchFailed { widget_type: WidgetType, message: String },
    
    #[error("External service error: {service} - {message}")]
    ExternalServiceError { service: String, message: String },
    
    #[cfg(feature = "database")]
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Cache error: {0}")]
    CacheError(#[from] redis::RedisError),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),
}

/// Widget refresh status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetRefreshStatus {
    pub widget_id: Uuid,
    pub last_refresh: DateTime<Utc>,
    pub next_refresh: DateTime<Utc>,
    pub refresh_status: RefreshStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefreshStatus {
    Success,
    Failed,
    InProgress,
    Scheduled,
}

impl From<Widget> for WidgetDataResponse {
    fn from(widget: Widget) -> Self {
        let layout: WidgetLayout = serde_json::from_value(widget.layout)
            .unwrap_or(WidgetLayout {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
                size: WidgetSize::Small,
            });

        Self {
            widget_id: widget.id,
            widget_type: widget.widget_type,
            title: widget.title,
            layout,
            data: WidgetData::NextPrayerTime(NextPrayerTimeWidget {
                prayer_name: "Loading...".to_string(),
                prayer_name_arabic: "جاري التحميل...".to_string(),
                prayer_time: Utc::now(),
                time_remaining: "Loading...".to_string(),
                time_remaining_minutes: 0,
                location: None,
                qibla_direction: None,
                is_prayer_time: false,
                next_prayer_after_current: None,
            }), // Placeholder data
            last_updated: widget.last_updated,
            refresh_interval_minutes: widget.refresh_interval_minutes,
        }
    }
}
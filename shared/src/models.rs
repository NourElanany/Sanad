use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Common response wrapper for all API endpoints
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: Utc::now(),
        }
    }
}

/// User model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub preferences: UserPreferences,
    pub created_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
}

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub language: String,
    pub preferred_tafsir: Vec<String>,
    pub prayer_calculation_method: CalculationMethod,
    pub notification_settings: NotificationSettings,
    pub display_settings: DisplaySettings,
}

/// Prayer calculation methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CalculationMethod {
    MuslimWorldLeague,
    IslamicSocietyOfNorthAmerica,
    EgyptianGeneralAuthorityOfSurvey,
    UmmAlQuraUniversityMakkah,
    UniversityOfIslamicSciencesKarachi,
    InstituteOfGeophysicsUniversityOfTehran,
    Shia,
    Custom {
        fajr_angle: f64,
        maghrib_angle: f64,
        isha_angle: f64,
    },
}

/// Notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub prayer_reminders: bool,
    pub prayer_reminder_minutes: i32,
    pub islamic_events: bool,
    pub khatma_reminders: bool,
    pub daily_verse: bool,
}

/// Display settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub theme: String,
    pub font_size: String,
    pub arabic_font: String,
    pub translation_font: String,
}

/// Content types in the Islamic database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Quran,
    Hadith,
    Tafsir,
    Story,
    Article,
}

/// Islamic content base structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicContent {
    pub id: Uuid,
    pub content_type: ContentType,
    pub title: String,
    pub content: String,
    pub source: String,
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub language: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Hadith authenticity grades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HadithGrade {
    Sahih,   // صحيح
    Hasan,   // حسن
    Daif,    // ضعيف
    Mawdu,   // موضوع
}

/// Location for prayer times calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub city: Option<String>,
    pub country: Option<String>,
}

/// Prayer times
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerTimes {
    pub fajr: DateTime<Utc>,
    pub sunrise: DateTime<Utc>,
    pub dhuhr: DateTime<Utc>,
    pub asr: DateTime<Utc>,
    pub maghrib: DateTime<Utc>,
    pub isha: DateTime<Utc>,
    pub location: Location,
    pub calculation_method: CalculationMethod,
}

/// Hijri date
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HijriDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub month_name: String,
}

/// Islamic events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicEvent {
    pub name: String,
    pub description: String,
    pub hijri_date: HijriDate,
    pub gregorian_date: DateTime<Utc>,
    pub event_type: EventType,
}

/// Types of Islamic events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Eid,
    HolyMonth,
    ImportantDay,
    ProphetBirthday,
    CompanionCommemoration,
}

/// Search filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub content_types: Option<Vec<ContentType>>,
    pub sources: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub hadith_grades: Option<Vec<HadithGrade>>,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

/// Unified search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSearchResult {
    pub results: Vec<SearchResultItem>,
    pub total_results: usize,
    pub search_time_ms: u64,
    pub query: String,
}

/// Individual search result item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItem {
    pub content: IslamicContent,
    pub relevance_score: f64,
    pub highlighted_text: String,
    pub context: Option<String>,
}
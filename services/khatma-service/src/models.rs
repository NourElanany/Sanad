use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveTime};
use std::collections::HashMap;

/// Khatma plan with interactive planning capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KhatmaPlan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub target_date: DateTime<Utc>,
    pub start_date: DateTime<Utc>,
    pub daily_portions: Vec<DailyPortion>,
    pub estimated_reading_time: i32, // minutes per day
    pub adaptive_schedule: bool,
    pub current_progress: f64, // percentage (0.0 to 100.0)
    pub reading_speed_wpm: f64, // words per minute
    pub preferred_reading_times: Vec<PreferredReadingTime>,
    pub status: KhatmaStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Daily portion of Quran reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyPortion {
    pub date: DateTime<Utc>,
    pub surah_start: u8,
    pub ayah_start: u16,
    pub surah_end: u8,
    pub ayah_end: u16,
    pub estimated_minutes: i32,
    pub word_count: u32,
    pub completed: bool,
    pub actual_reading_time: Option<i32>, // actual time spent in minutes
    pub completion_date: Option<DateTime<Utc>>,
}

/// User's preferred reading times
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferredReadingTime {
    pub time: NaiveTime,
    pub duration_minutes: i32,
    pub priority: ReadingTimePriority,
    pub days_of_week: Vec<u8>, // 0 = Sunday, 1 = Monday, etc.
}

/// Priority levels for reading times
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReadingTimePriority {
    High,
    Medium,
    Low,
}

/// Khatma status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KhatmaStatus {
    Active,
    Completed,
    Paused,
    Cancelled,
}

/// Reading session tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub khatma_plan_id: Uuid,
    pub surah_start: u8,
    pub ayah_start: u16,
    pub surah_end: u8,
    pub ayah_end: u16,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
    pub word_count: u32,
    pub reading_speed_wpm: Option<f64>,
    pub created_at: DateTime<Utc>,
}

/// User reading statistics and patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingStatistics {
    pub user_id: Uuid,
    pub average_reading_speed_wpm: f64,
    pub total_reading_time_minutes: i32,
    pub completed_khatmas: u32,
    pub preferred_reading_times: Vec<PreferredReadingTime>,
    pub reading_consistency_score: f64, // 0.0 to 1.0
    pub last_updated: DateTime<Utc>,
}

/// Khatma preferences for planning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KhatmaPreferences {
    pub target_completion_days: Option<i32>,
    pub daily_reading_time_minutes: Option<i32>,
    pub preferred_reading_times: Vec<PreferredReadingTime>,
    pub adaptive_scheduling: bool,
    pub reminder_settings: ReminderSettings,
    pub difficulty_preference: DifficultyPreference,
}

/// Reminder settings for khatma
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderSettings {
    pub enabled: bool,
    pub advance_minutes: i32,
    pub smart_timing: bool, // Use AI to suggest optimal times
    pub missed_reading_reminder: bool,
    pub progress_updates: bool,
}

/// Difficulty preference for reading portions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyPreference {
    Easy,    // Shorter portions, more time
    Medium,  // Balanced approach
    Hard,    // Longer portions, less time
    Custom,  // User-defined parameters
}

/// Plan adjustment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanAdjustmentRequest {
    pub khatma_plan_id: Uuid,
    pub new_target_date: Option<DateTime<Utc>>,
    pub new_daily_time_minutes: Option<i32>,
    pub reason: AdjustmentReason,
    pub requested_at: DateTime<Utc>,
}

/// Reasons for plan adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdjustmentReason {
    BehindSchedule,
    AheadOfSchedule,
    TimeConstraintChange,
    UserRequest,
    SystemOptimization,
}

/// Smart reminder suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartReminder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub khatma_plan_id: Uuid,
    pub suggested_time: DateTime<Utc>,
    pub duration_minutes: i32,
    pub portion: DailyPortion,
    pub confidence_score: f64, // How confident the system is in this suggestion
    pub reasoning: String,
    pub created_at: DateTime<Utc>,
}

/// Khatma statistics for completed plans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KhatmaStatistics {
    pub khatma_plan_id: Uuid,
    pub completion_date: DateTime<Utc>,
    pub planned_duration_days: i32,
    pub actual_duration_days: i32,
    pub total_reading_time_minutes: i32,
    pub average_daily_reading_minutes: f64,
    pub consistency_score: f64,
    pub portions_completed_on_time: u32,
    pub portions_completed_late: u32,
    pub portions_skipped: u32,
    pub reading_speed_improvement: f64, // percentage improvement
    pub achievements: Vec<Achievement>,
}

/// Achievements for gamification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub earned_at: DateTime<Utc>,
    pub category: AchievementCategory,
}

/// Achievement categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementCategory {
    Consistency,
    Speed,
    Completion,
    Improvement,
    Dedication,
}

/// Request to create a new khatma plan
#[derive(Debug, Deserialize)]
pub struct CreateKhatmaPlanRequest {
    pub target_date: DateTime<Utc>,
    pub preferences: KhatmaPreferences,
}

/// Request to update reading progress
#[derive(Debug, Deserialize)]
pub struct UpdateProgressRequest {
    pub khatma_plan_id: Uuid,
    pub reading_session: ReadingSession,
}

/// Response for plan suggestions
#[derive(Debug, Serialize)]
pub struct PlanSuggestionResponse {
    pub suggested_plans: Vec<KhatmaPlan>,
    pub recommendations: Vec<String>,
    pub estimated_success_rate: f64,
}

/// Response for reading time suggestions
#[derive(Debug, Serialize)]
pub struct ReadingTimeSuggestionResponse {
    pub suggested_times: Vec<SmartReminder>,
    pub optimal_daily_schedule: HashMap<String, Vec<PreferredReadingTime>>, // day_of_week -> times
    pub reasoning: String,
}
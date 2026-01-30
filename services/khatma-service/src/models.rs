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

/// User behavior analysis for smart reminders
#[derive(Debug, Clone, Default)]
pub struct UserBehaviorAnalysis {
    pub preferred_hours: Vec<u32>,
    pub preferred_days: Vec<u32>,
    pub session_duration_patterns: (i32, i32, i32), // (min, avg, max)
    pub consistency_score: f64,
    pub streak_patterns: (u32, u32), // (current_streak, max_streak)
    pub missed_session_patterns: Vec<u32>,
}

/// Comprehensive progress dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressDashboard {
    pub user_id: Uuid,
    pub current_khatma: Option<KhatmaPlan>,
    pub overall_progress: OverallProgress,
    pub recent_activity: RecentActivity,
    pub performance_metrics: PerformanceMetrics,
    pub upcoming_milestones: Vec<Milestone>,
    pub recommendations: Vec<PerformanceRecommendation>,
    pub generated_at: DateTime<Utc>,
}

/// Overall progress statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallProgress {
    pub total_khatmas_completed: u32,
    pub current_khatma_progress: f64, // percentage
    pub total_reading_time_hours: f64,
    pub average_daily_reading_minutes: f64,
    pub consistency_score: f64, // 0.0 to 1.0
    pub current_streak_days: u32,
    pub longest_streak_days: u32,
    pub pages_read_total: u32,
    pub surahs_completed: u32,
}

/// Recent activity summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentActivity {
    pub last_7_days: ActivityPeriod,
    pub last_30_days: ActivityPeriod,
    pub this_month: ActivityPeriod,
    pub recent_sessions: Vec<ReadingSessionSummary>,
}

/// Activity data for a specific period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityPeriod {
    pub total_reading_time_minutes: i32,
    pub sessions_count: u32,
    pub average_session_duration: f64,
    pub consistency_percentage: f64,
    pub pages_read: u32,
    pub best_day: Option<BestDayInfo>,
}

/// Best day information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestDayInfo {
    pub date: DateTime<Utc>,
    pub reading_time_minutes: i32,
    pub pages_read: u32,
    pub achievement: String,
}

/// Reading session summary for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingSessionSummary {
    pub date: DateTime<Utc>,
    pub duration_minutes: i32,
    pub surah_range: String, // e.g., "Al-Baqarah 1-10"
    pub reading_speed_wpm: f64,
    pub quality_score: f64, // 0.0 to 1.0
}

/// Performance metrics and analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub reading_speed_trend: SpeedTrend,
    pub consistency_trend: ConsistencyTrend,
    pub optimal_reading_times: Vec<OptimalTimeSlot>,
    pub productivity_patterns: ProductivityPatterns,
    pub goal_achievement_rate: f64, // percentage of goals met
}

/// Reading speed trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedTrend {
    pub current_wpm: f64,
    pub average_wpm: f64,
    pub improvement_percentage: f64,
    pub trend_direction: TrendDirection,
    pub weekly_speeds: Vec<WeeklySpeed>,
}

/// Trend direction indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
}

/// Weekly speed data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklySpeed {
    pub week_start: DateTime<Utc>,
    pub average_wpm: f64,
    pub sessions_count: u32,
}

/// Consistency trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyTrend {
    pub current_score: f64,
    pub trend_direction: TrendDirection,
    pub weekly_consistency: Vec<WeeklyConsistency>,
    pub best_consistency_period: Option<ConsistencyPeriod>,
}

/// Weekly consistency data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyConsistency {
    pub week_start: DateTime<Utc>,
    pub consistency_score: f64,
    pub days_read: u32,
    pub target_days: u32,
}

/// Best consistency period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyPeriod {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub consistency_score: f64,
    pub duration_days: u32,
}

/// Optimal time slot for reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimalTimeSlot {
    pub hour: u32,
    pub success_rate: f64, // percentage of successful sessions at this hour
    pub average_duration: i32,
    pub average_speed: f64,
    pub recommendation_strength: RecommendationStrength,
}

/// Recommendation strength levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationStrength {
    Strong,
    Moderate,
    Weak,
}

/// Productivity patterns analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductivityPatterns {
    pub best_days_of_week: Vec<DayProductivity>,
    pub best_times_of_day: Vec<HourProductivity>,
    pub session_length_effectiveness: SessionLengthAnalysis,
    pub environmental_factors: EnvironmentalFactors,
}

/// Day of week productivity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayProductivity {
    pub day_of_week: u32, // 0 = Sunday
    pub day_name: String,
    pub average_reading_time: i32,
    pub consistency_rate: f64,
    pub productivity_score: f64,
}

/// Hour of day productivity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourProductivity {
    pub hour: u32,
    pub sessions_count: u32,
    pub average_duration: i32,
    pub average_speed: f64,
    pub completion_rate: f64,
}

/// Session length effectiveness analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLengthAnalysis {
    pub optimal_duration_minutes: i32,
    pub short_sessions_effectiveness: f64, // < 20 minutes
    pub medium_sessions_effectiveness: f64, // 20-45 minutes
    pub long_sessions_effectiveness: f64, // > 45 minutes
    pub recommendation: String,
}

/// Environmental factors affecting reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalFactors {
    pub weekend_vs_weekday_performance: f64, // ratio
    pub morning_vs_evening_preference: f64, // ratio
    pub consistency_impact_score: f64,
}

/// Milestone tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: String,
    pub title: String,
    pub description: String,
    pub target_date: DateTime<Utc>,
    pub progress_percentage: f64,
    pub milestone_type: MilestoneType,
    pub reward: Option<String>,
}

/// Types of milestones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MilestoneType {
    KhatmaCompletion,
    ReadingStreak,
    SpeedImprovement,
    ConsistencyGoal,
    TimeGoal,
}

/// Performance improvement recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: RecommendationCategory,
    pub priority: RecommendationPriority,
    pub expected_impact: String,
    pub action_steps: Vec<String>,
    pub confidence_score: f64,
}

/// Recommendation categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    TimeManagement,
    Consistency,
    ReadingSpeed,
    SessionOptimization,
    GoalSetting,
    Motivation,
}

/// Recommendation priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    High,
    Medium,
    Low,
}

/// Khatma comparison data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KhatmaComparison {
    pub user_id: Uuid,
    pub current_khatma: Option<KhatmaPlan>,
    pub previous_khatmas: Vec<KhatmaStatistics>,
    pub comparison_metrics: ComparisonMetrics,
    pub improvement_areas: Vec<ImprovementArea>,
    pub achievements_comparison: AchievementsComparison,
    pub generated_at: DateTime<Utc>,
}

/// Comparison metrics between Khatmas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonMetrics {
    pub completion_time_comparison: TimeComparison,
    pub reading_speed_comparison: SpeedComparison,
    pub consistency_comparison: ConsistencyComparison,
    pub overall_improvement_score: f64, // 0.0 to 1.0
}

/// Time-based comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeComparison {
    pub current_pace_days: Option<i32>,
    pub average_previous_pace_days: f64,
    pub best_previous_pace_days: i32,
    pub improvement_percentage: f64,
    pub trend: TrendDirection,
}

/// Speed-based comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedComparison {
    pub current_average_wpm: Option<f64>,
    pub previous_average_wpm: f64,
    pub best_previous_wpm: f64,
    pub improvement_percentage: f64,
    pub trend: TrendDirection,
}

/// Consistency-based comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyComparison {
    pub current_consistency_score: Option<f64>,
    pub previous_average_consistency: f64,
    pub best_previous_consistency: f64,
    pub improvement_percentage: f64,
    pub trend: TrendDirection,
}

/// Areas for improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementArea {
    pub area: String,
    pub current_performance: f64,
    pub target_performance: f64,
    pub improvement_potential: f64,
    pub specific_recommendations: Vec<String>,
}

/// Achievements comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementsComparison {
    pub total_achievements_earned: u32,
    pub new_achievements_this_khatma: u32,
    pub achievement_categories_progress: HashMap<String, u32>,
    pub rarest_achievement: Option<Achievement>,
}

/// Request for generating dashboard
#[derive(Debug, Deserialize)]
pub struct DashboardRequest {
    pub include_recommendations: Option<bool>,
    pub include_comparisons: Option<bool>,
    pub time_period_days: Option<i32>,
}

/// Request for Khatma comparison
#[derive(Debug, Deserialize)]
pub struct ComparisonRequest {
    pub compare_with_count: Option<u32>, // Number of previous Khatmas to compare with
    pub include_detailed_metrics: Option<bool>,
}
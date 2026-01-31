use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveTime};
use std::collections::HashMap;

/// User behavior profile for smart customization
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserBehaviorProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    
    // Reading patterns
    pub preferred_reading_times: Vec<PreferredTimeSlot>,
    pub average_session_duration: i32, // minutes
    pub reading_consistency_score: f64, // 0.0 to 1.0
    pub preferred_content_types: Vec<ContentTypePreference>,
    
    // Interaction patterns
    pub notification_response_rate: f64, // 0.0 to 1.0
    pub preferred_notification_times: Vec<NaiveTime>,
    pub engagement_patterns: EngagementPatterns,
    
    // Learning preferences
    pub learning_style: LearningStyle,
    pub difficulty_preference: DifficultyLevel,
    pub language_preferences: Vec<String>,
    
    // Seasonal and contextual patterns
    pub seasonal_preferences: HashMap<String, SeasonalPreference>,
    pub location_based_preferences: Option<LocationPreferences>,
    
    // Adaptive metrics
    pub adaptation_score: f64, // How well the system adapts to user
    pub satisfaction_score: f64, // User satisfaction with recommendations
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Preferred time slot for activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferredTimeSlot {
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub activity_type: ActivityType,
    pub preference_strength: f64, // 0.0 to 1.0
    pub days_of_week: Vec<u8>, // 0 = Sunday
    pub success_rate: f64, // Historical success rate for this time slot
}

/// Types of activities for time preferences
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "activity_type", rename_all = "snake_case")]
pub enum ActivityType {
    QuranReading,
    HadithStudy,
    DhikrReminders,
    PrayerReminders,
    IslamicStories,
    Learning,
    Reflection,
}

/// Content type preferences with weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentTypePreference {
    pub content_type: ContentType,
    pub preference_weight: f64, // 0.0 to 1.0
    pub interaction_frequency: f64, // How often user interacts with this type
    pub completion_rate: f64, // How often user completes this type of content
}

/// Types of Islamic content
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "content_type", rename_all = "snake_case")]
pub enum ContentType {
    QuranVerses,
    HadithNarrations,
    IslamicStories,
    Tafsir,
    Dhikr,
    Duas,
    IslamicHistory,
    Fiqh,
    Aqeedah,
    Seerah,
}

/// User engagement patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementPatterns {
    pub peak_engagement_hours: Vec<u8>, // Hours of day (0-23)
    pub peak_engagement_days: Vec<u8>, // Days of week (0-6)
    pub average_session_length: i32, // minutes
    pub preferred_content_length: ContentLength,
    pub interaction_style: InteractionStyle,
    pub motivation_triggers: Vec<MotivationTrigger>,
}

/// Preferred content length
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "content_length", rename_all = "snake_case")]
pub enum ContentLength {
    Short,   // < 5 minutes
    Medium,  // 5-15 minutes
    Long,    // 15-30 minutes
    Extended, // > 30 minutes
}

/// User interaction style
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "interaction_style", rename_all = "snake_case")]
pub enum InteractionStyle {
    Casual,      // Occasional, relaxed interaction
    Structured,  // Prefers scheduled, organized approach
    Intensive,   // Deep, focused sessions
    Social,      // Prefers community features
    Independent, // Self-directed learning
}

/// Motivation triggers that work for the user
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "motivation_trigger", rename_all = "snake_case")]
pub enum MotivationTrigger {
    Progress,        // Progress tracking and achievements
    Community,       // Social features and sharing
    Reminders,       // Gentle reminders and notifications
    Challenges,      // Goals and challenges
    Rewards,         // Gamification and rewards
    Spiritual,       // Spiritual motivation and reflection
    Knowledge,       // Learning and knowledge acquisition
}

/// Learning style preferences
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "learning_style", rename_all = "snake_case")]
pub enum LearningStyle {
    Visual,      // Prefers visual content and layouts
    Auditory,    // Prefers audio content and recitations
    Reading,     // Prefers text-based content
    Kinesthetic, // Prefers interactive content
    Mixed,       // Combination of styles
}

/// Difficulty level preferences
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "difficulty_level", rename_all = "snake_case")]
pub enum DifficultyLevel {
    Beginner,     // Simple, basic content
    Intermediate, // Moderate complexity
    Advanced,     // Complex, detailed content
    Scholar,      // Academic level content
    Adaptive,     // System chooses based on performance
}

/// Seasonal preferences for different times of year
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalPreference {
    pub season: IslamicSeason,
    pub content_focus: Vec<ContentType>,
    pub activity_increase: f64, // Multiplier for activity during this season
    pub preferred_reminders: Vec<ReminderType>,
    pub special_interests: Vec<String>,
}

/// Islamic seasons and special periods
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "islamic_season", rename_all = "snake_case")]
pub enum IslamicSeason {
    Ramadan,
    DhulHijjah,
    Muharram,
    Rajab,
    Shaban,
    LaylatAlQadr,
    EidAlFitr,
    EidAlAdha,
    Ashura,
    Mawlid,
    IsraMiraj,
    Regular, // Non-special periods
}

/// Types of reminders
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "reminder_type", rename_all = "snake_case")]
pub enum ReminderType {
    Prayer,
    Dhikr,
    QuranReading,
    Charity,
    Fasting,
    Reflection,
    Learning,
    Community,
}

/// Location-based preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationPreferences {
    pub timezone: String,
    pub prayer_calculation_method: String,
    pub local_islamic_events: bool,
    pub community_features: bool,
    pub language_region: String,
}

/// Personalized content recommendation
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PersonalizedRecommendation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content_type: ContentType,
    pub content_id: String, // Reference to actual content
    pub title: String,
    pub description: String,
    pub recommendation_score: f64, // 0.0 to 1.0
    pub reasoning: String, // Why this was recommended
    
    // Recommendation metadata
    pub estimated_duration: i32, // minutes
    pub difficulty_level: DifficultyLevel,
    pub tags: Vec<String>,
    pub category: RecommendationCategory,
    
    // Tracking
    pub presented_at: Option<DateTime<Utc>>,
    pub interacted_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub user_rating: Option<f64>, // 1.0 to 5.0
    pub feedback: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Categories of recommendations
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "recommendation_category", rename_all = "snake_case")]
pub enum RecommendationCategory {
    DailyReading,     // Daily Quran/Hadith reading
    Seasonal,         // Seasonal content (Ramadan, Hajj, etc.)
    Learning,         // Educational content
    Spiritual,        // Spiritual development
    Community,        // Community-related content
    Personal,         // Based on personal interests
    Trending,         // Popular content
    Continuation,     // Continue previous content
    Discovery,        // New content to explore
}

/// Adaptive reminder with smart timing
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AdaptiveReminder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub reminder_type: ReminderType,
    pub title: String,
    pub message: String,
    
    // Smart timing
    pub suggested_time: DateTime<Utc>,
    pub optimal_time_window: TimeWindow,
    pub adaptation_confidence: f64, // How confident the system is in timing
    
    // Personalization
    pub personalization_factors: Vec<PersonalizationFactor>,
    pub content_customization: ContentCustomization,
    
    // Tracking and learning
    pub response_prediction: f64, // Predicted response rate
    pub actual_response: Option<ReminderResponse>,
    pub effectiveness_score: Option<f64>,
    
    // Scheduling
    pub is_recurring: bool,
    pub recurrence_pattern: Option<RecurrencePattern>,
    pub next_occurrence: Option<DateTime<Utc>>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Time window for optimal reminder delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub preferred_time: NaiveTime,
    pub flexibility_minutes: i32, // How much the time can vary
}

/// Factors used for personalization
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "personalization_factor", rename_all = "snake_case")]
pub enum PersonalizationFactor {
    HistoricalResponse,  // Based on past responses
    CurrentContext,      // Current time, location, etc.
    UserMood,           // Inferred user mood/state
    ActivityPattern,    // Current activity patterns
    SeasonalContext,    // Islamic calendar context
    PersonalGoals,      // User's stated goals
    ProgressStatus,     // Current progress in various areas
    SocialContext,      // Community activity
}

/// Content customization for reminders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentCustomization {
    pub language: String,
    pub tone: MessageTone,
    pub length: MessageLength,
    pub include_verse: bool,
    pub include_hadith: bool,
    pub include_motivation: bool,
    pub personalized_elements: Vec<String>, // User name, progress, etc.
}

/// Tone of reminder messages
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "message_tone", rename_all = "snake_case")]
pub enum MessageTone {
    Gentle,        // Soft, encouraging
    Motivational,  // Inspiring, energetic
    Formal,        // Respectful, traditional
    Friendly,      // Casual, warm
    Urgent,        // Important, time-sensitive
    Reflective,    // Thoughtful, contemplative
}

/// Length of reminder messages
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "message_length", rename_all = "snake_case")]
pub enum MessageLength {
    Brief,    // One sentence
    Short,    // 2-3 sentences
    Medium,   // Paragraph
    Detailed, // Multiple paragraphs
}

/// User response to reminders
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "reminder_response", rename_all = "snake_case")]
pub enum ReminderResponse {
    Ignored,      // No interaction
    Dismissed,    // Actively dismissed
    Postponed,    // Snoozed for later
    Acknowledged, // Viewed but no action
    Acted,        // Took the suggested action
    Completed,    // Fully completed the activity
}

/// Recurrence pattern for reminders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurrencePattern {
    pub frequency: RecurrenceFrequency,
    pub interval: i32, // Every N units
    pub days_of_week: Option<Vec<u8>>, // For weekly patterns
    pub days_of_month: Option<Vec<u8>>, // For monthly patterns
    pub end_condition: EndCondition,
}

/// Frequency of recurrence
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "recurrence_frequency", rename_all = "snake_case")]
pub enum RecurrenceFrequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Custom,
}

/// When to end recurring reminders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndCondition {
    Never,
    AfterOccurrences(u32),
    UntilDate(DateTime<Utc>),
    WhenGoalMet,
}

/// User preference learning record
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PreferenceLearningRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub preference_type: PreferenceType,
    pub old_value: serde_json::Value,
    pub new_value: serde_json::Value,
    pub confidence_score: f64, // How confident we are in this learning
    pub learning_source: LearningSource,
    pub validation_status: ValidationStatus,
    pub impact_score: f64, // How much this affects user experience
    pub created_at: DateTime<Utc>,
}

/// Types of preferences that can be learned
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "preference_type", rename_all = "snake_case")]
pub enum PreferenceType {
    ReadingTime,
    ContentType,
    NotificationTiming,
    SessionDuration,
    DifficultyLevel,
    LanguagePreference,
    InteractionStyle,
    MotivationTrigger,
    SeasonalPattern,
}

/// Source of preference learning
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "learning_source", rename_all = "snake_case")]
pub enum LearningSource {
    UserBehavior,     // Inferred from behavior
    ExplicitFeedback, // User explicitly stated
    InteractionPattern, // Pattern analysis
    ResponseRate,     // Response to recommendations
    CompletionRate,   // Task completion patterns
    TimeAnalysis,     // Time-based analysis
    ContextualClues,  // Environmental factors
}

/// Status of learned preference validation
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "validation_status", rename_all = "snake_case")]
pub enum ValidationStatus {
    Pending,     // Needs validation
    Confirmed,   // Validated as correct
    Rejected,    // Proven incorrect
    Uncertain,   // Mixed signals
    Expired,     // Too old to be relevant
}

/// Customization analytics and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationAnalytics {
    pub user_id: Uuid,
    pub analysis_period: AnalysisPeriod,
    
    // Effectiveness metrics
    pub recommendation_accuracy: f64, // How often recommendations are accepted
    pub reminder_effectiveness: f64, // How effective reminders are
    pub personalization_score: f64, // Overall personalization quality
    
    // Engagement metrics
    pub engagement_improvement: f64, // Change in engagement over time
    pub satisfaction_trend: f64, // User satisfaction trend
    pub retention_impact: f64, // Impact on user retention
    
    // Learning metrics
    pub preference_stability: f64, // How stable learned preferences are
    pub adaptation_speed: f64, // How quickly system adapts
    pub prediction_accuracy: f64, // Accuracy of behavior predictions
    
    // Content metrics
    pub content_diversity: f64, // Variety in recommended content
    pub content_relevance: f64, // Relevance of content to user
    pub completion_rate_improvement: f64, // Improvement in completion rates
    
    pub generated_at: DateTime<Utc>,
}

/// Time period for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisPeriod {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub period_type: PeriodType,
}

/// Types of analysis periods
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "period_type", rename_all = "snake_case")]
pub enum PeriodType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
    Custom,
}

// Request and Response models

/// Request to create or update user behavior profile
#[derive(Debug, Deserialize)]
pub struct UpdateBehaviorProfileRequest {
    pub preferred_reading_times: Option<Vec<PreferredTimeSlot>>,
    pub preferred_content_types: Option<Vec<ContentTypePreference>>,
    pub learning_style: Option<LearningStyle>,
    pub difficulty_preference: Option<DifficultyLevel>,
    pub language_preferences: Option<Vec<String>>,
    pub notification_preferences: Option<NotificationPreferencesUpdate>,
    pub seasonal_preferences: Option<HashMap<String, SeasonalPreference>>,
    pub location_preferences: Option<LocationPreferences>,
}

/// Notification preferences update
#[derive(Debug, Deserialize)]
pub struct NotificationPreferencesUpdate {
    pub preferred_times: Option<Vec<NaiveTime>>,
    pub response_rate_target: Option<f64>,
    pub tone_preference: Option<MessageTone>,
    pub length_preference: Option<MessageLength>,
}

/// Request for personalized recommendations
#[derive(Debug, Deserialize)]
pub struct RecommendationRequest {
    pub content_types: Option<Vec<ContentType>>,
    pub categories: Option<Vec<RecommendationCategory>>,
    pub max_recommendations: Option<u32>,
    pub time_context: Option<DateTime<Utc>>,
    pub session_duration: Option<i32>, // Available time in minutes
    pub difficulty_override: Option<DifficultyLevel>,
}

/// Request for adaptive reminders
#[derive(Debug, Deserialize)]
pub struct AdaptiveReminderRequest {
    pub reminder_types: Option<Vec<ReminderType>>,
    pub time_window: Option<TimeWindow>,
    pub max_reminders: Option<u32>,
    pub urgency_level: Option<UrgencyLevel>,
    pub context: Option<ReminderContext>,
}

/// Urgency level for reminders
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "urgency_level", rename_all = "snake_case")]
pub enum UrgencyLevel {
    Low,      // Can wait
    Normal,   // Standard priority
    High,     // Important
    Critical, // Urgent, time-sensitive
}

/// Context for reminder generation
#[derive(Debug, Deserialize)]
pub struct ReminderContext {
    pub current_activity: Option<String>,
    pub location_context: Option<String>,
    pub time_constraints: Option<TimeConstraints>,
    pub mood_context: Option<String>,
    pub social_context: Option<String>,
}

/// Time constraints for activities
#[derive(Debug, Deserialize)]
pub struct TimeConstraints {
    pub available_minutes: i32,
    pub hard_deadline: Option<DateTime<Utc>>,
    pub preferred_completion: Option<DateTime<Utc>>,
    pub flexibility: FlexibilityLevel,
}

/// How flexible timing can be
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "flexibility_level", rename_all = "snake_case")]
pub enum FlexibilityLevel {
    Rigid,     // Must be exact time
    Limited,   // ±15 minutes
    Moderate,  // ±30 minutes
    Flexible,  // ±1 hour
    VeryFlexible, // ±2+ hours
}

/// Response models

/// Response with personalized recommendations
#[derive(Debug, Serialize)]
pub struct RecommendationResponse {
    pub recommendations: Vec<PersonalizedRecommendation>,
    pub total_count: u32,
    pub personalization_score: f64,
    pub reasoning: String,
    pub next_update: DateTime<Utc>,
}

/// Response with adaptive reminders
#[derive(Debug, Serialize)]
pub struct AdaptiveReminderResponse {
    pub reminders: Vec<AdaptiveReminder>,
    pub optimization_score: f64,
    pub adaptation_reasoning: String,
    pub next_optimization: DateTime<Utc>,
}

/// Response with behavior profile
#[derive(Debug, Serialize)]
pub struct BehaviorProfileResponse {
    pub profile: UserBehaviorProfile,
    pub learning_insights: Vec<LearningInsight>,
    pub recommendations_for_improvement: Vec<String>,
    pub confidence_score: f64,
}

/// Learning insight about user behavior
#[derive(Debug, Serialize)]
pub struct LearningInsight {
    pub insight_type: InsightType,
    pub description: String,
    pub confidence: f64,
    pub impact: String,
    pub actionable_suggestion: Option<String>,
}

/// Types of insights
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "insight_type", rename_all = "snake_case")]
pub enum InsightType {
    Pattern,      // Behavioral pattern discovered
    Preference,   // Preference learned
    Opportunity,  // Improvement opportunity
    Trend,        // Trend in behavior
    Anomaly,      // Unusual behavior
    Achievement,  // Positive development
}

/// Response with customization analytics
#[derive(Debug, Serialize)]
pub struct AnalyticsResponse {
    pub analytics: CustomizationAnalytics,
    pub insights: Vec<LearningInsight>,
    pub improvement_suggestions: Vec<ImprovementSuggestion>,
    pub benchmark_comparison: Option<BenchmarkComparison>,
}

/// Suggestion for improving customization
#[derive(Debug, Serialize)]
pub struct ImprovementSuggestion {
    pub area: String,
    pub current_score: f64,
    pub target_score: f64,
    pub actions: Vec<String>,
    pub expected_impact: f64,
    pub priority: Priority,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "priority", rename_all = "snake_case")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Comparison with benchmarks
#[derive(Debug, Serialize)]
pub struct BenchmarkComparison {
    pub user_score: f64,
    pub average_score: f64,
    pub percentile: f64,
    pub areas_above_average: Vec<String>,
    pub areas_below_average: Vec<String>,
}
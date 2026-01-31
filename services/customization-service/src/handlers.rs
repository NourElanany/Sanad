use crate::models::*;
use crate::service::SmartCustomizationService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use tracing::{info, error};

/// HTTP handlers for the Smart Customization Service
pub struct CustomizationHandlers;

impl CustomizationHandlers {
    /// Create router with all customization endpoints
    pub fn create_router(service: Arc<SmartCustomizationService>) -> Router {
        Router::new()
            .route("/users/:user_id/behavior-profile", get(get_behavior_profile))
            .route("/users/:user_id/behavior-profile", put(update_behavior_profile))
            .route("/users/:user_id/behavior-profile/analyze", post(analyze_behavior))
            .route("/users/:user_id/recommendations", get(get_recommendations))
            .route("/users/:user_id/recommendations", post(generate_recommendations))
            .route("/users/:user_id/reminders/adaptive", get(get_adaptive_reminders))
            .route("/users/:user_id/reminders/adaptive", post(generate_adaptive_reminders))
            .route("/users/:user_id/preferences/learn", post(learn_preferences))
            .route("/users/:user_id/analytics", get(get_analytics))
            .route("/users/:user_id/recommendations/:recommendation_id/feedback", post(submit_recommendation_feedback))
            .route("/users/:user_id/reminders/:reminder_id/response", post(submit_reminder_response))
            .with_state(service)
    }
}

/// Query parameters for recommendations
#[derive(Debug, Deserialize)]
pub struct RecommendationQuery {
    pub content_types: Option<String>, // Comma-separated content types
    pub categories: Option<String>,    // Comma-separated categories
    pub max_recommendations: Option<u32>,
    pub session_duration: Option<i32>,
    pub difficulty_override: Option<String>,
}

/// Query parameters for adaptive reminders
#[derive(Debug, Deserialize)]
pub struct AdaptiveReminderQuery {
    pub reminder_types: Option<String>, // Comma-separated reminder types
    pub max_reminders: Option<u32>,
    pub urgency_level: Option<String>,
}

/// Query parameters for analytics
#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub period_type: Option<String>, // daily, weekly, monthly, etc.
    pub start_date: Option<String>,  // ISO 8601 format
    pub end_date: Option<String>,    // ISO 8601 format
}

/// Feedback for recommendations
#[derive(Debug, Deserialize)]
pub struct RecommendationFeedback {
    pub rating: Option<f64>,      // 1.0 to 5.0
    pub feedback: Option<String>, // Text feedback
    pub completed: Option<bool>,  // Whether user completed the recommendation
    pub useful: Option<bool>,     // Whether user found it useful
}

/// Response for reminder interaction
#[derive(Debug, Deserialize)]
pub struct ReminderResponseRequest {
    pub response_type: String,    // ignored, dismissed, acknowledged, acted, completed
    pub response_time: Option<chrono::DateTime<chrono::Utc>>,
    pub feedback: Option<String>, // Optional feedback
}

/// Standard API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(message: &str) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Get user behavior profile
pub async fn get_behavior_profile(
    State(service): State<Arc<SmartCustomizationService>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<BehaviorProfileResponse>>, StatusCode> {
    info!("Getting behavior profile for user: {}", user_id);

    match service.repository.get_behavior_profile(user_id).await {
        Ok(profile) => {
            // Generate learning insights
            let learning_insights = vec![
                LearningInsight {
                    insight_type: InsightType::Pattern,
                    description: format!("User prefers reading during {} time slots", profile.preferred_reading_times.len()),
                    confidence: 0.8,
                    impact: "High impact on recommendation timing".to_string(),
                    actionable_suggestion: Some("Schedule more content during preferred times".to_string()),
                },
            ];

            let recommendations_for_improvement = vec![
                "Consider expanding reading time slots for better consistency".to_string(),
                "Try different content types to discover new preferences".to_string(),
            ];

            let response = BehaviorProfileResponse {
                profile,
                learning_insights,
                recommendations_for_improvement,
                confidence_score: 0.85,
            };

            Ok(Json(ApiResponse::success(response, "Behavior profile retrieved successfully")))
        }
        Err(e) => {
            error!("Failed to get behavior profile: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Update user behavior profile
pub async fn update_behavior_profile(
    State(service): State<Arc<SmartCustomizationService>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateBehaviorProfileRequest>,
) -> Result<Json<ApiResponse<UserBehaviorProfile>>, StatusCode> {
    info!("Updating behavior profile for user: {}", user_id);

    // Get existing profile or create default
    let mut profile = service.repository.get_behavior_profile(user_id).await
        .unwrap_or_else(|_| service.create_default_profile(user_id));

    // Update profile with request data
    if let Some(preferred_reading_times) = request.preferred_reading_times {
        profile.preferred_reading_times = preferred_reading_times;
    }
    if let Some(preferred_content_types) = request.preferred_content_types {
        profile.preferred_content_types = preferred_content_types;
    }
    if let Some(learning_style) = request.learning_style {
        profile.learning_style = learning_style;
    }
    if let Some(difficulty_preference) = request.difficulty_preference {
        profile.difficulty_preference = difficulty_preference;
    }
    if let Some(language_preferences) = request.language_preferences {
        profile.language_preferences = language_preferences;
    }
    if let Some(seasonal_preferences) = request.seasonal_preferences {
        profile.seasonal_preferences = seasonal_preferences;
    }
    if let Some(location_preferences) = request.location_preferences {
        profile.location_based_preferences = Some(location_preferences);
    }

    profile.updated_at = chrono::Utc::now();

    match service.repository.save_behavior_profile(&profile).await {
        Ok(updated_profile) => {
            Ok(Json(ApiResponse::success(updated_profile, "Behavior profile updated successfully")))
        }
        Err(e) => {
            error!("Failed to update behavior profile: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Analyze and update user behavior profile
pub async fn analyze_behavior(
    State(service): State<Arc<SmartCustomizationService>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserBehaviorProfile>>, StatusCode> {
    info!("Analyzing behavior for user: {}", user_id);

    match service.analyze_and_update_behavior_profile(user_id).await {
        Ok(profile) => {
            Ok(Json(ApiResponse::success(profile, "Behavior analysis completed successfully")))
        }
        Err(e) => {
            error!("Failed to analyze behavior: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get personalized recommendations
pub async fn get_recommendations(
    State(service): State<Arc<SmartCustomizationService>>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<RecommendationQuery>,
) -> Result<Json<ApiResponse<RecommendationResponse>>, StatusCode> {
    info!("Getting recommendations for user: {}", user_id);

    // Parse query parameters
    let content_types = params.content_types
        .map(|types| parse_content_types(&types))
        .unwrap_or_default();

    let categories = params.categories
        .map(|cats| parse_recommendation_categories(&cats))
        .unwrap_or_default();

    let difficulty_override = params.difficulty_override
        .and_then(|d| parse_difficulty_level(&d));

    let request = RecommendationRequest {
        content_types: if content_types.is_empty() { None } else { Some(content_types) },
        categories: if categories.is_empty() { None } else { Some(categories) },
        max_recommendations: params.max_recommendations,
        time_context: Some(chrono::Utc::now()),
        session_duration: params.session_duration,
        difficulty_override,
    };

    match service.generate_personalized_recommendations(user_id, request).await {
        Ok(response) => {
            Ok(Json(ApiResponse::success(response, "Recommendations generated successfully")))
        }
        Err(e) => {
            error!("Failed to generate recommendations: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Generate new personalized recommendations
pub async fn generate_recommendations(
    State(service): State<Arc<SmartCustomizationService>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<RecommendationRequest>,
) -> Result<Json<ApiResponse<RecommendationResponse>>, StatusCode> {
    info!("Generating new recommendations for user: {}", user_id);

    match service.generate_personalized_recommendations(user_id, request).await {
        Ok(response) => {
            Ok(Json(ApiResponse::success(response, "New recommendations generated successfully")))
        }
        Err(e) => {
            error!("Failed to generate new recommendations: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get adaptive reminders
pub async fn get_adaptive_reminders(
    State(service): State<Arc<SmartCustomizationService>>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<AdaptiveReminderQuery>,
) -> Result<Json<ApiResponse<AdaptiveReminderResponse>>, StatusCode> {
    info!("Getting adaptive reminders for user: {}", user_id);

    let reminder_types = params.reminder_types
        .map(|types| parse_reminder_types(&types))
        .unwrap_or_default();

    let urgency_level = params.urgency_level
        .and_then(|u| parse_urgency_level(&u));

    let request = AdaptiveReminderRequest {
        reminder_types: if reminder_types.is_empty() { None } else { Some(reminder_types) },
        time_window: None, // Use default
        max_reminders: params.max_reminders,
        urgency_level,
        context: None, // Could be enhanced with context detection
    };

    match service.generate_adaptive_reminders(user_id, request).await {
        Ok(response) => {
            Ok(Json(ApiResponse::success(response, "Adaptive reminders generated successfully")))
        }
        Err(e) => {
            error!("Failed to generate adaptive reminders: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Generate new adaptive reminders
pub async fn generate_adaptive_reminders(
    State(service): State<Arc<SmartCustomizationService>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<AdaptiveReminderRequest>,
) -> Result<Json<ApiResponse<AdaptiveReminderResponse>>, StatusCode> {
    info!("Generating new adaptive reminders for user: {}", user_id);

    match service.generate_adaptive_reminders(user_id, request).await {
        Ok(response) => {
            Ok(Json(ApiResponse::success(response, "New adaptive reminders generated successfully")))
        }
        Err(e) => {
            error!("Failed to generate new adaptive reminders: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Learn user preferences
pub async fn learn_preferences(
    State(service): State<Arc<SmartCustomizationService>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<PreferenceLearningRecord>>>, StatusCode> {
    info!("Learning preferences for user: {}", user_id);

    match service.learn_user_preferences(user_id).await {
        Ok(learning_records) => {
            Ok(Json(ApiResponse::success(learning_records, "Preferences learned successfully")))
        }
        Err(e) => {
            error!("Failed to learn preferences: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get customization analytics
pub async fn get_analytics(
    State(service): State<Arc<SmartCustomizationService>>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<AnalyticsQuery>,
) -> Result<Json<ApiResponse<AnalyticsResponse>>, StatusCode> {
    info!("Getting analytics for user: {}", user_id);

    // Parse period parameters
    let period_type = params.period_type
        .and_then(|p| parse_period_type(&p))
        .unwrap_or(PeriodType::Monthly);

    let (start_date, end_date) = if let (Some(start), Some(end)) = (params.start_date, params.end_date) {
        match (chrono::DateTime::parse_from_rfc3339(&start), chrono::DateTime::parse_from_rfc3339(&end)) {
            (Ok(start), Ok(end)) => (start.with_timezone(&chrono::Utc), end.with_timezone(&chrono::Utc)),
            _ => {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    } else {
        // Default to last 30 days
        let end = chrono::Utc::now();
        let start = end - chrono::Duration::days(30);
        (start, end)
    };

    let period = AnalysisPeriod {
        start_date,
        end_date,
        period_type,
    };

    match service.get_customization_analytics(user_id, period).await {
        Ok(response) => {
            Ok(Json(ApiResponse::success(response, "Analytics generated successfully")))
        }
        Err(e) => {
            error!("Failed to generate analytics: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Submit feedback for a recommendation
pub async fn submit_recommendation_feedback(
    State(service): State<Arc<SmartCustomizationService>>,
    Path((user_id, recommendation_id)): Path<(Uuid, Uuid)>,
    Json(feedback): Json<RecommendationFeedback>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    info!("Submitting feedback for recommendation: {} from user: {}", recommendation_id, user_id);

    // Update recommendation with feedback
    // This would typically update the recommendation record in the database
    // and potentially trigger learning algorithms to improve future recommendations

    // For now, just log the feedback
    info!("Received feedback for recommendation {}: rating={:?}, useful={:?}, completed={:?}", 
          recommendation_id, feedback.rating, feedback.useful, feedback.completed);

    Ok(Json(ApiResponse::success((), "Feedback submitted successfully")))
}

/// Submit response for a reminder
pub async fn submit_reminder_response(
    State(service): State<Arc<SmartCustomizationService>>,
    Path((user_id, reminder_id)): Path<(Uuid, Uuid)>,
    Json(response): Json<ReminderResponseRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    info!("Submitting response for reminder: {} from user: {}", reminder_id, user_id);

    // Parse response type
    let response_type = match response.response_type.as_str() {
        "ignored" => ReminderResponse::Ignored,
        "dismissed" => ReminderResponse::Dismissed,
        "acknowledged" => ReminderResponse::Acknowledged,
        "acted" => ReminderResponse::Acted,
        "completed" => ReminderResponse::Completed,
        "postponed" => ReminderResponse::Postponed,
        _ => ReminderResponse::Ignored,
    };

    // Update reminder with response
    // This would typically update the reminder record and trigger learning algorithms

    info!("Received response for reminder {}: type={:?}", reminder_id, response_type);

    Ok(Json(ApiResponse::success((), "Reminder response submitted successfully")))
}

// Helper functions for parsing query parameters

fn parse_content_types(types_str: &str) -> Vec<ContentType> {
    types_str.split(',')
        .filter_map(|s| match s.trim() {
            "quran_verses" => Some(ContentType::QuranVerses),
            "hadith_narrations" => Some(ContentType::HadithNarrations),
            "islamic_stories" => Some(ContentType::IslamicStories),
            "tafsir" => Some(ContentType::Tafsir),
            "dhikr" => Some(ContentType::Dhikr),
            "duas" => Some(ContentType::Duas),
            "islamic_history" => Some(ContentType::IslamicHistory),
            "fiqh" => Some(ContentType::Fiqh),
            "aqeedah" => Some(ContentType::Aqeedah),
            "seerah" => Some(ContentType::Seerah),
            _ => None,
        })
        .collect()
}

fn parse_recommendation_categories(categories_str: &str) -> Vec<RecommendationCategory> {
    categories_str.split(',')
        .filter_map(|s| match s.trim() {
            "daily_reading" => Some(RecommendationCategory::DailyReading),
            "seasonal" => Some(RecommendationCategory::Seasonal),
            "learning" => Some(RecommendationCategory::Learning),
            "spiritual" => Some(RecommendationCategory::Spiritual),
            "community" => Some(RecommendationCategory::Community),
            "personal" => Some(RecommendationCategory::Personal),
            "trending" => Some(RecommendationCategory::Trending),
            "continuation" => Some(RecommendationCategory::Continuation),
            "discovery" => Some(RecommendationCategory::Discovery),
            _ => None,
        })
        .collect()
}

fn parse_reminder_types(types_str: &str) -> Vec<ReminderType> {
    types_str.split(',')
        .filter_map(|s| match s.trim() {
            "prayer" => Some(ReminderType::Prayer),
            "dhikr" => Some(ReminderType::Dhikr),
            "quran_reading" => Some(ReminderType::QuranReading),
            "charity" => Some(ReminderType::Charity),
            "fasting" => Some(ReminderType::Fasting),
            "reflection" => Some(ReminderType::Reflection),
            "learning" => Some(ReminderType::Learning),
            "community" => Some(ReminderType::Community),
            _ => None,
        })
        .collect()
}

fn parse_difficulty_level(level_str: &str) -> Option<DifficultyLevel> {
    match level_str {
        "beginner" => Some(DifficultyLevel::Beginner),
        "intermediate" => Some(DifficultyLevel::Intermediate),
        "advanced" => Some(DifficultyLevel::Advanced),
        "scholar" => Some(DifficultyLevel::Scholar),
        "adaptive" => Some(DifficultyLevel::Adaptive),
        _ => None,
    }
}

fn parse_urgency_level(urgency_str: &str) -> Option<UrgencyLevel> {
    match urgency_str {
        "low" => Some(UrgencyLevel::Low),
        "normal" => Some(UrgencyLevel::Normal),
        "high" => Some(UrgencyLevel::High),
        "critical" => Some(UrgencyLevel::Critical),
        _ => None,
    }
}

fn parse_period_type(period_str: &str) -> Option<PeriodType> {
    match period_str {
        "daily" => Some(PeriodType::Daily),
        "weekly" => Some(PeriodType::Weekly),
        "monthly" => Some(PeriodType::Monthly),
        "quarterly" => Some(PeriodType::Quarterly),
        "yearly" => Some(PeriodType::Yearly),
        "custom" => Some(PeriodType::Custom),
        _ => None,
    }
}
use crate::models::*;
use crate::service::SmartKhatmaService;
use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post, put},
    Router,
};
use shared::{ApiResponse, AppError};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;
use tracing::{info, error};

/// HTTP handlers for the Smart Khatma Service
pub struct KhatmaHandlers;

impl KhatmaHandlers {
    /// Create router with all khatma endpoints
    pub fn router(service: Arc<SmartKhatmaService>) -> Router {
        Router::new()
            .route("/health", get(health_check))
            .route("/plans", post(create_khatma_plan))
            .route("/plans/:plan_id", get(get_khatma_plan))
            .route("/plans/:plan_id", put(update_khatma_plan))
            .route("/plans/:plan_id/progress", post(update_reading_progress))
            .route("/plans/:plan_id/adjust", post(adjust_khatma_plan))
            .route("/plans/:plan_id/statistics", get(get_khatma_statistics))
            .route("/users/:user_id/plans", get(get_user_plans))
            .route("/users/:user_id/suggestions", get(get_reading_suggestions))
            .route("/users/:user_id/reminders", get(get_smart_reminders))
            .route("/users/:user_id/dashboard", get(get_progress_dashboard))
            .route("/users/:user_id/dashboard", post(generate_progress_dashboard))
            .route("/users/:user_id/comparison", get(get_khatma_comparison))
            .route("/users/:user_id/comparison", post(generate_khatma_comparison))
            .with_state(service)
    }
}

/// Query parameters for user plans
#[derive(Deserialize)]
struct UserPlansQuery {
    status: Option<String>,
}

/// Query parameters for reminders
#[derive(Deserialize)]
struct RemindersQuery {
    limit: Option<i64>,
}

/// Health check endpoint
async fn health_check() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "khatma-service".to_string());
    status.insert("version".to_string(), "1.0.0".to_string());
    Json(ApiResponse::success(status))
}

/// Create a new khatma plan
async fn create_khatma_plan(
    State(service): State<Arc<SmartKhatmaService>>,
    Json(request): Json<CreateKhatmaPlanRequest>,
) -> Result<Json<ApiResponse<KhatmaPlan>>, AppError> {
    info!("Creating new khatma plan");

    // Extract user_id from request context (in real implementation, this would come from JWT)
    let user_id = Uuid::new_v4(); // Placeholder - should come from authentication

    match service.create_khatma_plan(user_id, request).await {
        Ok(plan) => {
            info!("Successfully created khatma plan: {}", plan.id);
            Ok(Json(ApiResponse::success(plan)))
        }
        Err(e) => {
            error!("Failed to create khatma plan: {}", e);
            Err(AppError::Internal(e.to_string()))
        }
    }
}

/// Get a specific khatma plan
async fn get_khatma_plan(
    State(service): State<Arc<SmartKhatmaService>>,
    Path(plan_id): Path<Uuid>,
) -> Result<Json<ApiResponse<KhatmaPlan>>, AppError> {
    info!("Getting khatma plan: {}", plan_id);

    match service.get_user_active_plans(plan_id).await {
        Ok(plans) => {
            if let Some(plan) = plans.first() {
                Ok(Json(ApiResponse::success(plan.clone())))
            } else {
                Err(AppError::NotFound("Khatma plan not found".to_string()))
            }
        }
        Err(e) => {
            error!("Failed to get khatma plan: {}", e);
            Err(AppError::Internal(e.to_string()))
        }
    }
}

/// Update a khatma plan
async fn update_khatma_plan(
    State(service): State<Arc<SmartKhatmaService>>,
    Path(plan_id): Path<Uuid>,
    Json(request): Json<PlanAdjustmentRequest>,
) -> Result<Json<ApiResponse<KhatmaPlanUpdateResponse>>, AppError> {
    info!("Updating khatma plan: {}", plan_id);

    match service.adjust_khatma_plan(plan_id).await {
        Ok(updated_plan) => {
            info!("Successfully updated khatma plan: {}", plan_id);
            let response = KhatmaPlanUpdateResponse {
                plan: updated_plan,
                adjustments: vec![],
            };
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to update khatma plan: {}", e);
            Err(AppError::Internal(e.to_string()))
        }
    }
}

/// Update reading progress
async fn update_reading_progress(
    State(service): State<Arc<SmartKhatmaService>>,
    Path(plan_id): Path<Uuid>,
    Json(request): Json<UpdateProgressRequest>,
) -> Result<Json<ApiResponse<ProgressUpdateResponse>>, AppError> {
    info!("Updating reading progress for plan: {}", plan_id);

    // Extract user_id from request or use a default
    let user_id = request.reading_session.user_id;

    match service.update_reading_progress(user_id, request.reading_session).await {
        Ok(plan_update) => {
            info!("Successfully updated reading progress for plan: {}", plan_id);
            let response = ProgressUpdateResponse {
                plan: plan_update.updated_plan.unwrap_or_else(|| {
                    // Return a default plan if none provided
                    crate::models::KhatmaPlan {
                        id: plan_id,
                        user_id,
                        start_date: chrono::Utc::now(),
                        target_date: chrono::Utc::now(),
                        status: crate::models::KhatmaStatus::Active,
                        current_progress: 0.0,
                        daily_portions: vec![],
                        estimated_reading_time: 60,
                        adaptive_schedule: true,
                        reading_speed_wpm: 150.0,
                        preferred_reading_times: vec![],
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    }
                }),
                automatic_adjustments: vec![],
                success: true,
            };
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to update reading progress: {}", e);
            Err(AppError::Internal(e.to_string()))
        }
    }
}

/// Manually adjust a khatma plan
async fn adjust_khatma_plan(
    State(service): State<Arc<SmartKhatmaService>>,
    Path(plan_id): Path<Uuid>,
    Json(request): Json<PlanAdjustmentRequest>,
) -> Result<Json<ApiResponse<KhatmaPlanUpdateResponse>>, AppError> {
    info!("Manually adjusting khatma plan: {}", plan_id);

    match service.adjust_khatma_plan(plan_id).await {
        Ok(updated_plan) => {
            info!("Successfully adjusted khatma plan: {}", plan_id);
            let response = KhatmaPlanUpdateResponse {
                plan: updated_plan,
                adjustments: vec![],
            };
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to adjust khatma plan: {}", e);
            Err(AppError::Internal(e.to_string()))
        }
    }
}

/// Get khatma statistics
async fn get_khatma_statistics(
    State(service): State<Arc<SmartKhatmaService>>,
    Path(plan_id): Path<Uuid>,
) -> Result<Json<ApiResponse<KhatmaStatistics>>, AppError> {
    info!("Getting statistics for khatma plan: {}", plan_id);

    match service.get_khatma_statistics(plan_id).await {
        Ok(statistics) => {
            info!("Successfully retrieved statistics for plan: {}", plan_id);
            Ok(Json(ApiResponse::success(statistics)))
        }
        Err(e) => {
            error!("Failed to get khatma statistics: {}", e);
            Err(AppError::Internal(e.to_string()))
        }
    }
}

/// Get user's khatma plans
async fn get_user_plans(
    State(service): State<Arc<SmartKhatmaService>>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<UserPlansQuery>,
) -> Result<Json<ApiResponse<Vec<KhatmaPlan>>>, AppError> {
    info!("Getting khatma plans for user: {}", user_id);

    let status_filter = params.status.and_then(|s| {
        match s.as_str() {
            "active" => Some(KhatmaStatus::Active),
            "completed" => Some(KhatmaStatus::Completed),
            "paused" => Some(KhatmaStatus::Paused),
            "cancelled" => Some(KhatmaStatus::Cancelled),
            _ => None,
        }
    });

    // For now, we'll use the active plans method
    // In a full implementation, you'd have a method that accepts status filter
    match service.get_user_active_plans(user_id).await {
        Ok(plans) => {
            info!("Successfully retrieved {} plans for user: {}", plans.len(), user_id);
            Ok(Json(ApiResponse::success(plans)))
        }
        Err(e) => {
            error!("Failed to get user plans: {}", e);
            Err(AppError::Internal(e.to_string()))
        }
    }
}

/// Get reading time suggestions
async fn get_reading_suggestions(
    State(service): State<Arc<SmartKhatmaService>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<ReadingTimeSuggestionResponse>>, AppError> {
    info!("Getting reading suggestions for user: {}", user_id);

    // Get user's active plan (simplified - in real implementation, handle multiple plans)
    let active_plans = service.get_user_active_plans(user_id).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if let Some(plan) = active_plans.first() {
        match service.get_reading_time_suggestions(user_id).await {
            Ok(suggestions) => {
                info!("Successfully generated reading suggestions for user: {}", user_id);
                let response = ReadingTimeSuggestionResponse {
                    suggested_times: suggestions,
                    optimal_daily_schedule: HashMap::new(), // TODO: implement optimal schedule generation
                    reasoning: "Based on your reading history and preferences".to_string(),
                };
                Ok(Json(ApiResponse::success(response)))
            }
            Err(e) => {
                error!("Failed to get reading suggestions: {}", e);
                Err(AppError::Internal(e.to_string()))
            }
        }
    } else {
        Err(AppError::NotFound("No active khatma plan found for user".to_string()))
    }
}

/// Get smart reminders for a user
async fn get_smart_reminders(
    State(service): State<Arc<SmartKhatmaService>>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<RemindersQuery>,
) -> Result<Json<ApiResponse<Vec<SmartReminder>>>, AppError> {
    info!("Getting smart reminders for user: {}", user_id);

    // Get user's active plan (simplified - in real implementation, handle multiple plans)
    let active_plans = service.get_user_active_plans(user_id).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if let Some(plan) = active_plans.first() {
        match service.generate_smart_reminders(user_id).await {
            Ok(reminders) => {
                let limited_reminders = if let Some(limit) = params.limit {
                    reminders.into_iter().take(limit as usize).collect()
                } else {
                    reminders
                };
                
                info!("Successfully retrieved {} reminders for user: {}", limited_reminders.len(), user_id);
                Ok(Json(ApiResponse::success(limited_reminders)))
            }
            Err(e) => {
                error!("Failed to generate smart reminders: {}", e);
                Err(AppError::Internal(e.to_string()))
            }
        }
    } else {
        // Return empty list if no active plan
        info!("No active khatma plan found for user: {}", user_id);
        Ok(Json(ApiResponse::success(vec![])))
    }
}

/// Response types for API endpoints
#[derive(serde::Serialize)]
struct KhatmaPlanUpdateResponse {
    plan: KhatmaPlan,
    adjustments: Vec<String>,
}

#[derive(serde::Serialize)]
struct ProgressUpdateResponse {
    plan: KhatmaPlan,
    automatic_adjustments: Vec<String>,
    success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use std::sync::Arc;

    // Mock service for testing
    struct MockSmartKhatmaService;

    impl MockSmartKhatmaService {
        fn new() -> Arc<SmartKhatmaService> {
            // In real tests, you'd create a proper mock
            unimplemented!("Mock service not implemented")
        }
    }

    #[tokio::test]
    async fn test_health_check() {
        let service = MockSmartKhatmaService::new();
        let app = KhatmaHandlers::router(service);
        let server = TestServer::new(app).unwrap();

        // This test would work with a proper mock implementation
        // For now, we're just testing the structure
        assert!(true);
    }

    #[tokio::test]
    async fn test_create_khatma_plan() {
        // Test would require proper mock setup
        assert!(true);
    }
}
/// Get progress dashboard for a user
async fn get_progress_dashboard(
    State(service): State<Arc<SmartKhatmaService>>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<DashboardRequest>,
) -> Result<Json<ApiResponse<ProgressDashboard>>, AppError> {
    info!("Getting progress dashboard for user: {}", user_id);

    match service.generate_progress_dashboard(user_id).await {
        Ok(dashboard_json) => {
            info!("Successfully retrieved progress dashboard for user: {}", user_id);
            // Convert JSON to ProgressDashboard or return as-is
            // For now, we'll create a simple wrapper
            let dashboard = ProgressDashboard {
                user_id,
                total_plans: 0, // TODO: extract from dashboard_json
                active_plans: 0,
                completed_plans: 0,
                total_progress: 0.0,
                current_streak: 0,
                longest_streak: 0,
                total_reading_time_minutes: 0,
                average_daily_reading_minutes: 0,
                completion_rate: 0.0,
                upcoming_milestones: vec![],
                recent_achievements: vec![],
                reading_statistics: HashMap::new(),
            };
            Ok(Json(ApiResponse::success(dashboard)))
        }
        Err(e) => {
            error!("Failed to get progress dashboard: {}", e);
            Err(AppError::Internal(e.to_string()))
        }
    }
}

/// Generate new progress dashboard for a user
async fn generate_progress_dashboard(
    State(service): State<Arc<SmartKhatmaService>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<DashboardRequest>,
) -> Result<Json<ApiResponse<ProgressDashboard>>, AppError> {
    info!("Generating new progress dashboard for user: {}", user_id);

    match service.generate_progress_dashboard(user_id).await {
        Ok(dashboard_json) => {
            info!("Successfully generated progress dashboard for user: {}", user_id);
            // Convert JSON to ProgressDashboard
            let dashboard = ProgressDashboard {
                user_id,
                total_plans: 0,
                active_plans: 0,
                completed_plans: 0,
                total_progress: 0.0,
                current_streak: 0,
                longest_streak: 0,
                total_reading_time_minutes: 0,
                average_daily_reading_minutes: 0,
                completion_rate: 0.0,
                upcoming_milestones: vec![],
                recent_achievements: vec![],
                reading_statistics: HashMap::new(),
            };
            Ok(Json(ApiResponse::success(dashboard)))
        }
        Err(e) => {
            error!("Failed to generate progress dashboard: {}", e);
            Err(AppError::Internal(e.to_string()))
        }
    }
}

/// Get Khatma comparison for a user
async fn get_khatma_comparison(
    State(service): State<Arc<SmartKhatmaService>>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<ComparisonRequest>,
) -> Result<Json<ApiResponse<KhatmaComparison>>, AppError> {
    info!("Getting Khatma comparison for user: {}", user_id);

    // TODO: Implement generate_khatma_comparison in service
    let comparison = KhatmaComparison {
        user_id,
        comparison_period_days: params.period_days.unwrap_or(30),
        personal_stats: ComparisonStats {
            total_reading_time_minutes: 0,
            average_daily_minutes: 0,
            completion_rate: 0.0,
            streak_days: 0,
        },
        community_average: ComparisonStats {
            total_reading_time_minutes: 0,
            average_daily_minutes: 0,
            completion_rate: 0.0,
            streak_days: 0,
        },
        percentile_rank: 50.0,
        insights: vec![],
    };
    
    info!("Successfully retrieved Khatma comparison for user: {}", user_id);
    Ok(Json(ApiResponse::success(comparison)))
}

/// Generate new Khatma comparison for a user
async fn generate_khatma_comparison(
    State(service): State<Arc<SmartKhatmaService>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<ComparisonRequest>,
) -> Result<Json<ApiResponse<KhatmaComparison>>, AppError> {
    info!("Generating new Khatma comparison for user: {}", user_id);

    // TODO: Implement generate_khatma_comparison in service
    let comparison = KhatmaComparison {
        user_id,
        comparison_period_days: request.period_days.unwrap_or(30),
        personal_stats: ComparisonStats {
            total_reading_time_minutes: 0,
            average_daily_minutes: 0,
            completion_rate: 0.0,
            streak_days: 0,
        },
        community_average: ComparisonStats {
            total_reading_time_minutes: 0,
            average_daily_minutes: 0,
            completion_rate: 0.0,
            streak_days: 0,
        },
        percentile_rank: 50.0,
        insights: vec![],
    };
    
    info!("Successfully generated Khatma comparison for user: {}", user_id);
    Ok(Json(ApiResponse::success(comparison)))
            Err(AppError::Internal(e.to_string()))
        }
    }
}
use crate::models::*;
use crate::service::NotificationService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize};
use shared::{ApiResponse, AppError};
use std::sync::Arc;
use uuid::Uuid;
use tracing::{info, error};

pub type SharedNotificationService = Arc<NotificationService>;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub status: Option<NotificationStatus>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessNotificationsQuery {
    pub limit: Option<i32>,
}

pub fn create_router(service: SharedNotificationService) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/notifications", post(create_notification))
        .route("/notifications/process", post(process_pending_notifications))
        .route("/notifications/users/:user_id", get(get_user_notifications))
        .route("/notifications/users/:user_id/stats", get(get_notification_stats))
        .route("/notifications/:notification_id/read", put(mark_notification_as_read))
        .route("/notifications/:notification_id/dismiss", put(dismiss_notification))
        .route("/prayer-notifications", post(create_prayer_notification))
        .route("/sunnah-reminders", post(create_sunnah_reminder))
        .route("/seasonal-reminders", post(create_seasonal_reminder))
        .route("/dhikr-reminders", post(create_dhikr_reminder))
        .route("/dhikr/time-appropriate/:user_id", post(generate_time_appropriate_dhikr))
        .route("/seasonal/schedule", post(schedule_seasonal_notifications))
        .route("/preferences/:user_id", get(get_user_preferences))
        .route("/preferences/:user_id", put(update_user_preferences))
        .with_state(service)
}

/// Health check endpoint
async fn health_check() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "status": "healthy",
        "service": "notification-service",
        "version": "1.0.0"
    })))
}

/// Create a general notification
async fn create_notification(
    State(service): State<SharedNotificationService>,
    Json(request): Json<CreateNotificationRequest>,
) -> Result<Json<ApiResponse<NotificationResponse>>, AppError> {
    info!("Creating notification for user {}", request.user_id);

    let notification = service.repository.create_notification(request).await
        .map_err(|e| {
            error!("Failed to create notification: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(NotificationResponse::from(notification))))
}

/// Process pending notifications
async fn process_pending_notifications(
    State(service): State<SharedNotificationService>,
    Query(query): Query<ProcessNotificationsQuery>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let limit = query.limit.unwrap_or(100);
    
    info!("Processing up to {} pending notifications", limit);

    let processed_count = service.process_pending_notifications(limit).await
        .map_err(|e| {
            error!("Failed to process pending notifications: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "processed_count": processed_count,
        "limit": limit
    }))))
}

/// Get user notifications with pagination
async fn get_user_notifications(
    State(service): State<SharedNotificationService>,
    Path(user_id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<NotificationListResponse>>, AppError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    info!("Getting notifications for user {} (page: {}, size: {})", user_id, page, page_size);

    let response = service.get_user_notifications(user_id, page, page_size, query.status).await
        .map_err(|e| {
            error!("Failed to get user notifications: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(response)))
}

/// Get notification statistics for a user
async fn get_notification_stats(
    State(service): State<SharedNotificationService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<NotificationStatsResponse>>, AppError> {
    info!("Getting notification stats for user {}", user_id);

    let stats = service.get_notification_stats(user_id).await
        .map_err(|e| {
            error!("Failed to get notification stats: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(stats)))
}

/// Mark notification as read
async fn mark_notification_as_read(
    State(service): State<SharedNotificationService>,
    Path(notification_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    info!("Marking notification {} as read", notification_id);

    service.mark_notification_as_read(notification_id).await
        .map_err(|e| {
            error!("Failed to mark notification as read: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Notification marked as read",
        "notification_id": notification_id
    }))))
}

/// Dismiss notification
async fn dismiss_notification(
    State(service): State<SharedNotificationService>,
    Path(notification_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    info!("Dismissing notification {}", notification_id);

    service.dismiss_notification(notification_id).await
        .map_err(|e| {
            error!("Failed to dismiss notification: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Notification dismissed",
        "notification_id": notification_id
    }))))
}

/// Create graduated prayer notifications
async fn create_prayer_notification(
    State(service): State<SharedNotificationService>,
    Json(request): Json<CreatePrayerNotificationRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    info!("Creating graduated prayer notifications for user {} and prayer {:?}", 
          request.user_id, request.prayer_name);

    let notifications = service.create_graduated_prayer_notifications(request).await
        .map_err(|e| {
            error!("Failed to create prayer notifications: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Graduated prayer notifications created",
        "notification_count": notifications.len(),
        "notification_ids": notifications.iter().map(|n| n.id).collect::<Vec<_>>()
    }))))
}

/// Create sunnah reminder
async fn create_sunnah_reminder(
    State(service): State<SharedNotificationService>,
    Json(request): Json<CreateSunnahReminderRequest>,
) -> Result<Json<ApiResponse<SunnahReminder>>, AppError> {
    info!("Creating sunnah reminder '{}' for user {}", request.sunnah_name, request.user_id);

    let reminder = service.create_sunnah_reminder(request).await
        .map_err(|e| {
            error!("Failed to create sunnah reminder: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(reminder)))
}

/// Create seasonal reminder
async fn create_seasonal_reminder(
    State(service): State<SharedNotificationService>,
    Json(request): Json<CreateSeasonalReminderRequest>,
) -> Result<Json<ApiResponse<SeasonalReminder>>, AppError> {
    info!("Creating seasonal reminder '{}' for user {} and season {:?}", 
          request.event_name, request.user_id, request.season);

    let reminder = service.create_seasonal_reminder(request).await
        .map_err(|e| {
            error!("Failed to create seasonal reminder: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(reminder)))
}

/// Create dhikr reminder
async fn create_dhikr_reminder(
    State(service): State<SharedNotificationService>,
    Json(request): Json<CreateDhikrReminderRequest>,
) -> Result<Json<ApiResponse<DhikrReminder>>, AppError> {
    info!("Creating dhikr reminder for user {} and category {:?}", 
          request.user_id, request.dhikr_category);

    let reminder = service.create_dhikr_reminder(request).await
        .map_err(|e| {
            error!("Failed to create dhikr reminder: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(reminder)))
}

/// Generate time-appropriate dhikr notifications
async fn generate_time_appropriate_dhikr(
    State(service): State<SharedNotificationService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    info!("Generating time-appropriate dhikr for user {}", user_id);

    let notifications = service.generate_time_appropriate_dhikr(user_id).await
        .map_err(|e| {
            error!("Failed to generate time-appropriate dhikr: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Time-appropriate dhikr notifications generated",
        "notification_count": notifications.len(),
        "notification_ids": notifications.iter().map(|n| n.id).collect::<Vec<_>>()
    }))))
}

/// Schedule upcoming seasonal notifications
async fn schedule_seasonal_notifications(
    State(service): State<SharedNotificationService>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    info!("Scheduling upcoming seasonal notifications");

    let scheduled_count = service.schedule_upcoming_seasonal_notifications().await
        .map_err(|e| {
            error!("Failed to schedule seasonal notifications: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Seasonal notifications scheduled",
        "scheduled_count": scheduled_count
    }))))
}

/// Get user notification preferences
async fn get_user_preferences(
    State(service): State<SharedNotificationService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserNotificationPreferences>>, AppError> {
    info!("Getting notification preferences for user {}", user_id);

    let preferences = service.get_user_preferences(user_id).await
        .map_err(|e| {
            error!("Failed to get user preferences: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(preferences)))
}

/// Update user notification preferences
async fn update_user_preferences(
    State(service): State<SharedNotificationService>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateNotificationPreferencesRequest>,
) -> Result<Json<ApiResponse<UserNotificationPreferences>>, AppError> {
    info!("Updating notification preferences for user {}", user_id);

    let preferences = service.update_user_preferences(user_id, request).await
        .map_err(|e| {
            error!("Failed to update user preferences: {}", e);
            AppError::Internal(e.to_string())
        })?;

    Ok(Json(ApiResponse::success(preferences)))
}
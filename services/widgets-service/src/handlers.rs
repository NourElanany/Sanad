use crate::models::*;
use crate::service::WidgetService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::Deserialize;
use uuid::Uuid;

pub fn create_router(service: WidgetService) -> Router {
    Router::new()
        .route("/api/widgets", post(create_widget))
        .route("/api/widgets", get(get_user_widgets))
        .route("/api/widgets/:widget_id", get(get_widget))
        .route("/api/widgets/:widget_id", put(update_widget))
        .route("/api/widgets/:widget_id", delete(delete_widget))
        .route("/api/widgets/:widget_id/refresh", post(refresh_widget))
        .route("/api/widgets/available", get(get_available_widgets))
        .route("/api/dashboard", get(get_dashboard))
        .route("/api/dashboard", post(create_dashboard))
        .route("/api/dashboard/:dashboard_id", get(get_dashboard_by_id))
        .route("/api/dashboard/:dashboard_id", put(update_dashboard))
        .route("/api/dashboard/:dashboard_id", delete(delete_dashboard))
        .with_state(service)
}

#[derive(Deserialize)]
struct UserQuery {
    user_id: Uuid,
}

/// Create a new widget
async fn create_widget(
    State(service): State<WidgetService>,
    Query(query): Query<UserQuery>,
    Json(request): Json<CreateWidgetRequest>,
) -> Result<Json<WidgetDataResponse>, (StatusCode, Json<ErrorResponse>)> {
    match service.create_widget(query.user_id, request).await {
        Ok(widget) => Ok(Json(widget)),
        Err(e) => {
            tracing::error!("Failed to create widget: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to create widget".to_string(),
                    message: e.to_string(),
                }),
            ))
        }
    }
}

/// Get all widgets for a user
async fn get_user_widgets(
    State(service): State<WidgetService>,
    Query(query): Query<UserQuery>,
) -> Result<Json<WidgetListResponse>, (StatusCode, Json<ErrorResponse>)> {
    match service.get_user_widgets(query.user_id).await {
        Ok(widgets) => {
            let total_count = widgets.len();
            Ok(Json(WidgetListResponse {
                widgets,
                total_count,
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get user widgets: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to get widgets".to_string(),
                    message: e.to_string(),
                }),
            ))
        }
    }
}

/// Get a specific widget
async fn get_widget(
    State(service): State<WidgetService>,
    Path(widget_id): Path<Uuid>,
    Query(query): Query<UserQuery>,
) -> Result<Json<WidgetDataResponse>, (StatusCode, Json<ErrorResponse>)> {
    match service.refresh_widget(widget_id, query.user_id).await {
        Ok(widget) => Ok(Json(widget)),
        Err(WidgetError::WidgetNotFound { .. }) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Widget not found".to_string(),
                message: "The requested widget does not exist".to_string(),
            }),
        )),
        Err(e) => {
            tracing::error!("Failed to get widget: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to get widget".to_string(),
                    message: e.to_string(),
                }),
            ))
        }
    }
}

/// Update a widget
async fn update_widget(
    State(service): State<WidgetService>,
    Path(widget_id): Path<Uuid>,
    Query(query): Query<UserQuery>,
    Json(request): Json<UpdateWidgetRequest>,
) -> Result<Json<WidgetDataResponse>, (StatusCode, Json<ErrorResponse>)> {
    match service.update_widget(widget_id, query.user_id, request).await {
        Ok(widget) => Ok(Json(widget)),
        Err(WidgetError::WidgetNotFound { .. }) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Widget not found".to_string(),
                message: "The requested widget does not exist".to_string(),
            }),
        )),
        Err(e) => {
            tracing::error!("Failed to update widget: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to update widget".to_string(),
                    message: e.to_string(),
                }),
            ))
        }
    }
}

/// Delete a widget
async fn delete_widget(
    State(service): State<WidgetService>,
    Path(widget_id): Path<Uuid>,
    Query(query): Query<UserQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    match service.delete_widget(widget_id, query.user_id).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(WidgetError::WidgetNotFound { .. }) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Widget not found".to_string(),
                message: "The requested widget does not exist".to_string(),
            }),
        )),
        Err(e) => {
            tracing::error!("Failed to delete widget: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to delete widget".to_string(),
                    message: e.to_string(),
                }),
            ))
        }
    }
}

/// Refresh widget data
async fn refresh_widget(
    State(service): State<WidgetService>,
    Path(widget_id): Path<Uuid>,
    Query(query): Query<UserQuery>,
) -> Result<Json<WidgetDataResponse>, (StatusCode, Json<ErrorResponse>)> {
    match service.refresh_widget(widget_id, query.user_id).await {
        Ok(widget) => Ok(Json(widget)),
        Err(WidgetError::WidgetNotFound { .. }) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Widget not found".to_string(),
                message: "The requested widget does not exist".to_string(),
            }),
        )),
        Err(e) => {
            tracing::error!("Failed to refresh widget: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to refresh widget".to_string(),
                    message: e.to_string(),
                }),
            ))
        }
    }
}

/// Get available widget types
async fn get_available_widgets(
    State(service): State<WidgetService>,
) -> Result<Json<AvailableWidgetsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let available_widgets = service.get_available_widget_types();
    Ok(Json(AvailableWidgetsResponse { available_widgets }))
}

/// Get user's dashboard
async fn get_dashboard(
    State(service): State<WidgetService>,
    Query(query): Query<UserQuery>,
) -> Result<Json<DashboardResponse>, (StatusCode, Json<ErrorResponse>)> {
    match service.get_user_dashboard(query.user_id).await {
        Ok(dashboard) => Ok(Json(dashboard)),
        Err(e) => {
            tracing::error!("Failed to get dashboard: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to get dashboard".to_string(),
                    message: e.to_string(),
                }),
            ))
        }
    }
}

/// Create a new dashboard
async fn create_dashboard(
    State(_service): State<WidgetService>,
    Query(_query): Query<UserQuery>,
    Json(_request): Json<CreateDashboardRequest>,
) -> Result<Json<DashboardResponse>, (StatusCode, Json<ErrorResponse>)> {
    // This would be implemented in the service layer
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse {
            error: "Not implemented".to_string(),
            message: "Dashboard creation not yet implemented".to_string(),
        }),
    ))
}

/// Get dashboard by ID
async fn get_dashboard_by_id(
    State(_service): State<WidgetService>,
    Path(_dashboard_id): Path<Uuid>,
    Query(_query): Query<UserQuery>,
) -> Result<Json<DashboardResponse>, (StatusCode, Json<ErrorResponse>)> {
    // This would be implemented in the service layer
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse {
            error: "Not implemented".to_string(),
            message: "Dashboard retrieval by ID not yet implemented".to_string(),
        }),
    ))
}

/// Update dashboard
async fn update_dashboard(
    State(_service): State<WidgetService>,
    Path(_dashboard_id): Path<Uuid>,
    Query(_query): Query<UserQuery>,
    Json(_request): Json<UpdateDashboardRequest>,
) -> Result<Json<DashboardResponse>, (StatusCode, Json<ErrorResponse>)> {
    // This would be implemented in the service layer
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse {
            error: "Not implemented".to_string(),
            message: "Dashboard update not yet implemented".to_string(),
        }),
    ))
}

/// Delete dashboard
async fn delete_dashboard(
    State(_service): State<WidgetService>,
    Path(_dashboard_id): Path<Uuid>,
    Query(_query): Query<UserQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // This would be implemented in the service layer
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse {
            error: "Not implemented".to_string(),
            message: "Dashboard deletion not yet implemented".to_string(),
        }),
    ))
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}
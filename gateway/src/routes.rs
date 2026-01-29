use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use shared::{AppConfig, ApiResponse, SanadError, SanadResult};
use crate::{auth, proxy::ServiceRegistry};
use serde_json::Value;
use std::collections::HashMap;

/// Create all API routes
pub fn create_routes(service_registry: ServiceRegistry, config: AppConfig) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Authentication routes
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/auth/refresh", post(auth::refresh_token))
        .route("/auth/logout", post(auth::logout))
        
        // Placeholder routes - will be implemented in later tasks
        .route("/quran/surahs", get(placeholder_handler))
        .route("/hadith/search", get(placeholder_handler))
        .route("/stories/search", get(placeholder_handler))
        .route("/prayer-times", get(placeholder_handler))
        .route("/calendar/hijri/:date", get(placeholder_handler))
        .route("/ai/ask", post(placeholder_handler))
        .route("/search", get(placeholder_handler))
        .route("/audio/analyze", post(placeholder_handler))
        .route("/khatma/plans", get(placeholder_handler))
        .route("/notifications", get(placeholder_handler))
        
        // User management routes
        .route("/users/profile", get(get_user_profile))
        .route("/users/profile", put(update_user_profile))
        .route("/users/bookmarks", get(get_user_bookmarks))
        .route("/users/bookmarks", post(add_bookmark))
        .route("/users/bookmarks/:bookmark_id", delete(remove_bookmark))
        
        .with_state((service_registry, config))
}

/// Health check endpoint
async fn health_check() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "api-gateway".to_string());
    status.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
    
    Json(ApiResponse::success(status))
}

/// Fallback handler for unmatched routes
pub async fn fallback_handler() -> (StatusCode, Json<ApiResponse<()>>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiResponse::error("Route not found".to_string())),
    )
}

/// Placeholder handler for routes not yet implemented
async fn placeholder_handler() -> Json<ApiResponse<String>> {
    Json(ApiResponse::error("This endpoint is not yet implemented".to_string()))
}

// User management handlers (implemented directly in gateway)
async fn get_user_profile() -> Json<ApiResponse<Value>> {
    // TODO: Implement user profile retrieval
    Json(ApiResponse::error("Not implemented yet".to_string()))
}

async fn update_user_profile() -> Json<ApiResponse<Value>> {
    // TODO: Implement user profile update
    Json(ApiResponse::error("Not implemented yet".to_string()))
}

async fn get_user_bookmarks() -> Json<ApiResponse<Value>> {
    // TODO: Implement bookmarks retrieval
    Json(ApiResponse::error("Not implemented yet".to_string()))
}

async fn add_bookmark() -> Json<ApiResponse<Value>> {
    // TODO: Implement bookmark addition
    Json(ApiResponse::error("Not implemented yet".to_string()))
}

async fn remove_bookmark(Path(_bookmark_id): Path<String>) -> Json<ApiResponse<Value>> {
    // TODO: Implement bookmark removal
    Json(ApiResponse::error("Not implemented yet".to_string()))
}
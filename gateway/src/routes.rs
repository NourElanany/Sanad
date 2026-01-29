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
        
        // Quran service routes
        .route("/quran/surahs", get(proxy_to_quran_service))
        .route("/quran/surahs/:surah_number", get(proxy_to_quran_service))
        .route("/quran/surahs/:surah_number/ayahs/:ayah_number", get(proxy_to_quran_service))
        .route("/quran/search", get(proxy_to_quran_service))
        .route("/quran/tafsir/:surah_number/:ayah_number", get(proxy_to_quran_service))
        
        // Hadith service routes
        .route("/hadith/search", get(proxy_to_hadith_service))
        .route("/hadith/:hadith_id", get(proxy_to_hadith_service))
        .route("/hadith/books/:book_name", get(proxy_to_hadith_service))
        .route("/hadith/topics/:topic", get(proxy_to_hadith_service))
        
        // Stories service routes
        .route("/stories/search", get(proxy_to_stories_service))
        .route("/stories/:story_id", get(proxy_to_stories_service))
        .route("/stories/categories/:category", get(proxy_to_stories_service))
        
        // Prayer times service routes
        .route("/prayer-times", get(proxy_to_prayer_service))
        .route("/prayer-times/qibla", get(proxy_to_prayer_service))
        
        // Calendar service routes
        .route("/calendar/hijri/:date", get(proxy_to_calendar_service))
        .route("/calendar/gregorian/:hijri_date", get(proxy_to_calendar_service))
        .route("/calendar/events/:month/:year", get(proxy_to_calendar_service))
        
        // AI service routes
        .route("/ai/ask", post(proxy_to_ai_service))
        .route("/ai/sources", get(proxy_to_ai_service))
        
        // Search service routes
        .route("/search", get(proxy_to_search_service))
        .route("/search/semantic", get(proxy_to_search_service))
        .route("/search/suggestions", get(proxy_to_search_service))
        
        // Audio analysis service routes
        .route("/audio/analyze", post(proxy_to_audio_service))
        .route("/audio/compare", post(proxy_to_audio_service))
        .route("/audio/progress/:user_id", get(proxy_to_audio_service))
        
        // Khatma service routes
        .route("/khatma/plans", get(proxy_to_khatma_service))
        .route("/khatma/plans", post(proxy_to_khatma_service))
        .route("/khatma/plans/:plan_id", get(proxy_to_khatma_service))
        .route("/khatma/plans/:plan_id", put(proxy_to_khatma_service))
        .route("/khatma/progress/:plan_id", post(proxy_to_khatma_service))
        
        // Notification service routes
        .route("/notifications", get(proxy_to_notification_service))
        .route("/notifications/preferences", get(proxy_to_notification_service))
        .route("/notifications/preferences", put(proxy_to_notification_service))
        
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

// Proxy handlers for each service
async fn proxy_to_quran_service(
    State((registry, _)): State<(ServiceRegistry, AppConfig)>,
    uri: axum::http::Uri,
    method: axum::http::Method,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> Result<axum::response::Response, SanadError> {
    registry.proxy_request("quran-service", uri, method, headers, body).await
}

async fn proxy_to_hadith_service(
    State((registry, _)): State<(ServiceRegistry, AppConfig)>,
    uri: axum::http::Uri,
    method: axum::http::Method,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> Result<axum::response::Response, SanadError> {
    registry.proxy_request("hadith-service", uri, method, headers, body).await
}

async fn proxy_to_stories_service(
    State((registry, _)): State<(ServiceRegistry, AppConfig)>,
    uri: axum::http::Uri,
    method: axum::http::Method,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> Result<axum::response::Response, SanadError> {
    registry.proxy_request("stories-service", uri, method, headers, body).await
}

async fn proxy_to_prayer_service(
    State((registry, _)): State<(ServiceRegistry, AppConfig)>,
    uri: axum::http::Uri,
    method: axum::http::Method,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> Result<axum::response::Response, SanadError> {
    registry.proxy_request("prayer-times-service", uri, method, headers, body).await
}

async fn proxy_to_calendar_service(
    State((registry, _)): State<(ServiceRegistry, AppConfig)>,
    uri: axum::http::Uri,
    method: axum::http::Method,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> Result<axum::response::Response, SanadError> {
    registry.proxy_request("calendar-service", uri, method, headers, body).await
}

async fn proxy_to_ai_service(
    State((registry, _)): State<(ServiceRegistry, AppConfig)>,
    uri: axum::http::Uri,
    method: axum::http::Method,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> Result<axum::response::Response, SanadError> {
    registry.proxy_request("ai-service", uri, method, headers, body).await
}

async fn proxy_to_search_service(
    State((registry, _)): State<(ServiceRegistry, AppConfig)>,
    uri: axum::http::Uri,
    method: axum::http::Method,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> Result<axum::response::Response, SanadError> {
    registry.proxy_request("search-service", uri, method, headers, body).await
}

async fn proxy_to_audio_service(
    State((registry, _)): State<(ServiceRegistry, AppConfig)>,
    uri: axum::http::Uri,
    method: axum::http::Method,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> Result<axum::response::Response, SanadError> {
    registry.proxy_request("audio-analysis-service", uri, method, headers, body).await
}

async fn proxy_to_khatma_service(
    State((registry, _)): State<(ServiceRegistry, AppConfig)>,
    uri: axum::http::Uri,
    method: axum::http::Method,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> Result<axum::response::Response, SanadError> {
    registry.proxy_request("khatma-service", uri, method, headers, body).await
}

async fn proxy_to_notification_service(
    State((registry, _)): State<(ServiceRegistry, AppConfig)>,
    uri: axum::http::Uri,
    method: axum::http::Method,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> Result<axum::response::Response, SanadError> {
    registry.proxy_request("notification-service", uri, method, headers, body).await
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
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use shared::{
    cache::{AdvancedCacheManager, CacheConfig, CacheStats, CacheType},
    config::AppConfig,
    models::ApiResponse,
    SanadError, SanadResult,
};
use std::{collections::HashMap, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info, warn};
use base64::Engine;

/// Cache service application state
#[derive(Clone)]
pub struct AppState {
    pub cache_manager: Arc<AdvancedCacheManager>,
    pub config: Arc<AppConfig>,
}

/// Request to set a cache value
#[derive(Debug, Deserialize)]
pub struct SetCacheRequest {
    pub key: String,
    pub value: serde_json::Value,
    pub cache_type: Option<CacheType>,
    pub ttl_seconds: Option<u64>,
}

/// Request to get multiple cache values
#[derive(Debug, Deserialize)]
pub struct GetMultipleRequest {
    pub keys: Vec<String>,
}

/// Response for multiple cache values
#[derive(Debug, Serialize)]
pub struct GetMultipleResponse {
    pub values: HashMap<String, Option<serde_json::Value>>,
}

/// Request to invalidate cache by pattern
#[derive(Debug, Deserialize)]
pub struct InvalidatePatternRequest {
    pub pattern: String,
}

/// Response for cache invalidation
#[derive(Debug, Serialize)]
pub struct InvalidateResponse {
    pub deleted_count: u64,
}

/// Request to cache heavy content
#[derive(Debug, Deserialize)]
pub struct CacheHeavyContentRequest {
    pub data: String, // Base64 encoded data
    pub content_type: String,
}

/// Request to cache frequent query
#[derive(Debug, Deserialize)]
pub struct CacheFrequentQueryRequest {
    pub query: String,
    pub result: serde_json::Value,
}

/// Request to get frequent query
#[derive(Debug, Deserialize)]
pub struct GetFrequentQueryRequest {
    pub query: String,
}

#[tokio::main]
async fn main() -> SanadResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting Sanad Cache Service");

    // Load configuration
    let config = Arc::new(AppConfig::load().map_err(|e| {
        SanadError::Configuration(format!("Failed to load configuration: {}", e))
    })?);

    config.validate().map_err(SanadError::Configuration)?;

    // Initialize cache manager
    let cache_config = CacheConfig {
        default_ttl_seconds: config.redis.default_ttl_seconds,
        prayer_times_ttl_seconds: 86400,  // 24 hours
        semantic_query_ttl_seconds: 21600, // 6 hours
        quran_content_ttl_seconds: 2592000, // 30 days
        hadith_content_ttl_seconds: 604800, // 7 days
        max_memory_cache_size: 50000,
        enable_smart_invalidation: true,
        min_query_frequency_for_cache: 5,
        heavy_content_threshold_bytes: 1024 * 1024, // 1MB
        heavy_content_ttl_seconds: 7200, // 2 hours
        enable_query_tracking: true,
        enable_adaptive_ttl: true,
    };

    let cache_manager: Arc<AdvancedCacheManager> = Arc::new(
        AdvancedCacheManager::new(&config.redis.url, Some(cache_config))
            .await
            .map_err(|e| {
                SanadError::Configuration(format!("Failed to initialize cache manager: {}", e))
            })?,
    );

    // Warm up cache
    cache_manager.warm_up_cache().await?;

    // Create application state
    let app_state = AppState {
        cache_manager,
        config: config.clone(),
    };

    // Build router
    let app = create_router(app_state);

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await.map_err(|e| {
        SanadError::Configuration(format!("Failed to bind to address {}: {}", addr, e))
    })?;

    info!("Cache service listening on {}", addr);

    axum::serve(listener, app).await.map_err(|e| {
        SanadError::Internal(format!("Server error: {}", e))
    })?;

    Ok(())
}

fn create_router(state: AppState) -> Router {
    Router::new()
        // Basic cache operations
        .route("/cache/:key", get(get_cache_value))
        .route("/cache/:key", put(set_cache_value))
        .route("/cache/:key", delete(delete_cache_value))
        .route("/cache/multi", post(get_multiple_cache_values))
        .route("/cache/multi", delete(delete_multiple_cache_values))
        
        // Specialized cache operations
        .route("/cache/prayer-times", post(cache_prayer_times))
        .route("/cache/semantic-query", post(cache_semantic_query))
        .route("/cache/quran-content", post(cache_quran_content))
        .route("/cache/hadith-content", post(cache_hadith_content))
        .route("/cache/frequent-query", post(cache_frequent_query))
        .route("/cache/frequent-query", get(get_frequent_query))
        .route("/cache/heavy-content/:content_id", put(cache_heavy_content))
        .route("/cache/heavy-content/:content_id", get(get_heavy_content))
        
        // Cache invalidation
        .route("/cache/invalidate/pattern", post(invalidate_cache_pattern))
        .route("/cache/invalidate/prayer-times/:lat/:lng", delete(invalidate_prayer_times))
        .route("/cache/invalidate/semantic-queries", delete(invalidate_semantic_queries))
        .route("/cache/invalidate/quran/:surah", delete(invalidate_quran_surah))
        .route("/cache/invalidate/hadith/:collection", delete(invalidate_hadith_collection))
        
        // Cache management
        .route("/cache/stats", get(get_cache_stats))
        .route("/cache/cleanup", post(cleanup_cache))
        .route("/cache/warmup", post(warmup_cache))
        
        // Health check
        .route("/health", get(health_check))
        
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

// Handler functions

async fn get_cache_value(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<ApiResponse<Option<serde_json::Value>>>, StatusCode> {
    match state.cache_manager.get::<serde_json::Value>(&key).await {
        Ok(value) => Ok(Json(ApiResponse::success(value))),
        Err(e) => {
            error!("Failed to get cache value for key {}: {}", key, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn set_cache_value(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(request): Json<SetCacheRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let cache_type = request.cache_type.unwrap_or(CacheType::General);
    
    match state.cache_manager.set(&key, &request.value, cache_type).await {
        Ok(()) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            error!("Failed to set cache value for key {}: {}", key, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_cache_value(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.cache_manager.delete(&key).await {
        Ok(()) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            error!("Failed to delete cache value for key {}: {}", key, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_multiple_cache_values(
    State(state): State<AppState>,
    Json(request): Json<GetMultipleRequest>,
) -> Result<Json<ApiResponse<GetMultipleResponse>>, StatusCode> {
    let mut values = HashMap::new();
    
    for key in request.keys {
        match state.cache_manager.get::<serde_json::Value>(&key).await {
            Ok(value) => {
                values.insert(key, value);
            }
            Err(e) => {
                warn!("Failed to get cache value for key {}: {}", key, e);
                values.insert(key, None);
            }
        }
    }
    
    Ok(Json(ApiResponse::success(GetMultipleResponse { values })))
}

async fn delete_multiple_cache_values(
    State(state): State<AppState>,
    Json(request): Json<GetMultipleRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    for key in request.keys {
        if let Err(e) = state.cache_manager.delete(&key).await {
            warn!("Failed to delete cache value for key {}: {}", key, e);
        }
    }
    
    Ok(Json(ApiResponse::success(())))
}

async fn cache_prayer_times(
    State(_state): State<AppState>,
    Json(_request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // This would parse the prayer times data and cache it appropriately
    // For now, we'll just acknowledge the request
    info!("Caching prayer times data");
    Ok(Json(ApiResponse::success(())))
}

async fn cache_semantic_query(
    State(_state): State<AppState>,
    Json(_request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // This would parse the semantic query and results, then cache them
    info!("Caching semantic query results");
    Ok(Json(ApiResponse::success(())))
}

async fn cache_quran_content(
    State(_state): State<AppState>,
    Json(_request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // This would parse the Quran content and cache it with permanent storage
    info!("Caching Quran content");
    Ok(Json(ApiResponse::success(())))
}

async fn cache_hadith_content(
    State(_state): State<AppState>,
    Json(_request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // This would parse the hadith content and cache it with collection organization
    info!("Caching hadith content");
    Ok(Json(ApiResponse::success(())))
}

async fn invalidate_cache_pattern(
    State(state): State<AppState>,
    Json(request): Json<InvalidatePatternRequest>,
) -> Result<Json<ApiResponse<InvalidateResponse>>, StatusCode> {
    match state.cache_manager.invalidate_pattern(&request.pattern).await {
        Ok(deleted_count) => Ok(Json(ApiResponse::success(InvalidateResponse { deleted_count }))),
        Err(e) => {
            error!("Failed to invalidate cache pattern {}: {}", request.pattern, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn invalidate_prayer_times(
    State(state): State<AppState>,
    Path((lat, lng)): Path<(f64, f64)>,
) -> Result<Json<ApiResponse<InvalidateResponse>>, StatusCode> {
    match state.cache_manager.invalidate_prayer_times(lat, lng).await {
        Ok(deleted_count) => Ok(Json(ApiResponse::success(InvalidateResponse { deleted_count }))),
        Err(e) => {
            error!("Failed to invalidate prayer times cache for {}, {}: {}", lat, lng, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn invalidate_semantic_queries(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<InvalidateResponse>>, StatusCode> {
    match state.cache_manager.invalidate_semantic_queries().await {
        Ok(deleted_count) => Ok(Json(ApiResponse::success(InvalidateResponse { deleted_count }))),
        Err(e) => {
            error!("Failed to invalidate semantic queries cache: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn invalidate_quran_surah(
    State(state): State<AppState>,
    Path(surah): Path<u16>,
) -> Result<Json<ApiResponse<InvalidateResponse>>, StatusCode> {
    match state.cache_manager.invalidate_quran_surah(surah).await {
        Ok(deleted_count) => Ok(Json(ApiResponse::success(InvalidateResponse { deleted_count }))),
        Err(e) => {
            error!("Failed to invalidate Quran surah {} cache: {}", surah, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn invalidate_hadith_collection(
    State(state): State<AppState>,
    Path(collection): Path<String>,
) -> Result<Json<ApiResponse<InvalidateResponse>>, StatusCode> {
    match state.cache_manager.invalidate_hadith_collection(&collection).await {
        Ok(deleted_count) => Ok(Json(ApiResponse::success(InvalidateResponse { deleted_count }))),
        Err(e) => {
            error!("Failed to invalidate hadith collection {} cache: {}", collection, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_cache_stats(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<CacheStats>>, StatusCode> {
    match state.cache_manager.get_cache_stats().await {
        Ok(stats) => Ok(Json(ApiResponse::success(stats))),
        Err(e) => {
            error!("Failed to get cache stats: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn cleanup_cache(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<usize>>, StatusCode> {
    let cleaned_count = state.cache_manager.cleanup_expired_entries().await;
    Ok(Json(ApiResponse::success(cleaned_count)))
}

async fn warmup_cache(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.cache_manager.warm_up_cache().await {
        Ok(()) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            error!("Failed to warm up cache: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("Cache service is healthy".to_string()))
}

async fn cache_frequent_query(
    State(state): State<AppState>,
    Json(request): Json<CacheFrequentQueryRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.cache_manager.cache_frequent_query(&request.query, &request.result).await {
        Ok(()) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            error!("Failed to cache frequent query: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_frequent_query(
    State(state): State<AppState>,
    Json(request): Json<GetFrequentQueryRequest>,
) -> Result<Json<ApiResponse<Option<serde_json::Value>>>, StatusCode> {
    match state.cache_manager.get_frequent_query::<serde_json::Value>(&request.query).await {
        Ok(result) => Ok(Json(ApiResponse::success(result))),
        Err(e) => {
            error!("Failed to get frequent query: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn cache_heavy_content(
    State(state): State<AppState>,
    Path(content_id): Path<String>,
    Json(request): Json<CacheHeavyContentRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Decode base64 data
    let data = match base64::engine::general_purpose::STANDARD.decode(&request.data) {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to decode base64 data: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    match state.cache_manager.cache_heavy_content(&content_id, &data, &request.content_type).await {
        Ok(()) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            error!("Failed to cache heavy content: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_heavy_content(
    State(state): State<AppState>,
    Path(content_id): Path<String>,
) -> Result<Json<ApiResponse<Option<serde_json::Value>>>, StatusCode> {
    match state.cache_manager.get_heavy_content(&content_id).await {
        Ok(Some(data)) => {
            let response = serde_json::json!({
                "data": base64::engine::general_purpose::STANDARD.encode(&data),
                "size": data.len()
            });
            Ok(Json(ApiResponse::success(Some(response))))
        }
        Ok(None) => Ok(Json(ApiResponse::success(None))),
        Err(e) => {
            error!("Failed to get heavy content: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
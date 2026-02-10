//! HTTP Handlers for API Integration Service
//!
//! This module provides REST API endpoints for all service operations including:
//! - Quran text and audio retrieval
//! - Hadith search and retrieval
//! - Prayer times calculation
//! - Tafsir (interpretation) retrieval
//! - Islamic calendar operations
//! - Qibla direction calculation
//! - AI query processing
//! - Health monitoring

use crate::models::*;
use crate::service::ApiIntegrationService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info, warn};
use uuid::Uuid;

// ============================================================================
// State and Error Handling
// ============================================================================

/// Shared application state
pub type AppState = Arc<ApiIntegrationService>;

/// Standard API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiErrorResponse>,
    pub request_id: String,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            request_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn error(error: ApiErrorResponse) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            request_id: Uuid::new_v4().to_string(),
        }
    }
}

/// API error response
#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    pub category: ErrorCategory,
}

/// Custom error type for handlers
#[derive(Debug)]
pub struct HandlerError {
    pub status: StatusCode,
    pub error: ApiErrorResponse,
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let body = Json(ApiResponse::<()>::error(self.error));
        (self.status, body).into_response()
    }
}

impl From<anyhow::Error> for HandlerError {
    fn from(err: anyhow::Error) -> Self {
        error!("Handler error: {}", err);
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: ApiErrorResponse {
                code: "INTERNAL_ERROR".to_string(),
                message: err.to_string(),
                category: ErrorCategory::Unknown,
            },
        }
    }
}

impl From<ApiError> for HandlerError {
    fn from(err: ApiError) -> Self {
        let (status, code, category) = match &err {
            ApiError::Network(_) => (
                StatusCode::BAD_GATEWAY,
                "NETWORK_ERROR",
                ErrorCategory::Network,
            ),
            ApiError::RateLimitExceeded(_) => (
                StatusCode::TOO_MANY_REQUESTS,
                "RATE_LIMIT_EXCEEDED",
                ErrorCategory::RateLimit,
            ),
            ApiError::ApiKeyNotFound(_) | ApiError::ApiKeyInactive(_) | ApiError::ApiKeyExpired(_) => (
                StatusCode::UNAUTHORIZED,
                "AUTHENTICATION_ERROR",
                ErrorCategory::Authentication,
            ),
            ApiError::InvalidResponse(_, _) => (
                StatusCode::BAD_GATEWAY,
                "INVALID_RESPONSE",
                ErrorCategory::ServerError,
            ),
            ApiError::ApiError(_, _) => (
                StatusCode::BAD_GATEWAY,
                "API_ERROR",
                ErrorCategory::ServerError,
            ),
            ApiError::AllApisFailed => (
                StatusCode::SERVICE_UNAVAILABLE,
                "ALL_APIS_FAILED",
                ErrorCategory::ServerError,
            ),
            ApiError::CacheError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "CACHE_ERROR",
                ErrorCategory::Unknown,
            ),
            ApiError::Serialization(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "SERIALIZATION_ERROR",
                ErrorCategory::Validation,
            ),
            ApiError::UnknownApi(_) => (
                StatusCode::BAD_REQUEST,
                "UNKNOWN_API",
                ErrorCategory::Validation,
            ),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                ErrorCategory::Validation,
            ),
            ApiError::Timeout => (
                StatusCode::GATEWAY_TIMEOUT,
                "TIMEOUT",
                ErrorCategory::Timeout,
            ),
            ApiError::Authentication(_) => (
                StatusCode::UNAUTHORIZED,
                "AUTHENTICATION_ERROR",
                ErrorCategory::Authentication,
            ),
            ApiError::Validation(_) => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                ErrorCategory::Validation,
            ),
        };

        Self {
            status,
            error: ApiErrorResponse {
                code: code.to_string(),
                message: err.to_string(),
                category,
            },
        }
    }
}

// ============================================================================
// Quran Handlers
// ============================================================================

/// Query parameters for Quran text request
#[derive(Debug, Deserialize)]
pub struct QuranTextQuery {
    pub surah: u8,
    pub ayah: Option<u16>,
    pub translation: Option<String>,
    pub reciter: Option<String>,
}

/// GET /api/v1/quran/text
/// 
/// Get Quran text for a specific verse or entire surah
/// 
/// Query parameters:
/// - surah: Surah number (1-114)
/// - ayah: Optional ayah number
/// - translation: Optional translation identifier
/// - reciter: Optional reciter name for audio
pub async fn get_quran_text(
    State(service): State<AppState>,
    Query(params): Query<QuranTextQuery>,
) -> Result<Json<ApiResponse<QuranTextResponse>>, HandlerError> {
    info!(
        "Quran text request: surah={}, ayah={:?}",
        params.surah, params.ayah
    );

    // Validate surah number
    if params.surah < 1 || params.surah > 114 {
        return Err(HandlerError {
            status: StatusCode::BAD_REQUEST,
            error: ApiErrorResponse {
                code: "INVALID_SURAH".to_string(),
                message: "Surah number must be between 1 and 114".to_string(),
                category: ErrorCategory::Validation,
            },
        });
    }

    let request = QuranTextRequest {
        surah: params.surah,
        ayah: params.ayah,
        translation: params.translation,
        reciter: params.reciter,
    };

    let response = service.get_quran_text(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

/// Query parameters for Quran audio request
#[derive(Debug, Deserialize)]
pub struct QuranAudioQuery {
    pub surah: u8,
    pub ayah: u16,
    pub reciter: String,
}

/// GET /api/v1/quran/audio
/// 
/// Get audio recitation URL for a specific verse
/// 
/// Query parameters:
/// - surah: Surah number (1-114)
/// - ayah: Ayah number
/// - reciter: Reciter name
pub async fn get_quran_audio(
    State(service): State<AppState>,
    Query(params): Query<QuranAudioQuery>,
) -> Result<Json<ApiResponse<QuranAudioResponse>>, HandlerError> {
    info!(
        "Quran audio request: surah={}, ayah={}, reciter={}",
        params.surah, params.ayah, params.reciter
    );

    // Validate surah number
    if params.surah < 1 || params.surah > 114 {
        return Err(HandlerError {
            status: StatusCode::BAD_REQUEST,
            error: ApiErrorResponse {
                code: "INVALID_SURAH".to_string(),
                message: "Surah number must be between 1 and 114".to_string(),
                category: ErrorCategory::Validation,
            },
        });
    }

    let request = QuranAudioRequest {
        surah: params.surah,
        ayah: params.ayah,
        reciter: params.reciter,
    };

    let response = service.get_quran_audio(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

// ============================================================================
// Hadith Handlers
// ============================================================================

/// Query parameters for hadith search
#[derive(Debug, Deserialize)]
pub struct HadithSearchQuery {
    pub query: String,
    pub collection: Option<String>,
    pub book: Option<String>,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_language() -> String {
    "en".to_string()
}

fn default_limit() -> usize {
    10
}

/// GET /api/v1/hadith/search
/// 
/// Search for hadith across multiple collections
/// 
/// Query parameters:
/// - query: Search query text
/// - collection: Optional collection filter (e.g., "bukhari", "muslim")
/// - book: Optional book filter
/// - language: Language code (default: "en")
/// - limit: Maximum results (default: 10, max: 100)
pub async fn search_hadith(
    State(service): State<AppState>,
    Query(params): Query<HadithSearchQuery>,
) -> Result<Json<ApiResponse<HadithSearchResponse>>, HandlerError> {
    info!(
        "Hadith search request: query='{}', collection={:?}, limit={}",
        params.query, params.collection, params.limit
    );

    // Validate query
    if params.query.trim().is_empty() {
        return Err(HandlerError {
            status: StatusCode::BAD_REQUEST,
            error: ApiErrorResponse {
                code: "EMPTY_QUERY".to_string(),
                message: "Search query cannot be empty".to_string(),
                category: ErrorCategory::Validation,
            },
        });
    }

    // Validate limit
    let limit = params.limit.min(100);

    let request = HadithSearchRequest {
        query: params.query,
        collection: params.collection,
        book: params.book,
        language: params.language,
        limit,
    };

    let response = service.search_hadith(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

/// Path parameters for hadith by ID
#[derive(Debug, Deserialize)]
pub struct HadithByIdPath {
    pub collection: String,
    pub id: String,
}

/// GET /api/v1/hadith/:collection/:id
/// 
/// Get a specific hadith by collection and ID
/// 
/// Path parameters:
/// - collection: Collection name (e.g., "bukhari", "muslim")
/// - id: Hadith ID within the collection
pub async fn get_hadith_by_id(
    State(service): State<AppState>,
    Path(params): Path<HadithByIdPath>,
) -> Result<Json<ApiResponse<HadithResponse>>, HandlerError> {
    info!(
        "Hadith by ID request: collection={}, id={}",
        params.collection, params.id
    );

    let request = HadithByIdRequest {
        id: params.id,
        collection: params.collection,
    };

    let response = service.get_hadith_by_id(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

// ============================================================================
// Prayer Times Handlers
// ============================================================================

/// POST /api/v1/prayer-times
/// 
/// Get prayer times for a specific location and date
/// 
/// Request body: PrayerTimesRequest
pub async fn get_prayer_times(
    State(service): State<AppState>,
    Json(request): Json<PrayerTimesRequest>,
) -> Result<Json<ApiResponse<PrayerTimesResponse>>, HandlerError> {
    info!(
        "Prayer times request: lat={}, lon={}, date={}",
        request.latitude, request.longitude, request.date
    );

    // Validate coordinates
    if request.latitude < -90.0 || request.latitude > 90.0 {
        return Err(HandlerError {
            status: StatusCode::BAD_REQUEST,
            error: ApiErrorResponse {
                code: "INVALID_LATITUDE".to_string(),
                message: "Latitude must be between -90 and 90".to_string(),
                category: ErrorCategory::Validation,
            },
        });
    }

    if request.longitude < -180.0 || request.longitude > 180.0 {
        return Err(HandlerError {
            status: StatusCode::BAD_REQUEST,
            error: ApiErrorResponse {
                code: "INVALID_LONGITUDE".to_string(),
                message: "Longitude must be between -180 and 180".to_string(),
                category: ErrorCategory::Validation,
            },
        });
    }

    let response = service.get_prayer_times(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

// ============================================================================
// Tafsir Handlers
// ============================================================================

/// Query parameters for tafsir request
#[derive(Debug, Deserialize)]
pub struct TafsirQuery {
    pub surah: u8,
    pub ayah: u16,
    pub tafsir_id: Option<String>,
    #[serde(default = "default_language")]
    pub language: String,
}

/// GET /api/v1/tafsir
/// 
/// Get tafsir (interpretation) for a specific verse
/// 
/// Query parameters:
/// - surah: Surah number (1-114)
/// - ayah: Ayah number
/// - tafsir_id: Optional specific tafsir source
/// - language: Language code (default: "en")
pub async fn get_tafsir(
    State(service): State<AppState>,
    Query(params): Query<TafsirQuery>,
) -> Result<Json<ApiResponse<TafsirResponse>>, HandlerError> {
    info!(
        "Tafsir request: surah={}, ayah={}, tafsir_id={:?}",
        params.surah, params.ayah, params.tafsir_id
    );

    // Validate surah number
    if params.surah < 1 || params.surah > 114 {
        return Err(HandlerError {
            status: StatusCode::BAD_REQUEST,
            error: ApiErrorResponse {
                code: "INVALID_SURAH".to_string(),
                message: "Surah number must be between 1 and 114".to_string(),
                category: ErrorCategory::Validation,
            },
        });
    }

    let request = TafsirRequest {
        surah: params.surah,
        ayah: params.ayah,
        tafsir_id: params.tafsir_id,
        language: params.language,
    };

    let response = service.get_tafsir(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

// ============================================================================
// Calendar Handlers
// ============================================================================

/// POST /api/v1/calendar/convert
/// 
/// Convert between Gregorian and Hijri dates
/// 
/// Request body: DateConversionRequest
pub async fn convert_date(
    State(service): State<AppState>,
    Json(request): Json<DateConversionRequest>,
) -> Result<Json<ApiResponse<DateConversionResponse>>, HandlerError> {
    info!(
        "Date conversion request: date={}, direction={:?}",
        request.date, request.direction
    );

    let response = service.convert_date(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

/// POST /api/v1/calendar/events
/// 
/// Get Islamic events for a date range
/// 
/// Request body: IslamicEventsRequest
pub async fn get_islamic_events(
    State(service): State<AppState>,
    Json(request): Json<IslamicEventsRequest>,
) -> Result<Json<ApiResponse<IslamicEventsResponse>>, HandlerError> {
    info!(
        "Islamic events request: start={}, end={}",
        request.start_date, request.end_date
    );

    // Validate date range
    if request.end_date < request.start_date {
        return Err(HandlerError {
            status: StatusCode::BAD_REQUEST,
            error: ApiErrorResponse {
                code: "INVALID_DATE_RANGE".to_string(),
                message: "End date must be after start date".to_string(),
                category: ErrorCategory::Validation,
            },
        });
    }

    let response = service.get_islamic_events(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

// ============================================================================
// Qibla Handlers
// ============================================================================

/// POST /api/v1/qibla
/// 
/// Get Qibla direction for a specific location
/// 
/// Request body: QiblaRequest
pub async fn get_qibla_direction(
    State(service): State<AppState>,
    Json(request): Json<QiblaRequest>,
) -> Result<Json<ApiResponse<QiblaResponse>>, HandlerError> {
    info!(
        "Qibla direction request: lat={}, lon={}",
        request.latitude, request.longitude
    );

    // Validate coordinates
    if request.latitude < -90.0 || request.latitude > 90.0 {
        return Err(HandlerError {
            status: StatusCode::BAD_REQUEST,
            error: ApiErrorResponse {
                code: "INVALID_LATITUDE".to_string(),
                message: "Latitude must be between -90 and 90".to_string(),
                category: ErrorCategory::Validation,
            },
        });
    }

    if request.longitude < -180.0 || request.longitude > 180.0 {
        return Err(HandlerError {
            status: StatusCode::BAD_REQUEST,
            error: ApiErrorResponse {
                code: "INVALID_LONGITUDE".to_string(),
                message: "Longitude must be between -180 and 180".to_string(),
                category: ErrorCategory::Validation,
            },
        });
    }

    let response = service.get_qibla_direction(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

// ============================================================================
// AI Handlers
// ============================================================================

/// POST /api/v1/ai/query
/// 
/// Process an AI query with Islamic context
/// 
/// Request body: AiQueryRequest
pub async fn process_ai_query(
    State(service): State<AppState>,
    Json(request): Json<AiQueryRequest>,
) -> Result<Json<ApiResponse<AiQueryResponse>>, HandlerError> {
    info!("AI query request: query='{}'", request.query);

    // Validate query
    if request.query.trim().is_empty() {
        return Err(HandlerError {
            status: StatusCode::BAD_REQUEST,
            error: ApiErrorResponse {
                code: "EMPTY_QUERY".to_string(),
                message: "Query cannot be empty".to_string(),
                category: ErrorCategory::Validation,
            },
        });
    }

    let response = service.process_ai_query(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

// ============================================================================
// Health Check Handler
// ============================================================================

/// GET /api/v1/health
/// 
/// Get the health status of the service and all APIs
pub async fn health_check(
    State(service): State<AppState>,
) -> Result<Json<ApiResponse<HealthStatus>>, HandlerError> {
    info!("Health check request");

    let health = service.health_check().await;
    
    // Note: In a full implementation, we would use status_code to set the HTTP response status
    // based on the health status (OK for Healthy, SERVICE_UNAVAILABLE for Unhealthy)
    // For now, we always return OK since the service is in development mode

    if health.overall_status != ServiceStatus::Healthy {
        warn!("Service health is {:?}", health.overall_status);
    }

    Ok(Json(ApiResponse::success(health)))
}

// ============================================================================
// Router Configuration
// ============================================================================

use axum::{
    routing::{get, post},
    Router,
};

/// Create the API router with all endpoints
pub fn create_router(service: Arc<ApiIntegrationService>) -> Router {
    Router::new()
        // Quran endpoints
        .route("/api/v1/quran/text", get(get_quran_text))
        .route("/api/v1/quran/audio", get(get_quran_audio))
        // Hadith endpoints
        .route("/api/v1/hadith/search", get(search_hadith))
        .route("/api/v1/hadith/:collection/:id", get(get_hadith_by_id))
        // Prayer times endpoint
        .route("/api/v1/prayer-times", post(get_prayer_times))
        // Tafsir endpoint
        .route("/api/v1/tafsir", get(get_tafsir))
        // Calendar endpoints
        .route("/api/v1/calendar/convert", post(convert_date))
        .route("/api/v1/calendar/events", post(get_islamic_events))
        // Qibla endpoint
        .route("/api/v1/qibla", post(get_qibla_direction))
        // AI endpoint
        .route("/api/v1/ai/query", post(process_ai_query))
        // Health check endpoint
        .route("/api/v1/health", get(health_check))
        .with_state(service)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::Service; // for `call`

    async fn create_test_service() -> Arc<ApiIntegrationService> {
        let config = ServiceConfig {
            service: ServiceInfo {
                name: "test-service".to_string(),
                port: 8080,
                host: "localhost".to_string(),
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                pool_size: 10,
                connection_timeout: "5s".to_string(),
            },
            postgres: PostgresConfig {
                url: "postgresql://localhost:5432/test".to_string(),
                pool_size: 20,
                connection_timeout: "10s".to_string(),
            },
            apis: ApiConfigs {
                quran: vec![],
                hadith: vec![],
                prayer_times: vec![],
                tafsir: vec![],
                calendar: vec![],
                qibla: vec![],
                ai: vec![],
            },
            cache: CacheConfig {
                strategies: CacheStrategies {
                    quran_text: CacheStrategy {
                        ttl: "30d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("90d".to_string()),
                    },
                    hadith: CacheStrategy {
                        ttl: "30d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("90d".to_string()),
                    },
                    prayer_times: CacheStrategy {
                        ttl: "1d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("7d".to_string()),
                    },
                    tafsir: CacheStrategy {
                        ttl: "30d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("90d".to_string()),
                    },
                    calendar: CacheStrategy {
                        ttl: "7d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("30d".to_string()),
                    },
                    qibla: CacheStrategy {
                        ttl: "30d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("90d".to_string()),
                    },
                    ai_response: CacheStrategy {
                        ttl: "1h".to_string(),
                        allow_stale: false,
                        stale_ttl: None,
                    },
                },
            },
            health_monitor: HealthMonitorConfig {
                check_interval: "5m".to_string(),
                unhealthy_threshold: 3,
                recovery_threshold: 2,
            },
            retry: RetryConfig {
                max_attempts: 3,
                initial_delay: "1s".to_string(),
                max_delay: "10s".to_string(),
                multiplier: 2.0,
            },
        };

        Arc::new(ApiIntegrationService::new(config).await.unwrap())
    }

    #[tokio::test]
    async fn test_health_check_endpoint() {
        let service = create_test_service().await;
        let mut app = create_router(service);

        let response = app
            .call(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_invalid_surah_number() {
        let service = create_test_service().await;
        let mut app = create_router(service);

        let response = app
            .call(
                Request::builder()
                    .uri("/api/v1/quran/text?surah=200")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_empty_hadith_query() {
        let service = create_test_service().await;
        let mut app = create_router(service);

        let response = app
            .call(
                Request::builder()
                    .uri("/api/v1/hadith/search?query=")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

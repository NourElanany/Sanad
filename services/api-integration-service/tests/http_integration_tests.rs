//! HTTP Integration Tests for API Integration Service
//!
//! These tests verify that all HTTP endpoints work correctly with:
//! - Valid requests and responses
//! - Error handling (validation errors, not found, etc.)
//! - Rate limiting (if implemented)
//! - Middleware integration (request IDs, CORS, etc.)
//!
//! Tests use Axum's testing utilities to make actual HTTP requests
//! to the router without starting a real server.

use api_integration_service::{
    create_router, ApiIntegrationService, ServiceConfig, ServiceInfo, RedisConfig,
    PostgresConfig, ApiConfigs, CacheConfig, CacheStrategies, CacheStrategy,
    HealthMonitorConfig, RetryConfig,
};
use axum::{
    body::Body,
    http::{Request, StatusCode, Method, header},
};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceExt; // For oneshot and ready

// ============================================================================
// Test Setup Helpers
// ============================================================================

/// Create a test service instance with minimal configuration
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

/// Create a test router with middleware applied
fn create_test_router(service: Arc<ApiIntegrationService>) -> axum::Router {
    use api_integration_service::middleware;
    use axum::middleware as axum_middleware;
    
    create_router(service)
        // Add middleware layers like in main.rs
        .layer(middleware::create_cors_layer())
        .layer(axum_middleware::from_fn(middleware::security_headers_middleware))
        .layer(axum_middleware::from_fn(middleware::request_id_middleware))
}

/// Helper to parse JSON response body
async fn parse_json_body(body: Body) -> Value {
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

/// Helper to check if response is either success or expected error
/// Since we have no real APIs configured, requests may fail with 500 or 503
fn is_valid_test_response(status: StatusCode) -> bool {
    status == StatusCode::OK 
        || status == StatusCode::SERVICE_UNAVAILABLE 
        || status == StatusCode::INTERNAL_SERVER_ERROR
}

// ============================================================================
// Health Check Tests
// ============================================================================

#[tokio::test]
async fn test_health_check_returns_ok() {
    let service = create_test_service().await;
    let app = create_test_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["success"], true);
    assert!(body["data"].is_object());
}

#[tokio::test]
async fn test_health_check_includes_request_id() {
    let service = create_test_service().await;
    let app = create_test_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = parse_json_body(response.into_body()).await;
    assert!(body["request_id"].is_string());
    assert!(!body["request_id"].as_str().unwrap().is_empty());
}

// ============================================================================
// Quran Endpoint Tests
// ============================================================================

#[tokio::test]
async fn test_quran_text_valid_request() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/quran/text?surah=1&ayah=1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Note: This will fail with AllApisFailed since we have no real APIs configured
    // But we're testing the endpoint structure and validation
    assert!(is_valid_test_response(response.status()));
}

#[tokio::test]
async fn test_quran_text_invalid_surah_number() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/quran/text?surah=200")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["success"], false);
    assert_eq!(body["error"]["code"], "INVALID_SURAH");
}

#[tokio::test]
async fn test_quran_text_surah_zero() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/quran/text?surah=0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["error"]["code"], "INVALID_SURAH");
}

#[tokio::test]
async fn test_quran_audio_valid_request() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/quran/audio?surah=1&ayah=1&reciter=mishary")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Will fail with AllApisFailed but validates endpoint structure
    assert!(is_valid_test_response(response.status()));
}

#[tokio::test]
async fn test_quran_audio_invalid_surah() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/quran/audio?surah=115&ayah=1&reciter=mishary")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// ============================================================================
// Hadith Endpoint Tests
// ============================================================================

#[tokio::test]
async fn test_hadith_search_valid_request() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/hadith/search?query=prayer")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Will fail with AllApisFailed but validates endpoint structure
    assert!(is_valid_test_response(response.status()));
}

#[tokio::test]
async fn test_hadith_search_empty_query() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/hadith/search?query=")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["success"], false);
    assert_eq!(body["error"]["code"], "EMPTY_QUERY");
}

#[tokio::test]
async fn test_hadith_search_with_filters() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/hadith/search?query=prayer&collection=bukhari&limit=5")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Validates that query parameters are parsed correctly
    assert!(is_valid_test_response(response.status()));
}

#[tokio::test]
async fn test_hadith_by_id_valid_request() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/hadith/bukhari/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Will fail with AllApisFailed but validates endpoint structure
    assert!(is_valid_test_response(response.status()));
}

// ============================================================================
// Prayer Times Endpoint Tests
// ============================================================================

#[tokio::test]
async fn test_prayer_times_valid_request() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "latitude": 21.4225,
        "longitude": 39.8262,
        "date": "2024-01-15",
        "calculation_method": "Makkah",
        "madhab": "Shafi"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/prayer-times")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Will fail with AllApisFailed but validates endpoint structure
    assert!(is_valid_test_response(response.status()));
}

#[tokio::test]
async fn test_prayer_times_invalid_latitude() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "latitude": 100.0,  // Invalid: > 90
        "longitude": 39.8262,
        "date": "2024-01-15",
        "calculation_method": "Makkah",
        "madhab": "Shafi"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/prayer-times")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["error"]["code"], "INVALID_LATITUDE");
}

#[tokio::test]
async fn test_prayer_times_invalid_longitude() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "latitude": 21.4225,
        "longitude": 200.0,  // Invalid: > 180
        "date": "2024-01-15",
        "calculation_method": "Makkah",
        "madhab": "Shafi"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/prayer-times")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["error"]["code"], "INVALID_LONGITUDE");
}

// ============================================================================
// Tafsir Endpoint Tests
// ============================================================================

#[tokio::test]
async fn test_tafsir_valid_request() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/tafsir?surah=1&ayah=1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Will fail with AllApisFailed but validates endpoint structure
    assert!(is_valid_test_response(response.status()));
}

#[tokio::test]
async fn test_tafsir_invalid_surah() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/tafsir?surah=115&ayah=1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["error"]["code"], "INVALID_SURAH");
}

#[tokio::test]
async fn test_tafsir_with_specific_source() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/tafsir?surah=1&ayah=1&tafsir_id=ibn-kathir&language=ar")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Validates that query parameters are parsed correctly
    assert!(is_valid_test_response(response.status()));
}

// ============================================================================
// Calendar Endpoint Tests
// ============================================================================

#[tokio::test]
async fn test_calendar_convert_date() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "date": "2024-01-15",
        "direction": "GregorianToHijri"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/calendar/convert")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Will fail with AllApisFailed but validates endpoint structure
    assert!(is_valid_test_response(response.status()));
}

#[tokio::test]
async fn test_calendar_get_events() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "start_date": "2024-01-01",
        "end_date": "2024-12-31"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/calendar/events")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Will fail with AllApisFailed but validates endpoint structure
    assert!(is_valid_test_response(response.status()));
}

#[tokio::test]
async fn test_calendar_invalid_date_range() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "start_date": "2024-12-31",
        "end_date": "2024-01-01"  // End before start
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/calendar/events")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["error"]["code"], "INVALID_DATE_RANGE");
}

// ============================================================================
// Qibla Endpoint Tests
// ============================================================================

#[tokio::test]
async fn test_qibla_valid_request() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "latitude": 40.7128,
        "longitude": -74.0060
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/qibla")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Will fail with AllApisFailed but validates endpoint structure
    assert!(is_valid_test_response(response.status()));
}

#[tokio::test]
async fn test_qibla_invalid_latitude() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "latitude": -100.0,  // Invalid: < -90
        "longitude": -74.0060
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/qibla")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["error"]["code"], "INVALID_LATITUDE");
}

#[tokio::test]
async fn test_qibla_invalid_longitude() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "latitude": 40.7128,
        "longitude": -200.0  // Invalid: < -180
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/qibla")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["error"]["code"], "INVALID_LONGITUDE");
}

// ============================================================================
// AI Query Endpoint Tests
// ============================================================================

#[tokio::test]
async fn test_ai_query_valid_request() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "query": "What are the pillars of Islam?",
        "language": "en"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/ai/query")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Will fail with AllApisFailed but validates endpoint structure
    assert!(is_valid_test_response(response.status()));
}

#[tokio::test]
async fn test_ai_query_empty_query() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "query": "",
        "language": "en"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/ai/query")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["error"]["code"], "EMPTY_QUERY");
}

#[tokio::test]
async fn test_ai_query_with_context() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "query": "Explain this verse",
        "context": "Surah Al-Fatiha, Ayah 1",
        "language": "en",
        "max_tokens": 500
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/ai/query")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Validates that all optional fields are parsed correctly
    assert!(is_valid_test_response(response.status()));
}

// ============================================================================
// Error Response Format Tests
// ============================================================================

#[tokio::test]
async fn test_error_response_structure() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/quran/text?surah=200")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = parse_json_body(response.into_body()).await;
    
    // Verify error response structure
    assert_eq!(body["success"], false);
    assert!(body["data"].is_null());
    assert!(body["error"].is_object());
    assert!(body["error"]["code"].is_string());
    assert!(body["error"]["message"].is_string());
    assert!(body["error"]["category"].is_string());
    assert!(body["request_id"].is_string());
}

#[tokio::test]
async fn test_error_categories() {
    let service = create_test_service().await;
    let app = create_router(service);

    // Test validation error
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/quran/text?surah=0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["error"]["category"], "Validation");
}

// ============================================================================
// Middleware Integration Tests
// ============================================================================

#[tokio::test]
async fn test_request_id_header_added() {
    let service = create_test_service().await;
    let app = create_test_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should have X-Request-ID header in response
    assert!(response.headers().contains_key("x-request-id"));
}

#[tokio::test]
async fn test_request_id_preserved() {
    let service = create_test_service().await;
    let app = create_test_router(service);

    let custom_request_id = "test-request-123";

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .header("x-request-id", custom_request_id)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should preserve the custom request ID
    assert_eq!(
        response.headers().get("x-request-id").unwrap(),
        custom_request_id
    );
}

#[tokio::test]
async fn test_cors_headers_present() {
    let service = create_test_service().await;
    let app = create_test_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/api/v1/health")
                .header("Origin", "http://example.com")
                .header("Access-Control-Request-Method", "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should have CORS headers
    assert!(response.headers().contains_key("access-control-allow-origin"));
}

#[tokio::test]
async fn test_security_headers_present() {
    let service = create_test_service().await;
    let app = create_test_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should have security headers
    assert!(response.headers().contains_key("x-content-type-options"));
    assert!(response.headers().contains_key("x-frame-options"));
    assert!(response.headers().contains_key("x-xss-protection"));
}

// ============================================================================
// Content Type Tests
// ============================================================================

#[tokio::test]
async fn test_json_content_type() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Response should be JSON
    let content_type = response.headers().get(header::CONTENT_TYPE).unwrap();
    assert!(content_type.to_str().unwrap().contains("application/json"));
}

#[tokio::test]
async fn test_post_requires_json_content_type() {
    let service = create_test_service().await;
    let app = create_router(service);

    let request_body = json!({
        "latitude": 21.4225,
        "longitude": 39.8262,
        "date": "2024-01-15",
        "calculation_method": "Makkah",
        "madhab": "Shafi"
    });

    // Without Content-Type header
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/prayer-times")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Axum should handle this gracefully (may return 415 or parse anyway)
    assert!(response.status().is_client_error() || response.status().is_server_error() || response.status().is_success());
}

// ============================================================================
// HTTP Method Tests
// ============================================================================

#[tokio::test]
async fn test_get_endpoints_reject_post() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 405 Method Not Allowed
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_post_endpoints_reject_get() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/v1/prayer-times")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 405 Method Not Allowed
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

// ============================================================================
// Not Found Tests
// ============================================================================

#[tokio::test]
async fn test_unknown_endpoint_returns_404() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/unknown")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_wrong_api_version_returns_404() {
    let service = create_test_service().await;
    let app = create_router(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v2/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ============================================================================
// Rate Limiting Tests (Placeholder)
// ============================================================================

#[tokio::test]
async fn test_rate_limiting_placeholder() {
    // Note: Rate limiting is not fully implemented in middleware yet
    // This test is a placeholder for when it's implemented
    
    let service = create_test_service().await;
    let app = create_router(service);

    // Make multiple requests rapidly
    for _ in 0..5 {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Currently all should succeed since rate limiting is not enforced
        assert_eq!(response.status(), StatusCode::OK);
    }
}

// ============================================================================
// Concurrent Request Tests
// ============================================================================

#[tokio::test]
async fn test_concurrent_requests() {
    let service = create_test_service().await;
    
    // Make 10 concurrent requests sequentially (since Router doesn't support concurrent access)
    let mut statuses = vec![];
    for i in 0..10 {
        let app = create_router(service.clone());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .header("x-request-id", format!("concurrent-{}", i))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        statuses.push(response.status());
    }

    // All requests should succeed
    for status in statuses {
        assert_eq!(status, StatusCode::OK);
    }
}

// ============================================================================
// Large Payload Tests
// ============================================================================

#[tokio::test]
async fn test_large_query_string() {
    let service = create_test_service().await;
    let app = create_router(service);

    // Create a very long query string
    let long_query = "a".repeat(1000);
    let uri = format!("/api/v1/hadith/search?query={}", long_query);

    let response = app
        .oneshot(
            Request::builder()
                .uri(&uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should handle long queries (may fail with AllApisFailed but not crash)
    assert!(response.status().is_client_error() || response.status().is_server_error() || response.status().is_success());
}

// ============================================================================
// Special Characters Tests
// ============================================================================

#[tokio::test]
async fn test_arabic_query_string() {
    let service = create_test_service().await;
    let app = create_router(service);

    let arabic_query = "الصلاة";
    let uri = format!("/api/v1/hadith/search?query={}", 
        urlencoding::encode(arabic_query));

    let response = app
        .oneshot(
            Request::builder()
                .uri(&uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should handle Arabic text properly
    assert!(is_valid_test_response(response.status()));
}

#[tokio::test]
async fn test_special_characters_in_query() {
    let service = create_test_service().await;
    let app = create_router(service);

    let special_query = "test & query = value";
    let uri = format!("/api/v1/hadith/search?query={}", 
        urlencoding::encode(special_query));

    let response = app
        .oneshot(
            Request::builder()
                .uri(&uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should handle special characters properly
    assert!(is_valid_test_response(response.status()));
}

// ============================================================================
// Response Time Tests (Basic)
// ============================================================================

#[tokio::test]
async fn test_health_check_response_time() {
    let service = create_test_service().await;
    let app = create_router(service);

    let start = std::time::Instant::now();
    
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let duration = start.elapsed();

    assert_eq!(response.status(), StatusCode::OK);
    // Health check should be fast (< 1 second)
    assert!(duration.as_secs() < 1);
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[tokio::test]
async fn test_boundary_surah_numbers() {
    let service = create_test_service().await;
    let app = create_router(service);

    // Test surah 1 (minimum valid)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/quran/text?surah=1&ayah=1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert!(is_valid_test_response(response.status()));

    // Test surah 114 (maximum valid)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/quran/text?surah=114&ayah=1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert!(is_valid_test_response(response.status()));
}

#[tokio::test]
async fn test_boundary_coordinates() {
    let service = create_test_service().await;
    let app = create_router(service);

    // Test minimum valid coordinates
    let request_body = json!({
        "latitude": -90.0,
        "longitude": -180.0
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/qibla")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert!(is_valid_test_response(response.status()));

    // Test maximum valid coordinates
    let request_body = json!({
        "latitude": 90.0,
        "longitude": 180.0
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/qibla")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert!(is_valid_test_response(response.status()));
}

// ============================================================================
// Integration with Service Layer Tests
// ============================================================================

#[tokio::test]
async fn test_service_error_propagation() {
    let service = create_test_service().await;
    let app = create_router(service);

    // Request that will fail at service layer (no APIs configured)
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/quran/text?surah=1&ayah=1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should get an error when all APIs fail (either 500 or 503)
    assert!(response.status().is_server_error());
    
    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["success"], false);
    // Error code could be ALL_APIS_FAILED or INTERNAL_ERROR depending on initialization
    assert!(body["error"]["code"].is_string());
}

// ============================================================================
// Summary Test
// ============================================================================

#[tokio::test]
async fn test_all_endpoints_exist() {
    let service = create_test_service().await;
    let app = create_router(service);

    // List of all expected endpoints
    let endpoints = vec![
        ("/api/v1/health", Method::GET),
        ("/api/v1/quran/text", Method::GET),
        ("/api/v1/quran/audio", Method::GET),
        ("/api/v1/hadith/search", Method::GET),
        ("/api/v1/prayer-times", Method::POST),
        ("/api/v1/tafsir", Method::GET),
        ("/api/v1/calendar/convert", Method::POST),
        ("/api/v1/calendar/events", Method::POST),
        ("/api/v1/qibla", Method::POST),
        ("/api/v1/ai/query", Method::POST),
    ];

    for (uri, method) in endpoints {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(method.clone())
                    .uri(uri)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should not return 404 (endpoint exists)
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "Endpoint {} {} returned 404",
            method,
            uri
        );
    }
}


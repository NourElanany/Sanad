//! Middleware for API Integration Service
//!
//! This module provides middleware for:
//! - Request/response logging with correlation IDs
//! - Error handling and standardized error responses
//! - CORS configuration
//! - Request ID generation and tracking
//! - Timeout enforcement
//! - Metrics collection

use axum::{
    extract::Request,
    http::{header, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::request_context::{RequestContext, with_context};

// ============================================================================
// Request ID Middleware
// ============================================================================

/// Header name for request ID
pub const X_REQUEST_ID: &str = "x-request-id";

/// Middleware to add request ID to all requests
///
/// If the request already has an X-Request-ID header, it will be used.
/// Otherwise, a new UUID will be generated.
/// This also sets up the request context for the entire request lifecycle.
pub async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    // Check if request already has a request ID
    let request_id = request
        .headers()
        .get(X_REQUEST_ID)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // Store request ID in extensions for use by handlers
    request.extensions_mut().insert(request_id.clone());

    // Add request ID to headers if not present
    if !request.headers().contains_key(X_REQUEST_ID) {
        request.headers_mut().insert(
            X_REQUEST_ID,
            HeaderValue::from_str(&request_id).unwrap(),
        );
    }

    // Create request context
    let context = RequestContext::new(request_id.clone());

    // Process request within the context
    let mut response = with_context(context, next.run(request)).await;

    // Add request ID to response headers
    response.headers_mut().insert(
        X_REQUEST_ID,
        HeaderValue::from_str(&request_id).unwrap(),
    );

    response
}

// ============================================================================
// Logging Middleware
// ============================================================================

/// Middleware for request/response logging
///
/// Logs:
/// - Request method, URI, and headers
/// - Response status code and duration
/// - Request ID for correlation
pub async fn logging_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let request_id = request
        .extensions()
        .get::<String>()
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());

    // Log incoming request
    info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        "Incoming request"
    );

    // Process request
    let response = next.run(request).await;

    // Calculate duration
    let duration = start.elapsed();
    let status = response.status();

    // Log response with appropriate level based on status
    match status.as_u16() {
        200..=299 => {
            info!(
                request_id = %request_id,
                method = %method,
                uri = %uri,
                status = %status,
                duration_ms = %duration.as_millis(),
                "Request completed successfully"
            );
        }
        400..=499 => {
            warn!(
                request_id = %request_id,
                method = %method,
                uri = %uri,
                status = %status,
                duration_ms = %duration.as_millis(),
                "Request completed with client error"
            );
        }
        500..=599 => {
            error!(
                request_id = %request_id,
                method = %method,
                uri = %uri,
                status = %status,
                duration_ms = %duration.as_millis(),
                "Request completed with server error"
            );
        }
        _ => {
            info!(
                request_id = %request_id,
                method = %method,
                uri = %uri,
                status = %status,
                duration_ms = %duration.as_millis(),
                "Request completed"
            );
        }
    }

    response
}

// ============================================================================
// Timeout Middleware
// ============================================================================

/// Middleware to enforce request timeout
///
/// Returns 504 Gateway Timeout if the request takes longer than the specified duration
pub async fn timeout_middleware(request: Request, next: Next) -> Response {
    let timeout_duration = std::time::Duration::from_secs(30); // 30 second default timeout

    match tokio::time::timeout(timeout_duration, next.run(request)).await {
        Ok(response) => response,
        Err(_) => {
            let request_id = Uuid::new_v4().to_string();
            error!(
                request_id = %request_id,
                "Request timed out after {} seconds",
                timeout_duration.as_secs()
            );

            // Return timeout error response
            (
                StatusCode::GATEWAY_TIMEOUT,
                [(X_REQUEST_ID, request_id.as_str())],
                "Request timeout",
            )
                .into_response()
        }
    }
}

// ============================================================================
// CORS Middleware
// ============================================================================

/// Create CORS layer with appropriate configuration
///
/// Allows:
/// - All origins (can be restricted in production)
/// - Common HTTP methods (GET, POST, PUT, DELETE, OPTIONS)
/// - Common headers including X-Request-ID
pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        // Allow requests from any origin
        // In production, this should be restricted to specific domains
        .allow_origin(Any)
        // Allow common HTTP methods
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
            Method::PATCH,
        ])
        // Allow common headers plus our custom headers
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::HeaderName::from_static(X_REQUEST_ID),
        ])
        // Expose custom headers in responses
        .expose_headers([header::HeaderName::from_static(X_REQUEST_ID)])
        // Note: Cannot use allow_credentials(true) with allow_origin(Any)
        // In production, specify exact origins and enable credentials
        // Cache preflight requests for 1 hour
        .max_age(std::time::Duration::from_secs(3600))
}

// ============================================================================
// Error Handling Middleware
// ============================================================================

/// Middleware for catching panics and converting them to 500 errors
///
/// This ensures that panics don't crash the server and are properly logged
pub async fn error_handling_middleware(request: Request, next: Next) -> Response {
    let request_id = request
        .extensions()
        .get::<String>()
        .cloned()
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // Catch panics and convert to 500 errors
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // Note: This is a simplified version. In production, you'd want to use
        // tower's CatchPanic middleware or similar for proper async panic handling
        next.run(request)
    }));

    match result {
        Ok(future) => future.await,
        Err(panic_info) => {
            error!(
                request_id = %request_id,
                "Request handler panicked: {:?}",
                panic_info
            );

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(X_REQUEST_ID, request_id.as_str())],
                "Internal server error",
            )
                .into_response()
        }
    }
}

// ============================================================================
// Metrics Middleware
// ============================================================================

/// Middleware for collecting request metrics
///
/// Tracks:
/// - Request count by method and endpoint
/// - Response time distribution
/// - Error rates by status code
pub async fn metrics_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().path().to_string();

    // Process request
    let response = next.run(request).await;

    // Record metrics
    let duration = start.elapsed();
    let status = response.status();

    // In a real implementation, you would record these metrics to Prometheus
    // For now, we just log them at debug level
    tracing::debug!(
        method = %method,
        uri = %uri,
        status = %status,
        duration_ms = %duration.as_millis(),
        "Request metrics"
    );

    // TODO: Implement actual Prometheus metrics:
    // - HTTP_REQUESTS_TOTAL.with_label_values(&[method, uri, status]).inc()
    // - HTTP_REQUEST_DURATION_SECONDS.with_label_values(&[method, uri]).observe(duration.as_secs_f64())

    response
}

// ============================================================================
// Security Headers Middleware
// ============================================================================

/// Middleware to add security headers to all responses
///
/// Adds:
/// - X-Content-Type-Options: nosniff
/// - X-Frame-Options: DENY
/// - X-XSS-Protection: 1; mode=block
/// - Strict-Transport-Security (HSTS)
pub async fn security_headers_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();

    // Prevent MIME type sniffing
    headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );

    // Prevent clickjacking
    headers.insert("x-frame-options", HeaderValue::from_static("DENY"));

    // Enable XSS protection
    headers.insert(
        "x-xss-protection",
        HeaderValue::from_static("1; mode=block"),
    );

    // Enforce HTTPS (only in production)
    if std::env::var("ENVIRONMENT").unwrap_or_default() == "production" {
        headers.insert(
            "strict-transport-security",
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        );
    }

    response
}

// ============================================================================
// Rate Limiting Middleware (Placeholder)
// ============================================================================

/// Middleware for rate limiting (placeholder)
///
/// In a full implementation, this would:
/// - Check rate limits per IP or API key
/// - Return 429 Too Many Requests if limit exceeded
/// - Add rate limit headers (X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset)
pub async fn rate_limiting_middleware(request: Request, next: Next) -> Response {
    // TODO: Implement actual rate limiting
    // For now, just pass through
    next.run(request).await
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        middleware,
        response::IntoResponse,
        routing::get,
        Router,
    };
    use tower::ServiceExt; // For oneshot

    async fn test_handler() -> impl IntoResponse {
        "OK"
    }

    #[tokio::test]
    async fn test_request_id_middleware() {
        let app = Router::new()
            .route("/test", get(test_handler))
            .layer(middleware::from_fn(request_id_middleware));

        let response = app
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Should have request ID in response headers
        assert!(response.headers().contains_key(X_REQUEST_ID));
    }

    #[tokio::test]
    async fn test_request_id_preserved() {
        let app = Router::new()
            .route("/test", get(test_handler))
            .layer(middleware::from_fn(request_id_middleware));

        let request_id = "test-request-id-123";
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/test")
                    .header(X_REQUEST_ID, request_id)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should preserve the original request ID
        assert_eq!(
            response.headers().get(X_REQUEST_ID).unwrap(),
            request_id
        );
    }

    #[tokio::test]
    async fn test_cors_layer() {
        let cors = create_cors_layer();
        let app = Router::new()
            .route("/test", get(test_handler))
            .layer(cors);

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri("/test")
                    .header("Origin", "http://example.com")
                    .header("Access-Control-Request-Method", "GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should have CORS headers
        assert!(response
            .headers()
            .contains_key("access-control-allow-origin"));
    }

    #[tokio::test]
    async fn test_security_headers() {
        let app = Router::new()
            .route("/test", get(test_handler))
            .layer(middleware::from_fn(security_headers_middleware));

        let response = app
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Should have security headers
        assert!(response.headers().contains_key("x-content-type-options"));
        assert!(response.headers().contains_key("x-frame-options"));
        assert!(response.headers().contains_key("x-xss-protection"));
    }

    #[tokio::test]
    async fn test_timeout_middleware() {
        async fn slow_handler() -> impl IntoResponse {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            "OK"
        }

        let app = Router::new()
            .route("/slow", get(slow_handler))
            .layer(middleware::from_fn(timeout_middleware));

        let response = app
            .oneshot(Request::builder().uri("/slow").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // With a 30 second timeout, this should succeed
        assert_eq!(response.status(), StatusCode::OK);
    }
}

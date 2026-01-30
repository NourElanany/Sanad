use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use shared::{ApiResponse, AppConfig, SanadError};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user ID)
    pub username: String,
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
}

/// Authentication middleware
pub async fn auth_middleware(
    State(config): State<AppConfig>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, SanadError> {
    // Extract token from Authorization header
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| SanadError::Authentication("Missing or invalid authorization header".to_string()))?;

    // Validate JWT token
    let claims = validate_token(token, &config.security.jwt_secret)?;

    // Add user info to request extensions
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// Validate JWT token and extract claims
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, SanadError> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|e| SanadError::Authentication(format!("Invalid token: {}", e)))
}

/// Optional authentication middleware (doesn't fail if no token provided)
pub async fn optional_auth_middleware(
    State(config): State<AppConfig>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Response {
    // Try to extract and validate token, but don't fail if missing
    if let Some(token) = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
    {
        if let Ok(claims) = validate_token(token, &config.security.jwt_secret) {
            req.extensions_mut().insert(claims);
        }
    }

    next.run(req).await
}

/// Extract user claims from request extensions
pub fn extract_user_claims(req: &Request) -> Option<&Claims> {
    req.extensions().get::<Claims>()
}

/// Check if user has required permissions (placeholder for future implementation)
pub fn check_permissions(_claims: &Claims, _required_permission: &str) -> bool {
    // TODO: Implement proper permission checking
    true
}
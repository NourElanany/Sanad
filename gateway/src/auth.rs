use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use shared::{ApiResponse, AppConfig, SanadError, SanadResult};
use crate::middleware::auth::Claims;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use chrono::{Utc, Duration};

/// Login request structure
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Registration request structure
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Authentication response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

/// User information
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
}

/// Refresh token request
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Login handler
pub async fn login(
    State(config): State<AppConfig>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, SanadError> {
    // TODO: Implement actual user authentication against database
    // For now, this is a placeholder implementation
    
    // Validate credentials (placeholder)
    if request.username.is_empty() || request.password.is_empty() {
        return Err(SanadError::Validation("Username and password are required".to_string()));
    }

    // Mock user validation - replace with actual database lookup
    if request.username != "admin" || request.password != "password" {
        return Err(SanadError::Authentication("Invalid credentials".to_string()));
    }

    // Generate tokens
    let user_id = "user_123"; // This would come from database
    let access_token = generate_access_token(user_id, &request.username, &config)?;
    let refresh_token = generate_refresh_token(user_id, &config)?;

    let response = AuthResponse {
        access_token,
        refresh_token,
        expires_in: config.security.jwt_expiration_hours as i64 * 3600,
        user: UserInfo {
            id: user_id.to_string(),
            username: request.username,
            email: "admin@example.com".to_string(), // This would come from database
        },
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Registration handler
pub async fn register(
    State(config): State<AppConfig>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, SanadError> {
    // TODO: Implement actual user registration
    
    // Validate input
    if request.username.is_empty() || request.email.is_empty() || request.password.is_empty() {
        return Err(SanadError::Validation("All fields are required".to_string()));
    }

    if !shared::utils::is_valid_email(&request.email) {
        return Err(SanadError::Validation("Invalid email format".to_string()));
    }

    // TODO: Check if user already exists
    // TODO: Hash password
    // TODO: Save user to database

    // For now, return success with mock data
    let user_id = "new_user_123";
    let access_token = generate_access_token(user_id, &request.username, &config)?;
    let refresh_token = generate_refresh_token(user_id, &config)?;

    let response = AuthResponse {
        access_token,
        refresh_token,
        expires_in: config.security.jwt_expiration_hours as i64 * 3600,
        user: UserInfo {
            id: user_id.to_string(),
            username: request.username,
            email: request.email,
        },
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Refresh token handler
pub async fn refresh_token(
    State(_config): State<AppConfig>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, SanadError> {
    // TODO: Implement refresh token validation and new token generation
    // For now, this is a placeholder
    
    if request.refresh_token.is_empty() {
        return Err(SanadError::Validation("Refresh token is required".to_string()));
    }

    // TODO: Validate refresh token
    // TODO: Generate new access token
    
    Err(SanadError::Internal("Refresh token not implemented yet".to_string()))
}

/// Logout handler
pub async fn logout() -> Result<Json<ApiResponse<String>>, SanadError> {
    // TODO: Implement token blacklisting or session invalidation
    Ok(Json(ApiResponse::success("Logged out successfully".to_string())))
}

/// Generate access token
fn generate_access_token(user_id: &str, username: &str, config: &AppConfig) -> SanadResult<String> {
    let now = Utc::now();
    let expiration = now + Duration::hours(config.security.jwt_expiration_hours as i64);

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiration.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let encoding_key = EncodingKey::from_secret(config.security.jwt_secret.as_ref());
    let header = Header::new(Algorithm::HS256);

    encode(&header, &claims, &encoding_key)
        .map_err(|e| SanadError::Internal(format!("Failed to generate token: {}", e)))
}

/// Generate refresh token
fn generate_refresh_token(user_id: &str, config: &AppConfig) -> SanadResult<String> {
    let now = Utc::now();
    let expiration = now + Duration::days(30); // Refresh tokens last 30 days

    let claims = Claims {
        sub: user_id.to_string(),
        username: "refresh".to_string(), // Special marker for refresh tokens
        exp: expiration.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let encoding_key = EncodingKey::from_secret(config.security.jwt_secret.as_ref());
    let header = Header::new(Algorithm::HS256);

    encode(&header, &claims, &encoding_key)
        .map_err(|e| SanadError::Internal(format!("Failed to generate refresh token: {}", e)))
}
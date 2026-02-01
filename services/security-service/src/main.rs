use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use shared::{DigitalAuthenticator, ContentSignature, ContentType, VerificationResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, warn, error};
use uuid::Uuid;

mod models;
mod repository;
mod service;
mod backup_system;

use models::*;
use repository::SecurityRepository;
use service::SecurityService;
use backup_system::BackupSystem;

#[derive(Clone)]
pub struct AppState {
    pub security_service: Arc<SecurityService>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::init();

    // Initialize database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost/sanad_db".to_string());
    
    let pool = sqlx::PgPool::connect(&database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Initialize services
    let repository = SecurityRepository::new(pool);
    let security_service = Arc::new(SecurityService::new(repository).await?);

    let app_state = AppState {
        security_service,
    };

    // Build the router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/sign-content", post(sign_content))
        .route("/verify-content", post(verify_content))
        .route("/batch-verify", post(batch_verify_content))
        .route("/content/:id/signature", get(get_content_signature))
        .route("/reference-database", post(generate_reference_database))
        .route("/backup", post(create_backup))
        .route("/backup/:backup_id/verify", get(verify_backup))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    info!("Security service listening on {}", listener.local_addr()?);
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> &'static str {
    "Security Service is healthy"
}

async fn sign_content(
    State(state): State<AppState>,
    Json(request): Json<SignContentRequest>,
) -> Result<Json<SignContentResponse>, StatusCode> {
    match state.security_service.sign_content(request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            error!("Failed to sign content: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn verify_content(
    State(state): State<AppState>,
    Json(request): Json<VerifyContentRequest>,
) -> Result<Json<VerificationResult>, StatusCode> {
    match state.security_service.verify_content(request).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Failed to verify content: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn batch_verify_content(
    State(state): State<AppState>,
    Json(request): Json<BatchVerifyRequest>,
) -> Result<Json<BatchVerifyResponse>, StatusCode> {
    match state.security_service.batch_verify_content(request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            error!("Failed to batch verify content: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_content_signature(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ContentSignature>, StatusCode> {
    match state.security_service.get_content_signature(id).await {
        Ok(Some(signature)) => Ok(Json(signature)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to get content signature: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn generate_reference_database(
    State(state): State<AppState>,
    Json(request): Json<GenerateReferenceDbRequest>,
) -> Result<Json<GenerateReferenceDbResponse>, StatusCode> {
    match state.security_service.generate_reference_database(request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            error!("Failed to generate reference database: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_backup(
    State(state): State<AppState>,
    Json(request): Json<CreateBackupRequest>,
) -> Result<Json<CreateBackupResponse>, StatusCode> {
    match state.security_service.create_encrypted_backup(request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            error!("Failed to create backup: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn verify_backup(
    State(state): State<AppState>,
    Path(backup_id): Path<Uuid>,
) -> Result<Json<BackupVerificationResult>, StatusCode> {
    match state.security_service.verify_backup_integrity(backup_id).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Failed to verify backup: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
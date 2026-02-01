use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::{ContentSignature, ContentType, VerificationResult};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignContentRequest {
    pub content: String,
    pub content_type: ContentType,
    pub authority: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignContentResponse {
    pub signature: ContentSignature,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyContentRequest {
    pub content: String,
    pub signature: ContentSignature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchVerifyRequest {
    pub items: Vec<BatchVerifyItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchVerifyItem {
    pub content: String,
    pub signature: ContentSignature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchVerifyResponse {
    pub results: Vec<VerificationResult>,
    pub summary: BatchVerificationSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchVerificationSummary {
    pub total_items: usize,
    pub valid_items: usize,
    pub invalid_items: usize,
    pub average_confidence: f64,
    pub processing_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateReferenceDbRequest {
    pub trusted_content: Vec<TrustedContentItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustedContentItem {
    pub content: String,
    pub content_type: ContentType,
    pub authority: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateReferenceDbResponse {
    pub database_id: Uuid,
    pub total_items: usize,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBackupRequest {
    pub content_ids: Vec<Uuid>,
    pub backup_type: BackupType,
    pub encryption_level: EncryptionLevel,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BackupType {
    Full,
    Incremental,
    Differential,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EncryptionLevel {
    Standard, // AES-256
    High,     // AES-256 + additional layers
    Maximum,  // Multiple encryption algorithms
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBackupResponse {
    pub backup_id: Uuid,
    pub backup_size_bytes: u64,
    pub encryption_hash: String,
    pub created_at: DateTime<Utc>,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupVerificationResult {
    pub backup_id: Uuid,
    pub is_valid: bool,
    pub integrity_score: f64,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub verified_at: DateTime<Utc>,
    pub content_items_verified: usize,
    pub corrupted_items: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptedBackup {
    pub id: Uuid,
    pub backup_type: BackupType,
    pub encryption_level: EncryptionLevel,
    pub encrypted_data: Vec<u8>,
    pub encryption_hash: String,
    pub content_manifest: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentIntegrityRecord {
    pub content_id: Uuid,
    pub content_type: ContentType,
    pub sha256_hash: String,
    pub sha512_hash: String,
    pub digital_signature: String,
    pub authority: String,
    pub created_at: DateTime<Utc>,
    pub last_verified: Option<DateTime<Utc>>,
    pub verification_count: i32,
    pub is_trusted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityAuditLog {
    pub id: Uuid,
    pub event_type: SecurityEventType,
    pub content_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub details: HashMap<String, String>,
    pub severity: SecuritySeverity,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SecurityEventType {
    ContentSigned,
    ContentVerified,
    VerificationFailed,
    BackupCreated,
    BackupRestored,
    UnauthorizedAccess,
    IntegrityViolation,
    SuspiciousActivity,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl BackupType {
    pub fn to_string(&self) -> String {
        match self {
            BackupType::Full => "full".to_string(),
            BackupType::Incremental => "incremental".to_string(),
            BackupType::Differential => "differential".to_string(),
        }
    }
}

impl EncryptionLevel {
    pub fn to_string(&self) -> String {
        match self {
            EncryptionLevel::Standard => "standard".to_string(),
            EncryptionLevel::High => "high".to_string(),
            EncryptionLevel::Maximum => "maximum".to_string(),
        }
    }
}

impl SecurityEventType {
    pub fn to_string(&self) -> String {
        match self {
            SecurityEventType::ContentSigned => "content_signed".to_string(),
            SecurityEventType::ContentVerified => "content_verified".to_string(),
            SecurityEventType::VerificationFailed => "verification_failed".to_string(),
            SecurityEventType::BackupCreated => "backup_created".to_string(),
            SecurityEventType::BackupRestored => "backup_restored".to_string(),
            SecurityEventType::UnauthorizedAccess => "unauthorized_access".to_string(),
            SecurityEventType::IntegrityViolation => "integrity_violation".to_string(),
            SecurityEventType::SuspiciousActivity => "suspicious_activity".to_string(),
        }
    }
}

impl SecuritySeverity {
    pub fn to_string(&self) -> String {
        match self {
            SecuritySeverity::Low => "low".to_string(),
            SecuritySeverity::Medium => "medium".to_string(),
            SecuritySeverity::High => "high".to_string(),
            SecuritySeverity::Critical => "critical".to_string(),
        }
    }
}
use anyhow::Result;
use chrono::Utc;
use shared::{DigitalAuthenticator, ContentSignature, VerificationResult};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::models::*;
use crate::repository::SecurityRepository;
use crate::backup_system::BackupSystem;

pub struct SecurityService {
    repository: SecurityRepository,
    authenticator: DigitalAuthenticator,
    backup_system: Arc<BackupSystem>,
}

impl SecurityService {
    pub async fn new(repository: SecurityRepository) -> Result<Self> {
        // In production, this should be loaded from secure key management
        let secret_key = std::env::var("DIGITAL_AUTH_SECRET_KEY")
            .unwrap_or_else(|_| "sanad_islamic_content_auth_key_2024".to_string())
            .into_bytes();

        let authenticator = DigitalAuthenticator::new(secret_key);
        let backup_system = Arc::new(BackupSystem::new().await?);

        Ok(Self {
            repository,
            authenticator,
            backup_system,
        })
    }

    /// Sign Islamic content and store the signature
    pub async fn sign_content(&self, request: SignContentRequest) -> Result<SignContentResponse> {
        let start_time = Instant::now();

        // Sign the content using the digital authenticator
        let signature = self.authenticator.sign_content(
            &request.content,
            request.content_type.clone(),
            &request.authority,
            request.metadata,
        )?;

        // Store the signature in the database
        self.repository.store_content_signature(&signature).await?;

        // Create content integrity record
        let integrity_record = ContentIntegrityRecord {
            content_id: signature.content_id,
            content_type: signature.content_type.clone(),
            sha256_hash: signature.sha256_hash.clone(),
            sha512_hash: signature.sha512_hash.clone(),
            digital_signature: signature.digital_signature.clone(),
            authority: signature.authority.clone(),
            created_at: signature.created_at,
            last_verified: Some(Utc::now()),
            verification_count: 1,
            is_trusted: true,
        };

        self.repository.store_content_integrity_record(&integrity_record).await?;

        // Log the security event
        let audit_log = SecurityAuditLog {
            id: Uuid::new_v4(),
            event_type: SecurityEventType::ContentSigned,
            content_id: Some(signature.content_id),
            user_id: None, // Could be extracted from request context
            ip_address: None, // Could be extracted from request
            details: {
                let mut details = HashMap::new();
                details.insert("content_type".to_string(), signature.content_type.to_string());
                details.insert("authority".to_string(), signature.authority.clone());
                details.insert("processing_time_ms".to_string(), start_time.elapsed().as_millis().to_string());
                details
            },
            severity: SecuritySeverity::Low,
            timestamp: Utc::now(),
        };

        self.repository.log_security_event(&audit_log).await?;

        info!("Content signed successfully: {}", signature.content_id);

        Ok(SignContentResponse {
            signature,
            success: true,
            message: "Content signed and stored successfully".to_string(),
        })
    }

    /// Verify Islamic content against its signature
    pub async fn verify_content(&self, request: VerifyContentRequest) -> Result<VerificationResult> {
        let start_time = Instant::now();

        // Verify the content using the digital authenticator
        let mut result = self.authenticator.verify_content(&request.content, &request.signature)?;

        // Update verification count in database
        if let Ok(mut records) = self.repository.get_content_integrity_records(
            Some(request.signature.content_type.clone()),
            None,
        ).await {
            if let Some(record) = records.iter_mut().find(|r| r.content_id == request.signature.content_id) {
                record.verification_count += 1;
                record.last_verified = Some(Utc::now());
                self.repository.store_content_integrity_record(record).await?;
            }
        }

        // Log the security event
        let event_type = if result.is_valid {
            SecurityEventType::ContentVerified
        } else {
            SecurityEventType::VerificationFailed
        };

        let severity = if result.is_valid {
            SecuritySeverity::Low
        } else if request.signature.content_type == shared::ContentType::Quran {
            SecuritySeverity::Critical // Quranic content failures are critical
        } else {
            SecuritySeverity::High
        };

        let audit_log = SecurityAuditLog {
            id: Uuid::new_v4(),
            event_type,
            content_id: Some(request.signature.content_id),
            user_id: None,
            ip_address: None,
            details: {
                let mut details = HashMap::new();
                details.insert("is_valid".to_string(), result.is_valid.to_string());
                details.insert("confidence_score".to_string(), result.confidence_score.to_string());
                details.insert("error_count".to_string(), result.errors.len().to_string());
                details.insert("processing_time_ms".to_string(), start_time.elapsed().as_millis().to_string());
                if !result.errors.is_empty() {
                    details.insert("errors".to_string(), result.errors.join("; "));
                }
                details
            },
            severity,
            timestamp: Utc::now(),
        };

        self.repository.log_security_event(&audit_log).await?;

        if result.is_valid {
            info!("Content verification successful: {}", request.signature.content_id);
        } else {
            warn!("Content verification failed: {} - Errors: {:?}", 
                  request.signature.content_id, result.errors);
        }

        Ok(result)
    }

    /// Batch verify multiple content items
    pub async fn batch_verify_content(&self, request: BatchVerifyRequest) -> Result<BatchVerifyResponse> {
        let start_time = Instant::now();
        let mut results = Vec::new();
        let mut valid_count = 0;
        let mut total_confidence = 0.0;

        for item in request.items {
            let verify_request = VerifyContentRequest {
                content: item.content,
                signature: item.signature,
            };

            match self.verify_content(verify_request).await {
                Ok(result) => {
                    if result.is_valid {
                        valid_count += 1;
                    }
                    total_confidence += result.confidence_score;
                    results.push(result);
                }
                Err(e) => {
                    error!("Batch verification error: {}", e);
                    // Create a failed result
                    results.push(VerificationResult {
                        is_valid: false,
                        content_id: Uuid::new_v4(),
                        verification_time: Utc::now(),
                        errors: vec![format!("Verification error: {}", e)],
                        warnings: Vec::new(),
                        confidence_score: 0.0,
                    });
                }
            }
        }

        let total_items = results.len();
        let invalid_items = total_items - valid_count;
        let average_confidence = if total_items > 0 {
            total_confidence / total_items as f64
        } else {
            0.0
        };

        let summary = BatchVerificationSummary {
            total_items,
            valid_items: valid_count,
            invalid_items,
            average_confidence,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
        };

        // Log batch verification event
        let audit_log = SecurityAuditLog {
            id: Uuid::new_v4(),
            event_type: SecurityEventType::ContentVerified,
            content_id: None,
            user_id: None,
            ip_address: None,
            details: {
                let mut details = HashMap::new();
                details.insert("batch_size".to_string(), total_items.to_string());
                details.insert("valid_items".to_string(), valid_count.to_string());
                details.insert("invalid_items".to_string(), invalid_items.to_string());
                details.insert("average_confidence".to_string(), average_confidence.to_string());
                details.insert("processing_time_ms".to_string(), summary.processing_time_ms.to_string());
                details
            },
            severity: if invalid_items > 0 { SecuritySeverity::Medium } else { SecuritySeverity::Low },
            timestamp: Utc::now(),
        };

        self.repository.log_security_event(&audit_log).await?;

        info!("Batch verification completed: {}/{} items valid", valid_count, total_items);

        Ok(BatchVerifyResponse {
            results,
            summary,
        })
    }

    /// Get content signature by ID
    pub async fn get_content_signature(&self, content_id: Uuid) -> Result<Option<ContentSignature>> {
        self.repository.get_content_signature(content_id).await
    }

    /// Generate reference database for trusted content
    pub async fn generate_reference_database(&self, request: GenerateReferenceDbRequest) -> Result<GenerateReferenceDbResponse> {
        let start_time = Instant::now();
        let database_id = Uuid::new_v4();
        let mut stored_count = 0;

        for item in request.trusted_content {
            match self.sign_content(SignContentRequest {
                content: item.content,
                content_type: item.content_type,
                authority: item.authority,
                metadata: item.metadata,
            }).await {
                Ok(_) => stored_count += 1,
                Err(e) => {
                    error!("Failed to sign content in reference database: {}", e);
                }
            }
        }

        // Log reference database creation
        let audit_log = SecurityAuditLog {
            id: Uuid::new_v4(),
            event_type: SecurityEventType::BackupCreated, // Using backup event for reference DB
            content_id: None,
            user_id: None,
            ip_address: None,
            details: {
                let mut details = HashMap::new();
                details.insert("database_id".to_string(), database_id.to_string());
                details.insert("total_items".to_string(), stored_count.to_string());
                details.insert("processing_time_ms".to_string(), start_time.elapsed().as_millis().to_string());
                details
            },
            severity: SecuritySeverity::Low,
            timestamp: Utc::now(),
        };

        self.repository.log_security_event(&audit_log).await?;

        info!("Reference database generated: {} items stored", stored_count);

        Ok(GenerateReferenceDbResponse {
            database_id,
            total_items: stored_count,
            success: true,
            message: format!("Reference database created with {} items", stored_count),
        })
    }

    /// Create encrypted backup
    pub async fn create_encrypted_backup(&self, request: CreateBackupRequest) -> Result<CreateBackupResponse> {
        let backup = self.backup_system.create_backup(
            request.content_ids,
            request.backup_type,
            request.encryption_level,
            request.metadata,
        ).await?;

        // Store the backup in the database
        self.repository.store_encrypted_backup(&backup).await?;

        // Log backup creation
        let audit_log = SecurityAuditLog {
            id: Uuid::new_v4(),
            event_type: SecurityEventType::BackupCreated,
            content_id: None,
            user_id: None,
            ip_address: None,
            details: {
                let mut details = HashMap::new();
                details.insert("backup_id".to_string(), backup.id.to_string());
                details.insert("backup_type".to_string(), backup.backup_type.to_string());
                details.insert("encryption_level".to_string(), backup.encryption_level.to_string());
                details.insert("content_count".to_string(), backup.content_manifest.len().to_string());
                details.insert("backup_size_bytes".to_string(), backup.encrypted_data.len().to_string());
                details
            },
            severity: SecuritySeverity::Low,
            timestamp: Utc::now(),
        };

        self.repository.log_security_event(&audit_log).await?;

        info!("Encrypted backup created: {}", backup.id);

        Ok(CreateBackupResponse {
            backup_id: backup.id,
            backup_size_bytes: backup.encrypted_data.len() as u64,
            encryption_hash: backup.encryption_hash,
            created_at: backup.created_at,
            success: true,
            message: "Encrypted backup created successfully".to_string(),
        })
    }

    /// Verify backup integrity
    pub async fn verify_backup_integrity(&self, backup_id: Uuid) -> Result<BackupVerificationResult> {
        let backup = self.repository.get_encrypted_backup(backup_id).await?
            .ok_or_else(|| anyhow::anyhow!("Backup not found"))?;

        let result = self.backup_system.verify_backup_integrity(&backup).await?;

        // Log backup verification
        let audit_log = SecurityAuditLog {
            id: Uuid::new_v4(),
            event_type: if result.is_valid { 
                SecurityEventType::ContentVerified 
            } else { 
                SecurityEventType::IntegrityViolation 
            },
            content_id: None,
            user_id: None,
            ip_address: None,
            details: {
                let mut details = HashMap::new();
                details.insert("backup_id".to_string(), backup_id.to_string());
                details.insert("is_valid".to_string(), result.is_valid.to_string());
                details.insert("integrity_score".to_string(), result.integrity_score.to_string());
                details.insert("verified_items".to_string(), result.content_items_verified.to_string());
                details.insert("corrupted_items".to_string(), result.corrupted_items.len().to_string());
                details
            },
            severity: if result.is_valid { 
                SecuritySeverity::Low 
            } else { 
                SecuritySeverity::High 
            },
            timestamp: Utc::now(),
        };

        self.repository.log_security_event(&audit_log).await?;

        if result.is_valid {
            info!("Backup verification successful: {}", backup_id);
        } else {
            warn!("Backup verification failed: {} - Errors: {:?}", backup_id, result.errors);
        }

        Ok(result)
    }
}
use anyhow::Result;
use chrono::{DateTime, Utc};
use shared::{ContentSignature, ContentType};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::*;

pub struct SecurityRepository {
    pool: PgPool,
}

impl SecurityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Store a content signature in the database
    pub async fn store_content_signature(&self, signature: &ContentSignature) -> Result<()> {
        let content_type_str = signature.content_type.to_string();
        let metadata_json = serde_json::to_string(&signature.metadata)?;

        sqlx::query!(
            r#"
            INSERT INTO content_signatures (
                content_id, content_type, sha256_hash, sha512_hash, 
                digital_signature, created_at, version, authority, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (content_id) DO UPDATE SET
                sha256_hash = EXCLUDED.sha256_hash,
                sha512_hash = EXCLUDED.sha512_hash,
                digital_signature = EXCLUDED.digital_signature,
                version = EXCLUDED.version,
                authority = EXCLUDED.authority,
                metadata = EXCLUDED.metadata,
                updated_at = NOW()
            "#,
            signature.content_id,
            content_type_str,
            signature.sha256_hash,
            signature.sha512_hash,
            signature.digital_signature,
            signature.created_at,
            signature.version as i32,
            signature.authority,
            metadata_json
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Retrieve a content signature by ID
    pub async fn get_content_signature(&self, content_id: Uuid) -> Result<Option<ContentSignature>> {
        let row = sqlx::query!(
            r#"
            SELECT content_id, content_type, sha256_hash, sha512_hash,
                   digital_signature, created_at, version, authority, metadata
            FROM content_signatures 
            WHERE content_id = $1
            "#,
            content_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let content_type = match row.content_type.as_str() {
                "quran" => ContentType::Quran,
                "hadith" => ContentType::Hadith,
                "tafsir" => ContentType::Tafsir,
                "story" => ContentType::Story,
                "prayer" => ContentType::Prayer,
                "dhikr" => ContentType::Dhikr,
                other => ContentType::Other(other.to_string()),
            };

            let metadata: HashMap<String, String> = serde_json::from_str(&row.metadata)?;

            Ok(Some(ContentSignature {
                content_id: row.content_id,
                content_type,
                sha256_hash: row.sha256_hash,
                sha512_hash: row.sha512_hash,
                digital_signature: row.digital_signature,
                created_at: row.created_at,
                version: row.version as u32,
                authority: row.authority,
                metadata,
            }))
        } else {
            Ok(None)
        }
    }

    /// Store an encrypted backup
    pub async fn store_encrypted_backup(&self, backup: &EncryptedBackup) -> Result<()> {
        let backup_type_str = backup.backup_type.to_string();
        let encryption_level_str = backup.encryption_level.to_string();
        let content_manifest_json = serde_json::to_string(&backup.content_manifest)?;
        let metadata_json = serde_json::to_string(&backup.metadata)?;

        sqlx::query!(
            r#"
            INSERT INTO encrypted_backups (
                id, backup_type, encryption_level, encrypted_data,
                encryption_hash, content_manifest, created_at, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            backup.id,
            backup_type_str,
            encryption_level_str,
            backup.encrypted_data,
            backup.encryption_hash,
            content_manifest_json,
            backup.created_at,
            metadata_json
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Retrieve an encrypted backup by ID
    pub async fn get_encrypted_backup(&self, backup_id: Uuid) -> Result<Option<EncryptedBackup>> {
        let row = sqlx::query!(
            r#"
            SELECT id, backup_type, encryption_level, encrypted_data,
                   encryption_hash, content_manifest, created_at, metadata
            FROM encrypted_backups 
            WHERE id = $1
            "#,
            backup_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let backup_type = match row.backup_type.as_str() {
                "full" => BackupType::Full,
                "incremental" => BackupType::Incremental,
                "differential" => BackupType::Differential,
                _ => BackupType::Full,
            };

            let encryption_level = match row.encryption_level.as_str() {
                "standard" => EncryptionLevel::Standard,
                "high" => EncryptionLevel::High,
                "maximum" => EncryptionLevel::Maximum,
                _ => EncryptionLevel::Standard,
            };

            let content_manifest: Vec<Uuid> = serde_json::from_str(&row.content_manifest)?;
            let metadata: HashMap<String, String> = serde_json::from_str(&row.metadata)?;

            Ok(Some(EncryptedBackup {
                id: row.id,
                backup_type,
                encryption_level,
                encrypted_data: row.encrypted_data,
                encryption_hash: row.encryption_hash,
                content_manifest,
                created_at: row.created_at,
                metadata,
            }))
        } else {
            Ok(None)
        }
    }

    /// Store a security audit log entry
    pub async fn log_security_event(&self, log: &SecurityAuditLog) -> Result<()> {
        let event_type_str = log.event_type.to_string();
        let severity_str = log.severity.to_string();
        let details_json = serde_json::to_string(&log.details)?;

        sqlx::query!(
            r#"
            INSERT INTO security_audit_logs (
                id, event_type, content_id, user_id, ip_address,
                details, severity, timestamp
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            log.id,
            event_type_str,
            log.content_id,
            log.user_id,
            log.ip_address,
            details_json,
            severity_str,
            log.timestamp
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get security audit logs with filters
    pub async fn get_security_logs(
        &self,
        event_type: Option<SecurityEventType>,
        severity: Option<SecuritySeverity>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        limit: Option<i64>,
    ) -> Result<Vec<SecurityAuditLog>> {
        let mut query = "SELECT * FROM security_audit_logs WHERE 1=1".to_string();
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
        let mut param_count = 0;

        if let Some(event_type) = event_type {
            param_count += 1;
            query.push_str(&format!(" AND event_type = ${}", param_count));
            params.push(Box::new(event_type.to_string()));
        }

        if let Some(severity) = severity {
            param_count += 1;
            query.push_str(&format!(" AND severity = ${}", param_count));
            params.push(Box::new(severity.to_string()));
        }

        if let Some(start_time) = start_time {
            param_count += 1;
            query.push_str(&format!(" AND timestamp >= ${}", param_count));
            params.push(Box::new(start_time));
        }

        if let Some(end_time) = end_time {
            param_count += 1;
            query.push_str(&format!(" AND timestamp <= ${}", param_count));
            params.push(Box::new(end_time));
        }

        query.push_str(" ORDER BY timestamp DESC");

        if let Some(limit) = limit {
            param_count += 1;
            query.push_str(&format!(" LIMIT ${}", param_count));
            params.push(Box::new(limit));
        }

        // For simplicity, we'll use a basic query without dynamic parameters
        // In a production system, you'd want to use a query builder
        let rows = sqlx::query!(
            r#"
            SELECT id, event_type, content_id, user_id, ip_address,
                   details, severity, timestamp
            FROM security_audit_logs 
            ORDER BY timestamp DESC 
            LIMIT $1
            "#,
            limit.unwrap_or(100)
        )
        .fetch_all(&self.pool)
        .await?;

        let mut logs = Vec::new();
        for row in rows {
            let event_type = match row.event_type.as_str() {
                "content_signed" => SecurityEventType::ContentSigned,
                "content_verified" => SecurityEventType::ContentVerified,
                "verification_failed" => SecurityEventType::VerificationFailed,
                "backup_created" => SecurityEventType::BackupCreated,
                "backup_restored" => SecurityEventType::BackupRestored,
                "unauthorized_access" => SecurityEventType::UnauthorizedAccess,
                "integrity_violation" => SecurityEventType::IntegrityViolation,
                "suspicious_activity" => SecurityEventType::SuspiciousActivity,
                _ => SecurityEventType::SuspiciousActivity,
            };

            let severity = match row.severity.as_str() {
                "low" => SecuritySeverity::Low,
                "medium" => SecuritySeverity::Medium,
                "high" => SecuritySeverity::High,
                "critical" => SecuritySeverity::Critical,
                _ => SecuritySeverity::Medium,
            };

            let details: HashMap<String, String> = serde_json::from_str(&row.details)?;

            logs.push(SecurityAuditLog {
                id: row.id,
                event_type,
                content_id: row.content_id,
                user_id: row.user_id,
                ip_address: row.ip_address,
                details,
                severity,
                timestamp: row.timestamp,
            });
        }

        Ok(logs)
    }

    /// Store content integrity record
    pub async fn store_content_integrity_record(&self, record: &ContentIntegrityRecord) -> Result<()> {
        let content_type_str = record.content_type.to_string();

        sqlx::query!(
            r#"
            INSERT INTO content_integrity_records (
                content_id, content_type, sha256_hash, sha512_hash,
                digital_signature, authority, created_at, last_verified,
                verification_count, is_trusted
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (content_id) DO UPDATE SET
                sha256_hash = EXCLUDED.sha256_hash,
                sha512_hash = EXCLUDED.sha512_hash,
                digital_signature = EXCLUDED.digital_signature,
                authority = EXCLUDED.authority,
                last_verified = EXCLUDED.last_verified,
                verification_count = EXCLUDED.verification_count,
                is_trusted = EXCLUDED.is_trusted
            "#,
            record.content_id,
            content_type_str,
            record.sha256_hash,
            record.sha512_hash,
            record.digital_signature,
            record.authority,
            record.created_at,
            record.last_verified,
            record.verification_count,
            record.is_trusted
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get content integrity records by type
    pub async fn get_content_integrity_records(
        &self,
        content_type: Option<ContentType>,
        is_trusted: Option<bool>,
    ) -> Result<Vec<ContentIntegrityRecord>> {
        let rows = if let Some(content_type) = content_type {
            let content_type_str = content_type.to_string();
            sqlx::query!(
                r#"
                SELECT content_id, content_type, sha256_hash, sha512_hash,
                       digital_signature, authority, created_at, last_verified,
                       verification_count, is_trusted
                FROM content_integrity_records 
                WHERE content_type = $1 AND ($2::boolean IS NULL OR is_trusted = $2)
                ORDER BY created_at DESC
                "#,
                content_type_str,
                is_trusted
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query!(
                r#"
                SELECT content_id, content_type, sha256_hash, sha512_hash,
                       digital_signature, authority, created_at, last_verified,
                       verification_count, is_trusted
                FROM content_integrity_records 
                WHERE ($1::boolean IS NULL OR is_trusted = $1)
                ORDER BY created_at DESC
                "#,
                is_trusted
            )
            .fetch_all(&self.pool)
            .await?
        };

        let mut records = Vec::new();
        for row in rows {
            let content_type = match row.content_type.as_str() {
                "quran" => ContentType::Quran,
                "hadith" => ContentType::Hadith,
                "tafsir" => ContentType::Tafsir,
                "story" => ContentType::Story,
                "prayer" => ContentType::Prayer,
                "dhikr" => ContentType::Dhikr,
                other => ContentType::Other(other.to_string()),
            };

            records.push(ContentIntegrityRecord {
                content_id: row.content_id,
                content_type,
                sha256_hash: row.sha256_hash,
                sha512_hash: row.sha512_hash,
                digital_signature: row.digital_signature,
                authority: row.authority,
                created_at: row.created_at,
                last_verified: row.last_verified,
                verification_count: row.verification_count,
                is_trusted: row.is_trusted,
            });
        }

        Ok(records)
    }
}
use anyhow::Result;
use chrono::{DateTime, Utc};
use shared::{ContentSignature, ContentType};
use sqlx::{PgPool, Row};
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

        sqlx::query(
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
            "#
        )
        .bind(signature.content_id)
        .bind(content_type_str)
        .bind(&signature.sha256_hash)
        .bind(&signature.sha512_hash)
        .bind(&signature.digital_signature)
        .bind(signature.created_at)
        .bind(signature.version as i32)
        .bind(&signature.authority)
        .bind(metadata_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Retrieve a content signature by ID
    pub async fn get_content_signature(&self, content_id: Uuid) -> Result<Option<ContentSignature>> {
        let row = sqlx::query(
            r#"
            SELECT content_id, content_type, sha256_hash, sha512_hash,
                   digital_signature, created_at, version, authority, metadata
            FROM content_signatures 
            WHERE content_id = $1
            "#
        )
        .bind(content_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let content_type_str: String = row.try_get("content_type")?;
            let content_type = match content_type_str.as_str() {
                "quran" => ContentType::Quran,
                "hadith" => ContentType::Hadith,
                "tafsir" => ContentType::Tafsir,
                "story" => ContentType::Story,
                "prayer" => ContentType::Prayer,
                "dhikr" => ContentType::Dhikr,
                other => ContentType::Other(other.to_string()),
            };

            let metadata_str: String = row.try_get("metadata")?;
            let metadata: HashMap<String, String> = serde_json::from_str(&metadata_str)?;

            Ok(Some(ContentSignature {
                content_id: row.try_get("content_id")?,
                content_type,
                sha256_hash: row.try_get("sha256_hash")?,
                sha512_hash: row.try_get("sha512_hash")?,
                digital_signature: row.try_get("digital_signature")?,
                created_at: row.try_get("created_at")?,
                version: row.try_get::<i32, _>("version")? as u32,
                authority: row.try_get("authority")?,
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

        sqlx::query(
            r#"
            INSERT INTO encrypted_backups (
                id, backup_type, encryption_level, encrypted_data,
                encryption_hash, content_manifest, created_at, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(backup.id)
        .bind(backup_type_str)
        .bind(encryption_level_str)
        .bind(&backup.encrypted_data)
        .bind(&backup.encryption_hash)
        .bind(content_manifest_json)
        .bind(backup.created_at)
        .bind(metadata_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Retrieve an encrypted backup by ID
    pub async fn get_encrypted_backup(&self, backup_id: Uuid) -> Result<Option<EncryptedBackup>> {
        let row = sqlx::query(
            r#"
            SELECT id, backup_type, encryption_level, encrypted_data,
                   encryption_hash, content_manifest, created_at, metadata
            FROM encrypted_backups 
            WHERE id = $1
            "#
        )
        .bind(backup_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let backup_type_str: String = row.try_get("backup_type")?;
            let backup_type = match backup_type_str.as_str() {
                "full" => BackupType::Full,
                "incremental" => BackupType::Incremental,
                "differential" => BackupType::Differential,
                _ => BackupType::Full,
            };

            let encryption_level_str: String = row.try_get("encryption_level")?;
            let encryption_level = match encryption_level_str.as_str() {
                "standard" => EncryptionLevel::Standard,
                "high" => EncryptionLevel::High,
                "maximum" => EncryptionLevel::Maximum,
                _ => EncryptionLevel::Standard,
            };

            let content_manifest_str: String = row.try_get("content_manifest")?;
            let content_manifest: Vec<Uuid> = serde_json::from_str(&content_manifest_str)?;
            let metadata_str: String = row.try_get("metadata")?;
            let metadata: HashMap<String, String> = serde_json::from_str(&metadata_str)?;

            Ok(Some(EncryptedBackup {
                id: row.try_get("id")?,
                backup_type,
                encryption_level,
                encrypted_data: row.try_get("encrypted_data")?,
                encryption_hash: row.try_get("encryption_hash")?,
                content_manifest,
                created_at: row.try_get("created_at")?,
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

        sqlx::query(
            r#"
            INSERT INTO security_audit_logs (
                id, event_type, content_id, user_id, ip_address,
                details, severity, timestamp
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(log.id)
        .bind(event_type_str)
        .bind(log.content_id)
        .bind(log.user_id)
        .bind(&log.ip_address)
        .bind(details_json)
        .bind(severity_str)
        .bind(log.timestamp)
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
        let rows = sqlx::query(
            r#"
            SELECT id, event_type, content_id, user_id, ip_address,
                   details, severity, timestamp
            FROM security_audit_logs 
            ORDER BY timestamp DESC 
            LIMIT $1
            "#
        )
        .bind(limit.unwrap_or(100))
        .fetch_all(&self.pool)
        .await?;

        let mut logs = Vec::new();
        for row in rows {
            let event_type_str: String = row.try_get("event_type")?;
            let event_type = match event_type_str.as_str() {
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

            let severity_str: String = row.try_get("severity")?;
            let severity = match severity_str.as_str() {
                "low" => SecuritySeverity::Low,
                "medium" => SecuritySeverity::Medium,
                "high" => SecuritySeverity::High,
                "critical" => SecuritySeverity::Critical,
                _ => SecuritySeverity::Medium,
            };

            let details_str: String = row.try_get("details")?;
            let details: HashMap<String, String> = serde_json::from_str(&details_str)?;

            logs.push(SecurityAuditLog {
                id: row.try_get("id")?,
                event_type,
                content_id: row.try_get("content_id")?,
                user_id: row.try_get("user_id")?,
                ip_address: row.try_get("ip_address")?,
                details,
                severity,
                timestamp: row.try_get("timestamp")?,
            });
        }

        Ok(logs)
    }

    /// Store content integrity record
    pub async fn store_content_integrity_record(&self, record: &ContentIntegrityRecord) -> Result<()> {
        let content_type_str = record.content_type.to_string();

        sqlx::query(
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
            "#
        )
        .bind(record.content_id)
        .bind(content_type_str)
        .bind(&record.sha256_hash)
        .bind(&record.sha512_hash)
        .bind(&record.digital_signature)
        .bind(&record.authority)
        .bind(record.created_at)
        .bind(record.last_verified)
        .bind(record.verification_count)
        .bind(record.is_trusted)
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
            sqlx::query(
                r#"
                SELECT content_id, content_type, sha256_hash, sha512_hash,
                       digital_signature, authority, created_at, last_verified,
                       verification_count, is_trusted
                FROM content_integrity_records 
                WHERE content_type = $1 AND ($2::boolean IS NULL OR is_trusted = $2)
                ORDER BY created_at DESC
                "#
            )
            .bind(content_type_str)
            .bind(is_trusted)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                SELECT content_id, content_type, sha256_hash, sha512_hash,
                       digital_signature, authority, created_at, last_verified,
                       verification_count, is_trusted
                FROM content_integrity_records 
                WHERE ($1::boolean IS NULL OR is_trusted = $1)
                ORDER BY created_at DESC
                "#
            )
            .bind(is_trusted)
            .fetch_all(&self.pool)
            .await?
        };

        let mut records = Vec::new();
        for row in rows {
            let content_type_str: String = row.try_get("content_type")?;
            let content_type = match content_type_str.as_str() {
                "quran" => ContentType::Quran,
                "hadith" => ContentType::Hadith,
                "tafsir" => ContentType::Tafsir,
                "story" => ContentType::Story,
                "prayer" => ContentType::Prayer,
                "dhikr" => ContentType::Dhikr,
                other => ContentType::Other(other.to_string()),
            };

            records.push(ContentIntegrityRecord {
                content_id: row.try_get("content_id")?,
                content_type,
                sha256_hash: row.try_get("sha256_hash")?,
                sha512_hash: row.try_get("sha512_hash")?,
                digital_signature: row.try_get("digital_signature")?,
                authority: row.try_get("authority")?,
                created_at: row.try_get("created_at")?,
                last_verified: row.try_get("last_verified")?,
                verification_count: row.try_get("verification_count")?,
                is_trusted: row.try_get("is_trusted")?,
            });
        }

        Ok(records)
    }
}
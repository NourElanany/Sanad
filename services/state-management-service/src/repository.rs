use crate::models::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Repository for persisting state management data
pub struct StateRepository {
    pool: PgPool,
}

impl StateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Save user personal data
    pub async fn save_user_data(&self, user_data: &UserPersonalData) -> Result<()> {
        let serialized_data = serde_json::to_value(user_data)?;
        
        sqlx::query!(
            r#"
            INSERT INTO user_personal_data (user_id, data, last_updated)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id) 
            DO UPDATE SET 
                data = EXCLUDED.data,
                last_updated = EXCLUDED.last_updated
            "#,
            user_data.user_id,
            serialized_data,
            user_data.last_updated
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Load user personal data
    pub async fn load_user_data(&self, user_id: Uuid) -> Result<Option<UserPersonalData>> {
        let row = sqlx::query!(
            "SELECT data, last_updated FROM user_personal_data WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let user_data: UserPersonalData = serde_json::from_value(row.data)?;
            Ok(Some(user_data))
        } else {
            Ok(None)
        }
    }

    /// Save sync operation
    pub async fn save_sync_operation(&self, operation: &crate::sync::SyncOperation) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO sync_operations (id, operation_type, data, priority, created_at, retry_count)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            operation.id,
            serde_json::to_string(&operation.operation_type)?,
            operation.data,
            serde_json::to_string(&operation.priority)?,
            operation.created_at,
            operation.retry_count as i32
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Load pending sync operations
    pub async fn load_pending_sync_operations(&self) -> Result<Vec<crate::sync::SyncOperation>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, operation_type, data, priority, created_at, retry_count
            FROM sync_operations
            WHERE retry_count < 3
            ORDER BY priority DESC, created_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut operations = Vec::new();
        for row in rows {
            let operation = crate::sync::SyncOperation {
                id: row.id,
                operation_type: serde_json::from_str(&row.operation_type)?,
                data: row.data,
                priority: serde_json::from_str(&row.priority)?,
                created_at: row.created_at,
                retry_count: row.retry_count as u32,
            };
            operations.push(operation);
        }

        Ok(operations)
    }

    /// Delete completed sync operation
    pub async fn delete_sync_operation(&self, operation_id: Uuid) -> Result<()> {
        sqlx::query!(
            "DELETE FROM sync_operations WHERE id = $1",
            operation_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Save content metadata
    pub async fn save_content_metadata(&self, metadata: &crate::storage::ContentMetadata) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO content_metadata 
            (id, content_type, size_bytes, created_at, last_accessed, access_count, priority, compressed, checksum)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id)
            DO UPDATE SET
                last_accessed = EXCLUDED.last_accessed,
                access_count = EXCLUDED.access_count
            "#,
            metadata.id,
            metadata.content_type,
            metadata.size_bytes as i64,
            metadata.created_at,
            metadata.last_accessed,
            metadata.access_count as i32,
            serde_json::to_string(&metadata.priority)?,
            metadata.compressed,
            metadata.checksum
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Load content metadata by priority
    pub async fn load_content_by_priority(
        &self,
        priority: &crate::storage::StoragePriority,
    ) -> Result<Vec<crate::storage::ContentMetadata>> {
        let priority_str = serde_json::to_string(priority)?;
        
        let rows = sqlx::query!(
            r#"
            SELECT id, content_type, size_bytes, created_at, last_accessed, access_count, priority, compressed, checksum
            FROM content_metadata
            WHERE priority = $1
            ORDER BY last_accessed ASC
            "#,
            priority_str
        )
        .fetch_all(&self.pool)
        .await?;

        let mut metadata_list = Vec::new();
        for row in rows {
            let metadata = crate::storage::ContentMetadata {
                id: row.id,
                content_type: row.content_type,
                size_bytes: row.size_bytes as u64,
                created_at: row.created_at,
                last_accessed: row.last_accessed,
                access_count: row.access_count as u32,
                priority: serde_json::from_str(&row.priority)?,
                compressed: row.compressed,
                checksum: row.checksum,
            };
            metadata_list.push(metadata);
        }

        Ok(metadata_list)
    }

    /// Delete content metadata
    pub async fn delete_content_metadata(&self, content_id: Uuid) -> Result<()> {
        sqlx::query!(
            "DELETE FROM content_metadata WHERE id = $1",
            content_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Update content access statistics
    pub async fn update_content_access(&self, content_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE content_metadata 
            SET last_accessed = NOW(), access_count = access_count + 1
            WHERE id = $1
            "#,
            content_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get storage statistics
    pub async fn get_storage_statistics(&self) -> Result<StorageStats> {
        let row = sqlx::query!(
            r#"
            SELECT 
                COALESCE(SUM(size_bytes), 0) as total_bytes,
                COUNT(*) as items_count
            FROM content_metadata
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        let total_size_mb = row.total_bytes.unwrap_or(0) as f64 / (1024.0 * 1024.0);
        
        Ok(StorageStats {
            total_size_mb,
            available_space_mb: 500.0 - total_size_mb, // Assuming 500MB limit
            items_count: row.items_count.unwrap_or(0) as u32,
            last_cleanup: Utc::now(), // Would be stored in database
            compression_ratio: 1.2, // Would be calculated from actual data
        })
    }
}
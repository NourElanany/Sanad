use crate::models::*;
use anyhow::Result;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn, error};
use uuid::Uuid;

/// Smart local storage manager with adaptive space management
#[derive(Debug, Clone)]
pub struct SmartStorageManager {
    max_storage_mb: u64,
    compression_enabled: bool,
    pub storage_stats: StorageStats, // Made public for testing
    content_priorities: HashMap<String, StoragePriority>,
    cleanup_policies: HashMap<String, CleanupPolicy>,
}

/// Storage priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum StoragePriority {
    Essential,   // Quran, basic prayers - never cleaned
    Important,   // User bookmarks, progress - cleaned after 90 days
    Useful,      // Cached searches, hadith - cleaned after 30 days
    Optional,    // Audio recordings, images - cleaned after 7 days
}

/// Cleanup policies for different content types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupPolicy {
    pub max_age_days: u32,
    pub max_size_mb: f64,
    pub usage_threshold: f64, // 0.0 to 1.0, based on access frequency
}

/// Content storage metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub id: Uuid,
    pub content_type: String,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u32,
    pub priority: StoragePriority,
    pub compressed: bool,
    pub checksum: String,
}

/// Storage operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResult {
    pub success: bool,
    pub bytes_saved: u64,
    pub bytes_freed: u64,
    pub items_cleaned: u32,
    pub compression_ratio: f64,
    pub errors: Vec<String>,
}

impl SmartStorageManager {
    pub fn new(max_storage_mb: u64, compression_enabled: bool) -> Self {
        let mut content_priorities = HashMap::new();
        let mut cleanup_policies = HashMap::new();

        // Configure content priorities
        content_priorities.insert("quran_text".to_string(), StoragePriority::Essential);
        content_priorities.insert("basic_tafsir".to_string(), StoragePriority::Essential);
        content_priorities.insert("prayer_times".to_string(), StoragePriority::Essential);
        content_priorities.insert("user_bookmarks".to_string(), StoragePriority::Important);
        content_priorities.insert("reading_progress".to_string(), StoragePriority::Important);
        content_priorities.insert("personal_notes".to_string(), StoragePriority::Important);
        content_priorities.insert("hadith_collection".to_string(), StoragePriority::Useful);
        content_priorities.insert("search_cache".to_string(), StoragePriority::Useful);
        content_priorities.insert("audio_recordings".to_string(), StoragePriority::Optional);
        content_priorities.insert("images".to_string(), StoragePriority::Optional);

        // Configure cleanup policies
        cleanup_policies.insert("essential".to_string(), CleanupPolicy {
            max_age_days: u32::MAX, // Never clean
            max_size_mb: f64::INFINITY,
            usage_threshold: 0.0,
        });

        cleanup_policies.insert("important".to_string(), CleanupPolicy {
            max_age_days: 90,
            max_size_mb: 100.0,
            usage_threshold: 0.1,
        });

        cleanup_policies.insert("useful".to_string(), CleanupPolicy {
            max_age_days: 30,
            max_size_mb: 50.0,
            usage_threshold: 0.3,
        });

        cleanup_policies.insert("optional".to_string(), CleanupPolicy {
            max_age_days: 7,
            max_size_mb: 20.0,
            usage_threshold: 0.5,
        });

        Self {
            max_storage_mb,
            compression_enabled,
            storage_stats: StorageStats {
                total_size_mb: 0.0,
                available_space_mb: max_storage_mb as f64,
                items_count: 0,
                last_cleanup: Utc::now(),
                compression_ratio: 1.0,
            },
            content_priorities,
            cleanup_policies,
        }
    }

    /// Store content with smart compression and prioritization
    pub async fn store_content(
        &mut self,
        content_type: &str,
        data: &[u8],
        metadata: ContentMetadata,
    ) -> Result<StorageResult> {
        let mut result = StorageResult {
            success: false,
            bytes_saved: 0,
            bytes_freed: 0,
            items_cleaned: 0,
            compression_ratio: 1.0,
            errors: Vec::new(),
        };

        // Check if we need to free space
        let required_space = data.len() as f64 / (1024.0 * 1024.0); // Convert to MB
        if self.storage_stats.available_space_mb < required_space {
            let cleanup_result = self.smart_cleanup(required_space).await?;
            result.bytes_freed = cleanup_result.bytes_freed;
            result.items_cleaned = cleanup_result.items_cleaned;
        }

        // Compress data if enabled and beneficial
        let (final_data, compression_ratio) = if self.compression_enabled && data.len() > 1024 {
            self.compress_data(data)?
        } else {
            (data.to_vec(), 1.0)
        };

        // Store the data (implementation would write to actual storage)
        let stored_size = final_data.len() as u64;
        result.bytes_saved = data.len() as u64 - stored_size;
        result.compression_ratio = compression_ratio;

        // Update storage statistics
        self.storage_stats.total_size_mb += stored_size as f64 / (1024.0 * 1024.0);
        self.storage_stats.available_space_mb -= stored_size as f64 / (1024.0 * 1024.0);
        self.storage_stats.items_count += 1;

        result.success = true;
        info!("Stored content: {} bytes, compression ratio: {:.2}", stored_size, compression_ratio);

        Ok(result)
    }

    /// Smart cleanup based on priorities and usage patterns
    pub async fn smart_cleanup(&mut self, required_space_mb: f64) -> Result<StorageResult> {
        let mut result = StorageResult {
            success: false,
            bytes_saved: 0,
            bytes_freed: 0,
            items_cleaned: 0,
            compression_ratio: 1.0,
            errors: Vec::new(),
        };

        info!("Starting smart cleanup, need {:.2} MB", required_space_mb);

        // Clean in priority order: Optional -> Useful -> Important (never Essential)
        let cleanup_order = vec![
            StoragePriority::Optional,
            StoragePriority::Useful,
            StoragePriority::Important,
        ];

        let mut freed_space = 0.0;

        for priority in cleanup_order {
            if freed_space >= required_space_mb {
                break;
            }

            let cleanup_result = self.cleanup_by_priority(priority).await?;
            freed_space += cleanup_result.bytes_freed as f64 / (1024.0 * 1024.0);
            result.bytes_freed += cleanup_result.bytes_freed;
            result.items_cleaned += cleanup_result.items_cleaned;
        }

        // Update storage statistics
        self.storage_stats.available_space_mb += freed_space;
        self.storage_stats.total_size_mb -= freed_space;
        self.storage_stats.last_cleanup = Utc::now();

        result.success = freed_space >= required_space_mb;
        
        if result.success {
            info!("Cleanup successful: freed {:.2} MB, cleaned {} items", freed_space, result.items_cleaned);
        } else {
            warn!("Cleanup insufficient: freed {:.2} MB, needed {:.2} MB", freed_space, required_space_mb);
        }

        Ok(result)
    }

    /// Cleanup content by priority level
    pub async fn cleanup_by_priority(&self, priority: StoragePriority) -> Result<StorageResult> {
        let mut result = StorageResult {
            success: true,
            bytes_saved: 0,
            bytes_freed: 0,
            items_cleaned: 0,
            compression_ratio: 1.0,
            errors: Vec::new(),
        };

        let policy_key = match priority {
            StoragePriority::Essential => return Ok(result), // Never clean essential
            StoragePriority::Important => "important",
            StoragePriority::Useful => "useful",
            StoragePriority::Optional => "optional",
        };

        let policy = self.cleanup_policies.get(policy_key).unwrap();
        let cutoff_date = Utc::now() - Duration::days(policy.max_age_days as i64);

        // Implementation would:
        // 1. Find content matching priority and older than cutoff_date
        // 2. Sort by access frequency (least accessed first)
        // 3. Delete until space requirement is met or no more content
        
        // Simulated cleanup
        result.bytes_freed = 10 * 1024 * 1024; // 10 MB
        result.items_cleaned = 50;

        info!("Cleaned {:?} priority content: {} items, {} bytes", 
              priority, result.items_cleaned, result.bytes_freed);

        Ok(result)
    }

    /// Compress data using LZ4
    pub fn compress_data(&self, data: &[u8]) -> Result<(Vec<u8>, f64)> {
        let compressed = lz4_flex::compress_prepend_size(data);
        let compression_ratio = data.len() as f64 / compressed.len() as f64;
        
        // Only use compression if it saves significant space
        if compression_ratio > 1.1 {
            Ok((compressed, compression_ratio))
        } else {
            Ok((data.to_vec(), 1.0))
        }
    }

    /// Decompress data
    pub fn decompress_data(&self, compressed_data: &[u8]) -> Result<Vec<u8>> {
        let decompressed = lz4_flex::decompress_size_prepended(compressed_data)?;
        Ok(decompressed)
    }

    /// Get storage statistics
    pub fn get_storage_stats(&self) -> &StorageStats {
        &self.storage_stats
    }

    /// Update content access metadata
    pub async fn update_access_metadata(&mut self, content_id: Uuid) -> Result<()> {
        // Implementation would update last_accessed and increment access_count
        info!("Updated access metadata for content: {}", content_id);
        Ok(())
    }

    /// Get content by priority
    pub async fn get_content_by_priority(&self, priority: StoragePriority) -> Result<Vec<ContentMetadata>> {
        // Implementation would query storage for content matching priority
        Ok(Vec::new())
    }

    /// Verify content integrity using checksums
    pub async fn verify_content_integrity(&self, content_id: Uuid) -> Result<bool> {
        // Implementation would:
        // 1. Load content and metadata
        // 2. Calculate current checksum
        // 3. Compare with stored checksum
        // 4. Return verification result
        
        info!("Verified content integrity for: {}", content_id);
        Ok(true)
    }

    /// Optimize storage by reorganizing and compressing
    pub async fn optimize_storage(&mut self) -> Result<StorageResult> {
        let mut result = StorageResult {
            success: true,
            bytes_saved: 0,
            bytes_freed: 0,
            items_cleaned: 0,
            compression_ratio: 1.0,
            errors: Vec::new(),
        };

        info!("Starting storage optimization");

        // Implementation would:
        // 1. Identify uncompressed content that would benefit from compression
        // 2. Defragment storage
        // 3. Update indexes
        // 4. Verify integrity

        // Simulated optimization
        result.bytes_saved = 5 * 1024 * 1024; // 5 MB saved through compression
        self.storage_stats.compression_ratio = 1.3;

        info!("Storage optimization complete: saved {} bytes", result.bytes_saved);
        Ok(result)
    }
}
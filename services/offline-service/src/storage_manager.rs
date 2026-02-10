use crate::models::*;
use anyhow::{Result, Context};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{info, warn, error, debug};
use uuid::Uuid;
use sha2::{Sha256, Digest};

/// Advanced offline storage manager with intelligent space management
pub struct OfflineStorageManager {
    config: OfflineConfig,
    storage_path: PathBuf,
    content_index: HashMap<String, OfflineContent>,
    stats: OfflineStats,
}

impl OfflineStorageManager {
    /// Create a new offline storage manager
    pub async fn new(storage_path: PathBuf, config: OfflineConfig) -> Result<Self> {
        // Ensure storage directory exists
        fs::create_dir_all(&storage_path).await
            .context("Failed to create storage directory")?;

        let mut manager = Self {
            config,
            storage_path,
            content_index: HashMap::new(),
            stats: OfflineStats::default(),
        };

        // Load existing content index
        manager.load_content_index().await?;
        
        // Update statistics
        manager.update_statistics().await?;

        info!("Offline storage manager initialized at: {:?}", manager.storage_path);
        info!("Total items: {}, Total size: {:.2} MB", 
              manager.stats.total_items, manager.stats.total_size_mb);

        Ok(manager)
    }

    /// Store content offline with compression and metadata
    pub async fn store_content(
        &mut self,
        content_type: OfflineContentType,
        content_id: String,
        data: Vec<u8>,
        metadata: ContentMetadata,
    ) -> Result<Uuid> {
        // Check if we have enough space
        self.ensure_storage_space(data.len()).await?;

        // Compress data if enabled
        let (compressed_data, compression_info) = if self.config.enable_compression {
            self.compress_data(&data, &self.config.default_compression)?
        } else {
            (data, (CompressionAlgorithm::None, 1.0))
        };

        // Calculate content hash for integrity
        let content_hash = self.calculate_hash(&compressed_data);

        // Create storage info
        let storage_info = StorageInfo {
            priority: self.config.content_priorities
                .get(&content_type)
                .cloned()
                .unwrap_or_else(|| content_type.default_priority()),
            access_count: 0,
            last_accessed: Utc::now(),
            expires_at: self.calculate_expiry(&content_type),
            storage_path: None, // Will be set after writing to disk
            size_on_disk: compressed_data.len(),
            is_compressed: compression_info.0 != CompressionAlgorithm::None,
            compression_algorithm: compression_info.0,
        };

        // Create sync info
        let sync_info = SyncInfo {
            last_synced: None,
            sync_status: SyncStatus::Synced,
            sync_strategy: content_type.default_sync_strategy(),
            conflict_resolution: ConflictResolution::LastWriteWins,
            pending_changes: Vec::new(),
            sync_attempts: 0,
            last_sync_error: None,
        };

        // Create offline content entry
        let id = Uuid::new_v4();
        let mut offline_content = OfflineContent {
            id,
            content_type: content_type.clone(),
            content_id: content_id.clone(),
            data: compressed_data.clone(),
            metadata: ContentMetadata {
                content_hash,
                compressed_size: compressed_data.len(),
                compression_ratio: compression_info.1,
                ..metadata
            },
            storage_info,
            sync_info,
        };

        // Write to disk
        let file_path = self.get_content_file_path(&id);
        fs::write(&file_path, &compressed_data).await
            .context("Failed to write content to disk")?;

        // Update storage path
        offline_content.storage_info.storage_path = Some(file_path.to_string_lossy().to_string());

        // Add to index
        self.content_index.insert(content_id.clone(), offline_content);

        // Update statistics
        self.update_statistics().await?;

        // Save index
        self.save_content_index().await?;

        info!("Stored offline content: {} (type: {:?}, size: {} bytes, compressed: {:.2}x)",
              content_id, content_type, compressed_data.len(), compression_info.1);

        Ok(id)
    }

    /// Retrieve content from offline storage
    pub async fn get_content(&mut self, content_id: &str) -> Result<Option<OfflineResult<Vec<u8>>>> {
        // First, extract all needed data from the content
        let (data, compression_algorithm, expected_hash, is_compressed, updated_at, sync_status) = {
            if let Some(content) = self.content_index.get_mut(content_id) {
                // Update access statistics
                content.storage_info.access_count += 1;
                content.storage_info.last_accessed = Utc::now();

                // Check if content has expired
                if let Some(expires_at) = content.storage_info.expires_at {
                    if Utc::now() > expires_at {
                        warn!("Content expired: {}", content_id);
                        self.remove_content(content_id).await?;
                        return Ok(None);
                    }
                }

                // Read from disk if not in memory
                let data = if content.data.is_empty() {
                    if let Some(storage_path) = &content.storage_info.storage_path {
                        fs::read(storage_path).await
                            .context("Failed to read content from disk")?
                    } else {
                        return Ok(None);
                    }
                } else {
                    content.data.clone()
                };

                // Extract needed data before releasing the borrow
                (
                    data,
                    content.storage_info.compression_algorithm.clone(),
                    content.metadata.content_hash.clone(),
                    content.storage_info.is_compressed,
                    content.metadata.updated_at,
                    content.sync_info.sync_status.clone(),
                )
            } else {
                return Ok(None);
            }
        };

        // Now we can use self methods without borrowing issues
        // Decompress if needed
        let final_data = if is_compressed {
            self.decompress_data(&data, &compression_algorithm)?
        } else {
            data.clone()
        };

        // Verify integrity
        let calculated_hash = self.calculate_hash(&data);
        if calculated_hash != expected_hash {
            error!("Content integrity check failed for: {}", content_id);
            return Ok(Some(OfflineResult {
                success: false,
                data: None,
                error: Some("Content integrity check failed".to_string()),
                from_cache: true,
                sync_pending: false,
                last_updated: Some(updated_at),
            }));
        }

        debug!("Retrieved offline content: {} ({} bytes)", content_id, final_data.len());

        Ok(Some(OfflineResult {
            success: true,
            data: Some(final_data),
            error: None,
            from_cache: true,
            sync_pending: sync_status != SyncStatus::Synced,
            last_updated: Some(updated_at),
        }))
    }

    /// Remove content from offline storage
    pub async fn remove_content(&mut self, content_id: &str) -> Result<bool> {
        if let Some(content) = self.content_index.remove(content_id) {
            // Remove file from disk
            if let Some(storage_path) = &content.storage_info.storage_path {
                if let Err(e) = fs::remove_file(storage_path).await {
                    warn!("Failed to remove file {}: {}", storage_path, e);
                }
            }

            // Update statistics
            self.update_statistics().await?;

            // Save index
            self.save_content_index().await?;

            info!("Removed offline content: {}", content_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// List all offline content with optional filtering
    pub fn list_content(&self, content_type: Option<OfflineContentType>) -> Vec<&OfflineContent> {
        self.content_index
            .values()
            .filter(|content| {
                content_type.as_ref().map_or(true, |ct| &content.content_type == ct)
            })
            .collect()
    }

    /// Get storage statistics
    pub fn get_statistics(&self) -> &OfflineStats {
        &self.stats
    }

    /// Perform intelligent cleanup to free space
    pub async fn cleanup_storage(&mut self, target_free_space_mb: Option<f64>) -> Result<u64> {
        let target_space = target_free_space_mb.unwrap_or(self.config.min_free_space_mb as f64);
        let current_free_space = self.get_available_space_mb().await?;

        if current_free_space >= target_space {
            debug!("No cleanup needed. Free space: {:.2} MB", current_free_space);
            return Ok(0);
        }

        let space_to_free = target_space - current_free_space;
        info!("Starting cleanup to free {:.2} MB", space_to_free);

        let mut freed_space = 0.0;
        let mut removed_count = 0u64;

        // Get cleanup candidates sorted by priority and access time
        let candidates: Vec<_> = self.content_index
            .iter()
            .filter(|(_, content)| {
                // Don't cleanup essential content
                content.storage_info.priority != StoragePriority::Essential &&
                !content.storage_info.priority.should_keep_during_cleanup(content.storage_info.last_accessed)
            })
            .map(|(id, content)| (id.clone(), content.storage_info.size_on_disk, content.storage_info.priority.clone()))
            .collect();

        // Sort by priority (lowest first) and size (largest first for same priority)
        let mut sorted_candidates = candidates;
        sorted_candidates.sort_by(|(_, size_a, priority_a), (_, size_b, priority_b)| {
            priority_a.cmp(priority_a).then(size_b.cmp(size_a))
        });

        // Remove content until we have enough space
        for (content_id, content_size, _) in sorted_candidates {
            if freed_space >= space_to_free {
                break;
            }

            let content_size_mb = content_size as f64 / (1024.0 * 1024.0);
            
            if self.remove_content(&content_id).await? {
                freed_space += content_size_mb;
                removed_count += 1;
                
                info!("Cleaned up content: {} ({:.2} MB)", content_id, content_size_mb);
            }
        }

        // Update cleanup timestamp
        self.stats.last_cleanup = Some(Utc::now());

        info!("Cleanup completed: freed {:.2} MB by removing {} items", freed_space, removed_count);
        Ok(removed_count)
    }

    /// Optimize storage by compressing uncompressed content
    pub async fn optimize_storage(&mut self) -> Result<f64> {
        if !self.config.enable_compression {
            return Ok(0.0);
        }

        let mut space_saved = 0.0;
        let mut optimized_count = 0;

        // Find uncompressed content that could benefit from compression
        let content_ids: Vec<String> = self.content_index
            .iter()
            .filter(|(_, content)| {
                !content.storage_info.is_compressed && 
                content.storage_info.size_on_disk > 1024 // Only compress files > 1KB
            })
            .map(|(id, _)| id.clone())
            .collect();

        for content_id in content_ids {
            if let Some(content) = self.content_index.get(&content_id).cloned() {
                // Read current data
                let current_data = if let Some(storage_path) = &content.storage_info.storage_path {
                    fs::read(storage_path).await?
                } else {
                    content.data.clone()
                };

                // Compress data
                let (compressed_data, compression_info) = self.compress_data(
                    &current_data, 
                    &self.config.default_compression
                )?;

                // Only update if compression is beneficial
                if compressed_data.len() < current_data.len() {
                    let space_saved_bytes = current_data.len() - compressed_data.len();
                    space_saved += space_saved_bytes as f64 / (1024.0 * 1024.0);

                    // Update content
                    if let Some(content) = self.content_index.get_mut(&content_id) {
                        content.data = compressed_data.clone();
                        content.storage_info.is_compressed = true;
                        content.storage_info.compression_algorithm = compression_info.0;
                        content.storage_info.size_on_disk = compressed_data.len();
                        content.metadata.compressed_size = compressed_data.len();
                        content.metadata.compression_ratio = compression_info.1;

                        // Write compressed data to disk
                        if let Some(storage_path) = &content.storage_info.storage_path {
                            fs::write(storage_path, &compressed_data).await?;
                        }

                        optimized_count += 1;
                    }
                }
            }
        }

        // Update statistics
        self.update_statistics().await?;
        self.save_content_index().await?;

        info!("Storage optimization completed: saved {:.2} MB by optimizing {} items", 
              space_saved, optimized_count);

        Ok(space_saved)
    }

    /// Verify integrity of all stored content
    pub async fn verify_integrity(&self) -> Result<Vec<String>> {
        let mut corrupted_content = Vec::new();

        for (content_id, content) in &self.content_index {
            // Read data from disk
            let data = if let Some(storage_path) = &content.storage_info.storage_path {
                match fs::read(storage_path).await {
                    Ok(data) => data,
                    Err(_) => {
                        corrupted_content.push(format!("{}: File not found", content_id));
                        continue;
                    }
                }
            } else {
                content.data.clone()
            };

            // Verify hash
            let calculated_hash = self.calculate_hash(&data);
            if calculated_hash != content.metadata.content_hash {
                corrupted_content.push(format!("{}: Hash mismatch", content_id));
            }
        }

        if corrupted_content.is_empty() {
            info!("Integrity verification passed for all {} items", self.content_index.len());
        } else {
            warn!("Integrity verification found {} corrupted items", corrupted_content.len());
        }

        Ok(corrupted_content)
    }

    /// Get content that needs synchronization
    pub fn get_sync_pending_content(&self) -> Vec<&OfflineContent> {
        self.content_index
            .values()
            .filter(|content| content.sync_info.sync_status != SyncStatus::Synced)
            .collect()
    }

    /// Update sync status for content
    pub async fn update_sync_status(
        &mut self,
        content_id: &str,
        status: SyncStatus,
        error: Option<String>,
    ) -> Result<()> {
        if let Some(content) = self.content_index.get_mut(content_id) {
            let is_synced = status == SyncStatus::Synced;
            content.sync_info.sync_status = status;
            content.sync_info.last_sync_error = error;
            
            if is_synced {
                content.sync_info.last_synced = Some(Utc::now());
                content.sync_info.pending_changes.clear();
            }

            self.save_content_index().await?;
        }

        Ok(())
    }

    // Private helper methods

    async fn load_content_index(&mut self) -> Result<()> {
        let index_path = self.storage_path.join("content_index.json");
        
        if index_path.exists() {
            let index_data = fs::read_to_string(&index_path).await
                .context("Failed to read content index")?;
            
            self.content_index = serde_json::from_str(&index_data)
                .context("Failed to parse content index")?;
            
            debug!("Loaded content index with {} items", self.content_index.len());
        }

        Ok(())
    }

    async fn save_content_index(&self) -> Result<()> {
        let index_path = self.storage_path.join("content_index.json");
        let index_data = serde_json::to_string_pretty(&self.content_index)
            .context("Failed to serialize content index")?;
        
        fs::write(&index_path, index_data).await
            .context("Failed to write content index")?;

        Ok(())
    }

    async fn update_statistics(&mut self) -> Result<()> {
        let mut total_size = 0u64;
        let mut items_by_type = HashMap::new();
        let mut items_by_priority = HashMap::new();
        let mut total_original_size = 0u64;
        let mut total_compressed_size = 0u64;

        for content in self.content_index.values() {
            total_size += content.storage_info.size_on_disk as u64;
            
            *items_by_type.entry(content.content_type.clone()).or_insert(0) += 1;
            *items_by_priority.entry(content.storage_info.priority.clone()).or_insert(0) += 1;
            
            total_original_size += content.metadata.original_size as u64;
            total_compressed_size += content.metadata.compressed_size as u64;
        }

        let available_space = self.get_available_space_mb().await?;
        let total_size_mb = total_size as f64 / (1024.0 * 1024.0);
        let used_percentage = if self.config.max_storage_mb > 0 {
            (total_size_mb / self.config.max_storage_mb as f64) * 100.0
        } else {
            0.0
        };

        let compression_stats = CompressionStats {
            total_original_size_mb: total_original_size as f64 / (1024.0 * 1024.0),
            total_compressed_size_mb: total_compressed_size as f64 / (1024.0 * 1024.0),
            average_compression_ratio: if total_compressed_size > 0 {
                total_original_size as f64 / total_compressed_size as f64
            } else {
                1.0
            },
            space_saved_mb: (total_original_size - total_compressed_size) as f64 / (1024.0 * 1024.0),
            space_saved_percentage: if total_original_size > 0 {
                ((total_original_size - total_compressed_size) as f64 / total_original_size as f64) * 100.0
            } else {
                0.0
            },
        };

        // Calculate sync stats
        let sync_stats = self.calculate_sync_stats();

        self.stats = OfflineStats {
            total_items: self.content_index.len() as u64,
            total_size_mb,
            available_space_mb: available_space,
            used_space_percentage: used_percentage,
            items_by_type,
            items_by_priority,
            compression_stats,
            sync_stats,
            last_cleanup: self.stats.last_cleanup,
            last_sync: self.stats.last_sync,
        };

        Ok(())
    }

    fn calculate_sync_stats(&self) -> SyncStats {
        let mut pending_uploads = 0;
        let mut pending_downloads = 0;
        let mut conflicts = 0;
        let mut failed_syncs = 0;
        let mut successful_syncs = 0;

        for content in self.content_index.values() {
            match content.sync_info.sync_status {
                SyncStatus::PendingUpload => pending_uploads += 1,
                SyncStatus::PendingDownload => pending_downloads += 1,
                SyncStatus::Conflicted => conflicts += 1,
                SyncStatus::Failed => failed_syncs += 1,
                SyncStatus::Synced => successful_syncs += 1,
                _ => {}
            }
        }

        SyncStats {
            pending_uploads,
            pending_downloads,
            conflicts,
            failed_syncs,
            successful_syncs,
            last_sync_duration_ms: 0, // Would be tracked separately
            average_sync_time_ms: 0.0, // Would be calculated from history
            total_data_synced_mb: 0.0, // Would be tracked separately
        }
    }

    async fn get_available_space_mb(&self) -> Result<f64> {
        // This is a simplified implementation
        // In a real implementation, you would check actual disk space
        let used_space = self.content_index.values()
            .map(|c| c.storage_info.size_on_disk as u64)
            .sum::<u64>() as f64 / (1024.0 * 1024.0);
        
        Ok((self.config.max_storage_mb as f64 - used_space).max(0.0))
    }

    async fn ensure_storage_space(&mut self, required_bytes: usize) -> Result<()> {
        let required_mb = required_bytes as f64 / (1024.0 * 1024.0);
        let available_mb = self.get_available_space_mb().await?;

        if available_mb < required_mb + self.config.min_free_space_mb as f64 {
            let space_to_free = required_mb + self.config.min_free_space_mb as f64 - available_mb;
            self.cleanup_storage(Some(space_to_free)).await?;
        }

        Ok(())
    }

    fn get_content_file_path(&self, content_id: &Uuid) -> PathBuf {
        self.storage_path.join(format!("{}.dat", content_id))
    }

    fn calculate_hash(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    fn compress_data(&self, data: &[u8], algorithm: &CompressionAlgorithm) -> Result<(Vec<u8>, (CompressionAlgorithm, f64))> {
        match algorithm {
            CompressionAlgorithm::None => Ok((data.to_vec(), (CompressionAlgorithm::None, 1.0))),
            CompressionAlgorithm::Lz4 => {
                let compressed = lz4_flex::compress_prepend_size(data);
                let ratio = data.len() as f64 / compressed.len() as f64;
                Ok((compressed, (CompressionAlgorithm::Lz4, ratio)))
            }
            CompressionAlgorithm::Gzip => {
                use flate2::Compression;
                use flate2::write::GzEncoder;
                use std::io::Write;

                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(data)?;
                let compressed = encoder.finish()?;
                let ratio = data.len() as f64 / compressed.len() as f64;
                Ok((compressed, (CompressionAlgorithm::Gzip, ratio)))
            }
            CompressionAlgorithm::Brotli => {
                // Simplified - would use actual Brotli compression
                Ok((data.to_vec(), (CompressionAlgorithm::None, 1.0)))
            }
        }
    }

    fn decompress_data(&self, data: &[u8], algorithm: &CompressionAlgorithm) -> Result<Vec<u8>> {
        match algorithm {
            CompressionAlgorithm::None => Ok(data.to_vec()),
            CompressionAlgorithm::Lz4 => {
                let decompressed = lz4_flex::decompress_size_prepended(data)
                    .context("LZ4 decompression failed")?;
                Ok(decompressed)
            }
            CompressionAlgorithm::Gzip => {
                use flate2::read::GzDecoder;
                use std::io::Read;

                let mut decoder = GzDecoder::new(data);
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)?;
                Ok(decompressed)
            }
            CompressionAlgorithm::Brotli => {
                // Simplified - would use actual Brotli decompression
                Ok(data.to_vec())
            }
        }
    }

    fn calculate_expiry(&self, content_type: &OfflineContentType) -> Option<DateTime<Utc>> {
        match content_type {
            OfflineContentType::PrayerTimes => {
                // Prayer times expire after 24 hours
                Some(Utc::now() + Duration::hours(24))
            }
            OfflineContentType::SearchCache => {
                // Search cache expires after 6 hours
                Some(Utc::now() + Duration::hours(6))
            }
            _ => None, // No expiry for other content types
        }
    }
}

impl Default for OfflineStats {
    fn default() -> Self {
        Self {
            total_items: 0,
            total_size_mb: 0.0,
            available_space_mb: 0.0,
            used_space_percentage: 0.0,
            items_by_type: HashMap::new(),
            items_by_priority: HashMap::new(),
            compression_stats: CompressionStats {
                total_original_size_mb: 0.0,
                total_compressed_size_mb: 0.0,
                average_compression_ratio: 1.0,
                space_saved_mb: 0.0,
                space_saved_percentage: 0.0,
            },
            sync_stats: SyncStats {
                pending_uploads: 0,
                pending_downloads: 0,
                conflicts: 0,
                failed_syncs: 0,
                successful_syncs: 0,
                last_sync_duration_ms: 0,
                average_sync_time_ms: 0.0,
                total_data_synced_mb: 0.0,
            },
            last_cleanup: None,
            last_sync: None,
        }
    }
}
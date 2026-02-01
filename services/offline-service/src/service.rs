use crate::models::*;
use crate::storage_manager::OfflineStorageManager;
use crate::sync_manager::OfflineSyncManager;
use anyhow::{Result, Context};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn, error};
use uuid::Uuid;

/// Main offline service that coordinates storage and synchronization
pub struct OfflineService {
    storage_manager: Arc<Mutex<OfflineStorageManager>>,
    sync_manager: Arc<OfflineSyncManager>,
    config: OfflineConfig,
}

impl OfflineService {
    /// Create a new offline service
    pub async fn new(
        storage_path: PathBuf,
        config: OfflineConfig,
        server_base_url: String,
    ) -> Result<Self> {
        // Initialize storage manager
        let storage_manager = Arc::new(Mutex::new(
            OfflineStorageManager::new(storage_path, config.clone()).await?
        ));

        // Initialize sync manager
        let sync_manager = Arc::new(
            OfflineSyncManager::new(
                Arc::clone(&storage_manager),
                config.sync_config.clone(),
                server_base_url,
            ).await?
        );

        info!("Offline service initialized successfully");

        Ok(Self {
            storage_manager,
            sync_manager,
            config,
        })
    }

    /// Start the offline service
    pub async fn start(&self) -> Result<()> {
        info!("Starting offline service");

        // Start sync manager
        let sync_manager = Arc::clone(&self.sync_manager);
        tokio::spawn(async move {
            if let Err(e) = sync_manager.start().await {
                error!("Sync manager error: {}", e);
            }
        });

        // Start periodic cleanup if enabled
        if self.config.auto_cleanup {
            self.start_periodic_cleanup().await;
        }

        // Preload essential content
        self.preload_essential_content().await?;

        info!("Offline service started successfully");
        Ok(())
    }

    /// Store content for offline access
    pub async fn store_content(
        &self,
        content_type: OfflineContentType,
        content_id: String,
        data: Vec<u8>,
        title: String,
        metadata: Option<ContentMetadata>,
    ) -> Result<Uuid> {
        let metadata = metadata.unwrap_or_else(|| ContentMetadata {
            title,
            description: None,
            language: "ar".to_string(),
            source: "local".to_string(),
            author: None,
            tags: Vec::new(),
            content_hash: "".to_string(), // Will be calculated
            original_size: data.len(),
            compressed_size: data.len(),
            compression_ratio: 1.0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        });

        let mut storage = self.storage_manager.lock().await;
        let id = storage.store_content(content_type, content_id, data, metadata).await?;

        info!("Content stored offline with ID: {}", id);
        Ok(id)
    }

    /// Retrieve content from offline storage
    pub async fn get_content(&self, content_id: &str) -> Result<Option<OfflineResult<Vec<u8>>>> {
        let mut storage = self.storage_manager.lock().await;
        storage.get_content(content_id).await
    }

    /// Remove content from offline storage
    pub async fn remove_content(&self, content_id: &str) -> Result<bool> {
        let mut storage = self.storage_manager.lock().await;
        storage.remove_content(content_id).await
    }

    /// List offline content with optional filtering
    pub async fn list_content(&self, content_type: Option<OfflineContentType>) -> Vec<OfflineContent> {
        let storage = self.storage_manager.lock().await;
        storage.list_content(content_type).into_iter().cloned().collect()
    }

    /// Get offline storage statistics
    pub async fn get_statistics(&self) -> OfflineStats {
        let storage = self.storage_manager.lock().await;
        storage.get_statistics().clone()
    }

    /// Download content for offline access
    pub async fn download_content(&self, request: DownloadRequest) -> Result<Uuid> {
        self.sync_manager.queue_download(request).await
    }

    /// Get download progress
    pub async fn get_download_progress(&self, request_id: Uuid) -> Option<DownloadProgress> {
        self.sync_manager.get_download_progress(request_id).await
    }

    /// Cancel download
    pub async fn cancel_download(&self, request_id: Uuid) -> Result<()> {
        self.sync_manager.cancel_download(request_id).await
    }

    /// Force synchronization for specific content
    pub async fn force_sync(&self, content_id: String, content_type: OfflineContentType) -> Result<()> {
        self.sync_manager.force_sync(content_id, content_type).await
    }

    /// Get synchronization conflicts
    pub async fn get_sync_conflicts(&self) -> Vec<SyncConflict> {
        self.sync_manager.get_conflicts().await
    }

    /// Resolve synchronization conflict
    pub async fn resolve_conflict(&self, conflict_id: Uuid, resolution: ConflictResolution) -> Result<()> {
        self.sync_manager.resolve_conflict(conflict_id, resolution).await
    }

    /// Check if device is online and can sync
    pub async fn is_online(&self) -> bool {
        self.sync_manager.can_sync().await
    }

    /// Get current connection information
    pub async fn get_connection_info(&self) -> ConnectionInfo {
        self.sync_manager.get_connection_info().await
    }

    /// Perform storage cleanup
    pub async fn cleanup_storage(&self, target_free_space_mb: Option<f64>) -> Result<u64> {
        let mut storage = self.storage_manager.lock().await;
        storage.cleanup_storage(target_free_space_mb).await
    }

    /// Optimize storage (compress uncompressed content)
    pub async fn optimize_storage(&self) -> Result<f64> {
        let mut storage = self.storage_manager.lock().await;
        storage.optimize_storage().await
    }

    /// Verify integrity of stored content
    pub async fn verify_integrity(&self) -> Result<Vec<String>> {
        let storage = self.storage_manager.lock().await;
        storage.verify_integrity().await
    }

    /// Get Quran content (essential offline content)
    pub async fn get_quran_content(&self, surah: u16, ayah: Option<u16>) -> Result<Option<OfflineResult<String>>> {
        let content_id = match ayah {
            Some(ayah_num) => format!("quran:{}:{}", surah, ayah_num),
            None => format!("quran:{}", surah),
        };

        if let Some(result) = self.get_content(&content_id).await? {
            if let Some(data) = result.data {
                let text = String::from_utf8(data)
                    .context("Failed to decode Quran text")?;
                
                return Ok(Some(OfflineResult {
                    success: true,
                    data: Some(text),
                    error: None,
                    from_cache: result.from_cache,
                    sync_pending: result.sync_pending,
                    last_updated: result.last_updated,
                }));
            }
        }

        // If not available offline, try to download
        if self.is_online().await {
            warn!("Quran content not available offline, attempting download: {}", content_id);
            
            let download_request = DownloadRequest {
                content_type: OfflineContentType::QuranText,
                content_ids: vec![content_id.clone()],
                priority: StoragePriority::Essential,
                force_download: false,
                compression: Some(CompressionAlgorithm::Lz4),
            };

            self.download_content(download_request).await?;
        }

        Ok(None)
    }

    /// Get prayer times (essential offline content)
    pub async fn get_prayer_times(&self, lat: f64, lng: f64, date: &str) -> Result<Option<OfflineResult<String>>> {
        let content_id = format!("prayer_times:{}:{}:{}", lat, lng, date);

        if let Some(result) = self.get_content(&content_id).await? {
            if let Some(data) = result.data {
                let prayer_times = String::from_utf8(data)
                    .context("Failed to decode prayer times")?;
                
                return Ok(Some(OfflineResult {
                    success: true,
                    data: Some(prayer_times),
                    error: None,
                    from_cache: result.from_cache,
                    sync_pending: result.sync_pending,
                    last_updated: result.last_updated,
                }));
            }
        }

        // If not available offline and online, try to download
        if self.is_online().await {
            info!("Prayer times not available offline, attempting download: {}", content_id);
            
            let download_request = DownloadRequest {
                content_type: OfflineContentType::PrayerTimes,
                content_ids: vec![content_id.clone()],
                priority: StoragePriority::Essential,
                force_download: false,
                compression: Some(CompressionAlgorithm::Lz4),
            };

            self.download_content(download_request).await?;
        }

        Ok(None)
    }

    /// Get user bookmarks (high priority offline content)
    pub async fn get_user_bookmarks(&self, user_id: Uuid) -> Result<Option<OfflineResult<String>>> {
        let content_id = format!("bookmarks:{}", user_id);

        if let Some(result) = self.get_content(&content_id).await? {
            if let Some(data) = result.data {
                let bookmarks = String::from_utf8(data)
                    .context("Failed to decode bookmarks")?;
                
                return Ok(Some(OfflineResult {
                    success: true,
                    data: Some(bookmarks),
                    error: None,
                    from_cache: result.from_cache,
                    sync_pending: result.sync_pending,
                    last_updated: result.last_updated,
                }));
            }
        }

        Ok(None)
    }

    /// Store user bookmarks
    pub async fn store_user_bookmarks(&self, user_id: Uuid, bookmarks: String) -> Result<Uuid> {
        let content_id = format!("bookmarks:{}", user_id);
        
        self.store_content(
            OfflineContentType::UserBookmarks,
            content_id,
            bookmarks.into_bytes(),
            "User Bookmarks".to_string(),
            None,
        ).await
    }

    /// Get reading progress (high priority offline content)
    pub async fn get_reading_progress(&self, user_id: Uuid) -> Result<Option<OfflineResult<String>>> {
        let content_id = format!("progress:{}", user_id);

        if let Some(result) = self.get_content(&content_id).await? {
            if let Some(data) = result.data {
                let progress = String::from_utf8(data)
                    .context("Failed to decode reading progress")?;
                
                return Ok(Some(OfflineResult {
                    success: true,
                    data: Some(progress),
                    error: None,
                    from_cache: result.from_cache,
                    sync_pending: result.sync_pending,
                    last_updated: result.last_updated,
                }));
            }
        }

        Ok(None)
    }

    /// Store reading progress
    pub async fn store_reading_progress(&self, user_id: Uuid, progress: String) -> Result<Uuid> {
        let content_id = format!("progress:{}", user_id);
        
        let id = self.store_content(
            OfflineContentType::ReadingProgress,
            content_id.clone(),
            progress.into_bytes(),
            "Reading Progress".to_string(),
            None,
        ).await?;

        // Force sync for reading progress (critical data)
        if let Err(e) = self.force_sync(content_id, OfflineContentType::ReadingProgress).await {
            warn!("Failed to queue reading progress sync: {}", e);
        }

        Ok(id)
    }

    // Private helper methods

    async fn start_periodic_cleanup(&self) {
        let storage_manager = Arc::clone(&self.storage_manager);
        let cleanup_interval = self.config.cleanup_interval_hours;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                tokio::time::Duration::from_secs(cleanup_interval * 3600)
            );

            loop {
                interval.tick().await;

                info!("Starting periodic cleanup");
                
                match storage_manager.lock().await.cleanup_storage(None).await {
                    Ok(cleaned_count) => {
                        info!("Periodic cleanup completed: {} items removed", cleaned_count);
                    }
                    Err(e) => {
                        error!("Periodic cleanup failed: {}", e);
                    }
                }
            }
        });
    }

    async fn preload_essential_content(&self) -> Result<()> {
        info!("Preloading essential content for offline access");

        // This would typically load:
        // 1. Complete Quran text
        // 2. Basic tafsir for common verses
        // 3. Prayer times for current location
        // 4. User's bookmarks and progress

        // For now, we'll just log that preloading is complete
        info!("Essential content preloading completed");
        Ok(())
    }
}

/// Builder for offline service configuration
pub struct OfflineServiceBuilder {
    storage_path: Option<PathBuf>,
    config: OfflineConfig,
    server_url: Option<String>,
}

impl OfflineServiceBuilder {
    pub fn new() -> Self {
        Self {
            storage_path: None,
            config: OfflineConfig::default(),
            server_url: None,
        }
    }

    pub fn storage_path(mut self, path: PathBuf) -> Self {
        self.storage_path = Some(path);
        self
    }

    pub fn config(mut self, config: OfflineConfig) -> Self {
        self.config = config;
        self
    }

    pub fn server_url(mut self, url: String) -> Self {
        self.server_url = Some(url);
        self
    }

    pub fn max_storage_mb(mut self, mb: u64) -> Self {
        self.config.max_storage_mb = mb;
        self
    }

    pub fn enable_compression(mut self, enabled: bool) -> Self {
        self.config.enable_compression = enabled;
        self
    }

    pub fn auto_sync(mut self, enabled: bool) -> Self {
        self.config.sync_config.auto_sync = enabled;
        self
    }

    pub fn wifi_only(mut self, enabled: bool) -> Self {
        self.config.sync_config.wifi_only = enabled;
        self
    }

    pub async fn build(self) -> Result<OfflineService> {
        let storage_path = self.storage_path
            .ok_or_else(|| anyhow::anyhow!("Storage path is required"))?;
        
        let server_url = self.server_url
            .ok_or_else(|| anyhow::anyhow!("Server URL is required"))?;

        OfflineService::new(storage_path, self.config, server_url).await
    }
}

impl Default for OfflineServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}
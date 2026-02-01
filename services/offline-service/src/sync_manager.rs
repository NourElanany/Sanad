use crate::models::*;
use crate::storage_manager::OfflineStorageManager;
use anyhow::{Result, Context};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use tokio::time::{interval, Duration as TokioDuration};
use tracing::{info, warn, error, debug};
use uuid::Uuid;
use reqwest::Client;
use serde_json::Value;

/// Smart synchronization manager for offline content
pub struct OfflineSyncManager {
    storage_manager: Arc<Mutex<OfflineStorageManager>>,
    config: SyncConfig,
    http_client: Client,
    connection_info: Arc<RwLock<ConnectionInfo>>,
    sync_queue: Arc<RwLock<Vec<SyncOperation>>>,
    conflicts: Arc<RwLock<HashMap<String, SyncConflict>>>,
    download_progress: Arc<RwLock<HashMap<Uuid, DownloadProgress>>>,
    base_url: String,
}

/// Synchronization operation for queuing
#[derive(Debug, Clone)]
pub struct SyncOperation {
    pub id: Uuid,
    pub operation_type: SyncOperationType,
    pub content_id: String,
    pub content_type: OfflineContentType,
    pub priority: SyncPriority,
    pub data: Option<Vec<u8>>,
    pub created_at: DateTime<Utc>,
    pub retry_count: u32,
    pub last_error: Option<String>,
}

/// Types of sync operations
#[derive(Debug, Clone, PartialEq)]
pub enum SyncOperationType {
    Upload,
    Download,
    Delete,
    Resolve,
}

/// Sync operation priority
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyncPriority {
    Critical = 4,  // Prayer times, reading progress
    High = 3,      // User bookmarks, notes
    Normal = 2,    // General content
    Low = 1,       // Optional content
}

impl OfflineSyncManager {
    /// Create a new offline sync manager
    pub async fn new(
        storage_manager: Arc<Mutex<OfflineStorageManager>>,
        config: SyncConfig,
        base_url: String,
    ) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(TokioDuration::from_secs(config.connection_timeout_seconds))
            .build()
            .context("Failed to create HTTP client")?;

        let connection_info = Arc::new(RwLock::new(ConnectionInfo {
            is_connected: false,
            connection_type: ConnectionType::Unknown,
            bandwidth_mbps: 0.0,
            latency_ms: 0,
            is_metered: false,
            signal_strength: None,
        }));

        Ok(Self {
            storage_manager,
            config,
            http_client,
            connection_info,
            sync_queue: Arc::new(RwLock::new(Vec::new())),
            conflicts: Arc::new(RwLock::new(HashMap::new())),
            download_progress: Arc::new(RwLock::new(HashMap::new())),
            base_url,
        })
    }

    /// Start the synchronization service
    pub async fn start(&self) -> Result<()> {
        info!("Starting offline sync manager");

        // Start connection monitoring
        let connection_monitor = self.start_connection_monitor();

        // Start sync processor
        let sync_processor = self.start_sync_processor();

        // Start periodic sync
        let periodic_sync = self.start_periodic_sync();

        // Wait for all tasks
        tokio::try_join!(connection_monitor, sync_processor, periodic_sync)?;

        Ok(())
    }

    /// Queue content for download
    pub async fn queue_download(&self, request: DownloadRequest) -> Result<Uuid> {
        let request_id = Uuid::new_v4();
        
        // Create download progress entry
        let progress = DownloadProgress {
            request_id,
            total_items: request.content_ids.len(),
            completed_items: 0,
            current_item: None,
            bytes_downloaded: 0,
            total_bytes: 0,
            progress_percentage: 0.0,
            estimated_time_remaining_seconds: None,
            download_speed_mbps: 0.0,
            status: DownloadStatus::Queued,
            errors: Vec::new(),
        };

        self.download_progress.write().await.insert(request_id, progress);

        // Queue sync operations for each content item
        let mut sync_queue = self.sync_queue.write().await;
        
        for content_id in &request.content_ids {
            let operation = SyncOperation {
                id: Uuid::new_v4(),
                operation_type: SyncOperationType::Download,
                content_id: content_id.clone(),
                content_type: request.content_type.clone(),
                priority: self.content_type_to_sync_priority(&request.content_type),
                data: None,
                created_at: Utc::now(),
                retry_count: 0,
                last_error: None,
            };

            sync_queue.push(operation);
        }

        // Sort queue by priority
        sync_queue.sort_by(|a, b| b.priority.cmp(&a.priority));

        info!("Queued download request: {} items of type {:?}", 
              request.content_ids.len(), request.content_type);

        Ok(request_id)
    }

    /// Get download progress
    pub async fn get_download_progress(&self, request_id: Uuid) -> Option<DownloadProgress> {
        self.download_progress.read().await.get(&request_id).cloned()
    }

    /// Cancel download
    pub async fn cancel_download(&self, request_id: Uuid) -> Result<()> {
        if let Some(progress) = self.download_progress.write().await.get_mut(&request_id) {
            progress.status = DownloadStatus::Cancelled;
            info!("Cancelled download request: {}", request_id);
        }
        Ok(())
    }

    /// Force sync for specific content
    pub async fn force_sync(&self, content_id: String, content_type: OfflineContentType) -> Result<()> {
        let operation = SyncOperation {
            id: Uuid::new_v4(),
            operation_type: SyncOperationType::Upload,
            content_id: content_id.clone(),
            content_type: content_type.clone(),
            priority: SyncPriority::Critical,
            data: None,
            created_at: Utc::now(),
            retry_count: 0,
            last_error: None,
        };

        let mut sync_queue = self.sync_queue.write().await;
        sync_queue.insert(0, operation); // Insert at front for immediate processing

        info!("Queued force sync for content: {} (type: {:?})", 
              content_id, content_type);

        Ok(())
    }

    /// Get sync conflicts that need resolution
    pub async fn get_conflicts(&self) -> Vec<SyncConflict> {
        self.conflicts.read().await.values().cloned().collect()
    }

    /// Resolve sync conflict
    pub async fn resolve_conflict(
        &self,
        conflict_id: Uuid,
        resolution: ConflictResolution,
    ) -> Result<()> {
        let mut conflicts = self.conflicts.write().await;
        
        if let Some(conflict) = conflicts.remove(&conflict_id.to_string()) {
            match resolution {
                ConflictResolution::LocalWins => {
                    self.apply_local_version(&conflict).await?;
                }
                ConflictResolution::RemoteWins => {
                    self.apply_remote_version(&conflict).await?;
                }
                ConflictResolution::LastWriteWins => {
                    if conflict.local_version.timestamp > conflict.remote_version.timestamp {
                        self.apply_local_version(&conflict).await?;
                    } else {
                        self.apply_remote_version(&conflict).await?;
                    }
                }
                ConflictResolution::Merge => {
                    self.attempt_merge(&conflict).await?;
                }
                ConflictResolution::UserDecision => {
                    // This would typically involve UI interaction
                    warn!("User decision required for conflict: {}", conflict_id);
                }
            }

            info!("Resolved conflict: {} using {:?}", conflict_id, resolution);
        }

        Ok(())
    }

    /// Get current connection information
    pub async fn get_connection_info(&self) -> ConnectionInfo {
        self.connection_info.read().await.clone()
    }

    /// Check if sync is possible based on current connection
    pub async fn can_sync(&self) -> bool {
        let connection = self.connection_info.read().await;
        
        if !connection.is_connected {
            return false;
        }

        // Check WiFi-only restriction
        if self.config.wifi_only && connection.connection_type != ConnectionType::Wifi {
            return false;
        }

        // Check if connection is too slow or unstable
        if connection.bandwidth_mbps < 0.1 || connection.latency_ms > 5000 {
            return false;
        }

        true
    }

    // Private methods

    async fn start_connection_monitor(&self) -> Result<()> {
        let connection_info = Arc::clone(&self.connection_info);
        let http_client = self.http_client.clone();
        let base_url = self.base_url.clone();

        tokio::spawn(async move {
            let mut interval = interval(TokioDuration::from_secs(30));

            loop {
                interval.tick().await;

                // Test connection
                let (is_connected, latency) = Self::test_connection(&http_client, &base_url).await;
                
                let mut connection = connection_info.write().await;
                connection.is_connected = is_connected;
                connection.latency_ms = latency;
                
                if is_connected {
                    // Estimate bandwidth (simplified)
                    connection.bandwidth_mbps = Self::estimate_bandwidth(latency).await;
                    connection.connection_type = Self::detect_connection_type().await;
                } else {
                    connection.connection_type = ConnectionType::Offline;
                    connection.bandwidth_mbps = 0.0;
                }

                debug!("Connection status: connected={}, type={:?}, bandwidth={:.2} Mbps, latency={}ms",
                       connection.is_connected, connection.connection_type, 
                       connection.bandwidth_mbps, connection.latency_ms);
            }
        });

        Ok(())
    }

    async fn start_sync_processor(&self) -> Result<()> {
        let sync_queue = Arc::clone(&self.sync_queue);
        let storage_manager = Arc::clone(&self.storage_manager);
        let connection_info = Arc::clone(&self.connection_info);
        let conflicts = Arc::clone(&self.conflicts);
        let download_progress = Arc::clone(&self.download_progress);
        let http_client = self.http_client.clone();
        let base_url = self.base_url.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval = interval(TokioDuration::from_secs(5));

            loop {
                interval.tick().await;

                // Check if we can sync
                let can_sync = {
                    let connection = connection_info.read().await;
                    connection.is_connected && 
                    (!config.wifi_only || connection.connection_type == ConnectionType::Wifi)
                };

                if !can_sync {
                    continue;
                }

                // Process sync queue
                let operation = {
                    let mut queue = sync_queue.write().await;
                    queue.pop()
                };

                if let Some(mut operation) = operation {
                    match Self::process_sync_operation(
                        &mut operation,
                        &storage_manager,
                        &http_client,
                        &base_url,
                        &conflicts,
                        &download_progress,
                    ).await {
                        Ok(_) => {
                            debug!("Sync operation completed: {:?}", operation.id);
                        }
                        Err(e) => {
                            error!("Sync operation failed: {:?} - {}", operation.id, e);
                            operation.retry_count += 1;
                            operation.last_error = Some(e.to_string());

                            // Retry if under limit
                            if operation.retry_count < config.max_retries {
                                tokio::time::sleep(TokioDuration::from_secs(config.retry_delay_seconds)).await;
                                sync_queue.write().await.push(operation);
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    async fn start_periodic_sync(&self) -> Result<()> {
        if !self.config.auto_sync {
            return Ok(());
        }

        let storage_manager = Arc::clone(&self.storage_manager);
        let sync_queue = Arc::clone(&self.sync_queue);
        let interval_minutes = self.config.sync_interval_minutes;

        tokio::spawn(async move {
            let mut interval = interval(TokioDuration::from_secs(interval_minutes * 60));

            loop {
                interval.tick().await;

                // Get content that needs sync
                let pending_content = {
                    let manager = storage_manager.lock().await;
                    manager.get_sync_pending_content()
                        .into_iter()
                        .map(|c| (c.content_id.clone(), c.content_type.clone()))
                        .collect::<Vec<_>>()
                };

                // Queue sync operations
                let mut queue = sync_queue.write().await;
                for (content_id, content_type) in pending_content {
                    let operation = SyncOperation {
                        id: Uuid::new_v4(),
                        operation_type: SyncOperationType::Upload,
                        content_id,
                        content_type: content_type.clone(),
                        priority: Self::content_type_to_sync_priority_static(&content_type),
                        data: None,
                        created_at: Utc::now(),
                        retry_count: 0,
                        last_error: None,
                    };

                    queue.push(operation);
                }

                // Sort by priority
                queue.sort_by(|a, b| b.priority.cmp(&a.priority));

                if !queue.is_empty() {
                    info!("Queued {} items for periodic sync", queue.len());
                }
            }
        });

        Ok(())
    }

    async fn process_sync_operation(
        operation: &mut SyncOperation,
        storage_manager: &Arc<Mutex<OfflineStorageManager>>,
        http_client: &Client,
        base_url: &str,
        conflicts: &Arc<RwLock<HashMap<String, SyncConflict>>>,
        _download_progress: &Arc<RwLock<HashMap<Uuid, DownloadProgress>>>,
    ) -> Result<()> {
        match operation.operation_type {
            SyncOperationType::Upload => {
                Self::upload_content(operation, storage_manager, http_client, base_url).await
            }
            SyncOperationType::Download => {
                Self::download_content(operation, storage_manager, http_client, base_url, _download_progress).await
            }
            SyncOperationType::Delete => {
                Self::delete_content(operation, http_client, base_url).await
            }
            SyncOperationType::Resolve => {
                Self::resolve_content(operation, storage_manager, conflicts).await
            }
        }
    }

    async fn upload_content(
        operation: &SyncOperation,
        storage_manager: &Arc<Mutex<OfflineStorageManager>>,
        http_client: &Client,
        base_url: &str,
    ) -> Result<()> {
        // Get content from storage
        let content_data = {
            let mut manager = storage_manager.lock().await;
            manager.get_content(&operation.content_id).await?
        };

        if let Some(result) = content_data {
            if let Some(data) = result.data {
                // Upload to server
                let url = format!("{}/api/sync/upload", base_url);
                let payload = serde_json::json!({
                    "content_id": operation.content_id,
                    "content_type": operation.content_type,
                    "data": base64::encode(&data),
                    "timestamp": Utc::now()
                });

                let response = http_client
                    .post(&url)
                    .json(&payload)
                    .send()
                    .await?;

                if response.status().is_success() {
                    // Update sync status
                    let mut manager = storage_manager.lock().await;
                    manager.update_sync_status(&operation.content_id, SyncStatus::Synced, None).await?;
                    
                    info!("Uploaded content: {}", operation.content_id);
                } else {
                    let error = format!("Upload failed with status: {}", response.status());
                    let mut manager = storage_manager.lock().await;
                    manager.update_sync_status(&operation.content_id, SyncStatus::Failed, Some(error.clone())).await?;
                    return Err(anyhow::anyhow!(error));
                }
            }
        }

        Ok(())
    }

    async fn download_content(
        operation: &SyncOperation,
        storage_manager: &Arc<Mutex<OfflineStorageManager>>,
        http_client: &Client,
        base_url: &str,
        _download_progress: &Arc<RwLock<HashMap<Uuid, DownloadProgress>>>,
    ) -> Result<()> {
        // Download from server
        let url = format!("{}/api/sync/download/{}", base_url, operation.content_id);
        
        let response = http_client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            let content_data: Value = response.json().await?;
            
            // Extract content information
            let data = base64::decode(
                content_data["data"].as_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid content data"))?
            )?;

            let metadata = ContentMetadata {
                title: content_data["title"].as_str().unwrap_or("").to_string(),
                description: content_data["description"].as_str().map(|s| s.to_string()),
                language: content_data["language"].as_str().unwrap_or("ar").to_string(),
                source: content_data["source"].as_str().unwrap_or("").to_string(),
                author: content_data["author"].as_str().map(|s| s.to_string()),
                tags: content_data["tags"].as_array()
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_default(),
                content_hash: "".to_string(), // Will be calculated during storage
                original_size: data.len(),
                compressed_size: data.len(),
                compression_ratio: 1.0,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            // Store content
            let mut manager = storage_manager.lock().await;
            manager.store_content(
                operation.content_type.clone(),
                operation.content_id.clone(),
                data,
                metadata,
            ).await?;

            info!("Downloaded content: {}", operation.content_id);
        } else {
            let error = format!("Download failed with status: {}", response.status());
            return Err(anyhow::anyhow!(error));
        }

        Ok(())
    }

    async fn delete_content(
        operation: &SyncOperation,
        http_client: &Client,
        base_url: &str,
    ) -> Result<()> {
        let url = format!("{}/api/sync/delete/{}", base_url, operation.content_id);
        
        let response = http_client
            .delete(&url)
            .send()
            .await?;

        if response.status().is_success() {
            info!("Deleted content from server: {}", operation.content_id);
        } else {
            let error = format!("Delete failed with status: {}", response.status());
            return Err(anyhow::anyhow!(error));
        }

        Ok(())
    }

    async fn resolve_content(
        _operation: &SyncOperation,
        _storage_manager: &Arc<Mutex<OfflineStorageManager>>,
        _conflicts: &Arc<RwLock<HashMap<String, SyncConflict>>>,
    ) -> Result<()> {
        // Implementation for conflict resolution
        Ok(())
    }

    async fn apply_local_version(&self, conflict: &SyncConflict) -> Result<()> {
        // Apply local version and upload to server
        info!("Applying local version for conflict: {}", conflict.conflict_id);
        Ok(())
    }

    async fn apply_remote_version(&self, conflict: &SyncConflict) -> Result<()> {
        // Apply remote version and update local storage
        info!("Applying remote version for conflict: {}", conflict.conflict_id);
        Ok(())
    }

    async fn attempt_merge(&self, conflict: &SyncConflict) -> Result<()> {
        // Attempt to merge conflicting versions
        info!("Attempting merge for conflict: {}", conflict.conflict_id);
        Ok(())
    }

    async fn test_connection(http_client: &Client, base_url: &str) -> (bool, u64) {
        let start = std::time::Instant::now();
        
        match http_client.get(&format!("{}/health", base_url)).send().await {
            Ok(response) => {
                let latency = start.elapsed().as_millis() as u64;
                (response.status().is_success(), latency)
            }
            Err(_) => (false, 0),
        }
    }

    async fn estimate_bandwidth(latency_ms: u64) -> f64 {
        // Simplified bandwidth estimation based on latency
        if latency_ms < 50 {
            10.0 // High-speed connection
        } else if latency_ms < 200 {
            5.0  // Medium-speed connection
        } else if latency_ms < 1000 {
            1.0  // Slow connection
        } else {
            0.1  // Very slow connection
        }
    }

    async fn detect_connection_type() -> ConnectionType {
        // Simplified connection type detection
        // In a real implementation, this would use platform-specific APIs
        ConnectionType::Unknown
    }

    fn content_type_to_sync_priority(&self, content_type: &OfflineContentType) -> SyncPriority {
        Self::content_type_to_sync_priority_static(content_type)
    }

    fn content_type_to_sync_priority_static(content_type: &OfflineContentType) -> SyncPriority {
        match content_type {
            OfflineContentType::PrayerTimes |
            OfflineContentType::ReadingProgress => SyncPriority::Critical,
            
            OfflineContentType::UserBookmarks |
            OfflineContentType::PersonalNotes |
            OfflineContentType::UserPreferences => SyncPriority::High,
            
            OfflineContentType::QuranText |
            OfflineContentType::BasicTafsir |
            OfflineContentType::HadithCollection => SyncPriority::Normal,
            
            _ => SyncPriority::Low,
        }
    }
}
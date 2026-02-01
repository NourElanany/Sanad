use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Offline content management system for the Islamic application
/// Handles local storage, synchronization, and content prioritization

/// Offline content entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineContent {
    pub id: Uuid,
    pub content_type: OfflineContentType,
    pub content_id: String, // Reference to original content (e.g., "quran:1:1")
    pub data: Vec<u8>, // Compressed content data
    pub metadata: ContentMetadata,
    pub storage_info: StorageInfo,
    pub sync_info: SyncInfo,
}

/// Types of content that can be stored offline
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OfflineContentType {
    /// Essential content - always available offline
    QuranText,
    BasicTafsir,
    PrayerTimes,
    
    /// Important content - high priority for offline storage
    UserBookmarks,
    ReadingProgress,
    PersonalNotes,
    FavoriteHadith,
    
    /// Useful content - medium priority
    HadithCollection,
    IslamicStories,
    SearchCache,
    
    /// Optional content - low priority
    AudioRecordings,
    Images,
    ExtendedTafsir,
    
    /// User-generated content
    UserPreferences,
    CustomContent,
}

/// Content metadata for offline storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub title: String,
    pub description: Option<String>,
    pub language: String,
    pub source: String,
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub content_hash: String, // SHA-256 hash for integrity verification
    pub original_size: usize,
    pub compressed_size: usize,
    pub compression_ratio: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Storage information for offline content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub priority: StoragePriority,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub storage_path: Option<String>, // Local file path if stored on disk
    pub size_on_disk: usize,
    pub is_compressed: bool,
    pub compression_algorithm: CompressionAlgorithm,
}

/// Storage priority levels for offline content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StoragePriority {
    Essential = 4,   // Never removed, always synced
    High = 3,        // Removed only when critically low on space
    Medium = 2,      // Removed after 30 days of no access
    Low = 1,         // Removed after 7 days of no access
}

/// Compression algorithms supported
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionAlgorithm {
    None,
    Lz4,
    Gzip,
    Brotli,
}

/// Synchronization information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncInfo {
    pub last_synced: Option<DateTime<Utc>>,
    pub sync_status: SyncStatus,
    pub sync_strategy: SyncStrategy,
    pub conflict_resolution: ConflictResolution,
    pub pending_changes: Vec<PendingChange>,
    pub sync_attempts: u32,
    pub last_sync_error: Option<String>,
}

/// Synchronization status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    Synced,           // Content is up to date
    PendingUpload,    // Local changes need to be uploaded
    PendingDownload,  // Remote changes need to be downloaded
    Conflicted,       // Conflicts need resolution
    Failed,           // Last sync attempt failed
    Disabled,         // Sync disabled for this content
}

/// Synchronization strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStrategy {
    Immediate,        // Sync as soon as possible
    Periodic,         // Sync on schedule
    OnDemand,         // Sync only when requested
    WifiOnly,         // Sync only on WiFi
    Manual,           // Manual sync only
}

/// Conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolution {
    LocalWins,        // Keep local version
    RemoteWins,       // Keep remote version
    LastWriteWins,    // Keep version with latest timestamp
    Merge,            // Attempt to merge changes
    UserDecision,     // Ask user to resolve
}

/// Pending changes for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingChange {
    pub change_id: Uuid,
    pub change_type: ChangeType,
    pub timestamp: DateTime<Utc>,
    pub data: Vec<u8>,
    pub retry_count: u32,
}

/// Types of changes that can be synchronized
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Create,
    Update,
    Delete,
    Move,
    Rename,
}

/// Offline storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineConfig {
    /// Maximum storage space in MB
    pub max_storage_mb: u64,
    /// Minimum free space to maintain in MB
    pub min_free_space_mb: u64,
    /// Enable automatic cleanup
    pub auto_cleanup: bool,
    /// Cleanup interval in hours
    pub cleanup_interval_hours: u64,
    /// Enable compression
    pub enable_compression: bool,
    /// Default compression algorithm
    pub default_compression: CompressionAlgorithm,
    /// Sync configuration
    pub sync_config: SyncConfig,
    /// Content priorities
    pub content_priorities: HashMap<OfflineContentType, StoragePriority>,
}

/// Synchronization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Enable automatic sync
    pub auto_sync: bool,
    /// Sync interval in minutes
    pub sync_interval_minutes: u64,
    /// Sync only on WiFi
    pub wifi_only: bool,
    /// Maximum sync retries
    pub max_retries: u32,
    /// Retry delay in seconds
    pub retry_delay_seconds: u64,
    /// Batch size for sync operations
    pub batch_size: usize,
    /// Connection timeout in seconds
    pub connection_timeout_seconds: u64,
}

/// Offline storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineStats {
    pub total_items: u64,
    pub total_size_mb: f64,
    pub available_space_mb: f64,
    pub used_space_percentage: f64,
    pub items_by_type: HashMap<OfflineContentType, u64>,
    pub items_by_priority: HashMap<StoragePriority, u64>,
    pub compression_stats: CompressionStats,
    pub sync_stats: SyncStats,
    pub last_cleanup: Option<DateTime<Utc>>,
    pub last_sync: Option<DateTime<Utc>>,
}

/// Compression statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    pub total_original_size_mb: f64,
    pub total_compressed_size_mb: f64,
    pub average_compression_ratio: f64,
    pub space_saved_mb: f64,
    pub space_saved_percentage: f64,
}

/// Synchronization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStats {
    pub pending_uploads: u64,
    pub pending_downloads: u64,
    pub conflicts: u64,
    pub failed_syncs: u64,
    pub successful_syncs: u64,
    pub last_sync_duration_ms: u64,
    pub average_sync_time_ms: f64,
    pub total_data_synced_mb: f64,
}

/// Network connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub is_connected: bool,
    pub connection_type: ConnectionType,
    pub bandwidth_mbps: f64,
    pub latency_ms: u64,
    pub is_metered: bool,
    pub signal_strength: Option<f64>, // 0.0 to 1.0
}

/// Types of network connections
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    Wifi,
    Cellular,
    Ethernet,
    Unknown,
    Offline,
}

/// Offline operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub from_cache: bool,
    pub sync_pending: bool,
    pub last_updated: Option<DateTime<Utc>>,
}

/// Content download request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub content_type: OfflineContentType,
    pub content_ids: Vec<String>,
    pub priority: StoragePriority,
    pub force_download: bool, // Download even if already cached
    pub compression: Option<CompressionAlgorithm>,
}

/// Content download progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub request_id: Uuid,
    pub total_items: usize,
    pub completed_items: usize,
    pub current_item: Option<String>,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
    pub progress_percentage: f64,
    pub estimated_time_remaining_seconds: Option<u64>,
    pub download_speed_mbps: f64,
    pub status: DownloadStatus,
    pub errors: Vec<String>,
}

/// Download status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DownloadStatus {
    Queued,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    Paused,
}

/// Sync conflict that needs resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    pub conflict_id: Uuid,
    pub content_id: String,
    pub content_type: OfflineContentType,
    pub local_version: ConflictVersion,
    pub remote_version: ConflictVersion,
    pub conflict_type: ConflictType,
    pub created_at: DateTime<Utc>,
    pub auto_resolvable: bool,
    pub suggested_resolution: ConflictResolution,
}

/// Version information for conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictVersion {
    pub data: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub user_id: Option<Uuid>,
    pub checksum: String,
}

/// Types of synchronization conflicts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictType {
    ModifyModify,     // Both local and remote modified
    ModifyDelete,     // Local modified, remote deleted
    DeleteModify,     // Local deleted, remote modified
    CreateCreate,     // Created on both sides with different content
}

impl Default for OfflineConfig {
    fn default() -> Self {
        let mut content_priorities = HashMap::new();
        
        // Essential content
        content_priorities.insert(OfflineContentType::QuranText, StoragePriority::Essential);
        content_priorities.insert(OfflineContentType::BasicTafsir, StoragePriority::Essential);
        content_priorities.insert(OfflineContentType::PrayerTimes, StoragePriority::Essential);
        
        // Important content
        content_priorities.insert(OfflineContentType::UserBookmarks, StoragePriority::High);
        content_priorities.insert(OfflineContentType::ReadingProgress, StoragePriority::High);
        content_priorities.insert(OfflineContentType::PersonalNotes, StoragePriority::High);
        content_priorities.insert(OfflineContentType::UserPreferences, StoragePriority::High);
        
        // Useful content
        content_priorities.insert(OfflineContentType::HadithCollection, StoragePriority::Medium);
        content_priorities.insert(OfflineContentType::IslamicStories, StoragePriority::Medium);
        content_priorities.insert(OfflineContentType::SearchCache, StoragePriority::Medium);
        
        // Optional content
        content_priorities.insert(OfflineContentType::AudioRecordings, StoragePriority::Low);
        content_priorities.insert(OfflineContentType::Images, StoragePriority::Low);
        content_priorities.insert(OfflineContentType::ExtendedTafsir, StoragePriority::Low);

        Self {
            max_storage_mb: 2048, // 2GB default
            min_free_space_mb: 100, // 100MB minimum free space
            auto_cleanup: true,
            cleanup_interval_hours: 24,
            enable_compression: true,
            default_compression: CompressionAlgorithm::Lz4,
            sync_config: SyncConfig::default(),
            content_priorities,
        }
    }
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            auto_sync: true,
            sync_interval_minutes: 30,
            wifi_only: false,
            max_retries: 3,
            retry_delay_seconds: 60,
            batch_size: 50,
            connection_timeout_seconds: 30,
        }
    }
}

impl StoragePriority {
    /// Get cleanup threshold in days for this priority level
    pub fn cleanup_threshold_days(&self) -> Option<u32> {
        match self {
            StoragePriority::Essential => None, // Never cleanup
            StoragePriority::High => Some(90),  // 3 months
            StoragePriority::Medium => Some(30), // 1 month
            StoragePriority::Low => Some(7),     // 1 week
        }
    }
    
    /// Check if content with this priority should be kept during cleanup
    pub fn should_keep_during_cleanup(&self, last_accessed: DateTime<Utc>) -> bool {
        match self.cleanup_threshold_days() {
            None => true, // Essential content is never cleaned
            Some(days) => {
                let threshold = Utc::now() - chrono::Duration::days(days as i64);
                last_accessed > threshold
            }
        }
    }
}

impl OfflineContentType {
    /// Get the default storage priority for this content type
    pub fn default_priority(&self) -> StoragePriority {
        match self {
            OfflineContentType::QuranText |
            OfflineContentType::BasicTafsir |
            OfflineContentType::PrayerTimes => StoragePriority::Essential,
            
            OfflineContentType::UserBookmarks |
            OfflineContentType::ReadingProgress |
            OfflineContentType::PersonalNotes |
            OfflineContentType::UserPreferences |
            OfflineContentType::FavoriteHadith => StoragePriority::High,
            
            OfflineContentType::HadithCollection |
            OfflineContentType::IslamicStories |
            OfflineContentType::SearchCache => StoragePriority::Medium,
            
            OfflineContentType::AudioRecordings |
            OfflineContentType::Images |
            OfflineContentType::ExtendedTafsir |
            OfflineContentType::CustomContent => StoragePriority::Low,
        }
    }
    
    /// Get the default sync strategy for this content type
    pub fn default_sync_strategy(&self) -> SyncStrategy {
        match self {
            OfflineContentType::PrayerTimes |
            OfflineContentType::ReadingProgress => SyncStrategy::Immediate,
            
            OfflineContentType::UserBookmarks |
            OfflineContentType::PersonalNotes |
            OfflineContentType::UserPreferences => SyncStrategy::Periodic,
            
            OfflineContentType::AudioRecordings |
            OfflineContentType::Images |
            OfflineContentType::ExtendedTafsir => SyncStrategy::OnDemand,
            
            _ => SyncStrategy::Periodic,
        }
    }
}
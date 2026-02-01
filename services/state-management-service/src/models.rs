use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// User's personal data that needs CRDT synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPersonalData {
    pub user_id: Uuid,
    pub bookmarks: BookmarkSet,
    pub reading_progress: ReadingProgress,
    pub personal_notes: PersonalNotes,
    pub preferences: UserPreferences,
    pub last_updated: DateTime<Utc>,
}

/// Bookmark data using G-Set CRDT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkSet {
    pub bookmarks: HashMap<Uuid, Bookmark>,
    pub version_vector: HashMap<String, u64>,
}

/// Individual bookmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: Uuid,
    pub content_type: ContentType,
    pub content_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub folder: Option<String>,
    pub created_at: DateTime<Utc>,
    pub device_id: String,
}

/// Reading progress using PN-Counter CRDT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingProgress {
    pub quran_progress: HashMap<u8, SurahProgress>, // surah_number -> progress
    pub khatma_progress: HashMap<Uuid, KhatmaProgress>,
    pub version_vector: HashMap<String, u64>,
}

/// Progress for a specific Surah
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurahProgress {
    pub surah_number: u8,
    pub last_ayah_read: u16,
    pub completion_percentage: f64,
    pub last_read_at: DateTime<Utc>,
    pub device_id: String,
}

/// Progress for a Khatma plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KhatmaProgress {
    pub khatma_id: Uuid,
    pub completed_portions: u32,
    pub total_portions: u32,
    pub last_read_at: DateTime<Utc>,
    pub device_id: String,
}

/// Personal notes using RGA (Replicated Growable Array) CRDT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalNotes {
    pub notes: HashMap<Uuid, Note>,
    pub version_vector: HashMap<String, u64>,
}

/// Individual note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub content_type: ContentType,
    pub content_id: Uuid,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub device_id: String,
}

/// User preferences using LWW-Register CRDT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub language: LWWRegister<String>,
    pub theme: LWWRegister<String>,
    pub font_size: LWWRegister<String>,
    pub prayer_calculation_method: LWWRegister<String>,
    pub notification_settings: LWWRegister<NotificationSettings>,
    pub display_settings: LWWRegister<DisplaySettings>,
}

/// Last-Write-Wins Register for conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LWWRegister<T> {
    pub value: T,
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
}

impl<T> LWWRegister<T> {
    pub fn new(value: T, device_id: String) -> Self {
        Self {
            value,
            timestamp: Utc::now(),
            device_id,
        }
    }

    pub fn update(&mut self, new_value: T, device_id: String) {
        let now = Utc::now();
        if now > self.timestamp {
            self.value = new_value;
            self.timestamp = now;
            self.device_id = device_id;
        }
    }

    pub fn merge(&mut self, other: &LWWRegister<T>) 
    where 
        T: Clone 
    {
        if other.timestamp > self.timestamp || 
           (other.timestamp == self.timestamp && other.device_id > self.device_id) {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
            self.device_id = other.device_id.clone();
        }
    }
}

/// Content types for bookmarks and notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Quran,
    Hadith,
    Tafsir,
    Story,
    Article,
}

/// Notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub prayer_reminders: bool,
    pub prayer_reminder_minutes: i32,
    pub islamic_events: bool,
    pub khatma_reminders: bool,
    pub daily_verse: bool,
}

/// Display settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub theme: String,
    pub font_size: String,
    pub arabic_font: String,
    pub translation_font: String,
}

/// Synchronization strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStrategy {
    Immediate,  // For critical data like prayer times, khatma progress
    Periodic,   // For bookmarks, reading history, preferences
    OnDemand,   // For audio recordings, offline content
}

/// Conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolution {
    LastWriteWins,  // For user preferences, display settings
    SetUnion,       // For bookmarks, favorite surahs
    MaxValue,       // For reading progress, khatma completion
}

/// Sync operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub success: bool,
    pub conflicts_resolved: u32,
    pub items_synced: u32,
    pub sync_time_ms: u64,
    pub errors: Vec<String>,
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_size_mb: f64,
    pub available_space_mb: f64,
    pub items_count: u32,
    pub last_cleanup: DateTime<Utc>,
    pub compression_ratio: f64,
}
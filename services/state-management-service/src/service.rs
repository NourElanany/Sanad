use crate::{Config, models::*, crdt::CRDTManager, sync::SmartSyncManager, storage::SmartStorageManager};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};
use uuid::Uuid;

/// Main state management service
pub struct StateManagementService {
    config: Config,
    crdt_manager: Arc<CRDTManager>,
    sync_manager: Arc<SmartSyncManager>,
    storage_manager: Arc<RwLock<SmartStorageManager>>,
    user_data_cache: Arc<RwLock<std::collections::HashMap<Uuid, UserPersonalData>>>,
}

impl StateManagementService {
    pub async fn new(config: Config) -> Result<Self> {
        let device_id = format!("device_{}", Uuid::new_v4());
        
        let crdt_manager = Arc::new(CRDTManager::new(device_id.clone()));
        let sync_manager = Arc::new(SmartSyncManager::new(device_id));
        let storage_manager = Arc::new(RwLock::new(SmartStorageManager::new(
            config.max_storage_mb,
            config.compression_enabled,
        )));
        let user_data_cache = Arc::new(RwLock::new(std::collections::HashMap::new()));

        Ok(Self {
            config,
            crdt_manager,
            sync_manager,
            storage_manager,
            user_data_cache,
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("State Management Service starting on {}:{}", 
               self.config.server_host, self.config.server_port);

        // Start the sync manager
        let sync_handle = {
            let sync_manager = Arc::clone(&self.sync_manager);
            tokio::spawn(async move {
                if let Err(e) = sync_manager.start_adaptive_sync().await {
                    error!("Sync manager error: {}", e);
                }
            })
        };

        // Start periodic storage optimization
        let storage_handle = {
            let storage_manager = Arc::clone(&self.storage_manager);
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // Every hour
                loop {
                    interval.tick().await;
                    let mut storage = storage_manager.write().await;
                    if let Err(e) = storage.optimize_storage().await {
                        error!("Storage optimization error: {}", e);
                    }
                }
            })
        };

        // Wait for all services
        tokio::try_join!(sync_handle, storage_handle)?;

        Ok(())
    }

    /// Get user's personal data
    pub async fn get_user_data(&self, user_id: Uuid) -> Result<Option<UserPersonalData>> {
        // Check cache first
        {
            let cache = self.user_data_cache.read().await;
            if let Some(data) = cache.get(&user_id) {
                return Ok(Some(data.clone()));
            }
        }

        // Load from storage if not in cache
        // Implementation would load from database/storage
        Ok(None)
    }

    /// Update user's personal data using CRDTs
    pub async fn update_user_data(
        &self,
        user_id: Uuid,
        update: UserDataUpdate,
    ) -> Result<SyncResult> {
        let mut cache = self.user_data_cache.write().await;
        
        // Get or create user data
        let user_data = cache.entry(user_id).or_insert_with(|| {
            UserPersonalData {
                user_id,
                bookmarks: BookmarkSet {
                    bookmarks: std::collections::HashMap::new(),
                    version_vector: std::collections::HashMap::new(),
                },
                reading_progress: ReadingProgress {
                    quran_progress: std::collections::HashMap::new(),
                    khatma_progress: std::collections::HashMap::new(),
                    version_vector: std::collections::HashMap::new(),
                },
                personal_notes: PersonalNotes {
                    notes: std::collections::HashMap::new(),
                    version_vector: std::collections::HashMap::new(),
                },
                preferences: UserPreferences {
                    language: LWWRegister::new("ar".to_string(), "default".to_string()),
                    theme: LWWRegister::new("light".to_string(), "default".to_string()),
                    font_size: LWWRegister::new("medium".to_string(), "default".to_string()),
                    prayer_calculation_method: LWWRegister::new("MuslimWorldLeague".to_string(), "default".to_string()),
                    notification_settings: LWWRegister::new(
                        NotificationSettings {
                            prayer_reminders: true,
                            prayer_reminder_minutes: 15,
                            islamic_events: true,
                            khatma_reminders: true,
                            daily_verse: true,
                        },
                        "default".to_string()
                    ),
                    display_settings: LWWRegister::new(
                        DisplaySettings {
                            theme: "light".to_string(),
                            font_size: "medium".to_string(),
                            arabic_font: "Amiri".to_string(),
                            translation_font: "Arial".to_string(),
                        },
                        "default".to_string()
                    ),
                },
                last_updated: chrono::Utc::now(),
            }
        });

        // Apply the update using CRDT operations
        match update {
            UserDataUpdate::AddBookmark(bookmark) => {
                self.crdt_manager.add_bookmark(&mut user_data.bookmarks, bookmark)?;
            }
            UserDataUpdate::UpdateSurahProgress { surah_number, last_ayah_read, completion_percentage } => {
                self.crdt_manager.update_surah_progress(
                    &mut user_data.reading_progress,
                    surah_number,
                    last_ayah_read,
                    completion_percentage,
                )?;
            }
            UserDataUpdate::UpdateKhatmaProgress { khatma_id, completed_portions, total_portions } => {
                self.crdt_manager.update_khatma_progress(
                    &mut user_data.reading_progress,
                    khatma_id,
                    completed_portions,
                    total_portions,
                )?;
            }
            UserDataUpdate::UpdateNote { note_id, content_type, content_id, text } => {
                self.crdt_manager.update_note(
                    &mut user_data.personal_notes,
                    note_id,
                    content_type,
                    content_id,
                    text,
                )?;
            }
            UserDataUpdate::UpdateLanguage(language) => {
                self.crdt_manager.update_preference(&mut user_data.preferences.language, language)?;
            }
            UserDataUpdate::UpdateTheme(theme) => {
                self.crdt_manager.update_preference(&mut user_data.preferences.theme, theme)?;
            }
        }

        user_data.last_updated = chrono::Utc::now();

        // Queue sync operation
        let serialized_data = serde_json::to_vec(user_data)?;
        self.sync_manager.queue_sync_operation(
            crate::sync::SyncOperationType::FullSync,
            serialized_data,
            crate::sync::SyncPriority::High,
        )?;

        Ok(SyncResult {
            success: true,
            conflicts_resolved: 0,
            items_synced: 1,
            sync_time_ms: 0,
            errors: Vec::new(),
        })
    }

    /// Sync user data with remote
    pub async fn sync_user_data(
        &self,
        user_id: Uuid,
        remote_data: UserPersonalData,
    ) -> Result<SyncResult> {
        let mut cache = self.user_data_cache.write().await;
        
        if let Some(local_data) = cache.get_mut(&user_id) {
            self.crdt_manager.merge_personal_data(local_data, &remote_data)
        } else {
            // No local data, just store the remote data
            cache.insert(user_id, remote_data);
            Ok(SyncResult {
                success: true,
                conflicts_resolved: 0,
                items_synced: 1,
                sync_time_ms: 0,
                errors: Vec::new(),
            })
        }
    }

    /// Get storage statistics
    pub async fn get_storage_stats(&self) -> Result<StorageStats> {
        let storage = self.storage_manager.read().await;
        Ok(storage.get_storage_stats().clone())
    }

    /// Get sync statistics
    pub fn get_sync_stats(&self) -> crate::sync::SyncStats {
        self.sync_manager.get_sync_stats()
    }
}

/// User data update operations
#[derive(Debug, Clone)]
pub enum UserDataUpdate {
    AddBookmark(Bookmark),
    UpdateSurahProgress {
        surah_number: u8,
        last_ayah_read: u16,
        completion_percentage: f64,
    },
    UpdateKhatmaProgress {
        khatma_id: Uuid,
        completed_portions: u32,
        total_portions: u32,
    },
    UpdateNote {
        note_id: Uuid,
        content_type: ContentType,
        content_id: Uuid,
        text: String,
    },
    UpdateLanguage(String),
    UpdateTheme(String),
}
use crate::{models::*, service::{StateManagementService, UserDataUpdate}};
use shared::{ApiResponse};
use std::sync::Arc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// HTTP handlers for the state management service
pub struct StateHandlers {
    service: Arc<StateManagementService>,
}

impl StateHandlers {
    pub fn new(service: Arc<StateManagementService>) -> Self {
        Self { service }
    }

    /// Get user's personal data
    pub async fn get_user_data(&self, user_id: Uuid) -> ApiResponse<UserPersonalData> {
        match self.service.get_user_data(user_id).await {
            Ok(Some(data)) => ApiResponse::success(data),
            Ok(None) => ApiResponse::error("User data not found".to_string()),
            Err(e) => ApiResponse::error(format!("Failed to get user data: {}", e)),
        }
    }

    /// Add bookmark
    pub async fn add_bookmark(&self, request: AddBookmarkRequest) -> ApiResponse<SyncResult> {
        let bookmark = Bookmark {
            id: Uuid::new_v4(),
            content_type: request.content_type,
            content_id: request.content_id,
            title: request.title,
            description: request.description,
            tags: request.tags,
            folder: request.folder,
            created_at: chrono::Utc::now(),
            device_id: request.device_id,
        };

        let update = UserDataUpdate::AddBookmark(bookmark);
        
        match self.service.update_user_data(request.user_id, update).await {
            Ok(result) => ApiResponse::success(result),
            Err(e) => ApiResponse::error(format!("Failed to add bookmark: {}", e)),
        }
    }

    /// Update reading progress
    pub async fn update_reading_progress(&self, request: UpdateReadingProgressRequest) -> ApiResponse<SyncResult> {
        let update = UserDataUpdate::UpdateSurahProgress {
            surah_number: request.surah_number,
            last_ayah_read: request.last_ayah_read,
            completion_percentage: request.completion_percentage,
        };

        match self.service.update_user_data(request.user_id, update).await {
            Ok(result) => ApiResponse::success(result),
            Err(e) => ApiResponse::error(format!("Failed to update reading progress: {}", e)),
        }
    }

    /// Update khatma progress
    pub async fn update_khatma_progress(&self, request: UpdateKhatmaProgressRequest) -> ApiResponse<SyncResult> {
        let update = UserDataUpdate::UpdateKhatmaProgress {
            khatma_id: request.khatma_id,
            completed_portions: request.completed_portions,
            total_portions: request.total_portions,
        };

        match self.service.update_user_data(request.user_id, update).await {
            Ok(result) => ApiResponse::success(result),
            Err(e) => ApiResponse::error(format!("Failed to update khatma progress: {}", e)),
        }
    }

    /// Add or update personal note
    pub async fn update_note(&self, request: UpdateNoteRequest) -> ApiResponse<SyncResult> {
        let update = UserDataUpdate::UpdateNote {
            note_id: request.note_id,
            content_type: request.content_type,
            content_id: request.content_id,
            text: request.text,
        };

        match self.service.update_user_data(request.user_id, update).await {
            Ok(result) => ApiResponse::success(result),
            Err(e) => ApiResponse::error(format!("Failed to update note: {}", e)),
        }
    }

    /// Update user preferences
    pub async fn update_preferences(&self, request: UpdatePreferencesRequest) -> ApiResponse<SyncResult> {
        // Handle different preference updates
        let update = match request.preference_type.as_str() {
            "language" => UserDataUpdate::UpdateLanguage(request.value),
            "theme" => UserDataUpdate::UpdateTheme(request.value),
            _ => return ApiResponse::error("Invalid preference type".to_string()),
        };

        match self.service.update_user_data(request.user_id, update).await {
            Ok(result) => ApiResponse::success(result),
            Err(e) => ApiResponse::error(format!("Failed to update preferences: {}", e)),
        }
    }

    /// Sync user data with remote
    pub async fn sync_user_data(&self, request: SyncUserDataRequest) -> ApiResponse<SyncResult> {
        match self.service.sync_user_data(request.user_id, request.remote_data).await {
            Ok(result) => ApiResponse::success(result),
            Err(e) => ApiResponse::error(format!("Failed to sync user data: {}", e)),
        }
    }

    /// Get storage statistics
    pub async fn get_storage_stats(&self) -> ApiResponse<StorageStats> {
        match self.service.get_storage_stats().await {
            Ok(stats) => ApiResponse::success(stats),
            Err(e) => ApiResponse::error(format!("Failed to get storage stats: {}", e)),
        }
    }

    /// Get sync statistics
    pub async fn get_sync_stats(&self) -> ApiResponse<crate::sync::SyncStats> {
        let stats = self.service.get_sync_stats();
        ApiResponse::success(stats)
    }
}

/// Request models for API endpoints

#[derive(Debug, Deserialize)]
pub struct AddBookmarkRequest {
    pub user_id: Uuid,
    pub content_type: ContentType,
    pub content_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub folder: Option<String>,
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReadingProgressRequest {
    pub user_id: Uuid,
    pub surah_number: u8,
    pub last_ayah_read: u16,
    pub completion_percentage: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateKhatmaProgressRequest {
    pub user_id: Uuid,
    pub khatma_id: Uuid,
    pub completed_portions: u32,
    pub total_portions: u32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub user_id: Uuid,
    pub note_id: Uuid,
    pub content_type: ContentType,
    pub content_id: Uuid,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub user_id: Uuid,
    pub preference_type: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct SyncUserDataRequest {
    pub user_id: Uuid,
    pub remote_data: UserPersonalData,
}
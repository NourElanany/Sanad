use crate::models::*;
use crate::service::OfflineService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn, error};
use uuid::Uuid;

/// HTTP handlers for the offline service API
pub struct OfflineHandlers;

/// Request to store content offline
#[derive(Debug, Deserialize)]
pub struct StoreContentRequest {
    pub content_type: OfflineContentType,
    pub content_id: String,
    pub data: String, // Base64 encoded data
    pub title: String,
    pub metadata: Option<ContentMetadata>,
}

/// Response for store content operation
#[derive(Debug, Serialize)]
pub struct StoreContentResponse {
    pub id: Uuid,
    pub success: bool,
    pub message: String,
}

/// Query parameters for listing content
#[derive(Debug, Deserialize)]
pub struct ListContentQuery {
    pub content_type: Option<OfflineContentType>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Response for listing content
#[derive(Debug, Serialize)]
pub struct ListContentResponse {
    pub content: Vec<OfflineContentSummary>,
    pub total: usize,
    pub has_more: bool,
}

/// Summary of offline content for listing
#[derive(Debug, Serialize)]
pub struct OfflineContentSummary {
    pub id: Uuid,
    pub content_type: OfflineContentType,
    pub content_id: String,
    pub title: String,
    pub size_mb: f64,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub sync_status: SyncStatus,
    pub priority: StoragePriority,
}

/// Request to resolve sync conflict
#[derive(Debug, Deserialize)]
pub struct ResolveConflictRequest {
    pub resolution: ConflictResolution,
}

/// Generic API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl OfflineHandlers {
    /// Create router with all offline service endpoints
    pub fn create_router(service: Arc<OfflineService>) -> Router {
        Router::new()
            .route("/content", post(Self::store_content))
            .route("/content", get(Self::list_content))
            .route("/content/:content_id", get(Self::get_content))
            .route("/content/:content_id", delete(Self::remove_content))
            .route("/download", post(Self::download_content))
            .route("/download/:request_id", get(Self::get_download_progress))
            .route("/download/:request_id", delete(Self::cancel_download))
            .route("/sync/force", post(Self::force_sync))
            .route("/sync/conflicts", get(Self::get_conflicts))
            .route("/sync/conflicts/:conflict_id", post(Self::resolve_conflict))
            .route("/stats", get(Self::get_statistics))
            .route("/cleanup", post(Self::cleanup_storage))
            .route("/optimize", post(Self::optimize_storage))
            .route("/verify", post(Self::verify_integrity))
            .route("/connection", get(Self::get_connection_info))
            .route("/health", get(Self::health_check))
            // Specialized endpoints for Islamic content
            .route("/quran/:surah", get(Self::get_quran_surah))
            .route("/quran/:surah/:ayah", get(Self::get_quran_ayah))
            .route("/prayer-times/:lat/:lng/:date", get(Self::get_prayer_times))
            .route("/bookmarks/:user_id", get(Self::get_user_bookmarks))
            .route("/bookmarks/:user_id", put(Self::store_user_bookmarks))
            .route("/progress/:user_id", get(Self::get_reading_progress))
            .route("/progress/:user_id", put(Self::store_reading_progress))
            .with_state(service)
    }

    /// Store content for offline access
    async fn store_content(
        State(service): State<Arc<OfflineService>>,
        Json(request): Json<StoreContentRequest>,
    ) -> Result<Json<ApiResponse<StoreContentResponse>>, StatusCode> {
        info!("Storing content offline: {} (type: {:?})", request.content_id, request.content_type);

        // Decode base64 data
        let data = match base64::decode(&request.data) {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to decode base64 data: {}", e);
                return Ok(Json(ApiResponse::error(format!("Invalid base64 data: {}", e))));
            }
        };

        match service.store_content(
            request.content_type,
            request.content_id.clone(),
            data,
            request.title,
            request.metadata,
        ).await {
            Ok(id) => {
                let response = StoreContentResponse {
                    id,
                    success: true,
                    message: format!("Content stored successfully: {}", request.content_id),
                };
                Ok(Json(ApiResponse::success(response)))
            }
            Err(e) => {
                error!("Failed to store content: {}", e);
                Ok(Json(ApiResponse::error(format!("Failed to store content: {}", e))))
            }
        }
    }

    /// Get content from offline storage
    async fn get_content(
        State(service): State<Arc<OfflineService>>,
        Path(content_id): Path<String>,
    ) -> Result<Json<ApiResponse<Option<OfflineResult<String>>>>, StatusCode> {
        match service.get_content(&content_id).await {
            Ok(result) => {
                if let Some(offline_result) = result {
                    if let Some(data) = offline_result.data {
                        // Convert bytes to base64 string for JSON response
                        let encoded_data = base64::encode(&data);
                        let string_result = OfflineResult {
                            success: offline_result.success,
                            data: Some(encoded_data),
                            error: offline_result.error,
                            from_cache: offline_result.from_cache,
                            sync_pending: offline_result.sync_pending,
                            last_updated: offline_result.last_updated,
                        };
                        Ok(Json(ApiResponse::success(Some(string_result))))
                    } else {
                        let string_result = OfflineResult {
                            success: offline_result.success,
                            data: None,
                            error: offline_result.error,
                            from_cache: offline_result.from_cache,
                            sync_pending: offline_result.sync_pending,
                            last_updated: offline_result.last_updated,
                        };
                        Ok(Json(ApiResponse::success(Some(string_result))))
                    }
                } else {
                    Ok(Json(ApiResponse::success(None)))
                }
            }
            Err(e) => {
                error!("Failed to get content: {}", e);
                Ok(Json(ApiResponse::error(format!("Failed to get content: {}", e))))
            }
        }
    }

    /// List offline content
    async fn list_content(
        State(service): State<Arc<OfflineService>>,
        Query(query): Query<ListContentQuery>,
    ) -> Result<Json<ApiResponse<ListContentResponse>>, StatusCode> {
        let content = service.list_content(query.content_type).await;
        let total = content.len();
        
        // Apply pagination
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(50);
        let end = std::cmp::min(offset + limit, total);
        
        let paginated_content: Vec<OfflineContentSummary> = content
            .into_iter()
            .skip(offset)
            .take(limit)
            .map(|c| OfflineContentSummary {
                id: c.id,
                content_type: c.content_type,
                content_id: c.content_id,
                title: c.metadata.title,
                size_mb: c.storage_info.size_on_disk as f64 / (1024.0 * 1024.0),
                last_accessed: c.storage_info.last_accessed,
                sync_status: c.sync_info.sync_status,
                priority: c.storage_info.priority,
            })
            .collect();

        let response = ListContentResponse {
            content: paginated_content,
            total,
            has_more: end < total,
        };

        Ok(Json(ApiResponse::success(response)))
    }

    /// Remove content from offline storage
    async fn remove_content(
        State(service): State<Arc<OfflineService>>,
        Path(content_id): Path<String>,
    ) -> Result<Json<ApiResponse<bool>>, StatusCode> {
        match service.remove_content(&content_id).await {
            Ok(removed) => {
                if removed {
                    info!("Removed content from offline storage: {}", content_id);
                } else {
                    warn!("Content not found for removal: {}", content_id);
                }
                Ok(Json(ApiResponse::success(removed)))
            }
            Err(e) => {
                error!("Failed to remove content: {}", e);
                Ok(Json(ApiResponse::error(format!("Failed to remove content: {}", e))))
            }
        }
    }

    /// Download content for offline access
    async fn download_content(
        State(service): State<Arc<OfflineService>>,
        Json(request): Json<DownloadRequest>,
    ) -> Result<Json<ApiResponse<Uuid>>, StatusCode> {
        info!("Queuing download: {} items of type {:?}", 
              request.content_ids.len(), request.content_type);

        match service.download_content(request).await {
            Ok(request_id) => Ok(Json(ApiResponse::success(request_id))),
            Err(e) => {
                error!("Failed to queue download: {}", e);
                Ok(Json(ApiResponse::error(format!("Failed to queue download: {}", e))))
            }
        }
    }

    /// Get download progress
    async fn get_download_progress(
        State(service): State<Arc<OfflineService>>,
        Path(request_id): Path<Uuid>,
    ) -> Result<Json<ApiResponse<Option<DownloadProgress>>>, StatusCode> {
        let progress = service.get_download_progress(request_id).await;
        Ok(Json(ApiResponse::success(progress)))
    }

    /// Cancel download
    async fn cancel_download(
        State(service): State<Arc<OfflineService>>,
        Path(request_id): Path<Uuid>,
    ) -> Result<Json<ApiResponse<bool>>, StatusCode> {
        match service.cancel_download(request_id).await {
            Ok(_) => {
                info!("Cancelled download: {}", request_id);
                Ok(Json(ApiResponse::success(true)))
            }
            Err(e) => {
                error!("Failed to cancel download: {}", e);
                Ok(Json(ApiResponse::error(format!("Failed to cancel download: {}", e))))
            }
        }
    }

    /// Force synchronization
    async fn force_sync(
        State(service): State<Arc<OfflineService>>,
        Json(request): Json<serde_json::Value>,
    ) -> Result<Json<ApiResponse<bool>>, StatusCode> {
        let content_id = request["content_id"].as_str()
            .ok_or(StatusCode::BAD_REQUEST)?
            .to_string();
        
        let content_type: OfflineContentType = serde_json::from_value(request["content_type"].clone())
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        match service.force_sync(content_id.clone(), content_type).await {
            Ok(_) => {
                info!("Forced sync for content: {}", content_id);
                Ok(Json(ApiResponse::success(true)))
            }
            Err(e) => {
                error!("Failed to force sync: {}", e);
                Ok(Json(ApiResponse::error(format!("Failed to force sync: {}", e))))
            }
        }
    }

    /// Get sync conflicts
    async fn get_conflicts(
        State(service): State<Arc<OfflineService>>,
    ) -> Result<Json<ApiResponse<Vec<SyncConflict>>>, StatusCode> {
        let conflicts = service.get_sync_conflicts().await;
        Ok(Json(ApiResponse::success(conflicts)))
    }

    /// Resolve sync conflict
    async fn resolve_conflict(
        State(service): State<Arc<OfflineService>>,
        Path(conflict_id): Path<Uuid>,
        Json(request): Json<ResolveConflictRequest>,
    ) -> Result<Json<ApiResponse<bool>>, StatusCode> {
        match service.resolve_conflict(conflict_id, request.resolution).await {
            Ok(_) => {
                info!("Resolved conflict: {}", conflict_id);
                Ok(Json(ApiResponse::success(true)))
            }
            Err(e) => {
                error!("Failed to resolve conflict: {}", e);
                Ok(Json(ApiResponse::error(format!("Failed to resolve conflict: {}", e))))
            }
        }
    }

    /// Get storage statistics
    async fn get_statistics(
        State(service): State<Arc<OfflineService>>,
    ) -> Result<Json<ApiResponse<OfflineStats>>, StatusCode> {
        let stats = service.get_statistics().await;
        Ok(Json(ApiResponse::success(stats)))
    }

    /// Cleanup storage
    async fn cleanup_storage(
        State(service): State<Arc<OfflineService>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<u64>>, StatusCode> {
        let target_free_space = params.get("target_free_space_mb")
            .and_then(|s| s.parse::<f64>().ok());

        match service.cleanup_storage(target_free_space).await {
            Ok(cleaned_count) => {
                info!("Storage cleanup completed: {} items removed", cleaned_count);
                Ok(Json(ApiResponse::success(cleaned_count)))
            }
            Err(e) => {
                error!("Storage cleanup failed: {}", e);
                Ok(Json(ApiResponse::error(format!("Storage cleanup failed: {}", e))))
            }
        }
    }

    /// Optimize storage
    async fn optimize_storage(
        State(service): State<Arc<OfflineService>>,
    ) -> Result<Json<ApiResponse<f64>>, StatusCode> {
        match service.optimize_storage().await {
            Ok(space_saved) => {
                info!("Storage optimization completed: {:.2} MB saved", space_saved);
                Ok(Json(ApiResponse::success(space_saved)))
            }
            Err(e) => {
                error!("Storage optimization failed: {}", e);
                Ok(Json(ApiResponse::error(format!("Storage optimization failed: {}", e))))
            }
        }
    }

    /// Verify content integrity
    async fn verify_integrity(
        State(service): State<Arc<OfflineService>>,
    ) -> Result<Json<ApiResponse<Vec<String>>>, StatusCode> {
        match service.verify_integrity().await {
            Ok(corrupted) => {
                if corrupted.is_empty() {
                    info!("Content integrity verification passed");
                } else {
                    warn!("Found {} corrupted content items", corrupted.len());
                }
                Ok(Json(ApiResponse::success(corrupted)))
            }
            Err(e) => {
                error!("Content integrity verification failed: {}", e);
                Ok(Json(ApiResponse::error(format!("Integrity verification failed: {}", e))))
            }
        }
    }

    /// Get connection information
    async fn get_connection_info(
        State(service): State<Arc<OfflineService>>,
    ) -> Result<Json<ApiResponse<ConnectionInfo>>, StatusCode> {
        let connection_info = service.get_connection_info().await;
        Ok(Json(ApiResponse::success(connection_info)))
    }

    /// Health check endpoint
    async fn health_check(
        State(service): State<Arc<OfflineService>>,
    ) -> Result<Json<ApiResponse<HashMap<String, serde_json::Value>>>, StatusCode> {
        let stats = service.get_statistics().await;
        let connection = service.get_connection_info().await;
        
        let mut health_info = HashMap::new();
        health_info.insert("status".to_string(), serde_json::Value::String("healthy".to_string()));
        health_info.insert("total_items".to_string(), serde_json::Value::Number(stats.total_items.into()));
        health_info.insert("total_size_mb".to_string(), serde_json::json!(stats.total_size_mb));
        health_info.insert("available_space_mb".to_string(), serde_json::json!(stats.available_space_mb));
        health_info.insert("is_online".to_string(), serde_json::Value::Bool(connection.is_connected));
        health_info.insert("sync_pending".to_string(), serde_json::Value::Number(stats.sync_stats.pending_uploads.into()));

        Ok(Json(ApiResponse::success(health_info)))
    }

    // Specialized Islamic content endpoints

    /// Get Quran surah
    async fn get_quran_surah(
        State(service): State<Arc<OfflineService>>,
        Path(surah): Path<u16>,
    ) -> Result<Json<ApiResponse<Option<OfflineResult<String>>>>, StatusCode> {
        match service.get_quran_content(surah, None).await {
            Ok(result) => Ok(Json(ApiResponse::success(result))),
            Err(e) => {
                error!("Failed to get Quran surah {}: {}", surah, e);
                Ok(Json(ApiResponse::error(format!("Failed to get Quran surah: {}", e))))
            }
        }
    }

    /// Get specific Quran ayah
    async fn get_quran_ayah(
        State(service): State<Arc<OfflineService>>,
        Path((surah, ayah)): Path<(u16, u16)>,
    ) -> Result<Json<ApiResponse<Option<OfflineResult<String>>>>, StatusCode> {
        match service.get_quran_content(surah, Some(ayah)).await {
            Ok(result) => Ok(Json(ApiResponse::success(result))),
            Err(e) => {
                error!("Failed to get Quran ayah {}:{}: {}", surah, ayah, e);
                Ok(Json(ApiResponse::error(format!("Failed to get Quran ayah: {}", e))))
            }
        }
    }

    /// Get prayer times
    async fn get_prayer_times(
        State(service): State<Arc<OfflineService>>,
        Path((lat, lng, date)): Path<(f64, f64, String)>,
    ) -> Result<Json<ApiResponse<Option<OfflineResult<String>>>>, StatusCode> {
        match service.get_prayer_times(lat, lng, &date).await {
            Ok(result) => Ok(Json(ApiResponse::success(result))),
            Err(e) => {
                error!("Failed to get prayer times for {}, {} on {}: {}", lat, lng, date, e);
                Ok(Json(ApiResponse::error(format!("Failed to get prayer times: {}", e))))
            }
        }
    }

    /// Get user bookmarks
    async fn get_user_bookmarks(
        State(service): State<Arc<OfflineService>>,
        Path(user_id): Path<Uuid>,
    ) -> Result<Json<ApiResponse<Option<OfflineResult<String>>>>, StatusCode> {
        match service.get_user_bookmarks(user_id).await {
            Ok(result) => Ok(Json(ApiResponse::success(result))),
            Err(e) => {
                error!("Failed to get bookmarks for user {}: {}", user_id, e);
                Ok(Json(ApiResponse::error(format!("Failed to get bookmarks: {}", e))))
            }
        }
    }

    /// Store user bookmarks
    async fn store_user_bookmarks(
        State(service): State<Arc<OfflineService>>,
        Path(user_id): Path<Uuid>,
        Json(bookmarks): Json<serde_json::Value>,
    ) -> Result<Json<ApiResponse<Uuid>>, StatusCode> {
        let bookmarks_json = serde_json::to_string(&bookmarks)
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        match service.store_user_bookmarks(user_id, bookmarks_json).await {
            Ok(id) => {
                info!("Stored bookmarks for user: {}", user_id);
                Ok(Json(ApiResponse::success(id)))
            }
            Err(e) => {
                error!("Failed to store bookmarks for user {}: {}", user_id, e);
                Ok(Json(ApiResponse::error(format!("Failed to store bookmarks: {}", e))))
            }
        }
    }

    /// Get reading progress
    async fn get_reading_progress(
        State(service): State<Arc<OfflineService>>,
        Path(user_id): Path<Uuid>,
    ) -> Result<Json<ApiResponse<Option<OfflineResult<String>>>>, StatusCode> {
        match service.get_reading_progress(user_id).await {
            Ok(result) => Ok(Json(ApiResponse::success(result))),
            Err(e) => {
                error!("Failed to get reading progress for user {}: {}", user_id, e);
                Ok(Json(ApiResponse::error(format!("Failed to get reading progress: {}", e))))
            }
        }
    }

    /// Store reading progress
    async fn store_reading_progress(
        State(service): State<Arc<OfflineService>>,
        Path(user_id): Path<Uuid>,
        Json(progress): Json<serde_json::Value>,
    ) -> Result<Json<ApiResponse<Uuid>>, StatusCode> {
        let progress_json = serde_json::to_string(&progress)
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        match service.store_reading_progress(user_id, progress_json).await {
            Ok(id) => {
                info!("Stored reading progress for user: {}", user_id);
                Ok(Json(ApiResponse::success(id)))
            }
            Err(e) => {
                error!("Failed to store reading progress for user {}: {}", user_id, e);
                Ok(Json(ApiResponse::error(format!("Failed to store reading progress: {}", e))))
            }
        }
    }
}
use crate::{models::*, crdt::*, sync::*};
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_priority_ordering() {
        let mut priorities = vec![
            SyncPriority::Low,
            SyncPriority::Critical,
            SyncPriority::Normal,
            SyncPriority::High,
        ];
        
        priorities.sort();
        
        // Should be ordered from highest to lowest priority
        assert_eq!(priorities[0], SyncPriority::Critical);
        assert_eq!(priorities[1], SyncPriority::High);
        assert_eq!(priorities[2], SyncPriority::Normal);
        assert_eq!(priorities[3], SyncPriority::Low);
    }

    #[test]
    fn test_smart_sync_manager_creation() {
        let device_id = "test_device".to_string();
        let sync_manager = SmartSyncManager::new(device_id.clone());
        
        let stats = sync_manager.get_sync_stats();
        assert_eq!(stats.device_id, device_id);
        assert!(stats.connection_quality.bandwidth_mbps > 0.0);
        assert!(stats.connection_quality.stability_score > 0.0);
        assert!(stats.connection_quality.stability_score <= 1.0);
    }

    #[test]
    fn test_sync_operation_creation() {
        let operation = SyncOperation {
            id: Uuid::new_v4(),
            operation_type: SyncOperationType::BookmarkAdd,
            data: vec![1, 2, 3, 4],
            priority: SyncPriority::High,
            created_at: Utc::now(),
            retry_count: 0,
        };
        
        assert_eq!(operation.retry_count, 0);
        assert_eq!(operation.priority, SyncPriority::High);
        assert_eq!(operation.data.len(), 4);
    }

    #[test]
    fn test_connection_quality_assessment() {
        let quality = ConnectionQuality {
            bandwidth_mbps: 10.0,
            latency_ms: 50,
            stability_score: 0.9,
            last_assessed: Utc::now(),
        };
        
        assert_eq!(quality.bandwidth_mbps, 10.0);
        assert_eq!(quality.latency_ms, 50);
        assert_eq!(quality.stability_score, 0.9);
    }

    #[test]
    fn test_sync_strategies_configuration() {
        let device_id = "test_device".to_string();
        let sync_manager = SmartSyncManager::new(device_id);
        
        let stats = sync_manager.get_sync_stats();
        
        // Verify that sync strategies are configured
        assert!(stats.sync_strategies.contains_key("prayer_times"));
        assert!(stats.sync_strategies.contains_key("khatma_progress"));
        assert!(stats.sync_strategies.contains_key("bookmarks"));
        
        // Verify critical data uses immediate sync
        assert_eq!(stats.sync_strategies.get("prayer_times"), Some(&SyncStrategy::Immediate));
        assert_eq!(stats.sync_strategies.get("khatma_progress"), Some(&SyncStrategy::Immediate));
        
        // Verify non-critical data uses periodic sync
        assert_eq!(stats.sync_strategies.get("bookmarks"), Some(&SyncStrategy::Periodic));
        assert_eq!(stats.sync_strategies.get("reading_history"), Some(&SyncStrategy::Periodic));
    }

    #[test]
    fn test_conflict_resolution_strategies() {
        let device_id = "test_device".to_string();
        let sync_manager = SmartSyncManager::new(device_id);
        
        // Test different conflict resolution strategies
        let local_data = vec![1, 2, 3];
        let remote_data = vec![4, 5, 6];
        
        // Test last write wins
        let result = sync_manager.resolve_conflict("user_preferences", &local_data, &remote_data);
        assert!(result.is_ok());
        
        // Test set union
        let result = sync_manager.resolve_conflict("bookmarks", &local_data, &remote_data);
        assert!(result.is_ok());
        
        // Test max value
        let result = sync_manager.resolve_conflict("reading_progress", &local_data, &remote_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sync_queue_operations() {
        let queue = SyncQueue {
            operations: vec![],
            priority_operations: vec![],
            failed_operations: vec![],
        };
        
        assert_eq!(queue.operations.len(), 0);
        assert_eq!(queue.priority_operations.len(), 0);
        assert_eq!(queue.failed_operations.len(), 0);
    }

    #[test]
    fn test_failed_sync_operation() {
        let operation = SyncOperation {
            id: Uuid::new_v4(),
            operation_type: SyncOperationType::ProgressUpdate,
            data: vec![1, 2, 3],
            priority: SyncPriority::Critical,
            created_at: Utc::now(),
            retry_count: 1,
        };
        
        let failed_operation = FailedSyncOperation {
            operation,
            error: "Network timeout".to_string(),
            failed_at: Utc::now(),
            next_retry: Utc::now() + chrono::Duration::minutes(5),
        };
        
        assert_eq!(failed_operation.error, "Network timeout");
        assert_eq!(failed_operation.operation.retry_count, 1);
    }

    #[tokio::test]
    async fn test_sync_manager_queue_operation() {
        let device_id = "test_device".to_string();
        let sync_manager = SmartSyncManager::new(device_id);
        
        let data = vec![1, 2, 3, 4, 5];
        let result = sync_manager.queue_sync_operation(
            SyncOperationType::BookmarkAdd,
            data.clone(),
            SyncPriority::High,
        );
        
        assert!(result.is_ok());
        let operation_id = result.unwrap();
        assert!(!operation_id.is_nil());
    }

    #[test]
    fn test_adaptive_sync_interval_calculation() {
        let device_id = "test_device".to_string();
        let mut sync_manager = SmartSyncManager::new(device_id);
        
        // Test with high quality connection
        sync_manager.connection_quality.stability_score = 0.9;
        sync_manager.connection_quality.bandwidth_mbps = 10.0;
        let interval = sync_manager.calculate_adaptive_sync_interval();
        assert!(interval >= 30); // Base interval
        
        // Test with low quality connection
        sync_manager.connection_quality.stability_score = 0.3;
        sync_manager.connection_quality.bandwidth_mbps = 0.5;
        let interval = sync_manager.calculate_adaptive_sync_interval();
        assert!(interval > 30); // Should be longer than base interval
    }

    #[test]
    fn test_sync_operation_types() {
        // Test all sync operation types
        let operations = vec![
            SyncOperationType::BookmarkAdd,
            SyncOperationType::BookmarkUpdate,
            SyncOperationType::ProgressUpdate,
            SyncOperationType::NoteAdd,
            SyncOperationType::NoteUpdate,
            SyncOperationType::PreferenceUpdate,
            SyncOperationType::FullSync,
        ];
        
        assert_eq!(operations.len(), 7);
        
        // Verify serialization works
        for op_type in operations {
            let serialized = serde_json::to_string(&op_type);
            assert!(serialized.is_ok());
        }
    }

    #[test]
    fn test_sync_result_creation() {
        let result = SyncResult {
            success: true,
            conflicts_resolved: 2,
            items_synced: 10,
            sync_time_ms: 150,
            errors: vec![],
        };
        
        assert!(result.success);
        assert_eq!(result.conflicts_resolved, 2);
        assert_eq!(result.items_synced, 10);
        assert_eq!(result.sync_time_ms, 150);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_sync_result_with_errors() {
        let result = SyncResult {
            success: false,
            conflicts_resolved: 0,
            items_synced: 5,
            sync_time_ms: 300,
            errors: vec![
                "Network timeout".to_string(),
                "Invalid data format".to_string(),
            ],
        };
        
        assert!(!result.success);
        assert_eq!(result.errors.len(), 2);
        assert!(result.errors.contains(&"Network timeout".to_string()));
        assert!(result.errors.contains(&"Invalid data format".to_string()));
    }
}
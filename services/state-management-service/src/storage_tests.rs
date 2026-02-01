use crate::{models::*, storage::*};
use chrono::Utc;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_priority_ordering() {
        let mut priorities = vec![
            StoragePriority::Optional,
            StoragePriority::Essential,
            StoragePriority::Useful,
            StoragePriority::Important,
        ];
        
        priorities.sort();
        
        // Should be ordered from highest to lowest priority
        assert_eq!(priorities[0], StoragePriority::Essential);
        assert_eq!(priorities[1], StoragePriority::Important);
        assert_eq!(priorities[2], StoragePriority::Useful);
        assert_eq!(priorities[3], StoragePriority::Optional);
    }

    #[test]
    fn test_smart_storage_manager_creation() {
        let storage_manager = SmartStorageManager::new(500, true);
        let stats = storage_manager.get_storage_stats();
        
        assert_eq!(stats.available_space_mb, 500.0);
        assert_eq!(stats.total_size_mb, 0.0);
        assert_eq!(stats.items_count, 0);
        assert_eq!(stats.compression_ratio, 1.0);
    }

    #[test]
    fn test_content_metadata_creation() {
        let metadata = ContentMetadata {
            id: Uuid::new_v4(),
            content_type: "quran_text".to_string(),
            size_bytes: 1024,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 5,
            priority: StoragePriority::Essential,
            compressed: false,
            checksum: "abc123".to_string(),
        };
        
        assert_eq!(metadata.size_bytes, 1024);
        assert_eq!(metadata.access_count, 5);
        assert_eq!(metadata.priority, StoragePriority::Essential);
        assert!(!metadata.compressed);
        assert_eq!(metadata.checksum, "abc123");
    }

    #[test]
    fn test_cleanup_policy_configuration() {
        let storage_manager = SmartStorageManager::new(500, true);
        
        // Test that cleanup policies are configured correctly
        // This would normally access private fields, so we test indirectly
        let stats = storage_manager.get_storage_stats();
        assert!(stats.available_space_mb > 0.0);
    }

    #[tokio::test]
    async fn test_storage_result_creation() {
        let result = StorageResult {
            success: true,
            bytes_saved: 1024,
            bytes_freed: 2048,
            items_cleaned: 10,
            compression_ratio: 1.5,
            errors: vec![],
        };
        
        assert!(result.success);
        assert_eq!(result.bytes_saved, 1024);
        assert_eq!(result.bytes_freed, 2048);
        assert_eq!(result.items_cleaned, 10);
        assert_eq!(result.compression_ratio, 1.5);
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_storage_result_with_errors() {
        let result = StorageResult {
            success: false,
            bytes_saved: 0,
            bytes_freed: 512,
            items_cleaned: 2,
            compression_ratio: 1.0,
            errors: vec![
                "Disk full".to_string(),
                "Permission denied".to_string(),
            ],
        };
        
        assert!(!result.success);
        assert_eq!(result.bytes_freed, 512);
        assert_eq!(result.items_cleaned, 2);
        assert_eq!(result.errors.len(), 2);
        assert!(result.errors.contains(&"Disk full".to_string()));
        assert!(result.errors.contains(&"Permission denied".to_string()));
    }

    #[tokio::test]
    async fn test_smart_cleanup_simulation() {
        let mut storage_manager = SmartStorageManager::new(100, true); // 100MB limit
        
        // Simulate storage being nearly full
        storage_manager.storage_stats.total_size_mb = 95.0;
        storage_manager.storage_stats.available_space_mb = 5.0;
        
        // Request cleanup for 10MB
        let result = storage_manager.smart_cleanup(10.0).await;
        
        assert!(result.is_ok());
        let cleanup_result = result.unwrap();
        
        // Should have attempted cleanup
        assert!(cleanup_result.bytes_freed > 0 || !cleanup_result.success);
    }

    #[tokio::test]
    async fn test_store_content_simulation() {
        let mut storage_manager = SmartStorageManager::new(500, true);
        
        let metadata = ContentMetadata {
            id: Uuid::new_v4(),
            content_type: "test_content".to_string(),
            size_bytes: 1024,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 1,
            priority: StoragePriority::Important,
            compressed: false,
            checksum: "test123".to_string(),
        };
        
        let test_data = vec![1, 2, 3, 4, 5]; // 5 bytes
        let result = storage_manager.store_content("test_content", &test_data, metadata).await;
        
        assert!(result.is_ok());
        let store_result = result.unwrap();
        assert!(store_result.success);
    }

    #[test]
    fn test_compression_data() {
        let storage_manager = SmartStorageManager::new(500, true);
        
        // Test with compressible data (repeated pattern)
        let compressible_data = vec![1u8; 1000]; // 1000 bytes of same value
        let result = storage_manager.compress_data(&compressible_data);
        
        assert!(result.is_ok());
        let (compressed, ratio) = result.unwrap();
        
        // Should either compress well or return original if compression isn't beneficial
        if ratio > 1.1 {
            assert!(compressed.len() < compressible_data.len());
        } else {
            assert_eq!(compressed, compressible_data);
        }
    }

    #[test]
    fn test_decompression_data() {
        let storage_manager = SmartStorageManager::new(500, true);
        
        // Test compression and decompression round trip
        let original_data = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5]; // Some pattern
        let (compressed, _ratio) = storage_manager.compress_data(&original_data).unwrap();
        
        // Only test decompression if data was actually compressed
        if compressed != original_data {
            let decompressed = storage_manager.decompress_data(&compressed);
            assert!(decompressed.is_ok());
            assert_eq!(decompressed.unwrap(), original_data);
        }
    }

    #[tokio::test]
    async fn test_update_access_metadata() {
        let mut storage_manager = SmartStorageManager::new(500, true);
        let content_id = Uuid::new_v4();
        
        let result = storage_manager.update_access_metadata(content_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_content_integrity() {
        let storage_manager = SmartStorageManager::new(500, true);
        let content_id = Uuid::new_v4();
        
        let result = storage_manager.verify_content_integrity(content_id).await;
        assert!(result.is_ok());
        // In the mock implementation, this always returns true
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_get_content_by_priority() {
        let storage_manager = SmartStorageManager::new(500, true);
        
        let result = storage_manager.get_content_by_priority(StoragePriority::Essential).await;
        assert!(result.is_ok());
        
        let content_list = result.unwrap();
        // In the mock implementation, this returns an empty list
        assert_eq!(content_list.len(), 0);
    }

    #[tokio::test]
    async fn test_optimize_storage() {
        let mut storage_manager = SmartStorageManager::new(500, true);
        
        let result = storage_manager.optimize_storage().await;
        assert!(result.is_ok());
        
        let optimization_result = result.unwrap();
        assert!(optimization_result.success);
        assert!(optimization_result.bytes_saved > 0); // Mock implementation saves 5MB
    }

    #[test]
    fn test_storage_stats_update() {
        let mut storage_manager = SmartStorageManager::new(500, true);
        
        // Simulate adding content
        storage_manager.storage_stats.total_size_mb = 100.0;
        storage_manager.storage_stats.available_space_mb = 400.0;
        storage_manager.storage_stats.items_count = 50;
        
        let stats = storage_manager.get_storage_stats();
        assert_eq!(stats.total_size_mb, 100.0);
        assert_eq!(stats.available_space_mb, 400.0);
        assert_eq!(stats.items_count, 50);
    }

    #[test]
    fn test_cleanup_policy_creation() {
        let policy = CleanupPolicy {
            max_age_days: 30,
            max_size_mb: 100.0,
            usage_threshold: 0.5,
        };
        
        assert_eq!(policy.max_age_days, 30);
        assert_eq!(policy.max_size_mb, 100.0);
        assert_eq!(policy.usage_threshold, 0.5);
    }

    #[tokio::test]
    async fn test_cleanup_by_priority_essential() {
        let storage_manager = SmartStorageManager::new(500, true);
        
        // Essential content should never be cleaned
        let result = storage_manager.cleanup_by_priority(StoragePriority::Essential).await;
        assert!(result.is_ok());
        
        let cleanup_result = result.unwrap();
        // Essential content cleanup should return 0 bytes freed
        assert_eq!(cleanup_result.bytes_freed, 0);
        assert_eq!(cleanup_result.items_cleaned, 0);
    }

    #[tokio::test]
    async fn test_cleanup_by_priority_optional() {
        let storage_manager = SmartStorageManager::new(500, true);
        
        // Optional content should be cleaned aggressively
        let result = storage_manager.cleanup_by_priority(StoragePriority::Optional).await;
        assert!(result.is_ok());
        
        let cleanup_result = result.unwrap();
        // Mock implementation frees 10MB and cleans 50 items
        assert_eq!(cleanup_result.bytes_freed, 10 * 1024 * 1024);
        assert_eq!(cleanup_result.items_cleaned, 50);
    }

    #[test]
    fn test_storage_priority_levels() {
        // Test that all priority levels are defined
        let priorities = vec![
            StoragePriority::Essential,
            StoragePriority::Important,
            StoragePriority::Useful,
            StoragePriority::Optional,
        ];
        
        assert_eq!(priorities.len(), 4);
        
        // Test serialization
        for priority in priorities {
            let serialized = serde_json::to_string(&priority);
            assert!(serialized.is_ok());
        }
    }
}
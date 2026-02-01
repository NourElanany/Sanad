use crate::models::*;
use crate::storage_manager::OfflineStorageManager;
use crate::service::{OfflineService, OfflineServiceBuilder};
use chrono::Utc;
use tempfile::TempDir;
use uuid::Uuid;

/// Unit tests for offline storage functionality
#[cfg(test)]
mod storage_tests {
    use super::*;

    #[tokio::test]
    async fn test_storage_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = OfflineConfig::default();
        
        let manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_store_and_retrieve_content() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let config = OfflineConfig::default();
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await?;

        // Test data
        let content_type = OfflineContentType::QuranText;
        let content_id = "quran:1:1".to_string();
        let data = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ".as_bytes().to_vec();
        let metadata = ContentMetadata {
            title: "Al-Fatiha Verse 1".to_string(),
            description: Some("The opening verse of the Quran".to_string()),
            language: "ar".to_string(),
            source: "Quran".to_string(),
            author: None,
            tags: vec!["quran".to_string(), "al-fatiha".to_string()],
            content_hash: "".to_string(),
            original_size: data.len(),
            compressed_size: data.len(),
            compression_ratio: 1.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store content
        let id = manager.store_content(content_type, content_id.clone(), data.clone(), metadata).await?;
        assert!(!id.is_nil());

        // Retrieve content
        let result = manager.get_content(&content_id).await?;
        assert!(result.is_some());
        
        let offline_result = result.unwrap();
        assert!(offline_result.success);
        assert!(offline_result.data.is_some());
        
        let retrieved_data = offline_result.data.unwrap();
        assert_eq!(retrieved_data, data);

        Ok(())
    }

    #[tokio::test]
    async fn test_content_compression() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let mut config = OfflineConfig::default();
        config.enable_compression = true;
        config.default_compression = CompressionAlgorithm::Lz4;
        
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await?;

        // Large test data that should compress well
        let large_data = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ ".repeat(1000);
        let data = large_data.as_bytes().to_vec();
        
        let metadata = ContentMetadata {
            title: "Large Quran Text".to_string(),
            description: None,
            language: "ar".to_string(),
            source: "test".to_string(),
            author: None,
            tags: Vec::new(),
            content_hash: "".to_string(),
            original_size: data.len(),
            compressed_size: data.len(),
            compression_ratio: 1.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let content_id = "test:large_content".to_string();
        let id = manager.store_content(
            OfflineContentType::QuranText,
            content_id.clone(),
            data.clone(),
            metadata,
        ).await?;

        // Verify content was stored and compressed
        let stored_content = manager.list_content(None);
        let content = stored_content.iter().find(|c| c.id == id).unwrap();
        
        assert!(content.storage_info.is_compressed);
        assert!(content.metadata.compression_ratio > 1.0);
        assert!(content.metadata.compressed_size < content.metadata.original_size);

        // Verify we can retrieve the original data
        let result = manager.get_content(&content_id).await?;
        assert!(result.is_some());
        
        let retrieved_data = result.unwrap().data.unwrap();
        assert_eq!(retrieved_data, data);

        Ok(())
    }

    #[tokio::test]
    async fn test_storage_cleanup() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let mut config = OfflineConfig::default();
        config.max_storage_mb = 1; // Very small limit to force cleanup
        
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await?;

        // Store multiple items with different priorities
        let items = vec![
            (OfflineContentType::QuranText, StoragePriority::Essential, "essential_content"),
            (OfflineContentType::UserBookmarks, StoragePriority::High, "high_priority_content"),
            (OfflineContentType::SearchCache, StoragePriority::Medium, "medium_priority_content"),
            (OfflineContentType::AudioRecordings, StoragePriority::Low, "low_priority_content"),
        ];

        for (content_type, _priority, content_id) in &items {
            let data = format!("Test data for {}", content_id).repeat(1000).into_bytes();
            let metadata = ContentMetadata {
                title: content_id.to_string(),
                description: None,
                language: "en".to_string(),
                source: "test".to_string(),
                author: None,
                tags: Vec::new(),
                content_hash: "".to_string(),
                original_size: data.len(),
                compressed_size: data.len(),
                compression_ratio: 1.0,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            manager.store_content(content_type.clone(), content_id.to_string(), data, metadata).await?;
        }

        // Trigger cleanup
        let cleaned_count = manager.cleanup_storage(None).await?;
        
        // Essential content should never be cleaned
        let remaining_content = manager.list_content(None);
        let essential_remaining = remaining_content.iter()
            .any(|c| c.content_type == OfflineContentType::QuranText);
        assert!(essential_remaining);

        // Some low priority content should have been cleaned
        assert!(cleaned_count > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_content_expiry() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let config = OfflineConfig::default();
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await?;

        // Store prayer times (which have expiry)
        let content_id = "prayer_times:40.7128:-74.0060:2024-01-01".to_string();
        let data = r#"{"fajr":"05:30","dhuhr":"12:15","asr":"15:30","maghrib":"18:00","isha":"19:30"}"#.as_bytes().to_vec();
        
        let metadata = ContentMetadata {
            title: "Prayer Times NYC".to_string(),
            description: None,
            language: "en".to_string(),
            source: "prayer_service".to_string(),
            author: None,
            tags: vec!["prayer_times".to_string()],
            content_hash: "".to_string(),
            original_size: data.len(),
            compressed_size: data.len(),
            compression_ratio: 1.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        manager.store_content(
            OfflineContentType::PrayerTimes,
            content_id.clone(),
            data,
            metadata,
        ).await?;

        // Verify content is stored
        let result = manager.get_content(&content_id).await?;
        assert!(result.is_some());

        // Check that the content has an expiry time set
        let stored_content = manager.list_content(Some(OfflineContentType::PrayerTimes));
        assert!(!stored_content.is_empty());
        assert!(stored_content[0].storage_info.expires_at.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_integrity_verification() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let config = OfflineConfig::default();
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await?;

        // Store some content
        let content_id = "test:integrity".to_string();
        let data = "Test data for integrity verification".as_bytes().to_vec();
        let metadata = ContentMetadata {
            title: "Integrity Test".to_string(),
            description: None,
            language: "en".to_string(),
            source: "test".to_string(),
            author: None,
            tags: Vec::new(),
            content_hash: "".to_string(),
            original_size: data.len(),
            compressed_size: data.len(),
            compression_ratio: 1.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        manager.store_content(
            OfflineContentType::CustomContent,
            content_id,
            data,
            metadata,
        ).await?;

        // Verify integrity
        let corrupted = manager.verify_integrity().await?;
        assert!(corrupted.is_empty(), "No content should be corrupted initially");

        Ok(())
    }
}

/// Integration tests for the complete offline service
#[cfg(test)]
mod service_tests {
    use super::*;

    async fn create_test_service() -> Result<OfflineService> {
        let temp_dir = TempDir::new().unwrap();
        let config = OfflineConfig::default();
        
        OfflineServiceBuilder::new()
            .storage_path(temp_dir.path().to_path_buf())
            .config(config)
            .server_url("http://localhost:8080".to_string())
            .build()
            .await
    }

    #[tokio::test]
    async fn test_service_creation() {
        let service = create_test_service().await;
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_quran_content_storage_and_retrieval() -> Result<()> {
        let service = create_test_service().await?;

        // Store Quran content
        let surah_text = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
        let content_id = "quran:1:1".to_string();
        
        let id = service.store_content(
            OfflineContentType::QuranText,
            content_id.clone(),
            surah_text.as_bytes().to_vec(),
            "Al-Fatiha Verse 1".to_string(),
            None,
        ).await?;

        assert!(!id.is_nil());

        // Retrieve using specialized method
        let result = service.get_quran_content(1, Some(1)).await?;
        assert!(result.is_some());
        
        let quran_result = result.unwrap();
        assert!(quran_result.success);
        assert_eq!(quran_result.data.unwrap(), surah_text);

        Ok(())
    }

    #[tokio::test]
    async fn test_user_bookmarks_management() -> Result<()> {
        let service = create_test_service().await?;
        let user_id = Uuid::new_v4();

        // Store bookmarks
        let bookmarks = r#"{"bookmarks":[{"surah":1,"ayah":1,"note":"Beautiful verse"}]}"#;
        let id = service.store_user_bookmarks(user_id, bookmarks.to_string()).await?;
        assert!(!id.is_nil());

        // Retrieve bookmarks
        let result = service.get_user_bookmarks(user_id).await?;
        assert!(result.is_some());
        
        let bookmark_result = result.unwrap();
        assert!(bookmark_result.success);
        assert_eq!(bookmark_result.data.unwrap(), bookmarks);

        Ok(())
    }

    #[tokio::test]
    async fn test_reading_progress_management() -> Result<()> {
        let service = create_test_service().await?;
        let user_id = Uuid::new_v4();

        // Store reading progress
        let progress = r#"{"current_surah":2,"current_ayah":10,"completion_percentage":15.5}"#;
        let id = service.store_reading_progress(user_id, progress.to_string()).await?;
        assert!(!id.is_nil());

        // Retrieve reading progress
        let result = service.get_reading_progress(user_id).await?;
        assert!(result.is_some());
        
        let progress_result = result.unwrap();
        assert!(progress_result.success);
        assert_eq!(progress_result.data.unwrap(), progress);

        Ok(())
    }

    #[tokio::test]
    async fn test_content_listing_and_filtering() -> Result<()> {
        let service = create_test_service().await?;

        // Store different types of content
        let content_types = vec![
            OfflineContentType::QuranText,
            OfflineContentType::UserBookmarks,
            OfflineContentType::HadithCollection,
        ];

        for (i, content_type) in content_types.iter().enumerate() {
            let content_id = format!("test_content_{}", i);
            let data = format!("Test data {}", i).into_bytes();
            
            service.store_content(
                content_type.clone(),
                content_id,
                data,
                format!("Test Content {}", i),
                None,
            ).await?;
        }

        // List all content
        let all_content = service.list_content(None).await;
        assert_eq!(all_content.len(), 3);

        // List only Quran content
        let quran_content = service.list_content(Some(OfflineContentType::QuranText)).await;
        assert_eq!(quran_content.len(), 1);
        assert_eq!(quran_content[0].content_type, OfflineContentType::QuranText);

        Ok(())
    }

    #[tokio::test]
    async fn test_storage_statistics() -> Result<()> {
        let service = create_test_service().await?;

        // Store some content
        for i in 0..5 {
            let content_id = format!("stats_test_{}", i);
            let data = format!("Statistics test data {}", i).repeat(100).into_bytes();
            
            service.store_content(
                OfflineContentType::CustomContent,
                content_id,
                data,
                format!("Stats Test {}", i),
                None,
            ).await?;
        }

        // Get statistics
        let stats = service.get_statistics().await;
        
        assert_eq!(stats.total_items, 5);
        assert!(stats.total_size_mb > 0.0);
        assert!(stats.items_by_type.contains_key(&OfflineContentType::CustomContent));
        assert_eq!(stats.items_by_type[&OfflineContentType::CustomContent], 5);

        Ok(())
    }

    #[tokio::test]
    async fn test_content_removal() -> Result<()> {
        let service = create_test_service().await?;

        // Store content
        let content_id = "removal_test".to_string();
        let data = "Content to be removed".as_bytes().to_vec();
        
        service.store_content(
            OfflineContentType::CustomContent,
            content_id.clone(),
            data,
            "Removal Test".to_string(),
            None,
        ).await?;

        // Verify content exists
        let result = service.get_content(&content_id).await?;
        assert!(result.is_some());

        // Remove content
        let removed = service.remove_content(&content_id).await?;
        assert!(removed);

        // Verify content is gone
        let result = service.get_content(&content_id).await?;
        assert!(result.is_none());

        Ok(())
    }
}

/// Property-based tests for offline functionality
#[cfg(test)]
mod property_tests {
    use super::*;
    use quickcheck::TestResult;

    #[quickcheck]
    fn prop_content_round_trip(content_data: Vec<u8>) -> TestResult {
        if content_data.is_empty() {
            return TestResult::discard();
        }

        let rt = tokio::runtime::Runtime::new().unwrap();
        
        rt.block_on(async {
            let temp_dir = TempDir::new().unwrap();
            let config = OfflineConfig::default();
            let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await.unwrap();

            let content_id = "prop_test".to_string();
            let metadata = ContentMetadata {
                title: "Property Test".to_string(),
                description: None,
                language: "en".to_string(),
                source: "test".to_string(),
                author: None,
                tags: Vec::new(),
                content_hash: "".to_string(),
                original_size: content_data.len(),
                compressed_size: content_data.len(),
                compression_ratio: 1.0,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            // Store content
            let _id = manager.store_content(
                OfflineContentType::CustomContent,
                content_id.clone(),
                content_data.clone(),
                metadata,
            ).await.unwrap();

            // Retrieve content
            let result = manager.get_content(&content_id).await.unwrap();
            assert!(result.is_some());
            
            let retrieved_data = result.unwrap().data.unwrap();
            assert_eq!(retrieved_data, content_data);
        });

        TestResult::passed()
    }

    #[quickcheck]
    fn prop_compression_preserves_data(content_data: Vec<u8>) -> TestResult {
        if content_data.len() < 100 {
            return TestResult::discard(); // Skip small data that might not compress well
        }

        let rt = tokio::runtime::Runtime::new().unwrap();
        
        rt.block_on(async {
            let temp_dir = TempDir::new().unwrap();
            let mut config = OfflineConfig::default();
            config.enable_compression = true;
            config.default_compression = CompressionAlgorithm::Lz4;
            
            let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await.unwrap();

            let content_id = "compression_prop_test".to_string();
            let metadata = ContentMetadata {
                title: "Compression Property Test".to_string(),
                description: None,
                language: "en".to_string(),
                source: "test".to_string(),
                author: None,
                tags: Vec::new(),
                content_hash: "".to_string(),
                original_size: content_data.len(),
                compressed_size: content_data.len(),
                compression_ratio: 1.0,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            // Store content with compression
            let _id = manager.store_content(
                OfflineContentType::CustomContent,
                content_id.clone(),
                content_data.clone(),
                metadata,
            ).await.unwrap();

            // Retrieve and verify data is identical
            let result = manager.get_content(&content_id).await.unwrap();
            assert!(result.is_some());
            
            let retrieved_data = result.unwrap().data.unwrap();
            assert_eq!(retrieved_data, content_data);
        });

        TestResult::passed()
    }

    #[test]
    fn test_storage_priority_ordering() {
        // Test that storage priorities are correctly ordered
        assert!(StoragePriority::Essential > StoragePriority::High);
        assert!(StoragePriority::High > StoragePriority::Medium);
        assert!(StoragePriority::Medium > StoragePriority::Low);
    }

    #[test]
    fn test_content_type_default_priorities() {
        // Test that essential Islamic content has the highest priority
        assert_eq!(OfflineContentType::QuranText.default_priority(), StoragePriority::Essential);
        assert_eq!(OfflineContentType::BasicTafsir.default_priority(), StoragePriority::Essential);
        assert_eq!(OfflineContentType::PrayerTimes.default_priority(), StoragePriority::Essential);
        
        // Test that user data has high priority
        assert_eq!(OfflineContentType::UserBookmarks.default_priority(), StoragePriority::High);
        assert_eq!(OfflineContentType::ReadingProgress.default_priority(), StoragePriority::High);
        
        // Test that optional content has low priority
        assert_eq!(OfflineContentType::AudioRecordings.default_priority(), StoragePriority::Low);
        assert_eq!(OfflineContentType::Images.default_priority(), StoragePriority::Low);
    }

    #[test]
    fn test_sync_strategy_defaults() {
        // Test that critical content uses immediate sync
        assert_eq!(OfflineContentType::PrayerTimes.default_sync_strategy(), SyncStrategy::Immediate);
        assert_eq!(OfflineContentType::ReadingProgress.default_sync_strategy(), SyncStrategy::Immediate);
        
        // Test that user content uses periodic sync
        assert_eq!(OfflineContentType::UserBookmarks.default_sync_strategy(), SyncStrategy::Periodic);
        assert_eq!(OfflineContentType::PersonalNotes.default_sync_strategy(), SyncStrategy::Periodic);
        
        // Test that heavy content uses on-demand sync
        assert_eq!(OfflineContentType::AudioRecordings.default_sync_strategy(), SyncStrategy::OnDemand);
        assert_eq!(OfflineContentType::Images.default_sync_strategy(), SyncStrategy::OnDemand);
    }
}

/// Performance tests for offline functionality
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_large_content_storage_performance() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let mut config = OfflineConfig::default();
        config.enable_compression = true;
        
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await?;

        // Create large content (1MB)
        let large_content = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ ".repeat(50000);
        let data = large_content.as_bytes().to_vec();
        
        let metadata = ContentMetadata {
            title: "Large Performance Test".to_string(),
            description: None,
            language: "ar".to_string(),
            source: "test".to_string(),
            author: None,
            tags: Vec::new(),
            content_hash: "".to_string(),
            original_size: data.len(),
            compressed_size: data.len(),
            compression_ratio: 1.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Measure storage time
        let start = Instant::now();
        let content_id = "large_perf_test".to_string();
        let _id = manager.store_content(
            OfflineContentType::QuranText,
            content_id.clone(),
            data.clone(),
            metadata,
        ).await?;
        let storage_time = start.elapsed();

        // Measure retrieval time
        let start = Instant::now();
        let result = manager.get_content(&content_id).await?;
        let retrieval_time = start.elapsed();

        assert!(result.is_some());
        let retrieved_data = result.unwrap().data.unwrap();
        assert_eq!(retrieved_data, data);

        // Performance assertions (these are reasonable expectations)
        assert!(storage_time.as_millis() < 1000, "Storage should complete within 1 second");
        assert!(retrieval_time.as_millis() < 500, "Retrieval should complete within 500ms");

        println!("Large content storage time: {:?}", storage_time);
        println!("Large content retrieval time: {:?}", retrieval_time);
        println!("Original size: {} bytes", data.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_content_operations_performance() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let config = OfflineConfig::default();
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await?;

        let num_items = 100;
        let start = Instant::now();

        // Store multiple items
        for i in 0..num_items {
            let content_id = format!("perf_test_{}", i);
            let data = format!("Performance test data item {}", i).repeat(10).into_bytes();
            let metadata = ContentMetadata {
                title: format!("Performance Test {}", i),
                description: None,
                language: "en".to_string(),
                source: "test".to_string(),
                author: None,
                tags: Vec::new(),
                content_hash: "".to_string(),
                original_size: data.len(),
                compressed_size: data.len(),
                compression_ratio: 1.0,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            manager.store_content(
                OfflineContentType::CustomContent,
                content_id,
                data,
                metadata,
            ).await?;
        }

        let storage_time = start.elapsed();
        let avg_storage_time = storage_time.as_millis() / num_items as u128;

        // Retrieve all items
        let start = Instant::now();
        for i in 0..num_items {
            let content_id = format!("perf_test_{}", i);
            let result = manager.get_content(&content_id).await?;
            assert!(result.is_some());
        }
        let retrieval_time = start.elapsed();
        let avg_retrieval_time = retrieval_time.as_millis() / num_items as u128;

        // Performance assertions
        assert!(avg_storage_time < 50, "Average storage time should be under 50ms per item");
        assert!(avg_retrieval_time < 20, "Average retrieval time should be under 20ms per item");

        println!("Stored {} items in {:?} (avg: {}ms per item)", num_items, storage_time, avg_storage_time);
        println!("Retrieved {} items in {:?} (avg: {}ms per item)", num_items, retrieval_time, avg_retrieval_time);

        Ok(())
    }
}
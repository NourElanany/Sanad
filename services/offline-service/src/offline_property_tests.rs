use crate::models::*;
use crate::storage_manager::OfflineStorageManager;
use crate::service::{OfflineServiceBuilder};
use chrono::Utc;
use quickcheck::{quickcheck, Arbitrary, Gen, TestResult};
use quickcheck_macros::quickcheck;
use std::path::PathBuf;
use tempfile::TempDir;
use uuid::Uuid;

/// Property-based tests for offline mode functionality
/// **Validates: Requirements 11.3**

/// Test data generator for Islamic content
#[derive(Debug, Clone)]
struct IslamicContent {
    content_type: OfflineContentType,
    content_id: String,
    data: Vec<u8>,
    title: String,
}

impl Arbitrary for IslamicContent {
    fn arbitrary(g: &mut Gen) -> Self {
        let content_types = vec![
            OfflineContentType::QuranText,
            OfflineContentType::BasicTafsir,
            OfflineContentType::PrayerTimes,
            OfflineContentType::UserBookmarks,
            OfflineContentType::ReadingProgress,
            OfflineContentType::HadithCollection,
            OfflineContentType::IslamicStories,
        ];
        
        let content_type = g.choose(&content_types).unwrap().clone();
        let content_id = match content_type {
            OfflineContentType::QuranText => {
                let surah = (u8::arbitrary(g) % 114) + 1;
                let ayah = (u8::arbitrary(g) % 286) + 1;
                format!("quran:{}:{}", surah, ayah)
            }
            OfflineContentType::PrayerTimes => {
                let lat = (f64::arbitrary(g) % 180.0) - 90.0;
                let lng = (f64::arbitrary(g) % 360.0) - 180.0;
                format!("prayer_times:{}:{}:2024-01-01", lat, lng)
            }
            OfflineContentType::UserBookmarks => {
                format!("bookmarks:{}", Uuid::new_v4())
            }
            OfflineContentType::ReadingProgress => {
                format!("progress:{}", Uuid::new_v4())
            }
            _ => format!("{}:{}", content_type.to_string(), Uuid::new_v4()),
        };

        // Generate Islamic content data
        let islamic_texts = vec![
            "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
            "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ",
            "الرَّحْمَٰنِ الرَّحِيمِ",
            "مَالِكِ يَوْمِ الدِّينِ",
            "إِيَّاكَ نَعْبُدُ وَإِيَّاكَ نَسْتَعِينُ",
        ];
        
        let base_text = g.choose(&islamic_texts).unwrap();
        let repeat_count = (usize::arbitrary(g) % 100) + 1;
        let data = base_text.repeat(repeat_count).into_bytes();
        
        let title = format!("Islamic Content {}", Uuid::new_v4());

        Self {
            content_type,
            content_id,
            data,
            title,
        }
    }
}

impl ToString for OfflineContentType {
    fn to_string(&self) -> String {
        match self {
            OfflineContentType::QuranText => "quran_text".to_string(),
            OfflineContentType::BasicTafsir => "basic_tafsir".to_string(),
            OfflineContentType::PrayerTimes => "prayer_times".to_string(),
            OfflineContentType::UserBookmarks => "user_bookmarks".to_string(),
            OfflineContentType::ReadingProgress => "reading_progress".to_string(),
            OfflineContentType::HadithCollection => "hadith_collection".to_string(),
            OfflineContentType::IslamicStories => "islamic_stories".to_string(),
            _ => "custom_content".to_string(),
        }
    }
}

/// Property 1: Content Integrity - Any content stored offline must be retrievable with identical data
/// **Feature: islamic-app-comprehensive, Property 13: أداء النظام**
#[quickcheck]
fn prop_offline_content_integrity(islamic_content: IslamicContent) -> TestResult {
    if islamic_content.data.is_empty() {
        return TestResult::discard();
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        let temp_dir = TempDir::new().unwrap();
        let config = OfflineConfig::default();
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await.unwrap();

        let metadata = ContentMetadata {
            title: islamic_content.title.clone(),
            description: None,
            language: "ar".to_string(),
            source: "test".to_string(),
            author: None,
            tags: Vec::new(),
            content_hash: "".to_string(),
            original_size: islamic_content.data.len(),
            compressed_size: islamic_content.data.len(),
            compression_ratio: 1.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store content offline
        let _id = manager.store_content(
            islamic_content.content_type.clone(),
            islamic_content.content_id.clone(),
            islamic_content.data.clone(),
            metadata,
        ).await.unwrap();

        // Retrieve content
        let result = manager.get_content(&islamic_content.content_id).await.unwrap();
        assert!(result.is_some(), "Content must be retrievable after storage");
        
        let offline_result = result.unwrap();
        assert!(offline_result.success, "Retrieval must be successful");
        assert!(offline_result.from_cache, "Content must come from offline cache");
        
        let retrieved_data = offline_result.data.unwrap();
        assert_eq!(retrieved_data, islamic_content.data, "Retrieved data must be identical to stored data");
    });

    TestResult::passed()
}

/// Property 2: Compression Preservation - Compressed content must decompress to original data
/// **Feature: islamic-app-comprehensive, Property 13: أداء النظام**
#[quickcheck]
fn prop_offline_compression_preservation(islamic_content: IslamicContent) -> TestResult {
    if islamic_content.data.len() < 100 {
        return TestResult::discard(); // Skip small data that might not compress well
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        let temp_dir = TempDir::new().unwrap();
        let mut config = OfflineConfig::default();
        config.enable_compression = true;
        config.default_compression = CompressionAlgorithm::Lz4;
        
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await.unwrap();

        let metadata = ContentMetadata {
            title: islamic_content.title.clone(),
            description: None,
            language: "ar".to_string(),
            source: "test".to_string(),
            author: None,
            tags: Vec::new(),
            content_hash: "".to_string(),
            original_size: islamic_content.data.len(),
            compressed_size: islamic_content.data.len(),
            compression_ratio: 1.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store content with compression enabled
        let _id = manager.store_content(
            islamic_content.content_type.clone(),
            islamic_content.content_id.clone(),
            islamic_content.data.clone(),
            metadata,
        ).await.unwrap();

        // Retrieve and verify data integrity after compression/decompression
        let result = manager.get_content(&islamic_content.content_id).await.unwrap();
        assert!(result.is_some(), "Compressed content must be retrievable");
        
        let offline_result = result.unwrap();
        assert!(offline_result.success, "Compressed content retrieval must be successful");
        
        let retrieved_data = offline_result.data.unwrap();
        assert_eq!(retrieved_data, islamic_content.data, "Decompressed data must match original data exactly");
    });

    TestResult::passed()
}

/// Property 3: Priority-Based Storage - Essential Islamic content must never be removed during cleanup
/// **Feature: islamic-app-comprehensive, Property 13: أداء النظام**
#[quickcheck]
fn prop_essential_content_preservation(essential_content_count: u8) -> TestResult {
    let count = (essential_content_count % 10) + 1; // 1-10 items
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        let temp_dir = TempDir::new().unwrap();
        let mut config = OfflineConfig::default();
        config.max_storage_mb = 1; // Very small limit to force cleanup
        
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await.unwrap();

        let essential_types = vec![
            OfflineContentType::QuranText,
            OfflineContentType::BasicTafsir,
            OfflineContentType::PrayerTimes,
        ];

        let mut essential_content_ids = Vec::new();

        // Store essential Islamic content
        for i in 0..count {
            let content_type = essential_types[i as usize % essential_types.len()].clone();
            let content_id = format!("essential_{}_{}", content_type.to_string(), i);
            let data = format!("Essential Islamic content {}", i).repeat(1000).into_bytes();
            
            let metadata = ContentMetadata {
                title: format!("Essential Content {}", i),
                description: None,
                language: "ar".to_string(),
                source: "quran".to_string(),
                author: None,
                tags: vec!["essential".to_string()],
                content_hash: "".to_string(),
                original_size: data.len(),
                compressed_size: data.len(),
                compression_ratio: 1.0,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            manager.store_content(content_type, content_id.clone(), data, metadata).await.unwrap();
            essential_content_ids.push(content_id);
        }

        // Store some non-essential content to trigger cleanup
        for i in 0..5 {
            let content_id = format!("non_essential_{}", i);
            let data = format!("Non-essential content {}", i).repeat(2000).into_bytes();
            
            let metadata = ContentMetadata {
                title: format!("Non-Essential Content {}", i),
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

            manager.store_content(OfflineContentType::AudioRecordings, content_id, data, metadata).await.unwrap();
        }

        // Trigger cleanup
        let _cleaned_count = manager.cleanup_storage(None).await.unwrap();

        // Verify all essential content is still available
        for content_id in essential_content_ids {
            let result = manager.get_content(&content_id).await.unwrap();
            assert!(result.is_some(), "Essential Islamic content must never be removed during cleanup: {}", content_id);
            
            let offline_result = result.unwrap();
            assert!(offline_result.success, "Essential content must remain accessible after cleanup");
        }
    });

    TestResult::passed()
}

/// Property 4: Sync Status Consistency - Content sync status must be accurately tracked
/// **Feature: islamic-app-comprehensive, Property 11: حفظ التقدم والاستعادة**
#[quickcheck]
fn prop_sync_status_consistency(islamic_content: IslamicContent) -> TestResult {
    if islamic_content.data.is_empty() {
        return TestResult::discard();
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        let temp_dir = TempDir::new().unwrap();
        let config = OfflineConfig::default();
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await.unwrap();

        let metadata = ContentMetadata {
            title: islamic_content.title.clone(),
            description: None,
            language: "ar".to_string(),
            source: "test".to_string(),
            author: None,
            tags: Vec::new(),
            content_hash: "".to_string(),
            original_size: islamic_content.data.len(),
            compressed_size: islamic_content.data.len(),
            compression_ratio: 1.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store content (initially synced)
        let _id = manager.store_content(
            islamic_content.content_type.clone(),
            islamic_content.content_id.clone(),
            islamic_content.data.clone(),
            metadata,
        ).await.unwrap();

        // Verify initial sync status
        let result = manager.get_content(&islamic_content.content_id).await.unwrap();
        assert!(result.is_some());
        let initial_result = result.unwrap();
        assert!(!initial_result.sync_pending, "Newly stored content should not have pending sync");

        // Update sync status to pending
        manager.update_sync_status(&islamic_content.content_id, SyncStatus::PendingUpload, None).await.unwrap();

        // Verify sync status is updated
        let result = manager.get_content(&islamic_content.content_id).await.unwrap();
        assert!(result.is_some());
        let updated_result = result.unwrap();
        assert!(updated_result.sync_pending, "Content with pending upload should show sync_pending=true");

        // Mark as synced
        manager.update_sync_status(&islamic_content.content_id, SyncStatus::Synced, None).await.unwrap();

        // Verify sync status is cleared
        let result = manager.get_content(&islamic_content.content_id).await.unwrap();
        assert!(result.is_some());
        let synced_result = result.unwrap();
        assert!(!synced_result.sync_pending, "Synced content should not have pending sync");
    });

    TestResult::passed()
}

/// Property 5: Storage Space Management - System must respect storage limits and free space requirements
/// **Feature: islamic-app-comprehensive, Property 13: أداء النظام**
#[quickcheck]
fn prop_storage_space_management(content_count: u8) -> TestResult {
    let count = (content_count % 20) + 5; // 5-24 items
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        let temp_dir = TempDir::new().unwrap();
        let config = OfflineConfig::default();
        let max_storage_mb = config.max_storage_mb; // Store before moving
        
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await.unwrap();

        // Store content until we approach the limit
        for i in 0..count {
            let content_id = format!("space_test_{}", i);
            let data = format!("Space management test data {}", i).repeat(500).into_bytes(); // ~25KB per item
            
            let metadata = ContentMetadata {
                title: format!("Space Test {}", i),
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

            // This should either succeed or trigger cleanup
            let result = manager.store_content(
                OfflineContentType::CustomContent,
                content_id,
                data,
                metadata,
            ).await;

            // Storage should always succeed (either directly or after cleanup)
            assert!(result.is_ok(), "Storage should succeed even when approaching limits due to automatic cleanup");
        }

        // Verify storage statistics are within bounds
        let stats = manager.get_statistics();
        assert!(stats.total_size_mb <= max_storage_mb as f64, 
                "Total storage should not exceed configured maximum");
        assert!(stats.available_space_mb >= 0.0, 
                "Available space should never be negative");
    });

    TestResult::passed()
}

/// Property 6: Content Expiry Handling - Expired content must be properly handled
/// **Feature: islamic-app-comprehensive, Property 13: أداء النظام**
#[test]
fn prop_content_expiry_handling() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        let temp_dir = TempDir::new().unwrap();
        let config = OfflineConfig::default();
        let mut manager = OfflineStorageManager::new(temp_dir.path().to_path_buf(), config).await.unwrap();

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

        // Store content
        let _id = manager.store_content(
            OfflineContentType::PrayerTimes,
            content_id.clone(),
            data.clone(),
            metadata,
        ).await.unwrap();

        // Verify content is initially available
        let result = manager.get_content(&content_id).await.unwrap();
        assert!(result.is_some(), "Prayer times should be available immediately after storage");

        // Check that expiry is set for prayer times
        let stored_content = manager.list_content(Some(OfflineContentType::PrayerTimes));
        assert!(!stored_content.is_empty());
        assert!(stored_content[0].storage_info.expires_at.is_some(), 
                "Prayer times must have expiry time set");

        // Verify expiry time is reasonable (within 24 hours)
        let expiry = stored_content[0].storage_info.expires_at.unwrap();
        let now = Utc::now();
        let time_diff = expiry - now;
        assert!(time_diff.num_hours() <= 24, 
                "Prayer times expiry should be within 24 hours");
        assert!(time_diff.num_hours() > 0, 
                "Prayer times expiry should be in the future");
    });
}

/// Property 7: Islamic Content Priority - Islamic religious content must have appropriate priority levels
/// **Feature: islamic-app-comprehensive, Property 1: سلامة المحتوى الإسلامي**
#[test]
fn prop_islamic_content_priority() {
    // Test that essential Islamic content has the highest priority
    assert_eq!(OfflineContentType::QuranText.default_priority(), StoragePriority::Essential,
               "Quran text must have essential priority");
    assert_eq!(OfflineContentType::BasicTafsir.default_priority(), StoragePriority::Essential,
               "Basic Tafsir must have essential priority");
    assert_eq!(OfflineContentType::PrayerTimes.default_priority(), StoragePriority::Essential,
               "Prayer times must have essential priority");

    // Test that user Islamic data has high priority
    assert_eq!(OfflineContentType::UserBookmarks.default_priority(), StoragePriority::High,
               "User bookmarks must have high priority");
    assert_eq!(OfflineContentType::ReadingProgress.default_priority(), StoragePriority::High,
               "Reading progress must have high priority");
    assert_eq!(OfflineContentType::PersonalNotes.default_priority(), StoragePriority::High,
               "Personal notes must have high priority");
    assert_eq!(OfflineContentType::FavoriteHadith.default_priority(), StoragePriority::High,
               "Favorite Hadith must have high priority");

    // Test that Islamic educational content has medium priority
    assert_eq!(OfflineContentType::HadithCollection.default_priority(), StoragePriority::Medium,
               "Hadith collection must have medium priority");
    assert_eq!(OfflineContentType::IslamicStories.default_priority(), StoragePriority::Medium,
               "Islamic stories must have medium priority");

    // Test priority ordering
    assert!(StoragePriority::Essential > StoragePriority::High,
            "Essential priority must be higher than high priority");
    assert!(StoragePriority::High > StoragePriority::Medium,
            "High priority must be higher than medium priority");
    assert!(StoragePriority::Medium > StoragePriority::Low,
            "Medium priority must be higher than low priority");
}

/// Property 8: Offline Availability - Essential Islamic content must be available offline
/// **Feature: islamic-app-comprehensive, Property 13: أداء النظام**
#[quickcheck]
fn prop_offline_availability(quran_surah: u8, quran_ayah: u8) -> TestResult {
    let surah = (quran_surah % 114) + 1; // 1-114
    let ayah = (quran_ayah % 286) + 1;   // 1-286 (max in Al-Baqarah)
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        let temp_dir = TempDir::new().unwrap();
        let config = OfflineConfig::default();
        
        let service = OfflineServiceBuilder::new()
            .storage_path(temp_dir.path().to_path_buf())
            .config(config)
            .server_url("http://localhost:8080".to_string())
            .build()
            .await.unwrap();

        // Store Quran content
        let quran_text = format!("Quran {}:{} - Test verse content", surah, ayah);
        let content_id = format!("quran:{}:{}", surah, ayah);
        
        let _id = service.store_content(
            OfflineContentType::QuranText,
            content_id.clone(),
            quran_text.as_bytes().to_vec(),
            format!("Surah {} Ayah {}", surah, ayah),
            None,
        ).await.unwrap();

        // Retrieve content using offline service (simulating offline mode)
        let result = service.get_quran_content(surah as u16, Some(ayah as u16)).await.unwrap();
        assert!(result.is_some(), "Quran content must be available offline");
        
        let quran_result = result.unwrap();
        assert!(quran_result.success, "Offline Quran retrieval must be successful");
        assert!(quran_result.from_cache, "Quran content must come from offline cache");
        assert_eq!(quran_result.data.unwrap(), quran_text, "Offline Quran content must match stored content");
    });

    TestResult::passed()
}

#[cfg(test)]
mod test_runner {
    use super::*;

    #[test]
    fn run_all_offline_property_tests() {
        println!("Running offline mode property-based tests...");
        
        // Run quickcheck tests
        quickcheck(prop_offline_content_integrity as fn(IslamicContent) -> TestResult);
        quickcheck(prop_offline_compression_preservation as fn(IslamicContent) -> TestResult);
        quickcheck(prop_essential_content_preservation as fn(u8) -> TestResult);
        quickcheck(prop_sync_status_consistency as fn(IslamicContent) -> TestResult);
        quickcheck(prop_storage_space_management as fn(u8) -> TestResult);
        quickcheck(prop_offline_availability as fn(u8, u8) -> TestResult);
        
        // Run regular tests
        prop_content_expiry_handling();
        prop_islamic_content_priority();
        
        println!("All offline mode property tests passed!");
    }
}
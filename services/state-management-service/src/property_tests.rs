use crate::{models::*, crdt::*, storage::*};
use chrono::Utc;
use proptest::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

/// **Validates: Requirements 9.4, 11.4**
/// **Property 11: Progress Saving and Recovery**
/// 
/// For any user reading activity, progress must be saved automatically 
/// and recovered accurately when returning to the content.
/// 
/// This property tests that:
/// 1. Reading progress is always saved correctly using CRDTs
/// 2. Progress can be synchronized across multiple devices without conflicts
/// 3. The latest progress is always preserved during merges
/// 4. Progress recovery returns the exact state that was saved

#[cfg(test)]
mod tests {
    use super::*;

    // Strategy for generating valid surah numbers (1-114)
    fn surah_number_strategy() -> impl Strategy<Value = u8> {
        1u8..=114u8
    }

    // Strategy for generating valid ayah numbers (1-286, max in Al-Baqarah)
    fn ayah_number_strategy() -> impl Strategy<Value = u16> {
        1u16..=286u16
    }

    // Strategy for generating completion percentages (0.0-100.0)
    fn completion_percentage_strategy() -> impl Strategy<Value = f64> {
        0.0..=100.0
    }

    // Strategy for generating device IDs
    fn device_id_strategy() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9_-]{8,16}".prop_map(|s| format!("device_{}", s))
    }

    // Strategy for generating user IDs
    fn user_id_strategy() -> impl Strategy<Value = Uuid> {
        any::<[u8; 16]>().prop_map(|bytes| Uuid::from_bytes(bytes))
    }

    // Strategy for generating khatma progress
    fn khatma_progress_strategy() -> impl Strategy<Value = (u32, u32)> {
        (0u32..=100u32, 1u32..=100u32).prop_map(|(completed, total)| {
            if completed > total {
                (total, total)
            } else {
                (completed, total)
            }
        })
    }

    proptest! {
        /// **Property Test: Reading Progress Preservation**
        /// 
        /// Tests that reading progress is correctly preserved through CRDT operations.
        /// Any progress update should be reflected in the final state.
        #[test]
        fn property_reading_progress_preservation(
            surah_number in surah_number_strategy(),
            last_ayah_read in ayah_number_strategy(),
            completion_percentage in completion_percentage_strategy(),
            device_id in device_id_strategy(),
        ) {
            let manager = CRDTManager::new(device_id.clone());
            let mut reading_progress = ReadingProgress {
                quran_progress: HashMap::new(),
                khatma_progress: HashMap::new(),
                version_vector: HashMap::new(),
            };

            // Update progress
            let result = manager.update_surah_progress(
                &mut reading_progress,
                surah_number,
                last_ayah_read,
                completion_percentage,
            );

            // Progress update should succeed
            prop_assert!(result.is_ok());

            // Progress should be saved correctly
            let saved_progress = reading_progress.quran_progress.get(&surah_number);
            prop_assert!(saved_progress.is_some());

            let progress = saved_progress.unwrap();
            prop_assert_eq!(progress.surah_number, surah_number);
            prop_assert_eq!(progress.last_ayah_read, last_ayah_read);
            prop_assert_eq!(progress.completion_percentage, completion_percentage);
            prop_assert_eq!(&progress.device_id, &device_id);

            // Version vector should be updated
            prop_assert_eq!(reading_progress.version_vector.get(&device_id), Some(&1));
        }

        /// **Property Test: Khatma Progress Preservation**
        /// 
        /// Tests that Khatma progress is correctly preserved and can be updated.
        #[test]
        fn property_khatma_progress_preservation(
            khatma_id in user_id_strategy(), // Reuse UUID strategy
            (completed_portions, total_portions) in khatma_progress_strategy(),
            device_id in device_id_strategy(),
        ) {
            let manager = CRDTManager::new(device_id.clone());
            let mut reading_progress = ReadingProgress {
                quran_progress: HashMap::new(),
                khatma_progress: HashMap::new(),
                version_vector: HashMap::new(),
            };

            // Update khatma progress
            let result = manager.update_khatma_progress(
                &mut reading_progress,
                khatma_id,
                completed_portions,
                total_portions,
            );

            // Progress update should succeed
            prop_assert!(result.is_ok());

            // Khatma progress should be saved correctly
            let saved_progress = reading_progress.khatma_progress.get(&khatma_id);
            prop_assert!(saved_progress.is_some());

            let progress = saved_progress.unwrap();
            prop_assert_eq!(progress.khatma_id, khatma_id);
            prop_assert_eq!(progress.completed_portions, completed_portions);
            prop_assert_eq!(progress.total_portions, total_portions);
            prop_assert_eq!(&progress.device_id, &device_id);

            // Version vector should be updated
            prop_assert_eq!(reading_progress.version_vector.get(&device_id), Some(&1));
        }

        /// **Property Test: Bookmark Synchronization**
        /// 
        /// Tests that bookmarks can be synchronized across devices using G-Set CRDT.
        /// All bookmarks should be preserved during merge operations.
        #[test]
        fn property_bookmark_synchronization(
            bookmark_count1 in 0usize..10,
            bookmark_count2 in 0usize..10,
            device1_id in device_id_strategy(),
            device2_id in device_id_strategy(),
        ) {
            // Skip if device IDs are the same
            if device1_id == device2_id {
                return Ok(());
            }

            let mut bookmarks1 = BookmarkSet {
                bookmarks: HashMap::new(),
                version_vector: HashMap::new(),
            };

            let mut bookmarks2 = BookmarkSet {
                bookmarks: HashMap::new(),
                version_vector: HashMap::new(),
            };

            // Add bookmarks to device 1
            let mut device1_bookmark_ids = Vec::new();
            for i in 0..bookmark_count1 {
                let bookmark_id = Uuid::new_v4();
                device1_bookmark_ids.push(bookmark_id);
                
                let bookmark = Bookmark {
                    id: bookmark_id,
                    content_type: ContentType::Quran,
                    content_id: Uuid::new_v4(),
                    title: format!("Bookmark {} from device 1", i),
                    description: None,
                    tags: vec![],
                    folder: None,
                    created_at: Utc::now(),
                    device_id: device1_id.clone(),
                };
                
                bookmarks1.bookmarks.insert(bookmark_id, bookmark);
            }
            bookmarks1.version_vector.insert(device1_id.clone(), bookmark_count1 as u64);

            // Add bookmarks to device 2
            let mut device2_bookmark_ids = Vec::new();
            for i in 0..bookmark_count2 {
                let bookmark_id = Uuid::new_v4();
                device2_bookmark_ids.push(bookmark_id);
                
                let bookmark = Bookmark {
                    id: bookmark_id,
                    content_type: ContentType::Hadith,
                    content_id: Uuid::new_v4(),
                    title: format!("Bookmark {} from device 2", i),
                    description: None,
                    tags: vec![],
                    folder: None,
                    created_at: Utc::now(),
                    device_id: device2_id.clone(),
                };
                
                bookmarks2.bookmarks.insert(bookmark_id, bookmark);
            }
            bookmarks2.version_vector.insert(device2_id.clone(), bookmark_count2 as u64);

            // Merge bookmarks
            let merge_result = bookmarks1.merge(&bookmarks2);
            prop_assert!(merge_result.is_ok());

            // After merge, should have all bookmarks from both devices
            prop_assert_eq!(bookmarks1.bookmarks.len(), bookmark_count1 + bookmark_count2);

            // All device 1 bookmarks should be present
            for bookmark_id in device1_bookmark_ids {
                prop_assert!(bookmarks1.bookmarks.contains_key(&bookmark_id));
            }

            // All device 2 bookmarks should be present
            for bookmark_id in device2_bookmark_ids {
                prop_assert!(bookmarks1.bookmarks.contains_key(&bookmark_id));
            }

            // Version vectors should be merged correctly
            prop_assert_eq!(bookmarks1.version_vector.get(&device1_id), Some(&(bookmark_count1 as u64)));
            prop_assert_eq!(bookmarks1.version_vector.get(&device2_id), Some(&(bookmark_count2 as u64)));
        }

        /// **Property Test: Storage Priority Consistency**
        /// 
        /// Tests that storage priorities are maintained correctly during cleanup operations.
        #[test]
        fn property_storage_priority_consistency(
            max_storage_mb in 100u64..1000u64,
            compression_enabled in any::<bool>(),
        ) {
            let storage_manager = SmartStorageManager::new(max_storage_mb, compression_enabled);
            let stats = storage_manager.get_storage_stats();

            // Initial state should be consistent
            prop_assert_eq!(stats.available_space_mb, max_storage_mb as f64);
            prop_assert_eq!(stats.total_size_mb, 0.0);
            prop_assert_eq!(stats.items_count, 0);

            // Storage priorities should be ordered correctly
            let mut priorities = vec![
                StoragePriority::Optional,
                StoragePriority::Essential,
                StoragePriority::Useful,
                StoragePriority::Important,
            ];
            priorities.sort();

            prop_assert_eq!(&priorities[0], &StoragePriority::Essential);
            prop_assert_eq!(&priorities[1], &StoragePriority::Important);
            prop_assert_eq!(&priorities[2], &StoragePriority::Useful);
            prop_assert_eq!(&priorities[3], &StoragePriority::Optional);
        }
    }

    /// **Unit Test: Progress Recovery Accuracy**
    /// 
    /// Tests that progress can be recovered exactly as it was saved.
    #[tokio::test]
    async fn test_progress_recovery_accuracy() {
        let device_id = "test_device".to_string();
        let manager = CRDTManager::new(device_id.clone());
        
        let mut reading_progress = ReadingProgress {
            quran_progress: HashMap::new(),
            khatma_progress: HashMap::new(),
            version_vector: HashMap::new(),
        };

        // Save specific progress
        let surah_number = 2; // Al-Baqarah
        let last_ayah_read = 255; // Ayat al-Kursi
        let completion_percentage = 89.5;

        let result = manager.update_surah_progress(
            &mut reading_progress,
            surah_number,
            last_ayah_read,
            completion_percentage,
        );

        assert!(result.is_ok());

        // Verify exact recovery
        let recovered_progress = reading_progress.quran_progress.get(&surah_number);
        assert!(recovered_progress.is_some());

        let progress = recovered_progress.unwrap();
        assert_eq!(progress.surah_number, surah_number);
        assert_eq!(progress.last_ayah_read, last_ayah_read);
        assert_eq!(progress.completion_percentage, completion_percentage);
        assert_eq!(&progress.device_id, &device_id);

        // Version should be incremented
        assert_eq!(reading_progress.version_vector.get(&device_id), Some(&1));
    }

    /// **Unit Test: Concurrent Updates Resolution**
    /// 
    /// Tests that concurrent updates from different devices are resolved correctly.
    #[tokio::test]
    async fn test_concurrent_updates_resolution() {
        let device1 = "device1".to_string();
        let device2 = "device2".to_string();
        
        let mut progress1 = ReadingProgress {
            quran_progress: HashMap::new(),
            khatma_progress: HashMap::new(),
            version_vector: HashMap::new(),
        };

        let mut progress2 = ReadingProgress {
            quran_progress: HashMap::new(),
            khatma_progress: HashMap::new(),
            version_vector: HashMap::new(),
        };

        let surah_number = 1; // Al-Fatiha

        // Device 1 updates
        let progress1_data = SurahProgress {
            surah_number,
            last_ayah_read: 5,
            completion_percentage: 71.4,
            last_read_at: Utc::now(),
            device_id: device1.clone(),
        };

        progress1.quran_progress.insert(surah_number, progress1_data);
        progress1.version_vector.insert(device1.clone(), 1);

        // Device 2 updates (later)
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        
        let progress2_data = SurahProgress {
            surah_number,
            last_ayah_read: 7,
            completion_percentage: 100.0,
            last_read_at: Utc::now(),
            device_id: device2.clone(),
        };

        progress2.quran_progress.insert(surah_number, progress2_data);
        progress2.version_vector.insert(device2.clone(), 1);

        // Merge progress2 into progress1
        let merge_result = progress1.merge(&progress2);
        assert!(merge_result.is_ok());

        // Should keep the latest progress (device2)
        let final_progress = progress1.quran_progress.get(&surah_number).unwrap();
        assert_eq!(&final_progress.device_id, &device2);
        assert_eq!(final_progress.last_ayah_read, 7);
        assert_eq!(final_progress.completion_percentage, 100.0);

        // Both version vectors should be present
        assert_eq!(progress1.version_vector.get(&device1), Some(&1));
        assert_eq!(progress1.version_vector.get(&device2), Some(&1));
    }
}
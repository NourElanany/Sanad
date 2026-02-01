use crate::{models::*, crdt::*};
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lww_register_merge() {
        let device1 = "device1".to_string();
        let device2 = "device2".to_string();
        
        let mut register1 = LWWRegister::new("value1".to_string(), device1.clone());
        let register2 = LWWRegister::new("value2".to_string(), device2.clone());
        
        // register2 should win if it has a later timestamp
        std::thread::sleep(std::time::Duration::from_millis(1));
        let register2_later = LWWRegister::new("value2_later".to_string(), device2.clone());
        
        register1.merge(&register2_later);
        assert_eq!(register1.value, "value2_later");
    }

    #[test]
    fn test_bookmark_set_merge() {
        let device1 = "device1".to_string();
        let device2 = "device2".to_string();
        
        let mut bookmarks1 = BookmarkSet {
            bookmarks: HashMap::new(),
            version_vector: HashMap::new(),
        };
        
        let mut bookmarks2 = BookmarkSet {
            bookmarks: HashMap::new(),
            version_vector: HashMap::new(),
        };
        
        // Add different bookmarks to each set
        let bookmark1 = Bookmark {
            id: Uuid::new_v4(),
            content_type: ContentType::Quran,
            content_id: Uuid::new_v4(),
            title: "Bookmark 1".to_string(),
            description: None,
            tags: vec![],
            folder: None,
            created_at: Utc::now(),
            device_id: device1.clone(),
        };
        
        let bookmark2 = Bookmark {
            id: Uuid::new_v4(),
            content_type: ContentType::Hadith,
            content_id: Uuid::new_v4(),
            title: "Bookmark 2".to_string(),
            description: None,
            tags: vec![],
            folder: None,
            created_at: Utc::now(),
            device_id: device2.clone(),
        };
        
        bookmarks1.bookmarks.insert(bookmark1.id, bookmark1.clone());
        bookmarks1.version_vector.insert(device1.clone(), 1);
        
        bookmarks2.bookmarks.insert(bookmark2.id, bookmark2.clone());
        bookmarks2.version_vector.insert(device2.clone(), 1);
        
        // Merge bookmarks2 into bookmarks1
        bookmarks1.merge(&bookmarks2).unwrap();
        
        // Both bookmarks should be present
        assert_eq!(bookmarks1.bookmarks.len(), 2);
        assert!(bookmarks1.bookmarks.contains_key(&bookmark1.id));
        assert!(bookmarks1.bookmarks.contains_key(&bookmark2.id));
        
        // Version vectors should be merged
        assert_eq!(bookmarks1.version_vector.get(&device1), Some(&1));
        assert_eq!(bookmarks1.version_vector.get(&device2), Some(&1));
    }

    #[test]
    fn test_reading_progress_merge() {
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
        
        // Add progress for same surah from different devices
        let surah_progress1 = SurahProgress {
            surah_number: 1,
            last_ayah_read: 5,
            completion_percentage: 50.0,
            last_read_at: Utc::now(),
            device_id: device1.clone(),
        };
        
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        let surah_progress2 = SurahProgress {
            surah_number: 1,
            last_ayah_read: 7,
            completion_percentage: 70.0,
            last_read_at: Utc::now(),
            device_id: device2.clone(),
        };
        
        progress1.quran_progress.insert(1, surah_progress1);
        progress1.version_vector.insert(device1.clone(), 1);
        
        progress2.quran_progress.insert(1, surah_progress2.clone());
        progress2.version_vector.insert(device2.clone(), 1);
        
        // Merge progress2 into progress1
        progress1.merge(&progress2).unwrap();
        
        // Should keep the progress with the latest timestamp (progress2)
        let merged_progress = progress1.quran_progress.get(&1).unwrap();
        assert_eq!(merged_progress.last_ayah_read, 7);
        assert_eq!(merged_progress.completion_percentage, 70.0);
        assert_eq!(merged_progress.device_id, device2);
    }

    #[test]
    fn test_crdt_manager_operations() {
        let device_id = "test_device".to_string();
        let manager = CRDTManager::new(device_id.clone());
        
        let mut bookmarks = BookmarkSet {
            bookmarks: HashMap::new(),
            version_vector: HashMap::new(),
        };
        
        let bookmark = Bookmark {
            id: Uuid::new_v4(),
            content_type: ContentType::Quran,
            content_id: Uuid::new_v4(),
            title: "Test Bookmark".to_string(),
            description: Some("Test Description".to_string()),
            tags: vec!["test".to_string()],
            folder: Some("Test Folder".to_string()),
            created_at: Utc::now(),
            device_id: device_id.clone(),
        };
        
        // Add bookmark using CRDT manager
        manager.add_bookmark(&mut bookmarks, bookmark.clone()).unwrap();
        
        // Verify bookmark was added
        assert_eq!(bookmarks.bookmarks.len(), 1);
        assert!(bookmarks.bookmarks.contains_key(&bookmark.id));
        
        // Verify version vector was updated
        assert_eq!(bookmarks.version_vector.get(&device_id), Some(&1));
    }

    #[test]
    fn test_personal_notes_merge() {
        let device1 = "device1".to_string();
        let device2 = "device2".to_string();
        
        let mut notes1 = PersonalNotes {
            notes: HashMap::new(),
            version_vector: HashMap::new(),
        };
        
        let mut notes2 = PersonalNotes {
            notes: HashMap::new(),
            version_vector: HashMap::new(),
        };
        
        let note_id = Uuid::new_v4();
        
        // Create same note on different devices with different timestamps
        let note1 = Note {
            id: note_id,
            content_type: ContentType::Quran,
            content_id: Uuid::new_v4(),
            text: "Original note".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            device_id: device1.clone(),
        };
        
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        let note2 = Note {
            id: note_id,
            content_type: ContentType::Quran,
            content_id: Uuid::new_v4(),
            text: "Updated note".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            device_id: device2.clone(),
        };
        
        notes1.notes.insert(note_id, note1);
        notes1.version_vector.insert(device1.clone(), 1);
        
        notes2.notes.insert(note_id, note2.clone());
        notes2.version_vector.insert(device2.clone(), 1);
        
        // Merge notes2 into notes1
        notes1.merge(&notes2).unwrap();
        
        // Should keep the note with the latest update timestamp
        let merged_note = notes1.notes.get(&note_id).unwrap();
        assert_eq!(merged_note.text, "Updated note");
        assert_eq!(merged_note.device_id, device2);
    }

    #[test]
    fn test_crdt_merge_personal_data() {
        let device_id = "test_device".to_string();
        let manager = CRDTManager::new(device_id.clone());
        
        // Create local user data
        let mut local_data = UserPersonalData {
            user_id: Uuid::new_v4(),
            bookmarks: BookmarkSet {
                bookmarks: HashMap::new(),
                version_vector: HashMap::new(),
            },
            reading_progress: ReadingProgress {
                quran_progress: HashMap::new(),
                khatma_progress: HashMap::new(),
                version_vector: HashMap::new(),
            },
            personal_notes: PersonalNotes {
                notes: HashMap::new(),
                version_vector: HashMap::new(),
            },
            preferences: UserPreferences {
                language: LWWRegister::new("ar".to_string(), device_id.clone()),
                theme: LWWRegister::new("light".to_string(), device_id.clone()),
                font_size: LWWRegister::new("medium".to_string(), device_id.clone()),
                prayer_calculation_method: LWWRegister::new("MuslimWorldLeague".to_string(), device_id.clone()),
                notification_settings: LWWRegister::new(
                    NotificationSettings {
                        prayer_reminders: true,
                        prayer_reminder_minutes: 15,
                        islamic_events: true,
                        khatma_reminders: true,
                        daily_verse: true,
                    },
                    device_id.clone()
                ),
                display_settings: LWWRegister::new(
                    DisplaySettings {
                        theme: "light".to_string(),
                        font_size: "medium".to_string(),
                        arabic_font: "Amiri".to_string(),
                        translation_font: "Arial".to_string(),
                    },
                    device_id.clone()
                ),
            },
            last_updated: Utc::now(),
        };
        
        // Create remote user data with different preferences
        let remote_device = "remote_device".to_string();
        let remote_data = UserPersonalData {
            user_id: local_data.user_id,
            bookmarks: BookmarkSet {
                bookmarks: HashMap::new(),
                version_vector: HashMap::new(),
            },
            reading_progress: ReadingProgress {
                quran_progress: HashMap::new(),
                khatma_progress: HashMap::new(),
                version_vector: HashMap::new(),
            },
            personal_notes: PersonalNotes {
                notes: HashMap::new(),
                version_vector: HashMap::new(),
            },
            preferences: UserPreferences {
                language: LWWRegister::new("en".to_string(), remote_device.clone()),
                theme: LWWRegister::new("dark".to_string(), remote_device.clone()),
                font_size: LWWRegister::new("large".to_string(), remote_device.clone()),
                prayer_calculation_method: LWWRegister::new("IslamicSocietyOfNorthAmerica".to_string(), remote_device.clone()),
                notification_settings: LWWRegister::new(
                    NotificationSettings {
                        prayer_reminders: false,
                        prayer_reminder_minutes: 10,
                        islamic_events: false,
                        khatma_reminders: false,
                        daily_verse: false,
                    },
                    remote_device.clone()
                ),
                display_settings: LWWRegister::new(
                    DisplaySettings {
                        theme: "dark".to_string(),
                        font_size: "large".to_string(),
                        arabic_font: "Noto".to_string(),
                        translation_font: "Times".to_string(),
                    },
                    remote_device.clone()
                ),
            },
            last_updated: Utc::now(),
        };
        
        // Merge remote data into local data
        let result = manager.merge_personal_data(&mut local_data, &remote_data).unwrap();
        
        // Verify merge was successful
        assert!(result.success);
        assert!(result.errors.is_empty());
        
        // Preferences should be merged using LWW-Register logic
        // The exact result depends on timestamps, but merge should complete without errors
        println!("Merge completed successfully with {} items synced", result.items_synced);
    }
}
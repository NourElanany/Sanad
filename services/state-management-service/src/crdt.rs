use crate::models::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// CRDT operations for personal data synchronization
pub trait CRDTOperations<T> {
    fn merge(&mut self, other: &T) -> Result<()>;
    fn is_concurrent(&self, other: &T) -> bool;
    fn get_version_vector(&self) -> &HashMap<String, u64>;
    fn increment_version(&mut self, device_id: &str);
}

/// G-Set CRDT implementation for bookmarks
impl CRDTOperations<BookmarkSet> for BookmarkSet {
    fn merge(&mut self, other: &BookmarkSet) -> Result<()> {
        // G-Set: Union of all bookmarks (grow-only set)
        for (id, bookmark) in &other.bookmarks {
            if !self.bookmarks.contains_key(id) {
                self.bookmarks.insert(*id, bookmark.clone());
            }
        }

        // Merge version vectors
        for (device_id, version) in &other.version_vector {
            let current_version = self.version_vector.get(device_id).unwrap_or(&0);
            self.version_vector.insert(
                device_id.clone(),
                (*version).max(*current_version),
            );
        }

        Ok(())
    }

    fn is_concurrent(&self, other: &BookmarkSet) -> bool {
        // Check if version vectors are concurrent
        let self_dominates = self.version_vector.iter().all(|(device, version)| {
            other.version_vector.get(device).map_or(true, |other_version| version >= other_version)
        });

        let other_dominates = other.version_vector.iter().all(|(device, version)| {
            self.version_vector.get(device).map_or(true, |self_version| version >= self_version)
        });

        !(self_dominates || other_dominates)
    }

    fn get_version_vector(&self) -> &HashMap<String, u64> {
        &self.version_vector
    }

    fn increment_version(&mut self, device_id: &str) {
        let current = self.version_vector.get(device_id).unwrap_or(&0);
        self.version_vector.insert(device_id.to_string(), current + 1);
    }
}

/// PN-Counter CRDT implementation for reading progress
impl CRDTOperations<ReadingProgress> for ReadingProgress {
    fn merge(&mut self, other: &ReadingProgress) -> Result<()> {
        // Merge Quran progress - take maximum progress for each surah
        for (surah_number, other_progress) in &other.quran_progress {
            match self.quran_progress.get_mut(surah_number) {
                Some(current_progress) => {
                    // Take the progress with the latest timestamp
                    if other_progress.last_read_at > current_progress.last_read_at {
                        *current_progress = other_progress.clone();
                    }
                }
                None => {
                    self.quran_progress.insert(*surah_number, other_progress.clone());
                }
            }
        }

        // Merge Khatma progress
        for (khatma_id, other_progress) in &other.khatma_progress {
            match self.khatma_progress.get_mut(khatma_id) {
                Some(current_progress) => {
                    // Take the maximum completed portions
                    if other_progress.completed_portions > current_progress.completed_portions ||
                       (other_progress.completed_portions == current_progress.completed_portions &&
                        other_progress.last_read_at > current_progress.last_read_at) {
                        *current_progress = other_progress.clone();
                    }
                }
                None => {
                    self.khatma_progress.insert(*khatma_id, other_progress.clone());
                }
            }
        }

        // Merge version vectors
        for (device_id, version) in &other.version_vector {
            let current_version = self.version_vector.get(device_id).unwrap_or(&0);
            self.version_vector.insert(
                device_id.clone(),
                (*version).max(*current_version),
            );
        }

        Ok(())
    }

    fn is_concurrent(&self, other: &ReadingProgress) -> bool {
        let self_dominates = self.version_vector.iter().all(|(device, version)| {
            other.version_vector.get(device).map_or(true, |other_version| version >= other_version)
        });

        let other_dominates = other.version_vector.iter().all(|(device, version)| {
            self.version_vector.get(device).map_or(true, |self_version| version >= self_version)
        });

        !(self_dominates || other_dominates)
    }

    fn get_version_vector(&self) -> &HashMap<String, u64> {
        &self.version_vector
    }

    fn increment_version(&mut self, device_id: &str) {
        let current = self.version_vector.get(device_id).unwrap_or(&0);
        self.version_vector.insert(device_id.to_string(), current + 1);
    }
}

/// RGA CRDT implementation for personal notes
impl CRDTOperations<PersonalNotes> for PersonalNotes {
    fn merge(&mut self, other: &PersonalNotes) -> Result<()> {
        // RGA: Merge notes, keeping the latest version of each note
        for (note_id, other_note) in &other.notes {
            match self.notes.get_mut(note_id) {
                Some(current_note) => {
                    // Keep the note with the latest update timestamp
                    if other_note.updated_at > current_note.updated_at {
                        *current_note = other_note.clone();
                    }
                }
                None => {
                    self.notes.insert(*note_id, other_note.clone());
                }
            }
        }

        // Merge version vectors
        for (device_id, version) in &other.version_vector {
            let current_version = self.version_vector.get(device_id).unwrap_or(&0);
            self.version_vector.insert(
                device_id.clone(),
                (*version).max(*current_version),
            );
        }

        Ok(())
    }

    fn is_concurrent(&self, other: &PersonalNotes) -> bool {
        let self_dominates = self.version_vector.iter().all(|(device, version)| {
            other.version_vector.get(device).map_or(true, |other_version| version >= other_version)
        });

        let other_dominates = other.version_vector.iter().all(|(device, version)| {
            self.version_vector.get(device).map_or(true, |self_version| version >= self_version)
        });

        !(self_dominates || other_dominates)
    }

    fn get_version_vector(&self) -> &HashMap<String, u64> {
        &self.version_vector
    }

    fn increment_version(&mut self, device_id: &str) {
        let current = self.version_vector.get(device_id).unwrap_or(&0);
        self.version_vector.insert(device_id.to_string(), current + 1);
    }
}

/// CRDT manager for coordinating all personal data CRDTs
#[derive(Debug, Clone)]
pub struct CRDTManager {
    device_id: String,
}

impl CRDTManager {
    pub fn new(device_id: String) -> Self {
        Self { device_id }
    }

    /// Add a new bookmark
    pub fn add_bookmark(&self, bookmark_set: &mut BookmarkSet, bookmark: Bookmark) -> Result<()> {
        bookmark_set.bookmarks.insert(bookmark.id, bookmark);
        bookmark_set.increment_version(&self.device_id);
        Ok(())
    }

    /// Update reading progress for a surah
    pub fn update_surah_progress(
        &self,
        reading_progress: &mut ReadingProgress,
        surah_number: u8,
        last_ayah_read: u16,
        completion_percentage: f64,
    ) -> Result<()> {
        let progress = SurahProgress {
            surah_number,
            last_ayah_read,
            completion_percentage,
            last_read_at: Utc::now(),
            device_id: self.device_id.clone(),
        };

        reading_progress.quran_progress.insert(surah_number, progress);
        reading_progress.increment_version(&self.device_id);
        Ok(())
    }

    /// Update khatma progress
    pub fn update_khatma_progress(
        &self,
        reading_progress: &mut ReadingProgress,
        khatma_id: Uuid,
        completed_portions: u32,
        total_portions: u32,
    ) -> Result<()> {
        let progress = KhatmaProgress {
            khatma_id,
            completed_portions,
            total_portions,
            last_read_at: Utc::now(),
            device_id: self.device_id.clone(),
        };

        reading_progress.khatma_progress.insert(khatma_id, progress);
        reading_progress.increment_version(&self.device_id);
        Ok(())
    }

    /// Add or update a personal note
    pub fn update_note(
        &self,
        personal_notes: &mut PersonalNotes,
        note_id: Uuid,
        content_type: ContentType,
        content_id: Uuid,
        text: String,
    ) -> Result<()> {
        let now = Utc::now();
        let note = Note {
            id: note_id,
            content_type,
            content_id,
            text,
            created_at: personal_notes.notes.get(&note_id)
                .map(|n| n.created_at)
                .unwrap_or(now),
            updated_at: now,
            device_id: self.device_id.clone(),
        };

        personal_notes.notes.insert(note_id, note);
        personal_notes.increment_version(&self.device_id);
        Ok(())
    }

    /// Update user preference using LWW-Register
    pub fn update_preference<T: Clone>(
        &self,
        register: &mut LWWRegister<T>,
        new_value: T,
    ) -> Result<()> {
        register.update(new_value, self.device_id.clone());
        Ok(())
    }

    /// Merge two UserPersonalData instances
    pub fn merge_personal_data(
        &self,
        local: &mut UserPersonalData,
        remote: &UserPersonalData,
    ) -> Result<SyncResult> {
        let start_time = std::time::Instant::now();
        let mut conflicts_resolved = 0;
        let mut items_synced = 0;
        let mut errors = Vec::new();

        // Merge bookmarks
        if let Err(e) = local.bookmarks.merge(&remote.bookmarks) {
            errors.push(format!("Failed to merge bookmarks: {}", e));
        } else {
            items_synced += remote.bookmarks.bookmarks.len() as u32;
        }

        // Merge reading progress
        if let Err(e) = local.reading_progress.merge(&remote.reading_progress) {
            errors.push(format!("Failed to merge reading progress: {}", e));
        } else {
            items_synced += (remote.reading_progress.quran_progress.len() + 
                           remote.reading_progress.khatma_progress.len()) as u32;
        }

        // Merge personal notes
        if let Err(e) = local.personal_notes.merge(&remote.personal_notes) {
            errors.push(format!("Failed to merge personal notes: {}", e));
        } else {
            items_synced += remote.personal_notes.notes.len() as u32;
        }

        // Merge preferences using LWW-Register
        local.preferences.language.merge(&remote.preferences.language);
        local.preferences.theme.merge(&remote.preferences.theme);
        local.preferences.font_size.merge(&remote.preferences.font_size);
        local.preferences.prayer_calculation_method.merge(&remote.preferences.prayer_calculation_method);
        local.preferences.notification_settings.merge(&remote.preferences.notification_settings);
        local.preferences.display_settings.merge(&remote.preferences.display_settings);

        // Update last updated timestamp
        local.last_updated = Utc::now();

        let sync_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(SyncResult {
            success: errors.is_empty(),
            conflicts_resolved,
            items_synced,
            sync_time_ms,
            errors,
        })
    }
}
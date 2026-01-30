use anyhow::{Result, Context};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{info, error, warn};
use shared::{Reciter, ReferenceRecording, RecitationStyle, AudioRecording, AudioFormat};
use uuid::Uuid;
use chrono::Utc;

/// Manager for reference recordings and reciters database
pub struct ReferenceManager {
    reciters: HashMap<Uuid, Reciter>,
    reference_recordings: HashMap<(u8, u16), Vec<ReferenceRecording>>, // (surah, ayah) -> recordings
    reference_audio_path: PathBuf,
}

impl ReferenceManager {
    /// Create a new reference manager
    pub fn new<P: AsRef<Path>>(reference_audio_path: P) -> Result<Self> {
        let reference_audio_path = reference_audio_path.as_ref().to_path_buf();
        
        // Ensure reference audio directory exists
        std::fs::create_dir_all(&reference_audio_path)
            .context("Failed to create reference audio directory")?;
        
        let mut manager = Self {
            reciters: HashMap::new(),
            reference_recordings: HashMap::new(),
            reference_audio_path,
        };
        
        // Initialize with some well-known reciters
        manager.initialize_default_reciters()?;
        
        info!("Reference manager initialized with {} reciters", manager.reciters.len());
        Ok(manager)
    }
    
    /// Initialize with default well-known reciters
    fn initialize_default_reciters(&mut self) -> Result<()> {
        let default_reciters = vec![
            ("Abdul Rahman Al-Sudais", "عبد الرحمن السديس", RecitationStyle::Hafs),
            ("Saad Al-Ghamdi", "سعد الغامدي", RecitationStyle::Hafs),
            ("Mishary Rashid Alafasy", "مشاري راشد العفاسي", RecitationStyle::Hafs),
            ("Maher Al Mueaqly", "ماهر المعيقلي", RecitationStyle::Hafs),
            ("Ahmed ibn Ali al-Ajamy", "أحمد بن علي العجمي", RecitationStyle::Hafs),
            ("Yasser Al-Dosari", "ياسر الدوسري", RecitationStyle::Hafs),
            ("Nasser Al Qatami", "ناصر القطامي", RecitationStyle::Hafs),
            ("Warsh recitation", "رواية ورش", RecitationStyle::Warsh),
        ];
        
        for (name, arabic_name, style) in default_reciters {
            let reciter = Reciter {
                id: Uuid::new_v4(),
                name: name.to_string(),
                arabic_name: arabic_name.to_string(),
                biography: None,
                recitation_style: style,
                is_reference: true,
                created_at: Utc::now(),
            };
            
            self.reciters.insert(reciter.id, reciter);
        }
        
        Ok(())
    }
    
    /// Add a new reciter
    pub fn add_reciter(&mut self, reciter: Reciter) -> Result<()> {
        if self.reciters.contains_key(&reciter.id) {
            return Err(anyhow::anyhow!("Reciter with ID {} already exists", reciter.id));
        }
        
        info!("Adding reciter: {} ({})", reciter.name, reciter.arabic_name);
        self.reciters.insert(reciter.id, reciter);
        Ok(())
    }
    
    /// Get all reciters
    pub fn get_all_reciters(&self) -> Vec<&Reciter> {
        self.reciters.values().collect()
    }
    
    /// Get reciter by ID
    pub fn get_reciter(&self, reciter_id: &Uuid) -> Option<&Reciter> {
        self.reciters.get(reciter_id)
    }
    
    /// Get reciters by recitation style
    pub fn get_reciters_by_style(&self, style: &RecitationStyle) -> Vec<&Reciter> {
        self.reciters
            .values()
            .filter(|r| std::mem::discriminant(&r.recitation_style) == std::mem::discriminant(style))
            .collect()
    }
    
    /// Add a reference recording
    pub fn add_reference_recording(&mut self, recording: ReferenceRecording) -> Result<()> {
        let key = (recording.surah_number, recording.ayah_number);
        
        // Validate the recording file exists
        if !Path::new(&recording.audio_recording.file_path).exists() {
            return Err(anyhow::anyhow!(
                "Reference recording file not found: {}",
                recording.audio_recording.file_path
            ));
        }
        
        // Validate reciter exists
        if !self.reciters.contains_key(&recording.reciter_id) {
            return Err(anyhow::anyhow!(
                "Reciter with ID {} not found",
                recording.reciter_id
            ));
        }
        
        info!(
            "Adding reference recording for Surah {} Ayah {} by reciter {}",
            recording.surah_number,
            recording.ayah_number,
            recording.reciter_id
        );
        
        self.reference_recordings
            .entry(key)
            .or_insert_with(Vec::new)
            .push(recording);
        
        Ok(())
    }
    
    /// Get reference recordings for a specific ayah
    pub fn get_reference_recordings(&self, surah: u8, ayah: u16) -> Vec<&ReferenceRecording> {
        self.reference_recordings
            .get(&(surah, ayah))
            .map(|recordings| recordings.iter().collect())
            .unwrap_or_default()
    }
    
    /// Get reference recordings by reciter
    pub fn get_recordings_by_reciter(&self, reciter_id: &Uuid) -> Vec<&ReferenceRecording> {
        self.reference_recordings
            .values()
            .flatten()
            .filter(|r| r.reciter_id == *reciter_id)
            .collect()
    }
    
    /// Get the best reference recording for comparison
    pub fn get_best_reference(&self, surah: u8, ayah: u16, style: Option<RecitationStyle>) -> Option<&ReferenceRecording> {
        let recordings = self.get_reference_recordings(surah, ayah);
        
        if recordings.is_empty() {
            return None;
        }
        
        // Filter by recitation style if specified
        let filtered_recordings: Vec<&ReferenceRecording> = if let Some(target_style) = style {
            recordings
                .into_iter()
                .filter(|r| {
                    if let Some(reciter) = self.reciters.get(&r.reciter_id) {
                        std::mem::discriminant(&reciter.recitation_style) == std::mem::discriminant(&target_style)
                    } else {
                        false
                    }
                })
                .collect()
        } else {
            recordings
        };
        
        // Return the recording with highest quality score
        filtered_recordings
            .into_iter()
            .filter(|r| r.verified)
            .max_by(|a, b| a.quality_score.partial_cmp(&b.quality_score).unwrap())
            .or_else(|| {
                // If no verified recordings, return the best unverified one
                self.get_reference_recordings(surah, ayah)
                    .into_iter()
                    .max_by(|a, b| a.quality_score.partial_cmp(&b.quality_score).unwrap())
            })
    }
    
    /// Import reference recordings from a directory structure
    pub fn import_reference_recordings<P: AsRef<Path>>(&mut self, import_path: P) -> Result<usize> {
        let import_path = import_path.as_ref();
        
        if !import_path.exists() {
            return Err(anyhow::anyhow!("Import path does not exist: {:?}", import_path));
        }
        
        let mut imported_count = 0;
        
        // Expected directory structure: reciter_name/surah_number/ayah_number.wav
        for reciter_entry in std::fs::read_dir(import_path)? {
            let reciter_dir = reciter_entry?;
            if !reciter_dir.file_type()?.is_dir() {
                continue;
            }
            
            let reciter_name = reciter_dir.file_name().to_string_lossy().to_string();
            
            // Find or create reciter
            let reciter_id = self.find_or_create_reciter(&reciter_name)?;
            
            for surah_entry in std::fs::read_dir(reciter_dir.path())? {
                let surah_dir = surah_entry?;
                if !surah_dir.file_type()?.is_dir() {
                    continue;
                }
                
                let surah_number: u8 = surah_dir
                    .file_name()
                    .to_string_lossy()
                    .parse()
                    .context("Invalid surah number in directory name")?;
                
                for ayah_entry in std::fs::read_dir(surah_dir.path())? {
                    let ayah_file = ayah_entry?;
                    if !ayah_file.file_type()?.is_file() {
                        continue;
                    }
                    
                    let file_name_os = ayah_file.file_name();
                    let file_name = file_name_os.to_string_lossy();
                    if !file_name.ends_with(".wav") {
                        continue;
                    }
                    
                    let ayah_number: u16 = file_name
                        .trim_end_matches(".wav")
                        .parse()
                        .context("Invalid ayah number in file name")?;
                    
                    // Create reference recording
                    let audio_recording = self.create_audio_recording_from_file(
                        ayah_file.path(),
                        surah_number,
                        ayah_number,
                    )?;
                    
                    let reference_recording = ReferenceRecording {
                        id: Uuid::new_v4(),
                        reciter_id,
                        surah_number,
                        ayah_number,
                        audio_recording,
                        quality_score: 0.8, // Default quality score
                        verified: false,    // Needs manual verification
                    };
                    
                    self.add_reference_recording(reference_recording)?;
                    imported_count += 1;
                }
            }
        }
        
        info!("Imported {} reference recordings", imported_count);
        Ok(imported_count)
    }
    
    /// Find existing reciter or create a new one
    fn find_or_create_reciter(&mut self, name: &str) -> Result<Uuid> {
        // Try to find existing reciter by name
        for reciter in self.reciters.values() {
            if reciter.name.eq_ignore_ascii_case(name) {
                return Ok(reciter.id);
            }
        }
        
        // Create new reciter
        let reciter = Reciter {
            id: Uuid::new_v4(),
            name: name.to_string(),
            arabic_name: name.to_string(), // Would need proper Arabic name mapping
            biography: None,
            recitation_style: RecitationStyle::Hafs, // Default
            is_reference: true,
            created_at: Utc::now(),
        };
        
        let reciter_id = reciter.id;
        self.reciters.insert(reciter_id, reciter);
        Ok(reciter_id)
    }
    
    /// Create audio recording metadata from file
    fn create_audio_recording_from_file<P: AsRef<Path>>(
        &self,
        file_path: P,
        surah_number: u8,
        ayah_number: u16,
    ) -> Result<AudioRecording> {
        let file_path = file_path.as_ref();
        let metadata = std::fs::metadata(file_path)?;
        
        // For now, assume basic WAV file properties
        // In a full implementation, we'd parse the WAV header
        let recording = AudioRecording {
            id: Uuid::new_v4(),
            user_id: None,
            surah_number,
            ayah_start: ayah_number,
            ayah_end: ayah_number,
            format: AudioFormat::Wav,
            sample_rate: 44100, // Default assumption
            duration_seconds: 10.0, // Would need to calculate from file
            file_path: file_path.to_string_lossy().to_string(),
            file_size_bytes: metadata.len(),
            created_at: Utc::now(),
        };
        
        Ok(recording)
    }
    
    /// Get statistics about reference recordings
    pub fn get_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        stats.insert("total_reciters".to_string(), self.reciters.len());
        stats.insert("reference_reciters".to_string(), 
            self.reciters.values().filter(|r| r.is_reference).count());
        
        let total_recordings: usize = self.reference_recordings.values()
            .map(|recordings| recordings.len())
            .sum();
        stats.insert("total_recordings".to_string(), total_recordings);
        
        let verified_recordings: usize = self.reference_recordings.values()
            .flatten()
            .filter(|r| r.verified)
            .count();
        stats.insert("verified_recordings".to_string(), verified_recordings);
        
        stats.insert("unique_ayahs".to_string(), self.reference_recordings.len());
        
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_reference_manager_creation() {
        let temp_dir = tempdir().unwrap();
        let manager = ReferenceManager::new(temp_dir.path());
        assert!(manager.is_ok());
        
        let manager = manager.unwrap();
        assert!(!manager.reciters.is_empty());
    }
    
    #[test]
    fn test_add_reciter() {
        let temp_dir = tempdir().unwrap();
        let mut manager = ReferenceManager::new(temp_dir.path()).unwrap();
        
        let reciter = Reciter {
            id: Uuid::new_v4(),
            name: "Test Reciter".to_string(),
            arabic_name: "قارئ تجريبي".to_string(),
            biography: Some("Test biography".to_string()),
            recitation_style: RecitationStyle::Hafs,
            is_reference: true,
            created_at: Utc::now(),
        };
        
        let reciter_id = reciter.id;
        assert!(manager.add_reciter(reciter).is_ok());
        assert!(manager.get_reciter(&reciter_id).is_some());
    }
    
    #[test]
    fn test_get_reciters_by_style() {
        let temp_dir = tempdir().unwrap();
        let manager = ReferenceManager::new(temp_dir.path()).unwrap();
        
        let hafs_reciters = manager.get_reciters_by_style(&RecitationStyle::Hafs);
        let warsh_reciters = manager.get_reciters_by_style(&RecitationStyle::Warsh);
        
        assert!(!hafs_reciters.is_empty());
        assert!(!warsh_reciters.is_empty());
    }
    
    #[test]
    fn test_statistics() {
        let temp_dir = tempdir().unwrap();
        let manager = ReferenceManager::new(temp_dir.path()).unwrap();
        
        let stats = manager.get_statistics();
        assert!(stats.contains_key("total_reciters"));
        assert!(stats.contains_key("reference_reciters"));
        assert!(stats.contains_key("total_recordings"));
        assert!(stats["total_reciters"] > 0);
    }
}
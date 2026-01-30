use anyhow::{Result, Context};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{info, error, warn};
use shared::{AudioRecording, AudioFormat};
use uuid::Uuid;
use chrono::Utc;

/// Audio recorder for managing recording sessions
pub struct AudioRecorder {
    is_recording: Arc<Mutex<bool>>,
    recording_start_time: Arc<Mutex<Option<Instant>>>,
    max_recording_duration: Duration,
    min_recording_duration: Duration,
}

impl AudioRecorder {
    /// Create a new audio recorder
    pub fn new() -> Self {
        Self {
            is_recording: Arc::new(Mutex::new(false)),
            recording_start_time: Arc::new(Mutex::new(None)),
            max_recording_duration: Duration::from_secs(300), // 5 minutes max
            min_recording_duration: Duration::from_secs(1),   // 1 second min
        }
    }
    
    /// Start a new recording session
    pub fn start_session(&self) -> Result<String> {
        let mut is_recording = self.is_recording.lock().unwrap();
        
        if *is_recording {
            return Err(anyhow::anyhow!("Recording session already in progress"));
        }
        
        *is_recording = true;
        *self.recording_start_time.lock().unwrap() = Some(Instant::now());
        
        let session_id = Uuid::new_v4().to_string();
        info!("Started recording session: {}", session_id);
        
        Ok(session_id)
    }
    
    /// Stop the current recording session
    pub fn stop_session(&self) -> Result<Duration> {
        let mut is_recording = self.is_recording.lock().unwrap();
        
        if !*is_recording {
            return Err(anyhow::anyhow!("No recording session in progress"));
        }
        
        let start_time = self.recording_start_time.lock().unwrap().take()
            .context("Recording start time not found")?;
        
        let duration = start_time.elapsed();
        *is_recording = false;
        
        if duration < self.min_recording_duration {
            return Err(anyhow::anyhow!(
                "Recording too short: {:.2}s (minimum: {:.2}s)",
                duration.as_secs_f64(),
                self.min_recording_duration.as_secs_f64()
            ));
        }
        
        if duration > self.max_recording_duration {
            warn!("Recording exceeded maximum duration: {:.2}s", duration.as_secs_f64());
        }
        
        info!("Stopped recording session. Duration: {:.2}s", duration.as_secs_f64());
        Ok(duration)
    }
    
    /// Check if currently recording
    pub fn is_recording(&self) -> bool {
        *self.is_recording.lock().unwrap()
    }
    
    /// Get current recording duration
    pub fn get_current_duration(&self) -> Option<Duration> {
        if !self.is_recording() {
            return None;
        }
        
        self.recording_start_time.lock().unwrap()
            .map(|start_time| start_time.elapsed())
    }
    
    /// Validate recording parameters
    pub fn validate_recording_params(
        &self,
        surah_number: u8,
        ayah_start: u16,
        ayah_end: u16,
    ) -> Result<()> {
        if surah_number < 1 || surah_number > 114 {
            return Err(anyhow::anyhow!("Invalid surah number: {}", surah_number));
        }
        
        if ayah_start < 1 {
            return Err(anyhow::anyhow!("Invalid ayah start: {}", ayah_start));
        }
        
        if ayah_end < ayah_start {
            return Err(anyhow::anyhow!("Ayah end ({}) cannot be less than ayah start ({})", ayah_end, ayah_start));
        }
        
        // Validate ayah numbers for specific surahs (simplified validation)
        let max_ayahs = match surah_number {
            1 => 7,    // Al-Fatiha
            2 => 286,  // Al-Baqarah
            3 => 200,  // Ali 'Imran
            // Add more surah validations as needed
            _ => 300,  // Conservative upper bound for other surahs
        };
        
        if ayah_end > max_ayahs {
            return Err(anyhow::anyhow!(
                "Invalid ayah number {} for surah {} (max: {})",
                ayah_end, surah_number, max_ayahs
            ));
        }
        
        Ok(())
    }
    
    /// Create recording metadata
    pub fn create_recording_metadata(
        &self,
        user_id: Option<Uuid>,
        surah_number: u8,
        ayah_start: u16,
        ayah_end: u16,
        file_path: String,
        duration: Duration,
        sample_rate: u32,
    ) -> Result<AudioRecording> {
        self.validate_recording_params(surah_number, ayah_start, ayah_end)?;
        
        let file_size = std::fs::metadata(&file_path)
            .context("Failed to get file metadata")?
            .len();
        
        let recording = AudioRecording {
            id: Uuid::new_v4(),
            user_id,
            surah_number,
            ayah_start,
            ayah_end,
            format: AudioFormat::Wav, // Default format
            sample_rate,
            duration_seconds: duration.as_secs_f64(),
            file_path,
            file_size_bytes: file_size,
            created_at: Utc::now(),
        };
        
        info!("Created recording metadata: {:?}", recording.id);
        Ok(recording)
    }
    
    /// Set maximum recording duration
    pub fn set_max_duration(&mut self, duration: Duration) {
        self.max_recording_duration = duration;
        info!("Set maximum recording duration to {:.2}s", duration.as_secs_f64());
    }
    
    /// Set minimum recording duration
    pub fn set_min_duration(&mut self, duration: Duration) {
        self.min_recording_duration = duration;
        info!("Set minimum recording duration to {:.2}s", duration.as_secs_f64());
    }
    
    /// Get recording limits
    pub fn get_limits(&self) -> (Duration, Duration) {
        (self.min_recording_duration, self.max_recording_duration)
    }
}

impl Default for AudioRecorder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_recording_session() {
        let recorder = AudioRecorder::new();
        
        // Test starting a session
        let session_id = recorder.start_session();
        assert!(session_id.is_ok());
        assert!(recorder.is_recording());
        
        // Test that we can't start another session
        let second_session = recorder.start_session();
        assert!(second_session.is_err());
        
        // Wait a bit and stop
        thread::sleep(Duration::from_millis(1100)); // Just over minimum duration
        let duration = recorder.stop_session();
        assert!(duration.is_ok());
        assert!(!recorder.is_recording());
        
        let actual_duration = duration.unwrap();
        assert!(actual_duration >= Duration::from_secs(1));
    }
    
    #[test]
    fn test_recording_validation() {
        let recorder = AudioRecorder::new();
        
        // Valid parameters
        assert!(recorder.validate_recording_params(1, 1, 7).is_ok());
        assert!(recorder.validate_recording_params(2, 1, 286).is_ok());
        
        // Invalid surah number
        assert!(recorder.validate_recording_params(0, 1, 1).is_err());
        assert!(recorder.validate_recording_params(115, 1, 1).is_err());
        
        // Invalid ayah numbers
        assert!(recorder.validate_recording_params(1, 0, 1).is_err());
        assert!(recorder.validate_recording_params(1, 5, 3).is_err());
        assert!(recorder.validate_recording_params(1, 1, 10).is_err()); // Al-Fatiha has only 7 ayahs
    }
    
    #[test]
    fn test_duration_limits() {
        let mut recorder = AudioRecorder::new();
        
        // Test setting limits
        recorder.set_max_duration(Duration::from_secs(60));
        recorder.set_min_duration(Duration::from_millis(500));
        
        let (min, max) = recorder.get_limits();
        assert_eq!(min, Duration::from_millis(500));
        assert_eq!(max, Duration::from_secs(60));
    }
}
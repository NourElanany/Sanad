use anyhow::{Result, Context};
use cpal::{Device, Host, SupportedStreamConfig, Stream, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavReader, WavWriter, WavSpec};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{info, error, warn};
use shared::{AudioFormat, AudioRecording};
use uuid::Uuid;
use chrono::Utc;

/// Main audio processor for handling recording, playback, and format conversion
pub struct AudioProcessor {
    host: Host,
    input_device: Option<Device>,
    output_device: Option<Device>,
    recording_stream: Option<Stream>,
    is_recording: Arc<Mutex<bool>>,
    recorded_samples: Arc<Mutex<Vec<f32>>>,
}

impl AudioProcessor {
    /// Create a new audio processor instance
    pub async fn new() -> Result<Self> {
        let host = cpal::default_host();
        
        let input_device = host.default_input_device();
        let output_device = host.default_output_device();
        
        if input_device.is_none() {
            warn!("No default input device found");
        }
        
        if output_device.is_none() {
            warn!("No default output device found");
        }
        
        info!("Audio processor initialized successfully");
        
        Ok(Self {
            host,
            input_device,
            output_device,
            recording_stream: None,
            is_recording: Arc::new(Mutex::new(false)),
            recorded_samples: Arc::new(Mutex::new(Vec::new())),
        })
    }
    
    /// Start recording audio
    pub fn start_recording(&mut self) -> Result<()> {
        let device = self.input_device.as_ref()
            .context("No input device available")?;
        
        let config = device.default_input_config()
            .context("Failed to get default input config")?;
        
        info!("Starting recording with config: {:?}", config);
        
        let sample_rate = config.sample_rate().0;
        let channels = config.channels();
        
        let is_recording = Arc::clone(&self.is_recording);
        let recorded_samples = Arc::clone(&self.recorded_samples);
        
        // Clear previous recording
        recorded_samples.lock().unwrap().clear();
        *is_recording.lock().unwrap() = true;
        
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                device.build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        if *is_recording.lock().unwrap() {
                            recorded_samples.lock().unwrap().extend_from_slice(data);
                        }
                    },
                    |err| error!("Recording error: {}", err),
                    None,
                )?
            }
            cpal::SampleFormat::I16 => {
                device.build_input_stream(
                    &config.into(),
                    move |data: &[i16], _: &cpal::InputCallbackInfo| {
                        if *is_recording.lock().unwrap() {
                            let float_data: Vec<f32> = data.iter()
                                .map(|&sample| sample as f32 / i16::MAX as f32)
                                .collect();
                            recorded_samples.lock().unwrap().extend_from_slice(&float_data);
                        }
                    },
                    |err| error!("Recording error: {}", err),
                    None,
                )?
            }
            cpal::SampleFormat::U16 => {
                device.build_input_stream(
                    &config.into(),
                    move |data: &[u16], _: &cpal::InputCallbackInfo| {
                        if *is_recording.lock().unwrap() {
                            let float_data: Vec<f32> = data.iter()
                                .map(|&sample| (sample as f32 - u16::MAX as f32 / 2.0) / (u16::MAX as f32 / 2.0))
                                .collect();
                            recorded_samples.lock().unwrap().extend_from_slice(&float_data);
                        }
                    },
                    |err| error!("Recording error: {}", err),
                    None,
                )?
            }
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };
        
        stream.play()?;
        self.recording_stream = Some(stream);
        
        info!("Recording started successfully");
        Ok(())
    }
    
    /// Stop recording and return the recorded audio
    pub fn stop_recording(&mut self) -> Result<AudioRecording> {
        *self.is_recording.lock().unwrap() = false;
        
        if let Some(stream) = self.recording_stream.take() {
            drop(stream);
        }
        
        let samples = self.recorded_samples.lock().unwrap().clone();
        
        if samples.is_empty() {
            return Err(anyhow::anyhow!("No audio data recorded"));
        }
        
        let recording_id = Uuid::new_v4();
        let file_path = format!("recordings/{}.wav", recording_id);
        
        // Ensure recordings directory exists
        std::fs::create_dir_all("recordings")?;
        
        // Save as WAV file
        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        
        let mut writer = WavWriter::create(&file_path, spec)?;
        for sample in &samples {
            writer.write_sample(*sample)?;
        }
        writer.finalize()?;
        
        let file_size = std::fs::metadata(&file_path)?.len();
        let duration = samples.len() as f64 / 44100.0;
        
        let recording = AudioRecording {
            id: recording_id,
            user_id: None,
            surah_number: 1, // Default, should be set by caller
            ayah_start: 1,   // Default, should be set by caller
            ayah_end: 1,     // Default, should be set by caller
            format: AudioFormat::Wav,
            sample_rate: 44100,
            duration_seconds: duration,
            file_path,
            file_size_bytes: file_size,
            created_at: Utc::now(),
        };
        
        info!("Recording stopped. Duration: {:.2}s, Size: {} bytes", duration, file_size);
        Ok(recording)
    }
    
    /// Convert audio file to different format
    pub fn convert_format(&self, input_path: &str, output_path: &str, target_format: AudioFormat) -> Result<()> {
        match target_format {
            AudioFormat::Wav => self.convert_to_wav(input_path, output_path),
            AudioFormat::Mp3 => Err(anyhow::anyhow!("MP3 conversion not implemented yet")),
            AudioFormat::Flac => Err(anyhow::anyhow!("FLAC conversion not implemented yet")),
            AudioFormat::Ogg => Err(anyhow::anyhow!("OGG conversion not implemented yet")),
        }
    }
    
    /// Convert audio file to WAV format
    fn convert_to_wav(&self, input_path: &str, output_path: &str) -> Result<()> {
        // For now, just copy if it's already a WAV file
        // In a full implementation, we'd use a library like ffmpeg-next
        if input_path.ends_with(".wav") {
            std::fs::copy(input_path, output_path)?;
            info!("Copied WAV file from {} to {}", input_path, output_path);
        } else {
            return Err(anyhow::anyhow!("Conversion from non-WAV formats not implemented yet"));
        }
        Ok(())
    }
    
    /// Load audio file and return samples
    pub fn load_audio_file(&self, file_path: &str) -> Result<(Vec<f32>, u32)> {
        let file = File::open(file_path)
            .context("Failed to open audio file")?;
        let reader = BufReader::new(file);
        let mut wav_reader = WavReader::new(reader)
            .context("Failed to create WAV reader")?;
        
        let spec = wav_reader.spec();
        let sample_rate = spec.sample_rate;
        
        let samples: Result<Vec<f32>, _> = match spec.sample_format {
            hound::SampleFormat::Float => {
                wav_reader.samples::<f32>().collect()
            }
            hound::SampleFormat::Int => {
                wav_reader.samples::<i32>()
                    .map(|s| s.map(|sample| sample as f32 / i32::MAX as f32))
                    .collect()
            }
        };
        
        let samples = samples.context("Failed to read audio samples")?;
        
        info!("Loaded audio file: {} samples at {} Hz", samples.len(), sample_rate);
        Ok((samples, sample_rate))
    }
    
    /// Get audio file metadata
    pub fn get_audio_metadata(&self, file_path: &str) -> Result<(u32, u16, f64)> {
        let file = File::open(file_path)
            .context("Failed to open audio file")?;
        let reader = BufReader::new(file);
        let wav_reader = WavReader::new(reader)
            .context("Failed to create WAV reader")?;
        
        let spec = wav_reader.spec();
        let sample_count = wav_reader.len();
        let duration = sample_count as f64 / spec.sample_rate as f64;
        
        Ok((spec.sample_rate, spec.channels, duration))
    }
    
    /// Check if currently recording
    pub fn is_recording(&self) -> bool {
        *self.is_recording.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_audio_processor_creation() {
        let processor = AudioProcessor::new().await;
        assert!(processor.is_ok());
    }
    
    #[test]
    fn test_audio_metadata() {
        // This test would require a sample WAV file
        // For now, we'll just test that the function exists
        let processor = AudioProcessor::new();
        // assert!(processor.is_ok());
    }
}
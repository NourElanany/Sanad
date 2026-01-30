use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use shared::{AudioFormat, TajweedError, RecitationStyle};

/// Request to start recording
#[derive(Debug, Deserialize)]
pub struct StartRecordingRequest {
    pub user_id: Option<Uuid>,
    pub surah_number: u8,
    pub ayah_start: u16,
    pub ayah_end: u16,
    pub max_duration_seconds: Option<u32>,
}

/// Request to analyze a recording
#[derive(Debug, Deserialize)]
pub struct AnalyzeRecordingRequest {
    pub recording_id: Uuid,
    pub reference_reciter_id: Option<Uuid>,
    pub recitation_style: Option<RecitationStyle>,
}

/// Request to compare two recordings
#[derive(Debug, Deserialize)]
pub struct CompareRecordingsRequest {
    pub user_recording_id: Uuid,
    pub reference_recording_id: Uuid,
    pub analysis_type: AnalysisType,
}

/// Types of audio analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisType {
    Basic,           // Basic similarity comparison
    Tajweed,         // Full Tajweed error detection
    Pronunciation,   // Focus on pronunciation accuracy
    Timing,          // Focus on timing and rhythm
    Comprehensive,   // All analysis types
}

/// Audio upload metadata
#[derive(Debug, Deserialize)]
pub struct AudioUploadMetadata {
    pub user_id: Option<Uuid>,
    pub surah_number: u8,
    pub ayah_start: u16,
    pub ayah_end: u16,
    pub format: AudioFormat,
    pub description: Option<String>,
}

/// Recording session state
#[derive(Debug, Clone, Serialize)]
pub struct RecordingSession {
    pub session_id: String,
    pub user_id: Option<Uuid>,
    pub surah_number: u8,
    pub ayah_start: u16,
    pub ayah_end: u16,
    pub started_at: DateTime<Utc>,
    pub max_duration_seconds: u32,
    pub is_active: bool,
}

/// Audio processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioProcessingConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub bit_depth: u16,
    pub window_size: usize,
    pub overlap: f64,
    pub noise_reduction: bool,
    pub auto_gain_control: bool,
}

impl Default for AudioProcessingConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            channels: 1,
            bit_depth: 16,
            window_size: 2048,
            overlap: 0.5,
            noise_reduction: true,
            auto_gain_control: true,
        }
    }
}

/// Analysis progress update
#[derive(Debug, Clone, Serialize)]
pub struct AnalysisProgress {
    pub recording_id: Uuid,
    pub stage: AnalysisStage,
    pub progress_percent: u8,
    pub estimated_remaining_seconds: Option<u32>,
    pub current_operation: String,
}

/// Stages of audio analysis
#[derive(Debug, Clone, Serialize)]
pub enum AnalysisStage {
    Loading,
    Preprocessing,
    SpectralAnalysis,
    Comparison,
    ErrorDetection,
    GeneratingReport,
    Complete,
}

/// Detailed analysis report
#[derive(Debug, Clone, Serialize)]
pub struct DetailedAnalysisReport {
    pub recording_id: Uuid,
    pub reference_recording_id: Option<Uuid>,
    pub analysis_type: AnalysisType,
    pub overall_score: f64,
    pub detailed_scores: DetailedScores,
    pub errors: Vec<TajweedError>,
    pub recommendations: Vec<Recommendation>,
    pub spectral_features: SpectralFeatures,
    pub timing_analysis: TimingAnalysis,
    pub analyzed_at: DateTime<Utc>,
    pub analysis_duration_ms: u64,
}

/// Detailed scoring breakdown
#[derive(Debug, Clone, Serialize)]
pub struct DetailedScores {
    pub pronunciation_accuracy: f64,
    pub timing_accuracy: f64,
    pub tajweed_compliance: f64,
    pub fluency: f64,
    pub clarity: f64,
    pub rhythm: f64,
}

/// Recommendation for improvement
#[derive(Debug, Clone, Serialize)]
pub struct Recommendation {
    pub category: RecommendationCategory,
    pub priority: Priority,
    pub description: String,
    pub specific_advice: String,
    pub practice_exercises: Vec<String>,
    pub reference_materials: Vec<String>,
}

/// Categories of recommendations
#[derive(Debug, Clone, Serialize)]
pub enum RecommendationCategory {
    Pronunciation,
    Timing,
    Tajweed,
    Fluency,
    General,
}

/// Priority levels
#[derive(Debug, Clone, Serialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

/// Spectral analysis features
#[derive(Debug, Clone, Serialize)]
pub struct SpectralFeatures {
    pub fundamental_frequency: f64,
    pub formants: Vec<f64>,
    pub spectral_centroid: f64,
    pub spectral_rolloff: f64,
    pub spectral_bandwidth: f64,
    pub zero_crossing_rate: f64,
    pub mfcc_coefficients: Vec<f64>,
}

/// Timing analysis results
#[derive(Debug, Clone, Serialize)]
pub struct TimingAnalysis {
    pub total_duration: f64,
    pub speech_duration: f64,
    pub pause_duration: f64,
    pub speech_rate: f64, // syllables per second
    pub pause_frequency: f64,
    pub rhythm_regularity: f64,
}

/// Audio quality metrics
#[derive(Debug, Clone, Serialize)]
pub struct AudioQualityMetrics {
    pub signal_to_noise_ratio: f64,
    pub dynamic_range: f64,
    pub clipping_detected: bool,
    pub background_noise_level: f64,
    pub recording_quality_score: f64,
}

/// Batch processing request
#[derive(Debug, Deserialize)]
pub struct BatchProcessingRequest {
    pub recording_ids: Vec<Uuid>,
    pub analysis_type: AnalysisType,
    pub reference_reciter_id: Option<Uuid>,
    pub callback_url: Option<String>,
}

/// Batch processing status
#[derive(Debug, Clone, Serialize)]
pub struct BatchProcessingStatus {
    pub batch_id: Uuid,
    pub total_recordings: usize,
    pub completed_recordings: usize,
    pub failed_recordings: usize,
    pub status: BatchStatus,
    pub started_at: DateTime<Utc>,
    pub estimated_completion: Option<DateTime<Utc>>,
}

/// Batch processing status
#[derive(Debug, Clone, Serialize)]
pub enum BatchStatus {
    Queued,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// Audio format conversion request
#[derive(Debug, Deserialize)]
pub struct FormatConversionRequest {
    pub recording_id: Uuid,
    pub target_format: AudioFormat,
    pub quality_settings: Option<QualitySettings>,
}

/// Quality settings for format conversion
#[derive(Debug, Clone, Deserialize)]
pub struct QualitySettings {
    pub sample_rate: Option<u32>,
    pub bit_rate: Option<u32>,
    pub channels: Option<u16>,
    pub compression_level: Option<u8>,
}

/// System health status
#[derive(Debug, Clone, Serialize)]
pub struct SystemHealth {
    pub service_status: ServiceStatus,
    pub audio_devices: AudioDeviceStatus,
    pub storage: StorageStatus,
    pub performance: PerformanceMetrics,
    pub last_updated: DateTime<Utc>,
}

/// Service status
#[derive(Debug, Clone, Serialize)]
pub struct ServiceStatus {
    pub is_healthy: bool,
    pub active_recordings: usize,
    pub queued_analyses: usize,
    pub uptime_seconds: u64,
}

/// Audio device status
#[derive(Debug, Clone, Serialize)]
pub struct AudioDeviceStatus {
    pub input_devices_available: usize,
    pub output_devices_available: usize,
    pub default_input_working: bool,
    pub default_output_working: bool,
}

/// Storage status
#[derive(Debug, Clone, Serialize)]
pub struct StorageStatus {
    pub total_space_gb: f64,
    pub used_space_gb: f64,
    pub available_space_gb: f64,
    pub recordings_count: usize,
    pub reference_recordings_count: usize,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize)]
pub struct PerformanceMetrics {
    pub average_analysis_time_ms: f64,
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: f64,
    pub active_threads: usize,
}
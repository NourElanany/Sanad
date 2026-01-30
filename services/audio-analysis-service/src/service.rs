use anyhow::{Result, Context};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error, warn, debug};
use uuid::Uuid;
use chrono::Utc;
use shared::{
    AudioRecording, AudioFormat, RecitationAnalysis, TajweedError, 
    AudioSpectrum, AudioComparisonResult, Reciter, ReferenceRecording,
    RecitationStyle, ErrorSeverity, TajweedErrorType
};

use crate::audio_processor::AudioProcessor;
use crate::audio_recorder::AudioRecorder;
use crate::spectrum_analyzer::SpectrumAnalyzer;
use crate::reference_manager::ReferenceManager;
use crate::comparison_engine::ComparisonEngine;
use crate::scoring_system::RecitationScoringSystem;
use crate::progress_tracker::{ProgressTracker, UserProgressData, ProgressUpdate};
use crate::improvement_engine::{ImprovementEngine, LearningPlan, AdaptiveRecommendation, ProgressPrediction, MotivationalInsights};
use crate::reward_system::{RewardSystem, UserRewardStatus, RewardUpdate, GamificationStatus, Challenge, DailyGoal, WeeklyGoal};
use crate::models::*;

/// Main service for audio analysis and recitation correction
pub struct AudioAnalysisService {
    audio_processor: Arc<Mutex<AudioProcessor>>,
    audio_recorder: Arc<Mutex<AudioRecorder>>,
    spectrum_analyzer: SpectrumAnalyzer,
    reference_manager: Arc<Mutex<ReferenceManager>>,
    comparison_engine: ComparisonEngine,
    scoring_system: RecitationScoringSystem,
    progress_tracker: Arc<Mutex<ProgressTracker>>,
    improvement_engine: ImprovementEngine,
    reward_system: RewardSystem,
    active_sessions: Arc<Mutex<HashMap<String, RecordingSession>>>,
    // Store recordings in memory for now (in production, use database)
    recordings: Arc<Mutex<HashMap<Uuid, AudioRecording>>>,
}

impl AudioAnalysisService {
    /// Create a new audio analysis service
    pub async fn new() -> Result<Self> {
        info!("Initializing Audio Analysis Service...");
        
        // Initialize components
        let audio_processor = Arc::new(Mutex::new(AudioProcessor::new().await?));
        let audio_recorder = Arc::new(Mutex::new(AudioRecorder::new()));
        let spectrum_analyzer = SpectrumAnalyzer::new_for_speech();
        let reference_manager = Arc::new(Mutex::new(ReferenceManager::new("reference_audio")?));
        let comparison_engine = ComparisonEngine::new();
        let scoring_system = RecitationScoringSystem::new();
        let progress_tracker = Arc::new(Mutex::new(ProgressTracker::new()));
        let improvement_engine = ImprovementEngine::new();
        let reward_system = RewardSystem::new();
        let active_sessions = Arc::new(Mutex::new(HashMap::new()));
        let recordings = Arc::new(Mutex::new(HashMap::new()));
        
        // Ensure reference audio directory exists
        std::fs::create_dir_all("reference_audio")?;
        std::fs::create_dir_all("recordings")?;
        
        info!("Audio Analysis Service initialized successfully");
        
        Ok(Self {
            audio_processor,
            audio_recorder,
            spectrum_analyzer,
            reference_manager,
            comparison_engine,
            scoring_system,
            progress_tracker,
            improvement_engine,
            reward_system,
            active_sessions,
            recordings,
        })
    }
    
    /// Start a new recording session
    pub async fn start_recording_session(&self, request: StartRecordingRequest) -> Result<RecordingSession> {
        let recorder = self.audio_recorder.lock().await;
        
        // Validate recording parameters
        recorder.validate_recording_params(
            request.surah_number,
            request.ayah_start,
            request.ayah_end,
        )?;
        
        let session_id = recorder.start_session()?;
        
        let session = RecordingSession {
            session_id: session_id.clone(),
            user_id: request.user_id,
            surah_number: request.surah_number,
            ayah_start: request.ayah_start,
            ayah_end: request.ayah_end,
            started_at: Utc::now(),
            max_duration_seconds: request.max_duration_seconds.unwrap_or(300),
            is_active: true,
        };
        
        // Store session
        self.active_sessions.lock().await.insert(session_id.clone(), session.clone());
        
        info!("Started recording session: {}", session_id);
        Ok(session)
    }
    
    /// Stop a recording session and save the recording
    pub async fn stop_recording_session(&self, session_id: &str) -> Result<AudioRecording> {
        let mut sessions = self.active_sessions.lock().await;
        let session = sessions.remove(session_id)
            .context("Recording session not found")?;
        
        let recorder = self.audio_recorder.lock().await;
        let duration = recorder.stop_session()?;
        
        // Get the recorded audio from the processor
        let mut processor = self.audio_processor.lock().await;
        let audio_recording = processor.stop_recording()?;
        
        // Update recording metadata with session info
        let mut recording = audio_recording;
        recording.user_id = session.user_id;
        recording.surah_number = session.surah_number;
        recording.ayah_start = session.ayah_start;
        recording.ayah_end = session.ayah_end;
        
        // Store recording in memory
        self.recordings.lock().await.insert(recording.id, recording.clone());
        
        info!("Stopped recording session: {} (duration: {:.2}s)", session_id, duration.as_secs_f64());
        Ok(recording)
    }
    
    /// Upload audio file and create recording
    pub async fn upload_audio(&self, audio_data: Vec<u8>, metadata: AudioUploadMetadata) -> Result<AudioRecording> {
        // Validate metadata
        let recorder = self.audio_recorder.lock().await;
        recorder.validate_recording_params(
            metadata.surah_number,
            metadata.ayah_start,
            metadata.ayah_end,
        )?;
        
        // Generate unique filename
        let recording_id = Uuid::new_v4();
        let file_extension = match metadata.format {
            AudioFormat::Wav => "wav",
            AudioFormat::Mp3 => "mp3",
            AudioFormat::Flac => "flac",
            AudioFormat::Ogg => "ogg",
        };
        let file_path = format!("recordings/{}.{}", recording_id, file_extension);
        
        // Save audio file
        std::fs::write(&file_path, &audio_data)?;
        
        // Get audio metadata
        let processor = self.audio_processor.lock().await;
        let (sample_rate, _channels, duration) = processor.get_audio_metadata(&file_path)?;
        
        let recording = AudioRecording {
            id: recording_id,
            user_id: metadata.user_id,
            surah_number: metadata.surah_number,
            ayah_start: metadata.ayah_start,
            ayah_end: metadata.ayah_end,
            format: metadata.format,
            sample_rate,
            duration_seconds: duration,
            file_path,
            file_size_bytes: audio_data.len() as u64,
            created_at: Utc::now(),
        };
        
        // Store recording in memory
        self.recordings.lock().await.insert(recording.id, recording.clone());
        
        info!("Uploaded audio recording: {} (size: {} bytes)", recording_id, audio_data.len());
        Ok(recording)
    }
    
    /// Analyze a recording for Tajweed errors and quality
    pub async fn analyze_recording(&self, recording_id: Uuid, request: AnalyzeRecordingRequest) -> Result<RecitationAnalysis> {
        // Get the recording from memory
        let recording = {
            let recordings = self.recordings.lock().await;
            recordings.get(&recording_id)
                .cloned()
                .context("Recording not found")?
        };
        
        // Load audio samples
        let processor = self.audio_processor.lock().await;
        let (user_samples, sample_rate) = processor.load_audio_file(&recording.file_path)?;
        
        // For now, use a simple reference (in production, get from database)
        let reference_samples = user_samples.clone(); // Placeholder
        
        // Perform comparison analysis
        let comparison_result = self.comparison_engine.compare_recordings(
            &user_samples,
            &reference_samples,
            sample_rate,
        )?;
        
        // Detect Tajweed errors
        let tajweed_errors = self.comparison_engine.detect_tajweed_errors(
            &user_samples,
            &reference_samples,
            sample_rate,
        )?;
        
        // Calculate detailed scores using the scoring system
        let detailed_scores = self.scoring_system.calculate_detailed_scores(
            &comparison_result,
            &tajweed_errors,
            None, // Audio quality metrics would be calculated here in production
        )?;
        
        // Generate improvements and next steps using the scoring system
        let improvements = self.scoring_system.generate_feedback(&detailed_scores);
        let next_steps = self.scoring_system.generate_practice_recommendations(&detailed_scores, &tajweed_errors);
        
        let analysis = RecitationAnalysis {
            id: Uuid::new_v4(),
            user_recording_id: recording_id,
            reference_recording_id: Uuid::new_v4(), // Placeholder
            overall_score: detailed_scores.overall_score,
            tajweed_accuracy: detailed_scores.tajweed_accuracy,
            pronunciation_accuracy: detailed_scores.pronunciation_accuracy,
            timing_accuracy: detailed_scores.timing_accuracy,
            errors: tajweed_errors.clone(),
            improvements,
            next_steps,
            analyzed_at: Utc::now(),
        };
        
        info!("Analyzed recording: {} (score: {:.2})", recording_id, detailed_scores.overall_score);
        
        // Update user progress if user_id is available
        if let Some(user_id) = recording.user_id {
            let session_duration = 5; // Would calculate actual duration
            let progress_update = self.update_user_progress(
                user_id,
                recording.surah_number,
                recording.ayah_start,
                detailed_scores.overall_score,
                &tajweed_errors,
                session_duration,
            ).await?;
            
            info!("Updated progress for user: {} (new achievements: {})", 
                  user_id, progress_update.new_achievements.len());
        }
        
        Ok(analysis)
    }
    
    /// Compare two recordings
    pub async fn compare_recordings(&self, request: CompareRecordingsRequest) -> Result<AudioComparisonResult> {
        // Get both recordings
        let recordings = self.recordings.lock().await;
        let user_recording = recordings.get(&request.user_recording_id)
            .context("User recording not found")?;
        let reference_recording = recordings.get(&request.reference_recording_id)
            .context("Reference recording not found")?;
        
        // Load audio samples
        let processor = self.audio_processor.lock().await;
        let (user_samples, sample_rate) = processor.load_audio_file(&user_recording.file_path)?;
        let (reference_samples, _) = processor.load_audio_file(&reference_recording.file_path)?;
        
        // Perform comparison
        let result = self.comparison_engine.compare_recordings(
            &user_samples,
            &reference_samples,
            sample_rate,
        )?;
        
        info!("Compared recordings: {} vs {} (similarity: {:.2})", 
              request.user_recording_id, request.reference_recording_id, result.similarity_score);
        
        Ok(result)
    }
    
    /// Get all reciters
    pub async fn get_all_reciters(&self) -> Result<Vec<Reciter>> {
        let reference_manager = self.reference_manager.lock().await;
        let reciters = reference_manager.get_all_reciters()
            .into_iter()
            .cloned()
            .collect();
        Ok(reciters)
    }
    
    /// Get reference recordings for a specific ayah
    pub async fn get_reference_recordings(&self, surah: u8, ayah: u16) -> Result<Vec<ReferenceRecording>> {
        let reference_manager = self.reference_manager.lock().await;
        let recordings = reference_manager.get_reference_recordings(surah, ayah)
            .into_iter()
            .cloned()
            .collect();
        Ok(recordings)
    }
    
    /// Get audio spectrum for a recording
    pub async fn get_audio_spectrum(&self, recording_id: Uuid) -> Result<AudioSpectrum> {
        // Get the recording
        let recording = {
            let recordings = self.recordings.lock().await;
            recordings.get(&recording_id)
                .cloned()
                .context("Recording not found")?
        };
        
        // Load and analyze audio
        let processor = self.audio_processor.lock().await;
        let (samples, sample_rate) = processor.load_audio_file(&recording.file_path)?;
        
        let spectrum = self.spectrum_analyzer.analyze(&samples, sample_rate)?;
        
        Ok(spectrum)
    }
    
    /// Get system health status
    pub async fn get_system_health(&self) -> Result<SystemHealth> {
        // Check active recordings
        let active_recordings = self.active_sessions.lock().await.len();
        
        // Check storage
        let recordings_count = self.recordings.lock().await.len();
        
        // Get storage info (simplified)
        let storage = StorageStatus {
            total_space_gb: 100.0, // Would need actual disk space calculation
            used_space_gb: 10.0,   // Would need actual calculation
            available_space_gb: 90.0,
            recordings_count,
            reference_recordings_count: 0, // Placeholder
        };
        
        let service_status = ServiceStatus {
            is_healthy: true,
            active_recordings,
            queued_analyses: 0, // Would need actual queue implementation
            uptime_seconds: 0,  // Would need actual uptime tracking
        };
        
        let audio_devices = AudioDeviceStatus {
            input_devices_available: 1,  // Simplified
            output_devices_available: 1,
            default_input_working: true,
            default_output_working: true,
        };
        
        let performance = PerformanceMetrics {
            average_analysis_time_ms: 1500.0, // Would need actual metrics
            cpu_usage_percent: 25.0,
            memory_usage_mb: 256.0,
            active_threads: 4,
        };
        
        Ok(SystemHealth {
            service_status,
            audio_devices,
            storage,
            performance,
            last_updated: Utc::now(),
        })
    }
    
    // === TRACKING AND IMPROVEMENT SYSTEM METHODS ===
    
    /// Initialize progress tracking for a new user
    pub async fn initialize_user_progress(&self, user_id: Uuid) -> Result<()> {
        let mut tracker = self.progress_tracker.lock().await;
        tracker.initialize_user_progress(user_id)?;
        info!("Initialized progress tracking for user: {}", user_id);
        Ok(())
    }
    
    /// Update user progress after a practice session
    pub async fn update_user_progress(
        &self,
        user_id: Uuid,
        surah: u8,
        ayah: u16,
        score: f64,
        errors: &[TajweedError],
        session_duration_minutes: u32,
    ) -> Result<ProgressUpdate> {
        let mut tracker = self.progress_tracker.lock().await;
        let progress_update = tracker.update_progress(
            user_id,
            surah,
            ayah,
            score,
            errors,
            session_duration_minutes,
        )?;
        
        // Check for rewards
        if let Ok(user_progress) = self.get_user_progress_data(user_id).await {
            let reward_update = self.reward_system.check_rewards(&user_progress)?;
            
            if !reward_update.new_achievements.is_empty() || !reward_update.new_badges.is_empty() {
                info!("User {} earned {} new achievements and {} new badges", 
                      user_id, reward_update.new_achievements.len(), reward_update.new_badges.len());
            }
        }
        
        Ok(progress_update)
    }
    
    /// Get user's progress data
    pub async fn get_user_progress_data(&self, user_id: Uuid) -> Result<UserProgressData> {
        let tracker = self.progress_tracker.lock().await;
        tracker.get_user_progress(&user_id)
            .cloned()
            .context("User progress not found")
    }
    
    /// Generate personalized exercises for a user
    pub async fn generate_personalized_exercises(&self, user_id: Uuid) -> Result<Vec<crate::progress_tracker::Exercise>> {
        let tracker = self.progress_tracker.lock().await;
        tracker.generate_personalized_exercises(user_id)
    }
    
    /// Get detailed performance statistics for a user
    pub async fn get_performance_statistics(&self, user_id: Uuid) -> Result<crate::progress_tracker::PerformanceStatistics> {
        let tracker = self.progress_tracker.lock().await;
        tracker.get_performance_statistics(user_id)
    }
    
    /// Generate improvement recommendations
    pub async fn generate_improvement_recommendations(
        &self,
        user_id: Uuid,
        recent_errors: &[TajweedError],
        session_count: u32,
    ) -> Result<Vec<crate::progress_tracker::ImprovementRecommendation>> {
        let user_progress = self.get_user_progress_data(user_id).await?;
        self.improvement_engine.generate_recommendations(&user_progress, recent_errors, session_count)
    }
    
    /// Create a personalized learning plan
    pub async fn create_learning_plan(
        &self,
        user_id: Uuid,
        target_duration_weeks: u32,
        daily_practice_minutes: u32,
    ) -> Result<LearningPlan> {
        let user_progress = self.get_user_progress_data(user_id).await?;
        self.improvement_engine.create_learning_plan(&user_progress, target_duration_weeks, daily_practice_minutes)
    }
    
    /// Generate adaptive recommendations based on recent performance
    pub async fn generate_adaptive_recommendations(&self, user_id: Uuid) -> Result<Vec<AdaptiveRecommendation>> {
        let user_progress = self.get_user_progress_data(user_id).await?;
        self.improvement_engine.generate_adaptive_recommendations(&user_progress, &user_progress.practice_history)
    }
    
    /// Predict user progress
    pub async fn predict_progress(&self, user_id: Uuid, weeks_ahead: u32) -> Result<ProgressPrediction> {
        let user_progress = self.get_user_progress_data(user_id).await?;
        self.improvement_engine.predict_progress(&user_progress, weeks_ahead)
    }
    
    /// Generate motivational insights
    pub async fn generate_motivational_insights(&self, user_id: Uuid) -> Result<MotivationalInsights> {
        let user_progress = self.get_user_progress_data(user_id).await?;
        self.improvement_engine.generate_motivational_insights(&user_progress, None)
    }
    
    /// Get user's reward status
    pub async fn get_user_reward_status(&self, user_id: Uuid) -> Result<UserRewardStatus> {
        let user_progress = self.get_user_progress_data(user_id).await?;
        self.reward_system.get_user_reward_status(&user_progress)
    }
    
    /// Generate daily goals for a user
    pub async fn generate_daily_goals(&self, user_id: Uuid) -> Result<Vec<DailyGoal>> {
        let user_progress = self.get_user_progress_data(user_id).await?;
        self.reward_system.generate_daily_goals(&user_progress)
    }
    
    /// Generate weekly goals for a user
    pub async fn generate_weekly_goals(&self, user_id: Uuid) -> Result<Vec<WeeklyGoal>> {
        let user_progress = self.get_user_progress_data(user_id).await?;
        self.reward_system.generate_weekly_goals(&user_progress)
    }
    
    /// Generate challenges for a user
    pub async fn generate_challenges(&self, user_id: Uuid) -> Result<Vec<Challenge>> {
        let user_progress = self.get_user_progress_data(user_id).await?;
        self.reward_system.generate_challenges(&user_progress)
    }
    
    /// Get gamification status for a user
    pub async fn get_gamification_status(&self, user_id: Uuid) -> Result<GamificationStatus> {
        let daily_goals = self.generate_daily_goals(user_id).await?;
        let weekly_goals = self.generate_weekly_goals(user_id).await?;
        let challenges = self.generate_challenges(user_id).await?;
        
        Ok(GamificationStatus {
            user_id,
            current_challenges: challenges,
            leaderboard_position: None, // Would implement leaderboard system
            seasonal_events: Vec::new(), // Would implement seasonal events
            daily_goals,
            weekly_goals,
        })
    }
    
    /// Get comprehensive user dashboard data
    pub async fn get_user_dashboard(&self, user_id: Uuid) -> Result<UserDashboard> {
        let progress_data = self.get_user_progress_data(user_id).await?;
        let performance_stats = self.get_performance_statistics(user_id).await?;
        let reward_status = self.get_user_reward_status(user_id).await?;
        let gamification_status = self.get_gamification_status(user_id).await?;
        let personalized_exercises = self.generate_personalized_exercises(user_id).await?;
        let improvement_recommendations = self.generate_improvement_recommendations(
            user_id, 
            &[], // Would pass recent errors
            progress_data.practice_history.len() as u32
        ).await?;
        let motivational_insights = self.generate_motivational_insights(user_id).await?;
        
        Ok(UserDashboard {
            user_id,
            progress_data,
            performance_stats,
            reward_status,
            gamification_status,
            personalized_exercises,
            improvement_recommendations,
            motivational_insights,
            last_updated: Utc::now(),
        })
    }
    
    // Private helper methods
    
    fn generate_improvements(&self, comparison: &AudioComparisonResult, errors: &[TajweedError]) -> Vec<String> {
        let mut improvements = Vec::new();
        
        if comparison.similarity_score < 0.7 {
            improvements.push("Focus on overall recitation accuracy by practicing with reference recordings".to_string());
        }
        
        if comparison.frequency_correlation < 0.6 {
            improvements.push("Work on pronunciation clarity and vowel articulation".to_string());
        }
        
        if comparison.timing_correlation < 0.6 {
            improvements.push("Practice maintaining consistent rhythm and timing".to_string());
        }
        
        // Add specific improvements based on error types
        let error_types: std::collections::HashSet<_> = errors.iter()
            .map(|e| std::mem::discriminant(&e.error_type))
            .collect();
        
        if error_types.len() > 0 {
            improvements.push("Focus on specific Tajweed rules that need improvement".to_string());
        }
        
        improvements.extend(comparison.recommendations.clone());
        improvements
    }
    
    fn generate_next_steps(&self, overall_score: f64, errors: &[TajweedError]) -> Vec<String> {
        let mut next_steps = Vec::new();
        
        if overall_score < 0.5 {
            next_steps.push("Start with basic pronunciation exercises".to_string());
            next_steps.push("Practice individual letters and short words".to_string());
        } else if overall_score < 0.7 {
            next_steps.push("Focus on specific Tajweed rules".to_string());
            next_steps.push("Practice with longer verses".to_string());
        } else {
            next_steps.push("Continue practicing to maintain excellence".to_string());
            next_steps.push("Try more challenging verses".to_string());
        }
        
        if !errors.is_empty() {
            next_steps.push("Review the detected errors and practice corrections".to_string());
        }
        
        next_steps
    }
}
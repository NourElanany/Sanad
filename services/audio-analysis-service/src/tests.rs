use crate::audio_processor::AudioProcessor;
use crate::audio_recorder::AudioRecorder;
use crate::spectrum_analyzer::{SpectrumAnalyzer, WindowType};
use crate::comparison_engine::ComparisonEngine;
use crate::service::AudioAnalysisService;
use crate::models::*;
use std::f32::consts::PI;
use uuid::Uuid;
use shared::{TajweedError, TajweedErrorType, ErrorSeverity};

/// Generate a sine wave for testing
fn generate_sine_wave(frequency: f32, sample_rate: u32, duration: f32) -> Vec<f32> {
    let num_samples = (sample_rate as f32 * duration) as usize;
    (0..num_samples)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            (2.0 * PI * frequency * t).sin()
        })
        .collect()
}

#[tokio::test]
async fn test_audio_processor_initialization() {
    let processor = AudioProcessor::new().await;
    assert!(processor.is_ok(), "Audio processor should initialize successfully");
}

#[test]
fn test_audio_recorder_session_management() {
    let recorder = AudioRecorder::new();
    
    // Test starting a session
    let session_id = recorder.start_session();
    assert!(session_id.is_ok(), "Should be able to start a recording session");
    assert!(recorder.is_recording(), "Should be in recording state");
    
    // Test that we can't start another session
    let second_session = recorder.start_session();
    assert!(second_session.is_err(), "Should not be able to start multiple sessions");
    
    // Wait a bit and stop
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let duration = recorder.stop_session();
    assert!(duration.is_ok(), "Should be able to stop recording session");
    assert!(!recorder.is_recording(), "Should not be in recording state after stop");
}

#[test]
fn test_spectrum_analyzer_basic_functionality() {
    let analyzer = SpectrumAnalyzer::new(1024, 0.5, WindowType::Hanning);
    let sample_rate = 44100;
    
    // Generate a test signal
    let samples = generate_sine_wave(440.0, sample_rate, 1.0);
    
    let spectrum = analyzer.analyze(&samples, sample_rate);
    assert!(spectrum.is_ok(), "Spectrum analysis should succeed");
    
    let spectrum = spectrum.unwrap();
    assert_eq!(spectrum.sample_rate, sample_rate);
    assert!(!spectrum.frequencies.is_empty(), "Should have frequency data");
    assert!(!spectrum.magnitudes.is_empty(), "Should have magnitude data");
    assert_eq!(spectrum.frequencies.len(), spectrum.magnitudes.len());
}

#[test]
fn test_spectrum_analyzer_features() {
    let analyzer = SpectrumAnalyzer::new_for_speech();
    let sample_rate = 44100;
    
    // Generate a complex signal with multiple frequencies
    let mut samples = generate_sine_wave(500.0, sample_rate, 1.0);
    let samples2 = generate_sine_wave(1500.0, sample_rate, 1.0);
    
    for (i, s2) in samples2.iter().enumerate() {
        if i < samples.len() {
            samples[i] += s2;
        }
    }
    
    let spectrum = analyzer.analyze(&samples, sample_rate).unwrap();
    
    // Test spectral features
    let centroid = analyzer.spectral_centroid(&spectrum);
    assert!(centroid > 0.0, "Spectral centroid should be positive");
    
    let rolloff = analyzer.spectral_rolloff(&spectrum, 0.85);
    assert!(rolloff > 0.0, "Spectral rolloff should be positive");
    
    let zcr = analyzer.zero_crossing_rate(&samples);
    assert!(zcr > 0.0, "Zero crossing rate should be positive");
    
    let formants = analyzer.extract_formants(&spectrum, 3);
    assert!(!formants.is_empty(), "Should extract some formants");
}

#[test]
fn test_comparison_engine_identical_signals() {
    let engine = ComparisonEngine::new();
    let sample_rate = 44100;
    
    // Generate identical signals
    let samples = generate_sine_wave(440.0, sample_rate, 1.0);
    
    let result = engine.compare_recordings(&samples, &samples, sample_rate);
    assert!(result.is_ok(), "Comparison should succeed");
    
    let comparison = result.unwrap();
    assert!(comparison.similarity_score > 0.95, "Identical signals should have high similarity");
    assert!(comparison.frequency_correlation > 0.95, "Should have high frequency correlation");
}

#[test]
fn test_comparison_engine_different_signals() {
    let engine = ComparisonEngine::new();
    let sample_rate = 44100;
    
    // Generate different signals
    let samples1 = generate_sine_wave(440.0, sample_rate, 1.0);
    let samples2 = generate_sine_wave(880.0, sample_rate, 1.0); // One octave higher
    
    let result = engine.compare_recordings(&samples1, &samples2, sample_rate);
    assert!(result.is_ok(), "Comparison should succeed");
    
    let comparison = result.unwrap();
    assert!(comparison.similarity_score < 0.8, "Different signals should have lower similarity");
}

#[test]
fn test_tajweed_error_detection() {
    let engine = ComparisonEngine::new();
    let sample_rate = 44100;
    
    // Generate test signals with different characteristics
    let user_samples = generate_sine_wave(440.0, sample_rate, 1.0);
    let reference_samples = generate_sine_wave(440.0, sample_rate, 1.5); // Different duration
    
    let result = engine.detect_tajweed_errors(&user_samples, &reference_samples, sample_rate);
    assert!(result.is_ok(), "Error detection should succeed");
    
    let errors = result.unwrap();
    // Should detect timing/duration error due to different lengths
    assert!(!errors.is_empty(), "Should detect some errors");
}

#[test]
fn test_audio_recorder_validation() {
    let recorder = AudioRecorder::new();
    
    // Test valid parameters
    assert!(recorder.validate_recording_params(1, 1, 7).is_ok());
    assert!(recorder.validate_recording_params(2, 1, 286).is_ok());
    
    // Test invalid surah number
    assert!(recorder.validate_recording_params(0, 1, 1).is_err());
    assert!(recorder.validate_recording_params(115, 1, 1).is_err());
    
    // Test invalid ayah numbers
    assert!(recorder.validate_recording_params(1, 0, 1).is_err());
    assert!(recorder.validate_recording_params(1, 5, 3).is_err());
}

#[test]
fn test_audio_recorder_duration_limits() {
    let mut recorder = AudioRecorder::new();
    
    // Test setting limits
    recorder.set_max_duration(std::time::Duration::from_secs(60));
    recorder.set_min_duration(std::time::Duration::from_millis(500));
    
    let (min, max) = recorder.get_limits();
    assert_eq!(min, std::time::Duration::from_millis(500));
    assert_eq!(max, std::time::Duration::from_secs(60));
}

// === TRACKING AND IMPROVEMENT SYSTEM TESTS ===

#[tokio::test]
async fn test_user_progress_tracking() {
    let service = AudioAnalysisService::new().await.unwrap();
    let user_id = Uuid::new_v4();
    
    // Initialize user progress
    let result = service.initialize_user_progress(user_id).await;
    assert!(result.is_ok());
    
    // Update progress
    let errors = vec![
        TajweedError {
            error_type: TajweedErrorType::Ghunnah,
            start_time: 1.0,
            end_time: 2.0,
            severity: ErrorSeverity::Minor,
            description: "Weak Ghunnah".to_string(),
            correction_suggestion: "Increase nasal resonance".to_string(),
            reference_audio_path: None,
        }
    ];
    
    let update_result = service.update_user_progress(user_id, 1, 1, 0.8, &errors, 20).await;
    assert!(update_result.is_ok());
    
    let update = update_result.unwrap();
    // First update might or might not have achievements
    
    // Get progress data
    let progress_result = service.get_user_progress_data(user_id).await;
    assert!(progress_result.is_ok());
    
    let progress = progress_result.unwrap();
    assert_eq!(progress.user_id, user_id);
    assert_eq!(progress.overall_stats.total_recordings, 1);
    assert_eq!(progress.overall_stats.best_score, 0.8);
}

#[tokio::test]
async fn test_personalized_exercises() {
    let service = AudioAnalysisService::new().await.unwrap();
    let user_id = Uuid::new_v4();
    
    // Initialize user progress
    service.initialize_user_progress(user_id).await.unwrap();
    
    // Generate exercises
    let result = service.generate_personalized_exercises(user_id).await;
    assert!(result.is_ok());
    
    let exercises = result.unwrap();
    assert!(!exercises.is_empty());
    
    // Exercises should have proper structure
    for exercise in &exercises {
        assert!(!exercise.title.is_empty());
        assert!(!exercise.description.is_empty());
        assert!(!exercise.instructions.is_empty());
        assert!(exercise.estimated_duration_minutes > 0);
    }
}

#[tokio::test]
async fn test_improvement_recommendations() {
    let service = AudioAnalysisService::new().await.unwrap();
    let user_id = Uuid::new_v4();
    
    // Initialize user progress
    service.initialize_user_progress(user_id).await.unwrap();
    
    // Generate recommendations
    let result = service.generate_improvement_recommendations(user_id, &[], 5).await;
    assert!(result.is_ok());
    
    let recommendations = result.unwrap();
    assert!(!recommendations.is_empty());
    
    // Recommendations should have proper structure
    for rec in &recommendations {
        assert!(!rec.title.is_empty());
        assert!(!rec.description.is_empty());
        assert!(rec.estimated_time_minutes > 0);
        assert!(!rec.exercises.is_empty());
    }
}

#[tokio::test]
async fn test_learning_plan_creation() {
    let service = AudioAnalysisService::new().await.unwrap();
    let user_id = Uuid::new_v4();
    
    // Initialize user progress
    service.initialize_user_progress(user_id).await.unwrap();
    
    // Create learning plan
    let result = service.create_learning_plan(user_id, 12, 30).await;
    assert!(result.is_ok());
    
    let plan = result.unwrap();
    assert_eq!(plan.user_id, user_id);
    assert_eq!(plan.estimated_duration_weeks, 12);
    assert_eq!(plan.daily_practice_minutes, 30);
    assert!(!plan.phases.is_empty());
    
    // Phases should be properly structured
    for (i, phase) in plan.phases.iter().enumerate() {
        assert_eq!(phase.phase_number, (i + 1) as u32);
        assert!(!phase.title.is_empty());
        assert!(!phase.focus_skills.is_empty());
        assert!(!phase.exercises.is_empty());
    }
}

#[tokio::test]
async fn test_reward_system() {
    let service = AudioAnalysisService::new().await.unwrap();
    let user_id = Uuid::new_v4();
    
    // Initialize user progress
    service.initialize_user_progress(user_id).await.unwrap();
    
    // Get reward status
    let result = service.get_user_reward_status(user_id).await;
    assert!(result.is_ok());
    
    let status = result.unwrap();
    assert_eq!(status.user_id, user_id);
    assert!(status.current_level >= 1);
    assert!(!status.level_title.is_empty());
}

#[tokio::test]
async fn test_daily_goals() {
    let service = AudioAnalysisService::new().await.unwrap();
    let user_id = Uuid::new_v4();
    
    // Initialize user progress
    service.initialize_user_progress(user_id).await.unwrap();
    
    // Generate daily goals
    let result = service.generate_daily_goals(user_id).await;
    assert!(result.is_ok());
    
    let goals = result.unwrap();
    assert!(!goals.is_empty());
    
    // Goals should have proper structure
    for goal in &goals {
        assert!(!goal.description.is_empty());
        assert!(goal.target_value > 0.0);
        assert!(goal.reward_points > 0);
    }
}

#[tokio::test]
async fn test_user_dashboard() {
    let service = AudioAnalysisService::new().await.unwrap();
    let user_id = Uuid::new_v4();
    
    // Initialize user progress
    service.initialize_user_progress(user_id).await.unwrap();
    
    // Get dashboard
    let result = service.get_user_dashboard(user_id).await;
    assert!(result.is_ok());
    
    let dashboard = result.unwrap();
    assert_eq!(dashboard.user_id, user_id);
    assert!(!dashboard.personalized_exercises.is_empty());
    assert!(!dashboard.improvement_recommendations.is_empty());
    assert!(!dashboard.motivational_insights.encouragement_message.is_empty());
}

/// Property-based test for spectrum analysis consistency
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_spectrum_analysis_properties(
            frequency in 100.0f32..2000.0f32,
            sample_rate in 8000u32..48000u32,
            duration in 0.1f32..2.0f32
        ) {
            let analyzer = SpectrumAnalyzer::new_for_speech();
            let samples = generate_sine_wave(frequency, sample_rate, duration);
            
            if samples.len() >= 2048 { // Minimum window size
                let result = analyzer.analyze(&samples, sample_rate);
                prop_assert!(result.is_ok(), "Spectrum analysis should always succeed for valid inputs");
                
                let spectrum = result.unwrap();
                prop_assert_eq!(spectrum.sample_rate, sample_rate);
                prop_assert!(!spectrum.frequencies.is_empty());
                prop_assert!(!spectrum.magnitudes.is_empty());
                prop_assert_eq!(spectrum.frequencies.len(), spectrum.magnitudes.len());
                
                // Test spectral features are reasonable
                let centroid = analyzer.spectral_centroid(&spectrum);
                prop_assert!(centroid >= 0.0 && centroid <= sample_rate as f64 / 2.0);
                
                let rolloff = analyzer.spectral_rolloff(&spectrum, 0.85);
                prop_assert!(rolloff >= 0.0 && rolloff <= sample_rate as f64 / 2.0);
            }
        }
        
        #[test]
        fn test_comparison_symmetry(
            freq1 in 200.0f32..1000.0f32,
            freq2 in 200.0f32..1000.0f32,
            sample_rate in 16000u32..44100u32
        ) {
            let engine = ComparisonEngine::new();
            let samples1 = generate_sine_wave(freq1, sample_rate, 1.0);
            let samples2 = generate_sine_wave(freq2, sample_rate, 1.0);
            
            let result1 = engine.compare_recordings(&samples1, &samples2, sample_rate);
            let result2 = engine.compare_recordings(&samples2, &samples1, sample_rate);
            
            if result1.is_ok() && result2.is_ok() {
                let comp1 = result1.unwrap();
                let comp2 = result2.unwrap();
                
                // Comparison should be symmetric
                prop_assert!((comp1.similarity_score - comp2.similarity_score).abs() < 0.01);
                prop_assert!((comp1.frequency_correlation - comp2.frequency_correlation).abs() < 0.01);
            }
        }
    }
}
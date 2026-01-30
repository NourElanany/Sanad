use anyhow::{Result, Context};
use std::collections::HashMap;
use tracing::{info, debug, warn};
use shared::{TajweedError, TajweedErrorType, ErrorSeverity, AudioSpectrum};
use crate::spectrum_analyzer::SpectrumAnalyzer;

/// Advanced Tajweed error detection analyzer
pub struct TajweedAnalyzer {
    spectrum_analyzer: SpectrumAnalyzer,
    // Frequency ranges for different Tajweed rules
    ghunnah_freq_range: (f64, f64),
    qalqalah_freq_range: (f64, f64),
    madd_duration_thresholds: HashMap<MaddType, (f64, f64)>, // min, max duration in seconds
}

/// Types of Madd (elongation) in Tajweed
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum MaddType {
    MaddTabii,      // Natural elongation (2 counts)
    MaddWajib,      // Obligatory elongation (4-5 counts)
    MaddJaiz,       // Permissible elongation (2-4-6 counts)
    MaddLazim,      // Necessary elongation (6 counts)
}

/// Detailed analysis result for a specific Tajweed rule
#[derive(Debug, Clone)]
pub struct TajweedRuleAnalysis {
    pub rule_type: TajweedErrorType,
    pub confidence: f64,
    pub detected_value: f64,
    pub expected_range: (f64, f64),
    pub severity: ErrorSeverity,
    pub description: String,
    pub correction_advice: String,
}

impl TajweedAnalyzer {
    /// Create a new Tajweed analyzer
    pub fn new() -> Self {
        let mut madd_duration_thresholds = HashMap::new();
        
        // Duration thresholds in seconds (approximate)
        madd_duration_thresholds.insert(MaddType::MaddTabii, (0.3, 0.5));      // 2 counts
        madd_duration_thresholds.insert(MaddType::MaddWajib, (0.6, 0.8));      // 4-5 counts
        madd_duration_thresholds.insert(MaddType::MaddJaiz, (0.3, 0.9));       // 2-4-6 counts
        madd_duration_thresholds.insert(MaddType::MaddLazim, (0.9, 1.2));      // 6 counts
        
        Self {
            spectrum_analyzer: SpectrumAnalyzer::new_for_speech(),
            ghunnah_freq_range: (800.0, 1200.0),    // Nasal resonance frequency range
            qalqalah_freq_range: (100.0, 400.0),    // Low frequency plosive sounds
            madd_duration_thresholds,
        }
    }
    
    /// Comprehensive Tajweed error detection
    pub fn detect_comprehensive_errors(
        &self,
        user_samples: &[f32],
        reference_samples: &[f32],
        sample_rate: u32,
        expected_text: &str,
    ) -> Result<Vec<TajweedError>> {
        let mut errors = Vec::new();
        
        info!("Starting comprehensive Tajweed analysis");
        
        // Analyze both audio samples
        let user_spectrum = self.spectrum_analyzer.analyze(user_samples, sample_rate)?;
        let reference_spectrum = self.spectrum_analyzer.analyze(reference_samples, sample_rate)?;
        
        // Detect different types of Tajweed errors
        errors.extend(self.detect_ghunnah_errors(&user_spectrum, &reference_spectrum, sample_rate)?);
        errors.extend(self.detect_qalqalah_errors(&user_spectrum, &reference_spectrum, sample_rate)?);
        errors.extend(self.detect_madd_errors(user_samples, reference_samples, sample_rate, expected_text)?);
        errors.extend(self.detect_idgham_errors(&user_spectrum, &reference_spectrum, sample_rate)?);
        errors.extend(self.detect_ikhfa_errors(&user_spectrum, &reference_spectrum, sample_rate)?);
        errors.extend(self.detect_pronunciation_errors(&user_spectrum, &reference_spectrum, sample_rate)?);
        errors.extend(self.detect_timing_errors(user_samples, reference_samples, sample_rate)?);
        
        info!("Detected {} Tajweed errors", errors.len());
        Ok(errors)
    }
    
    /// Detect Ghunnah (nasal sound) errors
    pub fn detect_ghunnah_errors(
        &self,
        user_spectrum: &AudioSpectrum,
        reference_spectrum: &AudioSpectrum,
        sample_rate: u32,
    ) -> Result<Vec<TajweedError>> {
        let mut errors = Vec::new();
        
        // Calculate energy in the nasal frequency range
        let user_ghunnah_energy = self.calculate_energy_in_range(user_spectrum, &self.ghunnah_freq_range);
        let reference_ghunnah_energy = self.calculate_energy_in_range(reference_spectrum, &self.ghunnah_freq_range);
        
        if reference_ghunnah_energy > 0.0 {
            let energy_ratio = user_ghunnah_energy / reference_ghunnah_energy;
            
            // Check for insufficient Ghunnah
            if energy_ratio < 0.6 {
                errors.push(TajweedError {
                    error_type: TajweedErrorType::Ghunnah,
                    start_time: 0.0,
                    end_time: user_spectrum.window_size as f64 / sample_rate as f64,
                    severity: if energy_ratio < 0.3 { ErrorSeverity::Major } else { ErrorSeverity::Moderate },
                    description: format!("Insufficient Ghunnah (nasal sound). Detected {:.1}% of expected intensity", energy_ratio * 100.0),
                    correction_suggestion: "Practice proper nasal resonance. Ensure the sound resonates in the nasal cavity for letters with Ghunnah (ن، م with Tanween or Sukoon)".to_string(),
                    reference_audio_path: None,
                });
            }
            // Check for excessive Ghunnah
            else if energy_ratio > 1.5 {
                errors.push(TajweedError {
                    error_type: TajweedErrorType::Ghunnah,
                    start_time: 0.0,
                    end_time: user_spectrum.window_size as f64 / sample_rate as f64,
                    severity: if energy_ratio > 2.0 { ErrorSeverity::Major } else { ErrorSeverity::Moderate },
                    description: format!("Excessive Ghunnah. Detected {:.1}% of expected intensity", energy_ratio * 100.0),
                    correction_suggestion: "Reduce nasal resonance. Ghunnah should be present but not overpowering".to_string(),
                    reference_audio_path: None,
                });
            }
        }
        
        debug!("Detected {} Ghunnah errors", errors.len());
        Ok(errors)
    }
    
    /// Detect Qalqalah (echoing sound) errors
    pub fn detect_qalqalah_errors(
        &self,
        user_spectrum: &AudioSpectrum,
        reference_spectrum: &AudioSpectrum,
        sample_rate: u32,
    ) -> Result<Vec<TajweedError>> {
        let mut errors = Vec::new();
        
        // Qalqalah letters: ق ط ب ج د
        // These should have a distinctive low-frequency burst followed by a brief echo
        
        let user_qalqalah_energy = self.calculate_energy_in_range(user_spectrum, &self.qalqalah_freq_range);
        let reference_qalqalah_energy = self.calculate_energy_in_range(reference_spectrum, &self.qalqalah_freq_range);
        
        if reference_qalqalah_energy > 0.0 {
            let energy_ratio = user_qalqalah_energy / reference_qalqalah_energy;
            
            if energy_ratio < 0.7 {
                errors.push(TajweedError {
                    error_type: TajweedErrorType::Qalqalah,
                    start_time: 0.0,
                    end_time: user_spectrum.window_size as f64 / sample_rate as f64,
                    severity: if energy_ratio < 0.4 { ErrorSeverity::Major } else { ErrorSeverity::Moderate },
                    description: format!("Weak Qalqalah. Detected {:.1}% of expected intensity", energy_ratio * 100.0),
                    correction_suggestion: "Practice the echoing sound for Qalqalah letters (ق ط ب ج د). The sound should bounce back briefly after pronunciation".to_string(),
                    reference_audio_path: None,
                });
            }
        }
        
        debug!("Detected {} Qalqalah errors", errors.len());
        Ok(errors)
    }
    
    /// Detect Madd (elongation) errors
    pub fn detect_madd_errors(
        &self,
        user_samples: &[f32],
        reference_samples: &[f32],
        sample_rate: u32,
        expected_text: &str,
    ) -> Result<Vec<TajweedError>> {
        let mut errors = Vec::new();
        
        // Analyze vowel segments for duration
        let user_vowel_segments = self.detect_vowel_segments(user_samples, sample_rate)?;
        let reference_vowel_segments = self.detect_vowel_segments(reference_samples, sample_rate)?;
        
        // Determine expected Madd type from text analysis (simplified)
        let expected_madd_type = self.analyze_madd_type_from_text(expected_text);
        
        if let Some(madd_type) = expected_madd_type {
            let (min_duration, max_duration) = self.madd_duration_thresholds.get(&madd_type)
                .unwrap_or(&(0.3, 0.5));
            
            for (i, user_segment) in user_vowel_segments.iter().enumerate() {
                if let Some(reference_segment) = reference_vowel_segments.get(i) {
                    let duration_ratio = user_segment.duration / reference_segment.duration;
                    
                    if user_segment.duration < *min_duration || user_segment.duration > *max_duration {
                        let severity = if duration_ratio < 0.5 || duration_ratio > 2.0 {
                            ErrorSeverity::Major
                        } else {
                            ErrorSeverity::Moderate
                        };
                        
                        errors.push(TajweedError {
                            error_type: TajweedErrorType::Madd,
                            start_time: user_segment.start_time,
                            end_time: user_segment.end_time,
                            severity,
                            description: format!("Incorrect Madd duration. Expected {:.1}-{:.1}s, got {:.1}s", 
                                               min_duration, max_duration, user_segment.duration),
                            correction_suggestion: format!("Practice proper {:?} duration. Count the beats correctly", madd_type),
                            reference_audio_path: None,
                        });
                    }
                }
            }
        }
        
        debug!("Detected {} Madd errors", errors.len());
        Ok(errors)
    }
    
    /// Detect Idgham (merging) errors
    pub fn detect_idgham_errors(
        &self,
        user_spectrum: &AudioSpectrum,
        reference_spectrum: &AudioSpectrum,
        sample_rate: u32,
    ) -> Result<Vec<TajweedError>> {
        let mut errors = Vec::new();
        
        // Idgham involves merging sounds, which should show smooth spectral transitions
        let user_spectral_smoothness = self.calculate_spectral_smoothness(user_spectrum);
        let reference_spectral_smoothness = self.calculate_spectral_smoothness(reference_spectrum);
        
        let smoothness_ratio = user_spectral_smoothness / reference_spectral_smoothness;
        
        if smoothness_ratio < 0.7 {
            errors.push(TajweedError {
                error_type: TajweedErrorType::Idgham,
                start_time: 0.0,
                end_time: user_spectrum.window_size as f64 / sample_rate as f64,
                severity: if smoothness_ratio < 0.5 { ErrorSeverity::Major } else { ErrorSeverity::Moderate },
                description: format!("Rough Idgham transition. Smoothness: {:.1}% of expected", smoothness_ratio * 100.0),
                correction_suggestion: "Practice smooth merging of sounds in Idgham. The transition should be seamless".to_string(),
                reference_audio_path: None,
            });
        }
        
        debug!("Detected {} Idgham errors", errors.len());
        Ok(errors)
    }
    
    /// Detect Ikhfa (hiding) errors
    pub fn detect_ikhfa_errors(
        &self,
        user_spectrum: &AudioSpectrum,
        reference_spectrum: &AudioSpectrum,
        sample_rate: u32,
    ) -> Result<Vec<TajweedError>> {
        let mut errors = Vec::new();
        
        // Ikhfa involves partial hiding of the noon sound, creating a specific spectral pattern
        let user_mid_freq_energy = self.calculate_energy_in_range(user_spectrum, &(1000.0, 2000.0));
        let reference_mid_freq_energy = self.calculate_energy_in_range(reference_spectrum, &(1000.0, 2000.0));
        
        if reference_mid_freq_energy > 0.0 {
            let energy_ratio = user_mid_freq_energy / reference_mid_freq_energy;
            
            // Ikhfa should have moderate energy in mid frequencies
            if energy_ratio < 0.6 || energy_ratio > 1.4 {
                errors.push(TajweedError {
                    error_type: TajweedErrorType::Ikhfa,
                    start_time: 0.0,
                    end_time: user_spectrum.window_size as f64 / sample_rate as f64,
                    severity: ErrorSeverity::Moderate,
                    description: format!("Incorrect Ikhfa pronunciation. Energy ratio: {:.1}%", energy_ratio * 100.0),
                    correction_suggestion: "Practice proper Ikhfa. The noon sound should be partially hidden, not completely merged or fully pronounced".to_string(),
                    reference_audio_path: None,
                });
            }
        }
        
        debug!("Detected {} Ikhfa errors", errors.len());
        Ok(errors)
    }
    
    /// Detect general pronunciation errors
    pub fn detect_pronunciation_errors(
        &self,
        user_spectrum: &AudioSpectrum,
        reference_spectrum: &AudioSpectrum,
        sample_rate: u32,
    ) -> Result<Vec<TajweedError>> {
        let mut errors = Vec::new();
        
        // Extract formants for vowel analysis
        let user_formants = self.spectrum_analyzer.extract_formants(user_spectrum, 3);
        let reference_formants = self.spectrum_analyzer.extract_formants(reference_spectrum, 3);
        
        if user_formants.len() >= 2 && reference_formants.len() >= 2 {
            let f1_diff = (user_formants[0] - reference_formants[0]).abs();
            let f2_diff = (user_formants[1] - reference_formants[1]).abs();
            
            // Thresholds for Arabic vowels (more strict than general speech)
            if f1_diff > 80.0 || f2_diff > 150.0 {
                let severity = if f1_diff > 150.0 || f2_diff > 250.0 {
                    ErrorSeverity::Major
                } else {
                    ErrorSeverity::Moderate
                };
                
                errors.push(TajweedError {
                    error_type: TajweedErrorType::Pronunciation,
                    start_time: 0.0,
                    end_time: user_spectrum.window_size as f64 / sample_rate as f64,
                    severity,
                    description: format!("Vowel pronunciation error. F1 diff: {:.0}Hz, F2 diff: {:.0}Hz", f1_diff, f2_diff),
                    correction_suggestion: "Focus on proper Arabic vowel pronunciation. Pay attention to tongue position and mouth opening".to_string(),
                    reference_audio_path: None,
                });
            }
        }
        
        debug!("Detected {} pronunciation errors", errors.len());
        Ok(errors)
    }
    
    /// Detect timing and rhythm errors
    pub fn detect_timing_errors(
        &self,
        user_samples: &[f32],
        reference_samples: &[f32],
        sample_rate: u32,
    ) -> Result<Vec<TajweedError>> {
        let mut errors = Vec::new();
        
        // Analyze rhythm and timing patterns
        let user_rhythm = self.analyze_rhythm_pattern(user_samples, sample_rate)?;
        let reference_rhythm = self.analyze_rhythm_pattern(reference_samples, sample_rate)?;
        
        let rhythm_similarity = self.calculate_rhythm_similarity(&user_rhythm, &reference_rhythm);
        
        if rhythm_similarity < 0.7 {
            errors.push(TajweedError {
                error_type: TajweedErrorType::Timing,
                start_time: 0.0,
                end_time: user_samples.len() as f64 / sample_rate as f64,
                severity: if rhythm_similarity < 0.5 { ErrorSeverity::Major } else { ErrorSeverity::Moderate },
                description: format!("Timing and rhythm error. Similarity: {:.1}%", rhythm_similarity * 100.0),
                correction_suggestion: "Practice maintaining consistent rhythm and proper timing between syllables".to_string(),
                reference_audio_path: None,
            });
        }
        
        debug!("Detected {} timing errors", errors.len());
        Ok(errors)
    }
    
    /// Generate a quality score based on detected errors
    pub fn calculate_quality_score(&self, errors: &[TajweedError]) -> f64 {
        if errors.is_empty() {
            return 1.0;
        }
        
        let mut penalty = 0.0;
        
        for error in errors {
            let error_penalty = match error.severity {
                ErrorSeverity::Minor => 0.05,
                ErrorSeverity::Moderate => 0.10,
                ErrorSeverity::Major => 0.20,
            };
            penalty += error_penalty;
        }
        
        (1.0_f64 - penalty).max(0.0)
    }
    
    // Helper methods
    
    fn calculate_energy_in_range(&self, spectrum: &AudioSpectrum, freq_range: &(f64, f64)) -> f64 {
        spectrum.frequencies
            .iter()
            .zip(spectrum.magnitudes.iter())
            .filter(|(freq, _)| **freq >= freq_range.0 && **freq <= freq_range.1)
            .map(|(_, mag)| mag * mag)
            .sum()
    }
    
    fn calculate_spectral_smoothness(&self, spectrum: &AudioSpectrum) -> f64 {
        if spectrum.magnitudes.len() < 2 {
            return 0.0;
        }
        
        let mut smoothness = 0.0;
        for i in 1..spectrum.magnitudes.len() {
            let diff = (spectrum.magnitudes[i] - spectrum.magnitudes[i-1]).abs();
            smoothness += diff;
        }
        
        1.0 / (1.0 + smoothness / spectrum.magnitudes.len() as f64)
    }
    
    fn detect_vowel_segments(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<VowelSegment>> {
        let mut segments = Vec::new();
        
        // Simple vowel detection based on energy and spectral characteristics
        let window_size = sample_rate as usize / 10; // 100ms windows
        let mut i = 0;
        
        while i + window_size < samples.len() {
            let window = &samples[i..i + window_size];
            let energy = window.iter().map(|s| s * s).sum::<f32>();
            
            // If energy is above threshold, consider it a vowel segment
            if energy > 0.01 {
                let start_time = i as f64 / sample_rate as f64;
                let end_time = (i + window_size) as f64 / sample_rate as f64;
                let duration = end_time - start_time;
                
                segments.push(VowelSegment {
                    start_time,
                    end_time,
                    duration,
                    energy: energy as f64,
                });
            }
            
            i += window_size / 2; // 50% overlap
        }
        
        Ok(segments)
    }
    
    fn analyze_madd_type_from_text(&self, text: &str) -> Option<MaddType> {
        // Simplified text analysis for Madd type detection
        // In a real implementation, this would use Arabic NLP
        
        if text.contains("آ") || text.contains("ا") {
            Some(MaddType::MaddTabii)
        } else if text.contains("ء") {
            Some(MaddType::MaddWajib)
        } else {
            None
        }
    }
    
    fn analyze_rhythm_pattern(&self, samples: &[f32], sample_rate: u32) -> Result<Vec<f64>> {
        let mut rhythm_pattern = Vec::new();
        
        // Analyze energy patterns over time
        let window_size = sample_rate as usize / 20; // 50ms windows
        let mut i = 0;
        
        while i + window_size < samples.len() {
            let window = &samples[i..i + window_size];
            let energy = window.iter().map(|s| s * s).sum::<f32>();
            rhythm_pattern.push(energy as f64);
            i += window_size;
        }
        
        Ok(rhythm_pattern)
    }
    
    fn calculate_rhythm_similarity(&self, pattern1: &[f64], pattern2: &[f64]) -> f64 {
        if pattern1.is_empty() || pattern2.is_empty() {
            return 0.0;
        }
        
        let min_len = pattern1.len().min(pattern2.len());
        let mut correlation = 0.0;
        
        for i in 0..min_len {
            correlation += (pattern1[i] - pattern2[i]).abs();
        }
        
        1.0 / (1.0 + correlation / min_len as f64)
    }
}

/// Represents a detected vowel segment in audio
#[derive(Debug, Clone)]
struct VowelSegment {
    start_time: f64,
    end_time: f64,
    duration: f64,
    energy: f64,
}

impl Default for TajweedAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;
    
    fn generate_sine_wave(frequency: f32, sample_rate: u32, duration: f32) -> Vec<f32> {
        let num_samples = (sample_rate as f32 * duration) as usize;
        (0..num_samples)
            .map(|i| {
                let t = i as f32 / sample_rate as f32;
                (2.0 * PI * frequency * t).sin()
            })
            .collect()
    }
    
    #[test]
    fn test_tajweed_analyzer_creation() {
        let analyzer = TajweedAnalyzer::new();
        assert_eq!(analyzer.ghunnah_freq_range, (800.0, 1200.0));
        assert_eq!(analyzer.qalqalah_freq_range, (100.0, 400.0));
        assert!(analyzer.madd_duration_thresholds.contains_key(&MaddType::MaddTabii));
    }
    
    #[test]
    fn test_quality_score_calculation() {
        let analyzer = TajweedAnalyzer::new();
        
        // No errors should give perfect score
        let no_errors = vec![];
        assert_eq!(analyzer.calculate_quality_score(&no_errors), 1.0);
        
        // Minor error should reduce score slightly
        let minor_error = vec![TajweedError {
            error_type: TajweedErrorType::Pronunciation,
            start_time: 0.0,
            end_time: 1.0,
            severity: ErrorSeverity::Minor,
            description: "Test error".to_string(),
            correction_suggestion: "Test correction".to_string(),
            reference_audio_path: None,
        }];
        assert_eq!(analyzer.calculate_quality_score(&minor_error), 0.95);
        
        // Major error should reduce score significantly
        let major_error = vec![TajweedError {
            error_type: TajweedErrorType::Ghunnah,
            start_time: 0.0,
            end_time: 1.0,
            severity: ErrorSeverity::Major,
            description: "Test major error".to_string(),
            correction_suggestion: "Test correction".to_string(),
            reference_audio_path: None,
        }];
        assert_eq!(analyzer.calculate_quality_score(&major_error), 0.8);
    }
    
    #[test]
    fn test_vowel_segment_detection() {
        let analyzer = TajweedAnalyzer::new();
        let sample_rate = 44100;
        
        // Generate a signal with clear vowel-like segments
        let samples = generate_sine_wave(440.0, sample_rate, 1.0);
        
        let segments = analyzer.detect_vowel_segments(&samples, sample_rate);
        assert!(segments.is_ok());
        
        let segments = segments.unwrap();
        assert!(!segments.is_empty());
        
        // Check that segments have reasonable properties
        for segment in &segments {
            assert!(segment.duration > 0.0);
            assert!(segment.end_time > segment.start_time);
            assert!(segment.energy > 0.0);
        }
    }
    
    #[test]
    fn test_comprehensive_error_detection() {
        let analyzer = TajweedAnalyzer::new();
        let sample_rate = 44100;
        
        // Generate test signals
        let user_samples = generate_sine_wave(440.0, sample_rate, 1.0);
        let reference_samples = generate_sine_wave(440.0, sample_rate, 1.2); // Different duration
        
        let result = analyzer.detect_comprehensive_errors(
            &user_samples,
            &reference_samples,
            sample_rate,
            "test text"
        );
        
        assert!(result.is_ok());
        let errors = result.unwrap();
        
        // Should detect some timing errors due to different durations
        assert!(!errors.is_empty());
        
        // Check that errors have proper structure
        for error in &errors {
            assert!(!error.description.is_empty());
            assert!(!error.correction_suggestion.is_empty());
            assert!(error.start_time >= 0.0);
            assert!(error.end_time > error.start_time);
        }
    }
}
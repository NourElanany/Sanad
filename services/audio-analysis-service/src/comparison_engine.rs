use anyhow::{Result};
use std::collections::HashMap;
use tracing::{info, debug};
use shared::{AudioComparisonResult, TajweedError, AudioSpectrum};
use crate::spectrum_analyzer::SpectrumAnalyzer;
use crate::tajweed_analyzer::TajweedAnalyzer;

/// Engine for comparing user recordings with reference recordings
pub struct ComparisonEngine {
    spectrum_analyzer: SpectrumAnalyzer,
    tajweed_analyzer: TajweedAnalyzer,
    similarity_threshold: f64,
    error_detection_sensitivity: f64,
}

impl ComparisonEngine {
    /// Create a new comparison engine
    pub fn new() -> Self {
        Self {
            spectrum_analyzer: SpectrumAnalyzer::new_for_speech(),
            tajweed_analyzer: TajweedAnalyzer::new(),
            similarity_threshold: 0.7,
            error_detection_sensitivity: 0.5,
        }
    }
    
    /// Compare two audio recordings and return detailed analysis
    pub fn compare_recordings(
        &self,
        user_samples: &[f32],
        reference_samples: &[f32],
        sample_rate: u32,
    ) -> Result<AudioComparisonResult> {
        if user_samples.is_empty() || reference_samples.is_empty() {
            return Err(anyhow::anyhow!("Cannot compare empty audio samples"));
        }
        
        info!("Comparing recordings: user={} samples, reference={} samples", 
              user_samples.len(), reference_samples.len());
        
        // Analyze spectra for both recordings
        let user_spectrum = self.spectrum_analyzer.analyze(user_samples, sample_rate)?;
        let reference_spectrum = self.spectrum_analyzer.analyze(reference_samples, sample_rate)?;
        
        // Calculate various similarity metrics
        let frequency_correlation = self.calculate_frequency_correlation(&user_spectrum, &reference_spectrum)?;
        let timing_correlation = self.calculate_timing_correlation(user_samples, reference_samples)?;
        let spectral_distance = self.calculate_spectral_distance(&user_spectrum, &reference_spectrum)?;
        
        // Overall similarity score (weighted combination)
        let similarity_score = self.calculate_overall_similarity(
            frequency_correlation,
            timing_correlation,
            spectral_distance,
        );
        
        // Generate recommendations based on analysis
        let recommendations = self.generate_recommendations(
            similarity_score,
            frequency_correlation,
            timing_correlation,
            spectral_distance,
        );
        
        debug!("Comparison results: similarity={:.3}, freq_corr={:.3}, timing_corr={:.3}, spectral_dist={:.3}",
               similarity_score, frequency_correlation, timing_correlation, spectral_distance);
        
        Ok(AudioComparisonResult {
            similarity_score,
            frequency_correlation,
            timing_correlation,
            spectral_distance,
            recommendations,
        })
    }
    
    /// Detect Tajweed errors in user recording compared to reference
    pub fn detect_tajweed_errors(
        &self,
        user_samples: &[f32],
        reference_samples: &[f32],
        sample_rate: u32,
    ) -> Result<Vec<TajweedError>> {
        // Use the advanced Tajweed analyzer for comprehensive error detection
        self.tajweed_analyzer.detect_comprehensive_errors(
            user_samples,
            reference_samples,
            sample_rate,
            "", // In a real implementation, this would be the expected Arabic text
        )
    }
    
    /// Calculate overall quality score including Tajweed accuracy
    pub fn calculate_tajweed_quality_score(&self, errors: &[TajweedError]) -> f64 {
        self.tajweed_analyzer.calculate_quality_score(errors)
    }
    
    /// Calculate frequency domain correlation between spectra
    fn calculate_frequency_correlation(&self, user_spectrum: &AudioSpectrum, reference_spectrum: &AudioSpectrum) -> Result<f64> {
        if user_spectrum.magnitudes.len() != reference_spectrum.magnitudes.len() {
            return Err(anyhow::anyhow!("Spectrum lengths do not match"));
        }
        
        let correlation = self.pearson_correlation(&user_spectrum.magnitudes, &reference_spectrum.magnitudes);
        Ok(correlation.max(0.0)) // Clamp to positive values
    }
    
    /// Calculate timing correlation using cross-correlation
    fn calculate_timing_correlation(&self, user_samples: &[f32], reference_samples: &[f32]) -> Result<f64> {
        // Normalize lengths for comparison
        let min_len = user_samples.len().min(reference_samples.len());
        let user_normalized = &user_samples[..min_len];
        let reference_normalized = &reference_samples[..min_len];
        
        // Calculate cross-correlation at zero lag
        let correlation = self.cross_correlation_zero_lag(user_normalized, reference_normalized);
        Ok(correlation.abs()) // Take absolute value for similarity measure
    }
    
    /// Calculate spectral distance (lower is better)
    fn calculate_spectral_distance(&self, user_spectrum: &AudioSpectrum, reference_spectrum: &AudioSpectrum) -> Result<f64> {
        if user_spectrum.magnitudes.len() != reference_spectrum.magnitudes.len() {
            return Err(anyhow::anyhow!("Spectrum lengths do not match"));
        }
        
        // Calculate Euclidean distance between magnitude vectors
        let distance: f64 = user_spectrum.magnitudes
            .iter()
            .zip(reference_spectrum.magnitudes.iter())
            .map(|(u, r)| (u - r).powi(2))
            .sum::<f64>()
            .sqrt();
        
        // Normalize by vector length
        let normalized_distance = distance / user_spectrum.magnitudes.len() as f64;
        Ok(normalized_distance)
    }
    
    /// Calculate overall similarity score
    fn calculate_overall_similarity(&self, freq_corr: f64, timing_corr: f64, spectral_dist: f64) -> f64 {
        // Weighted combination of metrics
        let freq_weight = 0.4;
        let timing_weight = 0.3;
        let spectral_weight = 0.3;
        
        // Convert spectral distance to similarity (inverse relationship)
        let spectral_similarity = 1.0 / (1.0 + spectral_dist);
        
        let similarity = freq_weight * freq_corr + 
                        timing_weight * timing_corr + 
                        spectral_weight * spectral_similarity;
        
        similarity.clamp(0.0, 1.0)
    }
    
    /// Generate recommendations based on analysis results
    fn generate_recommendations(&self, similarity: f64, freq_corr: f64, timing_corr: f64, spectral_dist: f64) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if similarity < 0.5 {
            recommendations.push("Overall recitation needs significant improvement. Consider practicing with a teacher.".to_string());
        } else if similarity < 0.7 {
            recommendations.push("Good progress! Focus on specific areas for improvement.".to_string());
        } else {
            recommendations.push("Excellent recitation! Keep up the good work.".to_string());
        }
        
        if freq_corr < 0.6 {
            recommendations.push("Work on pronunciation accuracy. Pay attention to vowel sounds and consonant articulation.".to_string());
        }
        
        if timing_corr < 0.6 {
            recommendations.push("Focus on timing and rhythm. Practice with a metronome or follow along with reference recordings.".to_string());
        }
        
        if spectral_dist > 1.0 {
            recommendations.push("Your voice characteristics differ significantly from the reference. This is normal, but focus on proper Tajweed rules.".to_string());
        }
        
        recommendations
    }
    
    /// Calculate Pearson correlation coefficient
    fn pearson_correlation(&self, x: &[f64], y: &[f64]) -> f64 {
        if x.len() != y.len() || x.is_empty() {
            return 0.0;
        }
        
        let n = x.len() as f64;
        let sum_x: f64 = x.iter().sum();
        let sum_y: f64 = y.iter().sum();
        let sum_xy: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
        let sum_x2: f64 = x.iter().map(|a| a * a).sum();
        let sum_y2: f64 = y.iter().map(|b| b * b).sum();
        
        let numerator = n * sum_xy - sum_x * sum_y;
        let denominator = ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt();
        
        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }
    
    /// Calculate cross-correlation at zero lag
    fn cross_correlation_zero_lag(&self, x: &[f32], y: &[f32]) -> f64 {
        if x.len() != y.len() || x.is_empty() {
            return 0.0;
        }
        
        let dot_product: f64 = x.iter().zip(y.iter()).map(|(a, b)| (*a as f64) * (*b as f64)).sum();
        let norm_x: f64 = x.iter().map(|a| (*a as f64) * (*a as f64)).sum::<f64>().sqrt();
        let norm_y: f64 = y.iter().map(|b| (*b as f64) * (*b as f64)).sum::<f64>().sqrt();
        
        if norm_x == 0.0 || norm_y == 0.0 {
            0.0
        } else {
            dot_product / (norm_x * norm_y)
        }
    }
    
    /// Set similarity threshold
    pub fn set_similarity_threshold(&mut self, threshold: f64) {
        self.similarity_threshold = threshold.clamp(0.0, 1.0);
    }
    
    /// Set error detection sensitivity
    pub fn set_error_detection_sensitivity(&mut self, sensitivity: f64) {
        self.error_detection_sensitivity = sensitivity.clamp(0.0, 1.0);
    }
}

impl Default for ComparisonEngine {
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
    fn test_comparison_engine_creation() {
        let engine = ComparisonEngine::new();
        assert_eq!(engine.similarity_threshold, 0.7);
        assert_eq!(engine.error_detection_sensitivity, 0.5);
    }
    
    #[test]
    fn test_identical_recordings_comparison() {
        let engine = ComparisonEngine::new();
        let sample_rate = 44100;
        
        // Generate identical sine waves
        let samples = generate_sine_wave(440.0, sample_rate, 1.0);
        
        let result = engine.compare_recordings(&samples, &samples, sample_rate);
        assert!(result.is_ok());
        
        let comparison = result.unwrap();
        assert!(comparison.similarity_score > 0.9); // Should be very similar
        assert!(comparison.frequency_correlation > 0.9);
    }
    
    #[test]
    fn test_different_recordings_comparison() {
        let engine = ComparisonEngine::new();
        let sample_rate = 44100;
        
        // Generate different sine waves
        let samples1 = generate_sine_wave(440.0, sample_rate, 1.0);
        let samples2 = generate_sine_wave(880.0, sample_rate, 1.0); // One octave higher
        
        let result = engine.compare_recordings(&samples1, &samples2, sample_rate);
        assert!(result.is_ok());
        
        let comparison = result.unwrap();
        assert!(comparison.similarity_score < 0.8); // Should be less similar
    }
    
    #[test]
    fn test_tajweed_error_detection() {
        let engine = ComparisonEngine::new();
        let sample_rate = 44100;
        
        // Generate test recordings
        let user_samples = generate_sine_wave(440.0, sample_rate, 1.0);
        let reference_samples = generate_sine_wave(440.0, sample_rate, 1.5); // Different duration
        
        let result = engine.detect_tajweed_errors(&user_samples, &reference_samples, sample_rate);
        assert!(result.is_ok());
        
        let errors = result.unwrap();
        // Should detect timing/duration error
        assert!(!errors.is_empty());
    }
    
    #[test]
    fn test_pearson_correlation() {
        let engine = ComparisonEngine::new();
        
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // Perfect positive correlation
        
        let correlation = engine.pearson_correlation(&x, &y);
        assert!((correlation - 1.0).abs() < 0.001); // Should be very close to 1.0
    }
}
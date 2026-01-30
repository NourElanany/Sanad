use anyhow::Result;
use shared::{TajweedError, TajweedErrorType, ErrorSeverity, AudioComparisonResult};
use std::collections::HashMap;

/// Comprehensive scoring system for Quran recitation quality
pub struct RecitationScoringSystem {
    /// Weights for different aspects of recitation
    weights: ScoringWeights,
    /// Penalty values for different error types
    error_penalties: HashMap<TajweedErrorType, f64>,
}

/// Weights for different scoring aspects
#[derive(Debug, Clone)]
pub struct ScoringWeights {
    pub tajweed_accuracy: f64,
    pub pronunciation: f64,
    pub timing: f64,
    pub fluency: f64,
    pub clarity: f64,
    pub rhythm: f64,
}

/// Detailed scoring breakdown
#[derive(Debug, Clone)]
pub struct DetailedScores {
    pub overall_score: f64,
    pub tajweed_accuracy: f64,
    pub pronunciation_accuracy: f64,
    pub timing_accuracy: f64,
    pub fluency_score: f64,
    pub clarity_score: f64,
    pub rhythm_score: f64,
    pub error_penalty: f64,
    pub bonus_points: f64,
}

/// Performance level based on score
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceLevel {
    Beginner,      // 0.0 - 0.4
    Elementary,    // 0.4 - 0.6
    Intermediate,  // 0.6 - 0.75
    Advanced,      // 0.75 - 0.9
    Expert,        // 0.9 - 1.0
}

impl RecitationScoringSystem {
    /// Create a new scoring system with default weights
    pub fn new() -> Self {
        let weights = ScoringWeights {
            tajweed_accuracy: 0.35,  // Most important for Quran recitation
            pronunciation: 0.25,
            timing: 0.15,
            fluency: 0.10,
            clarity: 0.10,
            rhythm: 0.05,
        };
        
        let mut error_penalties = HashMap::new();
        error_penalties.insert(TajweedErrorType::Ghunnah, 0.15);
        error_penalties.insert(TajweedErrorType::Qalqalah, 0.12);
        error_penalties.insert(TajweedErrorType::Madd, 0.18);
        error_penalties.insert(TajweedErrorType::Idgham, 0.14);
        error_penalties.insert(TajweedErrorType::Ikhfa, 0.13);
        error_penalties.insert(TajweedErrorType::Pronunciation, 0.10);
        error_penalties.insert(TajweedErrorType::Timing, 0.08);
        
        Self {
            weights,
            error_penalties,
        }
    }
    
    /// Calculate comprehensive scores for a recitation
    pub fn calculate_detailed_scores(
        &self,
        comparison_result: &AudioComparisonResult,
        tajweed_errors: &[TajweedError],
        audio_quality_metrics: Option<&AudioQualityMetrics>,
    ) -> Result<DetailedScores> {
        // Base scores from comparison
        let pronunciation_accuracy = comparison_result.frequency_correlation;
        let timing_accuracy = comparison_result.timing_correlation;
        
        // Calculate Tajweed accuracy based on errors
        let tajweed_accuracy = self.calculate_tajweed_accuracy(tajweed_errors);
        
        // Calculate fluency score (based on spectral smoothness and consistency)
        let fluency_score = self.calculate_fluency_score(comparison_result);
        
        // Calculate clarity score (based on spectral distance and quality)
        let clarity_score = self.calculate_clarity_score(comparison_result, audio_quality_metrics);
        
        // Calculate rhythm score (based on timing consistency)
        let rhythm_score = self.calculate_rhythm_score(comparison_result);
        
        // Calculate error penalty
        let error_penalty = self.calculate_error_penalty(tajweed_errors);
        
        // Calculate bonus points for exceptional performance
        let bonus_points = self.calculate_bonus_points(
            tajweed_accuracy,
            pronunciation_accuracy,
            timing_accuracy,
        );
        
        // Calculate weighted overall score
        let base_score = self.weights.tajweed_accuracy * tajweed_accuracy
            + self.weights.pronunciation * pronunciation_accuracy
            + self.weights.timing * timing_accuracy
            + self.weights.fluency * fluency_score
            + self.weights.clarity * clarity_score
            + self.weights.rhythm * rhythm_score;
        
        let overall_score = (base_score - error_penalty + bonus_points).clamp(0.0, 1.0);
        
        Ok(DetailedScores {
            overall_score,
            tajweed_accuracy,
            pronunciation_accuracy,
            timing_accuracy,
            fluency_score,
            clarity_score,
            rhythm_score,
            error_penalty,
            bonus_points,
        })
    }
    
    /// Determine performance level based on overall score
    pub fn determine_performance_level(&self, overall_score: f64) -> PerformanceLevel {
        match overall_score {
            score if score >= 0.9 => PerformanceLevel::Expert,
            score if score >= 0.75 => PerformanceLevel::Advanced,
            score if score >= 0.6 => PerformanceLevel::Intermediate,
            score if score >= 0.4 => PerformanceLevel::Elementary,
            _ => PerformanceLevel::Beginner,
        }
    }
    
    /// Generate personalized feedback based on scores
    pub fn generate_feedback(&self, scores: &DetailedScores) -> Vec<String> {
        let mut feedback = Vec::new();
        
        let level = self.determine_performance_level(scores.overall_score);
        
        // Overall performance feedback
        match level {
            PerformanceLevel::Expert => {
                feedback.push("Excellent recitation! Your Tajweed is very accurate.".to_string());
            }
            PerformanceLevel::Advanced => {
                feedback.push("Very good recitation with minor areas for improvement.".to_string());
            }
            PerformanceLevel::Intermediate => {
                feedback.push("Good progress! Focus on specific Tajweed rules.".to_string());
            }
            PerformanceLevel::Elementary => {
                feedback.push("Keep practicing! Work on basic pronunciation and timing.".to_string());
            }
            PerformanceLevel::Beginner => {
                feedback.push("Start with fundamental Tajweed rules and basic pronunciation.".to_string());
            }
        }
        
        // Specific area feedback
        if scores.tajweed_accuracy < 0.7 {
            feedback.push("Focus on Tajweed rules - this is the most important aspect of Quran recitation.".to_string());
        }
        
        if scores.pronunciation_accuracy < 0.7 {
            feedback.push("Work on Arabic pronunciation. Pay attention to vowel sounds and consonant articulation.".to_string());
        }
        
        if scores.timing_accuracy < 0.7 {
            feedback.push("Practice maintaining consistent timing and rhythm.".to_string());
        }
        
        if scores.fluency_score < 0.7 {
            feedback.push("Work on smooth transitions between words and syllables.".to_string());
        }
        
        if scores.clarity_score < 0.7 {
            feedback.push("Improve voice clarity. Ensure clear articulation of each letter.".to_string());
        }
        
        if scores.error_penalty > 0.2 {
            feedback.push("Multiple Tajweed errors detected. Review the specific errors and practice corrections.".to_string());
        }
        
        if scores.bonus_points > 0.0 {
            feedback.push("Excellent performance in some areas! Keep up the good work.".to_string());
        }
        
        feedback
    }
    
    /// Generate specific practice recommendations
    pub fn generate_practice_recommendations(&self, scores: &DetailedScores, errors: &[TajweedError]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Error-specific recommendations
        let error_counts = self.count_error_types(errors);
        
        for (error_type, count) in error_counts {
            if count > 0 {
                let recommendation = match error_type {
                    TajweedErrorType::Ghunnah => "Practice nasal resonance exercises for letters with Ghunnah (ن، م)",
                    TajweedErrorType::Qalqalah => "Practice the echoing sound for Qalqalah letters (ق ط ب ج د)",
                    TajweedErrorType::Madd => "Practice proper elongation duration for different types of Madd",
                    TajweedErrorType::Idgham => "Practice smooth merging of sounds in Idgham rules",
                    TajweedErrorType::Ikhfa => "Practice partial hiding of noon sound in Ikhfa",
                    TajweedErrorType::Pronunciation => "Focus on correct Arabic letter pronunciation",
                    TajweedErrorType::Timing => "Practice with a metronome to improve timing consistency",
                    TajweedErrorType::Other(_) => "Review specific Tajweed rules mentioned in the errors",
                };
                recommendations.push(format!("{} ({} errors detected)", recommendation, count));
            }
        }
        
        // Score-based recommendations
        if scores.overall_score < 0.5 {
            recommendations.push("Start with basic Tajweed lessons and simple verses".to_string());
            recommendations.push("Practice individual letter pronunciation".to_string());
        } else if scores.overall_score < 0.75 {
            recommendations.push("Focus on specific Tajweed rules that need improvement".to_string());
            recommendations.push("Practice with longer verses to improve consistency".to_string());
        } else {
            recommendations.push("Continue practicing to maintain excellence".to_string());
            recommendations.push("Try more challenging verses or different recitation styles".to_string());
        }
        
        recommendations
    }
    
    // Private helper methods
    
    fn calculate_tajweed_accuracy(&self, errors: &[TajweedError]) -> f64 {
        if errors.is_empty() {
            return 1.0;
        }
        
        let mut penalty = 0.0;
        for error in errors {
            let base_penalty = self.error_penalties.get(&error.error_type).unwrap_or(&0.1);
            let severity_multiplier = match error.severity {
                ErrorSeverity::Minor => 0.5,
                ErrorSeverity::Moderate => 1.0,
                ErrorSeverity::Major => 1.5,
            };
            penalty += base_penalty * severity_multiplier;
        }
        
        (1.0_f64 - penalty).max(0.0)
    }
    
    fn calculate_fluency_score(&self, comparison: &AudioComparisonResult) -> f64 {
        // Fluency is related to smooth spectral transitions and consistent timing
        let base_fluency = comparison.similarity_score;
        let timing_consistency = comparison.timing_correlation;
        
        (base_fluency + timing_consistency) / 2.0
    }
    
    fn calculate_clarity_score(&self, comparison: &AudioComparisonResult, quality_metrics: Option<&AudioQualityMetrics>) -> f64 {
        let mut clarity = 1.0 - comparison.spectral_distance.min(1.0);
        
        if let Some(metrics) = quality_metrics {
            // Adjust based on audio quality
            if metrics.signal_to_noise_ratio < 20.0 {
                clarity *= 0.8; // Reduce score for poor audio quality
            }
            if metrics.clipping_detected {
                clarity *= 0.7; // Significant penalty for clipping
            }
        }
        
        clarity.clamp(0.0, 1.0)
    }
    
    fn calculate_rhythm_score(&self, comparison: &AudioComparisonResult) -> f64 {
        // Rhythm is primarily based on timing correlation
        comparison.timing_correlation
    }
    
    fn calculate_error_penalty(&self, errors: &[TajweedError]) -> f64 {
        let mut total_penalty = 0.0;
        
        for error in errors {
            let base_penalty = self.error_penalties.get(&error.error_type).unwrap_or(&0.05);
            let severity_multiplier = match error.severity {
                ErrorSeverity::Minor => 0.3,
                ErrorSeverity::Moderate => 0.6,
                ErrorSeverity::Major => 1.0,
            };
            total_penalty += base_penalty * severity_multiplier;
        }
        
        total_penalty.min(0.5_f64) // Cap penalty at 50%
    }
    
    fn calculate_bonus_points(&self, tajweed: f64, pronunciation: f64, timing: f64) -> f64 {
        let mut bonus = 0.0;
        
        // Bonus for exceptional performance in all areas
        if tajweed > 0.95 && pronunciation > 0.95 && timing > 0.95 {
            bonus += 0.05;
        }
        
        // Bonus for perfect Tajweed
        if tajweed >= 1.0 {
            bonus += 0.02;
        }
        
        bonus
    }
    
    fn count_error_types(&self, errors: &[TajweedError]) -> HashMap<TajweedErrorType, usize> {
        let mut counts = HashMap::new();
        
        for error in errors {
            *counts.entry(error.error_type.clone()).or_insert(0) += 1;
        }
        
        counts
    }
}

/// Audio quality metrics for scoring adjustment
#[derive(Debug, Clone)]
pub struct AudioQualityMetrics {
    pub signal_to_noise_ratio: f64,
    pub dynamic_range: f64,
    pub clipping_detected: bool,
    pub background_noise_level: f64,
    pub recording_quality_score: f64,
}

impl Default for RecitationScoringSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{TajweedErrorType, ErrorSeverity};
    
    #[test]
    fn test_scoring_system_creation() {
        let scoring_system = RecitationScoringSystem::new();
        
        // Check that weights sum to approximately 1.0
        let total_weight = scoring_system.weights.tajweed_accuracy
            + scoring_system.weights.pronunciation
            + scoring_system.weights.timing
            + scoring_system.weights.fluency
            + scoring_system.weights.clarity
            + scoring_system.weights.rhythm;
        
        assert!((total_weight - 1.0).abs() < 0.01);
        
        // Check that error penalties are defined
        assert!(scoring_system.error_penalties.contains_key(&TajweedErrorType::Ghunnah));
        assert!(scoring_system.error_penalties.contains_key(&TajweedErrorType::Madd));
    }
    
    #[test]
    fn test_performance_level_determination() {
        let scoring_system = RecitationScoringSystem::new();
        
        assert_eq!(scoring_system.determine_performance_level(0.95), PerformanceLevel::Expert);
        assert_eq!(scoring_system.determine_performance_level(0.8), PerformanceLevel::Advanced);
        assert_eq!(scoring_system.determine_performance_level(0.65), PerformanceLevel::Intermediate);
        assert_eq!(scoring_system.determine_performance_level(0.5), PerformanceLevel::Elementary);
        assert_eq!(scoring_system.determine_performance_level(0.3), PerformanceLevel::Beginner);
    }
    
    #[test]
    fn test_tajweed_accuracy_calculation() {
        let scoring_system = RecitationScoringSystem::new();
        
        // No errors should give perfect score
        let no_errors = vec![];
        assert_eq!(scoring_system.calculate_tajweed_accuracy(&no_errors), 1.0);
        
        // Single minor error should reduce score slightly
        let minor_error = vec![TajweedError {
            error_type: TajweedErrorType::Pronunciation,
            start_time: 0.0,
            end_time: 1.0,
            severity: ErrorSeverity::Minor,
            description: "Test error".to_string(),
            correction_suggestion: "Test correction".to_string(),
            reference_audio_path: None,
        }];
        
        let accuracy = scoring_system.calculate_tajweed_accuracy(&minor_error);
        assert!(accuracy < 1.0);
        assert!(accuracy > 0.9); // Should still be quite high for minor error
    }
    
    #[test]
    fn test_feedback_generation() {
        let scoring_system = RecitationScoringSystem::new();
        
        let excellent_scores = DetailedScores {
            overall_score: 0.95,
            tajweed_accuracy: 0.98,
            pronunciation_accuracy: 0.96,
            timing_accuracy: 0.94,
            fluency_score: 0.93,
            clarity_score: 0.97,
            rhythm_score: 0.92,
            error_penalty: 0.02,
            bonus_points: 0.05,
        };
        
        let feedback = scoring_system.generate_feedback(&excellent_scores);
        assert!(!feedback.is_empty());
        assert!(feedback[0].contains("Excellent"));
        
        let poor_scores = DetailedScores {
            overall_score: 0.3,
            tajweed_accuracy: 0.4,
            pronunciation_accuracy: 0.5,
            timing_accuracy: 0.6,
            fluency_score: 0.3,
            clarity_score: 0.4,
            rhythm_score: 0.5,
            error_penalty: 0.3,
            bonus_points: 0.0,
        };
        
        let feedback = scoring_system.generate_feedback(&poor_scores);
        assert!(!feedback.is_empty());
        assert!(feedback.iter().any(|f| f.contains("practice") || f.contains("fundamental")));
    }
}
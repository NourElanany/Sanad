use anyhow::{Result, Context};
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use shared::{TajweedError, TajweedErrorType, ErrorSeverity};

use crate::progress_tracker::{
    UserProgressData, WeakPoint, ImprovementRecommendation, Exercise, 
    RecommendationCategory, Priority, ExerciseType, DifficultyLevel, MasteryLevel
};

/// Engine for generating personalized improvement recommendations
pub struct ImprovementEngine {
    /// Exercise templates for different error types
    exercise_templates: HashMap<TajweedErrorType, Vec<ExerciseTemplate>>,
    /// Recommendation strategies
    strategies: Vec<RecommendationStrategy>,
}

/// Template for creating exercises
#[derive(Debug, Clone)]
pub struct ExerciseTemplate {
    pub title: String,
    pub description: String,
    pub exercise_type: ExerciseType,
    pub base_difficulty: DifficultyLevel,
    pub estimated_duration_minutes: u32,
    pub instructions: Vec<String>,
    pub success_criteria: Vec<String>,
    pub target_ayahs: Vec<(u8, u16)>,
    pub prerequisites: Vec<TajweedErrorType>,
}

/// Strategy for generating recommendations
#[derive(Debug, Clone)]
pub struct RecommendationStrategy {
    pub name: String,
    pub description: String,
    pub target_mastery_levels: Vec<MasteryLevel>,
    pub focus_areas: Vec<TajweedErrorType>,
    pub priority_weight: f64,
}

/// Personalized learning plan
#[derive(Debug, Clone, Serialize)]
pub struct LearningPlan {
    pub user_id: Uuid,
    pub plan_id: String,
    pub title: String,
    pub description: String,
    pub estimated_duration_weeks: u32,
    pub difficulty_level: DifficultyLevel,
    pub phases: Vec<LearningPhase>,
    pub daily_practice_minutes: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Phase in the learning plan
#[derive(Debug, Clone, Serialize)]
pub struct LearningPhase {
    pub phase_number: u32,
    pub title: String,
    pub description: String,
    pub duration_weeks: u32,
    pub focus_skills: Vec<TajweedErrorType>,
    pub target_ayahs: Vec<(u8, u16)>,
    pub exercises: Vec<Exercise>,
    pub success_criteria: Vec<String>,
    pub completed: bool,
}

/// Adaptive exercise recommendation
#[derive(Debug, Clone, Serialize)]
pub struct AdaptiveRecommendation {
    pub recommendation_id: String,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub urgency: UrgencyLevel,
    pub estimated_improvement: f64, // Expected score improvement
    pub confidence: f64, // Confidence in the recommendation
    pub exercises: Vec<Exercise>,
    pub tracking_metrics: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Urgency levels for recommendations
#[derive(Debug, Clone, Serialize)]
pub enum UrgencyLevel {
    Critical,  // Major errors that need immediate attention
    High,      // Important improvements
    Medium,    // General improvements
    Low,       // Optional enhancements
}

/// Progress prediction
#[derive(Debug, Clone, Serialize)]
pub struct ProgressPrediction {
    pub user_id: Uuid,
    pub predicted_score_in_weeks: HashMap<u32, f64>, // weeks -> predicted score
    pub estimated_mastery_dates: HashMap<TajweedErrorType, DateTime<Utc>>,
    pub bottleneck_skills: Vec<TajweedErrorType>,
    pub recommended_focus_areas: Vec<TajweedErrorType>,
    pub confidence_interval: (f64, f64), // (lower, upper) bounds
}

/// Motivational insights
#[derive(Debug, Clone, Serialize)]
pub struct MotivationalInsights {
    pub user_id: Uuid,
    pub current_streak: u32,
    pub improvement_highlights: Vec<String>,
    pub upcoming_milestones: Vec<String>,
    pub encouragement_message: String,
    pub challenge_suggestions: Vec<String>,
    pub peer_comparison: Option<PeerComparison>,
}

/// Peer comparison data (anonymized)
#[derive(Debug, Clone, Serialize)]
pub struct PeerComparison {
    pub user_percentile: f64, // User's position relative to peers (0-100)
    pub average_peer_score: f64,
    pub areas_above_average: Vec<TajweedErrorType>,
    pub areas_below_average: Vec<TajweedErrorType>,
}

impl ImprovementEngine {
    /// Create a new improvement engine
    pub fn new() -> Self {
        let mut engine = Self {
            exercise_templates: HashMap::new(),
            strategies: Vec::new(),
        };
        
        engine.initialize_exercise_templates();
        engine.initialize_strategies();
        engine
    }
    
    /// Generate comprehensive improvement recommendations
    pub fn generate_recommendations(
        &self,
        user_progress: &UserProgressData,
        recent_errors: &[TajweedError],
        session_count: u32,
    ) -> Result<Vec<ImprovementRecommendation>> {
        let mut recommendations = Vec::new();
        
        // Analyze weak points and generate targeted recommendations
        for weak_point in &user_progress.weak_points {
            if weak_point.frequency > 0.3 { // Focus on frequent errors
                let recommendation = self.create_targeted_recommendation(
                    weak_point,
                    &user_progress.skill_levels.overall_level.current_level,
                )?;
                recommendations.push(recommendation);
            }
        }
        
        // Generate general improvement recommendations
        if recommendations.len() < 3 {
            let general_recommendations = self.generate_general_recommendations(
                user_progress,
                session_count,
            )?;
            recommendations.extend(general_recommendations);
        }
        
        // Sort by priority and limit to top 5
        recommendations.sort_by(|a, b| {
            match (&a.priority, &b.priority) {
                (Priority::High, Priority::High) => std::cmp::Ordering::Equal,
                (Priority::High, _) => std::cmp::Ordering::Less,
                (_, Priority::High) => std::cmp::Ordering::Greater,
                (Priority::Medium, Priority::Medium) => std::cmp::Ordering::Equal,
                (Priority::Medium, Priority::Low) => std::cmp::Ordering::Less,
                (Priority::Low, Priority::Medium) => std::cmp::Ordering::Greater,
                (Priority::Low, Priority::Low) => std::cmp::Ordering::Equal,
            }
        });
        
        recommendations.truncate(5);
        Ok(recommendations)
    }
    
    /// Create a personalized learning plan
    pub fn create_learning_plan(
        &self,
        user_progress: &UserProgressData,
        target_duration_weeks: u32,
        daily_practice_minutes: u32,
    ) -> Result<LearningPlan> {
        let plan_id = format!("plan_{}", Uuid::new_v4());
        let now = Utc::now();
        
        // Determine difficulty level based on user's current level
        let difficulty_level = match user_progress.skill_levels.overall_level.current_level {
            MasteryLevel::Beginner => DifficultyLevel::Easy,
            MasteryLevel::Elementary => DifficultyLevel::Medium,
            MasteryLevel::Intermediate => DifficultyLevel::Medium,
            MasteryLevel::Advanced => DifficultyLevel::Hard,
            MasteryLevel::Expert => DifficultyLevel::Expert,
        };
        
        // Create phases based on weak points and skill level
        let phases = self.create_learning_phases(user_progress, target_duration_weeks)?;
        
        Ok(LearningPlan {
            user_id: user_progress.user_id,
            plan_id,
            title: "Personalized Quran Recitation Improvement Plan".to_string(),
            description: "A customized plan to improve your Quran recitation based on your current skill level and identified areas for improvement.".to_string(),
            estimated_duration_weeks: target_duration_weeks,
            difficulty_level,
            phases,
            daily_practice_minutes,
            created_at: now,
            updated_at: now,
        })
    }
    
    /// Generate adaptive recommendations based on recent performance
    pub fn generate_adaptive_recommendations(
        &self,
        user_progress: &UserProgressData,
        recent_sessions: &[crate::progress_tracker::PracticeSession],
    ) -> Result<Vec<AdaptiveRecommendation>> {
        let mut recommendations = Vec::new();
        
        // Analyze recent performance trends
        let performance_trend = self.analyze_performance_trend(recent_sessions);
        
        // Generate recommendations based on trends
        if performance_trend.is_declining {
            recommendations.push(self.create_recovery_recommendation(user_progress)?);
        }
        
        if performance_trend.has_plateau {
            recommendations.push(self.create_challenge_recommendation(user_progress)?);
        }
        
        // Generate skill-specific recommendations
        for weak_point in &user_progress.weak_points {
            if weak_point.improvement_rate < 0.1 { // Slow improvement
                recommendations.push(self.create_intensive_recommendation(weak_point)?);
            }
        }
        
        Ok(recommendations)
    }
    
    /// Predict user progress
    pub fn predict_progress(
        &self,
        user_progress: &UserProgressData,
        weeks_ahead: u32,
    ) -> Result<ProgressPrediction> {
        let mut predicted_scores = HashMap::new();
        let mut mastery_dates = HashMap::new();
        
        // Simple linear prediction based on current improvement rate
        let base_score = user_progress.overall_stats.average_score;
        let improvement_rate = user_progress.overall_stats.improvement_rate;
        
        for week in 1..=weeks_ahead {
            let predicted_score = (base_score + (improvement_rate * week as f64)).min(1.0);
            predicted_scores.insert(week, predicted_score);
        }
        
        // Predict mastery dates for each skill
        for weak_point in &user_progress.weak_points {
            if weak_point.improvement_rate > 0.0 {
                let weeks_to_mastery = (0.9 - weak_point.frequency) / weak_point.improvement_rate;
                let mastery_date = Utc::now() + Duration::weeks(weeks_to_mastery as i64);
                mastery_dates.insert(weak_point.error_type.clone(), mastery_date);
            }
        }
        
        // Identify bottleneck skills
        let bottleneck_skills = user_progress.weak_points.iter()
            .filter(|wp| wp.improvement_rate < 0.05)
            .map(|wp| wp.error_type.clone())
            .collect();
        
        Ok(ProgressPrediction {
            user_id: user_progress.user_id,
            predicted_score_in_weeks: predicted_scores,
            estimated_mastery_dates: mastery_dates,
            bottleneck_skills,
            recommended_focus_areas: self.identify_focus_areas(user_progress),
            confidence_interval: (0.8, 1.2), // Would be calculated based on historical accuracy
        })
    }
    
    /// Generate motivational insights
    pub fn generate_motivational_insights(
        &self,
        user_progress: &UserProgressData,
        peer_data: Option<&PeerComparison>,
    ) -> Result<MotivationalInsights> {
        let improvement_highlights = self.generate_improvement_highlights(user_progress);
        let upcoming_milestones = self.generate_upcoming_milestones(user_progress);
        let encouragement_message = self.generate_encouragement_message(user_progress);
        let challenge_suggestions = self.generate_challenge_suggestions(user_progress);
        
        Ok(MotivationalInsights {
            user_id: user_progress.user_id,
            current_streak: user_progress.overall_stats.current_streak_days,
            improvement_highlights,
            upcoming_milestones,
            encouragement_message,
            challenge_suggestions,
            peer_comparison: peer_data.cloned(),
        })
    }
    
    // Private helper methods
    
    fn initialize_exercise_templates(&mut self) {
        // Ghunnah exercises
        let ghunnah_exercises = vec![
            ExerciseTemplate {
                title: "Basic Ghunnah Practice".to_string(),
                description: "Practice nasal resonance with simple words".to_string(),
                exercise_type: ExerciseType::Repetition,
                base_difficulty: DifficultyLevel::Easy,
                estimated_duration_minutes: 15,
                instructions: vec![
                    "Place your hand on your nose while reciting".to_string(),
                    "Feel the vibration for ن and م letters".to_string(),
                    "Practice with words: من، عن، إن".to_string(),
                ],
                success_criteria: vec![
                    "Clear nasal resonance for 90% of attempts".to_string(),
                    "Consistent duration of nasal sound".to_string(),
                ],
                target_ayahs: vec![(1, 2), (1, 3)],
                prerequisites: vec![],
            },
            ExerciseTemplate {
                title: "Advanced Ghunnah with Idgham".to_string(),
                description: "Practice Ghunnah in complex Idgham situations".to_string(),
                exercise_type: ExerciseType::Comparison,
                base_difficulty: DifficultyLevel::Hard,
                estimated_duration_minutes: 25,
                instructions: vec![
                    "Identify Idgham with Ghunnah cases".to_string(),
                    "Practice smooth merging with nasal sound".to_string(),
                    "Compare with reference recordings".to_string(),
                ],
                success_criteria: vec![
                    "Correct Idgham with Ghunnah in 85% of cases".to_string(),
                    "Smooth transitions without breaks".to_string(),
                ],
                target_ayahs: vec![(2, 1), (2, 2)],
                prerequisites: vec![TajweedErrorType::Ghunnah],
            },
        ];
        self.exercise_templates.insert(TajweedErrorType::Ghunnah, ghunnah_exercises);
        
        // Madd exercises
        let madd_exercises = vec![
            ExerciseTemplate {
                title: "Madd Duration Practice".to_string(),
                description: "Practice correct elongation durations".to_string(),
                exercise_type: ExerciseType::Repetition,
                base_difficulty: DifficultyLevel::Medium,
                estimated_duration_minutes: 20,
                instructions: vec![
                    "Use a metronome for timing".to_string(),
                    "Practice 2, 4, and 6 count Madd".to_string(),
                    "Focus on smooth vowel elongation".to_string(),
                ],
                success_criteria: vec![
                    "Correct duration in 90% of attempts".to_string(),
                    "Smooth, consistent elongation".to_string(),
                ],
                target_ayahs: vec![(1, 1), (1, 4)],
                prerequisites: vec![],
            },
        ];
        self.exercise_templates.insert(TajweedErrorType::Madd, madd_exercises);
        
        // Add more exercise templates for other error types...
    }
    
    fn initialize_strategies(&mut self) {
        self.strategies = vec![
            RecommendationStrategy {
                name: "Foundation Building".to_string(),
                description: "Focus on basic Tajweed rules for beginners".to_string(),
                target_mastery_levels: vec![MasteryLevel::Beginner, MasteryLevel::Elementary],
                focus_areas: vec![TajweedErrorType::Pronunciation, TajweedErrorType::Ghunnah],
                priority_weight: 1.0,
            },
            RecommendationStrategy {
                name: "Skill Refinement".to_string(),
                description: "Refine specific Tajweed skills for intermediate learners".to_string(),
                target_mastery_levels: vec![MasteryLevel::Intermediate],
                focus_areas: vec![TajweedErrorType::Madd, TajweedErrorType::Qalqalah],
                priority_weight: 0.8,
            },
            RecommendationStrategy {
                name: "Mastery Achievement".to_string(),
                description: "Perfect advanced techniques for expert-level recitation".to_string(),
                target_mastery_levels: vec![MasteryLevel::Advanced, MasteryLevel::Expert],
                focus_areas: vec![TajweedErrorType::Idgham, TajweedErrorType::Ikhfa],
                priority_weight: 0.6,
            },
        ];
    }
    
    fn create_targeted_recommendation(
        &self,
        weak_point: &WeakPoint,
        mastery_level: &MasteryLevel,
    ) -> Result<ImprovementRecommendation> {
        let templates = self.exercise_templates.get(&weak_point.error_type)
            .context("No exercise templates found for error type")?;
        
        // Select appropriate template based on mastery level
        let template = templates.iter()
            .find(|t| self.is_template_appropriate(t, mastery_level))
            .or_else(|| templates.first())
            .context("No appropriate template found")?;
        
        let priority = match weak_point.frequency {
            f if f > 0.7 => Priority::High,
            f if f > 0.4 => Priority::Medium,
            _ => Priority::Low,
        };
        
        let exercises = vec![self.create_exercise_from_template(template, weak_point)?];
        
        Ok(ImprovementRecommendation {
            id: format!("rec_{}", Uuid::new_v4()),
            title: format!("Improve {}", self.error_type_to_string(&weak_point.error_type)),
            description: format!("Focus on improving {} which occurs frequently in your recitation", 
                               self.error_type_to_string(&weak_point.error_type)),
            category: self.error_type_to_category(&weak_point.error_type),
            priority,
            estimated_time_minutes: template.estimated_duration_minutes,
            target_skill: weak_point.error_type.clone(),
            exercises,
            success_criteria: template.success_criteria.clone(),
            created_at: Utc::now(),
        })
    }
    
    fn generate_general_recommendations(
        &self,
        user_progress: &UserProgressData,
        session_count: u32,
    ) -> Result<Vec<ImprovementRecommendation>> {
        let mut recommendations = Vec::new();
        
        // Recommendation based on session count
        if session_count < 5 {
            recommendations.push(ImprovementRecommendation {
                id: format!("rec_{}", Uuid::new_v4()),
                title: "Build Consistency".to_string(),
                description: "Practice regularly to build muscle memory and improve retention".to_string(),
                category: RecommendationCategory::General,
                priority: Priority::High,
                estimated_time_minutes: 15,
                target_skill: TajweedErrorType::Pronunciation,
                exercises: vec![self.create_consistency_exercise()],
                success_criteria: vec!["Practice at least 3 times per week".to_string()],
                created_at: Utc::now(),
            });
        }
        
        // Recommendation based on overall score
        if user_progress.overall_stats.average_score < 0.6 {
            recommendations.push(ImprovementRecommendation {
                id: format!("rec_{}", Uuid::new_v4()),
                title: "Focus on Fundamentals".to_string(),
                description: "Strengthen basic recitation skills before moving to advanced techniques".to_string(),
                category: RecommendationCategory::General,
                priority: Priority::High,
                estimated_time_minutes: 20,
                target_skill: TajweedErrorType::Pronunciation,
                exercises: vec![self.create_fundamentals_exercise()],
                success_criteria: vec!["Achieve 70% average score".to_string()],
                created_at: Utc::now(),
            });
        }
        
        Ok(recommendations)
    }
    
    fn create_learning_phases(
        &self,
        user_progress: &UserProgressData,
        total_weeks: u32,
    ) -> Result<Vec<LearningPhase>> {
        let mut phases = Vec::new();
        let weeks_per_phase = (total_weeks / 3).max(1);
        
        // Phase 1: Foundation
        phases.push(LearningPhase {
            phase_number: 1,
            title: "Foundation Building".to_string(),
            description: "Master basic Tajweed rules and pronunciation".to_string(),
            duration_weeks: weeks_per_phase,
            focus_skills: vec![TajweedErrorType::Pronunciation, TajweedErrorType::Ghunnah],
            target_ayahs: vec![(1, 1), (1, 2), (1, 3)],
            exercises: self.create_foundation_exercises()?,
            success_criteria: vec![
                "Achieve 70% accuracy in basic pronunciation".to_string(),
                "Master Ghunnah rules".to_string(),
            ],
            completed: false,
        });
        
        // Phase 2: Skill Development
        phases.push(LearningPhase {
            phase_number: 2,
            title: "Skill Development".to_string(),
            description: "Develop intermediate Tajweed skills".to_string(),
            duration_weeks: weeks_per_phase,
            focus_skills: vec![TajweedErrorType::Madd, TajweedErrorType::Qalqalah],
            target_ayahs: vec![(1, 4), (1, 5), (1, 6)],
            exercises: self.create_development_exercises()?,
            success_criteria: vec![
                "Master Madd rules with correct timing".to_string(),
                "Apply Qalqalah correctly".to_string(),
            ],
            completed: false,
        });
        
        // Phase 3: Mastery
        phases.push(LearningPhase {
            phase_number: 3,
            title: "Advanced Mastery".to_string(),
            description: "Perfect advanced techniques and fluency".to_string(),
            duration_weeks: total_weeks - (2 * weeks_per_phase),
            focus_skills: vec![TajweedErrorType::Idgham, TajweedErrorType::Ikhfa],
            target_ayahs: vec![(1, 7), (2, 1), (2, 2)],
            exercises: self.create_mastery_exercises()?,
            success_criteria: vec![
                "Achieve 90% overall accuracy".to_string(),
                "Demonstrate fluent recitation".to_string(),
            ],
            completed: false,
        });
        
        Ok(phases)
    }
    
    fn analyze_performance_trend(
        &self,
        recent_sessions: &[crate::progress_tracker::PracticeSession],
    ) -> PerformanceTrend {
        if recent_sessions.len() < 3 {
            return PerformanceTrend {
                is_declining: false,
                has_plateau: false,
                improvement_rate: 0.0,
            };
        }
        
        let scores: Vec<f64> = recent_sessions.iter()
            .map(|s| s.average_score)
            .collect();
        
        // Simple trend analysis
        let recent_avg = scores.iter().rev().take(3).sum::<f64>() / 3.0;
        let older_avg = scores.iter().take(3).sum::<f64>() / 3.0;
        
        let improvement_rate = recent_avg - older_avg;
        
        PerformanceTrend {
            is_declining: improvement_rate < -0.05,
            has_plateau: improvement_rate.abs() < 0.02,
            improvement_rate,
        }
    }
    
    fn create_recovery_recommendation(&self, user_progress: &UserProgressData) -> Result<AdaptiveRecommendation> {
        Ok(AdaptiveRecommendation {
            recommendation_id: format!("adaptive_{}", Uuid::new_v4()),
            user_id: user_progress.user_id,
            title: "Recovery Plan".to_string(),
            description: "Your recent performance shows a decline. Let's get back on track with focused practice.".to_string(),
            urgency: UrgencyLevel::High,
            estimated_improvement: 0.15,
            confidence: 0.8,
            exercises: vec![self.create_recovery_exercise()],
            tracking_metrics: vec!["Overall score improvement".to_string()],
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::weeks(2),
        })
    }
    
    fn create_challenge_recommendation(&self, user_progress: &UserProgressData) -> Result<AdaptiveRecommendation> {
        Ok(AdaptiveRecommendation {
            recommendation_id: format!("adaptive_{}", Uuid::new_v4()),
            user_id: user_progress.user_id,
            title: "Challenge Yourself".to_string(),
            description: "You've plateaued. Try these challenging exercises to break through.".to_string(),
            urgency: UrgencyLevel::Medium,
            estimated_improvement: 0.10,
            confidence: 0.7,
            exercises: vec![self.create_challenge_exercise()],
            tracking_metrics: vec!["Skill level advancement".to_string()],
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::weeks(3),
        })
    }
    
    fn create_intensive_recommendation(&self, weak_point: &WeakPoint) -> Result<AdaptiveRecommendation> {
        Ok(AdaptiveRecommendation {
            recommendation_id: format!("adaptive_{}", Uuid::new_v4()),
            user_id: Uuid::new_v4(), // Would be passed as parameter
            title: format!("Intensive {} Training", self.error_type_to_string(&weak_point.error_type)),
            description: format!("This skill needs intensive focus to see improvement."),
            urgency: UrgencyLevel::Critical,
            estimated_improvement: 0.20,
            confidence: 0.9,
            exercises: vec![self.create_intensive_exercise(&weak_point.error_type)],
            tracking_metrics: vec![format!("{} error frequency", self.error_type_to_string(&weak_point.error_type))],
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::weeks(1),
        })
    }
    
    // Helper methods for creating exercises
    
    fn create_exercise_from_template(&self, template: &ExerciseTemplate, weak_point: &WeakPoint) -> Result<Exercise> {
        Ok(Exercise {
            id: format!("ex_{}", Uuid::new_v4()),
            title: template.title.clone(),
            description: template.description.clone(),
            exercise_type: template.exercise_type.clone(),
            difficulty: self.adjust_difficulty(&template.base_difficulty, weak_point.frequency),
            estimated_duration_minutes: template.estimated_duration_minutes,
            target_ayahs: template.target_ayahs.clone(),
            instructions: template.instructions.clone(),
            success_criteria: template.success_criteria.clone(),
        })
    }
    
    fn create_consistency_exercise(&self) -> Exercise {
        Exercise {
            id: format!("ex_{}", Uuid::new_v4()),
            title: "Daily Practice Routine".to_string(),
            description: "Establish a consistent daily practice routine".to_string(),
            exercise_type: ExerciseType::Recording,
            difficulty: DifficultyLevel::Easy,
            estimated_duration_minutes: 15,
            target_ayahs: vec![(1, 1)],
            instructions: vec![
                "Choose the same time each day for practice".to_string(),
                "Start with Al-Fatiha".to_string(),
                "Record and review your progress".to_string(),
            ],
            success_criteria: vec!["Practice for 7 consecutive days".to_string()],
        }
    }
    
    fn create_fundamentals_exercise(&self) -> Exercise {
        Exercise {
            id: format!("ex_{}", Uuid::new_v4()),
            title: "Arabic Pronunciation Fundamentals".to_string(),
            description: "Master basic Arabic letter pronunciation".to_string(),
            exercise_type: ExerciseType::Repetition,
            difficulty: DifficultyLevel::Easy,
            estimated_duration_minutes: 20,
            target_ayahs: vec![(1, 1), (1, 2)],
            instructions: vec![
                "Practice each letter sound individually".to_string(),
                "Focus on correct tongue and lip positions".to_string(),
                "Record and compare with reference".to_string(),
            ],
            success_criteria: vec!["Clear pronunciation of all Arabic letters".to_string()],
        }
    }
    
    fn create_foundation_exercises(&self) -> Result<Vec<Exercise>> {
        Ok(vec![
            self.create_fundamentals_exercise(),
            self.create_consistency_exercise(),
        ])
    }
    
    fn create_development_exercises(&self) -> Result<Vec<Exercise>> {
        Ok(vec![
            Exercise {
                id: format!("ex_{}", Uuid::new_v4()),
                title: "Madd Practice".to_string(),
                description: "Master vowel elongation rules".to_string(),
                exercise_type: ExerciseType::Comparison,
                difficulty: DifficultyLevel::Medium,
                estimated_duration_minutes: 25,
                target_ayahs: vec![(1, 4), (1, 5)],
                instructions: vec![
                    "Identify different types of Madd".to_string(),
                    "Practice with metronome for timing".to_string(),
                    "Compare with reference recordings".to_string(),
                ],
                success_criteria: vec!["Correct Madd duration 90% of the time".to_string()],
            }
        ])
    }
    
    fn create_mastery_exercises(&self) -> Result<Vec<Exercise>> {
        Ok(vec![
            Exercise {
                id: format!("ex_{}", Uuid::new_v4()),
                title: "Advanced Tajweed Integration".to_string(),
                description: "Integrate all Tajweed rules in fluent recitation".to_string(),
                exercise_type: ExerciseType::Recording,
                difficulty: DifficultyLevel::Hard,
                estimated_duration_minutes: 30,
                target_ayahs: vec![(1, 7), (2, 1)],
                instructions: vec![
                    "Recite with all Tajweed rules applied".to_string(),
                    "Focus on smooth transitions".to_string(),
                    "Maintain consistent rhythm".to_string(),
                ],
                success_criteria: vec!["Fluent recitation with 95% Tajweed accuracy".to_string()],
            }
        ])
    }
    
    fn create_recovery_exercise(&self) -> Exercise {
        Exercise {
            id: format!("ex_{}", Uuid::new_v4()),
            title: "Back to Basics".to_string(),
            description: "Return to fundamental practices to rebuild confidence".to_string(),
            exercise_type: ExerciseType::Repetition,
            difficulty: DifficultyLevel::Easy,
            estimated_duration_minutes: 15,
            target_ayahs: vec![(1, 1)],
            instructions: vec![
                "Focus on accuracy over speed".to_string(),
                "Practice slowly and deliberately".to_string(),
                "Celebrate small improvements".to_string(),
            ],
            success_criteria: vec!["Consistent improvement over 5 sessions".to_string()],
        }
    }
    
    fn create_challenge_exercise(&self) -> Exercise {
        Exercise {
            id: format!("ex_{}", Uuid::new_v4()),
            title: "Advanced Challenge".to_string(),
            description: "Push your limits with challenging verses".to_string(),
            exercise_type: ExerciseType::Recording,
            difficulty: DifficultyLevel::Hard,
            estimated_duration_minutes: 30,
            target_ayahs: vec![(2, 255)], // Ayat al-Kursi
            instructions: vec![
                "Attempt a challenging verse".to_string(),
                "Focus on perfect Tajweed application".to_string(),
                "Don't worry about mistakes, focus on learning".to_string(),
            ],
            success_criteria: vec!["Complete the verse with 80% accuracy".to_string()],
        }
    }
    
    fn create_intensive_exercise(&self, error_type: &TajweedErrorType) -> Exercise {
        Exercise {
            id: format!("ex_{}", Uuid::new_v4()),
            title: format!("Intensive {} Training", self.error_type_to_string(error_type)),
            description: format!("Focused intensive practice for {}", self.error_type_to_string(error_type)),
            exercise_type: ExerciseType::Repetition,
            difficulty: DifficultyLevel::Medium,
            estimated_duration_minutes: 45,
            target_ayahs: vec![(1, 1), (1, 2), (1, 3)],
            instructions: vec![
                "Practice this specific skill repeatedly".to_string(),
                "Use slow, deliberate movements".to_string(),
                "Record multiple attempts".to_string(),
            ],
            success_criteria: vec!["Show measurable improvement in this specific area".to_string()],
        }
    }
    
    // Utility methods
    
    fn is_template_appropriate(&self, template: &ExerciseTemplate, mastery_level: &MasteryLevel) -> bool {
        match (mastery_level, &template.base_difficulty) {
            (MasteryLevel::Beginner, DifficultyLevel::Easy) => true,
            (MasteryLevel::Elementary, DifficultyLevel::Easy | DifficultyLevel::Medium) => true,
            (MasteryLevel::Intermediate, DifficultyLevel::Medium) => true,
            (MasteryLevel::Advanced, DifficultyLevel::Medium | DifficultyLevel::Hard) => true,
            (MasteryLevel::Expert, _) => true,
            _ => false,
        }
    }
    
    fn adjust_difficulty(&self, base_difficulty: &DifficultyLevel, frequency: f64) -> DifficultyLevel {
        match (base_difficulty, frequency) {
            (DifficultyLevel::Hard, f) if f > 0.8 => DifficultyLevel::Medium,
            (DifficultyLevel::Medium, f) if f > 0.9 => DifficultyLevel::Easy,
            _ => base_difficulty.clone(),
        }
    }
    
    fn error_type_to_string(&self, error_type: &TajweedErrorType) -> &str {
        match error_type {
            TajweedErrorType::Ghunnah => "Ghunnah",
            TajweedErrorType::Qalqalah => "Qalqalah",
            TajweedErrorType::Madd => "Madd",
            TajweedErrorType::Idgham => "Idgham",
            TajweedErrorType::Ikhfa => "Ikhfa",
            TajweedErrorType::Pronunciation => "Pronunciation",
            TajweedErrorType::Timing => "Timing",
            TajweedErrorType::Other(_) => "Other",
        }
    }
    
    fn error_type_to_category(&self, error_type: &TajweedErrorType) -> RecommendationCategory {
        match error_type {
            TajweedErrorType::Pronunciation => RecommendationCategory::Pronunciation,
            TajweedErrorType::Timing => RecommendationCategory::Timing,
            _ => RecommendationCategory::Tajweed,
        }
    }
    
    fn identify_focus_areas(&self, user_progress: &UserProgressData) -> Vec<TajweedErrorType> {
        user_progress.weak_points.iter()
            .filter(|wp| wp.frequency > 0.4)
            .map(|wp| wp.error_type.clone())
            .collect()
    }
    
    fn generate_improvement_highlights(&self, user_progress: &UserProgressData) -> Vec<String> {
        let mut highlights = Vec::new();
        
        if user_progress.overall_stats.improvement_rate > 0.05 {
            highlights.push("Your overall score has improved significantly this week!".to_string());
        }
        
        if user_progress.overall_stats.current_streak_days > 7 {
            highlights.push(format!("Amazing! You've practiced for {} days in a row!", 
                                  user_progress.overall_stats.current_streak_days));
        }
        
        if user_progress.overall_stats.mastered_ayahs > 0 {
            highlights.push(format!("You've mastered {} ayahs so far!", 
                                  user_progress.overall_stats.mastered_ayahs));
        }
        
        highlights
    }
    
    fn generate_upcoming_milestones(&self, user_progress: &UserProgressData) -> Vec<String> {
        let mut milestones = Vec::new();
        
        for milestone in &user_progress.learning_path.milestones {
            if !milestone.completed && milestone.current_value / milestone.target_value > 0.7 {
                milestones.push(format!("You're close to achieving: {}", milestone.title));
            }
        }
        
        milestones
    }
    
    fn generate_encouragement_message(&self, user_progress: &UserProgressData) -> String {
        match user_progress.skill_levels.overall_level.current_level {
            MasteryLevel::Beginner => "Every expert was once a beginner. Keep practicing!".to_string(),
            MasteryLevel::Elementary => "You're making great progress! Stay consistent.".to_string(),
            MasteryLevel::Intermediate => "You're developing real skill. Keep pushing forward!".to_string(),
            MasteryLevel::Advanced => "Your recitation is becoming beautiful. Excellence is within reach!".to_string(),
            MasteryLevel::Expert => "Mashallah! Your recitation is truly excellent. Keep inspiring others!".to_string(),
        }
    }
    
    fn generate_challenge_suggestions(&self, user_progress: &UserProgressData) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        match user_progress.skill_levels.overall_level.current_level {
            MasteryLevel::Beginner | MasteryLevel::Elementary => {
                suggestions.push("Try reciting Al-Fatiha perfectly 5 times in a row".to_string());
            }
            MasteryLevel::Intermediate => {
                suggestions.push("Challenge yourself with Ayat al-Kursi".to_string());
            }
            MasteryLevel::Advanced | MasteryLevel::Expert => {
                suggestions.push("Try reciting a full page with perfect Tajweed".to_string());
            }
        }
        
        suggestions
    }
}

// Supporting structures

#[derive(Debug)]
struct PerformanceTrend {
    is_declining: bool,
    has_plateau: bool,
    improvement_rate: f64,
}

impl Default for ImprovementEngine {
    fn default() -> Self {
        Self::new()
    }
}
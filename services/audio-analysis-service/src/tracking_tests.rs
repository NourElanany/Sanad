use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;
use shared::{TajweedError, TajweedErrorType, ErrorSeverity};

use crate::progress_tracker::{ProgressTracker, MasteryLevel};
use crate::improvement_engine::ImprovementEngine;
use crate::reward_system::RewardSystem;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_tracker_initialization() {
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user progress
        let result = tracker.initialize_user_progress(user_id);
        assert!(result.is_ok());
        
        // Verify user progress was created
        assert!(tracker.user_progress.contains_key(&user_id));
        
        let progress = &tracker.user_progress[&user_id];
        assert_eq!(progress.user_id, user_id);
        assert_eq!(progress.overall_stats.total_recordings, 0);
        assert_eq!(progress.overall_stats.average_score, 0.0);
        assert_eq!(progress.skill_levels.overall_level.current_level, MasteryLevel::Beginner);
    }
    
    #[test]
    fn test_progress_update() {
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user
        tracker.initialize_user_progress(user_id).unwrap();
        
        // Create some test errors
        let errors = vec![
            TajweedError {
                error_type: TajweedErrorType::Ghunnah,
                start_time: 1.0,
                end_time: 2.0,
                severity: ErrorSeverity::Minor,
                description: "Weak Ghunnah".to_string(),
                correction_suggestion: "Increase nasal resonance".to_string(),
                reference_audio_path: None,
            },
            TajweedError {
                error_type: TajweedErrorType::Madd,
                start_time: 3.0,
                end_time: 4.0,
                severity: ErrorSeverity::Moderate,
                description: "Short Madd".to_string(),
                correction_suggestion: "Extend vowel duration".to_string(),
                reference_audio_path: None,
            },
        ];
        
        // Update progress
        let result = tracker.update_progress(user_id, 1, 1, 0.75, &errors, 15);
        assert!(result.is_ok());
        
        let update = result.unwrap();
        assert!(!update.new_best_score); // First recording, so it's automatically the best
        
        // Verify progress was updated
        let progress = &tracker.user_progress[&user_id];
        assert_eq!(progress.overall_stats.total_recordings, 1);
        assert_eq!(progress.overall_stats.total_practice_time_minutes, 15);
        assert_eq!(progress.overall_stats.best_score, 0.75);
        assert_eq!(progress.overall_stats.average_score, 0.75);
        
        // Verify ayah progress
        let ayah_key = (1, 1);
        assert!(progress.ayah_progress.contains_key(&ayah_key));
        let ayah_progress = &progress.ayah_progress[&ayah_key];
        assert_eq!(ayah_progress.attempts_count, 1);
        assert_eq!(ayah_progress.best_score, 0.75);
        assert_eq!(ayah_progress.latest_score, 0.75);
        
        // Verify weak points were recorded
        assert_eq!(progress.weak_points.len(), 2);
        assert!(progress.weak_points.iter().any(|wp| wp.error_type == TajweedErrorType::Ghunnah));
        assert!(progress.weak_points.iter().any(|wp| wp.error_type == TajweedErrorType::Madd));
    }
    
    #[test]
    fn test_mastery_level_calculation() {
        let tracker = ProgressTracker::new();
        
        assert_eq!(tracker.calculate_mastery_level(0.2), MasteryLevel::Beginner);
        assert_eq!(tracker.calculate_mastery_level(0.5), MasteryLevel::Elementary);
        assert_eq!(tracker.calculate_mastery_level(0.7), MasteryLevel::Intermediate);
        assert_eq!(tracker.calculate_mastery_level(0.8), MasteryLevel::Advanced);
        assert_eq!(tracker.calculate_mastery_level(0.95), MasteryLevel::Expert);
    }
    
    #[test]
    fn test_personalized_exercises_generation() {
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user with some weak points
        tracker.initialize_user_progress(user_id).unwrap();
        
        // Add some weak points manually for testing
        let progress = tracker.user_progress.get_mut(&user_id).unwrap();
        progress.weak_points.push(crate::progress_tracker::WeakPoint {
            error_type: TajweedErrorType::Ghunnah,
            frequency: 0.6,
            severity_average: 0.7,
            improvement_rate: 0.1,
            last_occurrence: Utc::now(),
            targeted_exercises: Vec::new(),
        });
        
        // Generate exercises
        let result = tracker.generate_personalized_exercises(user_id);
        assert!(result.is_ok());
        
        let exercises = result.unwrap();
        assert!(!exercises.is_empty());
        
        // Should have exercises targeting the weak point
        assert!(exercises.iter().any(|ex| ex.title.contains("Ghunnah")));
    }
    
    #[test]
    fn test_improvement_engine_recommendations() {
        let engine = ImprovementEngine::new();
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user progress
        tracker.initialize_user_progress(user_id).unwrap();
        let user_progress = tracker.user_progress.get(&user_id).unwrap();
        
        // Generate recommendations
        let result = engine.generate_recommendations(user_progress, &[], 5);
        assert!(result.is_ok());
        
        let recommendations = result.unwrap();
        assert!(!recommendations.is_empty());
        
        // Should have general recommendations for new users
        assert!(recommendations.iter().any(|r| r.title.contains("Consistency") || r.title.contains("Fundamentals")));
    }
    
    #[test]
    fn test_learning_plan_creation() {
        let engine = ImprovementEngine::new();
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user progress
        tracker.initialize_user_progress(user_id).unwrap();
        let user_progress = tracker.user_progress.get(&user_id).unwrap();
        
        // Create learning plan
        let result = engine.create_learning_plan(user_progress, 12, 30);
        assert!(result.is_ok());
        
        let plan = result.unwrap();
        assert_eq!(plan.user_id, user_id);
        assert_eq!(plan.estimated_duration_weeks, 12);
        assert_eq!(plan.daily_practice_minutes, 30);
        assert!(!plan.phases.is_empty());
        
        // Should have multiple phases
        assert!(plan.phases.len() >= 3);
        
        // Phases should be ordered
        for (i, phase) in plan.phases.iter().enumerate() {
            assert_eq!(phase.phase_number, (i + 1) as u32);
        }
    }
    
    #[test]
    fn test_progress_prediction() {
        let engine = ImprovementEngine::new();
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user progress with some improvement rate
        tracker.initialize_user_progress(user_id).unwrap();
        let user_progress = tracker.user_progress.get_mut(&user_id).unwrap();
        user_progress.overall_stats.average_score = 0.6;
        user_progress.overall_stats.improvement_rate = 0.02; // 2% per week
        
        // Predict progress
        let result = engine.predict_progress(user_progress, 10);
        assert!(result.is_ok());
        
        let prediction = result.unwrap();
        assert_eq!(prediction.user_id, user_id);
        assert!(!prediction.predicted_score_in_weeks.is_empty());
        
        // Should predict improvement over time
        let week_1_score = prediction.predicted_score_in_weeks.get(&1).unwrap();
        let week_10_score = prediction.predicted_score_in_weeks.get(&10).unwrap();
        assert!(week_10_score > week_1_score);
    }
    
    #[test]
    fn test_reward_system_achievements() {
        let reward_system = RewardSystem::new();
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user progress
        tracker.initialize_user_progress(user_id).unwrap();
        let user_progress = tracker.user_progress.get_mut(&user_id).unwrap();
        
        // Set up conditions for first recording achievement
        user_progress.overall_stats.total_practice_time_minutes = 5;
        
        // Check for rewards
        let result = reward_system.check_rewards(user_progress);
        assert!(result.is_ok());
        
        let reward_update = result.unwrap();
        
        // Should have earned the first recording achievement
        assert!(!reward_update.new_achievements.is_empty());
        assert!(reward_update.experience_gained > 0);
    }
    
    #[test]
    fn test_reward_system_streaks() {
        let reward_system = RewardSystem::new();
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user progress with a streak
        tracker.initialize_user_progress(user_id).unwrap();
        let user_progress = tracker.user_progress.get_mut(&user_id).unwrap();
        user_progress.overall_stats.current_streak_days = 7;
        
        // Check for rewards
        let result = reward_system.check_rewards(user_progress);
        assert!(result.is_ok());
        
        let reward_update = result.unwrap();
        
        // Should have streak rewards
        assert!(!reward_update.streak_rewards.is_empty());
        
        // Should have weekly streak reward
        assert!(reward_update.streak_rewards.iter().any(|r| r.days == 7));
    }
    
    #[test]
    fn test_reward_system_levels() {
        let reward_system = RewardSystem::new();
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user progress
        tracker.initialize_user_progress(user_id).unwrap();
        let user_progress = tracker.user_progress.get(&user_id).unwrap();
        
        // Get reward status
        let result = reward_system.get_user_reward_status(user_progress);
        assert!(result.is_ok());
        
        let status = result.unwrap();
        assert_eq!(status.user_id, user_id);
        assert_eq!(status.current_level, 1); // Should start at level 1
        assert!(status.level_progress >= 0.0 && status.level_progress <= 1.0);
        assert!(!status.level_title.is_empty());
    }
    
    #[test]
    fn test_daily_goals_generation() {
        let reward_system = RewardSystem::new();
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user progress
        tracker.initialize_user_progress(user_id).unwrap();
        let user_progress = tracker.user_progress.get(&user_id).unwrap();
        
        // Generate daily goals
        let result = reward_system.generate_daily_goals(user_progress);
        assert!(result.is_ok());
        
        let goals = result.unwrap();
        assert!(!goals.is_empty());
        
        // Should have practice time goal
        assert!(goals.iter().any(|g| g.description.contains("Practice")));
        
        // Should have score improvement goal
        assert!(goals.iter().any(|g| g.description.contains("score")));
    }
    
    #[test]
    fn test_weekly_goals_generation() {
        let reward_system = RewardSystem::new();
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user progress
        tracker.initialize_user_progress(user_id).unwrap();
        let user_progress = tracker.user_progress.get(&user_id).unwrap();
        
        // Generate weekly goals
        let result = reward_system.generate_weekly_goals(user_progress);
        assert!(result.is_ok());
        
        let goals = result.unwrap();
        assert!(!goals.is_empty());
        
        // Should have weekly practice goal
        assert!(goals.iter().any(|g| g.description.contains("week")));
        
        // Goals should have reward badges
        assert!(goals.iter().any(|g| g.reward_badge.is_some()));
    }
    
    #[test]
    fn test_challenges_generation() {
        let reward_system = RewardSystem::new();
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user progress
        tracker.initialize_user_progress(user_id).unwrap();
        let user_progress = tracker.user_progress.get(&user_id).unwrap();
        
        // Generate challenges
        let result = reward_system.generate_challenges(user_progress);
        assert!(result.is_ok());
        
        let challenges = result.unwrap();
        assert!(!challenges.is_empty());
        
        // Should have beginner challenges for new users
        assert!(challenges.iter().any(|c| c.title.contains("First") || c.title.contains("Consistency")));
        
        // Challenges should have rewards
        assert!(challenges.iter().all(|c| c.reward_points > 0));
    }
    
    #[test]
    fn test_motivational_insights() {
        let engine = ImprovementEngine::new();
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user progress with some achievements
        tracker.initialize_user_progress(user_id).unwrap();
        let user_progress = tracker.user_progress.get_mut(&user_id).unwrap();
        user_progress.overall_stats.current_streak_days = 5;
        user_progress.overall_stats.improvement_rate = 0.03;
        
        // Generate insights
        let result = engine.generate_motivational_insights(user_progress, None);
        assert!(result.is_ok());
        
        let insights = result.unwrap();
        assert_eq!(insights.user_id, user_id);
        assert_eq!(insights.current_streak, 5);
        assert!(!insights.encouragement_message.is_empty());
        assert!(!insights.challenge_suggestions.is_empty());
    }
    
    #[test]
    fn test_adaptive_recommendations() {
        let engine = ImprovementEngine::new();
        let mut tracker = ProgressTracker::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user progress
        tracker.initialize_user_progress(user_id).unwrap();
        let user_progress = tracker.user_progress.get(&user_id).unwrap();
        
        // Create some practice sessions with declining performance
        let sessions = vec![
            crate::progress_tracker::PracticeSession {
                session_id: Uuid::new_v4(),
                started_at: Utc::now() - chrono::Duration::days(3),
                ended_at: Utc::now() - chrono::Duration::days(3) + chrono::Duration::minutes(20),
                ayahs_practiced: vec![(1, 1)],
                total_recordings: 1,
                average_score: 0.8,
                improvements_made: Vec::new(),
                focus_areas: Vec::new(),
            },
            crate::progress_tracker::PracticeSession {
                session_id: Uuid::new_v4(),
                started_at: Utc::now() - chrono::Duration::days(2),
                ended_at: Utc::now() - chrono::Duration::days(2) + chrono::Duration::minutes(20),
                ayahs_practiced: vec![(1, 2)],
                total_recordings: 1,
                average_score: 0.7,
                improvements_made: Vec::new(),
                focus_areas: Vec::new(),
            },
            crate::progress_tracker::PracticeSession {
                session_id: Uuid::new_v4(),
                started_at: Utc::now() - chrono::Duration::days(1),
                ended_at: Utc::now() - chrono::Duration::days(1) + chrono::Duration::minutes(20),
                ayahs_practiced: vec![(1, 3)],
                total_recordings: 1,
                average_score: 0.6,
                improvements_made: Vec::new(),
                focus_areas: Vec::new(),
            },
        ];
        
        // Generate adaptive recommendations
        let result = engine.generate_adaptive_recommendations(user_progress, &sessions);
        assert!(result.is_ok());
        
        let recommendations = result.unwrap();
        
        // Should detect declining performance and suggest recovery
        assert!(recommendations.iter().any(|r| r.title.contains("Recovery")));
    }
    
    #[test]
    fn test_comprehensive_integration() {
        let mut tracker = ProgressTracker::new();
        let engine = ImprovementEngine::new();
        let reward_system = RewardSystem::new();
        let user_id = Uuid::new_v4();
        
        // Initialize user
        tracker.initialize_user_progress(user_id).unwrap();
        
        // Simulate multiple practice sessions
        for i in 1..=10 {
            let score = 0.5 + (i as f64 * 0.03); // Gradual improvement
            let errors = if i <= 5 {
                vec![TajweedError {
                    error_type: TajweedErrorType::Ghunnah,
                    start_time: 1.0,
                    end_time: 2.0,
                    severity: ErrorSeverity::Minor,
                    description: "Weak Ghunnah".to_string(),
                    correction_suggestion: "Increase nasal resonance".to_string(),
                    reference_audio_path: None,
                }]
            } else {
                vec![] // Improvement over time
            };
            
            let result = tracker.update_progress(user_id, 1, i as u16, score, &errors, 15);
            assert!(result.is_ok());
        }
        
        let user_progress = tracker.user_progress.get(&user_id).unwrap();
        
        // Verify overall improvement
        assert!(user_progress.overall_stats.average_score > 0.5);
        assert_eq!(user_progress.overall_stats.total_recordings, 10);
        assert_eq!(user_progress.overall_stats.total_practice_time_minutes, 150);
        
        // Generate comprehensive recommendations
        let recommendations = engine.generate_recommendations(user_progress, &[], 10).unwrap();
        assert!(!recommendations.is_empty());
        
        // Check rewards
        let reward_update = reward_system.check_rewards(user_progress).unwrap();
        assert!(reward_update.experience_gained > 0);
        
        // Generate learning plan
        let learning_plan = engine.create_learning_plan(user_progress, 8, 25).unwrap();
        assert_eq!(learning_plan.estimated_duration_weeks, 8);
        assert_eq!(learning_plan.daily_practice_minutes, 25);
        
        // Verify the system works end-to-end
        assert!(user_progress.ayah_progress.len() >= 5); // Should have progress on multiple ayahs
        assert!(user_progress.weak_points.len() <= 1); // Should have reduced weak points over time
    }
}

// Property-based tests
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_progress_update_properties(
            score in 0.0f64..=1.0,
            session_duration in 1u32..=120,
            surah in 1u8..=114,
            ayah in 1u16..=286
        ) {
            let mut tracker = ProgressTracker::new();
            let user_id = Uuid::new_v4();
            
            // Initialize user
            tracker.initialize_user_progress(user_id).unwrap();
            
            // Update progress
            let result = tracker.update_progress(user_id, surah, ayah, score, &[], session_duration);
            prop_assert!(result.is_ok());
            
            let progress = &tracker.user_progress[&user_id];
            
            // Properties that should always hold
            prop_assert_eq!(progress.overall_stats.total_recordings, 1);
            prop_assert_eq!(progress.overall_stats.total_practice_time_minutes, session_duration);
            prop_assert_eq!(progress.overall_stats.best_score, score);
            prop_assert_eq!(progress.overall_stats.average_score, score);
            
            // Ayah progress should be recorded
            let ayah_key = (surah, ayah);
            prop_assert!(progress.ayah_progress.contains_key(&ayah_key));
            
            let ayah_progress = &progress.ayah_progress[&ayah_key];
            prop_assert_eq!(ayah_progress.attempts_count, 1);
            prop_assert_eq!(ayah_progress.best_score, score);
            prop_assert_eq!(ayah_progress.latest_score, score);
        }
        
        #[test]
        fn test_mastery_level_properties(score in 0.0f64..=1.0) {
            let tracker = ProgressTracker::new();
            let level = tracker.calculate_mastery_level(score);
            
            // Properties based on score ranges
            match score {
                s if s < 0.4 => prop_assert_eq!(level, MasteryLevel::Beginner),
                s if s < 0.6 => prop_assert_eq!(level, MasteryLevel::Elementary),
                s if s < 0.75 => prop_assert_eq!(level, MasteryLevel::Intermediate),
                s if s < 0.9 => prop_assert_eq!(level, MasteryLevel::Advanced),
                _ => prop_assert_eq!(level, MasteryLevel::Expert),
            }
        }
        
        #[test]
        fn test_reward_system_experience_calculation(
            practice_minutes in 0u32..=1000,
            average_score in 0.0f64..=1.0
        ) {
            let reward_system = RewardSystem::new();
            let mut tracker = ProgressTracker::new();
            let user_id = Uuid::new_v4();
            
            tracker.initialize_user_progress(user_id).unwrap();
            let user_progress = tracker.user_progress.get_mut(&user_id).unwrap();
            user_progress.overall_stats.total_practice_time_minutes = practice_minutes;
            user_progress.overall_stats.average_score = average_score;
            
            let total_exp = reward_system.calculate_total_experience(user_progress);
            
            // Experience should be non-negative and increase with practice time and score
            prop_assert!(total_exp >= 0);
            
            // More practice time should generally mean more experience
            if practice_minutes > 0 {
                prop_assert!(total_exp >= practice_minutes * 2); // Base experience from practice
            }
        }
    }
}
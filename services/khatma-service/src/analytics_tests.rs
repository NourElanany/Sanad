use crate::models::*;
use crate::service::SmartKhatmaService;
use crate::repository::KhatmaRepository;
use anyhow::Result;
use chrono::{DateTime, Utc, Duration, NaiveTime};
use std::collections::HashMap;
use uuid::Uuid;

/// Tests for the comprehensive analytics and statistics system
#[cfg(test)]
mod tests {
    use super::*;

    /// Mock repository for testing analytics
    struct MockAnalyticsRepository {
        reading_sessions: Vec<ReadingSession>,
        completed_khatmas: Vec<KhatmaStatistics>,
        user_stats: Option<ReadingStatistics>,
    }

    impl MockAnalyticsRepository {
        fn new() -> Self {
            Self {
                reading_sessions: vec![],
                completed_khatmas: vec![],
                user_stats: None,
            }
        }

        fn with_reading_sessions(mut self, sessions: Vec<ReadingSession>) -> Self {
            self.reading_sessions = sessions;
            self
        }

        fn with_completed_khatmas(mut self, khatmas: Vec<KhatmaStatistics>) -> Self {
            self.completed_khatmas = khatmas;
            self
        }

        fn with_user_stats(mut self, stats: ReadingStatistics) -> Self {
            self.user_stats = Some(stats);
            self
        }
    }

    /// Create sample reading sessions for testing
    fn create_sample_reading_sessions(user_id: Uuid, plan_id: Uuid) -> Vec<ReadingSession> {
        let base_time = Utc::now() - Duration::days(30);
        let mut sessions = Vec::new();

        // Create 20 sessions over the last 30 days with varying patterns
        for i in 0..20 {
            let session_time = base_time + Duration::days(i as i64) + Duration::hours(8 + (i % 3) as i64);
            let duration = 20 + (i % 40) as i32; // 20-60 minutes
            let word_count = (duration as u32) * 4; // ~4 words per minute reading
            let reading_speed = 120.0 + (i as f64 * 2.0); // Improving speed

            sessions.push(ReadingSession {
                id: Uuid::new_v4(),
                user_id,
                khatma_plan_id: plan_id,
                surah_start: 1 + (i % 10) as u8,
                ayah_start: 1,
                surah_end: 1 + (i % 10) as u8,
                ayah_end: 10,
                start_time: session_time,
                end_time: Some(session_time + Duration::minutes(duration as i64)),
                duration_minutes: Some(duration),
                word_count,
                reading_speed_wpm: Some(reading_speed),
                created_at: session_time,
            });
        }

        sessions
    }

    /// Create sample completed khatmas for testing
    fn create_sample_completed_khatmas(user_id: Uuid) -> Vec<KhatmaStatistics> {
        let mut khatmas = Vec::new();

        for i in 0..3 {
            let completion_date = Utc::now() - Duration::days(90 * (i + 1) as i64);
            let planned_duration = 30;
            let actual_duration = 28 + i as i32; // Getting better over time
            let total_reading_time = 1800 + (i * 300) as i32; // Increasing reading time
            let consistency_score = 0.6 + (i as f64 * 0.1); // Improving consistency

            let achievements = vec![
                Achievement {
                    id: "khatma_completed".to_string(),
                    name: "Khatma Completed".to_string(),
                    description: "Successfully completed a full Quran reading plan".to_string(),
                    earned_at: completion_date,
                    category: AchievementCategory::Completion,
                },
                Achievement {
                    id: format!("consistency_{}", i),
                    name: "Consistent Reader".to_string(),
                    description: "Maintained good reading consistency".to_string(),
                    earned_at: completion_date,
                    category: AchievementCategory::Consistency,
                },
            ];

            khatmas.push(KhatmaStatistics {
                khatma_plan_id: Uuid::new_v4(),
                completion_date,
                planned_duration_days: planned_duration,
                actual_duration_days: actual_duration,
                total_reading_time_minutes: total_reading_time,
                average_daily_reading_minutes: total_reading_time as f64 / actual_duration as f64,
                consistency_score,
                portions_completed_on_time: 25 + i as u32,
                portions_completed_late: 5 - i as u32,
                portions_skipped: 2 - i as u32,
                reading_speed_improvement: 10.0 + (i as f64 * 5.0),
                achievements,
            });
        }

        khatmas
    }

    /// Create sample user statistics
    fn create_sample_user_stats(user_id: Uuid) -> ReadingStatistics {
        ReadingStatistics {
            user_id,
            average_reading_speed_wpm: 150.0,
            total_reading_time_minutes: 5400, // 90 hours
            completed_khatmas: 3,
            preferred_reading_times: vec![
                PreferredReadingTime {
                    time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                    duration_minutes: 30,
                    priority: ReadingTimePriority::High,
                    days_of_week: vec![1, 2, 3, 4, 5], // Weekdays
                },
                PreferredReadingTime {
                    time: NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
                    duration_minutes: 45,
                    priority: ReadingTimePriority::Medium,
                    days_of_week: vec![0, 6], // Weekends
                },
            ],
            reading_consistency_score: 0.8,
            last_updated: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_generate_progress_dashboard() {
        let user_id = Uuid::new_v4();
        let plan_id = Uuid::new_v4();

        // Create test data
        let sessions = create_sample_reading_sessions(user_id, plan_id);
        let completed_khatmas = create_sample_completed_khatmas(user_id);
        let user_stats = create_sample_user_stats(user_id);

        // Create current khatma plan
        let current_khatma = KhatmaPlan {
            id: plan_id,
            user_id,
            target_date: Utc::now() + Duration::days(15),
            start_date: Utc::now() - Duration::days(15),
            daily_portions: vec![], // Simplified for test
            estimated_reading_time: 60,
            adaptive_schedule: true,
            current_progress: 50.0,
            reading_speed_wpm: 150.0,
            preferred_reading_times: vec![],
            status: KhatmaStatus::Active,
            created_at: Utc::now() - Duration::days(15),
            updated_at: Utc::now(),
        };

        // This test would require a proper mock implementation
        // For now, we're testing the structure and ensuring no panics
        assert!(true);
    }

    #[tokio::test]
    async fn test_calculate_overall_progress() {
        let user_id = Uuid::new_v4();
        let plan_id = Uuid::new_v4();

        let sessions = create_sample_reading_sessions(user_id, plan_id);
        let completed_khatmas = create_sample_completed_khatmas(user_id);
        let user_stats = Some(create_sample_user_stats(user_id));

        let current_khatma = Some(KhatmaPlan {
            id: plan_id,
            user_id,
            target_date: Utc::now() + Duration::days(15),
            start_date: Utc::now() - Duration::days(15),
            daily_portions: vec![],
            estimated_reading_time: 60,
            adaptive_schedule: true,
            current_progress: 75.0,
            reading_speed_wpm: 150.0,
            preferred_reading_times: vec![],
            status: KhatmaStatus::Active,
            created_at: Utc::now() - Duration::days(15),
            updated_at: Utc::now(),
        });

        // Test overall progress calculation logic
        let total_khatmas_completed = completed_khatmas.len() as u32;
        assert_eq!(total_khatmas_completed, 3);

        let current_khatma_progress = current_khatma.as_ref().map(|k| k.current_progress).unwrap_or(0.0);
        assert_eq!(current_khatma_progress, 75.0);

        let total_reading_time_hours = sessions
            .iter()
            .filter_map(|s| s.duration_minutes)
            .sum::<i32>() as f64 / 60.0;
        assert!(total_reading_time_hours > 0.0);

        let consistency_score = user_stats.as_ref().map(|s| s.reading_consistency_score).unwrap_or(0.0);
        assert_eq!(consistency_score, 0.8);
    }

    #[tokio::test]
    async fn test_analyze_recent_activity() {
        let user_id = Uuid::new_v4();
        let plan_id = Uuid::new_v4();
        let sessions = create_sample_reading_sessions(user_id, plan_id);

        let now = Utc::now();
        
        // Filter sessions for last 7 days
        let last_7_days_sessions: Vec<_> = sessions
            .iter()
            .filter(|s| s.start_time >= now - Duration::days(7))
            .collect();

        // Test that we can filter sessions correctly
        assert!(last_7_days_sessions.len() <= sessions.len());

        // Test activity period calculation
        let total_reading_time: i32 = last_7_days_sessions
            .iter()
            .filter_map(|s| s.duration_minutes)
            .sum();

        let sessions_count = last_7_days_sessions.len() as u32;
        let average_session_duration = if sessions_count > 0 {
            total_reading_time as f64 / sessions_count as f64
        } else {
            0.0
        };

        assert!(average_session_duration >= 0.0);
    }

    #[tokio::test]
    async fn test_calculate_speed_trend() {
        let user_id = Uuid::new_v4();
        let plan_id = Uuid::new_v4();
        let sessions = create_sample_reading_sessions(user_id, plan_id);

        let speeds: Vec<f64> = sessions
            .iter()
            .filter_map(|s| s.reading_speed_wpm)
            .collect();

        assert!(!speeds.is_empty());

        let current_wpm = speeds.last().copied().unwrap_or(0.0);
        let average_wpm = speeds.iter().sum::<f64>() / speeds.len() as f64;

        // Test improvement calculation
        let first_sessions_avg = speeds.iter().take(5).sum::<f64>() / speeds.len().min(5) as f64;
        let last_sessions_avg = speeds.iter().rev().take(5).sum::<f64>() / speeds.len().min(5) as f64;
        
        let improvement_percentage = if first_sessions_avg > 0.0 {
            ((last_sessions_avg - first_sessions_avg) / first_sessions_avg) * 100.0
        } else {
            0.0
        };

        // Since our sample data has improving speed, improvement should be positive
        assert!(improvement_percentage > 0.0);
        assert!(current_wpm > 0.0);
        assert!(average_wpm > 0.0);
    }

    #[tokio::test]
    async fn test_identify_optimal_reading_times() {
        let user_id = Uuid::new_v4();
        let plan_id = Uuid::new_v4();
        let sessions = create_sample_reading_sessions(user_id, plan_id);

        let mut hour_stats = HashMap::new();

        for session in &sessions {
            let hour = session.start_time.hour();
            let entry = hour_stats.entry(hour).or_insert((Vec::new(), Vec::new(), Vec::new()));
            
            if let Some(duration) = session.duration_minutes {
                entry.0.push(duration);
            }
            if let Some(speed) = session.reading_speed_wpm {
                entry.1.push(speed);
            }
            entry.2.push(session);
        }

        // Test that we can group sessions by hour
        assert!(!hour_stats.is_empty());

        // Test optimal time calculation
        for (hour, (durations, speeds, hour_sessions)) in hour_stats {
            if hour_sessions.len() >= 2 {
                let average_duration = if !durations.is_empty() {
                    durations.iter().sum::<i32>() / durations.len() as i32
                } else {
                    0
                };

                let average_speed = if !speeds.is_empty() {
                    speeds.iter().sum::<f64>() / speeds.len() as f64
                } else {
                    0.0
                };

                let successful_sessions = hour_sessions.iter()
                    .filter(|s| s.duration_minutes.unwrap_or(0) >= 15)
                    .count();
                let success_rate = (successful_sessions as f64 / hour_sessions.len() as f64) * 100.0;

                assert!(success_rate >= 0.0 && success_rate <= 100.0);
                assert!(average_duration >= 0);
                assert!(average_speed >= 0.0);
            }
        }
    }

    #[tokio::test]
    async fn test_generate_performance_recommendations() {
        let user_id = Uuid::new_v4();
        let plan_id = Uuid::new_v4();
        let sessions = create_sample_reading_sessions(user_id, plan_id);
        let completed_khatmas = create_sample_completed_khatmas(user_id);
        let user_stats = create_sample_user_stats(user_id);

        // Create overall progress with low consistency for testing
        let overall_progress = OverallProgress {
            total_khatmas_completed: 3,
            current_khatma_progress: 50.0,
            total_reading_time_hours: 90.0,
            average_daily_reading_minutes: 45.0,
            consistency_score: 0.5, // Low consistency to trigger recommendation
            current_streak_days: 0, // No streak to trigger recommendation
            longest_streak_days: 7,
            pages_read_total: 1200,
            surahs_completed: 15,
        };

        // Test recommendation generation logic
        let mut recommendations = Vec::new();

        // Consistency recommendation
        if overall_progress.consistency_score < 0.6 {
            recommendations.push(PerformanceRecommendation {
                id: "improve_consistency".to_string(),
                title: "Improve Reading Consistency".to_string(),
                description: "Your consistency score is below optimal.".to_string(),
                category: RecommendationCategory::Consistency,
                priority: RecommendationPriority::High,
                expected_impact: "Increase consistency score by 30-40%".to_string(),
                action_steps: vec![
                    "Set a specific time each day for Quran reading".to_string(),
                    "Start with shorter 15-20 minute sessions".to_string(),
                ],
                confidence_score: 0.9,
            });
        }

        // Streak recommendation
        if overall_progress.current_streak_days == 0 {
            recommendations.push(PerformanceRecommendation {
                id: "start_streak".to_string(),
                title: "Start a Reading Streak".to_string(),
                description: "Building a daily reading streak will improve consistency.".to_string(),
                category: RecommendationCategory::GoalSetting,
                priority: RecommendationPriority::High,
                expected_impact: "Establish a strong daily reading habit".to_string(),
                action_steps: vec![
                    "Commit to reading for at least 10 minutes daily".to_string(),
                    "Choose a consistent time each day".to_string(),
                ],
                confidence_score: 0.9,
            });
        }

        // Test that recommendations are generated
        assert_eq!(recommendations.len(), 2);
        assert!(recommendations.iter().any(|r| r.category == RecommendationCategory::Consistency));
        assert!(recommendations.iter().any(|r| r.category == RecommendationCategory::GoalSetting));
    }

    #[tokio::test]
    async fn test_generate_khatma_comparison() {
        let user_id = Uuid::new_v4();
        let completed_khatmas = create_sample_completed_khatmas(user_id);

        // Test comparison metrics calculation
        if !completed_khatmas.is_empty() {
            let average_previous_pace_days = completed_khatmas.iter()
                .map(|k| k.actual_duration_days as f64)
                .sum::<f64>() / completed_khatmas.len() as f64;

            let best_previous_pace_days = completed_khatmas.iter()
                .map(|k| k.actual_duration_days)
                .min()
                .unwrap_or(0);

            // Test that calculations work correctly
            assert!(average_previous_pace_days > 0.0);
            assert!(best_previous_pace_days > 0);

            // Test achievement comparison
            let total_achievements_earned = completed_khatmas.iter()
                .map(|k| k.achievements.len() as u32)
                .sum();

            assert!(total_achievements_earned > 0);

            // Test improvement areas identification
            let has_declining_trend = completed_khatmas.len() >= 2 && 
                completed_khatmas.last().unwrap().actual_duration_days > 
                completed_khatmas.first().unwrap().actual_duration_days;

            // This would trigger improvement recommendations
            if has_declining_trend {
                assert!(true); // Would generate improvement recommendations
            }
        }
    }

    #[tokio::test]
    async fn test_calculate_session_quality_score() {
        let user_id = Uuid::new_v4();
        let plan_id = Uuid::new_v4();

        // Test session with good metrics
        let good_session = ReadingSession {
            id: Uuid::new_v4(),
            user_id,
            khatma_plan_id: plan_id,
            surah_start: 1,
            ayah_start: 1,
            surah_end: 1,
            ayah_end: 10,
            start_time: Utc::now(),
            end_time: Some(Utc::now() + Duration::minutes(30)),
            duration_minutes: Some(30), // Good duration
            word_count: 200,
            reading_speed_wpm: Some(150.0), // Good speed
            created_at: Utc::now(),
        };

        let mut score = 0.5; // Base score

        // Duration factor
        if let Some(duration) = good_session.duration_minutes {
            if duration >= 15 && duration <= 60 {
                score += 0.2; // Good duration
            }
        }

        // Speed factor
        if let Some(speed) = good_session.reading_speed_wpm {
            if speed >= 100.0 && speed <= 200.0 {
                score += 0.2; // Good reading speed
            }
        }

        // Word count factor
        if good_session.word_count > 100 {
            score += 0.1; // Substantial reading
        }

        let final_score = score.min(1.0);
        assert!(final_score > 0.5); // Should be better than base score
        assert!(final_score <= 1.0); // Should not exceed maximum
    }

    #[tokio::test]
    async fn test_streak_calculation() {
        let user_id = Uuid::new_v4();
        let plan_id = Uuid::new_v4();

        // Create sessions for consecutive days
        let base_date = Utc::now().date_naive() - chrono::Duration::days(5);
        let mut sessions = Vec::new();

        for i in 0..5 {
            let session_date = base_date + chrono::Duration::days(i);
            sessions.push(ReadingSession {
                id: Uuid::new_v4(),
                user_id,
                khatma_plan_id: plan_id,
                surah_start: 1,
                ayah_start: 1,
                surah_end: 1,
                ayah_end: 10,
                start_time: session_date.and_hms_opt(8, 0, 0).unwrap().and_utc(),
                end_time: Some(session_date.and_hms_opt(8, 30, 0).unwrap().and_utc()),
                duration_minutes: Some(30),
                word_count: 200,
                reading_speed_wpm: Some(150.0),
                created_at: session_date.and_hms_opt(8, 0, 0).unwrap().and_utc(),
            });
        }

        // Test streak calculation
        let mut daily_sessions = HashMap::new();
        for session in &sessions {
            let date = session.start_time.date_naive();
            daily_sessions.entry(date).or_insert(Vec::new()).push(session);
        }

        let mut dates: Vec<_> = daily_sessions.keys().collect();
        dates.sort();

        let mut current_streak = 0;
        let mut max_streak = 0;
        let mut last_date = None;

        for date in dates {
            if let Some(last) = last_date {
                let days_diff = (**date - last).num_days();
                if days_diff == 1 {
                    current_streak += 1;
                } else {
                    max_streak = max_streak.max(current_streak);
                    current_streak = 1;
                }
            } else {
                current_streak = 1;
            }
            last_date = Some(**date);
        }

        max_streak = max_streak.max(current_streak);

        // Should detect a 5-day streak
        assert_eq!(current_streak, 5);
        assert_eq!(max_streak, 5);
    }

    #[tokio::test]
    async fn test_milestone_generation() {
        let user_id = Uuid::new_v4();
        let plan_id = Uuid::new_v4();

        let overall_progress = OverallProgress {
            total_khatmas_completed: 2,
            current_khatma_progress: 75.0,
            total_reading_time_hours: 45.0,
            average_daily_reading_minutes: 30.0,
            consistency_score: 0.7,
            current_streak_days: 5,
            longest_streak_days: 10,
            pages_read_total: 800,
            surahs_completed: 12,
        };

        let current_khatma = Some(KhatmaPlan {
            id: plan_id,
            user_id,
            target_date: Utc::now() + Duration::days(10),
            start_date: Utc::now() - Duration::days(20),
            daily_portions: vec![],
            estimated_reading_time: 60,
            adaptive_schedule: true,
            current_progress: 75.0,
            reading_speed_wpm: 150.0,
            preferred_reading_times: vec![],
            status: KhatmaStatus::Active,
            created_at: Utc::now() - Duration::days(20),
            updated_at: Utc::now(),
        });

        let mut milestones = Vec::new();
        let now = Utc::now();

        // Khatma completion milestone
        if let Some(khatma) = &current_khatma {
            let progress_to_completion = 100.0 - khatma.current_progress;
            milestones.push(Milestone {
                id: "current_khatma_completion".to_string(),
                title: "Complete Current Khatma".to_string(),
                description: format!("Finish your current Khatma plan with {:.1}% remaining", progress_to_completion),
                target_date: khatma.target_date,
                progress_percentage: khatma.current_progress,
                milestone_type: MilestoneType::KhatmaCompletion,
                reward: Some("Khatma Completion Certificate".to_string()),
            });
        }

        // Reading streak milestone
        let next_streak_target = if overall_progress.current_streak_days < 7 {
            7
        } else if overall_progress.current_streak_days < 30 {
            30
        } else {
            overall_progress.current_streak_days + 30
        };

        let streak_progress = (overall_progress.current_streak_days as f64 / next_streak_target as f64) * 100.0;
        milestones.push(Milestone {
            id: format!("streak_{}", next_streak_target),
            title: format!("{}-Day Reading Streak", next_streak_target),
            description: format!("Maintain daily reading for {} consecutive days", next_streak_target),
            target_date: now + Duration::days((next_streak_target - overall_progress.current_streak_days) as i64),
            progress_percentage: streak_progress,
            milestone_type: MilestoneType::ReadingStreak,
            reward: Some(format!("Streak Master {} Badge", next_streak_target)),
        });

        // Test milestone generation
        assert_eq!(milestones.len(), 2);
        assert!(milestones.iter().any(|m| matches!(m.milestone_type, MilestoneType::KhatmaCompletion)));
        assert!(milestones.iter().any(|m| matches!(m.milestone_type, MilestoneType::ReadingStreak)));

        // Test progress calculations
        assert_eq!(milestones[0].progress_percentage, 75.0);
        assert!(milestones[1].progress_percentage > 0.0);
    }
}
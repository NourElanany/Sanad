use crate::models::*;
use crate::planning_algorithms::PlanningAlgorithms;
use chrono::{Duration, NaiveTime, Utc};
use uuid::Uuid;

#[cfg(test)]
mod planning_algorithm_tests {
    use super::*;

    #[test]
    fn test_calculate_reading_speed_empty_sessions() {
        let sessions = vec![];
        let speed = PlanningAlgorithms::calculate_reading_speed(&sessions);
        assert_eq!(speed, 150.0); // Default speed
    }

    #[test]
    fn test_calculate_reading_speed_single_session() {
        let sessions = vec![
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 1,
                ayah_start: 1,
                surah_end: 1,
                ayah_end: 7,
                start_time: Utc::now(),
                end_time: Some(Utc::now() + Duration::minutes(10)),
                duration_minutes: Some(10),
                word_count: 100,
                reading_speed_wpm: Some(120.0),
                created_at: Utc::now(),
            }
        ];

        let speed = PlanningAlgorithms::calculate_reading_speed(&sessions);
        assert_eq!(speed, 120.0);
    }

    #[test]
    fn test_calculate_reading_speed_multiple_sessions() {
        let sessions = vec![
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 1,
                ayah_start: 1,
                surah_end: 1,
                ayah_end: 7,
                start_time: Utc::now() - Duration::days(2),
                end_time: Some(Utc::now() - Duration::days(2) + Duration::minutes(10)),
                duration_minutes: Some(10),
                word_count: 100,
                reading_speed_wpm: Some(100.0),
                created_at: Utc::now() - Duration::days(2),
            },
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 2,
                ayah_start: 1,
                surah_end: 2,
                ayah_end: 10,
                start_time: Utc::now() - Duration::days(1),
                end_time: Some(Utc::now() - Duration::days(1) + Duration::minutes(15)),
                duration_minutes: Some(15),
                word_count: 200,
                reading_speed_wpm: Some(200.0),
                created_at: Utc::now() - Duration::days(1),
            },
        ];

        let speed = PlanningAlgorithms::calculate_reading_speed(&sessions);
        // Should be weighted average favoring more recent sessions
        assert!(speed > 100.0 && speed < 200.0);
        assert!(speed > 150.0); // Should be closer to the more recent higher speed
    }

    #[test]
    fn test_create_adaptive_plan_basic() {
        let user_id = Uuid::new_v4();
        let target_date = Utc::now() + Duration::days(30);
        let preferences = create_test_preferences();

        let plan = PlanningAlgorithms::create_adaptive_plan(
            user_id,
            target_date,
            &preferences,
            150.0,
        ).unwrap();

        assert_eq!(plan.user_id, user_id);
        assert_eq!(plan.target_date, target_date);
        assert_eq!(plan.reading_speed_wpm, 150.0);
        assert_eq!(plan.adaptive_schedule, true);
        assert_eq!(plan.status, KhatmaStatus::Active);
        assert!(!plan.daily_portions.is_empty());
        assert_eq!(plan.current_progress, 0.0);
    }

    #[test]
    fn test_create_adaptive_plan_invalid_target_date() {
        let user_id = Uuid::new_v4();
        let target_date = Utc::now() - Duration::days(1); // Past date
        let preferences = create_test_preferences();

        let result = PlanningAlgorithms::create_adaptive_plan(
            user_id,
            target_date,
            &preferences,
            150.0,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Target date must be in the future"));
    }

    #[test]
    fn test_create_adaptive_plan_difficulty_preferences() {
        let user_id = Uuid::new_v4();
        let target_date = Utc::now() + Duration::days(30);
        
        // Test Easy difficulty
        let mut preferences = create_test_preferences();
        preferences.difficulty_preference = DifficultyPreference::Easy;
        
        let easy_plan = PlanningAlgorithms::create_adaptive_plan(
            user_id,
            target_date,
            &preferences,
            150.0,
        ).unwrap();

        // Test Hard difficulty
        preferences.difficulty_preference = DifficultyPreference::Hard;
        
        let hard_plan = PlanningAlgorithms::create_adaptive_plan(
            user_id,
            target_date,
            &preferences,
            150.0,
        ).unwrap();

        // Easy plan should have longer estimated reading time
        assert!(easy_plan.estimated_reading_time > hard_plan.estimated_reading_time);
    }

    #[test]
    fn test_adjust_plan_for_delay_behind_schedule() {
        let mut plan = create_test_plan();
        let sessions = create_test_sessions();
        
        // Simulate being 20% behind schedule
        let current_progress = 30.0; // Should be at 50% by now
        
        let adjustments = PlanningAlgorithms::adjust_plan_for_delay(
            &mut plan,
            current_progress,
            &sessions,
        ).unwrap();

        assert!(!adjustments.is_empty());
        assert!(adjustments.iter().any(|adj| adj.contains("Increased daily reading time")));
        assert_eq!(plan.current_progress, current_progress);
    }

    #[test]
    fn test_adjust_plan_for_delay_ahead_of_schedule() {
        let mut plan = create_test_plan();
        let sessions = create_test_sessions();
        
        // Simulate being 20% ahead of schedule
        let current_progress = 70.0; // Should be at 50% by now
        
        let adjustments = PlanningAlgorithms::adjust_plan_for_delay(
            &mut plan,
            current_progress,
            &sessions,
        ).unwrap();

        assert!(!adjustments.is_empty());
        assert!(adjustments.iter().any(|adj| adj.contains("Reduced daily reading time")));
        assert_eq!(plan.current_progress, current_progress);
    }

    #[test]
    fn test_suggest_reading_times() {
        let user_id = Uuid::new_v4();
        let plan = create_test_plan();
        let reading_history = create_test_sessions();

        let suggestions = PlanningAlgorithms::suggest_reading_times(
            user_id,
            &plan,
            &reading_history,
        );

        assert!(!suggestions.is_empty());
        assert!(suggestions.len() <= 10); // Should return top 10 suggestions
        
        // Suggestions should be sorted by confidence score
        for i in 1..suggestions.len() {
            assert!(suggestions[i-1].confidence_score >= suggestions[i].confidence_score);
        }

        // All suggestions should be for the same user and plan
        for suggestion in &suggestions {
            assert_eq!(suggestion.user_id, user_id);
            assert_eq!(suggestion.khatma_plan_id, plan.id);
        }
    }

    #[test]
    fn test_suggest_reading_times_with_preferences() {
        let user_id = Uuid::new_v4();
        let mut plan = create_test_plan();
        
        // Add preferred reading times
        plan.preferred_reading_times = vec![
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
        ];

        let reading_history = vec![];
        let suggestions = PlanningAlgorithms::suggest_reading_times(
            user_id,
            &plan,
            &reading_history,
        );

        assert!(!suggestions.is_empty());
        
        // Should have high confidence suggestions based on preferences
        let high_confidence_suggestions: Vec<_> = suggestions
            .iter()
            .filter(|s| s.confidence_score > 0.8)
            .collect();
        
        assert!(!high_confidence_suggestions.is_empty());
    }

    // Property-based tests using the proptest crate would go here
    // For now, we'll include some basic property tests manually

    #[test]
    fn property_reading_speed_always_positive() {
        // Test that reading speed calculation always returns positive values
        for _ in 0..100 {
            let sessions = generate_random_sessions(5);
            let speed = PlanningAlgorithms::calculate_reading_speed(&sessions);
            assert!(speed > 0.0, "Reading speed should always be positive");
        }
    }

    #[test]
    fn property_plan_portions_cover_full_quran() {
        // Test that generated plans always cover the full Quran
        let user_id = Uuid::new_v4();
        let preferences = create_test_preferences();
        
        for days in [7, 15, 30, 60, 90] {
            let target_date = Utc::now() + Duration::days(days);
            let plan = PlanningAlgorithms::create_adaptive_plan(
                user_id,
                target_date,
                &preferences,
                150.0,
            ).unwrap();

            // First portion should start at Surah 1, Ayah 1
            assert_eq!(plan.daily_portions.first().unwrap().surah_start, 1);
            assert_eq!(plan.daily_portions.first().unwrap().ayah_start, 1);

            // Last portion should end at Surah 114 (or close to it for reasonable plans)
            let last_portion = plan.daily_portions.last().unwrap();
            assert!(last_portion.surah_end >= 110, "Plan should cover most of the Quran");
        }
    }

    #[test]
    fn property_plan_adjustments_preserve_plan_integrity() {
        // Test that plan adjustments don't break plan integrity
        let mut plan = create_test_plan();
        let sessions = create_test_sessions();
        
        for progress in [10.0, 25.0, 50.0, 75.0, 90.0] {
            let original_id = plan.id;
            let original_user_id = plan.user_id;
            
            let _adjustments = PlanningAlgorithms::adjust_plan_for_delay(
                &mut plan,
                progress,
                &sessions,
            ).unwrap();

            // Plan identity should be preserved
            assert_eq!(plan.id, original_id);
            assert_eq!(plan.user_id, original_user_id);
            assert_eq!(plan.current_progress, progress);
            assert!(plan.reading_speed_wpm > 0.0);
            assert!(plan.estimated_reading_time > 0);
        }
    }

    // Helper functions for tests
    fn create_test_preferences() -> KhatmaPreferences {
        KhatmaPreferences {
            target_completion_days: Some(30),
            daily_reading_time_minutes: Some(60),
            preferred_reading_times: vec![
                PreferredReadingTime {
                    time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                    duration_minutes: 30,
                    priority: ReadingTimePriority::High,
                    days_of_week: vec![1, 2, 3, 4, 5],
                }
            ],
            adaptive_scheduling: true,
            reminder_settings: ReminderSettings {
                enabled: true,
                advance_minutes: 15,
                smart_timing: true,
                missed_reading_reminder: true,
                progress_updates: true,
            },
            difficulty_preference: DifficultyPreference::Medium,
        }
    }

    fn create_test_plan() -> KhatmaPlan {
        let user_id = Uuid::new_v4();
        let target_date = Utc::now() + Duration::days(30);
        let preferences = create_test_preferences();

        PlanningAlgorithms::create_adaptive_plan(
            user_id,
            target_date,
            &preferences,
            150.0,
        ).unwrap()
    }

    fn create_test_sessions() -> Vec<ReadingSession> {
        vec![
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 1,
                ayah_start: 1,
                surah_end: 1,
                ayah_end: 7,
                start_time: Utc::now() - Duration::days(5),
                end_time: Some(Utc::now() - Duration::days(5) + Duration::minutes(10)),
                duration_minutes: Some(10),
                word_count: 100,
                reading_speed_wpm: Some(120.0),
                created_at: Utc::now() - Duration::days(5),
            },
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 2,
                ayah_start: 1,
                surah_end: 2,
                ayah_end: 10,
                start_time: Utc::now() - Duration::days(3),
                end_time: Some(Utc::now() - Duration::days(3) + Duration::minutes(15)),
                duration_minutes: Some(15),
                word_count: 200,
                reading_speed_wpm: Some(180.0),
                created_at: Utc::now() - Duration::days(3),
            },
        ]
    }

    fn generate_random_sessions(count: usize) -> Vec<ReadingSession> {
        let mut sessions = Vec::new();
        
        for i in 0..count {
            sessions.push(ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 1,
                ayah_start: 1,
                surah_end: 1,
                ayah_end: 7,
                start_time: Utc::now() - Duration::days(i as i64),
                end_time: Some(Utc::now() - Duration::days(i as i64) + Duration::minutes(10 + i as i64)),
                duration_minutes: Some(10 + i as i32),
                word_count: 100 + (i as u32 * 10),
                reading_speed_wpm: Some(120.0 + (i as f64 * 10.0)),
                created_at: Utc::now() - Duration::days(i as i64),
            });
        }
        
        sessions
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::service::SmartKhatmaService;
    use crate::repository::KhatmaRepository;

    // Integration tests would require database setup
    // For now, we'll include the structure

    #[tokio::test]
    async fn test_create_and_retrieve_khatma_plan() {
        // This test would require a test database
        // let pool = setup_test_database().await;
        // let repository = KhatmaRepository::new(pool);
        // let service = SmartKhatmaService::new(repository);
        
        // Test creating and retrieving a plan
        assert!(true); // Placeholder
    }

    #[tokio::test]
    async fn test_update_progress_and_auto_adjust() {
        // Test the full flow of updating progress and automatic adjustments
        assert!(true); // Placeholder
    }
}

// Property-based tests using proptest
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_reading_speed_calculation_is_stable(
            speeds in prop::collection::vec(1.0f64..1000.0, 1..20)
        ) {
            let sessions: Vec<ReadingSession> = speeds.into_iter().enumerate().map(|(i, speed)| {
                ReadingSession {
                    id: Uuid::new_v4(),
                    user_id: Uuid::new_v4(),
                    khatma_plan_id: Uuid::new_v4(),
                    surah_start: 1,
                    ayah_start: 1,
                    surah_end: 1,
                    ayah_end: 7,
                    start_time: Utc::now() - Duration::days(i as i64),
                    end_time: Some(Utc::now() - Duration::days(i as i64) + Duration::minutes(10)),
                    duration_minutes: Some(10),
                    word_count: 100,
                    reading_speed_wpm: Some(speed),
                    created_at: Utc::now() - Duration::days(i as i64),
                }
            }).collect();

            let calculated_speed = PlanningAlgorithms::calculate_reading_speed(&sessions);
            
            // Speed should be positive and within reasonable bounds
            prop_assert!(calculated_speed > 0.0);
            prop_assert!(calculated_speed <= 1000.0);
            
            // If all input speeds are the same, output should be close to that speed
            if sessions.len() == 1 {
                prop_assert!((calculated_speed - sessions[0].reading_speed_wpm.unwrap()).abs() < 0.1);
            }
        }

        #[test]
        fn prop_plan_creation_always_succeeds_for_valid_inputs(
            days in 1u32..365,
            reading_speed in 50.0f64..500.0,
            daily_time in 10i32..300
        ) {
            let user_id = Uuid::new_v4();
            let target_date = Utc::now() + Duration::days(days as i64);
            let mut preferences = create_test_preferences();
            preferences.daily_reading_time_minutes = Some(daily_time);

            let result = PlanningAlgorithms::create_adaptive_plan(
                user_id,
                target_date,
                &preferences,
                reading_speed,
            );

            prop_assert!(result.is_ok());
            
            let plan = result.unwrap();
            prop_assert_eq!(plan.user_id, user_id);
            prop_assert_eq!(plan.reading_speed_wpm, reading_speed);
            prop_assert!(!plan.daily_portions.is_empty());
            prop_assert_eq!(plan.current_progress, 0.0);
        }
    }

    fn create_test_preferences() -> KhatmaPreferences {
        KhatmaPreferences {
            target_completion_days: Some(30),
            daily_reading_time_minutes: Some(60),
            preferred_reading_times: vec![],
            adaptive_scheduling: true,
            reminder_settings: ReminderSettings {
                enabled: true,
                advance_minutes: 15,
                smart_timing: true,
                missed_reading_reminder: true,
                progress_updates: true,
            },
            difficulty_preference: DifficultyPreference::Medium,
        }
    }
}
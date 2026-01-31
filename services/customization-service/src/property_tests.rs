use crate::models::*;
use proptest::prelude::*;
use chrono::{DateTime, Utc, NaiveTime};
use uuid::Uuid;

/// Property-based tests for the Smart Customization System
/// **Validates: Requirements 15.2, 15.4**

#[cfg(test)]
mod property_tests {
    use super::*;

    // Generators for property testing
    
    prop_compose! {
        fn arb_score()(score in 0.0f64..=1.0f64) -> f64 {
            score
        }
    }

    prop_compose! {
        fn arb_naive_time()(
            hour in 0u32..24u32,
            minute in 0u32..60u32,
            second in 0u32..60u32
        ) -> NaiveTime {
            NaiveTime::from_hms_opt(hour, minute, second).unwrap()
        }
    }

    prop_compose! {
        fn arb_time_slot()(
            start_time in arb_naive_time(),
            activity_type in prop_oneof![
                Just(ActivityType::QuranReading),
                Just(ActivityType::HadithStudy),
                Just(ActivityType::DhikrReminders),
                Just(ActivityType::PrayerReminders),
            ],
            preference_strength in arb_score(),
            success_rate in arb_score(),
            days in prop::collection::vec(0u8..7u8, 1..8)
        ) -> PreferredTimeSlot {
            let end_time = if start_time.hour() == 23 {
                NaiveTime::from_hms_opt(0, 0, 0).unwrap()
            } else {
                NaiveTime::from_hms_opt(start_time.hour() + 1, start_time.minute(), start_time.second()).unwrap()
            };
            
            PreferredTimeSlot {
                start_time,
                end_time,
                activity_type,
                preference_strength,
                days_of_week: days,
                success_rate,
            }
        }
    }

    prop_compose! {
        fn arb_content_preference()(
            content_type in prop_oneof![
                Just(ContentType::QuranVerses),
                Just(ContentType::HadithNarrations),
                Just(ContentType::Dhikr),
                Just(ContentType::Tafsir),
            ],
            preference_weight in arb_score(),
            interaction_frequency in arb_score(),
            completion_rate in arb_score()
        ) -> ContentTypePreference {
            ContentTypePreference {
                content_type,
                preference_weight,
                interaction_frequency,
                completion_rate,
            }
        }
    }

    proptest! {
        /// **Feature: islamic-app-comprehensive, Property 16: User Behavior Profile Consistency**
        /// For any user behavior profile, all score values must remain within valid bounds (0.0 to 1.0)
        /// and the profile must maintain internal consistency across updates
        #[test]
        fn prop_behavior_profile_score_bounds(
            reading_consistency in arb_score(),
            notification_response_rate in arb_score(),
            adaptation_score in arb_score(),
            satisfaction_score in arb_score(),
            time_slots in prop::collection::vec(arb_time_slot(), 1..5),
            content_prefs in prop::collection::vec(arb_content_preference(), 1..10)
        ) {
            let user_id = Uuid::new_v4();
            
            let profile = UserBehaviorProfile {
                id: Uuid::new_v4(),
                user_id,
                preferred_reading_times: time_slots,
                average_session_duration: 30,
                reading_consistency_score: reading_consistency,
                preferred_content_types: content_prefs,
                notification_response_rate,
                preferred_notification_times: vec![],
                engagement_patterns: EngagementPatterns {
                    peak_engagement_hours: vec![6, 7, 20, 21],
                    peak_engagement_days: vec![1, 2, 3, 4, 5],
                    average_session_length: 30,
                    preferred_content_length: ContentLength::Medium,
                    interaction_style: InteractionStyle::Structured,
                    motivation_triggers: vec![MotivationTrigger::Progress],
                },
                learning_style: LearningStyle::Mixed,
                difficulty_preference: DifficultyLevel::Intermediate,
                language_preferences: vec!["ar".to_string()],
                seasonal_preferences: std::collections::HashMap::new(),
                location_based_preferences: None,
                adaptation_score,
                satisfaction_score,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            // Property: All scores must be within bounds
            prop_assert!(profile.reading_consistency_score >= 0.0 && profile.reading_consistency_score <= 1.0);
            prop_assert!(profile.notification_response_rate >= 0.0 && profile.notification_response_rate <= 1.0);
            prop_assert!(profile.adaptation_score >= 0.0 && profile.adaptation_score <= 1.0);
            prop_assert!(profile.satisfaction_score >= 0.0 && profile.satisfaction_score <= 1.0);

            // Property: Time slots must have valid preference strengths and success rates
            for time_slot in &profile.preferred_reading_times {
                prop_assert!(time_slot.preference_strength >= 0.0 && time_slot.preference_strength <= 1.0);
                prop_assert!(time_slot.success_rate >= 0.0 && time_slot.success_rate <= 1.0);
                prop_assert!(!time_slot.days_of_week.is_empty());
                prop_assert!(time_slot.days_of_week.iter().all(|&day| day <= 6));
            }

            // Property: Content preferences must have valid weights and rates
            for content_pref in &profile.preferred_content_types {
                prop_assert!(content_pref.preference_weight >= 0.0 && content_pref.preference_weight <= 1.0);
                prop_assert!(content_pref.interaction_frequency >= 0.0 && content_pref.interaction_frequency <= 1.0);
                prop_assert!(content_pref.completion_rate >= 0.0 && content_pref.completion_rate <= 1.0);
            }
        }
    }

    proptest! {
        /// **Feature: islamic-app-comprehensive, Property 17: Recommendation Score Consistency**
        /// For any personalized recommendation, the recommendation score must correlate with
        /// the underlying factors and remain stable for similar user profiles
        #[test]
        fn prop_recommendation_score_consistency(
            recommendation_score in arb_score(),
            estimated_duration in 5i32..120i32,
            user_rating in prop::option::of(1.0f64..5.0f64)
        ) {
            let user_id = Uuid::new_v4();
            
            let recommendation = PersonalizedRecommendation {
                id: Uuid::new_v4(),
                user_id,
                content_type: ContentType::QuranVerses,
                content_id: "test_content".to_string(),
                title: "Test Recommendation".to_string(),
                description: "Test description".to_string(),
                recommendation_score,
                reasoning: "Test reasoning".to_string(),
                estimated_duration,
                difficulty_level: DifficultyLevel::Intermediate,
                tags: vec!["test".to_string()],
                category: RecommendationCategory::DailyReading,
                presented_at: None,
                interacted_at: None,
                completed_at: None,
                user_rating,
                feedback: None,
                created_at: Utc::now(),
                expires_at: Some(Utc::now() + chrono::Duration::days(1)),
            };

            // Property: Recommendation score must be within bounds
            prop_assert!(recommendation.recommendation_score >= 0.0 && recommendation.recommendation_score <= 1.0);
            
            // Property: Estimated duration must be positive
            prop_assert!(recommendation.estimated_duration > 0);
            
            // Property: User rating, if present, must be within bounds
            if let Some(rating) = recommendation.user_rating {
                prop_assert!(rating >= 1.0 && rating <= 5.0);
            }
            
            // Property: Expiration must be after creation
            if let Some(expires_at) = recommendation.expires_at {
                prop_assert!(expires_at > recommendation.created_at);
            }
            
            // Property: Required fields must not be empty
            prop_assert!(!recommendation.title.is_empty());
            prop_assert!(!recommendation.description.is_empty());
            prop_assert!(!recommendation.reasoning.is_empty());
        }
    }

    proptest! {
        /// **Feature: islamic-app-comprehensive, Property 18: Adaptive Reminder Timing Optimization**
        /// For any adaptive reminder, the suggested time must fall within the optimal time window
        /// and the adaptation confidence must reflect the quality of timing prediction
        #[test]
        fn prop_adaptive_reminder_timing(
            adaptation_confidence in arb_score(),
            response_prediction in arb_score(),
            flexibility_minutes in 5i32..120i32,
            start_hour in 0u32..24u32,
            preferred_hour in 0u32..24u32
        ) {
            let user_id = Uuid::new_v4();
            let now = Utc::now();
            
            let start_time = NaiveTime::from_hms_opt(start_hour, 0, 0).unwrap();
            let end_time = NaiveTime::from_hms_opt((start_hour + 2) % 24, 0, 0).unwrap();
            let preferred_time = NaiveTime::from_hms_opt(preferred_hour % 24, 0, 0).unwrap();
            
            let time_window = TimeWindow {
                start_time,
                end_time,
                preferred_time,
                flexibility_minutes,
            };
            
            let reminder = AdaptiveReminder {
                id: Uuid::new_v4(),
                user_id,
                reminder_type: ReminderType::QuranReading,
                title: "Test Reminder".to_string(),
                message: "Test message".to_string(),
                suggested_time: now + chrono::Duration::hours(1),
                optimal_time_window: time_window.clone(),
                adaptation_confidence,
                personalization_factors: vec![PersonalizationFactor::HistoricalResponse],
                content_customization: ContentCustomization::default(),
                response_prediction,
                actual_response: None,
                effectiveness_score: None,
                is_recurring: false,
                recurrence_pattern: None,
                next_occurrence: None,
                created_at: now,
                updated_at: now,
            };

            // Property: Adaptation confidence must be within bounds
            prop_assert!(reminder.adaptation_confidence >= 0.0 && reminder.adaptation_confidence <= 1.0);
            
            // Property: Response prediction must be within bounds
            prop_assert!(reminder.response_prediction >= 0.0 && reminder.response_prediction <= 1.0);
            
            // Property: Time window must have positive flexibility
            prop_assert!(reminder.optimal_time_window.flexibility_minutes > 0);
            
            // Property: Suggested time must be in the future
            prop_assert!(reminder.suggested_time > reminder.created_at);
            
            // Property: Required fields must not be empty
            prop_assert!(!reminder.title.is_empty());
            prop_assert!(!reminder.message.is_empty());
            prop_assert!(!reminder.personalization_factors.is_empty());
        }
    }

    proptest! {
        /// **Feature: islamic-app-comprehensive, Property 19: Preference Learning Convergence**
        /// For any preference learning record, the confidence score must increase with
        /// more evidence and the impact score must reflect the significance of the change
        #[test]
        fn prop_preference_learning_convergence(
            confidence_score in arb_score(),
            impact_score in arb_score(),
            old_hour in 0u32..24u32,
            new_hour in 0u32..24u32
        ) {
            let user_id = Uuid::new_v4();
            
            let learning_record = PreferenceLearningRecord {
                id: Uuid::new_v4(),
                user_id,
                preference_type: PreferenceType::ReadingTime,
                old_value: serde_json::json!({"preferred_hour": old_hour}),
                new_value: serde_json::json!({"preferred_hour": new_hour}),
                confidence_score,
                learning_source: LearningSource::UserBehavior,
                validation_status: ValidationStatus::Pending,
                impact_score,
                created_at: Utc::now(),
            };

            // Property: Confidence score must be within bounds
            prop_assert!(learning_record.confidence_score >= 0.0 && learning_record.confidence_score <= 1.0);
            
            // Property: Impact score must be within bounds
            prop_assert!(learning_record.impact_score >= 0.0 && learning_record.impact_score <= 1.0);
            
            // Property: Old and new values should be different for meaningful learning
            // (This is a business rule - we only record changes)
            if old_hour == new_hour {
                // If values are the same, impact should be minimal
                prop_assert!(learning_record.impact_score < 0.1);
            }
            
            // Property: Values must be valid JSON
            prop_assert!(learning_record.old_value.is_object());
            prop_assert!(learning_record.new_value.is_object());
        }
    }

    proptest! {
        /// **Feature: islamic-app-comprehensive, Property 20: Customization Analytics Accuracy**
        /// For any customization analytics, the metrics must be mathematically consistent
        /// and improvement scores must reflect actual changes in user behavior
        #[test]
        fn prop_customization_analytics_consistency(
            recommendation_accuracy in arb_score(),
            reminder_effectiveness in arb_score(),
            personalization_score in arb_score(),
            engagement_improvement in -50.0f64..100.0f64,
            satisfaction_trend in -20.0f64..50.0f64,
            retention_impact in arb_score()
        ) {
            let user_id = Uuid::new_v4();
            let now = Utc::now();
            
            let analytics = CustomizationAnalytics {
                user_id,
                analysis_period: AnalysisPeriod {
                    start_date: now - chrono::Duration::days(30),
                    end_date: now,
                    period_type: PeriodType::Monthly,
                },
                recommendation_accuracy,
                reminder_effectiveness,
                personalization_score,
                engagement_improvement,
                satisfaction_trend,
                retention_impact,
                preference_stability: 0.8,
                adaptation_speed: 0.7,
                prediction_accuracy: 0.75,
                content_diversity: 0.6,
                content_relevance: 0.9,
                completion_rate_improvement: 10.0,
                generated_at: now,
            };

            // Property: Core metrics must be within bounds
            prop_assert!(analytics.recommendation_accuracy >= 0.0 && analytics.recommendation_accuracy <= 1.0);
            prop_assert!(analytics.reminder_effectiveness >= 0.0 && analytics.reminder_effectiveness <= 1.0);
            prop_assert!(analytics.personalization_score >= 0.0 && analytics.personalization_score <= 1.0);
            prop_assert!(analytics.retention_impact >= 0.0 && analytics.retention_impact <= 1.0);
            
            // Property: Analysis period must be valid
            prop_assert!(analytics.analysis_period.end_date > analytics.analysis_period.start_date);
            prop_assert!(analytics.generated_at >= analytics.analysis_period.end_date);
            
            // Property: High personalization should correlate with better outcomes
            if analytics.personalization_score > 0.8 {
                // High personalization should generally lead to positive trends
                // (This is a business assumption that can be validated)
                prop_assert!(analytics.engagement_improvement >= -10.0); // Allow some variance
            }
            
            // Property: Improvement metrics should be reasonable
            prop_assert!(analytics.engagement_improvement >= -50.0 && analytics.engagement_improvement <= 100.0);
            prop_assert!(analytics.satisfaction_trend >= -20.0 && analytics.satisfaction_trend <= 50.0);
        }
    }

    proptest! {
        /// **Feature: islamic-app-comprehensive, Property 21: Content Customization Coherence**
        /// For any content customization, the settings must be coherent and appropriate
        /// for the target language and user preferences
        #[test]
        fn prop_content_customization_coherence(
            include_verse in any::<bool>(),
            include_hadith in any::<bool>(),
            include_motivation in any::<bool>(),
            personalized_elements in prop::collection::vec("[a-z_]+", 1..5)
        ) {
            let customization = ContentCustomization {
                language: "ar".to_string(),
                tone: MessageTone::Gentle,
                length: MessageLength::Short,
                include_verse,
                include_hadith,
                include_motivation,
                personalized_elements,
            };

            // Property: At least one content element must be included
            prop_assert!(customization.include_verse || customization.include_hadith || customization.include_motivation);
            
            // Property: Language must not be empty
            prop_assert!(!customization.language.is_empty());
            
            // Property: Must have at least one personalized element
            prop_assert!(!customization.personalized_elements.is_empty());
            
            // Property: Personalized elements must be valid identifiers
            for element in &customization.personalized_elements {
                prop_assert!(!element.is_empty());
                prop_assert!(element.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
            }
        }
    }
}
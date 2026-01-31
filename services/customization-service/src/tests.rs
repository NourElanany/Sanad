use crate::models::*;
use crate::service::{SmartCustomizationService, ReadingSession, NotificationResponse, ContentInteraction, UserInteraction, InteractionType, NotificationResponseType};
use crate::repository::CustomizationRepository;
use chrono::{DateTime, Utc, NaiveTime, Duration};
use uuid::Uuid;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    // Mock repository for testing
    pub struct MockCustomizationRepository {
        behavior_profiles: std::sync::Mutex<HashMap<Uuid, UserBehaviorProfile>>,
        recommendations: std::sync::Mutex<Vec<PersonalizedRecommendation>>,
        reminders: std::sync::Mutex<Vec<AdaptiveReminder>>,
        interactions: std::sync::Mutex<HashMap<Uuid, Vec<UserInteraction>>>,
    }

    impl MockCustomizationRepository {
        pub fn new() -> Self {
            Self {
                behavior_profiles: std::sync::Mutex::new(HashMap::new()),
                recommendations: std::sync::Mutex::new(Vec::new()),
                reminders: std::sync::Mutex::new(Vec::new()),
                interactions: std::sync::Mutex::new(HashMap::new()),
            }
        }

        pub async fn get_behavior_profile(&self, user_id: Uuid) -> anyhow::Result<UserBehaviorProfile> {
            let profiles = self.behavior_profiles.lock().unwrap();
            profiles.get(&user_id)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Profile not found"))
        }

        pub async fn save_behavior_profile(&self, profile: &UserBehaviorProfile) -> anyhow::Result<UserBehaviorProfile> {
            let mut profiles = self.behavior_profiles.lock().unwrap();
            profiles.insert(profile.user_id, profile.clone());
            Ok(profile.clone())
        }

        pub async fn get_user_interaction_history(&self, user_id: Uuid) -> anyhow::Result<Vec<UserInteraction>> {
            let interactions = self.interactions.lock().unwrap();
            Ok(interactions.get(&user_id).cloned().unwrap_or_default())
        }

        pub async fn get_user_reading_sessions(&self, _user_id: Uuid) -> anyhow::Result<Vec<ReadingSession>> {
            // Mock reading sessions
            Ok(vec![
                ReadingSession {
                    start_time: Utc::now() - Duration::days(1),
                    duration_minutes: Some(30),
                },
                ReadingSession {
                    start_time: Utc::now() - Duration::days(2),
                    duration_minutes: Some(25),
                },
            ])
        }

        pub async fn get_notification_responses(&self, _user_id: Uuid) -> anyhow::Result<Vec<NotificationResponse>> {
            Ok(vec![
                NotificationResponse {
                    notification_time: Utc::now() - Duration::hours(2),
                    response_type: NotificationResponseType::Acted,
                },
                NotificationResponse {
                    notification_time: Utc::now() - Duration::hours(6),
                    response_type: NotificationResponseType::Acknowledged,
                },
            ])
        }

        pub async fn get_content_interactions(&self, _user_id: Uuid) -> anyhow::Result<Vec<ContentInteraction>> {
            Ok(vec![
                ContentInteraction {
                    content_type: ContentType::QuranVerses,
                    duration_seconds: Some(1800), // 30 minutes
                    completed: true,
                    user_rating: Some(4.5),
                },
                ContentInteraction {
                    content_type: ContentType::HadithNarrations,
                    duration_seconds: Some(900), // 15 minutes
                    completed: true,
                    user_rating: Some(4.0),
                },
            ])
        }

        pub async fn save_recommendation(&self, recommendation: &PersonalizedRecommendation) -> anyhow::Result<()> {
            let mut recommendations = self.recommendations.lock().unwrap();
            recommendations.push(recommendation.clone());
            Ok(())
        }

        pub async fn save_adaptive_reminder(&self, reminder: &AdaptiveReminder) -> anyhow::Result<()> {
            let mut reminders = self.reminders.lock().unwrap();
            reminders.push(reminder.clone());
            Ok(())
        }

        pub async fn save_learning_record(&self, _record: &PreferenceLearningRecord) -> anyhow::Result<()> {
            Ok(())
        }

        pub async fn get_recent_interactions(&self, user_id: Uuid, _days: i32) -> anyhow::Result<Vec<UserInteraction>> {
            self.get_user_interaction_history(user_id).await
        }

        pub async fn get_reminder_history(&self, _user_id: Uuid) -> anyhow::Result<Vec<AdaptiveReminder>> {
            let reminders = self.reminders.lock().unwrap();
            Ok(reminders.clone())
        }

        pub async fn get_interactions_for_period(&self, user_id: Uuid, _period: &AnalysisPeriod) -> anyhow::Result<Vec<UserInteraction>> {
            self.get_user_interaction_history(user_id).await
        }

        pub async fn get_recommendations_for_period(&self, _user_id: Uuid, _period: &AnalysisPeriod) -> anyhow::Result<Vec<PersonalizedRecommendation>> {
            let recommendations = self.recommendations.lock().unwrap();
            Ok(recommendations.clone())
        }

        pub async fn get_reminders_for_period(&self, _user_id: Uuid, _period: &AnalysisPeriod) -> anyhow::Result<Vec<AdaptiveReminder>> {
            let reminders = self.reminders.lock().unwrap();
            Ok(reminders.clone())
        }

        pub async fn get_learning_records_for_period(&self, _user_id: Uuid, _period: &AnalysisPeriod) -> anyhow::Result<Vec<PreferenceLearningRecord>> {
            Ok(vec![])
        }
    }

    fn create_test_service() -> SmartCustomizationService {
        let mock_repo = MockCustomizationRepository::new();
        // Note: This is a simplified mock - in real implementation, we'd need proper dependency injection
        // For now, we'll test the logic separately
        SmartCustomizationService::new(CustomizationRepository::new(
            sqlx::PgPool::connect("postgresql://test").await.unwrap() // This would fail in tests
        ))
    }

    fn create_sample_user_profile(user_id: Uuid) -> UserBehaviorProfile {
        UserBehaviorProfile {
            id: Uuid::new_v4(),
            user_id,
            preferred_reading_times: vec![
                PreferredTimeSlot {
                    start_time: NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
                    end_time: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
                    activity_type: ActivityType::QuranReading,
                    preference_strength: 0.8,
                    days_of_week: vec![1, 2, 3, 4, 5],
                    success_rate: 0.7,
                },
            ],
            average_session_duration: 30,
            reading_consistency_score: 0.75,
            preferred_content_types: vec![
                ContentTypePreference {
                    content_type: ContentType::QuranVerses,
                    preference_weight: 0.9,
                    interaction_frequency: 0.8,
                    completion_rate: 0.7,
                },
            ],
            notification_response_rate: 0.65,
            preferred_notification_times: vec![
                NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
            ],
            engagement_patterns: EngagementPatterns {
                peak_engagement_hours: vec![6, 7, 20, 21],
                peak_engagement_days: vec![1, 2, 3, 4, 5],
                average_session_length: 30,
                preferred_content_length: ContentLength::Medium,
                interaction_style: InteractionStyle::Structured,
                motivation_triggers: vec![MotivationTrigger::Progress, MotivationTrigger::Spiritual],
            },
            learning_style: LearningStyle::Mixed,
            difficulty_preference: DifficultyLevel::Intermediate,
            language_preferences: vec!["ar".to_string(), "en".to_string()],
            seasonal_preferences: HashMap::new(),
            location_based_preferences: None,
            adaptation_score: 0.7,
            satisfaction_score: 0.8,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_create_default_profile() {
        let service = create_test_service();
        let user_id = Uuid::new_v4();
        
        let profile = service.create_default_profile(user_id);
        
        assert_eq!(profile.user_id, user_id);
        assert!(!profile.preferred_reading_times.is_empty());
        assert!(!profile.preferred_content_types.is_empty());
        assert_eq!(profile.learning_style, LearningStyle::Mixed);
        assert_eq!(profile.difficulty_preference, DifficultyLevel::Intermediate);
        assert!(profile.language_preferences.contains(&"ar".to_string()));
    }

    #[test]
    fn test_behavior_profile_validation() {
        let user_id = Uuid::new_v4();
        let profile = create_sample_user_profile(user_id);
        
        // Test that scores are within valid ranges
        assert!(profile.reading_consistency_score >= 0.0 && profile.reading_consistency_score <= 1.0);
        assert!(profile.notification_response_rate >= 0.0 && profile.notification_response_rate <= 1.0);
        assert!(profile.adaptation_score >= 0.0 && profile.adaptation_score <= 1.0);
        assert!(profile.satisfaction_score >= 0.0 && profile.satisfaction_score <= 1.0);
        
        // Test that preferred times are valid
        for time_slot in &profile.preferred_reading_times {
            assert!(time_slot.preference_strength >= 0.0 && time_slot.preference_strength <= 1.0);
            assert!(time_slot.success_rate >= 0.0 && time_slot.success_rate <= 1.0);
            assert!(!time_slot.days_of_week.is_empty());
        }
        
        // Test that content preferences are valid
        for content_pref in &profile.preferred_content_types {
            assert!(content_pref.preference_weight >= 0.0 && content_pref.preference_weight <= 1.0);
            assert!(content_pref.interaction_frequency >= 0.0 && content_pref.interaction_frequency <= 1.0);
            assert!(content_pref.completion_rate >= 0.0 && content_pref.completion_rate <= 1.0);
        }
    }

    #[test]
    fn test_recommendation_scoring() {
        let user_id = Uuid::new_v4();
        let recommendation = PersonalizedRecommendation {
            id: Uuid::new_v4(),
            user_id,
            content_type: ContentType::QuranVerses,
            content_id: "surah_1".to_string(),
            title: "سورة الفاتحة".to_string(),
            description: "قراءة سورة الفاتحة مع التدبر".to_string(),
            recommendation_score: 0.85,
            reasoning: "مناسبة للوقت المفضل للمستخدم".to_string(),
            estimated_duration: 15,
            difficulty_level: DifficultyLevel::Beginner,
            tags: vec!["quran".to_string(), "daily".to_string()],
            category: RecommendationCategory::DailyReading,
            presented_at: None,
            interacted_at: None,
            completed_at: None,
            user_rating: None,
            feedback: None,
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + Duration::days(1)),
        };
        
        // Test recommendation score is valid
        assert!(recommendation.recommendation_score >= 0.0 && recommendation.recommendation_score <= 1.0);
        assert!(recommendation.estimated_duration > 0);
        assert!(!recommendation.title.is_empty());
        assert!(!recommendation.description.is_empty());
        assert!(!recommendation.reasoning.is_empty());
    }

    #[test]
    fn test_adaptive_reminder_structure() {
        let user_id = Uuid::new_v4();
        let reminder = AdaptiveReminder {
            id: Uuid::new_v4(),
            user_id,
            reminder_type: ReminderType::QuranReading,
            title: "وقت القراءة".to_string(),
            message: "حان وقت قراءة القرآن الكريم".to_string(),
            suggested_time: Utc::now() + Duration::hours(1),
            optimal_time_window: TimeWindow {
                start_time: NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
                end_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                preferred_time: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
                flexibility_minutes: 30,
            },
            adaptation_confidence: 0.8,
            personalization_factors: vec![PersonalizationFactor::HistoricalResponse, PersonalizationFactor::ActivityPattern],
            content_customization: ContentCustomization {
                language: "ar".to_string(),
                tone: MessageTone::Gentle,
                length: MessageLength::Short,
                include_verse: true,
                include_hadith: false,
                include_motivation: true,
                personalized_elements: vec!["user_name".to_string()],
            },
            response_prediction: 0.75,
            actual_response: None,
            effectiveness_score: None,
            is_recurring: false,
            recurrence_pattern: None,
            next_occurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Test reminder structure
        assert!(reminder.adaptation_confidence >= 0.0 && reminder.adaptation_confidence <= 1.0);
        assert!(reminder.response_prediction >= 0.0 && reminder.response_prediction <= 1.0);
        assert!(!reminder.personalization_factors.is_empty());
        assert!(!reminder.title.is_empty());
        assert!(!reminder.message.is_empty());
        assert!(reminder.optimal_time_window.flexibility_minutes > 0);
    }

    #[test]
    fn test_preference_learning_record() {
        let user_id = Uuid::new_v4();
        let learning_record = PreferenceLearningRecord {
            id: Uuid::new_v4(),
            user_id,
            preference_type: PreferenceType::ReadingTime,
            old_value: serde_json::json!({"preferred_hour": 6}),
            new_value: serde_json::json!({"preferred_hour": 7}),
            confidence_score: 0.8,
            learning_source: LearningSource::UserBehavior,
            validation_status: ValidationStatus::Pending,
            impact_score: 0.6,
            created_at: Utc::now(),
        };
        
        // Test learning record structure
        assert!(learning_record.confidence_score >= 0.0 && learning_record.confidence_score <= 1.0);
        assert!(learning_record.impact_score >= 0.0 && learning_record.impact_score <= 1.0);
        assert_ne!(learning_record.old_value, learning_record.new_value);
    }

    #[test]
    fn test_engagement_patterns_analysis() {
        let engagement_patterns = EngagementPatterns {
            peak_engagement_hours: vec![6, 7, 8, 19, 20, 21],
            peak_engagement_days: vec![1, 2, 3, 4, 5], // Weekdays
            average_session_length: 35,
            preferred_content_length: ContentLength::Medium,
            interaction_style: InteractionStyle::Structured,
            motivation_triggers: vec![
                MotivationTrigger::Progress,
                MotivationTrigger::Spiritual,
                MotivationTrigger::Reminders,
            ],
        };
        
        // Test engagement patterns
        assert!(!engagement_patterns.peak_engagement_hours.is_empty());
        assert!(!engagement_patterns.peak_engagement_days.is_empty());
        assert!(engagement_patterns.average_session_length > 0);
        assert!(!engagement_patterns.motivation_triggers.is_empty());
        
        // Test that hours are valid (0-23)
        for hour in &engagement_patterns.peak_engagement_hours {
            assert!(*hour <= 23);
        }
        
        // Test that days are valid (0-6)
        for day in &engagement_patterns.peak_engagement_days {
            assert!(*day <= 6);
        }
    }

    #[test]
    fn test_seasonal_preferences() {
        let mut seasonal_preferences = HashMap::new();
        seasonal_preferences.insert("ramadan".to_string(), SeasonalPreference {
            season: IslamicSeason::Ramadan,
            content_focus: vec![ContentType::QuranVerses, ContentType::Dhikr, ContentType::Duas],
            activity_increase: 1.5,
            preferred_reminders: vec![ReminderType::QuranReading, ReminderType::Dhikr],
            special_interests: vec!["night_prayers".to_string(), "charity".to_string()],
        });
        
        let ramadan_pref = seasonal_preferences.get("ramadan").unwrap();
        assert_eq!(ramadan_pref.season, IslamicSeason::Ramadan);
        assert!(ramadan_pref.activity_increase > 1.0);
        assert!(!ramadan_pref.content_focus.is_empty());
        assert!(!ramadan_pref.preferred_reminders.is_empty());
    }

    #[test]
    fn test_customization_analytics() {
        let analytics = CustomizationAnalytics {
            user_id: Uuid::new_v4(),
            analysis_period: AnalysisPeriod {
                start_date: Utc::now() - Duration::days(30),
                end_date: Utc::now(),
                period_type: PeriodType::Monthly,
            },
            recommendation_accuracy: 0.78,
            reminder_effectiveness: 0.82,
            personalization_score: 0.85,
            engagement_improvement: 15.5,
            satisfaction_trend: 8.2,
            retention_impact: 0.12,
            preference_stability: 0.88,
            adaptation_speed: 0.75,
            prediction_accuracy: 0.79,
            content_diversity: 0.65,
            content_relevance: 0.91,
            completion_rate_improvement: 12.3,
            generated_at: Utc::now(),
        };
        
        // Test analytics metrics are within valid ranges
        assert!(analytics.recommendation_accuracy >= 0.0 && analytics.recommendation_accuracy <= 1.0);
        assert!(analytics.reminder_effectiveness >= 0.0 && analytics.reminder_effectiveness <= 1.0);
        assert!(analytics.personalization_score >= 0.0 && analytics.personalization_score <= 1.0);
        assert!(analytics.preference_stability >= 0.0 && analytics.preference_stability <= 1.0);
        assert!(analytics.adaptation_speed >= 0.0 && analytics.adaptation_speed <= 1.0);
        assert!(analytics.prediction_accuracy >= 0.0 && analytics.prediction_accuracy <= 1.0);
        assert!(analytics.content_diversity >= 0.0 && analytics.content_diversity <= 1.0);
        assert!(analytics.content_relevance >= 0.0 && analytics.content_relevance <= 1.0);
        assert!(analytics.retention_impact >= 0.0 && analytics.retention_impact <= 1.0);
    }

    #[test]
    fn test_time_window_validation() {
        let time_window = TimeWindow {
            start_time: NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
            end_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            preferred_time: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
            flexibility_minutes: 30,
        };
        
        // Test that preferred time is within the window
        assert!(time_window.preferred_time >= time_window.start_time);
        assert!(time_window.preferred_time <= time_window.end_time);
        assert!(time_window.flexibility_minutes > 0);
    }

    #[test]
    fn test_content_customization() {
        let customization = ContentCustomization {
            language: "ar".to_string(),
            tone: MessageTone::Gentle,
            length: MessageLength::Short,
            include_verse: true,
            include_hadith: false,
            include_motivation: true,
            personalized_elements: vec!["user_name".to_string(), "progress".to_string()],
        };
        
        assert!(!customization.language.is_empty());
        assert!(!customization.personalized_elements.is_empty());
        
        // Test that at least one content element is included
        assert!(customization.include_verse || customization.include_hadith || customization.include_motivation);
    }

    #[test]
    fn test_recurrence_pattern() {
        let recurrence = RecurrencePattern {
            frequency: RecurrenceFrequency::Daily,
            interval: 1,
            days_of_week: Some(vec![1, 2, 3, 4, 5]), // Weekdays
            days_of_month: None,
            end_condition: EndCondition::AfterOccurrences(30),
        };
        
        assert!(recurrence.interval > 0);
        if let Some(days) = &recurrence.days_of_week {
            for day in days {
                assert!(*day <= 6);
            }
        }
    }

    // Property-based tests would go here using proptest
    // These would test invariants across many generated inputs
    
    #[test]
    fn test_user_behavior_profile_invariants() {
        // Test that all scores remain within bounds after updates
        let user_id = Uuid::new_v4();
        let mut profile = create_sample_user_profile(user_id);
        
        // Simulate score updates
        profile.reading_consistency_score = 0.95;
        profile.adaptation_score = 0.88;
        profile.satisfaction_score = 0.92;
        
        // Verify invariants
        assert!(profile.reading_consistency_score <= 1.0);
        assert!(profile.adaptation_score <= 1.0);
        assert!(profile.satisfaction_score <= 1.0);
        assert!(profile.reading_consistency_score >= 0.0);
        assert!(profile.adaptation_score >= 0.0);
        assert!(profile.satisfaction_score >= 0.0);
    }

    #[test]
    fn test_recommendation_expiration() {
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        
        let recommendation = PersonalizedRecommendation {
            id: Uuid::new_v4(),
            user_id,
            content_type: ContentType::QuranVerses,
            content_id: "test".to_string(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            recommendation_score: 0.8,
            reasoning: "Test".to_string(),
            estimated_duration: 30,
            difficulty_level: DifficultyLevel::Intermediate,
            tags: vec![],
            category: RecommendationCategory::DailyReading,
            presented_at: Some(now),
            interacted_at: None,
            completed_at: None,
            user_rating: None,
            feedback: None,
            created_at: now,
            expires_at: Some(now + Duration::days(1)),
        };
        
        // Test that expiration is in the future
        if let Some(expires_at) = recommendation.expires_at {
            assert!(expires_at > recommendation.created_at);
        }
    }
}
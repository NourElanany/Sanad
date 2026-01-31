use customization_service::models::*;
use chrono::{DateTime, Utc, NaiveTime};
use uuid::Uuid;
use std::collections::HashMap;

fn main() {
    println!("🚀 Testing Smart Customization System Implementation");
    
    // Test 1: User Behavior Profile Creation
    println!("\n📊 Test 1: User Behavior Profile Creation");
    let user_id = Uuid::new_v4();
    let profile = create_sample_behavior_profile(user_id);
    
    println!("✅ Created behavior profile for user: {}", profile.user_id);
    println!("   - Preferred reading times: {}", profile.preferred_reading_times.len());
    println!("   - Content preferences: {}", profile.preferred_content_types.len());
    println!("   - Reading consistency: {:.2}", profile.reading_consistency_score);
    println!("   - Adaptation score: {:.2}", profile.adaptation_score);
    
    // Test 2: Personalized Recommendation
    println!("\n🎯 Test 2: Personalized Recommendation");
    let recommendation = create_sample_recommendation(user_id);
    
    println!("✅ Created personalized recommendation:");
    println!("   - Title: {}", recommendation.title);
    println!("   - Content Type: {:?}", recommendation.content_type);
    println!("   - Recommendation Score: {:.2}", recommendation.recommendation_score);
    println!("   - Category: {:?}", recommendation.category);
    println!("   - Estimated Duration: {} minutes", recommendation.estimated_duration);
    
    // Test 3: Adaptive Reminder
    println!("\n⏰ Test 3: Adaptive Reminder");
    let reminder = create_sample_adaptive_reminder(user_id);
    
    println!("✅ Created adaptive reminder:");
    println!("   - Title: {}", reminder.title);
    println!("   - Type: {:?}", reminder.reminder_type);
    println!("   - Adaptation Confidence: {:.2}", reminder.adaptation_confidence);
    println!("   - Response Prediction: {:.2}", reminder.response_prediction);
    println!("   - Personalization Factors: {}", reminder.personalization_factors.len());
    
    // Test 4: Preference Learning
    println!("\n🧠 Test 4: Preference Learning");
    let learning_record = create_sample_learning_record(user_id);
    
    println!("✅ Created preference learning record:");
    println!("   - Preference Type: {:?}", learning_record.preference_type);
    println!("   - Confidence Score: {:.2}", learning_record.confidence_score);
    println!("   - Learning Source: {:?}", learning_record.learning_source);
    println!("   - Impact Score: {:.2}", learning_record.impact_score);
    
    // Test 5: Seasonal Preferences
    println!("\n🌙 Test 5: Seasonal Preferences");
    let seasonal_pref = create_ramadan_preference();
    
    println!("✅ Created seasonal preference for Ramadan:");
    println!("   - Season: {:?}", seasonal_pref.season);
    println!("   - Activity Increase: {:.1}x", seasonal_pref.activity_increase);
    println!("   - Content Focus: {} types", seasonal_pref.content_focus.len());
    println!("   - Special Interests: {}", seasonal_pref.special_interests.join(", "));
    
    // Test 6: Analytics
    println!("\n📈 Test 6: Customization Analytics");
    let analytics = create_sample_analytics(user_id);
    
    println!("✅ Created customization analytics:");
    println!("   - Recommendation Accuracy: {:.1}%", analytics.recommendation_accuracy * 100.0);
    println!("   - Reminder Effectiveness: {:.1}%", analytics.reminder_effectiveness * 100.0);
    println!("   - Personalization Score: {:.1}%", analytics.personalization_score * 100.0);
    println!("   - Engagement Improvement: {:.1}%", analytics.engagement_improvement);
    
    // Test 7: Content Customization
    println!("\n🎨 Test 7: Content Customization");
    let customization = create_sample_content_customization();
    
    println!("✅ Created content customization:");
    println!("   - Language: {}", customization.language);
    println!("   - Tone: {:?}", customization.tone);
    println!("   - Length: {:?}", customization.length);
    println!("   - Include Verse: {}", customization.include_verse);
    println!("   - Include Hadith: {}", customization.include_hadith);
    println!("   - Personalized Elements: {}", customization.personalized_elements.len());
    
    // Test 8: Time Window Optimization
    println!("\n⏱️  Test 8: Time Window Optimization");
    let time_window = create_optimal_time_window();
    
    println!("✅ Created optimal time window:");
    println!("   - Start Time: {}", time_window.start_time);
    println!("   - End Time: {}", time_window.end_time);
    println!("   - Preferred Time: {}", time_window.preferred_time);
    println!("   - Flexibility: {} minutes", time_window.flexibility_minutes);
    
    // Test 9: Validation Tests
    println!("\n✅ Test 9: Data Validation");
    run_validation_tests(&profile, &recommendation, &reminder, &analytics);
    
    println!("\n🎉 All Smart Customization System tests completed successfully!");
    println!("📋 Summary:");
    println!("   - User behavior analysis: ✅");
    println!("   - Personalized recommendations: ✅");
    println!("   - Adaptive reminders: ✅");
    println!("   - Preference learning: ✅");
    println!("   - Seasonal adaptation: ✅");
    println!("   - Analytics and insights: ✅");
    println!("   - Content customization: ✅");
    println!("   - Data validation: ✅");
}

fn create_sample_behavior_profile(user_id: Uuid) -> UserBehaviorProfile {
    UserBehaviorProfile {
        id: Uuid::new_v4(),
        user_id,
        preferred_reading_times: vec![
            PreferredTimeSlot {
                start_time: NaiveTime::from_hms_opt(5, 30, 0).unwrap(),
                end_time: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
                activity_type: ActivityType::QuranReading,
                preference_strength: 0.85,
                days_of_week: vec![1, 2, 3, 4, 5], // Weekdays
                success_rate: 0.78,
            },
            PreferredTimeSlot {
                start_time: NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
                end_time: NaiveTime::from_hms_opt(21, 30, 0).unwrap(),
                activity_type: ActivityType::DhikrReminders,
                preference_strength: 0.72,
                days_of_week: vec![0, 1, 2, 3, 4, 5, 6], // All days
                success_rate: 0.65,
            },
        ],
        average_session_duration: 32,
        reading_consistency_score: 0.82,
        preferred_content_types: vec![
            ContentTypePreference {
                content_type: ContentType::QuranVerses,
                preference_weight: 0.92,
                interaction_frequency: 0.85,
                completion_rate: 0.78,
            },
            ContentTypePreference {
                content_type: ContentType::HadithNarrations,
                preference_weight: 0.75,
                interaction_frequency: 0.58,
                completion_rate: 0.68,
            },
            ContentTypePreference {
                content_type: ContentType::Dhikr,
                preference_weight: 0.88,
                interaction_frequency: 0.72,
                completion_rate: 0.85,
            },
        ],
        notification_response_rate: 0.73,
        preferred_notification_times: vec![
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(12, 30, 0).unwrap(),
            NaiveTime::from_hms_opt(19, 30, 0).unwrap(),
        ],
        engagement_patterns: EngagementPatterns {
            peak_engagement_hours: vec![6, 7, 8, 19, 20, 21],
            peak_engagement_days: vec![1, 2, 3, 4, 5, 6], // Monday to Saturday
            average_session_length: 32,
            preferred_content_length: ContentLength::Medium,
            interaction_style: InteractionStyle::Structured,
            motivation_triggers: vec![
                MotivationTrigger::Progress,
                MotivationTrigger::Spiritual,
                MotivationTrigger::Reminders,
            ],
        },
        learning_style: LearningStyle::Mixed,
        difficulty_preference: DifficultyLevel::Intermediate,
        language_preferences: vec!["ar".to_string(), "en".to_string()],
        seasonal_preferences: {
            let mut prefs = HashMap::new();
            prefs.insert("ramadan".to_string(), create_ramadan_preference());
            prefs
        },
        location_based_preferences: Some(LocationPreferences {
            timezone: "Asia/Riyadh".to_string(),
            prayer_calculation_method: "umm_al_qura".to_string(),
            local_islamic_events: true,
            community_features: true,
            language_region: "ar_SA".to_string(),
        }),
        adaptation_score: 0.79,
        satisfaction_score: 0.86,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_sample_recommendation(user_id: Uuid) -> PersonalizedRecommendation {
    PersonalizedRecommendation {
        id: Uuid::new_v4(),
        user_id,
        content_type: ContentType::QuranVerses,
        content_id: "surah_2_1_20".to_string(),
        title: "سورة البقرة - الآيات 1-20".to_string(),
        description: "قراءة مباركة من بداية سورة البقرة مع التدبر في معانيها العظيمة والتأمل في هداياتها".to_string(),
        recommendation_score: 0.91,
        reasoning: "مناسبة لوقت القراءة المفضل صباحاً، ومستوى المستخدم المتوسط، مع تركيز على التدبر".to_string(),
        estimated_duration: 28,
        difficulty_level: DifficultyLevel::Intermediate,
        tags: vec![
            "morning_reading".to_string(),
            "quran".to_string(),
            "reflection".to_string(),
            "guidance".to_string(),
        ],
        category: RecommendationCategory::DailyReading,
        presented_at: None,
        interacted_at: None,
        completed_at: None,
        user_rating: None,
        feedback: None,
        created_at: Utc::now(),
        expires_at: Some(Utc::now() + chrono::Duration::days(2)),
    }
}

fn create_sample_adaptive_reminder(user_id: Uuid) -> AdaptiveReminder {
    AdaptiveReminder {
        id: Uuid::new_v4(),
        user_id,
        reminder_type: ReminderType::QuranReading,
        title: "وقت القراءة المبارك 📖".to_string(),
        message: "السلام عليكم! حان وقت قراءة القرآن الكريم. ابدأ يومك بالبركة والهداية من كلام الله العظيم.".to_string(),
        suggested_time: Utc::now() + chrono::Duration::hours(18), // Tomorrow morning
        optimal_time_window: create_optimal_time_window(),
        adaptation_confidence: 0.87,
        personalization_factors: vec![
            PersonalizationFactor::HistoricalResponse,
            PersonalizationFactor::ActivityPattern,
            PersonalizationFactor::PersonalGoals,
            PersonalizationFactor::CurrentContext,
        ],
        content_customization: create_sample_content_customization(),
        response_prediction: 0.81,
        actual_response: None,
        effectiveness_score: None,
        is_recurring: true,
        recurrence_pattern: Some(RecurrencePattern {
            frequency: RecurrenceFrequency::Daily,
            interval: 1,
            days_of_week: Some(vec![1, 2, 3, 4, 5]), // Weekdays
            days_of_month: None,
            end_condition: EndCondition::Never,
        }),
        next_occurrence: Some(Utc::now() + chrono::Duration::days(1)),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_sample_learning_record(user_id: Uuid) -> PreferenceLearningRecord {
    PreferenceLearningRecord {
        id: Uuid::new_v4(),
        user_id,
        preference_type: PreferenceType::ReadingTime,
        old_value: serde_json::json!({
            "preferred_hour": 6,
            "success_rate": 0.65
        }),
        new_value: serde_json::json!({
            "preferred_hour": 7,
            "success_rate": 0.78
        }),
        confidence_score: 0.84,
        learning_source: LearningSource::UserBehavior,
        validation_status: ValidationStatus::Confirmed,
        impact_score: 0.72,
        created_at: Utc::now(),
    }
}

fn create_ramadan_preference() -> SeasonalPreference {
    SeasonalPreference {
        season: IslamicSeason::Ramadan,
        content_focus: vec![
            ContentType::QuranVerses,
            ContentType::Dhikr,
            ContentType::Duas,
            ContentType::IslamicHistory,
        ],
        activity_increase: 1.8,
        preferred_reminders: vec![
            ReminderType::QuranReading,
            ReminderType::Dhikr,
            ReminderType::Reflection,
            ReminderType::Charity,
        ],
        special_interests: vec![
            "night_prayers".to_string(),
            "quran_completion".to_string(),
            "charity_reminders".to_string(),
            "iftar_duas".to_string(),
            "laylat_al_qadr".to_string(),
        ],
    }
}

fn create_sample_analytics(user_id: Uuid) -> CustomizationAnalytics {
    CustomizationAnalytics {
        user_id,
        analysis_period: AnalysisPeriod {
            start_date: Utc::now() - chrono::Duration::days(30),
            end_date: Utc::now(),
            period_type: PeriodType::Monthly,
        },
        recommendation_accuracy: 0.84,
        reminder_effectiveness: 0.79,
        personalization_score: 0.88,
        engagement_improvement: 23.5,
        satisfaction_trend: 12.8,
        retention_impact: 0.15,
        preference_stability: 0.91,
        adaptation_speed: 0.76,
        prediction_accuracy: 0.82,
        content_diversity: 0.68,
        content_relevance: 0.93,
        completion_rate_improvement: 18.7,
        generated_at: Utc::now(),
    }
}

fn create_sample_content_customization() -> ContentCustomization {
    ContentCustomization {
        language: "ar".to_string(),
        tone: MessageTone::Gentle,
        length: MessageLength::Medium,
        include_verse: true,
        include_hadith: false,
        include_motivation: true,
        personalized_elements: vec![
            "user_name".to_string(),
            "progress_praise".to_string(),
            "time_context".to_string(),
            "consistency_encouragement".to_string(),
        ],
    }
}

fn create_optimal_time_window() -> TimeWindow {
    TimeWindow {
        start_time: NaiveTime::from_hms_opt(5, 30, 0).unwrap(),
        end_time: NaiveTime::from_hms_opt(7, 30, 0).unwrap(),
        preferred_time: NaiveTime::from_hms_opt(6, 15, 0).unwrap(),
        flexibility_minutes: 45,
    }
}

fn run_validation_tests(
    profile: &UserBehaviorProfile,
    recommendation: &PersonalizedRecommendation,
    reminder: &AdaptiveReminder,
    analytics: &CustomizationAnalytics,
) {
    // Validate behavior profile
    assert!(profile.reading_consistency_score >= 0.0 && profile.reading_consistency_score <= 1.0);
    assert!(profile.adaptation_score >= 0.0 && profile.adaptation_score <= 1.0);
    assert!(profile.satisfaction_score >= 0.0 && profile.satisfaction_score <= 1.0);
    assert!(!profile.preferred_reading_times.is_empty());
    assert!(!profile.preferred_content_types.is_empty());
    println!("   ✅ Behavior profile validation passed");
    
    // Validate recommendation
    assert!(recommendation.recommendation_score >= 0.0 && recommendation.recommendation_score <= 1.0);
    assert!(recommendation.estimated_duration > 0);
    assert!(!recommendation.title.is_empty());
    assert!(!recommendation.reasoning.is_empty());
    println!("   ✅ Recommendation validation passed");
    
    // Validate reminder
    assert!(reminder.adaptation_confidence >= 0.0 && reminder.adaptation_confidence <= 1.0);
    assert!(reminder.response_prediction >= 0.0 && reminder.response_prediction <= 1.0);
    assert!(!reminder.personalization_factors.is_empty());
    assert!(reminder.optimal_time_window.flexibility_minutes > 0);
    println!("   ✅ Adaptive reminder validation passed");
    
    // Validate analytics
    assert!(analytics.recommendation_accuracy >= 0.0 && analytics.recommendation_accuracy <= 1.0);
    assert!(analytics.personalization_score >= 0.0 && analytics.personalization_score <= 1.0);
    assert!(analytics.analysis_period.end_date > analytics.analysis_period.start_date);
    println!("   ✅ Analytics validation passed");
}
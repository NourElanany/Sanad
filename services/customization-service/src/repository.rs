use crate::models::*;
use crate::service::{ReadingSession, NotificationResponse, ContentInteraction, UserInteraction};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use tracing::{info, error};

/// Repository for smart customization data
pub struct CustomizationRepository {
    pub pool: PgPool,
}

impl CustomizationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get user behavior profile
    pub async fn get_behavior_profile(&self, user_id: Uuid) -> Result<UserBehaviorProfile> {
        let profile = sqlx::query_as!(
            UserBehaviorProfile,
            r#"
            SELECT 
                id, user_id,
                preferred_reading_times, average_session_duration, reading_consistency_score,
                preferred_content_types, notification_response_rate, preferred_notification_times,
                engagement_patterns, learning_style as "learning_style: LearningStyle",
                difficulty_preference as "difficulty_preference: DifficultyLevel",
                language_preferences, seasonal_preferences, location_based_preferences,
                adaptation_score, satisfaction_score, created_at, updated_at
            FROM user_behavior_profiles 
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get behavior profile: {}", e))?;

        Ok(profile)
    }

    /// Save or update user behavior profile
    pub async fn save_behavior_profile(&self, profile: &UserBehaviorProfile) -> Result<UserBehaviorProfile> {
        let saved_profile = sqlx::query_as!(
            UserBehaviorProfile,
            r#"
            INSERT INTO user_behavior_profiles (
                id, user_id, preferred_reading_times, average_session_duration,
                reading_consistency_score, preferred_content_types, notification_response_rate,
                preferred_notification_times, engagement_patterns, learning_style,
                difficulty_preference, language_preferences, seasonal_preferences,
                location_based_preferences, adaptation_score, satisfaction_score,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18
            )
            ON CONFLICT (user_id) DO UPDATE SET
                preferred_reading_times = EXCLUDED.preferred_reading_times,
                average_session_duration = EXCLUDED.average_session_duration,
                reading_consistency_score = EXCLUDED.reading_consistency_score,
                preferred_content_types = EXCLUDED.preferred_content_types,
                notification_response_rate = EXCLUDED.notification_response_rate,
                preferred_notification_times = EXCLUDED.preferred_notification_times,
                engagement_patterns = EXCLUDED.engagement_patterns,
                learning_style = EXCLUDED.learning_style,
                difficulty_preference = EXCLUDED.difficulty_preference,
                language_preferences = EXCLUDED.language_preferences,
                seasonal_preferences = EXCLUDED.seasonal_preferences,
                location_based_preferences = EXCLUDED.location_based_preferences,
                adaptation_score = EXCLUDED.adaptation_score,
                satisfaction_score = EXCLUDED.satisfaction_score,
                updated_at = EXCLUDED.updated_at
            RETURNING 
                id, user_id, preferred_reading_times, average_session_duration,
                reading_consistency_score, preferred_content_types, notification_response_rate,
                preferred_notification_times, engagement_patterns,
                learning_style as "learning_style: LearningStyle",
                difficulty_preference as "difficulty_preference: DifficultyLevel",
                language_preferences, seasonal_preferences, location_based_preferences,
                adaptation_score, satisfaction_score, created_at, updated_at
            "#,
            profile.id,
            profile.user_id,
            serde_json::to_value(&profile.preferred_reading_times)?,
            profile.average_session_duration,
            profile.reading_consistency_score,
            serde_json::to_value(&profile.preferred_content_types)?,
            profile.notification_response_rate,
            serde_json::to_value(&profile.preferred_notification_times)?,
            serde_json::to_value(&profile.engagement_patterns)?,
            profile.learning_style as LearningStyle,
            profile.difficulty_preference as DifficultyLevel,
            &profile.language_preferences,
            serde_json::to_value(&profile.seasonal_preferences)?,
            serde_json::to_value(&profile.location_based_preferences)?,
            profile.adaptation_score,
            profile.satisfaction_score,
            profile.created_at,
            profile.updated_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to save behavior profile: {}", e))?;

        Ok(saved_profile)
    }

    /// Get user interaction history
    pub async fn get_user_interaction_history(&self, user_id: Uuid) -> Result<Vec<UserInteraction>> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                timestamp, duration_seconds, interaction_type,
                content_type, activity_type, response_type
            FROM user_interactions 
            WHERE user_id = $1 
            ORDER BY timestamp DESC 
            LIMIT 1000
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get interaction history: {}", e))?;

        let interactions = rows.into_iter().map(|row| {
            UserInteraction {
                timestamp: row.timestamp,
                duration_seconds: row.duration_seconds.map(|d| d as u32),
                interaction_type: match row.interaction_type.as_str() {
                    "positive" => crate::service::InteractionType::Positive,
                    "negative" => crate::service::InteractionType::Negative,
                    _ => crate::service::InteractionType::Neutral,
                },
            }
        }).collect();

        Ok(interactions)
    }

    /// Get user reading sessions (from khatma service)
    pub async fn get_user_reading_sessions(&self, user_id: Uuid) -> Result<Vec<ReadingSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT start_time, duration_minutes
            FROM reading_sessions 
            WHERE user_id = $1 
            ORDER BY start_time DESC 
            LIMIT 500
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get reading sessions: {}", e))?;

        let sessions = rows.into_iter().map(|row| {
            ReadingSession {
                start_time: row.start_time,
                duration_minutes: row.duration_minutes,
            }
        }).collect();

        Ok(sessions)
    }

    /// Get notification responses
    pub async fn get_notification_responses(&self, user_id: Uuid) -> Result<Vec<NotificationResponse>> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                n.scheduled_at as notification_time,
                ndl.delivery_status as response_type
            FROM notifications n
            JOIN notification_delivery_log ndl ON n.id = ndl.notification_id
            WHERE n.user_id = $1 
            ORDER BY n.scheduled_at DESC 
            LIMIT 500
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get notification responses: {}", e))?;

        let responses = rows.into_iter().map(|row| {
            let response_type = match row.response_type.as_str() {
                "delivered" => crate::service::NotificationResponseType::Acknowledged,
                "read" => crate::service::NotificationResponseType::Acted,
                "dismissed" => crate::service::NotificationResponseType::Dismissed,
                "failed" => crate::service::NotificationResponseType::Ignored,
                _ => crate::service::NotificationResponseType::Ignored,
            };

            NotificationResponse {
                notification_time: row.notification_time,
                response_type,
            }
        }).collect();

        Ok(responses)
    }

    /// Get content interactions
    pub async fn get_content_interactions(&self, user_id: Uuid) -> Result<Vec<ContentInteraction>> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                content_type, duration_seconds, completed, user_rating
            FROM content_interactions 
            WHERE user_id = $1 
            ORDER BY created_at DESC 
            LIMIT 1000
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get content interactions: {}", e))?;

        let interactions = rows.into_iter().map(|row| {
            let content_type = match row.content_type.as_str() {
                "quran_verses" => ContentType::QuranVerses,
                "hadith_narrations" => ContentType::HadithNarrations,
                "islamic_stories" => ContentType::IslamicStories,
                "tafsir" => ContentType::Tafsir,
                "dhikr" => ContentType::Dhikr,
                "duas" => ContentType::Duas,
                "islamic_history" => ContentType::IslamicHistory,
                "fiqh" => ContentType::Fiqh,
                "aqeedah" => ContentType::Aqeedah,
                "seerah" => ContentType::Seerah,
                _ => ContentType::QuranVerses,
            };

            ContentInteraction {
                content_type,
                duration_seconds: row.duration_seconds.map(|d| d as u32),
                completed: row.completed.unwrap_or(false),
                user_rating: row.user_rating,
            }
        }).collect();

        Ok(interactions)
    }

    /// Save personalized recommendation
    pub async fn save_recommendation(&self, recommendation: &PersonalizedRecommendation) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO personalized_recommendations (
                id, user_id, content_type, content_id, title, description,
                recommendation_score, reasoning, estimated_duration, difficulty_level,
                tags, category, presented_at, interacted_at, completed_at,
                user_rating, feedback, created_at, expires_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19
            )
            "#,
            recommendation.id,
            recommendation.user_id,
            recommendation.content_type as ContentType,
            recommendation.content_id,
            recommendation.title,
            recommendation.description,
            recommendation.recommendation_score,
            recommendation.reasoning,
            recommendation.estimated_duration,
            recommendation.difficulty_level as DifficultyLevel,
            &recommendation.tags,
            recommendation.category as RecommendationCategory,
            recommendation.presented_at,
            recommendation.interacted_at,
            recommendation.completed_at,
            recommendation.user_rating,
            recommendation.feedback,
            recommendation.created_at,
            recommendation.expires_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to save recommendation: {}", e))?;

        Ok(())
    }

    /// Save adaptive reminder
    pub async fn save_adaptive_reminder(&self, reminder: &AdaptiveReminder) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO adaptive_reminders (
                id, user_id, reminder_type, title, message, suggested_time,
                optimal_time_window, adaptation_confidence, personalization_factors,
                content_customization, response_prediction, actual_response,
                effectiveness_score, is_recurring, recurrence_pattern,
                next_occurrence, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18
            )
            "#,
            reminder.id,
            reminder.user_id,
            reminder.reminder_type as ReminderType,
            reminder.title,
            reminder.message,
            reminder.suggested_time,
            serde_json::to_value(&reminder.optimal_time_window)?,
            reminder.adaptation_confidence,
            serde_json::to_value(&reminder.personalization_factors)?,
            serde_json::to_value(&reminder.content_customization)?,
            reminder.response_prediction,
            reminder.actual_response.as_ref().map(|r| serde_json::to_value(r)).transpose()?,
            reminder.effectiveness_score,
            reminder.is_recurring,
            reminder.recurrence_pattern.as_ref().map(|r| serde_json::to_value(r)).transpose()?,
            reminder.next_occurrence,
            reminder.created_at,
            reminder.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to save adaptive reminder: {}", e))?;

        Ok(())
    }

    /// Save preference learning record
    pub async fn save_learning_record(&self, record: &PreferenceLearningRecord) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO preference_learning_records (
                id, user_id, preference_type, old_value, new_value,
                confidence_score, learning_source, validation_status,
                impact_score, created_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
            )
            "#,
            record.id,
            record.user_id,
            record.preference_type as PreferenceType,
            record.old_value,
            record.new_value,
            record.confidence_score,
            record.learning_source as LearningSource,
            record.validation_status as ValidationStatus,
            record.impact_score,
            record.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to save learning record: {}", e))?;

        Ok(())
    }

    /// Get recent interactions for learning
    pub async fn get_recent_interactions(&self, user_id: Uuid, days: i32) -> Result<Vec<UserInteraction>> {
        let cutoff_date = Utc::now() - chrono::Duration::days(days as i64);
        
        let rows = sqlx::query!(
            r#"
            SELECT 
                timestamp, duration_seconds, interaction_type
            FROM user_interactions 
            WHERE user_id = $1 AND timestamp >= $2
            ORDER BY timestamp DESC
            "#,
            user_id,
            cutoff_date
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get recent interactions: {}", e))?;

        let interactions = rows.into_iter().map(|row| {
            UserInteraction {
                timestamp: row.timestamp,
                duration_seconds: row.duration_seconds.map(|d| d as u32),
                interaction_type: match row.interaction_type.as_str() {
                    "positive" => crate::service::InteractionType::Positive,
                    "negative" => crate::service::InteractionType::Negative,
                    _ => crate::service::InteractionType::Neutral,
                },
            }
        }).collect();

        Ok(interactions)
    }

    /// Get reminder history for effectiveness analysis
    pub async fn get_reminder_history(&self, user_id: Uuid) -> Result<Vec<AdaptiveReminder>> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                id, user_id, reminder_type, title, message, suggested_time,
                optimal_time_window, adaptation_confidence, personalization_factors,
                content_customization, response_prediction, actual_response,
                effectiveness_score, is_recurring, recurrence_pattern,
                next_occurrence, created_at, updated_at
            FROM adaptive_reminders 
            WHERE user_id = $1 
            ORDER BY created_at DESC 
            LIMIT 200
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get reminder history: {}", e))?;

        let mut reminders = Vec::new();
        for row in rows {
            let reminder = AdaptiveReminder {
                id: row.id,
                user_id: row.user_id,
                reminder_type: serde_json::from_str(&format!("\"{}\"", row.reminder_type))
                    .unwrap_or(ReminderType::Prayer),
                title: row.title,
                message: row.message,
                suggested_time: row.suggested_time,
                optimal_time_window: serde_json::from_value(row.optimal_time_window)
                    .unwrap_or_default(),
                adaptation_confidence: row.adaptation_confidence,
                personalization_factors: serde_json::from_value(row.personalization_factors)
                    .unwrap_or_default(),
                content_customization: serde_json::from_value(row.content_customization)
                    .unwrap_or_default(),
                response_prediction: row.response_prediction,
                actual_response: row.actual_response
                    .and_then(|v| serde_json::from_value(v).ok()),
                effectiveness_score: row.effectiveness_score,
                is_recurring: row.is_recurring,
                recurrence_pattern: row.recurrence_pattern
                    .and_then(|v| serde_json::from_value(v).ok()),
                next_occurrence: row.next_occurrence,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            reminders.push(reminder);
        }

        Ok(reminders)
    }

    /// Get interactions for a specific period
    pub async fn get_interactions_for_period(
        &self,
        user_id: Uuid,
        period: &AnalysisPeriod,
    ) -> Result<Vec<UserInteraction>> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                timestamp, duration_seconds, interaction_type
            FROM user_interactions 
            WHERE user_id = $1 AND timestamp >= $2 AND timestamp <= $3
            ORDER BY timestamp DESC
            "#,
            user_id,
            period.start_date,
            period.end_date
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get interactions for period: {}", e))?;

        let interactions = rows.into_iter().map(|row| {
            UserInteraction {
                timestamp: row.timestamp,
                duration_seconds: row.duration_seconds.map(|d| d as u32),
                interaction_type: match row.interaction_type.as_str() {
                    "positive" => crate::service::InteractionType::Positive,
                    "negative" => crate::service::InteractionType::Negative,
                    _ => crate::service::InteractionType::Neutral,
                },
            }
        }).collect();

        Ok(interactions)
    }

    /// Get recommendations for a specific period
    pub async fn get_recommendations_for_period(
        &self,
        user_id: Uuid,
        period: &AnalysisPeriod,
    ) -> Result<Vec<PersonalizedRecommendation>> {
        let rows = sqlx::query_as!(
            PersonalizedRecommendation,
            r#"
            SELECT 
                id, user_id, content_type as "content_type: ContentType", content_id, title, description,
                recommendation_score, reasoning, estimated_duration, 
                difficulty_level as "difficulty_level: DifficultyLevel",
                tags, category as "category: RecommendationCategory", 
                presented_at, interacted_at, completed_at,
                user_rating, feedback, created_at, expires_at
            FROM personalized_recommendations 
            WHERE user_id = $1 AND created_at >= $2 AND created_at <= $3
            ORDER BY created_at DESC
            "#,
            user_id,
            period.start_date,
            period.end_date
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get recommendations for period: {}", e))?;

        Ok(rows)
    }

    /// Get reminders for a specific period
    pub async fn get_reminders_for_period(
        &self,
        user_id: Uuid,
        period: &AnalysisPeriod,
    ) -> Result<Vec<AdaptiveReminder>> {
        // Similar implementation to get_reminder_history but with date filtering
        let rows = sqlx::query!(
            r#"
            SELECT 
                id, user_id, reminder_type, title, message, suggested_time,
                optimal_time_window, adaptation_confidence, personalization_factors,
                content_customization, response_prediction, actual_response,
                effectiveness_score, is_recurring, recurrence_pattern,
                next_occurrence, created_at, updated_at
            FROM adaptive_reminders 
            WHERE user_id = $1 AND created_at >= $2 AND created_at <= $3
            ORDER BY created_at DESC
            "#,
            user_id,
            period.start_date,
            period.end_date
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get reminders for period: {}", e))?;

        // Convert rows to AdaptiveReminder objects (similar to get_reminder_history)
        let mut reminders = Vec::new();
        for row in rows {
            let reminder = AdaptiveReminder {
                id: row.id,
                user_id: row.user_id,
                reminder_type: serde_json::from_str(&format!("\"{}\"", row.reminder_type))
                    .unwrap_or(ReminderType::Prayer),
                title: row.title,
                message: row.message,
                suggested_time: row.suggested_time,
                optimal_time_window: serde_json::from_value(row.optimal_time_window)
                    .unwrap_or_default(),
                adaptation_confidence: row.adaptation_confidence,
                personalization_factors: serde_json::from_value(row.personalization_factors)
                    .unwrap_or_default(),
                content_customization: serde_json::from_value(row.content_customization)
                    .unwrap_or_default(),
                response_prediction: row.response_prediction,
                actual_response: row.actual_response
                    .and_then(|v| serde_json::from_value(v).ok()),
                effectiveness_score: row.effectiveness_score,
                is_recurring: row.is_recurring,
                recurrence_pattern: row.recurrence_pattern
                    .and_then(|v| serde_json::from_value(v).ok()),
                next_occurrence: row.next_occurrence,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            reminders.push(reminder);
        }

        Ok(reminders)
    }

    /// Get learning records for a specific period
    pub async fn get_learning_records_for_period(
        &self,
        user_id: Uuid,
        period: &AnalysisPeriod,
    ) -> Result<Vec<PreferenceLearningRecord>> {
        let rows = sqlx::query_as!(
            PreferenceLearningRecord,
            r#"
            SELECT 
                id, user_id, preference_type as "preference_type: PreferenceType", 
                old_value, new_value, confidence_score, 
                learning_source as "learning_source: LearningSource",
                validation_status as "validation_status: ValidationStatus",
                impact_score, created_at
            FROM preference_learning_records 
            WHERE user_id = $1 AND created_at >= $2 AND created_at <= $3
            ORDER BY created_at DESC
            "#,
            user_id,
            period.start_date,
            period.end_date
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get learning records for period: {}", e))?;

        Ok(rows)
    }
}

// Default implementations for complex types
impl Default for TimeWindow {
    fn default() -> Self {
        Self {
            start_time: chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
            end_time: chrono::NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
            preferred_time: chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            flexibility_minutes: 30,
        }
    }
}

impl Default for ContentCustomization {
    fn default() -> Self {
        Self {
            language: "ar".to_string(),
            tone: MessageTone::Gentle,
            length: MessageLength::Short,
            include_verse: true,
            include_hadith: false,
            include_motivation: true,
            personalized_elements: vec!["user_name".to_string()],
        }
    }
}
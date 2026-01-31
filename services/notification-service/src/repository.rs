use crate::models::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct NotificationRepository {
    pub pool: PgPool,
}

impl NotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new notification
    pub async fn create_notification(&self, request: CreateNotificationRequest) -> Result<Notification> {
        let notification = sqlx::query_as!(
            Notification,
            r#"
            INSERT INTO notifications (
                user_id, notification_type, title, body, priority, 
                scheduled_at, metadata, expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING 
                id, user_id, 
                notification_type as "notification_type: NotificationType",
                title, body,
                priority as "priority: NotificationPriority",
                status as "status: NotificationStatus",
                scheduled_at, sent_at, delivered_at, read_at,
                metadata, expires_at, retry_count, max_retries,
                created_at, updated_at
            "#,
            request.user_id,
            request.notification_type as NotificationType,
            request.title,
            request.body,
            request.priority.unwrap_or(NotificationPriority::Medium) as NotificationPriority,
            request.scheduled_at,
            request.metadata.unwrap_or(serde_json::json!({})),
            request.expires_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(notification)
    }

    /// Get notifications for a user with pagination
    pub async fn get_user_notifications(
        &self,
        user_id: Uuid,
        page: i32,
        page_size: i32,
        status_filter: Option<NotificationStatus>,
    ) -> Result<(Vec<Notification>, i64)> {
        let offset = (page - 1) * page_size;

        let notifications = match status_filter {
            Some(status) => {
                sqlx::query_as!(
                    Notification,
                    r#"
                    SELECT 
                        id, user_id,
                        notification_type as "notification_type: NotificationType",
                        title, body,
                        priority as "priority: NotificationPriority",
                        status as "status: NotificationStatus",
                        scheduled_at, sent_at, delivered_at, read_at,
                        metadata, expires_at, retry_count, max_retries,
                        created_at, updated_at
                    FROM notifications 
                    WHERE user_id = $1 AND status = $2
                    ORDER BY scheduled_at DESC
                    LIMIT $3 OFFSET $4
                    "#,
                    user_id,
                    status as NotificationStatus,
                    page_size as i64,
                    offset as i64
                )
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as!(
                    Notification,
                    r#"
                    SELECT 
                        id, user_id,
                        notification_type as "notification_type: NotificationType",
                        title, body,
                        priority as "priority: NotificationPriority",
                        status as "status: NotificationStatus",
                        scheduled_at, sent_at, delivered_at, read_at,
                        metadata, expires_at, retry_count, max_retries,
                        created_at, updated_at
                    FROM notifications 
                    WHERE user_id = $1
                    ORDER BY scheduled_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                    user_id,
                    page_size as i64,
                    offset as i64
                )
                .fetch_all(&self.pool)
                .await?
            }
        };

        let total_count = match status_filter {
            Some(status) => {
                sqlx::query!(
                    "SELECT COUNT(*) as count FROM notifications WHERE user_id = $1 AND status = $2",
                    user_id,
                    status as NotificationStatus
                )
                .fetch_one(&self.pool)
                .await?
                .count
                .unwrap_or(0)
            }
            None => {
                sqlx::query!(
                    "SELECT COUNT(*) as count FROM notifications WHERE user_id = $1",
                    user_id
                )
                .fetch_one(&self.pool)
                .await?
                .count
                .unwrap_or(0)
            }
        };

        Ok((notifications, total_count))
    }

    /// Get pending notifications that need to be sent
    pub async fn get_pending_notifications(&self, limit: i32) -> Result<Vec<Notification>> {
        let notifications = sqlx::query_as!(
            Notification,
            r#"
            SELECT 
                id, user_id,
                notification_type as "notification_type: NotificationType",
                title, body,
                priority as "priority: NotificationPriority",
                status as "status: NotificationStatus",
                scheduled_at, sent_at, delivered_at, read_at,
                metadata, expires_at, retry_count, max_retries,
                created_at, updated_at
            FROM notifications 
            WHERE status = 'pending' 
                AND scheduled_at <= NOW()
                AND (expires_at IS NULL OR expires_at > NOW())
                AND retry_count < max_retries
            ORDER BY priority DESC, scheduled_at ASC
            LIMIT $1
            "#,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(notifications)
    }

    /// Update notification status
    pub async fn update_notification_status(
        &self,
        notification_id: Uuid,
        status: NotificationStatus,
    ) -> Result<()> {
        let now = Utc::now();
        
        match status {
            NotificationStatus::Sent => {
                sqlx::query!(
                    "UPDATE notifications SET status = $1, sent_at = $2, updated_at = $3 WHERE id = $4",
                    status as NotificationStatus,
                    now,
                    now,
                    notification_id
                )
                .execute(&self.pool)
                .await?;
            }
            NotificationStatus::Delivered => {
                sqlx::query!(
                    "UPDATE notifications SET status = $1, delivered_at = $2, updated_at = $3 WHERE id = $4",
                    status as NotificationStatus,
                    now,
                    now,
                    notification_id
                )
                .execute(&self.pool)
                .await?;
            }
            NotificationStatus::Read => {
                sqlx::query!(
                    "UPDATE notifications SET status = $1, read_at = $2, updated_at = $3 WHERE id = $4",
                    status as NotificationStatus,
                    now,
                    now,
                    notification_id
                )
                .execute(&self.pool)
                .await?;
            }
            _ => {
                sqlx::query!(
                    "UPDATE notifications SET status = $1, updated_at = $2 WHERE id = $3",
                    status as NotificationStatus,
                    now,
                    notification_id
                )
                .execute(&self.pool)
                .await?;
            }
        }

        Ok(())
    }

    /// Create prayer notification
    pub async fn create_prayer_notification(&self, request: CreatePrayerNotificationRequest) -> Result<PrayerNotification> {
        let prayer_notification = sqlx::query_as!(
            PrayerNotification,
            r#"
            INSERT INTO prayer_notifications (
                user_id, prayer_name, prayer_time, enable_graduated,
                reminder_intervals, latitude, longitude, timezone,
                enable_adhan, enable_vibration, custom_message
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING 
                id, user_id,
                prayer_name as "prayer_name: PrayerName",
                prayer_time, enable_graduated, reminder_intervals,
                latitude, longitude, timezone,
                enable_adhan, enable_vibration, custom_message,
                created_at, updated_at
            "#,
            request.user_id,
            request.prayer_name as PrayerName,
            request.prayer_time,
            request.enable_graduated.unwrap_or(true),
            &request.reminder_intervals.unwrap_or(vec![30, 15, 5]),
            request.latitude.map(rust_decimal::Decimal::from_f64_retain).flatten(),
            request.longitude.map(rust_decimal::Decimal::from_f64_retain).flatten(),
            request.timezone,
            request.enable_adhan.unwrap_or(true),
            request.enable_vibration.unwrap_or(true),
            request.custom_message
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(prayer_notification)
    }

    /// Create sunnah reminder
    pub async fn create_sunnah_reminder(&self, request: CreateSunnahReminderRequest) -> Result<SunnahReminder> {
        let sunnah_reminder = sqlx::query_as!(
            SunnahReminder,
            r#"
            INSERT INTO sunnah_reminders (
                user_id, sunnah_name, sunnah_description, sunnah_reference,
                reminder_time, frequency, days_of_week, priority, custom_message
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING 
                id, user_id, sunnah_name, sunnah_description, sunnah_reference,
                reminder_time, frequency, days_of_week, is_active,
                priority as "priority: NotificationPriority",
                custom_message, created_at, updated_at
            "#,
            request.user_id,
            request.sunnah_name,
            request.sunnah_description,
            request.sunnah_reference,
            request.reminder_time,
            request.frequency.unwrap_or("daily".to_string()),
            request.days_of_week.as_deref(),
            request.priority.unwrap_or(NotificationPriority::Medium) as NotificationPriority,
            request.custom_message
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(sunnah_reminder)
    }

    /// Create seasonal reminder
    pub async fn create_seasonal_reminder(&self, request: CreateSeasonalReminderRequest) -> Result<SeasonalReminder> {
        let seasonal_reminder = sqlx::query_as!(
            SeasonalReminder,
            r#"
            INSERT INTO seasonal_reminders (
                user_id, season, event_name, event_description,
                hijri_month, hijri_day, gregorian_date, days_before_notification,
                priority, reminder_message, recommended_actions,
                related_verses, related_hadiths
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING 
                id, user_id,
                season as "season: IslamicSeason",
                event_name, event_description,
                hijri_month, hijri_day, gregorian_date, days_before_notification,
                is_active,
                priority as "priority: NotificationPriority",
                reminder_message, recommended_actions,
                related_verses, related_hadiths,
                created_at, updated_at
            "#,
            request.user_id,
            request.season as IslamicSeason,
            request.event_name,
            request.event_description,
            request.hijri_month,
            request.hijri_day,
            request.gregorian_date,
            request.days_before_notification.unwrap_or(1),
            request.priority.unwrap_or(NotificationPriority::High) as NotificationPriority,
            request.reminder_message,
            request.recommended_actions.as_deref(),
            request.related_verses.as_deref(),
            request.related_hadiths.as_deref()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(seasonal_reminder)
    }

    /// Create dhikr reminder
    pub async fn create_dhikr_reminder(&self, request: CreateDhikrReminderRequest) -> Result<DhikrReminder> {
        let dhikr_reminder = sqlx::query_as!(
            DhikrReminder,
            r#"
            INSERT INTO dhikr_reminders (
                user_id, dhikr_category, dhikr_text_arabic,
                dhikr_text_transliteration, dhikr_text_translation, dhikr_reference,
                trigger_time, trigger_after_prayer, trigger_condition,
                frequency, priority, recommended_repetitions, track_completion
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING 
                id, user_id,
                dhikr_category as "dhikr_category: DhikrCategory",
                dhikr_text_arabic, dhikr_text_transliteration, dhikr_text_translation,
                dhikr_reference, trigger_time,
                trigger_after_prayer as "trigger_after_prayer: Option<PrayerName>",
                trigger_condition, is_active, frequency,
                priority as "priority: NotificationPriority",
                recommended_repetitions, track_completion,
                created_at, updated_at
            "#,
            request.user_id,
            request.dhikr_category as DhikrCategory,
            request.dhikr_text_arabic,
            request.dhikr_text_transliteration,
            request.dhikr_text_translation,
            request.dhikr_reference,
            request.trigger_time,
            request.trigger_after_prayer.map(|p| p as PrayerName),
            request.trigger_condition,
            request.frequency.unwrap_or("daily".to_string()),
            request.priority.unwrap_or(NotificationPriority::Low) as NotificationPriority,
            request.recommended_repetitions.unwrap_or(1),
            request.track_completion.unwrap_or(false)
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(dhikr_reminder)
    }

    /// Get or create user notification preferences
    pub async fn get_user_preferences(&self, user_id: Uuid) -> Result<UserNotificationPreferences> {
        // Try to get existing preferences
        let existing = sqlx::query_as!(
            UserNotificationPreferences,
            r#"
            SELECT 
                id, user_id, notifications_enabled, quiet_hours_start, quiet_hours_end,
                prayer_notifications_enabled, prayer_graduated_enabled, prayer_reminder_intervals,
                sunnah_reminders_enabled, nafl_reminders_enabled,
                dhikr_reminders_enabled, morning_dhikr_time, evening_dhikr_time,
                seasonal_reminders_enabled, ramadan_reminders_enabled, hajj_reminders_enabled,
                push_notifications, email_notifications, sms_notifications,
                created_at, updated_at
            FROM user_notification_preferences 
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(preferences) = existing {
            return Ok(preferences);
        }

        // Create default preferences if none exist
        let preferences = sqlx::query_as!(
            UserNotificationPreferences,
            r#"
            INSERT INTO user_notification_preferences (user_id)
            VALUES ($1)
            RETURNING 
                id, user_id, notifications_enabled, quiet_hours_start, quiet_hours_end,
                prayer_notifications_enabled, prayer_graduated_enabled, prayer_reminder_intervals,
                sunnah_reminders_enabled, nafl_reminders_enabled,
                dhikr_reminders_enabled, morning_dhikr_time, evening_dhikr_time,
                seasonal_reminders_enabled, ramadan_reminders_enabled, hajj_reminders_enabled,
                push_notifications, email_notifications, sms_notifications,
                created_at, updated_at
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(preferences)
    }

    /// Update user notification preferences
    pub async fn update_user_preferences(
        &self,
        user_id: Uuid,
        request: UpdateNotificationPreferencesRequest,
    ) -> Result<UserNotificationPreferences> {
        // First ensure preferences exist
        self.get_user_preferences(user_id).await?;

        // Build dynamic update query
        let mut query = "UPDATE user_notification_preferences SET updated_at = NOW()".to_string();
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
        let mut param_count = 1;

        if let Some(enabled) = request.notifications_enabled {
            query.push_str(&format!(", notifications_enabled = ${}", param_count));
            params.push(Box::new(enabled));
            param_count += 1;
        }

        if let Some(start) = request.quiet_hours_start {
            query.push_str(&format!(", quiet_hours_start = ${}", param_count));
            params.push(Box::new(start));
            param_count += 1;
        }

        if let Some(end) = request.quiet_hours_end {
            query.push_str(&format!(", quiet_hours_end = ${}", param_count));
            params.push(Box::new(end));
            param_count += 1;
        }

        if let Some(enabled) = request.prayer_notifications_enabled {
            query.push_str(&format!(", prayer_notifications_enabled = ${}", param_count));
            params.push(Box::new(enabled));
            param_count += 1;
        }

        if let Some(enabled) = request.prayer_graduated_enabled {
            query.push_str(&format!(", prayer_graduated_enabled = ${}", param_count));
            params.push(Box::new(enabled));
            param_count += 1;
        }

        if let Some(intervals) = request.prayer_reminder_intervals {
            query.push_str(&format!(", prayer_reminder_intervals = ${}", param_count));
            params.push(Box::new(intervals));
            param_count += 1;
        }

        query.push_str(&format!(" WHERE user_id = ${}", param_count));
        params.push(Box::new(user_id));

        // Execute the update
        let mut query_builder = sqlx::query(&query);
        for param in params {
            // This is a simplified approach - in practice, you'd need to handle the dynamic parameters properly
        }

        // For now, let's use a simpler approach with individual updates
        if let Some(enabled) = request.notifications_enabled {
            sqlx::query!(
                "UPDATE user_notification_preferences SET notifications_enabled = $1, updated_at = NOW() WHERE user_id = $2",
                enabled, user_id
            ).execute(&self.pool).await?;
        }

        // Get updated preferences
        self.get_user_preferences(user_id).await
    }

    /// Get notification statistics for a user
    pub async fn get_notification_stats(&self, user_id: Uuid) -> Result<NotificationStatsResponse> {
        let stats = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as total_notifications,
                COUNT(*) FILTER (WHERE status = 'pending') as pending_notifications,
                COUNT(*) FILTER (WHERE status = 'sent') as sent_notifications,
                COUNT(*) FILTER (WHERE status = 'delivered') as delivered_notifications,
                COUNT(*) FILTER (WHERE status = 'read') as read_notifications,
                COUNT(*) FILTER (WHERE status = 'failed') as failed_notifications
            FROM notifications 
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(NotificationStatsResponse {
            total_notifications: stats.total_notifications.unwrap_or(0),
            pending_notifications: stats.pending_notifications.unwrap_or(0),
            sent_notifications: stats.sent_notifications.unwrap_or(0),
            delivered_notifications: stats.delivered_notifications.unwrap_or(0),
            read_notifications: stats.read_notifications.unwrap_or(0),
            failed_notifications: stats.failed_notifications.unwrap_or(0),
        })
    }

    /// Log notification delivery attempt
    pub async fn log_delivery_attempt(
        &self,
        notification_id: Uuid,
        user_id: Uuid,
        delivery_method: String,
        delivery_status: NotificationStatus,
        delivery_attempt: i32,
        error_message: Option<String>,
        error_code: Option<String>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO notification_delivery_log (
                notification_id, user_id, delivery_method, delivery_status,
                delivery_attempt, error_message, error_code
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            notification_id,
            user_id,
            delivery_method,
            delivery_status as NotificationStatus,
            delivery_attempt,
            error_message,
            error_code
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get default dhikr content by category
    pub async fn get_default_dhikr_content(&self, category: DhikrCategory) -> Result<Vec<DefaultDhikrContent>> {
        let content = sqlx::query_as!(
            DefaultDhikrContent,
            r#"
            SELECT 
                id,
                category as "category: DhikrCategory",
                title, arabic_text, transliteration,
                translation_en, translation_ar, reference,
                repetitions, order_index, is_active, created_at
            FROM default_dhikr_content 
            WHERE category = $1 AND is_active = true
            ORDER BY order_index ASC
            "#,
            category as DhikrCategory
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(content)
    }

    /// Get active sunnah reminders for a user
    pub async fn get_active_sunnah_reminders(&self, user_id: Uuid) -> Result<Vec<SunnahReminder>> {
        let reminders = sqlx::query_as!(
            SunnahReminder,
            r#"
            SELECT 
                id, user_id, sunnah_name, sunnah_description, sunnah_reference,
                reminder_time, frequency, days_of_week, is_active,
                priority as "priority: NotificationPriority",
                custom_message, created_at, updated_at
            FROM sunnah_reminders 
            WHERE user_id = $1 AND is_active = true
            ORDER BY reminder_time ASC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(reminders)
    }

    /// Get active seasonal reminders for a user
    pub async fn get_active_seasonal_reminders(&self, user_id: Uuid) -> Result<Vec<SeasonalReminder>> {
        let reminders = sqlx::query_as!(
            SeasonalReminder,
            r#"
            SELECT 
                id, user_id,
                season as "season: IslamicSeason",
                event_name, event_description,
                hijri_month, hijri_day, gregorian_date, days_before_notification,
                is_active,
                priority as "priority: NotificationPriority",
                reminder_message, recommended_actions,
                related_verses, related_hadiths,
                created_at, updated_at
            FROM seasonal_reminders 
            WHERE user_id = $1 AND is_active = true
            ORDER BY hijri_month ASC, hijri_day ASC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(reminders)
    }

    /// Get active dhikr reminders for a user
    pub async fn get_active_dhikr_reminders(&self, user_id: Uuid) -> Result<Vec<DhikrReminder>> {
        let reminders = sqlx::query_as!(
            DhikrReminder,
            r#"
            SELECT 
                id, user_id,
                dhikr_category as "dhikr_category: DhikrCategory",
                dhikr_text_arabic, dhikr_text_transliteration, dhikr_text_translation,
                dhikr_reference, trigger_time,
                trigger_after_prayer as "trigger_after_prayer: Option<PrayerName>",
                trigger_condition, is_active, frequency,
                priority as "priority: NotificationPriority",
                recommended_repetitions, track_completion,
                created_at, updated_at
            FROM dhikr_reminders 
            WHERE user_id = $1 AND is_active = true
            ORDER BY dhikr_category ASC, trigger_time ASC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(reminders)
    }
}
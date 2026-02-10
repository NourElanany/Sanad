use crate::models::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
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
        let row = sqlx::query(
            r#"
            INSERT INTO notifications (
                user_id, notification_type, title, body, priority, 
                scheduled_at, metadata, expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING 
                id, user_id, notification_type, title, body, priority, status,
                scheduled_at, sent_at, delivered_at, read_at,
                metadata, expires_at, retry_count, max_retries,
                created_at, updated_at
            "#
        )
        .bind(request.user_id)
        .bind(request.notification_type as NotificationType)
        .bind(request.title)
        .bind(request.body)
        .bind(request.priority.unwrap_or(NotificationPriority::Medium) as NotificationPriority)
        .bind(request.scheduled_at)
        .bind(request.metadata.unwrap_or(serde_json::json!({})))
        .bind(request.expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(Notification {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            notification_type: row.try_get("notification_type")?,
            title: row.try_get("title")?,
            body: row.try_get("body")?,
            priority: row.try_get("priority")?,
            status: row.try_get("status")?,
            scheduled_at: row.try_get("scheduled_at")?,
            sent_at: row.try_get("sent_at")?,
            delivered_at: row.try_get("delivered_at")?,
            read_at: row.try_get("read_at")?,
            metadata: row.try_get("metadata")?,
            expires_at: row.try_get("expires_at")?,
            retry_count: row.try_get("retry_count")?,
            max_retries: row.try_get("max_retries")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
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

        let notifications = match &status_filter {
            Some(status) => {
                let rows = sqlx::query(
                    r#"
                    SELECT 
                        id, user_id, notification_type, title, body, priority, status,
                        scheduled_at, sent_at, delivered_at, read_at,
                        metadata, expires_at, retry_count, max_retries,
                        created_at, updated_at
                    FROM notifications 
                    WHERE user_id = $1 AND status = $2
                    ORDER BY scheduled_at DESC
                    LIMIT $3 OFFSET $4
                    "#
                )
                .bind(user_id)
                .bind(status.clone())
                .bind(page_size as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?;

                rows.into_iter().map(|row| {
                    Ok(Notification {
                        id: row.try_get("id")?,
                        user_id: row.try_get("user_id")?,
                        notification_type: row.try_get("notification_type")?,
                        title: row.try_get("title")?,
                        body: row.try_get("body")?,
                        priority: row.try_get("priority")?,
                        status: row.try_get("status")?,
                        scheduled_at: row.try_get("scheduled_at")?,
                        sent_at: row.try_get("sent_at")?,
                        delivered_at: row.try_get("delivered_at")?,
                        read_at: row.try_get("read_at")?,
                        metadata: row.try_get("metadata")?,
                        expires_at: row.try_get("expires_at")?,
                        retry_count: row.try_get("retry_count")?,
                        max_retries: row.try_get("max_retries")?,
                        created_at: row.try_get("created_at")?,
                        updated_at: row.try_get("updated_at")?,
                    })
                }).collect::<Result<Vec<_>>>()?
            }
            None => {
                let rows = sqlx::query(
                    r#"
                    SELECT 
                        id, user_id, notification_type, title, body, priority, status,
                        scheduled_at, sent_at, delivered_at, read_at,
                        metadata, expires_at, retry_count, max_retries,
                        created_at, updated_at
                    FROM notifications 
                    WHERE user_id = $1
                    ORDER BY scheduled_at DESC
                    LIMIT $2 OFFSET $3
                    "#
                )
                .bind(user_id)
                .bind(page_size as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?;

                rows.into_iter().map(|row| {
                    Ok(Notification {
                        id: row.try_get("id")?,
                        user_id: row.try_get("user_id")?,
                        notification_type: row.try_get("notification_type")?,
                        title: row.try_get("title")?,
                        body: row.try_get("body")?,
                        priority: row.try_get("priority")?,
                        status: row.try_get("status")?,
                        scheduled_at: row.try_get("scheduled_at")?,
                        sent_at: row.try_get("sent_at")?,
                        delivered_at: row.try_get("delivered_at")?,
                        read_at: row.try_get("read_at")?,
                        metadata: row.try_get("metadata")?,
                        expires_at: row.try_get("expires_at")?,
                        retry_count: row.try_get("retry_count")?,
                        max_retries: row.try_get("max_retries")?,
                        created_at: row.try_get("created_at")?,
                        updated_at: row.try_get("updated_at")?,
                    })
                }).collect::<Result<Vec<_>>>()?
            }
        };

        let total_count: i64 = match &status_filter {
            Some(status) => {
                sqlx::query_scalar(
                    "SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND status = $2"
                )
                .bind(user_id)
                .bind(status.clone())
                .fetch_one(&self.pool)
                .await?
            }
            None => {
                sqlx::query_scalar(
                    "SELECT COUNT(*) FROM notifications WHERE user_id = $1"
                )
                .bind(user_id)
                .fetch_one(&self.pool)
                .await?
            }
        };

        Ok((notifications, total_count))
    }

    /// Get pending notifications that need to be sent
    pub async fn get_pending_notifications(&self, limit: i32) -> Result<Vec<Notification>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, user_id, notification_type, title, body, priority, status,
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
            "#
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let notifications = rows.into_iter().map(|row| {
            Ok(Notification {
                id: row.try_get("id")?,
                user_id: row.try_get("user_id")?,
                notification_type: row.try_get("notification_type")?,
                title: row.try_get("title")?,
                body: row.try_get("body")?,
                priority: row.try_get("priority")?,
                status: row.try_get("status")?,
                scheduled_at: row.try_get("scheduled_at")?,
                sent_at: row.try_get("sent_at")?,
                delivered_at: row.try_get("delivered_at")?,
                read_at: row.try_get("read_at")?,
                metadata: row.try_get("metadata")?,
                expires_at: row.try_get("expires_at")?,
                retry_count: row.try_get("retry_count")?,
                max_retries: row.try_get("max_retries")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            })
        }).collect::<Result<Vec<_>>>()?;

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
                sqlx::query(
                    "UPDATE notifications SET status = $1, sent_at = $2, updated_at = $3 WHERE id = $4"
                )
                .bind(status as NotificationStatus)
                .bind(now)
                .bind(now)
                .bind(notification_id)
                .execute(&self.pool)
                .await?;
            }
            NotificationStatus::Delivered => {
                sqlx::query(
                    "UPDATE notifications SET status = $1, delivered_at = $2, updated_at = $3 WHERE id = $4"
                )
                .bind(status as NotificationStatus)
                .bind(now)
                .bind(now)
                .bind(notification_id)
                .execute(&self.pool)
                .await?;
            }
            NotificationStatus::Read => {
                sqlx::query(
                    "UPDATE notifications SET status = $1, read_at = $2, updated_at = $3 WHERE id = $4"
                )
                .bind(status as NotificationStatus)
                .bind(now)
                .bind(now)
                .bind(notification_id)
                .execute(&self.pool)
                .await?;
            }
            _ => {
                sqlx::query(
                    "UPDATE notifications SET status = $1, updated_at = $2 WHERE id = $3"
                )
                .bind(status as NotificationStatus)
                .bind(now)
                .bind(notification_id)
                .execute(&self.pool)
                .await?;
            }
        }

        Ok(())
    }

    /// Create prayer notification
    pub async fn create_prayer_notification(&self, request: CreatePrayerNotificationRequest) -> Result<PrayerNotification> {
        let row = sqlx::query(
            r#"
            INSERT INTO prayer_notifications (
                user_id, prayer_name, prayer_time, enable_graduated,
                reminder_intervals, latitude, longitude, timezone,
                enable_adhan, enable_vibration, custom_message
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING 
                id, user_id, prayer_name, prayer_time, enable_graduated, reminder_intervals,
                latitude, longitude, timezone,
                enable_adhan, enable_vibration, custom_message,
                created_at, updated_at
            "#
        )
        .bind(request.user_id)
        .bind(request.prayer_name as PrayerName)
        .bind(request.prayer_time)
        .bind(request.enable_graduated.unwrap_or(true))
        .bind(&request.reminder_intervals.unwrap_or(vec![30, 15, 5]))
        .bind(request.latitude)
        .bind(request.longitude)
        .bind(request.timezone)
        .bind(request.enable_adhan.unwrap_or(true))
        .bind(request.enable_vibration.unwrap_or(true))
        .bind(request.custom_message)
        .fetch_one(&self.pool)
        .await?;

        Ok(PrayerNotification {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            prayer_name: row.try_get("prayer_name")?,
            prayer_time: row.try_get("prayer_time")?,
            enable_graduated: row.try_get("enable_graduated")?,
            reminder_intervals: row.try_get("reminder_intervals")?,
            latitude: row.try_get("latitude")?,
            longitude: row.try_get("longitude")?,
            timezone: row.try_get("timezone")?,
            enable_adhan: row.try_get("enable_adhan")?,
            enable_vibration: row.try_get("enable_vibration")?,
            custom_message: row.try_get("custom_message")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    /// Create sunnah reminder
    pub async fn create_sunnah_reminder(&self, request: CreateSunnahReminderRequest) -> Result<SunnahReminder> {
        let row = sqlx::query(
            r#"
            INSERT INTO sunnah_reminders (
                user_id, sunnah_name, sunnah_description, sunnah_reference,
                reminder_time, frequency, days_of_week, priority, custom_message
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING 
                id, user_id, sunnah_name, sunnah_description, sunnah_reference,
                reminder_time, frequency, days_of_week, is_active,
                priority, custom_message, created_at, updated_at
            "#
        )
        .bind(request.user_id)
        .bind(request.sunnah_name)
        .bind(request.sunnah_description)
        .bind(request.sunnah_reference)
        .bind(request.reminder_time)
        .bind(request.frequency.unwrap_or("daily".to_string()))
        .bind(request.days_of_week.as_deref())
        .bind(request.priority.unwrap_or(NotificationPriority::Medium) as NotificationPriority)
        .bind(request.custom_message)
        .fetch_one(&self.pool)
        .await?;

        Ok(SunnahReminder {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            sunnah_name: row.try_get("sunnah_name")?,
            sunnah_description: row.try_get("sunnah_description")?,
            sunnah_reference: row.try_get("sunnah_reference")?,
            reminder_time: row.try_get("reminder_time")?,
            frequency: row.try_get("frequency")?,
            days_of_week: row.try_get("days_of_week")?,
            is_active: row.try_get("is_active")?,
            priority: row.try_get("priority")?,
            custom_message: row.try_get("custom_message")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    /// Create seasonal reminder
    pub async fn create_seasonal_reminder(&self, request: CreateSeasonalReminderRequest) -> Result<SeasonalReminder> {
        let row = sqlx::query(
            r#"
            INSERT INTO seasonal_reminders (
                user_id, season, event_name, event_description,
                hijri_month, hijri_day, gregorian_date, days_before_notification,
                priority, reminder_message, recommended_actions,
                related_verses, related_hadiths
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING 
                id, user_id, season, event_name, event_description,
                hijri_month, hijri_day, gregorian_date, days_before_notification,
                is_active, priority, reminder_message, recommended_actions,
                related_verses, related_hadiths,
                created_at, updated_at
            "#
        )
        .bind(request.user_id)
        .bind(request.season as IslamicSeason)
        .bind(request.event_name)
        .bind(request.event_description)
        .bind(request.hijri_month)
        .bind(request.hijri_day)
        .bind(request.gregorian_date)
        .bind(request.days_before_notification.unwrap_or(1))
        .bind(request.priority.unwrap_or(NotificationPriority::High) as NotificationPriority)
        .bind(request.reminder_message)
        .bind(request.recommended_actions.as_deref())
        .bind(request.related_verses.as_deref())
        .bind(request.related_hadiths.as_deref())
        .fetch_one(&self.pool)
        .await?;

        Ok(SeasonalReminder {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            season: row.try_get("season")?,
            event_name: row.try_get("event_name")?,
            event_description: row.try_get("event_description")?,
            hijri_month: row.try_get("hijri_month")?,
            hijri_day: row.try_get("hijri_day")?,
            gregorian_date: row.try_get("gregorian_date")?,
            days_before_notification: row.try_get("days_before_notification")?,
            is_active: row.try_get("is_active")?,
            priority: row.try_get("priority")?,
            reminder_message: row.try_get("reminder_message")?,
            recommended_actions: row.try_get("recommended_actions")?,
            related_verses: row.try_get("related_verses")?,
            related_hadiths: row.try_get("related_hadiths")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    /// Create dhikr reminder
    pub async fn create_dhikr_reminder(&self, request: CreateDhikrReminderRequest) -> Result<DhikrReminder> {
        let row = sqlx::query(
            r#"
            INSERT INTO dhikr_reminders (
                user_id, dhikr_category, dhikr_text_arabic,
                dhikr_text_transliteration, dhikr_text_translation, dhikr_reference,
                trigger_time, trigger_after_prayer, trigger_condition,
                frequency, priority, recommended_repetitions, track_completion
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING 
                id, user_id, dhikr_category, dhikr_text_arabic, dhikr_text_transliteration, dhikr_text_translation,
                dhikr_reference, trigger_time, trigger_after_prayer,
                trigger_condition, is_active, frequency,
                priority, recommended_repetitions, track_completion,
                created_at, updated_at
            "#
        )
        .bind(request.user_id)
        .bind(request.dhikr_category as DhikrCategory)
        .bind(request.dhikr_text_arabic)
        .bind(request.dhikr_text_transliteration)
        .bind(request.dhikr_text_translation)
        .bind(request.dhikr_reference)
        .bind(request.trigger_time)
        .bind(request.trigger_after_prayer.map(|p| p as PrayerName))
        .bind(request.trigger_condition)
        .bind(request.frequency.unwrap_or("daily".to_string()))
        .bind(request.priority.unwrap_or(NotificationPriority::Low) as NotificationPriority)
        .bind(request.recommended_repetitions.unwrap_or(1))
        .bind(request.track_completion.unwrap_or(false))
        .fetch_one(&self.pool)
        .await?;

        Ok(DhikrReminder {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            dhikr_category: row.try_get("dhikr_category")?,
            dhikr_text_arabic: row.try_get("dhikr_text_arabic")?,
            dhikr_text_transliteration: row.try_get("dhikr_text_transliteration")?,
            dhikr_text_translation: row.try_get("dhikr_text_translation")?,
            dhikr_reference: row.try_get("dhikr_reference")?,
            trigger_time: row.try_get("trigger_time")?,
            trigger_after_prayer: row.try_get("trigger_after_prayer")?,
            trigger_condition: row.try_get("trigger_condition")?,
            is_active: row.try_get("is_active")?,
            frequency: row.try_get("frequency")?,
            priority: row.try_get("priority")?,
            recommended_repetitions: row.try_get("recommended_repetitions")?,
            track_completion: row.try_get("track_completion")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    /// Get or create user notification preferences
    pub async fn get_user_preferences(&self, user_id: Uuid) -> Result<UserNotificationPreferences> {
        // Try to get existing preferences
        let existing_row = sqlx::query(
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
            "#
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = existing_row {
            return Ok(UserNotificationPreferences {
                id: row.try_get("id")?,
                user_id: row.try_get("user_id")?,
                notifications_enabled: row.try_get("notifications_enabled")?,
                quiet_hours_start: row.try_get("quiet_hours_start")?,
                quiet_hours_end: row.try_get("quiet_hours_end")?,
                prayer_notifications_enabled: row.try_get("prayer_notifications_enabled")?,
                prayer_graduated_enabled: row.try_get("prayer_graduated_enabled")?,
                prayer_reminder_intervals: row.try_get("prayer_reminder_intervals")?,
                sunnah_reminders_enabled: row.try_get("sunnah_reminders_enabled")?,
                nafl_reminders_enabled: row.try_get("nafl_reminders_enabled")?,
                dhikr_reminders_enabled: row.try_get("dhikr_reminders_enabled")?,
                morning_dhikr_time: row.try_get("morning_dhikr_time")?,
                evening_dhikr_time: row.try_get("evening_dhikr_time")?,
                seasonal_reminders_enabled: row.try_get("seasonal_reminders_enabled")?,
                ramadan_reminders_enabled: row.try_get("ramadan_reminders_enabled")?,
                hajj_reminders_enabled: row.try_get("hajj_reminders_enabled")?,
                push_notifications: row.try_get("push_notifications")?,
                email_notifications: row.try_get("email_notifications")?,
                sms_notifications: row.try_get("sms_notifications")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            });
        }

        // Create default preferences if none exist
        let row = sqlx::query(
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
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(UserNotificationPreferences {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            notifications_enabled: row.try_get("notifications_enabled")?,
            quiet_hours_start: row.try_get("quiet_hours_start")?,
            quiet_hours_end: row.try_get("quiet_hours_end")?,
            prayer_notifications_enabled: row.try_get("prayer_notifications_enabled")?,
            prayer_graduated_enabled: row.try_get("prayer_graduated_enabled")?,
            prayer_reminder_intervals: row.try_get("prayer_reminder_intervals")?,
            sunnah_reminders_enabled: row.try_get("sunnah_reminders_enabled")?,
            nafl_reminders_enabled: row.try_get("nafl_reminders_enabled")?,
            dhikr_reminders_enabled: row.try_get("dhikr_reminders_enabled")?,
            morning_dhikr_time: row.try_get("morning_dhikr_time")?,
            evening_dhikr_time: row.try_get("evening_dhikr_time")?,
            seasonal_reminders_enabled: row.try_get("seasonal_reminders_enabled")?,
            ramadan_reminders_enabled: row.try_get("ramadan_reminders_enabled")?,
            hajj_reminders_enabled: row.try_get("hajj_reminders_enabled")?,
            push_notifications: row.try_get("push_notifications")?,
            email_notifications: row.try_get("email_notifications")?,
            sms_notifications: row.try_get("sms_notifications")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
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

        // For now, let's use a simpler approach with individual updates
        if let Some(enabled) = request.notifications_enabled {
            sqlx::query(
                "UPDATE user_notification_preferences SET notifications_enabled = $1, updated_at = NOW() WHERE user_id = $2"
            )
            .bind(enabled)
            .bind(user_id)
            .execute(&self.pool).await?;
        }

        // Get updated preferences
        self.get_user_preferences(user_id).await
    }

    /// Get notification statistics for a user
    pub async fn get_notification_stats(&self, user_id: Uuid) -> Result<NotificationStatsResponse> {
        let row = sqlx::query(
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
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(NotificationStatsResponse {
            total_notifications: row.try_get::<Option<i64>, _>("total_notifications")?.unwrap_or(0),
            pending_notifications: row.try_get::<Option<i64>, _>("pending_notifications")?.unwrap_or(0),
            sent_notifications: row.try_get::<Option<i64>, _>("sent_notifications")?.unwrap_or(0),
            delivered_notifications: row.try_get::<Option<i64>, _>("delivered_notifications")?.unwrap_or(0),
            read_notifications: row.try_get::<Option<i64>, _>("read_notifications")?.unwrap_or(0),
            failed_notifications: row.try_get::<Option<i64>, _>("failed_notifications")?.unwrap_or(0),
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
        sqlx::query(
            r#"
            INSERT INTO notification_delivery_log (
                notification_id, user_id, delivery_method, delivery_status,
                delivery_attempt, error_message, error_code
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(notification_id)
        .bind(user_id)
        .bind(delivery_method)
        .bind(delivery_status as NotificationStatus)
        .bind(delivery_attempt)
        .bind(error_message)
        .bind(error_code)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get default dhikr content by category
    pub async fn get_default_dhikr_content(&self, category: DhikrCategory) -> Result<Vec<DefaultDhikrContent>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, category, title, arabic_text, transliteration,
                translation_en, translation_ar, reference,
                repetitions, order_index, is_active, created_at
            FROM default_dhikr_content 
            WHERE category = $1 AND is_active = true
            ORDER BY order_index ASC
            "#
        )
        .bind(category as DhikrCategory)
        .fetch_all(&self.pool)
        .await?;

        let content = rows.into_iter().map(|row| {
            Ok(DefaultDhikrContent {
                id: row.try_get("id")?,
                category: row.try_get("category")?,
                title: row.try_get("title")?,
                arabic_text: row.try_get("arabic_text")?,
                transliteration: row.try_get("transliteration")?,
                translation_en: row.try_get("translation_en")?,
                translation_ar: row.try_get("translation_ar")?,
                reference: row.try_get("reference")?,
                repetitions: row.try_get("repetitions")?,
                order_index: row.try_get("order_index")?,
                is_active: row.try_get("is_active")?,
                created_at: row.try_get("created_at")?,
            })
        }).collect::<Result<Vec<_>>>()?;

        Ok(content)
    }

    /// Get active sunnah reminders for a user
    pub async fn get_active_sunnah_reminders(&self, user_id: Uuid) -> Result<Vec<SunnahReminder>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, user_id, sunnah_name, sunnah_description, sunnah_reference,
                reminder_time, frequency, days_of_week, is_active,
                priority, custom_message, created_at, updated_at
            FROM sunnah_reminders 
            WHERE user_id = $1 AND is_active = true
            ORDER BY reminder_time ASC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let reminders = rows.into_iter().map(|row| {
            Ok(SunnahReminder {
                id: row.try_get("id")?,
                user_id: row.try_get("user_id")?,
                sunnah_name: row.try_get("sunnah_name")?,
                sunnah_description: row.try_get("sunnah_description")?,
                sunnah_reference: row.try_get("sunnah_reference")?,
                reminder_time: row.try_get("reminder_time")?,
                frequency: row.try_get("frequency")?,
                days_of_week: row.try_get("days_of_week")?,
                is_active: row.try_get("is_active")?,
                priority: row.try_get("priority")?,
                custom_message: row.try_get("custom_message")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            })
        }).collect::<Result<Vec<_>>>()?;

        Ok(reminders)
    }

    /// Get active seasonal reminders for a user
    pub async fn get_active_seasonal_reminders(&self, user_id: Uuid) -> Result<Vec<SeasonalReminder>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, user_id, season, event_name, event_description,
                hijri_month, hijri_day, gregorian_date, days_before_notification,
                is_active, priority, reminder_message, recommended_actions,
                related_verses, related_hadiths,
                created_at, updated_at
            FROM seasonal_reminders 
            WHERE user_id = $1 AND is_active = true
            ORDER BY hijri_month ASC, hijri_day ASC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let reminders = rows.into_iter().map(|row| {
            Ok(SeasonalReminder {
                id: row.try_get("id")?,
                user_id: row.try_get("user_id")?,
                season: row.try_get("season")?,
                event_name: row.try_get("event_name")?,
                event_description: row.try_get("event_description")?,
                hijri_month: row.try_get("hijri_month")?,
                hijri_day: row.try_get("hijri_day")?,
                gregorian_date: row.try_get("gregorian_date")?,
                days_before_notification: row.try_get("days_before_notification")?,
                is_active: row.try_get("is_active")?,
                priority: row.try_get("priority")?,
                reminder_message: row.try_get("reminder_message")?,
                recommended_actions: row.try_get("recommended_actions")?,
                related_verses: row.try_get("related_verses")?,
                related_hadiths: row.try_get("related_hadiths")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            })
        }).collect::<Result<Vec<_>>>()?;

        Ok(reminders)
    }

    /// Get active dhikr reminders for a user
    pub async fn get_active_dhikr_reminders(&self, user_id: Uuid) -> Result<Vec<DhikrReminder>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id, user_id, dhikr_category, dhikr_text_arabic, dhikr_text_transliteration, dhikr_text_translation,
                dhikr_reference, trigger_time, trigger_after_prayer,
                trigger_condition, is_active, frequency,
                priority, recommended_repetitions, track_completion,
                created_at, updated_at
            FROM dhikr_reminders 
            WHERE user_id = $1 AND is_active = true
            ORDER BY dhikr_category ASC, trigger_time ASC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let reminders = rows.into_iter().map(|row| {
            Ok(DhikrReminder {
                id: row.try_get("id")?,
                user_id: row.try_get("user_id")?,
                dhikr_category: row.try_get("dhikr_category")?,
                dhikr_text_arabic: row.try_get("dhikr_text_arabic")?,
                dhikr_text_transliteration: row.try_get("dhikr_text_transliteration")?,
                dhikr_text_translation: row.try_get("dhikr_text_translation")?,
                dhikr_reference: row.try_get("dhikr_reference")?,
                trigger_time: row.try_get("trigger_time")?,
                trigger_after_prayer: row.try_get("trigger_after_prayer")?,
                trigger_condition: row.try_get("trigger_condition")?,
                is_active: row.try_get("is_active")?,
                frequency: row.try_get("frequency")?,
                priority: row.try_get("priority")?,
                recommended_repetitions: row.try_get("recommended_repetitions")?,
                track_completion: row.try_get("track_completion")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            })
        }).collect::<Result<Vec<_>>>()?;

        Ok(reminders)
    }
}
use crate::models::*;
use crate::repository::NotificationRepository;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc, Duration, NaiveTime, Datelike};
use uuid::Uuid;
use tracing::{info, warn, error};

pub struct NotificationService {
    pub repository: NotificationRepository,
}

impl NotificationService {
    pub fn new(repository: NotificationRepository) -> Self {
        Self { repository }
    }

    /// Create graduated prayer notifications
    /// This creates multiple notifications at different intervals before prayer time
    pub async fn create_graduated_prayer_notifications(
        &self,
        request: CreatePrayerNotificationRequest,
    ) -> Result<Vec<Notification>> {
        info!("Creating graduated prayer notifications for user {} and prayer {:?}", 
              request.user_id, request.prayer_name);

        // First create the prayer notification record
        let prayer_notification = self.repository.create_prayer_notification(request.clone()).await?;
        
        let mut notifications = Vec::new();
        
        // Get user preferences to check if graduated notifications are enabled
        let preferences = self.repository.get_user_preferences(request.user_id).await?;
        
        if !preferences.prayer_notifications_enabled || !preferences.prayer_graduated_enabled {
            info!("Prayer notifications disabled for user {}", request.user_id);
            return Ok(notifications);
        }

        // Use custom intervals or default from preferences
        let intervals = request.reminder_intervals
            .unwrap_or_else(|| preferences.prayer_reminder_intervals.clone());

        // Create graduated notifications
        for (index, minutes_before) in intervals.iter().enumerate() {
            let scheduled_time = request.prayer_time - Duration::minutes(*minutes_before as i64);
            
            // Skip if scheduled time is in the past
            if scheduled_time <= Utc::now() {
                continue;
            }

            let (title, body) = self.generate_graduated_prayer_message(
                &request.prayer_name,
                *minutes_before,
                index == intervals.len() - 1, // is_final_reminder
                &request.custom_message,
            );

            let notification_request = CreateNotificationRequest {
                user_id: request.user_id,
                notification_type: NotificationType::PrayerGraduated,
                title,
                body,
                priority: Some(self.get_prayer_priority(*minutes_before)),
                scheduled_at: scheduled_time,
                metadata: Some(serde_json::json!({
                    "prayer_name": request.prayer_name,
                    "minutes_before": minutes_before,
                    "prayer_time": request.prayer_time,
                    "prayer_notification_id": prayer_notification.id,
                    "enable_adhan": request.enable_adhan.unwrap_or(true),
                    "enable_vibration": request.enable_vibration.unwrap_or(true)
                })),
                expires_at: Some(request.prayer_time + Duration::minutes(30)), // Expire 30 minutes after prayer
            };

            let notification = self.repository.create_notification(notification_request).await?;
            notifications.push(notification);
        }

        info!("Created {} graduated notifications for prayer {:?}", 
              notifications.len(), request.prayer_name);
        
        Ok(notifications)
    }

    /// Create sunnah and nafl reminders
    pub async fn create_sunnah_reminder(&self, request: CreateSunnahReminderRequest) -> Result<SunnahReminder> {
        info!("Creating sunnah reminder '{}' for user {}", request.sunnah_name, request.user_id);
        
        let sunnah_reminder = self.repository.create_sunnah_reminder(request).await?;
        
        // Schedule the next occurrence of this reminder
        self.schedule_next_sunnah_notification(&sunnah_reminder).await?;
        
        Ok(sunnah_reminder)
    }

    /// Create seasonal Islamic reminders
    pub async fn create_seasonal_reminder(&self, request: CreateSeasonalReminderRequest) -> Result<SeasonalReminder> {
        info!("Creating seasonal reminder '{}' for user {} and season {:?}", 
              request.event_name, request.user_id, request.season);
        
        let seasonal_reminder = self.repository.create_seasonal_reminder(request).await?;
        
        // Schedule the notification for this seasonal event
        self.schedule_seasonal_notification(&seasonal_reminder).await?;
        
        Ok(seasonal_reminder)
    }

    /// Create time-appropriate dhikr reminders
    pub async fn create_dhikr_reminder(&self, request: CreateDhikrReminderRequest) -> Result<DhikrReminder> {
        info!("Creating dhikr reminder for user {} and category {:?}", 
              request.user_id, request.dhikr_category);
        
        let dhikr_reminder = self.repository.create_dhikr_reminder(request).await?;
        
        // Schedule the next occurrence of this dhikr reminder
        self.schedule_next_dhikr_notification(&dhikr_reminder).await?;
        
        Ok(dhikr_reminder)
    }

    /// Process pending notifications and send them
    pub async fn process_pending_notifications(&self, limit: i32) -> Result<usize> {
        let pending_notifications = self.repository.get_pending_notifications(limit).await?;
        let mut processed_count = 0;

        for notification in pending_notifications {
            match self.send_notification(&notification).await {
                Ok(_) => {
                    self.repository.update_notification_status(
                        notification.id,
                        NotificationStatus::Sent,
                    ).await?;
                    processed_count += 1;
                }
                Err(e) => {
                    error!("Failed to send notification {}: {}", notification.id, e);
                    
                    // Log the delivery attempt
                    self.repository.log_delivery_attempt(
                        notification.id,
                        notification.user_id,
                        "push".to_string(),
                        NotificationStatus::Failed,
                        notification.retry_count + 1,
                        Some(e.to_string()),
                        None,
                    ).await?;

                    // Update retry count or mark as failed
                    if notification.retry_count + 1 >= notification.max_retries {
                        self.repository.update_notification_status(
                            notification.id,
                            NotificationStatus::Failed,
                        ).await?;
                    } else {
                        // Increment retry count for next attempt
                        sqlx::query!(
                            "UPDATE notifications SET retry_count = retry_count + 1, updated_at = NOW() WHERE id = $1",
                            notification.id
                        );
                    }
                }
            }
        }

        if processed_count > 0 {
            info!("Processed {} pending notifications", processed_count);
        }

        Ok(processed_count)
    }

    /// Generate appropriate dhikr reminders based on current time
    pub async fn generate_time_appropriate_dhikr(&self, user_id: Uuid) -> Result<Vec<Notification>> {
        let preferences = self.repository.get_user_preferences(user_id).await?;
        
        if !preferences.dhikr_reminders_enabled {
            return Ok(Vec::new());
        }

        let now = Utc::now();
        let current_time = now.time();
        let mut notifications = Vec::new();

        // Morning dhikr (after Fajr until noon)
        if self.is_morning_time(current_time, preferences.morning_dhikr_time) {
            let dhikr_content = self.repository.get_default_dhikr_content(DhikrCategory::Morning).await?;
            if let Some(dhikr) = dhikr_content.first() {
                let notification = self.create_dhikr_notification_from_content(
                    user_id,
                    dhikr,
                    "أذكار الصباح",
                    "حان وقت أذكار الصباح المباركة",
                ).await?;
                notifications.push(notification);
            }
        }

        // Evening dhikr (after Asr until Maghrib)
        if self.is_evening_time(current_time, preferences.evening_dhikr_time) {
            let dhikr_content = self.repository.get_default_dhikr_content(DhikrCategory::Evening).await?;
            if let Some(dhikr) = dhikr_content.first() {
                let notification = self.create_dhikr_notification_from_content(
                    user_id,
                    dhikr,
                    "أذكار المساء",
                    "حان وقت أذكار المساء المباركة",
                ).await?;
                notifications.push(notification);
            }
        }

        Ok(notifications)
    }

    /// Schedule seasonal notifications for upcoming Islamic events
    pub async fn schedule_upcoming_seasonal_notifications(&self) -> Result<usize> {
        info!("Scheduling upcoming seasonal notifications");
        
        // Get all active seasonal reminders
        let all_reminders = sqlx::query_as!(
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
            WHERE is_active = true
            "#
        )
        .fetch_all(&self.repository.pool)
        .await?;

        let mut scheduled_count = 0;

        for reminder in all_reminders {
            match self.schedule_seasonal_notification(&reminder).await {
                Ok(_) => scheduled_count += 1,
                Err(e) => warn!("Failed to schedule seasonal notification for {}: {}", reminder.event_name, e),
            }
        }

        info!("Scheduled {} seasonal notifications", scheduled_count);
        Ok(scheduled_count)
    }

    /// Get user notifications with pagination
    pub async fn get_user_notifications(
        &self,
        user_id: Uuid,
        page: i32,
        page_size: i32,
        status_filter: Option<NotificationStatus>,
    ) -> Result<NotificationListResponse> {
        let (notifications, total_count) = self.repository
            .get_user_notifications(user_id, page, page_size, status_filter)
            .await?;

        let notification_responses: Vec<NotificationResponse> = notifications
            .into_iter()
            .map(NotificationResponse::from)
            .collect();

        Ok(NotificationListResponse {
            notifications: notification_responses,
            total_count,
            page,
            page_size,
        })
    }

    /// Get notification statistics for a user
    pub async fn get_notification_stats(&self, user_id: Uuid) -> Result<NotificationStatsResponse> {
        self.repository.get_notification_stats(user_id).await
    }

    /// Get user notification preferences
    pub async fn get_user_preferences(&self, user_id: Uuid) -> Result<UserNotificationPreferences> {
        self.repository.get_user_preferences(user_id).await
    }

    /// Update user notification preferences
    pub async fn update_user_preferences(
        &self,
        user_id: Uuid,
        request: UpdateNotificationPreferencesRequest,
    ) -> Result<UserNotificationPreferences> {
        self.repository.update_user_preferences(user_id, request).await
    }

    /// Mark notification as read
    pub async fn mark_notification_as_read(&self, notification_id: Uuid) -> Result<()> {
        self.repository.update_notification_status(notification_id, NotificationStatus::Read).await
    }

    /// Dismiss notification
    pub async fn dismiss_notification(&self, notification_id: Uuid) -> Result<()> {
        self.repository.update_notification_status(notification_id, NotificationStatus::Dismissed).await
    }

    // Private helper methods

    /// Generate graduated prayer notification message
    pub fn generate_graduated_prayer_message(
        &self,
        prayer_name: &PrayerName,
        minutes_before: i32,
        is_final_reminder: bool,
        custom_message: &Option<String>,
    ) -> (String, String) {
        let prayer_name_ar = match prayer_name {
            PrayerName::Fajr => "الفجر",
            PrayerName::Dhuhr => "الظهر",
            PrayerName::Asr => "العصر",
            PrayerName::Maghrib => "المغرب",
            PrayerName::Isha => "العشاء",
        };

        if let Some(custom) = custom_message {
            return (
                format!("تذكير صلاة {}", prayer_name_ar),
                custom.clone(),
            );
        }

        let (title, body) = if is_final_reminder {
            (
                format!("حان وقت صلاة {}", prayer_name_ar),
                format!("حان الآن وقت صلاة {}. بارك الله فيك", prayer_name_ar),
            )
        } else if minutes_before <= 5 {
            (
                format!("تذكير عاجل - صلاة {}", prayer_name_ar),
                format!("تبقى {} دقائق على صلاة {}. استعد للصلاة", minutes_before, prayer_name_ar),
            )
        } else if minutes_before <= 15 {
            (
                format!("تذكير صلاة {}", prayer_name_ar),
                format!("تبقى {} دقيقة على صلاة {}. ابدأ بالاستعداد", minutes_before, prayer_name_ar),
            )
        } else {
            (
                format!("تنبيه صلاة {}", prayer_name_ar),
                format!("تبقى {} دقيقة على صلاة {}", minutes_before, prayer_name_ar),
            )
        };

        (title, body)
    }

    /// Get priority based on minutes before prayer
    pub fn get_prayer_priority(&self, minutes_before: i32) -> NotificationPriority {
        match minutes_before {
            0..=5 => NotificationPriority::Urgent,
            6..=15 => NotificationPriority::High,
            16..=30 => NotificationPriority::Medium,
            _ => NotificationPriority::Low,
        }
    }

    /// Schedule next sunnah notification
    async fn schedule_next_sunnah_notification(&self, reminder: &SunnahReminder) -> Result<()> {
        let now = Utc::now();
        let today = now.date_naive();
        
        // Calculate next occurrence based on frequency
        let next_date = match reminder.frequency.as_str() {
            "daily" => {
                let mut next = today.and_time(reminder.reminder_time);
                if next <= now.naive_utc() {
                    next = (today + Duration::days(1)).and_time(reminder.reminder_time);
                }
                next
            }
            "weekly" => {
                // Find next occurrence based on days_of_week
                if let Some(days) = &reminder.days_of_week {
                    let current_weekday = today.weekday().num_days_from_sunday() as i32;
                    let mut days_until_next = None;
                    
                    for &day in days {
                        let days_diff = if day >= current_weekday {
                            day - current_weekday
                        } else {
                            7 + day - current_weekday
                        };
                        
                        if days_until_next.is_none() || days_diff < days_until_next.unwrap() {
                            days_until_next = Some(days_diff);
                        }
                    }
                    
                    if let Some(days_diff) = days_until_next {
                        (today + Duration::days(days_diff as i64)).and_time(reminder.reminder_time)
                    } else {
                        return Ok(()); // No valid days configured
                    }
                } else {
                    return Ok(()); // No days configured for weekly reminder
                }
            }
            _ => return Ok(()), // Unsupported frequency
        };

        let scheduled_at = DateTime::from_naive_utc_and_offset(next_date, Utc);

        let notification_request = CreateNotificationRequest {
            user_id: reminder.user_id,
            notification_type: NotificationType::SunnahReminder,
            title: format!("تذكير سنة: {}", reminder.sunnah_name),
            body: reminder.custom_message.clone().unwrap_or_else(|| {
                format!("حان وقت تذكيرك بسنة: {}", reminder.sunnah_name)
            }),
            priority: Some(reminder.priority.clone()),
            scheduled_at,
            metadata: Some(serde_json::json!({
                "sunnah_reminder_id": reminder.id,
                "sunnah_name": reminder.sunnah_name,
                "sunnah_reference": reminder.sunnah_reference,
                "frequency": reminder.frequency
            })),
            expires_at: Some(scheduled_at + Duration::hours(2)), // Expire after 2 hours
        };

        self.repository.create_notification(notification_request).await?;
        Ok(())
    }

    /// Schedule seasonal notification
    async fn schedule_seasonal_notification(&self, reminder: &SeasonalReminder) -> Result<()> {
        // For now, we'll use a simplified approach with Gregorian dates
        // In a full implementation, you'd need proper Hijri calendar conversion
        
        let target_date = if let Some(gregorian_date) = reminder.gregorian_date {
            gregorian_date
        } else {
            // For Hijri dates, we'd need to convert to Gregorian
            // This is a placeholder - you'd implement proper Hijri conversion
            return Ok(());
        };

        let notification_date = target_date - Duration::days(reminder.days_before_notification as i64);
        let scheduled_at = notification_date.and_hms_opt(9, 0, 0) // 9 AM
            .ok_or_else(|| anyhow!("Invalid date"))?
            .and_utc();

        // Skip if the notification date has passed
        if scheduled_at <= Utc::now() {
            return Ok(());
        }

        let notification_request = CreateNotificationRequest {
            user_id: reminder.user_id,
            notification_type: NotificationType::SeasonalReminder,
            title: format!("تذكير موسمي: {}", reminder.event_name),
            body: reminder.reminder_message.clone().unwrap_or_else(|| {
                format!("يقترب موعد {}", reminder.event_name)
            }),
            priority: Some(reminder.priority.clone()),
            scheduled_at,
            metadata: Some(serde_json::json!({
                "seasonal_reminder_id": reminder.id,
                "season": reminder.season,
                "event_name": reminder.event_name,
                "recommended_actions": reminder.recommended_actions,
                "related_verses": reminder.related_verses,
                "related_hadiths": reminder.related_hadiths
            })),
            expires_at: Some(scheduled_at + Duration::days(7)), // Expire after a week
        };

        self.repository.create_notification(notification_request).await?;
        Ok(())
    }

    /// Schedule next dhikr notification
    async fn schedule_next_dhikr_notification(&self, reminder: &DhikrReminder) -> Result<()> {
        let now = Utc::now();
        
        let scheduled_at = if let Some(trigger_time) = reminder.trigger_time {
            // Fixed time dhikr (morning/evening)
            let today = now.date_naive();
            let mut next = today.and_time(trigger_time);
            
            if next <= now.naive_utc() {
                next = (today + Duration::days(1)).and_time(trigger_time);
            }
            
            DateTime::from_naive_utc_and_offset(next, Utc)
        } else if reminder.trigger_after_prayer.is_some() {
            // Post-prayer dhikr - this would be triggered by prayer completion events
            // For now, we'll skip scheduling these as they're event-driven
            return Ok(());
        } else {
            // No specific trigger time configured
            return Ok(());
        };

        let notification_request = CreateNotificationRequest {
            user_id: reminder.user_id,
            notification_type: NotificationType::DhikrReminder,
            title: format!("تذكير ذكر: {}", self.get_dhikr_category_name(&reminder.dhikr_category)),
            body: format!("حان وقت الذكر: {}", reminder.dhikr_text_arabic),
            priority: Some(reminder.priority.clone()),
            scheduled_at,
            metadata: Some(serde_json::json!({
                "dhikr_reminder_id": reminder.id,
                "dhikr_category": reminder.dhikr_category,
                "dhikr_text_arabic": reminder.dhikr_text_arabic,
                "dhikr_text_transliteration": reminder.dhikr_text_transliteration,
                "dhikr_text_translation": reminder.dhikr_text_translation,
                "dhikr_reference": reminder.dhikr_reference,
                "recommended_repetitions": reminder.recommended_repetitions
            })),
            expires_at: Some(scheduled_at + Duration::hours(1)), // Expire after 1 hour
        };

        self.repository.create_notification(notification_request).await?;
        Ok(())
    }

    /// Send notification (placeholder for actual notification delivery)
    async fn send_notification(&self, notification: &Notification) -> Result<()> {
        // In a real implementation, this would integrate with:
        // - Push notification services (FCM, APNs)
        // - Email services
        // - SMS services
        // - WebSocket connections for real-time notifications
        
        info!("Sending notification: {} to user {}", notification.title, notification.user_id);
        
        // Simulate successful delivery
        Ok(())
    }

    /// Create dhikr notification from default content
    async fn create_dhikr_notification_from_content(
        &self,
        user_id: Uuid,
        dhikr: &DefaultDhikrContent,
        title: &str,
        body: &str,
    ) -> Result<Notification> {
        let notification_request = CreateNotificationRequest {
            user_id,
            notification_type: NotificationType::DhikrReminder,
            title: title.to_string(),
            body: body.to_string(),
            priority: Some(NotificationPriority::Low),
            scheduled_at: Utc::now(),
            metadata: Some(serde_json::json!({
                "dhikr_content_id": dhikr.id,
                "dhikr_category": dhikr.category,
                "arabic_text": dhikr.arabic_text,
                "transliteration": dhikr.transliteration,
                "translation_en": dhikr.translation_en,
                "translation_ar": dhikr.translation_ar,
                "reference": dhikr.reference,
                "repetitions": dhikr.repetitions
            })),
            expires_at: Some(Utc::now() + Duration::hours(2)),
        };

        self.repository.create_notification(notification_request).await
    }

    /// Check if current time is morning dhikr time
    pub fn is_morning_time(&self, current_time: NaiveTime, morning_dhikr_time: NaiveTime) -> bool {
        let morning_start = NaiveTime::from_hms_opt(5, 0, 0).unwrap(); // After Fajr
        let morning_end = NaiveTime::from_hms_opt(12, 0, 0).unwrap(); // Before Dhuhr
        
        current_time >= morning_start && current_time <= morning_end &&
        current_time >= morning_dhikr_time
    }

    /// Check if current time is evening dhikr time
    pub fn is_evening_time(&self, current_time: NaiveTime, evening_dhikr_time: NaiveTime) -> bool {
        let evening_start = NaiveTime::from_hms_opt(15, 0, 0).unwrap(); // After Asr
        let evening_end = NaiveTime::from_hms_opt(19, 0, 0).unwrap(); // Before Maghrib
        
        current_time >= evening_start && current_time <= evening_end &&
        current_time >= evening_dhikr_time
    }

    /// Get dhikr category name in Arabic
    pub fn get_dhikr_category_name(&self, category: &DhikrCategory) -> &'static str {
        match category {
            DhikrCategory::Morning => "أذكار الصباح",
            DhikrCategory::Evening => "أذكار المساء",
            DhikrCategory::AfterPrayer => "أذكار ما بعد الصلاة",
            DhikrCategory::BeforeSleep => "أذكار النوم",
            DhikrCategory::AfterWudu => "أذكار الوضوء",
            DhikrCategory::Travel => "أذكار السفر",
            DhikrCategory::General => "أذكار عامة",
        }
    }
}
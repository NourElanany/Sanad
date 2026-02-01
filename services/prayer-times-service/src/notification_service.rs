use chrono::{DateTime, Utc, Duration, NaiveTime, Datelike};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use shared::{SanadResult, SanadError, PrayerTimes, Location};
use crate::models::{
    UserPrayerPreferences, PrayerNotificationSettings, NotificationScheduleRequest,
    ScheduledNotification, IslamicEventDetails,
};

/// Notification types for prayer times and Islamic events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrayerNotificationType {
    PrayerReminder,
    PrayerGraduated,
    IslamicEvent,
    FridayReminder,
    SurahKahfReminder,
}

/// Priority levels for notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Notification delivery status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationStatus {
    Pending,
    Sent,
    Delivered,
    Read,
    Dismissed,
    Failed,
}

/// Prayer notification with customizable settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerNotification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub prayer_name: String,
    pub prayer_time: DateTime<Utc>,
    pub notification_time: DateTime<Utc>,
    pub notification_type: PrayerNotificationType,
    pub priority: NotificationPriority,
    pub title_arabic: String,
    pub title_english: String,
    pub message_arabic: String,
    pub message_english: String,
    pub is_graduated: bool,
    pub minutes_before: i32,
    pub status: NotificationStatus,
    pub metadata: String, // JSON string instead of serde_json::Value
    pub created_at: DateTime<Utc>,
}

/// Islamic event notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicEventNotification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event: IslamicEventDetails,
    pub notification_time: DateTime<Utc>,
    pub priority: NotificationPriority,
    pub title_arabic: String,
    pub title_english: String,
    pub message_arabic: String,
    pub message_english: String,
    pub status: NotificationStatus,
    pub created_at: DateTime<Utc>,
}

/// User notification preferences for prayer times
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub user_id: Uuid,
    pub notifications_enabled: bool,
    pub quiet_hours_start: NaiveTime,
    pub quiet_hours_end: NaiveTime,
    pub prayer_settings: Vec<PrayerNotificationSettings>,
    pub islamic_events_enabled: bool,
    pub friday_reminders_enabled: bool,
    pub surah_kahf_reminder_enabled: bool,
    pub graduated_notifications_enabled: bool,
    pub default_intervals: Vec<i32>, // Default: [30, 15, 5] minutes
    pub language_preference: String, // "ar" or "en"
}

/// Prayer times notification service
pub struct PrayerNotificationService {
    // In a real implementation, this would have HTTP client to communicate with notification-service
    notification_service_url: String,
}

impl PrayerNotificationService {
    pub fn new(notification_service_url: String) -> Self {
        Self {
            notification_service_url,
        }
    }

    /// Schedule prayer notifications for a user based on prayer times
    pub async fn schedule_prayer_notifications(
        &self,
        user_id: Uuid,
        prayer_times: &PrayerTimes,
        preferences: &NotificationPreferences,
    ) -> SanadResult<Vec<PrayerNotification>> {
        if !preferences.notifications_enabled {
            return Ok(Vec::new());
        }

        let mut notifications = Vec::new();

        // Schedule notifications for each prayer
        for prayer_setting in &preferences.prayer_settings {
            if !prayer_setting.enabled {
                continue;
            }

            let prayer_time = self.get_prayer_time_by_name(&prayer_setting.prayer_name, prayer_times)?;
            
            if preferences.graduated_notifications_enabled && prayer_setting.graduated_enabled {
                // Create graduated notifications
                let intervals = if prayer_setting.graduated_intervals.is_empty() {
                    &preferences.default_intervals
                } else {
                    &prayer_setting.graduated_intervals
                };

                for (index, &minutes_before) in intervals.iter().enumerate() {
                    let notification_time = prayer_time - Duration::minutes(minutes_before as i64);
                    
                    // Skip if notification time is in the past
                    if notification_time <= Utc::now() {
                        continue;
                    }

                    // Skip if notification falls in quiet hours
                    if self.is_in_quiet_hours(notification_time, preferences) {
                        continue;
                    }

                    let is_final = index == intervals.len() - 1;
                    let priority = self.get_prayer_priority(minutes_before, is_final);
                    
                    let (title_ar, title_en, message_ar, message_en) = self.generate_prayer_message(
                        &prayer_setting.prayer_name,
                        minutes_before,
                        is_final,
                        &preferences.language_preference,
                    );

                    let notification = PrayerNotification {
                        id: Uuid::new_v4(),
                        user_id,
                        prayer_name: prayer_setting.prayer_name.clone(),
                        prayer_time,
                        notification_time,
                        notification_type: if is_final {
                            PrayerNotificationType::PrayerReminder
                        } else {
                            PrayerNotificationType::PrayerGraduated
                        },
                        priority,
                        title_arabic: title_ar,
                        title_english: title_en,
                        message_arabic: message_ar,
                        message_english: message_en,
                        is_graduated: true,
                        minutes_before,
                        status: NotificationStatus::Pending,
                        metadata: format!(r#"{{"prayer_time":"{}","location":{{"latitude":{},"longitude":{}}},"calculation_method":"{}","is_final_reminder":{}}}"#,
                            prayer_time.to_rfc3339(),
                            prayer_times.location.latitude,
                            prayer_times.location.longitude,
                            format!("{:?}", prayer_times.calculation_method),
                            is_final
                        ),
                        created_at: Utc::now(),
                    };

                    notifications.push(notification);
                }
            } else {
                // Create single notification
                let notification_time = prayer_time - Duration::minutes(prayer_setting.minutes_before as i64);
                
                if notification_time > Utc::now() && !self.is_in_quiet_hours(notification_time, preferences) {
                    let (title_ar, title_en, message_ar, message_en) = self.generate_prayer_message(
                        &prayer_setting.prayer_name,
                        prayer_setting.minutes_before,
                        true,
                        &preferences.language_preference,
                    );

                    let notification = PrayerNotification {
                        id: Uuid::new_v4(),
                        user_id,
                        prayer_name: prayer_setting.prayer_name.clone(),
                        prayer_time,
                        notification_time,
                        notification_type: PrayerNotificationType::PrayerReminder,
                        priority: self.get_prayer_priority(prayer_setting.minutes_before, true),
                        title_arabic: title_ar,
                        title_english: title_en,
                        message_arabic: message_ar,
                        message_english: message_en,
                        is_graduated: false,
                        minutes_before: prayer_setting.minutes_before,
                        status: NotificationStatus::Pending,
                        metadata: format!(r#"{{"prayer_time":"{}","location":{{"latitude":{},"longitude":{}}},"calculation_method":"{}"}}"#,
                            prayer_time.to_rfc3339(),
                            prayer_times.location.latitude,
                            prayer_times.location.longitude,
                            format!("{:?}", prayer_times.calculation_method)
                        ),
                        created_at: Utc::now(),
                    };

                    notifications.push(notification);
                }
            }
        }

        // Schedule Friday-specific reminders
        if preferences.friday_reminders_enabled {
            notifications.extend(self.schedule_friday_reminders(user_id, prayer_times, preferences).await?);
        }

        Ok(notifications)
    }

    /// Schedule Islamic event notifications
    pub async fn schedule_islamic_event_notifications(
        &self,
        user_id: Uuid,
        events: &[IslamicEventDetails],
        preferences: &NotificationPreferences,
    ) -> SanadResult<Vec<IslamicEventNotification>> {
        if !preferences.islamic_events_enabled {
            return Ok(Vec::new());
        }

        let mut notifications = Vec::new();

        for event in events {
            if !event.notification_enabled {
                continue;
            }

            // Calculate notification time (typically 1 day before the event)
            let event_date = self.calculate_event_date(event)?;
            let notification_time = event_date - Duration::days(1);
            
            // Skip if notification time is in the past
            if notification_time <= Utc::now() {
                continue;
            }

            // Skip if notification falls in quiet hours
            if self.is_in_quiet_hours(notification_time, preferences) {
                continue;
            }

            let priority = match event.importance_level {
                5 => NotificationPriority::Urgent,
                4 => NotificationPriority::High,
                3 => NotificationPriority::Medium,
                _ => NotificationPriority::Low,
            };

            let (title_ar, title_en, message_ar, message_en) = self.generate_event_message(
                event,
                &preferences.language_preference,
            );

            let notification = IslamicEventNotification {
                id: Uuid::new_v4(),
                user_id,
                event: event.clone(),
                notification_time,
                priority,
                title_arabic: title_ar,
                title_english: title_en,
                message_arabic: message_ar,
                message_english: message_en,
                status: NotificationStatus::Pending,
                created_at: Utc::now(),
            };

            notifications.push(notification);
        }

        Ok(notifications)
    }

    /// Schedule Friday-specific reminders (Surah Al-Kahf, Jumu'ah prayer)
    async fn schedule_friday_reminders(
        &self,
        user_id: Uuid,
        prayer_times: &PrayerTimes,
        preferences: &NotificationPreferences,
    ) -> SanadResult<Vec<PrayerNotification>> {
        let mut notifications = Vec::new();

        // Check if today is Friday
        let today = Utc::now().date_naive();
        if today.weekday() != chrono::Weekday::Fri {
            return Ok(notifications);
        }

        // Surah Al-Kahf reminder (morning)
        if preferences.surah_kahf_reminder_enabled {
            let reminder_time = today.and_hms_opt(9, 0, 0)
                .ok_or_else(|| SanadError::Internal("Invalid time".to_string()))?
                .and_utc();

            if reminder_time > Utc::now() && !self.is_in_quiet_hours(reminder_time, preferences) {
                let notification = PrayerNotification {
                    id: Uuid::new_v4(),
                    user_id,
                    prayer_name: "friday_surah_kahf".to_string(),
                    prayer_time: reminder_time,
                    notification_time: reminder_time,
                    notification_type: PrayerNotificationType::SurahKahfReminder,
                    priority: NotificationPriority::Medium,
                    title_arabic: "تذكير قراءة سورة الكهف".to_string(),
                    title_english: "Surah Al-Kahf Reminder".to_string(),
                    message_arabic: "اليوم يوم الجمعة، لا تنس قراءة سورة الكهف المباركة".to_string(),
                    message_english: "Today is Friday, don't forget to read the blessed Surah Al-Kahf".to_string(),
                    is_graduated: false,
                    minutes_before: 0,
                    status: NotificationStatus::Pending,
                    metadata: format!(r#"{{"reminder_type":"surah_kahf","day":"friday"}}"#),
                    created_at: Utc::now(),
                };

                notifications.push(notification);
            }
        }

        // Jumu'ah prayer reminder (30 minutes before Dhuhr)
        let jumu_ah_time = prayer_times.dhuhr - Duration::minutes(30);
        if jumu_ah_time > Utc::now() && !self.is_in_quiet_hours(jumu_ah_time, preferences) {
            let notification = PrayerNotification {
                id: Uuid::new_v4(),
                user_id,
                prayer_name: "jumu_ah".to_string(),
                prayer_time: prayer_times.dhuhr,
                notification_time: jumu_ah_time,
                notification_type: PrayerNotificationType::FridayReminder,
                priority: NotificationPriority::High,
                title_arabic: "تذكير صلاة الجمعة".to_string(),
                title_english: "Jumu'ah Prayer Reminder".to_string(),
                message_arabic: "حان وقت التوجه لصلاة الجمعة المباركة".to_string(),
                message_english: "Time to head to the blessed Jumu'ah prayer".to_string(),
                is_graduated: false,
                minutes_before: 30,
                status: NotificationStatus::Pending,
                metadata: format!(r#"{{"prayer_type":"jumu_ah","original_prayer":"dhuhr","day":"friday"}}"#),
                created_at: Utc::now(),
            };

            notifications.push(notification);
        }

        Ok(notifications)
    }

    /// Get prayer time by name
    pub fn get_prayer_time_by_name(&self, prayer_name: &str, prayer_times: &PrayerTimes) -> SanadResult<DateTime<Utc>> {
        match prayer_name.to_lowercase().as_str() {
            "fajr" => Ok(prayer_times.fajr),
            "dhuhr" => Ok(prayer_times.dhuhr),
            "asr" => Ok(prayer_times.asr),
            "maghrib" => Ok(prayer_times.maghrib),
            "isha" => Ok(prayer_times.isha),
            "sunrise" => Ok(prayer_times.sunrise),
            _ => Err(SanadError::Validation(format!("Unknown prayer name: {}", prayer_name))),
        }
    }

    /// Check if notification time falls within quiet hours
    pub fn is_in_quiet_hours(&self, notification_time: DateTime<Utc>, preferences: &NotificationPreferences) -> bool {
        let time = notification_time.time();
        let start = preferences.quiet_hours_start;
        let end = preferences.quiet_hours_end;

        if start <= end {
            // Same day quiet hours (e.g., 22:00 - 06:00)
            time >= start && time <= end
        } else {
            // Overnight quiet hours (e.g., 22:00 - 06:00 next day)
            time >= start || time <= end
        }
    }

    /// Get notification priority based on minutes before prayer
    pub fn get_prayer_priority(&self, minutes_before: i32, is_final: bool) -> NotificationPriority {
        if is_final {
            NotificationPriority::Urgent
        } else {
            match minutes_before {
                0..=5 => NotificationPriority::Urgent,
                6..=15 => NotificationPriority::High,
                16..=30 => NotificationPriority::Medium,
                _ => NotificationPriority::Low,
            }
        }
    }

    /// Generate prayer notification message
    pub fn generate_prayer_message(
        &self,
        prayer_name: &str,
        minutes_before: i32,
        is_final: bool,
        language: &str,
    ) -> (String, String, String, String) {
        let prayer_name_ar = match prayer_name.to_lowercase().as_str() {
            "fajr" => "الفجر",
            "dhuhr" => "الظهر",
            "asr" => "العصر",
            "maghrib" => "المغرب",
            "isha" => "العشاء",
            "sunrise" => "الشروق",
            _ => prayer_name,
        };

        let prayer_name_en = match prayer_name.to_lowercase().as_str() {
            "fajr" => "Fajr",
            "dhuhr" => "Dhuhr",
            "asr" => "Asr",
            "maghrib" => "Maghrib",
            "isha" => "Isha",
            "sunrise" => "Sunrise",
            _ => prayer_name,
        };

        if is_final || minutes_before == 0 {
            (
                format!("حان وقت صلاة {}", prayer_name_ar),
                format!("{} Prayer Time", prayer_name_en),
                format!("حان الآن وقت صلاة {}. بارك الله فيك", prayer_name_ar),
                format!("It's now time for {} prayer. May Allah bless you", prayer_name_en),
            )
        } else if minutes_before <= 5 {
            (
                format!("تذكير عاجل - صلاة {}", prayer_name_ar),
                format!("Urgent Reminder - {} Prayer", prayer_name_en),
                format!("تبقى {} دقائق على صلاة {}. استعد للصلاة", minutes_before, prayer_name_ar),
                format!("{} minutes left for {} prayer. Get ready to pray", minutes_before, prayer_name_en),
            )
        } else if minutes_before <= 15 {
            (
                format!("تذكير صلاة {}", prayer_name_ar),
                format!("{} Prayer Reminder", prayer_name_en),
                format!("تبقى {} دقيقة على صلاة {}. ابدأ بالاستعداد", minutes_before, prayer_name_ar),
                format!("{} minutes left for {} prayer. Start preparing", minutes_before, prayer_name_en),
            )
        } else {
            (
                format!("تنبيه صلاة {}", prayer_name_ar),
                format!("{} Prayer Alert", prayer_name_en),
                format!("تبقى {} دقيقة على صلاة {}", minutes_before, prayer_name_ar),
                format!("{} minutes left for {} prayer", minutes_before, prayer_name_en),
            )
        }
    }

    /// Generate Islamic event notification message
    fn generate_event_message(
        &self,
        event: &IslamicEventDetails,
        language: &str,
    ) -> (String, String, String, String) {
        let title_ar = format!("مناسبة إسلامية: {}", event.name_arabic);
        let title_en = format!("Islamic Event: {}", event.name_english);
        
        let message_ar = if let Some(desc) = &event.description_arabic {
            format!("غداً يصادف {}. {}", event.name_arabic, desc)
        } else {
            format!("غداً يصادف {}", event.name_arabic)
        };
        
        let message_en = if let Some(desc) = &event.description_english {
            format!("Tomorrow is {}. {}", event.name_english, desc)
        } else {
            format!("Tomorrow is {}", event.name_english)
        };

        (title_ar, title_en, message_ar, message_en)
    }

    /// Calculate event date from Islamic event details
    fn calculate_event_date(&self, event: &IslamicEventDetails) -> SanadResult<DateTime<Utc>> {
        // For now, we'll use a simplified approach
        // In a real implementation, you'd need proper Hijri calendar conversion
        
        if let (Some(month), Some(day)) = (event.hijri_month, event.hijri_day) {
            // This is a placeholder - you'd need to convert Hijri to Gregorian
            // For now, we'll just use today + 1 day as an example
            Ok(Utc::now() + Duration::days(1))
        } else {
            Err(SanadError::Validation("Event date not properly specified".to_string()))
        }
    }

    /// Create default notification preferences for a new user
    pub fn create_default_preferences(user_id: Uuid) -> NotificationPreferences {
        NotificationPreferences {
            user_id,
            notifications_enabled: true,
            quiet_hours_start: NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
            quiet_hours_end: NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
            prayer_settings: vec![
                PrayerNotificationSettings {
                    prayer_name: "fajr".to_string(),
                    enabled: true,
                    minutes_before: 15,
                    graduated_enabled: true,
                    graduated_intervals: vec![30, 15, 5],
                },
                PrayerNotificationSettings {
                    prayer_name: "dhuhr".to_string(),
                    enabled: true,
                    minutes_before: 15,
                    graduated_enabled: true,
                    graduated_intervals: vec![30, 15, 5],
                },
                PrayerNotificationSettings {
                    prayer_name: "asr".to_string(),
                    enabled: true,
                    minutes_before: 15,
                    graduated_enabled: true,
                    graduated_intervals: vec![30, 15, 5],
                },
                PrayerNotificationSettings {
                    prayer_name: "maghrib".to_string(),
                    enabled: true,
                    minutes_before: 15,
                    graduated_enabled: true,
                    graduated_intervals: vec![30, 15, 5],
                },
                PrayerNotificationSettings {
                    prayer_name: "isha".to_string(),
                    enabled: true,
                    minutes_before: 15,
                    graduated_enabled: true,
                    graduated_intervals: vec![30, 15, 5],
                },
            ],
            islamic_events_enabled: true,
            friday_reminders_enabled: true,
            surah_kahf_reminder_enabled: true,
            graduated_notifications_enabled: true,
            default_intervals: vec![30, 15, 5],
            language_preference: "ar".to_string(),
        }
    }
}

impl NotificationPreferences {
    /// Create default notification preferences for a new user
    pub fn create_default_preferences(user_id: Uuid) -> NotificationPreferences {
        NotificationPreferences {
            user_id,
            notifications_enabled: true,
            quiet_hours_start: NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
            quiet_hours_end: NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
            prayer_settings: vec![
                PrayerNotificationSettings {
                    prayer_name: "fajr".to_string(),
                    enabled: true,
                    minutes_before: 15,
                    graduated_enabled: true,
                    graduated_intervals: vec![30, 15, 5],
                },
                PrayerNotificationSettings {
                    prayer_name: "dhuhr".to_string(),
                    enabled: true,
                    minutes_before: 15,
                    graduated_enabled: true,
                    graduated_intervals: vec![30, 15, 5],
                },
                PrayerNotificationSettings {
                    prayer_name: "asr".to_string(),
                    enabled: true,
                    minutes_before: 15,
                    graduated_enabled: true,
                    graduated_intervals: vec![30, 15, 5],
                },
                PrayerNotificationSettings {
                    prayer_name: "maghrib".to_string(),
                    enabled: true,
                    minutes_before: 15,
                    graduated_enabled: true,
                    graduated_intervals: vec![30, 15, 5],
                },
                PrayerNotificationSettings {
                    prayer_name: "isha".to_string(),
                    enabled: true,
                    minutes_before: 15,
                    graduated_enabled: true,
                    graduated_intervals: vec![30, 15, 5],
                },
            ],
            islamic_events_enabled: true,
            friday_reminders_enabled: true,
            surah_kahf_reminder_enabled: true,
            graduated_notifications_enabled: true,
            default_intervals: vec![30, 15, 5],
            language_preference: "ar".to_string(),
        }
    }
}

impl Default for NotificationPreferences {
    fn default() -> Self {
        Self::create_default_preferences(Uuid::new_v4())
    }
}
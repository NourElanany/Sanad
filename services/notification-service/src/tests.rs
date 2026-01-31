use crate::models::*;
use crate::repository::NotificationRepository;
use crate::service::NotificationService;
use chrono::{DateTime, Utc, Duration, NaiveTime};
use sqlx::PgPool;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create test database pool
    async fn create_test_pool() -> PgPool {
        // In a real test environment, you'd use a test database
        // For now, this is a placeholder
        todo!("Implement test database setup")
    }

    #[tokio::test]
    async fn test_create_graduated_prayer_notifications() {
        let pool = create_test_pool().await;
        let repository = NotificationRepository::new(pool);
        let service = NotificationService::new(repository);

        let user_id = Uuid::new_v4();
        let prayer_time = Utc::now() + Duration::hours(1);

        let request = CreatePrayerNotificationRequest {
            user_id,
            prayer_name: PrayerName::Fajr,
            prayer_time,
            enable_graduated: Some(true),
            reminder_intervals: Some(vec![30, 15, 5]),
            latitude: Some(24.7136),
            longitude: Some(46.6753),
            timezone: Some("Asia/Riyadh".to_string()),
            enable_adhan: Some(true),
            enable_vibration: Some(true),
            custom_message: None,
        };

        let notifications = service.create_graduated_prayer_notifications(request).await.unwrap();

        // Should create 3 notifications (30, 15, 5 minutes before)
        assert_eq!(notifications.len(), 3);

        // Check that notifications are scheduled at correct times
        let expected_times = vec![
            prayer_time - Duration::minutes(30),
            prayer_time - Duration::minutes(15),
            prayer_time - Duration::minutes(5),
        ];

        for (i, notification) in notifications.iter().enumerate() {
            assert_eq!(notification.scheduled_at, expected_times[i]);
            assert_eq!(notification.notification_type, NotificationType::PrayerGraduated);
            assert_eq!(notification.user_id, user_id);
        }
    }

    #[tokio::test]
    async fn test_create_sunnah_reminder() {
        let pool = create_test_pool().await;
        let repository = NotificationRepository::new(pool);
        let service = NotificationService::new(repository);

        let user_id = Uuid::new_v4();
        let reminder_time = NaiveTime::from_hms_opt(6, 0, 0).unwrap();

        let request = CreateSunnahReminderRequest {
            user_id,
            sunnah_name: "قراءة سورة الكهف يوم الجمعة".to_string(),
            sunnah_description: Some("قراءة سورة الكهف في يوم الجمعة سنة مستحبة".to_string()),
            sunnah_reference: Some("صحيح الجامع".to_string()),
            reminder_time,
            frequency: Some("weekly".to_string()),
            days_of_week: Some(vec![5]), // Friday
            priority: Some(NotificationPriority::Medium),
            custom_message: None,
        };

        let reminder = service.create_sunnah_reminder(request).await.unwrap();

        assert_eq!(reminder.user_id, user_id);
        assert_eq!(reminder.sunnah_name, "قراءة سورة الكهف يوم الجمعة");
        assert_eq!(reminder.frequency, "weekly");
        assert_eq!(reminder.days_of_week, Some(vec![5]));
        assert_eq!(reminder.priority, NotificationPriority::Medium);
        assert!(reminder.is_active);
    }

    #[tokio::test]
    async fn test_create_seasonal_reminder() {
        let pool = create_test_pool().await;
        let repository = NotificationRepository::new(pool);
        let service = NotificationService::new(repository);

        let user_id = Uuid::new_v4();

        let request = CreateSeasonalReminderRequest {
            user_id,
            season: IslamicSeason::Ramadan,
            event_name: "بداية شهر رمضان المبارك".to_string(),
            event_description: Some("شهر الصيام والقيام والقرآن".to_string()),
            hijri_month: Some(9), // Ramadan
            hijri_day: Some(1),
            gregorian_date: None,
            days_before_notification: Some(3),
            priority: Some(NotificationPriority::High),
            reminder_message: Some("استعد لاستقبال شهر رمضان المبارك".to_string()),
            recommended_actions: Some(vec![
                "الإكثار من الدعاء".to_string(),
                "قراءة القرآن".to_string(),
                "الصدقة".to_string(),
            ]),
            related_verses: Some(vec!["يَا أَيُّهَا الَّذِينَ آمَنُوا كُتِبَ عَلَيْكُمُ الصِّيَامُ".to_string()]),
            related_hadiths: Some(vec!["من صام رمضان إيماناً واحتساباً غفر له ما تقدم من ذنبه".to_string()]),
        };

        let reminder = service.create_seasonal_reminder(request).await.unwrap();

        assert_eq!(reminder.user_id, user_id);
        assert_eq!(reminder.season, IslamicSeason::Ramadan);
        assert_eq!(reminder.event_name, "بداية شهر رمضان المبارك");
        assert_eq!(reminder.hijri_month, Some(9));
        assert_eq!(reminder.hijri_day, Some(1));
        assert_eq!(reminder.days_before_notification, 3);
        assert_eq!(reminder.priority, NotificationPriority::High);
        assert!(reminder.is_active);
    }

    #[tokio::test]
    async fn test_create_dhikr_reminder() {
        let pool = create_test_pool().await;
        let repository = NotificationRepository::new(pool);
        let service = NotificationService::new(repository);

        let user_id = Uuid::new_v4();
        let trigger_time = NaiveTime::from_hms_opt(6, 30, 0).unwrap();

        let request = CreateDhikrReminderRequest {
            user_id,
            dhikr_category: DhikrCategory::Morning,
            dhikr_text_arabic: "سُبْحَانَ اللَّهِ وَبِحَمْدِهِ".to_string(),
            dhikr_text_transliteration: Some("Subhan'allahi wa bihamdihi".to_string()),
            dhikr_text_translation: Some("Glory is to Allah and praise is to Him".to_string()),
            dhikr_reference: Some("صحيح البخاري".to_string()),
            trigger_time: Some(trigger_time),
            trigger_after_prayer: None,
            trigger_condition: None,
            frequency: Some("daily".to_string()),
            priority: Some(NotificationPriority::Low),
            recommended_repetitions: Some(100),
            track_completion: Some(true),
        };

        let reminder = service.create_dhikr_reminder(request).await.unwrap();

        assert_eq!(reminder.user_id, user_id);
        assert_eq!(reminder.dhikr_category, DhikrCategory::Morning);
        assert_eq!(reminder.dhikr_text_arabic, "سُبْحَانَ اللَّهِ وَبِحَمْدِهِ");
        assert_eq!(reminder.trigger_time, Some(trigger_time));
        assert_eq!(reminder.frequency, "daily");
        assert_eq!(reminder.priority, NotificationPriority::Low);
        assert_eq!(reminder.recommended_repetitions, 100);
        assert!(reminder.track_completion);
        assert!(reminder.is_active);
    }

    #[tokio::test]
    async fn test_notification_priority_assignment() {
        let pool = create_test_pool().await;
        let repository = NotificationRepository::new(pool);
        let service = NotificationService::new(repository);

        // Test priority assignment based on minutes before prayer
        assert_eq!(service.get_prayer_priority(0), NotificationPriority::Urgent);
        assert_eq!(service.get_prayer_priority(5), NotificationPriority::Urgent);
        assert_eq!(service.get_prayer_priority(10), NotificationPriority::High);
        assert_eq!(service.get_prayer_priority(15), NotificationPriority::High);
        assert_eq!(service.get_prayer_priority(20), NotificationPriority::Medium);
        assert_eq!(service.get_prayer_priority(30), NotificationPriority::Medium);
        assert_eq!(service.get_prayer_priority(60), NotificationPriority::Low);
    }

    #[tokio::test]
    async fn test_prayer_message_generation() {
        let pool = create_test_pool().await;
        let repository = NotificationRepository::new(pool);
        let service = NotificationService::new(repository);

        // Test final reminder message
        let (title, body) = service.generate_graduated_prayer_message(
            &PrayerName::Fajr,
            0,
            true,
            &None,
        );
        assert_eq!(title, "حان وقت صلاة الفجر");
        assert!(body.contains("حان الآن وقت صلاة الفجر"));

        // Test urgent reminder message
        let (title, body) = service.generate_graduated_prayer_message(
            &PrayerName::Dhuhr,
            5,
            false,
            &None,
        );
        assert_eq!(title, "تذكير عاجل - صلاة الظهر");
        assert!(body.contains("تبقى 5 دقائق"));

        // Test regular reminder message
        let (title, body) = service.generate_graduated_prayer_message(
            &PrayerName::Asr,
            30,
            false,
            &None,
        );
        assert_eq!(title, "تنبيه صلاة العصر");
        assert!(body.contains("تبقى 30 دقيقة"));

        // Test custom message
        let custom_message = Some("تذكير خاص بالصلاة".to_string());
        let (title, body) = service.generate_graduated_prayer_message(
            &PrayerName::Maghrib,
            15,
            false,
            &custom_message,
        );
        assert_eq!(title, "تذكير صلاة المغرب");
        assert_eq!(body, "تذكير خاص بالصلاة");
    }

    #[tokio::test]
    async fn test_time_appropriate_dhikr_detection() {
        let pool = create_test_pool().await;
        let repository = NotificationRepository::new(pool);
        let service = NotificationService::new(repository);

        // Test morning time detection
        let morning_time = NaiveTime::from_hms_opt(7, 0, 0).unwrap();
        let morning_dhikr_time = NaiveTime::from_hms_opt(6, 0, 0).unwrap();
        assert!(service.is_morning_time(morning_time, morning_dhikr_time));

        let too_early = NaiveTime::from_hms_opt(4, 0, 0).unwrap();
        assert!(!service.is_morning_time(too_early, morning_dhikr_time));

        let too_late = NaiveTime::from_hms_opt(13, 0, 0).unwrap();
        assert!(!service.is_morning_time(too_late, morning_dhikr_time));

        // Test evening time detection
        let evening_time = NaiveTime::from_hms_opt(17, 0, 0).unwrap();
        let evening_dhikr_time = NaiveTime::from_hms_opt(16, 0, 0).unwrap();
        assert!(service.is_evening_time(evening_time, evening_dhikr_time));

        let too_early_evening = NaiveTime::from_hms_opt(14, 0, 0).unwrap();
        assert!(!service.is_evening_time(too_early_evening, evening_dhikr_time));

        let too_late_evening = NaiveTime::from_hms_opt(20, 0, 0).unwrap();
        assert!(!service.is_evening_time(too_late_evening, evening_dhikr_time));
    }

    #[tokio::test]
    async fn test_dhikr_category_names() {
        let pool = create_test_pool().await;
        let repository = NotificationRepository::new(pool);
        let service = NotificationService::new(repository);

        assert_eq!(service.get_dhikr_category_name(&DhikrCategory::Morning), "أذكار الصباح");
        assert_eq!(service.get_dhikr_category_name(&DhikrCategory::Evening), "أذكار المساء");
        assert_eq!(service.get_dhikr_category_name(&DhikrCategory::AfterPrayer), "أذكار ما بعد الصلاة");
        assert_eq!(service.get_dhikr_category_name(&DhikrCategory::BeforeSleep), "أذكار النوم");
        assert_eq!(service.get_dhikr_category_name(&DhikrCategory::AfterWudu), "أذكار الوضوء");
        assert_eq!(service.get_dhikr_category_name(&DhikrCategory::Travel), "أذكار السفر");
        assert_eq!(service.get_dhikr_category_name(&DhikrCategory::General), "أذكار عامة");
    }

    #[tokio::test]
    async fn test_notification_expiration() {
        let pool = create_test_pool().await;
        let repository = NotificationRepository::new(pool);

        let user_id = Uuid::new_v4();
        let now = Utc::now();

        // Create notification that expires in 1 hour
        let request = CreateNotificationRequest {
            user_id,
            notification_type: NotificationType::PrayerReminder,
            title: "Test Notification".to_string(),
            body: "This is a test notification".to_string(),
            priority: Some(NotificationPriority::Medium),
            scheduled_at: now,
            metadata: Some(serde_json::json!({})),
            expires_at: Some(now + Duration::hours(1)),
        };

        let notification = repository.create_notification(request).await.unwrap();

        assert_eq!(notification.expires_at, Some(now + Duration::hours(1)));
        assert_eq!(notification.retry_count, 0);
        assert_eq!(notification.max_retries, 3);
    }

    #[tokio::test]
    async fn test_user_preferences_defaults() {
        let pool = create_test_pool().await;
        let repository = NotificationRepository::new(pool);

        let user_id = Uuid::new_v4();

        // Get preferences for new user (should create defaults)
        let preferences = repository.get_user_preferences(user_id).await.unwrap();

        assert_eq!(preferences.user_id, user_id);
        assert!(preferences.notifications_enabled);
        assert!(preferences.prayer_notifications_enabled);
        assert!(preferences.prayer_graduated_enabled);
        assert_eq!(preferences.prayer_reminder_intervals, vec![30, 15, 5]);
        assert!(preferences.sunnah_reminders_enabled);
        assert!(preferences.dhikr_reminders_enabled);
        assert!(preferences.seasonal_reminders_enabled);
        assert!(preferences.push_notifications);
        assert!(!preferences.email_notifications);
        assert!(!preferences.sms_notifications);
    }
}
use chrono::{Utc, Duration, NaiveTime};
use uuid::Uuid;
use shared::{Location, CalculationMethod, PrayerTimes};
use crate::notification_service::{PrayerNotificationService, NotificationPreferences};
use crate::models::PrayerNotificationSettings;

#[test]
fn test_prayer_message_generation() {
    let notification_service = PrayerNotificationService::new("http://localhost:8080".to_string());
    
    // Test final reminder message
    let (title_ar, title_en, message_ar, message_en) = notification_service
        .generate_prayer_message("fajr", 0, true, "ar");
    
    assert!(title_ar.contains("حان وقت"));
    assert!(title_ar.contains("الفجر"));
    assert!(title_en.contains("Fajr"));
    assert!(title_en.contains("Time"));
    
    // Test graduated reminder message
    let (title_ar, title_en, message_ar, message_en) = notification_service
        .generate_prayer_message("dhuhr", 15, false, "ar");
    
    assert!(title_ar.contains("تذكير"));
    assert!(title_ar.contains("الظهر"));
    assert!(message_ar.contains("15"));
    assert!(message_en.contains("15"));
}

#[test]
fn test_notification_priority() {
    let notification_service = PrayerNotificationService::new("http://localhost:8080".to_string());
    
    // Test urgent priority for final reminder
    let priority = notification_service.get_prayer_priority(0, true);
    assert!(matches!(priority, crate::notification_service::NotificationPriority::Urgent));
    
    // Test high priority for close reminders
    let priority = notification_service.get_prayer_priority(5, false);
    assert!(matches!(priority, crate::notification_service::NotificationPriority::Urgent));
    
    // Test medium priority for moderate reminders
    let priority = notification_service.get_prayer_priority(15, false);
    assert!(matches!(priority, crate::notification_service::NotificationPriority::High));
    
    // Test low priority for early reminders
    let priority = notification_service.get_prayer_priority(60, false);
    assert!(matches!(priority, crate::notification_service::NotificationPriority::Low));
}

#[test]
fn test_quiet_hours() {
    let notification_service = PrayerNotificationService::new("http://localhost:8080".to_string());
    
    let preferences = NotificationPreferences {
        user_id: Uuid::new_v4(),
        notifications_enabled: true,
        quiet_hours_start: NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
        quiet_hours_end: NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        prayer_settings: vec![],
        islamic_events_enabled: true,
        friday_reminders_enabled: true,
        surah_kahf_reminder_enabled: true,
        graduated_notifications_enabled: true,
        default_intervals: vec![30, 15, 5],
        language_preference: "ar".to_string(),
    };
    
    // Test time during quiet hours (midnight)
    let midnight = chrono::Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
    assert!(notification_service.is_in_quiet_hours(midnight, &preferences));
    
    // Test time outside quiet hours (noon)
    let noon = chrono::Utc::now().date_naive().and_hms_opt(12, 0, 0).unwrap().and_utc();
    assert!(!notification_service.is_in_quiet_hours(noon, &preferences));
}

#[test]
fn test_default_preferences_creation() {
    let user_id = Uuid::new_v4();
    let preferences = NotificationPreferences::create_default_preferences(user_id);
    
    assert_eq!(preferences.user_id, user_id);
    assert!(preferences.notifications_enabled);
    assert!(preferences.graduated_notifications_enabled);
    assert_eq!(preferences.prayer_settings.len(), 5); // All 5 prayers
    assert_eq!(preferences.default_intervals, vec![30, 15, 5]);
    assert_eq!(preferences.language_preference, "ar");
    
    // Check that all prayers are enabled by default
    for setting in &preferences.prayer_settings {
        assert!(setting.enabled);
        assert_eq!(setting.minutes_before, 15);
        assert!(setting.graduated_enabled);
        assert_eq!(setting.graduated_intervals, vec![30, 15, 5]);
    }
}

#[test]
fn test_prayer_time_extraction() {
    let notification_service = PrayerNotificationService::new("http://localhost:8080".to_string());
    
    let location = Location {
        latitude: 21.4225,
        longitude: 39.8262,
        timezone: "Asia/Riyadh".to_string(),
        city: Some("Makkah".to_string()),
        country: Some("Saudi Arabia".to_string()),
    };
    
    let prayer_times = PrayerTimes {
        fajr: Utc::now() + Duration::hours(1),
        sunrise: Utc::now() + Duration::hours(2),
        dhuhr: Utc::now() + Duration::hours(6),
        asr: Utc::now() + Duration::hours(9),
        maghrib: Utc::now() + Duration::hours(12),
        isha: Utc::now() + Duration::hours(13),
        location: location.clone(),
        calculation_method: CalculationMethod::UmmAlQuraUniversityMakkah,
    };
    
    // Test getting prayer times by name
    assert_eq!(
        notification_service.get_prayer_time_by_name("fajr", &prayer_times).unwrap(),
        prayer_times.fajr
    );
    assert_eq!(
        notification_service.get_prayer_time_by_name("dhuhr", &prayer_times).unwrap(),
        prayer_times.dhuhr
    );
    
    // Test invalid prayer name
    assert!(notification_service.get_prayer_time_by_name("invalid", &prayer_times).is_err());
}
#[cfg(test)]
mod simple_tests {
    use crate::models::*;
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_widget_layout_serialization() {
        let layout = WidgetLayout {
            x: 1,
            y: 2,
            width: 3,
            height: 4,
            size: WidgetSize::Large,
        };

        let json = serde_json::to_string(&layout).unwrap();
        let deserialized: WidgetLayout = serde_json::from_str(&json).unwrap();

        assert_eq!(layout.x, deserialized.x);
        assert_eq!(layout.y, deserialized.y);
        assert_eq!(layout.width, deserialized.width);
        assert_eq!(layout.height, deserialized.height);
        assert_eq!(layout.size, deserialized.size);
    }

    #[test]
    fn test_widget_types() {
        let widget_type = WidgetType::NextPrayerTime;
        assert_eq!(widget_type, WidgetType::NextPrayerTime);
        
        let json = serde_json::to_string(&widget_type).unwrap();
        let deserialized: WidgetType = serde_json::from_str(&json).unwrap();
        assert_eq!(widget_type, deserialized);
    }

    #[test]
    fn test_next_prayer_time_widget_data() {
        let widget_data = NextPrayerTimeWidget {
            prayer_name: "Maghrib".to_string(),
            prayer_name_arabic: "المغرب".to_string(),
            prayer_time: Utc::now(),
            time_remaining: "2h 30m".to_string(),
            time_remaining_minutes: 150,
            location: Some("Mecca".to_string()),
            qibla_direction: Some(0.0),
            is_prayer_time: false,
            next_prayer_after_current: None,
        };

        assert_eq!(widget_data.prayer_name, "Maghrib");
        assert_eq!(widget_data.prayer_name_arabic, "المغرب");
        assert_eq!(widget_data.time_remaining_minutes, 150);
        assert!(!widget_data.is_prayer_time);
    }

    #[test]
    fn test_verse_of_day_widget_data() {
        let widget_data = VerseOfTheDayWidget {
            surah_number: 2,
            surah_name: "Al-Baqarah".to_string(),
            surah_name_arabic: "البقرة".to_string(),
            ayah_number: 255,
            ayah_text_arabic: "اللَّهُ لَا إِلَٰهَ إِلَّا هُوَ".to_string(),
            ayah_text_transliteration: Some("Allahu la ilaha illa huwa".to_string()),
            ayah_text_translation: Some("Allah - there is no deity except Him".to_string()),
            tafsir_brief: Some("This is Ayat al-Kursi".to_string()),
            tafsir_source: Some("Ibn Kathir".to_string()),
            audio_url: Some("/audio/2/255".to_string()),
            share_url: "/verse/2/255".to_string(),
            date: Utc::now(),
        };

        assert_eq!(widget_data.surah_number, 2);
        assert_eq!(widget_data.ayah_number, 255);
        assert_eq!(widget_data.surah_name, "Al-Baqarah");
        assert_eq!(widget_data.surah_name_arabic, "البقرة");
    }

    #[test]
    fn test_khatma_progress_widget_data() {
        let widget_data = KhatmaProgressWidget {
            khatma_id: Some(Uuid::new_v4()),
            is_active: true,
            progress_percentage: 45.5,
            current_surah: Some("Al-Baqarah".to_string()),
            current_surah_arabic: Some("البقرة".to_string()),
            current_ayah: Some(150),
            target_completion_date: Some(Utc::now()),
            days_remaining: Some(30),
            daily_target_pages: Some(5.0),
            pages_read_today: 2.5,
            streak_days: 15,
            total_pages_read: 275,
            estimated_completion_date: Some(Utc::now()),
            is_on_track: true,
            next_reading_suggestion: None,
        };

        assert!(widget_data.is_active);
        assert_eq!(widget_data.progress_percentage, 45.5);
        assert_eq!(widget_data.streak_days, 15);
        assert_eq!(widget_data.total_pages_read, 275);
        assert!(widget_data.is_on_track);
    }

    #[test]
    fn test_dhikr_reminder_widget_data() {
        let widget_data = DhikrReminderWidget {
            dhikr_text_arabic: "سُبْحَانَ اللَّهِ".to_string(),
            dhikr_text_transliteration: Some("Subhan Allah".to_string()),
            dhikr_text_translation: Some("Glory is to Allah".to_string()),
            dhikr_category: DhikrCategory::Morning,
            repetitions: 33,
            completed_today: 10,
            source_reference: Some("Sahih Muslim".to_string()),
            audio_url: None,
            next_dhikr_time: Some(Utc::now()),
        };

        assert_eq!(widget_data.dhikr_text_arabic, "سُبْحَانَ اللَّهِ");
        assert_eq!(widget_data.repetitions, 33);
        assert_eq!(widget_data.completed_today, 10);
        assert_eq!(widget_data.dhikr_category, DhikrCategory::Morning);
    }

    #[test]
    fn test_widget_data_enum_serialization() {
        let prayer_widget = NextPrayerTimeWidget {
            prayer_name: "Fajr".to_string(),
            prayer_name_arabic: "الفجر".to_string(),
            prayer_time: Utc::now(),
            time_remaining: "1h 30m".to_string(),
            time_remaining_minutes: 90,
            location: None,
            qibla_direction: None,
            is_prayer_time: false,
            next_prayer_after_current: None,
        };

        let widget_data = WidgetData::NextPrayerTime(prayer_widget);
        let json = serde_json::to_string(&widget_data).unwrap();
        let deserialized: WidgetData = serde_json::from_str(&json).unwrap();

        match deserialized {
            WidgetData::NextPrayerTime(data) => {
                assert_eq!(data.prayer_name, "Fajr");
                assert_eq!(data.prayer_name_arabic, "الفجر");
            }
            _ => panic!("Wrong widget data type"),
        }
    }

    #[test]
    fn test_widget_error_types() {
        let widget_id = Uuid::new_v4();
        let error = WidgetError::WidgetNotFound { widget_id };
        
        match error {
            WidgetError::WidgetNotFound { widget_id: id } => {
                assert_eq!(id, widget_id);
            }
            _ => panic!("Wrong error type"),
        }

        let config_error = WidgetError::InvalidConfiguration {
            message: "Invalid layout".to_string(),
        };

        match config_error {
            WidgetError::InvalidConfiguration { message } => {
                assert_eq!(message, "Invalid layout");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_widget_type_info() {
        let widget_info = WidgetTypeInfo {
            widget_type: WidgetType::VerseOfTheDay,
            name: "Verse of the Day".to_string(),
            name_arabic: "آية اليوم".to_string(),
            description: "Daily Quranic verse".to_string(),
            description_arabic: "آية قرآنية يومية".to_string(),
            default_size: WidgetSize::Large,
            configurable_options: vec!["language".to_string()],
            refresh_interval_minutes: 1440,
        };

        assert_eq!(widget_info.widget_type, WidgetType::VerseOfTheDay);
        assert_eq!(widget_info.name, "Verse of the Day");
        assert_eq!(widget_info.name_arabic, "آية اليوم");
        assert_eq!(widget_info.default_size, WidgetSize::Large);
        assert_eq!(widget_info.refresh_interval_minutes, 1440);
    }
}
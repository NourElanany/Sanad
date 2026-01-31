use crate::models::*;
use crate::repository::WidgetRepository;
use uuid::Uuid;
use chrono::{DateTime, Utc, Timelike};
use anyhow::Result;
use redis::AsyncCommands;
use serde_json::Value;
use std::collections::HashMap;
use reqwest::Client;

#[derive(Clone)]
pub struct WidgetService {
    repository: WidgetRepository,
    redis_client: redis::Client,
    http_client: Client,
    prayer_service_url: String,
    quran_service_url: String,
    khatma_service_url: String,
    notification_service_url: String,
}

impl WidgetService {
    pub fn new(
        repository: WidgetRepository,
        redis_client: redis::Client,
        prayer_service_url: String,
        quran_service_url: String,
        khatma_service_url: String,
        notification_service_url: String,
    ) -> Self {
        Self {
            repository,
            redis_client,
            http_client: Client::new(),
            prayer_service_url,
            quran_service_url,
            khatma_service_url,
            notification_service_url,
        }
    }

    /// Create a new widget for a user
    pub async fn create_widget(
        &self,
        user_id: Uuid,
        request: CreateWidgetRequest,
    ) -> Result<WidgetDataResponse, WidgetError> {
        let title = request.title.unwrap_or_else(|| self.get_default_widget_title(&request.widget_type));
        let refresh_interval = request.refresh_interval_minutes.unwrap_or_else(|| {
            self.get_default_refresh_interval(&request.widget_type)
        });

        let widget = self.repository.create_widget(
            user_id,
            request.widget_type,
            title,
            request.layout,
            request.configuration,
            refresh_interval,
        ).await?;

        // Fetch initial data for the widget
        let widget_data = self.fetch_widget_data(&widget).await?;
        
        Ok(widget_data)
    }

    /// Get all widgets for a user with their current data
    pub async fn get_user_widgets(&self, user_id: Uuid) -> Result<Vec<WidgetDataResponse>, WidgetError> {
        let widgets = self.repository.get_user_widgets(user_id).await?;
        let mut widget_responses = Vec::new();

        for widget in widgets {
            let widget_data = self.fetch_widget_data(&widget).await?;
            widget_responses.push(widget_data);
        }

        Ok(widget_responses)
    }

    /// Get user's dashboard with widget data
    pub async fn get_user_dashboard(&self, user_id: Uuid) -> Result<DashboardResponse, WidgetError> {
        // Get default dashboard or create one if it doesn't exist
        let dashboard = match self.repository.get_default_dashboard(user_id).await? {
            Some(dashboard) => dashboard,
            None => self.create_default_dashboard(user_id).await?,
        };

        // Get widget IDs from dashboard
        let widget_ids: Vec<Uuid> = serde_json::from_value(dashboard.widgets.clone())
            .unwrap_or_default();

        // Fetch widget data
        let mut widgets = Vec::new();
        for widget_id in widget_ids {
            if let Ok(widget) = self.repository.get_widget(widget_id, user_id).await {
                if let Ok(widget_data) = self.fetch_widget_data(&widget).await {
                    widgets.push(widget_data);
                }
            }
        }

        Ok(DashboardResponse { dashboard, widgets })
    }

    /// Update a widget
    pub async fn update_widget(
        &self,
        widget_id: Uuid,
        user_id: Uuid,
        request: UpdateWidgetRequest,
    ) -> Result<WidgetDataResponse, WidgetError> {
        let widget = self.repository.update_widget(
            widget_id,
            user_id,
            request.title,
            request.is_enabled,
            request.layout,
            request.configuration,
            request.refresh_interval_minutes,
        ).await?;

        let widget_data = self.fetch_widget_data(&widget).await?;
        Ok(widget_data)
    }

    /// Delete a widget
    pub async fn delete_widget(&self, widget_id: Uuid, user_id: Uuid) -> Result<(), WidgetError> {
        self.repository.delete_widget(widget_id, user_id).await
    }

    /// Refresh widget data
    pub async fn refresh_widget(&self, widget_id: Uuid, user_id: Uuid) -> Result<WidgetDataResponse, WidgetError> {
        let widget = self.repository.get_widget(widget_id, user_id).await?;
        let widget_data = self.fetch_widget_data(&widget).await?;
        
        // Update timestamp
        self.repository.update_widget_timestamp(widget_id).await?;
        
        Ok(widget_data)
    }

    /// Fetch widget data based on widget type
    async fn fetch_widget_data(&self, widget: &Widget) -> Result<WidgetDataResponse, WidgetError> {
        let layout: WidgetLayout = serde_json::from_value(widget.layout.clone())
            .map_err(|e| WidgetError::InvalidConfiguration { 
                message: format!("Invalid layout: {}", e) 
            })?;

        let data = match widget.widget_type {
            WidgetType::NextPrayerTime => {
                WidgetData::NextPrayerTime(self.fetch_next_prayer_time_data(widget).await?)
            },
            WidgetType::VerseOfTheDay => {
                WidgetData::VerseOfTheDay(self.fetch_verse_of_day_data(widget).await?)
            },
            WidgetType::KhatmaProgress => {
                WidgetData::KhatmaProgress(self.fetch_khatma_progress_data(widget).await?)
            },
            WidgetType::IslamicCalendar => {
                WidgetData::IslamicCalendar(self.fetch_islamic_calendar_data(widget).await?)
            },
            WidgetType::DhikrReminder => {
                WidgetData::DhikrReminder(self.fetch_dhikr_reminder_data(widget).await?)
            },
            WidgetType::QuickStats => {
                WidgetData::QuickStats(self.fetch_quick_stats_data(widget).await?)
            },
            WidgetType::RecentActivity => {
                WidgetData::RecentActivity(self.fetch_recent_activity_data(widget).await?)
            },
            WidgetType::Notifications => {
                WidgetData::Notifications(self.fetch_notifications_data(widget).await?)
            },
        };

        Ok(WidgetDataResponse {
            widget_id: widget.id,
            widget_type: widget.widget_type.clone(),
            title: widget.title.clone(),
            layout,
            data,
            last_updated: widget.last_updated,
            refresh_interval_minutes: widget.refresh_interval_minutes,
        })
    }

    /// Fetch next prayer time data
    async fn fetch_next_prayer_time_data(&self, widget: &Widget) -> Result<NextPrayerTimeWidget, WidgetError> {
        // Try to get from cache first
        let cache_key = format!("widget:prayer_time:{}", widget.user_id);
        
        if let Ok(mut conn) = self.redis_client.get_async_connection().await {
            if let Ok(cached_data) = conn.get::<_, String>(&cache_key).await {
                if let Ok(data) = serde_json::from_str::<NextPrayerTimeWidget>(&cached_data) {
                    return Ok(data);
                }
            }
        }

        // Fetch from prayer service
        let url = format!("{}/api/prayer-times/next/{}", self.prayer_service_url, widget.user_id);
        let response = self.http_client.get(&url).send().await
            .map_err(|e| WidgetError::ExternalServiceError {
                service: "prayer-times-service".to_string(),
                message: e.to_string(),
            })?;

        if !response.status().is_success() {
            return Err(WidgetError::ExternalServiceError {
                service: "prayer-times-service".to_string(),
                message: format!("HTTP {}", response.status()),
            });
        }

        let prayer_data: Value = response.json().await
            .map_err(|e| WidgetError::ExternalServiceError {
                service: "prayer-times-service".to_string(),
                message: e.to_string(),
            })?;

        // Convert to our widget format
        let widget_data = self.convert_prayer_data_to_widget(prayer_data)?;

        // Cache the result
        if let Ok(mut conn) = self.redis_client.get_async_connection().await {
            let cache_data = serde_json::to_string(&widget_data).unwrap_or_default();
            let _: Result<(), _> = conn.set_ex(&cache_key, cache_data, 300).await; // Cache for 5 minutes
        }

        Ok(widget_data)
    }

    /// Fetch verse of the day data
    async fn fetch_verse_of_day_data(&self, widget: &Widget) -> Result<VerseOfTheDayWidget, WidgetError> {
        let cache_key = format!("widget:verse_of_day:{}", Utc::now().format("%Y-%m-%d"));
        
        if let Ok(mut conn) = self.redis_client.get_async_connection().await {
            if let Ok(cached_data) = conn.get::<_, String>(&cache_key).await {
                if let Ok(data) = serde_json::from_str::<VerseOfTheDayWidget>(&cached_data) {
                    return Ok(data);
                }
            }
        }

        // Fetch from Quran service
        let url = format!("{}/api/verse-of-day", self.quran_service_url);
        let response = self.http_client.get(&url).send().await
            .map_err(|e| WidgetError::ExternalServiceError {
                service: "quran-service".to_string(),
                message: e.to_string(),
            })?;

        if !response.status().is_success() {
            return Err(WidgetError::ExternalServiceError {
                service: "quran-service".to_string(),
                message: format!("HTTP {}", response.status()),
            });
        }

        let verse_data: Value = response.json().await
            .map_err(|e| WidgetError::ExternalServiceError {
                service: "quran-service".to_string(),
                message: e.to_string(),
            })?;

        let widget_data = self.convert_verse_data_to_widget(verse_data)?;

        // Cache for the whole day
        if let Ok(mut conn) = self.redis_client.get_async_connection().await {
            let cache_data = serde_json::to_string(&widget_data).unwrap_or_default();
            let _: Result<(), _> = conn.set_ex(&cache_key, cache_data, 86400).await; // Cache for 24 hours
        }

        Ok(widget_data)
    }

    /// Fetch Khatma progress data
    async fn fetch_khatma_progress_data(&self, widget: &Widget) -> Result<KhatmaProgressWidget, WidgetError> {
        let cache_key = format!("widget:khatma_progress:{}", widget.user_id);
        
        if let Ok(mut conn) = self.redis_client.get_async_connection().await {
            if let Ok(cached_data) = conn.get::<_, String>(&cache_key).await {
                if let Ok(data) = serde_json::from_str::<KhatmaProgressWidget>(&cached_data) {
                    return Ok(data);
                }
            }
        }

        // Fetch from Khatma service
        let url = format!("{}/api/khatma/current/{}", self.khatma_service_url, widget.user_id);
        let response = self.http_client.get(&url).send().await
            .map_err(|e| WidgetError::ExternalServiceError {
                service: "khatma-service".to_string(),
                message: e.to_string(),
            })?;

        let widget_data = if response.status().is_success() {
            let khatma_data: Value = response.json().await
                .map_err(|e| WidgetError::ExternalServiceError {
                    service: "khatma-service".to_string(),
                    message: e.to_string(),
                })?;
            
            self.convert_khatma_data_to_widget(khatma_data)?
        } else {
            // No active Khatma
            KhatmaProgressWidget {
                khatma_id: None,
                is_active: false,
                progress_percentage: 0.0,
                current_surah: None,
                current_surah_arabic: None,
                current_ayah: None,
                target_completion_date: None,
                days_remaining: None,
                daily_target_pages: None,
                pages_read_today: 0.0,
                streak_days: 0,
                total_pages_read: 0,
                estimated_completion_date: None,
                is_on_track: true,
                next_reading_suggestion: None,
            }
        };

        // Cache for 10 minutes
        if let Ok(mut conn) = self.redis_client.get_async_connection().await {
            let cache_data = serde_json::to_string(&widget_data).unwrap_or_default();
            let _: Result<(), _> = conn.set_ex(&cache_key, cache_data, 600).await;
        }

        Ok(widget_data)
    }

    /// Fetch Islamic calendar data
    async fn fetch_islamic_calendar_data(&self, _widget: &Widget) -> Result<IslamicCalendarWidget, WidgetError> {
        let cache_key = format!("widget:islamic_calendar:{}", Utc::now().format("%Y-%m-%d"));
        
        if let Ok(mut conn) = self.redis_client.get_async_connection().await {
            if let Ok(cached_data) = conn.get::<_, String>(&cache_key).await {
                if let Ok(data) = serde_json::from_str::<IslamicCalendarWidget>(&cached_data) {
                    return Ok(data);
                }
            }
        }

        // Generate Islamic calendar data (simplified implementation)
        let now = Utc::now();
        let widget_data = IslamicCalendarWidget {
            hijri_date: self.convert_to_hijri(now),
            gregorian_date: now,
            islamic_events_today: self.get_islamic_events_for_date(now).await,
            upcoming_events: self.get_upcoming_islamic_events().await,
            current_islamic_month_info: self.get_current_month_info().await,
        };

        // Cache for the whole day
        if let Ok(mut conn) = self.redis_client.get_async_connection().await {
            let cache_data = serde_json::to_string(&widget_data).unwrap_or_default();
            let _: Result<(), _> = conn.set_ex(&cache_key, cache_data, 86400).await;
        }

        Ok(widget_data)
    }

    /// Fetch dhikr reminder data
    async fn fetch_dhikr_reminder_data(&self, widget: &Widget) -> Result<DhikrReminderWidget, WidgetError> {
        let now = Utc::now();
        let hour = now.hour();
        
        // Determine appropriate dhikr category based on time
        let category = match hour {
            5..=11 => DhikrCategory::Morning,
            17..=19 => DhikrCategory::Evening,
            _ => DhikrCategory::General,
        };

        // Get dhikr for the current time/category
        let widget_data = self.get_dhikr_for_category(category, widget.user_id).await?;

        Ok(widget_data)
    }

    /// Fetch quick stats data
    async fn fetch_quick_stats_data(&self, _widget: &Widget) -> Result<QuickStatsWidget, WidgetError> {
        // This would typically aggregate data from multiple services
        let widget_data = QuickStatsWidget {
            prayers_completed_today: 3, // Mock data
            prayers_total_today: 5,
            quran_pages_read_today: 2.5,
            dhikr_completed_today: 15,
            current_streak_days: 7,
            total_khatmas_completed: 3,
            monthly_reading_goal_progress: 65.0,
            weekly_consistency_score: 0.85,
        };

        Ok(widget_data)
    }

    /// Fetch recent activity data
    async fn fetch_recent_activity_data(&self, _widget: &Widget) -> Result<RecentActivityWidget, WidgetError> {
        // Mock implementation - would fetch from activity tracking service
        let activities = vec![
            ActivityItem {
                activity_type: ActivityType::QuranReading,
                description: "Read Surah Al-Baqarah 1-10".to_string(),
                description_arabic: Some("قراءة سورة البقرة 1-10".to_string()),
                timestamp: Utc::now() - chrono::Duration::hours(2),
                duration_minutes: Some(15),
                metadata: HashMap::new(),
            },
            ActivityItem {
                activity_type: ActivityType::PrayerCompleted,
                description: "Completed Dhuhr prayer".to_string(),
                description_arabic: Some("أداء صلاة الظهر".to_string()),
                timestamp: Utc::now() - chrono::Duration::hours(4),
                duration_minutes: Some(5),
                metadata: HashMap::new(),
            },
        ];

        let widget_data = RecentActivityWidget {
            recent_activities: activities,
            activity_summary: ActivitySummary {
                total_activities_today: 5,
                most_active_hour: Some(14),
                primary_activity_type: Some(ActivityType::QuranReading),
                productivity_score: 0.78,
            },
        };

        Ok(widget_data)
    }

    /// Fetch notifications data
    async fn fetch_notifications_data(&self, widget: &Widget) -> Result<NotificationsWidget, WidgetError> {
        let url = format!("{}/api/notifications/recent/{}", self.notification_service_url, widget.user_id);
        let response = self.http_client.get(&url).send().await
            .map_err(|e| WidgetError::ExternalServiceError {
                service: "notification-service".to_string(),
                message: e.to_string(),
            })?;

        let widget_data = if response.status().is_success() {
            let notifications_data: Value = response.json().await
                .map_err(|e| WidgetError::ExternalServiceError {
                    service: "notification-service".to_string(),
                    message: e.to_string(),
                })?;
            
            self.convert_notifications_data_to_widget(notifications_data)?
        } else {
            NotificationsWidget {
                unread_count: 0,
                recent_notifications: vec![],
                priority_notifications: vec![],
            }
        };

        Ok(widget_data)
    }

    /// Create default dashboard for new users
    async fn create_default_dashboard(&self, user_id: Uuid) -> Result<WidgetDashboard, WidgetError> {
        // Create default widgets
        let default_widgets = vec![
            (WidgetType::NextPrayerTime, WidgetLayout { x: 0, y: 0, width: 2, height: 1, size: WidgetSize::Medium }),
            (WidgetType::VerseOfTheDay, WidgetLayout { x: 2, y: 0, width: 2, height: 2, size: WidgetSize::Large }),
            (WidgetType::KhatmaProgress, WidgetLayout { x: 0, y: 1, width: 2, height: 1, size: WidgetSize::Medium }),
            (WidgetType::QuickStats, WidgetLayout { x: 0, y: 2, width: 4, height: 1, size: WidgetSize::Wide }),
        ];

        let mut widget_ids = Vec::new();
        for (widget_type, layout) in default_widgets {
            let title = self.get_default_widget_title(&widget_type);
            let refresh_interval = self.get_default_refresh_interval(&widget_type);
            
            let widget = self.repository.create_widget(
                user_id,
                widget_type,
                title,
                layout,
                None,
                refresh_interval,
            ).await?;
            
            widget_ids.push(widget.id);
        }

        // Create dashboard
        let dashboard = self.repository.create_dashboard(
            user_id,
            "الشاشة الرئيسية".to_string(),
            true,
            None,
            widget_ids,
        ).await?;

        Ok(dashboard)
    }

    /// Get default widget title based on type
    fn get_default_widget_title(&self, widget_type: &WidgetType) -> String {
        match widget_type {
            WidgetType::NextPrayerTime => "وقت الصلاة التالي".to_string(),
            WidgetType::VerseOfTheDay => "آية اليوم".to_string(),
            WidgetType::KhatmaProgress => "تقدم الختمة".to_string(),
            WidgetType::IslamicCalendar => "التقويم الهجري".to_string(),
            WidgetType::DhikrReminder => "تذكير الأذكار".to_string(),
            WidgetType::QuickStats => "الإحصائيات السريعة".to_string(),
            WidgetType::RecentActivity => "النشاط الأخير".to_string(),
            WidgetType::Notifications => "الإشعارات".to_string(),
        }
    }

    /// Get default refresh interval based on widget type
    fn get_default_refresh_interval(&self, widget_type: &WidgetType) -> i32 {
        match widget_type {
            WidgetType::NextPrayerTime => 5,  // 5 minutes
            WidgetType::VerseOfTheDay => 1440, // 24 hours
            WidgetType::KhatmaProgress => 10,  // 10 minutes
            WidgetType::IslamicCalendar => 1440, // 24 hours
            WidgetType::DhikrReminder => 60,   // 1 hour
            WidgetType::QuickStats => 15,      // 15 minutes
            WidgetType::RecentActivity => 5,   // 5 minutes
            WidgetType::Notifications => 2,    // 2 minutes
        }
    }

    // Helper methods for data conversion (simplified implementations)
    
    fn convert_prayer_data_to_widget(&self, _data: Value) -> Result<NextPrayerTimeWidget, WidgetError> {
        // Simplified conversion - in real implementation, parse the actual API response
        Ok(NextPrayerTimeWidget {
            prayer_name: "Maghrib".to_string(),
            prayer_name_arabic: "المغرب".to_string(),
            prayer_time: Utc::now() + chrono::Duration::hours(2),
            time_remaining: "2h 15m".to_string(),
            time_remaining_minutes: 135,
            location: Some("Mecca, Saudi Arabia".to_string()),
            qibla_direction: Some(0.0),
            is_prayer_time: false,
            next_prayer_after_current: Some(NextPrayerInfo {
                prayer_name: "Isha".to_string(),
                prayer_name_arabic: "العشاء".to_string(),
                prayer_time: Utc::now() + chrono::Duration::hours(4),
            }),
        })
    }

    fn convert_verse_data_to_widget(&self, _data: Value) -> Result<VerseOfTheDayWidget, WidgetError> {
        Ok(VerseOfTheDayWidget {
            surah_number: 2,
            surah_name: "Al-Baqarah".to_string(),
            surah_name_arabic: "البقرة".to_string(),
            ayah_number: 255,
            ayah_text_arabic: "اللَّهُ لَا إِلَٰهَ إِلَّا هُوَ الْحَيُّ الْقَيُّومُ".to_string(),
            ayah_text_transliteration: Some("Allahu la ilaha illa huwa al-hayyu al-qayyum".to_string()),
            ayah_text_translation: Some("Allah - there is no deity except Him, the Ever-Living, the Sustainer of existence.".to_string()),
            tafsir_brief: Some("This is Ayat al-Kursi, one of the greatest verses in the Quran.".to_string()),
            tafsir_source: Some("Ibn Kathir".to_string()),
            audio_url: Some("/api/audio/2/255".to_string()),
            share_url: "/verse/2/255".to_string(),
            date: Utc::now(),
        })
    }

    fn convert_khatma_data_to_widget(&self, _data: Value) -> Result<KhatmaProgressWidget, WidgetError> {
        Ok(KhatmaProgressWidget {
            khatma_id: Some(Uuid::new_v4()),
            is_active: true,
            progress_percentage: 35.5,
            current_surah: Some("Al-Baqarah".to_string()),
            current_surah_arabic: Some("البقرة".to_string()),
            current_ayah: Some(150),
            target_completion_date: Some(Utc::now() + chrono::Duration::days(45)),
            days_remaining: Some(45),
            daily_target_pages: Some(4.5),
            pages_read_today: 2.0,
            streak_days: 12,
            total_pages_read: 215,
            estimated_completion_date: Some(Utc::now() + chrono::Duration::days(48)),
            is_on_track: false,
            next_reading_suggestion: Some(ReadingSuggestion {
                surah_start: 2,
                ayah_start: 151,
                surah_end: 2,
                ayah_end: 170,
                estimated_minutes: 25,
                suggested_time: Some(Utc::now() + chrono::Duration::hours(2)),
            }),
        })
    }

    fn convert_notifications_data_to_widget(&self, _data: Value) -> Result<NotificationsWidget, WidgetError> {
        Ok(NotificationsWidget {
            unread_count: 3,
            recent_notifications: vec![
                NotificationItem {
                    id: Uuid::new_v4(),
                    title: "Prayer Time Reminder".to_string(),
                    body: "Maghrib prayer in 10 minutes".to_string(),
                    notification_type: "prayer_reminder".to_string(),
                    priority: "high".to_string(),
                    timestamp: Utc::now() - chrono::Duration::minutes(5),
                    is_read: false,
                    action_url: Some("/prayers".to_string()),
                },
            ],
            priority_notifications: vec![],
        })
    }

    // Helper methods for Islamic calendar and dhikr
    
    fn convert_to_hijri(&self, _date: DateTime<Utc>) -> HijriDate {
        // Simplified Hijri conversion - in real implementation, use proper conversion library
        HijriDate {
            day: 15,
            month: 3,
            year: 1445,
            month_name_arabic: "ربيع الأول".to_string(),
            month_name_english: "Rabi' al-Awwal".to_string(),
            day_name_arabic: "الجمعة".to_string(),
            day_name_english: "Friday".to_string(),
        }
    }

    async fn get_islamic_events_for_date(&self, _date: DateTime<Utc>) -> Vec<IslamicEvent> {
        // Mock implementation
        vec![]
    }

    async fn get_upcoming_islamic_events(&self) -> Vec<IslamicEvent> {
        // Mock implementation
        vec![
            IslamicEvent {
                name: "Laylat al-Qadr".to_string(),
                name_arabic: "ليلة القدر".to_string(),
                description: Some("The Night of Power".to_string()),
                date: Utc::now() + chrono::Duration::days(10),
                hijri_date: self.convert_to_hijri(Utc::now() + chrono::Duration::days(10)),
                event_type: IslamicEventType::HolyNight,
                significance: EventSignificance::Major,
            },
        ]
    }

    async fn get_current_month_info(&self) -> IslamicMonthInfo {
        IslamicMonthInfo {
            month_number: 3,
            month_name_arabic: "ربيع الأول".to_string(),
            month_name_english: "Rabi' al-Awwal".to_string(),
            significance: Some("The month of the Prophet's birth".to_string()),
            recommended_actions: vec![
                "Increase prayers upon the Prophet".to_string(),
                "Study the Seerah".to_string(),
            ],
            special_days: vec![12], // 12th of Rabi' al-Awwal
        }
    }

    async fn get_dhikr_for_category(&self, category: DhikrCategory, _user_id: Uuid) -> Result<DhikrReminderWidget, WidgetError> {
        let (arabic, transliteration, translation, repetitions) = match category {
            DhikrCategory::Morning => (
                "أَصْبَحْنَا وَأَصْبَحَ الْمُلْكُ لِلَّهِ".to_string(),
                "Asbahna wa asbahal-mulku lillah".to_string(),
                "We have reached the morning and with it Allah's sovereignty".to_string(),
                1,
            ),
            DhikrCategory::Evening => (
                "أَمْسَيْنَا وَأَمْسَى الْمُلْكُ لِلَّهِ".to_string(),
                "Amsayna wa amsal-mulku lillah".to_string(),
                "We have reached the evening and with it Allah's sovereignty".to_string(),
                1,
            ),
            _ => (
                "سُبْحَانَ اللَّهِ وَبِحَمْدِهِ".to_string(),
                "Subhan Allah wa bihamdihi".to_string(),
                "Glory is to Allah and praise is to Him".to_string(),
                100,
            ),
        };

        Ok(DhikrReminderWidget {
            dhikr_text_arabic: arabic,
            dhikr_text_transliteration: Some(transliteration),
            dhikr_text_translation: Some(translation),
            dhikr_category: category,
            repetitions,
            completed_today: 0,
            source_reference: Some("Sahih Muslim".to_string()),
            audio_url: None,
            next_dhikr_time: Some(Utc::now() + chrono::Duration::hours(1)),
        })
    }

    /// Get available widget types
    pub fn get_available_widget_types(&self) -> Vec<WidgetTypeInfo> {
        vec![
            WidgetTypeInfo {
                widget_type: WidgetType::NextPrayerTime,
                name: "Next Prayer Time".to_string(),
                name_arabic: "وقت الصلاة التالي".to_string(),
                description: "Shows the next prayer time and remaining time".to_string(),
                description_arabic: "يعرض وقت الصلاة التالي والوقت المتبقي".to_string(),
                default_size: WidgetSize::Medium,
                configurable_options: vec!["location".to_string(), "calculation_method".to_string()],
                refresh_interval_minutes: 5,
            },
            WidgetTypeInfo {
                widget_type: WidgetType::VerseOfTheDay,
                name: "Verse of the Day".to_string(),
                name_arabic: "آية اليوم".to_string(),
                description: "Daily Quranic verse with translation and brief tafsir".to_string(),
                description_arabic: "آية قرآنية يومية مع الترجمة والتفسير المختصر".to_string(),
                default_size: WidgetSize::Large,
                configurable_options: vec!["language".to_string(), "tafsir_source".to_string()],
                refresh_interval_minutes: 1440,
            },
            WidgetTypeInfo {
                widget_type: WidgetType::KhatmaProgress,
                name: "Khatma Progress".to_string(),
                name_arabic: "تقدم الختمة".to_string(),
                description: "Shows current Quran reading progress and goals".to_string(),
                description_arabic: "يعرض تقدم قراءة القرآن الحالي والأهداف".to_string(),
                default_size: WidgetSize::Medium,
                configurable_options: vec!["show_suggestions".to_string(), "show_streak".to_string()],
                refresh_interval_minutes: 10,
            },
        ]
    }
}
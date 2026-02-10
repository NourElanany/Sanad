use chrono::{NaiveDate, Utc, Datelike};
use uuid::Uuid;
use shared::{SanadResult, SanadError, Location, CalculationMethod, PrayerTimes, HijriDate};
use crate::{
    models::{
        PrayerTimesRequest, QiblaRequest, QiblaDirection, HijriConversionRequest,
        GregorianConversionRequest, IslamicEventsRequest, PrayerTimesResponse,
        CalculationMetadata, AnglesUsed, PrayerAdjustments, MonthlyCalendarResponse,
        CalendarDay, IslamicEventDetails, HijriGregorianConversion, DailyPrayerTimes,
        NotificationScheduleRequest, ScheduledNotification, UserPrayerPreferences,
    },
    calculator::PrayerTimesCalculator,
    hijri_calendar::HijriCalendar,
    repository::PrayerTimesRepository,
};

/// Prayer times and calendar service
pub struct PrayerTimesService {
    repository: PrayerTimesRepository,
}

impl PrayerTimesService {
    pub fn new(repository: PrayerTimesRepository) -> Self {
        Self { 
            repository,
        }
    }
    
    /// Calculate prayer times for a location and date
    pub async fn calculate_prayer_times(
        &self,
        request: PrayerTimesRequest,
    ) -> SanadResult<PrayerTimesResponse> {
        let method = request.calculation_method.unwrap_or(CalculationMethod::MuslimWorldLeague);
        let adjustments = request.adjustments.unwrap_or_default();
        
        // Get or create location
        let location_id = self.repository.get_or_create_location(&request.location).await?;
        
        // Check cache first
        if let Some(cached) = self.repository.get_daily_prayer_times(
            location_id,
            &method,
            request.date,
        ).await? {
            return self.build_response_from_cached(cached, &request.location).await;
        }
        
        // Calculate prayer times
        let prayer_times = PrayerTimesCalculator::calculate_prayer_times(
            &request.location,
            request.date,
            &method,
            Some(&adjustments),
        ).map_err(|e| SanadError::PrayerTimeCalculation(e.to_string()))?;
        
        // Calculate Qibla direction
        let qibla = PrayerTimesCalculator::calculate_qibla_direction(
            request.location.latitude,
            request.location.longitude,
        ).map_err(|e| SanadError::PrayerTimeCalculation(e.to_string()))?;
        
        // Get Islamic events for this date
        let hijri_date = HijriCalendar::gregorian_to_hijri(request.date)
            .map_err(|e| SanadError::Internal(e.to_string()))?;
        
        let islamic_events = self.repository.get_islamic_events_for_date(
            hijri_date.month as i32,
            hijri_date.day as i32,
        ).await?;
        
        // Cache the results
        let daily_prayer_times = DailyPrayerTimes {
            id: Uuid::new_v4(),
            location_id,
            calculation_method: method.clone(),
            date: request.date,
            fajr_time: prayer_times.fajr,
            sunrise_time: prayer_times.sunrise,
            dhuhr_time: prayer_times.dhuhr,
            asr_time: prayer_times.asr,
            maghrib_time: prayer_times.maghrib,
            isha_time: prayer_times.isha,
            qibla_direction: qibla.direction_degrees,
            fajr_angle: self.get_fajr_angle(&method),
            maghrib_angle: self.get_maghrib_angle(&method),
            isha_angle: self.get_isha_angle(&method),
            asr_method: 1, // Default to Shafi method
            created_at: Utc::now(),
        };
        
        self.repository.save_daily_prayer_times(&daily_prayer_times).await?;
        
        // Build response
        let calculation_metadata = CalculationMetadata {
            method_used: method.clone(),
            angles_used: AnglesUsed {
                fajr_angle: self.get_fajr_angle(&method).unwrap_or(18.0),
                maghrib_angle: self.get_maghrib_angle(&method).unwrap_or(0.0),
                isha_angle: self.get_isha_angle(&method).unwrap_or(17.0),
                asr_method: 1,
            },
            adjustments_applied: adjustments,
            high_latitude_method: None,
            calculation_timestamp: Utc::now(),
        };
        
        Ok(PrayerTimesResponse {
            prayer_times,
            qibla_direction: qibla,
            calculation_metadata,
            islamic_events,
        })
    }

    /// Calculate Qibla direction
    pub async fn calculate_qibla_direction(
        &self,
        request: QiblaRequest,
    ) -> SanadResult<QiblaDirection> {
        PrayerTimesCalculator::calculate_qibla_direction(
            request.latitude,
            request.longitude,
        ).map_err(|e| SanadError::PrayerTimeCalculation(e.to_string()))
    }
    
    /// Convert Gregorian date to Hijri
    pub async fn gregorian_to_hijri(
        &self,
        request: HijriConversionRequest,
    ) -> SanadResult<HijriDate> {
        // Check cache first
        if let Some(cached) = self.repository.get_hijri_conversion(request.date).await? {
            return Ok(HijriDate {
                year: cached.hijri_year,
                month: cached.hijri_month as u8,
                day: cached.hijri_day as u8,
                month_name: self.get_month_name(cached.hijri_month)?,
            });
        }
        
        // Calculate conversion
        let hijri_date = HijriCalendar::gregorian_to_hijri(request.date)
            .map_err(|e| SanadError::Internal(e.to_string()))?;
        
        // Cache the result
        let conversion = HijriGregorianConversion {
            id: Uuid::new_v4(),
            gregorian_date: request.date,
            hijri_year: hijri_date.year,
            hijri_month: hijri_date.month as i32,
            hijri_day: hijri_date.day as i32,
            julian_day_number: self.gregorian_to_julian(request.date),
            created_at: Utc::now(),
        };
        
        self.repository.save_hijri_conversion(&conversion).await?;
        
        Ok(hijri_date)
    }
    
    /// Convert Hijri date to Gregorian
    pub async fn hijri_to_gregorian(
        &self,
        request: GregorianConversionRequest,
    ) -> SanadResult<NaiveDate> {
        HijriCalendar::hijri_to_gregorian(
            request.hijri_year,
            request.hijri_month,
            request.hijri_day,
        ).map_err(|e| SanadError::Internal(e.to_string()))
    }
    
    /// Get Islamic events
    pub async fn get_islamic_events(
        &self,
        request: IslamicEventsRequest,
    ) -> SanadResult<Vec<IslamicEventDetails>> {
        if let Some(hijri_month) = request.hijri_month {
            if let Some(hijri_day) = request.hijri_day {
                // Get events for specific date
                self.repository.get_islamic_events_for_date(hijri_month, hijri_day).await
            } else {
                // Get events for entire month
                self.repository.get_islamic_events_for_month(hijri_month).await
            }
        } else {
            // Return events from HijriCalendar for today if no specific date requested
            let today = chrono::Utc::now().date_naive();
            let hijri_today = HijriCalendar::gregorian_to_hijri(today)
                .map_err(|e| SanadError::Internal(e.to_string()))?;
            
            let calendar_events = HijriCalendar::get_islamic_events_for_date(
                hijri_today.month as i32,
                hijri_today.day as i32,
            );
            
            // Convert to IslamicEventDetails
            let mut events = Vec::new();
            for event in calendar_events {
                events.push(IslamicEventDetails {
                    id: uuid::Uuid::new_v4(),
                    name_arabic: event.name.split(" / ").next().unwrap_or(&event.name).to_string(),
                    name_english: event.name.split(" / ").nth(1).unwrap_or(&event.name).to_string(),
                    description_arabic: Some(event.description.split(" / ").next().unwrap_or(&event.description).to_string()),
                    description_english: Some(event.description.split(" / ").nth(1).unwrap_or(&event.description).to_string()),
                    hijri_month: Some(event.hijri_date.month as i32),
                    hijri_day: Some(event.hijri_date.day as i32),
                    hijri_end_month: None,
                    hijri_end_day: None,
                    event_type: match event.event_type {
                        shared::EventType::Eid => "eid".to_string(),
                        shared::EventType::HolyMonth => "holy_month".to_string(),
                        shared::EventType::ImportantDay => "important_day".to_string(),
                        shared::EventType::ProphetBirthday => "prophet_birthday".to_string(),
                        shared::EventType::CompanionCommemoration => "companion_commemoration".to_string(),
                    },
                    importance_level: match event.event_type {
                        shared::EventType::Eid => 5,
                        shared::EventType::ProphetBirthday => 5,
                        shared::EventType::HolyMonth => 4,
                        shared::EventType::ImportantDay => 4,
                        shared::EventType::CompanionCommemoration => 3,
                    },
                    notification_enabled: true,
                    special_calculation: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                });
            }
            
            Ok(events)
        }
    }
    
    /// Get monthly Islamic calendar
    pub async fn get_monthly_calendar(
        &self,
        hijri_year: i32,
        hijri_month: i32,
    ) -> SanadResult<MonthlyCalendarResponse> {
        // Get month information
        let hijri_months = self.repository.get_hijri_months().await?;
        let month_info = hijri_months.into_iter()
            .find(|m| m.month_number == hijri_month)
            .ok_or_else(|| SanadError::Validation("Invalid Hijri month".to_string()))?;
        
        // Get events for this month
        let events = self.repository.get_islamic_events_for_month(hijri_month).await?;
        
        // Generate calendar days
        let days_in_month = HijriCalendar::days_in_hijri_month(hijri_year, hijri_month);
        let mut days = Vec::new();
        
        for day in 1..=days_in_month {
            let hijri_date = HijriDate {
                year: hijri_year,
                month: hijri_month as u8,
                day: day as u8,
                month_name: month_info.name_english.clone(),
            };
            
            let gregorian_date = HijriCalendar::hijri_to_gregorian(hijri_year, hijri_month, day)
                .map_err(|e| SanadError::Internal(e.to_string()))?;
            
            let day_events: Vec<IslamicEventDetails> = events.iter()
                .filter(|e| e.hijri_day == Some(day))
                .cloned()
                .collect();
            
            let is_friday = HijriCalendar::is_friday(hijri_year, hijri_month, day)
                .map_err(|e| SanadError::Internal(e.to_string()))?;
            
            days.push(CalendarDay {
                hijri_date,
                gregorian_date,
                day_of_week: self.get_day_of_week_name(gregorian_date),
                events: day_events,
                is_friday,
                is_weekend: is_friday || gregorian_date.weekday().num_days_from_sunday() == 6,
            });
        }
        
        Ok(MonthlyCalendarResponse {
            hijri_month: month_info,
            hijri_year,
            days,
            events,
        })
    }
    
    // Helper methods
    
    async fn build_response_from_cached(
        &self,
        cached: DailyPrayerTimes,
        location: &Location,
    ) -> SanadResult<PrayerTimesResponse> {
        let prayer_times = PrayerTimes {
            fajr: cached.fajr_time,
            sunrise: cached.sunrise_time,
            dhuhr: cached.dhuhr_time,
            asr: cached.asr_time,
            maghrib: cached.maghrib_time,
            isha: cached.isha_time,
            location: location.clone(),
            calculation_method: cached.calculation_method.clone(),
        };
        
        let qibla = QiblaDirection::new(cached.qibla_direction, 0.0); // Distance not cached
        
        let hijri_date = HijriCalendar::gregorian_to_hijri(cached.date)
            .map_err(|e| SanadError::Internal(e.to_string()))?;
        
        let islamic_events = self.repository.get_islamic_events_for_date(
            hijri_date.month as i32,
            hijri_date.day as i32,
        ).await?;
        
        let calculation_metadata = CalculationMetadata {
            method_used: cached.calculation_method.clone(),
            angles_used: AnglesUsed {
                fajr_angle: cached.fajr_angle.unwrap_or(18.0),
                maghrib_angle: cached.maghrib_angle.unwrap_or(0.0),
                isha_angle: cached.isha_angle.unwrap_or(17.0),
                asr_method: cached.asr_method,
            },
            adjustments_applied: PrayerAdjustments::default(),
            high_latitude_method: None,
            calculation_timestamp: cached.created_at,
        };
        
        Ok(PrayerTimesResponse {
            prayer_times,
            qibla_direction: qibla,
            calculation_metadata,
            islamic_events,
        })
    }

    fn get_fajr_angle(&self, method: &CalculationMethod) -> Option<f64> {
        match method {
            CalculationMethod::MuslimWorldLeague => Some(18.0),
            CalculationMethod::IslamicSocietyOfNorthAmerica => Some(15.0),
            CalculationMethod::EgyptianGeneralAuthorityOfSurvey => Some(19.5),
            CalculationMethod::UmmAlQuraUniversityMakkah => Some(18.5),
            CalculationMethod::UniversityOfIslamicSciencesKarachi => Some(18.0),
            CalculationMethod::InstituteOfGeophysicsUniversityOfTehran => Some(17.7),
            CalculationMethod::Shia => Some(16.0),
            CalculationMethod::Custom { fajr_angle, .. } => Some(*fajr_angle),
        }
    }
    
    fn get_maghrib_angle(&self, method: &CalculationMethod) -> Option<f64> {
        match method {
            CalculationMethod::InstituteOfGeophysicsUniversityOfTehran => Some(4.5),
            CalculationMethod::Shia => Some(4.0),
            CalculationMethod::Custom { maghrib_angle, .. } => Some(*maghrib_angle),
            _ => Some(0.0),
        }
    }
    
    fn get_isha_angle(&self, method: &CalculationMethod) -> Option<f64> {
        match method {
            CalculationMethod::MuslimWorldLeague => Some(17.0),
            CalculationMethod::IslamicSocietyOfNorthAmerica => Some(15.0),
            CalculationMethod::EgyptianGeneralAuthorityOfSurvey => Some(17.5),
            CalculationMethod::UmmAlQuraUniversityMakkah => Some(0.0), // 90 minutes after Maghrib
            CalculationMethod::UniversityOfIslamicSciencesKarachi => Some(18.0),
            CalculationMethod::InstituteOfGeophysicsUniversityOfTehran => Some(14.0),
            CalculationMethod::Shia => Some(14.0),
            CalculationMethod::Custom { isha_angle, .. } => Some(*isha_angle),
        }
    }
    
    fn get_month_name(&self, month_number: i32) -> SanadResult<String> {
        let months = HijriCalendar::get_hijri_months();
        months.iter()
            .find(|m| m.month_number == month_number)
            .map(|m| m.name_english.clone())
            .ok_or_else(|| SanadError::Validation("Invalid month number".to_string()))
    }
    
    fn gregorian_to_julian(&self, date: NaiveDate) -> i32 {
        let year = date.year();
        let month = date.month() as i32;
        let day = date.day() as i32;
        
        let a = (14 - month) / 12;
        let y = year + 4800 - a;
        let m = month + 12 * a - 3;
        
        day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
    }
    
    /// Get detailed information about an Islamic event
    pub async fn get_event_details(&self, event_name: &str) -> SanadResult<Option<String>> {
        Ok(HijriCalendar::get_event_details(event_name))
    }
    
    /// Get current Hijri date
    pub async fn get_current_hijri_date(&self) -> SanadResult<HijriDate> {
        let today = chrono::Utc::now().date_naive();
        HijriCalendar::gregorian_to_hijri(today)
            .map_err(|e| SanadError::Internal(e.to_string()))
    }
    
    /// Validate Hijri date
    pub async fn validate_hijri_date(
        &self,
        hijri_year: i32,
        hijri_month: i32,
        hijri_day: i32,
    ) -> SanadResult<bool> {
        Ok(HijriCalendar::is_valid_hijri_date(hijri_year, hijri_month, hijri_day))
    }
    
    fn get_day_of_week_name(&self, date: NaiveDate) -> String {
        match date.weekday() {
            chrono::Weekday::Mon => "Monday".to_string(),
            chrono::Weekday::Tue => "Tuesday".to_string(),
            chrono::Weekday::Wed => "Wednesday".to_string(),
            chrono::Weekday::Thu => "Thursday".to_string(),
            chrono::Weekday::Fri => "Friday".to_string(),
            chrono::Weekday::Sat => "Saturday".to_string(),
            chrono::Weekday::Sun => "Sunday".to_string(),
        }
    }

    /// Schedule prayer time notifications for a user
    pub async fn schedule_prayer_notifications(
        &self,
        _user_id: Uuid,
        _location: &Location,
        _date: NaiveDate,
        _preferences: &NotificationPreferences,
    ) -> SanadResult<Vec<ScheduledNotification>> {
        // Stub implementation - would integrate with notification service
        Ok(Vec::new())
    }

    /// Schedule notifications for multiple days
    pub async fn schedule_notifications_for_period(
        &self,
        _request: NotificationScheduleRequest,
    ) -> SanadResult<Vec<ScheduledNotification>> {
        // Stub implementation - would integrate with notification service
        Ok(Vec::new())
    }

    /// Get user prayer notification preferences
    pub async fn get_user_prayer_preferences(&self, user_id: Uuid) -> SanadResult<UserPrayerPreferences> {
        self.repository.get_user_prayer_preferences(user_id).await
    }

    /// Update user prayer notification preferences
    pub async fn update_user_prayer_preferences(
        &self,
        user_id: Uuid,
        preferences: UserPrayerPreferences,
    ) -> SanadResult<UserPrayerPreferences> {
        self.repository.update_user_prayer_preferences(user_id, preferences).await
    }

    /// Create default prayer preferences for a new user
    pub async fn create_default_prayer_preferences(&self, user_id: Uuid) -> SanadResult<UserPrayerPreferences> {
        let default_preferences = UserPrayerPreferences {
            id: Uuid::new_v4(),
            user_id,
            fajr_notification_enabled: true,
            fajr_notification_minutes: 15,
            dhuhr_notification_enabled: true,
            dhuhr_notification_minutes: 15,
            asr_notification_enabled: true,
            asr_notification_minutes: 15,
            maghrib_notification_enabled: true,
            maghrib_notification_minutes: 15,
            isha_notification_enabled: true,
            isha_notification_minutes: 15,
            sunrise_notification_enabled: false,
            sunrise_notification_minutes: 15,
            graduated_notifications_enabled: true,
            graduated_intervals: vec![30, 15, 5],
            show_qibla_direction: true,
            qibla_compass_style: "modern".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repository.create_user_prayer_preferences(default_preferences).await
    }

    /// Convert UserPrayerPreferences to NotificationPreferences
    fn convert_to_notification_preferences(
        &self,
        user_id: Uuid,
        prayer_settings: &[crate::models::PrayerNotificationSettings],
    ) -> NotificationPreferences {
        NotificationPreferences {
            user_id,
            notifications_enabled: true,
            quiet_hours_start: chrono::NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
            quiet_hours_end: chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
            prayer_settings: prayer_settings.to_vec(),
            islamic_events_enabled: true,
            friday_reminders_enabled: true,
            surah_kahf_reminder_enabled: true,
            graduated_notifications_enabled: true,
            default_intervals: vec![30, 15, 5],
            language_preference: "ar".to_string(),
        }
    }

    /// Get upcoming prayer notifications for a user
    pub async fn get_upcoming_notifications(
        &self,
        _user_id: Uuid,
        _location: &Location,
        _days_ahead: i32,
    ) -> SanadResult<Vec<ScheduledNotification>> {
        // Stub implementation - would integrate with notification service
        Ok(Vec::new())
    }

    /// Convert UserPrayerPreferences to NotificationPreferences
    pub fn convert_user_prefs_to_notification_prefs(&self, user_prefs: UserPrayerPreferences) -> NotificationPreferences {
        NotificationPreferences {
            user_id: user_prefs.user_id,
            notifications_enabled: true,
            quiet_hours_start: chrono::NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
            quiet_hours_end: chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
            prayer_settings: vec![
                crate::models::PrayerNotificationSettings {
                    prayer_name: "fajr".to_string(),
                    enabled: user_prefs.fajr_notification_enabled,
                    minutes_before: user_prefs.fajr_notification_minutes,
                    graduated_enabled: user_prefs.graduated_notifications_enabled,
                    graduated_intervals: user_prefs.graduated_intervals.clone(),
                },
                crate::models::PrayerNotificationSettings {
                    prayer_name: "dhuhr".to_string(),
                    enabled: user_prefs.dhuhr_notification_enabled,
                    minutes_before: user_prefs.dhuhr_notification_minutes,
                    graduated_enabled: user_prefs.graduated_notifications_enabled,
                    graduated_intervals: user_prefs.graduated_intervals.clone(),
                },
                crate::models::PrayerNotificationSettings {
                    prayer_name: "asr".to_string(),
                    enabled: user_prefs.asr_notification_enabled,
                    minutes_before: user_prefs.asr_notification_minutes,
                    graduated_enabled: user_prefs.graduated_notifications_enabled,
                    graduated_intervals: user_prefs.graduated_intervals.clone(),
                },
                crate::models::PrayerNotificationSettings {
                    prayer_name: "maghrib".to_string(),
                    enabled: user_prefs.maghrib_notification_enabled,
                    minutes_before: user_prefs.maghrib_notification_minutes,
                    graduated_enabled: user_prefs.graduated_notifications_enabled,
                    graduated_intervals: user_prefs.graduated_intervals.clone(),
                },
                crate::models::PrayerNotificationSettings {
                    prayer_name: "isha".to_string(),
                    enabled: user_prefs.isha_notification_enabled,
                    minutes_before: user_prefs.isha_notification_minutes,
                    graduated_enabled: user_prefs.graduated_notifications_enabled,
                    graduated_intervals: user_prefs.graduated_intervals.clone(),
                },
            ],
            islamic_events_enabled: true,
            friday_reminders_enabled: true,
            surah_kahf_reminder_enabled: true,
            graduated_notifications_enabled: user_prefs.graduated_notifications_enabled,
            default_intervals: user_prefs.graduated_intervals,
            language_preference: "ar".to_string(),
        }
    }
}

// Stub struct for NotificationPreferences
#[derive(Debug, Clone)]
pub struct NotificationPreferences {
    pub user_id: Uuid,
    pub notifications_enabled: bool,
    pub quiet_hours_start: chrono::NaiveTime,
    pub quiet_hours_end: chrono::NaiveTime,
    pub prayer_settings: Vec<crate::models::PrayerNotificationSettings>,
    pub islamic_events_enabled: bool,
    pub friday_reminders_enabled: bool,
    pub surah_kahf_reminder_enabled: bool,
    pub graduated_notifications_enabled: bool,
    pub default_intervals: Vec<i32>,
    pub language_preference: String,
}
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::NaiveDate;
use shared::{SanadResult, SanadError, Location, CalculationMethod};
use crate::models::{
    DailyPrayerTimes, HijriGregorianConversion, IslamicEventDetails, HijriMonth,
    UserPrayerPreferences,
};

/// Repository for prayer times and calendar data
pub struct PrayerTimesRepository {
    pool: PgPool,
}

impl PrayerTimesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Get or create location
    pub async fn get_or_create_location(&self, location: &Location) -> SanadResult<Uuid> {
        // First try to find existing location
        let existing = sqlx::query!(
            "SELECT id FROM locations WHERE 
             ABS(latitude - $1) < 0.001 AND ABS(longitude - $2) < 0.001",
            location.latitude as f64,
            location.longitude as f64
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| SanadError::Database(e.to_string()))?;
        
        if let Some(row) = existing {
            return Ok(row.id);
        }
        
        // Create new location
        let location_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO locations (id, name, city, country, latitude, longitude, timezone)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            location_id,
            location.city.as_deref().unwrap_or("Unknown"),
            location.city.as_deref(),
            location.country.as_deref(),
            location.latitude as f64,
            location.longitude as f64,
            location.timezone
        )
        .execute(&self.pool)
        .await
        .map_err(|e| SanadError::Database(e.to_string()))?;
        
        Ok(location_id)
    }
    
    /// Get daily prayer times from cache
    pub async fn get_daily_prayer_times(
        &self,
        location_id: Uuid,
        method: &CalculationMethod,
        date: NaiveDate,
    ) -> SanadResult<Option<DailyPrayerTimes>> {
        let method_str = Self::calculation_method_to_string(method);
        
        let row = sqlx::query!(
            "SELECT * FROM daily_prayer_times 
             WHERE location_id = $1 AND calculation_method = $2 AND date = $3",
            location_id,
            method_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| SanadError::Database(e.to_string()))?;
        
        if let Some(row) = row {
            Ok(Some(DailyPrayerTimes {
                id: row.id,
                location_id: row.location_id,
                calculation_method: Self::string_to_calculation_method(&row.calculation_method)?,
                date: row.date,
                fajr_time: row.fajr_time,
                sunrise_time: row.sunrise_time,
                dhuhr_time: row.dhuhr_time,
                asr_time: row.asr_time,
                maghrib_time: row.maghrib_time,
                isha_time: row.isha_time,
                qibla_direction: row.qibla_direction.unwrap_or(0.0),
                fajr_angle: row.fajr_angle,
                maghrib_angle: row.maghrib_angle,
                isha_angle: row.isha_angle,
                asr_method: row.asr_method.unwrap_or(1),
                created_at: row.created_at.unwrap_or_else(|| chrono::Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Save daily prayer times to cache
    pub async fn save_daily_prayer_times(&self, times: &DailyPrayerTimes) -> SanadResult<()> {
        let method_str = Self::calculation_method_to_string(&times.calculation_method);
        
        sqlx::query!(
            "INSERT INTO daily_prayer_times 
             (id, location_id, calculation_method, date, fajr_time, sunrise_time, 
              dhuhr_time, asr_time, maghrib_time, isha_time, qibla_direction,
              fajr_angle, maghrib_angle, isha_angle, asr_method, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
             ON CONFLICT (location_id, calculation_method, date) DO UPDATE SET
             fajr_time = EXCLUDED.fajr_time,
             sunrise_time = EXCLUDED.sunrise_time,
             dhuhr_time = EXCLUDED.dhuhr_time,
             asr_time = EXCLUDED.asr_time,
             maghrib_time = EXCLUDED.maghrib_time,
             isha_time = EXCLUDED.isha_time,
             qibla_direction = EXCLUDED.qibla_direction",
            times.id,
            times.location_id,
            method_str,
            times.date,
            times.fajr_time,
            times.sunrise_time,
            times.dhuhr_time,
            times.asr_time,
            times.maghrib_time,
            times.isha_time,
            times.qibla_direction,
            times.fajr_angle,
            times.maghrib_angle,
            times.isha_angle,
            times.asr_method,
            times.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| SanadError::Database(e.to_string()))?;
        
        Ok(())
    }
    
    /// Get Hijri conversion from cache
    pub async fn get_hijri_conversion(&self, date: NaiveDate) -> SanadResult<Option<HijriGregorianConversion>> {
        let row = sqlx::query!(
            "SELECT * FROM hijri_gregorian_conversion WHERE gregorian_date = $1",
            date
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| SanadError::Database(e.to_string()))?;
        
        if let Some(row) = row {
            Ok(Some(HijriGregorianConversion {
                id: row.id,
                gregorian_date: row.gregorian_date,
                hijri_year: row.hijri_year,
                hijri_month: row.hijri_month,
                hijri_day: row.hijri_day,
                julian_day_number: row.julian_day_number,
                created_at: row.created_at.unwrap_or_else(|| chrono::Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Save Hijri conversion to cache
    pub async fn save_hijri_conversion(&self, conversion: &HijriGregorianConversion) -> SanadResult<()> {
        sqlx::query!(
            "INSERT INTO hijri_gregorian_conversion 
             (id, gregorian_date, hijri_year, hijri_month, hijri_day, julian_day_number, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             ON CONFLICT (gregorian_date) DO NOTHING",
            conversion.id,
            conversion.gregorian_date,
            conversion.hijri_year,
            conversion.hijri_month,
            conversion.hijri_day,
            conversion.julian_day_number,
            conversion.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| SanadError::Database(e.to_string()))?;
        
        Ok(())
    }
    
    /// Get Islamic events for a specific date
    pub async fn get_islamic_events_for_date(
        &self,
        hijri_month: i32,
        hijri_day: i32,
    ) -> SanadResult<Vec<IslamicEventDetails>> {
        let rows = sqlx::query!(
            "SELECT * FROM islamic_events 
             WHERE hijri_month = $1 AND hijri_day = $2 AND notification_enabled = true
             ORDER BY importance_level DESC",
            hijri_month,
            hijri_day
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| SanadError::Database(e.to_string()))?;
        
        let mut events = Vec::new();
        for row in rows {
            events.push(IslamicEventDetails {
                id: row.id,
                name_arabic: row.name_arabic,
                name_english: row.name_english,
                description_arabic: row.description_arabic,
                description_english: row.description_english,
                hijri_month: row.hijri_month,
                hijri_day: row.hijri_day,
                hijri_end_month: row.hijri_end_month,
                hijri_end_day: row.hijri_end_day,
                event_type: row.event_type,
                importance_level: row.importance_level,
                notification_enabled: row.notification_enabled,
                special_calculation: row.special_calculation,
                created_at: row.created_at.unwrap_or_else(|| chrono::Utc::now()),
                updated_at: row.updated_at.unwrap_or_else(|| chrono::Utc::now()),
            });
        }
        
        Ok(events)
    }
    
    /// Get Islamic events for a month
    pub async fn get_islamic_events_for_month(&self, hijri_month: i32) -> SanadResult<Vec<IslamicEventDetails>> {
        let rows = sqlx::query!(
            "SELECT * FROM islamic_events 
             WHERE hijri_month = $1 AND notification_enabled = true
             ORDER BY hijri_day ASC, importance_level DESC",
            hijri_month
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| SanadError::Database(e.to_string()))?;
        
        let mut events = Vec::new();
        for row in rows {
            events.push(IslamicEventDetails {
                id: row.id,
                name_arabic: row.name_arabic,
                name_english: row.name_english,
                description_arabic: row.description_arabic,
                description_english: row.description_english,
                hijri_month: row.hijri_month,
                hijri_day: row.hijri_day,
                hijri_end_month: row.hijri_end_month,
                hijri_end_day: row.hijri_end_day,
                event_type: row.event_type,
                importance_level: row.importance_level,
                notification_enabled: row.notification_enabled,
                special_calculation: row.special_calculation,
                created_at: row.created_at.unwrap_or_else(|| chrono::Utc::now()),
                updated_at: row.updated_at.unwrap_or_else(|| chrono::Utc::now()),
            });
        }
        
        Ok(events)
    }
    
    /// Get Hijri months
    pub async fn get_hijri_months(&self) -> SanadResult<Vec<HijriMonth>> {
        let rows = sqlx::query!(
            "SELECT * FROM hijri_months ORDER BY month_number"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| SanadError::Database(e.to_string()))?;
        
        let mut months = Vec::new();
        for row in rows {
            months.push(HijriMonth {
                month_number: row.month_number,
                name_arabic: row.name_arabic,
                name_english: row.name_english,
                name_transliteration: row.name_transliteration,
            });
        }
        
        Ok(months)
    }
    
    /// Get user prayer preferences
    pub async fn get_user_prayer_preferences(&self, user_id: Uuid) -> SanadResult<UserPrayerPreferences> {
        let row = sqlx::query!(
            "SELECT * FROM user_prayer_preferences WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| SanadError::Database(e.to_string()))?;
        
        if let Some(row) = row {
            Ok(UserPrayerPreferences {
                id: row.id,
                user_id: row.user_id,
                fajr_notification_enabled: row.fajr_notification_enabled,
                fajr_notification_minutes: row.fajr_notification_minutes,
                dhuhr_notification_enabled: row.dhuhr_notification_enabled,
                dhuhr_notification_minutes: row.dhuhr_notification_minutes,
                asr_notification_enabled: row.asr_notification_enabled,
                asr_notification_minutes: row.asr_notification_minutes,
                maghrib_notification_enabled: row.maghrib_notification_enabled,
                maghrib_notification_minutes: row.maghrib_notification_minutes,
                isha_notification_enabled: row.isha_notification_enabled,
                isha_notification_minutes: row.isha_notification_minutes,
                sunrise_notification_enabled: row.sunrise_notification_enabled,
                sunrise_notification_minutes: row.sunrise_notification_minutes,
                graduated_notifications_enabled: row.graduated_notifications_enabled,
                graduated_intervals: row.graduated_intervals.unwrap_or_default(),
                show_qibla_direction: row.show_qibla_direction,
                qibla_compass_style: row.qibla_compass_style,
                created_at: row.created_at.unwrap_or_else(|| chrono::Utc::now()),
                updated_at: row.updated_at.unwrap_or_else(|| chrono::Utc::now()),
            })
        } else {
            Err(SanadError::NotFound("User prayer preferences not found".to_string()))
        }
    }

    /// Create user prayer preferences
    pub async fn create_user_prayer_preferences(&self, preferences: UserPrayerPreferences) -> SanadResult<UserPrayerPreferences> {
        sqlx::query!(
            r#"
            INSERT INTO user_prayer_preferences (
                id, user_id,
                fajr_notification_enabled, fajr_notification_minutes,
                dhuhr_notification_enabled, dhuhr_notification_minutes,
                asr_notification_enabled, asr_notification_minutes,
                maghrib_notification_enabled, maghrib_notification_minutes,
                isha_notification_enabled, isha_notification_minutes,
                sunrise_notification_enabled, sunrise_notification_minutes,
                graduated_notifications_enabled, graduated_intervals,
                show_qibla_direction, qibla_compass_style,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20
            )
            "#,
            preferences.id,
            preferences.user_id,
            preferences.fajr_notification_enabled,
            preferences.fajr_notification_minutes,
            preferences.dhuhr_notification_enabled,
            preferences.dhuhr_notification_minutes,
            preferences.asr_notification_enabled,
            preferences.asr_notification_minutes,
            preferences.maghrib_notification_enabled,
            preferences.maghrib_notification_minutes,
            preferences.isha_notification_enabled,
            preferences.isha_notification_minutes,
            preferences.sunrise_notification_enabled,
            preferences.sunrise_notification_minutes,
            preferences.graduated_notifications_enabled,
            &preferences.graduated_intervals,
            preferences.show_qibla_direction,
            preferences.qibla_compass_style,
            preferences.created_at,
            preferences.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| SanadError::Database(e.to_string()))?;

        Ok(preferences)
    }

    /// Update user prayer preferences
    pub async fn update_user_prayer_preferences(&self, user_id: Uuid, preferences: UserPrayerPreferences) -> SanadResult<UserPrayerPreferences> {
        sqlx::query!(
            r#"
            UPDATE user_prayer_preferences SET
                fajr_notification_enabled = $2,
                fajr_notification_minutes = $3,
                dhuhr_notification_enabled = $4,
                dhuhr_notification_minutes = $5,
                asr_notification_enabled = $6,
                asr_notification_minutes = $7,
                maghrib_notification_enabled = $8,
                maghrib_notification_minutes = $9,
                isha_notification_enabled = $10,
                isha_notification_minutes = $11,
                sunrise_notification_enabled = $12,
                sunrise_notification_minutes = $13,
                graduated_notifications_enabled = $14,
                graduated_intervals = $15,
                show_qibla_direction = $16,
                qibla_compass_style = $17,
                updated_at = $18
            WHERE user_id = $1
            "#,
            user_id,
            preferences.fajr_notification_enabled,
            preferences.fajr_notification_minutes,
            preferences.dhuhr_notification_enabled,
            preferences.dhuhr_notification_minutes,
            preferences.asr_notification_enabled,
            preferences.asr_notification_minutes,
            preferences.maghrib_notification_enabled,
            preferences.maghrib_notification_minutes,
            preferences.isha_notification_enabled,
            preferences.isha_notification_minutes,
            preferences.sunrise_notification_enabled,
            preferences.sunrise_notification_minutes,
            preferences.graduated_notifications_enabled,
            &preferences.graduated_intervals,
            preferences.show_qibla_direction,
            preferences.qibla_compass_style,
            preferences.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| SanadError::Database(e.to_string()))?;

        Ok(preferences)
    }
    
    // Helper methods for enum conversion
    
    fn calculation_method_to_string(method: &CalculationMethod) -> String {
        match method {
            CalculationMethod::MuslimWorldLeague => "muslim_world_league".to_string(),
            CalculationMethod::IslamicSocietyOfNorthAmerica => "islamic_society_north_america".to_string(),
            CalculationMethod::EgyptianGeneralAuthorityOfSurvey => "egyptian_general_authority".to_string(),
            CalculationMethod::UmmAlQuraUniversityMakkah => "umm_al_qura_makkah".to_string(),
            CalculationMethod::UniversityOfIslamicSciencesKarachi => "university_islamic_sciences_karachi".to_string(),
            CalculationMethod::InstituteOfGeophysicsUniversityOfTehran => "institute_geophysics_tehran".to_string(),
            CalculationMethod::Shia => "shia".to_string(),
            CalculationMethod::Custom { .. } => "custom".to_string(),
        }
    }
    
    fn string_to_calculation_method(method_str: &str) -> SanadResult<CalculationMethod> {
        match method_str {
            "muslim_world_league" => Ok(CalculationMethod::MuslimWorldLeague),
            "islamic_society_north_america" => Ok(CalculationMethod::IslamicSocietyOfNorthAmerica),
            "egyptian_general_authority" => Ok(CalculationMethod::EgyptianGeneralAuthorityOfSurvey),
            "umm_al_qura_makkah" => Ok(CalculationMethod::UmmAlQuraUniversityMakkah),
            "university_islamic_sciences_karachi" => Ok(CalculationMethod::UniversityOfIslamicSciencesKarachi),
            "institute_geophysics_tehran" => Ok(CalculationMethod::InstituteOfGeophysicsUniversityOfTehran),
            "shia" => Ok(CalculationMethod::Shia),
            "custom" => Ok(CalculationMethod::Custom {
                fajr_angle: 18.0,
                maghrib_angle: 0.0,
                isha_angle: 17.0,
            }),
            _ => Err(SanadError::Validation(format!("Unknown calculation method: {}", method_str))),
        }
    }
}
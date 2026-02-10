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
        let existing = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM locations WHERE 
             ABS(latitude - $1) < 0.001 AND ABS(longitude - $2) < 0.001"
        )
        .bind(location.latitude as f64)
        .bind(location.longitude as f64)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(id) = existing {
            return Ok(id);
        }
        
        // Create new location
        let location_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO locations (id, name, city, country, latitude, longitude, timezone)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(location_id)
        .bind(location.city.as_deref().unwrap_or("Unknown"))
        .bind(location.city.as_deref())
        .bind(location.country.as_deref())
        .bind(location.latitude as f64)
        .bind(location.longitude as f64)
        .bind(&location.timezone)
        .execute(&self.pool)
        .await?;
        
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
        
        let row = sqlx::query(
            "SELECT id, location_id, calculation_method, date, fajr_time, sunrise_time, 
             dhuhr_time, asr_time, maghrib_time, isha_time, qibla_direction, 
             fajr_angle, maghrib_angle, isha_angle, asr_method, created_at
             FROM daily_prayer_times 
             WHERE location_id = $1 AND calculation_method = $2 AND date = $3"
        )
        .bind(location_id)
        .bind(&method_str)
        .bind(date)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = row {
            Ok(Some(DailyPrayerTimes {
                id: row.try_get("id")?,
                location_id: row.try_get("location_id")?,
                calculation_method: Self::string_to_calculation_method(row.try_get("calculation_method")?)?,
                date: row.try_get("date")?,
                fajr_time: row.try_get("fajr_time")?,
                sunrise_time: row.try_get("sunrise_time")?,
                dhuhr_time: row.try_get("dhuhr_time")?,
                asr_time: row.try_get("asr_time")?,
                maghrib_time: row.try_get("maghrib_time")?,
                isha_time: row.try_get("isha_time")?,
                qibla_direction: row.try_get("qibla_direction").unwrap_or(0.0),
                fajr_angle: row.try_get("fajr_angle")?,
                maghrib_angle: row.try_get("maghrib_angle")?,
                isha_angle: row.try_get("isha_angle")?,
                asr_method: row.try_get("asr_method").unwrap_or(1),
                created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Save daily prayer times to cache
    pub async fn save_daily_prayer_times(&self, times: &DailyPrayerTimes) -> SanadResult<()> {
        let method_str = Self::calculation_method_to_string(&times.calculation_method);
        
        sqlx::query(
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
             qibla_direction = EXCLUDED.qibla_direction"
        )
        .bind(times.id)
        .bind(times.location_id)
        .bind(&method_str)
        .bind(times.date)
        .bind(times.fajr_time)
        .bind(times.sunrise_time)
        .bind(times.dhuhr_time)
        .bind(times.asr_time)
        .bind(times.maghrib_time)
        .bind(times.isha_time)
        .bind(times.qibla_direction)
        .bind(times.fajr_angle)
        .bind(times.maghrib_angle)
        .bind(times.isha_angle)
        .bind(times.asr_method)
        .bind(times.created_at)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get Hijri conversion from cache
    pub async fn get_hijri_conversion(&self, date: NaiveDate) -> SanadResult<Option<HijriGregorianConversion>> {
        let row = sqlx::query(
            "SELECT id, gregorian_date, hijri_year, hijri_month, hijri_day, julian_day_number, created_at 
             FROM hijri_gregorian_conversion WHERE gregorian_date = $1"
        )
        .bind(date)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = row {
            Ok(Some(HijriGregorianConversion {
                id: row.try_get("id")?,
                gregorian_date: row.try_get("gregorian_date")?,
                hijri_year: row.try_get("hijri_year")?,
                hijri_month: row.try_get("hijri_month")?,
                hijri_day: row.try_get("hijri_day")?,
                julian_day_number: row.try_get("julian_day_number")?,
                created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Save Hijri conversion to cache
    pub async fn save_hijri_conversion(&self, conversion: &HijriGregorianConversion) -> SanadResult<()> {
        sqlx::query(
            "INSERT INTO hijri_gregorian_conversion 
             (id, gregorian_date, hijri_year, hijri_month, hijri_day, julian_day_number, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             ON CONFLICT (gregorian_date) DO NOTHING"
        )
        .bind(conversion.id)
        .bind(conversion.gregorian_date)
        .bind(conversion.hijri_year)
        .bind(conversion.hijri_month)
        .bind(conversion.hijri_day)
        .bind(conversion.julian_day_number)
        .bind(conversion.created_at)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get Islamic events for a specific date
    pub async fn get_islamic_events_for_date(
        &self,
        hijri_month: i32,
        hijri_day: i32,
    ) -> SanadResult<Vec<IslamicEventDetails>> {
        let rows = sqlx::query(
            "SELECT id, name_arabic, name_english, description_arabic, description_english, 
             hijri_month, hijri_day, hijri_end_month, hijri_end_day, event_type, 
             importance_level, notification_enabled, special_calculation, created_at, updated_at
             FROM islamic_events 
             WHERE hijri_month = $1 AND hijri_day = $2 AND notification_enabled = true
             ORDER BY importance_level DESC"
        )
        .bind(hijri_month)
        .bind(hijri_day)
        .fetch_all(&self.pool)
        .await?;
        
        let mut events = Vec::new();
        for row in rows {
            events.push(IslamicEventDetails {
                id: row.try_get("id")?,
                name_arabic: row.try_get("name_arabic")?,
                name_english: row.try_get("name_english")?,
                description_arabic: row.try_get("description_arabic")?,
                description_english: row.try_get("description_english")?,
                hijri_month: row.try_get("hijri_month")?,
                hijri_day: row.try_get("hijri_day")?,
                hijri_end_month: row.try_get("hijri_end_month")?,
                hijri_end_day: row.try_get("hijri_end_day")?,
                event_type: row.try_get("event_type")?,
                importance_level: row.try_get("importance_level")?,
                notification_enabled: row.try_get("notification_enabled")?,
                special_calculation: row.try_get("special_calculation")?,
                created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
            });
        }
        
        Ok(events)
    }
    
    /// Get Islamic events for a month
    pub async fn get_islamic_events_for_month(&self, hijri_month: i32) -> SanadResult<Vec<IslamicEventDetails>> {
        let rows = sqlx::query(
            "SELECT id, name_arabic, name_english, description_arabic, description_english, 
             hijri_month, hijri_day, hijri_end_month, hijri_end_day, event_type, 
             importance_level, notification_enabled, special_calculation, created_at, updated_at
             FROM islamic_events 
             WHERE hijri_month = $1 AND notification_enabled = true
             ORDER BY hijri_day ASC, importance_level DESC"
        )
        .bind(hijri_month)
        .fetch_all(&self.pool)
        .await?;
        
        let mut events = Vec::new();
        for row in rows {
            events.push(IslamicEventDetails {
                id: row.try_get("id")?,
                name_arabic: row.try_get("name_arabic")?,
                name_english: row.try_get("name_english")?,
                description_arabic: row.try_get("description_arabic")?,
                description_english: row.try_get("description_english")?,
                hijri_month: row.try_get("hijri_month")?,
                hijri_day: row.try_get("hijri_day")?,
                hijri_end_month: row.try_get("hijri_end_month")?,
                hijri_end_day: row.try_get("hijri_end_day")?,
                event_type: row.try_get("event_type")?,
                importance_level: row.try_get("importance_level")?,
                notification_enabled: row.try_get("notification_enabled")?,
                special_calculation: row.try_get("special_calculation")?,
                created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
            });
        }
        
        Ok(events)
    }
    
    /// Get Hijri months
    pub async fn get_hijri_months(&self) -> SanadResult<Vec<HijriMonth>> {
        let rows = sqlx::query(
            "SELECT month_number, name_arabic, name_english, name_transliteration 
             FROM hijri_months ORDER BY month_number"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut months = Vec::new();
        for row in rows {
            months.push(HijriMonth {
                month_number: row.try_get("month_number")?,
                name_arabic: row.try_get("name_arabic")?,
                name_english: row.try_get("name_english")?,
                name_transliteration: row.try_get("name_transliteration")?,
            });
        }
        
        Ok(months)
    }
    
    /// Get user prayer preferences
    pub async fn get_user_prayer_preferences(&self, user_id: Uuid) -> SanadResult<UserPrayerPreferences> {
        let row = sqlx::query(
            "SELECT id, user_id, fajr_notification_enabled, fajr_notification_minutes, 
             dhuhr_notification_enabled, dhuhr_notification_minutes, asr_notification_enabled, 
             asr_notification_minutes, maghrib_notification_enabled, maghrib_notification_minutes, 
             isha_notification_enabled, isha_notification_minutes, sunrise_notification_enabled, 
             sunrise_notification_minutes, graduated_notifications_enabled, graduated_intervals, 
             show_qibla_direction, qibla_compass_style, created_at, updated_at
             FROM user_prayer_preferences WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = row {
            Ok(UserPrayerPreferences {
                id: row.try_get("id")?,
                user_id: row.try_get("user_id")?,
                fajr_notification_enabled: row.try_get("fajr_notification_enabled")?,
                fajr_notification_minutes: row.try_get("fajr_notification_minutes")?,
                dhuhr_notification_enabled: row.try_get("dhuhr_notification_enabled")?,
                dhuhr_notification_minutes: row.try_get("dhuhr_notification_minutes")?,
                asr_notification_enabled: row.try_get("asr_notification_enabled")?,
                asr_notification_minutes: row.try_get("asr_notification_minutes")?,
                maghrib_notification_enabled: row.try_get("maghrib_notification_enabled")?,
                maghrib_notification_minutes: row.try_get("maghrib_notification_minutes")?,
                isha_notification_enabled: row.try_get("isha_notification_enabled")?,
                isha_notification_minutes: row.try_get("isha_notification_minutes")?,
                sunrise_notification_enabled: row.try_get("sunrise_notification_enabled")?,
                sunrise_notification_minutes: row.try_get("sunrise_notification_minutes")?,
                graduated_notifications_enabled: row.try_get("graduated_notifications_enabled")?,
                graduated_intervals: row.try_get("graduated_intervals").unwrap_or_default(),
                show_qibla_direction: row.try_get("show_qibla_direction")?,
                qibla_compass_style: row.try_get("qibla_compass_style")?,
                created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
            })
        } else {
            Err(SanadError::NotFound("User prayer preferences not found".to_string()))
        }
    }

    /// Create user prayer preferences
    pub async fn create_user_prayer_preferences(&self, preferences: UserPrayerPreferences) -> SanadResult<UserPrayerPreferences> {
        sqlx::query(
            "INSERT INTO user_prayer_preferences (
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
            )"
        )
        .bind(preferences.id)
        .bind(preferences.user_id)
        .bind(preferences.fajr_notification_enabled)
        .bind(preferences.fajr_notification_minutes)
        .bind(preferences.dhuhr_notification_enabled)
        .bind(preferences.dhuhr_notification_minutes)
        .bind(preferences.asr_notification_enabled)
        .bind(preferences.asr_notification_minutes)
        .bind(preferences.maghrib_notification_enabled)
        .bind(preferences.maghrib_notification_minutes)
        .bind(preferences.isha_notification_enabled)
        .bind(preferences.isha_notification_minutes)
        .bind(preferences.sunrise_notification_enabled)
        .bind(preferences.sunrise_notification_minutes)
        .bind(preferences.graduated_notifications_enabled)
        .bind(&preferences.graduated_intervals)
        .bind(preferences.show_qibla_direction)
        .bind(&preferences.qibla_compass_style)
        .bind(preferences.created_at)
        .bind(preferences.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(preferences)
    }

    /// Update user prayer preferences
    pub async fn update_user_prayer_preferences(&self, user_id: Uuid, preferences: UserPrayerPreferences) -> SanadResult<UserPrayerPreferences> {
        sqlx::query(
            "UPDATE user_prayer_preferences SET
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
            WHERE user_id = $1"
        )
        .bind(user_id)
        .bind(preferences.fajr_notification_enabled)
        .bind(preferences.fajr_notification_minutes)
        .bind(preferences.dhuhr_notification_enabled)
        .bind(preferences.dhuhr_notification_minutes)
        .bind(preferences.asr_notification_enabled)
        .bind(preferences.asr_notification_minutes)
        .bind(preferences.maghrib_notification_enabled)
        .bind(preferences.maghrib_notification_minutes)
        .bind(preferences.isha_notification_enabled)
        .bind(preferences.isha_notification_minutes)
        .bind(preferences.sunrise_notification_enabled)
        .bind(preferences.sunrise_notification_minutes)
        .bind(preferences.graduated_notifications_enabled)
        .bind(&preferences.graduated_intervals)
        .bind(preferences.show_qibla_direction)
        .bind(&preferences.qibla_compass_style)
        .bind(preferences.updated_at)
        .execute(&self.pool)
        .await?;

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
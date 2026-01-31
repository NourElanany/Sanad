use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, Utc};
use shared::{SanadResult, SanadError, Location, CalculationMethod};
use crate::models::{
    PrayerCalculationSettings, DailyPrayerTimes, HijriGregorianConversion,
    IslamicEventDetails, UserPrayerPreferences, PrayerTimeHistory,
    HijriMonth,
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
        let existing = sqlx::query!(
            "SELECT id FROM locations WHERE latitude = $1 AND longitude = $2",
            location.latitude,
            location.longitude
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = existing {
            return Ok(row.id);
        }
        
        let location_id = sqlx::query!(
            r#"
            INSERT INTO locations (name, city, country, latitude, longitude, timezone)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            location.city.as_deref().unwrap_or("Unknown"),
            location.city,
            location.country,
            location.latitude,
            location.longitude,
            location.timezone
        )
        .fetch_one(&self.pool)
        .await?
        .id;
        
        Ok(location_id)
    }
    
    /// Get user prayer calculation settings
    pub async fn get_user_prayer_settings(&self, user_id: Uuid) -> SanadResult<Option<PrayerCalculationSettings>> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, location_id, calculation_method, fajr_angle, maghrib_angle, 
                   isha_angle, fajr_adjustment, dhuhr_adjustment, asr_adjustment, 
                   maghrib_adjustment, isha_adjustment, asr_method, high_latitude_adjustment,
                   created_at, updated_at
            FROM prayer_calculation_settings 
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = row {
            let calculation_method = match row.calculation_method.as_str() {
                "muslim_world_league" => CalculationMethod::MuslimWorldLeague,
                "islamic_society_north_america" => CalculationMethod::IslamicSocietyOfNorthAmerica,
                "egyptian_general_authority" => CalculationMethod::EgyptianGeneralAuthorityOfSurvey,
                "umm_al_qura_makkah" => CalculationMethod::UmmAlQuraUniversityMakkah,
                "university_islamic_sciences_karachi" => CalculationMethod::UniversityOfIslamicSciencesKarachi,
                "institute_geophysics_tehran" => CalculationMethod::InstituteOfGeophysicsUniversityOfTehran,
                "shia" => CalculationMethod::Shia,
                "custom" => CalculationMethod::Custom {
                    fajr_angle: row.fajr_angle.unwrap_or(18.0),
                    maghrib_angle: row.maghrib_angle.unwrap_or(0.0),
                    isha_angle: row.isha_angle.unwrap_or(17.0),
                },
                _ => CalculationMethod::MuslimWorldLeague,
            };
            
            Ok(Some(PrayerCalculationSettings {
                id: row.id,
                user_id: row.user_id,
                location_id: row.location_id,
                calculation_method,
                fajr_angle: row.fajr_angle,
                maghrib_angle: row.maghrib_angle,
                isha_angle: row.isha_angle,
                fajr_adjustment: row.fajr_adjustment,
                dhuhr_adjustment: row.dhuhr_adjustment,
                asr_adjustment: row.asr_adjustment,
                maghrib_adjustment: row.maghrib_adjustment,
                isha_adjustment: row.isha_adjustment,
                asr_method: row.asr_method,
                high_latitude_adjustment: row.high_latitude_adjustment,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }))
        } else {
            Ok(None)
        }
    }
}
    /// Save daily prayer times to cache
    pub async fn save_daily_prayer_times(&self, prayer_times: &DailyPrayerTimes) -> SanadResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO daily_prayer_times (
                location_id, calculation_method, date, fajr_time, sunrise_time, 
                dhuhr_time, asr_time, maghrib_time, isha_time, qibla_direction,
                fajr_angle, maghrib_angle, isha_angle, asr_method
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (location_id, calculation_method, date) 
            DO UPDATE SET
                fajr_time = EXCLUDED.fajr_time,
                sunrise_time = EXCLUDED.sunrise_time,
                dhuhr_time = EXCLUDED.dhuhr_time,
                asr_time = EXCLUDED.asr_time,
                maghrib_time = EXCLUDED.maghrib_time,
                isha_time = EXCLUDED.isha_time,
                qibla_direction = EXCLUDED.qibla_direction
            "#,
            prayer_times.location_id,
            String::from(prayer_times.calculation_method.clone()),
            prayer_times.date,
            prayer_times.fajr_time,
            prayer_times.sunrise_time,
            prayer_times.dhuhr_time,
            prayer_times.asr_time,
            prayer_times.maghrib_time,
            prayer_times.isha_time,
            prayer_times.qibla_direction,
            prayer_times.fajr_angle,
            prayer_times.maghrib_angle,
            prayer_times.isha_angle,
            prayer_times.asr_method
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get cached daily prayer times
    pub async fn get_daily_prayer_times(
        &self,
        location_id: Uuid,
        calculation_method: &CalculationMethod,
        date: NaiveDate,
    ) -> SanadResult<Option<DailyPrayerTimes>> {
        let method_str = String::from(calculation_method.clone());
        
        let row = sqlx::query!(
            r#"
            SELECT id, location_id, calculation_method, date, fajr_time, sunrise_time,
                   dhuhr_time, asr_time, maghrib_time, isha_time, qibla_direction,
                   fajr_angle, maghrib_angle, isha_angle, asr_method, created_at
            FROM daily_prayer_times
            WHERE location_id = $1 AND calculation_method = $2 AND date = $3
            "#,
            location_id,
            method_str,
            date
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = row {
            let calculation_method = match row.calculation_method.as_str() {
                "muslim_world_league" => CalculationMethod::MuslimWorldLeague,
                "islamic_society_north_america" => CalculationMethod::IslamicSocietyOfNorthAmerica,
                "egyptian_general_authority" => CalculationMethod::EgyptianGeneralAuthorityOfSurvey,
                "umm_al_qura_makkah" => CalculationMethod::UmmAlQuraUniversityMakkah,
                "university_islamic_sciences_karachi" => CalculationMethod::UniversityOfIslamicSciencesKarachi,
                "institute_geophysics_tehran" => CalculationMethod::InstituteOfGeophysicsUniversityOfTehran,
                "shia" => CalculationMethod::Shia,
                "custom" => CalculationMethod::Custom {
                    fajr_angle: row.fajr_angle.unwrap_or(18.0),
                    maghrib_angle: row.maghrib_angle.unwrap_or(0.0),
                    isha_angle: row.isha_angle.unwrap_or(17.0),
                },
                _ => CalculationMethod::MuslimWorldLeague,
            };
            
            Ok(Some(DailyPrayerTimes {
                id: row.id,
                location_id: row.location_id,
                calculation_method,
                date: row.date,
                fajr_time: row.fajr_time,
                sunrise_time: row.sunrise_time,
                dhuhr_time: row.dhuhr_time,
                asr_time: row.asr_time,
                maghrib_time: row.maghrib_time,
                isha_time: row.isha_time,
                qibla_direction: row.qibla_direction,
                fajr_angle: row.fajr_angle,
                maghrib_angle: row.maghrib_angle,
                isha_angle: row.isha_angle,
                asr_method: row.asr_method,
                created_at: row.created_at,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Get Hijri-Gregorian conversion
    pub async fn get_hijri_conversion(&self, gregorian_date: NaiveDate) -> SanadResult<Option<HijriGregorianConversion>> {
        let row = sqlx::query!(
            r#"
            SELECT id, gregorian_date, hijri_year, hijri_month, hijri_day, 
                   julian_day_number, created_at
            FROM hijri_gregorian_conversion
            WHERE gregorian_date = $1
            "#,
            gregorian_date
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = row {
            Ok(Some(HijriGregorianConversion {
                id: row.id,
                gregorian_date: row.gregorian_date,
                hijri_year: row.hijri_year,
                hijri_month: row.hijri_month,
                hijri_day: row.hijri_day,
                julian_day_number: row.julian_day_number,
                created_at: row.created_at,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Save Hijri-Gregorian conversion
    pub async fn save_hijri_conversion(&self, conversion: &HijriGregorianConversion) -> SanadResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO hijri_gregorian_conversion (
                gregorian_date, hijri_year, hijri_month, hijri_day, julian_day_number
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (gregorian_date) DO NOTHING
            "#,
            conversion.gregorian_date,
            conversion.hijri_year,
            conversion.hijri_month,
            conversion.hijri_day,
            conversion.julian_day_number
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
    /// Get Islamic events for a specific Hijri date
    pub async fn get_islamic_events_for_date(
        &self,
        hijri_month: i32,
        hijri_day: i32,
    ) -> SanadResult<Vec<IslamicEventDetails>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name_arabic, name_english, description_arabic, description_english,
                   hijri_month, hijri_day, hijri_end_month, hijri_end_day, event_type,
                   importance_level, notification_enabled, special_calculation,
                   created_at, updated_at
            FROM islamic_events
            WHERE hijri_month = $1 AND hijri_day = $2 AND notification_enabled = true
            ORDER BY importance_level DESC
            "#,
            hijri_month,
            hijri_day
        )
        .fetch_all(&self.pool)
        .await?;
        
        let events = rows.into_iter().map(|row| IslamicEventDetails {
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
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();
        
        Ok(events)
    }
    
    /// Get Islamic events for a month
    pub async fn get_islamic_events_for_month(
        &self,
        hijri_month: i32,
    ) -> SanadResult<Vec<IslamicEventDetails>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name_arabic, name_english, description_arabic, description_english,
                   hijri_month, hijri_day, hijri_end_month, hijri_end_day, event_type,
                   importance_level, notification_enabled, special_calculation,
                   created_at, updated_at
            FROM islamic_events
            WHERE hijri_month = $1 AND notification_enabled = true
            ORDER BY hijri_day ASC, importance_level DESC
            "#,
            hijri_month
        )
        .fetch_all(&self.pool)
        .await?;
        
        let events = rows.into_iter().map(|row| IslamicEventDetails {
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
            created_at: row.created_at,
            updated_at: row.updated_at,
        }).collect();
        
        Ok(events)
    }
    
    /// Get Hijri months
    pub async fn get_hijri_months(&self) -> SanadResult<Vec<HijriMonth>> {
        let rows = sqlx::query!(
            r#"
            SELECT month_number, name_arabic, name_english, name_transliteration
            FROM hijri_months
            ORDER BY month_number
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        let months = rows.into_iter().map(|row| HijriMonth {
            month_number: row.month_number,
            name_arabic: row.name_arabic,
            name_english: row.name_english,
            name_transliteration: row.name_transliteration,
        }).collect();
        
        Ok(months)
    }
    
    /// Get user prayer preferences
    pub async fn get_user_prayer_preferences(&self, user_id: Uuid) -> SanadResult<Option<UserPrayerPreferences>> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, fajr_notification_enabled, fajr_notification_minutes,
                   dhuhr_notification_enabled, dhuhr_notification_minutes,
                   asr_notification_enabled, asr_notification_minutes,
                   maghrib_notification_enabled, maghrib_notification_minutes,
                   isha_notification_enabled, isha_notification_minutes,
                   sunrise_notification_enabled, sunrise_notification_minutes,
                   graduated_notifications_enabled, graduated_intervals,
                   show_qibla_direction, qibla_compass_style,
                   created_at, updated_at
            FROM user_prayer_preferences
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = row {
            Ok(Some(UserPrayerPreferences {
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
                graduated_intervals: row.graduated_intervals,
                show_qibla_direction: row.show_qibla_direction,
                qibla_compass_style: row.qibla_compass_style,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Save prayer time history
    pub async fn save_prayer_history(&self, history: &PrayerTimeHistory) -> SanadResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO prayer_time_history (
                user_id, prayer_name, scheduled_time, actual_prayer_time,
                location_id, prayer_completed, completion_method,
                prayed_in_congregation, mosque_name
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            history.user_id,
            history.prayer_name,
            history.scheduled_time,
            history.actual_prayer_time,
            history.location_id,
            history.prayer_completed,
            history.completion_method,
            history.prayed_in_congregation,
            history.mosque_name
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
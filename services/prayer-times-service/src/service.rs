use chrono::{NaiveDate, DateTime, Utc};
use uuid::Uuid;
use shared::{SanadResult, SanadError, Location, CalculationMethod, PrayerTimes, HijriDate};
use crate::{
    models::{
        PrayerTimesRequest, QiblaRequest, QiblaDirection, HijriConversionRequest,
        GregorianConversionRequest, IslamicEventsRequest, PrayerTimesResponse,
        CalculationMetadata, AnglesUsed, PrayerAdjustments, MonthlyCalendarResponse,
        CalendarDay, IslamicEventDetails, HijriGregorianConversion, DailyPrayerTimes,
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
        Self { repository }
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
            self.repository.get_islamic_events_for_month(hijri_month).await
        } else {
            // Return all events if no specific month requested
            Ok(vec![]) // TODO: Implement get_all_events if needed
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
}
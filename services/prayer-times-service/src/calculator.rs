use chrono::{DateTime, Utc, NaiveDate, TimeZone, Datelike};
use shared::{Location, CalculationMethod, PrayerTimes};
use crate::models::{PrayerAdjustments, CustomAngles, QiblaDirection};
use std::f64::consts::PI;

/// Prayer times calculator with astronomical algorithms
pub struct PrayerTimesCalculator;

impl PrayerTimesCalculator {
    /// Calculate prayer times for a given location and date
    pub fn calculate_prayer_times(
        location: &Location,
        date: NaiveDate,
        method: &CalculationMethod,
        adjustments: Option<&PrayerAdjustments>,
    ) -> Result<PrayerTimes, Box<dyn std::error::Error>> {
        let calculator = Self::new();
        let julian_day = calculator.get_julian_day(date);
        
        // Get calculation parameters
        let params = calculator.get_calculation_parameters(method);
        
        // Calculate sun times
        let sun_times = calculator.calculate_sun_times(
            julian_day,
            location.latitude,
            location.longitude,
            &params,
        )?;
        
        // Apply adjustments if provided
        let adjusted_times = if let Some(adj) = adjustments {
            calculator.apply_adjustments(&sun_times, adj)
        } else {
            sun_times
        };
        
        // Convert to UTC timestamps
        let prayer_times = calculator.convert_to_utc_times(
            &adjusted_times,
            date,
            &location.timezone,
        )?;
        
        Ok(PrayerTimes {
            fajr: prayer_times.fajr,
            sunrise: prayer_times.sunrise,
            dhuhr: prayer_times.dhuhr,
            asr: prayer_times.asr,
            maghrib: prayer_times.maghrib,
            isha: prayer_times.isha,
            location: location.clone(),
            calculation_method: method.clone(),
        })
    }
    
    /// Calculate Qibla direction from given coordinates
    pub fn calculate_qibla_direction(
        latitude: f64,
        longitude: f64,
    ) -> Result<QiblaDirection, Box<dyn std::error::Error>> {
        // Kaaba coordinates
        const KAABA_LAT: f64 = 21.4224779;
        const KAABA_LNG: f64 = 39.8251832;
        
        let lat_rad = latitude.to_radians();
        let lng_rad = longitude.to_radians();
        let kaaba_lat_rad = KAABA_LAT.to_radians();
        let kaaba_lng_rad = KAABA_LNG.to_radians();
        
        let delta_lng = kaaba_lng_rad - lng_rad;
        
        // Calculate bearing using spherical trigonometry
        let y = delta_lng.sin() * kaaba_lat_rad.cos();
        let x = lat_rad.cos() * kaaba_lat_rad.sin() 
              - lat_rad.sin() * kaaba_lat_rad.cos() * delta_lng.cos();
        
        let mut bearing = y.atan2(x).to_degrees();
        
        // Normalize to 0-360 degrees
        if bearing < 0.0 {
            bearing += 360.0;
        }
        
        // Calculate distance to Kaaba
        let distance = Self::calculate_distance(latitude, longitude, KAABA_LAT, KAABA_LNG);
        
        Ok(QiblaDirection::new(bearing, distance))
    }

    pub fn new() -> Self {
        Self
    }
    
    /// Get Julian day number for a given date
    pub fn get_julian_day(&self, date: NaiveDate) -> f64 {
        let year = date.year() as f64;
        let month = date.month() as f64;
        let day = date.day() as f64;
        
        let a = ((14.0 - month) / 12.0).floor();
        let y = year + 4800.0 - a;
        let m = month + 12.0 * a - 3.0;
        
        day + ((153.0 * m + 2.0) / 5.0).floor() + 365.0 * y + (y / 4.0).floor()
            - (y / 100.0).floor() + (y / 400.0).floor() - 32045.0
    }
    
    /// Get calculation parameters for different methods
    pub fn get_calculation_parameters(&self, method: &CalculationMethod) -> CalculationParams {
        match method {
            CalculationMethod::MuslimWorldLeague => CalculationParams {
                fajr_angle: 18.0,
                maghrib_angle: 0.0,
                isha_angle: 17.0,
                asr_method: 1,
            },
            CalculationMethod::IslamicSocietyOfNorthAmerica => CalculationParams {
                fajr_angle: 15.0,
                maghrib_angle: 0.0,
                isha_angle: 15.0,
                asr_method: 1,
            },
            CalculationMethod::EgyptianGeneralAuthorityOfSurvey => CalculationParams {
                fajr_angle: 19.5,
                maghrib_angle: 0.0,
                isha_angle: 17.5,
                asr_method: 1,
            },
            CalculationMethod::UmmAlQuraUniversityMakkah => CalculationParams {
                fajr_angle: 18.5,
                maghrib_angle: 0.0,
                isha_angle: 0.0, // 90 minutes after Maghrib
                asr_method: 1,
            },
            CalculationMethod::UniversityOfIslamicSciencesKarachi => CalculationParams {
                fajr_angle: 18.0,
                maghrib_angle: 0.0,
                isha_angle: 18.0,
                asr_method: 2, // Hanafi
            },
            CalculationMethod::InstituteOfGeophysicsUniversityOfTehran => CalculationParams {
                fajr_angle: 17.7,
                maghrib_angle: 4.5,
                isha_angle: 14.0,
                asr_method: 1,
            },
            CalculationMethod::Shia => CalculationParams {
                fajr_angle: 16.0,
                maghrib_angle: 4.0,
                isha_angle: 14.0,
                asr_method: 1,
            },
            CalculationMethod::Custom { fajr_angle, maghrib_angle, isha_angle } => {
                CalculationParams {
                    fajr_angle: *fajr_angle,
                    maghrib_angle: *maghrib_angle,
                    isha_angle: *isha_angle,
                    asr_method: 1,
                }
            }
        }
    }
    
    /// Calculate sun times using astronomical algorithms
    fn calculate_sun_times(
        &self,
        julian_day: f64,
        latitude: f64,
        longitude: f64,
        params: &CalculationParams,
    ) -> Result<SunTimes, Box<dyn std::error::Error>> {
        let lat_rad = latitude.to_radians();
        
        // Calculate equation of time and solar declination
        let n = julian_day - 2451545.0;
        let l = (280.460 + 0.9856474 * n) % 360.0;
        let g = (357.528 + 0.9856003 * n).to_radians();
        let lambda = (l + 1.915 * g.sin() + 0.020 * (2.0 * g).sin()).to_radians();
        
        let alpha = lambda.cos().atan2(0.91746 * lambda.sin());
        let delta = (0.39782 * lambda.sin()).asin();
        
        let equation_of_time = 4.0 * (l.to_radians() - alpha);
        
        // Calculate prayer times
        let dhuhr_time = 12.0 - longitude / 15.0 - equation_of_time / 60.0;
        
        // Fajr
        let fajr_angle_rad = params.fajr_angle.to_radians();
        let fajr_time = self.calculate_time_for_angle(
            dhuhr_time, lat_rad, delta, fajr_angle_rad, false
        );
        
        // Sunrise
        let sunrise_angle_rad = (0.833_f64).to_radians(); // 50 arcminutes
        let sunrise_time = self.calculate_time_for_angle(
            dhuhr_time, lat_rad, delta, sunrise_angle_rad, false
        );
        
        // Asr
        let asr_time = self.calculate_asr_time(
            dhuhr_time, lat_rad, delta, params.asr_method
        );
        
        // Maghrib
        let maghrib_time = if params.maghrib_angle > 0.0 {
            let maghrib_angle_rad = params.maghrib_angle.to_radians();
            self.calculate_time_for_angle(
                dhuhr_time, lat_rad, delta, maghrib_angle_rad, true
            )
        } else {
            self.calculate_time_for_angle(
                dhuhr_time, lat_rad, delta, sunrise_angle_rad, true
            )
        };
        
        // Isha
        let isha_time = if params.isha_angle > 0.0 {
            let isha_angle_rad = params.isha_angle.to_radians();
            self.calculate_time_for_angle(
                dhuhr_time, lat_rad, delta, isha_angle_rad, true
            )
        } else {
            // For Umm al-Qura method: 90 minutes after Maghrib
            maghrib_time + 1.5
        };
        
        Ok(SunTimes {
            fajr: fajr_time,
            sunrise: sunrise_time,
            dhuhr: dhuhr_time,
            asr: asr_time,
            maghrib: maghrib_time,
            isha: isha_time,
        })
    }
    /// Calculate time for a given sun angle
    fn calculate_time_for_angle(
        &self,
        dhuhr_time: f64,
        lat_rad: f64,
        delta: f64,
        angle_rad: f64,
        is_evening: bool,
    ) -> f64 {
        // Standard formula: cos(H) = (sin(h) - sin(φ)sin(δ)) / (cos(φ)cos(δ))
        // where h is the altitude angle (negative for depression angles)
        let altitude = if is_evening {
            -angle_rad // Depression angle below horizon
        } else {
            -angle_rad // Depression angle below horizon for Fajr
        };
        
        let cos_hour_angle = (altitude.sin() - lat_rad.sin() * delta.sin()) 
                           / (lat_rad.cos() * delta.cos());
        
        if cos_hour_angle.abs() > 1.0 {
            // Handle extreme latitudes - sun doesn't reach required angle
            return if is_evening { 18.0 } else { 6.0 };
        }
        
        let hour_angle = cos_hour_angle.acos() * 12.0 / PI;
        
        if is_evening {
            dhuhr_time + hour_angle
        } else {
            dhuhr_time - hour_angle
        }
    }
    
    /// Calculate Asr time based on shadow length
    fn calculate_asr_time(
        &self,
        dhuhr_time: f64,
        lat_rad: f64,
        delta: f64,
        asr_method: i32,
    ) -> f64 {
        let shadow_factor = if asr_method == 2 { 2.0 } else { 1.0 }; // Hanafi vs Shafi
        
        let cot_alpha = shadow_factor + (lat_rad - delta).tan().abs();
        let cos_hour_angle = (cot_alpha.recip() - lat_rad.sin() * delta.sin()) 
                           / (lat_rad.cos() * delta.cos());
        
        if cos_hour_angle.abs() > 1.0 {
            return dhuhr_time + 3.0; // Default fallback
        }
        
        let hour_angle = cos_hour_angle.acos() * 12.0 / PI;
        dhuhr_time + hour_angle
    }
    
    /// Apply prayer time adjustments
    fn apply_adjustments(&self, times: &SunTimes, adjustments: &PrayerAdjustments) -> SunTimes {
        SunTimes {
            fajr: times.fajr + adjustments.fajr as f64 / 60.0,
            sunrise: times.sunrise,
            dhuhr: times.dhuhr + adjustments.dhuhr as f64 / 60.0,
            asr: times.asr + adjustments.asr as f64 / 60.0,
            maghrib: times.maghrib + adjustments.maghrib as f64 / 60.0,
            isha: times.isha + adjustments.isha as f64 / 60.0,
        }
    }
    
    /// Convert decimal hours to UTC timestamps
    fn convert_to_utc_times(
        &self,
        times: &SunTimes,
        date: NaiveDate,
        timezone: &str,
    ) -> Result<PrayerTimes, Box<dyn std::error::Error>> {
        let tz: chrono_tz::Tz = timezone.parse()?;
        
        let convert_time = |decimal_hour: f64| -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
            // Handle edge cases where calculation failed
            if decimal_hour < 0.0 || decimal_hour >= 24.0 {
                return Err("Invalid decimal hour".into());
            }
            
            let hours = decimal_hour.floor() as u32;
            let minutes = ((decimal_hour - hours as f64) * 60.0).round() as u32;
            
            // Ensure valid time components
            if hours >= 24 || minutes >= 60 {
                return Err("Invalid time components".into());
            }
            
            let local_time = date.and_hms_opt(hours, minutes, 0)
                .ok_or("Invalid time")?;
            let local_dt = tz.from_local_datetime(&local_time)
                .single()
                .ok_or("Ambiguous local time")?;
            
            Ok(local_dt.with_timezone(&Utc))
        };
        
        Ok(PrayerTimes {
            fajr: convert_time(times.fajr)?,
            sunrise: convert_time(times.sunrise)?,
            dhuhr: convert_time(times.dhuhr)?,
            asr: convert_time(times.asr)?,
            maghrib: convert_time(times.maghrib)?,
            isha: convert_time(times.isha)?,
            location: Location {
                latitude: 0.0,
                longitude: 0.0,
                timezone: timezone.to_string(),
                city: None,
                country: None,
            },
            calculation_method: CalculationMethod::MuslimWorldLeague,
        })
    }
    
    /// Calculate distance between two points using Haversine formula
    fn calculate_distance(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;
        
        let lat1_rad = lat1.to_radians();
        let lat2_rad = lat2.to_radians();
        let delta_lat = (lat2 - lat1).to_radians();
        let delta_lng = (lng2 - lng1).to_radians();
        
        let a = (delta_lat / 2.0).sin().powi(2) 
              + lat1_rad.cos() * lat2_rad.cos() * (delta_lng / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        EARTH_RADIUS_KM * c
    }
}

/// Calculation parameters for different methods
#[derive(Debug, Clone)]
pub struct CalculationParams {
    pub fajr_angle: f64,
    pub maghrib_angle: f64,
    pub isha_angle: f64,
    pub asr_method: i32,
}

/// Sun times in decimal hours
#[derive(Debug, Clone)]
struct SunTimes {
    fajr: f64,
    sunrise: f64,
    dhuhr: f64,
    asr: f64,
    maghrib: f64,
    isha: f64,
}
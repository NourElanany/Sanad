use axum::{
    extract::{Query, Path},
    response::Json,
    Extension,
};
use serde::Deserialize;
use chrono::NaiveDate;
use shared::{ApiResponse, AppError};
use crate::{
    models::{
        PrayerTimesRequest, QiblaRequest, HijriConversionRequest,
        GregorianConversionRequest, IslamicEventsRequest,
    },
    service::PrayerTimesService,
};

/// Query parameters for prayer times calculation
#[derive(Debug, Deserialize)]
pub struct PrayerTimesQuery {
    pub latitude: f64,
    pub longitude: f64,
    pub date: Option<NaiveDate>,
    pub timezone: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub method: Option<String>,
}

/// Query parameters for Qibla direction
#[derive(Debug, Deserialize)]
pub struct QiblaQuery {
    pub latitude: f64,
    pub longitude: f64,
}

/// Query parameters for Hijri conversion
#[derive(Debug, Deserialize)]
pub struct HijriQuery {
    pub date: NaiveDate,
}

/// Query parameters for Gregorian conversion
#[derive(Debug, Deserialize)]
pub struct GregorianQuery {
    pub hijri_year: i32,
    pub hijri_month: i32,
    pub hijri_day: i32,
}

/// Query parameters for Islamic events
#[derive(Debug, Deserialize)]
pub struct EventsQuery {
    pub hijri_month: Option<i32>,
    pub hijri_year: Option<i32>,
    pub importance_level: Option<i32>,
    pub event_type: Option<String>,
}

/// Calculate prayer times for a location and date
pub async fn calculate_prayer_times(
    Query(params): Query<PrayerTimesQuery>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<crate::models::PrayerTimesResponse>>, AppError> {
    let location = shared::Location {
        latitude: params.latitude,
        longitude: params.longitude,
        timezone: params.timezone.unwrap_or_else(|| "UTC".to_string()),
        city: params.city,
        country: params.country,
    };
    
    let calculation_method = params.method.as_deref().map(|m| match m {
        "muslim_world_league" => shared::CalculationMethod::MuslimWorldLeague,
        "islamic_society_north_america" => shared::CalculationMethod::IslamicSocietyOfNorthAmerica,
        "egyptian_general_authority" => shared::CalculationMethod::EgyptianGeneralAuthorityOfSurvey,
        "umm_al_qura_makkah" => shared::CalculationMethod::UmmAlQuraUniversityMakkah,
        "university_islamic_sciences_karachi" => shared::CalculationMethod::UniversityOfIslamicSciencesKarachi,
        "institute_geophysics_tehran" => shared::CalculationMethod::InstituteOfGeophysicsUniversityOfTehran,
        "shia" => shared::CalculationMethod::Shia,
        _ => shared::CalculationMethod::MuslimWorldLeague,
    });
    
    let request = PrayerTimesRequest {
        location,
        date: params.date.unwrap_or_else(|| chrono::Utc::now().date_naive()),
        calculation_method,
        custom_angles: None,
        adjustments: None,
    };
    
    let result = service.calculate_prayer_times(request).await?;
    Ok(Json(ApiResponse::success(result)))
}

/// Calculate Qibla direction
pub async fn calculate_qibla_direction(
    Query(params): Query<QiblaQuery>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<crate::models::QiblaDirection>>, AppError> {
    let request = QiblaRequest {
        latitude: params.latitude,
        longitude: params.longitude,
    };
    
    let result = service.calculate_qibla_direction(request).await?;
    Ok(Json(ApiResponse::success(result)))
}

/// Convert Gregorian date to Hijri
pub async fn gregorian_to_hijri(
    Query(params): Query<HijriQuery>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<shared::HijriDate>>, AppError> {
    let request = HijriConversionRequest {
        date: params.date,
    };
    
    let result = service.gregorian_to_hijri(request).await?;
    Ok(Json(ApiResponse::success(result)))
}

/// Convert Hijri date to Gregorian
pub async fn hijri_to_gregorian(
    Query(params): Query<GregorianQuery>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<NaiveDate>>, AppError> {
    let request = GregorianConversionRequest {
        hijri_year: params.hijri_year,
        hijri_month: params.hijri_month,
        hijri_day: params.hijri_day,
    };
    
    let result = service.hijri_to_gregorian(request).await?;
    Ok(Json(ApiResponse::success(result)))
}

/// Get Islamic events
pub async fn get_islamic_events(
    Query(params): Query<EventsQuery>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<Vec<crate::models::IslamicEventDetails>>>, AppError> {
    let request = IslamicEventsRequest {
        hijri_month: params.hijri_month,
        hijri_year: params.hijri_year,
        importance_level: params.importance_level,
        event_type: params.event_type,
    };
    
    let result = service.get_islamic_events(request).await?;
    Ok(Json(ApiResponse::success(result)))
}

/// Get monthly Islamic calendar
pub async fn get_monthly_calendar(
    Path((hijri_year, hijri_month)): Path<(i32, i32)>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<crate::models::MonthlyCalendarResponse>>, AppError> {
    let result = service.get_monthly_calendar(hijri_year, hijri_month).await?;
    Ok(Json(ApiResponse::success(result)))
}

/// Health check endpoint
pub async fn health_check() -> Json<ApiResponse<std::collections::HashMap<String, String>>> {
    let mut status = std::collections::HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "prayer-times-service".to_string());
    status.insert("version".to_string(), "1.0.0".to_string());
    Json(ApiResponse::success(status))
}
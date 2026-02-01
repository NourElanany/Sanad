use axum::{
    extract::{Query, Path},
    response::Json,
    Extension,
};
use serde::Deserialize;
use chrono::NaiveDate;
use uuid::Uuid;
use shared::{ApiResponse, AppError, Location};
use crate::{
    models::{
        PrayerTimesRequest, QiblaRequest, HijriConversionRequest,
        GregorianConversionRequest, IslamicEventsRequest, NotificationScheduleRequest,
        PrayerNotificationSettings, UserPrayerPreferences,
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
    pub hijri_day: Option<i32>,
    pub importance_level: Option<i32>,
    pub event_type: Option<String>,
}

/// Query parameters for notification scheduling
#[derive(Debug, Deserialize)]
pub struct NotificationQuery {
    pub user_id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub days_ahead: Option<i32>,
}

/// Query parameters for user preferences
#[derive(Debug, Deserialize)]
pub struct UserPreferencesQuery {
    pub user_id: Uuid,
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
    
    let result = service.calculate_prayer_times(request).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
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
    
    let result = service.calculate_qibla_direction(request).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
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
    
    let result = service.gregorian_to_hijri(request).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
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
    
    let result = service.hijri_to_gregorian(request).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
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
        hijri_day: params.hijri_day,
        importance_level: params.importance_level,
        event_type: params.event_type,
    };
    
    let result = service.get_islamic_events(request).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(ApiResponse::success(result)))
}

/// Get monthly Islamic calendar
pub async fn get_monthly_calendar(
    Path((hijri_year, hijri_month)): Path<(i32, i32)>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<crate::models::MonthlyCalendarResponse>>, AppError> {
    let result = service.get_monthly_calendar(hijri_year, hijri_month).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(ApiResponse::success(result)))
}

/// Get event details
pub async fn get_event_details(
    Query(params): Query<std::collections::HashMap<String, String>>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<Option<String>>>, AppError> {
    let event_name = params.get("event_name")
        .ok_or_else(|| AppError::BadRequest("Missing event_name parameter".to_string()))?;
    
    let result = service.get_event_details(event_name).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(ApiResponse::success(result)))
}

/// Get current Hijri date
pub async fn get_current_hijri_date(
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<shared::HijriDate>>, AppError> {
    let result = service.get_current_hijri_date().await
        .map_err(|e| AppError::Internal(e.to_string()))?;
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

/// Schedule prayer notifications for a user
pub async fn schedule_prayer_notifications(
    Query(params): Query<NotificationQuery>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<Vec<crate::models::ScheduledNotification>>>, AppError> {
    let location = Location {
        latitude: params.latitude,
        longitude: params.longitude,
        timezone: params.timezone.unwrap_or_else(|| "UTC".to_string()),
        city: params.city,
        country: params.country,
    };

    let start_date = params.start_date.unwrap_or_else(|| chrono::Utc::now().date_naive());
    let end_date = params.end_date.unwrap_or_else(|| {
        start_date + chrono::Duration::days(params.days_ahead.unwrap_or(7) as i64)
    });

    // Get user preferences or create default ones
    let user_prefs = match service.get_user_prayer_preferences(params.user_id).await {
        Ok(prefs) => prefs,
        Err(_) => service.create_default_prayer_preferences(params.user_id).await
            .map_err(|e| AppError::Internal(e.to_string()))?,
    };

    let preferences = service.convert_user_prefs_to_notification_prefs(user_prefs);
    
    let request = NotificationScheduleRequest {
        user_id: params.user_id,
        location,
        preferences: preferences.prayer_settings.iter().map(|p| {
            PrayerNotificationSettings {
                prayer_name: p.prayer_name.clone(),
                enabled: p.enabled,
                minutes_before: p.minutes_before,
                graduated_enabled: p.graduated_enabled,
                graduated_intervals: p.graduated_intervals.clone(),
            }
        }).collect(),
        start_date,
        end_date,
    };

    let result = service.schedule_notifications_for_period(request).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(ApiResponse::success(result)))
}

/// Get upcoming prayer notifications for a user
pub async fn get_upcoming_notifications(
    Query(params): Query<NotificationQuery>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<Vec<crate::models::ScheduledNotification>>>, AppError> {
    let location = Location {
        latitude: params.latitude,
        longitude: params.longitude,
        timezone: params.timezone.unwrap_or_else(|| "UTC".to_string()),
        city: params.city,
        country: params.country,
    };

    let days_ahead = params.days_ahead.unwrap_or(7);

    let result = service.get_upcoming_notifications(params.user_id, &location, days_ahead).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(ApiResponse::success(result)))
}

/// Get user prayer notification preferences
pub async fn get_user_prayer_preferences(
    Path(user_id): Path<Uuid>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<UserPrayerPreferences>>, AppError> {
    let result = service.get_user_prayer_preferences(user_id).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(ApiResponse::success(result)))
}

/// Update user prayer notification preferences
pub async fn update_user_prayer_preferences(
    Path(user_id): Path<Uuid>,
    Json(preferences): Json<UserPrayerPreferences>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<UserPrayerPreferences>>, AppError> {
    let mut updated_preferences = preferences;
    updated_preferences.user_id = user_id;
    updated_preferences.updated_at = chrono::Utc::now();

    let result = service.update_user_prayer_preferences(user_id, updated_preferences).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(ApiResponse::success(result)))
}

/// Create default prayer preferences for a user
pub async fn create_default_prayer_preferences(
    Path(user_id): Path<Uuid>,
    Extension(service): Extension<PrayerTimesService>,
) -> Result<Json<ApiResponse<UserPrayerPreferences>>, AppError> {
    let result = service.create_default_prayer_preferences(user_id).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(ApiResponse::success(result)))
}
//! Data models for API Integration Service

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

// ============================================================================
// Configuration Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub service: ServiceInfo,
    pub redis: RedisConfig,
    pub postgres: PostgresConfig,
    pub apis: ApiConfigs,
    pub cache: CacheConfig,
    pub health_monitor: HealthMonitorConfig,
    pub retry: RetryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
    pub connection_timeout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    pub url: String,
    pub pool_size: u32,
    pub connection_timeout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfigs {
    pub quran: Vec<ApiConfig>,
    pub hadith: Vec<ApiConfig>,
    pub prayer_times: Vec<ApiConfig>,
    pub tafsir: Vec<ApiConfig>,
    pub calendar: Vec<ApiConfig>,
    pub qibla: Vec<ApiConfig>,
    pub ai: Vec<ApiConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub name: String,
    pub base_url: String,
    pub priority: u8,
    pub requires_key: Option<bool>,
    pub rate_limit: RateLimitConfig,
    pub timeout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub requests_per_day: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub strategies: CacheStrategies,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStrategies {
    pub quran_text: CacheStrategy,
    pub hadith: CacheStrategy,
    pub prayer_times: CacheStrategy,
    pub tafsir: CacheStrategy,
    pub calendar: CacheStrategy,
    pub qibla: CacheStrategy,
    pub ai_response: CacheStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStrategy {
    pub ttl: String,
    pub allow_stale: bool,
    pub stale_ttl: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitorConfig {
    pub check_interval: String,
    pub unhealthy_threshold: u32,
    pub recovery_threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: String,
    pub max_delay: String,
    pub multiplier: f64,
}

// ============================================================================
// Quran Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuranTextRequest {
    pub surah: u8,
    pub ayah: Option<u16>,
    pub translation: Option<String>,
    pub reciter: Option<String>,
}

impl QuranTextRequest {
    pub fn cache_key(&self) -> String {
        format!(
            "quran:{}:{}:{}:{}",
            self.surah,
            self.ayah.map(|a| a.to_string()).unwrap_or_else(|| "all".to_string()),
            self.translation.as_deref().unwrap_or("none"),
            self.reciter.as_deref().unwrap_or("none")
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuranTextResponse {
    pub surah: u8,
    pub ayah: u16,
    pub text_arabic: String,
    pub text_translation: Option<String>,
    pub audio_url: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuranAudioRequest {
    pub surah: u8,
    pub ayah: u16,
    pub reciter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuranAudioResponse {
    pub surah: u8,
    pub ayah: u16,
    pub audio_url: String,
    pub reciter: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurahData {
    pub number: u8,
    pub name_arabic: String,
    pub name_english: String,
    pub ayahs: Vec<AyahData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AyahData {
    pub surah: u8,
    pub ayah: u16,
    pub text_arabic: String,
    pub text_translation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData {
    pub page_number: u16,
    pub ayahs: Vec<AyahData>,
}

// ============================================================================
// Hadith Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HadithSearchRequest {
    pub query: String,
    pub collection: Option<String>,
    pub book: Option<String>,
    pub language: String,
    pub limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HadithSearchResponse {
    pub results: Vec<HadithResult>,
    pub total: usize,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HadithByIdRequest {
    pub id: String,
    pub collection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HadithResponse {
    pub hadith: HadithResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HadithResult {
    pub id: String,
    pub collection: String,
    pub book: String,
    pub hadith_number: String,
    pub text_arabic: String,
    pub text_translation: Option<String>,
    pub grade: Option<String>,
    pub narrator: String,
    pub source: String,
}

// ============================================================================
// Prayer Times Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerTimesRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub date: NaiveDate,
    pub calculation_method: CalculationMethod,
    pub madhab: Madhab,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CalculationMethod {
    MWL,           // Muslim World League
    ISNA,          // Islamic Society of North America
    Egypt,         // Egyptian General Authority of Survey
    Makkah,        // Umm Al-Qura University, Makkah
    Karachi,       // University of Islamic Sciences, Karachi
    Tehran,        // Institute of Geophysics, University of Tehran
    Jafari,        // Shia Ithna-Ashari, Leva Institute, Qum
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Madhab {
    Shafi,
    Hanafi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerTimesResponse {
    pub date: NaiveDate,
    pub fajr: NaiveTime,
    pub sunrise: NaiveTime,
    pub dhuhr: NaiveTime,
    pub asr: NaiveTime,
    pub maghrib: NaiveTime,
    pub isha: NaiveTime,
    pub source: String,
}

// ============================================================================
// Tafsir Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TafsirRequest {
    pub surah: u8,
    pub ayah: u16,
    pub tafsir_id: Option<String>,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TafsirResponse {
    pub surah: u8,
    pub ayah: u16,
    pub tafsirs: Vec<TafsirEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TafsirEntry {
    pub tafsir_id: String,
    pub tafsir_name: String,
    pub scholar: String,
    pub text: String,
    pub language: String,
    pub source: String,
}

// ============================================================================
// Calendar Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateConversionRequest {
    pub date: NaiveDate,
    pub direction: ConversionDirection,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversionDirection {
    GregorianToHijri,
    HijriToGregorian,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateConversionResponse {
    pub gregorian: NaiveDate,
    pub hijri: HijriDate,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HijriDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub month_name_ar: String,
    pub month_name_en: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicEventsRequest {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicEventsResponse {
    pub events: Vec<IslamicEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicEvent {
    pub date: NaiveDate,
    pub hijri_date: HijriDate,
    pub event_name_ar: String,
    pub event_name_en: String,
    pub description: Option<String>,
}

// ============================================================================
// Qibla Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QiblaRequest {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QiblaResponse {
    pub direction: f64,  // Degrees from North (0-360)
    pub distance_km: f64,
    pub source: String,
}

// ============================================================================
// AI Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiQueryRequest {
    pub query: String,
    pub context: Option<String>,
    pub language: String,
    pub max_tokens: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiQueryResponse {
    pub response: String,
    pub sources: Vec<String>,
    pub confidence: f64,
    pub model: String,
}

// ============================================================================
// Health Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub overall_status: ServiceStatus,
    pub apis: Vec<ApiHealthStatus>,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiHealthStatus {
    pub api_name: String,
    pub is_healthy: bool,
    pub last_check: SystemTime,
    pub last_success: Option<SystemTime>,
    pub last_failure: Option<SystemTime>,
    pub success_rate: f64,
    pub avg_response_time: Duration,
    pub consecutive_failures: u32,
}

impl ApiHealthStatus {
    pub fn new(api_name: &str) -> Self {
        Self {
            api_name: api_name.to_string(),
            is_healthy: true,
            last_check: SystemTime::now(),
            last_success: None,
            last_failure: None,
            success_rate: 1.0,
            avg_response_time: Duration::from_millis(0),
            consecutive_failures: 0,
        }
    }

    pub fn update_response_time(&mut self, duration: Duration) {
        // Simple moving average
        let current_avg = self.avg_response_time.as_millis() as f64;
        let new_duration = duration.as_millis() as f64;
        let new_avg = (current_avg * 0.9) + (new_duration * 0.1);
        self.avg_response_time = Duration::from_millis(new_avg as u64);
    }

    pub fn update_success_rate(&mut self) {
        // Calculate success rate based on recent history
        // This is a simplified version - in production, you'd track more history
        if self.consecutive_failures > 0 {
            self.success_rate = self.success_rate * 0.9;
        } else {
            self.success_rate = (self.success_rate * 0.9) + 0.1;
        }
        self.success_rate = self.success_rate.clamp(0.0, 1.0);
    }
}

// ============================================================================
// Error Models
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Rate limit exceeded for API: {0}")]
    RateLimitExceeded(String),
    
    #[error("API key not found: {0}")]
    ApiKeyNotFound(String),
    
    #[error("API key inactive: {0}")]
    ApiKeyInactive(String),
    
    #[error("API key expired: {0}")]
    ApiKeyExpired(String),
    
    #[error("Invalid response from API {0}: {1}")]
    InvalidResponse(String, String),
    
    #[error("API {0} returned error: {1}")]
    ApiError(String, String),
    
    #[error("All APIs failed for request")]
    AllApisFailed,
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Unknown API: {0}")]
    UnknownApi(String),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Timeout")]
    Timeout,
    
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_code: String,
    pub error_message: String,
    pub error_category: ErrorCategory,
    pub timestamp: SystemTime,
    pub request_id: String,
    pub retry_after: Option<Duration>,
    pub fallback_used: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorCategory {
    Network,
    Authentication,
    RateLimit,
    ServerError,
    Validation,
    Timeout,
    Unknown,
}

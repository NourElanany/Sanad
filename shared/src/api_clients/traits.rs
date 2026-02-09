//! Core traits for API clients

use super::{ApiError, RateLimitConfig};
use async_trait::async_trait;
use std::fmt::Debug;

/// Base trait for all API clients
/// 
/// This trait provides a common interface for all external API integrations,
/// enabling consistent handling of health checks, rate limiting, and priority-based
/// fallback mechanisms.
#[async_trait]
pub trait ApiClient: Send + Sync + Debug {
    /// Get the API name for logging and monitoring
    fn api_name(&self) -> &str;
    
    /// Get the priority level (lower number = higher priority)
    /// Priority 1 = Primary API, Priority 2 = Secondary, etc.
    fn priority(&self) -> u8;
    
    /// Check if the API is currently healthy
    /// This should perform a lightweight health check (e.g., ping endpoint)
    async fn is_healthy(&self) -> bool;
    
    /// Get rate limit configuration for this API
    fn rate_limit(&self) -> RateLimitConfig;
}

/// Trait for Quran API clients
/// 
/// Provides methods for retrieving Quran text, translations, and audio recitations
/// from various official Quran APIs.
#[async_trait]
pub trait QuranApiClient: ApiClient {
    /// Get a complete surah (chapter) with all its verses
    async fn get_surah(&self, surah_number: u8) -> Result<SurahData, ApiError>;
    
    /// Get a specific verse (ayah)
    async fn get_ayah(&self, surah: u8, ayah: u16) -> Result<AyahData, ApiError>;
    
    /// Get all verses on a specific page of the Quran
    async fn get_page(&self, page: u16) -> Result<PageData, ApiError>;
}

/// Trait for Hadith API clients
/// 
/// Provides methods for searching and retrieving hadith from various collections
/// (Bukhari, Muslim, Tirmidhi, etc.)
#[async_trait]
pub trait HadithApiClient: ApiClient {
    /// Search for hadith by text query
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<HadithResult>, ApiError>;
    
    /// Get a specific hadith by its ID
    async fn get_by_id(&self, id: &str) -> Result<HadithResult, ApiError>;
    
    /// Get hadith from a specific collection
    async fn get_by_collection(&self, collection: &str, limit: usize) -> Result<Vec<HadithResult>, ApiError>;
}

/// Trait for Prayer Times API clients
/// 
/// Provides methods for calculating prayer times based on location and calculation method
#[async_trait]
pub trait PrayerTimesApiClient: ApiClient {
    /// Get prayer times for a specific location and date
    async fn get_times(&self, request: &PrayerTimesRequest) -> Result<PrayerTimesResponse, ApiError>;
    
    /// Get prayer times for a date range
    async fn get_times_range(
        &self,
        request: &PrayerTimesRequest,
        days: u32,
    ) -> Result<Vec<PrayerTimesResponse>, ApiError>;
}

/// Trait for Tafsir API clients
/// 
/// Provides methods for retrieving Quran interpretations (tafsir) from various scholars
#[async_trait]
pub trait TafsirApiClient: ApiClient {
    /// Get tafsir for a specific verse
    async fn get_tafsir(&self, surah: u8, ayah: u16, tafsir_id: Option<&str>) -> Result<Vec<TafsirEntry>, ApiError>;
    
    /// List available tafsir sources
    async fn list_tafsir_sources(&self) -> Result<Vec<TafsirSource>, ApiError>;
}

/// Trait for Calendar API clients
/// 
/// Provides methods for Hijri calendar conversions and Islamic events
#[async_trait]
pub trait CalendarApiClient: ApiClient {
    /// Convert Gregorian date to Hijri
    async fn gregorian_to_hijri(&self, date: chrono::NaiveDate) -> Result<HijriDate, ApiError>;
    
    /// Convert Hijri date to Gregorian
    async fn hijri_to_gregorian(&self, hijri: &HijriDate) -> Result<chrono::NaiveDate, ApiError>;
    
    /// Get Islamic events for a date range
    async fn get_events(
        &self,
        start: chrono::NaiveDate,
        end: chrono::NaiveDate,
    ) -> Result<Vec<IslamicEvent>, ApiError>;
}

/// Trait for Qibla API clients
/// 
/// Provides methods for calculating Qibla direction from any location
#[async_trait]
pub trait QiblaApiClient: ApiClient {
    /// Get Qibla direction for a specific location
    async fn get_direction(&self, latitude: f64, longitude: f64) -> Result<QiblaResponse, ApiError>;
}

/// Trait for AI/NLP API clients
/// 
/// Provides methods for processing Islamic queries using AI models
/// Note: AI is used ONLY for language processing, NOT for Islamic rulings
#[async_trait]
pub trait AiApiClient: ApiClient {
    /// Process a query with AI
    async fn process_query(&self, request: &AiQueryRequest) -> Result<AiQueryResponse, ApiError>;
    
    /// Generate embeddings for semantic search
    async fn generate_embeddings(&self, text: &str) -> Result<Vec<f32>, ApiError>;
}

// ============================================================================
// Data structures used by traits
// ============================================================================

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TafsirEntry {
    pub tafsir_id: String,
    pub tafsir_name: String,
    pub scholar: String,
    pub text: String,
    pub language: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TafsirSource {
    pub id: String,
    pub name: String,
    pub scholar: String,
    pub language: String,
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
pub struct IslamicEvent {
    pub date: NaiveDate,
    pub hijri_date: HijriDate,
    pub event_name_ar: String,
    pub event_name_en: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QiblaResponse {
    pub direction: f64,  // Degrees from North (0-360)
    pub distance_km: f64,
    pub source: String,
}

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

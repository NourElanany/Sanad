use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt;

/// Supported languages in the Islamic application
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SupportedLanguage {
    Arabic,
    English,
    French,
    Spanish,
    Turkish,
    Urdu,
    Indonesian,
    Malay,
    Bengali,
    Persian,
}

impl SupportedLanguage {
    /// Get the language code (ISO 639-1)
    pub fn code(&self) -> &'static str {
        match self {
            SupportedLanguage::Arabic => "ar",
            SupportedLanguage::English => "en",
            SupportedLanguage::French => "fr",
            SupportedLanguage::Spanish => "es",
            SupportedLanguage::Turkish => "tr",
            SupportedLanguage::Urdu => "ur",
            SupportedLanguage::Indonesian => "id",
            SupportedLanguage::Malay => "ms",
            SupportedLanguage::Bengali => "bn",
            SupportedLanguage::Persian => "fa",
        }
    }

    /// Get the native name of the language
    pub fn native_name(&self) -> &'static str {
        match self {
            SupportedLanguage::Arabic => "العربية",
            SupportedLanguage::English => "English",
            SupportedLanguage::French => "Français",
            SupportedLanguage::Spanish => "Español",
            SupportedLanguage::Turkish => "Türkçe",
            SupportedLanguage::Urdu => "اردو",
            SupportedLanguage::Indonesian => "Bahasa Indonesia",
            SupportedLanguage::Malay => "Bahasa Melayu",
            SupportedLanguage::Bengali => "বাংলা",
            SupportedLanguage::Persian => "فارسی",
        }
    }

    /// Get the English name of the language
    pub fn english_name(&self) -> &'static str {
        match self {
            SupportedLanguage::Arabic => "Arabic",
            SupportedLanguage::English => "English",
            SupportedLanguage::French => "French",
            SupportedLanguage::Spanish => "Spanish",
            SupportedLanguage::Turkish => "Turkish",
            SupportedLanguage::Urdu => "Urdu",
            SupportedLanguage::Indonesian => "Indonesian",
            SupportedLanguage::Malay => "Malay",
            SupportedLanguage::Bengali => "Bengali",
            SupportedLanguage::Persian => "Persian",
        }
    }

    /// Check if the language uses right-to-left text direction
    pub fn is_rtl(&self) -> bool {
        matches!(self, 
            SupportedLanguage::Arabic | 
            SupportedLanguage::Urdu | 
            SupportedLanguage::Persian
        )
    }

    /// Get all supported languages
    pub fn all() -> Vec<SupportedLanguage> {
        vec![
            SupportedLanguage::Arabic,
            SupportedLanguage::English,
            SupportedLanguage::French,
            SupportedLanguage::Spanish,
            SupportedLanguage::Turkish,
            SupportedLanguage::Urdu,
            SupportedLanguage::Indonesian,
            SupportedLanguage::Malay,
            SupportedLanguage::Bengali,
            SupportedLanguage::Persian,
        ]
    }

    /// Parse language from code
    pub fn from_code(code: &str) -> Option<SupportedLanguage> {
        match code.to_lowercase().as_str() {
            "ar" => Some(SupportedLanguage::Arabic),
            "en" => Some(SupportedLanguage::English),
            "fr" => Some(SupportedLanguage::French),
            "es" => Some(SupportedLanguage::Spanish),
            "tr" => Some(SupportedLanguage::Turkish),
            "ur" => Some(SupportedLanguage::Urdu),
            "id" => Some(SupportedLanguage::Indonesian),
            "ms" => Some(SupportedLanguage::Malay),
            "bn" => Some(SupportedLanguage::Bengali),
            "fa" => Some(SupportedLanguage::Persian),
            _ => None,
        }
    }
}

impl fmt::Display for SupportedLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.english_name())
    }
}

/// Text direction for different languages
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
}

impl TextDirection {
    pub fn css_value(&self) -> &'static str {
        match self {
            TextDirection::LeftToRight => "ltr",
            TextDirection::RightToLeft => "rtl",
        }
    }
}

/// Translation key-value pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub key: String,
    pub value: String,
    pub context: Option<String>,
    pub plural_forms: Option<HashMap<String, String>>,
}

/// Translation namespace (e.g., "common", "prayers", "quran")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationNamespace {
    pub namespace: String,
    pub language: SupportedLanguage,
    pub translations: HashMap<String, Translation>,
    pub version: String,
    pub last_updated: DateTime<Utc>,
}

/// Language pack containing all translations for a specific language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePack {
    pub language: SupportedLanguage,
    pub namespaces: HashMap<String, TranslationNamespace>,
    pub metadata: LanguagePackMetadata,
}

/// Metadata for language pack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePackMetadata {
    pub version: String,
    pub contributors: Vec<String>,
    pub completion_percentage: f32,
    pub last_updated: DateTime<Utc>,
    pub quality_score: f32,
}

/// User language preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLanguagePreferences {
    pub user_id: Uuid,
    pub primary_language: SupportedLanguage,
    pub fallback_languages: Vec<SupportedLanguage>,
    pub quran_translation_languages: Vec<SupportedLanguage>,
    pub interface_language: SupportedLanguage,
    pub content_language_preferences: HashMap<String, SupportedLanguage>,
    pub updated_at: DateTime<Utc>,
}

/// Language detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDetectionResult {
    pub detected_language: SupportedLanguage,
    pub confidence: f32,
    pub alternative_languages: Vec<(SupportedLanguage, f32)>,
}

/// Translation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationRequest {
    pub key: String,
    pub namespace: Option<String>,
    pub language: SupportedLanguage,
    pub fallback_languages: Option<Vec<SupportedLanguage>>,
    pub interpolation_values: Option<HashMap<String, String>>,
    pub plural_count: Option<i32>,
}

/// Translation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResponse {
    pub key: String,
    pub value: String,
    pub language: SupportedLanguage,
    pub namespace: String,
    pub is_fallback: bool,
    pub interpolated: bool,
}

/// Bulk translation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkTranslationRequest {
    pub keys: Vec<String>,
    pub namespace: Option<String>,
    pub language: SupportedLanguage,
    pub fallback_languages: Option<Vec<SupportedLanguage>>,
}

/// Bulk translation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkTranslationResponse {
    pub translations: HashMap<String, TranslationResponse>,
    pub missing_keys: Vec<String>,
    pub language: SupportedLanguage,
}

/// Language switching request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSwitchRequest {
    pub user_id: Option<Uuid>,
    pub new_language: SupportedLanguage,
    pub apply_to_content: bool,
    pub apply_to_interface: bool,
}

/// Language switching response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSwitchResponse {
    pub success: bool,
    pub new_language: SupportedLanguage,
    pub text_direction: TextDirection,
    pub updated_preferences: UserLanguagePreferences,
    pub required_ui_updates: Vec<String>,
}

/// Available translations for content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableTranslations {
    pub content_id: Uuid,
    pub content_type: String,
    pub available_languages: Vec<SupportedLanguage>,
    pub default_language: SupportedLanguage,
    pub quality_scores: HashMap<SupportedLanguage, f32>,
}

/// Translation quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationQuality {
    pub language: SupportedLanguage,
    pub namespace: String,
    pub completion_percentage: f32,
    pub accuracy_score: f32,
    pub consistency_score: f32,
    pub last_reviewed: Option<DateTime<Utc>>,
    pub reviewer_notes: Option<String>,
}

/// Localization context for dynamic content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizationContext {
    pub user_id: Option<Uuid>,
    pub language: SupportedLanguage,
    pub region: Option<String>,
    pub timezone: Option<String>,
    pub cultural_preferences: HashMap<String, String>,
}

/// Error types for i18n service
#[derive(Debug, thiserror::Error)]
pub enum I18nError {
    #[error("Language not supported: {0}")]
    UnsupportedLanguage(String),
    
    #[error("Translation key not found: {key} in namespace {namespace}")]
    TranslationNotFound { key: String, namespace: String },
    
    #[error("Language pack not found for language: {0}")]
    LanguagePackNotFound(SupportedLanguage),
    
    #[error("Invalid translation format: {0}")]
    InvalidTranslationFormat(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

pub type I18nResult<T> = Result<T, I18nError>;
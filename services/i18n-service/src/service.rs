use crate::models::*;
use crate::repository::I18nRepository;
use crate::translation_loader::TranslationLoader;
use crate::language_detector::LanguageDetector;
use crate::text_direction::TextDirectionManager;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use tracing::{info, warn, debug};
use chrono::Utc;
use regex::Regex;

/// Main internationalization service
pub struct I18nService {
    repository: I18nRepository,
    translation_loader: Arc<RwLock<TranslationLoader>>,
    language_detector: LanguageDetector,
    // In-memory cache for frequently accessed translations
    translation_cache: Arc<RwLock<HashMap<String, TranslationResponse>>>,
    // Cache for language packs
    language_pack_cache: Arc<RwLock<HashMap<SupportedLanguage, LanguagePack>>>,
}

impl I18nService {
    pub fn new(
        repository: I18nRepository,
        translations_path: String,
    ) -> Self {
        let translation_loader = Arc::new(RwLock::new(TranslationLoader::new(translations_path)));
        let language_detector = LanguageDetector::new();
        let translation_cache = Arc::new(RwLock::new(HashMap::new()));
        let language_pack_cache = Arc::new(RwLock::new(HashMap::new()));

        Self {
            repository,
            translation_loader,
            language_detector,
            translation_cache,
            language_pack_cache,
        }
    }

    /// Initialize the service by loading all language packs
    pub async fn initialize(&self) -> I18nResult<()> {
        info!("Initializing I18n service");

        let mut loader = self.translation_loader.write().await;
        let mut cache = self.language_pack_cache.write().await;

        for language in SupportedLanguage::all() {
            match loader.load_language_pack(language.clone()).await {
                Ok(pack) => {
                    info!("Loaded language pack for {}", language.code());
                    cache.insert(language, pack);
                }
                Err(e) => {
                    warn!("Failed to load language pack for {}: {}", language.code(), e);
                }
            }
        }

        info!("I18n service initialized with {} language packs", cache.len());
        Ok(())
    }

    /// Get translation for a specific key
    pub async fn get_translation(&self, request: TranslationRequest) -> I18nResult<TranslationResponse> {
        let cache_key = format!("{}:{}:{}", 
            request.language.code(), 
            request.namespace.as_deref().unwrap_or("common"), 
            request.key
        );

        // Check cache first
        {
            let cache = self.translation_cache.read().await;
            if let Some(cached) = cache.get(&cache_key) {
                debug!("Cache hit for translation key: {}", cache_key);
                return Ok(cached.clone());
            }
        }

        // Get from language pack
        let translation = self.get_translation_from_pack(&request).await?;

        // Apply interpolation if needed
        let final_translation = if let Some(values) = &request.interpolation_values {
            self.interpolate_translation(&translation.value, values)?
        } else {
            translation.value.clone()
        };

        // Handle pluralization if needed
        let final_value = if let Some(count) = request.plural_count {
            self.handle_pluralization(&translation, count, &request.language)?
        } else {
            final_translation
        };

        let response = TranslationResponse {
            key: request.key.clone(),
            value: final_value,
            language: translation.language,
            namespace: request.namespace.unwrap_or_else(|| "common".to_string()),
            is_fallback: translation.is_fallback,
            interpolated: request.interpolation_values.is_some(),
        };

        // Cache the result
        {
            let mut cache = self.translation_cache.write().await;
            cache.insert(cache_key, response.clone());
        }

        Ok(response)
    }

    /// Get multiple translations at once
    pub async fn get_bulk_translations(&self, request: BulkTranslationRequest) -> I18nResult<BulkTranslationResponse> {
        let mut translations = HashMap::new();
        let mut missing_keys = Vec::new();

        for key in &request.keys {
            let translation_request = TranslationRequest {
                key: key.clone(),
                namespace: request.namespace.clone(),
                language: request.language.clone(),
                fallback_languages: request.fallback_languages.clone(),
                interpolation_values: None,
                plural_count: None,
            };

            match self.get_translation(translation_request).await {
                Ok(translation) => {
                    translations.insert(key.clone(), translation);
                }
                Err(_) => {
                    missing_keys.push(key.clone());
                }
            }
        }

        Ok(BulkTranslationResponse {
            translations,
            missing_keys,
            language: request.language,
        })
    }

    /// Switch user language
    pub async fn switch_language(&self, request: LanguageSwitchRequest) -> I18nResult<LanguageSwitchResponse> {
        let text_direction = TextDirectionManager::get_direction(&request.new_language);

        // Update user preferences if user_id is provided
        let updated_preferences = if let Some(user_id) = request.user_id {
            let mut preferences = self.repository
                .get_user_preferences(user_id)
                .await?
                .unwrap_or_else(|| self.repository.get_default_preferences(user_id));

            if request.apply_to_interface {
                preferences.interface_language = request.new_language.clone();
            }

            if request.apply_to_content {
                preferences.primary_language = request.new_language.clone();
            }

            preferences.updated_at = Utc::now();

            self.repository.save_user_preferences(&preferences).await?;
            preferences
        } else {
            // Create temporary preferences for anonymous users
            UserLanguagePreferences {
                user_id: Uuid::new_v4(), // Temporary ID
                primary_language: request.new_language.clone(),
                fallback_languages: vec![SupportedLanguage::English],
                quran_translation_languages: vec![SupportedLanguage::English],
                interface_language: request.new_language.clone(),
                content_language_preferences: HashMap::new(),
                updated_at: Utc::now(),
            }
        };

        // Generate list of UI updates needed
        let required_ui_updates = vec![
            "update_text_direction".to_string(),
            "reload_interface_strings".to_string(),
            "update_font_family".to_string(),
            "refresh_content_layout".to_string(),
        ];

        Ok(LanguageSwitchResponse {
            success: true,
            new_language: request.new_language,
            text_direction,
            updated_preferences,
            required_ui_updates,
        })
    }

    /// Detect language from text
    pub async fn detect_language(&self, text: &str) -> I18nResult<LanguageDetectionResult> {
        Ok(self.language_detector.detect_language(text))
    }

    /// Detect language from HTTP headers
    pub async fn detect_language_from_headers(&self, accept_language: &str) -> I18nResult<Option<SupportedLanguage>> {
        Ok(self.language_detector.detect_from_accept_language(accept_language))
    }

    /// Get user language preferences
    pub async fn get_user_preferences(&self, user_id: Uuid) -> I18nResult<UserLanguagePreferences> {
        Ok(self.repository
            .get_user_preferences(user_id)
            .await?
            .unwrap_or_else(|| self.repository.get_default_preferences(user_id)))
    }

    /// Update user language preferences
    pub async fn update_user_preferences(&self, preferences: UserLanguagePreferences) -> I18nResult<()> {
        self.repository.save_user_preferences(&preferences).await
    }

    /// Get available translations for content
    pub async fn get_available_translations(&self, content_id: Uuid) -> I18nResult<Option<AvailableTranslations>> {
        self.repository.get_available_translations(content_id).await
    }

    /// Get supported languages
    pub async fn get_supported_languages(&self) -> I18nResult<Vec<SupportedLanguage>> {
        Ok(SupportedLanguage::all())
    }

    /// Get language information
    pub async fn get_language_info(&self, language: &SupportedLanguage) -> I18nResult<LanguageInfo> {
        let text_direction = TextDirectionManager::get_direction(language);
        let css_classes = TextDirectionManager::generate_css_classes(language);
        let font_recommendations = TextDirectionManager::get_recommended_fonts(language);

        Ok(LanguageInfo {
            language: language.clone(),
            code: language.code().to_string(),
            native_name: language.native_name().to_string(),
            english_name: language.english_name().to_string(),
            text_direction,
            is_rtl: language.is_rtl(),
            css_classes,
            font_recommendations,
        })
    }

    /// Generate CSS for all languages
    pub async fn generate_all_languages_css(&self) -> I18nResult<String> {
        let mut css = String::new();
        
        for language in SupportedLanguage::all() {
            css.push_str(&TextDirectionManager::generate_language_css(&language));
            css.push('\n');
        }

        Ok(css)
    }

    /// Reload translations from files
    pub async fn reload_translations(&self) -> I18nResult<()> {
        info!("Reloading translations");

        // Clear caches
        {
            let mut translation_cache = self.translation_cache.write().await;
            translation_cache.clear();
        }

        {
            let mut language_pack_cache = self.language_pack_cache.write().await;
            language_pack_cache.clear();
        }

        // Reload from files
        let mut loader = self.translation_loader.write().await;
        loader.reload_translations().await?;

        // Reinitialize
        self.initialize().await?;

        info!("Translations reloaded successfully");
        Ok(())
    }

    /// Get translation statistics
    pub async fn get_translation_stats(&self) -> I18nResult<TranslationStats> {
        let language_pack_cache = self.language_pack_cache.read().await;
        let translation_cache = self.translation_cache.read().await;

        let mut stats = TranslationStats {
            total_languages: SupportedLanguage::all().len(),
            loaded_languages: language_pack_cache.len(),
            cached_translations: translation_cache.len(),
            language_completion: HashMap::new(),
        };

        for (language, pack) in language_pack_cache.iter() {
            let total_keys: usize = pack.namespaces.values()
                .map(|ns| ns.translations.len())
                .sum();
            
            stats.language_completion.insert(language.clone(), total_keys);
        }

        Ok(stats)
    }

    // Private helper methods

    async fn get_translation_from_pack(&self, request: &TranslationRequest) -> I18nResult<TranslationWithFallback> {
        let namespace = request.namespace.as_deref().unwrap_or("common");
        
        // Try primary language first
        if let Some(translation) = self.get_translation_from_language_pack(&request.language, namespace, &request.key).await? {
            return Ok(TranslationWithFallback {
                value: translation.value,
                language: request.language.clone(),
                is_fallback: false,
                plural_forms: translation.plural_forms,
            });
        }

        // Try fallback languages
        if let Some(fallback_languages) = &request.fallback_languages {
            for fallback_lang in fallback_languages {
                if let Some(translation) = self.get_translation_from_language_pack(fallback_lang, namespace, &request.key).await? {
                    return Ok(TranslationWithFallback {
                        value: translation.value,
                        language: fallback_lang.clone(),
                        is_fallback: true,
                        plural_forms: translation.plural_forms,
                    });
                }
            }
        }

        // Try English as final fallback
        if request.language != SupportedLanguage::English {
            if let Some(translation) = self.get_translation_from_language_pack(&SupportedLanguage::English, namespace, &request.key).await? {
                return Ok(TranslationWithFallback {
                    value: translation.value,
                    language: SupportedLanguage::English,
                    is_fallback: true,
                    plural_forms: translation.plural_forms,
                });
            }
        }

        Err(I18nError::TranslationNotFound {
            key: request.key.clone(),
            namespace: namespace.to_string(),
        })
    }

    async fn get_translation_from_language_pack(
        &self,
        language: &SupportedLanguage,
        namespace: &str,
        key: &str,
    ) -> I18nResult<Option<Translation>> {
        let language_pack_cache = self.language_pack_cache.read().await;
        
        if let Some(pack) = language_pack_cache.get(language) {
            if let Some(ns) = pack.namespaces.get(namespace) {
                if let Some(translation) = ns.translations.get(key) {
                    return Ok(Some(translation.clone()));
                }
            }
        }

        Ok(None)
    }

    fn interpolate_translation(&self, template: &str, values: &HashMap<String, String>) -> I18nResult<String> {
        let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
        let mut result = template.to_string();

        for captures in re.captures_iter(template) {
            if let Some(key) = captures.get(1) {
                let key_str = key.as_str();
                if let Some(value) = values.get(key_str) {
                    let placeholder = format!("{{{{{}}}}}", key_str);
                    result = result.replace(&placeholder, value);
                }
            }
        }

        Ok(result)
    }

    fn handle_pluralization(
        &self,
        translation: &TranslationWithFallback,
        count: i32,
        language: &SupportedLanguage,
    ) -> I18nResult<String> {
        // This is a simplified pluralization logic
        // In a real implementation, you'd want to use proper pluralization rules for each language
        
        if let Some(plural_forms) = &translation.plural_forms {
            let form = match language {
                SupportedLanguage::Arabic => {
                    // Arabic has complex plural rules
                    if count == 0 {
                        "zero"
                    } else if count == 1 {
                        "one"
                    } else if count == 2 {
                        "two"
                    } else if count <= 10 {
                        "few"
                    } else {
                        "many"
                    }
                }
                _ => {
                    // Simple English-like pluralization
                    if count == 1 {
                        "one"
                    } else {
                        "other"
                    }
                }
            };

            if let Some(plural_form) = plural_forms.get(form) {
                return Ok(plural_form.clone());
            }
        }

        Ok(translation.value.clone())
    }
}

// Helper structs

#[derive(Debug, Clone)]
struct TranslationWithFallback {
    value: String,
    language: SupportedLanguage,
    is_fallback: bool,
    plural_forms: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LanguageInfo {
    pub language: SupportedLanguage,
    pub code: String,
    pub native_name: String,
    pub english_name: String,
    pub text_direction: TextDirection,
    pub is_rtl: bool,
    pub css_classes: Vec<String>,
    pub font_recommendations: crate::text_direction::FontRecommendations,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TranslationStats {
    pub total_languages: usize,
    pub loaded_languages: usize,
    pub cached_translations: usize,
    pub language_completion: HashMap<SupportedLanguage, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    // use sqlx::PgPool;  // Commented out until we need it

    // Mock repository for testing
    struct MockRepository;

    impl MockRepository {
        fn new() -> Self {
            MockRepository
        }
    }

    #[tokio::test]
    #[ignore] // Ignore until we have proper test setup
    async fn test_service_initialization() {
        // let repo = MockRepository::new();
        // let service = I18nService::new(repo, "test_translations".to_string());
        
        // This would test service initialization
        // service.initialize().await.unwrap();
    }

    #[test]
    fn test_interpolation() {
        // let repo = MockRepository::new();
        // let service = I18nService::new(repo, "test_translations".to_string());
        
        let mut values = HashMap::new();
        values.insert("name".to_string(), "أحمد".to_string());
        values.insert("count".to_string(), "5".to_string());
        
        // This would test interpolation logic
        // let result = service.interpolate_translation("مرحباً {{name}}، لديك {{count}} رسائل", &values).unwrap();
        // assert_eq!(result, "مرحباً أحمد، لديك 5 رسائل");
    }
}
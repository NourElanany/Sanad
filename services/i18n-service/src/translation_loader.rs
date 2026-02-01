use crate::models::*;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use tracing::{info, warn};
use chrono::Utc;

/// Translation loader responsible for loading translation files
pub struct TranslationLoader {
    translations_path: String,
    cache: HashMap<(SupportedLanguage, String), TranslationNamespace>,
}

impl TranslationLoader {
    pub fn new(translations_path: String) -> Self {
        Self {
            translations_path,
            cache: HashMap::new(),
        }
    }

    /// Load all translations for a specific language
    pub async fn load_language_pack(&mut self, language: SupportedLanguage) -> I18nResult<LanguagePack> {
        let language_dir = format!("{}/{}", self.translations_path, language.code());
        
        if !Path::new(&language_dir).exists() {
            return Err(I18nError::LanguagePackNotFound(language.clone()));
        }

        let mut namespaces = HashMap::new();
        let mut entries = fs::read_dir(&language_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let namespace_name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                match self.load_namespace(&language, &namespace_name, &path).await {
                    Ok(namespace) => {
                        namespaces.insert(namespace_name.clone(), namespace);
                    }
                    Err(e) => {
                        warn!("Failed to load namespace {}: {}", namespace_name, e);
                    }
                }
            }
        }

        let metadata = self.load_language_metadata(&language).await?;

        Ok(LanguagePack {
            language,
            namespaces,
            metadata,
        })
    }

    /// Load a specific namespace for a language
    pub async fn load_namespace(
        &mut self,
        language: &SupportedLanguage,
        namespace: &str,
        file_path: &Path,
    ) -> I18nResult<TranslationNamespace> {
        let cache_key = (language.clone(), namespace.to_string());
        
        // Check cache first
        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        info!("Loading namespace {} for language {}", namespace, language.code());

        let content = fs::read_to_string(file_path).await?;
        let raw_translations: HashMap<String, serde_yaml::Value> = 
            serde_yaml::from_str(&content)
                .map_err(|e| I18nError::InvalidTranslationFormat(e.to_string()))?;

        let mut translations = HashMap::new();

        for (key, value) in raw_translations {
            let translation = self.parse_translation_value(key.clone(), value)?;
            translations.insert(key, translation);
        }

        let translation_namespace = TranslationNamespace {
            namespace: namespace.to_string(),
            language: language.clone(),
            translations,
            version: "1.0.0".to_string(), // TODO: Load from metadata
            last_updated: Utc::now(),
        };

        // Cache the loaded namespace
        self.cache.insert(cache_key, translation_namespace.clone());

        Ok(translation_namespace)
    }

    /// Parse translation value from YAML
    fn parse_translation_value(&self, key: String, value: serde_yaml::Value) -> I18nResult<Translation> {
        match value {
            serde_yaml::Value::String(s) => Ok(Translation {
                key: key.clone(),
                value: s,
                context: None,
                plural_forms: None,
            }),
            serde_yaml::Value::Mapping(map) => {
                let mut translation = Translation {
                    key: key.clone(),
                    value: String::new(),
                    context: None,
                    plural_forms: None,
                };

                for (k, v) in map {
                    match k.as_str() {
                        Some("value") => {
                            translation.value = v.as_str()
                                .ok_or_else(|| I18nError::InvalidTranslationFormat(
                                    format!("Invalid value for key {}", key)
                                ))?
                                .to_string();
                        }
                        Some("context") => {
                            translation.context = v.as_str().map(|s| s.to_string());
                        }
                        Some("plural") => {
                            if let serde_yaml::Value::Mapping(plural_map) = v {
                                let mut plural_forms = HashMap::new();
                                for (pk, pv) in plural_map {
                                    if let (Some(plural_key), Some(plural_value)) = 
                                        (pk.as_str(), pv.as_str()) {
                                        plural_forms.insert(
                                            plural_key.to_string(), 
                                            plural_value.to_string()
                                        );
                                    }
                                }
                                translation.plural_forms = Some(plural_forms);
                            }
                        }
                        _ => {
                            warn!("Unknown translation property: {:?}", k);
                        }
                    }
                }

                if translation.value.is_empty() {
                    return Err(I18nError::InvalidTranslationFormat(
                        format!("Missing value for key {}", key)
                    ));
                }

                Ok(translation)
            }
            _ => Err(I18nError::InvalidTranslationFormat(
                format!("Invalid translation format for key {}", key)
            )),
        }
    }

    /// Load language metadata
    async fn load_language_metadata(&self, language: &SupportedLanguage) -> I18nResult<LanguagePackMetadata> {
        let metadata_path = format!("{}/{}/metadata.yaml", self.translations_path, language.code());
        
        if Path::new(&metadata_path).exists() {
            let content = fs::read_to_string(&metadata_path).await?;
            let metadata: LanguagePackMetadata = serde_yaml::from_str(&content)
                .map_err(|e| I18nError::InvalidTranslationFormat(e.to_string()))?;
            Ok(metadata)
        } else {
            // Default metadata if file doesn't exist
            Ok(LanguagePackMetadata {
                version: "1.0.0".to_string(),
                contributors: vec!["System".to_string()],
                completion_percentage: 100.0,
                last_updated: Utc::now(),
                quality_score: 1.0,
            })
        }
    }

    /// Reload translations (clear cache and reload)
    pub async fn reload_translations(&mut self) -> I18nResult<()> {
        info!("Reloading all translations");
        self.cache.clear();
        
        for language in SupportedLanguage::all() {
            match self.load_language_pack(language.clone()).await {
                Ok(_) => info!("Reloaded translations for {}", language.code()),
                Err(e) => warn!("Failed to reload translations for {}: {}", language.code(), e),
            }
        }
        
        Ok(())
    }

    /// Get cached namespace
    pub fn get_cached_namespace(
        &self, 
        language: &SupportedLanguage, 
        namespace: &str
    ) -> Option<&TranslationNamespace> {
        let cache_key = (language.clone(), namespace.to_string());
        self.cache.get(&cache_key)
    }

    /// Clear cache for specific language
    pub fn clear_language_cache(&mut self, language: &SupportedLanguage) {
        self.cache.retain(|(lang, _), _| lang != language);
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        for (language, _) in &self.cache {
            let lang_code = language.0.code().to_string();
            *stats.entry(lang_code).or_insert(0) += 1;
        }
        
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;
    // use std::path::PathBuf;  // Commented out until needed

    async fn create_test_translation_files() -> String {
        let temp_dir = std::env::temp_dir().join("i18n_test");
        fs::create_dir_all(&temp_dir).await.unwrap();
        
        // Create Arabic translations
        let ar_dir = temp_dir.join("ar");
        fs::create_dir_all(&ar_dir).await.unwrap();
        
        let common_ar = r#"
welcome: "مرحباً"
goodbye: "وداعاً"
prayer_times: "مواقيت الصلاة"
quran: "القرآن الكريم"
settings:
  value: "الإعدادات"
  context: "Application settings menu"
"#;
        fs::write(ar_dir.join("common.yaml"), common_ar).await.unwrap();
        
        // Create English translations
        let en_dir = temp_dir.join("en");
        fs::create_dir_all(&en_dir).await.unwrap();
        
        let common_en = r#"
welcome: "Welcome"
goodbye: "Goodbye"
prayer_times: "Prayer Times"
quran: "Holy Quran"
settings:
  value: "Settings"
  context: "Application settings menu"
"#;
        fs::write(en_dir.join("common.yaml"), common_en).await.unwrap();
        
        temp_dir.to_string_lossy().to_string()
    }

    #[tokio::test]
    #[ignore] // Ignore until we have proper test setup
    async fn test_load_language_pack() {
        let translations_path = create_test_translation_files().await;
        let mut loader = TranslationLoader::new(translations_path);
        
        let arabic_pack = loader.load_language_pack(SupportedLanguage::Arabic).await.unwrap();
        assert_eq!(arabic_pack.language, SupportedLanguage::Arabic);
        assert!(arabic_pack.namespaces.contains_key("common"));
        
        let common_namespace = &arabic_pack.namespaces["common"];
        assert!(common_namespace.translations.contains_key("welcome"));
        assert_eq!(common_namespace.translations["welcome"].value, "مرحباً");
    }

    #[tokio::test]
    #[ignore] // Ignore until we have proper test setup
    async fn test_cache_functionality() {
        let translations_path = create_test_translation_files().await;
        let mut loader = TranslationLoader::new(translations_path);
        
        // Load once
        loader.load_language_pack(SupportedLanguage::Arabic).await.unwrap();
        
        // Should be cached now
        let cached = loader.get_cached_namespace(&SupportedLanguage::Arabic, "common");
        assert!(cached.is_some());
        
        // Clear cache
        loader.clear_language_cache(&SupportedLanguage::Arabic);
        let cached_after_clear = loader.get_cached_namespace(&SupportedLanguage::Arabic, "common");
        assert!(cached_after_clear.is_none());
    }
}
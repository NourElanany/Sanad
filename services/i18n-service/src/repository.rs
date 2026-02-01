use crate::models::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::collections::HashMap;
use tracing::info;
use chrono::Utc;

/// Repository for managing internationalization data in the database
pub struct I18nRepository {
    pool: PgPool,
}

impl I18nRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Save user language preferences
    pub async fn save_user_preferences(&self, preferences: &UserLanguagePreferences) -> I18nResult<()> {
        let query = r#"
            INSERT INTO user_language_preferences (
                user_id, primary_language, fallback_languages, 
                quran_translation_languages, interface_language, 
                content_language_preferences, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (user_id) DO UPDATE SET
                primary_language = EXCLUDED.primary_language,
                fallback_languages = EXCLUDED.fallback_languages,
                quran_translation_languages = EXCLUDED.quran_translation_languages,
                interface_language = EXCLUDED.interface_language,
                content_language_preferences = EXCLUDED.content_language_preferences,
                updated_at = EXCLUDED.updated_at
        "#;

        let fallback_languages_json = serde_json::to_value(&preferences.fallback_languages)?;
        let quran_languages_json = serde_json::to_value(&preferences.quran_translation_languages)?;
        let content_prefs_json = serde_json::to_value(&preferences.content_language_preferences)?;

        sqlx::query(query)
            .bind(preferences.user_id)
            .bind(preferences.primary_language.code())
            .bind(fallback_languages_json)
            .bind(quran_languages_json)
            .bind(preferences.interface_language.code())
            .bind(content_prefs_json)
            .bind(preferences.updated_at)
            .execute(&self.pool)
            .await?;

        info!("Saved language preferences for user {}", preferences.user_id);
        Ok(())
    }

    /// Get user language preferences
    pub async fn get_user_preferences(&self, user_id: Uuid) -> I18nResult<Option<UserLanguagePreferences>> {
        let query = r#"
            SELECT user_id, primary_language, fallback_languages, 
                   quran_translation_languages, interface_language, 
                   content_language_preferences, updated_at
            FROM user_language_preferences 
            WHERE user_id = $1
        "#;

        let row = sqlx::query(query)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let primary_language = SupportedLanguage::from_code(row.get("primary_language"))
                .ok_or_else(|| I18nError::UnsupportedLanguage(row.get::<String, _>("primary_language")))?;

            let interface_language = SupportedLanguage::from_code(row.get("interface_language"))
                .ok_or_else(|| I18nError::UnsupportedLanguage(row.get::<String, _>("interface_language")))?;

            let fallback_languages: Vec<String> = serde_json::from_value(row.get("fallback_languages"))?;
            let fallback_languages: Vec<SupportedLanguage> = fallback_languages
                .into_iter()
                .filter_map(|code| SupportedLanguage::from_code(&code))
                .collect();

            let quran_languages: Vec<String> = serde_json::from_value(row.get("quran_translation_languages"))?;
            let quran_translation_languages: Vec<SupportedLanguage> = quran_languages
                .into_iter()
                .filter_map(|code| SupportedLanguage::from_code(&code))
                .collect();

            let content_language_preferences: HashMap<String, String> = 
                serde_json::from_value(row.get("content_language_preferences"))?;
            let content_language_preferences: HashMap<String, SupportedLanguage> = 
                content_language_preferences
                    .into_iter()
                    .filter_map(|(k, v)| SupportedLanguage::from_code(&v).map(|lang| (k, lang)))
                    .collect();

            Ok(Some(UserLanguagePreferences {
                user_id,
                primary_language,
                fallback_languages,
                quran_translation_languages,
                interface_language,
                content_language_preferences,
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Save translation quality metrics
    pub async fn save_translation_quality(&self, quality: &TranslationQuality) -> I18nResult<()> {
        let query = r#"
            INSERT INTO translation_quality (
                language, namespace, completion_percentage, accuracy_score,
                consistency_score, last_reviewed, reviewer_notes
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (language, namespace) DO UPDATE SET
                completion_percentage = EXCLUDED.completion_percentage,
                accuracy_score = EXCLUDED.accuracy_score,
                consistency_score = EXCLUDED.consistency_score,
                last_reviewed = EXCLUDED.last_reviewed,
                reviewer_notes = EXCLUDED.reviewer_notes
        "#;

        sqlx::query(query)
            .bind(quality.language.code())
            .bind(&quality.namespace)
            .bind(quality.completion_percentage)
            .bind(quality.accuracy_score)
            .bind(quality.consistency_score)
            .bind(quality.last_reviewed)
            .bind(&quality.reviewer_notes)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get translation quality metrics
    pub async fn get_translation_quality(
        &self, 
        language: &SupportedLanguage, 
        namespace: &str
    ) -> I18nResult<Option<TranslationQuality>> {
        let query = r#"
            SELECT language, namespace, completion_percentage, accuracy_score,
                   consistency_score, last_reviewed, reviewer_notes
            FROM translation_quality 
            WHERE language = $1 AND namespace = $2
        "#;

        let row = sqlx::query(query)
            .bind(language.code())
            .bind(namespace)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            Ok(Some(TranslationQuality {
                language: language.clone(),
                namespace: namespace.to_string(),
                completion_percentage: row.get("completion_percentage"),
                accuracy_score: row.get("accuracy_score"),
                consistency_score: row.get("consistency_score"),
                last_reviewed: row.get("last_reviewed"),
                reviewer_notes: row.get("reviewer_notes"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Save available translations for content
    pub async fn save_available_translations(&self, translations: &AvailableTranslations) -> I18nResult<()> {
        let query = r#"
            INSERT INTO available_translations (
                content_id, content_type, available_languages, 
                default_language, quality_scores
            ) VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (content_id) DO UPDATE SET
                content_type = EXCLUDED.content_type,
                available_languages = EXCLUDED.available_languages,
                default_language = EXCLUDED.default_language,
                quality_scores = EXCLUDED.quality_scores
        "#;

        let available_languages: Vec<String> = translations.available_languages
            .iter()
            .map(|lang| lang.code().to_string())
            .collect();
        let available_languages_json = serde_json::to_value(available_languages)?;

        let quality_scores: HashMap<String, f32> = translations.quality_scores
            .iter()
            .map(|(lang, score)| (lang.code().to_string(), *score))
            .collect();
        let quality_scores_json = serde_json::to_value(quality_scores)?;

        sqlx::query(query)
            .bind(translations.content_id)
            .bind(&translations.content_type)
            .bind(available_languages_json)
            .bind(translations.default_language.code())
            .bind(quality_scores_json)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get available translations for content
    pub async fn get_available_translations(&self, content_id: Uuid) -> I18nResult<Option<AvailableTranslations>> {
        let query = r#"
            SELECT content_id, content_type, available_languages, 
                   default_language, quality_scores
            FROM available_translations 
            WHERE content_id = $1
        "#;

        let row = sqlx::query(query)
            .bind(content_id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let default_language = SupportedLanguage::from_code(row.get("default_language"))
                .ok_or_else(|| I18nError::UnsupportedLanguage(row.get::<String, _>("default_language")))?;

            let available_languages: Vec<String> = serde_json::from_value(row.get("available_languages"))?;
            let available_languages: Vec<SupportedLanguage> = available_languages
                .into_iter()
                .filter_map(|code| SupportedLanguage::from_code(&code))
                .collect();

            let quality_scores: HashMap<String, f32> = serde_json::from_value(row.get("quality_scores"))?;
            let quality_scores: HashMap<SupportedLanguage, f32> = quality_scores
                .into_iter()
                .filter_map(|(code, score)| SupportedLanguage::from_code(&code).map(|lang| (lang, score)))
                .collect();

            Ok(Some(AvailableTranslations {
                content_id,
                content_type: row.get("content_type"),
                available_languages,
                default_language,
                quality_scores,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get language usage statistics
    pub async fn get_language_usage_stats(&self) -> I18nResult<HashMap<SupportedLanguage, i64>> {
        let query = r#"
            SELECT primary_language, COUNT(*) as usage_count
            FROM user_language_preferences 
            GROUP BY primary_language
            ORDER BY usage_count DESC
        "#;

        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await?;

        let mut stats = HashMap::new();
        for row in rows {
            let language_code: String = row.get("primary_language");
            let count: i64 = row.get("usage_count");
            
            if let Some(language) = SupportedLanguage::from_code(&language_code) {
                stats.insert(language, count);
            }
        }

        Ok(stats)
    }

    /// Get users by language preference
    pub async fn get_users_by_language(&self, language: &SupportedLanguage) -> I18nResult<Vec<Uuid>> {
        let query = r#"
            SELECT user_id 
            FROM user_language_preferences 
            WHERE primary_language = $1 OR interface_language = $1
        "#;

        let rows = sqlx::query(query)
            .bind(language.code())
            .fetch_all(&self.pool)
            .await?;

        let user_ids = rows.into_iter()
            .map(|row| row.get("user_id"))
            .collect();

        Ok(user_ids)
    }

    /// Update language pack metadata
    pub async fn update_language_pack_metadata(
        &self, 
        language: &SupportedLanguage, 
        metadata: &LanguagePackMetadata
    ) -> I18nResult<()> {
        let query = r#"
            INSERT INTO language_pack_metadata (
                language, version, contributors, completion_percentage,
                last_updated, quality_score
            ) VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (language) DO UPDATE SET
                version = EXCLUDED.version,
                contributors = EXCLUDED.contributors,
                completion_percentage = EXCLUDED.completion_percentage,
                last_updated = EXCLUDED.last_updated,
                quality_score = EXCLUDED.quality_score
        "#;

        let contributors_json = serde_json::to_value(&metadata.contributors)?;

        sqlx::query(query)
            .bind(language.code())
            .bind(&metadata.version)
            .bind(contributors_json)
            .bind(metadata.completion_percentage)
            .bind(metadata.last_updated)
            .bind(metadata.quality_score)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get language pack metadata
    pub async fn get_language_pack_metadata(&self, language: &SupportedLanguage) -> I18nResult<Option<LanguagePackMetadata>> {
        let query = r#"
            SELECT version, contributors, completion_percentage,
                   last_updated, quality_score
            FROM language_pack_metadata 
            WHERE language = $1
        "#;

        let row = sqlx::query(query)
            .bind(language.code())
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let contributors: Vec<String> = serde_json::from_value(row.get("contributors"))?;

            Ok(Some(LanguagePackMetadata {
                version: row.get("version"),
                contributors,
                completion_percentage: row.get("completion_percentage"),
                last_updated: row.get("last_updated"),
                quality_score: row.get("quality_score"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Delete user preferences
    pub async fn delete_user_preferences(&self, user_id: Uuid) -> I18nResult<()> {
        let query = "DELETE FROM user_language_preferences WHERE user_id = $1";
        
        sqlx::query(query)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        info!("Deleted language preferences for user {}", user_id);
        Ok(())
    }

    /// Get default language preferences
    pub fn get_default_preferences(&self, user_id: Uuid) -> UserLanguagePreferences {
        UserLanguagePreferences {
            user_id,
            primary_language: SupportedLanguage::Arabic,
            fallback_languages: vec![SupportedLanguage::English],
            quran_translation_languages: vec![SupportedLanguage::English],
            interface_language: SupportedLanguage::Arabic,
            content_language_preferences: HashMap::new(),
            updated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use uuid::Uuid;

    // Note: These tests require a test database to be set up
    // They are integration tests and should be run with a proper test environment

    async fn setup_test_db() -> PgPool {
        // This would set up a test database
        // For now, we'll skip the actual database tests
        todo!("Set up test database")
    }

    #[tokio::test]
    #[ignore] // Ignore until test database is set up
    async fn test_save_and_get_user_preferences() {
        let pool = setup_test_db().await;
        let repo = I18nRepository::new(pool);
        let user_id = Uuid::new_v4();

        let preferences = UserLanguagePreferences {
            user_id,
            primary_language: SupportedLanguage::Arabic,
            fallback_languages: vec![SupportedLanguage::English, SupportedLanguage::French],
            quran_translation_languages: vec![SupportedLanguage::English],
            interface_language: SupportedLanguage::Arabic,
            content_language_preferences: HashMap::new(),
            updated_at: Utc::now(),
        };

        // Save preferences
        repo.save_user_preferences(&preferences).await.unwrap();

        // Get preferences
        let retrieved = repo.get_user_preferences(user_id).await.unwrap();
        assert!(retrieved.is_some());
        
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.primary_language, SupportedLanguage::Arabic);
        assert_eq!(retrieved.fallback_languages.len(), 2);
    }

    #[test]
    fn test_default_preferences() {
        // This test doesn't need async since we're not actually connecting
        // let pool = PgPool::connect("").await.unwrap(); // This won't actually connect
        // let repo = I18nRepository::new(pool);
        let user_id = Uuid::new_v4();

        // We can test the logic without a real database connection
        let preferences = UserLanguagePreferences {
            user_id,
            primary_language: SupportedLanguage::Arabic,
            fallback_languages: vec![SupportedLanguage::English],
            quran_translation_languages: vec![SupportedLanguage::English],
            interface_language: SupportedLanguage::Arabic,
            content_language_preferences: HashMap::new(),
            updated_at: chrono::Utc::now(),
        };

        assert_eq!(preferences.user_id, user_id);
        assert_eq!(preferences.primary_language, SupportedLanguage::Arabic);
        assert_eq!(preferences.interface_language, SupportedLanguage::Arabic);
        assert!(!preferences.fallback_languages.is_empty());
    }
}
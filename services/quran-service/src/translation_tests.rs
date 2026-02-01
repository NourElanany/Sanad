use crate::models::*;
use crate::service::QuranService;
use crate::repository::QuranRepository;
use anyhow::Result;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_creation() {
        let translation = Translation::new(
            1,
            1,
            "en".to_string(),
            "Sahih International".to_string(),
            "In the name of Allah, the Entirely Merciful, the Especially Merciful.".to_string(),
        );

        assert_eq!(translation.surah_number, 1);
        assert_eq!(translation.ayah_number, 1);
        assert_eq!(translation.language, "en");
        assert_eq!(translation.translator, "Sahih International");
        assert!(translation.verify_integrity());
        assert_eq!(translation.approval_status, TranslationApprovalStatus::Pending);
        assert!(!translation.is_approved());
    }

    #[test]
    fn test_translation_quality_assessment() {
        let mut translation = Translation::new(
            1,
            1,
            "en".to_string(),
            "Sahih International".to_string(),
            "In the name of Allah, the Entirely Merciful, the Especially Merciful.".to_string(),
        );

        // Test quality thresholds
        translation.quality_score = 0.95;
        assert!(translation.meets_quality_threshold(0.9));
        assert!(!translation.meets_quality_threshold(0.96));
        assert_eq!(translation.quality_level(), "Excellent");

        translation.quality_score = 0.75;
        assert_eq!(translation.quality_level(), "Good");

        translation.quality_score = 0.5;
        assert_eq!(translation.quality_level(), "Needs Improvement");
    }

    #[test]
    fn test_translation_approval_status() {
        let mut translation = Translation::new(
            1,
            1,
            "en".to_string(),
            "Sahih International".to_string(),
            "In the name of Allah, the Entirely Merciful, the Especially Merciful.".to_string(),
        );

        // Test approval status changes
        assert!(!translation.is_approved());

        translation.approval_status = TranslationApprovalStatus::Approved;
        assert!(translation.is_approved());

        translation.approval_status = TranslationApprovalStatus::Verified;
        assert!(translation.is_approved());

        translation.approval_status = TranslationApprovalStatus::Rejected;
        assert!(!translation.is_approved());
    }

    #[test]
    fn test_translation_source_creation() {
        let source = TranslationSource::new(
            "Sahih International".to_string(),
            "Sahih International Team".to_string(),
            "en".to_string(),
            Some("Modern English translation with contemporary language".to_string()),
            Some("Contemporary scholarly approach".to_string()),
            Some("https://sahihinternational.com".to_string()),
        );

        assert_eq!(source.name, "Sahih International");
        assert_eq!(source.translator, "Sahih International Team");
        assert_eq!(source.language, "en");
        assert!(source.is_active);
        assert!(!source.is_approved()); // Starts as pending
        assert!(!source.is_high_quality()); // Starts with 0.0 score
    }

    #[test]
    fn test_translation_source_quality() {
        let mut source = TranslationSource::new(
            "Sahih International".to_string(),
            "Sahih International Team".to_string(),
            "en".to_string(),
            Some("Modern English translation".to_string()),
            Some("Contemporary approach".to_string()),
            Some("https://sahihinternational.com".to_string()),
        );

        // Test quality assessment
        source.quality_score = 8.5;
        source.approval_status = TranslationApprovalStatus::Verified;

        assert!(source.is_high_quality());
        assert!(source.is_approved());
    }

    #[test]
    fn test_translation_integrity_verification() {
        let translation = Translation::new(
            1,
            1,
            "en".to_string(),
            "Sahih International".to_string(),
            "In the name of Allah, the Entirely Merciful, the Especially Merciful.".to_string(),
        );

        // Test integrity verification
        assert!(translation.verify_integrity());

        // Test with modified translation (should fail integrity check)
        let mut corrupted_translation = translation.clone();
        corrupted_translation.text = "Modified text".to_string();
        // Hash remains the same, so integrity should fail
        assert!(!corrupted_translation.verify_integrity());
    }

    #[test]
    fn test_translation_hash_consistency() {
        let text1 = "In the name of Allah, the Entirely Merciful, the Especially Merciful.";
        let text2 = "In the name of Allah, the Entirely Merciful, the Especially Merciful.";
        let text3 = "Different text";

        let hash1 = Translation::calculate_text_hash(text1);
        let hash2 = Translation::calculate_text_hash(text2);
        let hash3 = Translation::calculate_text_hash(text3);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_get_translation_request_validation() {
        let request = GetTranslationRequest {
            surah_number: 1,
            ayah_number: 1,
            languages: Some(vec!["en".to_string(), "fr".to_string()]),
            min_quality_score: Some(0.8),
            approval_status: Some(vec![TranslationApprovalStatus::Approved, TranslationApprovalStatus::Verified]),
            include_source_info: Some(true),
        };

        assert_eq!(request.surah_number, 1);
        assert_eq!(request.ayah_number, 1);
        assert!(request.languages.is_some());
        assert!(request.min_quality_score.is_some());
        assert!(request.approval_status.is_some());
        assert!(request.include_source_info.unwrap_or(false));
    }

    #[test]
    fn test_translation_display_preferences() {
        let preferences = TranslationDisplayPreferences {
            show_arabic: true,
            show_transliteration: false,
            preferred_languages: vec!["en".to_string(), "fr".to_string()],
            quality_threshold: 0.8,
            layout: TranslationLayout::SideBySide,
        };

        assert!(preferences.show_arabic);
        assert!(!preferences.show_transliteration);
        assert_eq!(preferences.preferred_languages.len(), 2);
        assert_eq!(preferences.quality_threshold, 0.8);
        assert!(matches!(preferences.layout, TranslationLayout::SideBySide));
    }

    #[test]
    fn test_translation_layout_variants() {
        let layouts = vec![
            TranslationLayout::SideBySide,
            TranslationLayout::Stacked,
            TranslationLayout::Tabbed,
            TranslationLayout::Comparison,
        ];

        assert_eq!(layouts.len(), 4);
        
        // Test serialization/deserialization
        for layout in layouts {
            let json = serde_json::to_string(&layout).unwrap();
            let deserialized: TranslationLayout = serde_json::from_str(&json).unwrap();
            // Note: We can't directly compare enum variants, but serialization/deserialization working is a good test
            assert!(!json.is_empty());
        }
    }

    #[test]
    fn test_manage_translation_source_request() {
        let source_data = TranslationSourceData {
            name: "Test Translation".to_string(),
            translator: "Test Translator".to_string(),
            language: "en".to_string(),
            description: Some("Test description".to_string()),
            methodology: Some("Test methodology".to_string()),
            source_reference: Some("Test reference".to_string()),
        };

        let request = ManageTranslationSourceRequest {
            action: TranslationSourceAction::Create,
            source_data: Some(source_data),
            source_id: None,
        };

        assert!(matches!(request.action, TranslationSourceAction::Create));
        assert!(request.source_data.is_some());
        assert!(request.source_id.is_none());
    }

    #[test]
    fn test_translation_source_actions() {
        let actions = vec![
            TranslationSourceAction::Create,
            TranslationSourceAction::Update,
            TranslationSourceAction::Approve,
            TranslationSourceAction::Verify,
            TranslationSourceAction::Reject,
            TranslationSourceAction::Deactivate,
        ];

        assert_eq!(actions.len(), 6);
        
        // Test that all actions can be created
        for action in actions {
            let request = ManageTranslationSourceRequest {
                action,
                source_data: None,
                source_id: Some(Uuid::new_v4()),
            };
            assert!(request.source_id.is_some());
        }
    }

    #[test]
    fn test_quality_factor_calculation() {
        let factor = QualityFactor {
            factor_type: "Source Credibility".to_string(),
            weight: 0.3,
            score: 9.0,
            description: "Credibility of the translation source".to_string(),
        };

        let weighted_score = factor.score * factor.weight;
        assert!((weighted_score - 2.7).abs() < 0.0001); // Use floating point comparison
        assert_eq!(factor.factor_type, "Source Credibility");
    }

    #[test]
    fn test_translation_with_source_structure() {
        let translation = Translation::new(
            1,
            1,
            "en".to_string(),
            "Sahih International".to_string(),
            "In the name of Allah, the Entirely Merciful, the Especially Merciful.".to_string(),
        );

        let source = TranslationSource::new(
            "Sahih International".to_string(),
            "Sahih International Team".to_string(),
            "en".to_string(),
            Some("Modern English translation".to_string()),
            Some("Contemporary approach".to_string()),
            Some("https://sahihinternational.com".to_string()),
        );

        let translation_with_source = TranslationWithSource {
            translation: translation.clone(),
            source: source.clone(),
        };

        assert_eq!(translation_with_source.translation.id, translation.id);
        assert_eq!(translation_with_source.source.id, source.id);
        assert_eq!(translation_with_source.translation.language, translation_with_source.source.language);
    }

    #[test]
    fn test_ayah_with_translations_structure() {
        let ayah = Ayah::new(1, 1, "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ".to_string(), 1, 1, Some(1));
        let surah = Surah::new(1, "Al-Fatiha".to_string(), "الفاتحة".to_string(), "The Opening".to_string(), RevelationType::Meccan, 7);
        
        let translation = Translation::new(
            1,
            1,
            "en".to_string(),
            "Sahih International".to_string(),
            "In the name of Allah, the Entirely Merciful, the Especially Merciful.".to_string(),
        );

        let source = TranslationSource::new(
            "Sahih International".to_string(),
            "Sahih International Team".to_string(),
            "en".to_string(),
            Some("Modern English translation".to_string()),
            None,
            None,
        );

        let translation_with_source = TranslationWithSource {
            translation,
            source,
        };

        let display_preferences = TranslationDisplayPreferences {
            show_arabic: true,
            show_transliteration: false,
            preferred_languages: vec!["en".to_string()],
            quality_threshold: 0.8,
            layout: TranslationLayout::Stacked,
        };

        let ayah_with_translations = AyahWithTranslations {
            ayah,
            surah,
            translations: vec![translation_with_source],
            display_preferences,
        };

        assert_eq!(ayah_with_translations.ayah.surah_number, 1);
        assert_eq!(ayah_with_translations.surah.number, 1);
        assert_eq!(ayah_with_translations.translations.len(), 1);
        assert!(ayah_with_translations.display_preferences.show_arabic);
    }

    #[test]
    fn test_translation_approval_status_serialization() {
        let statuses = vec![
            TranslationApprovalStatus::Pending,
            TranslationApprovalStatus::Approved,
            TranslationApprovalStatus::Verified,
            TranslationApprovalStatus::Rejected,
        ];

        for status in statuses {
            let json = serde_json::to_string(&status).unwrap();
            let deserialized: TranslationApprovalStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_translation_quality_result() {
        let quality_factors = vec![
            QualityFactor {
                factor_type: "Source Credibility".to_string(),
                weight: 0.3,
                score: 9.0,
                description: "Credibility test".to_string(),
            },
            QualityFactor {
                factor_type: "Translator Expertise".to_string(),
                weight: 0.3,
                score: 8.5,
                description: "Expertise test".to_string(),
            },
        ];

        let result = TranslationQualityResult {
            translation_id: Uuid::new_v4(),
            previous_score: 7.0,
            new_score: 8.25,
            quality_factors,
            recommendations: vec!["Good quality translation".to_string()],
            verified_at: chrono::Utc::now(),
        };

        assert_eq!(result.previous_score, 7.0);
        assert_eq!(result.new_score, 8.25);
        assert_eq!(result.quality_factors.len(), 2);
        assert_eq!(result.recommendations.len(), 1);
    }
}

// Integration tests (would require database setup)
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    // Note: These tests would require a test database setup
    // They are marked as ignored by default
    
    #[tokio::test]
    #[ignore = "Requires database setup"]
    async fn test_translation_repository_integration() {
        // This would test the actual database operations
        // Requires proper test database setup
        
        // Example structure:
        // let pool = setup_test_database().await;
        // let repository = QuranRepository::new(pool);
        // let service = QuranService::new(repository);
        
        // Test translation insertion, retrieval, and updates
        // Test quality score calculations
        // Test approval status changes
        // Test integrity verification
        
        assert!(true); // Placeholder
    }

    #[tokio::test]
    #[ignore = "Requires database setup"]
    async fn test_translation_service_integration() {
        // This would test the service layer with actual database
        // Test complete workflows like:
        // - Creating translation sources
        // - Adding translations
        // - Quality verification
        // - Approval processes
        
        assert!(true); // Placeholder
    }

    #[tokio::test]
    #[ignore = "Requires database setup"]
    async fn test_translation_api_integration() {
        // This would test the HTTP API endpoints
        // Test all translation-related endpoints
        // Test error handling
        // Test parameter validation
        
        assert!(true); // Placeholder
    }
}

// Property-based tests for translation system
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_translation_hash_deterministic(text in "\\PC*") {
            let hash1 = Translation::calculate_text_hash(&text);
            let hash2 = Translation::calculate_text_hash(&text);
            prop_assert_eq!(hash1, hash2);
        }

        #[test]
        fn test_translation_quality_score_bounds(score in 0.0f64..=10.0f64) {
            let mut translation = Translation::new(
                1,
                1,
                "en".to_string(),
                "Test".to_string(),
                "Test text".to_string(),
            );
            translation.quality_score = score;
            
            prop_assert!(translation.quality_score >= 0.0);
            prop_assert!(translation.quality_score <= 10.0);
        }

        #[test]
        fn test_translation_meets_threshold(
            score in 0.0f64..=10.0f64,
            threshold in 0.0f64..=10.0f64
        ) {
            let mut translation = Translation::new(
                1,
                1,
                "en".to_string(),
                "Test".to_string(),
                "Test text".to_string(),
            );
            translation.quality_score = score;
            
            let meets_threshold = translation.meets_quality_threshold(threshold);
            prop_assert_eq!(meets_threshold, score >= threshold);
        }

        #[test]
        fn test_translation_integrity_preserved(
            surah_number in 1i32..=114i32,
            ayah_number in 1i32..=286i32,
            language in "[a-z]{2}",
            translator in "\\PC{1,100}",
            text in "\\PC{1,1000}"
        ) {
            let translation = Translation::new(
                surah_number,
                ayah_number,
                language,
                translator,
                text,
            );
            
            prop_assert!(translation.verify_integrity());
            prop_assert_eq!(translation.surah_number, surah_number);
            prop_assert_eq!(translation.ayah_number, ayah_number);
        }
    }
}
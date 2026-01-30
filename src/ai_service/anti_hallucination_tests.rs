use super::anti_hallucination::*;
use super::*;
use tokio;

/// Test suite for the enhanced anti-hallucination system
/// **Validates: Requirements 5.3, 5.4, 5.6**

#[cfg(test)]
mod tests {
    use super::*;

    /// Test confidence scoring system
    #[tokio::test]
    async fn test_confidence_scoring_system() {
        let confidence_assessor = ConfidenceAssessor::new();
        
        // Test with high-quality sources
        let high_quality_sources = vec![
            create_test_quran_source(),
            create_test_sahih_hadith_source(),
        ];
        
        let simple_query = create_test_query(ComplexityLevel::Simple);
        let response_text = "الصلاة هي الركن الثاني من أركان الإسلام، وهي فرض على كل مسلم بالغ عاقل.";
        
        let confidence = confidence_assessor
            .assess_confidence(response_text, &high_quality_sources, &simple_query)
            .await
            .unwrap();
        
        assert!(confidence > 0.7, "High-quality sources should result in high confidence: {}", confidence);
        
        // Test with low-quality sources
        let low_quality_sources = vec![
            create_test_weak_hadith_source(),
        ];
        
        let confidence_low = confidence_assessor
            .assess_confidence(response_text, &low_quality_sources, &simple_query)
            .await
            .unwrap();
        
        assert!(confidence_low < confidence, "Low-quality sources should result in lower confidence");
        
        // Test with no sources
        let confidence_no_sources = confidence_assessor
            .assess_confidence(response_text, &[], &simple_query)
            .await
            .unwrap();
        
        assert!(confidence_no_sources < 0.3, "No sources should result in very low confidence");
    }

    /// Test fabricated content detection
    #[tokio::test]
    async fn test_fabricated_content_detection() {
        let anti_hallucination = AntiHallucinationSystem::new();
        
        // Test with fabricated Quranic verse
        let fabricated_quran_response = r#"قال الله تعالى: "إن الصلاة تنهى عن الفحشاء والمنكر والبغي""#;
        let sources = vec![create_test_quran_source()];
        let query = create_test_query(ComplexityLevel::Simple);
        
        let result = anti_hallucination
            .check_response(fabricated_quran_response, &sources, &query)
            .await
            .unwrap();
        
        // Should detect potential fabrication if the verse is not in sources
        assert!(result.hallucination_risk_score > 0.0, "Should detect potential fabrication risk");
        
        // Test with fabricated hadith
        let fabricated_hadith_response = r#"قال الرسول صلى الله عليه وسلم: "من صلى الفجر في جماعة فله أجر عظيم جداً""#;
        
        let hadith_result = anti_hallucination
            .check_response(fabricated_hadith_response, &sources, &query)
            .await
            .unwrap();
        
        assert!(hadith_result.hallucination_risk_score > 0.0, "Should detect potential hadith fabrication");
    }

    /// Test unsupported claims detection
    #[tokio::test]
    async fn test_unsupported_claims_detection() {
        let anti_hallucination = AntiHallucinationSystem::new();
        
        // Response with unsupported claims
        let response_with_claims = "الصلاة واجبة على كل مسلم. وقد ثبت أن 99% من المسلمين يصلون يومياً. قال العالم الفلاني: الصلاة هي أهم العبادات.";
        let limited_sources = vec![create_test_quran_source()]; // Only Quran source, no statistics or scholar quotes
        let query = create_test_query(ComplexityLevel::Intermediate);
        
        let result = anti_hallucination
            .check_response(response_with_claims, &limited_sources, &query)
            .await
            .unwrap();
        
        assert!(!result.unsupported_claims.is_empty(), "Should detect unsupported statistical claims");
        
        // Check for specific types of unsupported claims
        let has_statistical_claim = result.unsupported_claims.iter()
            .any(|claim| claim.claim.contains("99%"));
        assert!(has_statistical_claim, "Should detect unsupported statistical claim");
    }

    /// Test contradiction detection
    #[tokio::test]
    async fn test_contradiction_detection() {
        let anti_hallucination = AntiHallucinationSystem::new();
        
        // Response with internal contradictions
        let contradictory_response = "الصلاة واجبة على كل مسلم. لكن الصلاة مكروهة في بعض الأوقات. الصلاة حرام في أوقات النهي.";
        let sources = vec![create_test_quran_source()];
        let query = create_test_query(ComplexityLevel::Advanced);
        
        let result = anti_hallucination
            .check_response(contradictory_response, &sources, &query)
            .await
            .unwrap();
        
        assert!(!result.contradictions.is_empty(), "Should detect internal contradictions");
        assert!(result.hallucination_risk_score > 0.5, "Contradictions should increase hallucination risk");
    }

    /// Test recommendation system
    #[tokio::test]
    async fn test_recommendation_system() {
        let anti_hallucination = AntiHallucinationSystem::new();
        
        // Test with high-risk content (fake Quranic verse)
        let high_risk_response = r#"قال الله تعالى: "هذه آية مختلقة لا توجد في القرآن""#;
        let sources = vec![create_test_quran_source()];
        let query = create_test_query(ComplexityLevel::Simple);
        
        let result = anti_hallucination
            .check_response(high_risk_response, &sources, &query)
            .await
            .unwrap();
        
        // Should recommend rejection for fabricated Quranic content
        assert!(matches!(result.recommendation, ResponseRecommendation::Reject), 
                "Should recommend rejection for fabricated Quranic content");
        
        // Test with medium-risk content
        let medium_risk_response = "الصلاة مستحبة. قال بعض العلماء أنها واجبة.";
        
        let medium_result = anti_hallucination
            .check_response(medium_risk_response, &sources, &query)
            .await
            .unwrap();
        
        // Should recommend revision or warning for inconsistent content
        assert!(matches!(medium_result.recommendation, 
                ResponseRecommendation::RequireRevision | 
                ResponseRecommendation::ApproveWithWarning),
                "Should recommend revision or warning for inconsistent content");
    }

    /// Test enhanced source verification
    #[tokio::test]
    async fn test_enhanced_source_verification() {
        let source_verifier = SourceVerifier::new();
        
        // Test exact match
        let exact_fact = ExtractedFact {
            claim: "الصلاة فرض على كل مسلم".to_string(),
            position: TextPosition { start: 0, end: 20, line: 1 },
            fact_type: FactType::ReligiousRuling,
            confidence: 0.9,
        };
        
        let matching_source = IslamicSource {
            id: "test_source".to_string(),
            content_type: SourceType::Quran,
            text: "الصلاة فرض على كل مسلم بالغ عاقل".to_string(),
            reference: "Test".to_string(),
            author: None,
            authenticity: AuthenticityLevel::Verified,
            language: Language::Arabic,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        };
        
        let support_level = source_verifier
            .verify_fact_support_detailed(&exact_fact, &[matching_source])
            .await
            .unwrap();
        
        assert!(support_level.support_score > 0.8, "Exact match should have high support score");
        assert!(matches!(support_level.support_type, SupportType::ExactMatch | SupportType::StrongSupport));
        
        // Test no support
        let unsupported_fact = ExtractedFact {
            claim: "الصلاة مكروهة في جميع الأوقات".to_string(),
            position: TextPosition { start: 0, end: 30, line: 1 },
            fact_type: FactType::ReligiousRuling,
            confidence: 0.9,
        };
        
        let no_support = source_verifier
            .verify_fact_support_detailed(&unsupported_fact, &[matching_source])
            .await
            .unwrap();
        
        assert!(no_support.support_score < 0.3, "Contradictory claim should have low support score");
    }

    /// Test out-of-scope question handling with fallback
    #[tokio::test]
    async fn test_out_of_scope_fallback() {
        let out_of_scope_detector = OutOfScopeDetector::new();
        
        // Test clearly out-of-scope question
        let tech_question = "كيف أبرمج تطبيق للهاتف الذكي؟";
        let analysis = out_of_scope_detector.get_scope_analysis(tech_question);
        
        assert_eq!(analysis.scope_status, ScopeStatus::OutOfScope);
        assert!(!analysis.out_of_scope_keywords.is_empty());
        
        let fallback = out_of_scope_detector.generate_fallback_response(&analysis);
        assert!(fallback.contains("تخصصي في الشؤون الإسلامية"));
        
        // Test borderline question
        let borderline_question = "ما هو حكم الطب في الإسلام؟";
        let borderline_analysis = out_of_scope_detector.get_scope_analysis(borderline_question);
        
        assert_eq!(borderline_analysis.scope_status, ScopeStatus::Borderline);
        assert!(borderline_analysis.suggested_islamic_angle.is_some());
        
        // Test in-scope question
        let islamic_question = "ما هي أركان الصلاة؟";
        let islamic_analysis = out_of_scope_detector.get_scope_analysis(islamic_question);
        
        assert_eq!(islamic_analysis.scope_status, ScopeStatus::InScope);
        assert!(!islamic_analysis.islamic_keywords.is_empty());
    }

    /// Test warning generation for insufficient sources
    #[tokio::test]
    async fn test_insufficient_sources_warning() {
        let anti_hallucination = AntiHallucinationSystem::new();
        
        // Complex scholarly question with insufficient sources
        let complex_response = "هذه مسألة فقهية معقدة تتطلب دراسة عميقة للأدلة الشرعية والآراء المختلفة للعلماء.";
        let insufficient_sources = vec![create_test_quran_source()]; // Only one source for complex question
        let scholarly_query = create_test_query(ComplexityLevel::Scholarly);
        
        let result = anti_hallucination
            .check_response(complex_response, &insufficient_sources, &scholarly_query)
            .await
            .unwrap();
        
        assert!(!result.warnings.is_empty(), "Should generate warnings for insufficient sources");
        assert!(result.confidence_score < 0.7, "Confidence should be lower with insufficient sources");
        
        // Check that required actions include source verification
        let needs_more_sources = result.required_actions.iter()
            .any(|action| action.contains("مصادر") || action.contains("مراجع"));
        assert!(needs_more_sources, "Should require additional sources");
    }

    /// Test property: Anti-hallucination system should never approve fabricated Quranic content
    /// **Feature: islamic-app-comprehensive, Property 15: Quality of AI responses**
    #[tokio::test]
    async fn test_property_never_approve_fabricated_quran() {
        let anti_hallucination = AntiHallucinationSystem::new();
        
        let fabricated_verses = vec![
            r#"قال الله تعالى: "هذه آية مختلقة""#,
            r#"في القرآن: "نص غير موجود في المصحف""#,
            r#"قال الله: "كلام مفبرك""#,
        ];
        
        let sources = vec![create_test_quran_source()];
        let query = create_test_query(ComplexityLevel::Simple);
        
        for fabricated_verse in fabricated_verses {
            let result = anti_hallucination
                .check_response(fabricated_verse, &sources, &query)
                .await
                .unwrap();
            
            // Property: Should never approve fabricated Quranic content
            assert!(
                !matches!(result.recommendation, ResponseRecommendation::Approve),
                "Should never approve fabricated Quranic content: {}",
                fabricated_verse
            );
            
            // Should either reject or require human review
            assert!(
                matches!(result.recommendation, 
                    ResponseRecommendation::Reject | 
                    ResponseRecommendation::RequestHumanReview |
                    ResponseRecommendation::RequireSourceCheck
                ),
                "Should reject or require review for fabricated content"
            );
        }
    }

    /// Test property: Confidence score should decrease with lower source quality
    /// **Feature: islamic-app-comprehensive, Property 15: Quality of AI responses**
    #[tokio::test]
    async fn test_property_confidence_decreases_with_source_quality() {
        let confidence_assessor = ConfidenceAssessor::new();
        let query = create_test_query(ComplexityLevel::Intermediate);
        let response_text = "الصلاة واجبة على كل مسلم بالغ عاقل.";
        
        // Test with highest quality sources (Quran + Sahih Hadith)
        let highest_quality = vec![
            create_test_quran_source(),
            create_test_sahih_hadith_source(),
        ];
        
        let confidence_highest = confidence_assessor
            .assess_confidence(response_text, &highest_quality, &query)
            .await
            .unwrap();
        
        // Test with medium quality sources (Hasan Hadith + Tafsir)
        let medium_quality = vec![
            create_test_hasan_hadith_source(),
            create_test_tafsir_source(),
        ];
        
        let confidence_medium = confidence_assessor
            .assess_confidence(response_text, &medium_quality, &query)
            .await
            .unwrap();
        
        // Test with lowest quality sources (Weak Hadith)
        let lowest_quality = vec![
            create_test_weak_hadith_source(),
        ];
        
        let confidence_lowest = confidence_assessor
            .assess_confidence(response_text, &lowest_quality, &query)
            .await
            .unwrap();
        
        // Property: Confidence should decrease with source quality
        assert!(
            confidence_highest > confidence_medium,
            "Highest quality sources should have higher confidence than medium: {} vs {}",
            confidence_highest, confidence_medium
        );
        
        assert!(
            confidence_medium > confidence_lowest,
            "Medium quality sources should have higher confidence than lowest: {} vs {}",
            confidence_medium, confidence_lowest
        );
    }

    // Helper functions for creating test data

    fn create_test_quran_source() -> IslamicSource {
        IslamicSource {
            id: "quran_test".to_string(),
            content_type: SourceType::Quran,
            text: "وَأَقِيمُوا الصَّلَاةَ وَآتُوا الزَّكَاةَ وَارْكَعُوا مَعَ الرَّاكِعِينَ".to_string(),
            reference: "البقرة: 43".to_string(),
            author: None,
            authenticity: AuthenticityLevel::Verified,
            language: Language::Arabic,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        }
    }

    fn create_test_sahih_hadith_source() -> IslamicSource {
        IslamicSource {
            id: "hadith_sahih_test".to_string(),
            content_type: SourceType::SahihHadith,
            text: "بني الإسلام على خمس: شهادة أن لا إله إلا الله وأن محمداً رسول الله، وإقام الصلاة".to_string(),
            reference: "صحيح البخاري".to_string(),
            author: Some("البخاري".to_string()),
            authenticity: AuthenticityLevel::Verified,
            language: Language::Arabic,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        }
    }

    fn create_test_hasan_hadith_source() -> IslamicSource {
        IslamicSource {
            id: "hadith_hasan_test".to_string(),
            content_type: SourceType::HasanHadith,
            text: "الصلاة عماد الدين".to_string(),
            reference: "حديث حسن".to_string(),
            author: Some("الترمذي".to_string()),
            authenticity: AuthenticityLevel::Reliable,
            language: Language::Arabic,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        }
    }

    fn create_test_weak_hadith_source() -> IslamicSource {
        IslamicSource {
            id: "hadith_weak_test".to_string(),
            content_type: SourceType::DaifHadith,
            text: "حديث ضعيف عن الصلاة".to_string(),
            reference: "حديث ضعيف".to_string(),
            author: Some("راوي ضعيف".to_string()),
            authenticity: AuthenticityLevel::Questionable,
            language: Language::Arabic,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        }
    }

    fn create_test_tafsir_source() -> IslamicSource {
        IslamicSource {
            id: "tafsir_test".to_string(),
            content_type: SourceType::Tafsir,
            text: "الصلاة هي الركن الثاني من أركان الإسلام وهي فرض على كل مسلم".to_string(),
            reference: "تفسير ابن كثير".to_string(),
            author: Some("ابن كثير".to_string()),
            authenticity: AuthenticityLevel::Verified,
            language: Language::Arabic,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        }
    }

    fn create_test_query(complexity: ComplexityLevel) -> ProcessedQuestion {
        ProcessedQuestion {
            original_text: "ما حكم الصلاة؟".to_string(),
            normalized_text: "ما حكم الصلاة".to_string(),
            keywords: vec!["حكم".to_string(), "صلاة".to_string()],
            concepts: vec!["صلاة".to_string(), "فقه".to_string()],
            question_type: QuestionType::Fiqh,
            complexity_level: complexity,
            language: Language::Arabic,
            is_controversial: false,
            requires_multiple_sources: false,
            embedding: None,
        }
    }
}
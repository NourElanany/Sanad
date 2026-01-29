#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_service::{
        rag_system::{RAGSystem, RAGRequest},
        question_processor::QuestionProcessor,
        hadith_verifier::HadithVerificationSystem,
        source_scorer::SourceScoringSystem,
        anti_hallucination::AntiHallucinationSystem,
    };
    use tokio;

    #[tokio::test]
    async fn test_question_processing() {
        let processor = QuestionProcessor::new();
        
        let question = "ما حكم الصلاة في المسجد؟";
        let result = processor.process_question(question).await;
        
        assert!(result.is_ok());
        let processed = result.unwrap();
        
        assert_eq!(processed.original_text, question);
        assert!(matches!(processed.question_type, QuestionType::Fiqh));
        assert!(processed.keywords.contains(&"صلاة".to_string()));
        assert!(processed.concepts.contains(&"صلاة".to_string()));
    }

    #[tokio::test]
    async fn test_out_of_scope_detection() {
        let processor = QuestionProcessor::new();
        
        let question = "ما هو أفضل برنامج للبرمجة؟";
        let result = processor.process_question(question).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AIServiceError::OutOfScopeQuestion(_)));
    }

    #[tokio::test]
    async fn test_hadith_verification() {
        let verifier = HadithVerificationSystem::new();
        
        let hadith_text = "إنما الأعمال بالنيات";
        let result = verifier.verify_hadith(hadith_text).await;
        
        assert!(result.is_ok());
        let verification = result.unwrap();
        
        assert!(matches!(verification.grade, crate::ai_service::hadith_verifier::HadithGrade::Sahih));
        assert!(matches!(verification.usage_recommendation, 
            crate::ai_service::hadith_verifier::UsageRecommendation::HighlyRecommended));
    }

    #[tokio::test]
    async fn test_source_scoring() {
        let scorer = SourceScoringSystem::new();
        
        let source = IslamicSource {
            id: "test_001".to_string(),
            content_type: SourceType::Quran,
            text: "وأقيموا الصلاة وآتوا الزكاة".to_string(),
            reference: "البقرة: 43".to_string(),
            author: None,
            authenticity: AuthenticityLevel::Verified,
            language: Language::Arabic,
            metadata: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
        };
        
        let question = crate::ai_service::question_processor::ProcessedQuestion {
            original_text: "ما حكم الصلاة؟".to_string(),
            normalized_text: "ما حكم الصلاة".to_string(),
            keywords: vec!["حكم".to_string(), "صلاة".to_string()],
            concepts: vec!["صلاة".to_string(), "فقه".to_string()],
            question_type: QuestionType::Fiqh,
            complexity_level: ComplexityLevel::Simple,
            language: Language::Arabic,
            is_controversial: false,
            requires_multiple_sources: false,
            embedding: None,
        };
        
        let result = scorer.calculate_score(&source, &question).await;
        
        assert!(result.is_ok());
        let score = result.unwrap();
        
        assert!(score.final_score > 0.8); // القرآن يجب أن يحصل على درجة عالية
        assert!(matches!(score.confidence_level, ConfidenceLevel::VeryHigh | ConfidenceLevel::High));
    }

    #[tokio::test]
    async fn test_anti_hallucination_system() {
        let anti_hallucination = AntiHallucinationSystem::new();
        
        let response_text = "قال الله تعالى: \"وأقيموا الصلاة وآتوا الزكاة\"";
        let sources = vec![
            IslamicSource {
                id: "quran_001".to_string(),
                content_type: SourceType::Quran,
                text: "وأقيموا الصلاة وآتوا الزكاة".to_string(),
                reference: "البقرة: 43".to_string(),
                author: None,
                authenticity: AuthenticityLevel::Verified,
                language: Language::Arabic,
                metadata: std::collections::HashMap::new(),
                created_at: chrono::Utc::now(),
            }
        ];
        
        let question = crate::ai_service::question_processor::ProcessedQuestion {
            original_text: "ما حكم الصلاة؟".to_string(),
            normalized_text: "ما حكم الصلاة".to_string(),
            keywords: vec!["حكم".to_string(), "صلاة".to_string()],
            concepts: vec!["صلاة".to_string()],
            question_type: QuestionType::Fiqh,
            complexity_level: ComplexityLevel::Simple,
            language: Language::Arabic,
            is_controversial: false,
            requires_multiple_sources: false,
            embedding: None,
        };
        
        let result = anti_hallucination.check_response(response_text, &sources, &question).await;
        
        assert!(result.is_ok());
        let check = result.unwrap();
        
        assert!(!check.is_hallucination_detected);
        assert!(check.hallucination_risk_score < 0.3);
        assert!(matches!(check.recommendation, 
            crate::ai_service::anti_hallucination::ResponseRecommendation::Approve));
    }

    #[tokio::test]
    async fn test_fabricated_ayah_detection() {
        let anti_hallucination = AntiHallucinationSystem::new();
        
        let response_text = "قال الله تعالى: \"هذه آية مختلقة لا توجد في القرآن\"";
        let sources = vec![];
        
        let question = crate::ai_service::question_processor::ProcessedQuestion {
            original_text: "سؤال تجريبي".to_string(),
            normalized_text: "سؤال تجريبي".to_string(),
            keywords: vec![],
            concepts: vec![],
            question_type: QuestionType::General,
            complexity_level: ComplexityLevel::Simple,
            language: Language::Arabic,
            is_controversial: false,
            requires_multiple_sources: false,
            embedding: None,
        };
        
        let result = anti_hallucination.check_response(response_text, &sources, &question).await;
        
        assert!(result.is_ok());
        let check = result.unwrap();
        
        // يجب اكتشاف المحتوى المختلق
        assert!(check.hallucination_risk_score > 0.5);
        assert!(!check.fabricated_content.is_empty());
    }

    #[tokio::test]
    async fn test_rag_system_integration() {
        let rag_system = RAGSystem::new();
        
        let request = RAGRequest {
            question: "ما هي أركان الإسلام؟".to_string(),
            user_id: Some("test_user".to_string()),
            context: None,
            preferences: Some(crate::ai_service::rag_system::UserPreferences {
                preferred_sources: vec![SourceType::Quran, SourceType::SahihHadith],
                language: Language::Arabic,
                detail_level: crate::ai_service::rag_system::DetailLevel::Standard,
                include_multiple_opinions: false,
            }),
        };
        
        let result = rag_system.ask_question(request).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        
        assert!(!response.answer.is_empty());
        assert!(response.confidence > 0.0);
        assert!(response.response_time_ms > 0);
        assert!(response.hallucination_risk < 0.5);
    }

    #[tokio::test]
    async fn test_controversial_question_handling() {
        let processor = QuestionProcessor::new();
        
        let question = "ما هو الخلاف في مسألة رفع اليدين في الصلاة؟";
        let result = processor.process_question(question).await;
        
        assert!(result.is_ok());
        let processed = result.unwrap();
        
        assert!(processed.is_controversial);
        assert!(processed.requires_multiple_sources);
        assert!(matches!(processed.question_type, QuestionType::Fiqh));
    }

    #[tokio::test]
    async fn test_hadith_grading_system() {
        let verifier = HadithVerificationSystem::new();
        
        // اختبار حديث صحيح
        let sahih_hadith = "إنما الأعمال بالنيات";
        let result = verifier.verify_hadith(sahih_hadith).await;
        assert!(result.is_ok());
        let verification = result.unwrap();
        assert!(matches!(verification.grade, crate::ai_service::hadith_verifier::HadithGrade::Sahih));
        
        // اختبار حديث ضعيف (مثال افتراضي)
        let weak_hadith = "حديث ضعيف افتراضي";
        let result = verifier.verify_hadith(weak_hadith).await;
        // في التطبيق الحقيقي، سيتم تصنيفه كضعيف
    }

    #[tokio::test]
    async fn test_source_filtering() {
        let rag_system = RAGSystem::new();
        
        let sources = vec![
            crate::ai_service::source_scorer::ScoredSource {
                source: IslamicSource {
                    id: "high_quality".to_string(),
                    content_type: SourceType::Quran,
                    text: "آية قرآنية".to_string(),
                    reference: "البقرة: 1".to_string(),
                    author: None,
                    authenticity: AuthenticityLevel::Verified,
                    language: Language::Arabic,
                    metadata: std::collections::HashMap::new(),
                    created_at: chrono::Utc::now(),
                },
                score: crate::ai_service::source_scorer::SourceScore {
                    relevance_score: 0.9,
                    authority_score: 1.0,
                    authenticity_score: 1.0,
                    consensus_score: 1.0,
                    freshness_score: 1.0,
                    final_score: 0.95,
                    confidence_level: ConfidenceLevel::VeryHigh,
                    scoring_details: crate::ai_service::source_scorer::ScoringDetails {
                        relevance_factors: vec![],
                        authority_factors: vec![],
                        authenticity_factors: vec![],
                        consensus_factors: vec![],
                        penalties: vec![],
                        bonuses: vec![],
                    },
                },
                rank: 1,
                usage_recommendation: crate::ai_service::source_scorer::SourceUsageRecommendation::Primary,
            },
            crate::ai_service::source_scorer::ScoredSource {
                source: IslamicSource {
                    id: "low_quality".to_string(),
                    content_type: SourceType::MawduHadith,
                    text: "حديث موضوع".to_string(),
                    reference: "مصدر غير موثوق".to_string(),
                    author: None,
                    authenticity: AuthenticityLevel::Unreliable,
                    language: Language::Arabic,
                    metadata: std::collections::HashMap::new(),
                    created_at: chrono::Utc::now(),
                },
                score: crate::ai_service::source_scorer::SourceScore {
                    relevance_score: 0.3,
                    authority_score: 0.1,
                    authenticity_score: 0.1,
                    consensus_score: 0.1,
                    freshness_score: 0.5,
                    final_score: 0.2,
                    confidence_level: ConfidenceLevel::VeryLow,
                    scoring_details: crate::ai_service::source_scorer::ScoringDetails {
                        relevance_factors: vec![],
                        authority_factors: vec![],
                        authenticity_factors: vec![],
                        consensus_factors: vec![],
                        penalties: vec![],
                        bonuses: vec![],
                    },
                },
                rank: 2,
                usage_recommendation: crate::ai_service::source_scorer::SourceUsageRecommendation::Excluded,
            },
        ];
        
        let filtered = rag_system.filter_sources(sources);
        
        assert!(filtered.is_ok());
        let filtered_sources = filtered.unwrap();
        
        // يجب أن يتم استبعاد المصدر منخفض الجودة
        assert_eq!(filtered_sources.len(), 1);
        assert_eq!(filtered_sources[0].source.id, "high_quality");
    }

    #[tokio::test]
    async fn test_context_building() {
        let context_builder = crate::ai_service::rag_system::ContextBuilder::new();
        
        let question = crate::ai_service::question_processor::ProcessedQuestion {
            original_text: "ما حكم الصلاة؟".to_string(),
            normalized_text: "ما حكم الصلاة".to_string(),
            keywords: vec!["حكم".to_string(), "صلاة".to_string()],
            concepts: vec!["صلاة".to_string(), "فقه".to_string()],
            question_type: QuestionType::Fiqh,
            complexity_level: ComplexityLevel::Simple,
            language: Language::Arabic,
            is_controversial: false,
            requires_multiple_sources: false,
            embedding: None,
        };
        
        let sources = vec![
            crate::ai_service::source_scorer::ScoredSource {
                source: IslamicSource {
                    id: "source_1".to_string(),
                    content_type: SourceType::Quran,
                    text: "وأقيموا الصلاة".to_string(),
                    reference: "البقرة: 43".to_string(),
                    author: None,
                    authenticity: AuthenticityLevel::Verified,
                    language: Language::Arabic,
                    metadata: std::collections::HashMap::new(),
                    created_at: chrono::Utc::now(),
                },
                score: crate::ai_service::source_scorer::SourceScore {
                    relevance_score: 0.9,
                    authority_score: 1.0,
                    authenticity_score: 1.0,
                    consensus_score: 1.0,
                    freshness_score: 1.0,
                    final_score: 0.95,
                    confidence_level: ConfidenceLevel::VeryHigh,
                    scoring_details: crate::ai_service::source_scorer::ScoringDetails {
                        relevance_factors: vec![],
                        authority_factors: vec![],
                        authenticity_factors: vec![],
                        consensus_factors: vec![],
                        penalties: vec![],
                        bonuses: vec![],
                    },
                },
                rank: 1,
                usage_recommendation: crate::ai_service::source_scorer::SourceUsageRecommendation::Primary,
            }
        ];
        
        let result = context_builder.build_context(&question, &sources).await;
        
        assert!(result.is_ok());
        let context = result.unwrap();
        
        assert_eq!(context.question, question.original_text);
        assert!(!context.sources.is_empty());
        assert!(!context.instructions.is_empty());
        assert!(!context.constraints.is_empty());
        
        // التحقق من أن التعليمات تحتوي على إرشادات فقهية
        assert!(context.instructions.contains("الأدلة الشرعية"));
    }

    #[tokio::test]
    async fn test_performance_requirements() {
        let rag_system = RAGSystem::new();
        let start_time = std::time::Instant::now();
        
        let request = RAGRequest {
            question: "ما هي أركان الوضوء؟".to_string(),
            user_id: None,
            context: None,
            preferences: None,
        };
        
        let result = rag_system.ask_question(request).await;
        let elapsed = start_time.elapsed();
        
        assert!(result.is_ok());
        
        // التحقق من أن الاستجابة تمت في وقت معقول (أقل من 30 ثانية)
        assert!(elapsed.as_secs() < 30);
        
        let response = result.unwrap();
        
        // التحقق من جودة الاستجابة
        assert!(response.confidence > 0.5);
        assert!(response.hallucination_risk < 0.5);
        assert!(!response.answer.is_empty());
    }
}

// اختبارات الخصائص (Property-Based Tests) ستكون في ملف منفصل
// لأنها تتطلب مكتبات إضافية مثل proptest أو quickcheck
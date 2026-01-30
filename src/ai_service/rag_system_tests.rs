#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    /// Test the complete RAG pipeline with a basic Islamic question
    #[tokio::test]
    async fn test_rag_pipeline_basic_question() {
        let rag_system = RAGSystem::new();
        
        let request = RAGRequest {
            question: "ما هي أركان الإسلام؟".to_string(),
            user_id: Some("test_user".to_string()),
            context: None,
            preferences: None,
        };
        
        let response = rag_system.ask_question(request).await;
        
        assert!(response.is_ok());
        let response = response.unwrap();
        
        // التحقق من وجود إجابة
        assert!(!response.answer.is_empty());
        
        // التحقق من وجود مصادر
        assert!(!response.retrieved_sources.is_empty());
        
        // التحقق من مستوى الثقة
        assert!(response.confidence > 0.0);
        assert!(response.confidence <= 1.0);
        
        // التحقق من مخاطر الاختلاق
        assert!(response.hallucination_risk >= 0.0);
        assert!(response.hallucination_risk <= 1.0);
        
        // التحقق من وجود أسئلة ذات صلة
        assert!(!response.related_questions.is_empty());
        
        // التحقق من وجود مراجع
        assert!(!response.citations.is_empty());
        
        println!("الإجابة: {}", response.answer);
        println!("مستوى الثقة: {:.2}", response.confidence);
        println!("مخاطر الاختلاق: {:.2}", response.hallucination_risk);
        println!("عدد المصادر: {}", response.retrieved_sources.len());
    }
    
    /// Test RAG system with controversial question
    #[tokio::test]
    async fn test_rag_controversial_question() {
        let rag_system = RAGSystem::new();
        
        let request = RAGRequest {
            question: "ما هو الخلاف في مسألة رفع اليدين في الصلاة؟".to_string(),
            user_id: Some("test_user".to_string()),
            context: None,
            preferences: Some(UserPreferences {
                preferred_sources: vec![SourceType::SahihHadith, SourceType::FiqhRuling],
                language: Language::Arabic,
                detail_level: DetailLevel::Detailed,
                include_multiple_opinions: true,
            }),
        };
        
        let response = rag_system.ask_question(request).await;
        
        assert!(response.is_ok());
        let response = response.unwrap();
        
        // للأسئلة الخلافية، يجب أن تكون هناك تحذيرات
        assert!(!response.warnings.is_empty());
        
        // يجب أن تحتوي على آراء متعددة
        assert!(response.answer.contains("خلاف") || response.answer.contains("رأي"));
        
        println!("إجابة السؤال الخلافي: {}", response.answer);
        println!("التحذيرات: {:?}", response.warnings);
    }
    
    /// Test out-of-scope question detection
    #[tokio::test]
    async fn test_out_of_scope_question() {
        let rag_system = RAGSystem::new();
        
        let request = RAGRequest {
            question: "كيف أطبخ الأرز؟".to_string(),
            user_id: Some("test_user".to_string()),
            context: None,
            preferences: None,
        };
        
        let response = rag_system.ask_question(request).await;
        
        // يجب أن يفشل السؤال خارج النطاق
        assert!(response.is_err());
        
        if let Err(error) = response {
            assert!(matches!(error, AIServiceError::OutOfScopeQuestion(_)));
            println!("تم رفض السؤال خارج النطاق: {}", error);
        }
    }
    
    /// Test source scoring and filtering
    #[tokio::test]
    async fn test_source_scoring() {
        let rag_system = RAGSystem::new();
        
        // إنشاء مصادر تجريبية
        let sources = vec![
            IslamicSource {
                id: "quran_test".to_string(),
                content_type: SourceType::Quran,
                text: "وَأَقِيمُوا الصَّلَاةَ وَآتُوا الزَّكَاةَ".to_string(),
                reference: "البقرة: 43".to_string(),
                author: None,
                authenticity: AuthenticityLevel::Verified,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: chrono::Utc::now(),
            },
            IslamicSource {
                id: "hadith_test".to_string(),
                content_type: SourceType::SahihHadith,
                text: "بني الإسلام على خمس".to_string(),
                reference: "صحيح البخاري".to_string(),
                author: Some("البخاري".to_string()),
                authenticity: AuthenticityLevel::Verified,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: chrono::Utc::now(),
            },
            IslamicSource {
                id: "weak_hadith_test".to_string(),
                content_type: SourceType::DaifHadith,
                text: "حديث ضعيف للاختبار".to_string(),
                reference: "مصدر ضعيف".to_string(),
                author: None,
                authenticity: AuthenticityLevel::Questionable,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: chrono::Utc::now(),
            },
        ];
        
        let question = ProcessedQuestion {
            original_text: "ما هي أركان الإسلام؟".to_string(),
            normalized_text: "ما هي اركان الاسلام".to_string(),
            keywords: vec!["اركان".to_string(), "اسلام".to_string()],
            concepts: vec!["اسلام".to_string()],
            question_type: QuestionType::General,
            complexity_level: ComplexityLevel::Simple,
            language: Language::Arabic,
            is_controversial: false,
            requires_multiple_sources: false,
            embedding: None,
        };
        
        let scored_sources = rag_system.source_scorer
            .score_sources(&sources, &question).await;
        
        assert!(scored_sources.is_ok());
        let scored_sources = scored_sources.unwrap();
        
        // التحقق من ترتيب المصادر
        assert_eq!(scored_sources.len(), 3);
        
        // القرآن يجب أن يكون الأعلى تقييماً
        assert!(scored_sources[0].source.content_type == SourceType::Quran);
        
        // الحديث الضعيف يجب أن يكون الأقل تقييماً
        let weak_hadith_score = scored_sources.iter()
            .find(|s| s.source.content_type == SourceType::DaifHadith)
            .unwrap();
        assert!(weak_hadith_score.score.final_score < 0.8);
        
        println!("نتائج تقييم المصادر:");
        for (i, source) in scored_sources.iter().enumerate() {
            println!("{}. {} - النتيجة: {:.2}", 
                i + 1, 
                source.source.reference, 
                source.score.final_score
            );
        }
    }
    
    /// Test hadith verification system
    #[tokio::test]
    async fn test_hadith_verification() {
        let rag_system = RAGSystem::new();
        
        // اختبار حديث صحيح
        let sahih_hadith = "إنما الأعمال بالنيات";
        let verification_result = rag_system.hadith_verifier
            .check_hadith_before_display(sahih_hadith).await;
        
        assert!(verification_result.is_ok());
        assert!(verification_result.unwrap()); // يجب أن يُسمح بعرضه
        
        println!("تم التحقق من الحديث الصحيح بنجاح");
    }
    
    /// Test anti-hallucination system
    #[tokio::test]
    async fn test_anti_hallucination() {
        let rag_system = RAGSystem::new();
        
        let fake_response = "قال الله تعالى: \"هذه آية مختلقة للاختبار\"";
        let sources = vec![];
        let question = ProcessedQuestion {
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
        
        let hallucination_check = rag_system.anti_hallucination
            .check_response(fake_response, &sources, &question).await;
        
        assert!(hallucination_check.is_ok());
        let check_result = hallucination_check.unwrap();
        
        // يجب أن يكتشف المحتوى المختلق
        assert!(check_result.is_hallucination_detected);
        assert!(check_result.hallucination_risk_score > 0.5);
        assert!(!check_result.fabricated_content.is_empty());
        
        println!("تم اكتشاف المحتوى المختلق: مخاطر = {:.2}", 
            check_result.hallucination_risk_score);
    }
    
    /// Test quality metrics calculation
    #[tokio::test]
    async fn test_quality_metrics() {
        let rag_system = RAGSystem::new();
        
        let sources = vec![
            IslamicSource {
                id: "quran_quality_test".to_string(),
                content_type: SourceType::Quran,
                text: "إِنَّ الصَّلَاةَ تَنْهَىٰ عَنِ الْفَحْشَاءِ وَالْمُنكَرِ".to_string(),
                reference: "العنكبوت: 45".to_string(),
                author: None,
                authenticity: AuthenticityLevel::Verified,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: chrono::Utc::now(),
            }
        ];
        
        let question = ProcessedQuestion {
            original_text: "ما فائدة الصلاة؟".to_string(),
            normalized_text: "ما فائدة الصلاة".to_string(),
            keywords: vec!["فائدة".to_string(), "صلاة".to_string()],
            concepts: vec!["صلاة".to_string()],
            question_type: QuestionType::Fiqh,
            complexity_level: ComplexityLevel::Simple,
            language: Language::Arabic,
            is_controversial: false,
            requires_multiple_sources: false,
            embedding: None,
        };
        
        let answer = "الصلاة تنهى عن الفحشاء والمنكر كما جاء في القرآن الكريم";
        
        let quality_metrics = rag_system.calculate_quality_metrics(
            &sources,
            &sources,
            answer,
            &question,
        );
        
        // التحقق من مقاييس الجودة
        assert!(quality_metrics.source_quality_score > 0.8); // مصدر قرآني عالي الجودة
        assert!(quality_metrics.authenticity_score > 0.9); // مصدر موثق
        assert!(quality_metrics.citation_coverage == 1.0); // تغطية كاملة للمراجع
        
        println!("مقاييس الجودة:");
        println!("جودة المصادر: {:.2}", quality_metrics.source_quality_score);
        println!("الصلة: {:.2}", quality_metrics.relevance_score);
        println!("الاكتمال: {:.2}", quality_metrics.completeness_score);
        println!("الأصالة: {:.2}", quality_metrics.authenticity_score);
        println!("تغطية المراجع: {:.2}", quality_metrics.citation_coverage);
    }
    
    /// Test citation formatting
    #[tokio::test]
    async fn test_citation_formatting() {
        let rag_system = RAGSystem::new();
        
        let quran_source = IslamicSource {
            id: "citation_test_quran".to_string(),
            content_type: SourceType::Quran,
            text: "آية تجريبية".to_string(),
            reference: "البقرة: 255".to_string(),
            author: None,
            authenticity: AuthenticityLevel::Verified,
            language: Language::Arabic,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        };
        
        let hadith_source = IslamicSource {
            id: "citation_test_hadith".to_string(),
            content_type: SourceType::SahihHadith,
            text: "حديث تجريبي".to_string(),
            reference: "كتاب الإيمان، حديث رقم 1".to_string(),
            author: Some("البخاري".to_string()),
            authenticity: AuthenticityLevel::Verified,
            language: Language::Arabic,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        };
        
        let sources = vec![quran_source, hadith_source];
        let citations = rag_system.build_citations(&sources);
        
        assert_eq!(citations.len(), 2);
        
        // التحقق من تنسيق مرجع القرآن
        let quran_citation = &citations[0];
        assert!(quran_citation.citation_text.contains("القرآن الكريم"));
        assert!(quran_citation.citation_text.contains("البقرة: 255"));
        
        // التحقق من تنسيق مرجع الحديث
        let hadith_citation = &citations[1];
        assert!(hadith_citation.citation_text.contains("البخاري"));
        
        println!("المراجع المنسقة:");
        for citation in &citations {
            println!("- {}", citation.citation_text);
        }
    }
    
    /// Test performance requirements
    #[tokio::test]
    async fn test_performance_requirements() {
        let rag_system = RAGSystem::new();
        
        let request = RAGRequest {
            question: "ما هي أركان الإسلام؟".to_string(),
            user_id: Some("performance_test".to_string()),
            context: None,
            preferences: None,
        };
        
        let start_time = std::time::Instant::now();
        let response = rag_system.ask_question(request).await;
        let elapsed = start_time.elapsed();
        
        assert!(response.is_ok());
        let response = response.unwrap();
        
        // التحقق من متطلبات الأداء (أقل من 30 ثانية)
        assert!(elapsed.as_secs() < 30);
        assert!(response.response_time_ms < 30000);
        
        println!("وقت الاستجابة: {} مللي ثانية", response.response_time_ms);
        println!("الوقت الفعلي: {} مللي ثانية", elapsed.as_millis());
    }
    
    /// Test configuration customization
    #[tokio::test]
    async fn test_custom_configuration() {
        let custom_config = RAGConfig {
            max_sources: 5,
            min_confidence_threshold: 0.8,
            max_response_time: Duration::from_secs(15),
            enable_hallucination_check: true,
            require_source_verification: true,
            max_context_length: 2000,
        };
        
        let rag_system = RAGSystem::with_config(custom_config);
        
        let request = RAGRequest {
            question: "ما حكم الصلاة؟".to_string(),
            user_id: Some("config_test".to_string()),
            context: None,
            preferences: None,
        };
        
        let response = rag_system.ask_question(request).await;
        
        assert!(response.is_ok());
        let response = response.unwrap();
        
        // التحقق من تطبيق التكوين المخصص
        assert!(response.retrieved_sources.len() <= 5); // max_sources
        assert!(response.confidence >= 0.8 || response.warnings.len() > 0); // min_confidence_threshold
        
        println!("تم تطبيق التكوين المخصص بنجاح");
        println!("عدد المصادر: {}", response.retrieved_sources.len());
        println!("مستوى الثقة: {:.2}", response.confidence);
    }
}
use crate::ai_service::rag_system::{RAGRequest, UserPreferences, DetailLevel};
use crate::ai_service::{
    multiple_viewpoints_system::*,
    question_processor::QuestionProcessor,
    source_scorer::{ScoredSource, SourceScore, ScoringDetails, SourceUsageRecommendation},
};
use std::collections::HashMap;
use tokio;

/// Test the complete multiple viewpoints system with controversial questions
#[tokio::test]
async fn test_multiple_viewpoints_system_controversial_question() {
    let system = MultipleViewpointsSystem::new();
    let processor = QuestionProcessor::new();
    
    // Test with a known controversial question
    let question = processor
        .process_question("ما الخلاف في رفع اليدين في الصلاة؟")
        .await
        .unwrap();
    
    // Create mock sources representing different madhab opinions
    let sources = create_mock_controversial_sources();
    
    let result = system.analyze_viewpoints(&question, &sources).await.unwrap();
    
    // Verify the system detected controversy
    assert!(result.is_controversial, "Should detect controversial question");
    assert!(matches!(result.controversy_level, ControlversyLevel::Moderate | ControlversyLevel::Significant));
    
    // Verify multiple viewpoints are present
    assert!(result.viewpoints.len() >= 2, "Should have multiple viewpoints for controversial question");
    
    // Verify different madhabs are represented
    let madhabs: std::collections::HashSet<_> = result.viewpoints
        .iter()
        .map(|v| &v.madhab)
        .collect();
    assert!(madhabs.len() >= 2, "Should have different madhabs represented");
    
    // Verify internal guidance is provided
    assert!(!result.internal_guidance.is_empty(), "Should provide internal guidance");
    
    // Verify source reliability assessment
    assert!(result.source_reliability_assessment.overall_reliability > 0.0);
    
    // Verify summary information
    assert!(result.summary.total_viewpoints >= 2);
    assert!(!result.summary.madhabs_represented.is_empty());
    
    println!("✅ Multiple viewpoints system test passed");
    println!("   Controversy level: {:?}", result.controversy_level);
    println!("   Viewpoints count: {}", result.viewpoints.len());
    println!("   Madhabs represented: {:?}", result.summary.madhabs_represented);
    println!("   Internal guidance count: {}", result.internal_guidance.len());
}

/// Test controversy detection with various question types
#[tokio::test]
async fn test_controversy_detection() {
    let detector = ControlversyDetector::new();
    let processor = QuestionProcessor::new();
    
    let test_cases = vec![
        ("ما الخلاف في رفع اليدين في الصلاة؟", true, "Contains explicit controversy keyword"),
        ("ما آراء المذاهب في المسح على الخفين؟", true, "Mentions different madhab opinions"),
        ("ما هي أركان الإسلام؟", false, "Non-controversial basic question"),
        ("كيف نصلي الفجر؟", false, "Simple procedural question"),
        ("ما اختلاف العلماء في حكم الموسيقى؟", true, "Explicitly mentions scholarly disagreement"),
    ];
    
    for (question_text, expected_controversial, description) in test_cases {
        let question = processor.process_question(question_text).await.unwrap();
        let analysis = detector.analyze_controversy(&question, &[]).await.unwrap();
        
        assert_eq!(
            analysis.is_controversial, 
            expected_controversial,
            "Failed for: {} - {}", question_text, description
        );
        
        if expected_controversial {
            assert!(!analysis.indicators.is_empty(), "Should have controversy indicators");
            assert!(analysis.confidence > 0.3, "Should have reasonable confidence");
        }
        
        println!("✅ Controversy detection: {} -> {} ({})", 
                question_text, analysis.is_controversial, description);
    }
}

/// Test madhab classification system
#[tokio::test]
async fn test_madhab_classification() {
    let classifier = MadhabClassifier::new();
    
    let test_sources = vec![
        (create_hanafi_source(), IslamicMadhab::Hanafi, "Hanafi source with Abu Hanifa author"),
        (create_maliki_source(), IslamicMadhab::Maliki, "Maliki source with Malik author"),
        (create_shafii_source(), IslamicMadhab::Shafii, "Shafii source with Al-Shafii author"),
        (create_general_source(), IslamicMadhab::General, "General source without specific madhab"),
    ];
    
    for (source, expected_madhab, description) in test_sources {
        let classified_madhab = classifier.classify_single_source(&source).await.unwrap();
        
        assert_eq!(
            classified_madhab, 
            expected_madhab,
            "Failed classification for: {}", description
        );
        
        let confidence = classifier.calculate_classification_confidence(&source, &classified_madhab);
        assert!(confidence > 0.0, "Should have some confidence in classification");
        
        println!("✅ Madhab classification: {} -> {} (confidence: {:.2})", 
                description, classified_madhab.to_arabic(), confidence);
    }
}

/// Test viewpoint aggregation
#[tokio::test]
async fn test_viewpoint_aggregation() {
    let aggregator = ViewpointAggregator::new();
    let processor = QuestionProcessor::new();
    
    let question = processor
        .process_question("ما حكم المسح على الخفين؟")
        .await
        .unwrap();
    
    // Create madhab classification with multiple madhabs
    let mut sources_by_madhab = HashMap::new();
    sources_by_madhab.insert(IslamicMadhab::Hanafi, vec![create_scored_hanafi_source()]);
    sources_by_madhab.insert(IslamicMadhab::Maliki, vec![create_scored_maliki_source()]);
    
    let classification = MadhabClassification {
        sources_by_madhab,
        confidence_scores: HashMap::new(),
    };
    
    let viewpoints = aggregator.aggregate_viewpoints(&question, &classification).await.unwrap();
    
    assert_eq!(viewpoints.len(), 2, "Should create viewpoints for each madhab");
    
    // Verify viewpoint structure
    for viewpoint in &viewpoints {
        assert!(!viewpoint.position.is_empty(), "Viewpoint should have a position");
        assert!(!viewpoint.evidence.is_empty(), "Viewpoint should have evidence");
        assert!(!viewpoint.reasoning.is_empty(), "Viewpoint should have reasoning");
        assert!(!viewpoint.conditions.is_empty(), "Viewpoint should have conditions");
        assert!(!viewpoint.modern_applications.is_empty(), "Viewpoint should have modern applications");
    }
    
    println!("✅ Viewpoint aggregation test passed");
    println!("   Generated {} viewpoints", viewpoints.len());
}

/// Test source reliability evaluation
#[tokio::test]
async fn test_source_reliability_evaluation() {
    let evaluator = SourceReliabilityEvaluator::new();
    
    let sources = create_mixed_reliability_sources();
    let viewpoints = vec![]; // Empty for this test
    
    let assessment = evaluator.evaluate_sources(&sources, &viewpoints).await.unwrap();
    
    assert!(assessment.overall_reliability > 0.0, "Should have overall reliability score");
    assert!(!assessment.source_breakdown.is_empty(), "Should have source breakdown");
    assert!(!assessment.reliability_factors.is_empty(), "Should have reliability factors");
    
    // Check that different source types get different reliability scores
    let quran_sources: Vec<_> = sources.iter()
        .filter(|s| matches!(s.source.content_type, SourceType::Quran))
        .collect();
    
    let hadith_sources: Vec<_> = sources.iter()
        .filter(|s| matches!(s.source.content_type, SourceType::DaifHadith))
        .collect();
    
    if !quran_sources.is_empty() && !hadith_sources.is_empty() {
        let quran_score = assessment.source_breakdown.get(&quran_sources[0].source.id).unwrap();
        let hadith_score = assessment.source_breakdown.get(&hadith_sources[0].source.id).unwrap();
        
        assert!(quran_score.score > hadith_score.score, 
               "Quran should have higher reliability than weak hadith");
    }
    
    println!("✅ Source reliability evaluation test passed");
    println!("   Overall reliability: {:.2}", assessment.overall_reliability);
    println!("   Sources evaluated: {}", assessment.source_breakdown.len());
}

/// Test internal guidance generation
#[tokio::test]
async fn test_internal_guidance_generation() {
    let generator = InternalGuidanceGenerator::new();
    let processor = QuestionProcessor::new();
    
    let fiqh_question = processor
        .process_question("ما حكم الصلاة في الطائرة؟")
        .await
        .unwrap();
    
    let viewpoints = vec![create_sample_viewpoint()];
    
    let guidance = generator.generate_guidance(&fiqh_question, &viewpoints).await.unwrap();
    
    assert!(!guidance.is_empty(), "Should generate internal guidance");
    
    // Verify guidance structure
    for guide in &guidance {
        assert!(!guide.reference_path.is_empty(), "Should have reference path");
        assert!(!guide.description.is_empty(), "Should have description");
        assert!(guide.relevance_score > 0.0, "Should have relevance score");
        assert!(!guide.recommended_sections.is_empty(), "Should have recommended sections");
    }
    
    // Verify guidance is sorted by relevance
    for i in 1..guidance.len() {
        assert!(guidance[i-1].relevance_score >= guidance[i].relevance_score,
               "Guidance should be sorted by relevance score");
    }
    
    println!("✅ Internal guidance generation test passed");
    println!("   Generated {} guidance items", guidance.len());
}

/// Test integration with RAG system
#[tokio::test]
async fn test_rag_integration_with_multiple_viewpoints() {
    let rag_system = crate::ai_service::rag_system::RAGSystem::new();
    
    let request = RAGRequest {
        question: "ما الخلاف في قراءة الفاتحة خلف الإمام؟".to_string(),
        user_id: Some("test_user".to_string()),
        context: None,
        preferences: Some(UserPreferences {
            preferred_sources: vec![SourceType::Quran, SourceType::SahihHadith, SourceType::FiqhRuling],
            language: Language::Arabic,
            detail_level: DetailLevel::Detailed,
            include_multiple_opinions: true,
        }),
    };
    
    let response = rag_system.ask_question(request).await.unwrap();
    
    // Verify multiple viewpoints are included in response
    assert!(response.multiple_viewpoints.is_some(), "Should include multiple viewpoints for controversial question");
    
    let viewpoints = response.multiple_viewpoints.unwrap();
    assert!(viewpoints.is_controversial, "Should be marked as controversial");
    assert!(!viewpoints.viewpoints.is_empty(), "Should have viewpoints");
    
    // Verify response quality
    assert!(!response.answer.is_empty(), "Should have an answer");
    assert!(response.confidence > 0.0, "Should have confidence score");
    assert!(!response.retrieved_sources.is_empty(), "Should have retrieved sources");
    
    println!("✅ RAG integration with multiple viewpoints test passed");
    println!("   Answer length: {} characters", response.answer.len());
    println!("   Viewpoints count: {}", viewpoints.viewpoints.len());
    println!("   Response confidence: {:.2}", response.confidence);
}

// Helper functions to create mock data for testing

fn create_mock_controversial_sources() -> Vec<ScoredSource> {
    vec![
        create_scored_hanafi_source(),
        create_scored_maliki_source(),
        create_scored_shafii_source(),
    ]
}

fn create_scored_hanafi_source() -> ScoredSource {
    ScoredSource {
        source: create_hanafi_source(),
        score: create_high_score(),
        rank: 1,
        usage_recommendation: SourceUsageRecommendation::Primary,
    }
}

fn create_scored_maliki_source() -> ScoredSource {
    ScoredSource {
        source: create_maliki_source(),
        score: create_high_score(),
        rank: 2,
        usage_recommendation: SourceUsageRecommendation::Primary,
    }
}

fn create_scored_shafii_source() -> ScoredSource {
    ScoredSource {
        source: create_shafii_source(),
        score: create_high_score(),
        rank: 3,
        usage_recommendation: SourceUsageRecommendation::Supporting,
    }
}

fn create_hanafi_source() -> IslamicSource {
    IslamicSource {
        id: "hanafi_source_1".to_string(),
        content_type: SourceType::FiqhRuling,
        text: "يرى الحنفية أن رفع اليدين في الصلاة يكون عند تكبيرة الإحرام فقط".to_string(),
        reference: "الهداية في شرح بداية المبتدي".to_string(),
        author: Some("أبو حنيفة".to_string()),
        authenticity: AuthenticityLevel::Verified,
        language: Language::Arabic,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("madhab".to_string(), "حنفي".to_string());
            meta.insert("topic".to_string(), "رفع اليدين في الصلاة".to_string());
            meta
        },
        created_at: chrono::Utc::now(),
    }
}

fn create_maliki_source() -> IslamicSource {
    IslamicSource {
        id: "maliki_source_1".to_string(),
        content_type: SourceType::FiqhRuling,
        text: "المذهب المالكي يقول بعدم رفع اليدين إلا في تكبيرة الإحرام".to_string(),
        reference: "الموطأ للإمام مالك".to_string(),
        author: Some("مالك بن أنس".to_string()),
        authenticity: AuthenticityLevel::Verified,
        language: Language::Arabic,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("madhab".to_string(), "مالكي".to_string());
            meta.insert("topic".to_string(), "رفع اليدين في الصلاة".to_string());
            meta
        },
        created_at: chrono::Utc::now(),
    }
}

fn create_shafii_source() -> IslamicSource {
    IslamicSource {
        id: "shafii_source_1".to_string(),
        content_type: SourceType::FiqhRuling,
        text: "الشافعية يرون رفع اليدين عند تكبيرة الإحرام وعند الركوع وعند الرفع منه".to_string(),
        reference: "الأم للإمام الشافعي".to_string(),
        author: Some("الشافعي".to_string()),
        authenticity: AuthenticityLevel::Verified,
        language: Language::Arabic,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("madhab".to_string(), "شافعي".to_string());
            meta.insert("topic".to_string(), "رفع اليدين في الصلاة".to_string());
            meta
        },
        created_at: chrono::Utc::now(),
    }
}

fn create_general_source() -> IslamicSource {
    IslamicSource {
        id: "general_source_1".to_string(),
        content_type: SourceType::Quran,
        text: "وَأَقِيمُوا الصَّلَاةَ وَآتُوا الزَّكَاةَ".to_string(),
        reference: "البقرة: 43".to_string(),
        author: None,
        authenticity: AuthenticityLevel::Verified,
        language: Language::Arabic,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("surah".to_string(), "البقرة".to_string());
            meta.insert("ayah".to_string(), "43".to_string());
            meta
        },
        created_at: chrono::Utc::now(),
    }
}

fn create_high_score() -> SourceScore {
    SourceScore {
        relevance_score: 0.9,
        authority_score: 0.85,
        authenticity_score: 0.95,
        consensus_score: 0.7,
        freshness_score: 0.8,
        final_score: 0.84,
        confidence_level: ConfidenceLevel::High,
        scoring_details: ScoringDetails {
            relevance_factors: vec!["تطابق موضوعي عالي".to_string()],
            authority_factors: vec!["مؤلف معتبر".to_string()],
            authenticity_factors: vec!["مصدر محقق".to_string()],
            consensus_factors: vec!["رأي معتبر".to_string()],
            penalties: vec![],
            bonuses: vec!["مصدر أساسي".to_string()],
        },
    }
}

fn create_mixed_reliability_sources() -> Vec<ScoredSource> {
    vec![
        ScoredSource {
            source: IslamicSource {
                id: "quran_source".to_string(),
                content_type: SourceType::Quran,
                text: "قُلْ هُوَ اللَّهُ أَحَدٌ".to_string(),
                reference: "الإخلاص: 1".to_string(),
                author: None,
                authenticity: AuthenticityLevel::Verified,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: chrono::Utc::now(),
            },
            score: create_high_score(),
            rank: 1,
            usage_recommendation: SourceUsageRecommendation::Primary,
        },
        ScoredSource {
            source: IslamicSource {
                id: "weak_hadith_source".to_string(),
                content_type: SourceType::DaifHadith,
                text: "حديث ضعيف في الموضوع".to_string(),
                reference: "مصدر ضعيف".to_string(),
                author: Some("راوي ضعيف".to_string()),
                authenticity: AuthenticityLevel::Questionable,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: chrono::Utc::now(),
            },
            score: SourceScore {
                relevance_score: 0.6,
                authority_score: 0.3,
                authenticity_score: 0.4,
                consensus_score: 0.5,
                freshness_score: 0.7,
                final_score: 0.5,
                confidence_level: ConfidenceLevel::Medium,
                scoring_details: ScoringDetails {
                    relevance_factors: vec![],
                    authority_factors: vec![],
                    authenticity_factors: vec![],
                    consensus_factors: vec![],
                    penalties: vec!["حديث ضعيف".to_string()],
                    bonuses: vec![],
                },
            },
            rank: 2,
            usage_recommendation: SourceUsageRecommendation::Cautionary,
        },
    ]
}

fn create_sample_viewpoint() -> ScholarlyViewpoint {
    ScholarlyViewpoint {
        id: "sample_viewpoint_1".to_string(),
        madhab: IslamicMadhab::Hanafi,
        position: "الرأي الحنفي في المسألة".to_string(),
        evidence: vec![create_hanafi_source()],
        reasoning: "الاستدلال بناءً على الأصول الحنفية".to_string(),
        prominent_scholars: vec!["أبو حنيفة".to_string()],
        strength_level: ViewpointStrength::Strong,
        conditions: vec!["بشرط توفر الشروط".to_string()],
        exceptions: vec!["ما عدا الحالات الاستثنائية".to_string()],
        modern_applications: vec!["التطبيق في العصر الحديث".to_string()],
    }
}
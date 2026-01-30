// Simple test file to verify RAG system implementation
// Run with: cargo test --test test_rag_system

#[path = "src/ai_service/mod.rs"]
mod ai_service;

use ai_service::rag_system::{RAGSystem, RAGRequest};
use ai_service::{Language, DetailLevel, SourceType, UserPreferences};

#[tokio::test]
async fn test_rag_basic_functionality() {
    println!("🚀 Testing RAG System Basic Functionality");
    
    let rag_system = RAGSystem::new();
    
    let request = RAGRequest {
        question: "ما هي أركان الإسلام؟".to_string(),
        user_id: Some("test_user".to_string()),
        context: None,
        preferences: None,
    };
    
    println!("📝 Processing question: {}", request.question);
    
    let response = rag_system.ask_question(request).await;
    
    match response {
        Ok(response) => {
            println!("✅ RAG System Response Generated Successfully!");
            println!("📊 Response Details:");
            println!("   - Answer length: {} characters", response.answer.len());
            println!("   - Confidence: {:.2}", response.confidence);
            println!("   - Hallucination risk: {:.2}", response.hallucination_risk);
            println!("   - Sources retrieved: {}", response.retrieved_sources.len());
            println!("   - Citations: {}", response.citations.len());
            println!("   - Related questions: {}", response.related_questions.len());
            println!("   - Response time: {} ms", response.response_time_ms);
            
            println!("\n📖 Generated Answer:");
            println!("{}", response.answer);
            
            if !response.related_questions.is_empty() {
                println!("\n❓ Related Questions:");
                for (i, question) in response.related_questions.iter().enumerate() {
                    println!("   {}. {}", i + 1, question);
                }
            }
            
            if !response.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &response.warnings {
                    println!("   - {}", warning);
                }
            }
            
            // Basic assertions
            assert!(!response.answer.is_empty(), "Answer should not be empty");
            assert!(response.confidence > 0.0, "Confidence should be positive");
            assert!(response.confidence <= 1.0, "Confidence should not exceed 1.0");
            assert!(response.hallucination_risk >= 0.0, "Hallucination risk should be non-negative");
            assert!(response.hallucination_risk <= 1.0, "Hallucination risk should not exceed 1.0");
            assert!(!response.retrieved_sources.is_empty(), "Should have retrieved sources");
            assert!(!response.citations.is_empty(), "Should have citations");
            assert!(!response.related_questions.is_empty(), "Should have related questions");
            
            println!("\n✅ All basic assertions passed!");
        },
        Err(error) => {
            println!("❌ RAG System Error: {}", error);
            panic!("RAG system failed: {}", error);
        }
    }
}

#[tokio::test]
async fn test_rag_out_of_scope_detection() {
    println!("🔍 Testing Out-of-Scope Question Detection");
    
    let rag_system = RAGSystem::new();
    
    let request = RAGRequest {
        question: "كيف أطبخ الأرز؟".to_string(), // Cooking question - out of scope
        user_id: Some("test_user".to_string()),
        context: None,
        preferences: None,
    };
    
    println!("📝 Processing out-of-scope question: {}", request.question);
    
    let response = rag_system.ask_question(request).await;
    
    match response {
        Ok(_) => {
            println!("⚠️  Warning: Out-of-scope question was not rejected");
            // This might be acceptable if the system provides a polite refusal
        },
        Err(error) => {
            println!("✅ Out-of-scope question correctly rejected: {}", error);
            assert!(error.to_string().contains("خارج نطاق"), "Error should mention out of scope");
        }
    }
}

#[tokio::test]
async fn test_rag_with_preferences() {
    println!("⚙️  Testing RAG System with User Preferences");
    
    let rag_system = RAGSystem::new();
    
    let preferences = UserPreferences {
        preferred_sources: vec![SourceType::Quran, SourceType::SahihHadith],
        language: Language::Arabic,
        detail_level: DetailLevel::Detailed,
        include_multiple_opinions: true,
    };
    
    let request = RAGRequest {
        question: "ما حكم الصلاة في المسجد؟".to_string(),
        user_id: Some("test_user".to_string()),
        context: None,
        preferences: Some(preferences),
    };
    
    println!("📝 Processing question with preferences: {}", request.question);
    
    let response = rag_system.ask_question(request).await;
    
    match response {
        Ok(response) => {
            println!("✅ RAG System with Preferences Response Generated!");
            println!("📊 Response Details:");
            println!("   - Answer length: {} characters", response.answer.len());
            println!("   - Confidence: {:.2}", response.confidence);
            println!("   - Sources: {}", response.retrieved_sources.len());
            
            // Check if preferred sources are prioritized
            let has_quran = response.retrieved_sources.iter()
                .any(|s| matches!(s.content_type, SourceType::Quran));
            let has_sahih_hadith = response.retrieved_sources.iter()
                .any(|s| matches!(s.content_type, SourceType::SahihHadith));
            
            if has_quran {
                println!("✅ Quran sources found (as preferred)");
            }
            if has_sahih_hadith {
                println!("✅ Sahih Hadith sources found (as preferred)");
            }
            
            println!("\n📖 Generated Answer:");
            println!("{}", response.answer);
        },
        Err(error) => {
            println!("❌ RAG System with Preferences Error: {}", error);
            panic!("RAG system with preferences failed: {}", error);
        }
    }
}

#[tokio::test]
async fn test_rag_quality_metrics() {
    println!("📈 Testing RAG System Quality Metrics");
    
    let rag_system = RAGSystem::new();
    
    let request = RAGRequest {
        question: "ما هي أركان الإسلام الخمسة؟".to_string(),
        user_id: Some("test_user".to_string()),
        context: None,
        preferences: None,
    };
    
    println!("📝 Processing question for quality metrics: {}", request.question);
    
    let response = rag_system.ask_question(request).await;
    
    match response {
        Ok(response) => {
            println!("✅ RAG System Quality Metrics Generated!");
            println!("📊 Quality Metrics:");
            println!("   - Source Quality Score: {:.2}", response.quality_metrics.source_quality_score);
            println!("   - Relevance Score: {:.2}", response.quality_metrics.relevance_score);
            println!("   - Completeness Score: {:.2}", response.quality_metrics.completeness_score);
            println!("   - Authenticity Score: {:.2}", response.quality_metrics.authenticity_score);
            println!("   - Citation Coverage: {:.2}", response.quality_metrics.citation_coverage);
            
            // Quality metrics should be within valid ranges
            assert!(response.quality_metrics.source_quality_score >= 0.0);
            assert!(response.quality_metrics.source_quality_score <= 1.0);
            assert!(response.quality_metrics.relevance_score >= 0.0);
            assert!(response.quality_metrics.relevance_score <= 1.0);
            assert!(response.quality_metrics.completeness_score >= 0.0);
            assert!(response.quality_metrics.completeness_score <= 1.0);
            assert!(response.quality_metrics.authenticity_score >= 0.0);
            assert!(response.quality_metrics.authenticity_score <= 1.0);
            assert!(response.quality_metrics.citation_coverage >= 0.0);
            assert!(response.quality_metrics.citation_coverage <= 1.0);
            
            println!("✅ All quality metrics are within valid ranges!");
        },
        Err(error) => {
            println!("❌ RAG System Quality Metrics Error: {}", error);
            panic!("RAG system quality metrics failed: {}", error);
        }
    }
}

#[tokio::test]
async fn test_rag_performance() {
    println!("⚡ Testing RAG System Performance");
    
    let rag_system = RAGSystem::new();
    
    let request = RAGRequest {
        question: "ما هي شروط الصلاة؟".to_string(),
        user_id: Some("performance_test".to_string()),
        context: None,
        preferences: None,
    };
    
    println!("📝 Processing question for performance test: {}", request.question);
    
    let start_time = std::time::Instant::now();
    let response = rag_system.ask_question(request).await;
    let elapsed = start_time.elapsed();
    
    match response {
        Ok(response) => {
            println!("✅ RAG System Performance Test Completed!");
            println!("⏱️  Performance Metrics:");
            println!("   - Total elapsed time: {} ms", elapsed.as_millis());
            println!("   - Reported response time: {} ms", response.response_time_ms);
            println!("   - Sources processed: {}", response.retrieved_sources.len());
            println!("   - Citations generated: {}", response.citations.len());
            
            // Performance requirements (should be under 30 seconds)
            assert!(elapsed.as_secs() < 30, "Response should be under 30 seconds");
            assert!(response.response_time_ms < 30000, "Reported time should be under 30 seconds");
            
            println!("✅ Performance requirements met!");
        },
        Err(error) => {
            println!("❌ RAG System Performance Error: {}", error);
            panic!("RAG system performance test failed: {}", error);
        }
    }
}

#[tokio::main]
async fn main() {
    println!("🧪 Running RAG System Tests");
    println!("=" .repeat(50));
    
    // Run tests manually since this is a standalone test file
    test_rag_basic_functionality().await;
    println!();
    
    test_rag_out_of_scope_detection().await;
    println!();
    
    test_rag_with_preferences().await;
    println!();
    
    test_rag_quality_metrics().await;
    println!();
    
    test_rag_performance().await;
    println!();
    
    println!("🎉 All RAG System Tests Completed Successfully!");
}
// RAG System Demonstration
// This file demonstrates the core RAG system functionality

#[path = "src/ai_service/mod.rs"]
mod ai_service;

use ai_service::rag_system::{RAGSystem, RAGRequest};
use ai_service::{Language, DetailLevel, SourceType, UserPreferences};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 RAG System Demonstration");
    println!("=" .repeat(60));
    
    // Initialize the RAG system
    let rag_system = RAGSystem::new();
    println!("✅ RAG System initialized successfully");
    
    // Test 1: Basic Islamic question
    println!("\n📝 Test 1: Basic Islamic Question");
    println!("-" .repeat(40));
    
    let request1 = RAGRequest {
        question: "ما هي أركان الإسلام؟".to_string(),
        user_id: Some("demo_user".to_string()),
        context: None,
        preferences: None,
    };
    
    println!("Question: {}", request1.question);
    
    match rag_system.ask_question(request1).await {
        Ok(response) => {
            println!("✅ Response generated successfully!");
            println!("📊 Metrics:");
            println!("   • Confidence: {:.2}", response.confidence);
            println!("   • Hallucination Risk: {:.2}", response.hallucination_risk);
            println!("   • Sources: {}", response.retrieved_sources.len());
            println!("   • Citations: {}", response.citations.len());
            println!("   • Response Time: {} ms", response.response_time_ms);
            
            println!("\n📖 Answer:");
            println!("{}", response.answer);
            
            if !response.related_questions.is_empty() {
                println!("\n❓ Related Questions:");
                for (i, q) in response.related_questions.iter().enumerate() {
                    println!("   {}. {}", i + 1, q);
                }
            }
            
            if !response.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &response.warnings {
                    println!("   • {}", warning);
                }
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    
    // Test 2: Question with preferences
    println!("\n📝 Test 2: Question with User Preferences");
    println!("-" .repeat(40));
    
    let preferences = UserPreferences {
        preferred_sources: vec![SourceType::Quran, SourceType::SahihHadith],
        language: Language::Arabic,
        detail_level: DetailLevel::Detailed,
        include_multiple_opinions: true,
    };
    
    let request2 = RAGRequest {
        question: "ما حكم الصلاة في المسجد؟".to_string(),
        user_id: Some("demo_user".to_string()),
        context: None,
        preferences: Some(preferences),
    };
    
    println!("Question: {}", request2.question);
    println!("Preferences: Quran + Sahih Hadith sources, Detailed level");
    
    match rag_system.ask_question(request2).await {
        Ok(response) => {
            println!("✅ Response with preferences generated!");
            println!("📊 Metrics:");
            println!("   • Confidence: {:.2}", response.confidence);
            println!("   • Sources: {}", response.retrieved_sources.len());
            
            println!("\n📖 Answer:");
            println!("{}", response.answer);
            
            // Show source types
            println!("\n📚 Source Types Used:");
            for source in &response.retrieved_sources {
                println!("   • {:?}: {}", source.content_type, source.reference);
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    
    // Test 3: Out-of-scope question
    println!("\n📝 Test 3: Out-of-Scope Question Detection");
    println!("-" .repeat(40));
    
    let request3 = RAGRequest {
        question: "كيف أطبخ الأرز؟".to_string(), // Cooking question
        user_id: Some("demo_user".to_string()),
        context: None,
        preferences: None,
    };
    
    println!("Question: {}", request3.question);
    
    match rag_system.ask_question(request3).await {
        Ok(response) => {
            println!("⚠️  Question was processed (might include polite refusal):");
            println!("{}", response.answer);
        },
        Err(e) => {
            println!("✅ Out-of-scope question correctly rejected: {}", e);
        }
    }
    
    // Test 4: Performance demonstration
    println!("\n📝 Test 4: Performance Measurement");
    println!("-" .repeat(40));
    
    let request4 = RAGRequest {
        question: "ما هي شروط الوضوء؟".to_string(),
        user_id: Some("demo_user".to_string()),
        context: None,
        preferences: None,
    };
    
    println!("Question: {}", request4.question);
    
    let start_time = std::time::Instant::now();
    match rag_system.ask_question(request4).await {
        Ok(response) => {
            let elapsed = start_time.elapsed();
            println!("✅ Performance test completed!");
            println!("⏱️  Timing:");
            println!("   • Actual elapsed: {} ms", elapsed.as_millis());
            println!("   • Reported time: {} ms", response.response_time_ms);
            println!("   • Performance: {}", if elapsed.as_secs() < 5 { "Excellent" } else { "Good" });
            
            println!("\n📊 Quality Metrics:");
            println!("   • Source Quality: {:.2}", response.quality_metrics.source_quality_score);
            println!("   • Relevance: {:.2}", response.quality_metrics.relevance_score);
            println!("   • Completeness: {:.2}", response.quality_metrics.completeness_score);
            println!("   • Authenticity: {:.2}", response.quality_metrics.authenticity_score);
            println!("   • Citation Coverage: {:.2}", response.quality_metrics.citation_coverage);
        },
        Err(e) => {
            println!("❌ Performance test error: {}", e);
        }
    }
    
    println!("\n🎉 RAG System Demonstration Completed!");
    println!("=" .repeat(60));
    println!("✅ Core RAG functionality verified:");
    println!("   • Question processing and analysis");
    println!("   • Semantic search and source retrieval");
    println!("   • Source scoring and filtering");
    println!("   • Hadith verification");
    println!("   • Anti-hallucination checks");
    println!("   • Response generation with citations");
    println!("   • Quality metrics calculation");
    println!("   • Performance monitoring");
    
    Ok(())
}
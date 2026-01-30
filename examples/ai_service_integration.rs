use sanad::ai_service::{
    config::AIServiceConfig,
    service_manager::AIServiceManager,
    integration_service::RAGProcessingRequest,
    error_handler::{ErrorHandler, ErrorHandlerConfig, ErrorContext},
};
use std::time::Duration;
use tokio;
use tracing::{info, error};

/// Example demonstrating the complete AI service integration setup
/// This example shows how to:
/// 1. Load configuration
/// 2. Initialize the service manager
/// 3. Process RAG requests
/// 4. Handle errors and fallbacks
/// 5. Monitor service health
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting AI Service Integration Example");

    // Step 1: Load configuration
    let config = load_configuration().await?;
    info!("Configuration loaded successfully");

    // Step 2: Initialize service manager
    let service_manager = AIServiceManager::new(config)?;
    let initialization_result = service_manager.initialize().await?;
    
    if !initialization_result.success {
        error!("Failed to initialize AI services: {:?}", initialization_result.services_failed);
        return Err("Service initialization failed".into());
    }
    
    info!("AI services initialized successfully: {:?}", initialization_result.services_initialized);

    // Step 3: Demonstrate RAG processing
    demonstrate_rag_processing(&service_manager).await?;

    // Step 4: Demonstrate error handling
    demonstrate_error_handling().await?;

    // Step 5: Monitor service health
    monitor_service_health(&service_manager).await?;

    // Step 6: Demonstrate caching
    demonstrate_caching(&service_manager).await?;

    // Step 7: Demonstrate fallback mechanisms
    demonstrate_fallback_mechanisms(&service_manager).await?;

    // Cleanup
    service_manager.shutdown().await?;
    info!("AI Service Integration Example completed successfully");

    Ok(())
}

/// Load configuration from file or environment
async fn load_configuration() -> Result<AIServiceConfig, Box<dyn std::error::Error>> {
    // Try to load from file first
    match AIServiceConfig::from_file("config/ai_service_config.yaml") {
        Ok(config) => {
            info!("Configuration loaded from file");
            Ok(config)
        }
        Err(_) => {
            info!("Configuration file not found, using environment variables");
            let config = AIServiceConfig::from_env();
            config.validate().map_err(|e| format!("Configuration validation failed: {}", e))?;
            Ok(config)
        }
    }
}

/// Demonstrate RAG processing with various question types
async fn demonstrate_rag_processing(service_manager: &AIServiceManager) -> Result<(), Box<dyn std::error::Error>> {
    info!("Demonstrating RAG processing...");

    let integration_service = service_manager.get_integration_service().await;
    if integration_service.is_none() {
        info!("Integration service not available, skipping RAG demonstration");
        return Ok(());
    }

    let mut service = integration_service.unwrap();

    // Test questions in different categories
    let test_questions = vec![
        ("ما هي أركان الإسلام؟", "Basic Islamic knowledge"),
        ("كيف نتوضأ؟", "Practical Islamic guidance"),
        ("ما تفسير آية الكرسي؟", "Quranic interpretation"),
        ("ما حكم الصلاة في المسجد؟", "Islamic jurisprudence"),
        ("من هو أبو بكر الصديق؟", "Islamic history"),
    ];

    for (question, category) in test_questions {
        info!("Processing question ({}): {}", category, question);
        
        let request = RAGProcessingRequest {
            question: question.to_string(),
            context: Some(format!("Category: {}", category)),
            max_sources: Some(5),
            similarity_threshold: Some(0.7),
            preferred_source_types: Some(vec!["quran".to_string(), "hadith".to_string()]),
            language: Some("Arabic".to_string()),
            user_id: Some("example_user".to_string()),
        };

        match service.process_rag_request(request).await {
            Ok(response) => {
                info!("✅ Question processed successfully");
                info!("   Answer length: {} characters", response.answer.len());
                info!("   Confidence: {:.2}", response.confidence);
                info!("   Sources found: {}", response.sources.len());
                info!("   Processing time: {}ms", response.processing_time_ms);
                info!("   Cache hit: {}", response.cache_hit);
                info!("   Model used: {}", response.model_used);
                
                if !response.warnings.is_empty() {
                    info!("   Warnings: {:?}", response.warnings);
                }
            }
            Err(e) => {
                error!("❌ Failed to process question: {}", e);
            }
        }
        
        // Small delay between requests
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}

/// Demonstrate error handling capabilities
async fn demonstrate_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    info!("Demonstrating error handling...");

    let mut error_handler = ErrorHandler::new(ErrorHandlerConfig::default());

    // Simulate different types of errors
    let test_errors = vec![
        (sanad::ai_service::AIServiceError::ExternalAPIError("rate limit exceeded".to_string()), "Rate Limit"),
        (sanad::ai_service::AIServiceError::DatabaseError("connection timeout".to_string()), "Database"),
        (sanad::ai_service::AIServiceError::ExternalAPIError("model is loading".to_string()), "Model Loading"),
        (sanad::ai_service::AIServiceError::ServiceUnavailable("service down".to_string()), "Service Unavailable"),
    ];

    for (error, error_name) in test_errors {
        info!("Testing error handling for: {}", error_name);
        
        let context = ErrorContext::new(
            "test_operation".to_string(),
            "test_service".to_string(),
        ).with_user_id("test_user".to_string());

        let recovery_action = error_handler.handle_error(&error, &context, 1).await;
        info!("   Recovery action: {:?}", recovery_action);

        match error_handler.execute_recovery_action(recovery_action, &context).await {
            Ok(Some(result)) => {
                info!("   ✅ Recovery successful: {}", result);
            }
            Ok(None) => {
                info!("   ✅ Recovery action executed (retry)");
            }
            Err(e) => {
                info!("   ❌ Recovery failed: {}", e);
            }
        }
    }

    // Display error metrics
    let metrics = error_handler.get_metrics();
    info!("Error handling metrics:");
    info!("   Total errors: {}", metrics.total_errors);
    info!("   Retry attempts: {}", metrics.retry_attempts);
    info!("   Fallback activations: {}", metrics.fallback_activations);
    info!("   Circuit breaker trips: {}", metrics.circuit_breaker_trips);

    Ok(())
}

/// Monitor service health
async fn monitor_service_health(service_manager: &AIServiceManager) -> Result<(), Box<dyn std::error::Error>> {
    info!("Monitoring service health...");

    for i in 1..=3 {
        info!("Health check #{}", i);
        
        let health_status = service_manager.get_health_status().await;
        info!("   Overall status: {:?}", health_status.overall_status);
        info!("   Hugging Face: {:?}", health_status.hugging_face_status);
        info!("   Vector DB: {:?}", health_status.vector_db_status);
        info!("   Cache: {:?}", health_status.cache_status);
        info!("   Last check: {}", health_status.last_check);
        
        if !health_status.error_details.is_empty() {
            info!("   Error details: {:?}", health_status.error_details);
        }

        let metrics = service_manager.get_metrics().await;
        info!("   Total requests: {}", metrics.total_requests);
        info!("   Success rate: {:.2}%", 
            if metrics.total_requests > 0 {
                (metrics.successful_requests as f64 / metrics.total_requests as f64) * 100.0
            } else {
                0.0
            }
        );
        info!("   Average response time: {:.2}ms", metrics.average_response_time_ms);
        info!("   Cache hit rate: {:.2}%", metrics.cache_hit_rate * 100.0);
        info!("   Fallback usage: {:.2}%", metrics.fallback_usage_rate * 100.0);

        if i < 3 {
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    Ok(())
}

/// Demonstrate caching capabilities
async fn demonstrate_caching(service_manager: &AIServiceManager) -> Result<(), Box<dyn std::error::Error>> {
    info!("Demonstrating caching capabilities...");

    let integration_service = service_manager.get_integration_service().await;
    if integration_service.is_none() {
        info!("Integration service not available, skipping caching demonstration");
        return Ok(());
    }

    let mut service = integration_service.unwrap();

    let test_question = "ما هي أركان الإسلام؟";
    let request = RAGProcessingRequest {
        question: test_question.to_string(),
        context: None,
        max_sources: Some(3),
        similarity_threshold: Some(0.7),
        preferred_source_types: None,
        language: Some("Arabic".to_string()),
        user_id: Some("cache_test_user".to_string()),
    };

    // First request (should miss cache)
    info!("Making first request (cache miss expected)...");
    let start_time = std::time::Instant::now();
    match service.process_rag_request(request.clone()).await {
        Ok(response) => {
            let duration = start_time.elapsed();
            info!("   ✅ First request completed");
            info!("   Cache hit: {} (expected: false)", response.cache_hit);
            info!("   Processing time: {}ms", duration.as_millis());
        }
        Err(e) => {
            error!("   ❌ First request failed: {}", e);
        }
    }

    // Small delay
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Second request (should hit cache)
    info!("Making second request (cache hit expected)...");
    let start_time = std::time::Instant::now();
    match service.process_rag_request(request).await {
        Ok(response) => {
            let duration = start_time.elapsed();
            info!("   ✅ Second request completed");
            info!("   Cache hit: {} (expected: true)", response.cache_hit);
            info!("   Processing time: {}ms (should be faster)", duration.as_millis());
        }
        Err(e) => {
            error!("   ❌ Second request failed: {}", e);
        }
    }

    Ok(())
}

/// Demonstrate fallback mechanisms
async fn demonstrate_fallback_mechanisms(service_manager: &AIServiceManager) -> Result<(), Box<dyn std::error::Error>> {
    info!("Demonstrating fallback mechanisms...");

    // This would typically involve:
    // 1. Simulating service failures
    // 2. Testing model fallbacks
    // 3. Testing offline responses
    // 4. Testing service degradation

    info!("Fallback scenarios:");
    info!("   1. Primary model failure → Fallback to secondary model");
    info!("   2. Network failure → Use cached responses");
    info!("   3. Service unavailable → Offline response");
    info!("   4. Rate limit exceeded → Service degradation");

    // For demonstration purposes, we'll just show the configuration
    let config = AIServiceConfig::from_env();
    info!("Configured fallback models: {:?}", config.get_fallback_models());
    
    if config.fallback.enable_offline_mode {
        info!("Offline mode enabled with {} responses", config.fallback.offline_responses.len());
        for (key, response) in &config.fallback.offline_responses {
            info!("   {}: {}", key, response);
        }
    }

    Ok(())
}

/// Demonstrate content indexing (if vector database is available)
async fn demonstrate_content_indexing(service_manager: &AIServiceManager) -> Result<(), Box<dyn std::error::Error>> {
    info!("Demonstrating content indexing...");

    let integration_service = service_manager.get_integration_service().await;
    if integration_service.is_none() {
        info!("Integration service not available, skipping indexing demonstration");
        return Ok(());
    }

    let mut service = integration_service.unwrap();

    // Create sample Islamic content
    let sample_content = sanad::ai_service::IslamicSource {
        id: "example_ayah_001".to_string(),
        content_type: sanad::ai_service::SourceType::Quran,
        text: "بسم الله الرحمن الرحيم".to_string(),
        reference: "الفاتحة: 1".to_string(),
        author: None,
        authenticity: sanad::ai_service::AuthenticityLevel::Verified,
        language: sanad::ai_service::Language::Arabic,
        metadata: std::collections::HashMap::new(),
        created_at: chrono::Utc::now(),
    };

    info!("Indexing sample content: {}", sample_content.text);
    match service.index_content(sample_content).await {
        Ok(_) => {
            info!("   ✅ Content indexed successfully");
        }
        Err(e) => {
            error!("   ❌ Failed to index content: {}", e);
        }
    }

    Ok(())
}

/// Print configuration summary
fn print_configuration_summary(config: &AIServiceConfig) {
    info!("Configuration Summary:");
    info!("   Hugging Face API: {}", if config.hugging_face.api_key.is_empty() { "Not configured" } else { "Configured" });
    info!("   Vector Database: {}:{}", config.vector_database.host, config.vector_database.port);
    info!("   Cache enabled: {}", config.cache.enable_response_cache);
    info!("   Fallback enabled: {}", config.fallback.enable_fallback);
    info!("   Rate limiting: {} req/min", config.rate_limiting.requests_per_minute);
    info!("   Islamic models: {}", config.hugging_face.islamic_models.len());
}

/// Print example usage instructions
fn print_usage_instructions() {
    println!("\n🚀 AI Service Integration Example");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("This example demonstrates the complete AI service integration setup.");
    println!("");
    println!("📋 Prerequisites:");
    println!("   1. Set HUGGING_FACE_API_KEY environment variable");
    println!("   2. Start Qdrant vector database (docker run -p 6333:6333 qdrant/qdrant)");
    println!("   3. Optionally start Redis (docker run -p 6379:6379 redis:alpine)");
    println!("");
    println!("🔧 Environment Variables:");
    println!("   HUGGING_FACE_API_KEY=your_api_key_here");
    println!("   QDRANT_HOST=localhost (optional)");
    println!("   QDRANT_PORT=6333 (optional)");
    println!("   REDIS_URL=redis://localhost:6379 (optional)");
    println!("");
    println!("▶️  Run with: cargo run --example ai_service_integration");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
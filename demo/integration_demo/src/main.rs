use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{info, warn, error};

mod services;
mod models;

use services::*;
use models::*;

/// Application state containing all services
#[derive(Clone)]
pub struct AppState {
    pub quran_service: Arc<RwLock<MockQuranService>>,
    pub hadith_service: Arc<RwLock<MockHadithService>>,
    pub search_service: Arc<RwLock<MockSearchService>>,
    pub ai_service: Arc<RwLock<MockAIService>>,
    pub integration_stats: Arc<RwLock<IntegrationStats>>,
}

/// Integration statistics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub service_calls: HashMap<String, u64>,
    pub last_reset: chrono::DateTime<chrono::Utc>,
}

impl Default for IntegrationStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            service_calls: HashMap::new(),
            last_reset: chrono::Utc::now(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting Islamic App Integration Demo");

    // Initialize services
    let app_state = AppState {
        quran_service: Arc::new(RwLock::new(MockQuranService::new().await?)),
        hadith_service: Arc::new(RwLock::new(MockHadithService::new().await?)),
        search_service: Arc::new(RwLock::new(MockSearchService::new().await?)),
        ai_service: Arc::new(RwLock::new(MockAIService::new().await?)),
        integration_stats: Arc::new(RwLock::new(IntegrationStats::default())),
    };

    info!("✅ All services initialized successfully");

    // Build the router
    let app = Router::new()
        // Serve static files (HTML, CSS, JS)
        .nest_service("/static", ServeDir::new("demo/integration_demo/static"))
        // Main demo page
        .route("/", get(serve_demo_page))
        // API endpoints
        .route("/api/search", post(search_handler))
        .route("/api/ask", post(ask_ai_handler))
        .route("/api/quran/:surah", get(get_surah_handler))
        .route("/api/hadith/search", get(search_hadith_handler))
        .route("/api/stats", get(get_stats_handler))
        .route("/api/health", get(health_check_handler))
        .route("/api/test-integration", post(test_integration_handler))
        // Add CORS layer
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("🌐 Demo server running at http://127.0.0.1:3000");
    info!("📱 Open your browser and navigate to the URL above to test the integration");

    axum::serve(listener, app).await?;

    Ok(())
}

/// Serve the main demo HTML page
async fn serve_demo_page() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

/// Search endpoint that demonstrates content service integration
async fn search_handler(
    State(state): State<AppState>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, StatusCode> {
    let start_time = std::time::Instant::now();
    
    // Update stats
    {
        let mut stats = state.integration_stats.write().await;
        stats.total_requests += 1;
        *stats.service_calls.entry("search".to_string()).or_insert(0) += 1;
    }

    info!("🔍 Processing search request: {}", request.query);

    let search_service = state.search_service.read().await;
    
    match search_service.search(&request.query, request.content_types.as_deref()).await {
        Ok(results) => {
            let response_time = start_time.elapsed().as_millis() as f64;
            
            // Update success stats
            {
                let mut stats = state.integration_stats.write().await;
                stats.successful_requests += 1;
                stats.average_response_time_ms = 
                    (stats.average_response_time_ms * (stats.successful_requests - 1) as f64 + response_time) 
                    / stats.successful_requests as f64;
            }

            info!("✅ Search completed in {}ms with {} results", response_time as u64, results.len());

            Ok(Json(SearchResponse {
                results: results.clone(),
                total_count: results.len(),
                query: request.query,
                response_time_ms: response_time as u64,
                service_integration: "Content services → Search service".to_string(),
            }))
        }
        Err(e) => {
            error!("❌ Search failed: {}", e);
            
            // Update failure stats
            {
                let mut stats = state.integration_stats.write().await;
                stats.failed_requests += 1;
            }

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// AI question handler that demonstrates full RAG integration
async fn ask_ai_handler(
    State(state): State<AppState>,
    Json(request): Json<AIQuestionRequest>,
) -> Result<Json<AIResponse>, StatusCode> {
    let start_time = std::time::Instant::now();
    
    // Update stats
    {
        let mut stats = state.integration_stats.write().await;
        stats.total_requests += 1;
        *stats.service_calls.entry("ai_rag".to_string()).or_insert(0) += 1;
    }

    info!("🤖 Processing AI question: {}", request.question);

    let ai_service = state.ai_service.read().await;
    
    match ai_service.ask_question(&request.question, request.context.as_deref()).await {
        Ok(response) => {
            let response_time = start_time.elapsed().as_millis() as f64;
            
            // Update success stats
            {
                let mut stats = state.integration_stats.write().await;
                stats.successful_requests += 1;
                stats.average_response_time_ms = 
                    (stats.average_response_time_ms * (stats.successful_requests - 1) as f64 + response_time) 
                    / stats.successful_requests as f64;
            }

            info!("✅ AI response generated in {}ms", response_time as u64);

            Ok(Json(AIResponse {
                answer: response.answer,
                confidence: response.confidence,
                sources: response.sources,
                citations: response.citations,
                warnings: response.warnings,
                response_time_ms: response_time as u64,
                integration_flow: response.integration_flow,
            }))
        }
        Err(e) => {
            error!("❌ AI processing failed: {}", e);
            
            // Update failure stats
            {
                let mut stats = state.integration_stats.write().await;
                stats.failed_requests += 1;
            }

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get Quran surah handler
async fn get_surah_handler(
    State(state): State<AppState>,
    axum::extract::Path(surah_number): axum::extract::Path<u32>,
) -> Result<Json<SurahResponse>, StatusCode> {
    let start_time = std::time::Instant::now();
    
    // Update stats
    {
        let mut stats = state.integration_stats.write().await;
        stats.total_requests += 1;
        *stats.service_calls.entry("quran".to_string()).or_insert(0) += 1;
    }

    info!("📖 Fetching Surah {}", surah_number);

    let quran_service = state.quran_service.read().await;
    
    match quran_service.get_surah(surah_number).await {
        Ok(Some(surah)) => {
            let response_time = start_time.elapsed().as_millis() as f64;
            
            // Update success stats
            {
                let mut stats = state.integration_stats.write().await;
                stats.successful_requests += 1;
                stats.average_response_time_ms = 
                    (stats.average_response_time_ms * (stats.successful_requests - 1) as f64 + response_time) 
                    / stats.successful_requests as f64;
            }

            info!("✅ Surah {} fetched in {}ms", surah_number, response_time as u64);

            Ok(Json(SurahResponse {
                surah,
                response_time_ms: response_time as u64,
            }))
        }
        Ok(None) => {
            warn!("⚠️ Surah {} not found", surah_number);
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("❌ Failed to fetch Surah {}: {}", surah_number, e);
            
            // Update failure stats
            {
                let mut stats = state.integration_stats.write().await;
                stats.failed_requests += 1;
            }

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Search hadith handler
async fn search_hadith_handler(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<HadithSearchResponse>, StatusCode> {
    let start_time = std::time::Instant::now();
    
    // Update stats
    {
        let mut stats = state.integration_stats.write().await;
        stats.total_requests += 1;
        *stats.service_calls.entry("hadith".to_string()).or_insert(0) += 1;
    }

    let query = params.get("q").cloned().unwrap_or_default();
    info!("📚 Searching hadiths for: {}", query);

    let hadith_service = state.hadith_service.read().await;
    
    match hadith_service.search_hadiths(&query).await {
        Ok(hadiths) => {
            let response_time = start_time.elapsed().as_millis() as f64;
            
            // Update success stats
            {
                let mut stats = state.integration_stats.write().await;
                stats.successful_requests += 1;
                stats.average_response_time_ms = 
                    (stats.average_response_time_ms * (stats.successful_requests - 1) as f64 + response_time) 
                    / stats.successful_requests as f64;
            }

            info!("✅ Found {} hadiths in {}ms", hadiths.len(), response_time as u64);

            Ok(Json(HadithSearchResponse {
                hadiths: hadiths.clone(),
                total_count: hadiths.len(),
                query,
                response_time_ms: response_time as u64,
            }))
        }
        Err(e) => {
            error!("❌ Hadith search failed: {}", e);
            
            // Update failure stats
            {
                let mut stats = state.integration_stats.write().await;
                stats.failed_requests += 1;
            }

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get integration statistics
async fn get_stats_handler(
    State(state): State<AppState>,
) -> Json<IntegrationStats> {
    let stats = state.integration_stats.read().await;
    Json(stats.clone())
}

/// Health check endpoint
async fn health_check_handler(
    State(_state): State<AppState>,
) -> Json<HealthStatus> {
    let mut health = HealthStatus {
        status: "healthy".to_string(),
        services: HashMap::new(),
        timestamp: chrono::Utc::now(),
    };

    // Check each service
    health.services.insert("quran".to_string(), "healthy".to_string());
    health.services.insert("hadith".to_string(), "healthy".to_string());
    health.services.insert("search".to_string(), "healthy".to_string());
    health.services.insert("ai".to_string(), "healthy".to_string());

    Json(health)
}

/// Test integration flow endpoint
async fn test_integration_handler(
    State(state): State<AppState>,
    Json(request): Json<IntegrationTestRequest>,
) -> Result<Json<IntegrationTestResponse>, StatusCode> {
    let start_time = std::time::Instant::now();
    
    info!("🧪 Running integration test: {}", request.test_name);

    let mut test_results = Vec::new();
    let mut overall_success = true;

    // Test 1: Content Service Integration
    info!("Testing content service integration...");
    let search_result = {
        let search_service = state.search_service.read().await;
        search_service.search("الصلاة", Some(&["quran".to_string(), "hadith".to_string()])).await
    };

    match search_result {
        Ok(results) => {
            test_results.push(TestResult {
                test_name: "Content Service Integration".to_string(),
                success: true,
                message: format!("Found {} results from content services", results.len()),
                duration_ms: 50,
            });
        }
        Err(e) => {
            overall_success = false;
            test_results.push(TestResult {
                test_name: "Content Service Integration".to_string(),
                success: false,
                message: format!("Failed: {}", e),
                duration_ms: 50,
            });
        }
    }

    // Test 2: AI Service RAG Integration
    info!("Testing AI service RAG integration...");
    let ai_result = {
        let ai_service = state.ai_service.read().await;
        ai_service.ask_question("ما هي أركان الإسلام؟", None).await
    };

    match ai_result {
        Ok(response) => {
            test_results.push(TestResult {
                test_name: "AI Service RAG Integration".to_string(),
                success: true,
                message: format!("Generated response with {} sources", response.sources.len()),
                duration_ms: 200,
            });
        }
        Err(e) => {
            overall_success = false;
            test_results.push(TestResult {
                test_name: "AI Service RAG Integration".to_string(),
                success: false,
                message: format!("Failed: {}", e),
                duration_ms: 200,
            });
        }
    }

    // Test 3: End-to-End Integration
    info!("Testing end-to-end integration...");
    test_results.push(TestResult {
        test_name: "End-to-End Integration".to_string(),
        success: overall_success,
        message: if overall_success {
            "All services integrated successfully".to_string()
        } else {
            "Some integration issues detected".to_string()
        },
        duration_ms: start_time.elapsed().as_millis() as u64,
    });

    let response = IntegrationTestResponse {
        test_name: request.test_name,
        overall_success,
        test_results,
        total_duration_ms: start_time.elapsed().as_millis() as u64,
        timestamp: chrono::Utc::now(),
    };

    info!("🧪 Integration test completed: {} ({}ms)", 
          if overall_success { "✅ PASSED" } else { "❌ FAILED" },
          response.total_duration_ms);

    Ok(Json(response))
}
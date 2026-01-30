use axum::{
    extract::{Query, State},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use shared::{ApiResponse, AppError};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, error};

mod semantic_search;
mod embedding_service;
mod text_processor;
mod indexing_service;
mod models;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod property_tests;

#[cfg(test)]
mod contextual_search_tests;

#[cfg(test)]
mod advanced_search_tests;

use semantic_search::SemanticSearchEngine;
use embedding_service::EmbeddingService;
use indexing_service::IndexingService;
use models::*;

#[derive(Clone)]
pub struct AppState {
    pub semantic_search: Arc<SemanticSearchEngine>,
    pub embedding_service: Arc<EmbeddingService>,
    pub indexing_service: Arc<IndexingService>,
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    info!("Starting Semantic Search Service on port 8087");

    // Initialize services
    let embedding_service = Arc::new(EmbeddingService::new().await?);
    let semantic_search = Arc::new(SemanticSearchEngine::new().await?);
    let indexing_service = Arc::new(IndexingService::new(
        embedding_service.clone(),
        semantic_search.clone(),
    ).await?);

    let state = AppState {
        semantic_search,
        embedding_service,
        indexing_service,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/search/semantic", get(semantic_search_handler))
        .route("/search/similar", get(find_similar_handler))
        .route("/search/suggestions", get(query_suggestions_handler))
        .route("/index/document", post(index_document_handler))
        .route("/index/batch", post(index_batch_handler))
        .route("/index/stats", get(get_stats_handler))
        .route("/index/rebuild", post(rebuild_index_handler))
        .route("/index/validate", get(validate_index_handler))
        .route("/index/sample", post(index_sample_data_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8087").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "semantic-search-service".to_string());
    status.insert("version".to_string(), "1.0.0".to_string());
    status.insert("features".to_string(), "semantic_search,vector_indexing,arabic_embeddings,contextual_search,root_analysis".to_string());
    status.insert("endpoints".to_string(), "/search/semantic,/search/similar,/search/contextual,/search/roots,/search/suggestions,/index/*".to_string());
    Json(ApiResponse::success(status))
}

#[derive(Debug, Deserialize)]
struct SemanticSearchQuery {
    query: String,
    limit: Option<usize>,
    content_types: Option<String>, // comma-separated
    min_similarity: Option<f32>,
    include_metadata: Option<bool>,
    page: Option<usize>,
    page_size: Option<usize>,
    include_suggestions: Option<bool>,
    enable_caching: Option<bool>,
    sort_by: Option<String>,
    sort_direction: Option<String>,
}

async fn semantic_search_handler(
    State(state): State<AppState>,
    Query(params): Query<SemanticSearchQuery>,
) -> std::result::Result<Json<ApiResponse<SemanticSearchResponse>>, AppError> {
    info!("Semantic search request: {}", params.query);

    let content_types: Vec<String> = params.content_types
        .map(|types| types.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let sort_by = params.sort_by.as_deref().and_then(|s| match s {
        "similarity" => Some(SortBy::Similarity),
        "priority" => Some(SortBy::Priority),
        "created_at" => Some(SortBy::CreatedAt),
        "updated_at" => Some(SortBy::UpdatedAt),
        "text_length" => Some(SortBy::TextLength),
        "relevance" => Some(SortBy::Relevance),
        _ => None,
    });

    let sort_direction = params.sort_direction.as_deref().and_then(|s| match s {
        "asc" => Some(SortDirection::Asc),
        "desc" => Some(SortDirection::Desc),
        _ => None,
    });

    let search_request = SemanticSearchRequest {
        query: params.query,
        limit: params.limit.unwrap_or(20),
        content_types: if content_types.is_empty() { None } else { Some(content_types) },
        min_similarity: params.min_similarity.unwrap_or(0.5),
        include_metadata: params.include_metadata.unwrap_or(true),
        filters: None,
        offset: None,
        page: params.page,
        page_size: params.page_size,
        include_suggestions: params.include_suggestions.unwrap_or(false),
        enable_caching: params.enable_caching.unwrap_or(true),
        sort_by,
        sort_direction,
    };

    match state.semantic_search.search(search_request).await {
        Ok(response) => {
            info!("Found {} results", response.results.len());
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Semantic search failed: {}", e);
            Err(AppError::Internal(format!("Search failed: {}", e)))
        }
    }
}

#[derive(Debug, Deserialize)]
struct SimilarDocumentsQuery {
    document_id: String,
    limit: Option<usize>,
    content_types: Option<String>,
}

async fn find_similar_handler(
    State(state): State<AppState>,
    Query(params): Query<SimilarDocumentsQuery>,
) -> std::result::Result<Json<ApiResponse<Vec<SearchResult>>>, AppError> {
    info!("Finding similar documents for: {}", params.document_id);

    let content_types = params.content_types
        .map(|types| types.split(',').map(|s| s.trim().to_string()).collect());

    match state.semantic_search.find_similar_documents(
        &params.document_id,
        params.limit.unwrap_or(5),
        content_types,
    ).await {
        Ok(results) => {
            info!("Found {} similar documents", results.len());
            Ok(Json(ApiResponse::success(results)))
        }
        Err(e) => {
            error!("Similar documents search failed: {}", e);
            Err(AppError::Internal(format!("Similar search failed: {}", e)))
        }
    }
}

#[derive(Debug, Deserialize)]
struct ContextualSearchQuery {
    query: String,
    context: Option<String>,
    search_mode: Option<String>,
    expand_synonyms: Option<bool>,
    expand_roots: Option<bool>,
    expand_concepts: Option<bool>,
    limit: Option<usize>,
    content_types: Option<String>, // comma-separated
    min_similarity: Option<f32>,
}

async fn contextual_search_handler(
    State(state): State<AppState>,
    Query(params): Query<ContextualSearchQuery>,
) -> std::result::Result<Json<ApiResponse<Vec<semantic_search::ContextualSearchResult>>>, AppError> {
    info!("Contextual search request: {}", params.query);

    let search_mode = match params.search_mode.as_deref() {
        Some("exact") => semantic_search::SearchMode::Exact,
        Some("expanded") => semantic_search::SearchMode::Expanded,
        Some("conceptual") => semantic_search::SearchMode::Conceptual,
        Some("root_based") => semantic_search::SearchMode::RootBased,
        Some("hybrid") => semantic_search::SearchMode::Hybrid,
        _ => semantic_search::SearchMode::Hybrid,
    };

    let content_types = params.content_types
        .map(|types| types.split(',').map(|s| s.trim().to_string()).collect());

    let contextual_request = semantic_search::ContextualSearchRequest {
        query: params.query,
        context: params.context,
        search_mode,
        expand_synonyms: params.expand_synonyms.unwrap_or(true),
        expand_roots: params.expand_roots.unwrap_or(true),
        expand_concepts: params.expand_concepts.unwrap_or(true),
        limit: params.limit.unwrap_or(20),
        content_types,
        min_similarity: params.min_similarity.unwrap_or(0.4),
        filters: None,
    };

    match state.semantic_search.contextual_search(contextual_request).await {
        Ok(results) => {
            info!("Found {} contextual results", results.len());
            Ok(Json(ApiResponse::success(results)))
        }
        Err(e) => {
            error!("Contextual search failed: {}", e);
            Err(AppError::Internal(format!("Contextual search failed: {}", e)))
        }
    }
}

#[derive(Debug, Deserialize)]
struct SearchByRootsQuery {
    roots: String, // comma-separated
    limit: Option<usize>,
    content_types: Option<String>, // comma-separated
}

async fn search_by_roots_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchByRootsQuery>,
) -> std::result::Result<Json<ApiResponse<Vec<semantic_search::ContextualSearchResult>>>, AppError> {
    let roots: Vec<String> = params.roots.split(',').map(|s| s.trim().to_string()).collect();
    info!("Search by roots request: {:?}", roots);

    let content_types = params.content_types
        .map(|types| types.split(',').map(|s| s.trim().to_string()).collect());

    match state.semantic_search.search_by_roots(
        roots,
        params.limit.unwrap_or(20),
        content_types,
    ).await {
        Ok(results) => {
            info!("Found {} root-based results", results.len());
            Ok(Json(ApiResponse::success(results)))
        }
        Err(e) => {
            error!("Root-based search failed: {}", e);
            Err(AppError::Internal(format!("Root-based search failed: {}", e)))
        }
    }
}

#[derive(Debug, Deserialize)]
struct QuerySuggestionsQuery {
    query: String,
    limit: Option<usize>,
}

async fn query_suggestions_handler(
    State(state): State<AppState>,
    Query(params): Query<QuerySuggestionsQuery>,
) -> std::result::Result<Json<ApiResponse<Vec<QuerySuggestion>>>, AppError> {
    info!("Query suggestions request: {}", params.query);

    // Process the query first
    let text_processor = match text_processor::ArabicTextProcessor::new() {
        Ok(processor) => processor,
        Err(e) => return Err(AppError::Internal(format!("Text processor error: {}", e))),
    };
    
    let processed = match text_processor.process_text(&params.query) {
        Ok(processed) => processed,
        Err(e) => return Err(AppError::Internal(format!("Text processing failed: {}", e))),
    };

    match state.semantic_search.generate_query_suggestions(&params.query, &processed).await {
        Ok(mut suggestions) => {
            // Limit suggestions if requested
            if let Some(limit) = params.limit {
                suggestions.truncate(limit);
            }
            info!("Generated {} query suggestions", suggestions.len());
            Ok(Json(ApiResponse::success(suggestions)))
        }
        Err(e) => {
            error!("Query suggestions failed: {}", e);
            Err(AppError::Internal(format!("Query suggestions failed: {}", e)))
        }
    }
}

async fn index_document_handler(
    State(state): State<AppState>,
    Json(document): Json<IslamicDocument>,
) -> std::result::Result<Json<ApiResponse<IndexingResult>>, AppError> {
    info!("Indexing document: {}", document.id);

    match state.indexing_service.index_document(document).await {
        Ok(result) => {
            info!("Document indexed successfully");
            Ok(Json(ApiResponse::success(result)))
        }
        Err(e) => {
            error!("Document indexing failed: {}", e);
            Err(AppError::Internal(format!("Indexing failed: {}", e)))
        }
    }
}

#[derive(Debug, Deserialize)]
struct BatchIndexRequest {
    documents: Vec<IslamicDocument>,
    batch_size: Option<usize>,
}

async fn index_batch_handler(
    State(state): State<AppState>,
    Json(request): Json<BatchIndexRequest>,
) -> std::result::Result<Json<ApiResponse<BatchIndexingResult>>, AppError> {
    info!("Batch indexing {} documents", request.documents.len());

    match state.indexing_service.index_documents_batch(
        request.documents,
        request.batch_size.unwrap_or(100),
    ).await {
        Ok(result) => {
            info!("Batch indexing completed: {} successful, {} failed", 
                  result.successful_count, result.failed_count);
            Ok(Json(ApiResponse::success(result)))
        }
        Err(e) => {
            error!("Batch indexing failed: {}", e);
            Err(AppError::Internal(format!("Batch indexing failed: {}", e)))
        }
    }
}

async fn get_stats_handler(
    State(state): State<AppState>,
) -> std::result::Result<Json<ApiResponse<IndexStats>>, AppError> {
    match state.indexing_service.get_stats().await {
        Ok(stats) => Ok(Json(ApiResponse::success(stats))),
        Err(e) => {
            error!("Failed to get stats: {}", e);
            Err(AppError::Internal(format!("Stats retrieval failed: {}", e)))
        }
    }
}

#[derive(Debug, Deserialize)]
struct RebuildIndexRequest {
    content_types: Option<Vec<String>>,
    force: Option<bool>,
}

async fn rebuild_index_handler(
    State(state): State<AppState>,
    Json(request): Json<RebuildIndexRequest>,
) -> std::result::Result<Json<ApiResponse<RebuildResult>>, AppError> {
    info!("Rebuilding index for content types: {:?}", request.content_types);

    match state.indexing_service.rebuild_index(
        request.content_types,
        request.force.unwrap_or(false),
    ).await {
        Ok(result) => {
            info!("Index rebuild completed: {} documents processed", result.processed_count);
            Ok(Json(ApiResponse::success(result)))
        }
        Err(e) => {
            error!("Index rebuild failed: {}", e);
            Err(AppError::Internal(format!("Index rebuild failed: {}", e)))
        }
    }
}

async fn validate_index_handler(
    State(state): State<AppState>,
) -> std::result::Result<Json<ApiResponse<indexing_service::ValidationResult>>, AppError> {
    info!("Validating index integrity");

    match state.indexing_service.validate_index().await {
        Ok(result) => {
            info!("Index validation completed: valid={}", result.is_valid);
            Ok(Json(ApiResponse::success(result)))
        }
        Err(e) => {
            error!("Index validation failed: {}", e);
            Err(AppError::Internal(format!("Index validation failed: {}", e)))
        }
    }
}

async fn index_sample_data_handler(
    State(state): State<AppState>,
) -> std::result::Result<Json<ApiResponse<BatchIndexingResult>>, AppError> {
    info!("Indexing sample Islamic data");

    match state.indexing_service.index_from_database(None).await {
        Ok(result) => {
            info!("Sample data indexing completed: {} successful, {} failed", 
                  result.successful_count, result.failed_count);
            Ok(Json(ApiResponse::success(result)))
        }
        Err(e) => {
            error!("Sample data indexing failed: {}", e);
            Err(AppError::Internal(format!("Sample data indexing failed: {}", e)))
        }
    }
}


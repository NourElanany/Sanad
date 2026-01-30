use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
use shared::ApiResponse;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

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

// use semantic_search::SemanticSearchEngine;
use embedding_service::EmbeddingService;
// use indexing_service::IndexingService;

#[derive(Clone)]
pub struct AppState {
    // pub semantic_search: Arc<SemanticSearchEngine>,
    pub embedding_service: Arc<EmbeddingService>,
    // pub indexing_service: Arc<IndexingService>,
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    info!("Starting Semantic Search Service on port 8087");

    // Initialize services
    let embedding_service = Arc::new(EmbeddingService::new().await?);
    // let semantic_search = Arc::new(SemanticSearchEngine::new().await?);
    // let indexing_service = Arc::new(IndexingService::new(
    //     embedding_service.clone(),
    //     semantic_search.clone(),
    // ).await?);

    let state = AppState {
        // semantic_search,
        embedding_service,
        // indexing_service,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        // .route("/search/semantic", get(semantic_search_handler))
        // .route("/search/similar", get(find_similar_handler))
        // .route("/index/document", post(index_document_handler))
        // .route("/index/batch", post(index_batch_handler))
        // .route("/index/stats", get(get_stats_handler))
        // .route("/index/rebuild", post(rebuild_index_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8087").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "semantic-search-service".to_string());
    status.insert("features".to_string(), "semantic_search,vector_indexing,arabic_embeddings".to_string());
    Json(ApiResponse::success(status))
}

#[derive(Debug, Deserialize)]
struct SemanticSearchQuery {
    query: String,
    limit: Option<usize>,
    content_types: Option<String>, // comma-separated
    min_similarity: Option<f32>,
    include_metadata: Option<bool>,
}

/*
async fn semantic_search_handler(
    State(state): State<AppState>,
    Query(params): Query<SemanticSearchQuery>,
) -> std::result::Result<Json<ApiResponse<SemanticSearchResponse>>, AppError> {
    info!("Semantic search request: {}", params.query);

    let content_types: Vec<String> = params.content_types
        .map(|types| types.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let search_request = SemanticSearchRequest {
        query: params.query,
        limit: params.limit.unwrap_or(10),
        content_types: if content_types.is_empty() { None } else { Some(content_types) },
        min_similarity: params.min_similarity.unwrap_or(0.5),
        include_metadata: params.include_metadata.unwrap_or(true),
        filters: None,
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
*/

/*
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
*/
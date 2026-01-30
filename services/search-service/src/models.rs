use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Islamic document for semantic indexing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicDocument {
    pub id: String,
    pub text: String,
    pub content_type: ContentType,
    pub source: String,
    pub author: Option<String>,
    pub language: Language,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Types of Islamic content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Quran,
    SahihHadith,
    HasanHadith,
    DaifHadith,
    MawduHadith,
    Tafsir,
    FiqhRuling,
    ScholarOpinion,
    IslamicStory,
    Dua,
    Dhikr,
    Biography,
    History,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Quran => "quran",
            ContentType::SahihHadith => "sahih_hadith",
            ContentType::HasanHadith => "hasan_hadith",
            ContentType::DaifHadith => "daif_hadith",
            ContentType::MawduHadith => "mawdu_hadith",
            ContentType::Tafsir => "tafsir",
            ContentType::FiqhRuling => "fiqh_ruling",
            ContentType::ScholarOpinion => "scholar_opinion",
            ContentType::IslamicStory => "islamic_story",
            ContentType::Dua => "dua",
            ContentType::Dhikr => "dhikr",
            ContentType::Biography => "biography",
            ContentType::History => "history",
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            ContentType::Quran => 1,
            ContentType::SahihHadith => 2,
            ContentType::HasanHadith => 3,
            ContentType::Tafsir => 4,
            ContentType::FiqhRuling => 5,
            ContentType::DaifHadith => 6,
            ContentType::ScholarOpinion => 7,
            ContentType::IslamicStory => 8,
            ContentType::Dua => 9,
            ContentType::Dhikr => 10,
            ContentType::Biography => 11,
            ContentType::History => 12,
            ContentType::MawduHadith => 13, // Lowest priority
        }
    }

    /// Get authenticity grade for hadith content types
    pub fn authenticity_grade(&self) -> Option<AuthenticityGrade> {
        match self {
            ContentType::SahihHadith => Some(AuthenticityGrade::Sahih),
            ContentType::HasanHadith => Some(AuthenticityGrade::Hasan),
            ContentType::DaifHadith => Some(AuthenticityGrade::Daif),
            ContentType::MawduHadith => Some(AuthenticityGrade::Mawdu),
            _ => None,
        }
    }
}

impl AuthenticityGrade {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuthenticityGrade::Sahih => "sahih",
            AuthenticityGrade::Hasan => "hasan",
            AuthenticityGrade::Daif => "daif",
            AuthenticityGrade::Mawdu => "mawdu",
            AuthenticityGrade::Unknown => "unknown",
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            AuthenticityGrade::Sahih => 1,
            AuthenticityGrade::Hasan => 2,
            AuthenticityGrade::Daif => 3,
            AuthenticityGrade::Unknown => 4,
            AuthenticityGrade::Mawdu => 5, // Lowest priority
        }
    }
}

/// Supported languages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Arabic,
    English,
    French,
    Urdu,
    Turkish,
    Indonesian,
    Malay,
}

/// Semantic search request with pagination and advanced filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchRequest {
    pub query: String,
    pub limit: usize,
    pub content_types: Option<Vec<String>>,
    pub min_similarity: f32,
    pub include_metadata: bool,
    pub filters: Option<SearchFilters>,
    /// Pagination offset
    pub offset: Option<usize>,
    /// Page number (alternative to offset)
    pub page: Option<usize>,
    /// Page size (alternative to limit)
    pub page_size: Option<usize>,
    /// Enable query suggestions
    pub include_suggestions: bool,
    /// Enable result caching
    pub enable_caching: bool,
    /// Sort order for results
    pub sort_by: Option<SortBy>,
    /// Sort direction
    pub sort_direction: Option<SortDirection>,
}

/// Sort options for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    /// Sort by similarity score (default)
    Similarity,
    /// Sort by content type priority
    Priority,
    /// Sort by creation date
    CreatedAt,
    /// Sort by update date
    UpdatedAt,
    /// Sort by text length
    TextLength,
    /// Sort by relevance (combined score)
    Relevance,
}

/// Sort direction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    /// Ascending order
    Asc,
    /// Descending order (default)
    Desc,
}

/// Advanced search filters with enhanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub source: Option<Vec<String>>,
    pub author: Option<Vec<String>>,
    pub language: Option<Language>,
    pub date_range: Option<DateRange>,
    pub metadata_filters: Option<HashMap<String, serde_json::Value>>,
    /// Filter by content types (Quran, Hadith, etc.)
    pub content_types: Option<Vec<ContentType>>,
    /// Filter by authenticity grade for Hadith
    pub authenticity_grades: Option<Vec<AuthenticityGrade>>,
    /// Minimum similarity score threshold
    pub min_similarity: Option<f32>,
    /// Maximum similarity score threshold
    pub max_similarity: Option<f32>,
    /// Filter by text length range
    pub text_length_range: Option<RangeFilter<usize>>,
    /// Filter by priority level
    pub priority_range: Option<RangeFilter<u8>>,
}

/// Authenticity grades for Islamic content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticityGrade {
    /// Authentic (صحيح)
    Sahih,
    /// Good (حسن)
    Hasan,
    /// Weak (ضعيف)
    Daif,
    /// Fabricated (موضوع)
    Mawdu,
    /// Unknown authenticity
    Unknown,
}

/// Generic range filter for numeric values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeFilter<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

/// Date range filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

/// Search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub document: IslamicDocument,
    pub similarity_score: f32,
    pub rank: usize,
    pub highlighted_text: Option<String>,
    pub explanation: Option<String>,
}

/// Semantic search response with pagination and suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchResponse {
    pub results: Vec<SearchResult>,
    pub total_results: usize,
    pub search_time_ms: u64,
    pub query_embedding_time_ms: u64,
    pub search_metadata: SearchMetadata,
    /// Pagination information
    pub pagination: Option<PaginationInfo>,
    /// Query suggestions based on semantic similarity
    pub suggestions: Option<Vec<QuerySuggestion>>,
    /// Indicates if results were served from cache
    pub from_cache: bool,
    /// Cache key for this query (if cached)
    pub cache_key: Option<String>,
}

/// Pagination information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub current_page: usize,
    pub total_pages: usize,
    pub page_size: usize,
    pub total_items: usize,
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub next_page: Option<usize>,
    pub previous_page: Option<usize>,
}

/// Query suggestion based on semantic similarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySuggestion {
    pub suggested_query: String,
    pub similarity_score: f32,
    pub expected_results_count: usize,
    pub suggestion_type: SuggestionType,
    pub explanation: Option<String>,
}

/// Types of query suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionType {
    /// Synonym-based suggestion
    Synonym,
    /// Concept-based suggestion
    Conceptual,
    /// Root-based suggestion
    Morphological,
    /// Popular query suggestion
    Popular,
    /// Corrected query suggestion
    Correction,
}

/// Search metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMetadata {
    pub query_processed: String,
    pub query_keywords: Vec<String>,
    pub content_types_searched: Vec<String>,
    pub filters_applied: bool,
    pub embedding_model: String,
}

/// Indexing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingResult {
    pub document_id: String,
    pub success: bool,
    pub embedding_generated: bool,
    pub indexed_at: DateTime<Utc>,
    pub processing_time_ms: u64,
    pub error: Option<String>,
}

/// Batch indexing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchIndexingResult {
    pub total_documents: usize,
    pub successful_count: usize,
    pub failed_count: usize,
    pub processing_time_ms: u64,
    pub failed_documents: Vec<FailedIndexing>,
}

/// Failed indexing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedIndexing {
    pub document_id: String,
    pub error: String,
}

/// Index statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_documents: u64,
    pub documents_by_type: HashMap<String, u64>,
    pub documents_by_language: HashMap<String, u64>,
    pub index_size_mb: f64,
    pub last_updated: DateTime<Utc>,
    pub embedding_model: String,
    pub vector_dimensions: usize,
}

/// Index rebuild result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebuildResult {
    pub processed_count: usize,
    pub successful_count: usize,
    pub failed_count: usize,
    pub processing_time_ms: u64,
    pub content_types_processed: Vec<String>,
}

/// Embedding vector with metadata
#[derive(Debug, Clone)]
pub struct DocumentEmbedding {
    pub document_id: String,
    pub embedding: Vec<f32>,
    pub text_hash: String,
    pub generated_at: DateTime<Utc>,
}

/// Text processing result
#[derive(Debug, Clone)]
pub struct ProcessedText {
    pub original: String,
    pub normalized: String,
    pub keywords: Vec<String>,
    pub language_detected: Option<Language>,
    pub text_length: usize,
    pub word_count: usize,
}

/// Similarity calculation result
#[derive(Debug, Clone)]
pub struct SimilarityResult {
    pub document_id: String,
    pub similarity_score: f32,
    pub distance: f32,
    pub rank: usize,
}

/// Vector search parameters
#[derive(Debug, Clone)]
pub struct VectorSearchParams {
    pub query_vector: Vec<f32>,
    pub limit: usize,
    pub score_threshold: f32,
    pub content_type_filter: Option<Vec<ContentType>>,
    pub metadata_filter: Option<HashMap<String, serde_json::Value>>,
}

/// Collection configuration for Qdrant
#[derive(Debug, Clone)]
pub struct CollectionConfig {
    pub name: String,
    pub vector_size: usize,
    pub distance_metric: DistanceMetric,
    pub shard_number: Option<u32>,
    pub replication_factor: Option<u32>,
}

/// Distance metrics for vector similarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistanceMetric {
    Cosine,
    Euclidean,
    Dot,
}

impl Default for DistanceMetric {
    fn default() -> Self {
        DistanceMetric::Cosine
    }
}

/// Error types for the search service
#[derive(Debug, thiserror::Error)]
pub enum SearchServiceError {
    #[error("Embedding generation failed: {0}")]
    EmbeddingError(String),
    
    #[error("Vector database error: {0}")]
    VectorDatabaseError(String),
    
    #[error("Text processing error: {0}")]
    TextProcessingError(String),
    
    #[error("Document not found: {0}")]
    DocumentNotFound(String),
    
    #[error("Invalid search parameters: {0}")]
    InvalidParameters(String),
    
    #[error("Indexing failed: {0}")]
    IndexingError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("External service error: {0}")]
    ExternalServiceError(String),
    
    #[error("Database connection error: {0}")]
    DatabaseError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<SearchServiceError> for std::io::Error {
    fn from(err: SearchServiceError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, SearchServiceError>;

impl Default for SemanticSearchRequest {
    fn default() -> Self {
        Self {
            query: String::new(),
            limit: 20,
            content_types: None,
            min_similarity: 0.5,
            include_metadata: true,
            filters: None,
            offset: None,
            page: None,
            page_size: None,
            include_suggestions: false,
            enable_caching: true,
            sort_by: None,
            sort_direction: None,
        }
    }
}

impl Default for SortBy {
    fn default() -> Self {
        SortBy::Similarity
    }
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Desc
    }
}

/// Configuration for the search service
#[derive(Debug, Clone)]
pub struct SearchServiceConfig {
    pub embedding_model: String,
    pub qdrant_url: String,
    pub collection_name: String,
    pub vector_size: usize,
    pub batch_size: usize,
    pub max_search_results: usize,
    pub default_similarity_threshold: f32,
    pub cache_embeddings: bool,
    pub cache_ttl_seconds: u64,
}

impl Default for SearchServiceConfig {
    fn default() -> Self {
        Self {
            embedding_model: "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2".to_string(),
            qdrant_url: "http://localhost:6333".to_string(),
            collection_name: "islamic_content".to_string(),
            vector_size: 384,
            batch_size: 100,
            max_search_results: 100,
            default_similarity_threshold: 0.5,
            cache_embeddings: true,
            cache_ttl_seconds: 3600,
        }
    }
}
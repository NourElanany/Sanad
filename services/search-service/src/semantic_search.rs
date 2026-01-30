use crate::models::*;
use crate::text_processor::ArabicTextProcessor;
use qdrant_client::{
    Qdrant,
    qdrant::{
        vectors_config::Config, CreateCollection, Distance, PointStruct, SearchPoints,
        VectorParams, VectorsConfig, Filter, Condition, FieldCondition, Match, Value,
        ScoredPoint, PointId, UpsertPointsBuilder, DeletePointsBuilder,
    },
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{info, error, debug, warn};
use sha2::{Sha256, Digest};

/// Semantic search engine using Qdrant vector database
#[derive(Clone)]
pub struct SemanticSearchEngine {
    pub client: Qdrant,
    pub config: SearchServiceConfig,
    pub text_processor: ArabicTextProcessor,
    pub collection_name: String,
    pub synonym_map: HashMap<String, Vec<String>>,
    pub concept_map: HashMap<String, Vec<String>>,
    pub root_to_words: HashMap<String, Vec<String>>,
    /// Cache for popular queries and their suggestions
    pub query_cache: Arc<RwLock<HashMap<String, CachedQueryResult>>>,
    /// Cache for query suggestions
    pub suggestion_cache: Arc<RwLock<HashMap<String, Vec<QuerySuggestion>>>>,
}

/// Cached query result with metadata
#[derive(Debug, Clone)]
pub struct CachedQueryResult {
    pub response: SemanticSearchResponse,
    pub cached_at: std::time::Instant,
    pub access_count: u64,
    pub last_accessed: std::time::Instant,
}

/// Enhanced semantic search request with contextual understanding
#[derive(Debug, Clone)]
pub struct ContextualSearchRequest {
    pub query: String,
    pub context: Option<String>,
    pub search_mode: SearchMode,
    pub expand_synonyms: bool,
    pub expand_roots: bool,
    pub expand_concepts: bool,
    pub limit: usize,
    pub content_types: Option<Vec<String>>,
    pub min_similarity: f32,
    pub filters: Option<SearchFilters>,
}

/// Search modes for different types of semantic understanding
#[derive(Debug, Clone)]
pub enum SearchMode {
    /// Exact semantic matching
    Exact,
    /// Expanded search with synonyms and related terms
    Expanded,
    /// Conceptual search understanding broader meanings
    Conceptual,
    /// Root-based search for Arabic morphological analysis
    RootBased,
    /// Hybrid approach combining multiple methods
    Hybrid,
}

/// Enhanced search result with contextual information
#[derive(Debug, Clone)]
pub struct ContextualSearchResult {
    pub document: IslamicDocument,
    pub similarity_score: f32,
    pub contextual_score: f32,
    pub combined_score: f32,
    pub rank: usize,
    pub match_type: MatchType,
    pub matched_terms: Vec<String>,
    pub expanded_terms: Vec<String>,
    pub highlighted_text: Option<String>,
    pub explanation: Option<String>,
}

/// Types of matches found in contextual search
#[derive(Debug, Clone)]
pub enum MatchType {
    DirectMatch,
    SynonymMatch,
    RootMatch,
    ConceptualMatch,
    ContextualMatch,
}

/// Query understanding result
#[derive(Debug, Clone)]
pub struct QueryUnderstanding {
    pub original_query: String,
    pub normalized_query: String,
    pub query_keywords: Vec<String>,
    pub detected_language: Option<Language>,
    pub extracted_concepts: Vec<String>,
    pub identified_roots: Vec<String>,
    pub synonyms: Vec<String>,
    pub related_terms: Vec<String>,
    pub query_intent: QueryIntent,
    pub confidence: f32,
}

/// Types of query intents
#[derive(Debug, Clone)]
pub enum QueryIntent {
    FactualQuestion,
    ConceptualInquiry,
    TextualSearch,
    ComparativeAnalysis,
    DefinitionRequest,
    Unknown,
}

impl SemanticSearchEngine {
    /// Create a new semantic search engine
    pub async fn new() -> Result<Self> {
        let config = SearchServiceConfig::default();
        Self::with_config(config).await
    }

    /// Create semantic search engine with custom configuration
    pub async fn with_config(config: SearchServiceConfig) -> Result<Self> {
        let client = Qdrant::from_url(&config.qdrant_url)
            .build()
            .map_err(|e| SearchServiceError::VectorDatabaseError(format!("Failed to connect to Qdrant: {}", e)))?;

        let text_processor = ArabicTextProcessor::new()?;
        let collection_name = config.collection_name.clone();

        let mut engine = Self {
            client,
            config,
            text_processor,
            collection_name,
            synonym_map: HashMap::new(),
            concept_map: HashMap::new(),
            root_to_words: HashMap::new(),
            query_cache: Arc::new(RwLock::new(HashMap::new())),
            suggestion_cache: Arc::new(RwLock::new(HashMap::new())),
        };

        // Initialize collection if it doesn't exist
        engine.ensure_collection_exists().await?;
        
        // Initialize semantic knowledge bases
        engine.initialize_semantic_knowledge().await?;

        info!("Semantic search engine initialized with collection: {}", engine.collection_name);
        Ok(engine)
    }

    /// Ensure the collection exists in Qdrant
    async fn ensure_collection_exists(&mut self) -> Result<()> {
        // Check if collection exists
        match self.client.collection_info(&self.collection_name).await {
            Ok(_) => {
                info!("Collection '{}' already exists", self.collection_name);
                Ok(())
            }
            Err(_) => {
                info!("Creating new collection: {}", self.collection_name);
                self.create_collection().await
            }
        }
    }

    /// Create a new collection in Qdrant
    async fn create_collection(&mut self) -> Result<()> {
        let create_collection = CreateCollection {
            collection_name: self.collection_name.clone(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: self.config.vector_size as u64,
                    distance: Distance::Cosine.into(),
                    hnsw_config: None,
                    quantization_config: None,
                    on_disk: None,
                    datatype: None,
                    multivector_config: None,
                })),
            }),
            hnsw_config: None,
            wal_config: None,
            optimizers_config: None,
            shard_number: Some(1),
            on_disk_payload: None,
            timeout: Some(60),
            replication_factor: None,
            write_consistency_factor: None,
            quantization_config: None,
            sharding_method: None,
            sparse_vectors_config: None,
            strict_mode_config: None,
            metadata: HashMap::new(),
        };

        self.client
            .create_collection(create_collection)
            .await
            .map_err(|e| SearchServiceError::VectorDatabaseError(format!("Failed to create collection: {}", e)))?;

        info!("Successfully created collection: {}", self.collection_name);
        Ok(())
    }

    /// Index a document in the vector database
    pub async fn index_document(&mut self, document: &IslamicDocument, embedding: Vec<f32>) -> Result<()> {
        let point_id = self.generate_point_id(&document.id);
        
        // Prepare payload
        let mut payload = HashMap::new();
        payload.insert("id".to_string(), Value::from(document.id.clone()));
        payload.insert("text".to_string(), Value::from(document.text.clone()));
        payload.insert("content_type".to_string(), Value::from(document.content_type.as_str()));
        payload.insert("source".to_string(), Value::from(document.source.clone()));
        payload.insert("language".to_string(), Value::from(format!("{:?}", document.language)));
        
        if let Some(author) = &document.author {
            payload.insert("author".to_string(), Value::from(author.clone()));
        }

        // Add metadata
        for (key, value) in &document.metadata {
            payload.insert(format!("metadata_{}", key), self.json_value_to_qdrant_value(value));
        }

        // Add processed text information
        let processed = self.text_processor.process_text(&document.text)?;
        payload.insert("keywords".to_string(), Value::from(processed.keywords.join(",")));
        payload.insert("text_length".to_string(), Value::from(processed.text_length as i64));
        payload.insert("word_count".to_string(), Value::from(processed.word_count as i64));
        
        if let Some(lang) = processed.language_detected {
            payload.insert("detected_language".to_string(), Value::from(format!("{:?}", lang)));
        }

        // Add timestamps
        if let Some(created_at) = document.created_at {
            payload.insert("created_at".to_string(), Value::from(created_at.timestamp()));
        }
        if let Some(updated_at) = document.updated_at {
            payload.insert("updated_at".to_string(), Value::from(updated_at.timestamp()));
        }

        // Add content type priority for ranking
        payload.insert("priority".to_string(), Value::from(document.content_type.priority() as i64));

        let point = PointStruct::new(point_id, embedding, payload);

        let upsert_request = UpsertPointsBuilder::new(self.collection_name.clone(), vec![point]);

        self.client
            .upsert_points(upsert_request)
            .await
            .map_err(|e| SearchServiceError::VectorDatabaseError(format!("Failed to index document: {}", e)))?;

        debug!("Successfully indexed document: {}", document.id);
        Ok(())
    }

    /// Index multiple documents in batch
    pub async fn index_documents_batch(&mut self, documents: &[(IslamicDocument, Vec<f32>)]) -> Result<usize> {
        let mut points = Vec::new();
        let mut successful_count = 0;

        for (document, embedding) in documents {
            match self.create_point_struct(document, embedding.clone()) {
                Ok(point) => {
                    points.push(point);
                    successful_count += 1;
                }
                Err(e) => {
                    error!("Failed to create point for document {}: {}", document.id, e);
                }
            }
        }

        if !points.is_empty() {
            let upsert_request = UpsertPointsBuilder::new(self.collection_name.clone(), points);
            
            self.client
                .upsert_points(upsert_request)
                .await
                .map_err(|e| SearchServiceError::VectorDatabaseError(format!("Failed to batch index documents: {}", e)))?;
        }

        info!("Successfully indexed {} documents in batch", successful_count);
        Ok(successful_count)
    }

    /// Perform semantic search with advanced filtering, pagination, and caching
    pub async fn search(&self, request: SemanticSearchRequest) -> Result<SemanticSearchResponse> {
        let start_time = Instant::now();
        
        // Generate cache key for this query
        let cache_key = self.generate_cache_key(&request);
        
        // Check cache first if caching is enabled
        if request.enable_caching {
            if let Some(cached_result) = self.get_cached_result(&cache_key).await {
                debug!("Returning cached result for query: {}", request.query);
                return Ok(cached_result);
            }
        }
        
        // Process the query
        let processed_query = self.text_processor.process_text(&request.query)?;
        let embedding_start = Instant::now();
        
        // Generate query embedding
        let query_embedding = self.generate_mock_query_embedding(&processed_query.normalized);
        let query_embedding_time_ms = embedding_start.elapsed().as_millis() as u64;

        // Build advanced search filter
        let filter = self.build_advanced_search_filter(&request)?;

        // Calculate pagination parameters
        let (limit, offset) = self.calculate_pagination(&request);

        // Perform vector search with pagination
        let search_points = SearchPoints {
            collection_name: self.collection_name.clone(),
            vector: query_embedding,
            filter,
            limit: (limit * 2) as u64, // Get more results for better filtering
            with_vectors: Some(false.into()),
            with_payload: Some(true.into()),
            params: None,
            score_threshold: Some(request.min_similarity),
            offset: Some(offset as u64),
            vector_name: None,
            read_consistency: None,
            timeout: Some(30),
            shard_key_selector: None,
            sparse_indices: None,
        };

        let search_result = self.client
            .search_points(search_points)
            .await
            .map_err(|e| SearchServiceError::VectorDatabaseError(format!("Search failed: {}", e)))?;

        // Convert and filter results
        let mut results = Vec::new();
        for (rank, scored_point) in search_result.result.into_iter().enumerate() {
            if let Ok(search_result) = self.convert_scored_point_to_search_result(scored_point, rank + 1) {
                // Apply additional filtering that couldn't be done at vector level
                if self.passes_advanced_filters(&search_result, &request) {
                    results.push(search_result);
                }
            }
        }

        // Apply sorting
        self.sort_results(&mut results, &request);

        // Apply pagination to final results
        let total_results = results.len();
        let paginated_results = self.apply_pagination(results, &request);

        // Generate pagination info
        let pagination = self.generate_pagination_info(&request, total_results);

        // Generate query suggestions if requested
        let suggestions = if request.include_suggestions {
            Some(self.generate_query_suggestions(&request.query, &processed_query).await?)
        } else {
            None
        };

        let search_time_ms = start_time.elapsed().as_millis() as u64;

        let response = SemanticSearchResponse {
            results: paginated_results,
            total_results,
            search_time_ms,
            query_embedding_time_ms,
            search_metadata: SearchMetadata {
                query_processed: processed_query.normalized,
                query_keywords: processed_query.keywords,
                content_types_searched: request.content_types.unwrap_or_default(),
                filters_applied: request.filters.is_some(),
                embedding_model: self.config.embedding_model.clone(),
            },
            pagination,
            suggestions,
            from_cache: false,
            cache_key: Some(cache_key.clone()),
        };

        // Cache the result if caching is enabled
        if request.enable_caching {
            self.cache_result(cache_key, &response).await;
        }

        Ok(response)
    }

    /// Find similar documents to a given document
    pub async fn find_similar_documents(
        &self,
        document_id: &str,
        limit: usize,
        content_types: Option<Vec<String>>,
    ) -> Result<Vec<SearchResult>> {
        // First, get the document's embedding
        let _point_id = self.generate_point_id(document_id);
        
        // For now, we'll use a mock implementation
        // In production, this would retrieve the actual document and use its embedding
        let mock_embedding = self.generate_mock_query_embedding(document_id);

        let filter = if let Some(types) = content_types {
            Some(self.build_content_type_filter(&types)?)
        } else {
            None
        };

        let search_points = SearchPoints {
            collection_name: self.collection_name.clone(),
            vector: mock_embedding,
            filter,
            limit: (limit + 1) as u64, // +1 to exclude the original document
            with_vectors: Some(false.into()),
            with_payload: Some(true.into()),
            params: None,
            score_threshold: Some(0.3),
            offset: None,
            vector_name: None,
            read_consistency: None,
            timeout: Some(30),
            shard_key_selector: None,
            sparse_indices: None,
        };

        let search_result = self.client
            .search_points(search_points)
            .await
            .map_err(|e| SearchServiceError::VectorDatabaseError(format!("Similar search failed: {}", e)))?;

        let mut results = Vec::new();
        for (rank, scored_point) in search_result.result.into_iter().enumerate() {
            if let Ok(search_result) = self.convert_scored_point_to_search_result(scored_point, rank + 1) {
                // Skip the original document
                if search_result.document.id != document_id {
                    results.push(search_result);
                }
            }
        }

        // Limit to requested number
        results.truncate(limit);

        Ok(results)
    }

    /// Get collection statistics
    pub async fn get_collection_stats(&self) -> Result<IndexStats> {
        let collection_info = self.client
            .collection_info(&self.collection_name)
            .await
            .map_err(|e| SearchServiceError::VectorDatabaseError(format!("Failed to get collection info: {}", e)))?;

        // For detailed statistics, we would need to scroll through all points
        // This is a simplified version
        let total_documents = collection_info.result
            .as_ref()
            .and_then(|info| info.points_count)
            .unwrap_or(0);

        Ok(IndexStats {
            total_documents,
            documents_by_type: HashMap::new(), // Would need to aggregate from all points
            documents_by_language: HashMap::new(), // Would need to aggregate from all points
            index_size_mb: 0.0, // Would need to calculate from collection info
            last_updated: chrono::Utc::now(),
            embedding_model: self.config.embedding_model.clone(),
            vector_dimensions: self.config.vector_size,
        })
    }

    /// Delete a document from the index
    pub async fn delete_document(&mut self, document_id: &str) -> Result<()> {
        let point_id = self.generate_point_id(document_id);
        
        let delete_request = DeletePointsBuilder::new(self.collection_name.clone())
            .points(vec![PointId::from(point_id)]);
        
        self.client
            .delete_points(delete_request)
            .await
            .map_err(|e| SearchServiceError::VectorDatabaseError(format!("Failed to delete document: {}", e)))?;

        debug!("Successfully deleted document: {}", document_id);
        Ok(())
    }

    /// Clear all documents from the collection
    pub async fn clear_collection(&mut self) -> Result<()> {
        // Delete the collection and recreate it
        let _ = self.client.delete_collection(&self.collection_name).await;
        self.create_collection().await?;
        
        info!("Successfully cleared collection: {}", self.collection_name);
        Ok(())
    }

    /// Initialize semantic knowledge bases (synonyms, concepts, roots)
    async fn initialize_semantic_knowledge(&mut self) -> Result<()> {
        info!("Initializing semantic knowledge bases...");
        
        // Initialize Arabic synonyms map
        self.synonym_map = self.build_arabic_synonyms_map();
        
        // Initialize Islamic concepts map
        self.concept_map = self.build_islamic_concepts_map();
        
        // Initialize Arabic root-to-words mapping
        self.root_to_words = self.build_arabic_roots_map();
        
        info!("Semantic knowledge bases initialized with {} synonyms, {} concepts, {} roots", 
              self.synonym_map.len(), self.concept_map.len(), self.root_to_words.len());
        
        Ok(())
    }

    /// Perform contextual semantic search with enhanced understanding
    pub async fn contextual_search(&self, request: ContextualSearchRequest) -> Result<Vec<ContextualSearchResult>> {
        let start_time = Instant::now();
        
        // Step 1: Understand the query
        let query_understanding = self.understand_query(&request.query, request.context.as_deref()).await?;
        
        // Step 2: Generate search variants based on understanding
        let search_variants = self.generate_search_variants(&query_understanding, &request).await?;
        
        // Step 3: Perform multiple searches and combine results
        let mut all_results = Vec::new();
        
        for (variant, weight) in search_variants {
            let variant_request = SemanticSearchRequest {
                query: variant,
                limit: request.limit * 2, // Get more results for merging
                content_types: request.content_types.clone(),
                min_similarity: request.min_similarity * 0.8, // Lower threshold for variants
                include_metadata: true,
                filters: request.filters.clone(),
                offset: None,
                page: None,
                page_size: None,
                include_suggestions: false,
                enable_caching: false,
                sort_by: None,
                sort_direction: None,
            };
            
            match self.search(variant_request).await {
                Ok(response) => {
                    for result in response.results {
                        all_results.push((result, weight));
                    }
                }
                Err(e) => {
                    warn!("Search variant failed: {}", e);
                }
            }
        }
        
        // Step 4: Merge and rank results
        let contextual_results = self.merge_and_rank_contextual_results(
            all_results, 
            &query_understanding, 
            &request
        ).await?;
        
        let search_time = start_time.elapsed();
        info!("Contextual search completed in {:?} with {} results", search_time, contextual_results.len());
        
        Ok(contextual_results)
    }

    /// Understand the query intent and extract semantic information
    pub async fn understand_query(&self, query: &str, context: Option<&str>) -> Result<QueryUnderstanding> {
        let processed = self.text_processor.process_text(query)?;
        
        // Extract concepts from the query
        let concepts = self.extract_concepts(&processed.normalized);
        
        // Identify Arabic roots
        let roots = self.text_processor.extract_arabic_roots(&processed.normalized)?;
        
        // Find synonyms for key terms
        let synonyms = self.find_synonyms(&processed.keywords);
        
        // Find related terms through concept mapping
        let related_terms = self.find_related_terms(&concepts);
        
        // Determine query intent
        let intent = self.classify_query_intent(&processed.normalized, context);
        
        // Calculate confidence based on various factors
        let confidence = self.calculate_understanding_confidence(&processed, &concepts, &roots);
        
        Ok(QueryUnderstanding {
            original_query: query.to_string(),
            normalized_query: processed.normalized,
            query_keywords: processed.keywords,
            detected_language: processed.language_detected,
            extracted_concepts: concepts,
            identified_roots: roots,
            synonyms,
            related_terms,
            query_intent: intent,
            confidence,
        })
    }

    /// Generate search variants based on query understanding
    pub async fn generate_search_variants(&self, understanding: &QueryUnderstanding, request: &ContextualSearchRequest) -> Result<Vec<(String, f32)>> {
        let mut variants = Vec::new();
        
        // Original query (highest weight)
        variants.push((understanding.normalized_query.clone(), 1.0));
        
        match request.search_mode {
            SearchMode::Exact => {
                // Only use the original query
            }
            SearchMode::Expanded => {
                // Add synonym variants
                if request.expand_synonyms {
                    for synonym_variant in self.create_synonym_variants(&understanding.normalized_query, &understanding.synonyms) {
                        variants.push((synonym_variant, 0.8));
                    }
                }
                
                // Add root-based variants
                if request.expand_roots {
                    for root_variant in self.create_root_variants(&understanding.identified_roots) {
                        variants.push((root_variant, 0.7));
                    }
                }
            }
            SearchMode::Conceptual => {
                // Add conceptual variants
                if request.expand_concepts {
                    for concept_variant in self.create_conceptual_variants(&understanding.extracted_concepts) {
                        variants.push((concept_variant, 0.9));
                    }
                }
                
                // Add related term variants
                for related_variant in self.create_related_term_variants(&understanding.related_terms) {
                    variants.push((related_variant, 0.6));
                }
            }
            SearchMode::RootBased => {
                // Focus on root-based expansion
                for root in &understanding.identified_roots {
                    if let Some(words) = self.root_to_words.get(root) {
                        for word in words.iter().take(5) { // Limit to top 5 words per root
                            variants.push((word.clone(), 0.8));
                        }
                    }
                }
            }
            SearchMode::Hybrid => {
                // Combine all approaches with balanced weights
                if request.expand_synonyms {
                    for synonym_variant in self.create_synonym_variants(&understanding.normalized_query, &understanding.synonyms) {
                        variants.push((synonym_variant, 0.7));
                    }
                }
                
                if request.expand_concepts {
                    for concept_variant in self.create_conceptual_variants(&understanding.extracted_concepts) {
                        variants.push((concept_variant, 0.8));
                    }
                }
                
                if request.expand_roots {
                    for root_variant in self.create_root_variants(&understanding.identified_roots) {
                        variants.push((root_variant, 0.6));
                    }
                }
            }
        }
        
        // Remove duplicates and sort by weight
        let mut unique_variants: HashMap<String, f32> = HashMap::new();
        for (variant, weight) in variants {
            unique_variants.entry(variant)
                .and_modify(|w: &mut f32| *w = w.max(weight))
                .or_insert(weight);
        }
        
        let mut result: Vec<_> = unique_variants.into_iter().collect();
        result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(result)
    }

    /// Merge and rank contextual search results
    async fn merge_and_rank_contextual_results(
        &self,
        results: Vec<(SearchResult, f32)>,
        understanding: &QueryUnderstanding,
        request: &ContextualSearchRequest,
    ) -> Result<Vec<ContextualSearchResult>> {
        let mut document_scores: HashMap<String, (SearchResult, f32, Vec<f32>)> = HashMap::new();
        
        // Aggregate scores for each document
        for (result, variant_weight) in results {
            let doc_id = result.document.id.clone();
            let weighted_score = result.similarity_score * variant_weight;
            
            document_scores.entry(doc_id)
                .and_modify(|(existing_result, best_score, scores)| {
                    if weighted_score > *best_score {
                        *existing_result = result.clone();
                        *best_score = weighted_score;
                    }
                    scores.push(weighted_score);
                })
                .or_insert((result, weighted_score, vec![weighted_score]));
        }
        
        // Convert to contextual results with enhanced scoring
        let mut contextual_results = Vec::new();
        
        for (_, (result, best_score, all_scores)) in document_scores {
            let contextual_score = self.calculate_contextual_score(&result.document, understanding, request);
            let combined_score = self.combine_scores(best_score, contextual_score, &all_scores);
            
            let match_type = self.determine_match_type(&result.document, understanding);
            let matched_terms = self.extract_matched_terms(&result.document, understanding);
            let expanded_terms = self.extract_expanded_terms(&result.document, understanding);
            
            contextual_results.push(ContextualSearchResult {
                document: result.document,
                similarity_score: result.similarity_score,
                contextual_score,
                combined_score,
                rank: 0, // Will be set after sorting
                match_type: match_type.clone(),
                matched_terms: matched_terms.clone(),
                expanded_terms: expanded_terms.clone(),
                highlighted_text: result.highlighted_text,
                explanation: self.generate_match_explanation(&match_type, &matched_terms, &expanded_terms),
            });
        }
        
        // Sort by combined score
        contextual_results.sort_by(|a, b| b.combined_score.partial_cmp(&a.combined_score).unwrap_or(std::cmp::Ordering::Equal));
        
        // Set ranks and limit results
        for (i, result) in contextual_results.iter_mut().enumerate() {
            result.rank = i + 1;
        }
        
        contextual_results.truncate(request.limit);
        
        Ok(contextual_results)
    }

    /// Search by Arabic linguistic roots with semantic understanding
    pub async fn search_by_roots(&self, roots: Vec<String>, limit: usize, content_types: Option<Vec<String>>) -> Result<Vec<ContextualSearchResult>> {
        let mut all_terms = Vec::new();
        
        // Expand roots to related words
        for root in &roots {
            if let Some(words) = self.root_to_words.get(root) {
                all_terms.extend(words.iter().cloned());
            }
            // Also add the root itself
            all_terms.push(root.clone());
        }
        
        if all_terms.is_empty() {
            return Ok(Vec::new());
        }
        
        // Create a search query from expanded terms
        let expanded_query = all_terms.join(" ");
        
        let request = ContextualSearchRequest {
            query: expanded_query,
            context: None,
            search_mode: SearchMode::RootBased,
            expand_synonyms: false,
            expand_roots: true,
            expand_concepts: false,
            limit,
            content_types,
            min_similarity: 0.3, // Lower threshold for root-based search
            filters: None,
        };
        
        self.contextual_search(request).await
    }

    /// Search for synonyms and related concepts
    pub async fn search_synonyms_and_concepts(&self, term: &str, limit: usize) -> Result<Vec<ContextualSearchResult>> {
        let mut expanded_terms = vec![term.to_string()];
        
        // Add synonyms
        if let Some(synonyms) = self.synonym_map.get(term) {
            expanded_terms.extend(synonyms.iter().cloned());
        }
        
        // Add related concepts
        if let Some(concepts) = self.concept_map.get(term) {
            expanded_terms.extend(concepts.iter().cloned());
        }
        
        let expanded_query = expanded_terms.join(" ");
        
        let request = ContextualSearchRequest {
            query: expanded_query,
            context: None,
            search_mode: SearchMode::Conceptual,
            expand_synonyms: true,
            expand_roots: false,
            expand_concepts: true,
            limit,
            content_types: None,
            min_similarity: 0.4,
            filters: None,
        };
        
        self.contextual_search(request).await
    }

    // Helper methods for advanced search functionality

    /// Generate cache key for a search request (public for testing)
    pub fn generate_cache_key(&self, request: &SemanticSearchRequest) -> String {
        let mut hasher = Sha256::new();
        
        // Hash the query
        hasher.update(request.query.as_bytes());
        
        // Hash filters if present
        if let Some(filters) = &request.filters {
            if let Ok(filters_json) = serde_json::to_string(filters) {
                hasher.update(filters_json.as_bytes());
            }
        }
        
        // Hash other parameters
        hasher.update(request.limit.to_string().as_bytes());
        hasher.update(request.min_similarity.to_string().as_bytes());
        
        if let Some(content_types) = &request.content_types {
            hasher.update(content_types.join(",").as_bytes());
        }
        
        if let Some(page) = request.page {
            hasher.update(page.to_string().as_bytes());
        }
        
        if let Some(page_size) = request.page_size {
            hasher.update(page_size.to_string().as_bytes());
        }
        
        format!("search:{:x}", hasher.finalize())
    }

    /// Get cached result if available and not expired
    async fn get_cached_result(&self, cache_key: &str) -> Option<SemanticSearchResponse> {
        let cache = self.query_cache.read().await;
        if let Some(cached) = cache.get(cache_key) {
            // Check if cache is still valid (5 minutes TTL)
            if cached.cached_at.elapsed().as_secs() < 300 {
                let mut response = cached.response.clone();
                response.from_cache = true;
                return Some(response);
            }
        }
        None
    }

    /// Cache search result
    async fn cache_result(&self, cache_key: String, response: &SemanticSearchResponse) {
        let mut cache = self.query_cache.write().await;
        
        // Limit cache size to prevent memory issues
        if cache.len() >= 1000 {
            // Remove oldest entries
            let mut entries: Vec<_> = cache.iter().map(|(k, v)| (k.clone(), v.cached_at)).collect();
            entries.sort_by_key(|(_, time)| *time);
            
            // Remove oldest 10% of entries
            let remove_count = cache.len() / 10;
            for (key, _) in entries.iter().take(remove_count) {
                cache.remove(key);
            }
        }
        
        cache.insert(cache_key, CachedQueryResult {
            response: response.clone(),
            cached_at: Instant::now(),
            access_count: 1,
            last_accessed: Instant::now(),
        });
    }

    /// Build advanced search filter with all filter types (public for testing)
    pub fn build_advanced_search_filter(&self, request: &SemanticSearchRequest) -> Result<Option<Filter>> {
        let mut conditions = Vec::new();

        // Content type filter
        if let Some(content_types) = &request.content_types {
            conditions.push(self.build_content_type_condition(content_types)?);
        }

        // Apply additional filters if present
        if let Some(filters) = &request.filters {
            // Content types from filters
            if let Some(content_types) = &filters.content_types {
                let content_type_strings: Vec<String> = content_types.iter()
                    .map(|ct| ct.as_str().to_string())
                    .collect();
                conditions.push(self.build_content_type_condition(&content_type_strings)?);
            }

            // Source filter
            if let Some(sources) = &filters.source {
                conditions.push(self.build_field_condition("source", sources)?);
            }

            // Author filter
            if let Some(authors) = &filters.author {
                conditions.push(self.build_field_condition("author", authors)?);
            }

            // Language filter
            if let Some(language) = &filters.language {
                conditions.push(self.build_field_condition("language", &[format!("{:?}", language)])?);
            }

            // Priority range filter
            if let Some(priority_range) = &filters.priority_range {
                conditions.push(self.build_u8_range_condition("priority", priority_range)?);
            }

            // Text length range filter
            if let Some(text_length_range) = &filters.text_length_range {
                conditions.push(self.build_usize_range_condition("text_length", text_length_range)?);
            }

            // Date range filter
            if let Some(date_range) = &filters.date_range {
                if let Some(from) = date_range.from {
                    conditions.push(self.build_date_condition("created_at", from.timestamp(), true)?);
                }
                if let Some(to) = date_range.to {
                    conditions.push(self.build_date_condition("created_at", to.timestamp(), false)?);
                }
            }
        }

        if conditions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Filter {
                should: vec![],
                must: conditions,
                must_not: vec![],
                min_should: None,
            }))
        }
    }

    /// Calculate pagination parameters (public for testing)
    pub fn calculate_pagination(&self, request: &SemanticSearchRequest) -> (usize, usize) {
        if let (Some(page), Some(page_size)) = (request.page, request.page_size) {
            let limit = page_size;
            let offset = (page.saturating_sub(1)) * page_size;
            (limit, offset)
        } else if let Some(offset) = request.offset {
            (request.limit, offset)
        } else {
            (request.limit, 0)
        }
    }

    /// Check if result passes advanced filters that couldn't be applied at vector level (public for testing)
    pub fn passes_advanced_filters(&self, result: &SearchResult, request: &SemanticSearchRequest) -> bool {
        if let Some(filters) = &request.filters {
            // Authenticity grade filter for hadith
            if let Some(grades) = &filters.authenticity_grades {
                if let Some(grade_str) = result.document.metadata.get("authenticity_grade") {
                    if let Ok(grade) = serde_json::from_value::<AuthenticityGrade>(grade_str.clone()) {
                        if !grades.contains(&grade) {
                            return false;
                        }
                    }
                }
            }

            // Similarity range filter
            if let Some(min_sim) = filters.min_similarity {
                if result.similarity_score < min_sim {
                    return false;
                }
            }
            if let Some(max_sim) = filters.max_similarity {
                if result.similarity_score > max_sim {
                    return false;
                }
            }

            // Text length filter
            if let Some(text_range) = &filters.text_length_range {
                let text_length = result.document.text.len();
                if let Some(min_len) = text_range.min {
                    if text_length < min_len {
                        return false;
                    }
                }
                if let Some(max_len) = text_range.max {
                    if text_length > max_len {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Sort results based on request parameters (public for testing)
    pub fn sort_results(&self, results: &mut Vec<SearchResult>, request: &SemanticSearchRequest) {
        let sort_by = request.sort_by.as_ref().unwrap_or(&SortBy::Similarity);
        let sort_direction = request.sort_direction.as_ref().unwrap_or(&SortDirection::Desc);

        results.sort_by(|a, b| {
            let ordering = match sort_by {
                SortBy::Similarity => a.similarity_score.partial_cmp(&b.similarity_score).unwrap_or(std::cmp::Ordering::Equal),
                SortBy::Priority => a.document.content_type.priority().cmp(&b.document.content_type.priority()),
                SortBy::CreatedAt => {
                    match (&a.document.created_at, &b.document.created_at) {
                        (Some(a_date), Some(b_date)) => a_date.cmp(b_date),
                        (Some(_), None) => std::cmp::Ordering::Greater,
                        (None, Some(_)) => std::cmp::Ordering::Less,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                },
                SortBy::UpdatedAt => {
                    match (&a.document.updated_at, &b.document.updated_at) {
                        (Some(a_date), Some(b_date)) => a_date.cmp(b_date),
                        (Some(_), None) => std::cmp::Ordering::Greater,
                        (None, Some(_)) => std::cmp::Ordering::Less,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                },
                SortBy::TextLength => a.document.text.len().cmp(&b.document.text.len()),
                SortBy::Relevance => {
                    // Combined score: similarity + priority
                    let a_relevance = a.similarity_score + (1.0 / a.document.content_type.priority() as f32);
                    let b_relevance = b.similarity_score + (1.0 / b.document.content_type.priority() as f32);
                    a_relevance.partial_cmp(&b_relevance).unwrap_or(std::cmp::Ordering::Equal)
                },
            };

            match sort_direction {
                SortDirection::Asc => ordering,
                SortDirection::Desc => ordering.reverse(),
            }
        });

        // Update ranks after sorting
        for (i, result) in results.iter_mut().enumerate() {
            result.rank = i + 1;
        }
    }

    /// Apply pagination to results
    fn apply_pagination(&self, mut results: Vec<SearchResult>, request: &SemanticSearchRequest) -> Vec<SearchResult> {
        let (limit, offset) = self.calculate_pagination(request);
        
        if offset >= results.len() {
            return Vec::new();
        }
        
        let end = (offset + limit).min(results.len());
        results.drain(offset..end).collect()
    }

    /// Generate pagination information
    fn generate_pagination_info(&self, request: &SemanticSearchRequest, total_results: usize) -> Option<PaginationInfo> {
        if let (Some(page), Some(page_size)) = (request.page, request.page_size) {
            let total_pages = (total_results + page_size - 1) / page_size; // Ceiling division
            
            Some(PaginationInfo {
                current_page: page,
                total_pages,
                page_size,
                total_items: total_results,
                has_next_page: page < total_pages,
                has_previous_page: page > 1,
                next_page: if page < total_pages { Some(page + 1) } else { None },
                previous_page: if page > 1 { Some(page - 1) } else { None },
            })
        } else {
            None
        }
    }

    /// Generate query suggestions based on semantic similarity
    pub async fn generate_query_suggestions(&self, original_query: &str, processed: &ProcessedText) -> Result<Vec<QuerySuggestion>> {
        let mut suggestions = Vec::new();

        // Check cache first
        if let Some(cached_suggestions) = self.get_cached_suggestions(original_query).await {
            return Ok(cached_suggestions);
        }

        // Generate synonym-based suggestions
        for keyword in &processed.keywords {
            if let Some(synonyms) = self.synonym_map.get(keyword) {
                for synonym in synonyms.iter().take(2) { // Limit to 2 synonyms per keyword
                    let suggested_query = original_query.replace(keyword, synonym);
                    if suggested_query != original_query {
                        suggestions.push(QuerySuggestion {
                            suggested_query,
                            similarity_score: 0.8,
                            expected_results_count: 0, // Would be calculated in production
                            suggestion_type: SuggestionType::Synonym,
                            explanation: Some(format!("استبدال '{}' بـ '{}'", keyword, synonym)),
                        });
                    }
                }
            }
        }

        // Generate concept-based suggestions
        let concepts = self.extract_concepts(&processed.normalized);
        for concept in concepts.iter().take(3) { // Limit to 3 concepts
            if let Some(related_terms) = self.concept_map.get(concept) {
                for term in related_terms.iter().take(2) { // Limit to 2 terms per concept
                    suggestions.push(QuerySuggestion {
                        suggested_query: format!("{} {}", original_query, term),
                        similarity_score: 0.7,
                        expected_results_count: 0,
                        suggestion_type: SuggestionType::Conceptual,
                        explanation: Some(format!("إضافة مصطلح مفاهيمي: '{}'", term)),
                    });
                }
            }
        }

        // Generate morphological (root-based) suggestions
        if let Ok(roots) = self.text_processor.extract_arabic_roots(&processed.normalized) {
            for root in roots.iter().take(2) { // Limit to 2 roots
                if let Some(words) = self.root_to_words.get(root) {
                    for word in words.iter().take(2) { // Limit to 2 words per root
                        if !processed.keywords.contains(word) {
                            suggestions.push(QuerySuggestion {
                                suggested_query: format!("{} {}", original_query, word),
                                similarity_score: 0.6,
                                expected_results_count: 0,
                                suggestion_type: SuggestionType::Morphological,
                                explanation: Some(format!("كلمة من نفس الجذر: '{}'", word)),
                            });
                        }
                    }
                }
            }
        }

        // Sort suggestions by similarity score
        suggestions.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap_or(std::cmp::Ordering::Equal));
        
        // Limit total suggestions
        suggestions.truncate(10);

        // Cache the suggestions
        self.cache_suggestions(original_query.to_string(), &suggestions).await;

        Ok(suggestions)
    }

    /// Get cached suggestions
    async fn get_cached_suggestions(&self, query: &str) -> Option<Vec<QuerySuggestion>> {
        let cache = self.suggestion_cache.read().await;
        cache.get(query).cloned()
    }

    /// Cache query suggestions
    async fn cache_suggestions(&self, query: String, suggestions: &[QuerySuggestion]) {
        let mut cache = self.suggestion_cache.write().await;
        
        // Limit cache size
        if cache.len() >= 500 {
            // Remove random entries to make space
            let keys_to_remove: Vec<_> = cache.keys().take(50).cloned().collect();
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }
        
        cache.insert(query, suggestions.to_vec());
    }

    /// Build field condition for filtering
    fn build_field_condition(&self, field: &str, values: &[String]) -> Result<Condition> {
        Ok(Condition {
            condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(
                FieldCondition {
                    key: field.to_string(),
                    r#match: Some(Match {
                        match_value: Some(qdrant_client::qdrant::r#match::MatchValue::Keywords(
                            qdrant_client::qdrant::RepeatedStrings {
                                strings: values.to_vec(),
                            }
                        )),
                    }),
                    range: None,
                    geo_bounding_box: None,
                    geo_radius: None,
                    geo_polygon: None,
                    values_count: None,
                    datetime_range: None,
                    is_empty: None,
                    is_null: None,
                }
            )),
        })
    }

    /// Build range condition for numeric fields
    fn build_range_condition<T>(&self, field: &str, range: &RangeFilter<T>) -> Result<Condition>
    where
        T: Into<f64> + Clone,
    {
        let mut range_condition = qdrant_client::qdrant::Range::default();
        
        if let Some(min) = &range.min {
            range_condition.gte = Some(min.clone().into());
        }
        
        if let Some(max) = &range.max {
            range_condition.lte = Some(max.clone().into());
        }

        Ok(Condition {
            condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(
                FieldCondition {
                    key: field.to_string(),
                    r#match: None,
                    range: Some(range_condition),
                    geo_bounding_box: None,
                    geo_radius: None,
                    geo_polygon: None,
                    values_count: None,
                    datetime_range: None,
                    is_empty: None,
                    is_null: None,
                }
            )),
        })
    }

    /// Build range condition for usize fields
    fn build_usize_range_condition(&self, field: &str, range: &RangeFilter<usize>) -> Result<Condition> {
        let mut range_condition = qdrant_client::qdrant::Range::default();
        
        if let Some(min) = range.min {
            range_condition.gte = Some(min as f64);
        }
        
        if let Some(max) = range.max {
            range_condition.lte = Some(max as f64);
        }

        Ok(Condition {
            condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(
                FieldCondition {
                    key: field.to_string(),
                    r#match: None,
                    range: Some(range_condition),
                    geo_bounding_box: None,
                    geo_radius: None,
                    geo_polygon: None,
                    values_count: None,
                    datetime_range: None,
                    is_empty: None,
                    is_null: None,
                }
            )),
        })
    }

    /// Build range condition for u8 fields
    fn build_u8_range_condition(&self, field: &str, range: &RangeFilter<u8>) -> Result<Condition> {
        let mut range_condition = qdrant_client::qdrant::Range::default();
        
        if let Some(min) = range.min {
            range_condition.gte = Some(min as f64);
        }
        
        if let Some(max) = range.max {
            range_condition.lte = Some(max as f64);
        }

        Ok(Condition {
            condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(
                FieldCondition {
                    key: field.to_string(),
                    r#match: None,
                    range: Some(range_condition),
                    geo_bounding_box: None,
                    geo_radius: None,
                    geo_polygon: None,
                    values_count: None,
                    datetime_range: None,
                    is_empty: None,
                    is_null: None,
                }
            )),
        })
    }

    /// Build date condition for timestamp fields
    fn build_date_condition(&self, field: &str, timestamp: i64, is_gte: bool) -> Result<Condition> {
        let mut range_condition = qdrant_client::qdrant::Range::default();
        
        if is_gte {
            range_condition.gte = Some(timestamp as f64);
        } else {
            range_condition.lte = Some(timestamp as f64);
        }

        Ok(Condition {
            condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(
                FieldCondition {
                    key: field.to_string(),
                    r#match: None,
                    range: Some(range_condition),
                    geo_bounding_box: None,
                    geo_radius: None,
                    geo_polygon: None,
                    values_count: None,
                    datetime_range: None,
                    is_empty: None,
                    is_null: None,
                }
            )),
        })
    }

    /// Build Arabic synonyms map for semantic expansion
    fn build_arabic_synonyms_map(&self) -> HashMap<String, Vec<String>> {
        let mut synonyms = HashMap::new();
        
        // Islamic terminology synonyms
        synonyms.insert("الله".to_string(), vec!["رب".to_string(), "الخالق".to_string(), "الرحمن".to_string(), "الرحيم".to_string()]);
        synonyms.insert("رسول".to_string(), vec!["نبي".to_string(), "رسول الله".to_string(), "النبي".to_string()]);
        synonyms.insert("قرآن".to_string(), vec!["كتاب الله".to_string(), "القرآن الكريم".to_string(), "المصحف".to_string(), "الذكر".to_string()]);
        synonyms.insert("صلاة".to_string(), vec!["صلوات".to_string(), "فريضة".to_string(), "عبادة".to_string()]);
        synonyms.insert("زكاة".to_string(), vec!["صدقة".to_string(), "زكاة المال".to_string(), "حق المال".to_string()]);
        synonyms.insert("حج".to_string(), vec!["حج البيت".to_string(), "الحج الأكبر".to_string(), "زيارة البيت".to_string()]);
        synonyms.insert("صوم".to_string(), vec!["صيام".to_string(), "إمساك".to_string(), "صوم رمضان".to_string()]);
        
        // Moral and ethical terms
        synonyms.insert("صبر".to_string(), vec!["تحمل".to_string(), "احتساب".to_string(), "ثبات".to_string()]);
        synonyms.insert("شكر".to_string(), vec!["حمد".to_string(), "امتنان".to_string(), "تقدير".to_string()]);
        synonyms.insert("توبة".to_string(), vec!["استغفار".to_string(), "إنابة".to_string(), "رجوع".to_string()]);
        synonyms.insert("عدل".to_string(), vec!["إنصاف".to_string(), "قسط".to_string(), "عدالة".to_string()]);
        synonyms.insert("رحمة".to_string(), vec!["شفقة".to_string(), "عطف".to_string(), "حنان".to_string()]);
        
        // Knowledge and wisdom terms
        synonyms.insert("علم".to_string(), vec!["معرفة".to_string(), "فقه".to_string(), "دراية".to_string()]);
        synonyms.insert("حكمة".to_string(), vec!["حكم".to_string(), "بصيرة".to_string(), "فطنة".to_string()]);
        synonyms.insert("فهم".to_string(), vec!["إدراك".to_string(), "وعي".to_string(), "استيعاب".to_string()]);
        
        // Worship and devotion terms
        synonyms.insert("عبادة".to_string(), vec!["طاعة".to_string(), "تقوى".to_string(), "خشوع".to_string()]);
        synonyms.insert("دعاء".to_string(), vec!["ابتهال".to_string(), "تضرع".to_string(), "استغاثة".to_string()]);
        synonyms.insert("ذكر".to_string(), vec!["تسبيح".to_string(), "تهليل".to_string(), "تكبير".to_string()]);
        
        // Community and social terms
        synonyms.insert("أمة".to_string(), vec!["مجتمع".to_string(), "جماعة".to_string(), "شعب".to_string()]);
        synonyms.insert("أخوة".to_string(), vec!["إخاء".to_string(), "تآخي".to_string(), "وحدة".to_string()]);
        synonyms.insert("عدالة".to_string(), vec!["إنصاف".to_string(), "حق".to_string(), "قسط".to_string()]);
        
        synonyms
    }

    /// Build Islamic concepts map for conceptual search
    fn build_islamic_concepts_map(&self) -> HashMap<String, Vec<String>> {
        let mut concepts = HashMap::new();
        
        // Faith and belief concepts
        concepts.insert("إيمان".to_string(), vec![
            "توحيد".to_string(), "عقيدة".to_string(), "يقين".to_string(), 
            "تصديق".to_string(), "اعتقاد".to_string(), "ثقة بالله".to_string()
        ]);
        
        // Worship concepts
        concepts.insert("عبادة".to_string(), vec![
            "صلاة".to_string(), "زكاة".to_string(), "صوم".to_string(), "حج".to_string(),
            "دعاء".to_string(), "ذكر".to_string(), "تلاوة".to_string(), "تسبيح".to_string()
        ]);
        
        // Moral excellence concepts
        concepts.insert("أخلاق".to_string(), vec![
            "صدق".to_string(), "أمانة".to_string(), "عدل".to_string(), "رحمة".to_string(),
            "صبر".to_string(), "شكر".to_string(), "تواضع".to_string(), "كرم".to_string()
        ]);
        
        // Knowledge and learning concepts
        concepts.insert("طلب العلم".to_string(), vec![
            "تعلم".to_string(), "تعليم".to_string(), "دراسة".to_string(), "بحث".to_string(),
            "فهم".to_string(), "حفظ".to_string(), "مراجعة".to_string(), "تدبر".to_string()
        ]);
        
        // Social justice concepts
        concepts.insert("عدالة اجتماعية".to_string(), vec![
            "مساواة".to_string(), "إنصاف".to_string(), "حقوق".to_string(), "واجبات".to_string(),
            "تكافل".to_string(), "تعاون".to_string(), "مساعدة".to_string(), "بر".to_string()
        ]);
        
        // Spiritual purification concepts
        concepts.insert("تزكية".to_string(), vec![
            "تطهير".to_string(), "تصفية".to_string(), "إصلاح".to_string(), "تهذيب".to_string(),
            "تربية".to_string(), "جهاد النفس".to_string(), "مراقبة".to_string(), "محاسبة".to_string()
        ]);
        
        // Family and relationships concepts
        concepts.insert("أسرة".to_string(), vec![
            "زواج".to_string(), "والدين".to_string(), "أطفال".to_string(), "تربية".to_string(),
            "بر الوالدين".to_string(), "صلة الرحم".to_string(), "حسن المعاشرة".to_string()
        ]);
        
        concepts
    }

    /// Build Arabic roots to words mapping
    fn build_arabic_roots_map(&self) -> HashMap<String, Vec<String>> {
        let mut roots = HashMap::new();
        
        // Root: ص-ل-ي (prayer related)
        roots.insert("صلي".to_string(), vec![
            "صلاة".to_string(), "صلى".to_string(), "يصلي".to_string(), "مصلى".to_string(),
            "صلوات".to_string(), "مصل".to_string(), "صالح".to_string()
        ]);
        
        // Root: ز-ك-ي (purification/charity)
        roots.insert("زكي".to_string(), vec![
            "زكاة".to_string(), "زكى".to_string(), "يزكي".to_string(), "تزكية".to_string(),
            "زكي".to_string(), "أزكى".to_string(), "مزكي".to_string()
        ]);
        
        // Root: ص-و-م (fasting)
        roots.insert("صوم".to_string(), vec![
            "صوم".to_string(), "صام".to_string(), "يصوم".to_string(), "صيام".to_string(),
            "صائم".to_string(), "مصوم".to_string(), "إفطار".to_string()
        ]);
        
        // Root: ح-ج-ج (pilgrimage)
        roots.insert("حجج".to_string(), vec![
            "حج".to_string(), "حج".to_string(), "يحج".to_string(), "حاج".to_string(),
            "حجة".to_string(), "حجيج".to_string(), "محج".to_string()
        ]);
        
        // Root: ع-ب-د (worship)
        roots.insert("عبد".to_string(), vec![
            "عبادة".to_string(), "عبد".to_string(), "يعبد".to_string(), "عابد".to_string(),
            "معبود".to_string(), "تعبد".to_string(), "عبودية".to_string()
        ]);
        
        // Root: ش-ك-ر (gratitude)
        roots.insert("شكر".to_string(), vec![
            "شكر".to_string(), "شكر".to_string(), "يشكر".to_string(), "شاكر".to_string(),
            "مشكور".to_string(), "شكور".to_string(), "شكران".to_string()
        ]);
        
        // Root: ص-ب-ر (patience)
        roots.insert("صبر".to_string(), vec![
            "صبر".to_string(), "صبر".to_string(), "يصبر".to_string(), "صابر".to_string(),
            "صبور".to_string(), "مصبور".to_string(), "اصطبار".to_string()
        ]);
        
        // Root: ع-ل-م (knowledge)
        roots.insert("علم".to_string(), vec![
            "علم".to_string(), "علم".to_string(), "يعلم".to_string(), "عالم".to_string(),
            "معلوم".to_string(), "تعلم".to_string(), "تعليم".to_string(), "علماء".to_string()
        ]);
        
        // Root: ح-ك-م (wisdom/judgment)
        roots.insert("حكم".to_string(), vec![
            "حكمة".to_string(), "حكم".to_string(), "يحكم".to_string(), "حاكم".to_string(),
            "محكوم".to_string(), "حكيم".to_string(), "أحكام".to_string()
        ]);
        
        // Root: ر-ح-م (mercy)
        roots.insert("رحم".to_string(), vec![
            "رحمة".to_string(), "رحم".to_string(), "يرحم".to_string(), "راحم".to_string(),
            "مرحوم".to_string(), "رحيم".to_string(), "رحمن".to_string()
        ]);
        
        roots
    }

    /// Extract concepts from text
    fn extract_concepts(&self, text: &str) -> Vec<String> {
        let mut concepts = Vec::new();
        let _words: Vec<&str> = text.split_whitespace().collect();
        
        // Look for concept keywords in the text
        for concept_key in self.concept_map.keys() {
            if text.contains(concept_key) {
                concepts.push(concept_key.clone());
            }
        }
        
        // Look for multi-word concepts
        let text_lower = text.to_lowercase();
        for (concept, related_terms) in &self.concept_map {
            for term in related_terms {
                if text_lower.contains(&term.to_lowercase()) {
                    if !concepts.contains(concept) {
                        concepts.push(concept.clone());
                    }
                }
            }
        }
        
        concepts
    }

    /// Find synonyms for given terms
    fn find_synonyms(&self, terms: &[String]) -> Vec<String> {
        let mut synonyms = Vec::new();
        
        for term in terms {
            if let Some(term_synonyms) = self.synonym_map.get(term) {
                synonyms.extend(term_synonyms.iter().cloned());
            }
        }
        
        // Remove duplicates
        synonyms.sort();
        synonyms.dedup();
        
        synonyms
    }

    /// Find related terms through concept mapping
    fn find_related_terms(&self, concepts: &[String]) -> Vec<String> {
        let mut related_terms = Vec::new();
        
        for concept in concepts {
            if let Some(terms) = self.concept_map.get(concept) {
                related_terms.extend(terms.iter().cloned());
            }
        }
        
        // Remove duplicates
        related_terms.sort();
        related_terms.dedup();
        
        related_terms
    }

    /// Classify query intent
    fn classify_query_intent(&self, query: &str, _context: Option<&str>) -> QueryIntent {
        let query_lower = query.to_lowercase();
        
        // Question patterns
        if query_lower.contains("ما") || query_lower.contains("من") || query_lower.contains("كيف") || 
           query_lower.contains("متى") || query_lower.contains("أين") || query_lower.contains("لماذا") {
            return QueryIntent::FactualQuestion;
        }
        
        // Definition patterns
        if query_lower.contains("تعريف") || query_lower.contains("معنى") || query_lower.contains("مفهوم") {
            return QueryIntent::DefinitionRequest;
        }
        
        // Comparison patterns
        if query_lower.contains("مقارنة") || query_lower.contains("فرق") || query_lower.contains("بين") {
            return QueryIntent::ComparativeAnalysis;
        }
        
        // Conceptual inquiry patterns
        if query_lower.contains("مفهوم") || query_lower.contains("فكرة") || query_lower.contains("نظرية") {
            return QueryIntent::ConceptualInquiry;
        }
        
        // Default to textual search
        QueryIntent::TextualSearch
    }

    /// Calculate understanding confidence
    fn calculate_understanding_confidence(&self, processed: &ProcessedText, concepts: &[String], roots: &[String]) -> f32 {
        let mut confidence = 0.5; // Base confidence
        
        // Boost confidence based on language detection
        if processed.language_detected.is_some() {
            confidence += 0.1;
        }
        
        // Boost confidence based on extracted concepts
        confidence += (concepts.len() as f32 * 0.1).min(0.3);
        
        // Boost confidence based on identified roots
        confidence += (roots.len() as f32 * 0.05).min(0.2);
        
        // Boost confidence based on keyword quality
        if processed.keywords.len() > 2 {
            confidence += 0.1;
        }
        
        confidence.min(1.0)
    }

    /// Create synonym variants of the query
    fn create_synonym_variants(&self, query: &str, synonyms: &[String]) -> Vec<String> {
        let mut variants = Vec::new();
        
        for synonym in synonyms {
            // Simple replacement strategy - could be more sophisticated
            let variant = query.replace(synonym, &format!("{} {}", synonym, query));
            if variant != query {
                variants.push(variant);
            }
        }
        
        variants
    }

    /// Create root-based variants
    fn create_root_variants(&self, roots: &[String]) -> Vec<String> {
        let mut variants = Vec::new();
        
        for root in roots {
            if let Some(words) = self.root_to_words.get(root) {
                // Create variants using different words from the same root
                for word in words.iter().take(3) { // Limit to 3 words per root
                    variants.push(word.clone());
                }
            }
        }
        
        variants
    }

    /// Create conceptual variants
    fn create_conceptual_variants(&self, concepts: &[String]) -> Vec<String> {
        let mut variants = Vec::new();
        
        for concept in concepts {
            if let Some(related_terms) = self.concept_map.get(concept) {
                // Create variants using related conceptual terms
                for term in related_terms.iter().take(3) { // Limit to 3 terms per concept
                    variants.push(term.clone());
                }
            }
        }
        
        variants
    }

    /// Create related term variants
    fn create_related_term_variants(&self, related_terms: &[String]) -> Vec<String> {
        related_terms.iter().take(5).cloned().collect() // Limit to 5 related terms
    }

    /// Calculate contextual score
    pub fn calculate_contextual_score(&self, document: &IslamicDocument, understanding: &QueryUnderstanding, _request: &ContextualSearchRequest) -> f32 {
        let mut score: f32 = 0.0;
        
        // Content type priority
        score += match document.content_type {
            ContentType::Quran => 0.3,
            ContentType::SahihHadith => 0.25,
            ContentType::HasanHadith => 0.2,
            ContentType::Tafsir => 0.15,
            _ => 0.1,
        };
        
        // Language match
        if let Some(detected_lang) = &understanding.detected_language {
            if document.language == *detected_lang {
                score += 0.1;
            }
        }
        
        // Concept relevance
        let doc_text_lower = document.text.to_lowercase();
        for concept in &understanding.extracted_concepts {
            if doc_text_lower.contains(&concept.to_lowercase()) {
                score += 0.05;
            }
        }
        
        // Root relevance
        for root in &understanding.identified_roots {
            if let Some(words) = self.root_to_words.get(root) {
                for word in words {
                    if doc_text_lower.contains(&word.to_lowercase()) {
                        score += 0.03;
                    }
                }
            }
        }
        
        score.min(1.0)
    }

    /// Combine different scores
    fn combine_scores(&self, _similarity_score: f32, contextual_score: f32, all_scores: &[f32]) -> f32 {
        let avg_similarity = all_scores.iter().sum::<f32>() / all_scores.len() as f32;
        let max_similarity = all_scores.iter().fold(0.0f32, |a, &b| a.max(b));
        
        // Weighted combination
        0.4 * max_similarity + 0.3 * avg_similarity + 0.3 * contextual_score
    }

    /// Determine match type
    pub fn determine_match_type(&self, document: &IslamicDocument, understanding: &QueryUnderstanding) -> MatchType {
        let doc_text_lower = document.text.to_lowercase();
        let query_lower = understanding.normalized_query.to_lowercase();
        
        // Check for direct match
        if doc_text_lower.contains(&query_lower) {
            return MatchType::DirectMatch;
        }
        
        // Check for synonym match
        for synonym in &understanding.synonyms {
            if doc_text_lower.contains(&synonym.to_lowercase()) {
                return MatchType::SynonymMatch;
            }
        }
        
        // Check for root match
        for root in &understanding.identified_roots {
            if let Some(words) = self.root_to_words.get(root) {
                for word in words {
                    if doc_text_lower.contains(&word.to_lowercase()) {
                        return MatchType::RootMatch;
                    }
                }
            }
        }
        
        // Check for conceptual match
        for concept in &understanding.extracted_concepts {
            if let Some(related_terms) = self.concept_map.get(concept) {
                for term in related_terms {
                    if doc_text_lower.contains(&term.to_lowercase()) {
                        return MatchType::ConceptualMatch;
                    }
                }
            }
        }
        
        MatchType::ContextualMatch
    }

    /// Extract matched terms
    fn extract_matched_terms(&self, document: &IslamicDocument, understanding: &QueryUnderstanding) -> Vec<String> {
        let mut matched_terms = Vec::new();
        let doc_text_lower = document.text.to_lowercase();
        
        // Check query keywords
        for keyword in &understanding.query_keywords {
            if doc_text_lower.contains(&keyword.to_lowercase()) {
                matched_terms.push(keyword.clone());
            }
        }
        
        matched_terms
    }

    /// Extract expanded terms that matched
    fn extract_expanded_terms(&self, document: &IslamicDocument, understanding: &QueryUnderstanding) -> Vec<String> {
        let mut expanded_terms = Vec::new();
        let doc_text_lower = document.text.to_lowercase();
        
        // Check synonyms
        for synonym in &understanding.synonyms {
            if doc_text_lower.contains(&synonym.to_lowercase()) {
                expanded_terms.push(synonym.clone());
            }
        }
        
        // Check related terms
        for term in &understanding.related_terms {
            if doc_text_lower.contains(&term.to_lowercase()) {
                expanded_terms.push(term.clone());
            }
        }
        
        expanded_terms
    }

    /// Generate match explanation
    fn generate_match_explanation(&self, match_type: &MatchType, matched_terms: &[String], expanded_terms: &[String]) -> Option<String> {
        match match_type {
            MatchType::DirectMatch => Some("تطابق مباشر مع النص".to_string()),
            MatchType::SynonymMatch => Some(format!("تطابق مع المرادفات: {}", matched_terms.join(", "))),
            MatchType::RootMatch => Some(format!("تطابق مع الجذور اللغوية: {}", matched_terms.join(", "))),
            MatchType::ConceptualMatch => Some(format!("تطابق مفاهيمي: {}", expanded_terms.join(", "))),
            MatchType::ContextualMatch => Some("تطابق سياقي".to_string()),
        }
    }

    // Existing helper methods

    fn create_point_struct(&self, document: &IslamicDocument, embedding: Vec<f32>) -> Result<PointStruct> {
        let point_id = self.generate_point_id(&document.id);
        
        let mut payload = HashMap::new();
        payload.insert("id".to_string(), Value::from(document.id.clone()));
        payload.insert("text".to_string(), Value::from(document.text.clone()));
        payload.insert("content_type".to_string(), Value::from(document.content_type.as_str()));
        payload.insert("source".to_string(), Value::from(document.source.clone()));
        payload.insert("language".to_string(), Value::from(format!("{:?}", document.language)));
        
        if let Some(author) = &document.author {
            payload.insert("author".to_string(), Value::from(author.clone()));
        }

        // Add metadata
        for (key, value) in &document.metadata {
            payload.insert(format!("metadata_{}", key), self.json_value_to_qdrant_value(value));
        }

        Ok(PointStruct::new(point_id, embedding, payload))
    }

    fn build_search_filter(&self, request: &SemanticSearchRequest) -> Result<Option<Filter>> {
        let mut conditions = Vec::new();

        // Content type filter
        if let Some(content_types) = &request.content_types {
            conditions.push(self.build_content_type_condition(content_types)?);
        }

        // Additional filters
        if let Some(filters) = &request.filters {
            if let Some(sources) = &filters.source {
                conditions.push(Condition {
                    condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(
                        FieldCondition {
                            key: "source".to_string(),
                            r#match: Some(Match {
                                match_value: Some(qdrant_client::qdrant::r#match::MatchValue::Keywords(
                                    qdrant_client::qdrant::RepeatedStrings {
                                        strings: sources.clone(),
                                    }
                                )),
                            }),
                            range: None,
                            geo_bounding_box: None,
                            geo_radius: None,
                            geo_polygon: None,
                            values_count: None,
                            datetime_range: None,
                            is_empty: None,
                            is_null: None,
                        }
                    )),
                });
            }

            if let Some(authors) = &filters.author {
                conditions.push(Condition {
                    condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(
                        FieldCondition {
                            key: "author".to_string(),
                            r#match: Some(Match {
                                match_value: Some(qdrant_client::qdrant::r#match::MatchValue::Keywords(
                                    qdrant_client::qdrant::RepeatedStrings {
                                        strings: authors.clone(),
                                    }
                                )),
                            }),
                            range: None,
                            geo_bounding_box: None,
                            geo_radius: None,
                            geo_polygon: None,
                            values_count: None,
                            datetime_range: None,
                            is_empty: None,
                            is_null: None,
                        }
                    )),
                });
            }

            if let Some(language) = &filters.language {
                conditions.push(Condition {
                    condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(
                        FieldCondition {
                            key: "language".to_string(),
                            r#match: Some(Match {
                                match_value: Some(qdrant_client::qdrant::r#match::MatchValue::Keyword(
                                    format!("{:?}", language)
                                )),
                            }),
                            range: None,
                            geo_bounding_box: None,
                            geo_radius: None,
                            geo_polygon: None,
                            values_count: None,
                            datetime_range: None,
                            is_empty: None,
                            is_null: None,
                        }
                    )),
                });
            }
        }

        if conditions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Filter {
                should: vec![],
                must: conditions,
                must_not: vec![],
                min_should: None,
            }))
        }
    }

    fn build_content_type_filter(&self, content_types: &[String]) -> Result<Filter> {
        let condition = self.build_content_type_condition(content_types)?;
        Ok(Filter {
            should: vec![],
            must: vec![condition],
            must_not: vec![],
            min_should: None,
        })
    }

    fn build_content_type_condition(&self, content_types: &[String]) -> Result<Condition> {
        Ok(Condition {
            condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(
                FieldCondition {
                    key: "content_type".to_string(),
                    r#match: Some(Match {
                        match_value: Some(qdrant_client::qdrant::r#match::MatchValue::Keywords(
                            qdrant_client::qdrant::RepeatedStrings {
                                strings: content_types.to_vec(),
                            }
                        )),
                    }),
                    range: None,
                    geo_bounding_box: None,
                    geo_radius: None,
                    geo_polygon: None,
                    values_count: None,
                    datetime_range: None,
                    is_empty: None,
                    is_null: None,
                }
            )),
        })
    }

    fn convert_scored_point_to_search_result(&self, scored_point: ScoredPoint, rank: usize) -> Result<SearchResult> {
        let payload = scored_point.payload;
        
        let id = payload.get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SearchServiceError::VectorDatabaseError("Missing document ID".to_string()))?
            .to_string();

        let text = payload.get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SearchServiceError::VectorDatabaseError("Missing document text".to_string()))?
            .to_string();

        let content_type_str = payload.get("content_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SearchServiceError::VectorDatabaseError("Missing content type".to_string()))?;

        let content_type = self.parse_content_type(content_type_str)?;

        let source = payload.get("source")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let author = payload.get("author")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let language = payload.get("language")
            .and_then(|v| v.as_str())
            .and_then(|s| self.parse_language(s))
            .unwrap_or(Language::Arabic);

        // Extract metadata
        let mut metadata = HashMap::new();
        for (key, value) in payload.iter() {
            if key.starts_with("metadata_") {
                let metadata_key = key.strip_prefix("metadata_").unwrap();
                metadata.insert(metadata_key.to_string(), self.qdrant_value_to_json_value(value));
            }
        }

        let document = IslamicDocument {
            id,
            text,
            content_type,
            source,
            author,
            language,
            metadata,
            created_at: None, // Could extract from payload if needed
            updated_at: None,
        };

        Ok(SearchResult {
            document,
            similarity_score: scored_point.score,
            rank,
            highlighted_text: None, // Could implement text highlighting
            explanation: None,
        })
    }

    fn parse_content_type(&self, content_type_str: &str) -> Result<ContentType> {
        match content_type_str {
            "quran" => Ok(ContentType::Quran),
            "sahih_hadith" => Ok(ContentType::SahihHadith),
            "hasan_hadith" => Ok(ContentType::HasanHadith),
            "daif_hadith" => Ok(ContentType::DaifHadith),
            "mawdu_hadith" => Ok(ContentType::MawduHadith),
            "tafsir" => Ok(ContentType::Tafsir),
            "fiqh_ruling" => Ok(ContentType::FiqhRuling),
            "scholar_opinion" => Ok(ContentType::ScholarOpinion),
            "islamic_story" => Ok(ContentType::IslamicStory),
            "dua" => Ok(ContentType::Dua),
            "dhikr" => Ok(ContentType::Dhikr),
            "biography" => Ok(ContentType::Biography),
            "history" => Ok(ContentType::History),
            _ => Err(SearchServiceError::VectorDatabaseError(format!("Unknown content type: {}", content_type_str))),
        }
    }

    fn parse_language(&self, language_str: &str) -> Option<Language> {
        match language_str {
            "Arabic" => Some(Language::Arabic),
            "English" => Some(Language::English),
            "French" => Some(Language::French),
            "Urdu" => Some(Language::Urdu),
            "Turkish" => Some(Language::Turkish),
            "Indonesian" => Some(Language::Indonesian),
            "Malay" => Some(Language::Malay),
            _ => None,
        }
    }

    fn json_value_to_qdrant_value(&self, value: &serde_json::Value) -> Value {
        match value {
            serde_json::Value::String(s) => Value::from(s.clone()),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::from(i)
                } else if let Some(f) = n.as_f64() {
                    Value::from(f)
                } else {
                    Value::from(n.to_string())
                }
            }
            serde_json::Value::Bool(b) => Value::from(*b),
            _ => Value::from(value.to_string()),
        }
    }

    fn qdrant_value_to_json_value(&self, value: &Value) -> serde_json::Value {
        match value {
            Value { kind: Some(qdrant_client::qdrant::value::Kind::StringValue(s)) } => {
                serde_json::Value::String(s.clone())
            }
            Value { kind: Some(qdrant_client::qdrant::value::Kind::IntegerValue(i)) } => {
                serde_json::Value::Number(serde_json::Number::from(*i))
            }
            Value { kind: Some(qdrant_client::qdrant::value::Kind::DoubleValue(f)) } => {
                serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap_or_else(|| serde_json::Number::from(0)))
            }
            Value { kind: Some(qdrant_client::qdrant::value::Kind::BoolValue(b)) } => {
                serde_json::Value::Bool(*b)
            }
            _ => serde_json::Value::Null,
        }
    }

    fn generate_point_id(&self, document_id: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        document_id.hash(&mut hasher);
        hasher.finish()
    }

    fn generate_mock_query_embedding(&self, text: &str) -> Vec<f32> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let seed = hasher.finish();

        let mut rng_state = seed;
        let mut embedding = Vec::with_capacity(self.config.vector_size);

        for i in 0..self.config.vector_size {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345 + i as u64);
            let normalized = (rng_state as f32) / (u64::MAX as f32);
            embedding.push((normalized - 0.5) * 2.0);
        }

        // Normalize to unit vector
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for value in &mut embedding {
                *value /= magnitude;
            }
        }

        embedding
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_semantic_search_engine_creation() {
        // This test would require a running Qdrant instance
        // In a real test environment, you would use a test container
    }

    #[tokio::test]
    async fn test_content_type_parsing() {
        // This test would require a running Qdrant instance
        // In a real test environment, you would use a test container
        // For now, just test the parsing logic directly
        let engine = SemanticSearchEngine {
            client: Qdrant::from_url("http://localhost:6333").build().unwrap(),
            config: SearchServiceConfig::default(),
            text_processor: ArabicTextProcessor::new().unwrap(),
            collection_name: "test".to_string(),
            synonym_map: HashMap::new(),
            concept_map: HashMap::new(),
            root_to_words: HashMap::new(),
            query_cache: Arc::new(RwLock::new(HashMap::new())),
            suggestion_cache: Arc::new(RwLock::new(HashMap::new())),
        };
        
        assert_eq!(engine.parse_content_type("quran").unwrap(), ContentType::Quran);
        assert_eq!(engine.parse_content_type("sahih_hadith").unwrap(), ContentType::SahihHadith);
        assert_eq!(engine.parse_content_type("tafsir").unwrap(), ContentType::Tafsir);
    }

    #[tokio::test]
    async fn test_point_id_generation() {
        let engine = SemanticSearchEngine {
            client: Qdrant::from_url("http://localhost:6333").build().unwrap(),
            config: SearchServiceConfig::default(),
            text_processor: ArabicTextProcessor::new().unwrap(),
            collection_name: "test".to_string(),
            synonym_map: HashMap::new(),
            concept_map: HashMap::new(),
            root_to_words: HashMap::new(),
            query_cache: Arc::new(RwLock::new(HashMap::new())),
            suggestion_cache: Arc::new(RwLock::new(HashMap::new())),
        };
        
        let id1 = engine.generate_point_id("doc1");
        let id2 = engine.generate_point_id("doc2");
        let id3 = engine.generate_point_id("doc1"); // Same as id1
        
        assert_ne!(id1, id2);
        assert_eq!(id1, id3);
    }
}
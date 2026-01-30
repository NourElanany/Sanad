use super::*;
use qdrant_client::{
    Qdrant,
    qdrant::{
        vectors_config::Config, CreateCollection, Distance, PointStruct, SearchPoints,
        VectorParams, VectorsConfig, Filter, Condition, FieldCondition, Match, Value,
        ScoredPoint, PointId, UpsertPointsBuilder, DeletePointsBuilder, ScrollPoints,
        CountPoints, GetPoints, UpdatePointVectorsBuilder,
    },
};
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, error, debug, warn};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Vector database client for Islamic content storage and retrieval
#[derive(Clone)]
pub struct VectorDatabaseClient {
    client: Qdrant,
    config: VectorDatabaseConfig,
    collection_name: String,
}

/// Configuration for Vector Database
#[derive(Debug, Clone)]
pub struct VectorDatabaseConfig {
    pub host: String,
    pub port: u16,
    pub collection_name: String,
    pub vector_size: usize,
    pub distance_metric: DistanceMetric,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub batch_size: usize,
}

/// Distance metrics supported by the vector database
#[derive(Debug, Clone)]
pub enum DistanceMetric {
    Cosine,
    Euclidean,
    Dot,
}

/// Search request for vector database
#[derive(Debug, Clone)]
pub struct VectorSearchRequest {
    pub query_vector: Vec<f32>,
    pub limit: usize,
    pub score_threshold: Option<f32>,
    pub filter: Option<VectorFilter>,
    pub with_payload: bool,
    pub with_vectors: bool,
}

/// Filter for vector search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorFilter {
    pub content_types: Option<Vec<String>>,
    pub sources: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub authenticity_levels: Option<Vec<String>>,
    pub date_range: Option<DateRange>,
    pub metadata_filters: Option<HashMap<String, String>>,
}

/// Date range filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

/// Search result from vector database
#[derive(Debug, Clone)]
pub struct VectorSearchResult {
    pub id: String,
    pub score: f32,
    pub payload: HashMap<String, serde_json::Value>,
    pub vector: Option<Vec<f32>>,
}

/// Document for indexing in vector database
#[derive(Debug, Clone)]
pub struct VectorDocument {
    pub id: String,
    pub vector: Vec<f32>,
    pub payload: VectorPayload,
}

/// Payload structure for vector documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorPayload {
    pub text: String,
    pub content_type: String,
    pub source: String,
    pub author: Option<String>,
    pub language: String,
    pub authenticity: String,
    pub reference: String,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub keywords: Vec<String>,
    pub concepts: Vec<String>,
    pub text_length: usize,
    pub word_count: usize,
}

/// Collection statistics
#[derive(Debug, Clone)]
pub struct CollectionStats {
    pub total_points: u64,
    pub indexed_points: u64,
    pub points_by_type: HashMap<String, u64>,
    pub points_by_language: HashMap<String, u64>,
    pub collection_size_bytes: u64,
    pub last_updated: DateTime<Utc>,
}

/// Batch operation result
#[derive(Debug, Clone)]
pub struct BatchOperationResult {
    pub successful_count: usize,
    pub failed_count: usize,
    pub errors: Vec<String>,
    pub processing_time_ms: u64,
}

impl Default for VectorDatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 6333,
            collection_name: "islamic_sources".to_string(),
            vector_size: 384,
            distance_metric: DistanceMetric::Cosine,
            timeout_seconds: 30,
            max_retries: 3,
            batch_size: 100,
        }
    }
}

impl VectorDatabaseClient {
    /// Create a new vector database client
    pub async fn new(config: VectorDatabaseConfig) -> Result<Self> {
        let url = format!("http://{}:{}", config.host, config.port);
        
        let client = Qdrant::from_url(&url)
            .build()
            .map_err(|e| AIServiceError::DatabaseError(format!("Failed to connect to Qdrant: {}", e)))?;

        let collection_name = config.collection_name.clone();

        let mut db_client = Self {
            client,
            config,
            collection_name,
        };

        // Ensure collection exists
        db_client.ensure_collection_exists().await?;

        info!("Vector database client initialized for collection: {}", db_client.collection_name);
        Ok(db_client)
    }

    /// Ensure the collection exists, create if it doesn't
    async fn ensure_collection_exists(&mut self) -> Result<()> {
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

    /// Create a new collection
    async fn create_collection(&mut self) -> Result<()> {
        let distance = match self.config.distance_metric {
            DistanceMetric::Cosine => Distance::Cosine,
            DistanceMetric::Euclidean => Distance::Euclid,
            DistanceMetric::Dot => Distance::Dot,
        };

        let create_collection = CreateCollection {
            collection_name: self.collection_name.clone(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: self.config.vector_size as u64,
                    distance: distance.into(),
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
            timeout: Some(self.config.timeout_seconds),
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
            .map_err(|e| AIServiceError::DatabaseError(format!("Failed to create collection: {}", e)))?;

        info!("Successfully created collection: {}", self.collection_name);
        Ok(())
    }

    /// Index a single document
    pub async fn index_document(&mut self, document: VectorDocument) -> Result<()> {
        let point_id = self.generate_point_id(&document.id);
        let payload = self.payload_to_qdrant_values(&document.payload)?;
        
        let point = PointStruct::new(point_id, document.vector, payload);
        let upsert_request = UpsertPointsBuilder::new(self.collection_name.clone(), vec![point]);

        self.client
            .upsert_points(upsert_request)
            .await
            .map_err(|e| AIServiceError::DatabaseError(format!("Failed to index document: {}", e)))?;

        debug!("Successfully indexed document: {}", document.id);
        Ok(())
    }

    /// Index multiple documents in batch
    pub async fn index_documents_batch(&mut self, documents: Vec<VectorDocument>) -> Result<BatchOperationResult> {
        let start_time = Instant::now();
        let mut successful_count = 0;
        let mut failed_count = 0;
        let mut errors = Vec::new();

        // Process in batches to avoid overwhelming the database
        for chunk in documents.chunks(self.config.batch_size) {
            let mut points = Vec::new();
            
            for document in chunk {
                match self.create_point_struct(document) {
                    Ok(point) => points.push(point),
                    Err(e) => {
                        failed_count += 1;
                        errors.push(format!("Failed to create point for document {}: {}", document.id, e));
                    }
                }
            }

            if !points.is_empty() {
                let upsert_request = UpsertPointsBuilder::new(self.collection_name.clone(), points.clone());
                
                match self.client.upsert_points(upsert_request).await {
                    Ok(_) => successful_count += points.len(),
                    Err(e) => {
                        failed_count += points.len();
                        errors.push(format!("Batch upsert failed: {}", e));
                    }
                }
            }
        }

        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        info!("Batch indexing completed: {} successful, {} failed", successful_count, failed_count);

        Ok(BatchOperationResult {
            successful_count,
            failed_count,
            errors,
            processing_time_ms,
        })
    }

    /// Search for similar vectors
    pub async fn search(&self, request: VectorSearchRequest) -> Result<Vec<VectorSearchResult>> {
        let filter = if let Some(vector_filter) = &request.filter {
            Some(self.build_qdrant_filter(vector_filter)?)
        } else {
            None
        };

        let search_points = SearchPoints {
            collection_name: self.collection_name.clone(),
            vector: request.query_vector,
            filter,
            limit: request.limit as u64,
            with_vectors: Some(request.with_vectors.into()),
            with_payload: Some(request.with_payload.into()),
            params: None,
            score_threshold: request.score_threshold,
            offset: None,
            vector_name: None,
            read_consistency: None,
            timeout: Some(self.config.timeout_seconds),
            shard_key_selector: None,
            sparse_indices: None,
        };

        let search_result = self.client
            .search_points(search_points)
            .await
            .map_err(|e| AIServiceError::DatabaseError(format!("Search failed: {}", e)))?;

        let mut results = Vec::new();
        for scored_point in search_result.result {
            let result = self.convert_scored_point_to_result(scored_point)?;
            results.push(result);
        }

        debug!("Search completed with {} results", results.len());
        Ok(results)
    }

    /// Get document by ID
    pub async fn get_document(&self, document_id: &str) -> Result<Option<VectorSearchResult>> {
        let point_id = self.generate_point_id(document_id);
        
        let get_points = GetPoints {
            collection_name: self.collection_name.clone(),
            ids: vec![PointId::from(point_id)],
            with_vectors: Some(true.into()),
            with_payload: Some(true.into()),
            read_consistency: None,
            shard_key_selector: None,
        };

        let response = self.client
            .get_points(get_points)
            .await
            .map_err(|e| AIServiceError::DatabaseError(format!("Failed to get document: {}", e)))?;

        if let Some(point) = response.result.into_iter().next() {
            let result = VectorSearchResult {
                id: document_id.to_string(),
                score: 1.0, // Perfect match for exact retrieval
                payload: self.qdrant_payload_to_json(&point.payload)?,
                vector: point.vectors.and_then(|v| match v.vectors_options {
                    Some(qdrant_client::qdrant::vectors::VectorsOptions::Vector(vector)) => Some(vector.data),
                    _ => None,
                }),
            };
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    /// Delete document by ID
    pub async fn delete_document(&mut self, document_id: &str) -> Result<()> {
        let point_id = self.generate_point_id(document_id);
        
        let delete_request = DeletePointsBuilder::new(self.collection_name.clone())
            .points(vec![PointId::from(point_id)]);
        
        self.client
            .delete_points(delete_request)
            .await
            .map_err(|e| AIServiceError::DatabaseError(format!("Failed to delete document: {}", e)))?;

        debug!("Successfully deleted document: {}", document_id);
        Ok(())
    }

    /// Update document vector
    pub async fn update_document_vector(&mut self, document_id: &str, new_vector: Vec<f32>) -> Result<()> {
        let point_id = self.generate_point_id(document_id);
        
        let update_request = UpdatePointVectorsBuilder::new(
            self.collection_name.clone(),
            vec![PointId::from(point_id)],
        ).vectors(vec![new_vector]);
        
        self.client
            .update_point_vectors(update_request)
            .await
            .map_err(|e| AIServiceError::DatabaseError(format!("Failed to update document vector: {}", e)))?;

        debug!("Successfully updated vector for document: {}", document_id);
        Ok(())
    }

    /// Get collection statistics
    pub async fn get_collection_stats(&self) -> Result<CollectionStats> {
        let collection_info = self.client
            .collection_info(&self.collection_name)
            .await
            .map_err(|e| AIServiceError::DatabaseError(format!("Failed to get collection info: {}", e)))?;

        let info = collection_info.result
            .ok_or_else(|| AIServiceError::DatabaseError("No collection info returned".to_string()))?;

        let total_points = info.points_count.unwrap_or(0);
        let indexed_points = info.indexed_vectors_count.unwrap_or(0);

        // Get detailed statistics by scrolling through points
        let (points_by_type, points_by_language) = self.get_detailed_stats().await?;

        Ok(CollectionStats {
            total_points,
            indexed_points,
            points_by_type,
            points_by_language,
            collection_size_bytes: 0, // Would need additional API call to get this
            last_updated: Utc::now(),
        })
    }

    /// Clear all documents from the collection
    pub async fn clear_collection(&mut self) -> Result<()> {
        // Delete the collection and recreate it
        let _ = self.client.delete_collection(&self.collection_name).await;
        self.create_collection().await?;
        
        info!("Successfully cleared collection: {}", self.collection_name);
        Ok(())
    }

    /// Count documents matching filter
    pub async fn count_documents(&self, filter: Option<VectorFilter>) -> Result<u64> {
        let qdrant_filter = if let Some(f) = filter {
            Some(self.build_qdrant_filter(&f)?)
        } else {
            None
        };

        let count_request = CountPoints {
            collection_name: self.collection_name.clone(),
            filter: qdrant_filter,
            exact: Some(false), // Use approximate count for better performance
        };

        let response = self.client
            .count(count_request)
            .await
            .map_err(|e| AIServiceError::DatabaseError(format!("Count failed: {}", e)))?;

        Ok(response.result.map(|r| r.count).unwrap_or(0))
    }

    /// Scroll through all documents (for batch processing)
    pub async fn scroll_documents(&self, limit: usize, offset: Option<PointId>) -> Result<(Vec<VectorSearchResult>, Option<PointId>)> {
        let scroll_request = ScrollPoints {
            collection_name: self.collection_name.clone(),
            filter: None,
            offset,
            limit: Some(limit as u32),
            with_vectors: Some(false.into()),
            with_payload: Some(true.into()),
            read_consistency: None,
        };

        let response = self.client
            .scroll(scroll_request)
            .await
            .map_err(|e| AIServiceError::DatabaseError(format!("Scroll failed: {}", e)))?;

        let mut results = Vec::new();
        for point in response.result {
            let result = VectorSearchResult {
                id: format!("{:?}", point.id), // Convert PointId to string
                score: 1.0,
                payload: self.qdrant_payload_to_json(&point.payload)?,
                vector: None, // Not requested in scroll
            };
            results.push(result);
        }

        Ok((results, response.next_page_offset))
    }

    // Helper methods

    /// Generate point ID from document ID
    fn generate_point_id(&self, document_id: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        document_id.hash(&mut hasher);
        hasher.finish()
    }

    /// Create point struct from vector document
    fn create_point_struct(&self, document: &VectorDocument) -> Result<PointStruct> {
        let point_id = self.generate_point_id(&document.id);
        let payload = self.payload_to_qdrant_values(&document.payload)?;
        Ok(PointStruct::new(point_id, document.vector.clone(), payload))
    }

    /// Convert payload to Qdrant values
    fn payload_to_qdrant_values(&self, payload: &VectorPayload) -> Result<HashMap<String, Value>> {
        let mut values = HashMap::new();
        
        values.insert("text".to_string(), Value::from(payload.text.clone()));
        values.insert("content_type".to_string(), Value::from(payload.content_type.clone()));
        values.insert("source".to_string(), Value::from(payload.source.clone()));
        values.insert("language".to_string(), Value::from(payload.language.clone()));
        values.insert("authenticity".to_string(), Value::from(payload.authenticity.clone()));
        values.insert("reference".to_string(), Value::from(payload.reference.clone()));
        values.insert("text_length".to_string(), Value::from(payload.text_length as i64));
        values.insert("word_count".to_string(), Value::from(payload.word_count as i64));
        
        if let Some(author) = &payload.author {
            values.insert("author".to_string(), Value::from(author.clone()));
        }
        
        if let Some(created_at) = payload.created_at {
            values.insert("created_at".to_string(), Value::from(created_at));
        }
        
        if let Some(updated_at) = payload.updated_at {
            values.insert("updated_at".to_string(), Value::from(updated_at));
        }
        
        // Add keywords as comma-separated string
        values.insert("keywords".to_string(), Value::from(payload.keywords.join(",")));
        values.insert("concepts".to_string(), Value::from(payload.concepts.join(",")));
        
        // Add metadata
        for (key, value) in &payload.metadata {
            values.insert(format!("metadata_{}", key), self.json_value_to_qdrant_value(value));
        }
        
        Ok(values)
    }

    /// Convert JSON value to Qdrant value
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

    /// Convert Qdrant payload to JSON
    fn qdrant_payload_to_json(&self, payload: &HashMap<String, Value>) -> Result<HashMap<String, serde_json::Value>> {
        let mut json_payload = HashMap::new();
        
        for (key, value) in payload {
            let json_value = match value {
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
            };
            json_payload.insert(key.clone(), json_value);
        }
        
        Ok(json_payload)
    }

    /// Build Qdrant filter from vector filter
    fn build_qdrant_filter(&self, filter: &VectorFilter) -> Result<Filter> {
        let mut conditions = Vec::new();

        // Content type filter
        if let Some(content_types) = &filter.content_types {
            conditions.push(self.build_field_condition("content_type", content_types)?);
        }

        // Source filter
        if let Some(sources) = &filter.sources {
            conditions.push(self.build_field_condition("source", sources)?);
        }

        // Author filter
        if let Some(authors) = &filter.authors {
            conditions.push(self.build_field_condition("author", authors)?);
        }

        // Language filter
        if let Some(languages) = &filter.languages {
            conditions.push(self.build_field_condition("language", languages)?);
        }

        // Authenticity filter
        if let Some(authenticity_levels) = &filter.authenticity_levels {
            conditions.push(self.build_field_condition("authenticity", authenticity_levels)?);
        }

        // Date range filter
        if let Some(date_range) = &filter.date_range {
            if let Some(from) = date_range.from {
                conditions.push(self.build_range_condition("created_at", from.timestamp() as f64, true)?);
            }
            if let Some(to) = date_range.to {
                conditions.push(self.build_range_condition("created_at", to.timestamp() as f64, false)?);
            }
        }

        // Metadata filters
        if let Some(metadata_filters) = &filter.metadata_filters {
            for (key, value) in metadata_filters {
                let field_name = format!("metadata_{}", key);
                conditions.push(self.build_field_condition(&field_name, &[value.clone()])?);
            }
        }

        Ok(Filter {
            should: vec![],
            must: conditions,
            must_not: vec![],
            min_should: None,
        })
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
    fn build_range_condition(&self, field: &str, value: f64, is_gte: bool) -> Result<Condition> {
        let mut range_condition = qdrant_client::qdrant::Range::default();
        
        if is_gte {
            range_condition.gte = Some(value);
        } else {
            range_condition.lte = Some(value);
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

    /// Convert scored point to search result
    fn convert_scored_point_to_result(&self, scored_point: ScoredPoint) -> Result<VectorSearchResult> {
        let id = scored_point.id
            .map(|id| format!("{:?}", id))
            .unwrap_or_else(|| "unknown".to_string());

        let payload = self.qdrant_payload_to_json(&scored_point.payload)?;
        
        let vector = scored_point.vectors.and_then(|v| match v.vectors_options {
            Some(qdrant_client::qdrant::vectors::VectorsOptions::Vector(vector)) => Some(vector.data),
            _ => None,
        });

        Ok(VectorSearchResult {
            id,
            score: scored_point.score,
            payload,
            vector,
        })
    }

    /// Get detailed statistics by content type and language
    async fn get_detailed_stats(&self) -> Result<(HashMap<String, u64>, HashMap<String, u64>)> {
        let mut points_by_type = HashMap::new();
        let mut points_by_language = HashMap::new();
        
        // This is a simplified implementation
        // In production, you might want to use aggregation queries or maintain separate counters
        let mut offset = None;
        let batch_size = 1000;
        
        loop {
            let (results, next_offset) = self.scroll_documents(batch_size, offset).await?;
            
            if results.is_empty() {
                break;
            }
            
            for result in results {
                // Count by content type
                if let Some(content_type) = result.payload.get("content_type") {
                    if let Some(ct_str) = content_type.as_str() {
                        *points_by_type.entry(ct_str.to_string()).or_insert(0) += 1;
                    }
                }
                
                // Count by language
                if let Some(language) = result.payload.get("language") {
                    if let Some(lang_str) = language.as_str() {
                        *points_by_language.entry(lang_str.to_string()).or_insert(0) += 1;
                    }
                }
            }
            
            offset = next_offset;
            if offset.is_none() {
                break;
            }
        }
        
        Ok((points_by_type, points_by_language))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vector_database_client_creation() {
        // This test would require a running Qdrant instance
        // In a real test environment, you would use a test container
        let config = VectorDatabaseConfig::default();
        
        // This would fail without a running Qdrant instance
        // let client = VectorDatabaseClient::new(config).await;
        // assert!(client.is_ok());
    }

    #[test]
    fn test_point_id_generation() {
        let config = VectorDatabaseConfig::default();
        let client = VectorDatabaseClient {
            client: Qdrant::from_url("http://localhost:6333").build().unwrap(),
            config,
            collection_name: "test".to_string(),
        };
        
        let id1 = client.generate_point_id("test_doc_1");
        let id2 = client.generate_point_id("test_doc_2");
        let id3 = client.generate_point_id("test_doc_1"); // Same as id1
        
        assert_ne!(id1, id2);
        assert_eq!(id1, id3);
    }

    #[test]
    fn test_payload_conversion() {
        let config = VectorDatabaseConfig::default();
        let client = VectorDatabaseClient {
            client: Qdrant::from_url("http://localhost:6333").build().unwrap(),
            config,
            collection_name: "test".to_string(),
        };
        
        let mut metadata = HashMap::new();
        metadata.insert("surah".to_string(), serde_json::Value::String("Al-Fatiha".to_string()));
        metadata.insert("ayah".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
        
        let payload = VectorPayload {
            text: "بسم الله الرحمن الرحيم".to_string(),
            content_type: "quran".to_string(),
            source: "Quran".to_string(),
            author: None,
            language: "Arabic".to_string(),
            authenticity: "Verified".to_string(),
            reference: "Al-Fatiha:1".to_string(),
            created_at: Some(1234567890),
            updated_at: None,
            metadata,
            keywords: vec!["بسم".to_string(), "الله".to_string()],
            concepts: vec!["basmala".to_string()],
            text_length: 22,
            word_count: 4,
        };
        
        let qdrant_values = client.payload_to_qdrant_values(&payload).unwrap();
        
        assert!(qdrant_values.contains_key("text"));
        assert!(qdrant_values.contains_key("content_type"));
        assert!(qdrant_values.contains_key("metadata_surah"));
        assert!(qdrant_values.contains_key("metadata_ayah"));
    }
}
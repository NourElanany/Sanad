use crate::models::*;
use crate::embedding_service::{EmbeddingService, BatchEmbeddingRequest};
use crate::semantic_search::SemanticSearchEngine;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tracing::{info, warn, error};
use chrono::Utc;

/// Service for indexing Islamic documents with semantic embeddings
pub struct IndexingService {
    embedding_service: Arc<Mutex<EmbeddingService>>,
    search_engine: Arc<Mutex<SemanticSearchEngine>>,
    config: SearchServiceConfig,
}

impl IndexingService {
    /// Create a new indexing service
    pub async fn new(
        embedding_service: Arc<EmbeddingService>,
        search_engine: Arc<SemanticSearchEngine>,
    ) -> Result<Self> {
        let config = SearchServiceConfig::default();
        
        Ok(Self {
            embedding_service: Arc::new(Mutex::new((*embedding_service).clone())),
            search_engine: Arc::new(Mutex::new((*search_engine).clone())),
            config,
        })
    }

    /// Index a single document
    pub async fn index_document(&self, document: IslamicDocument) -> Result<IndexingResult> {
        let start_time = Instant::now();
        let document_id = document.id.clone();

        info!("Starting indexing for document: {}", document_id);

        // Generate embedding
        let embedding = {
            let mut embedding_service = self.embedding_service.lock().await;
            match embedding_service.generate_embedding(&document.text).await {
                Ok(embedding) => embedding,
                Err(e) => {
                    error!("Failed to generate embedding for document {}: {}", document_id, e);
                    return Ok(IndexingResult {
                        document_id,
                        success: false,
                        embedding_generated: false,
                        indexed_at: Utc::now(),
                        processing_time_ms: start_time.elapsed().as_millis() as u64,
                        error: Some(format!("Embedding generation failed: {}", e)),
                    });
                }
            }
        };

        // Index in vector database
        let index_result = {
            let mut search_engine = self.search_engine.lock().await;
            search_engine.index_document(&document, embedding).await
        };

        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        match index_result {
            Ok(()) => {
                info!("Successfully indexed document: {} in {}ms", document_id, processing_time_ms);
                Ok(IndexingResult {
                    document_id,
                    success: true,
                    embedding_generated: true,
                    indexed_at: Utc::now(),
                    processing_time_ms,
                    error: None,
                })
            }
            Err(e) => {
                error!("Failed to index document {}: {}", document_id, e);
                Ok(IndexingResult {
                    document_id,
                    success: false,
                    embedding_generated: true,
                    indexed_at: Utc::now(),
                    processing_time_ms,
                    error: Some(format!("Vector indexing failed: {}", e)),
                })
            }
        }
    }

    /// Index multiple documents in batch
    pub async fn index_documents_batch(
        &self,
        documents: Vec<IslamicDocument>,
        batch_size: usize,
    ) -> Result<BatchIndexingResult> {
        let start_time = Instant::now();
        let total_documents = documents.len();

        info!("Starting batch indexing for {} documents with batch size {}", total_documents, batch_size);

        let mut successful_count = 0;
        let mut failed_count = 0;
        let mut failed_documents = Vec::new();

        // Process documents in batches
        for (batch_idx, batch) in documents.chunks(batch_size).enumerate() {
            info!("Processing batch {} ({} documents)", batch_idx + 1, batch.len());

            match self.process_batch(batch.to_vec()).await {
                Ok(batch_result) => {
                    successful_count += batch_result.successful_count;
                    failed_count += batch_result.failed_count;
                    failed_documents.extend(batch_result.failed_documents);
                }
                Err(e) => {
                    error!("Batch {} failed completely: {}", batch_idx + 1, e);
                    failed_count += batch.len();
                    for doc in batch {
                        failed_documents.push(FailedIndexing {
                            document_id: doc.id.clone(),
                            error: format!("Batch processing failed: {}", e),
                        });
                    }
                }
            }

            // Add small delay between batches to prevent overwhelming the system
            if batch_idx < documents.chunks(batch_size).len() - 1 {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }

        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        info!(
            "Batch indexing completed: {}/{} successful, {} failed in {}ms",
            successful_count, total_documents, failed_count, processing_time_ms
        );

        Ok(BatchIndexingResult {
            total_documents,
            successful_count,
            failed_count,
            processing_time_ms,
            failed_documents,
        })
    }

    /// Process a single batch of documents
    async fn process_batch(&self, documents: Vec<IslamicDocument>) -> Result<BatchIndexingResult> {
        let start_time = Instant::now();
        let total_documents = documents.len();

        // Prepare batch embedding request
        let texts: Vec<String> = documents.iter().map(|doc| doc.text.clone()).collect();
        let document_ids: Vec<String> = documents.iter().map(|doc| doc.id.clone()).collect();

        let batch_request = BatchEmbeddingRequest {
            texts,
            document_ids: document_ids.clone(),
        };

        // Generate embeddings for the batch
        let embedding_response = {
            let mut embedding_service = self.embedding_service.lock().await;
            embedding_service.generate_batch_embeddings(batch_request).await?
        };

        // Create document-embedding pairs
        let mut document_embedding_pairs = Vec::new();
        let mut failed_documents = Vec::new();

        for document in documents {
            if let Some(doc_embedding) = embedding_response.embeddings
                .iter()
                .find(|e| e.document_id == document.id) {
                document_embedding_pairs.push((document, doc_embedding.embedding.clone()));
            } else {
                failed_documents.push(FailedIndexing {
                    document_id: document.id,
                    error: "No embedding generated".to_string(),
                });
            }
        }

        // Index documents with embeddings in vector database
        let indexed_count = if !document_embedding_pairs.is_empty() {
            let mut search_engine = self.search_engine.lock().await;
            match search_engine.index_documents_batch(&document_embedding_pairs).await {
                Ok(count) => count,
                Err(e) => {
                    error!("Vector database batch indexing failed: {}", e);
                    // Mark all documents as failed
                    for (doc, _) in document_embedding_pairs {
                        failed_documents.push(FailedIndexing {
                            document_id: doc.id,
                            error: format!("Vector indexing failed: {}", e),
                        });
                    }
                    0
                }
            }
        } else {
            0
        };

        let successful_count = indexed_count;
        let failed_count = total_documents - successful_count;
        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(BatchIndexingResult {
            total_documents,
            successful_count,
            failed_count,
            processing_time_ms,
            failed_documents,
        })
    }

    /// Get indexing statistics
    pub async fn get_stats(&self) -> Result<IndexStats> {
        let search_engine = self.search_engine.lock().await;
        search_engine.get_collection_stats().await
    }

    /// Rebuild the entire index
    pub async fn rebuild_index(
        &self,
        content_types: Option<Vec<String>>,
        force: bool,
    ) -> Result<RebuildResult> {
        let start_time = Instant::now();

        info!("Starting index rebuild for content types: {:?}, force: {}", content_types, force);

        if force {
            // Clear the existing collection
            let mut search_engine = self.search_engine.lock().await;
            search_engine.clear_collection().await?;
            info!("Cleared existing collection");
        }

        // In a real implementation, this would:
        // 1. Query the main database for all documents of specified content types
        // 2. Re-index all documents
        // For now, we'll return a mock result

        let processed_count = 0; // Would be actual count from database
        let successful_count = 0;
        let failed_count = 0;
        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        info!("Index rebuild completed in {}ms", processing_time_ms);

        Ok(RebuildResult {
            processed_count,
            successful_count,
            failed_count,
            processing_time_ms,
            content_types_processed: content_types.unwrap_or_default(),
        })
    }

    /// Delete a document from the index
    pub async fn delete_document(&self, document_id: &str) -> Result<()> {
        info!("Deleting document from index: {}", document_id);
        
        let mut search_engine = self.search_engine.lock().await;
        search_engine.delete_document(document_id).await?;
        
        info!("Successfully deleted document: {}", document_id);
        Ok(())
    }

    /// Update a document in the index
    pub async fn update_document(&self, document: IslamicDocument) -> Result<IndexingResult> {
        info!("Updating document in index: {}", document.id);
        
        // Delete the old version first
        if let Err(e) = self.delete_document(&document.id).await {
            warn!("Failed to delete old version of document {}: {}", document.id, e);
        }
        
        // Index the new version
        self.index_document(document).await
    }

    /// Index documents from the main database
    pub async fn index_from_database(&self, content_types: Option<Vec<ContentType>>) -> Result<BatchIndexingResult> {
        info!("Starting indexing from database for content types: {:?}", content_types);

        // In a real implementation, this would:
        // 1. Connect to the main PostgreSQL database
        // 2. Query for documents of specified content types
        // 3. Convert database records to IslamicDocument structs
        // 4. Index them in batches

        // For now, we'll create some sample documents for demonstration
        let sample_documents = self.create_sample_documents();
        
        self.index_documents_batch(sample_documents, self.config.batch_size).await
    }

    /// Create sample Islamic documents for testing
    fn create_sample_documents(&self) -> Vec<IslamicDocument> {
        use std::collections::HashMap;

        vec![
            IslamicDocument {
                id: "quran_001_001".to_string(),
                text: "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ".to_string(),
                content_type: ContentType::Quran,
                source: "القرآن الكريم".to_string(),
                author: None,
                language: Language::Arabic,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("surah".to_string(), serde_json::Value::String("الفاتحة".to_string()));
                    meta.insert("surah_number".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
                    meta.insert("ayah".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
                    meta.insert("juz".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
                    meta.insert("page".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
                    meta
                },
                created_at: Some(Utc::now()),
                updated_at: Some(Utc::now()),
            },
            IslamicDocument {
                id: "quran_001_002".to_string(),
                text: "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ".to_string(),
                content_type: ContentType::Quran,
                source: "القرآن الكريم".to_string(),
                author: None,
                language: Language::Arabic,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("surah".to_string(), serde_json::Value::String("الفاتحة".to_string()));
                    meta.insert("surah_number".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
                    meta.insert("ayah".to_string(), serde_json::Value::Number(serde_json::Number::from(2)));
                    meta.insert("juz".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
                    meta.insert("page".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
                    meta
                },
                created_at: Some(Utc::now()),
                updated_at: Some(Utc::now()),
            },
            IslamicDocument {
                id: "hadith_bukhari_001".to_string(),
                text: "إنما الأعمال بالنيات وإنما لكل امرئ ما نوى فمن كانت هجرته إلى الله ورسوله فهجرته إلى الله ورسوله ومن كانت هجرته لدنيا يصيبها أو امرأة ينكحها فهجرته إلى ما هاجر إليه".to_string(),
                content_type: ContentType::SahihHadith,
                source: "صحيح البخاري".to_string(),
                author: Some("الإمام البخاري".to_string()),
                language: Language::Arabic,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("book".to_string(), serde_json::Value::String("صحيح البخاري".to_string()));
                    meta.insert("chapter".to_string(), serde_json::Value::String("بدء الوحي".to_string()));
                    meta.insert("hadith_number".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
                    meta.insert("grade".to_string(), serde_json::Value::String("صحيح".to_string()));
                    meta.insert("narrator".to_string(), serde_json::Value::String("عمر بن الخطاب".to_string()));
                    meta
                },
                created_at: Some(Utc::now()),
                updated_at: Some(Utc::now()),
            },
            IslamicDocument {
                id: "tafsir_kathir_001_001".to_string(),
                text: "الحمد لله رب العالمين: أي الثناء على الله بصفاته التي كلها أوصاف كمال، وبنعمه الظاهرة والباطنة، الدينية والدنيوية، وفي ضمنه الحمد لله بجميع المحامد".to_string(),
                content_type: ContentType::Tafsir,
                source: "تفسير ابن كثير".to_string(),
                author: Some("ابن كثير".to_string()),
                language: Language::Arabic,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("mufassir".to_string(), serde_json::Value::String("ابن كثير".to_string()));
                    meta.insert("surah".to_string(), serde_json::Value::String("الفاتحة".to_string()));
                    meta.insert("surah_number".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
                    meta.insert("ayah".to_string(), serde_json::Value::Number(serde_json::Number::from(2)));
                    meta.insert("school".to_string(), serde_json::Value::String("أهل السنة والجماعة".to_string()));
                    meta
                },
                created_at: Some(Utc::now()),
                updated_at: Some(Utc::now()),
            },
            IslamicDocument {
                id: "story_prophets_001".to_string(),
                text: "قصة آدم عليه السلام: خلق الله آدم من تراب، ثم نفخ فيه من روحه، فكان أول البشر وأبو الأنبياء. علمه الله الأسماء كلها، وأسجد له الملائكة إلا إبليس الذي أبى واستكبر".to_string(),
                content_type: ContentType::IslamicStory,
                source: "قصص الأنبياء".to_string(),
                author: Some("ابن كثير".to_string()),
                language: Language::Arabic,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("category".to_string(), serde_json::Value::String("قصص الأنبياء".to_string()));
                    meta.insert("prophet".to_string(), serde_json::Value::String("آدم".to_string()));
                    meta.insert("characters".to_string(), serde_json::Value::String("آدم، الملائكة، إبليس".to_string()));
                    meta.insert("lessons".to_string(), serde_json::Value::String("الطاعة، التواضع، خطر الكبر".to_string()));
                    meta.insert("period".to_string(), serde_json::Value::String("بداية الخلق".to_string()));
                    meta
                },
                created_at: Some(Utc::now()),
                updated_at: Some(Utc::now()),
            },
        ]
    }

    /// Get embedding service statistics
    pub async fn get_embedding_stats(&self) -> Option<(usize, usize)> {
        let embedding_service = self.embedding_service.lock().await;
        embedding_service.get_cache_stats()
    }

    /// Clear embedding cache
    pub async fn clear_embedding_cache(&self) {
        let mut embedding_service = self.embedding_service.lock().await;
        embedding_service.clear_cache();
        info!("Cleared embedding cache");
    }

    /// Validate index integrity
    pub async fn validate_index(&self) -> Result<ValidationResult> {
        info!("Starting index validation");
        
        let stats = self.get_stats().await?;
        
        // Basic validation - in a real implementation, this would be more comprehensive
        let is_valid = stats.total_documents > 0;
        let issues = if is_valid {
            Vec::new()
        } else {
            vec!["Index appears to be empty".to_string()]
        };

        Ok(ValidationResult {
            is_valid,
            total_documents: stats.total_documents,
            issues,
            checked_at: Utc::now(),
        })
    }
}

/// Result of index validation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub total_documents: u64,
    pub issues: Vec<String>,
    pub checked_at: chrono::DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_indexing_service_creation() {
        // This test would require proper setup of embedding service and search engine
        // In a real test environment, you would use mock services or test containers
    }

    #[test]
    fn test_sample_documents_creation() {
        let _config = SearchServiceConfig::default();
        // Create a mock indexing service for testing
        // let service = IndexingService::new(...).await.unwrap();
        // let docs = service.create_sample_documents();
        // assert!(!docs.is_empty());
        // assert_eq!(docs.len(), 5);
    }
}
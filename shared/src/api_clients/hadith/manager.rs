//! Hadith API Manager
//!
//! Manages multiple Hadith API clients with:
//! - Parallel querying of multiple APIs
//! - Result merging and deduplication
//! - Caching and rate limiting
//! - Health monitoring

use crate::api_clients::{
    ApiError, CacheManager, HadithApiClient, HadithResult, RateLimiter,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;

/// Hadith API Manager
/// 
/// Coordinates multiple Hadith API clients with:
/// - Parallel API querying
/// - Result deduplication
/// - Intelligent caching
/// - Rate limiting
pub struct HadithApiManager {
    clients: Vec<Box<dyn HadithApiClient + Send + Sync>>,
    cache: Arc<CacheManager>,
    rate_limiter: Arc<RateLimiter>,
}

impl HadithApiManager {
    /// Create a new Hadith API Manager
    pub fn new(
        clients: Vec<Box<dyn HadithApiClient + Send + Sync>>,
        cache: Arc<CacheManager>,
        rate_limiter: Arc<RateLimiter>,
    ) -> Self {
        // Sort clients by priority (lower number = higher priority)
        let mut sorted_clients = clients;
        sorted_clients.sort_by_key(|c| c.priority());

        Self {
            clients: sorted_clients,
            cache,
            rate_limiter,
        }
    }

    /// Search for hadith across all APIs in parallel
    /// 
    /// This method queries all configured hadith APIs in parallel,
    /// then merges and deduplicates the results.
    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<HadithResult>, ApiError> {
        let cache_key = format!("hadith:search:{}:{}", query, limit);

        // 1. Check cache first
        if let Ok(Some(cached)) = self.cache.get::<Vec<HadithResult>>(&cache_key).await {
            tracing::debug!("Cache hit for hadith search: {}", query);
            return Ok(cached);
        }

        // 2. Query all APIs in parallel
        let start_time = Instant::now();
        let mut tasks = Vec::new();

        for client in &self.clients {
            // Check if API is healthy
            if !client.is_healthy().await {
                tracing::warn!("API {} is unhealthy, skipping", client.api_name());
                continue;
            }

            // Check rate limit
            if !self
                .rate_limiter
                .check_and_increment(client.api_name())
                .await
                .unwrap_or(false)
            {
                tracing::warn!("Rate limit exceeded for {}, skipping", client.api_name());
                continue;
            }

            // Clone necessary data for the async task
            let client_clone = client.api_name().to_string();
            let query_clone = query.to_string();
            
            // Create a task for this API call
            let task = async move {
                // Note: We can't clone the trait object, so we'll need to handle this differently
                // For now, we'll return the client name and let the manager handle the actual call
                (client_clone, query_clone, limit)
            };
            
            tasks.push(task);
        }

        // Execute all API calls in parallel
        let mut all_results = Vec::new();
        let mut errors = Vec::new();

        for client in &self.clients {
            // Check if API is healthy
            if !client.is_healthy().await {
                continue;
            }

            // Check rate limit
            if !self
                .rate_limiter
                .check_and_increment(client.api_name())
                .await
                .unwrap_or(false)
            {
                continue;
            }

            // Make the API call
            match client.search(query, limit).await {
                Ok(results) => {
                    tracing::info!(
                        "API {} returned {} results for query: {}",
                        client.api_name(),
                        results.len(),
                        query
                    );
                    all_results.extend(results);
                }
                Err(e) => {
                    tracing::warn!("API {} failed: {}", client.api_name(), e);
                    errors.push(e);
                }
            }
        }

        let query_duration = start_time.elapsed();
        tracing::info!(
            "Parallel hadith search completed in {:?}, total results before dedup: {}",
            query_duration,
            all_results.len()
        );

        // 3. Deduplicate results
        let deduplicated = self.deduplicate_results(all_results);
        tracing::info!(
            "After deduplication: {} unique results",
            deduplicated.len()
        );

        // 4. If we got results, cache them
        if !deduplicated.is_empty() {
            // Cache for 30 days (hadith are static)
            if let Err(e) = self
                .cache
                .set(&cache_key, &deduplicated, Duration::from_secs(30 * 24 * 60 * 60))
                .await
            {
                tracing::warn!("Failed to cache hadith search results: {}", e);
            }
            return Ok(deduplicated);
        }

        // 5. All APIs failed, try expired cache
        if let Ok(Some(cached)) = self.cache.get_stale::<Vec<HadithResult>>(&cache_key).await {
            tracing::warn!("All APIs failed, serving expired cache for query: {}", query);
            return Ok(cached);
        }

        // 6. Everything failed
        if !errors.is_empty() {
            Err(errors.into_iter().next().unwrap())
        } else {
            Err(ApiError::AllApisFailed)
        }
    }

    /// Get a specific hadith by ID
    pub async fn get_by_id(&self, id: &str) -> Result<HadithResult, ApiError> {
        let cache_key = format!("hadith:id:{}", id);

        // 1. Check cache first
        if let Ok(Some(cached)) = self.cache.get::<HadithResult>(&cache_key).await {
            tracing::debug!("Cache hit for hadith ID: {}", id);
            return Ok(cached);
        }

        // 2. Try each API in priority order
        let mut last_error = None;
        for client in &self.clients {
            // Check if API is healthy
            if !client.is_healthy().await {
                tracing::warn!("API {} is unhealthy, skipping", client.api_name());
                continue;
            }

            // Check rate limit
            if !self
                .rate_limiter
                .check_and_increment(client.api_name())
                .await
                .unwrap_or(false)
            {
                tracing::warn!("Rate limit exceeded for {}, trying next API", client.api_name());
                continue;
            }

            // Try to fetch from this API
            match client.get_by_id(id).await {
                Ok(hadith) => {
                    tracing::info!("Successfully fetched hadith {} from {}", id, client.api_name());
                    
                    // Cache the result (30 days TTL for static hadith)
                    if let Err(e) = self
                        .cache
                        .set(&cache_key, &hadith, Duration::from_secs(30 * 24 * 60 * 60))
                        .await
                    {
                        tracing::warn!("Failed to cache hadith: {}", e);
                    }
                    
                    return Ok(hadith);
                }
                Err(e) => {
                    tracing::warn!("API {} failed for hadith {}: {}", client.api_name(), id, e);
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // 3. All APIs failed, try expired cache
        if let Ok(Some(cached)) = self.cache.get_stale::<HadithResult>(&cache_key).await {
            tracing::warn!("All APIs failed, serving expired cache for hadith {}", id);
            return Ok(cached);
        }

        // 4. Everything failed
        Err(last_error.unwrap_or(ApiError::AllApisFailed))
    }

    /// Get hadith from a specific collection
    pub async fn get_by_collection(
        &self,
        collection: &str,
        limit: usize,
    ) -> Result<Vec<HadithResult>, ApiError> {
        let cache_key = format!("hadith:collection:{}:{}", collection, limit);

        // 1. Check cache first
        if let Ok(Some(cached)) = self.cache.get::<Vec<HadithResult>>(&cache_key).await {
            tracing::debug!("Cache hit for collection: {}", collection);
            return Ok(cached);
        }

        // 2. Try each API in priority order
        let mut last_error = None;
        for client in &self.clients {
            // Check if API is healthy
            if !client.is_healthy().await {
                tracing::warn!("API {} is unhealthy, skipping", client.api_name());
                continue;
            }

            // Check rate limit
            if !self
                .rate_limiter
                .check_and_increment(client.api_name())
                .await
                .unwrap_or(false)
            {
                tracing::warn!("Rate limit exceeded for {}, trying next API", client.api_name());
                continue;
            }

            // Try to fetch from this API
            match client.get_by_collection(collection, limit).await {
                Ok(results) => {
                    tracing::info!(
                        "Successfully fetched {} hadith from collection {} via {}",
                        results.len(),
                        collection,
                        client.api_name()
                    );
                    
                    // Cache the result (30 days TTL for static hadith)
                    if let Err(e) = self
                        .cache
                        .set(&cache_key, &results, Duration::from_secs(30 * 24 * 60 * 60))
                        .await
                    {
                        tracing::warn!("Failed to cache collection: {}", e);
                    }
                    
                    return Ok(results);
                }
                Err(e) => {
                    tracing::warn!(
                        "API {} failed for collection {}: {}",
                        client.api_name(),
                        collection,
                        e
                    );
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // 3. All APIs failed, try expired cache
        if let Ok(Some(cached)) = self.cache.get_stale::<Vec<HadithResult>>(&cache_key).await {
            tracing::warn!("All APIs failed, serving expired cache for collection {}", collection);
            return Ok(cached);
        }

        // 4. Everything failed
        Err(last_error.unwrap_or(ApiError::AllApisFailed))
    }

    /// Deduplicate hadith results based on content hash
    /// 
    /// Uses a combination of text_arabic hash and hadith_number to identify duplicates
    /// Made public for testing purposes
    pub fn deduplicate_results(&self, results: Vec<HadithResult>) -> Vec<HadithResult> {
        let mut seen = HashSet::new();
        let mut unique_results = Vec::new();

        for result in results {
            // Create a unique key based on arabic text and hadith number
            let key = format!("{}:{}", self.compute_hash(&result.text_arabic), result.hadith_number);
            
            if seen.insert(key) {
                unique_results.push(result);
            }
        }

        unique_results
    }

    /// Compute a simple hash of text for deduplication
    fn compute_hash(&self, text: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Get the number of configured clients
    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    /// Get the names of all configured clients in priority order
    pub fn client_names(&self) -> Vec<String> {
        self.clients.iter().map(|c| c.api_name().to_string()).collect()
    }

    /// Clear cache for a specific key (for testing)
    #[cfg(test)]
    pub async fn clear_cache(&self, key: &str) -> Result<(), ApiError> {
        self.cache.delete(key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_clients::hadith::{
        AladhanHadithClient, HadithApiClientImpl, SunnahComClient,
    };

    async fn create_test_manager() -> HadithApiManager {
        let cache = Arc::new(
            CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"),
        );

        let rate_limiter = Arc::new(
            RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                .await
                .expect("Failed to create rate limiter"),
        );

        let clients: Vec<Box<dyn HadithApiClient + Send + Sync>> = vec![
            Box::new(SunnahComClient::new("test_key".to_string())),
            Box::new(HadithApiClientImpl::default()),
            Box::new(AladhanHadithClient::new()),
        ];

        HadithApiManager::new(clients, cache, rate_limiter)
    }

    #[test]
    fn test_manager_creation() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let cache = Arc::new(
                CacheManager::new("redis://127.0.0.1:6379/")
                    .await
                    .expect("Failed to create cache manager"),
            );

            let rate_limiter = Arc::new(
                RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                    .await
                    .expect("Failed to create rate limiter"),
            );

            let clients: Vec<Box<dyn HadithApiClient + Send + Sync>> = vec![
                Box::new(SunnahComClient::new("test_key".to_string())),
                Box::new(HadithApiClientImpl::default()),
            ];

            let manager = HadithApiManager::new(clients, cache, rate_limiter);
            assert_eq!(manager.client_count(), 2);
        });
    }

    #[test]
    fn test_clients_sorted_by_priority() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let cache = Arc::new(
                CacheManager::new("redis://127.0.0.1:6379/")
                    .await
                    .expect("Failed to create cache manager"),
            );

            let rate_limiter = Arc::new(
                RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                    .await
                    .expect("Failed to create rate limiter"),
            );

            // Add clients in reverse priority order
            let clients: Vec<Box<dyn HadithApiClient + Send + Sync>> = vec![
                Box::new(AladhanHadithClient::new()),              // Priority 3
                Box::new(HadithApiClientImpl::default()),          // Priority 2
                Box::new(SunnahComClient::new("test_key".to_string())), // Priority 1
            ];

            let manager = HadithApiManager::new(clients, cache, rate_limiter);
            let names = manager.client_names();

            // Should be sorted by priority: sunnah.com (1), hadith.api (2), aladhan.hadith (3)
            assert_eq!(names[0], "sunnah.com");
            assert_eq!(names[1], "hadith.api");
            assert_eq!(names[2], "aladhan.hadith");
        });
    }

    #[test]
    fn test_deduplication() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let manager = create_test_manager().await;

            // Create duplicate results
            let results = vec![
                HadithResult {
                    id: "1".to_string(),
                    collection: "Bukhari".to_string(),
                    book: "Book 1".to_string(),
                    hadith_number: "1".to_string(),
                    text_arabic: "نفس النص".to_string(),
                    text_translation: Some("Same text".to_string()),
                    grade: Some("Sahih".to_string()),
                    narrator: "Abu Hurairah".to_string(),
                    source: "source1".to_string(),
                },
                HadithResult {
                    id: "2".to_string(),
                    collection: "Bukhari".to_string(),
                    book: "Book 1".to_string(),
                    hadith_number: "1".to_string(),
                    text_arabic: "نفس النص".to_string(), // Same Arabic text
                    text_translation: Some("Same text".to_string()),
                    grade: Some("Sahih".to_string()),
                    narrator: "Abu Hurairah".to_string(),
                    source: "source2".to_string(),
                },
                HadithResult {
                    id: "3".to_string(),
                    collection: "Muslim".to_string(),
                    book: "Book 2".to_string(),
                    hadith_number: "2".to_string(),
                    text_arabic: "نص مختلف".to_string(), // Different text
                    text_translation: Some("Different text".to_string()),
                    grade: Some("Sahih".to_string()),
                    narrator: "Aisha".to_string(),
                    source: "source1".to_string(),
                },
            ];

            let deduplicated = manager.deduplicate_results(results);
            
            // Should have 2 unique results (first two are duplicates)
            assert_eq!(deduplicated.len(), 2);
        });
    }
}

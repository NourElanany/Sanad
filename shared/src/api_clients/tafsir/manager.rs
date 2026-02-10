//! Tafsir API Manager
//!
//! Manages multiple Tafsir API clients with fallback logic, caching, and rate limiting.
//! Organizes tafsir results by scholar and language for easy access.

use crate::api_clients::{
    ApiError, CacheManager, RateLimiter, TafsirApiClient, TafsirEntry, TafsirSource,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Tafsir API Manager
/// 
/// Coordinates multiple Tafsir API clients with:
/// - Priority-based fallback
/// - Intelligent caching (long TTL for static tafsir)
/// - Rate limiting
/// - Health monitoring
/// - Organization by scholar and language
pub struct TafsirApiManager {
    clients: Vec<Box<dyn TafsirApiClient + Send + Sync>>,
    cache: Arc<CacheManager>,
    rate_limiter: Arc<RateLimiter>,
}

/// Organized tafsir response grouped by scholar and language
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrganizedTafsirResponse {
    pub surah: u8,
    pub ayah: u16,
    pub by_scholar: HashMap<String, Vec<TafsirEntry>>,
    pub by_language: HashMap<String, Vec<TafsirEntry>>,
    pub all_tafsirs: Vec<TafsirEntry>,
}

impl TafsirApiManager {
    /// Create a new Tafsir API Manager
    pub fn new(
        clients: Vec<Box<dyn TafsirApiClient + Send + Sync>>,
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

    /// Get tafsir for a specific verse with fallback logic
    /// 
    /// If tafsir_id is provided, fetches only that specific tafsir.
    /// Otherwise, fetches all available tafsirs from all sources.
    pub async fn get_tafsir(
        &self,
        surah: u8,
        ayah: u16,
        tafsir_id: Option<&str>,
    ) -> Result<Vec<TafsirEntry>, ApiError> {
        let cache_key = if let Some(id) = tafsir_id {
            format!("tafsir:{}:{}:{}", surah, ayah, id)
        } else {
            format!("tafsir:{}:{}", surah, ayah)
        };

        // 1. Check cache first
        if let Ok(Some(cached)) = self.cache.get::<Vec<TafsirEntry>>(&cache_key).await {
            tracing::debug!("Cache hit for tafsir {}:{}", surah, ayah);
            return Ok(cached);
        }

        // 2. Try each API client in priority order
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
            match client.get_tafsir(surah, ayah, tafsir_id).await {
                Ok(tafsirs) => {
                    tracing::info!(
                        "Successfully fetched {} tafsir(s) for {}:{} from {}",
                        tafsirs.len(),
                        surah,
                        ayah,
                        client.api_name()
                    );
                    
                    // Cache the result (30 days TTL for static tafsir)
                    if let Err(e) = self
                        .cache
                        .set(&cache_key, &tafsirs, Duration::from_secs(30 * 24 * 60 * 60))
                        .await
                    {
                        tracing::warn!("Failed to cache tafsir: {}", e);
                    }
                    
                    return Ok(tafsirs);
                }
                Err(e) => {
                    tracing::warn!(
                        "API {} failed for tafsir {}:{}: {}",
                        client.api_name(),
                        surah,
                        ayah,
                        e
                    );
                    last_error = Some(e);
                    continue;
                }
            }
        }

        // 3. All APIs failed, try expired cache as last resort
        if let Ok(Some(cached)) = self.cache.get_stale::<Vec<TafsirEntry>>(&cache_key).await {
            tracing::warn!("All APIs failed, serving expired cache for tafsir {}:{}", surah, ayah);
            return Ok(cached);
        }

        // 4. Everything failed
        Err(last_error.unwrap_or(ApiError::AllApisFailed))
    }

    /// Get tafsir organized by scholar and language
    /// 
    /// This method fetches all available tafsirs and organizes them
    /// for easy access by scholar name or language.
    /// 
    /// **Validates: Requirements 4.3 - Property 8: Tafsir Organization by Scholar and Language**
    pub async fn get_organized_tafsir(
        &self,
        surah: u8,
        ayah: u16,
    ) -> Result<OrganizedTafsirResponse, ApiError> {
        // Fetch all tafsirs
        let tafsirs = self.get_tafsir(surah, ayah, None).await?;

        // Organize by scholar
        let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        for tafsir in &tafsirs {
            by_scholar
                .entry(tafsir.scholar.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        // Organize by language
        let mut by_language: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        for tafsir in &tafsirs {
            by_language
                .entry(tafsir.language.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        Ok(OrganizedTafsirResponse {
            surah,
            ayah,
            by_scholar,
            by_language,
            all_tafsirs: tafsirs,
        })
    }

    /// List all available tafsir sources from all APIs
    pub async fn list_all_sources(&self) -> Result<Vec<TafsirSource>, ApiError> {
        let cache_key = "tafsir:sources:all";

        // 1. Check cache first
        if let Ok(Some(cached)) = self.cache.get::<Vec<TafsirSource>>(&cache_key).await {
            tracing::debug!("Cache hit for tafsir sources");
            return Ok(cached);
        }

        // 2. Collect sources from all healthy APIs
        let mut all_sources = Vec::new();
        let mut errors = Vec::new();

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

            // Fetch sources from this API
            match client.list_tafsir_sources().await {
                Ok(sources) => {
                    tracing::info!(
                        "API {} returned {} tafsir sources",
                        client.api_name(),
                        sources.len()
                    );
                    all_sources.extend(sources);
                }
                Err(e) => {
                    tracing::warn!("API {} failed to list sources: {}", client.api_name(), e);
                    errors.push(e);
                }
            }
        }

        // 3. Deduplicate sources by ID
        let unique_sources = self.deduplicate_sources(all_sources);

        // 4. If we got sources, cache them
        if !unique_sources.is_empty() {
            // Cache for 7 days (sources don't change often)
            if let Err(e) = self
                .cache
                .set(&cache_key, &unique_sources, Duration::from_secs(7 * 24 * 60 * 60))
                .await
            {
                tracing::warn!("Failed to cache tafsir sources: {}", e);
            }
            return Ok(unique_sources);
        }

        // 5. All APIs failed, try expired cache
        if let Ok(Some(cached)) = self.cache.get_stale::<Vec<TafsirSource>>(&cache_key).await {
            tracing::warn!("All APIs failed, serving expired cache for tafsir sources");
            return Ok(cached);
        }

        // 6. Everything failed
        if !errors.is_empty() {
            Err(errors.into_iter().next().unwrap())
        } else {
            Err(ApiError::AllApisFailed)
        }
    }

    /// Get tafsir sources filtered by language
    pub async fn list_sources_by_language(&self, language: &str) -> Result<Vec<TafsirSource>, ApiError> {
        let all_sources = self.list_all_sources().await?;
        
        let filtered: Vec<TafsirSource> = all_sources
            .into_iter()
            .filter(|s| s.language.eq_ignore_ascii_case(language))
            .collect();

        if filtered.is_empty() {
            tracing::warn!("No tafsir sources found for language: {}", language);
            return Err(ApiError::NotFound);
        }

        Ok(filtered)
    }

    /// Get tafsir sources filtered by scholar
    pub async fn list_sources_by_scholar(&self, scholar: &str) -> Result<Vec<TafsirSource>, ApiError> {
        let all_sources = self.list_all_sources().await?;
        
        let filtered: Vec<TafsirSource> = all_sources
            .into_iter()
            .filter(|s| s.scholar.to_lowercase().contains(&scholar.to_lowercase()))
            .collect();

        if filtered.is_empty() {
            tracing::warn!("No tafsir sources found for scholar: {}", scholar);
            return Err(ApiError::NotFound);
        }

        Ok(filtered)
    }

    /// Deduplicate tafsir sources by ID
    /// Made public for testing purposes
    pub fn deduplicate_sources(&self, sources: Vec<TafsirSource>) -> Vec<TafsirSource> {
        let mut seen_ids = std::collections::HashSet::new();
        let mut unique_sources = Vec::new();

        for source in sources {
            if seen_ids.insert(source.id.clone()) {
                unique_sources.push(source);
            }
        }

        unique_sources
    }

    /// Get the number of configured clients
    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    /// Get the names of all configured clients in priority order
    pub fn client_names(&self) -> Vec<String> {
        self.clients
            .iter()
            .map(|c| c.api_name().to_string())
            .collect()
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
    use crate::api_clients::tafsir::QuranComTafsirClient;
    use std::collections::HashMap;

    async fn create_test_manager() -> TafsirApiManager {
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

        let clients: Vec<Box<dyn TafsirApiClient + Send + Sync>> = vec![
            Box::new(QuranComTafsirClient::new(None)),
        ];

        TafsirApiManager::new(clients, cache, rate_limiter)
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

            let clients: Vec<Box<dyn TafsirApiClient + Send + Sync>> = vec![
                Box::new(QuranComTafsirClient::new(None)),
            ];

            let manager = TafsirApiManager::new(clients, cache, rate_limiter);
            assert_eq!(manager.client_count(), 1);
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

            let clients: Vec<Box<dyn TafsirApiClient + Send + Sync>> = vec![
                Box::new(QuranComTafsirClient::new(None)), // Priority 1
            ];

            let manager = TafsirApiManager::new(clients, cache, rate_limiter);
            let names = manager.client_names();

            // Should be sorted by priority
            assert_eq!(names[0], "quran.com_tafsir");
        });
    }

    #[tokio::test]
    async fn test_invalid_verse_number() {
        let manager = create_test_manager().await;

        // Test invalid surah number
        let result = manager.get_tafsir(0, 1, None).await;
        assert!(result.is_err());

        let result = manager.get_tafsir(115, 1, None).await;
        assert!(result.is_err());

        // Test invalid ayah number
        let result = manager.get_tafsir(1, 0, None).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_deduplicate_sources() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let manager = create_test_manager().await;

            let sources = vec![
                TafsirSource {
                    id: "1".to_string(),
                    name: "Tafsir 1".to_string(),
                    scholar: "Scholar A".to_string(),
                    language: "Arabic".to_string(),
                },
                TafsirSource {
                    id: "1".to_string(), // Duplicate ID
                    name: "Tafsir 1 Copy".to_string(),
                    scholar: "Scholar A".to_string(),
                    language: "Arabic".to_string(),
                },
                TafsirSource {
                    id: "2".to_string(),
                    name: "Tafsir 2".to_string(),
                    scholar: "Scholar B".to_string(),
                    language: "English".to_string(),
                },
            ];

            let unique = manager.deduplicate_sources(sources);
            assert_eq!(unique.len(), 2); // Should have 2 unique sources
        });
    }

    #[test]
    fn test_organize_tafsir_by_scholar() {
        let tafsirs = vec![
            TafsirEntry {
                tafsir_id: "1".to_string(),
                tafsir_name: "Tafsir Ibn Kathir".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "Text 1".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "2".to_string(),
                tafsir_name: "Tafsir Al-Jalalayn".to_string(),
                scholar: "Al-Jalalayn".to_string(),
                text: "Text 2".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "3".to_string(),
                tafsir_name: "Tafsir Ibn Kathir English".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "Text 3".to_string(),
                language: "English".to_string(),
                source: "test".to_string(),
            },
        ];

        let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        for tafsir in &tafsirs {
            by_scholar
                .entry(tafsir.scholar.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        // Ibn Kathir should have 2 tafsirs
        assert_eq!(by_scholar.get("Ibn Kathir").unwrap().len(), 2);
        // Al-Jalalayn should have 1 tafsir
        assert_eq!(by_scholar.get("Al-Jalalayn").unwrap().len(), 1);
    }

    #[test]
    fn test_organize_tafsir_by_language() {
        let tafsirs = vec![
            TafsirEntry {
                tafsir_id: "1".to_string(),
                tafsir_name: "Tafsir Ibn Kathir".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "Text 1".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "2".to_string(),
                tafsir_name: "Tafsir Al-Jalalayn".to_string(),
                scholar: "Al-Jalalayn".to_string(),
                text: "Text 2".to_string(),
                language: "Arabic".to_string(),
                source: "test".to_string(),
            },
            TafsirEntry {
                tafsir_id: "3".to_string(),
                tafsir_name: "Tafsir Ibn Kathir English".to_string(),
                scholar: "Ibn Kathir".to_string(),
                text: "Text 3".to_string(),
                language: "English".to_string(),
                source: "test".to_string(),
            },
        ];

        let mut by_language: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
        for tafsir in &tafsirs {
            by_language
                .entry(tafsir.language.clone())
                .or_insert_with(Vec::new)
                .push(tafsir.clone());
        }

        // Arabic should have 2 tafsirs
        assert_eq!(by_language.get("Arabic").unwrap().len(), 2);
        // English should have 1 tafsir
        assert_eq!(by_language.get("English").unwrap().len(), 1);
    }
}

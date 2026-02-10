//! Property-based tests for Quran API clients
//!
//! These tests verify universal properties that should hold across all inputs.

use super::*;
use crate::api_clients::{
    ApiClient, ApiError, AyahData, CacheManager, PageData, QuranApiClient, RateLimiter,
    SurahData,
};
use async_trait::async_trait;
use proptest::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

// ============================================================================
// Mock API Client for Testing
// ============================================================================

/// Mock Quran API client that can be configured to fail
#[derive(Debug, Clone)]
struct MockQuranClient {
    name: String,
    priority: u8,
    should_fail: Arc<AtomicBool>,
    call_count: Arc<AtomicUsize>,
}

impl MockQuranClient {
    fn new(name: String, priority: u8) -> Self {
        Self {
            name,
            priority,
            should_fail: Arc::new(AtomicBool::new(false)),
            call_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn set_should_fail(&self, should_fail: bool) {
        self.should_fail.store(should_fail, Ordering::SeqCst);
    }

    fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }

    fn reset_call_count(&self) {
        self.call_count.store(0, Ordering::SeqCst);
    }
}

#[async_trait]
impl ApiClient for MockQuranClient {
    fn api_name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> u8 {
        self.priority
    }

    async fn is_healthy(&self) -> bool {
        !self.should_fail.load(Ordering::SeqCst)
    }

    fn rate_limit(&self) -> crate::api_clients::RateLimitConfig {
        crate::api_clients::RateLimitConfig {
            requests_per_minute: 1000,
            requests_per_hour: 10000,
            requests_per_day: 100000,
        }
    }
}

#[async_trait]
impl QuranApiClient for MockQuranClient {
    async fn get_surah(&self, surah_number: u8) -> Result<SurahData, ApiError> {
        self.call_count.fetch_add(1, Ordering::SeqCst);

        if self.should_fail.load(Ordering::SeqCst) {
            return Err(ApiError::ApiError(
                self.name.clone(),
                "Mock failure".to_string(),
            ));
        }

        Ok(SurahData {
            number: surah_number,
            name_arabic: format!("سورة {}", surah_number),
            name_english: format!("Surah {}", surah_number),
            ayahs: vec![AyahData {
                surah: surah_number,
                ayah: 1,
                text_arabic: format!("Mock text for surah {}", surah_number),
                text_translation: None,
            }],
        })
    }

    async fn get_ayah(&self, surah: u8, ayah: u16) -> Result<AyahData, ApiError> {
        self.call_count.fetch_add(1, Ordering::SeqCst);

        if self.should_fail.load(Ordering::SeqCst) {
            return Err(ApiError::ApiError(
                self.name.clone(),
                "Mock failure".to_string(),
            ));
        }

        Ok(AyahData {
            surah,
            ayah,
            text_arabic: format!("Mock text for {}:{}", surah, ayah),
            text_translation: None,
        })
    }

    async fn get_page(&self, page: u16) -> Result<PageData, ApiError> {
        self.call_count.fetch_add(1, Ordering::SeqCst);

        if self.should_fail.load(Ordering::SeqCst) {
            return Err(ApiError::ApiError(
                self.name.clone(),
                "Mock failure".to_string(),
            ));
        }

        Ok(PageData {
            page_number: page,
            ayahs: vec![AyahData {
                surah: 1,
                ayah: 1,
                text_arabic: format!("Mock text for page {}", page),
                text_translation: None,
            }],
        })
    }
}

// ============================================================================
// Property Tests
// ============================================================================

/// Helper to create a test manager with real API clients
async fn create_test_manager_with_real_apis() -> QuranApiManager {
    use crate::api_clients::quran::{QuranComClient, AlquranCloudClient, TanzilClient};
    
    let cache = Arc::new(CacheManager::new("redis://127.0.0.1:6379/")
        .await
        .expect("Failed to create cache manager"));
    
    let rate_limiter = Arc::new(RateLimiter::new(
        "redis://127.0.0.1:6379/",
        HashMap::new()
    )
        .await
        .expect("Failed to create rate limiter"));

    let clients: Vec<Box<dyn QuranApiClient + Send + Sync>> = vec![
        Box::new(QuranComClient::new(None)),
        Box::new(AlquranCloudClient::new()),
        Box::new(TanzilClient::new()),
    ];

    QuranApiManager::new(clients, cache, rate_limiter)
}

/// Helper to create a test manager with mock clients (for fallback testing)
async fn create_test_manager_with_mocks() -> (
    QuranApiManager,
    Arc<MockQuranClient>,
    Arc<MockQuranClient>,
    Arc<MockQuranClient>,
) {
    let cache = Arc::new(CacheManager::new("redis://127.0.0.1:6379/")
        .await
        .expect("Failed to create cache manager"));
    
    let rate_limiter = Arc::new(RateLimiter::new(
        "redis://127.0.0.1:6379/",
        HashMap::new()
    )
        .await
        .expect("Failed to create rate limiter"));

    let client1 = Arc::new(MockQuranClient::new("primary".to_string(), 1));
    let client2 = Arc::new(MockQuranClient::new("secondary".to_string(), 2));
    let client3 = Arc::new(MockQuranClient::new("tertiary".to_string(), 3));

    let clients: Vec<Box<dyn QuranApiClient + Send + Sync>> = vec![
        Box::new((*client1).clone()),
        Box::new((*client2).clone()),
        Box::new((*client3).clone()),
    ];

    let manager = QuranApiManager::new(clients, cache, rate_limiter);

    (manager, client1, client2, client3)
}

/// Feature: official-apis-integration, Property 2: Fallback Chain Execution
/// 
/// **Validates: Requirements 1.2, 3.3, 6.4, 11.4, 12.1**
/// 
/// For any API request, if the primary API fails, the system should attempt
/// secondary APIs in priority order until one succeeds or all fail, and each
/// attempt should be logged.
#[cfg(test)]
mod fallback_chain_tests {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(20))]

        /// Property: When primary API fails, secondary APIs are tried in priority order
        #[test]
        fn prop_fallback_chain_execution_surah(surah_number in 1u8..=114) {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let (manager, client1, client2, client3) = create_test_manager_with_mocks().await;

                // Reset call counts
                client1.reset_call_count();
                client2.reset_call_count();
                client3.reset_call_count();

                // Make primary fail, secondary succeed
                client1.set_should_fail(true);
                client2.set_should_fail(false);
                client3.set_should_fail(false);

                // Clear cache to ensure we hit the APIs
                let cache_key = format!("quran:surah:{}", surah_number);
                let _ = manager.clear_cache(&cache_key).await;

                // Request should succeed via fallback
                let result = manager.get_surah(surah_number).await;
                prop_assert!(result.is_ok(), "Request should succeed via fallback");

                // Verify fallback chain: primary was tried, secondary succeeded
                prop_assert!(client1.get_call_count() > 0, "Primary API should be tried");
                prop_assert!(client2.get_call_count() > 0, "Secondary API should be tried after primary fails");

                Ok(())
            })?;
        }

        /// Property: When primary and secondary fail, tertiary is tried
        #[test]
        fn prop_fallback_to_tertiary(surah_number in 1u8..=114) {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let (manager, client1, client2, client3) = create_test_manager_with_mocks().await;

                // Reset call counts
                client1.reset_call_count();
                client2.reset_call_count();
                client3.reset_call_count();

                // Make primary and secondary fail, tertiary succeed
                client1.set_should_fail(true);
                client2.set_should_fail(true);
                client3.set_should_fail(false);

                // Clear cache
                let cache_key = format!("quran:surah:{}", surah_number);
                let _ = manager.clear_cache(&cache_key).await;

                // Request should succeed via tertiary
                let result = manager.get_surah(surah_number).await;
                prop_assert!(result.is_ok(), "Request should succeed via tertiary fallback");

                // Verify all three were tried
                prop_assert!(client1.get_call_count() > 0, "Primary should be tried");
                prop_assert!(client2.get_call_count() > 0, "Secondary should be tried");
                prop_assert!(client3.get_call_count() > 0, "Tertiary should be tried");

                Ok(())
            })?;
        }

        /// Property: When all APIs fail, error is returned
        #[test]
        fn prop_all_apis_fail(surah_number in 1u8..=114) {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let (manager, client1, client2, client3) = create_test_manager_with_mocks().await;

                // Reset call counts
                client1.reset_call_count();
                client2.reset_call_count();
                client3.reset_call_count();

                // Make all APIs fail
                client1.set_should_fail(true);
                client2.set_should_fail(true);
                client3.set_should_fail(true);

                // Clear cache
                let cache_key = format!("quran:surah:{}", surah_number);
                let _ = manager.clear_cache(&cache_key).await;

                // Request should fail
                let result = manager.get_surah(surah_number).await;
                prop_assert!(result.is_err(), "Request should fail when all APIs fail");

                // Verify all were tried
                prop_assert!(client1.get_call_count() > 0, "Primary should be tried");
                prop_assert!(client2.get_call_count() > 0, "Secondary should be tried");
                prop_assert!(client3.get_call_count() > 0, "Tertiary should be tried");

                Ok(())
            })?;
        }

        /// Property: Fallback chain works for ayah requests
        #[test]
        fn prop_fallback_chain_ayah(surah in 1u8..=114, ayah in 1u16..=286) {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let (manager, client1, client2, client3) = create_test_manager_with_mocks().await;

                // Reset call counts
                client1.reset_call_count();
                client2.reset_call_count();
                client3.reset_call_count();

                // Make primary fail, secondary succeed
                client1.set_should_fail(true);
                client2.set_should_fail(false);
                client3.set_should_fail(false);

                // Clear cache
                let cache_key = format!("quran:ayah:{}:{}", surah, ayah);
                let _ = manager.clear_cache(&cache_key).await;

                // Request should succeed via fallback
                let result = manager.get_ayah(surah, ayah).await;
                prop_assert!(result.is_ok(), "Ayah request should succeed via fallback");

                // Verify fallback was used
                prop_assert!(client1.get_call_count() > 0, "Primary should be tried");
                prop_assert!(client2.get_call_count() > 0, "Secondary should be tried");

                Ok(())
            })?;
        }

        /// Property: Fallback chain works for page requests
        #[test]
        fn prop_fallback_chain_page(page in 1u16..=604) {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let (manager, client1, client2, client3) = create_test_manager_with_mocks().await;

                // Reset call counts
                client1.reset_call_count();
                client2.reset_call_count();
                client3.reset_call_count();

                // Make primary fail, secondary succeed
                client1.set_should_fail(true);
                client2.set_should_fail(false);
                client3.set_should_fail(false);

                // Clear cache
                let cache_key = format!("quran:page:{}", page);
                let _ = manager.clear_cache(&cache_key).await;

                // Request should succeed via fallback
                let result = manager.get_page(page).await;
                prop_assert!(result.is_ok(), "Page request should succeed via fallback");

                // Verify fallback was used
                prop_assert!(client1.get_call_count() > 0, "Primary should be tried");
                prop_assert!(client2.get_call_count() > 0, "Secondary should be tried");

                Ok(())
            })?;
        }
    }
}


// ============================================================================
// Response Validation Property Tests
// ============================================================================

/// Feature: official-apis-integration, Property 3: Response Validation Consistency
/// 
/// **Validates: Requirements 1.4, 2.4, 3.4, 4.4, 5.4, 6.3**
/// 
/// For any API response, the Response_Validator should verify that the response
/// structure matches the expected schema and that all required fields are present
/// and valid before returning to the caller.
#[cfg(test)]
mod response_validation_tests {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(20))]

        /// Property: Surah responses have valid structure
        #[test]
        fn prop_surah_response_valid_structure(surah_number in 1u8..=114) {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let manager = create_test_manager_with_real_apis().await;

                // Clear cache to ensure we hit the real APIs
                let cache_key = format!("quran:surah:{}", surah_number);
                let _ = manager.clear_cache(&cache_key).await;

                let result = manager.get_surah(surah_number).await;
                prop_assert!(result.is_ok(), "Request should succeed with real API");

                let surah = result.unwrap();

                // Validate response structure from real API
                prop_assert_eq!(surah.number, surah_number, "Surah number should match request");
                prop_assert!(!surah.name_arabic.is_empty(), "Arabic name should not be empty");
                prop_assert!(!surah.name_english.is_empty(), "English name should not be empty");
                prop_assert!(!surah.ayahs.is_empty(), "Surah should have at least one ayah");

                // Validate each ayah
                for ayah in &surah.ayahs {
                    prop_assert_eq!(ayah.surah, surah_number, "Ayah surah number should match");
                    prop_assert!(ayah.ayah > 0, "Ayah number should be positive");
                    prop_assert!(!ayah.text_arabic.is_empty(), "Ayah text should not be empty");
                }

                Ok(())
            })?;
        }

        /// Property: Ayah responses have valid structure
        #[test]
        fn prop_ayah_response_valid_structure(surah in 1u8..=114, ayah in 1u16..=10) {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let manager = create_test_manager_with_real_apis().await;

                // Clear cache to ensure we hit the real APIs
                let cache_key = format!("quran:ayah:{}:{}", surah, ayah);
                let _ = manager.clear_cache(&cache_key).await;

                let result = manager.get_ayah(surah, ayah).await;
                prop_assert!(result.is_ok(), "Request should succeed with real API");

                let ayah_data = result.unwrap();

                // Validate response structure from real API
                prop_assert_eq!(ayah_data.surah, surah, "Surah number should match request");
                prop_assert_eq!(ayah_data.ayah, ayah, "Ayah number should match request");
                prop_assert!(!ayah_data.text_arabic.is_empty(), "Arabic text should not be empty");

                Ok(())
            })?;
        }

        /// Property: Page responses have valid structure
        #[test]
        fn prop_page_response_valid_structure(page in 1u16..=604) {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let manager = create_test_manager_with_real_apis().await;

                // Clear cache to ensure we hit the real APIs
                let cache_key = format!("quran:page:{}", page);
                let _ = manager.clear_cache(&cache_key).await;

                let result = manager.get_page(page).await;
                prop_assert!(result.is_ok(), "Request should succeed with real API");

                let page_data = result.unwrap();

                // Validate response structure from real API
                prop_assert_eq!(page_data.page_number, page, "Page number should match request");
                prop_assert!(!page_data.ayahs.is_empty(), "Page should have at least one ayah");

                // Validate each ayah
                for ayah in &page_data.ayahs {
                    prop_assert!(ayah.surah >= 1 && ayah.surah <= 114, "Surah number should be valid");
                    prop_assert!(ayah.ayah > 0, "Ayah number should be positive");
                    prop_assert!(!ayah.text_arabic.is_empty(), "Ayah text should not be empty");
                }

                Ok(())
            })?;
        }

        /// Property: Invalid surah numbers are rejected
        #[test]
        fn prop_invalid_surah_rejected(surah_number in 115u8..=255) {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let manager = create_test_manager_with_real_apis().await;

                let result = manager.get_surah(surah_number).await;
                prop_assert!(result.is_err(), "Invalid surah number should be rejected");

                Ok(())
            })?;
        }

        /// Property: Invalid ayah numbers are rejected
        #[test]
        fn prop_invalid_ayah_rejected(surah in 1u8..=114) {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let manager = create_test_manager_with_real_apis().await;

                let result = manager.get_ayah(surah, 0).await;
                prop_assert!(result.is_err(), "Invalid ayah number (0) should be rejected");

                Ok(())
            })?;
        }

        /// Property: Invalid page numbers are rejected
        #[test]
        fn prop_invalid_page_rejected(page in 605u16..=1000) {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let manager = create_test_manager_with_real_apis().await;

                let result = manager.get_page(page).await;
                prop_assert!(result.is_err(), "Invalid page number should be rejected");

                Ok(())
            })?;
        }

        /// Property: Ayah numbers in surah are sequential
        #[test]
        fn prop_ayah_numbers_sequential(surah_number in 1u8..=114) {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let manager = create_test_manager_with_real_apis().await;

                // Clear cache to ensure we hit the real APIs
                let cache_key = format!("quran:surah:{}", surah_number);
                let _ = manager.clear_cache(&cache_key).await;

                let result = manager.get_surah(surah_number).await;
                prop_assert!(result.is_ok(), "Request should succeed with real API");

                let surah = result.unwrap();

                // Check that ayah numbers are sequential
                if surah.ayahs.len() > 1 {
                    for i in 1..surah.ayahs.len() {
                        prop_assert!(
                            surah.ayahs[i].ayah > surah.ayahs[i-1].ayah,
                            "Ayah numbers should be sequential"
                        );
                    }
                }

                Ok(())
            })?;
        }
    }
}

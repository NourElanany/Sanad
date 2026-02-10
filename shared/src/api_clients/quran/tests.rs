//! Unit tests for Quran API clients
//!
//! These tests verify specific examples and edge cases for each client.

#[cfg(test)]
mod quran_com_client_tests {
    use crate::api_clients::quran::QuranComClient;
    use crate::api_clients::{ApiClient, QuranApiClient};

    #[test]
    fn test_client_initialization() {
        let client = QuranComClient::new(None);
        assert_eq!(client.api_name(), "quran.com");
        assert_eq!(client.priority(), 1);
    }

    #[test]
    fn test_client_with_api_key() {
        let client = QuranComClient::new(Some("test_key".to_string()));
        assert_eq!(client.api_name(), "quran.com");
    }

    #[test]
    fn test_rate_limit_configuration() {
        let client = QuranComClient::new(None);
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.requests_per_hour, 1000);
        assert_eq!(config.requests_per_day, 10000);
    }

    #[tokio::test]
    async fn test_validation_surah_zero() {
        let client = QuranComClient::new(None);
        let result = client.get_surah(0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validation_surah_too_high() {
        let client = QuranComClient::new(None);
        let result = client.get_surah(115).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validation_ayah_zero() {
        let client = QuranComClient::new(None);
        let result = client.get_ayah(1, 0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validation_page_zero() {
        let client = QuranComClient::new(None);
        let result = client.get_page(0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validation_page_too_high() {
        let client = QuranComClient::new(None);
        let result = client.get_page(605).await;
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod alquran_cloud_client_tests {
    use crate::api_clients::quran::AlquranCloudClient;
    use crate::api_clients::{ApiClient, QuranApiClient};

    #[test]
    fn test_client_initialization() {
        let client = AlquranCloudClient::new();
        assert_eq!(client.api_name(), "alquran.cloud");
        assert_eq!(client.priority(), 2);
    }

    #[test]
    fn test_client_with_custom_edition() {
        let client = AlquranCloudClient::with_edition("en.sahih".to_string());
        assert_eq!(client.api_name(), "alquran.cloud");
    }

    #[test]
    fn test_default_edition() {
        let client = AlquranCloudClient::new();
        // Edition is private, but we can test that the client was created
        assert_eq!(client.api_name(), "alquran.cloud");
    }

    #[test]
    fn test_rate_limit_configuration() {
        let client = AlquranCloudClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 30);
        assert_eq!(config.requests_per_hour, 500);
        assert_eq!(config.requests_per_day, 5000);
    }

    #[tokio::test]
    async fn test_validation_surah_boundaries() {
        let client = AlquranCloudClient::new();
        
        // Test lower boundary
        let result = client.get_surah(0).await;
        assert!(result.is_err());
        
        // Test upper boundary
        let result = client.get_surah(115).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validation_ayah_boundaries() {
        let client = AlquranCloudClient::new();
        
        // Test zero ayah
        let result = client.get_ayah(1, 0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validation_page_boundaries() {
        let client = AlquranCloudClient::new();
        
        // Test lower boundary
        let result = client.get_page(0).await;
        assert!(result.is_err());
        
        // Test upper boundary
        let result = client.get_page(605).await;
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tanzil_client_tests {
    use crate::api_clients::quran::TanzilClient;
    use crate::api_clients::{ApiClient, QuranApiClient};

    #[test]
    fn test_client_initialization() {
        let client = TanzilClient::new();
        assert_eq!(client.api_name(), "tanzil.net");
        assert_eq!(client.priority(), 3);
    }

    #[test]
    fn test_client_with_custom_text_type() {
        let client = TanzilClient::with_text_type("simple".to_string());
        assert_eq!(client.api_name(), "tanzil.net");
    }

    #[test]
    fn test_rate_limit_configuration() {
        let client = TanzilClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 20);
        assert_eq!(config.requests_per_hour, 300);
        assert_eq!(config.requests_per_day, 3000);
    }

    #[test]
    fn test_parse_valid_line() {
        let client = TanzilClient::new();
        let line = "1|1|بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
        let result = client.parse_line(line);
        
        assert!(result.is_some());
        let (surah, ayah, text) = result.unwrap();
        assert_eq!(surah, 1);
        assert_eq!(ayah, 1);
        assert_eq!(text, "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ");
    }

    #[test]
    fn test_parse_invalid_line() {
        let client = TanzilClient::new();
        
        // Test line with insufficient parts
        let line = "1|1";
        let result = client.parse_line(line);
        assert!(result.is_none());
        
        // Test line with non-numeric surah
        let line = "abc|1|text";
        let result = client.parse_line(line);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_line_with_pipe_in_text() {
        let client = TanzilClient::new();
        let line = "2|255|اللَّهُ لَا إِلَٰهَ إِلَّا هُوَ|extra";
        let result = client.parse_line(line);
        
        assert!(result.is_some());
        let (surah, ayah, text) = result.unwrap();
        assert_eq!(surah, 2);
        assert_eq!(ayah, 255);
        // Text should be the third part only
        assert_eq!(text, "اللَّهُ لَا إِلَٰهَ إِلَّا هُوَ");
    }

    #[tokio::test]
    async fn test_validation_surah_boundaries() {
        let client = TanzilClient::new();
        
        let result = client.get_surah(0).await;
        assert!(result.is_err());
        
        let result = client.get_surah(115).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validation_ayah_boundaries() {
        let client = TanzilClient::new();
        
        let result = client.get_ayah(1, 0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_page_not_supported() {
        let client = TanzilClient::new();
        
        // Tanzil doesn't support page-based access
        let result = client.get_page(1).await;
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod everyayah_client_tests {
    use crate::api_clients::quran::everyayah_client::{EveryayahClient, Reciter};
    use crate::api_clients::{ApiClient, QuranApiClient};

    #[test]
    fn test_client_initialization() {
        let client = EveryayahClient::new();
        assert_eq!(client.api_name(), "everyayah.com");
        assert_eq!(client.priority(), 1);
    }

    #[test]
    fn test_client_with_custom_reciter() {
        let client = EveryayahClient::with_reciter("Alafasy_128kbps".to_string());
        assert_eq!(client.api_name(), "everyayah.com");
    }

    #[test]
    fn test_rate_limit_configuration() {
        let client = EveryayahClient::new();
        let config = client.rate_limit();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.requests_per_hour, 1000);
        assert_eq!(config.requests_per_day, 10000);
    }

    #[test]
    fn test_audio_url_generation() {
        let client = EveryayahClient::new();
        
        // Test Al-Fatihah, verse 1
        let url = client.get_audio_url(1, 1);
        assert!(url.contains("001001.mp3"));
        assert!(url.contains("Abdul_Basit_Murattal_192kbps"));
        
        // Test Al-Baqarah, verse 255
        let url = client.get_audio_url(2, 255);
        assert!(url.contains("002255.mp3"));
    }

    #[test]
    fn test_audio_url_format() {
        let client = EveryayahClient::new();
        
        // Test that surah and ayah are zero-padded to 3 digits
        let url = client.get_audio_url(1, 1);
        assert!(url.ends_with("001001.mp3"));
        
        let url = client.get_audio_url(114, 6);
        assert!(url.ends_with("114006.mp3"));
    }

    #[test]
    fn test_reciter_directory_names() {
        assert_eq!(
            Reciter::AbdulBasitMurattal.directory_name(),
            "Abdul_Basit_Murattal_192kbps"
        );
        assert_eq!(
            Reciter::MisharyRashidAlafasy.directory_name(),
            "Alafasy_128kbps"
        );
        assert_eq!(
            Reciter::MaherAlMuaiqly.directory_name(),
            "MaherAlMuaiqly128kbps"
        );
    }

    #[test]
    fn test_reciter_display_names() {
        assert_eq!(
            Reciter::AbdulBasitMurattal.display_name(),
            "Abdul Basit (Murattal)"
        );
        assert_eq!(
            Reciter::MisharyRashidAlafasy.display_name(),
            "Mishary Rashid Alafasy"
        );
    }

    #[tokio::test]
    async fn test_get_surah_not_supported() {
        let client = EveryayahClient::new();
        
        // EveryAyah only provides audio, not text
        let result = client.get_surah(1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_page_not_supported() {
        let client = EveryayahClient::new();
        
        // EveryAyah doesn't support page-based access
        let result = client.get_page(1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validation_surah_boundaries() {
        let client = EveryayahClient::new();
        
        let result = client.get_surah(0).await;
        assert!(result.is_err());
        
        let result = client.get_surah(115).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validation_ayah_boundaries() {
        let client = EveryayahClient::new();
        
        let result = client.get_ayah(1, 0).await;
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod quran_api_manager_tests {
    use crate::api_clients::quran::{AlquranCloudClient, QuranApiManager, QuranComClient, TanzilClient};
    use crate::api_clients::{ApiClient, CacheManager, QuranApiClient, RateLimiter};
    use std::collections::HashMap;
    use std::sync::Arc;

    async fn create_test_manager() -> QuranApiManager {
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

    #[tokio::test]
    async fn test_manager_initialization() {
        let manager = create_test_manager().await;
        assert_eq!(manager.client_count(), 3);
    }

    #[tokio::test]
    async fn test_clients_sorted_by_priority() {
        let manager = create_test_manager().await;
        let names = manager.client_names();
        
        // Should be sorted: quran.com (1), alquran.cloud (2), tanzil.net (3)
        assert_eq!(names[0], "quran.com");
        assert_eq!(names[1], "alquran.cloud");
        assert_eq!(names[2], "tanzil.net");
    }

    #[tokio::test]
    async fn test_manager_with_reversed_priority_clients() {
        let cache = Arc::new(CacheManager::new("redis://127.0.0.1:6379/")
            .await
            .expect("Failed to create cache manager"));
        
        let rate_limiter = Arc::new(RateLimiter::new(
            "redis://127.0.0.1:6379/",
            HashMap::new()
        )
            .await
            .expect("Failed to create rate limiter"));

        // Add clients in reverse priority order
        let clients: Vec<Box<dyn QuranApiClient + Send + Sync>> = vec![
            Box::new(TanzilClient::new()),        // Priority 3
            Box::new(AlquranCloudClient::new()),  // Priority 2
            Box::new(QuranComClient::new(None)),  // Priority 1
        ];

        let manager = QuranApiManager::new(clients, cache, rate_limiter);
        let names = manager.client_names();
        
        // Should still be sorted correctly
        assert_eq!(names[0], "quran.com");
        assert_eq!(names[1], "alquran.cloud");
        assert_eq!(names[2], "tanzil.net");
    }

    #[tokio::test]
    async fn test_manager_validation_surah() {
        let manager = create_test_manager().await;
        
        // Test invalid surah numbers
        let result = manager.get_surah(0).await;
        assert!(result.is_err());
        
        let result = manager.get_surah(115).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_manager_validation_ayah() {
        let manager = create_test_manager().await;
        
        // Test invalid ayah number
        let result = manager.get_ayah(1, 0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_manager_validation_page() {
        let manager = create_test_manager().await;
        
        // Test invalid page numbers
        let result = manager.get_page(0).await;
        assert!(result.is_err());
        
        let result = manager.get_page(605).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_manager_with_single_client() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
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
            ];

            let manager = QuranApiManager::new(clients, cache, rate_limiter);
            assert_eq!(manager.client_count(), 1);
            assert_eq!(manager.client_names()[0], "quran.com");
        });
    }
}

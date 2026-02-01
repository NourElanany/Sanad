use crate::models::*;
use crate::service::I18nService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use shared::models::ApiResponse;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use tracing::{info, error};

/// HTTP handlers for the internationalization service
pub struct I18nHandlers;

impl I18nHandlers {
    /// Get translation for a specific key
    pub async fn get_translation(
        State(service): State<Arc<I18nService>>,
        Query(params): Query<GetTranslationParams>,
    ) -> Result<Json<ApiResponse<TranslationResponse>>, StatusCode> {
        let request = TranslationRequest {
            key: params.key,
            namespace: params.namespace,
            language: params.language.unwrap_or(SupportedLanguage::Arabic),
            fallback_languages: params.fallback_languages,
            interpolation_values: params.interpolation_values,
            plural_count: params.plural_count,
        };

        match service.get_translation(request).await {
            Ok(translation) => Ok(Json(ApiResponse::success(translation))),
            Err(e) => {
                error!("Failed to get translation: {}", e);
                Err(StatusCode::NOT_FOUND)
            }
        }
    }

    /// Get multiple translations at once
    pub async fn get_bulk_translations(
        State(service): State<Arc<I18nService>>,
        Json(request): Json<BulkTranslationRequest>,
    ) -> Result<Json<ApiResponse<BulkTranslationResponse>>, StatusCode> {
        match service.get_bulk_translations(request).await {
            Ok(response) => Ok(Json(ApiResponse::success(response))),
            Err(e) => {
                error!("Failed to get bulk translations: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Switch user language
    pub async fn switch_language(
        State(service): State<Arc<I18nService>>,
        Json(request): Json<LanguageSwitchRequest>,
    ) -> Result<Json<ApiResponse<LanguageSwitchResponse>>, StatusCode> {
        match service.switch_language(request).await {
            Ok(response) => {
                info!("Language switched successfully to {}", response.new_language.code());
                Ok(Json(ApiResponse::success(response)))
            }
            Err(e) => {
                error!("Failed to switch language: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Detect language from text
    pub async fn detect_language(
        State(service): State<Arc<I18nService>>,
        Json(request): Json<LanguageDetectionRequest>,
    ) -> Result<Json<ApiResponse<LanguageDetectionResult>>, StatusCode> {
        match service.detect_language(&request.text).await {
            Ok(result) => Ok(Json(ApiResponse::success(result))),
            Err(e) => {
                error!("Failed to detect language: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Get user language preferences
    pub async fn get_user_preferences(
        State(service): State<Arc<I18nService>>,
        Path(user_id): Path<Uuid>,
    ) -> Result<Json<ApiResponse<UserLanguagePreferences>>, StatusCode> {
        match service.get_user_preferences(user_id).await {
            Ok(preferences) => Ok(Json(ApiResponse::success(preferences))),
            Err(e) => {
                error!("Failed to get user preferences: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Update user language preferences
    pub async fn update_user_preferences(
        State(service): State<Arc<I18nService>>,
        Json(preferences): Json<UserLanguagePreferences>,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        match service.update_user_preferences(preferences).await {
            Ok(_) => Ok(Json(ApiResponse::success(()))),
            Err(e) => {
                error!("Failed to update user preferences: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Get supported languages
    pub async fn get_supported_languages(
        State(service): State<Arc<I18nService>>,
    ) -> Result<Json<ApiResponse<Vec<SupportedLanguage>>>, StatusCode> {
        match service.get_supported_languages().await {
            Ok(languages) => Ok(Json(ApiResponse::success(languages))),
            Err(e) => {
                error!("Failed to get supported languages: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Get language information
    pub async fn get_language_info(
        State(service): State<Arc<I18nService>>,
        Path(language_code): Path<String>,
    ) -> Result<Json<ApiResponse<crate::service::LanguageInfo>>, StatusCode> {
        let language = match SupportedLanguage::from_code(&language_code) {
            Some(lang) => lang,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        match service.get_language_info(&language).await {
            Ok(info) => Ok(Json(ApiResponse::success(info))),
            Err(e) => {
                error!("Failed to get language info: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Get available translations for content
    pub async fn get_available_translations(
        State(service): State<Arc<I18nService>>,
        Path(content_id): Path<Uuid>,
    ) -> Result<Json<ApiResponse<Option<AvailableTranslations>>>, StatusCode> {
        match service.get_available_translations(content_id).await {
            Ok(translations) => Ok(Json(ApiResponse::success(translations))),
            Err(e) => {
                error!("Failed to get available translations: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Generate CSS for all languages
    pub async fn generate_languages_css(
        State(service): State<Arc<I18nService>>,
    ) -> Result<String, StatusCode> {
        match service.generate_all_languages_css().await {
            Ok(css) => Ok(css),
            Err(e) => {
                error!("Failed to generate CSS: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Reload translations from files
    pub async fn reload_translations(
        State(service): State<Arc<I18nService>>,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        match service.reload_translations().await {
            Ok(_) => {
                info!("Translations reloaded successfully");
                Ok(Json(ApiResponse::success(())))
            }
            Err(e) => {
                error!("Failed to reload translations: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Get translation statistics
    pub async fn get_translation_stats(
        State(service): State<Arc<I18nService>>,
    ) -> Result<Json<ApiResponse<crate::service::TranslationStats>>, StatusCode> {
        match service.get_translation_stats().await {
            Ok(stats) => Ok(Json(ApiResponse::success(stats))),
            Err(e) => {
                error!("Failed to get translation stats: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Detect language from HTTP headers
    pub async fn detect_language_from_headers(
        State(service): State<Arc<I18nService>>,
        headers: axum::http::HeaderMap,
    ) -> Result<Json<ApiResponse<Option<SupportedLanguage>>>, StatusCode> {
        let accept_language = headers
            .get("accept-language")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");

        match service.detect_language_from_headers(accept_language).await {
            Ok(language) => Ok(Json(ApiResponse::success(language))),
            Err(e) => {
                error!("Failed to detect language from headers: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Health check endpoint
    pub async fn health_check() -> Result<Json<ApiResponse<HealthStatus>>, StatusCode> {
        Ok(Json(ApiResponse::success(HealthStatus {
            status: "healthy".to_string(),
            service: "i18n-service".to_string(),
            version: "1.0.0".to_string(),
        })))
    }
}

// Request/Response DTOs

#[derive(Debug, Deserialize)]
pub struct GetTranslationParams {
    pub key: String,
    pub namespace: Option<String>,
    pub language: Option<SupportedLanguage>,
    pub fallback_languages: Option<Vec<SupportedLanguage>>,
    pub interpolation_values: Option<HashMap<String, String>>,
    pub plural_count: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct LanguageDetectionRequest {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct HeaderLanguageDetection {
    pub accept_language: String,
}

#[derive(Debug, Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub service: String,
    pub version: String,
}

// Router setup
use axum::{
    routing::{get, post, put},
    Router,
};

pub fn create_i18n_routes(service: Arc<I18nService>) -> Router {
    Router::new()
        // Translation endpoints
        .route("/translations", get(I18nHandlers::get_translation))
        .route("/translations/bulk", post(I18nHandlers::get_bulk_translations))
        .route("/translations/reload", post(I18nHandlers::reload_translations))
        .route("/translations/stats", get(I18nHandlers::get_translation_stats))
        
        // Language management
        .route("/languages", get(I18nHandlers::get_supported_languages))
        .route("/languages/:code", get(I18nHandlers::get_language_info))
        .route("/languages/switch", post(I18nHandlers::switch_language))
        .route("/languages/detect", post(I18nHandlers::detect_language))
        .route("/languages/detect/headers", get(I18nHandlers::detect_language_from_headers))
        
        // User preferences
        .route("/users/:user_id/preferences", get(I18nHandlers::get_user_preferences))
        .route("/users/preferences", put(I18nHandlers::update_user_preferences))
        
        // Content translations
        .route("/content/:content_id/translations", get(I18nHandlers::get_available_translations))
        
        // CSS generation
        .route("/css/languages", get(I18nHandlers::generate_languages_css))
        
        // Health check
        .route("/health", get(I18nHandlers::health_check))
        
        .with_state(service)
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use axum::http::StatusCode;
    // use axum_test::TestServer;  // Commented out until we add the dependency
    // use std::sync::Arc;

    // Mock service for testing
    struct MockI18nService;

    #[tokio::test]
    #[ignore] // Ignore until we have proper test setup
    async fn test_health_check() {
        // let app = Router::new()
        //     .route("/health", get(I18nHandlers::health_check));
        
        // let server = TestServer::new(app).unwrap();
        // let response = server.get("/health").await;
        
        // assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    #[ignore] // Ignore until we have proper test setup
    async fn test_get_supported_languages() {
        // This would test the supported languages endpoint
        // let service = Arc::new(MockI18nService);
        // let app = create_i18n_routes(service);
        // let server = TestServer::new(app).unwrap();
        // let response = server.get("/languages").await;
        // assert_eq!(response.status_code(), StatusCode::OK);
    }
}
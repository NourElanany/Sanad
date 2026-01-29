use crate::models::*;
use crate::service::QuranService;
use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post},
    Router,
};
use shared::{ApiResponse, AppError};
use std::collections::HashMap;
use uuid::Uuid;

/// Create the router for Quran service endpoints
pub fn create_router(service: QuranService) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/surahs", get(get_all_surahs))
        .route("/surahs/:surah_number", get(get_surah))
        .route("/surahs/:surah_number/ayahs", get(get_ayahs_by_surah))
        .route("/surahs/:surah_number/ayahs/:ayah_number", get(get_ayah))
        .route("/surahs/:surah_number/ayahs/:ayah_number/tafsir", get(get_tafsir))
        .route("/surahs/:surah_number/ayahs/:ayah_number/tafsir/compare", post(compare_tafsir))
        .route("/surahs/:surah_number/ayahs/:ayah_number/translations", get(get_translations))
        .route("/surahs/:surah_number/ayahs/:ayah_number/navigation", get(get_ayah_navigation))
        .route("/surahs/revelation/:revelation_type", get(get_surahs_by_revelation_type))
        .route("/search", get(search_quran))
        .route("/search/advanced", get(advanced_search))
        .route("/search/suggestions", get(get_search_suggestions))
        .route("/juz/:juz_number/ayahs", get(get_ayahs_by_juz))
        .route("/pages/:page_number/ayahs", get(get_ayahs_by_page))
        .route("/tafsir/sources", get(get_tafsir_sources))
        .route("/tafsir/sources/manage", post(manage_tafsir_source))
        .route("/tafsir/search/advanced", get(advanced_tafsir_search))
        .route("/tafsir/analytics", get(get_tafsir_analytics))
        .route("/recitation/styles", get(get_recitation_styles))
        .route("/statistics", get(get_statistics))
        .route("/integrity/verify", post(verify_integrity))
        .route("/ayahs/range", get(get_ayah_range))
        .with_state(service)
}

/// Health check endpoint
async fn health_check() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "quran-service".to_string());
    Json(ApiResponse::success(status))
}

/// Get all Surahs
async fn get_all_surahs(
    State(service): State<QuranService>,
) -> Result<Json<ApiResponse<Vec<Surah>>>, AppError> {
    let surahs = service.get_all_surahs().await?;
    Ok(Json(ApiResponse::success(surahs)))
}

/// Get a specific Surah
async fn get_surah(
    State(service): State<QuranService>,
    Path(surah_number): Path<i32>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<SurahResponse>>, AppError> {
    let include_ayahs = params.get("include_ayahs")
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);

    let request = GetSurahRequest {
        surah_number,
        include_ayahs: Some(include_ayahs),
    };

    match service.get_surah(request).await? {
        Some(response) => Ok(Json(ApiResponse::success(response))),
        None => Err(AppError::NotFound("Surah not found".to_string())),
    }
}

/// Get Ayahs for a specific Surah
async fn get_ayahs_by_surah(
    State(service): State<QuranService>,
    Path(surah_number): Path<i32>,
) -> Result<Json<ApiResponse<Vec<Ayah>>>, AppError> {
    match service.get_surah_with_ayahs(surah_number).await? {
        Some(surah_with_ayahs) => Ok(Json(ApiResponse::success(surah_with_ayahs.ayahs))),
        None => Err(AppError::NotFound("Surah not found".to_string())),
    }
}

/// Get a specific Ayah
async fn get_ayah(
    State(service): State<QuranService>,
    Path((surah_number, ayah_number)): Path<(i32, i32)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<AyahResponse>>, AppError> {
    let include_tafsir = params.get("include_tafsir")
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);

    let request = GetAyahRequest {
        surah_number,
        ayah_number,
        include_tafsir: Some(include_tafsir),
    };

    match service.get_ayah(request).await? {
        Some(response) => Ok(Json(ApiResponse::success(response))),
        None => Err(AppError::NotFound("Ayah not found".to_string())),
    }
}

/// Get Tafsir for a specific Ayah
async fn get_tafsir(
    State(service): State<QuranService>,
    Path((surah_number, ayah_number)): Path<(i32, i32)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<TafsirResponse>>, AppError> {
    let source_ids = params.get("source_ids")
        .and_then(|ids_str| {
            ids_str.split(',')
                .map(|id| Uuid::parse_str(id.trim()).ok())
                .collect::<Option<Vec<Uuid>>>()
        });

    let request = GetTafsirRequest {
        surah_number,
        ayah_number,
        source_ids,
    };

    match service.get_tafsir(request).await? {
        Some(response) => Ok(Json(ApiResponse::success(response))),
        None => Err(AppError::NotFound("Ayah not found".to_string())),
    }
}

/// Get navigation info for an Ayah
async fn get_ayah_navigation(
    State(service): State<QuranService>,
    Path((surah_number, ayah_number)): Path<(i32, i32)>,
) -> Result<Json<ApiResponse<crate::service::AyahNavigation>>, AppError> {
    match service.get_ayah_navigation(surah_number, ayah_number).await? {
        Some(navigation) => Ok(Json(ApiResponse::success(navigation))),
        None => Err(AppError::NotFound("Ayah not found".to_string())),
    }
}

/// Search in Quran
async fn search_quran(
    State(service): State<QuranService>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<SearchResponse>>, AppError> {
    let query = params.get("q")
        .ok_or_else(|| AppError::BadRequest("Query parameter 'q' is required".to_string()))?
        .clone();

    let surah_numbers = params.get("surahs")
        .and_then(|surahs_str| {
            surahs_str.split(',')
                .map(|s| s.trim().parse::<i32>().ok())
                .collect::<Option<Vec<i32>>>()
        });

    let search_type = params.get("type")
        .and_then(|t| match t.as_str() {
            "text" => Some(SearchType::Text),
            "semantic" => Some(SearchType::Semantic),
            "root" => Some(SearchType::Root),
            "exact" => Some(SearchType::Exact),
            _ => None,
        });

    let revelation_type = params.get("revelation")
        .and_then(|r| match r.as_str() {
            "meccan" => Some(RevelationType::Meccan),
            "medinan" => Some(RevelationType::Medinan),
            _ => None,
        });

    let juz_numbers = params.get("juz")
        .and_then(|juz_str| {
            juz_str.split(',')
                .map(|j| j.trim().parse::<i32>().ok())
                .collect::<Option<Vec<i32>>>()
        });

    let limit = params.get("limit")
        .and_then(|l| l.parse::<i32>().ok())
        .unwrap_or(20);

    let offset = params.get("offset")
        .and_then(|o| o.parse::<i32>().ok())
        .unwrap_or(0);

    let request = SearchQuranRequest {
        query,
        surah_numbers,
        search_type,
        revelation_type,
        juz_numbers,
        limit: Some(limit),
        offset: Some(offset),
    };

    let response = service.search_quran(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

/// Get all Tafsir sources
async fn get_tafsir_sources(
    State(service): State<QuranService>,
) -> Result<Json<ApiResponse<Vec<TafsirSource>>>, AppError> {
    let sources = service.get_tafsir_sources().await?;
    Ok(Json(ApiResponse::success(sources)))
}

/// Get Quran statistics
async fn get_statistics(
    State(service): State<QuranService>,
) -> Result<Json<ApiResponse<crate::service::QuranStatistics>>, AppError> {
    let stats = service.get_quran_statistics().await?;
    Ok(Json(ApiResponse::success(stats)))
}

/// Verify content integrity
async fn verify_integrity(
    State(service): State<QuranService>,
) -> Result<Json<ApiResponse<crate::service::ContentIntegrityReport>>, AppError> {
    let report = service.verify_content_integrity().await?;
    Ok(Json(ApiResponse::success(report)))
}

/// Get Ayah range (for Khatma planning)
async fn get_ayah_range(
    State(service): State<QuranService>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<Ayah>>>, AppError> {
    let start_surah = params.get("start_surah")
        .and_then(|s| s.parse::<i32>().ok())
        .ok_or_else(|| AppError::BadRequest("start_surah parameter is required".to_string()))?;

    let start_ayah = params.get("start_ayah")
        .and_then(|a| a.parse::<i32>().ok())
        .ok_or_else(|| AppError::BadRequest("start_ayah parameter is required".to_string()))?;

    let end_surah = params.get("end_surah")
        .and_then(|s| s.parse::<i32>().ok())
        .ok_or_else(|| AppError::BadRequest("end_surah parameter is required".to_string()))?;

    let end_ayah = params.get("end_ayah")
        .and_then(|a| a.parse::<i32>().ok())
        .ok_or_else(|| AppError::BadRequest("end_ayah parameter is required".to_string()))?;

    let ayahs = service.get_ayah_range(start_surah, start_ayah, end_surah, end_ayah).await?;
    Ok(Json(ApiResponse::success(ayahs)))
}

/// Get translations for a specific Ayah
async fn get_translations(
    State(service): State<QuranService>,
    Path((surah_number, ayah_number)): Path<(i32, i32)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<TranslationResponse>>, AppError> {
    let languages = params.get("languages")
        .map(|langs_str| {
            langs_str.split(',')
                .map(|l| l.trim().to_string())
                .collect::<Vec<String>>()
        });

    let request = GetTranslationRequest {
        surah_number,
        ayah_number,
        languages,
    };

    match service.get_translations(request).await? {
        Some(response) => Ok(Json(ApiResponse::success(response))),
        None => Err(AppError::NotFound("Ayah not found".to_string())),
    }
}

/// Get Surahs by revelation type
async fn get_surahs_by_revelation_type(
    State(service): State<QuranService>,
    Path(revelation_type): Path<String>,
) -> Result<Json<ApiResponse<Vec<Surah>>>, AppError> {
    let rev_type = match revelation_type.as_str() {
        "meccan" => RevelationType::Meccan,
        "medinan" => RevelationType::Medinan,
        _ => return Err(AppError::BadRequest("Invalid revelation type. Use 'meccan' or 'medinan'".to_string())),
    };

    let surahs = service.get_surahs_by_revelation_type(rev_type).await?;
    Ok(Json(ApiResponse::success(surahs)))
}

/// Advanced search with filters
async fn advanced_search(
    State(service): State<QuranService>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<SearchResponse>>, AppError> {
    let query = params.get("q")
        .ok_or_else(|| AppError::BadRequest("Query parameter 'q' is required".to_string()))?
        .clone();

    let filters = AdvancedSearchFilters {
        surah_numbers: params.get("surahs")
            .and_then(|s| s.split(',').map(|n| n.trim().parse().ok()).collect()),
        revelation_type: params.get("revelation")
            .and_then(|r| match r.as_str() {
                "meccan" => Some(RevelationType::Meccan),
                "medinan" => Some(RevelationType::Medinan),
                _ => None,
            }),
        juz_numbers: params.get("juz")
            .and_then(|j| j.split(',').map(|n| n.trim().parse().ok()).collect()),
        page_range: params.get("page_range")
            .and_then(|pr| {
                let parts: Vec<&str> = pr.split('-').collect();
                if parts.len() == 2 {
                    if let (Ok(start), Ok(end)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                        Some((start, end))
                    } else { None }
                } else { None }
            }),
        word_count_range: params.get("word_count_range")
            .and_then(|wcr| {
                let parts: Vec<&str> = wcr.split('-').collect();
                if parts.len() == 2 {
                    if let (Ok(start), Ok(end)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                        Some((start, end))
                    } else { None }
                } else { None }
            }),
        include_context: params.get("include_context")
            .and_then(|ic| ic.parse().ok()),
    };

    let limit = params.get("limit")
        .and_then(|l| l.parse::<i32>().ok())
        .unwrap_or(20);

    let offset = params.get("offset")
        .and_then(|o| o.parse::<i32>().ok())
        .unwrap_or(0);

    let response = service.advanced_search(&query, filters, limit, offset).await?;
    Ok(Json(ApiResponse::success(response)))
}

/// Get search suggestions
async fn get_search_suggestions(
    State(service): State<QuranService>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<String>>>, AppError> {
    let query = params.get("q")
        .ok_or_else(|| AppError::BadRequest("Query parameter 'q' is required".to_string()))?;

    let suggestions = service.get_search_suggestions(query).await?;
    Ok(Json(ApiResponse::success(suggestions)))
}

/// Get Ayahs by Juz
async fn get_ayahs_by_juz(
    State(service): State<QuranService>,
    Path(juz_number): Path<i32>,
) -> Result<Json<ApiResponse<Vec<Ayah>>>, AppError> {
    let ayahs = service.get_ayahs_by_juz(juz_number).await?;
    Ok(Json(ApiResponse::success(ayahs)))
}

/// Get Ayahs by page
async fn get_ayahs_by_page(
    State(service): State<QuranService>,
    Path(page_number): Path<i32>,
) -> Result<Json<ApiResponse<Vec<Ayah>>>, AppError> {
    let ayahs = service.get_ayahs_by_page(page_number).await?;
    Ok(Json(ApiResponse::success(ayahs)))
}

/// Get all available recitation styles
async fn get_recitation_styles(
    State(service): State<QuranService>,
) -> Result<Json<ApiResponse<Vec<RecitationStyle>>>, AppError> {
    let styles = service.get_recitation_styles().await?;
    Ok(Json(ApiResponse::success(styles)))
}

/// Compare Tafsir interpretations
async fn compare_tafsir(
    State(service): State<QuranService>,
    Path((surah_number, ayah_number)): Path<(i32, i32)>,
    Json(request): Json<TafsirComparisonRequest>,
) -> Result<Json<ApiResponse<TafsirComparisonResponse>>, AppError> {
    // Validate that the path parameters match the request
    if request.surah_number != surah_number || request.ayah_number != ayah_number {
        return Err(AppError::BadRequest("Path parameters don't match request body".to_string()));
    }

    match service.compare_tafsir(request).await? {
        Some(response) => Ok(Json(ApiResponse::success(response))),
        None => Err(AppError::NotFound("Ayah or Tafsir entries not found".to_string())),
    }
}

/// Manage Tafsir sources
async fn manage_tafsir_source(
    State(service): State<QuranService>,
    Json(request): Json<ManageTafsirSourceRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let result = service.manage_tafsir_source(request).await?;
    Ok(Json(ApiResponse::success(result)))
}

/// Advanced Tafsir search
async fn advanced_tafsir_search(
    State(service): State<QuranService>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<AdvancedTafsirSearchResponse>>, AppError> {
    let query = params.get("q")
        .ok_or_else(|| AppError::BadRequest("Query parameter 'q' is required".to_string()))?
        .clone();

    // Parse search criteria
    let search_criteria = params.get("criteria")
        .map(|criteria_str| {
            criteria_str.split(',')
                .filter_map(|c| match c.trim() {
                    "text_content" => Some(TafsirSearchCriteria::TextContent),
                    "themes" => Some(TafsirSearchCriteria::Themes),
                    "cross_references" => Some(TafsirSearchCriteria::CrossReferences),
                    "author_name" => Some(TafsirSearchCriteria::AuthorName),
                    "methodology" => Some(TafsirSearchCriteria::Methodology),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec![TafsirSearchCriteria::TextContent]);

    // Parse source filters
    let source_filters = if params.contains_key("source_types") || 
                           params.contains_key("auth_levels") || 
                           params.contains_key("languages") {
        Some(TafsirSourceFilters {
            source_types: params.get("source_types")
                .map(|types_str| {
                    types_str.split(',')
                        .filter_map(|t| match t.trim() {
                            "classical" => Some(TafsirSourceType::Classical),
                            "contemporary" => Some(TafsirSourceType::Contemporary),
                            "linguistic" => Some(TafsirSourceType::Linguistic),
                            "thematic" => Some(TafsirSourceType::Thematic),
                            "sectarian" => Some(TafsirSourceType::Sectarian),
                            _ => None,
                        })
                        .collect()
                }),
            authentication_levels: params.get("auth_levels")
                .map(|levels_str| {
                    levels_str.split(',')
                        .filter_map(|l| match l.trim() {
                            "highly_authenticated" => Some(ScholarlyAuthentication::HighlyAuthenticated),
                            "authenticated" => Some(ScholarlyAuthentication::Authenticated),
                            "verified" => Some(ScholarlyAuthentication::Verified),
                            "unverified" => Some(ScholarlyAuthentication::Unverified),
                            _ => None,
                        })
                        .collect()
                }),
            languages: params.get("languages")
                .map(|langs_str| {
                    langs_str.split(',')
                        .map(|l| l.trim().to_string())
                        .collect()
                }),
            credibility_range: params.get("credibility_range")
                .and_then(|range_str| {
                    let parts: Vec<&str> = range_str.split('-').collect();
                    if parts.len() == 2 {
                        if let (Ok(min), Ok(max)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                            Some((min, max))
                        } else { None }
                    } else { None }
                }),
            publication_year_range: params.get("year_range")
                .and_then(|range_str| {
                    let parts: Vec<&str> = range_str.split('-').collect();
                    if parts.len() == 2 {
                        if let (Ok(min), Ok(max)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                            Some((min, max))
                        } else { None }
                    } else { None }
                }),
        })
    } else {
        None
    };

    let limit = params.get("limit")
        .and_then(|l| l.parse::<i32>().ok())
        .unwrap_or(20);

    let offset = params.get("offset")
        .and_then(|o| o.parse::<i32>().ok())
        .unwrap_or(0);

    let request = AdvancedTafsirSearchRequest {
        query,
        search_criteria,
        source_filters,
        limit: Some(limit),
        offset: Some(offset),
    };

    let response = service.advanced_tafsir_search(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

/// Get Tafsir analytics
async fn get_tafsir_analytics(
    State(service): State<QuranService>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<TafsirAnalyticsResponse>>, AppError> {
    let analysis_type = params.get("type")
        .and_then(|t| match t.as_str() {
            "coverage" => Some(AnalysisType::Coverage),
            "themes" => Some(AnalysisType::Themes),
            "methodology" => Some(AnalysisType::Methodology),
            "consensus" => Some(AnalysisType::Consensus),
            _ => None,
        })
        .unwrap_or(AnalysisType::Coverage);

    let surah_number = params.get("surah")
        .and_then(|s| s.parse::<i32>().ok());

    let ayah_range = params.get("ayah_range")
        .and_then(|range_str| {
            let parts: Vec<&str> = range_str.split('-').collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    Some((start, end))
                } else { None }
            } else { None }
        });

    let source_ids = params.get("sources")
        .and_then(|sources_str| {
            sources_str.split(',')
                .map(|id| Uuid::parse_str(id.trim()).ok())
                .collect::<Option<Vec<Uuid>>>()
        });

    let request = TafsirAnalyticsRequest {
        surah_number,
        ayah_range,
        source_ids,
        analysis_type,
    };

    let response = service.get_tafsir_analytics(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_parameter_parsing() {
        // Test parsing of query parameters
        let mut params = HashMap::new();
        params.insert("include_ayahs".to_string(), "true".to_string());
        
        let include_ayahs = params.get("include_ayahs")
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(false);
        
        assert!(include_ayahs);
    }

    #[test]
    fn test_uuid_parsing() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let parsed_uuid = Uuid::parse_str(uuid_str);
        assert!(parsed_uuid.is_ok());
    }

    #[test]
    fn test_surah_numbers_parsing() {
        let surahs_str = "1,2,3,4,5";
        let surah_numbers: Option<Vec<i32>> = surahs_str.split(',')
            .map(|s| s.trim().parse::<i32>().ok())
            .collect();
        
        assert!(surah_numbers.is_some());
        let numbers = surah_numbers.unwrap();
        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
    }
}
use crate::models::*;
use crate::service::HadithService;
use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post},
    Router,
};
use shared::{ApiResponse, AppError};
use std::collections::HashMap;
use uuid::Uuid;

/// Create the router for Hadith service endpoints
pub fn create_router(service: HadithService) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/hadiths/:hadith_id", get(get_hadith_by_id))
        .route("/hadiths/number/:hadith_number/book/:book_name", get(get_hadith_by_number))
        .route("/hadiths", get(get_hadiths))
        .route("/hadiths", post(create_hadith))
        .route("/search", get(search_hadiths))
        .route("/search/suggestions", get(get_search_suggestions))
        .route("/books", get(get_hadith_books))
        .route("/books", post(create_hadith_book))
        .route("/books/:book_name/hadiths", get(get_hadiths_by_book))
        .route("/books/:book_id/chapters", get(get_book_chapters))
        .route("/topics/:topic", get(get_hadiths_by_topic))
        .route("/analytics", get(get_hadith_analytics))
        .route("/integrity/verify", post(verify_hadith_integrity))
        .with_state(service)
}

/// Health check endpoint
async fn health_check() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "hadith-service".to_string());
    Json(ApiResponse::success(status))
}

/// Get a Hadith by ID
async fn get_hadith_by_id(
    State(service): State<HadithService>,
    Path(hadith_id): Path<Uuid>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<HadithResponse>>, AppError> {
    let include_sanad = params.get("include_sanad")
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);

    let include_explanations = params.get("include_explanations")
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);

    let request = GetHadithRequest {
        hadith_id: Some(hadith_id),
        hadith_number: None,
        book_name: None,
        include_sanad: Some(include_sanad),
        include_explanations: Some(include_explanations),
    };

    match service.get_hadith(request).await? {
        Some(response) => Ok(Json(ApiResponse::success(response))),
        None => Err(AppError::NotFound("Hadith not found".to_string())),
    }
}

/// Get a Hadith by number and book
async fn get_hadith_by_number(
    State(service): State<HadithService>,
    Path((hadith_number, book_name)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<HadithResponse>>, AppError> {
    let include_sanad = params.get("include_sanad")
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);

    let include_explanations = params.get("include_explanations")
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);

    let request = GetHadithRequest {
        hadith_id: None,
        hadith_number: Some(hadith_number),
        book_name: Some(book_name),
        include_sanad: Some(include_sanad),
        include_explanations: Some(include_explanations),
    };

    match service.get_hadith(request).await? {
        Some(response) => Ok(Json(ApiResponse::success(response))),
        None => Err(AppError::NotFound("Hadith not found".to_string())),
    }
}

/// Get Hadiths with optional filtering
async fn get_hadiths(
    State(service): State<HadithService>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<Hadith>>>, AppError> {
    if let Some(book_name) = params.get("book") {
        let limit = params.get("limit")
            .and_then(|l| l.parse::<i32>().ok());
        let offset = params.get("offset")
            .and_then(|o| o.parse::<i32>().ok());

        let hadiths = service.get_hadiths_by_book(book_name, limit, offset).await?;
        Ok(Json(ApiResponse::success(hadiths)))
    } else {
        Err(AppError::BadRequest("Book parameter is required".to_string()))
    }
}

/// Search Hadiths
async fn search_hadiths(
    State(service): State<HadithService>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<HadithSearchResponse>>, AppError> {
    let query = params.get("q")
        .ok_or_else(|| AppError::BadRequest("Query parameter 'q' is required".to_string()))?
        .clone();

    let books = params.get("books")
        .map(|books_str| {
            books_str.split(',')
                .map(|b| b.trim().to_string())
                .collect::<Vec<String>>()
        });

    let grades = params.get("grades")
        .and_then(|grades_str| {
            grades_str.split(',')
                .map(|g| match g.trim() {
                    "sahih" => Some(HadithGrade::Sahih),
                    "hasan" => Some(HadithGrade::Hasan),
                    "daif" => Some(HadithGrade::Daif),
                    "mawdu" => Some(HadithGrade::Mawdu),
                    _ => None,
                })
                .collect::<Option<Vec<HadithGrade>>>()
        });

    let themes = params.get("themes")
        .map(|themes_str| {
            themes_str.split(',')
                .map(|t| t.trim().to_string())
                .collect::<Vec<String>>()
        });

    let search_type = params.get("type")
        .and_then(|t| match t.as_str() {
            "text" => Some(SearchType::Text),
            "semantic" => Some(SearchType::Semantic),
            "narrator" => Some(SearchType::Narrator),
            "theme" => Some(SearchType::Theme),
            "exact" => Some(SearchType::Exact),
            _ => None,
        });

    let limit = params.get("limit")
        .and_then(|l| l.parse::<i32>().ok())
        .unwrap_or(20);

    let offset = params.get("offset")
        .and_then(|o| o.parse::<i32>().ok())
        .unwrap_or(0);

    let request = SearchHadithRequest {
        query,
        books,
        grades,
        themes,
        search_type,
        limit: Some(limit),
        offset: Some(offset),
    };

    let response = service.search_hadiths(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

/// Get search suggestions
async fn get_search_suggestions(
    State(service): State<HadithService>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<String>>>, AppError> {
    let query = params.get("q")
        .ok_or_else(|| AppError::BadRequest("Query parameter 'q' is required".to_string()))?;

    let suggestions = service.get_search_suggestions(query).await?;
    Ok(Json(ApiResponse::success(suggestions)))
}

/// Get all Hadith books
async fn get_hadith_books(
    State(service): State<HadithService>,
) -> Result<Json<ApiResponse<Vec<HadithBook>>>, AppError> {
    let books = service.get_hadith_books().await?;
    Ok(Json(ApiResponse::success(books)))
}

/// Create a new Hadith book
async fn create_hadith_book(
    State(service): State<HadithService>,
    Json(book): Json<HadithBook>,
) -> Result<Json<ApiResponse<HadithBook>>, AppError> {
    let created_book = service.create_hadith_book(book).await?;
    Ok(Json(ApiResponse::success(created_book)))
}

/// Get Hadiths by book
async fn get_hadiths_by_book(
    State(service): State<HadithService>,
    Path(book_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<Hadith>>>, AppError> {
    let limit = params.get("limit")
        .and_then(|l| l.parse::<i32>().ok());
    let offset = params.get("offset")
        .and_then(|o| o.parse::<i32>().ok());

    let hadiths = service.get_hadiths_by_book(&book_name, limit, offset).await?;
    Ok(Json(ApiResponse::success(hadiths)))
}

/// Get chapters for a book
async fn get_book_chapters(
    State(service): State<HadithService>,
    Path(book_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<HadithChapter>>>, AppError> {
    let chapters = service.get_book_chapters(book_id).await?;
    Ok(Json(ApiResponse::success(chapters)))
}

/// Get Hadiths by topic/theme
async fn get_hadiths_by_topic(
    State(service): State<HadithService>,
    Path(topic): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<HadithTopicResponse>>, AppError> {
    let include_related = params.get("include_related")
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);

    let grades = params.get("grades")
        .and_then(|grades_str| {
            grades_str.split(',')
                .map(|g| match g.trim() {
                    "sahih" => Some(HadithGrade::Sahih),
                    "hasan" => Some(HadithGrade::Hasan),
                    "daif" => Some(HadithGrade::Daif),
                    "mawdu" => Some(HadithGrade::Mawdu),
                    _ => None,
                })
                .collect::<Option<Vec<HadithGrade>>>()
        });

    let limit = params.get("limit")
        .and_then(|l| l.parse::<i32>().ok())
        .unwrap_or(20);

    let offset = params.get("offset")
        .and_then(|o| o.parse::<i32>().ok())
        .unwrap_or(0);

    let request = GetHadithsByTopicRequest {
        topic,
        include_related: Some(include_related),
        grades,
        limit: Some(limit),
        offset: Some(offset),
    };

    let response = service.get_hadiths_by_topic(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

/// Get Hadith analytics
async fn get_hadith_analytics(
    State(service): State<HadithService>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<HadithAnalyticsResponse>>, AppError> {
    let analysis_type = params.get("type")
        .and_then(|t| match t.as_str() {
            "grade_distribution" => Some(AnalysisType::GradeDistribution),
            "theme_analysis" => Some(AnalysisType::ThemeAnalysis),
            "narrator_frequency" => Some(AnalysisType::NarratorFrequency),
            "book_statistics" => Some(AnalysisType::BookStatistics),
            _ => None,
        })
        .unwrap_or(AnalysisType::GradeDistribution);

    let book_ids = params.get("books")
        .and_then(|books_str| {
            books_str.split(',')
                .map(|id| Uuid::parse_str(id.trim()).ok())
                .collect::<Option<Vec<Uuid>>>()
        });

    let date_range = params.get("date_range")
        .and_then(|range_str| {
            let parts: Vec<&str> = range_str.split(',').collect();
            if parts.len() == 2 {
                let start = chrono::DateTime::parse_from_rfc3339(parts[0].trim()).ok()?.with_timezone(&chrono::Utc);
                let end = chrono::DateTime::parse_from_rfc3339(parts[1].trim()).ok()?.with_timezone(&chrono::Utc);
                Some((start, end))
            } else {
                None
            }
        });

    let request = HadithAnalyticsRequest {
        book_ids,
        analysis_type,
        date_range,
    };

    let response = service.get_hadith_analytics(request).await?;
    Ok(Json(ApiResponse::success(response)))
}

/// Verify Hadith content integrity
async fn verify_hadith_integrity(
    State(service): State<HadithService>,
) -> Result<Json<ApiResponse<crate::service::HadithIntegrityReport>>, AppError> {
    let report = service.verify_hadith_integrity().await?;
    Ok(Json(ApiResponse::success(report)))
}

/// Create a new Hadith
async fn create_hadith(
    State(service): State<HadithService>,
    Json(hadith): Json<Hadith>,
) -> Result<Json<ApiResponse<Hadith>>, AppError> {
    let created_hadith = service.create_hadith(hadith).await?;
    Ok(Json(ApiResponse::success(created_hadith)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_parameter_parsing() {
        // Test parsing of query parameters
        let mut params = HashMap::new();
        params.insert("include_sanad".to_string(), "true".to_string());
        
        let include_sanad = params.get("include_sanad")
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(false);
        
        assert!(include_sanad);
    }

    #[test]
    fn test_grade_parsing() {
        let grades_str = "sahih,hasan,daif";
        let grades: Option<Vec<HadithGrade>> = grades_str.split(',')
            .map(|g| match g.trim() {
                "sahih" => Some(HadithGrade::Sahih),
                "hasan" => Some(HadithGrade::Hasan),
                "daif" => Some(HadithGrade::Daif),
                "mawdu" => Some(HadithGrade::Mawdu),
                _ => None,
            })
            .collect();
        
        assert!(grades.is_some());
        let parsed_grades = grades.unwrap();
        assert_eq!(parsed_grades.len(), 3);
        assert!(parsed_grades.contains(&HadithGrade::Sahih));
        assert!(parsed_grades.contains(&HadithGrade::Hasan));
        assert!(parsed_grades.contains(&HadithGrade::Daif));
    }

    #[test]
    fn test_search_type_parsing() {
        let search_types = vec![
            ("text", SearchType::Text),
            ("semantic", SearchType::Semantic),
            ("narrator", SearchType::Narrator),
            ("theme", SearchType::Theme),
            ("exact", SearchType::Exact),
        ];

        for (type_str, expected_type) in search_types {
            let parsed_type = match type_str {
                "text" => Some(SearchType::Text),
                "semantic" => Some(SearchType::Semantic),
                "narrator" => Some(SearchType::Narrator),
                "theme" => Some(SearchType::Theme),
                "exact" => Some(SearchType::Exact),
                _ => None,
            };

            assert_eq!(parsed_type, Some(expected_type));
        }
    }

    #[test]
    fn test_uuid_parsing() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let parsed_uuid = Uuid::parse_str(uuid_str);
        assert!(parsed_uuid.is_ok());
    }

    #[test]
    fn test_books_list_parsing() {
        let books_str = "صحيح البخاري,صحيح مسلم,سنن أبي داود";
        let books: Vec<String> = books_str.split(',')
            .map(|b| b.trim().to_string())
            .collect();
        
        assert_eq!(books.len(), 3);
        assert!(books.contains(&"صحيح البخاري".to_string()));
        assert!(books.contains(&"صحيح مسلم".to_string()));
        assert!(books.contains(&"سنن أبي داود".to_string()));
    }

    #[test]
    fn test_themes_list_parsing() {
        let themes_str = "عقيدة,عبادة,أخلاق";
        let themes: Vec<String> = themes_str.split(',')
            .map(|t| t.trim().to_string())
            .collect();
        
        assert_eq!(themes.len(), 3);
        assert!(themes.contains(&"عقيدة".to_string()));
        assert!(themes.contains(&"عبادة".to_string()));
        assert!(themes.contains(&"أخلاق".to_string()));
    }
}
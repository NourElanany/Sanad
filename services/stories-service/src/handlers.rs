use crate::models::*;
use crate::service::StoryService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use shared::ApiResponse;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

/// Application state containing the story service
pub type AppState = Arc<StoryService>;

/// Create the router for the stories service
pub fn create_router(service: StoryService) -> Router {
    let app_state = Arc::new(service);

    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Story endpoints
        .route("/stories", post(create_story))
        .route("/stories", get(search_stories))
        .route("/stories/:id", get(get_story))
        .route("/stories/:id", put(update_story))
        .route("/stories/:id", delete(delete_story))
        .route("/stories/category/:category", get(get_stories_by_category))
        .route("/stories/character/:character_name", get(get_stories_by_character))
        .route("/stories/theme/:theme", get(get_stories_by_theme))
        .route("/stories/:id/integrity", get(verify_story_integrity))
        
        // Character endpoints
        .route("/characters", post(create_character))
        .route("/characters/:id", get(get_character))
        .route("/characters/search", get(search_characters))
        .route("/stories/:story_id/characters/:character_id", post(add_character_to_story))
        
        // Lesson endpoints
        .route("/lessons", post(create_lesson))
        .route("/lessons/:id", get(get_lesson))
        .route("/lessons/search", get(search_lessons))
        .route("/stories/:story_id/lessons", get(get_story_lessons))
        .route("/stories/:story_id/lessons/:lesson_id", post(add_lesson_to_story))
        
        // Source endpoints
        .route("/stories/:story_id/sources", get(get_story_sources))
        .route("/stories/:story_id/sources", post(create_story_source))
        
        // Advanced search endpoints
        .route("/search/by-theme", get(search_stories_by_theme))
        .route("/search/by-lesson", get(search_stories_by_lesson))
        .route("/search/by-moral", get(search_stories_by_moral_category))
        
        // Analytics endpoints
        .route("/analytics/categories", get(get_category_statistics))
        .route("/analytics/integrity", get(verify_all_stories_integrity))
        
        .with_state(app_state)
}

/// Health check endpoint
async fn health_check() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "stories-service".to_string());
    status.insert("version".to_string(), "1.0.0".to_string());
    Json(ApiResponse::success(status))
}

/// Create a new story
async fn create_story(
    State(service): State<AppState>,
    Json(request): Json<CreateStoryRequest>,
) -> Result<Json<ApiResponse<Story>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Creating new story: {}", request.title);

    match service.create_story(
        request.title,
        request.arabic_title,
        request.content,
        request.category,
        request.age_group,
        request.language.unwrap_or_else(|| "ar".to_string()),
        request.authenticity_level,
    ).await {
        Ok(story) => {
            info!("Successfully created story with ID: {}", story.id);
            Ok(Json(ApiResponse::success(story)))
        }
        Err(e) => {
            error!("Failed to create story: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(format!("Failed to create story: {}", e))),
            ))
        }
    }
}

/// Get a story by ID
async fn get_story(
    State(service): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<GetStoryParams>,
) -> Result<Json<ApiResponse<StoryResponse>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Getting story with ID: {}", id);

    match service.get_story(id, params.include_details.unwrap_or(false)).await {
        Ok(Some(story)) => Ok(Json(ApiResponse::success(story))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Story not found".to_string())),
        )),
        Err(e) => {
            error!("Failed to get story {}: {}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to get story: {}", e))),
            ))
        }
    }
}

/// Search stories
async fn search_stories(
    State(service): State<AppState>,
    Query(request): Query<SearchStoriesRequest>,
) -> Result<Json<ApiResponse<StorySearchResponse>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Searching stories with query: {}", request.query);

    match service.search_stories(request).await {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(e) => {
            error!("Failed to search stories: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to search stories: {}", e))),
            ))
        }
    }
}

/// Update a story
async fn update_story(
    State(service): State<AppState>,
    Path(id): Path<Uuid>,
    Json(mut story): Json<Story>,
) -> Result<Json<ApiResponse<Story>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Updating story with ID: {}", id);

    // Ensure the ID matches
    story.id = id;

    match service.update_story(story).await {
        Ok(updated_story) => {
            info!("Successfully updated story with ID: {}", id);
            Ok(Json(ApiResponse::success(updated_story)))
        }
        Err(e) => {
            error!("Failed to update story {}: {}", id, e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(format!("Failed to update story: {}", e))),
            ))
        }
    }
}

/// Delete a story
async fn delete_story(
    State(service): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Deleting story with ID: {}", id);

    match service.delete_story(id).await {
        Ok(deleted) => {
            if deleted {
                info!("Successfully deleted story with ID: {}", id);
                Ok(Json(ApiResponse::success(true)))
            } else {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse::error("Story not found".to_string())),
                ))
            }
        }
        Err(e) => {
            error!("Failed to delete story {}: {}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to delete story: {}", e))),
            ))
        }
    }
}

/// Get stories by category
async fn get_stories_by_category(
    State(service): State<AppState>,
    Path(category): Path<String>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<Story>>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Getting stories by category: {}", category);

    // Parse category string to enum
    let category_enum = match category.as_str() {
        "prophets" => StoryCategory::Prophets,
        "companions" => StoryCategory::Companions,
        "righteous_predecessors" => StoryCategory::RighteousPredecessors,
        "historical_events" => StoryCategory::HistoricalEvents,
        "moral_lessons" => StoryCategory::MoralLessons,
        "miracles" => StoryCategory::Miracles,
        "battles" => StoryCategory::Battles,
        "conversions" => StoryCategory::Conversions,
        "women_in_islam" => StoryCategory::WomenInIslam,
        "children_stories" => StoryCategory::ChildrenStories,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Invalid category".to_string())),
            ));
        }
    };

    match service.get_stories_by_category(category_enum, params.limit, params.offset).await {
        Ok(stories) => Ok(Json(ApiResponse::success(stories))),
        Err(e) => {
            error!("Failed to get stories by category {}: {}", category, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to get stories: {}", e))),
            ))
        }
    }
}

/// Get stories by character
async fn get_stories_by_character(
    State(service): State<AppState>,
    Path(character_name): Path<String>,
    Query(params): Query<GetStoriesByCharacterParams>,
) -> Result<Json<ApiResponse<CharacterStoriesResponse>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Getting stories by character: {}", character_name);

    let request = GetStoriesByCharacterRequest {
        character_name,
        character_type: params.character_type,
        include_related: params.include_related,
        limit: params.limit,
        offset: params.offset,
    };

    match service.get_stories_by_character(request).await {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(e) => {
            error!("Failed to get stories by character: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to get stories: {}", e))),
            ))
        }
    }
}

/// Get stories by theme
async fn get_stories_by_theme(
    State(service): State<AppState>,
    Path(theme): Path<String>,
    Query(params): Query<GetStoriesByThemeParams>,
) -> Result<Json<ApiResponse<ThemeStoriesResponse>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Getting stories by theme: {}", theme);

    let request = GetStoriesByThemeRequest {
        theme,
        lesson_type: params.lesson_type,
        moral_category: params.moral_category,
        age_group: params.age_group,
        limit: params.limit,
        offset: params.offset,
    };

    match service.get_stories_by_theme(request).await {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(e) => {
            error!("Failed to get stories by theme: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to get stories: {}", e))),
            ))
        }
    }
}

/// Create a new character
async fn create_character(
    State(service): State<AppState>,
    Json(request): Json<CreateCharacterRequest>,
) -> Result<Json<ApiResponse<Character>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Creating new character: {}", request.name);

    match service.create_character(
        request.name,
        request.arabic_name,
        request.character_type,
        request.description,
        request.historical_period,
    ).await {
        Ok(character) => {
            info!("Successfully created character with ID: {}", character.id);
            Ok(Json(ApiResponse::success(character)))
        }
        Err(e) => {
            error!("Failed to create character: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(format!("Failed to create character: {}", e))),
            ))
        }
    }
}

/// Get a character by ID
async fn get_character(
    State(service): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Character>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Getting character with ID: {}", id);

    match service.get_character_by_id(id).await {
        Ok(Some(character)) => Ok(Json(ApiResponse::success(character))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Character not found".to_string())),
        )),
        Err(e) => {
            error!("Failed to get character {}: {}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to get character: {}", e))),
            ))
        }
    }
}

/// Search characters
async fn search_characters(
    State(service): State<AppState>,
    Query(params): Query<SearchCharactersParams>,
) -> Result<Json<ApiResponse<Vec<Character>>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Searching characters with query: {}", params.query);

    match service.search_characters(params).await {
        Ok(characters) => Ok(Json(ApiResponse::success(characters))),
        Err(e) => {
            error!("Failed to search characters: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to search characters: {}", e))),
            ))
        }
    }
}

/// Add a character to a story
async fn add_character_to_story(
    State(service): State<AppState>,
    Path((story_id, character_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<AddCharacterToStoryRequest>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Adding character {} to story {}", character_id, story_id);

    match service.add_character_to_story(
        story_id,
        character_id,
        request.role,
        request.importance,
        request.description,
    ).await {
        Ok(()) => Ok(Json(ApiResponse::success("Character added to story successfully".to_string()))),
        Err(e) => {
            error!("Failed to add character to story: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(format!("Failed to add character to story: {}", e))),
            ))
        }
    }
}

/// Create a new lesson
async fn create_lesson(
    State(service): State<AppState>,
    Json(request): Json<CreateLessonRequest>,
) -> Result<Json<ApiResponse<Lesson>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Creating new lesson: {}", request.title);

    match service.create_lesson(
        request.title,
        request.arabic_title,
        request.description,
        request.lesson_type,
        request.moral_category,
    ).await {
        Ok(lesson) => {
            info!("Successfully created lesson with ID: {}", lesson.id);
            Ok(Json(ApiResponse::success(lesson)))
        }
        Err(e) => {
            error!("Failed to create lesson: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(format!("Failed to create lesson: {}", e))),
            ))
        }
    }
}

/// Add a lesson to a story
async fn add_lesson_to_story(
    State(service): State<AppState>,
    Path((story_id, lesson_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<AddLessonToStoryRequest>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Adding lesson {} to story {}", lesson_id, story_id);

    match service.add_lesson_to_story(
        story_id,
        lesson_id,
        request.relevance_score,
        request.explanation,
    ).await {
        Ok(()) => Ok(Json(ApiResponse::success("Lesson added to story successfully".to_string()))),
        Err(e) => {
            error!("Failed to add lesson to story: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(format!("Failed to add lesson to story: {}", e))),
            ))
        }
    }
}

/// Get a lesson by ID
async fn get_lesson(
    State(service): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Lesson>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Getting lesson with ID: {}", id);

    match service.get_lesson_by_id(id).await {
        Ok(Some(lesson)) => Ok(Json(ApiResponse::success(lesson))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Lesson not found".to_string())),
        )),
        Err(e) => {
            error!("Failed to get lesson {}: {}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to get lesson: {}", e))),
            ))
        }
    }
}

/// Search lessons
async fn search_lessons(
    State(service): State<AppState>,
    Query(params): Query<SearchLessonsParams>,
) -> Result<Json<ApiResponse<Vec<Lesson>>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Searching lessons with query: {}", params.query);

    match service.search_lessons(params).await {
        Ok(lessons) => Ok(Json(ApiResponse::success(lessons))),
        Err(e) => {
            error!("Failed to search lessons: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to search lessons: {}", e))),
            ))
        }
    }
}

/// Get lessons for a story
async fn get_story_lessons(
    State(service): State<AppState>,
    Path(story_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<LessonInStory>>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Getting lessons for story: {}", story_id);

    match service.get_story_lessons(story_id).await {
        Ok(lessons) => Ok(Json(ApiResponse::success(lessons))),
        Err(e) => {
            error!("Failed to get story lessons: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to get story lessons: {}", e))),
            ))
        }
    }
}

/// Get sources for a story
async fn get_story_sources(
    State(service): State<AppState>,
    Path(story_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<StorySource>>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Getting sources for story: {}", story_id);

    match service.get_story_sources(story_id).await {
        Ok(sources) => Ok(Json(ApiResponse::success(sources))),
        Err(e) => {
            error!("Failed to get story sources: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to get story sources: {}", e))),
            ))
        }
    }
}

/// Search stories by theme
async fn search_stories_by_theme(
    State(service): State<AppState>,
    Query(params): Query<SearchByThemeParams>,
) -> Result<Json<ApiResponse<ThemeStoriesResponse>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Searching stories by theme: {}", params.theme);

    let request = GetStoriesByThemeRequest {
        theme: params.theme,
        lesson_type: params.lesson_type,
        moral_category: params.moral_category,
        age_group: params.age_group,
        limit: params.limit,
        offset: params.offset,
    };

    match service.get_stories_by_theme(request).await {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(e) => {
            error!("Failed to search stories by theme: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to search stories by theme: {}", e))),
            ))
        }
    }
}

/// Search stories by lesson
async fn search_stories_by_lesson(
    State(service): State<AppState>,
    Query(params): Query<SearchByLessonParams>,
) -> Result<Json<ApiResponse<Vec<StorySearchResult>>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Searching stories by lesson: {}", params.lesson_title);

    match service.search_stories_by_lesson(params).await {
        Ok(results) => Ok(Json(ApiResponse::success(results))),
        Err(e) => {
            error!("Failed to search stories by lesson: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to search stories by lesson: {}", e))),
            ))
        }
    }
}

/// Search stories by moral category
async fn search_stories_by_moral_category(
    State(service): State<AppState>,
    Query(params): Query<SearchByMoralParams>,
) -> Result<Json<ApiResponse<Vec<StorySearchResult>>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Searching stories by moral category: {:?}", params.moral_category);

    match service.search_stories_by_moral_category(params).await {
        Ok(results) => Ok(Json(ApiResponse::success(results))),
        Err(e) => {
            error!("Failed to search stories by moral category: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to search stories by moral category: {}", e))),
            ))
        }
    }
}

/// Create a story source
async fn create_story_source(
    State(service): State<AppState>,
    Path(story_id): Path<Uuid>,
    Json(request): Json<CreateStorySourceRequest>,
) -> Result<Json<ApiResponse<StorySource>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Creating new source for story: {}", story_id);

    match service.create_story_source(
        story_id,
        request.source_type,
        request.source_name,
        request.arabic_source_name,
        request.reference,
        request.author,
        request.authenticity_grade,
    ).await {
        Ok(source) => {
            info!("Successfully created source with ID: {}", source.id);
            Ok(Json(ApiResponse::success(source)))
        }
        Err(e) => {
            error!("Failed to create story source: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(format!("Failed to create story source: {}", e))),
            ))
        }
    }
}

/// Verify story integrity
async fn verify_story_integrity(
    State(service): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<IntegrityCheckResult>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Verifying integrity for story: {}", id);

    match service.verify_story_integrity(id).await {
        Ok(is_valid) => {
            let result = IntegrityCheckResult {
                story_id: id,
                is_valid,
                message: if is_valid {
                    "Story content integrity verified".to_string()
                } else {
                    "Story content integrity check failed".to_string()
                },
            };
            Ok(Json(ApiResponse::success(result)))
        }
        Err(e) => {
            error!("Failed to verify story integrity: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to verify integrity: {}", e))),
            ))
        }
    }
}

/// Get category statistics
async fn get_category_statistics(
    State(service): State<AppState>,
) -> Result<Json<ApiResponse<HashMap<String, i64>>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Getting category statistics");

    match service.get_category_statistics().await {
        Ok(stats) => Ok(Json(ApiResponse::success(stats))),
        Err(e) => {
            error!("Failed to get category statistics: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to get statistics: {}", e))),
            ))
        }
    }
}

/// Verify integrity of all stories
async fn verify_all_stories_integrity(
    State(service): State<AppState>,
) -> Result<Json<ApiResponse<IntegrityCheckSummary>>, (StatusCode, Json<ApiResponse<String>>)> {
    info!("Verifying integrity of all stories");

    match service.verify_all_stories_integrity().await {
        Ok(problematic_stories) => {
            let summary = IntegrityCheckSummary {
                total_checked: 0, // Would be calculated in a full implementation
                valid_stories: 0, // Would be calculated
                invalid_stories: problematic_stories.len() as i64,
                problematic_story_ids: problematic_stories,
            };
            Ok(Json(ApiResponse::success(summary)))
        }
        Err(e) => {
            error!("Failed to verify all stories integrity: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to verify integrity: {}", e))),
            ))
        }
    }
}

// Request/Response models for API endpoints

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStoryRequest {
    pub title: String,
    pub arabic_title: String,
    pub content: String,
    pub category: StoryCategory,
    pub age_group: AgeGroup,
    pub language: Option<String>,
    pub authenticity_level: AuthenticityLevel,
    pub summary: Option<String>,
    pub subcategory: Option<String>,
    pub time_period: Option<TimePeriod>,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetStoryParams {
    pub include_details: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GetStoriesByCharacterParams {
    pub character_type: Option<CharacterType>,
    pub include_related: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GetStoriesByThemeParams {
    pub lesson_type: Option<LessonType>,
    pub moral_category: Option<MoralCategory>,
    pub age_group: Option<AgeGroup>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCharacterRequest {
    pub name: String,
    pub arabic_name: String,
    pub character_type: CharacterType,
    pub description: Option<String>,
    pub historical_period: Option<TimePeriod>,
    pub birth_year: Option<i32>,
    pub death_year: Option<i32>,
    pub biography: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddCharacterToStoryRequest {
    pub role: CharacterRole,
    pub importance: ImportanceLevel,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLessonRequest {
    pub title: String,
    pub arabic_title: String,
    pub description: String,
    pub lesson_type: LessonType,
    pub moral_category: MoralCategory,
    pub practical_application: Option<String>,
    pub target_audience: Option<Vec<AgeGroup>>,
}

#[derive(Debug, Deserialize)]
pub struct AddLessonToStoryRequest {
    pub relevance_score: f64,
    pub explanation: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateStorySourceRequest {
    pub source_type: SourceType,
    pub source_name: String,
    pub arabic_source_name: String,
    pub reference: String,
    pub author: Option<String>,
    pub authenticity_grade: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IntegrityCheckResult {
    pub story_id: Uuid,
    pub is_valid: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct IntegrityCheckSummary {
    pub total_checked: i64,
    pub valid_stories: i64,
    pub invalid_stories: i64,
    pub problematic_story_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct SearchLessonsParams {
    pub query: String,
    pub lesson_type: Option<LessonType>,
    pub moral_category: Option<MoralCategory>,
    pub target_audience: Option<AgeGroup>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SearchByThemeParams {
    pub theme: String,
    pub lesson_type: Option<LessonType>,
    pub moral_category: Option<MoralCategory>,
    pub age_group: Option<AgeGroup>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SearchByLessonParams {
    pub lesson_title: String,
    pub lesson_type: Option<LessonType>,
    pub moral_category: Option<MoralCategory>,
    pub age_group: Option<AgeGroup>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SearchByMoralParams {
    pub moral_category: MoralCategory,
    pub lesson_type: Option<LessonType>,
    pub age_group: Option<AgeGroup>,
    pub authenticity_level: Option<AuthenticityLevel>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    // Note: These tests would require a test setup with a real service
    // They are provided as examples of how to test the handlers

    async fn setup_test_server() -> TestServer {
        // This would set up a test server with a mock service
        // For now, we'll just return a placeholder
        todo!("Set up test server")
    }

    #[tokio::test]
    async fn test_health_check() {
        let server = setup_test_server().await;
        
        let response = server.get("/health").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let body: ApiResponse<HashMap<String, String>> = response.json();
        assert_eq!(body.data.as_ref().unwrap().get("status"), Some(&"healthy".to_string()));
    }

    #[tokio::test]
    async fn test_create_story() {
        let server = setup_test_server().await;
        
        let request = CreateStoryRequest {
            title: "Test Story".to_string(),
            arabic_title: "قصة تجريبية".to_string(),
            content: "This is a test story".to_string(),
            category: StoryCategory::MoralLessons,
            age_group: AgeGroup::Children,
            language: Some("en".to_string()),
            authenticity_level: AuthenticityLevel::Educational,
            summary: None,
            subcategory: None,
            time_period: None,
            location: None,
        };

        let response = server.post("/stories").json(&request).await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_story() {
        let server = setup_test_server().await;
        
        // This would test getting a story by ID
        // First create a story, then get it
        let story_id = Uuid::new_v4();
        let response = server.get(&format!("/stories/{}", story_id)).await;
        
        // In a real test, we'd check for the appropriate response
        // For now, we expect NOT_FOUND since no story exists
        assert!(response.status_code() == StatusCode::NOT_FOUND || response.status_code() == StatusCode::OK);
    }

    #[tokio::test]
    async fn test_search_stories() {
        let server = setup_test_server().await;
        
        let response = server.get("/stories?query=test&limit=10").await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_category_statistics() {
        let server = setup_test_server().await;
        
        let response = server.get("/analytics/categories").await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
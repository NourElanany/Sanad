use crate::models::*;
use crate::repository::StoryRepository;
use crate::handlers::{SearchLessonsParams, SearchByLessonParams, SearchByMoralParams};
use anyhow::{Result, anyhow};
use uuid::Uuid;
use std::collections::HashMap;
use tracing::{info, warn, error};

/// Service layer for Islamic stories business logic
pub struct StoryService {
    repository: StoryRepository,
}

impl StoryService {
    /// Create a new StoryService
    pub fn new(repository: StoryRepository) -> Self {
        Self { repository }
    }

    /// Create a new story with validation and integrity checks
    pub async fn create_story(
        &self,
        title: String,
        arabic_title: String,
        content: String,
        category: StoryCategory,
        age_group: AgeGroup,
        language: String,
        authenticity_level: AuthenticityLevel,
    ) -> Result<Story> {
        // Validate input
        if title.trim().is_empty() {
            return Err(anyhow!("Story title cannot be empty"));
        }
        if arabic_title.trim().is_empty() {
            return Err(anyhow!("Arabic title cannot be empty"));
        }
        if content.trim().is_empty() {
            return Err(anyhow!("Story content cannot be empty"));
        }

        // Create story with automatic hash generation
        let mut story = Story::new(
            title,
            arabic_title,
            content,
            category,
            age_group,
            language,
            authenticity_level,
        );

        // Auto-generate themes and keywords based on content
        story.themes = self.extract_themes(&story.content, &story.category);
        story.keywords = self.extract_keywords(&story.content);

        // Create the story in the database
        let created_story = self.repository.create_story(&story).await?;
        
        info!("Created new story: {} (ID: {})", created_story.title, created_story.id);
        Ok(created_story)
    }

    /// Get a story by ID with optional details
    pub async fn get_story(
        &self,
        story_id: Uuid,
        include_details: bool,
    ) -> Result<Option<StoryResponse>> {
        if include_details {
            if let Some(story_details) = self.repository.get_story_with_details(story_id).await? {
                Ok(Some(StoryResponse {
                    story: story_details.story,
                    characters: Some(story_details.characters),
                    lessons: Some(story_details.lessons),
                    sources: Some(story_details.sources),
                    collections: Some(story_details.collections),
                }))
            } else {
                Ok(None)
            }
        } else {
            if let Some(story) = self.repository.get_story_by_id(story_id).await? {
                Ok(Some(StoryResponse {
                    story,
                    characters: None,
                    lessons: None,
                    sources: None,
                    collections: None,
                }))
            } else {
                Ok(None)
            }
        }
    }

    /// Get a story by title
    pub async fn get_story_by_title(&self, title: &str) -> Result<Option<Story>> {
        self.repository.get_story_by_title(title).await
    }

    /// Search stories with comprehensive filtering
    pub async fn search_stories(&self, request: SearchStoriesRequest) -> Result<StorySearchResponse> {
        let start_time = std::time::Instant::now();

        // Convert enum vectors to slices for repository call
        let categories = request.categories.as_deref();
        let age_groups = request.age_groups.as_deref();
        let authenticity_levels = request.authenticity_levels.as_deref();

        let limit = request.limit.unwrap_or(20).min(100); // Cap at 100
        let offset = request.offset.unwrap_or(0);

        let stories = self.repository.search_stories(
            &request.query,
            categories,
            age_groups,
            authenticity_levels,
            limit,
            offset,
        ).await?;

        // Convert stories to search results with relevance scoring
        let mut results = Vec::new();
        for story in stories {
            let characters = self.repository.get_story_characters(story.id).await?
                .into_iter()
                .map(|c| c.character)
                .collect();

            let main_lessons = story.moral_lessons.clone();
            let relevance_score = self.calculate_relevance_score(&story, &request.query);
            let highlighted_text = self.highlight_text(&story.content, &request.query);
            let matching_criteria = self.get_matching_criteria(&story, &request);

            results.push(StorySearchResult {
                story,
                characters,
                main_lessons,
                relevance_score,
                highlighted_text,
                matching_criteria,
            });
        }

        // Sort by relevance score
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));

        let search_time_ms = start_time.elapsed().as_millis() as u64;
        let total_count = results.len() as i64; // This would be a separate count query in production

        Ok(StorySearchResponse {
            results,
            total_count,
            query: request.query,
            search_type: request.search_type.unwrap_or(SearchType::Text),
            search_time_ms,
            facets: None, // Would be implemented for advanced filtering
        })
    }

    /// Get stories by category
    pub async fn get_stories_by_category(
        &self,
        category: StoryCategory,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Story>> {
        let limit = limit.unwrap_or(20).min(100);
        let offset = offset.unwrap_or(0);

        self.repository.get_stories_by_category(category, limit, offset).await
    }

    /// Get stories by character
    pub async fn get_stories_by_character(
        &self,
        request: GetStoriesByCharacterRequest,
    ) -> Result<CharacterStoriesResponse> {
        let limit = request.limit.unwrap_or(20).min(100);
        let offset = request.offset.unwrap_or(0);

        // Get the character first
        let characters = self.repository.get_characters_by_name(&request.character_name).await?;
        if characters.is_empty() {
            return Err(anyhow!("Character not found: {}", request.character_name));
        }

        let character = characters.into_iter().next().unwrap();

        // Get stories for this character
        let stories = self.repository.get_stories_by_character(&request.character_name, limit, offset).await?;

        // Convert to detailed stories
        let mut detailed_stories = Vec::new();
        for story in stories {
            if let Some(details) = self.repository.get_story_with_details(story.id).await? {
                detailed_stories.push(details);
            }
        }

        // Get related characters (characters that appear in the same stories)
        let related_characters = self.get_related_characters(&character).await?;
        let total_count = detailed_stories.len() as i64;

        Ok(CharacterStoriesResponse {
            character,
            stories: detailed_stories,
            related_characters,
            total_count,
        })
    }

    /// Get stories by theme
    pub async fn get_stories_by_theme(
        &self,
        request: GetStoriesByThemeRequest,
    ) -> Result<ThemeStoriesResponse> {
        let limit = request.limit.unwrap_or(20).min(100);
        let offset = request.offset.unwrap_or(0);

        let stories = self.repository.get_stories_by_theme(&request.theme, limit, offset).await?;

        // Convert to detailed stories
        let mut detailed_stories = Vec::new();
        for story in stories {
            if let Some(details) = self.repository.get_story_with_details(story.id).await? {
                detailed_stories.push(details);
            }
        }

        // Get related themes and lessons
        let related_themes = self.get_related_themes(&request.theme).await?;
        let related_lessons = self.get_lessons_by_theme(&request.theme).await?;
        let total_count = detailed_stories.len() as i64;

        Ok(ThemeStoriesResponse {
            theme: request.theme,
            stories: detailed_stories,
            related_themes,
            related_lessons,
            total_count,
        })
    }

    /// Get a character by ID
    pub async fn get_character_by_id(&self, character_id: Uuid) -> Result<Option<Character>> {
        self.repository.get_character_by_id(character_id).await
    }

    /// Search characters
    pub async fn search_characters(&self, params: SearchCharactersParams) -> Result<Vec<Character>> {
        let mut characters = self.repository.get_characters_by_name(&params.query).await?;

        // Filter by character type if specified
        if let Some(char_type) = params.character_type {
            characters.retain(|c| c.character_type == char_type);
        }

        // Filter by historical period if specified
        if let Some(period) = params.historical_period {
            characters.retain(|c| c.historical_period == Some(period));
        }

        // Apply pagination
        let limit = params.limit.unwrap_or(20).min(100) as usize;
        let offset = params.offset.unwrap_or(0) as usize;
        
        if offset < characters.len() {
            let end = (offset + limit).min(characters.len());
            characters = characters[offset..end].to_vec();
        } else {
            characters.clear();
        }

        Ok(characters)
    }

    /// Get a lesson by ID
    pub async fn get_lesson_by_id(&self, lesson_id: Uuid) -> Result<Option<Lesson>> {
        self.repository.get_lesson_by_id(lesson_id).await
    }

    /// Search lessons
    pub async fn search_lessons(&self, params: SearchLessonsParams) -> Result<Vec<Lesson>> {
        self.repository.search_lessons(
            &params.query,
            params.lesson_type,
            params.moral_category,
            params.target_audience,
            params.limit.unwrap_or(20).min(100),
            params.offset.unwrap_or(0),
        ).await
    }

    /// Get lessons for a story
    pub async fn get_story_lessons(&self, story_id: Uuid) -> Result<Vec<LessonInStory>> {
        self.repository.get_story_lessons(story_id).await
    }

    /// Get sources for a story
    pub async fn get_story_sources(&self, story_id: Uuid) -> Result<Vec<StorySource>> {
        self.repository.get_story_sources(story_id).await
    }

    /// Search stories by lesson
    pub async fn search_stories_by_lesson(&self, params: SearchByLessonParams) -> Result<Vec<StorySearchResult>> {
        let stories = self.repository.get_stories_by_lesson(
            &params.lesson_title,
            params.lesson_type,
            params.moral_category,
            params.limit.unwrap_or(20).min(100),
            params.offset.unwrap_or(0),
        ).await?;

        let mut results = Vec::new();
        for story in stories {
            let characters = self.repository.get_story_characters(story.id).await?
                .into_iter()
                .map(|c| c.character)
                .collect();

            let main_lessons = story.moral_lessons.clone();
            let relevance_score = self.calculate_lesson_relevance_score(&story, &params.lesson_title);
            let highlighted_text = self.highlight_text(&story.content, &params.lesson_title);
            let matching_criteria = vec!["Lesson".to_string()];

            results.push(StorySearchResult {
                story,
                characters,
                main_lessons,
                relevance_score,
                highlighted_text,
                matching_criteria,
            });
        }

        // Sort by relevance score
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(results)
    }

    /// Search stories by moral category
    pub async fn search_stories_by_moral_category(&self, params: SearchByMoralParams) -> Result<Vec<StorySearchResult>> {
        let moral_category = params.moral_category;
        let stories = self.repository.get_stories_by_moral_category(
            moral_category,
            params.lesson_type,
            params.age_group,
            params.authenticity_level,
            params.limit.unwrap_or(20).min(100),
            params.offset.unwrap_or(0),
        ).await?;

        let mut results = Vec::new();
        for story in stories {
            let characters = self.repository.get_story_characters(story.id).await?
                .into_iter()
                .map(|c| c.character)
                .collect();

            let main_lessons = story.moral_lessons.clone();
            let relevance_score = self.calculate_moral_relevance_score(&story, &moral_category);
            let highlighted_text = format!("Story focusing on {}", moral_category.arabic_name());
            let matching_criteria = vec!["Moral Category".to_string()];

            results.push(StorySearchResult {
                story,
                characters,
                main_lessons,
                relevance_score,
                highlighted_text,
                matching_criteria,
            });
        }

        // Sort by relevance score
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(results)
    }

    /// Create a new character
    pub async fn create_character(
        &self,
        name: String,
        arabic_name: String,
        character_type: CharacterType,
        description: Option<String>,
        historical_period: Option<TimePeriod>,
    ) -> Result<Character> {
        if name.trim().is_empty() {
            return Err(anyhow!("Character name cannot be empty"));
        }
        if arabic_name.trim().is_empty() {
            return Err(anyhow!("Arabic name cannot be empty"));
        }

        let mut character = Character::new(name, arabic_name, character_type);
        character.description = description;
        character.historical_period = historical_period;

        let created_character = self.repository.create_character(&character).await?;
        info!("Created new character: {} (ID: {})", created_character.name, created_character.id);
        
        Ok(created_character)
    }

    /// Add a character to a story
    pub async fn add_character_to_story(
        &self,
        story_id: Uuid,
        character_id: Uuid,
        role: CharacterRole,
        importance: ImportanceLevel,
        description: Option<String>,
    ) -> Result<()> {
        // Verify story and character exist
        if self.repository.get_story_by_id(story_id).await?.is_none() {
            return Err(anyhow!("Story not found"));
        }
        if self.repository.get_character_by_id(character_id).await?.is_none() {
            return Err(anyhow!("Character not found"));
        }

        self.repository.add_character_to_story(story_id, character_id, role.clone(), importance, description).await?;
        info!("Added character {} to story {} with role {:?}", character_id, story_id, role);
        
        Ok(())
    }

    /// Create a new lesson
    pub async fn create_lesson(
        &self,
        title: String,
        arabic_title: String,
        description: String,
        lesson_type: LessonType,
        moral_category: MoralCategory,
    ) -> Result<Lesson> {
        if title.trim().is_empty() {
            return Err(anyhow!("Lesson title cannot be empty"));
        }
        if description.trim().is_empty() {
            return Err(anyhow!("Lesson description cannot be empty"));
        }

        let lesson = Lesson::new(title, arabic_title, description, lesson_type, moral_category);
        let created_lesson = self.repository.create_lesson(&lesson).await?;
        
        info!("Created new lesson: {} (ID: {})", created_lesson.title, created_lesson.id);
        Ok(created_lesson)
    }

    /// Add a lesson to a story
    pub async fn add_lesson_to_story(
        &self,
        story_id: Uuid,
        lesson_id: Uuid,
        relevance_score: f64,
        explanation: Option<String>,
    ) -> Result<()> {
        // Validate relevance score
        if !(0.0..=10.0).contains(&relevance_score) {
            return Err(anyhow!("Relevance score must be between 0.0 and 10.0"));
        }

        // Verify story and lesson exist
        if self.repository.get_story_by_id(story_id).await?.is_none() {
            return Err(anyhow!("Story not found"));
        }

        self.repository.add_lesson_to_story(story_id, lesson_id, relevance_score, explanation).await?;
        info!("Added lesson {} to story {} with relevance score {}", lesson_id, story_id, relevance_score);
        
        Ok(())
    }

    /// Create a story source
    pub async fn create_story_source(
        &self,
        story_id: Uuid,
        source_type: SourceType,
        source_name: String,
        arabic_source_name: String,
        reference: String,
        author: Option<String>,
        authenticity_grade: Option<String>,
    ) -> Result<StorySource> {
        // Verify story exists
        if self.repository.get_story_by_id(story_id).await?.is_none() {
            return Err(anyhow!("Story not found"));
        }

        let mut source = StorySource::new(
            story_id,
            source_type,
            source_name,
            arabic_source_name,
            reference,
        );
        source.author = author;
        source.authenticity_grade = authenticity_grade;

        // Set credibility score based on source type
        source.credibility_score = self.calculate_source_credibility(&source);

        let created_source = self.repository.create_story_source(&source).await?;
        info!("Created new story source: {} for story {}", created_source.source_name, story_id);
        
        Ok(created_source)
    }

    /// Update a story
    pub async fn update_story(&self, mut story: Story) -> Result<Story> {
        // Verify story exists
        if self.repository.get_story_by_id(story.id).await?.is_none() {
            return Err(anyhow!("Story not found"));
        }

        // Recalculate metrics and hash
        story.update_metrics();

        // Update themes and keywords
        story.themes = self.extract_themes(&story.content, &story.category);
        story.keywords = self.extract_keywords(&story.content);

        let updated_story = self.repository.update_story(&story).await?;
        info!("Updated story: {} (ID: {})", updated_story.title, updated_story.id);
        
        Ok(updated_story)
    }

    /// Delete a story
    pub async fn delete_story(&self, story_id: Uuid) -> Result<bool> {
        let deleted = self.repository.delete_story(story_id).await?;
        if deleted {
            info!("Deleted story with ID: {}", story_id);
        } else {
            warn!("Attempted to delete non-existent story: {}", story_id);
        }
        Ok(deleted)
    }

    /// Get category statistics
    pub async fn get_category_statistics(&self) -> Result<HashMap<String, i64>> {
        self.repository.get_category_statistics().await
    }

    /// Verify integrity of all stories
    pub async fn verify_all_stories_integrity(&self) -> Result<Vec<Uuid>> {
        let problematic_stories = self.repository.get_stories_with_integrity_issues().await?;
        
        if !problematic_stories.is_empty() {
            error!("Found {} stories with integrity issues", problematic_stories.len());
        } else {
            info!("All stories passed integrity verification");
        }

        Ok(problematic_stories)
    }

    /// Verify integrity of a specific story
    pub async fn verify_story_integrity(&self, story_id: Uuid) -> Result<bool> {
        self.repository.verify_story_integrity(story_id).await
    }

    // Private helper methods

    /// Extract themes from story content based on category and content analysis
    fn extract_themes(&self, content: &str, category: &StoryCategory) -> Vec<String> {
        let mut themes = Vec::new();
        let content_lower = content.to_lowercase();

        // Category-based themes
        match category {
            StoryCategory::Prophets => {
                themes.extend_from_slice(&["Prophethood", "Divine Guidance", "Patience", "Faith"]);
            }
            StoryCategory::Companions => {
                themes.extend_from_slice(&["Loyalty", "Sacrifice", "Brotherhood", "Faith"]);
            }
            StoryCategory::MoralLessons => {
                themes.extend_from_slice(&["Morality", "Ethics", "Life Lessons", "Character Building"]);
            }
            StoryCategory::HistoricalEvents => {
                themes.extend_from_slice(&["History", "Islamic Civilization", "Lessons from History"]);
            }
            _ => {
                themes.push("Islamic Values");
            }
        }

        // Content-based theme extraction (simplified)
        let theme_keywords = vec![
            ("patience", "Patience"),
            ("صبر", "Patience"),
            ("justice", "Justice"),
            ("عدل", "Justice"),
            ("mercy", "Mercy"),
            ("رحمة", "Mercy"),
            ("forgiveness", "Forgiveness"),
            ("مغفرة", "Forgiveness"),
            ("courage", "Courage"),
            ("شجاعة", "Courage"),
            ("honesty", "Honesty"),
            ("صدق", "Honesty"),
            ("prayer", "Prayer"),
            ("صلاة", "Prayer"),
            ("charity", "Charity"),
            ("زكاة", "Charity"),
            ("pilgrimage", "Hajj"),
            ("حج", "Hajj"),
            ("fasting", "Fasting"),
            ("صوم", "Fasting"),
        ];

        for (keyword, theme) in theme_keywords {
            if content_lower.contains(keyword) {
                themes.push(theme);
            }
        }

        // Remove duplicates and convert to owned strings
        themes.sort();
        themes.dedup();
        themes.into_iter().map(|s| s.to_string()).collect()
    }

    /// Extract keywords from story content for search optimization
    fn extract_keywords(&self, content: &str) -> Vec<String> {
        let mut keywords = Vec::new();
        
        // Simple keyword extraction (in production, this would use NLP)
        let words: Vec<&str> = content
            .split_whitespace()
            .filter(|word| word.len() > 3) // Filter short words
            .take(20) // Limit to first 20 meaningful words
            .collect();

        for word in words {
            let clean_word = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
            if !clean_word.is_empty() && clean_word.len() > 3 {
                keywords.push(clean_word);
            }
        }

        keywords.sort();
        keywords.dedup();
        keywords
    }

    /// Calculate relevance score for search results
    fn calculate_relevance_score(&self, story: &Story, query: &str) -> f64 {
        let mut score: f64 = 0.0;
        let query_lower = query.to_lowercase();

        // Title match (highest weight)
        if story.title.to_lowercase().contains(&query_lower) {
            score += 10.0;
        }
        if story.arabic_title.to_lowercase().contains(&query_lower) {
            score += 10.0;
        }

        // Content match
        if story.content.to_lowercase().contains(&query_lower) {
            score += 5.0;
        }

        // Theme match
        for theme in &story.themes {
            if theme.to_lowercase().contains(&query_lower) {
                score += 3.0;
            }
        }

        // Keyword match
        for keyword in &story.keywords {
            if keyword.contains(&query_lower) {
                score += 2.0;
            }
        }

        // Moral lesson match
        for lesson in &story.moral_lessons {
            if lesson.to_lowercase().contains(&query_lower) {
                score += 4.0;
            }
        }

        // Boost score based on authenticity level
        match story.authenticity_level {
            AuthenticityLevel::Authentic => score *= 1.5,
            AuthenticityLevel::WellDocumented => score *= 1.3,
            AuthenticityLevel::Probable => score *= 1.1,
            _ => {}
        }

        score.min(100.0) // Cap at 100
    }

    /// Highlight matching text in search results
    fn highlight_text(&self, content: &str, query: &str) -> String {
        let query_lower = query.to_lowercase();
        let content_lower = content.to_lowercase();

        if let Some(pos) = content_lower.find(&query_lower) {
            let start = pos.saturating_sub(50);
            let end = (pos + query.len() + 50).min(content.len());
            let snippet = &content[start..end];
            
            // Simple highlighting (in production, would use proper HTML escaping)
            snippet.replace(query, &format!("**{}**", query))
        } else {
            // Return first 100 characters if no match
            content.chars().take(100).collect::<String>() + "..."
        }
    }

    /// Get matching criteria for search results
    fn get_matching_criteria(&self, story: &Story, request: &SearchStoriesRequest) -> Vec<String> {
        let mut criteria = Vec::new();
        let query_lower = request.query.to_lowercase();

        if story.title.to_lowercase().contains(&query_lower) {
            criteria.push("Title".to_string());
        }
        if story.content.to_lowercase().contains(&query_lower) {
            criteria.push("Content".to_string());
        }
        if story.themes.iter().any(|t| t.to_lowercase().contains(&query_lower)) {
            criteria.push("Themes".to_string());
        }
        if story.moral_lessons.iter().any(|l| l.to_lowercase().contains(&query_lower)) {
            criteria.push("Moral Lessons".to_string());
        }

        if let Some(categories) = &request.categories {
            if categories.contains(&story.category) {
                criteria.push("Category".to_string());
            }
        }

        if let Some(age_groups) = &request.age_groups {
            if age_groups.contains(&story.age_group) {
                criteria.push("Age Group".to_string());
            }
        }

        criteria
    }

    /// Calculate credibility score for a source
    fn calculate_source_credibility(&self, source: &StorySource) -> f64 {
        let base_score = match source.source_type {
            SourceType::Quran => 10.0,
            SourceType::Hadith => {
                match source.authenticity_grade.as_deref() {
                    Some("sahih") => 9.5,
                    Some("hasan") => 8.5,
                    Some("daif") => 6.0,
                    Some("mawdu") => 2.0,
                    _ => 7.0,
                }
            }
            SourceType::HistoricalBook => 7.5,
            SourceType::Biography => 7.0,
            SourceType::Tafsir => 8.0,
            SourceType::ScholarlyWork => 6.5,
        };

        // Adjust based on verification status
        match source.verification_status {
            VerificationStatus::Verified => base_score,
            VerificationStatus::Unverified => base_score * 0.8,
            VerificationStatus::Questionable => base_score * 0.5,
        }
    }

    /// Calculate relevance score for lesson-based search
    fn calculate_lesson_relevance_score(&self, story: &Story, lesson_title: &str) -> f64 {
        let mut score: f64 = 0.0;
        let lesson_lower = lesson_title.to_lowercase();

        // Check if lesson title appears in moral lessons
        for moral_lesson in &story.moral_lessons {
            if moral_lesson.to_lowercase().contains(&lesson_lower) {
                score += 10.0;
            }
        }

        // Check themes
        for theme in &story.themes {
            if theme.to_lowercase().contains(&lesson_lower) {
                score += 5.0;
            }
        }

        // Check content
        if story.content.to_lowercase().contains(&lesson_lower) {
            score += 3.0;
        }

        // Boost based on authenticity
        match story.authenticity_level {
            AuthenticityLevel::Authentic => score *= 1.5,
            AuthenticityLevel::WellDocumented => score *= 1.3,
            _ => {}
        }

        score.min(100.0)
    }

    /// Calculate relevance score for moral category search
    fn calculate_moral_relevance_score(&self, story: &Story, moral_category: &MoralCategory) -> f64 {
        let mut score: f64 = 0.0;
        let moral_arabic = moral_category.arabic_name();
        let moral_english = format!("{:?}", moral_category).to_lowercase();

        // Check moral lessons
        for lesson in &story.moral_lessons {
            let lesson_lower = lesson.to_lowercase();
            if lesson_lower.contains(&moral_english) || lesson.contains(moral_arabic) {
                score += 15.0;
            }
        }

        // Check themes
        for theme in &story.themes {
            let theme_lower = theme.to_lowercase();
            if theme_lower.contains(&moral_english) || theme.contains(moral_arabic) {
                score += 10.0;
            }
        }

        // Check content
        let content_lower = story.content.to_lowercase();
        if content_lower.contains(&moral_english) || story.content.contains(moral_arabic) {
            score += 5.0;
        }

        // Boost based on authenticity
        match story.authenticity_level {
            AuthenticityLevel::Authentic => score *= 1.5,
            AuthenticityLevel::WellDocumented => score *= 1.3,
            _ => {}
        }

        score.min(100.0)
    }

    /// Get related characters (simplified implementation)
    async fn get_related_characters(&self, _character: &Character) -> Result<Vec<Character>> {
        // In a full implementation, this would find characters that appear in the same stories
        // For now, return empty vector
        Ok(Vec::new())
    }

    /// Get related themes (simplified implementation)
    async fn get_related_themes(&self, _theme: &str) -> Result<Vec<String>> {
        // In a full implementation, this would find semantically related themes
        // For now, return some common related themes
        Ok(vec![
            "Islamic Values".to_string(),
            "Moral Guidance".to_string(),
            "Character Building".to_string(),
        ])
    }

    /// Get lessons by theme (simplified implementation)
    async fn get_lessons_by_theme(&self, _theme: &str) -> Result<Vec<Lesson>> {
        // In a full implementation, this would query lessons related to the theme
        // For now, return empty vector
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::StoryRepository;
    use sqlx::PgPool;

    // Note: These tests would require a test database setup
    // They are provided as examples of how to test the service

    async fn setup_test_service() -> StoryService {
        // This would set up a test database and repository
        // For now, we'll just return a placeholder
        todo!("Set up test service")
    }

    #[tokio::test]
    async fn test_create_story() {
        let service = setup_test_service().await;

        let story = service.create_story(
            "Test Story".to_string(),
            "قصة تجريبية".to_string(),
            "This is a test story about patience and faith.".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::Children,
            "en".to_string(),
            AuthenticityLevel::Educational,
        ).await.unwrap();

        assert_eq!(story.title, "Test Story");
        assert!(story.themes.contains(&"Patience".to_string()));
        assert!(story.verify_integrity());
    }

    #[tokio::test]
    async fn test_search_stories() {
        let service = setup_test_service().await;

        let request = SearchStoriesRequest {
            query: "patience".to_string(),
            categories: Some(vec![StoryCategory::MoralLessons]),
            age_groups: Some(vec![AgeGroup::Children]),
            time_periods: None,
            authenticity_levels: None,
            character_names: None,
            themes: None,
            search_type: Some(SearchType::Text),
            limit: Some(10),
            offset: Some(0),
        };

        let response = service.search_stories(request).await.unwrap();
        assert!(response.results.len() <= 10);
        assert_eq!(response.query, "patience");
    }

    #[tokio::test]
    async fn test_theme_extraction() {
        let service = setup_test_service().await;

        let themes = service.extract_themes(
            "This story teaches about patience and justice in difficult times",
            &StoryCategory::MoralLessons,
        );

        assert!(themes.contains(&"Patience".to_string()));
        assert!(themes.contains(&"Justice".to_string()));
        assert!(themes.contains(&"Morality".to_string()));
    }

    #[tokio::test]
    async fn test_relevance_scoring() {
        let service = setup_test_service().await;

        let story = Story::new(
            "Story of Patience".to_string(),
            "قصة الصبر".to_string(),
            "This story teaches about patience in adversity".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::AllAges,
            "en".to_string(),
            AuthenticityLevel::Authentic,
        );

        let score = service.calculate_relevance_score(&story, "patience");
        assert!(score > 0.0);

        let score_title_match = service.calculate_relevance_score(&story, "Patience");
        assert!(score_title_match > score); // Title match should score higher
    }

    #[tokio::test]
    async fn test_source_credibility_calculation() {
        let service = setup_test_service().await;

        let quran_source = StorySource::new(
            Uuid::new_v4(),
            SourceType::Quran,
            "Holy Quran".to_string(),
            "القرآن الكريم".to_string(),
            "Surah Al-Baqarah 2:155".to_string(),
        );

        let credibility = service.calculate_source_credibility(&quran_source);
        assert_eq!(credibility, 10.0); // Quran should have maximum credibility

        let mut hadith_source = StorySource::new(
            Uuid::new_v4(),
            SourceType::Hadith,
            "Sahih Bukhari".to_string(),
            "صحيح البخاري".to_string(),
            "Book 1, Hadith 1".to_string(),
        );
        hadith_source.authenticity_grade = Some("sahih".to_string());

        let hadith_credibility = service.calculate_source_credibility(&hadith_source);
        assert_eq!(hadith_credibility, 9.5); // Sahih hadith should have high credibility
    }
}
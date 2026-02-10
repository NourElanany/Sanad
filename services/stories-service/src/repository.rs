use crate::models::*;
use anyhow::Result;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::collections::HashMap;

/// Repository for managing Islamic stories data access
pub struct StoryRepository {
    pool: PgPool,
}

impl StoryRepository {
    /// Create a new StoryRepository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new story
    pub async fn create_story(&self, story: &Story) -> Result<Story> {
        let row = sqlx::query(
            r#"
            INSERT INTO stories (
                id, title, arabic_title, content, content_hash, summary,
                category, subcategory, time_period, location, word_count,
                estimated_reading_time, age_group, moral_lessons, themes,
                keywords, language, authenticity_level, scholarly_verification
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19
            ) RETURNING *
            "#
        )
        .bind(story.id)
        .bind(&story.title)
        .bind(&story.arabic_title)
        .bind(&story.content)
        .bind(&story.content_hash)
        .bind(&story.summary)
        .bind(format!("{:?}", story.category).to_lowercase())
        .bind(&story.subcategory)
        .bind(story.time_period.as_ref().map(|tp| format!("{:?}", tp).to_lowercase()))
        .bind(&story.location)
        .bind(story.word_count)
        .bind(story.estimated_reading_time)
        .bind(format!("{:?}", story.age_group).to_lowercase())
        .bind(&story.moral_lessons)
        .bind(&story.themes)
        .bind(&story.keywords)
        .bind(&story.language)
        .bind(format!("{:?}", story.authenticity_level).to_lowercase())
        .bind(format!("{:?}", story.scholarly_verification).to_lowercase())
        .fetch_one(&self.pool)
        .await?;

        Ok(Story {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            arabic_title: row.try_get("arabic_title")?,
            content: row.try_get("content")?,
            content_hash: row.try_get("content_hash")?,
            summary: row.try_get("summary")?,
            category: row.try_get::<String, _>("category")?.parse().unwrap_or(StoryCategory::MoralLessons),
            subcategory: row.try_get("subcategory")?,
            time_period: row.try_get::<Option<String>, _>("time_period")?.map(|tp| tp.parse().unwrap_or(TimePeriod::Modern)),
            location: row.try_get("location")?,
            word_count: row.try_get("word_count")?,
            estimated_reading_time: row.try_get("estimated_reading_time")?,
            age_group: row.try_get::<String, _>("age_group")?.parse().unwrap_or(AgeGroup::AllAges),
            moral_lessons: row.try_get::<Option<Vec<String>>, _>("moral_lessons")?.unwrap_or_default(),
            themes: row.try_get::<Option<Vec<String>>, _>("themes")?.unwrap_or_default(),
            keywords: row.try_get::<Option<Vec<String>>, _>("keywords")?.unwrap_or_default(),
            language: row.try_get("language")?,
            authenticity_level: row.try_get::<String, _>("authenticity_level")?.parse().unwrap_or(AuthenticityLevel::Educational),
            scholarly_verification: row.try_get::<String, _>("scholarly_verification")?.parse().unwrap_or(ScholarlyVerification::Pending),
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    /// Get a story by ID
    pub async fn get_story_by_id(&self, story_id: Uuid) -> Result<Option<Story>> {
        let row = sqlx::query(
            "SELECT id, title, arabic_title, content, content_hash, summary, category, subcategory, 
             time_period, location, word_count, estimated_reading_time, age_group, moral_lessons, 
             themes, keywords, language, authenticity_level, scholarly_verification, created_at, updated_at
             FROM stories WHERE id = $1"
        )
        .bind(story_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Story {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                arabic_title: row.try_get("arabic_title")?,
                content: row.try_get("content")?,
                content_hash: row.try_get("content_hash")?,
                summary: row.try_get("summary")?,
                category: row.try_get::<String, _>("category")?.parse().unwrap_or(StoryCategory::MoralLessons),
                subcategory: row.try_get("subcategory")?,
                time_period: row.try_get::<Option<String>, _>("time_period")?.map(|tp| tp.parse().unwrap_or(TimePeriod::Modern)),
                location: row.try_get("location")?,
                word_count: row.try_get("word_count")?,
                estimated_reading_time: row.try_get("estimated_reading_time")?,
                age_group: row.try_get::<String, _>("age_group")?.parse().unwrap_or(AgeGroup::AllAges),
                moral_lessons: row.try_get::<Option<Vec<String>>, _>("moral_lessons")?.unwrap_or_default(),
                themes: row.try_get::<Option<Vec<String>>, _>("themes")?.unwrap_or_default(),
                keywords: row.try_get::<Option<Vec<String>>, _>("keywords")?.unwrap_or_default(),
                language: row.try_get("language")?,
                authenticity_level: row.try_get::<String, _>("authenticity_level")?.parse().unwrap_or(AuthenticityLevel::Educational),
                scholarly_verification: row.try_get::<String, _>("scholarly_verification")?.parse().unwrap_or(ScholarlyVerification::Pending),
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get a story by title
    pub async fn get_story_by_title(&self, title: &str) -> Result<Option<Story>> {
        let row = sqlx::query(
            "SELECT id, title, arabic_title, content, content_hash, summary, category, subcategory, 
             time_period, location, word_count, estimated_reading_time, age_group, moral_lessons, 
             themes, keywords, language, authenticity_level, scholarly_verification, created_at, updated_at
             FROM stories WHERE title = $1 OR arabic_title = $1"
        )
        .bind(title)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Story {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                arabic_title: row.try_get("arabic_title")?,
                content: row.try_get("content")?,
                content_hash: row.try_get("content_hash")?,
                summary: row.try_get("summary")?,
                category: row.try_get::<String, _>("category")?.parse().unwrap_or(StoryCategory::MoralLessons),
                subcategory: row.try_get("subcategory")?,
                time_period: row.try_get::<Option<String>, _>("time_period")?.map(|tp| tp.parse().unwrap_or(TimePeriod::Modern)),
                location: row.try_get("location")?,
                word_count: row.try_get("word_count")?,
                estimated_reading_time: row.try_get("estimated_reading_time")?,
                age_group: row.try_get::<String, _>("age_group")?.parse().unwrap_or(AgeGroup::AllAges),
                moral_lessons: row.try_get::<Option<Vec<String>>, _>("moral_lessons")?.unwrap_or_default(),
                themes: row.try_get::<Option<Vec<String>>, _>("themes")?.unwrap_or_default(),
                keywords: row.try_get::<Option<Vec<String>>, _>("keywords")?.unwrap_or_default(),
                language: row.try_get("language")?,
                authenticity_level: row.try_get::<String, _>("authenticity_level")?.parse().unwrap_or(AuthenticityLevel::Educational),
                scholarly_verification: row.try_get::<String, _>("scholarly_verification")?.parse().unwrap_or(ScholarlyVerification::Pending),
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Search stories with filters
    pub async fn search_stories(
        &self,
        query: &str,
        categories: Option<&[StoryCategory]>,
        age_groups: Option<&[AgeGroup]>,
        authenticity_levels: Option<&[AuthenticityLevel]>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Story>> {
        let mut sql = String::from(
            r#"
            SELECT * FROM stories 
            WHERE (
                title ILIKE $1 OR 
                arabic_title ILIKE $1 OR 
                content ILIKE $1 OR
                $1 = ANY(themes) OR
                $1 = ANY(keywords) OR
                $1 = ANY(moral_lessons)
            )
            "#
        );

        let mut param_count = 1;
        let search_pattern = format!("%{}%", query);

        // Add category filter
        if let Some(cats) = categories {
            if !cats.is_empty() {
                param_count += 1;
                sql.push_str(&format!(" AND category = ANY(${}) ", param_count));
            }
        }

        // Add age group filter
        if let Some(ages) = age_groups {
            if !ages.is_empty() {
                param_count += 1;
                sql.push_str(&format!(" AND age_group = ANY(${}) ", param_count));
            }
        }

        // Add authenticity level filter
        if let Some(auth_levels) = authenticity_levels {
            if !auth_levels.is_empty() {
                param_count += 1;
                sql.push_str(&format!(" AND authenticity_level = ANY(${}) ", param_count));
            }
        }

        sql.push_str(" ORDER BY created_at DESC LIMIT $");
        sql.push_str(&(param_count + 1).to_string());
        sql.push_str(" OFFSET $");
        sql.push_str(&(param_count + 2).to_string());

        let mut query_builder = sqlx::query(&sql).bind(&search_pattern);

        // Bind category parameters
        if let Some(cats) = categories {
            if !cats.is_empty() {
                let cat_strings: Vec<String> = cats.iter().map(|c| format!("{:?}", c).to_lowercase()).collect();
                query_builder = query_builder.bind(cat_strings);
            }
        }

        // Bind age group parameters
        if let Some(ages) = age_groups {
            if !ages.is_empty() {
                let age_strings: Vec<String> = ages.iter().map(|a| format!("{:?}", a).to_lowercase()).collect();
                query_builder = query_builder.bind(age_strings);
            }
        }

        // Bind authenticity level parameters
        if let Some(auth_levels) = authenticity_levels {
            if !auth_levels.is_empty() {
                let auth_strings: Vec<String> = auth_levels.iter().map(|a| format!("{:?}", a).to_lowercase()).collect();
                query_builder = query_builder.bind(auth_strings);
            }
        }

        query_builder = query_builder.bind(limit).bind(offset);

        let rows = query_builder.fetch_all(&self.pool).await?;

        let mut stories = Vec::new();
        for row in rows {
            stories.push(Story {
                id: row.get("id"),
                title: row.get("title"),
                arabic_title: row.get("arabic_title"),
                content: row.get("content"),
                content_hash: row.get("content_hash"),
                summary: row.get("summary"),
                category: row.get::<String, _>("category").parse().unwrap_or(StoryCategory::MoralLessons),
                subcategory: row.get("subcategory"),
                time_period: row.get::<Option<String>, _>("time_period").map(|tp| tp.parse().unwrap_or(TimePeriod::Modern)),
                location: row.get("location"),
                word_count: row.get("word_count"),
                estimated_reading_time: row.get("estimated_reading_time"),
                age_group: row.get::<String, _>("age_group").parse().unwrap_or(AgeGroup::AllAges),
                moral_lessons: row.get::<Option<Vec<String>>, _>("moral_lessons").unwrap_or_default(),
                themes: row.get::<Option<Vec<String>>, _>("themes").unwrap_or_default(),
                keywords: row.get::<Option<Vec<String>>, _>("keywords").unwrap_or_default(),
                language: row.get("language"),
                authenticity_level: row.get::<String, _>("authenticity_level").parse().unwrap_or(AuthenticityLevel::Educational),
                scholarly_verification: row.get::<String, _>("scholarly_verification").parse().unwrap_or(ScholarlyVerification::Pending),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(stories)
    }

    /// Get stories by category
    pub async fn get_stories_by_category(
        &self,
        category: StoryCategory,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Story>> {
        let category_str = format!("{:?}", category).to_lowercase();
        let rows = sqlx::query(
            "SELECT * FROM stories WHERE category = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(category_str)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let mut stories = Vec::new();
        for row in rows {
            stories.push(Story {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                arabic_title: row.try_get("arabic_title")?,
                content: row.try_get("content")?,
                content_hash: row.try_get("content_hash")?,
                summary: row.try_get("summary")?,
                category: row.try_get::<String, _>("category")?.parse().unwrap_or(StoryCategory::MoralLessons),
                subcategory: row.try_get("subcategory")?,
                time_period: row.try_get::<Option<String>, _>("time_period")?.map(|tp| tp.parse().unwrap_or(TimePeriod::Modern)),
                location: row.try_get("location")?,
                word_count: row.try_get("word_count")?,
                estimated_reading_time: row.try_get("estimated_reading_time")?,
                age_group: row.try_get::<String, _>("age_group")?.parse().unwrap_or(AgeGroup::AllAges),
                moral_lessons: row.try_get::<Option<Vec<String>>, _>("moral_lessons")?.unwrap_or_default(),
                themes: row.try_get::<Option<Vec<String>>, _>("themes")?.unwrap_or_default(),
                keywords: row.try_get::<Option<Vec<String>>, _>("keywords")?.unwrap_or_default(),
                language: row.try_get("language")?,
                authenticity_level: row.try_get::<String, _>("authenticity_level")?.parse().unwrap_or(AuthenticityLevel::Educational),
                scholarly_verification: row.try_get::<String, _>("scholarly_verification")?.parse().unwrap_or(ScholarlyVerification::Pending),
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            });
        }

        Ok(stories)
    }

    /// Update a story
    pub async fn update_story(&self, story: &Story) -> Result<Story> {
        let row = sqlx::query(
            r#"
            UPDATE stories SET
                title = $2, arabic_title = $3, content = $4, content_hash = $5,
                summary = $6, category = $7, subcategory = $8, time_period = $9,
                location = $10, word_count = $11, estimated_reading_time = $12,
                age_group = $13, moral_lessons = $14, themes = $15, keywords = $16,
                language = $17, authenticity_level = $18, scholarly_verification = $19,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(story.id)
        .bind(&story.title)
        .bind(&story.arabic_title)
        .bind(&story.content)
        .bind(&story.content_hash)
        .bind(&story.summary)
        .bind(format!("{:?}", story.category).to_lowercase())
        .bind(&story.subcategory)
        .bind(story.time_period.as_ref().map(|tp| format!("{:?}", tp).to_lowercase()))
        .bind(&story.location)
        .bind(story.word_count)
        .bind(story.estimated_reading_time)
        .bind(format!("{:?}", story.age_group).to_lowercase())
        .bind(&story.moral_lessons)
        .bind(&story.themes)
        .bind(&story.keywords)
        .bind(&story.language)
        .bind(format!("{:?}", story.authenticity_level).to_lowercase())
        .bind(format!("{:?}", story.scholarly_verification).to_lowercase())
        .fetch_one(&self.pool)
        .await?;

        Ok(Story {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            arabic_title: row.try_get("arabic_title")?,
            content: row.try_get("content")?,
            content_hash: row.try_get("content_hash")?,
            summary: row.try_get("summary")?,
            category: row.try_get::<String, _>("category")?.parse().unwrap_or(StoryCategory::MoralLessons),
            subcategory: row.try_get("subcategory")?,
            time_period: row.try_get::<Option<String>, _>("time_period")?.map(|tp| tp.parse().unwrap_or(TimePeriod::Modern)),
            location: row.try_get("location")?,
            word_count: row.try_get("word_count")?,
            estimated_reading_time: row.try_get("estimated_reading_time")?,
            age_group: row.try_get::<String, _>("age_group")?.parse().unwrap_or(AgeGroup::AllAges),
            moral_lessons: row.try_get::<Option<Vec<String>>, _>("moral_lessons")?.unwrap_or_default(),
            themes: row.try_get::<Option<Vec<String>>, _>("themes")?.unwrap_or_default(),
            keywords: row.try_get::<Option<Vec<String>>, _>("keywords")?.unwrap_or_default(),
            language: row.try_get("language")?,
            authenticity_level: row.try_get::<String, _>("authenticity_level")?.parse().unwrap_or(AuthenticityLevel::Educational),
            scholarly_verification: row.try_get::<String, _>("scholarly_verification")?.parse().unwrap_or(ScholarlyVerification::Pending),
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    /// Delete a story
    pub async fn delete_story(&self, story_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "DELETE FROM stories WHERE id = $1"
        )
        .bind(story_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Create a new character
    pub async fn create_character(&self, character: &Character) -> Result<Character> {
        let row = sqlx::query(
            r#"
            INSERT INTO characters (
                id, name, arabic_name, character_type, description,
                historical_period, birth_year, death_year, biography,
                virtues, role_significance
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            ) RETURNING id, name, arabic_name, character_type, description,
                historical_period, birth_year, death_year, biography,
                virtues, role_significance, related_stories_count, created_at, updated_at
            "#
        )
        .bind(character.id)
        .bind(&character.name)
        .bind(&character.arabic_name)
        .bind(format!("{:?}", character.character_type).to_lowercase())
        .bind(&character.description)
        .bind(character.historical_period.as_ref().map(|hp| format!("{:?}", hp).to_lowercase()))
        .bind(character.birth_year)
        .bind(character.death_year)
        .bind(&character.biography)
        .bind(&character.virtues)
        .bind(&character.role_significance)
        .fetch_one(&self.pool)
        .await?;

        Ok(Character {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            arabic_name: row.try_get("arabic_name")?,
            character_type: row.try_get::<String, _>("character_type")?.parse().unwrap_or(CharacterType::HistoricalFigure),
            description: row.try_get("description")?,
            historical_period: row.try_get::<Option<String>, _>("historical_period")?.map(|hp| hp.parse().unwrap_or(TimePeriod::Modern)),
            birth_year: row.try_get("birth_year")?,
            death_year: row.try_get("death_year")?,
            biography: row.try_get("biography")?,
            virtues: row.try_get::<Option<Vec<String>>, _>("virtues")?.unwrap_or_default(),
            role_significance: row.try_get("role_significance")?,
            related_stories_count: row.try_get("related_stories_count")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    /// Get a character by ID
    pub async fn get_character_by_id(&self, character_id: Uuid) -> Result<Option<Character>> {
        let row = sqlx::query(
            "SELECT id, name, arabic_name, character_type, description, historical_period, 
             birth_year, death_year, biography, virtues, role_significance, related_stories_count, 
             created_at, updated_at FROM characters WHERE id = $1"
        )
        .bind(character_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Character {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                arabic_name: row.try_get("arabic_name")?,
                character_type: row.try_get::<String, _>("character_type")?.parse().unwrap_or(CharacterType::HistoricalFigure),
                description: row.try_get("description")?,
                historical_period: row.try_get::<Option<String>, _>("historical_period")?.map(|hp| hp.parse().unwrap_or(TimePeriod::Modern)),
                birth_year: row.try_get("birth_year")?,
                death_year: row.try_get("death_year")?,
                biography: row.try_get("biography")?,
                virtues: row.try_get::<Option<Vec<String>>, _>("virtues")?.unwrap_or_default(),
                role_significance: row.try_get("role_significance")?,
                related_stories_count: row.try_get("related_stories_count")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get characters by name
    pub async fn get_characters_by_name(&self, name: &str) -> Result<Vec<Character>> {
        let rows = sqlx::query(
            "SELECT id, name, arabic_name, character_type, description, historical_period, 
             birth_year, death_year, biography, virtues, role_significance, related_stories_count, 
             created_at, updated_at FROM characters WHERE name ILIKE $1 OR arabic_name ILIKE $1"
        )
        .bind(format!("%{}%", name))
        .fetch_all(&self.pool)
        .await?;

        let mut characters = Vec::new();
        for row in rows {
            characters.push(Character {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                arabic_name: row.try_get("arabic_name")?,
                character_type: row.try_get::<String, _>("character_type")?.parse().unwrap_or(CharacterType::HistoricalFigure),
                description: row.try_get("description")?,
                historical_period: row.try_get::<Option<String>, _>("historical_period")?.map(|hp| hp.parse().unwrap_or(TimePeriod::Modern)),
                birth_year: row.try_get("birth_year")?,
                death_year: row.try_get("death_year")?,
                biography: row.try_get("biography")?,
                virtues: row.try_get::<Option<Vec<String>>, _>("virtues")?.unwrap_or_default(),
                role_significance: row.try_get("role_significance")?,
                related_stories_count: row.try_get("related_stories_count")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            });
        }

        Ok(characters)
    }

    /// Get characters for a story
    pub async fn get_story_characters(&self, story_id: Uuid) -> Result<Vec<CharacterInStory>> {
        let rows = sqlx::query(
            r#"
            SELECT c.id, c.name, c.arabic_name, c.character_type, c.description, c.historical_period,
                   c.birth_year, c.death_year, c.biography, c.virtues, c.role_significance, 
                   c.related_stories_count, c.created_at, c.updated_at,
                   sc.role_in_story, sc.importance_level, sc.character_description_in_story
            FROM characters c
            JOIN story_characters sc ON c.id = sc.character_id
            WHERE sc.story_id = $1
            ORDER BY sc.importance_level, c.name
            "#
        )
        .bind(story_id)
        .fetch_all(&self.pool)
        .await?;

        let mut characters = Vec::new();
        for row in rows {
            characters.push(CharacterInStory {
                character: Character {
                    id: row.try_get("id")?,
                    name: row.try_get("name")?,
                    arabic_name: row.try_get("arabic_name")?,
                    character_type: row.try_get::<String, _>("character_type")?.parse().unwrap_or(CharacterType::HistoricalFigure),
                    description: row.try_get("description")?,
                    historical_period: row.try_get::<Option<String>, _>("historical_period")?.map(|hp| hp.parse().unwrap_or(TimePeriod::Modern)),
                    birth_year: row.try_get("birth_year")?,
                    death_year: row.try_get("death_year")?,
                    biography: row.try_get("biography")?,
                    virtues: row.try_get::<Option<Vec<String>>, _>("virtues")?.unwrap_or_default(),
                    role_significance: row.try_get("role_significance")?,
                    related_stories_count: row.try_get("related_stories_count")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                },
                role_in_story: row.try_get::<String, _>("role_in_story")?.parse().unwrap_or(CharacterRole::Supporting),
                importance_level: row.try_get::<String, _>("importance_level")?.parse().unwrap_or(ImportanceLevel::Minor),
                character_description_in_story: row.try_get("character_description_in_story")?,
            });
        }

        Ok(characters)
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
        sqlx::query(
            r#"
            INSERT INTO story_characters (story_id, character_id, role_in_story, importance_level, character_description_in_story)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (story_id, character_id) DO UPDATE SET
                role_in_story = EXCLUDED.role_in_story,
                importance_level = EXCLUDED.importance_level,
                character_description_in_story = EXCLUDED.character_description_in_story
            "#
        )
        .bind(story_id)
        .bind(character_id)
        .bind(format!("{:?}", role).to_lowercase())
        .bind(format!("{:?}", importance).to_lowercase())
        .bind(description)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Create a new lesson
    pub async fn create_lesson(&self, lesson: &Lesson) -> Result<Lesson> {
        let row = sqlx::query(
            r#"
            INSERT INTO lessons (
                id, title, arabic_title, description, lesson_type,
                moral_category, practical_application, target_audience,
                related_verses, related_hadiths
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
            ) RETURNING id, title, arabic_title, description, lesson_type,
                moral_category, practical_application, target_audience,
                related_verses, related_hadiths, created_at, updated_at
            "#
        )
        .bind(lesson.id)
        .bind(&lesson.title)
        .bind(&lesson.arabic_title)
        .bind(&lesson.description)
        .bind(format!("{:?}", lesson.lesson_type).to_lowercase())
        .bind(format!("{:?}", lesson.moral_category).to_lowercase())
        .bind(&lesson.practical_application)
        .bind(&lesson.target_audience.iter().map(|ag| format!("{:?}", ag).to_lowercase()).collect::<Vec<_>>())
        .bind(&lesson.related_verses)
        .bind(&lesson.related_hadiths)
        .fetch_one(&self.pool)
        .await?;

        Ok(Lesson {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            arabic_title: row.try_get("arabic_title")?,
            description: row.try_get("description")?,
            lesson_type: row.try_get::<String, _>("lesson_type")?.parse().unwrap_or(LessonType::Moral),
            moral_category: row.try_get::<String, _>("moral_category")?.parse().unwrap_or(MoralCategory::Faith),
            practical_application: row.try_get("practical_application")?,
            target_audience: row.try_get::<Option<Vec<String>>, _>("target_audience")?.unwrap_or_default().iter()
                .filter_map(|s| s.parse().ok())
                .collect(),
            related_verses: row.try_get::<Option<Vec<String>>, _>("related_verses")?.unwrap_or_default(),
            related_hadiths: row.try_get::<Option<Vec<String>>, _>("related_hadiths")?.unwrap_or_default(),
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    /// Get lessons for a story
    pub async fn get_story_lessons(&self, story_id: Uuid) -> Result<Vec<LessonInStory>> {
        let rows = sqlx::query(
            r#"
            SELECT l.id, l.title, l.arabic_title, l.description, l.lesson_type,
                   l.moral_category, l.practical_application, l.target_audience,
                   l.related_verses, l.related_hadiths, l.created_at, l.updated_at,
                   sl.relevance_score, sl.explanation
            FROM lessons l
            JOIN story_lessons sl ON l.id = sl.lesson_id
            WHERE sl.story_id = $1
            ORDER BY sl.relevance_score DESC
            "#
        )
        .bind(story_id)
        .fetch_all(&self.pool)
        .await?;

        let mut lessons = Vec::new();
        for row in rows {
            lessons.push(LessonInStory {
                lesson: Lesson {
                    id: row.try_get("id")?,
                    title: row.try_get("title")?,
                    arabic_title: row.try_get("arabic_title")?,
                    description: row.try_get("description")?,
                    lesson_type: row.try_get::<String, _>("lesson_type")?.parse().unwrap_or(LessonType::Moral),
                    moral_category: row.try_get::<String, _>("moral_category")?.parse().unwrap_or(MoralCategory::Faith),
                    practical_application: row.try_get("practical_application")?,
                    target_audience: row.try_get::<Option<Vec<String>>, _>("target_audience")?.unwrap_or_default().iter()
                        .filter_map(|s| s.parse().ok())
                        .collect(),
                    related_verses: row.try_get::<Option<Vec<String>>, _>("related_verses")?.unwrap_or_default(),
                    related_hadiths: row.try_get::<Option<Vec<String>>, _>("related_hadiths")?.unwrap_or_default(),
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                },
                relevance_score: row.try_get::<Option<f32>, _>("relevance_score")?.unwrap_or(5.0) as f64,
                explanation: row.try_get("explanation")?,
            });
        }

        Ok(lessons)
    }

    /// Add a lesson to a story
    pub async fn add_lesson_to_story(
        &self,
        story_id: Uuid,
        lesson_id: Uuid,
        relevance_score: f64,
        explanation: Option<String>,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO story_lessons (story_id, lesson_id, relevance_score, explanation)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (story_id, lesson_id) DO UPDATE SET
                relevance_score = EXCLUDED.relevance_score,
                explanation = EXCLUDED.explanation
            "#
        )
        .bind(story_id)
        .bind(lesson_id)
        .bind(relevance_score as f32)
        .bind(explanation)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Create a story source
    pub async fn create_story_source(&self, source: &StorySource) -> Result<StorySource> {
        let row = sqlx::query(
            r#"
            INSERT INTO story_sources (
                id, story_id, source_type, source_name, arabic_source_name,
                author, reference, authenticity_grade, credibility_score,
                verification_status, notes
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            ) RETURNING id, story_id, source_type, source_name, arabic_source_name,
                author, reference, authenticity_grade, credibility_score,
                verification_status, notes, created_at, updated_at
            "#
        )
        .bind(source.id)
        .bind(source.story_id)
        .bind(format!("{:?}", source.source_type).to_lowercase())
        .bind(&source.source_name)
        .bind(&source.arabic_source_name)
        .bind(&source.author)
        .bind(&source.reference)
        .bind(&source.authenticity_grade)
        .bind(source.credibility_score as f32)
        .bind(format!("{:?}", source.verification_status).to_lowercase())
        .bind(&source.notes)
        .fetch_one(&self.pool)
        .await?;

        Ok(StorySource {
            id: row.try_get("id")?,
            story_id: row.try_get("story_id")?,
            source_type: row.try_get::<String, _>("source_type")?.parse().unwrap_or(SourceType::ScholarlyWork),
            source_name: row.try_get("source_name")?,
            arabic_source_name: row.try_get("arabic_source_name")?,
            author: row.try_get("author")?,
            reference: row.try_get("reference")?,
            authenticity_grade: row.try_get("authenticity_grade")?,
            credibility_score: row.try_get::<Option<f32>, _>("credibility_score")?.unwrap_or(5.0) as f64,
            verification_status: row.try_get::<String, _>("verification_status")?.parse().unwrap_or(VerificationStatus::Unverified),
            notes: row.try_get("notes")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    /// Get sources for a story
    pub async fn get_story_sources(&self, story_id: Uuid) -> Result<Vec<StorySource>> {
        let rows = sqlx::query(
            "SELECT id, story_id, source_type, source_name, arabic_source_name, author, reference, 
             authenticity_grade, credibility_score, verification_status, notes, created_at, updated_at 
             FROM story_sources WHERE story_id = $1 ORDER BY credibility_score DESC"
        )
        .bind(story_id)
        .fetch_all(&self.pool)
        .await?;

        let mut sources = Vec::new();
        for row in rows {
            sources.push(StorySource {
                id: row.try_get("id")?,
                story_id: row.try_get("story_id")?,
                source_type: row.try_get::<String, _>("source_type")?.parse().unwrap_or(SourceType::ScholarlyWork),
                source_name: row.try_get("source_name")?,
                arabic_source_name: row.try_get("arabic_source_name")?,
                author: row.try_get("author")?,
                reference: row.try_get("reference")?,
                authenticity_grade: row.try_get("authenticity_grade")?,
                credibility_score: row.try_get::<Option<f32>, _>("credibility_score")?.unwrap_or(5.0) as f64,
                verification_status: row.try_get::<String, _>("verification_status")?.parse().unwrap_or(VerificationStatus::Unverified),
                notes: row.try_get("notes")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            });
        }

        Ok(sources)
    }

    /// Get story with all details
    pub async fn get_story_with_details(&self, story_id: Uuid) -> Result<Option<StoryWithDetails>> {
        if let Some(story) = self.get_story_by_id(story_id).await? {
            let characters = self.get_story_characters(story_id).await?;
            let lessons = self.get_story_lessons(story_id).await?;
            let sources = self.get_story_sources(story_id).await?;
            let collections = self.get_story_collections(story_id).await?;

            Ok(Some(StoryWithDetails {
                story,
                characters,
                lessons,
                sources,
                collections,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get collections that contain a story
    pub async fn get_story_collections(&self, story_id: Uuid) -> Result<Vec<StoryCollection>> {
        let rows = sqlx::query(
            r#"
            SELECT sc.id, sc.name, sc.arabic_name, sc.description, sc.collection_type, 
                   sc.story_count, sc.target_age_group, sc.themes, sc.created_at, sc.updated_at
            FROM story_collections sc
            JOIN story_collection_members scm ON sc.id = scm.collection_id
            WHERE scm.story_id = $1
            ORDER BY sc.name
            "#
        )
        .bind(story_id)
        .fetch_all(&self.pool)
        .await?;

        let mut collections = Vec::new();
        for row in rows {
            collections.push(StoryCollection {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                arabic_name: row.try_get("arabic_name")?,
                description: row.try_get("description")?,
                collection_type: row.try_get::<String, _>("collection_type")?.parse().unwrap_or(CollectionType::Thematic),
                story_count: row.try_get("story_count")?,
                target_age_group: row.try_get::<Option<String>, _>("target_age_group")?.map(|ag| ag.parse().unwrap_or(AgeGroup::AllAges)),
                themes: row.try_get::<Option<Vec<String>>, _>("themes")?.unwrap_or_default(),
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            });
        }

        Ok(collections)
    }

    /// Get stories by character
    pub async fn get_stories_by_character(
        &self,
        character_name: &str,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Story>> {
        let rows = sqlx::query(
            r#"
            SELECT DISTINCT s.id, s.title, s.arabic_title, s.content, s.content_hash, s.summary,
                   s.category, s.subcategory, s.time_period, s.location, s.word_count,
                   s.estimated_reading_time, s.age_group, s.moral_lessons, s.themes, s.keywords,
                   s.language, s.authenticity_level, s.scholarly_verification, s.created_at, s.updated_at
            FROM stories s
            JOIN story_characters sc ON s.id = sc.story_id
            JOIN characters c ON sc.character_id = c.id
            WHERE c.name ILIKE $1 OR c.arabic_name ILIKE $1
            ORDER BY s.created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(format!("%{}%", character_name))
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let mut stories = Vec::new();
        for row in rows {
            stories.push(Story {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                arabic_title: row.try_get("arabic_title")?,
                content: row.try_get("content")?,
                content_hash: row.try_get("content_hash")?,
                summary: row.try_get("summary")?,
                category: row.try_get::<String, _>("category")?.parse().unwrap_or(StoryCategory::MoralLessons),
                subcategory: row.try_get("subcategory")?,
                time_period: row.try_get::<Option<String>, _>("time_period")?.map(|tp| tp.parse().unwrap_or(TimePeriod::Modern)),
                location: row.try_get("location")?,
                word_count: row.try_get("word_count")?,
                estimated_reading_time: row.try_get("estimated_reading_time")?,
                age_group: row.try_get::<String, _>("age_group")?.parse().unwrap_or(AgeGroup::AllAges),
                moral_lessons: row.try_get::<Option<Vec<String>>, _>("moral_lessons")?.unwrap_or_default(),
                themes: row.try_get::<Option<Vec<String>>, _>("themes")?.unwrap_or_default(),
                keywords: row.try_get::<Option<Vec<String>>, _>("keywords")?.unwrap_or_default(),
                language: row.try_get("language")?,
                authenticity_level: row.try_get::<String, _>("authenticity_level")?.parse().unwrap_or(AuthenticityLevel::Educational),
                scholarly_verification: row.try_get::<String, _>("scholarly_verification")?.parse().unwrap_or(ScholarlyVerification::Pending),
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            });
        }

        Ok(stories)
    }

    /// Get stories by theme
    pub async fn get_stories_by_theme(
        &self,
        theme: &str,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Story>> {
        let rows = sqlx::query(
            "SELECT id, title, arabic_title, content, content_hash, summary, category, subcategory,
             time_period, location, word_count, estimated_reading_time, age_group, moral_lessons,
             themes, keywords, language, authenticity_level, scholarly_verification, created_at, updated_at
             FROM stories WHERE $1 = ANY(themes) ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(theme)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let mut stories = Vec::new();
        for row in rows {
            stories.push(Story {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                arabic_title: row.try_get("arabic_title")?,
                content: row.try_get("content")?,
                content_hash: row.try_get("content_hash")?,
                summary: row.try_get("summary")?,
                category: row.try_get::<String, _>("category")?.parse().unwrap_or(StoryCategory::MoralLessons),
                subcategory: row.try_get("subcategory")?,
                time_period: row.try_get::<Option<String>, _>("time_period")?.map(|tp| tp.parse().unwrap_or(TimePeriod::Modern)),
                location: row.try_get("location")?,
                word_count: row.try_get("word_count")?,
                estimated_reading_time: row.try_get("estimated_reading_time")?,
                age_group: row.try_get::<String, _>("age_group")?.parse().unwrap_or(AgeGroup::AllAges),
                moral_lessons: row.try_get::<Option<Vec<String>>, _>("moral_lessons")?.unwrap_or_default(),
                themes: row.try_get::<Option<Vec<String>>, _>("themes")?.unwrap_or_default(),
                keywords: row.try_get::<Option<Vec<String>>, _>("keywords")?.unwrap_or_default(),
                language: row.try_get("language")?,
                authenticity_level: row.try_get::<String, _>("authenticity_level")?.parse().unwrap_or(AuthenticityLevel::Educational),
                scholarly_verification: row.try_get::<String, _>("scholarly_verification")?.parse().unwrap_or(ScholarlyVerification::Pending),
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            });
        }

        Ok(stories)
    }

    /// Get category statistics
    pub async fn get_category_statistics(&self) -> Result<HashMap<String, i64>> {
        let rows = sqlx::query(
            "SELECT category, COUNT(*) as count FROM stories GROUP BY category"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut stats = HashMap::new();
        for row in rows {
            stats.insert(row.try_get("category")?, row.try_get::<Option<i64>, _>("count")?.unwrap_or(0));
        }

        Ok(stats)
    }

    /// Verify story integrity
    pub async fn verify_story_integrity(&self, story_id: Uuid) -> Result<bool> {
        if let Some(story) = self.get_story_by_id(story_id).await? {
            Ok(story.verify_integrity())
        } else {
            Ok(false)
        }
    }

    /// Get stories with integrity issues
    pub async fn get_stories_with_integrity_issues(&self) -> Result<Vec<Uuid>> {
        let rows = sqlx::query(
            "SELECT id, content, content_hash FROM stories"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut problematic_stories = Vec::new();
        for row in rows {
            let id: Uuid = row.try_get("id")?;
            let content: String = row.try_get("content")?;
            let content_hash: String = row.try_get("content_hash")?;
            let calculated_hash = Story::generate_hash(&content);
            if calculated_hash != content_hash {
                problematic_stories.push(id);
            }
        }

        Ok(problematic_stories)
    }

    /// Get a lesson by ID
    pub async fn get_lesson_by_id(&self, lesson_id: Uuid) -> Result<Option<Lesson>> {
        let row = sqlx::query(
            "SELECT id, title, arabic_title, description, lesson_type, moral_category, 
             practical_application, target_audience, related_verses, related_hadiths, 
             created_at, updated_at FROM lessons WHERE id = $1"
        )
        .bind(lesson_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Lesson {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                arabic_title: row.try_get("arabic_title")?,
                description: row.try_get("description")?,
                lesson_type: row.try_get::<String, _>("lesson_type")?.parse().unwrap_or(LessonType::Moral),
                moral_category: row.try_get::<String, _>("moral_category")?.parse().unwrap_or(MoralCategory::Faith),
                practical_application: row.try_get("practical_application")?,
                target_audience: row.try_get::<Option<Vec<String>>, _>("target_audience")?.unwrap_or_default().iter()
                    .filter_map(|s| s.parse().ok())
                    .collect(),
                related_verses: row.try_get::<Option<Vec<String>>, _>("related_verses")?.unwrap_or_default(),
                related_hadiths: row.try_get::<Option<Vec<String>>, _>("related_hadiths")?.unwrap_or_default(),
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Search lessons
    pub async fn search_lessons(
        &self,
        query: &str,
        lesson_type: Option<LessonType>,
        moral_category: Option<MoralCategory>,
        target_audience: Option<AgeGroup>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Lesson>> {
        let mut sql = String::from(
            r#"
            SELECT * FROM lessons 
            WHERE (
                title ILIKE $1 OR 
                arabic_title ILIKE $1 OR 
                description ILIKE $1
            )
            "#
        );

        let mut param_count = 1;
        let search_pattern = format!("%{}%", query);

        // Add lesson type filter
        if let Some(lt) = lesson_type {
            param_count += 1;
            sql.push_str(&format!(" AND lesson_type = ${} ", param_count));
        }

        // Add moral category filter
        if let Some(mc) = moral_category {
            param_count += 1;
            sql.push_str(&format!(" AND moral_category = ${} ", param_count));
        }

        // Add target audience filter
        if let Some(ta) = target_audience {
            param_count += 1;
            sql.push_str(&format!(" AND ${} = ANY(target_audience) ", param_count));
        }

        sql.push_str(" ORDER BY created_at DESC LIMIT $");
        sql.push_str(&(param_count + 1).to_string());
        sql.push_str(" OFFSET $");
        sql.push_str(&(param_count + 2).to_string());

        let mut query_builder = sqlx::query(&sql).bind(&search_pattern);

        if let Some(lt) = lesson_type {
            query_builder = query_builder.bind(format!("{:?}", lt).to_lowercase());
        }

        if let Some(mc) = moral_category {
            query_builder = query_builder.bind(format!("{:?}", mc).to_lowercase());
        }

        if let Some(ta) = target_audience {
            query_builder = query_builder.bind(format!("{:?}", ta).to_lowercase());
        }

        query_builder = query_builder.bind(limit).bind(offset);

        let rows = query_builder.fetch_all(&self.pool).await?;

        let mut lessons = Vec::new();
        for row in rows {
            lessons.push(Lesson {
                id: row.get("id"),
                title: row.get("title"),
                arabic_title: row.get("arabic_title"),
                description: row.get("description"),
                lesson_type: row.get::<String, _>("lesson_type").parse().unwrap_or(LessonType::Moral),
                moral_category: row.get::<String, _>("moral_category").parse().unwrap_or(MoralCategory::Faith),
                practical_application: row.get("practical_application"),
                target_audience: row.get::<Option<Vec<String>>, _>("target_audience").unwrap_or_default().iter()
                    .filter_map(|s| s.parse().ok())
                    .collect(),
                related_verses: row.get::<Option<Vec<String>>, _>("related_verses").unwrap_or_default(),
                related_hadiths: row.get::<Option<Vec<String>>, _>("related_hadiths").unwrap_or_default(),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(lessons)
    }

    /// Get stories by lesson
    pub async fn get_stories_by_lesson(
        &self,
        lesson_title: &str,
        lesson_type: Option<LessonType>,
        moral_category: Option<MoralCategory>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Story>> {
        let mut sql = String::from(
            r#"
            SELECT DISTINCT s.*
            FROM stories s
            JOIN story_lessons sl ON s.id = sl.story_id
            JOIN lessons l ON sl.lesson_id = l.id
            WHERE (l.title ILIKE $1 OR l.arabic_title ILIKE $1)
            "#
        );

        let mut param_count = 1;
        let search_pattern = format!("%{}%", lesson_title);

        if let Some(lt) = lesson_type {
            param_count += 1;
            sql.push_str(&format!(" AND l.lesson_type = ${} ", param_count));
        }

        if let Some(mc) = moral_category {
            param_count += 1;
            sql.push_str(&format!(" AND l.moral_category = ${} ", param_count));
        }

        sql.push_str(" ORDER BY sl.relevance_score DESC, s.created_at DESC LIMIT $");
        sql.push_str(&(param_count + 1).to_string());
        sql.push_str(" OFFSET $");
        sql.push_str(&(param_count + 2).to_string());

        let mut query_builder = sqlx::query(&sql).bind(&search_pattern);

        if let Some(lt) = lesson_type {
            query_builder = query_builder.bind(format!("{:?}", lt).to_lowercase());
        }

        if let Some(mc) = moral_category {
            query_builder = query_builder.bind(format!("{:?}", mc).to_lowercase());
        }

        query_builder = query_builder.bind(limit).bind(offset);

        let rows = query_builder.fetch_all(&self.pool).await?;

        let mut stories = Vec::new();
        for row in rows {
            stories.push(Story {
                id: row.get("id"),
                title: row.get("title"),
                arabic_title: row.get("arabic_title"),
                content: row.get("content"),
                content_hash: row.get("content_hash"),
                summary: row.get("summary"),
                category: row.get::<String, _>("category").parse().unwrap_or(StoryCategory::MoralLessons),
                subcategory: row.get("subcategory"),
                time_period: row.get::<Option<String>, _>("time_period").map(|tp| tp.parse().unwrap_or(TimePeriod::Modern)),
                location: row.get("location"),
                word_count: row.get("word_count"),
                estimated_reading_time: row.get("estimated_reading_time"),
                age_group: row.get::<String, _>("age_group").parse().unwrap_or(AgeGroup::AllAges),
                moral_lessons: row.get::<Option<Vec<String>>, _>("moral_lessons").unwrap_or_default(),
                themes: row.get::<Option<Vec<String>>, _>("themes").unwrap_or_default(),
                keywords: row.get::<Option<Vec<String>>, _>("keywords").unwrap_or_default(),
                language: row.get("language"),
                authenticity_level: row.get::<String, _>("authenticity_level").parse().unwrap_or(AuthenticityLevel::Educational),
                scholarly_verification: row.get::<String, _>("scholarly_verification").parse().unwrap_or(ScholarlyVerification::Pending),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(stories)
    }

    /// Get stories by moral category
    pub async fn get_stories_by_moral_category(
        &self,
        moral_category: MoralCategory,
        lesson_type: Option<LessonType>,
        age_group: Option<AgeGroup>,
        authenticity_level: Option<AuthenticityLevel>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Story>> {
        let mut sql = String::from(
            r#"
            SELECT DISTINCT s.*
            FROM stories s
            JOIN story_lessons sl ON s.id = sl.story_id
            JOIN lessons l ON sl.lesson_id = l.id
            WHERE l.moral_category = $1
            "#
        );

        let mut param_count = 1;

        if let Some(lt) = lesson_type {
            param_count += 1;
            sql.push_str(&format!(" AND l.lesson_type = ${} ", param_count));
        }

        if let Some(ag) = age_group {
            param_count += 1;
            sql.push_str(&format!(" AND s.age_group = ${} ", param_count));
        }

        if let Some(al) = authenticity_level {
            param_count += 1;
            sql.push_str(&format!(" AND s.authenticity_level = ${} ", param_count));
        }

        sql.push_str(" ORDER BY sl.relevance_score DESC, s.created_at DESC LIMIT $");
        sql.push_str(&(param_count + 1).to_string());
        sql.push_str(" OFFSET $");
        sql.push_str(&(param_count + 2).to_string());

        let mut query_builder = sqlx::query(&sql).bind(format!("{:?}", moral_category).to_lowercase());

        if let Some(lt) = lesson_type {
            query_builder = query_builder.bind(format!("{:?}", lt).to_lowercase());
        }

        if let Some(ag) = age_group {
            query_builder = query_builder.bind(format!("{:?}", ag).to_lowercase());
        }

        if let Some(al) = authenticity_level {
            query_builder = query_builder.bind(format!("{:?}", al).to_lowercase());
        }

        query_builder = query_builder.bind(limit).bind(offset);

        let rows = query_builder.fetch_all(&self.pool).await?;

        let mut stories = Vec::new();
        for row in rows {
            stories.push(Story {
                id: row.get("id"),
                title: row.get("title"),
                arabic_title: row.get("arabic_title"),
                content: row.get("content"),
                content_hash: row.get("content_hash"),
                summary: row.get("summary"),
                category: row.get::<String, _>("category").parse().unwrap_or(StoryCategory::MoralLessons),
                subcategory: row.get("subcategory"),
                time_period: row.get::<Option<String>, _>("time_period").map(|tp| tp.parse().unwrap_or(TimePeriod::Modern)),
                location: row.get("location"),
                word_count: row.get("word_count"),
                estimated_reading_time: row.get("estimated_reading_time"),
                age_group: row.get::<String, _>("age_group").parse().unwrap_or(AgeGroup::AllAges),
                moral_lessons: row.get::<Option<Vec<String>>, _>("moral_lessons").unwrap_or_default(),
                themes: row.get::<Option<Vec<String>>, _>("themes").unwrap_or_default(),
                keywords: row.get::<Option<Vec<String>>, _>("keywords").unwrap_or_default(),
                language: row.get("language"),
                authenticity_level: row.get::<String, _>("authenticity_level").parse().unwrap_or(AuthenticityLevel::Educational),
                scholarly_verification: row.get::<String, _>("scholarly_verification").parse().unwrap_or(ScholarlyVerification::Pending),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(stories)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    // Note: These tests would require a test database setup
    // They are provided as examples of how to test the repository

    async fn setup_test_db() -> PgPool {
        // This would set up a test database
        // For now, we'll just return a placeholder
        todo!("Set up test database")
    }

    #[tokio::test]
    async fn test_create_and_get_story() {
        let pool = setup_test_db().await;
        let repo = StoryRepository::new(pool);

        let story = Story::new(
            "Test Story".to_string(),
            "قصة تجريبية".to_string(),
            "This is a test story content".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::Children,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );

        let created_story = repo.create_story(&story).await.unwrap();
        assert_eq!(created_story.title, story.title);

        let retrieved_story = repo.get_story_by_id(created_story.id).await.unwrap();
        assert!(retrieved_story.is_some());
        assert_eq!(retrieved_story.unwrap().title, story.title);
    }

    #[tokio::test]
    async fn test_search_stories() {
        let pool = setup_test_db().await;
        let repo = StoryRepository::new(pool);

        let results = repo.search_stories(
            "test",
            Some(&[StoryCategory::MoralLessons]),
            Some(&[AgeGroup::Children]),
            None,
            10,
            0,
        ).await.unwrap();

        // Test would verify search functionality
        assert!(results.len() <= 10);
    }

    #[tokio::test]
    async fn test_story_integrity_verification() {
        let pool = setup_test_db().await;
        let repo = StoryRepository::new(pool);

        let story = Story::new(
            "Integrity Test".to_string(),
            "اختبار التكامل".to_string(),
            "Content for integrity test".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::AllAges,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );

        let created_story = repo.create_story(&story).await.unwrap();
        let is_valid = repo.verify_story_integrity(created_story.id).await.unwrap();
        assert!(is_valid);
    }
}
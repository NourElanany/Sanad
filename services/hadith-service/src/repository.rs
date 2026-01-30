use crate::models::*;
use anyhow::Result;
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Repository for Hadith-related database operations
#[derive(Clone)]
pub struct HadithRepository {
    pool: PgPool,
}

impl HadithRepository {
    /// Create a new HadithRepository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get a Hadith by its ID
    pub async fn get_hadith(&self, hadith_id: Uuid) -> Result<Option<Hadith>> {
        let hadith = sqlx::query_as::<_, Hadith>(
            "SELECT id, hadith_number, text, text_hash, narrator, book, chapter, 
                    chapter_number, hadith_number_in_chapter, grade, source, language, 
                    word_count, themes, keywords, created_at, updated_at
             FROM hadiths WHERE id = $1"
        )
        .bind(hadith_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(hadith)
    }

    /// Get a Hadith by its number and book
    pub async fn get_hadith_by_number(&self, hadith_number: &str, book_name: &str) -> Result<Option<Hadith>> {
        let hadith = sqlx::query_as::<_, Hadith>(
            "SELECT id, hadith_number, text, text_hash, narrator, book, chapter, 
                    chapter_number, hadith_number_in_chapter, grade, source, language, 
                    word_count, themes, keywords, created_at, updated_at
             FROM hadiths WHERE hadith_number = $1 AND book = $2"
        )
        .bind(hadith_number)
        .bind(book_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(hadith)
    }

    /// Get Hadiths by book
    pub async fn get_hadiths_by_book(&self, book_name: &str, limit: i32, offset: i32) -> Result<Vec<Hadith>> {
        let hadiths = sqlx::query_as::<_, Hadith>(
            "SELECT id, hadith_number, text, text_hash, narrator, book, chapter, 
                    chapter_number, hadith_number_in_chapter, grade, source, language, 
                    word_count, themes, keywords, created_at, updated_at
             FROM hadiths WHERE book = $1 
             ORDER BY chapter_number, hadith_number_in_chapter, hadith_number
             LIMIT $2 OFFSET $3"
        )
        .bind(book_name)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(hadiths)
    }

    /// Get Hadiths by theme/topic
    pub async fn get_hadiths_by_theme(&self, theme: &str, limit: i32, offset: i32) -> Result<Vec<Hadith>> {
        let hadiths = sqlx::query_as::<_, Hadith>(
            "SELECT id, hadith_number, text, text_hash, narrator, book, chapter, 
                    chapter_number, hadith_number_in_chapter, grade, source, language, 
                    word_count, themes, keywords, created_at, updated_at
             FROM hadiths WHERE $1 = ANY(themes)
             ORDER BY grade, book, chapter_number, hadith_number_in_chapter
             LIMIT $2 OFFSET $3"
        )
        .bind(theme)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(hadiths)
    }

    /// Search Hadiths with advanced options
    pub async fn search_hadiths(
        &self,
        query: &str,
        search_type: SearchType,
        filters: Option<&HadithSearchFilters>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<HadithSearchResult>> {
        let mut bind_count = 1;
        let mut sql: String;

        match search_type {
            SearchType::Text => {
                sql = String::from(
                    "SELECT h.id, h.hadith_number, h.text, h.text_hash, h.narrator, h.book, h.chapter, 
                            h.chapter_number, h.hadith_number_in_chapter, h.grade, h.source, h.language, 
                            h.word_count, h.themes, h.keywords, h.created_at, h.updated_at,
                            hb.id as book_id, hb.name as book_name, hb.arabic_name as book_arabic_name, 
                            hb.author, hb.author_arabic_name, hb.description as book_description, 
                            hb.compilation_year, hb.total_hadiths, hb.book_type, hb.authenticity_level, 
                            hb.language as book_language, hb.created_at as book_created_at, hb.updated_at as book_updated_at,
                            hc.id as chapter_id, hc.book_id as chapter_book_id, hc.chapter_number as chap_num, 
                            hc.title as chapter_title, hc.arabic_title as chapter_arabic_title, 
                            hc.description as chapter_description, hc.hadith_count, hc.themes as chapter_themes, 
                            hc.created_at as chapter_created_at,
                            ts_rank(to_tsvector('arabic', h.text), plainto_tsquery('arabic', $1)) as relevance_score
                     FROM hadiths h
                     LEFT JOIN hadith_books hb ON h.book = hb.name
                     LEFT JOIN hadith_chapters hc ON hb.id = hc.book_id AND h.chapter_number = hc.chapter_number
                     WHERE to_tsvector('arabic', h.text) @@ plainto_tsquery('arabic', $1)"
                );
            }
            SearchType::Semantic => {
                // For semantic search, we'll use text search for now
                // In production, this would use vector embeddings
                sql = String::from(
                    "SELECT h.id, h.hadith_number, h.text, h.text_hash, h.narrator, h.book, h.chapter, 
                            h.chapter_number, h.hadith_number_in_chapter, h.grade, h.source, h.language, 
                            h.word_count, h.themes, h.keywords, h.created_at, h.updated_at,
                            hb.id as book_id, hb.name as book_name, hb.arabic_name as book_arabic_name, 
                            hb.author, hb.author_arabic_name, hb.description as book_description, 
                            hb.compilation_year, hb.total_hadiths, hb.book_type, hb.authenticity_level, 
                            hb.language as book_language, hb.created_at as book_created_at, hb.updated_at as book_updated_at,
                            hc.id as chapter_id, hc.book_id as chapter_book_id, hc.chapter_number as chap_num, 
                            hc.title as chapter_title, hc.arabic_title as chapter_arabic_title, 
                            hc.description as chapter_description, hc.hadith_count, hc.themes as chapter_themes, 
                            hc.created_at as chapter_created_at,
                            ts_rank(to_tsvector('arabic', h.text), plainto_tsquery('arabic', $1)) as relevance_score
                     FROM hadiths h
                     LEFT JOIN hadith_books hb ON h.book = hb.name
                     LEFT JOIN hadith_chapters hc ON hb.id = hc.book_id AND h.chapter_number = hc.chapter_number
                     WHERE to_tsvector('arabic', h.text) @@ plainto_tsquery('arabic', $1)"
                );
            }
            SearchType::Narrator => {
                sql = String::from(
                    "SELECT h.id, h.hadith_number, h.text, h.text_hash, h.narrator, h.book, h.chapter, 
                            h.chapter_number, h.hadith_number_in_chapter, h.grade, h.source, h.language, 
                            h.word_count, h.themes, h.keywords, h.created_at, h.updated_at,
                            hb.id as book_id, hb.name as book_name, hb.arabic_name as book_arabic_name, 
                            hb.author, hb.author_arabic_name, hb.description as book_description, 
                            hb.compilation_year, hb.total_hadiths, hb.book_type, hb.authenticity_level, 
                            hb.language as book_language, hb.created_at as book_created_at, hb.updated_at as book_updated_at,
                            hc.id as chapter_id, hc.book_id as chapter_book_id, hc.chapter_number as chap_num, 
                            hc.title as chapter_title, hc.arabic_title as chapter_arabic_title, 
                            hc.description as chapter_description, hc.hadith_count, hc.themes as chapter_themes, 
                            hc.created_at as chapter_created_at,
                            1.0 as relevance_score
                     FROM hadiths h
                     LEFT JOIN hadith_books hb ON h.book = hb.name
                     LEFT JOIN hadith_chapters hc ON hb.id = hc.book_id AND h.chapter_number = hc.chapter_number
                     WHERE h.narrator ILIKE '%' || $1 || '%'"
                );
            }
            SearchType::Theme => {
                sql = String::from(
                    "SELECT h.id, h.hadith_number, h.text, h.text_hash, h.narrator, h.book, h.chapter, 
                            h.chapter_number, h.hadith_number_in_chapter, h.grade, h.source, h.language, 
                            h.word_count, h.themes, h.keywords, h.created_at, h.updated_at,
                            hb.id as book_id, hb.name as book_name, hb.arabic_name as book_arabic_name, 
                            hb.author, hb.author_arabic_name, hb.description as book_description, 
                            hb.compilation_year, hb.total_hadiths, hb.book_type, hb.authenticity_level, 
                            hb.language as book_language, hb.created_at as book_created_at, hb.updated_at as book_updated_at,
                            hc.id as chapter_id, hc.book_id as chapter_book_id, hc.chapter_number as chap_num, 
                            hc.title as chapter_title, hc.arabic_title as chapter_arabic_title, 
                            hc.description as chapter_description, hc.hadith_count, hc.themes as chapter_themes, 
                            hc.created_at as chapter_created_at,
                            1.0 as relevance_score
                     FROM hadiths h
                     LEFT JOIN hadith_books hb ON h.book = hb.name
                     LEFT JOIN hadith_chapters hc ON hb.id = hc.book_id AND h.chapter_number = hc.chapter_number
                     WHERE $1 = ANY(h.themes)"
                );
            }
            SearchType::Exact => {
                sql = String::from(
                    "SELECT h.id, h.hadith_number, h.text, h.text_hash, h.narrator, h.book, h.chapter, 
                            h.chapter_number, h.hadith_number_in_chapter, h.grade, h.source, h.language, 
                            h.word_count, h.themes, h.keywords, h.created_at, h.updated_at,
                            hb.id as book_id, hb.name as book_name, hb.arabic_name as book_arabic_name, 
                            hb.author, hb.author_arabic_name, hb.description as book_description, 
                            hb.compilation_year, hb.total_hadiths, hb.book_type, hb.authenticity_level, 
                            hb.language as book_language, hb.created_at as book_created_at, hb.updated_at as book_updated_at,
                            hc.id as chapter_id, hc.book_id as chapter_book_id, hc.chapter_number as chap_num, 
                            hc.title as chapter_title, hc.arabic_title as chapter_arabic_title, 
                            hc.description as chapter_description, hc.hadith_count, hc.themes as chapter_themes, 
                            hc.created_at as chapter_created_at,
                            1.0 as relevance_score
                     FROM hadiths h
                     LEFT JOIN hadith_books hb ON h.book = hb.name
                     LEFT JOIN hadith_chapters hc ON hb.id = hc.book_id AND h.chapter_number = hc.chapter_number
                     WHERE h.text LIKE $1"
                );
            }
        }

        // Apply filters
        if let Some(filters) = filters {
            if let Some(books) = &filters.books {
                if !books.is_empty() {
                    bind_count += 1;
                    sql.push_str(&format!(" AND h.book = ANY(${})", bind_count));
                }
            }

            if let Some(grades) = &filters.grades {
                if !grades.is_empty() {
                    bind_count += 1;
                    sql.push_str(&format!(" AND h.grade = ANY(${})", bind_count));
                }
            }

            if let Some(themes) = &filters.themes {
                if !themes.is_empty() {
                    bind_count += 1;
                    sql.push_str(&format!(" AND h.themes && ${}", bind_count));
                }
            }
        }

        sql.push_str(" ORDER BY relevance_score DESC, h.grade, h.book, h.chapter_number");
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", bind_count + 1, bind_count + 2));

        let query_param = match search_type {
            SearchType::Exact => format!("%{}%", query),
            _ => query.to_string(),
        };

        let mut query_obj = sqlx::query(&sql).bind(query_param);

        // Bind filter parameters
        if let Some(filters) = filters {
            if let Some(books) = &filters.books {
                if !books.is_empty() {
                    query_obj = query_obj.bind(books);
                }
            }

            if let Some(grades) = &filters.grades {
                if !grades.is_empty() {
                    // Convert enum values to strings for database binding
                    let grade_strings: Vec<String> = grades.iter()
                        .map(|g| match g {
                            HadithGrade::Sahih => "sahih".to_string(),
                            HadithGrade::Hasan => "hasan".to_string(),
                            HadithGrade::Daif => "daif".to_string(),
                            HadithGrade::Mawdu => "mawdu".to_string(),
                        })
                        .collect();
                    query_obj = query_obj.bind(grade_strings);
                }
            }

            if let Some(themes) = &filters.themes {
                if !themes.is_empty() {
                    query_obj = query_obj.bind(themes);
                }
            }
        }

        query_obj = query_obj.bind(limit).bind(offset);

        let rows = query_obj.fetch_all(&self.pool).await?;

        let mut results = Vec::new();
        for row in rows {
            let hadith = Hadith {
                id: row.get("id"),
                hadith_number: row.get("hadith_number"),
                text: row.get("text"),
                text_hash: row.get("text_hash"),
                narrator: row.get("narrator"),
                book: row.get("book"),
                chapter: row.get("chapter"),
                chapter_number: row.get("chapter_number"),
                hadith_number_in_chapter: row.get("hadith_number_in_chapter"),
                grade: row.get("grade"),
                source: row.get("source"),
                language: row.get("language"),
                word_count: row.get("word_count"),
                themes: row.get("themes"),
                keywords: row.get("keywords"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };

            let book = if let Ok(book_id) = row.try_get::<Option<Uuid>, _>("book_id") {
                book_id.map(|_| HadithBook {
                    id: row.get("book_id"),
                    name: row.get("book_name"),
                    arabic_name: row.get("book_arabic_name"),
                    author: row.get("author"),
                    author_arabic_name: row.get("author_arabic_name"),
                    description: row.get("book_description"),
                    compilation_year: row.get("compilation_year"),
                    total_hadiths: row.get("total_hadiths"),
                    book_type: row.get("book_type"),
                    authenticity_level: row.get("authenticity_level"),
                    language: row.get("book_language"),
                    created_at: row.get("book_created_at"),
                    updated_at: row.get("book_updated_at"),
                })
            } else {
                None
            };

            let chapter = if let Ok(chapter_id) = row.try_get::<Option<Uuid>, _>("chapter_id") {
                chapter_id.map(|_| HadithChapter {
                    id: row.get("chapter_id"),
                    book_id: row.get("chapter_book_id"),
                    chapter_number: row.get("chap_num"),
                    title: row.get("chapter_title"),
                    arabic_title: row.get("chapter_arabic_title"),
                    description: row.get("chapter_description"),
                    hadith_count: row.get("hadith_count"),
                    themes: row.get("chapter_themes"),
                    created_at: row.get("chapter_created_at"),
                })
            } else {
                None
            };

            let relevance_score: f64 = row.get("relevance_score");
            let highlighted_text = highlight_search_term(&hadith.text, query);

            // Determine matching criteria based on search type
            let matching_criteria = match search_type {
                SearchType::Text | SearchType::Semantic => vec!["text_content".to_string()],
                SearchType::Narrator => vec!["narrator".to_string()],
                SearchType::Theme => vec!["themes".to_string()],
                SearchType::Exact => vec!["exact_match".to_string()],
            };

            results.push(HadithSearchResult {
                hadith: hadith.clone(),
                book: book.unwrap_or_else(|| create_default_book(&hadith.book)),
                chapter,
                relevance_score,
                highlighted_text,
                matching_criteria,
            });
        }

        Ok(results)
    }

    /// Count search results
    pub async fn count_search_results(
        &self,
        query: &str,
        search_type: SearchType,
        filters: Option<&HadithSearchFilters>,
    ) -> Result<i64> {
        let mut sql = match search_type {
            SearchType::Text | SearchType::Semantic => String::from(
                "SELECT COUNT(*) as count FROM hadiths h WHERE to_tsvector('arabic', h.text) @@ plainto_tsquery('arabic', $1)"
            ),
            SearchType::Narrator => String::from(
                "SELECT COUNT(*) as count FROM hadiths h WHERE h.narrator ILIKE '%' || $1 || '%'"
            ),
            SearchType::Theme => String::from(
                "SELECT COUNT(*) as count FROM hadiths h WHERE $1 = ANY(h.themes)"
            ),
            SearchType::Exact => String::from(
                "SELECT COUNT(*) as count FROM hadiths h WHERE h.text LIKE $1"
            ),
        };

        let query_param = match search_type {
            SearchType::Exact => format!("%{}%", query),
            _ => query.to_string(),
        };

        let mut query_obj = sqlx::query(&sql).bind(&query_param);
        let mut bind_count = 1;

        // Apply filters
        if let Some(filters) = filters {
            if let Some(books) = &filters.books {
                if !books.is_empty() {
                    bind_count += 1;
                    sql.push_str(&format!(" AND h.book = ANY(${})", bind_count));
                    query_obj = sqlx::query(&sql).bind(&query_param).bind(books);
                }
            }

            if let Some(grades) = &filters.grades {
                if !grades.is_empty() {
                    bind_count += 1;
                    sql.push_str(&format!(" AND h.grade = ANY(${})", bind_count));
                    let grade_strings: Vec<String> = grades.iter()
                        .map(|g| match g {
                            HadithGrade::Sahih => "sahih".to_string(),
                            HadithGrade::Hasan => "hasan".to_string(),
                            HadithGrade::Daif => "daif".to_string(),
                            HadithGrade::Mawdu => "mawdu".to_string(),
                        })
                        .collect();
                    query_obj = sqlx::query(&sql).bind(&query_param).bind(grade_strings);
                }
            }

            if let Some(themes) = &filters.themes {
                if !themes.is_empty() {
                    bind_count += 1;
                    sql.push_str(&format!(" AND h.themes && ${}", bind_count));
                    query_obj = sqlx::query(&sql).bind(&query_param).bind(themes);
                }
            }
        }

        let row = query_obj.fetch_one(&self.pool).await?;
        Ok(row.get("count"))
    }

    /// Get Sanad (chain of narration) for a Hadith
    pub async fn get_sanad(&self, hadith_id: Uuid) -> Result<Option<Sanad>> {
        let sanad = sqlx::query_as::<_, Sanad>(
            "SELECT id, hadith_id, chain_text, chain_hash, narrators, chain_grade, 
                    chain_analysis, created_at, updated_at
             FROM sanad WHERE hadith_id = $1"
        )
        .bind(hadith_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(sanad)
    }

    /// Get explanations for a Hadith
    pub async fn get_hadith_explanations(&self, hadith_id: Uuid) -> Result<Vec<HadithExplanationWithScholar>> {
        let rows = sqlx::query(
            "SELECT he.id, he.hadith_id, he.scholar_id, he.explanation_text, he.explanation_hash, 
                    he.word_count, he.key_points, he.related_verses, he.related_hadiths, 
                    he.language, he.created_at, he.updated_at,
                    s.id as scholar_id, s.name, s.arabic_name, s.birth_year, s.death_year, 
                    s.biography, s.specialization, s.credibility_score, s.scholarly_authentication, 
                    s.school_of_thought, s.major_works, s.created_at as scholar_created_at, 
                    s.updated_at as scholar_updated_at
             FROM hadith_explanations he
             JOIN scholars s ON he.scholar_id = s.id
             WHERE he.hadith_id = $1
             ORDER BY s.credibility_score DESC, s.name"
        )
        .bind(hadith_id)
        .fetch_all(&self.pool)
        .await?;

        let mut results = Vec::new();
        for row in rows {
            let explanation = HadithExplanation {
                id: row.get("id"),
                hadith_id: row.get("hadith_id"),
                scholar_id: row.get("scholar_id"),
                explanation_text: row.get("explanation_text"),
                explanation_hash: row.get("explanation_hash"),
                word_count: row.get("word_count"),
                key_points: row.get("key_points"),
                related_verses: row.get("related_verses"),
                related_hadiths: row.get("related_hadiths"),
                language: row.get("language"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };

            let scholar = Scholar {
                id: row.get("scholar_id"),
                name: row.get("name"),
                arabic_name: row.get("arabic_name"),
                birth_year: row.get("birth_year"),
                death_year: row.get("death_year"),
                biography: row.get("biography"),
                specialization: row.get("specialization"),
                credibility_score: row.get("credibility_score"),
                scholarly_authentication: row.get("scholarly_authentication"),
                school_of_thought: row.get("school_of_thought"),
                major_works: row.get("major_works"),
                created_at: row.get("scholar_created_at"),
                updated_at: row.get("scholar_updated_at"),
            };

            results.push(HadithExplanationWithScholar {
                explanation,
                scholar,
            });
        }

        Ok(results)
    }

    /// Get all Hadith books
    pub async fn get_hadith_books(&self) -> Result<Vec<HadithBook>> {
        let books = sqlx::query_as::<_, HadithBook>(
            "SELECT id, name, arabic_name, author, author_arabic_name, description, 
                    compilation_year, total_hadiths, book_type, authenticity_level, 
                    language, created_at, updated_at
             FROM hadith_books 
             ORDER BY authenticity_level, name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(books)
    }

    /// Get chapters for a specific book
    pub async fn get_book_chapters(&self, book_id: Uuid) -> Result<Vec<HadithChapter>> {
        let chapters = sqlx::query_as::<_, HadithChapter>(
            "SELECT id, book_id, chapter_number, title, arabic_title, description, 
                    hadith_count, themes, created_at
             FROM hadith_chapters 
             WHERE book_id = $1
             ORDER BY chapter_number"
        )
        .bind(book_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(chapters)
    }

    /// Get search suggestions
    pub async fn get_search_suggestions(&self, partial_query: &str, limit: i32) -> Result<Vec<String>> {
        let suggestions = sqlx::query(
            "SELECT DISTINCT unnest(keywords) as suggestion
             FROM hadiths 
             WHERE unnest(keywords) ILIKE $1
             ORDER BY suggestion
             LIMIT $2"
        )
        .bind(format!("{}%", partial_query))
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(suggestions.into_iter().map(|row| row.get("suggestion")).collect())
    }

    /// Get search facets for filtering
    pub async fn get_search_facets(&self, query: &str, search_type: SearchType) -> Result<SearchFacets> {
        // Get book facets
        let book_facets = match search_type {
            SearchType::Text | SearchType::Semantic => {
                sqlx::query(
                    "SELECT h.book as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE to_tsvector('arabic', h.text) @@ plainto_tsquery('arabic', $1)
                     GROUP BY h.book
                     ORDER BY count DESC
                     LIMIT 20"
                )
                .bind(query)
                .fetch_all(&self.pool)
                .await?
            }
            SearchType::Narrator => {
                sqlx::query(
                    "SELECT h.book as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE h.narrator ILIKE '%' || $1 || '%'
                     GROUP BY h.book
                     ORDER BY count DESC
                     LIMIT 20"
                )
                .bind(query)
                .fetch_all(&self.pool)
                .await?
            }
            SearchType::Theme => {
                sqlx::query(
                    "SELECT h.book as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE $1 = ANY(h.themes)
                     GROUP BY h.book
                     ORDER BY count DESC
                     LIMIT 20"
                )
                .bind(query)
                .fetch_all(&self.pool)
                .await?
            }
            SearchType::Exact => {
                sqlx::query(
                    "SELECT h.book as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE h.text LIKE $1
                     GROUP BY h.book
                     ORDER BY count DESC
                     LIMIT 20"
                )
                .bind(format!("%{}%", query))
                .fetch_all(&self.pool)
                .await?
            }
        };

        let books: Vec<FacetCount> = book_facets.iter().map(|row| FacetCount {
            value: row.get::<String, _>("value"),
            count: row.get("count"),
        }).collect();

        // Get grade facets
        let grade_facets = match search_type {
            SearchType::Text | SearchType::Semantic => {
                sqlx::query(
                    "SELECT h.grade as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE to_tsvector('arabic', h.text) @@ plainto_tsquery('arabic', $1)
                     GROUP BY h.grade
                     ORDER BY count DESC"
                )
                .bind(query)
                .fetch_all(&self.pool)
                .await?
            }
            SearchType::Narrator => {
                sqlx::query(
                    "SELECT h.grade as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE h.narrator ILIKE '%' || $1 || '%'
                     GROUP BY h.grade
                     ORDER BY count DESC"
                )
                .bind(query)
                .fetch_all(&self.pool)
                .await?
            }
            SearchType::Theme => {
                sqlx::query(
                    "SELECT h.grade as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE $1 = ANY(h.themes)
                     GROUP BY h.grade
                     ORDER BY count DESC"
                )
                .bind(query)
                .fetch_all(&self.pool)
                .await?
            }
            SearchType::Exact => {
                sqlx::query(
                    "SELECT h.grade as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE h.text LIKE $1
                     GROUP BY h.grade
                     ORDER BY count DESC"
                )
                .bind(format!("%{}%", query))
                .fetch_all(&self.pool)
                .await?
            }
        };

        let grades: Vec<FacetCount> = grade_facets.iter().map(|row| FacetCount {
            value: row.get::<String, _>("value"),
            count: row.get("count"),
        }).collect();

        // Get theme facets
        let theme_facets = match search_type {
            SearchType::Text | SearchType::Semantic => {
                sqlx::query(
                    "SELECT unnest(h.themes) as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE to_tsvector('arabic', h.text) @@ plainto_tsquery('arabic', $1)
                     GROUP BY unnest(h.themes)
                     ORDER BY count DESC
                     LIMIT 20"
                )
                .bind(query)
                .fetch_all(&self.pool)
                .await?
            }
            SearchType::Narrator => {
                sqlx::query(
                    "SELECT unnest(h.themes) as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE h.narrator ILIKE '%' || $1 || '%'
                     GROUP BY unnest(h.themes)
                     ORDER BY count DESC
                     LIMIT 20"
                )
                .bind(query)
                .fetch_all(&self.pool)
                .await?
            }
            SearchType::Theme => {
                sqlx::query(
                    "SELECT unnest(h.themes) as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE $1 = ANY(h.themes)
                     GROUP BY unnest(h.themes)
                     ORDER BY count DESC
                     LIMIT 20"
                )
                .bind(query)
                .fetch_all(&self.pool)
                .await?
            }
            SearchType::Exact => {
                sqlx::query(
                    "SELECT unnest(h.themes) as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE h.text LIKE $1
                     GROUP BY unnest(h.themes)
                     ORDER BY count DESC
                     LIMIT 20"
                )
                .bind(format!("%{}%", query))
                .fetch_all(&self.pool)
                .await?
            }
        };

        let themes: Vec<FacetCount> = theme_facets.iter().map(|row| FacetCount {
            value: row.get::<String, _>("value"),
            count: row.get("count"),
        }).collect();

        // Get narrator facets
        let narrator_facets = match search_type {
            SearchType::Text | SearchType::Semantic => {
                sqlx::query(
                    "SELECT h.narrator as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE to_tsvector('arabic', h.text) @@ plainto_tsquery('arabic', $1)
                     GROUP BY h.narrator
                     ORDER BY count DESC
                     LIMIT 20"
                )
                .bind(query)
                .fetch_all(&self.pool)
                .await?
            }
            SearchType::Narrator => {
                sqlx::query(
                    "SELECT h.narrator as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE h.narrator ILIKE '%' || $1 || '%'
                     GROUP BY h.narrator
                     ORDER BY count DESC
                     LIMIT 20"
                )
                .bind(query)
                .fetch_all(&self.pool)
                .await?
            }
            SearchType::Theme => {
                sqlx::query(
                    "SELECT h.narrator as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE $1 = ANY(h.themes)
                     GROUP BY h.narrator
                     ORDER BY count DESC
                     LIMIT 20"
                )
                .bind(query)
                .fetch_all(&self.pool)
                .await?
            }
            SearchType::Exact => {
                sqlx::query(
                    "SELECT h.narrator as value, COUNT(*) as count
                     FROM hadiths h
                     WHERE h.text LIKE $1
                     GROUP BY h.narrator
                     ORDER BY count DESC
                     LIMIT 20"
                )
                .bind(format!("%{}%", query))
                .fetch_all(&self.pool)
                .await?
            }
        };

        let narrators: Vec<FacetCount> = narrator_facets.iter().map(|row| FacetCount {
            value: row.get::<String, _>("value"),
            count: row.get("count"),
        }).collect();

        Ok(SearchFacets {
            books,
            grades,
            themes,
            narrators,
        })
    }

    /// Verify Hadith integrity
    pub async fn verify_hadith_integrity(&self) -> Result<Vec<(Uuid, bool)>> {
        let rows = sqlx::query(
            "SELECT id, text, text_hash FROM hadiths"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut results = Vec::new();
        for row in rows {
            let id: Uuid = row.get("id");
            let text: String = row.get("text");
            let stored_hash: String = row.get("text_hash");
            
            let calculated_hash = Hadith::generate_hash(&text);
            let is_valid = calculated_hash == stored_hash;
            
            results.push((id, is_valid));
        }

        Ok(results)
    }

    /// Get Hadith analytics
    pub async fn get_hadith_analytics(&self, request: &HadithAnalyticsRequest) -> Result<serde_json::Value> {
        match request.analysis_type {
            AnalysisType::GradeDistribution => {
                let grade_data = sqlx::query(
                    "SELECT grade, COUNT(*) as count
                     FROM hadiths
                     GROUP BY grade
                     ORDER BY count DESC"
                )
                .fetch_all(&self.pool)
                .await?;

                let mut distribution = serde_json::Map::new();
                for row in grade_data {
                    let grade: String = row.get("grade");
                    let count: i64 = row.get("count");
                    distribution.insert(grade, serde_json::json!(count));
                }

                Ok(serde_json::json!({
                    "grade_distribution": distribution
                }))
            }
            AnalysisType::ThemeAnalysis => {
                let theme_data = sqlx::query(
                    "SELECT unnest(themes) as theme, COUNT(*) as count
                     FROM hadiths
                     GROUP BY unnest(themes)
                     ORDER BY count DESC
                     LIMIT 50"
                )
                .fetch_all(&self.pool)
                .await?;

                let mut themes = serde_json::Map::new();
                for row in theme_data {
                    let theme: String = row.get("theme");
                    let count: i64 = row.get("count");
                    themes.insert(theme, serde_json::json!(count));
                }

                Ok(serde_json::json!({
                    "theme_analysis": themes
                }))
            }
            AnalysisType::NarratorFrequency => {
                let narrator_data = sqlx::query(
                    "SELECT narrator, COUNT(*) as count
                     FROM hadiths
                     GROUP BY narrator
                     ORDER BY count DESC
                     LIMIT 50"
                )
                .fetch_all(&self.pool)
                .await?;

                let mut narrators = serde_json::Map::new();
                for row in narrator_data {
                    let narrator: String = row.get("narrator");
                    let count: i64 = row.get("count");
                    narrators.insert(narrator, serde_json::json!(count));
                }

                Ok(serde_json::json!({
                    "narrator_frequency": narrators
                }))
            }
            AnalysisType::BookStatistics => {
                let book_data = sqlx::query(
                    "SELECT h.book, COUNT(*) as hadith_count, 
                            COUNT(DISTINCT h.narrator) as narrator_count,
                            AVG(h.word_count) as avg_word_count
                     FROM hadiths h
                     GROUP BY h.book
                     ORDER BY hadith_count DESC"
                )
                .fetch_all(&self.pool)
                .await?;

                let mut books = serde_json::Map::new();
                for row in book_data {
                    let book: String = row.get("book");
                    let hadith_count: i64 = row.get("hadith_count");
                    let narrator_count: i64 = row.get("narrator_count");
                    let avg_word_count: Option<f64> = row.get("avg_word_count");
                    
                    books.insert(book, serde_json::json!({
                        "hadith_count": hadith_count,
                        "narrator_count": narrator_count,
                        "avg_word_count": avg_word_count.unwrap_or(0.0)
                    }));
                }

                Ok(serde_json::json!({
                    "book_statistics": books
                }))
            }
        }
    }

    /// Insert a new Hadith
    pub async fn insert_hadith(&self, hadith: &Hadith) -> Result<()> {
        sqlx::query(
            "INSERT INTO hadiths (id, hadith_number, text, text_hash, narrator, book, chapter, 
                                 chapter_number, hadith_number_in_chapter, grade, source, language, 
                                 word_count, themes, keywords, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)"
        )
        .bind(&hadith.id)
        .bind(&hadith.hadith_number)
        .bind(&hadith.text)
        .bind(&hadith.text_hash)
        .bind(&hadith.narrator)
        .bind(&hadith.book)
        .bind(&hadith.chapter)
        .bind(hadith.chapter_number)
        .bind(hadith.hadith_number_in_chapter)
        .bind(&hadith.grade)
        .bind(&hadith.source)
        .bind(&hadith.language)
        .bind(hadith.word_count)
        .bind(&hadith.themes)
        .bind(&hadith.keywords)
        .bind(&hadith.created_at)
        .bind(&hadith.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Insert a new Hadith book
    pub async fn insert_hadith_book(&self, book: &HadithBook) -> Result<()> {
        sqlx::query(
            "INSERT INTO hadith_books (id, name, arabic_name, author, author_arabic_name, description, 
                                      compilation_year, total_hadiths, book_type, authenticity_level, 
                                      language, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)"
        )
        .bind(&book.id)
        .bind(&book.name)
        .bind(&book.arabic_name)
        .bind(&book.author)
        .bind(&book.author_arabic_name)
        .bind(&book.description)
        .bind(book.compilation_year)
        .bind(book.total_hadiths)
        .bind(&book.book_type)
        .bind(&book.authenticity_level)
        .bind(&book.language)
        .bind(&book.created_at)
        .bind(&book.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

/// Helper function to highlight search terms in text
fn highlight_search_term(text: &str, search_term: &str) -> String {
    // Simple highlighting - in production this would be more sophisticated
    let highlighted = text.replace(search_term, &format!("<mark>{}</mark>", search_term));
    
    // If no direct match, try to highlight individual words
    if highlighted == text {
        let words: Vec<&str> = search_term.split_whitespace().collect();
        let mut result = text.to_string();
        
        for word in words {
            if word.len() > 2 { // Only highlight meaningful words
                result = result.replace(word, &format!("<mark>{}</mark>", word));
            }
        }
        
        result
    } else {
        highlighted
    }
}

/// Helper function to create a default book when book data is not available
fn create_default_book(book_name: &str) -> HadithBook {
    HadithBook::new(
        book_name.to_string(),
        book_name.to_string(),
        "Unknown".to_string(),
        "غير معروف".to_string(),
        HadithBookType::Jami,
        BookAuthenticityLevel::Variable,
        "ar".to_string(),
    )
}

/// Filters for Hadith search
#[derive(Debug, Clone)]
pub struct HadithSearchFilters {
    pub books: Option<Vec<String>>,
    pub grades: Option<Vec<HadithGrade>>,
    pub themes: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_search_term() {
        let text = "إنما الأعمال بالنيات";
        let search_term = "الأعمال";
        let highlighted = highlight_search_term(text, search_term);
        
        assert!(highlighted.contains("<mark>الأعمال</mark>"));
    }

    #[test]
    fn test_create_default_book() {
        let book = create_default_book("صحيح البخاري");
        
        assert_eq!(book.name, "صحيح البخاري");
        assert_eq!(book.arabic_name, "صحيح البخاري");
        assert_eq!(book.author, "Unknown");
        assert_eq!(book.language, "ar");
    }
}
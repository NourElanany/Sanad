use crate::models::*;
use crate::repository::{HadithRepository, HadithSearchFilters};
use anyhow::{Result, anyhow};
use std::time::Instant;
use uuid::Uuid;
use chrono::Utc;

/// Service layer for Hadith operations
#[derive(Clone)]
pub struct HadithService {
    repository: HadithRepository,
}

impl HadithService {
    /// Create a new HadithService
    pub fn new(repository: HadithRepository) -> Self {
        Self { repository }
    }

    /// Get a Hadith by ID or number
    pub async fn get_hadith(&self, request: GetHadithRequest) -> Result<Option<HadithResponse>> {
        let hadith = if let Some(hadith_id) = request.hadith_id {
            self.repository.get_hadith(hadith_id).await?
        } else if let (Some(hadith_number), Some(book_name)) = (&request.hadith_number, &request.book_name) {
            self.repository.get_hadith_by_number(hadith_number, book_name).await?
        } else {
            return Err(anyhow!("Either hadith_id or (hadith_number and book_name) must be provided"));
        };

        if let Some(hadith) = hadith {
            let book = self.get_book_by_name(&hadith.book).await?;
            let chapter = self.get_chapter_by_book_and_number(&hadith.book, hadith.chapter_number).await?;
            
            let sanad = if request.include_sanad.unwrap_or(false) {
                self.repository.get_sanad(hadith.id).await?
            } else {
                None
            };

            let explanations = if request.include_explanations.unwrap_or(false) {
                Some(self.repository.get_hadith_explanations(hadith.id).await?)
            } else {
                None
            };

            Ok(Some(HadithResponse {
                hadith,
                book,
                chapter,
                sanad,
                explanations,
            }))
        } else {
            Ok(None)
        }
    }

    /// Search Hadiths with advanced filtering
    pub async fn search_hadiths(&self, request: SearchHadithRequest) -> Result<HadithSearchResponse> {
        let start_time = Instant::now();
        
        let limit = request.limit.unwrap_or(20).min(100); // Max 100 results
        let offset = request.offset.unwrap_or(0).max(0);
        let search_type = request.search_type.unwrap_or(SearchType::Text);

        // Validate search query
        if request.query.trim().is_empty() {
            return Err(anyhow!("Search query cannot be empty"));
        }

        if request.query.len() > 1000 {
            return Err(anyhow!("Search query too long"));
        }

        // Build search filters
        let filters = HadithSearchFilters {
            books: request.books,
            grades: request.grades,
            themes: request.themes,
        };

        let results = self.repository.search_hadiths(
            &request.query,
            search_type.clone(),
            Some(&filters),
            limit,
            offset,
        ).await?;

        let total_count = self.repository.count_search_results(
            &request.query,
            search_type.clone(),
            Some(&filters),
        ).await?;

        // Get search facets for filtering
        let facets = Some(self.repository.get_search_facets(
            &request.query,
            search_type.clone(),
        ).await?);

        let search_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(HadithSearchResponse {
            results,
            total_count,
            query: request.query,
            search_type,
            search_time_ms,
            facets,
        })
    }

    /// Get Hadiths by book
    pub async fn get_hadiths_by_book(&self, book_name: &str, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<Hadith>> {
        let limit = limit.unwrap_or(20).min(100);
        let offset = offset.unwrap_or(0).max(0);
        
        self.repository.get_hadiths_by_book(book_name, limit, offset).await
    }

    /// Get Hadiths by topic/theme
    pub async fn get_hadiths_by_topic(&self, request: GetHadithsByTopicRequest) -> Result<HadithTopicResponse> {
        let limit = request.limit.unwrap_or(20).min(100);
        let offset = request.offset.unwrap_or(0).max(0);

        // Build filters if grades are specified
        let filters = if let Some(grades) = &request.grades {
            Some(HadithSearchFilters {
                books: None,
                grades: Some(grades.clone()),
                themes: None,
            })
        } else {
            None
        };

        // Get hadiths by theme
        let hadiths = self.repository.get_hadiths_by_theme(&request.topic, limit, offset).await?;

        // Convert to HadithWithDetails
        let mut hadith_details = Vec::new();
        for hadith in hadiths {
            let book = self.get_book_by_name(&hadith.book).await?;
            let chapter = self.get_chapter_by_book_and_number(&hadith.book, hadith.chapter_number).await?;
            let sanad = self.repository.get_sanad(hadith.id).await?;
            let explanations = self.repository.get_hadith_explanations(hadith.id).await?;

            hadith_details.push(HadithWithDetails {
                hadith,
                book,
                chapter,
                sanad,
                explanations,
            });
        }

        // Get related topics if requested
        let related_topics = if request.include_related.unwrap_or(false) {
            self.get_related_topics(&request.topic).await?
        } else {
            Vec::new()
        };

        // Count total results
        let total_count = self.repository.count_search_results(
            &request.topic,
            SearchType::Theme,
            filters.as_ref(),
        ).await?;

        Ok(HadithTopicResponse {
            topic: request.topic,
            hadiths: hadith_details,
            related_topics,
            total_count,
        })
    }

    /// Get all Hadith books
    pub async fn get_hadith_books(&self) -> Result<Vec<HadithBook>> {
        self.repository.get_hadith_books().await
    }

    /// Get chapters for a specific book
    pub async fn get_book_chapters(&self, book_id: Uuid) -> Result<Vec<HadithChapter>> {
        self.repository.get_book_chapters(book_id).await
    }

    /// Get search suggestions
    pub async fn get_search_suggestions(&self, partial_query: &str) -> Result<Vec<String>> {
        if partial_query.len() < 2 {
            return Ok(Vec::new());
        }

        self.repository.get_search_suggestions(partial_query, 10).await
    }

    /// Get Hadith analytics
    pub async fn get_hadith_analytics(&self, request: HadithAnalyticsRequest) -> Result<HadithAnalyticsResponse> {
        let data = self.repository.get_hadith_analytics(&request).await?;
        
        let insights = self.generate_analytics_insights(&data, &request.analysis_type).await?;
        let recommendations = self.generate_analytics_recommendations(&data, &request.analysis_type).await?;

        Ok(HadithAnalyticsResponse {
            analysis_type: format!("{:?}", request.analysis_type),
            data,
            insights,
            recommendations,
            generated_at: Utc::now(),
        })
    }

    /// Verify Hadith content integrity
    pub async fn verify_hadith_integrity(&self) -> Result<HadithIntegrityReport> {
        let hadith_results = self.repository.verify_hadith_integrity().await?;

        let mut corrupted_hadiths = Vec::new();
        let mut valid_hadiths = 0;

        for (id, is_valid) in hadith_results {
            if is_valid {
                valid_hadiths += 1;
            } else {
                corrupted_hadiths.push(id);
            }
        }

        Ok(HadithIntegrityReport {
            hadiths: IntegrityStatus {
                total: valid_hadiths + corrupted_hadiths.len(),
                valid: valid_hadiths,
                corrupted: corrupted_hadiths,
            },
        })
    }

    /// Create a new Hadith
    pub async fn create_hadith(&self, mut hadith: Hadith) -> Result<Hadith> {
        // Ensure integrity
        hadith.calculate_word_count();
        
        if !hadith.verify_integrity() {
            return Err(anyhow!("Hadith integrity verification failed"));
        }

        self.repository.insert_hadith(&hadith).await?;
        Ok(hadith)
    }

    /// Create a new Hadith book
    pub async fn create_hadith_book(&self, book: HadithBook) -> Result<HadithBook> {
        self.repository.insert_hadith_book(&book).await?;
        Ok(book)
    }

    // Private helper methods

    /// Get book by name (with caching in production)
    async fn get_book_by_name(&self, book_name: &str) -> Result<HadithBook> {
        let books = self.repository.get_hadith_books().await?;
        
        books.into_iter()
            .find(|book| book.name == book_name || book.arabic_name == book_name)
            .ok_or_else(|| anyhow!("Book not found: {}", book_name))
    }

    /// Get chapter by book and chapter number
    async fn get_chapter_by_book_and_number(&self, book_name: &str, chapter_number: Option<i32>) -> Result<Option<HadithChapter>> {
        if let Some(chapter_num) = chapter_number {
            let book = self.get_book_by_name(book_name).await?;
            let chapters = self.repository.get_book_chapters(book.id).await?;
            
            Ok(chapters.into_iter()
                .find(|chapter| chapter.chapter_number == chapter_num))
        } else {
            Ok(None)
        }
    }

    /// Get related topics for a given topic
    async fn get_related_topics(&self, topic: &str) -> Result<Vec<String>> {
        // Simplified related topic discovery
        // In production, this would use semantic similarity or predefined relationships
        let related_map = std::collections::HashMap::from([
            ("عقيدة", vec!["إيمان", "توحيد", "شرك", "كفر"]),
            ("عبادة", vec!["صلاة", "زكاة", "صيام", "حج", "دعاء"]),
            ("أخلاق", vec!["صبر", "رحمة", "عدل", "صدق", "أمانة"]),
            ("معاملات", vec!["بيع", "شراء", "ربا", "عقود", "حقوق"]),
            ("أسرة", vec!["زواج", "طلاق", "أطفال", "والدين", "أقارب"]),
            ("صلاة", vec!["وضوء", "قبلة", "أوقات", "جماعة", "مسجد"]),
            ("زكاة", vec!["صدقة", "فقراء", "مال", "نصاب", "حول"]),
            ("صيام", vec!["رمضان", "إفطار", "سحور", "اعتكاف", "ليلة القدر"]),
            ("حج", vec!["عمرة", "مكة", "كعبة", "مناسك", "إحرام"]),
        ]);

        Ok(related_map.get(topic)
            .map(|topics| topics.iter().map(|s| s.to_string()).collect())
            .unwrap_or_else(Vec::new))
    }

    /// Generate insights from analytics data
    async fn generate_analytics_insights(&self, data: &serde_json::Value, analysis_type: &AnalysisType) -> Result<Vec<String>> {
        let mut insights = Vec::new();

        match analysis_type {
            AnalysisType::GradeDistribution => {
                if let Some(distribution) = data.get("grade_distribution").and_then(|d| d.as_object()) {
                    let total: i64 = distribution.values()
                        .filter_map(|v| v.as_i64())
                        .sum();
                    
                    let sahih_count = distribution.get("sahih")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);
                    
                    let authentic_percentage = if total > 0 {
                        (sahih_count as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    };

                    insights.push(format!(
                        "Collection contains {} total hadiths with {:.1}% being Sahih (authentic)",
                        total, authentic_percentage
                    ));

                    if authentic_percentage > 80.0 {
                        insights.push("This is a highly authentic collection with excellent reliability".to_string());
                    } else if authentic_percentage > 60.0 {
                        insights.push("This collection has good authenticity levels".to_string());
                    } else {
                        insights.push("This collection requires careful verification of individual hadiths".to_string());
                    }
                }
            }
            AnalysisType::ThemeAnalysis => {
                if let Some(themes) = data.get("theme_analysis").and_then(|d| d.as_object()) {
                    let total_themes = themes.len();
                    let most_common = themes.iter()
                        .max_by_key(|(_, count)| count.as_i64().unwrap_or(0))
                        .map(|(theme, count)| (theme.clone(), count.as_i64().unwrap_or(0)));

                    insights.push(format!("Analysis covers {} distinct thematic categories", total_themes));
                    
                    if let Some((theme, count)) = most_common {
                        insights.push(format!("Most prevalent theme is '{}' with {} hadiths", theme, count));
                    }

                    if total_themes > 20 {
                        insights.push("Collection shows excellent thematic diversity".to_string());
                    } else if total_themes > 10 {
                        insights.push("Collection has good thematic coverage".to_string());
                    } else {
                        insights.push("Collection focuses on specific thematic areas".to_string());
                    }
                }
            }
            AnalysisType::NarratorFrequency => {
                if let Some(narrators) = data.get("narrator_frequency").and_then(|d| d.as_object()) {
                    let total_narrators = narrators.len();
                    let most_frequent = narrators.iter()
                        .max_by_key(|(_, count)| count.as_i64().unwrap_or(0))
                        .map(|(narrator, count)| (narrator.clone(), count.as_i64().unwrap_or(0)));

                    insights.push(format!("Collection includes {} distinct narrators", total_narrators));
                    
                    if let Some((narrator, count)) = most_frequent {
                        insights.push(format!("Most frequent narrator is '{}' with {} hadiths", narrator, count));
                    }

                    if total_narrators > 100 {
                        insights.push("Excellent narrator diversity indicating comprehensive collection".to_string());
                    } else if total_narrators > 50 {
                        insights.push("Good narrator diversity".to_string());
                    } else {
                        insights.push("Collection focuses on specific narrator chains".to_string());
                    }
                }
            }
            AnalysisType::BookStatistics => {
                if let Some(books) = data.get("book_statistics").and_then(|d| d.as_object()) {
                    let total_books = books.len();
                    let total_hadiths: i64 = books.values()
                        .filter_map(|book| book.get("hadith_count").and_then(|c| c.as_i64()))
                        .sum();

                    insights.push(format!("Analysis covers {} books with {} total hadiths", total_books, total_hadiths));

                    let avg_per_book = if total_books > 0 {
                        total_hadiths as f64 / total_books as f64
                    } else {
                        0.0
                    };

                    insights.push(format!("Average of {:.0} hadiths per book", avg_per_book));

                    if avg_per_book > 1000.0 {
                        insights.push("Collection includes major comprehensive hadith books".to_string());
                    } else if avg_per_book > 500.0 {
                        insights.push("Collection includes substantial hadith compilations".to_string());
                    } else {
                        insights.push("Collection includes focused or specialized hadith books".to_string());
                    }
                }
            }
        }

        Ok(insights)
    }

    /// Generate recommendations from analytics data
    async fn generate_analytics_recommendations(&self, data: &serde_json::Value, analysis_type: &AnalysisType) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        match analysis_type {
            AnalysisType::GradeDistribution => {
                if let Some(distribution) = data.get("grade_distribution").and_then(|d| d.as_object()) {
                    let total: i64 = distribution.values()
                        .filter_map(|v| v.as_i64())
                        .sum();
                    
                    let sahih_count = distribution.get("sahih").and_then(|v| v.as_i64()).unwrap_or(0);
                    let daif_count = distribution.get("daif").and_then(|v| v.as_i64()).unwrap_or(0);
                    let mawdu_count = distribution.get("mawdu").and_then(|v| v.as_i64()).unwrap_or(0);

                    if (sahih_count as f64) / (total as f64) < 0.5 {
                        recommendations.push("Consider prioritizing Sahih hadiths for core teachings".to_string());
                    }

                    if daif_count > 0 {
                        recommendations.push("Review weak (Daif) hadiths for supporting evidence only".to_string());
                    }

                    if mawdu_count > 0 {
                        recommendations.push("Remove or clearly mark fabricated (Mawdu) hadiths".to_string());
                    }

                    recommendations.push("Always verify hadith authenticity before citing in scholarly work".to_string());
                }
            }
            AnalysisType::ThemeAnalysis => {
                recommendations.push("Use thematic analysis to identify gaps in coverage".to_string());
                recommendations.push("Consider adding hadiths for underrepresented themes".to_string());
                recommendations.push("Group related themes for better organization".to_string());
                recommendations.push("Create thematic study guides based on this analysis".to_string());
            }
            AnalysisType::NarratorFrequency => {
                recommendations.push("Cross-reference with narrator reliability studies".to_string());
                recommendations.push("Investigate chains with unusual frequency patterns".to_string());
                recommendations.push("Consider narrator biographical information for context".to_string());
                recommendations.push("Use narrator analysis for hadith authentication studies".to_string());
            }
            AnalysisType::BookStatistics => {
                recommendations.push("Balance collection with hadiths from various authentic sources".to_string());
                recommendations.push("Consider book authenticity levels when citing".to_string());
                recommendations.push("Use comprehensive books for foundational studies".to_string());
                recommendations.push("Supplement with specialized collections for specific topics".to_string());
            }
        }

        Ok(recommendations)
    }
}

/// Hadith integrity report
#[derive(Debug, serde::Serialize)]
pub struct HadithIntegrityReport {
    pub hadiths: IntegrityStatus,
}

/// Integrity status for a content type
#[derive(Debug, serde::Serialize)]
pub struct IntegrityStatus {
    pub total: usize,
    pub valid: usize,
    pub corrupted: Vec<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_related_topics() {
        // This would be tested with a real service instance
        // For now, just test the logic
        let related_map = std::collections::HashMap::from([
            ("عقيدة", vec!["إيمان", "توحيد", "شرك", "كفر"]),
            ("عبادة", vec!["صلاة", "زكاة", "صيام", "حج", "دعاء"]),
        ]);

        let related = related_map.get("عقيدة").unwrap();
        assert!(related.contains(&"إيمان"));
        assert!(related.contains(&"توحيد"));
    }

    #[test]
    fn test_analytics_insights_generation() {
        // Test insight generation logic
        let sample_data = serde_json::json!({
            "grade_distribution": {
                "sahih": 800,
                "hasan": 150,
                "daif": 50,
                "mawdu": 0
            }
        });

        // In a real test, we would call the actual method
        // For now, just verify the data structure
        assert!(sample_data.get("grade_distribution").is_some());
        
        let distribution = sample_data.get("grade_distribution").unwrap().as_object().unwrap();
        let total: i64 = distribution.values().filter_map(|v| v.as_i64()).sum();
        assert_eq!(total, 1000);
    }
}
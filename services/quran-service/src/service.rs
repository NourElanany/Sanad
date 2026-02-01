use crate::models::*;
use crate::repository::QuranRepository;
use anyhow::{Result, anyhow};
use std::time::Instant;
use uuid::Uuid;
use chrono::Utc;

/// Service layer for Quran operations
#[derive(Clone)]
pub struct QuranService {
    repository: QuranRepository,
}

impl QuranService {
    /// Create a new QuranService
    pub fn new(repository: QuranRepository) -> Self {
        Self { repository }
    }

    /// Get a Surah by number
    pub async fn get_surah(&self, request: GetSurahRequest) -> Result<Option<SurahResponse>> {
        let surah = self.repository.get_surah(request.surah_number).await?;
        
        if let Some(surah) = surah {
            let ayahs = if request.include_ayahs.unwrap_or(false) {
                Some(self.repository.get_ayahs_by_surah(request.surah_number).await?)
            } else {
                None
            };

            Ok(Some(SurahResponse { surah, ayahs }))
        } else {
            Ok(None)
        }
    }

    /// Get all Surahs
    pub async fn get_all_surahs(&self) -> Result<Vec<Surah>> {
        self.repository.get_all_surahs().await
    }

    /// Get a specific Ayah
    pub async fn get_ayah(&self, request: GetAyahRequest) -> Result<Option<AyahResponse>> {
        let ayah = self.repository.get_ayah(request.surah_number, request.ayah_number).await?;
        
        if let Some(ayah) = ayah {
            let surah = self.repository.get_surah(request.surah_number).await?
                .ok_or_else(|| anyhow!("Surah not found"))?;

            let tafsir_entries = if request.include_tafsir.unwrap_or(false) {
                Some(self.repository.get_tafsir(request.surah_number, request.ayah_number, None).await?)
            } else {
                None
            };

            Ok(Some(AyahResponse {
                ayah,
                surah,
                tafsir_entries,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get Surah with all its Ayahs
    pub async fn get_surah_with_ayahs(&self, surah_number: i32) -> Result<Option<SurahWithAyahs>> {
        self.repository.get_surah_with_ayahs(surah_number).await
    }

    /// Search in Quran
    pub async fn search_quran(&self, request: SearchQuranRequest) -> Result<SearchResponse> {
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

        // Clone the surah_numbers for the count query
        let surah_numbers_for_count = request.surah_numbers.clone();
        let search_type_for_count = search_type.clone();

        let results = self.repository.search_quran(
            &request.query,
            request.surah_numbers,
            search_type.clone(),
            limit,
            offset,
        ).await?;

        let total_count = self.repository.count_search_results(
            &request.query,
            surah_numbers_for_count,
            search_type_for_count,
        ).await?;

        // Get search suggestions for partial queries
        let suggestions = if request.query.len() < 50 {
            Some(self.repository.get_search_suggestions(&request.query, 5).await.unwrap_or_default())
        } else {
            None
        };

        let search_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(SearchResponse {
            results,
            total_count,
            query: request.query,
            search_type,
            search_time_ms,
            suggestions,
        })
    }

    /// Get Tafsir for an Ayah
    pub async fn get_tafsir(&self, request: GetTafsirRequest) -> Result<Option<TafsirResponse>> {
        let ayah = self.repository.get_ayah(request.surah_number, request.ayah_number).await?;
        
        if let Some(ayah) = ayah {
            let surah = self.repository.get_surah(request.surah_number).await?
                .ok_or_else(|| anyhow!("Surah not found"))?;

            let tafsir_entries = self.repository.get_tafsir(
                request.surah_number,
                request.ayah_number,
                request.source_ids,
            ).await?;

            Ok(Some(TafsirResponse {
                ayah,
                surah,
                tafsir_entries,
            }))
        } else {
            Ok(None)
        }
    }

    /// Compare Tafsir interpretations from different sources
    pub async fn compare_tafsir(&self, request: TafsirComparisonRequest) -> Result<Option<TafsirComparisonResponse>> {
        let ayah = self.repository.get_ayah(request.surah_number, request.ayah_number).await?;
        
        if let Some(ayah) = ayah {
            let surah = self.repository.get_surah(request.surah_number).await?
                .ok_or_else(|| anyhow!("Surah not found"))?;

            // Get Tafsir entries for the specified sources
            let tafsir_entries = self.repository.get_tafsir(
                request.surah_number,
                request.ayah_number,
                Some(request.source_ids.clone()),
            ).await?;

            if tafsir_entries.is_empty() {
                return Ok(None);
            }

            // Perform comparison analysis
            let comparisons = self.analyze_tafsir_comparisons(&tafsir_entries).await?;
            let summary = self.generate_comparison_summary(&tafsir_entries, &comparisons).await?;
            let recommendations = self.generate_reading_recommendations(&tafsir_entries).await?;

            Ok(Some(TafsirComparisonResponse {
                ayah,
                surah,
                comparisons,
                summary,
                recommendations,
            }))
        } else {
            Ok(None)
        }
    }

    /// Manage Tafsir sources (create, update, verify credibility)
    pub async fn manage_tafsir_source(&self, request: ManageTafsirSourceRequest) -> Result<serde_json::Value> {
        match request.action {
            SourceManagementAction::Create => {
                let source_data = request.source_data
                    .ok_or_else(|| anyhow!("Source data required for create action"))?;
                
                let source = TafsirSource::new(
                    source_data.name,
                    source_data.author,
                    source_data.language,
                    source_data.description,
                    source_data.source_type,
                    source_data.scholarly_authentication.unwrap_or(ScholarlyAuthentication::Unverified),
                );

                self.repository.insert_tafsir_source(&source).await?;
                Ok(serde_json::json!({
                    "action": "created",
                    "source_id": source.id,
                    "credibility_score": source.credibility_score
                }))
            }
            SourceManagementAction::Update => {
                let source_id = request.source_id
                    .ok_or_else(|| anyhow!("Source ID required for update action"))?;
                let source_data = request.source_data
                    .ok_or_else(|| anyhow!("Source data required for update action"))?;

                self.repository.update_tafsir_source(source_id, source_data).await?;
                Ok(serde_json::json!({
                    "action": "updated",
                    "source_id": source_id
                }))
            }
            SourceManagementAction::VerifyCredibility => {
                let source_id = request.source_id
                    .ok_or_else(|| anyhow!("Source ID required for credibility verification"))?;

                let verification_result = self.verify_source_credibility(source_id).await?;
                Ok(serde_json::to_value(verification_result)?)
            }
            SourceManagementAction::UpdateAuthentication => {
                let source_id = request.source_id
                    .ok_or_else(|| anyhow!("Source ID required for authentication update"))?;
                let source_data = request.source_data
                    .ok_or_else(|| anyhow!("Source data with authentication required"))?;

                if let Some(auth) = source_data.scholarly_authentication {
                    self.repository.update_source_authentication(source_id, auth).await?;
                    Ok(serde_json::json!({
                        "action": "authentication_updated",
                        "source_id": source_id
                    }))
                } else {
                    Err(anyhow!("Scholarly authentication required"))
                }
            }
            SourceManagementAction::Deactivate => {
                let source_id = request.source_id
                    .ok_or_else(|| anyhow!("Source ID required for deactivation"))?;

                self.repository.deactivate_tafsir_source(source_id).await?;
                Ok(serde_json::json!({
                    "action": "deactivated",
                    "source_id": source_id
                }))
            }
        }
    }

    /// Advanced search in Tafsir content
    pub async fn advanced_tafsir_search(&self, request: AdvancedTafsirSearchRequest) -> Result<AdvancedTafsirSearchResponse> {
        let start_time = Instant::now();
        
        let limit = request.limit.unwrap_or(20).min(100);
        let offset = request.offset.unwrap_or(0).max(0);

        let results = self.repository.advanced_tafsir_search(
            &request.query,
            &request.search_criteria,
            request.source_filters.as_ref(),
            limit,
            offset,
        ).await?;

        let total_count = self.repository.count_tafsir_search_results(
            &request.query,
            &request.search_criteria,
            request.source_filters.as_ref(),
        ).await?;

        let facets = self.repository.get_tafsir_search_facets(
            &request.query,
            &request.search_criteria,
        ).await?;

        let search_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(AdvancedTafsirSearchResponse {
            results,
            total_count,
            search_time_ms,
            facets,
        })
    }

    /// Get Tafsir analytics
    pub async fn get_tafsir_analytics(&self, request: TafsirAnalyticsRequest) -> Result<TafsirAnalyticsResponse> {
        let data = match request.analysis_type {
            AnalysisType::Coverage => {
                self.repository.analyze_tafsir_coverage(
                    request.surah_number,
                    request.ayah_range,
                    request.source_ids.as_ref(),
                ).await?
            }
            AnalysisType::Themes => {
                self.repository.analyze_tafsir_themes(
                    request.surah_number,
                    request.ayah_range,
                    request.source_ids.as_ref(),
                ).await?
            }
            AnalysisType::Methodology => {
                self.repository.analyze_tafsir_methodology(
                    request.source_ids.as_ref(),
                ).await?
            }
            AnalysisType::Consensus => {
                self.repository.analyze_scholarly_consensus(
                    request.surah_number,
                    request.ayah_range,
                    request.source_ids.as_ref(),
                ).await?
            }
        };

        let insights = self.generate_analytics_insights(&data, &request.analysis_type).await?;
        let recommendations = self.generate_analytics_recommendations(&data, &request.analysis_type).await?;

        Ok(TafsirAnalyticsResponse {
            analysis_type: format!("{:?}", request.analysis_type),
            data,
            insights,
            recommendations,
            generated_at: Utc::now(),
        })
    }

    // Private helper methods

    /// Analyze Tafsir comparisons
    async fn analyze_tafsir_comparisons(&self, tafsir_entries: &[TafsirWithSource]) -> Result<Vec<TafsirComparison>> {
        let mut comparisons = Vec::new();

        for entry in tafsir_entries {
            let key_points = self.extract_key_points(&entry.tafsir.text).await?;
            let unique_insights = self.identify_unique_insights(&entry.tafsir, tafsir_entries).await?;
            let methodology_notes = self.analyze_methodology(&entry.source).await?;

            comparisons.push(TafsirComparison {
                source: entry.source.clone(),
                tafsir: entry.tafsir.clone(),
                key_points,
                unique_insights,
                methodology_notes,
            });
        }

        Ok(comparisons)
    }

    /// Generate comparison summary
    async fn generate_comparison_summary(&self, tafsir_entries: &[TafsirWithSource], comparisons: &[TafsirComparison]) -> Result<ComparisonSummary> {
        let common_themes = self.identify_common_themes(tafsir_entries).await?;
        let divergent_views = self.identify_divergent_views(comparisons).await?;
        let scholarly_consensus = self.determine_scholarly_consensus(tafsir_entries).await?;
        let recommended_reading_order = self.recommend_reading_order(tafsir_entries).await?;

        Ok(ComparisonSummary {
            common_themes,
            divergent_views,
            scholarly_consensus,
            recommended_reading_order,
        })
    }

    /// Generate reading recommendations
    async fn generate_reading_recommendations(&self, tafsir_entries: &[TafsirWithSource]) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        // Sort by credibility score
        let mut sorted_entries = tafsir_entries.to_vec();
        sorted_entries.sort_by(|a, b| b.source.credibility_score.partial_cmp(&a.source.credibility_score).unwrap());

        if let Some(best_source) = sorted_entries.first() {
            recommendations.push(format!(
                "Start with '{}' by {} for the most credible interpretation (credibility: {:.1}/10)",
                best_source.source.name,
                best_source.source.author,
                best_source.source.credibility_score
            ));
        }

        // Recommend classical sources
        let classical_sources: Vec<_> = sorted_entries.iter()
            .filter(|entry| matches!(entry.source.source_type, TafsirSourceType::Classical))
            .collect();

        if !classical_sources.is_empty() {
            recommendations.push("Consider reading classical interpretations for traditional scholarly perspectives".to_string());
        }

        // Recommend linguistic sources for complex verses
        let linguistic_sources: Vec<_> = sorted_entries.iter()
            .filter(|entry| matches!(entry.source.source_type, TafsirSourceType::Linguistic))
            .collect();

        if !linguistic_sources.is_empty() {
            recommendations.push("Consult linguistic interpretations for detailed Arabic grammar and word analysis".to_string());
        }

        Ok(recommendations)
    }

    /// Verify source credibility
    async fn verify_source_credibility(&self, source_id: Uuid) -> Result<CredibilityVerificationResult> {
        let source = self.repository.get_tafsir_source_by_id(source_id).await?
            .ok_or_else(|| anyhow!("Tafsir source not found"))?;

        let previous_score = source.credibility_score;
        
        // Perform credibility verification (simplified implementation)
        let verification_factors = vec![
            VerificationFactor {
                factor_type: "Scholarly Authentication".to_string(),
                weight: 0.4,
                score: match source.scholarly_authentication {
                    ScholarlyAuthentication::HighlyAuthenticated => 10.0,
                    ScholarlyAuthentication::Authenticated => 8.0,
                    ScholarlyAuthentication::Verified => 6.0,
                    ScholarlyAuthentication::Unverified => 3.0,
                },
                description: "Level of scholarly authentication".to_string(),
            },
            VerificationFactor {
                factor_type: "Source Type".to_string(),
                weight: 0.3,
                score: match source.source_type {
                    TafsirSourceType::Classical => 9.0,
                    TafsirSourceType::Contemporary => 7.0,
                    TafsirSourceType::Linguistic => 8.0,
                    TafsirSourceType::Thematic => 7.0,
                    TafsirSourceType::Sectarian => 5.0,
                },
                description: "Type and approach of the source".to_string(),
            },
            VerificationFactor {
                factor_type: "Historical Acceptance".to_string(),
                weight: 0.3,
                score: if matches!(source.source_type, TafsirSourceType::Classical) { 9.0 } else { 6.0 },
                description: "Historical acceptance by scholars".to_string(),
            },
        ];

        let new_score = verification_factors.iter()
            .map(|f| f.score * f.weight)
            .sum::<f64>()
            .min(10.0);

        // Update the score in the database
        self.repository.update_source_credibility_score(source_id, new_score).await?;

        let recommendations = self.generate_credibility_recommendations(new_score, &source).await?;

        Ok(CredibilityVerificationResult {
            source_id,
            previous_score,
            new_score,
            verification_factors,
            recommendations,
            verified_at: Utc::now(),
        })
    }

    // Additional helper methods (simplified implementations)

    async fn extract_key_points(&self, text: &str) -> Result<Vec<String>> {
        // Simplified key point extraction
        // In production, this would use NLP techniques
        let sentences: Vec<&str> = text.split('.')
            .filter(|s| !s.trim().is_empty())
            .take(3) // Take first 3 sentences as key points
            .collect();
        
        Ok(sentences.iter().map(|s| s.trim().to_string()).collect())
    }

    async fn identify_unique_insights(&self, tafsir: &Tafsir, all_entries: &[TafsirWithSource]) -> Result<Vec<String>> {
        // Simplified unique insight identification
        let mut unique_insights = Vec::new();
        
        // Check for unique themes
        for theme in &tafsir.themes {
            let is_unique = all_entries.iter()
                .filter(|entry| entry.tafsir.id != tafsir.id)
                .all(|entry| !entry.tafsir.themes.contains(theme));
            
            if is_unique {
                unique_insights.push(format!("Unique focus on: {}", theme));
            }
        }

        Ok(unique_insights)
    }

    async fn analyze_methodology(&self, source: &TafsirSource) -> Result<Option<String>> {
        let methodology = match source.source_type {
            TafsirSourceType::Classical => Some("Traditional exegetical methodology with emphasis on transmitted knowledge".to_string()),
            TafsirSourceType::Contemporary => Some("Modern interpretive approach considering contemporary context".to_string()),
            TafsirSourceType::Linguistic => Some("Linguistic analysis focusing on Arabic grammar and etymology".to_string()),
            TafsirSourceType::Thematic => Some("Thematic approach organizing interpretation by topics".to_string()),
            TafsirSourceType::Sectarian => Some("Interpretation from specific school of thought perspective".to_string()),
        };

        Ok(methodology)
    }

    async fn identify_common_themes(&self, tafsir_entries: &[TafsirWithSource]) -> Result<Vec<String>> {
        let mut theme_counts = std::collections::HashMap::new();
        
        for entry in tafsir_entries {
            for theme in &entry.tafsir.themes {
                *theme_counts.entry(theme.clone()).or_insert(0) += 1;
            }
        }

        let common_themes: Vec<String> = theme_counts.into_iter()
            .filter(|(_, count)| *count >= 2) // Appears in at least 2 sources
            .map(|(theme, _)| theme)
            .collect();

        Ok(common_themes)
    }

    async fn identify_divergent_views(&self, comparisons: &[TafsirComparison]) -> Result<Vec<DivergentView>> {
        // Simplified divergent view identification
        // In production, this would be much more sophisticated
        let mut divergent_views = Vec::new();

        if comparisons.len() >= 2 {
            divergent_views.push(DivergentView {
                topic: "Interpretive Approach".to_string(),
                source_positions: comparisons.iter().map(|comp| SourcePosition {
                    source_id: comp.source.id,
                    source_name: comp.source.name.clone(),
                    position: format!("{:?} approach", comp.source.source_type),
                    evidence: comp.key_points.clone(),
                }).collect(),
                significance: ViewSignificance::Moderate,
            });
        }

        Ok(divergent_views)
    }

    async fn determine_scholarly_consensus(&self, tafsir_entries: &[TafsirWithSource]) -> Result<Option<String>> {
        let authenticated_sources = tafsir_entries.iter()
            .filter(|entry| entry.source.is_authenticated())
            .count();

        if authenticated_sources >= 2 {
            Ok(Some("Multiple authenticated sources provide consistent interpretation".to_string()))
        } else {
            Ok(None)
        }
    }

    async fn recommend_reading_order(&self, tafsir_entries: &[TafsirWithSource]) -> Result<Vec<Uuid>> {
        let mut sorted_entries = tafsir_entries.to_vec();
        
        // Sort by credibility score (highest first), then by source type preference
        sorted_entries.sort_by(|a, b| {
            let score_cmp = b.source.credibility_score.partial_cmp(&a.source.credibility_score).unwrap();
            if score_cmp == std::cmp::Ordering::Equal {
                // Prefer classical sources
                match (&a.source.source_type, &b.source.source_type) {
                    (TafsirSourceType::Classical, _) => std::cmp::Ordering::Less,
                    (_, TafsirSourceType::Classical) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                }
            } else {
                score_cmp
            }
        });

        Ok(sorted_entries.iter().map(|entry| entry.source.id).collect())
    }

    async fn generate_analytics_insights(&self, _data: &serde_json::Value, analysis_type: &AnalysisType) -> Result<Vec<String>> {
        // Simplified insight generation
        let mut insights = Vec::new();
        
        match analysis_type {
            AnalysisType::Coverage => {
                insights.push("Coverage analysis shows distribution of interpretations across verses".to_string());
            }
            AnalysisType::Themes => {
                insights.push("Thematic analysis reveals common interpretive themes".to_string());
            }
            AnalysisType::Methodology => {
                insights.push("Methodological analysis shows diversity in interpretive approaches".to_string());
            }
            AnalysisType::Consensus => {
                insights.push("Consensus analysis identifies areas of scholarly agreement".to_string());
            }
        }

        Ok(insights)
    }

    async fn generate_analytics_recommendations(&self, _data: &serde_json::Value, analysis_type: &AnalysisType) -> Result<Vec<String>> {
        // Simplified recommendation generation
        let mut recommendations = Vec::new();
        
        match analysis_type {
            AnalysisType::Coverage => {
                recommendations.push("Consider adding interpretations for verses with limited coverage".to_string());
            }
            AnalysisType::Themes => {
                recommendations.push("Explore underrepresented themes in current interpretations".to_string());
            }
            AnalysisType::Methodology => {
                recommendations.push("Balance different methodological approaches for comprehensive understanding".to_string());
            }
            AnalysisType::Consensus => {
                recommendations.push("Focus on areas of consensus for foundational understanding".to_string());
            }
        }

        Ok(recommendations)
    }

    async fn generate_credibility_recommendations(&self, score: f64, source: &TafsirSource) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        if score < 6.0 {
            recommendations.push("Consider additional scholarly verification".to_string());
            recommendations.push("Cross-reference with established classical sources".to_string());
        } else if score < 8.0 {
            recommendations.push("Good credibility - suitable for general reference".to_string());
        } else {
            recommendations.push("Excellent credibility - highly recommended for scholarly work".to_string());
        }

        if !source.is_authenticated() {
            recommendations.push("Consider upgrading authentication level with proper scholarly review".to_string());
        }

        Ok(recommendations)
    }

    /// Get all Tafsir sources
    pub async fn get_tafsir_sources(&self) -> Result<Vec<TafsirSource>> {
        self.repository.get_tafsir_sources().await
    }

    /// Verify content integrity
    pub async fn verify_content_integrity(&self) -> Result<ContentIntegrityReport> {
        let ayah_results = self.repository.verify_ayah_integrity().await?;
        let tafsir_results = self.repository.verify_tafsir_integrity().await?;

        let mut corrupted_ayahs = Vec::new();
        let mut valid_ayahs = 0;

        for (id, is_valid) in ayah_results {
            if is_valid {
                valid_ayahs += 1;
            } else {
                corrupted_ayahs.push(id);
            }
        }

        let mut corrupted_tafsir = Vec::new();
        let mut valid_tafsir = 0;

        for (id, is_valid) in tafsir_results {
            if is_valid {
                valid_tafsir += 1;
            } else {
                corrupted_tafsir.push(id);
            }
        }

        Ok(ContentIntegrityReport {
            ayahs: IntegrityStatus {
                total: valid_ayahs + corrupted_ayahs.len(),
                valid: valid_ayahs,
                corrupted: corrupted_ayahs,
            },
            tafsir: IntegrityStatus {
                total: valid_tafsir + corrupted_tafsir.len(),
                valid: valid_tafsir,
                corrupted: corrupted_tafsir,
            },
        })
    }

    /// Get Ayah range for Khatma planning
    pub async fn get_ayah_range(&self, start_surah: i32, start_ayah: i32, end_surah: i32, end_ayah: i32) -> Result<Vec<Ayah>> {
        // Validate input ranges
        if start_surah < 1 || start_surah > 114 || end_surah < 1 || end_surah > 114 {
            return Err(anyhow!("Invalid Surah number. Must be between 1 and 114"));
        }

        if start_ayah < 1 || end_ayah < 1 {
            return Err(anyhow!("Invalid Ayah number. Must be greater than 0"));
        }

        if start_surah > end_surah || (start_surah == end_surah && start_ayah > end_ayah) {
            return Err(anyhow!("Invalid range. Start position must be before end position"));
        }

        self.repository.get_ayah_range(start_surah, start_ayah, end_surah, end_ayah).await
    }

    /// Get navigation info for an Ayah (previous/next)
    pub async fn get_ayah_navigation(&self, surah_number: i32, ayah_number: i32) -> Result<Option<AyahNavigation>> {
        let current_ayah = self.repository.get_ayah(surah_number, ayah_number).await?;
        
        if current_ayah.is_none() {
            return Ok(None);
        }

        let current_surah = self.repository.get_surah(surah_number).await?
            .ok_or_else(|| anyhow!("Surah not found"))?;

        // Get previous Ayah
        let previous = if ayah_number > 1 {
            // Previous Ayah in same Surah
            Some(AyahReference {
                surah_number,
                ayah_number: ayah_number - 1,
            })
        } else if surah_number > 1 {
            // Last Ayah of previous Surah
            let prev_surah = self.repository.get_surah(surah_number - 1).await?;
            if let Some(prev_surah) = prev_surah {
                Some(AyahReference {
                    surah_number: surah_number - 1,
                    ayah_number: prev_surah.number_of_ayahs,
                })
            } else {
                None
            }
        } else {
            None
        };

        // Get next Ayah
        let next = if ayah_number < current_surah.number_of_ayahs {
            // Next Ayah in same Surah
            Some(AyahReference {
                surah_number,
                ayah_number: ayah_number + 1,
            })
        } else if surah_number < 114 {
            // First Ayah of next Surah
            Some(AyahReference {
                surah_number: surah_number + 1,
                ayah_number: 1,
            })
        } else {
            None
        };

        Ok(Some(AyahNavigation {
            current: AyahReference {
                surah_number,
                ayah_number,
            },
            previous,
            next,
        }))
    }

    /// Get Quran statistics
    pub async fn get_quran_statistics(&self) -> Result<QuranStatistics> {
        let surahs = self.repository.get_all_surahs().await?;
        
        let total_surahs = surahs.len();
        let meccan_surahs = surahs.iter().filter(|s| s.is_meccan()).count();
        let medinan_surahs = surahs.iter().filter(|s| s.is_medinan()).count();
        let total_ayahs: i32 = surahs.iter().map(|s| s.number_of_ayahs).sum();

        Ok(QuranStatistics {
            total_surahs,
            meccan_surahs,
            medinan_surahs,
            total_ayahs: total_ayahs as usize,
        })
    }

    /// Get translations for an Ayah with enhanced filtering
    pub async fn get_translations(&self, request: GetTranslationRequest) -> Result<Option<TranslationResponse>> {
        let ayah = self.repository.get_ayah(request.surah_number, request.ayah_number).await?;
        
        if let Some(ayah) = ayah {
            let surah = self.repository.get_surah(request.surah_number).await?
                .ok_or_else(|| anyhow!("Surah not found"))?;

            let translations = self.repository.get_translations(&request).await?;

            Ok(Some(TranslationResponse {
                ayah,
                surah,
                translations,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get Ayah with translations in a formatted display
    pub async fn get_ayah_with_translations(&self, request: GetTranslationRequest, display_preferences: TranslationDisplayPreferences) -> Result<Option<AyahWithTranslations>> {
        let ayah = self.repository.get_ayah(request.surah_number, request.ayah_number).await?;
        
        if let Some(ayah) = ayah {
            let surah = self.repository.get_surah(request.surah_number).await?
                .ok_or_else(|| anyhow!("Surah not found"))?;

            // Filter translations based on display preferences
            let mut filtered_request = request;
            filtered_request.languages = Some(display_preferences.preferred_languages.clone());
            filtered_request.min_quality_score = Some(display_preferences.quality_threshold);
            filtered_request.approval_status = Some(vec![TranslationApprovalStatus::Approved, TranslationApprovalStatus::Verified]);

            let translations = self.repository.get_translations(&filtered_request).await?;

            Ok(Some(AyahWithTranslations {
                ayah,
                surah,
                translations,
                display_preferences,
            }))
        } else {
            Ok(None)
        }
    }

    /// Manage translation sources
    pub async fn manage_translation_source(&self, request: ManageTranslationSourceRequest) -> Result<serde_json::Value> {
        match request.action {
            TranslationSourceAction::Create => {
                let source_data = request.source_data
                    .ok_or_else(|| anyhow!("Source data required for create action"))?;
                
                let source = TranslationSource::new(
                    source_data.name,
                    source_data.translator,
                    source_data.language,
                    source_data.description,
                    source_data.methodology,
                    source_data.source_reference,
                );

                self.repository.insert_translation_source(&source).await?;
                Ok(serde_json::json!({
                    "action": "created",
                    "source_id": source.id,
                    "quality_score": source.quality_score
                }))
            }
            TranslationSourceAction::Update => {
                let source_id = request.source_id
                    .ok_or_else(|| anyhow!("Source ID required for update action"))?;
                let source_data = request.source_data
                    .ok_or_else(|| anyhow!("Source data required for update action"))?;

                self.repository.update_translation_source(source_id, source_data).await?;
                Ok(serde_json::json!({
                    "action": "updated",
                    "source_id": source_id
                }))
            }
            TranslationSourceAction::Approve => {
                let source_id = request.source_id
                    .ok_or_else(|| anyhow!("Source ID required for approval"))?;

                self.repository.update_translation_source_approval(source_id, TranslationApprovalStatus::Approved).await?;
                Ok(serde_json::json!({
                    "action": "approved",
                    "source_id": source_id
                }))
            }
            TranslationSourceAction::Verify => {
                let source_id = request.source_id
                    .ok_or_else(|| anyhow!("Source ID required for verification"))?;

                let verification_result = self.verify_translation_source_quality(source_id).await?;
                self.repository.update_translation_source_approval(source_id, TranslationApprovalStatus::Verified).await?;
                
                Ok(serde_json::to_value(verification_result)?)
            }
            TranslationSourceAction::Reject => {
                let source_id = request.source_id
                    .ok_or_else(|| anyhow!("Source ID required for rejection"))?;

                self.repository.update_translation_source_approval(source_id, TranslationApprovalStatus::Rejected).await?;
                Ok(serde_json::json!({
                    "action": "rejected",
                    "source_id": source_id
                }))
            }
            TranslationSourceAction::Deactivate => {
                let source_id = request.source_id
                    .ok_or_else(|| anyhow!("Source ID required for deactivation"))?;

                self.repository.deactivate_translation_source(source_id).await?;
                Ok(serde_json::json!({
                    "action": "deactivated",
                    "source_id": source_id
                }))
            }
        }
    }

    /// Verify translation source quality
    async fn verify_translation_source_quality(&self, source_id: Uuid) -> Result<TranslationQualityResult> {
        let source = self.repository.get_translation_source_by_id(source_id).await?
            .ok_or_else(|| anyhow!("Translation source not found"))?;

        let previous_score = source.quality_score;
        
        // Perform quality verification (simplified implementation)
        let quality_factors = vec![
            QualityFactor {
                factor_type: "Source Credibility".to_string(),
                weight: 0.3,
                score: if source.source_reference.is_some() { 9.0 } else { 6.0 },
                description: "Credibility of the translation source".to_string(),
            },
            QualityFactor {
                factor_type: "Translator Expertise".to_string(),
                weight: 0.3,
                score: match source.language.as_str() {
                    "en" => 8.5,
                    "fr" | "es" | "de" => 8.0,
                    "ur" | "tr" | "id" => 7.5,
                    _ => 7.0,
                },
                description: "Expertise level of the translator".to_string(),
            },
            QualityFactor {
                factor_type: "Methodology".to_string(),
                weight: 0.2,
                score: if source.methodology.is_some() { 8.0 } else { 6.0 },
                description: "Translation methodology documentation".to_string(),
            },
            QualityFactor {
                factor_type: "Community Acceptance".to_string(),
                weight: 0.2,
                score: match source.approval_status {
                    TranslationApprovalStatus::Verified => 9.0,
                    TranslationApprovalStatus::Approved => 7.5,
                    TranslationApprovalStatus::Pending => 5.0,
                    TranslationApprovalStatus::Rejected => 2.0,
                },
                description: "Level of community acceptance".to_string(),
            },
        ];

        let new_score = quality_factors.iter()
            .map(|f| f.score * f.weight)
            .sum::<f64>()
            .min(10.0);

        // Update the score in the database
        self.repository.update_translation_source_quality(source_id, new_score).await?;

        let recommendations = self.generate_translation_quality_recommendations(new_score, &source).await?;

        Ok(TranslationQualityResult {
            translation_id: source_id, // Using source_id as translation_id for this context
            previous_score,
            new_score,
            quality_factors,
            recommendations,
            verified_at: Utc::now(),
        })
    }

    /// Generate quality recommendations for translations
    async fn generate_translation_quality_recommendations(&self, score: f64, source: &TranslationSource) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        if score < 6.0 {
            recommendations.push("Consider additional scholarly review and verification".to_string());
            recommendations.push("Cross-reference with established classical translations".to_string());
        } else if score < 8.0 {
            recommendations.push("Good quality - suitable for general reference".to_string());
            if source.methodology.is_none() {
                recommendations.push("Consider documenting translation methodology".to_string());
            }
        } else {
            recommendations.push("Excellent quality - highly recommended for scholarly work".to_string());
        }

        if source.source_reference.is_none() {
            recommendations.push("Consider adding source references for better credibility".to_string());
        }

        if !source.is_approved() {
            recommendations.push("Consider submitting for community approval".to_string());
        }

        Ok(recommendations)
    }

    /// Get all translation sources
    pub async fn get_translation_sources(&self) -> Result<Vec<TranslationSource>> {
        self.repository.get_translation_sources().await
    }

    /// Get translation statistics
    pub async fn get_translation_statistics(&self) -> Result<serde_json::Value> {
        self.repository.get_translation_statistics().await
    }

    /// Verify translation content integrity
    pub async fn verify_translation_integrity(&self) -> Result<TranslationIntegrityReport> {
        let translation_results = self.repository.verify_translation_integrity().await?;

        let mut corrupted_translations = Vec::new();
        let mut valid_translations = 0;

        for (id, is_valid) in translation_results {
            if is_valid {
                valid_translations += 1;
            } else {
                corrupted_translations.push(id);
            }
        }

        Ok(TranslationIntegrityReport {
            translations: IntegrityStatus {
                total: valid_translations + corrupted_translations.len(),
                valid: valid_translations,
                corrupted: corrupted_translations,
            },
        })
    }

    /// Get all available recitation styles
    pub async fn get_recitation_styles(&self) -> Result<Vec<RecitationStyle>> {
        self.repository.get_recitation_styles().await
    }

    /// Get Ayahs by Juz
    pub async fn get_ayahs_by_juz(&self, juz_number: i32) -> Result<Vec<Ayah>> {
        if juz_number < 1 || juz_number > 30 {
            return Err(anyhow!("Invalid Juz number. Must be between 1 and 30"));
        }
        self.repository.get_ayahs_by_juz(juz_number).await
    }

    /// Get Ayahs by page
    pub async fn get_ayahs_by_page(&self, page_number: i32) -> Result<Vec<Ayah>> {
        if page_number < 1 || page_number > 604 {
            return Err(anyhow!("Invalid page number. Must be between 1 and 604"));
        }
        self.repository.get_ayahs_by_page(page_number).await
    }

    /// Get Surahs by revelation type
    pub async fn get_surahs_by_revelation_type(&self, revelation_type: RevelationType) -> Result<Vec<Surah>> {
        self.repository.get_surahs_by_revelation_type(revelation_type).await
    }

    /// Advanced search with filters
    pub async fn advanced_search(&self, query: &str, filters: AdvancedSearchFilters, limit: i32, offset: i32) -> Result<SearchResponse> {
        // Build search request from filters
        let search_request = SearchQuranRequest {
            query: query.to_string(),
            surah_numbers: filters.surah_numbers,
            search_type: Some(SearchType::Text), // Default to text search
            revelation_type: filters.revelation_type,
            juz_numbers: filters.juz_numbers,
            limit: Some(limit),
            offset: Some(offset),
        };

        // For now, use the regular search - in production this would be more sophisticated
        self.search_quran(search_request).await
    }

    /// Get search suggestions
    pub async fn get_search_suggestions(&self, partial_query: &str) -> Result<Vec<String>> {
        if partial_query.trim().is_empty() || partial_query.len() < 2 {
            return Ok(vec![]);
        }
        self.repository.get_search_suggestions(partial_query, 10).await
    }
}

/// Content integrity report
#[derive(Debug, serde::Serialize)]
pub struct ContentIntegrityReport {
    pub ayahs: IntegrityStatus,
    pub tafsir: IntegrityStatus,
}

/// Translation integrity report
#[derive(Debug, serde::Serialize)]
pub struct TranslationIntegrityReport {
    pub translations: IntegrityStatus,
}

/// Integrity status for a content type
#[derive(Debug, serde::Serialize)]
pub struct IntegrityStatus {
    pub total: usize,
    pub valid: usize,
    pub corrupted: Vec<Uuid>,
}

/// Ayah reference for navigation
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AyahReference {
    pub surah_number: i32,
    pub ayah_number: i32,
}

/// Navigation information for an Ayah
#[derive(Debug, serde::Serialize)]
pub struct AyahNavigation {
    pub current: AyahReference,
    pub previous: Option<AyahReference>,
    pub next: Option<AyahReference>,
}

/// Quran statistics
#[derive(Debug, serde::Serialize)]
pub struct QuranStatistics {
    pub total_surahs: usize,
    pub meccan_surahs: usize,
    pub medinan_surahs: usize,
    pub total_ayahs: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ayah_reference_creation() {
        let reference = AyahReference {
            surah_number: 1,
            ayah_number: 1,
        };

        assert_eq!(reference.surah_number, 1);
        assert_eq!(reference.ayah_number, 1);
    }

    #[test]
    fn test_content_integrity_report_creation() {
        let report = ContentIntegrityReport {
            ayahs: IntegrityStatus {
                total: 100,
                valid: 99,
                corrupted: vec![Uuid::new_v4()],
            },
            tafsir: IntegrityStatus {
                total: 50,
                valid: 50,
                corrupted: vec![],
            },
        };

        assert_eq!(report.ayahs.total, 100);
        assert_eq!(report.ayahs.valid, 99);
        assert_eq!(report.ayahs.corrupted.len(), 1);
        assert_eq!(report.tafsir.corrupted.len(), 0);
    }
}
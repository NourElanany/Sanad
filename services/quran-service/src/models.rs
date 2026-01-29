use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

/// Represents a Surah (chapter) in the Quran
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Surah {
    pub number: i32,
    pub name: String,
    pub arabic_name: String,
    pub english_name: String,
    pub revelation_type: RevelationType,
    pub number_of_ayahs: i32,
    pub created_at: DateTime<Utc>,
}

/// Represents an Ayah (verse) in the Quran
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ayah {
    pub id: Uuid,
    pub surah_number: i32,
    pub ayah_number: i32,
    pub text: String,
    pub text_hash: String,
    pub juz: i32,
    pub page: i32,
    pub ruku: Option<i32>,
    pub created_at: DateTime<Utc>,
}

/// Represents a Tafsir (interpretation) source
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TafsirSource {
    pub id: Uuid,
    pub name: String,
    pub author: String,
    pub language: String,
    pub description: Option<String>,
    pub credibility_score: f64, // 0.0 to 10.0 scale
    pub scholarly_authentication: ScholarlyAuthentication,
    pub source_type: TafsirSourceType,
    pub publication_year: Option<i32>,
    pub methodology: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Scholarly authentication level for Tafsir sources
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum ScholarlyAuthentication {
    #[serde(rename = "highly_authenticated")]
    #[sqlx(rename = "highly_authenticated")]
    HighlyAuthenticated, // Classical scholars with ijaza chains
    #[serde(rename = "authenticated")]
    #[sqlx(rename = "authenticated")]
    Authenticated, // Modern scholars with proper credentials
    #[serde(rename = "verified")]
    #[sqlx(rename = "verified")]
    Verified, // Peer-reviewed works
    #[serde(rename = "unverified")]
    #[sqlx(rename = "unverified")]
    Unverified, // Requires further verification
}

/// Type of Tafsir source
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum TafsirSourceType {
    #[serde(rename = "classical")]
    #[sqlx(rename = "classical")]
    Classical, // Classical tafsir works (Ibn Kathir, Tabari, etc.)
    #[serde(rename = "contemporary")]
    #[sqlx(rename = "contemporary")]
    Contemporary, // Modern tafsir works
    #[serde(rename = "linguistic")]
    #[sqlx(rename = "linguistic")]
    Linguistic, // Focus on Arabic language and grammar
    #[serde(rename = "thematic")]
    #[sqlx(rename = "thematic")]
    Thematic, // Thematic interpretation
    #[serde(rename = "sectarian")]
    #[sqlx(rename = "sectarian")]
    Sectarian, // Specific to certain schools of thought
}

/// Represents a Tafsir (interpretation) entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tafsir {
    pub id: Uuid,
    pub surah_number: i32,
    pub ayah_number: i32,
    pub source_id: Uuid,
    pub text: String,
    pub text_hash: String,
    pub word_count: i32,
    pub themes: Vec<String>, // Thematic tags for the interpretation
    pub cross_references: Vec<String>, // References to other verses or hadiths
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Revelation type of a Surah
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "text")]
pub enum RevelationType {
    #[serde(rename = "meccan")]
    #[sqlx(rename = "meccan")]
    Meccan,
    #[serde(rename = "medinan")]
    #[sqlx(rename = "medinan")]
    Medinan,
}

/// Complete Surah with all its Ayahs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurahWithAyahs {
    pub surah: Surah,
    pub ayahs: Vec<Ayah>,
}

/// Ayah with its associated Tafsir entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AyahWithTafsir {
    pub ayah: Ayah,
    pub tafsir_entries: Vec<TafsirWithSource>,
}

/// Tafsir entry with its source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TafsirWithSource {
    pub tafsir: Tafsir,
    pub source: TafsirSource,
}

/// Search result for Quran content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuranSearchResult {
    pub ayah: Ayah,
    pub surah: Surah,
    pub relevance_score: f64,
    pub highlighted_text: String,
    pub context: Option<String>,
}

/// Request to get a specific Surah
#[derive(Debug, Deserialize)]
pub struct GetSurahRequest {
    pub surah_number: i32,
    pub include_ayahs: Option<bool>,
}

/// Request to get a specific Ayah
#[derive(Debug, Deserialize)]
pub struct GetAyahRequest {
    pub surah_number: i32,
    pub ayah_number: i32,
    pub include_tafsir: Option<bool>,
}

/// Request to search in Quran
#[derive(Debug, Deserialize)]
pub struct SearchQuranRequest {
    pub query: String,
    pub surah_numbers: Option<Vec<i32>>,
    pub search_type: Option<SearchType>,
    pub revelation_type: Option<RevelationType>,
    pub juz_numbers: Option<Vec<i32>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Types of search supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "semantic")]
    Semantic,
    #[serde(rename = "root")]
    Root,
    #[serde(rename = "exact")]
    Exact,
}

/// Request to get Tafsir for an Ayah
#[derive(Debug, Deserialize)]
pub struct GetTafsirRequest {
    pub surah_number: i32,
    pub ayah_number: i32,
    pub source_ids: Option<Vec<Uuid>>,
}

/// Response for Surah queries
#[derive(Debug, Serialize)]
pub struct SurahResponse {
    pub surah: Surah,
    pub ayahs: Option<Vec<Ayah>>,
}

/// Response for Ayah queries
#[derive(Debug, Serialize)]
pub struct AyahResponse {
    pub ayah: Ayah,
    pub surah: Surah,
    pub tafsir_entries: Option<Vec<TafsirWithSource>>,
}

/// Response for search queries
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<QuranSearchResult>,
    pub total_count: i64,
    pub query: String,
    pub search_type: SearchType,
    pub search_time_ms: u64,
    pub suggestions: Option<Vec<String>>,
}

/// Represents a recitation style/qira'ah
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RecitationStyle {
    pub id: Uuid,
    pub name: String,
    pub arabic_name: String,
    pub reciter: String,
    pub description: Option<String>,
    pub language: String,
    pub created_at: DateTime<Utc>,
}

/// Represents a translation of Quran meanings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Translation {
    pub id: Uuid,
    pub surah_number: i32,
    pub ayah_number: i32,
    pub language: String,
    pub translator: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

/// Request to get translations
#[derive(Debug, Deserialize)]
pub struct GetTranslationRequest {
    pub surah_number: i32,
    pub ayah_number: i32,
    pub languages: Option<Vec<String>>,
}

/// Response for translation queries
#[derive(Debug, Serialize)]
pub struct TranslationResponse {
    pub ayah: Ayah,
    pub surah: Surah,
    pub translations: Vec<Translation>,
}

/// Response for Tafsir queries
#[derive(Debug, Serialize)]
pub struct TafsirResponse {
    pub ayah: Ayah,
    pub surah: Surah,
    pub tafsir_entries: Vec<TafsirWithSource>,
}

/// Tafsir comparison request
#[derive(Debug, Deserialize)]
pub struct TafsirComparisonRequest {
    pub surah_number: i32,
    pub ayah_number: i32,
    pub source_ids: Vec<Uuid>,
    pub comparison_criteria: Option<Vec<ComparisonCriteria>>,
}

/// Criteria for comparing Tafsir interpretations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonCriteria {
    #[serde(rename = "linguistic")]
    Linguistic, // Compare linguistic interpretations
    #[serde(rename = "thematic")]
    Thematic, // Compare thematic approaches
    #[serde(rename = "historical")]
    Historical, // Compare historical contexts
    #[serde(rename = "jurisprudential")]
    Jurisprudential, // Compare legal implications
    #[serde(rename = "spiritual")]
    Spiritual, // Compare spiritual insights
}

/// Tafsir comparison response
#[derive(Debug, Serialize)]
pub struct TafsirComparisonResponse {
    pub ayah: Ayah,
    pub surah: Surah,
    pub comparisons: Vec<TafsirComparison>,
    pub summary: ComparisonSummary,
    pub recommendations: Vec<String>,
}

/// Individual Tafsir comparison
#[derive(Debug, Serialize)]
pub struct TafsirComparison {
    pub source: TafsirSource,
    pub tafsir: Tafsir,
    pub key_points: Vec<String>,
    pub unique_insights: Vec<String>,
    pub methodology_notes: Option<String>,
}

/// Summary of Tafsir comparison
#[derive(Debug, Serialize)]
pub struct ComparisonSummary {
    pub common_themes: Vec<String>,
    pub divergent_views: Vec<DivergentView>,
    pub scholarly_consensus: Option<String>,
    pub recommended_reading_order: Vec<Uuid>, // Source IDs in recommended order
}

/// Divergent view between interpretations
#[derive(Debug, Serialize)]
pub struct DivergentView {
    pub topic: String,
    pub source_positions: Vec<SourcePosition>,
    pub significance: ViewSignificance,
}

/// Position of a source on a particular topic
#[derive(Debug, Serialize)]
pub struct SourcePosition {
    pub source_id: Uuid,
    pub source_name: String,
    pub position: String,
    pub evidence: Vec<String>,
}

/// Significance level of a divergent view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewSignificance {
    #[serde(rename = "major")]
    Major, // Significant theological or jurisprudential difference
    #[serde(rename = "moderate")]
    Moderate, // Notable difference in interpretation
    #[serde(rename = "minor")]
    Minor, // Minor variation in understanding
}

/// Request to manage Tafsir sources
#[derive(Debug, Deserialize)]
pub struct ManageTafsirSourceRequest {
    pub action: SourceManagementAction,
    pub source_data: Option<TafsirSourceData>,
    pub source_id: Option<Uuid>,
}

/// Actions for managing Tafsir sources
#[derive(Debug, Deserialize)]
pub enum SourceManagementAction {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "verify_credibility")]
    VerifyCredibility,
    #[serde(rename = "update_authentication")]
    UpdateAuthentication,
    #[serde(rename = "deactivate")]
    Deactivate,
}

/// Data for creating or updating Tafsir sources
#[derive(Debug, Deserialize)]
pub struct TafsirSourceData {
    pub name: String,
    pub author: String,
    pub language: String,
    pub description: Option<String>,
    pub source_type: TafsirSourceType,
    pub publication_year: Option<i32>,
    pub methodology: Option<String>,
    pub scholarly_authentication: Option<ScholarlyAuthentication>,
}

/// Credibility verification result
#[derive(Debug, Serialize)]
pub struct CredibilityVerificationResult {
    pub source_id: Uuid,
    pub previous_score: f64,
    pub new_score: f64,
    pub verification_factors: Vec<VerificationFactor>,
    pub recommendations: Vec<String>,
    pub verified_at: DateTime<Utc>,
}

/// Factor used in credibility verification
#[derive(Debug, Serialize)]
pub struct VerificationFactor {
    pub factor_type: String,
    pub weight: f64,
    pub score: f64,
    pub description: String,
}

/// Advanced Tafsir search request
#[derive(Debug, Deserialize)]
pub struct AdvancedTafsirSearchRequest {
    pub query: String,
    pub search_criteria: Vec<TafsirSearchCriteria>,
    pub source_filters: Option<TafsirSourceFilters>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Criteria for searching Tafsir
#[derive(Debug, Deserialize)]
pub enum TafsirSearchCriteria {
    #[serde(rename = "text_content")]
    TextContent,
    #[serde(rename = "themes")]
    Themes,
    #[serde(rename = "cross_references")]
    CrossReferences,
    #[serde(rename = "author_name")]
    AuthorName,
    #[serde(rename = "methodology")]
    Methodology,
}

/// Filters for Tafsir sources
#[derive(Debug, Deserialize)]
pub struct TafsirSourceFilters {
    pub source_types: Option<Vec<TafsirSourceType>>,
    pub authentication_levels: Option<Vec<ScholarlyAuthentication>>,
    pub languages: Option<Vec<String>>,
    pub credibility_range: Option<(f64, f64)>,
    pub publication_year_range: Option<(i32, i32)>,
}

/// Advanced Tafsir search response
#[derive(Debug, Serialize)]
pub struct AdvancedTafsirSearchResponse {
    pub results: Vec<TafsirSearchResult>,
    pub total_count: i64,
    pub search_time_ms: u64,
    pub facets: SearchFacets,
}

/// Individual Tafsir search result
#[derive(Debug, Serialize)]
pub struct TafsirSearchResult {
    pub tafsir: Tafsir,
    pub source: TafsirSource,
    pub ayah: Ayah,
    pub surah: Surah,
    pub relevance_score: f64,
    pub highlighted_text: String,
    pub matching_criteria: Vec<String>,
}

/// Search facets for filtering results
#[derive(Debug, Serialize)]
pub struct SearchFacets {
    pub source_types: Vec<FacetCount>,
    pub authentication_levels: Vec<FacetCount>,
    pub languages: Vec<FacetCount>,
    pub authors: Vec<FacetCount>,
}

/// Count for a specific facet value
#[derive(Debug, Serialize)]
pub struct FacetCount {
    pub value: String,
    pub count: i64,
}

/// Tafsir analytics request
#[derive(Debug, Deserialize)]
pub struct TafsirAnalyticsRequest {
    pub surah_number: Option<i32>,
    pub ayah_range: Option<(i32, i32)>,
    pub source_ids: Option<Vec<Uuid>>,
    pub analysis_type: AnalysisType,
}

/// Type of Tafsir analysis
#[derive(Debug, Deserialize)]
pub enum AnalysisType {
    #[serde(rename = "coverage")]
    Coverage, // Analyze coverage of verses by different sources
    #[serde(rename = "themes")]
    Themes, // Analyze thematic distribution
    #[serde(rename = "methodology")]
    Methodology, // Analyze methodological approaches
    #[serde(rename = "consensus")]
    Consensus, // Analyze areas of scholarly consensus
}

/// Tafsir analytics response
#[derive(Debug, Serialize)]
pub struct TafsirAnalyticsResponse {
    pub analysis_type: String,
    pub data: serde_json::Value,
    pub insights: Vec<String>,
    pub recommendations: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

/// Advanced search filters
#[derive(Debug, Deserialize)]
pub struct AdvancedSearchFilters {
    pub surah_numbers: Option<Vec<i32>>,
    pub revelation_type: Option<RevelationType>,
    pub juz_numbers: Option<Vec<i32>>,
    pub page_range: Option<(i32, i32)>,
    pub word_count_range: Option<(i32, i32)>,
    pub include_context: Option<bool>,
}

/// Context information for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchContext {
    pub previous_ayah: Option<Ayah>,
    pub next_ayah: Option<Ayah>,
    pub surah_info: Surah,
}

/// Enhanced search result with context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedQuranSearchResult {
    pub ayah: Ayah,
    pub surah: Surah,
    pub relevance_score: f64,
    pub highlighted_text: String,
    pub context: Option<SearchContext>,
    pub word_positions: Vec<(usize, usize)>, // Start and end positions of matched words
}

impl Ayah {
    /// Verify the integrity of the Ayah text using SHA-256 hash
    pub fn verify_integrity(&self) -> bool {
        let calculated_hash = Self::calculate_text_hash(&self.text);
        calculated_hash == self.text_hash
    }

    /// Calculate SHA-256 hash for the given text
    pub fn calculate_text_hash(text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Create a new Ayah with calculated hash
    pub fn new(
        surah_number: i32,
        ayah_number: i32,
        text: String,
        juz: i32,
        page: i32,
        ruku: Option<i32>,
    ) -> Self {
        let text_hash = Self::calculate_text_hash(&text);
        Self {
            id: Uuid::new_v4(),
            surah_number,
            ayah_number,
            text,
            text_hash,
            juz,
            page,
            ruku,
            created_at: Utc::now(),
        }
    }
}

impl Tafsir {
    /// Verify the integrity of the Tafsir text using SHA-256 hash
    pub fn verify_integrity(&self) -> bool {
        let calculated_hash = Self::calculate_text_hash(&self.text);
        calculated_hash == self.text_hash
    }

    /// Calculate SHA-256 hash for the given text
    pub fn calculate_text_hash(text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Create a new Tafsir with calculated hash
    pub fn new(
        surah_number: i32,
        ayah_number: i32,
        source_id: Uuid,
        text: String,
    ) -> Self {
        let text_hash = Self::calculate_text_hash(&text);
        let word_count = text.split_whitespace().count() as i32;
        
        Self {
            id: Uuid::new_v4(),
            surah_number,
            ayah_number,
            source_id,
            text,
            text_hash,
            word_count,
            themes: Vec::new(),
            cross_references: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Create a new Tafsir with themes and cross-references
    pub fn new_with_metadata(
        surah_number: i32,
        ayah_number: i32,
        source_id: Uuid,
        text: String,
        themes: Vec<String>,
        cross_references: Vec<String>,
    ) -> Self {
        let text_hash = Self::calculate_text_hash(&text);
        let word_count = text.split_whitespace().count() as i32;
        
        Self {
            id: Uuid::new_v4(),
            surah_number,
            ayah_number,
            source_id,
            text,
            text_hash,
            word_count,
            themes,
            cross_references,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Extract key themes from the Tafsir text (simplified implementation)
    pub fn extract_themes(&self) -> Vec<String> {
        // This is a simplified implementation
        // In production, this would use NLP techniques for Arabic text
        let mut themes = Vec::new();
        
        let text_lower = self.text.to_lowercase();
        
        // Common Islamic themes (this would be much more sophisticated in production)
        let theme_keywords = vec![
            ("توحيد", "Tawhid"),
            ("صلاة", "Prayer"),
            ("زكاة", "Zakat"),
            ("صوم", "Fasting"),
            ("حج", "Hajj"),
            ("جهاد", "Jihad"),
            ("صبر", "Patience"),
            ("تقوى", "Taqwa"),
            ("رحمة", "Mercy"),
            ("عدل", "Justice"),
        ];
        
        for (arabic, english) in theme_keywords {
            if text_lower.contains(arabic) {
                themes.push(english.to_string());
            }
        }
        
        themes
    }

    /// Check if this Tafsir is comprehensive (word count > 100)
    pub fn is_comprehensive(&self) -> bool {
        self.word_count > 100
    }

    /// Get reading time estimate in minutes
    pub fn estimated_reading_time(&self) -> i32 {
        // Assuming average reading speed of 200 words per minute for Arabic
        (self.word_count as f32 / 200.0).ceil() as i32
    }
}

impl Surah {
    /// Create a new Surah
    pub fn new(
        number: i32,
        name: String,
        arabic_name: String,
        english_name: String,
        revelation_type: RevelationType,
        number_of_ayahs: i32,
    ) -> Self {
        Self {
            number,
            name,
            arabic_name,
            english_name,
            revelation_type,
            number_of_ayahs,
            created_at: Utc::now(),
        }
    }

    /// Check if this is a Meccan Surah
    pub fn is_meccan(&self) -> bool {
        matches!(self.revelation_type, RevelationType::Meccan)
    }

    /// Check if this is a Medinan Surah
    pub fn is_medinan(&self) -> bool {
        matches!(self.revelation_type, RevelationType::Medinan)
    }
}

impl TafsirSource {
    /// Create a new Tafsir source
    pub fn new(
        name: String,
        author: String,
        language: String,
        description: Option<String>,
        source_type: TafsirSourceType,
        scholarly_authentication: ScholarlyAuthentication,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            author,
            language,
            description,
            credibility_score: Self::calculate_initial_credibility_score(&scholarly_authentication, &source_type),
            scholarly_authentication,
            source_type,
            publication_year: None,
            methodology: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Calculate initial credibility score based on authentication and type
    pub fn calculate_initial_credibility_score(
        authentication: &ScholarlyAuthentication,
        source_type: &TafsirSourceType,
    ) -> f64 {
        let auth_score: f64 = match authentication {
            ScholarlyAuthentication::HighlyAuthenticated => 9.0,
            ScholarlyAuthentication::Authenticated => 7.5,
            ScholarlyAuthentication::Verified => 6.0,
            ScholarlyAuthentication::Unverified => 4.0,
        };

        let type_modifier: f64 = match source_type {
            TafsirSourceType::Classical => 1.0,
            TafsirSourceType::Contemporary => 0.9,
            TafsirSourceType::Linguistic => 0.95,
            TafsirSourceType::Thematic => 0.9,
            TafsirSourceType::Sectarian => 0.8,
        };

        (auth_score * type_modifier).min(10.0)
    }

    /// Check if source is highly credible (score >= 8.0)
    pub fn is_highly_credible(&self) -> bool {
        self.credibility_score >= 8.0
    }

    /// Check if source is authenticated
    pub fn is_authenticated(&self) -> bool {
        matches!(
            self.scholarly_authentication,
            ScholarlyAuthentication::HighlyAuthenticated | ScholarlyAuthentication::Authenticated
        )
    }

    /// Get credibility level as string
    pub fn credibility_level(&self) -> String {
        match self.credibility_score {
            9.0..=10.0 => "Excellent".to_string(),
            7.5..=8.9 => "Very Good".to_string(),
            6.0..=7.4 => "Good".to_string(),
            4.0..=5.9 => "Fair".to_string(),
            _ => "Poor".to_string(),
        }
    }
}

/// Trait for content integrity verification
pub trait ContentIntegrity {
    fn verify_integrity(&self) -> bool;
    fn calculate_hash(&self) -> String;
}

impl ContentIntegrity for Ayah {
    fn verify_integrity(&self) -> bool {
        self.verify_integrity()
    }

    fn calculate_hash(&self) -> String {
        Self::calculate_text_hash(&self.text)
    }
}

impl ContentIntegrity for Tafsir {
    fn verify_integrity(&self) -> bool {
        self.verify_integrity()
    }

    fn calculate_hash(&self) -> String {
        Self::calculate_text_hash(&self.text)
    }
}

/// Trait for serialization and deserialization
pub trait Serializable: Serialize + for<'de> Deserialize<'de> {}

impl Serializable for Surah {}
impl Serializable for Ayah {}
impl Serializable for Tafsir {}
impl Serializable for TafsirSource {}
impl Serializable for SurahWithAyahs {}
impl Serializable for AyahWithTafsir {}
impl Serializable for TafsirWithSource {}
impl Serializable for QuranSearchResult {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ayah_integrity_verification() {
        let ayah = Ayah::new(
            1,
            1,
            "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ".to_string(),
            1,
            1,
            Some(1),
        );

        assert!(ayah.verify_integrity());
    }

    #[test]
    fn test_ayah_integrity_failure() {
        let mut ayah = Ayah::new(
            1,
            1,
            "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ".to_string(),
            1,
            1,
            Some(1),
        );

        // Tamper with the text
        ayah.text = "modified text".to_string();

        assert!(!ayah.verify_integrity());
    }

    #[test]
    fn test_tafsir_integrity_verification() {
        let tafsir = Tafsir::new(
            1,
            1,
            Uuid::new_v4(),
            "تفسير البسملة".to_string(),
        );

        assert!(tafsir.verify_integrity());
    }

    #[test]
    fn test_tafsir_with_metadata() {
        let themes = vec!["Tawhid".to_string(), "Mercy".to_string()];
        let cross_refs = vec!["2:163".to_string(), "17:110".to_string()];
        
        let tafsir = Tafsir::new_with_metadata(
            1,
            1,
            Uuid::new_v4(),
            "تفسير البسملة مع التوحيد والرحمة".to_string(),
            themes.clone(),
            cross_refs.clone(),
        );

        assert!(tafsir.verify_integrity());
        assert_eq!(tafsir.themes, themes);
        assert_eq!(tafsir.cross_references, cross_refs);
        assert!(tafsir.word_count > 0);
    }

    #[test]
    fn test_tafsir_source_credibility() {
        let source = TafsirSource::new(
            "تفسير ابن كثير".to_string(),
            "ابن كثير".to_string(),
            "ar".to_string(),
            Some("Classical Tafsir".to_string()),
            TafsirSourceType::Classical,
            ScholarlyAuthentication::HighlyAuthenticated,
        );

        assert!(source.is_highly_credible());
        assert!(source.is_authenticated());
        assert_eq!(source.credibility_level(), "Excellent");
    }

    #[test]
    fn test_credibility_score_calculation() {
        let score = TafsirSource::calculate_initial_credibility_score(
            &ScholarlyAuthentication::HighlyAuthenticated,
            &TafsirSourceType::Classical,
        );
        assert_eq!(score, 9.0);

        let score2 = TafsirSource::calculate_initial_credibility_score(
            &ScholarlyAuthentication::Verified,
            &TafsirSourceType::Sectarian,
        );
        assert_eq!(score2, 4.8); // 6.0 * 0.8
    }

    #[test]
    fn test_tafsir_theme_extraction() {
        let tafsir = Tafsir::new(
            1,
            1,
            Uuid::new_v4(),
            "هذا تفسير يتحدث عن التوحيد والصلاة والصبر".to_string(),
        );

        let themes = tafsir.extract_themes();
        assert!(themes.contains(&"Tawhid".to_string()));
        assert!(themes.contains(&"Prayer".to_string()));
        assert!(themes.contains(&"Patience".to_string()));
    }

    #[test]
    fn test_tafsir_reading_time() {
        let long_text = "كلمة ".repeat(250); // 250 words
        let tafsir = Tafsir::new(
            1,
            1,
            Uuid::new_v4(),
            long_text,
        );

        assert_eq!(tafsir.word_count, 250);
        assert_eq!(tafsir.estimated_reading_time(), 2); // 250/200 = 1.25, rounded up to 2
        assert!(tafsir.is_comprehensive());
    }

    #[test]
    fn test_surah_type_checks() {
        let meccan_surah = Surah::new(
            1,
            "Al-Fatiha".to_string(),
            "الفاتحة".to_string(),
            "The Opening".to_string(),
            RevelationType::Meccan,
            7,
        );

        let medinan_surah = Surah::new(
            2,
            "Al-Baqarah".to_string(),
            "البقرة".to_string(),
            "The Cow".to_string(),
            RevelationType::Medinan,
            286,
        );

        assert!(meccan_surah.is_meccan());
        assert!(!meccan_surah.is_medinan());
        assert!(medinan_surah.is_medinan());
        assert!(!medinan_surah.is_meccan());
    }

    #[test]
    fn test_hash_consistency() {
        let text = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
        let hash1 = Ayah::calculate_text_hash(text);
        let hash2 = Ayah::calculate_text_hash(text);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_serialization() {
        let surah = Surah::new(
            1,
            "Al-Fatiha".to_string(),
            "الفاتحة".to_string(),
            "The Opening".to_string(),
            RevelationType::Meccan,
            7,
        );

        let json = serde_json::to_string(&surah).unwrap();
        let deserialized: Surah = serde_json::from_str(&json).unwrap();

        assert_eq!(surah.number, deserialized.number);
        assert_eq!(surah.name, deserialized.name);
        assert_eq!(surah.arabic_name, deserialized.arabic_name);
    }
}
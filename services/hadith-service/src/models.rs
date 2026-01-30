use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

/// Trait for content integrity verification - ensures Islamic content authenticity
pub trait ContentIntegrity {
    fn verify_integrity(&self) -> bool;
    fn calculate_hash(&self) -> String;
}

/// Trait for serialization and deserialization
pub trait Serializable: Serialize + for<'de> Deserialize<'de> {}

/// Represents a Hadith (prophetic tradition)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Hadith {
    pub id: Uuid,
    pub hadith_number: String,
    pub text: String,
    pub text_hash: String, // SHA-256 hash for integrity verification
    pub narrator: String,
    pub book: String,
    pub chapter: String,
    pub chapter_number: Option<i32>,
    pub hadith_number_in_chapter: Option<i32>,
    pub grade: HadithGrade,
    pub source: String,
    pub language: String,
    pub word_count: i32,
    pub themes: Vec<String>, // Thematic tags for categorization
    pub keywords: Vec<String>, // Keywords for search optimization
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents the chain of narration (Sanad) for a Hadith
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Sanad {
    pub id: Uuid,
    pub hadith_id: Uuid,
    pub chain_text: String,
    pub chain_hash: String, // SHA-256 hash for integrity verification
    pub narrators: Vec<String>, // Ordered list of narrators in the chain
    pub chain_grade: ChainGrade,
    pub chain_analysis: Option<String>, // Scholarly analysis of the chain
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents an explanation/commentary (Sharh) of a Hadith
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HadithExplanation {
    pub id: Uuid,
    pub hadith_id: Uuid,
    pub scholar_id: Uuid,
    pub explanation_text: String,
    pub explanation_hash: String, // SHA-256 hash for integrity verification
    pub word_count: i32,
    pub key_points: Vec<String>, // Main points covered in the explanation
    pub related_verses: Vec<String>, // Related Quranic verses
    pub related_hadiths: Vec<String>, // Related Hadith references
    pub language: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents a scholar who provided Hadith explanations
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Scholar {
    pub id: Uuid,
    pub name: String,
    pub arabic_name: String,
    pub birth_year: Option<i32>,
    pub death_year: Option<i32>,
    pub biography: Option<String>,
    pub specialization: Vec<String>, // Areas of expertise
    pub credibility_score: f64, // 0.0 to 10.0 scale
    pub scholarly_authentication: ScholarlyAuthentication,
    pub school_of_thought: Option<String>, // Madhab or scholarly approach
    pub major_works: Vec<String>, // List of major scholarly works
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents a Hadith book/collection
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HadithBook {
    pub id: Uuid,
    pub name: String,
    pub arabic_name: String,
    pub author: String,
    pub author_arabic_name: String,
    pub description: Option<String>,
    pub compilation_year: Option<i32>,
    pub total_hadiths: i32,
    pub book_type: HadithBookType,
    pub authenticity_level: BookAuthenticityLevel,
    pub language: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents a chapter within a Hadith book
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HadithChapter {
    pub id: Uuid,
    pub book_id: Uuid,
    pub chapter_number: i32,
    pub title: String,
    pub arabic_title: String,
    pub description: Option<String>,
    pub hadith_count: i32,
    pub themes: Vec<String>, // Thematic tags for the chapter
    pub created_at: DateTime<Utc>,
}

/// Hadith authenticity grades according to Islamic scholarship
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum HadithGrade {
    #[serde(rename = "sahih")]
    #[sqlx(rename = "sahih")]
    Sahih, // صحيح - Authentic
    #[serde(rename = "hasan")]
    #[sqlx(rename = "hasan")]
    Hasan, // حسن - Good
    #[serde(rename = "daif")]
    #[sqlx(rename = "daif")]
    Daif, // ضعيف - Weak
    #[serde(rename = "mawdu")]
    #[sqlx(rename = "mawdu")]
    Mawdu, // موضوع - Fabricated
}

/// Chain of narration grades for Sanad authenticity
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum ChainGrade {
    #[serde(rename = "sahih")]
    #[sqlx(rename = "sahih")]
    Sahih, // صحيح - Authentic chain
    #[serde(rename = "hasan")]
    #[sqlx(rename = "hasan")]
    Hasan, // حسن - Good chain
    #[serde(rename = "daif")]
    #[sqlx(rename = "daif")]
    Daif, // ضعيف - Weak chain
    #[serde(rename = "munqati")]
    #[sqlx(rename = "munqati")]
    Munqati, // منقطع - Broken chain
    #[serde(rename = "mursal")]
    #[sqlx(rename = "mursal")]
    Mursal, // مرسل - Missing companion
}

/// Scholarly authentication levels
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

/// Types of Hadith books
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum HadithBookType {
    #[serde(rename = "sahih")]
    #[sqlx(rename = "sahih")]
    Sahih, // صحيح - Authentic collections (Bukhari, Muslim)
    #[serde(rename = "sunan")]
    #[sqlx(rename = "sunan")]
    Sunan, // سنن - Sunan collections (Abu Dawud, Tirmidhi, etc.)
    #[serde(rename = "musnad")]
    #[sqlx(rename = "musnad")]
    Musnad, // مسند - Musnad collections (Ahmad, etc.)
    #[serde(rename = "mujam")]
    #[sqlx(rename = "mujam")]
    Mujam, // معجم - Dictionary-style collections
    #[serde(rename = "mustadrak")]
    #[sqlx(rename = "mustadrak")]
    Mustadrak, // مستدرك - Supplementary collections
    #[serde(rename = "jami")]
    #[sqlx(rename = "jami")]
    Jami, // جامع - Comprehensive collections
}

/// Book authenticity levels
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum BookAuthenticityLevel {
    #[serde(rename = "highest")]
    #[sqlx(rename = "highest")]
    Highest, // أعلى درجة - Like Bukhari and Muslim
    #[serde(rename = "high")]
    #[sqlx(rename = "high")]
    High, // عالية - Like the four Sunan
    #[serde(rename = "moderate")]
    #[sqlx(rename = "moderate")]
    Moderate, // متوسطة - Mixed authenticity
    #[serde(rename = "variable")]
    #[sqlx(rename = "variable")]
    Variable, // متغيرة - Requires individual hadith verification
}

/// Thematic categories for Hadith classification
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum HadithTheme {
    #[serde(rename = "aqidah")]
    #[sqlx(rename = "aqidah")]
    Aqidah, // عقيدة - Creed and belief
    #[serde(rename = "worship")]
    #[sqlx(rename = "worship")]
    Worship, // عبادة - Acts of worship
    #[serde(rename = "transactions")]
    #[sqlx(rename = "transactions")]
    Transactions, // معاملات - Business and transactions
    #[serde(rename = "family")]
    #[sqlx(rename = "family")]
    Family, // أسرة - Family and marriage
    #[serde(rename = "ethics")]
    #[sqlx(rename = "ethics")]
    Ethics, // أخلاق - Ethics and morality
    #[serde(rename = "history")]
    #[sqlx(rename = "history")]
    History, // تاريخ - Historical events
    #[serde(rename = "prophecies")]
    #[sqlx(rename = "prophecies")]
    Prophecies, // نبوءات - Prophetic predictions
    #[serde(rename = "jurisprudence")]
    #[sqlx(rename = "jurisprudence")]
    Jurisprudence, // فقه - Islamic jurisprudence
}

impl Hadith {
    /// Create a new Hadith with automatic hash generation
    pub fn new(
        hadith_number: String,
        text: String,
        narrator: String,
        book: String,
        chapter: String,
        grade: HadithGrade,
        source: String,
        language: String,
    ) -> Self {
        let text_hash = Self::generate_hash(&text);
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            hadith_number,
            text,
            text_hash,
            narrator,
            book,
            chapter,
            chapter_number: None,
            hadith_number_in_chapter: None,
            grade,
            source,
            language,
            word_count: 0, // Will be calculated separately
            themes: Vec::new(),
            keywords: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Generate SHA-256 hash for text integrity verification
    pub fn generate_hash(text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Verify text integrity using stored hash
    pub fn verify_integrity(&self) -> bool {
        Self::generate_hash(&self.text) == self.text_hash
    }

    /// Calculate word count for the hadith text
    pub fn calculate_word_count(&mut self) {
        self.word_count = self.text.split_whitespace().count() as i32;
    }

    /// Add a theme to the hadith
    pub fn add_theme(&mut self, theme: String) {
        if !self.themes.contains(&theme) {
            self.themes.push(theme);
        }
    }

    /// Add a keyword for search optimization
    pub fn add_keyword(&mut self, keyword: String) {
        if !self.keywords.contains(&keyword) {
            self.keywords.push(keyword);
        }
    }

    /// Check if hadith is authentic (Sahih or Hasan)
    pub fn is_authentic(&self) -> bool {
        matches!(self.grade, HadithGrade::Sahih | HadithGrade::Hasan)
    }

    /// Get Arabic name for the grade
    pub fn grade_arabic(&self) -> &'static str {
        match self.grade {
            HadithGrade::Sahih => "صحيح",
            HadithGrade::Hasan => "حسن",
            HadithGrade::Daif => "ضعيف",
            HadithGrade::Mawdu => "موضوع",
        }
    }
}

impl Sanad {
    /// Create a new Sanad with automatic hash generation
    pub fn new(
        hadith_id: Uuid,
        chain_text: String,
        narrators: Vec<String>,
        chain_grade: ChainGrade,
    ) -> Self {
        let chain_hash = Self::generate_hash(&chain_text);
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            hadith_id,
            chain_text,
            chain_hash,
            narrators,
            chain_grade,
            chain_analysis: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Generate SHA-256 hash for chain integrity verification
    pub fn generate_hash(chain_text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(chain_text.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Verify chain integrity using stored hash
    pub fn verify_integrity(&self) -> bool {
        Self::generate_hash(&self.chain_text) == self.chain_hash
    }

    /// Get the number of narrators in the chain
    pub fn narrator_count(&self) -> usize {
        self.narrators.len()
    }

    /// Check if the chain is continuous (no breaks)
    pub fn is_continuous(&self) -> bool {
        !matches!(self.chain_grade, ChainGrade::Munqati | ChainGrade::Mursal)
    }

    /// Get Arabic name for the chain grade
    pub fn grade_arabic(&self) -> &'static str {
        match self.chain_grade {
            ChainGrade::Sahih => "صحيح",
            ChainGrade::Hasan => "حسن",
            ChainGrade::Daif => "ضعيف",
            ChainGrade::Munqati => "منقطع",
            ChainGrade::Mursal => "مرسل",
        }
    }
}

impl HadithExplanation {
    /// Create a new HadithExplanation with automatic hash generation
    pub fn new(
        hadith_id: Uuid,
        scholar_id: Uuid,
        explanation_text: String,
        language: String,
    ) -> Self {
        let explanation_hash = Self::generate_hash(&explanation_text);
        let word_count = explanation_text.split_whitespace().count() as i32;
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            hadith_id,
            scholar_id,
            explanation_text,
            explanation_hash,
            word_count,
            key_points: Vec::new(),
            related_verses: Vec::new(),
            related_hadiths: Vec::new(),
            language,
            created_at: now,
            updated_at: now,
        }
    }

    /// Generate SHA-256 hash for explanation integrity verification
    pub fn generate_hash(text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Verify explanation integrity using stored hash
    pub fn verify_integrity(&self) -> bool {
        Self::generate_hash(&self.explanation_text) == self.explanation_hash
    }

    /// Add a key point to the explanation
    pub fn add_key_point(&mut self, point: String) {
        if !self.key_points.contains(&point) {
            self.key_points.push(point);
        }
    }

    /// Add a related Quranic verse reference
    pub fn add_related_verse(&mut self, verse_ref: String) {
        if !self.related_verses.contains(&verse_ref) {
            self.related_verses.push(verse_ref);
        }
    }

    /// Add a related hadith reference
    pub fn add_related_hadith(&mut self, hadith_ref: String) {
        if !self.related_hadiths.contains(&hadith_ref) {
            self.related_hadiths.push(hadith_ref);
        }
    }

    /// Estimate reading time in minutes (assuming 200 words per minute for Arabic)
    pub fn estimated_reading_time(&self) -> f64 {
        self.word_count as f64 / 200.0
    }
}

impl Scholar {
    /// Create a new Scholar
    pub fn new(
        name: String,
        arabic_name: String,
        scholarly_authentication: ScholarlyAuthentication,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            name,
            arabic_name,
            birth_year: None,
            death_year: None,
            biography: None,
            specialization: Vec::new(),
            credibility_score: 5.0, // Default middle score
            scholarly_authentication,
            school_of_thought: None,
            major_works: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if the scholar is from classical period (before 1400 CE)
    pub fn is_classical(&self) -> bool {
        self.death_year.map_or(false, |year| year < 1400)
    }

    /// Check if the scholar is highly credible (score >= 8.0)
    pub fn is_highly_credible(&self) -> bool {
        self.credibility_score >= 8.0
    }

    /// Add a specialization area
    pub fn add_specialization(&mut self, area: String) {
        if !self.specialization.contains(&area) {
            self.specialization.push(area);
        }
    }

    /// Add a major work
    pub fn add_major_work(&mut self, work: String) {
        if !self.major_works.contains(&work) {
            self.major_works.push(work);
        }
    }
}

impl HadithBook {
    /// Create a new HadithBook
    pub fn new(
        name: String,
        arabic_name: String,
        author: String,
        author_arabic_name: String,
        book_type: HadithBookType,
        authenticity_level: BookAuthenticityLevel,
        language: String,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            name,
            arabic_name,
            author,
            author_arabic_name,
            description: None,
            compilation_year: None,
            total_hadiths: 0,
            book_type,
            authenticity_level,
            language,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if the book is from the most authentic collections
    pub fn is_most_authentic(&self) -> bool {
        matches!(self.authenticity_level, BookAuthenticityLevel::Highest)
    }

    /// Get the Arabic name for book type
    pub fn book_type_arabic(&self) -> &'static str {
        match self.book_type {
            HadithBookType::Sahih => "صحيح",
            HadithBookType::Sunan => "سنن",
            HadithBookType::Musnad => "مسند",
            HadithBookType::Mujam => "معجم",
            HadithBookType::Mustadrak => "مستدرك",
            HadithBookType::Jami => "جامع",
        }
    }
}

impl HadithChapter {
    /// Create a new HadithChapter
    pub fn new(
        book_id: Uuid,
        chapter_number: i32,
        title: String,
        arabic_title: String,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            book_id,
            chapter_number,
            title,
            arabic_title,
            description: None,
            hadith_count: 0,
            themes: Vec::new(),
            created_at: now,
        }
    }

    /// Add a theme to the chapter
    pub fn add_theme(&mut self, theme: String) {
        if !self.themes.contains(&theme) {
            self.themes.push(theme);
        }
    }
}

// Display implementations for better debugging and logging
impl std::fmt::Display for HadithGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HadithGrade::Sahih => write!(f, "Sahih (صحيح)"),
            HadithGrade::Hasan => write!(f, "Hasan (حسن)"),
            HadithGrade::Daif => write!(f, "Daif (ضعيف)"),
            HadithGrade::Mawdu => write!(f, "Mawdu (موضوع)"),
        }
    }
}

impl std::fmt::Display for ChainGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChainGrade::Sahih => write!(f, "Sahih (صحيح)"),
            ChainGrade::Hasan => write!(f, "Hasan (حسن)"),
            ChainGrade::Daif => write!(f, "Daif (ضعيف)"),
            ChainGrade::Munqati => write!(f, "Munqati (منقطع)"),
            ChainGrade::Mursal => write!(f, "Mursal (مرسل)"),
        }
    }
}

// Implement ContentIntegrity trait for main structs
impl ContentIntegrity for Hadith {
    fn verify_integrity(&self) -> bool {
        self.verify_integrity()
    }

    fn calculate_hash(&self) -> String {
        Self::generate_hash(&self.text)
    }
}

impl ContentIntegrity for Sanad {
    fn verify_integrity(&self) -> bool {
        self.verify_integrity()
    }

    fn calculate_hash(&self) -> String {
        Self::generate_hash(&self.chain_text)
    }
}

impl ContentIntegrity for HadithExplanation {
    fn verify_integrity(&self) -> bool {
        self.verify_integrity()
    }

    fn calculate_hash(&self) -> String {
        Self::generate_hash(&self.explanation_text)
    }
}

// Implement Serializable trait for all main structs
impl Serializable for Hadith {}
impl Serializable for Sanad {}
impl Serializable for HadithExplanation {}
impl Serializable for Scholar {}
impl Serializable for HadithBook {}
impl Serializable for HadithChapter {}

/// Request/Response models for API endpoints

/// Request to get a specific Hadith
#[derive(Debug, Deserialize)]
pub struct GetHadithRequest {
    pub hadith_id: Option<Uuid>,
    pub hadith_number: Option<String>,
    pub book_name: Option<String>,
    pub include_sanad: Option<bool>,
    pub include_explanations: Option<bool>,
}

/// Request to search Hadiths
#[derive(Debug, Deserialize)]
pub struct SearchHadithRequest {
    pub query: String,
    pub books: Option<Vec<String>>,
    pub grades: Option<Vec<HadithGrade>>,
    pub themes: Option<Vec<String>>,
    pub search_type: Option<SearchType>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Types of search supported for Hadiths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "semantic")]
    Semantic,
    #[serde(rename = "narrator")]
    Narrator,
    #[serde(rename = "theme")]
    Theme,
    #[serde(rename = "exact")]
    Exact,
}

/// Complete Hadith with all related information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HadithWithDetails {
    pub hadith: Hadith,
    pub book: HadithBook,
    pub chapter: Option<HadithChapter>,
    pub sanad: Option<Sanad>,
    pub explanations: Vec<HadithExplanationWithScholar>,
}

/// Hadith explanation with scholar information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HadithExplanationWithScholar {
    pub explanation: HadithExplanation,
    pub scholar: Scholar,
}

/// Search result for Hadith content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HadithSearchResult {
    pub hadith: Hadith,
    pub book: HadithBook,
    pub chapter: Option<HadithChapter>,
    pub relevance_score: f64,
    pub highlighted_text: String,
    pub matching_criteria: Vec<String>,
}

/// Response for Hadith queries
#[derive(Debug, Serialize)]
pub struct HadithResponse {
    pub hadith: Hadith,
    pub book: HadithBook,
    pub chapter: Option<HadithChapter>,
    pub sanad: Option<Sanad>,
    pub explanations: Option<Vec<HadithExplanationWithScholar>>,
}

/// Response for search queries
#[derive(Debug, Serialize)]
pub struct HadithSearchResponse {
    pub results: Vec<HadithSearchResult>,
    pub total_count: i64,
    pub query: String,
    pub search_type: SearchType,
    pub search_time_ms: u64,
    pub facets: Option<SearchFacets>,
}

/// Search facets for filtering results
#[derive(Debug, Serialize)]
pub struct SearchFacets {
    pub books: Vec<FacetCount>,
    pub grades: Vec<FacetCount>,
    pub themes: Vec<FacetCount>,
    pub narrators: Vec<FacetCount>,
}

/// Count for a specific facet value
#[derive(Debug, Serialize)]
pub struct FacetCount {
    pub value: String,
    pub count: i64,
}

/// Request to get Hadiths by topic/theme
#[derive(Debug, Deserialize)]
pub struct GetHadithsByTopicRequest {
    pub topic: String,
    pub include_related: Option<bool>,
    pub grades: Option<Vec<HadithGrade>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Response for topic-based queries
#[derive(Debug, Serialize)]
pub struct HadithTopicResponse {
    pub topic: String,
    pub hadiths: Vec<HadithWithDetails>,
    pub related_topics: Vec<String>,
    pub total_count: i64,
}

/// Request for Hadith analytics
#[derive(Debug, Deserialize)]
pub struct HadithAnalyticsRequest {
    pub book_ids: Option<Vec<Uuid>>,
    pub analysis_type: AnalysisType,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

/// Type of Hadith analysis
#[derive(Debug, Deserialize)]
pub enum AnalysisType {
    #[serde(rename = "grade_distribution")]
    GradeDistribution,
    #[serde(rename = "theme_analysis")]
    ThemeAnalysis,
    #[serde(rename = "narrator_frequency")]
    NarratorFrequency,
    #[serde(rename = "book_statistics")]
    BookStatistics,
}

/// Hadith analytics response
#[derive(Debug, Serialize)]
pub struct HadithAnalyticsResponse {
    pub analysis_type: String,
    pub data: serde_json::Value,
    pub insights: Vec<String>,
    pub recommendations: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadith_creation() {
        let hadith = Hadith::new(
            "1".to_string(),
            "إنما الأعمال بالنيات".to_string(),
            "عمر بن الخطاب".to_string(),
            "صحيح البخاري".to_string(),
            "كتاب بدء الوحي".to_string(),
            HadithGrade::Sahih,
            "البخاري".to_string(),
            "ar".to_string(),
        );

        assert_eq!(hadith.hadith_number, "1");
        assert_eq!(hadith.grade, HadithGrade::Sahih);
        assert!(hadith.is_authentic());
        assert!(hadith.verify_integrity());
    }

    #[test]
    fn test_hash_generation() {
        let text = "إنما الأعمال بالنيات";
        let hash1 = Hadith::generate_hash(text);
        let hash2 = Hadith::generate_hash(text);
        
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA-256 produces 64-character hex string
    }

    #[test]
    fn test_sanad_creation() {
        let hadith_id = Uuid::new_v4();
        let narrators = vec![
            "عمر بن الخطاب".to_string(),
            "علقمة بن وقاص".to_string(),
        ];
        
        let sanad = Sanad::new(
            hadith_id,
            "حدثنا عمر بن الخطاب".to_string(),
            narrators.clone(),
            ChainGrade::Sahih,
        );

        assert_eq!(sanad.hadith_id, hadith_id);
        assert_eq!(sanad.narrators, narrators);
        assert_eq!(sanad.narrator_count(), 2);
        assert!(sanad.is_continuous());
        assert!(sanad.verify_integrity());
    }

    #[test]
    fn test_scholar_creation() {
        let scholar = Scholar::new(
            "Al-Bukhari".to_string(),
            "الإمام البخاري".to_string(),
            ScholarlyAuthentication::HighlyAuthenticated,
        );

        assert_eq!(scholar.name, "Al-Bukhari");
        assert_eq!(scholar.credibility_score, 5.0);
        assert_eq!(scholar.scholarly_authentication, ScholarlyAuthentication::HighlyAuthenticated);
    }

    #[test]
    fn test_hadith_book_creation() {
        let book = HadithBook::new(
            "Sahih Bukhari".to_string(),
            "صحيح البخاري".to_string(),
            "Al-Bukhari".to_string(),
            "الإمام البخاري".to_string(),
            HadithBookType::Sahih,
            BookAuthenticityLevel::Highest,
            "ar".to_string(),
        );

        assert_eq!(book.name, "Sahih Bukhari");
        assert!(book.is_most_authentic());
        assert_eq!(book.book_type_arabic(), "صحيح");
    }

    #[test]
    fn test_hadith_explanation_creation() {
        let hadith_id = Uuid::new_v4();
        let scholar_id = Uuid::new_v4();
        
        let explanation = HadithExplanation::new(
            hadith_id,
            scholar_id,
            "هذا الحديث يبين أهمية النية في الأعمال".to_string(),
            "ar".to_string(),
        );

        assert_eq!(explanation.hadith_id, hadith_id);
        assert_eq!(explanation.scholar_id, scholar_id);
        assert!(explanation.word_count > 0);
        assert!(explanation.verify_integrity());
        assert!(explanation.estimated_reading_time() > 0.0);
    }
}
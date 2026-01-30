use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use std::str::FromStr;

/// Trait for content integrity verification - ensures Islamic content authenticity
pub trait ContentIntegrity {
    fn verify_integrity(&self) -> bool;
    fn calculate_hash(&self) -> String;
}

/// Trait for serialization and deserialization
pub trait Serializable: Serialize + for<'de> Deserialize<'de> {}

/// Represents an Islamic story with comprehensive metadata
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Story {
    pub id: Uuid,
    pub title: String,
    pub arabic_title: String,
    pub content: String,
    pub content_hash: String, // SHA-256 hash for integrity verification
    pub summary: Option<String>,
    pub category: StoryCategory,
    pub subcategory: Option<String>,
    pub time_period: Option<TimePeriod>,
    pub location: Option<String>,
    pub word_count: i32,
    pub estimated_reading_time: i32, // in minutes
    pub age_group: AgeGroup,
    pub moral_lessons: Vec<String>, // Key moral lessons from the story
    pub themes: Vec<String>, // Thematic tags for categorization
    pub keywords: Vec<String>, // Keywords for search optimization
    pub language: String,
    pub authenticity_level: AuthenticityLevel,
    pub scholarly_verification: ScholarlyVerification,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents a character in Islamic stories
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub arabic_name: String,
    pub character_type: CharacterType,
    pub description: Option<String>,
    pub historical_period: Option<TimePeriod>,
    pub birth_year: Option<i32>, // Hijri year
    pub death_year: Option<i32>, // Hijri year
    pub biography: Option<String>,
    pub virtues: Vec<String>, // Character virtues and qualities
    pub role_significance: Option<String>, // Significance in Islamic history
    pub related_stories_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents the relationship between stories and characters
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoryCharacter {
    pub id: Uuid,
    pub story_id: Uuid,
    pub character_id: Uuid,
    pub role_in_story: CharacterRole,
    pub importance_level: ImportanceLevel,
    pub character_description_in_story: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Represents lessons and morals derived from Islamic stories
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Lesson {
    pub id: Uuid,
    pub title: String,
    pub arabic_title: String,
    pub description: String,
    pub lesson_type: LessonType,
    pub moral_category: MoralCategory,
    pub practical_application: Option<String>,
    pub target_audience: Vec<AgeGroup>,
    pub related_verses: Vec<String>, // Related Quranic verses
    pub related_hadiths: Vec<String>, // Related Hadith references
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents the relationship between stories and lessons
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoryLesson {
    pub id: Uuid,
    pub story_id: Uuid,
    pub lesson_id: Uuid,
    pub relevance_score: f64, // 0.0 to 10.0 scale
    pub explanation: Option<String>, // How this lesson applies to the story
    pub created_at: DateTime<Utc>,
}

/// Represents sources and references for Islamic stories
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StorySource {
    pub id: Uuid,
    pub story_id: Uuid,
    pub source_type: SourceType,
    pub source_name: String,
    pub arabic_source_name: String,
    pub author: Option<String>,
    pub reference: String, // Specific reference (verse, hadith number, page, etc.)
    pub authenticity_grade: Option<String>, // For Hadith sources
    pub credibility_score: f64, // 0.0 to 10.0 scale
    pub verification_status: VerificationStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents collections or series of related stories
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoryCollection {
    pub id: Uuid,
    pub name: String,
    pub arabic_name: String,
    pub description: Option<String>,
    pub collection_type: CollectionType,
    pub story_count: i32,
    pub target_age_group: Option<AgeGroup>,
    pub themes: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents the relationship between stories and collections
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoryCollectionMember {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub story_id: Uuid,
    pub order_in_collection: i32,
    pub added_at: DateTime<Utc>,
}

/// Categories of Islamic stories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq, Hash)]
#[sqlx(type_name = "text")]
pub enum StoryCategory {
    #[serde(rename = "prophets")]
    #[sqlx(rename = "prophets")]
    Prophets, // قصص الأنبياء - Stories of Prophets
    #[serde(rename = "companions")]
    #[sqlx(rename = "companions")]
    Companions, // قصص الصحابة - Stories of Companions
    #[serde(rename = "righteous_predecessors")]
    #[sqlx(rename = "righteous_predecessors")]
    RighteousPredecessors, // قصص السلف الصالح - Stories of Righteous Predecessors
    #[serde(rename = "historical_events")]
    #[sqlx(rename = "historical_events")]
    HistoricalEvents, // الأحداث التاريخية - Historical Events
    #[serde(rename = "moral_lessons")]
    #[sqlx(rename = "moral_lessons")]
    MoralLessons, // العبر والمواعظ - Moral Lessons and Admonitions
    #[serde(rename = "miracles")]
    #[sqlx(rename = "miracles")]
    Miracles, // المعجزات - Miracles and Divine Signs
    #[serde(rename = "battles")]
    #[sqlx(rename = "battles")]
    Battles, // الغزوات والمعارك - Battles and Military Campaigns
    #[serde(rename = "conversions")]
    #[sqlx(rename = "conversions")]
    Conversions, // قصص الإسلام - Conversion Stories
    #[serde(rename = "women_in_islam")]
    #[sqlx(rename = "women_in_islam")]
    WomenInIslam, // نساء في الإسلام - Women in Islam
    #[serde(rename = "children_stories")]
    #[sqlx(rename = "children_stories")]
    ChildrenStories, // قصص الأطفال - Children's Stories
}

impl FromStr for StoryCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "prophets" => Ok(StoryCategory::Prophets),
            "companions" => Ok(StoryCategory::Companions),
            "righteous_predecessors" => Ok(StoryCategory::RighteousPredecessors),
            "historical_events" => Ok(StoryCategory::HistoricalEvents),
            "moral_lessons" => Ok(StoryCategory::MoralLessons),
            "miracles" => Ok(StoryCategory::Miracles),
            "battles" => Ok(StoryCategory::Battles),
            "conversions" => Ok(StoryCategory::Conversions),
            "women_in_islam" => Ok(StoryCategory::WomenInIslam),
            "children_stories" => Ok(StoryCategory::ChildrenStories),
            _ => Err(format!("Invalid story category: {}", s)),
        }
    }
}

/// Time periods in Islamic history
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum TimePeriod {
    #[serde(rename = "pre_islamic")]
    #[sqlx(rename = "pre_islamic")]
    PreIslamic, // ما قبل الإسلام - Pre-Islamic Era
    #[serde(rename = "prophetic_era")]
    #[sqlx(rename = "prophetic_era")]
    PropheticEra, // العهد النبوي - Prophetic Era (610-632 CE)
    #[serde(rename = "rightly_guided_caliphs")]
    #[sqlx(rename = "rightly_guided_caliphs")]
    RightlyGuidedCaliphs, // عهد الخلفاء الراشدين - Era of Rightly-Guided Caliphs (632-661 CE)
    #[serde(rename = "umayyad")]
    #[sqlx(rename = "umayyad")]
    Umayyad, // العهد الأموي - Umayyad Period (661-750 CE)
    #[serde(rename = "abbasid")]
    #[sqlx(rename = "abbasid")]
    Abbasid, // العهد العباسي - Abbasid Period (750-1258 CE)
    #[serde(rename = "ottoman")]
    #[sqlx(rename = "ottoman")]
    Ottoman, // العهد العثماني - Ottoman Period (1299-1922 CE)
    #[serde(rename = "modern")]
    #[sqlx(rename = "modern")]
    Modern, // العصر الحديث - Modern Era (1800-present)
    #[serde(rename = "ancient_prophets")]
    #[sqlx(rename = "ancient_prophets")]
    AncientProphets, // الأنبياء القدماء - Ancient Prophets Era
}

impl FromStr for TimePeriod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pre_islamic" => Ok(TimePeriod::PreIslamic),
            "prophetic_era" => Ok(TimePeriod::PropheticEra),
            "rightly_guided_caliphs" => Ok(TimePeriod::RightlyGuidedCaliphs),
            "umayyad" => Ok(TimePeriod::Umayyad),
            "abbasid" => Ok(TimePeriod::Abbasid),
            "ottoman" => Ok(TimePeriod::Ottoman),
            "modern" => Ok(TimePeriod::Modern),
            "ancient_prophets" => Ok(TimePeriod::AncientProphets),
            _ => Err(format!("Invalid time period: {}", s)),
        }
    }
}

/// Age groups for story targeting
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum AgeGroup {
    #[serde(rename = "children")]
    #[sqlx(rename = "children")]
    Children, // الأطفال - Children (5-12 years)
    #[serde(rename = "teenagers")]
    #[sqlx(rename = "teenagers")]
    Teenagers, // المراهقون - Teenagers (13-18 years)
    #[serde(rename = "young_adults")]
    #[sqlx(rename = "young_adults")]
    YoungAdults, // الشباب - Young Adults (19-35 years)
    #[serde(rename = "adults")]
    #[sqlx(rename = "adults")]
    Adults, // البالغون - Adults (36+ years)
    #[serde(rename = "all_ages")]
    #[sqlx(rename = "all_ages")]
    AllAges, // جميع الأعمار - All Ages
}

impl FromStr for AgeGroup {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "children" => Ok(AgeGroup::Children),
            "teenagers" => Ok(AgeGroup::Teenagers),
            "young_adults" => Ok(AgeGroup::YoungAdults),
            "adults" => Ok(AgeGroup::Adults),
            "all_ages" => Ok(AgeGroup::AllAges),
            _ => Err(format!("Invalid age group: {}", s)),
        }
    }
}

/// Types of characters in Islamic stories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum CharacterType {
    #[serde(rename = "prophet")]
    #[sqlx(rename = "prophet")]
    Prophet, // نبي - Prophet
    #[serde(rename = "messenger")]
    #[sqlx(rename = "messenger")]
    Messenger, // رسول - Messenger
    #[serde(rename = "companion")]
    #[sqlx(rename = "companion")]
    Companion, // صحابي - Companion of the Prophet
    #[serde(rename = "righteous_person")]
    #[sqlx(rename = "righteous_person")]
    RighteousPerson, // صالح - Righteous Person
    #[serde(rename = "scholar")]
    #[sqlx(rename = "scholar")]
    Scholar, // عالم - Islamic Scholar
    #[serde(rename = "ruler")]
    #[sqlx(rename = "ruler")]
    Ruler, // حاكم - Ruler/Leader
    #[serde(rename = "martyr")]
    #[sqlx(rename = "martyr")]
    Martyr, // شهيد - Martyr
    #[serde(rename = "convert")]
    #[sqlx(rename = "convert")]
    Convert, // مسلم جديد - New Muslim/Convert
    #[serde(rename = "historical_figure")]
    #[sqlx(rename = "historical_figure")]
    HistoricalFigure, // شخصية تاريخية - Historical Figure
    #[serde(rename = "antagonist")]
    #[sqlx(rename = "antagonist")]
    Antagonist, // معارض - Antagonist/Opponent
}

impl FromStr for CharacterType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "prophet" => Ok(CharacterType::Prophet),
            "messenger" => Ok(CharacterType::Messenger),
            "companion" => Ok(CharacterType::Companion),
            "righteous_person" => Ok(CharacterType::RighteousPerson),
            "scholar" => Ok(CharacterType::Scholar),
            "ruler" => Ok(CharacterType::Ruler),
            "martyr" => Ok(CharacterType::Martyr),
            "convert" => Ok(CharacterType::Convert),
            "historical_figure" => Ok(CharacterType::HistoricalFigure),
            "antagonist" => Ok(CharacterType::Antagonist),
            _ => Err(format!("Invalid character type: {}", s)),
        }
    }
}

/// Roles of characters within specific stories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum CharacterRole {
    #[serde(rename = "protagonist")]
    #[sqlx(rename = "protagonist")]
    Protagonist, // البطل - Main Character/Hero
    #[serde(rename = "supporting")]
    #[sqlx(rename = "supporting")]
    Supporting, // مساعد - Supporting Character
    #[serde(rename = "mentor")]
    #[sqlx(rename = "mentor")]
    Mentor, // معلم - Mentor/Teacher
    #[serde(rename = "antagonist")]
    #[sqlx(rename = "antagonist")]
    Antagonist, // معارض - Antagonist
    #[serde(rename = "witness")]
    #[sqlx(rename = "witness")]
    Witness, // شاهد - Witness
    #[serde(rename = "narrator")]
    #[sqlx(rename = "narrator")]
    Narrator, // راوي - Narrator
}

impl FromStr for CharacterRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "protagonist" => Ok(CharacterRole::Protagonist),
            "supporting" => Ok(CharacterRole::Supporting),
            "mentor" => Ok(CharacterRole::Mentor),
            "antagonist" => Ok(CharacterRole::Antagonist),
            "witness" => Ok(CharacterRole::Witness),
            "narrator" => Ok(CharacterRole::Narrator),
            _ => Err(format!("Invalid character role: {}", s)),
        }
    }
}

/// Importance levels of characters in stories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum ImportanceLevel {
    #[serde(rename = "primary")]
    #[sqlx(rename = "primary")]
    Primary, // أساسي - Primary Character
    #[serde(rename = "secondary")]
    #[sqlx(rename = "secondary")]
    Secondary, // ثانوي - Secondary Character
    #[serde(rename = "minor")]
    #[sqlx(rename = "minor")]
    Minor, // فرعي - Minor Character
}

impl FromStr for ImportanceLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "primary" => Ok(ImportanceLevel::Primary),
            "secondary" => Ok(ImportanceLevel::Secondary),
            "minor" => Ok(ImportanceLevel::Minor),
            _ => Err(format!("Invalid importance level: {}", s)),
        }
    }
}

/// Types of lessons derived from stories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum LessonType {
    #[serde(rename = "moral")]
    #[sqlx(rename = "moral")]
    Moral, // أخلاقي - Moral Lesson
    #[serde(rename = "spiritual")]
    #[sqlx(rename = "spiritual")]
    Spiritual, // روحي - Spiritual Lesson
    #[serde(rename = "practical")]
    #[sqlx(rename = "practical")]
    Practical, // عملي - Practical Lesson
    #[serde(rename = "historical")]
    #[sqlx(rename = "historical")]
    Historical, // تاريخي - Historical Lesson
    #[serde(rename = "theological")]
    #[sqlx(rename = "theological")]
    Theological, // عقدي - Theological Lesson
    #[serde(rename = "social")]
    #[sqlx(rename = "social")]
    Social, // اجتماعي - Social Lesson
}

impl FromStr for LessonType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "moral" => Ok(LessonType::Moral),
            "spiritual" => Ok(LessonType::Spiritual),
            "practical" => Ok(LessonType::Practical),
            "historical" => Ok(LessonType::Historical),
            "theological" => Ok(LessonType::Theological),
            "social" => Ok(LessonType::Social),
            _ => Err(format!("Invalid lesson type: {}", s)),
        }
    }
}

/// Categories of moral lessons
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum MoralCategory {
    #[serde(rename = "patience")]
    #[sqlx(rename = "patience")]
    Patience, // الصبر - Patience
    #[serde(rename = "gratitude")]
    #[sqlx(rename = "gratitude")]
    Gratitude, // الشكر - Gratitude
    #[serde(rename = "justice")]
    #[sqlx(rename = "justice")]
    Justice, // العدل - Justice
    #[serde(rename = "mercy")]
    #[sqlx(rename = "mercy")]
    Mercy, // الرحمة - Mercy
    #[serde(rename = "honesty")]
    #[sqlx(rename = "honesty")]
    Honesty, // الصدق - Honesty
    #[serde(rename = "courage")]
    #[sqlx(rename = "courage")]
    Courage, // الشجاعة - Courage
    #[serde(rename = "humility")]
    #[sqlx(rename = "humility")]
    Humility, // التواضع - Humility
    #[serde(rename = "forgiveness")]
    #[sqlx(rename = "forgiveness")]
    Forgiveness, // المغفرة - Forgiveness
    #[serde(rename = "perseverance")]
    #[sqlx(rename = "perseverance")]
    Perseverance, // المثابرة - Perseverance
    #[serde(rename = "faith")]
    #[sqlx(rename = "faith")]
    Faith, // الإيمان - Faith
}

impl FromStr for MoralCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "patience" => Ok(MoralCategory::Patience),
            "gratitude" => Ok(MoralCategory::Gratitude),
            "justice" => Ok(MoralCategory::Justice),
            "mercy" => Ok(MoralCategory::Mercy),
            "honesty" => Ok(MoralCategory::Honesty),
            "courage" => Ok(MoralCategory::Courage),
            "humility" => Ok(MoralCategory::Humility),
            "forgiveness" => Ok(MoralCategory::Forgiveness),
            "perseverance" => Ok(MoralCategory::Perseverance),
            "faith" => Ok(MoralCategory::Faith),
            _ => Err(format!("Invalid moral category: {}", s)),
        }
    }
}

/// Types of sources for Islamic stories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum SourceType {
    #[serde(rename = "quran")]
    #[sqlx(rename = "quran")]
    Quran, // القرآن الكريم - Holy Quran
    #[serde(rename = "hadith")]
    #[sqlx(rename = "hadith")]
    Hadith, // الحديث النبوي - Prophetic Hadith
    #[serde(rename = "historical_book")]
    #[sqlx(rename = "historical_book")]
    HistoricalBook, // كتاب تاريخي - Historical Book
    #[serde(rename = "biography")]
    #[sqlx(rename = "biography")]
    Biography, // سيرة - Biography
    #[serde(rename = "tafsir")]
    #[sqlx(rename = "tafsir")]
    Tafsir, // تفسير - Quranic Commentary
    #[serde(rename = "scholarly_work")]
    #[sqlx(rename = "scholarly_work")]
    ScholarlyWork, // عمل علمي - Scholarly Work
}

impl FromStr for SourceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "quran" => Ok(SourceType::Quran),
            "hadith" => Ok(SourceType::Hadith),
            "historical_book" => Ok(SourceType::HistoricalBook),
            "biography" => Ok(SourceType::Biography),
            "tafsir" => Ok(SourceType::Tafsir),
            "scholarly_work" => Ok(SourceType::ScholarlyWork),
            _ => Err(format!("Invalid source type: {}", s)),
        }
    }
}

/// Authenticity levels for Islamic stories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum AuthenticityLevel {
    #[serde(rename = "authentic")]
    #[sqlx(rename = "authentic")]
    Authentic, // صحيح - Authentic (from Quran/Sahih Hadith)
    #[serde(rename = "well_documented")]
    #[sqlx(rename = "well_documented")]
    WellDocumented, // موثق جيداً - Well-documented historical account
    #[serde(rename = "probable")]
    #[sqlx(rename = "probable")]
    Probable, // محتمل - Probable based on multiple sources
    #[serde(rename = "traditional")]
    #[sqlx(rename = "traditional")]
    Traditional, // تراثي - Traditional account (needs verification)
    #[serde(rename = "educational")]
    #[sqlx(rename = "educational")]
    Educational, // تعليمي - Educational story with moral lessons
}

impl FromStr for AuthenticityLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "authentic" => Ok(AuthenticityLevel::Authentic),
            "well_documented" => Ok(AuthenticityLevel::WellDocumented),
            "probable" => Ok(AuthenticityLevel::Probable),
            "traditional" => Ok(AuthenticityLevel::Traditional),
            "educational" => Ok(AuthenticityLevel::Educational),
            _ => Err(format!("Invalid authenticity level: {}", s)),
        }
    }
}

/// Scholarly verification status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum ScholarlyVerification {
    #[serde(rename = "verified")]
    #[sqlx(rename = "verified")]
    Verified, // تم التحقق - Verified by scholars
    #[serde(rename = "under_review")]
    #[sqlx(rename = "under_review")]
    UnderReview, // قيد المراجعة - Under scholarly review
    #[serde(rename = "pending")]
    #[sqlx(rename = "pending")]
    Pending, // في الانتظار - Pending verification
    #[serde(rename = "disputed")]
    #[sqlx(rename = "disputed")]
    Disputed, // محل خلاف - Disputed among scholars
}

impl FromStr for ScholarlyVerification {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "verified" => Ok(ScholarlyVerification::Verified),
            "under_review" => Ok(ScholarlyVerification::UnderReview),
            "pending" => Ok(ScholarlyVerification::Pending),
            "disputed" => Ok(ScholarlyVerification::Disputed),
            _ => Err(format!("Invalid scholarly verification: {}", s)),
        }
    }
}

/// Verification status for sources
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum VerificationStatus {
    #[serde(rename = "verified")]
    #[sqlx(rename = "verified")]
    Verified, // تم التحقق - Verified
    #[serde(rename = "unverified")]
    #[sqlx(rename = "unverified")]
    Unverified, // غير محقق - Unverified
    #[serde(rename = "questionable")]
    #[sqlx(rename = "questionable")]
    Questionable, // مشكوك فيه - Questionable
}

impl FromStr for VerificationStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "verified" => Ok(VerificationStatus::Verified),
            "unverified" => Ok(VerificationStatus::Unverified),
            "questionable" => Ok(VerificationStatus::Questionable),
            _ => Err(format!("Invalid verification status: {}", s)),
        }
    }
}

/// Types of story collections
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "text")]
pub enum CollectionType {
    #[serde(rename = "thematic")]
    #[sqlx(rename = "thematic")]
    Thematic, // موضوعي - Thematic collection
    #[serde(rename = "chronological")]
    #[sqlx(rename = "chronological")]
    Chronological, // زمني - Chronological collection
    #[serde(rename = "character_based")]
    #[sqlx(rename = "character_based")]
    CharacterBased, // شخصي - Character-based collection
    #[serde(rename = "age_specific")]
    #[sqlx(rename = "age_specific")]
    AgeSpecific, // عمري - Age-specific collection
    #[serde(rename = "educational")]
    #[sqlx(rename = "educational")]
    Educational, // تعليمي - Educational collection
}

impl FromStr for CollectionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "thematic" => Ok(CollectionType::Thematic),
            "chronological" => Ok(CollectionType::Chronological),
            "character_based" => Ok(CollectionType::CharacterBased),
            "age_specific" => Ok(CollectionType::AgeSpecific),
            "educational" => Ok(CollectionType::Educational),
            _ => Err(format!("Invalid collection type: {}", s)),
        }
    }
}

impl Story {
    /// Create a new Story with automatic hash generation and metadata calculation
    pub fn new(
        title: String,
        arabic_title: String,
        content: String,
        category: StoryCategory,
        age_group: AgeGroup,
        language: String,
        authenticity_level: AuthenticityLevel,
    ) -> Self {
        let content_hash = Self::generate_hash(&content);
        let word_count = content.split_whitespace().count() as i32;
        let estimated_reading_time = Self::calculate_reading_time(word_count);
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            title,
            arabic_title,
            content,
            content_hash,
            summary: None,
            category,
            subcategory: None,
            time_period: None,
            location: None,
            word_count,
            estimated_reading_time,
            age_group,
            moral_lessons: Vec::new(),
            themes: Vec::new(),
            keywords: Vec::new(),
            language,
            authenticity_level,
            scholarly_verification: ScholarlyVerification::Pending,
            created_at: now,
            updated_at: now,
        }
    }

    /// Generate SHA-256 hash for content integrity verification
    pub fn generate_hash(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Calculate estimated reading time in minutes (assuming 200 words per minute for Arabic)
    pub fn calculate_reading_time(word_count: i32) -> i32 {
        if word_count == 0 {
            return 0;
        }
        ((word_count as f32 / 200.0).ceil() as i32).max(1)
    }

    /// Verify content integrity using stored hash
    pub fn verify_integrity(&self) -> bool {
        Self::generate_hash(&self.content) == self.content_hash
    }

    /// Update word count and reading time
    pub fn update_metrics(&mut self) {
        self.word_count = self.content.split_whitespace().count() as i32;
        self.estimated_reading_time = Self::calculate_reading_time(self.word_count);
        self.updated_at = Utc::now();
    }

    /// Add a moral lesson to the story
    pub fn add_moral_lesson(&mut self, lesson: String) {
        if !self.moral_lessons.contains(&lesson) {
            self.moral_lessons.push(lesson);
            self.updated_at = Utc::now();
        }
    }

    /// Add a theme to the story
    pub fn add_theme(&mut self, theme: String) {
        if !self.themes.contains(&theme) {
            self.themes.push(theme);
            self.updated_at = Utc::now();
        }
    }

    /// Add a keyword for search optimization
    pub fn add_keyword(&mut self, keyword: String) {
        if !self.keywords.contains(&keyword) {
            self.keywords.push(keyword);
            self.updated_at = Utc::now();
        }
    }

    /// Check if the story is suitable for children
    pub fn is_suitable_for_children(&self) -> bool {
        matches!(self.age_group, AgeGroup::Children | AgeGroup::AllAges)
    }

    /// Check if the story is historically authentic
    pub fn is_historically_authentic(&self) -> bool {
        matches!(
            self.authenticity_level,
            AuthenticityLevel::Authentic | AuthenticityLevel::WellDocumented
        )
    }

    /// Get Arabic name for the category
    pub fn category_arabic(&self) -> &'static str {
        match self.category {
            StoryCategory::Prophets => "قصص الأنبياء",
            StoryCategory::Companions => "قصص الصحابة",
            StoryCategory::RighteousPredecessors => "قصص السلف الصالح",
            StoryCategory::HistoricalEvents => "الأحداث التاريخية",
            StoryCategory::MoralLessons => "العبر والمواعظ",
            StoryCategory::Miracles => "المعجزات",
            StoryCategory::Battles => "الغزوات والمعارك",
            StoryCategory::Conversions => "قصص الإسلام",
            StoryCategory::WomenInIslam => "نساء في الإسلام",
            StoryCategory::ChildrenStories => "قصص الأطفال",
        }
    }

    /// Get reading difficulty level based on word count and age group
    pub fn get_difficulty_level(&self) -> String {
        match (&self.age_group, self.word_count) {
            (AgeGroup::Children, 0..=500) => "Easy".to_string(),
            (AgeGroup::Children, 501..=1000) => "Moderate".to_string(),
            (AgeGroup::Children, _) => "Challenging".to_string(),
            (AgeGroup::Teenagers, 0..=1000) => "Easy".to_string(),
            (AgeGroup::Teenagers, 1001..=2000) => "Moderate".to_string(),
            (AgeGroup::Teenagers, _) => "Challenging".to_string(),
            (_, 0..=1500) => "Easy".to_string(),
            (_, 1501..=3000) => "Moderate".to_string(),
            (_, _) => "Challenging".to_string(),
        }
    }
}

impl Character {
    /// Create a new Character
    pub fn new(
        name: String,
        arabic_name: String,
        character_type: CharacterType,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            name,
            arabic_name,
            character_type,
            description: None,
            historical_period: None,
            birth_year: None,
            death_year: None,
            biography: None,
            virtues: Vec::new(),
            role_significance: None,
            related_stories_count: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Add a virtue to the character
    pub fn add_virtue(&mut self, virtue: String) {
        if !self.virtues.contains(&virtue) {
            self.virtues.push(virtue);
            self.updated_at = Utc::now();
        }
    }

    /// Check if the character is a Prophet or Messenger
    pub fn is_prophet(&self) -> bool {
        matches!(self.character_type, CharacterType::Prophet | CharacterType::Messenger)
    }

    /// Check if the character is from the early Islamic period
    pub fn is_early_islamic(&self) -> bool {
        matches!(
            self.historical_period,
            Some(TimePeriod::PropheticEra) | Some(TimePeriod::RightlyGuidedCaliphs)
        )
    }

    /// Get the character's lifespan if both birth and death years are known
    pub fn get_lifespan(&self) -> Option<i32> {
        match (self.birth_year, self.death_year) {
            (Some(birth), Some(death)) => Some(death - birth),
            _ => None,
        }
    }

    /// Get Arabic name for the character type
    pub fn character_type_arabic(&self) -> &'static str {
        match self.character_type {
            CharacterType::Prophet => "نبي",
            CharacterType::Messenger => "رسول",
            CharacterType::Companion => "صحابي",
            CharacterType::RighteousPerson => "صالح",
            CharacterType::Scholar => "عالم",
            CharacterType::Ruler => "حاكم",
            CharacterType::Martyr => "شهيد",
            CharacterType::Convert => "مسلم جديد",
            CharacterType::HistoricalFigure => "شخصية تاريخية",
            CharacterType::Antagonist => "معارض",
        }
    }
}

impl Lesson {
    /// Create a new Lesson
    pub fn new(
        title: String,
        arabic_title: String,
        description: String,
        lesson_type: LessonType,
        moral_category: MoralCategory,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            title,
            arabic_title,
            description,
            lesson_type,
            moral_category,
            practical_application: None,
            target_audience: vec![AgeGroup::AllAges],
            related_verses: Vec::new(),
            related_hadiths: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Add a related Quranic verse
    pub fn add_related_verse(&mut self, verse_ref: String) {
        if !self.related_verses.contains(&verse_ref) {
            self.related_verses.push(verse_ref);
            self.updated_at = Utc::now();
        }
    }

    /// Add a related Hadith reference
    pub fn add_related_hadith(&mut self, hadith_ref: String) {
        if !self.related_hadiths.contains(&hadith_ref) {
            self.related_hadiths.push(hadith_ref);
            self.updated_at = Utc::now();
        }
    }

    /// Check if the lesson is suitable for a specific age group
    pub fn is_suitable_for_age(&self, age_group: &AgeGroup) -> bool {
        self.target_audience.contains(age_group) || self.target_audience.contains(&AgeGroup::AllAges)
    }

    /// Get Arabic name for the lesson type
    pub fn lesson_type_arabic(&self) -> &'static str {
        match self.lesson_type {
            LessonType::Moral => "أخلاقي",
            LessonType::Spiritual => "روحي",
            LessonType::Practical => "عملي",
            LessonType::Historical => "تاريخي",
            LessonType::Theological => "عقدي",
            LessonType::Social => "اجتماعي",
        }
    }

    /// Get Arabic name for the moral category
    pub fn moral_category_arabic(&self) -> &'static str {
        match self.moral_category {
            MoralCategory::Patience => "الصبر",
            MoralCategory::Gratitude => "الشكر",
            MoralCategory::Justice => "العدل",
            MoralCategory::Mercy => "الرحمة",
            MoralCategory::Honesty => "الصدق",
            MoralCategory::Courage => "الشجاعة",
            MoralCategory::Humility => "التواضع",
            MoralCategory::Forgiveness => "المغفرة",
            MoralCategory::Perseverance => "المثابرة",
            MoralCategory::Faith => "الإيمان",
        }
    }
}

impl StorySource {
    /// Create a new StorySource
    pub fn new(
        story_id: Uuid,
        source_type: SourceType,
        source_name: String,
        arabic_source_name: String,
        reference: String,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            story_id,
            source_type,
            source_name,
            arabic_source_name,
            author: None,
            reference,
            authenticity_grade: None,
            credibility_score: 5.0, // Default middle score
            verification_status: VerificationStatus::Unverified,
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if the source is from primary Islamic texts
    pub fn is_primary_source(&self) -> bool {
        matches!(self.source_type, SourceType::Quran | SourceType::Hadith)
    }

    /// Check if the source is highly credible
    pub fn is_highly_credible(&self) -> bool {
        self.credibility_score >= 8.0 && self.verification_status == VerificationStatus::Verified
    }

    /// Get Arabic name for the source type
    pub fn source_type_arabic(&self) -> &'static str {
        match self.source_type {
            SourceType::Quran => "القرآن الكريم",
            SourceType::Hadith => "الحديث النبوي",
            SourceType::HistoricalBook => "كتاب تاريخي",
            SourceType::Biography => "سيرة",
            SourceType::Tafsir => "تفسير",
            SourceType::ScholarlyWork => "عمل علمي",
        }
    }
}

impl StoryCollection {
    /// Create a new StoryCollection
    pub fn new(
        name: String,
        arabic_name: String,
        collection_type: CollectionType,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            name,
            arabic_name,
            description: None,
            collection_type,
            story_count: 0,
            target_age_group: None,
            themes: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Add a theme to the collection
    pub fn add_theme(&mut self, theme: String) {
        if !self.themes.contains(&theme) {
            self.themes.push(theme);
            self.updated_at = Utc::now();
        }
    }

    /// Get Arabic name for the collection type
    pub fn collection_type_arabic(&self) -> &'static str {
        match self.collection_type {
            CollectionType::Thematic => "موضوعي",
            CollectionType::Chronological => "زمني",
            CollectionType::CharacterBased => "شخصي",
            CollectionType::AgeSpecific => "عمري",
            CollectionType::Educational => "تعليمي",
        }
    }
}

impl MoralCategory {
    pub fn arabic_name(&self) -> &'static str {
        match self {
            MoralCategory::Patience => "الصبر",
            MoralCategory::Gratitude => "الشكر",
            MoralCategory::Justice => "العدل",
            MoralCategory::Mercy => "الرحمة",
            MoralCategory::Honesty => "الصدق",
            MoralCategory::Courage => "الشجاعة",
            MoralCategory::Humility => "التواضع",
            MoralCategory::Forgiveness => "المغفرة",
            MoralCategory::Perseverance => "المثابرة",
            MoralCategory::Faith => "الإيمان",
        }
    }
}

// Display implementations for better debugging and logging
impl std::fmt::Display for StoryCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", format!("{:?}", self), self.arabic_name())
    }
}

impl StoryCategory {
    pub fn arabic_name(&self) -> &'static str {
        match self {
            StoryCategory::Prophets => "قصص الأنبياء",
            StoryCategory::Companions => "قصص الصحابة",
            StoryCategory::RighteousPredecessors => "قصص السلف الصالح",
            StoryCategory::HistoricalEvents => "الأحداث التاريخية",
            StoryCategory::MoralLessons => "العبر والمواعظ",
            StoryCategory::Miracles => "المعجزات",
            StoryCategory::Battles => "الغزوات والمعارك",
            StoryCategory::Conversions => "قصص الإسلام",
            StoryCategory::WomenInIslam => "نساء في الإسلام",
            StoryCategory::ChildrenStories => "قصص الأطفال",
        }
    }
}

impl std::fmt::Display for CharacterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", format!("{:?}", self), self.arabic_name())
    }
}

impl CharacterType {
    pub fn arabic_name(&self) -> &'static str {
        match self {
            CharacterType::Prophet => "نبي",
            CharacterType::Messenger => "رسول",
            CharacterType::Companion => "صحابي",
            CharacterType::RighteousPerson => "صالح",
            CharacterType::Scholar => "عالم",
            CharacterType::Ruler => "حاكم",
            CharacterType::Martyr => "شهيد",
            CharacterType::Convert => "مسلم جديد",
            CharacterType::HistoricalFigure => "شخصية تاريخية",
            CharacterType::Antagonist => "معارض",
        }
    }
}

// Implement ContentIntegrity trait for main structs
impl ContentIntegrity for Story {
    fn verify_integrity(&self) -> bool {
        self.verify_integrity()
    }

    fn calculate_hash(&self) -> String {
        Self::generate_hash(&self.content)
    }
}

// Implement Serializable trait for all main structs
impl Serializable for Story {}
impl Serializable for Character {}
impl Serializable for StoryCharacter {}
impl Serializable for Lesson {}
impl Serializable for StoryLesson {}
impl Serializable for StorySource {}
impl Serializable for StoryCollection {}
impl Serializable for StoryCollectionMember {}

/// Request/Response models for API endpoints

/// Complete story with all related information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryWithDetails {
    pub story: Story,
    pub characters: Vec<CharacterInStory>,
    pub lessons: Vec<LessonInStory>,
    pub sources: Vec<StorySource>,
    pub collections: Vec<StoryCollection>,
}

/// Character with their role in a specific story
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterInStory {
    pub character: Character,
    pub role_in_story: CharacterRole,
    pub importance_level: ImportanceLevel,
    pub character_description_in_story: Option<String>,
}

/// Lesson with its relevance to a specific story
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonInStory {
    pub lesson: Lesson,
    pub relevance_score: f64,
    pub explanation: Option<String>,
}

/// Request to get a specific story
#[derive(Debug, Deserialize)]
pub struct GetStoryRequest {
    pub story_id: Option<Uuid>,
    pub title: Option<String>,
    pub include_characters: Option<bool>,
    pub include_lessons: Option<bool>,
    pub include_sources: Option<bool>,
}

/// Request to search stories
#[derive(Debug, Deserialize)]
pub struct SearchStoriesRequest {
    pub query: String,
    pub categories: Option<Vec<StoryCategory>>,
    pub age_groups: Option<Vec<AgeGroup>>,
    pub time_periods: Option<Vec<TimePeriod>>,
    pub authenticity_levels: Option<Vec<AuthenticityLevel>>,
    pub character_names: Option<Vec<String>>,
    pub themes: Option<Vec<String>>,
    pub search_type: Option<SearchType>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Types of search supported for stories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SearchType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "semantic")]
    Semantic,
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "theme")]
    Theme,
    #[serde(rename = "lesson")]
    Lesson,
    #[serde(rename = "exact")]
    Exact,
}

/// Search result for story content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorySearchResult {
    pub story: Story,
    pub characters: Vec<Character>,
    pub main_lessons: Vec<String>,
    pub relevance_score: f64,
    pub highlighted_text: String,
    pub matching_criteria: Vec<String>,
}

/// Response for story queries
#[derive(Debug, Serialize)]
pub struct StoryResponse {
    pub story: Story,
    pub characters: Option<Vec<CharacterInStory>>,
    pub lessons: Option<Vec<LessonInStory>>,
    pub sources: Option<Vec<StorySource>>,
    pub collections: Option<Vec<StoryCollection>>,
}

/// Response for search queries
#[derive(Debug, Serialize)]
pub struct StorySearchResponse {
    pub results: Vec<StorySearchResult>,
    pub total_count: i64,
    pub query: String,
    pub search_type: SearchType,
    pub search_time_ms: u64,
    pub facets: Option<SearchFacets>,
}

/// Search facets for filtering results
#[derive(Debug, Serialize)]
pub struct SearchFacets {
    pub categories: Vec<FacetCount>,
    pub age_groups: Vec<FacetCount>,
    pub time_periods: Vec<FacetCount>,
    pub authenticity_levels: Vec<FacetCount>,
    pub characters: Vec<FacetCount>,
    pub themes: Vec<FacetCount>,
}

/// Count for a specific facet value
#[derive(Debug, Serialize)]
pub struct FacetCount {
    pub value: String,
    pub count: i64,
}

/// Request to get stories by character
#[derive(Debug, Deserialize)]
pub struct GetStoriesByCharacterRequest {
    pub character_name: String,
    pub character_type: Option<CharacterType>,
    pub include_related: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Response for character-based queries
#[derive(Debug, Serialize)]
pub struct CharacterStoriesResponse {
    pub character: Character,
    pub stories: Vec<StoryWithDetails>,
    pub related_characters: Vec<Character>,
    pub total_count: i64,
}

/// Request to get stories by theme/lesson
#[derive(Debug, Deserialize)]
pub struct GetStoriesByThemeRequest {
    pub theme: String,
    pub lesson_type: Option<LessonType>,
    pub moral_category: Option<MoralCategory>,
    pub age_group: Option<AgeGroup>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Response for theme-based queries
#[derive(Debug, Serialize)]
pub struct ThemeStoriesResponse {
    pub theme: String,
    pub stories: Vec<StoryWithDetails>,
    pub related_themes: Vec<String>,
    pub related_lessons: Vec<Lesson>,
    pub total_count: i64,
}

/// Request for story analytics
#[derive(Debug, Deserialize)]
pub struct StoryAnalyticsRequest {
    pub categories: Option<Vec<StoryCategory>>,
    pub analysis_type: AnalysisType,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

/// Type of story analysis
#[derive(Debug, Deserialize)]
pub enum AnalysisType {
    #[serde(rename = "category_distribution")]
    CategoryDistribution,
    #[serde(rename = "character_frequency")]
    CharacterFrequency,
    #[serde(rename = "theme_analysis")]
    ThemeAnalysis,
    #[serde(rename = "authenticity_levels")]
    AuthenticityLevels,
    #[serde(rename = "age_group_suitability")]
    AgeGroupSuitability,
}

/// Story analytics response
#[derive(Debug, Serialize)]
pub struct StoryAnalyticsResponse {
    pub analysis_type: String,
    pub data: serde_json::Value,
    pub insights: Vec<String>,
    pub recommendations: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

/// Request to search characters
#[derive(Debug, Deserialize)]
pub struct SearchCharactersParams {
    pub query: String,
    pub character_type: Option<CharacterType>,
    pub historical_period: Option<TimePeriod>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_story_creation() {
        let story = Story::new(
            "The Story of Prophet Yusuf".to_string(),
            "قصة النبي يوسف".to_string(),
            "This is the story of Prophet Yusuf...".to_string(),
            StoryCategory::Prophets,
            AgeGroup::AllAges,
            "en".to_string(),
            AuthenticityLevel::Authentic,
        );

        assert_eq!(story.category, StoryCategory::Prophets);
        assert_eq!(story.age_group, AgeGroup::AllAges);
        assert!(story.verify_integrity());
        assert!(story.word_count > 0);
        assert!(story.estimated_reading_time > 0);
        assert!(story.is_historically_authentic());
    }

    #[test]
    fn test_story_integrity_verification() {
        let mut story = Story::new(
            "Test Story".to_string(),
            "قصة تجريبية".to_string(),
            "Original content".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::Children,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );

        assert!(story.verify_integrity());

        // Tamper with content
        story.content = "Modified content".to_string();
        assert!(!story.verify_integrity());
    }

    #[test]
    fn test_character_creation() {
        let character = Character::new(
            "Prophet Muhammad".to_string(),
            "النبي محمد".to_string(),
            CharacterType::Prophet,
        );

        assert_eq!(character.character_type, CharacterType::Prophet);
        assert!(character.is_prophet());
        assert_eq!(character.character_type_arabic(), "نبي");
    }

    #[test]
    fn test_lesson_creation() {
        let lesson = Lesson::new(
            "Patience in Adversity".to_string(),
            "الصبر في المحن".to_string(),
            "This lesson teaches about patience...".to_string(),
            LessonType::Moral,
            MoralCategory::Patience,
        );

        assert_eq!(lesson.lesson_type, LessonType::Moral);
        assert_eq!(lesson.moral_category, MoralCategory::Patience);
        assert!(lesson.is_suitable_for_age(&AgeGroup::AllAges));
        assert_eq!(lesson.moral_category_arabic(), "الصبر");
    }

    #[test]
    fn test_story_source_creation() {
        let story_id = Uuid::new_v4();
        let source = StorySource::new(
            story_id,
            SourceType::Quran,
            "Holy Quran".to_string(),
            "القرآن الكريم".to_string(),
            "Surah Yusuf (12:1-111)".to_string(),
        );

        assert_eq!(source.story_id, story_id);
        assert_eq!(source.source_type, SourceType::Quran);
        assert!(source.is_primary_source());
        assert_eq!(source.source_type_arabic(), "القرآن الكريم");
    }

    #[test]
    fn test_story_collection_creation() {
        let collection = StoryCollection::new(
            "Stories of the Prophets".to_string(),
            "قصص الأنبياء".to_string(),
            CollectionType::Thematic,
        );

        assert_eq!(collection.collection_type, CollectionType::Thematic);
        assert_eq!(collection.collection_type_arabic(), "موضوعي");
    }

    #[test]
    fn test_story_metrics_update() {
        let mut story = Story::new(
            "Test Story".to_string(),
            "قصة تجريبية".to_string(),
            "Short content".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::Children,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );

        let original_word_count = story.word_count;
        story.content = "This is a much longer content with many more words to test the update functionality".to_string();
        story.update_metrics();

        assert!(story.word_count > original_word_count);
        assert!(story.estimated_reading_time > 0);
    }

    #[test]
    fn test_reading_time_calculation() {
        assert_eq!(Story::calculate_reading_time(0), 0);
        assert_eq!(Story::calculate_reading_time(100), 1);
        assert_eq!(Story::calculate_reading_time(200), 1);
        assert_eq!(Story::calculate_reading_time(300), 2);
        assert_eq!(Story::calculate_reading_time(400), 2);
    }

    #[test]
    fn test_story_difficulty_level() {
        let children_easy = Story::new(
            "Easy Story".to_string(),
            "قصة سهلة".to_string(),
            "Short ".repeat(100), // 200 words
            StoryCategory::ChildrenStories,
            AgeGroup::Children,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );

        assert_eq!(children_easy.get_difficulty_level(), "Easy");

        let adult_challenging = Story::new(
            "Complex Story".to_string(),
            "قصة معقدة".to_string(),
            "Complex ".repeat(2000), // 4000 words
            StoryCategory::HistoricalEvents,
            AgeGroup::Adults,
            "en".to_string(),
            AuthenticityLevel::WellDocumented,
        );

        assert_eq!(adult_challenging.get_difficulty_level(), "Challenging");
    }

    #[test]
    fn test_character_lifespan() {
        let mut character = Character::new(
            "Test Character".to_string(),
            "شخصية تجريبية".to_string(),
            CharacterType::Scholar,
        );

        assert_eq!(character.get_lifespan(), None);

        character.birth_year = Some(150);
        character.death_year = Some(200);
        assert_eq!(character.get_lifespan(), Some(50));
    }

    #[test]
    fn test_hash_consistency() {
        let content = "This is test content for hash verification";
        let hash1 = Story::generate_hash(content);
        let hash2 = Story::generate_hash(content);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA-256 produces 64-character hex string
    }

    #[test]
    fn test_story_methods() {
        let mut story = Story::new(
            "Test Story".to_string(),
            "قصة تجريبية".to_string(),
            "Test content".to_string(),
            StoryCategory::ChildrenStories,
            AgeGroup::Children,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );

        assert!(story.is_suitable_for_children());

        story.add_moral_lesson("Honesty".to_string());
        story.add_theme("Virtue".to_string());
        story.add_keyword("moral".to_string());

        assert!(story.moral_lessons.contains(&"Honesty".to_string()));
        assert!(story.themes.contains(&"Virtue".to_string()));
        assert!(story.keywords.contains(&"moral".to_string()));

        // Test duplicate prevention
        story.add_moral_lesson("Honesty".to_string());
        assert_eq!(story.moral_lessons.len(), 1);
    }

    #[test]
    fn test_serialization() {
        let story = Story::new(
            "Test Story".to_string(),
            "قصة تجريبية".to_string(),
            "Test content".to_string(),
            StoryCategory::Prophets,
            AgeGroup::AllAges,
            "en".to_string(),
            AuthenticityLevel::Authentic,
        );

        let json = serde_json::to_string(&story).unwrap();
        let deserialized: Story = serde_json::from_str(&json).unwrap();

        assert_eq!(story.title, deserialized.title);
        assert_eq!(story.category, deserialized.category);
        assert_eq!(story.authenticity_level, deserialized.authenticity_level);
    }
}
pub mod rag_system;
pub mod question_processor;
pub mod semantic_search;
pub mod hadith_verifier;
pub mod source_scorer;
pub mod anti_hallucination;
pub mod context_builder;
pub mod hugging_face_client;
pub mod vector_database;
pub mod integration_service;
pub mod config;
pub mod service_manager;
pub mod error_handler;

#[cfg(test)]
pub mod tests;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Core types for the AI service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicSource {
    pub id: String,
    pub content_type: SourceType,
    pub text: String,
    pub reference: String,
    pub author: Option<String>,
    pub authenticity: AuthenticityLevel,
    pub language: Language,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    Quran,
    SahihHadith,
    HasanHadith,
    DaifHadith,
    MawduHadith,
    Tafsir,
    FiqhRuling,
    ScholarOpinion,
    IslamicStory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticityLevel {
    Verified,      // موثق ومتحقق منه
    Reliable,      // موثوق
    Questionable,  // مشكوك فيه
    Unreliable,    // غير موثوق
    Unknown,       // غير معروف
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Arabic,
    English,
    French,
    Urdu,
    Turkish,
    Indonesian,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    Aqeedah,      // عقيدة
    Fiqh,         // فقه
    Tafsir,       // تفسير
    Hadith,       // حديث
    Sirah,        // سيرة
    Akhlaq,       // أخلاق
    Dua,          // دعاء
    General,      // عام
    OutOfScope,   // خارج النطاق
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Simple,       // بسيط
    Intermediate, // متوسط
    Advanced,     // متقدم
    Scholarly,    // علمي متخصص
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    VeryHigh,   // ثقة عالية جداً (> 0.9)
    High,       // ثقة عالية (0.7 - 0.9)
    Medium,     // ثقة متوسطة (0.5 - 0.7)
    Low,        // ثقة منخفضة (0.3 - 0.5)
    VeryLow,    // ثقة منخفضة جداً (< 0.3)
}

impl ConfidenceLevel {
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s > 0.9 => ConfidenceLevel::VeryHigh,
            s if s > 0.7 => ConfidenceLevel::High,
            s if s > 0.5 => ConfidenceLevel::Medium,
            s if s > 0.3 => ConfidenceLevel::Low,
            _ => ConfidenceLevel::VeryLow,
        }
    }
    
    pub fn to_score(&self) -> f32 {
        match self {
            ConfidenceLevel::VeryHigh => 0.95,
            ConfidenceLevel::High => 0.8,
            ConfidenceLevel::Medium => 0.6,
            ConfidenceLevel::Low => 0.4,
            ConfidenceLevel::VeryLow => 0.2,
        }
    }
}

/// RAG Response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGResponse {
    pub answer: String,
    pub confidence: f32,
    pub retrieved_sources: Vec<IslamicSource>,
    pub cited_sources: Vec<IslamicSource>,
    pub citations: Vec<Citation>,
    pub related_questions: Vec<String>,
    pub warnings: Vec<String>,
    pub hallucination_risk: f32,
    pub response_time_ms: u64,
    pub metadata: HashMap<String, String>,
    pub quality_metrics: QualityMetrics,
}

/// Quality metrics for the RAG response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub source_quality_score: f32,
    pub relevance_score: f32,
    pub completeness_score: f32,
    pub authenticity_score: f32,
    pub citation_coverage: f32,
}

/// Citation structure for source references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub id: String,
    pub source: IslamicSource,
    pub citation_text: String,
    pub relevance_score: f32,
    pub usage_type: CitationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CitationType {
    Primary,    // مصدر أساسي
    Supporting, // مصدر داعم
    Reference,  // مصدر مرجعي
}

/// Error types for the AI service
#[derive(Debug, thiserror::Error)]
pub enum AIServiceError {
    #[error("Question processing failed: {0}")]
    QuestionProcessingError(String),
    
    #[error("Semantic search failed: {0}")]
    SemanticSearchError(String),
    
    #[error("Source verification failed: {0}")]
    SourceVerificationError(String),
    
    #[error("Response generation failed: {0}")]
    ResponseGenerationError(String),
    
    #[error("Hallucination detected: {0}")]
    HallucinationDetected(String),
    
    #[error("Out of scope question: {0}")]
    OutOfScopeQuestion(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("External API error: {0}")]
    ExternalAPIError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

pub type Result<T> = std::result::Result<T, AIServiceError>;
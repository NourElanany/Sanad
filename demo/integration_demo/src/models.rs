use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Search request model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub content_types: Option<Vec<String>>,
    pub limit: Option<usize>,
}

/// Search response model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total_count: usize,
    pub query: String,
    pub response_time_ms: u64,
    pub service_integration: String,
}

/// Individual search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub content: String,
    pub content_type: String,
    pub source: String,
    pub relevance_score: f32,
    pub metadata: HashMap<String, String>,
}

/// AI question request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIQuestionRequest {
    pub question: String,
    pub context: Option<String>,
    pub max_sources: Option<usize>,
}

/// AI response model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub answer: String,
    pub confidence: f32,
    pub sources: Vec<AISource>,
    pub citations: Vec<String>,
    pub warnings: Vec<String>,
    pub response_time_ms: u64,
    pub integration_flow: Vec<String>,
}

/// AI source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISource {
    pub id: String,
    pub content_type: String,
    pub reference: String,
    pub text: String,
    pub authenticity: String,
    pub relevance_score: f32,
}

/// Surah model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Surah {
    pub number: u32,
    pub name: String,
    pub arabic_name: String,
    pub english_name: String,
    pub number_of_ayahs: u32,
    pub revelation_type: String,
    pub ayahs: Vec<Ayah>,
}

/// Ayah model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ayah {
    pub number: u32,
    pub text: String,
    pub translation: Option<String>,
}

/// Surah response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurahResponse {
    pub surah: Surah,
    pub response_time_ms: u64,
}

/// Hadith model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hadith {
    pub id: String,
    pub text: String,
    pub narrator: String,
    pub book: String,
    pub chapter: String,
    pub grade: String,
    pub reference: String,
}

/// Hadith search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HadithSearchResponse {
    pub hadiths: Vec<Hadith>,
    pub total_count: usize,
    pub query: String,
    pub response_time_ms: u64,
}

/// Health status model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub services: HashMap<String, String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Integration test request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationTestRequest {
    pub test_name: String,
    pub test_scenarios: Option<Vec<String>>,
}

/// Integration test response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationTestResponse {
    pub test_name: String,
    pub overall_success: bool,
    pub test_results: Vec<TestResult>,
    pub total_duration_ms: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Individual test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_name: String,
    pub success: bool,
    pub message: String,
    pub duration_ms: u64,
}
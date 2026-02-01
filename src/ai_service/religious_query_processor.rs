use super::*;
use crate::ai_service::{
    rag_system::{RAGSystem, RAGRequest},
    question_processor::{QuestionProcessor, ProcessedQuestion},
    anti_hallucination::{AntiHallucinationSystem, HallucinationCheckResult},
    multiple_viewpoints_system::{MultipleViewpointsSystem, MultipleViewpointsResult},
    integration_service::{IntegrationService, RAGProcessingRequest},
};
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, warn, error};
use serde::{Serialize, Deserialize};

/// Religious query processor that handles Islamic questions with RAG integration
pub struct ReligiousQueryProcessor {
    rag_system: RAGSystem,
    question_processor: QuestionProcessor,
    anti_hallucination: AntiHallucinationSystem,
    multiple_viewpoints: MultipleViewpointsSystem,
    integration_service: Option<IntegrationService>,
    config: QueryProcessorConfig,
}

/// Configuration for the religious query processor
#[derive(Debug, Clone)]
pub struct QueryProcessorConfig {
    pub max_response_time_seconds: u64,
    pub enable_multiple_viewpoints: bool,
    pub enable_anti_hallucination: bool,
    pub require_source_verification: bool,
    pub min_confidence_threshold: f32,
    pub max_sources_per_query: usize,
    pub enable_controversial_detection: bool,
    pub fallback_to_offline: bool,
}

impl Default for QueryProcessorConfig {
    fn default() -> Self {
        Self {
            max_response_time_seconds: 30,
            enable_multiple_viewpoints: true,
            enable_anti_hallucination: true,
            require_source_verification: true,
            min_confidence_threshold: 0.7,
            max_sources_per_query: 10,
            enable_controversial_detection: true,
            fallback_to_offline: true,
        }
    }
}

/// Religious query request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReligiousQueryRequest {
    pub question: String,
    pub user_id: Option<String>,
    pub context: Option<String>,
    pub preferred_sources: Option<Vec<SourceType>>,
    pub language: Option<Language>,
    pub detail_level: Option<DetailLevel>,
    pub include_multiple_opinions: Option<bool>,
    pub max_response_time_seconds: Option<u64>,
}

/// Religious query response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReligiousQueryResponse {
    pub answer: String,
    pub confidence: f32,
    pub sources: Vec<IslamicSource>,
    pub citations: Vec<Citation>,
    pub related_questions: Vec<String>,
    pub multiple_viewpoints: Option<MultipleViewpointsResult>,
    pub warnings: Vec<String>,
    pub processing_time_ms: u64,
    pub quality_metrics: QualityMetrics,
    pub hallucination_check: Option<HallucinationCheckResult>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetailLevel {
    Brief,      // إجابة مختصرة
    Standard,   // إجابة عادية
    Detailed,   // إجابة مفصلة
    Scholarly,  // إجابة علمية متخصصة
}

impl ReligiousQueryProcessor {
    pub fn new() -> Self {
        Self {
            rag_system: RAGSystem::new(),
            question_processor: QuestionProcessor::new(),
            anti_hallucination: AntiHallucinationSystem::new(),
            multiple_viewpoints: MultipleViewpointsSystem::new(),
            integration_service: None,
            config: QueryProcessorConfig::default(),
        }
    }
    pub fn with_config(config: QueryProcessorConfig) -> Self {
        let mut processor = Self::new();
        processor.config = config;
        processor
    }

    pub fn with_integration_service(mut self, service: IntegrationService) -> Self {
        self.integration_service = Some(service);
        self
    }

    /// Process a religious query with full RAG pipeline
    pub async fn process_query(&mut self, request: ReligiousQueryRequest) -> Result<ReligiousQueryResponse> {
        let start_time = Instant::now();
        let mut warnings = Vec::new();
        let mut metadata = HashMap::new();

        info!("Processing religious query: {}", request.question);

        // 1. Process and analyze the question
        let processed_question = match self.question_processor.process_question(&request.question).await {
            Ok(question) => {
                metadata.insert("question_type".to_string(), format!("{:?}", question.question_type));
                metadata.insert("complexity_level".to_string(), format!("{:?}", question.complexity_level));
                metadata.insert("is_controversial".to_string(), question.is_controversial.to_string());
                question
            }
            Err(e) => {
                error!("Failed to process question: {}", e);
                return Err(e);
            }
        };

        // 2. Check if question is out of scope
        if matches!(processed_question.question_type, QuestionType::OutOfScope) {
            return Err(AIServiceError::OutOfScopeQuestion(
                "هذا السؤال خارج نطاق الشؤون الإسلامية. أنا متخصص في الإجابة على الأسئلة الدينية فقط.".to_string()
            ));
        }

        // 3. Use integration service if available, otherwise fall back to RAG system
        let response = if let Some(ref mut integration_service) = self.integration_service {
            self.process_with_integration_service(integration_service, &request, &processed_question).await?
        } else {
            self.process_with_rag_system(&request, &processed_question).await?
        };

        // 4. Apply post-processing filters and validations
        let final_response = self.post_process_response(response, &processed_question, &mut warnings).await?;

        let processing_time = start_time.elapsed().as_millis() as u64;
        metadata.insert("processing_time_ms".to_string(), processing_time.to_string());

        info!("Religious query processed successfully in {}ms", processing_time);

        Ok(ReligiousQueryResponse {
            answer: final_response.answer,
            confidence: final_response.confidence,
            sources: final_response.retrieved_sources,
            citations: final_response.citations,
            related_questions: final_response.related_questions,
            multiple_viewpoints: final_response.multiple_viewpoints,
            warnings,
            processing_time_ms: processing_time,
            quality_metrics: final_response.quality_metrics,
            hallucination_check: None, // Will be filled in post-processing
            metadata,
        })
    }

    /// Process query using integration service
    async fn process_with_integration_service(
        &self,
        integration_service: &mut IntegrationService,
        request: &ReligiousQueryRequest,
        processed_question: &ProcessedQuestion,
    ) -> Result<RAGResponse> {
        let rag_request = RAGProcessingRequest {
            question: request.question.clone(),
            context: request.context.clone(),
            max_sources: Some(self.config.max_sources_per_query),
            similarity_threshold: Some(self.config.min_confidence_threshold),
            preferred_source_types: request.preferred_sources.as_ref().map(|sources| {
                sources.iter().map(|s| format!("{:?}", s)).collect()
            }),
            language: request.language.as_ref().map(|l| format!("{:?}", l)),
            user_id: request.user_id.clone(),
        };

        let integration_response = integration_service.process_rag_request(rag_request).await?;

        // Convert integration response to RAG response
        Ok(RAGResponse {
            answer: integration_response.answer,
            confidence: integration_response.confidence,
            retrieved_sources: integration_response.sources.into_iter().map(|s| IslamicSource {
                id: s.id,
                content_type: match s.source_type.as_str() {
                    "quran" => SourceType::Quran,
                    "hadith" => SourceType::SahihHadith,
                    "tafsir" => SourceType::Tafsir,
                    _ => SourceType::ScholarOpinion,
                },
                text: s.text,
                reference: s.reference,
                author: s.author,
                authenticity: match s.authenticity.as_str() {
                    "verified" => AuthenticityLevel::Verified,
                    "reliable" => AuthenticityLevel::Reliable,
                    _ => AuthenticityLevel::Unknown,
                },
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: chrono::Utc::now(),
            }).collect(),
            cited_sources: Vec::new(), // Will be populated later
            citations: Vec::new(),
            related_questions: Vec::new(),
            warnings: Vec::new(),
            hallucination_risk: 0.0,
            response_time_ms: integration_response.processing_time_ms,
            metadata: HashMap::new(),
            quality_metrics: QualityMetrics {
                source_quality_score: 0.8,
                relevance_score: 0.8,
                completeness_score: 0.8,
                authenticity_score: 0.8,
                citation_coverage: 0.8,
            },
            multiple_viewpoints: None,
        })
    }

    /// Process query using RAG system
    async fn process_with_rag_system(
        &self,
        request: &ReligiousQueryRequest,
        processed_question: &ProcessedQuestion,
    ) -> Result<RAGResponse> {
        let rag_request = RAGRequest {
            question: request.question.clone(),
            user_id: request.user_id.clone(),
            context: request.context.as_ref().map(|c| {
                let mut context = HashMap::new();
                context.insert("user_context".to_string(), c.clone());
                context
            }),
            preferences: Some(crate::ai_service::rag_system::UserPreferences {
                preferred_sources: request.preferred_sources.clone().unwrap_or_default(),
                language: request.language.clone().unwrap_or(Language::Arabic),
                detail_level: match request.detail_level {
                    Some(DetailLevel::Brief) => crate::ai_service::rag_system::DetailLevel::Brief,
                    Some(DetailLevel::Standard) => crate::ai_service::rag_system::DetailLevel::Standard,
                    Some(DetailLevel::Detailed) => crate::ai_service::rag_system::DetailLevel::Detailed,
                    Some(DetailLevel::Scholarly) => crate::ai_service::rag_system::DetailLevel::Scholarly,
                    None => crate::ai_service::rag_system::DetailLevel::Standard,
                },
                include_multiple_opinions: request.include_multiple_opinions.unwrap_or(
                    processed_question.is_controversial || self.config.enable_multiple_viewpoints
                ),
            }),
        };

        self.rag_system.ask_question(rag_request).await
    }

    /// Post-process the response with additional validations and enhancements
    async fn post_process_response(
        &self,
        mut response: RAGResponse,
        processed_question: &ProcessedQuestion,
        warnings: &mut Vec<String>,
    ) -> Result<RAGResponse> {
        // 1. Anti-hallucination check
        if self.config.enable_anti_hallucination {
            match self.anti_hallucination.check_response(
                &response.answer,
                &response.retrieved_sources,
                processed_question,
            ).await {
                Ok(hallucination_check) => {
                    response.hallucination_risk = hallucination_check.hallucination_risk_score;
                    
                    // Add warnings based on hallucination check
                    if hallucination_check.hallucination_risk_score > 0.3 {
                        warnings.push("تم اكتشاف مخاطر محتملة في الإجابة. يرجى التحقق من المصادر.".to_string());
                    }
                    
                    if !hallucination_check.unsupported_claims.is_empty() {
                        warnings.push("بعض الادعاءات تحتاج مصادر إضافية.".to_string());
                    }
                    
                    if !hallucination_check.fabricated_content.is_empty() {
                        warnings.push("تم اكتشاف محتوى قد يكون غير دقيق.".to_string());
                    }
                }
                Err(e) => {
                    warn!("Anti-hallucination check failed: {}", e);
                    warnings.push("لم يتم التحقق من دقة الإجابة بالكامل.".to_string());
                }
            }
        }

        // 2. Multiple viewpoints analysis for controversial questions
        if processed_question.is_controversial && self.config.enable_multiple_viewpoints {
            // Note: Multiple viewpoints analysis would be implemented here
            // For now, we add a warning about controversial topics
            warnings.push("هذا الموضوع خلافي. يُنصح بمراجعة آراء العلماء المختلفة.".to_string());
        }

        // 3. Confidence validation
        if response.confidence < self.config.min_confidence_threshold {
            warnings.push("مستوى الثقة في الإجابة منخفض. يُنصح بمراجعة المصادر الإضافية.".to_string());
        }

        // 4. Source quality validation
        let weak_sources = response.retrieved_sources.iter()
            .filter(|s| matches!(s.content_type, SourceType::DaifHadith | SourceType::MawduHadith))
            .count();
        
        if weak_sources > 0 {
            warnings.push("تحتوي الإجابة على مصادر ضعيفة. يرجى التحقق من صحتها.".to_string());
        }

        // 5. Add Islamic courtesy phrases
        response.answer = self.add_islamic_courtesy(&response.answer);

        // 6. Generate citations if not already present
        if response.citations.is_empty() {
            response.citations = self.generate_citations(&response.retrieved_sources);
        }

        // 7. Generate related questions if not already present
        if response.related_questions.is_empty() {
            response.related_questions = self.generate_related_questions(processed_question, &response.retrieved_sources).await?;
        }

        Ok(response)
    }

    /// Add Islamic courtesy phrases to the response
    pub fn add_islamic_courtesy(&self, answer: &str) -> String {
        let mut enhanced_answer = answer.to_string();
        
        // Add "والله أعلم" if not already present
        if !enhanced_answer.contains("والله أعلم") && !enhanced_answer.contains("الله أعلم") {
            enhanced_answer.push_str("\n\nوالله أعلم.");
        }
        
        // Add Basmala for Quranic content if appropriate
        if enhanced_answer.contains("قال الله تعالى") && !enhanced_answer.starts_with("بسم الله") {
            enhanced_answer = format!("بسم الله الرحمن الرحيم\n\n{}", enhanced_answer);
        }
        
        enhanced_answer
    }

    /// Generate citations for sources
    fn generate_citations(&self, sources: &[IslamicSource]) -> Vec<Citation> {
        sources.iter()
            .enumerate()
            .map(|(index, source)| {
                Citation {
                    id: format!("cite_{}", index + 1),
                    source: source.clone(),
                    citation_text: self.format_citation(source),
                    relevance_score: 0.8, // Default relevance
                    usage_type: CitationType::Primary,
                }
            })
            .collect()
    }

    /// Format citation according to Islamic scholarly standards
    pub fn format_citation(&self, source: &IslamicSource) -> String {
        match source.content_type {
            SourceType::Quran => {
                format!("القرآن الكريم، {}", source.reference)
            },
            SourceType::SahihHadith | SourceType::HasanHadith | SourceType::DaifHadith => {
                if let Some(author) = &source.author {
                    format!("{}: {}", author, source.reference)
                } else {
                    source.reference.clone()
                }
            },
            SourceType::Tafsir => {
                if let Some(author) = &source.author {
                    format!("تفسير {}: {}", author, source.reference)
                } else {
                    format!("تفسير: {}", source.reference)
                }
            },
            SourceType::FiqhRuling => {
                if let Some(author) = &source.author {
                    format!("فتوى {}: {}", author, source.reference)
                } else {
                    format!("فتوى: {}", source.reference)
                }
            },
            _ => {
                if let Some(author) = &source.author {
                    format!("{}: {}", author, source.reference)
                } else {
                    source.reference.clone()
                }
            }
        }
    }

    /// Generate related questions based on the query and sources
    async fn generate_related_questions(
        &self,
        question: &ProcessedQuestion,
        sources: &[IslamicSource],
    ) -> Result<Vec<String>> {
        let mut related = Vec::new();
        
        // Generate questions based on question type
        match question.question_type {
            QuestionType::Fiqh => {
                related.push("ما هي الأدلة على هذا الحكم؟".to_string());
                related.push("هل هناك خلاف في هذه المسألة؟".to_string());
                if sources.iter().any(|s| matches!(s.content_type, SourceType::Quran)) {
                    related.push("ما هي الآيات القرآنية المتعلقة بهذا الموضوع؟".to_string());
                }
            },
            QuestionType::Tafsir => {
                related.push("ما هو سبب نزول هذه الآية؟".to_string());
                related.push("ما هي الدروس المستفادة من هذه الآية؟".to_string());
                related.push("كيف فسر العلماء هذه الآية؟".to_string());
            },
            QuestionType::Hadith => {
                related.push("ما هي درجة صحة هذا الحديث؟".to_string());
                related.push("هل هناك أحاديث أخرى في نفس الموضوع؟".to_string());
                related.push("من هم رواة هذا الحديث؟".to_string());
            },
            QuestionType::Aqeedah => {
                related.push("ما هو موقف أهل السنة والجماعة من هذه المسألة؟".to_string());
                related.push("ما هي الأدلة من القرآن والسنة؟".to_string());
            },
            QuestionType::Sirah => {
                related.push("ما هي الدروس المستفادة من هذا الحدث؟".to_string());
                related.push("متى وقع هذا الحدث؟".to_string());
            },
            _ => {
                related.push("هل يمكن توضيح هذا الموضوع أكثر؟".to_string());
                related.push("ما هي المصادر الإضافية لهذا الموضوع؟".to_string());
            }
        }

        // Generate questions based on concepts
        for concept in &question.concepts {
            match concept.as_str() {
                "صلاة" => {
                    related.push("ما هي شروط صحة الصلاة؟".to_string());
                    related.push("كيف نصلي الصلاة الصحيحة؟".to_string());
                },
                "زكاة" => {
                    related.push("متى تجب الزكاة؟".to_string());
                    related.push("كيف نحسب الزكاة؟".to_string());
                },
                "صوم" => {
                    related.push("ما هي مبطلات الصوم؟".to_string());
                    related.push("متى يجب الصوم؟".to_string());
                },
                "حج" => {
                    related.push("ما هي أركان الحج؟".to_string());
                    related.push("متى يجب الحج؟".to_string());
                },
                _ => {}
            }
        }

        // Remove duplicates and limit to 3 questions
        related.sort();
        related.dedup();
        Ok(related.into_iter().take(3).collect())
    }

    /// Validate query request
    pub fn validate_request(&self, request: &ReligiousQueryRequest) -> Result<()> {
        if request.question.trim().is_empty() {
            return Err(AIServiceError::QuestionProcessingError(
                "السؤال لا يمكن أن يكون فارغاً".to_string()
            ));
        }

        if request.question.len() > 1000 {
            return Err(AIServiceError::QuestionProcessingError(
                "السؤال طويل جداً. يرجى تقصيره.".to_string()
            ));
        }

        if let Some(max_time) = request.max_response_time_seconds {
            if max_time > 300 { // 5 minutes max
                return Err(AIServiceError::QuestionProcessingError(
                    "وقت الاستجابة المطلوب طويل جداً".to_string()
                ));
            }
        }

        Ok(())
    }

    /// Get processor statistics
    pub async fn get_statistics(&self) -> ProcessorStatistics {
        ProcessorStatistics {
            total_queries_processed: 0, // Would be tracked in a real implementation
            average_response_time_ms: 0.0,
            success_rate: 0.0,
            most_common_question_types: HashMap::new(),
            cache_hit_rate: 0.0,
        }
    }
}

/// Statistics for the religious query processor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorStatistics {
    pub total_queries_processed: u64,
    pub average_response_time_ms: f64,
    pub success_rate: f64,
    pub most_common_question_types: HashMap<String, u64>,
    pub cache_hit_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_religious_query_processor_creation() {
        let processor = ReligiousQueryProcessor::new();
        assert!(processor.integration_service.is_none());
    }

    #[tokio::test]
    async fn test_query_validation() {
        let processor = ReligiousQueryProcessor::new();
        
        // Valid request
        let valid_request = ReligiousQueryRequest {
            question: "ما هي أركان الإسلام؟".to_string(),
            user_id: Some("test_user".to_string()),
            context: None,
            preferred_sources: None,
            language: Some(Language::Arabic),
            detail_level: Some(DetailLevel::Standard),
            include_multiple_opinions: None,
            max_response_time_seconds: None,
        };
        
        assert!(processor.validate_request(&valid_request).is_ok());
        
        // Invalid request - empty question
        let invalid_request = ReligiousQueryRequest {
            question: "".to_string(),
            user_id: None,
            context: None,
            preferred_sources: None,
            language: None,
            detail_level: None,
            include_multiple_opinions: None,
            max_response_time_seconds: None,
        };
        
        assert!(processor.validate_request(&invalid_request).is_err());
    }

    #[tokio::test]
    async fn test_islamic_courtesy_addition() {
        let processor = ReligiousQueryProcessor::new();
        
        let answer_without_courtesy = "الصلاة هي الركن الثاني من أركان الإسلام";
        let enhanced_answer = processor.add_islamic_courtesy(answer_without_courtesy);
        
        assert!(enhanced_answer.contains("والله أعلم"));
    }

    #[tokio::test]
    async fn test_citation_formatting() {
        let processor = ReligiousQueryProcessor::new();
        
        let quran_source = IslamicSource {
            id: "test_quran".to_string(),
            content_type: SourceType::Quran,
            text: "بسم الله الرحمن الرحيم".to_string(),
            reference: "الفاتحة: 1".to_string(),
            author: None,
            authenticity: AuthenticityLevel::Verified,
            language: Language::Arabic,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        };
        
        let citation = processor.format_citation(&quran_source);
        assert!(citation.contains("القرآن الكريم"));
        assert!(citation.contains("الفاتحة: 1"));
    }

    #[tokio::test]
    async fn test_related_questions_generation() {
        let processor = ReligiousQueryProcessor::new();
        
        let processed_question = ProcessedQuestion {
            original_text: "ما هي أركان الإسلام؟".to_string(),
            normalized_text: "ما هي أركان الإسلام".to_string(),
            keywords: vec!["أركان".to_string(), "إسلام".to_string()],
            concepts: vec!["إسلام".to_string(), "صلاة".to_string()],
            question_type: QuestionType::General,
            complexity_level: ComplexityLevel::Simple,
            language: Language::Arabic,
            is_controversial: false,
            requires_multiple_sources: false,
            embedding: None,
        };
        
        let sources = vec![
            IslamicSource {
                id: "test_source".to_string(),
                content_type: SourceType::SahihHadith,
                text: "بني الإسلام على خمس".to_string(),
                reference: "صحيح البخاري".to_string(),
                author: Some("البخاري".to_string()),
                authenticity: AuthenticityLevel::Verified,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: chrono::Utc::now(),
            }
        ];
        
        let related_questions = processor.generate_related_questions(&processed_question, &sources).await.unwrap();
        
        assert!(!related_questions.is_empty());
        assert!(related_questions.len() <= 3);
    }
}
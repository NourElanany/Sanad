use super::*;
use std::collections::HashMap;

/// Source scoring system for evaluating retrieved Islamic sources
pub struct SourceScoringSystem {
    relevance_calculator: RelevanceCalculator,
    authority_evaluator: AuthorityEvaluator,
    freshness_assessor: FreshnessAssessor,
    consensus_checker: ConsensusChecker,
    weights: ScoringWeights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceScore {
    pub relevance_score: f32,      // درجة الصلة (0.0 - 1.0)
    pub authority_score: f32,      // درجة الموثوقية (0.0 - 1.0)
    pub authenticity_score: f32,   // درجة الأصالة (0.0 - 1.0)
    pub consensus_score: f32,      // درجة الإجماع (0.0 - 1.0)
    pub freshness_score: f32,      // درجة الحداثة (0.0 - 1.0)
    pub final_score: f32,          // الدرجة النهائية
    pub confidence_level: ConfidenceLevel,
    pub scoring_details: ScoringDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringDetails {
    pub relevance_factors: Vec<String>,
    pub authority_factors: Vec<String>,
    pub authenticity_factors: Vec<String>,
    pub consensus_factors: Vec<String>,
    pub penalties: Vec<String>,
    pub bonuses: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ScoringWeights {
    pub relevance_weight: f32,
    pub authority_weight: f32,
    pub authenticity_weight: f32,
    pub consensus_weight: f32,
    pub freshness_weight: f32,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            relevance_weight: 0.35,    // الصلة هي الأهم
            authority_weight: 0.25,    // الموثوقية مهمة جداً
            authenticity_weight: 0.25, // الأصالة أساسية
            consensus_weight: 0.10,    // الإجماع مفيد
            freshness_weight: 0.05,    // الحداثة أقل أهمية للنصوص الإسلامية
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredSource {
    pub source: IslamicSource,
    pub score: SourceScore,
    pub rank: usize,
    pub usage_recommendation: SourceUsageRecommendation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceUsageRecommendation {
    Primary,      // مصدر أساسي - يُستخدم في الإجابة الرئيسية
    Supporting,   // مصدر داعم - يُستخدم للتأكيد
    Reference,    // مصدر مرجعي - يُذكر للاستزادة
    Cautionary,   // مصدر تحذيري - يُستخدم مع تنبيه
    Excluded,     // مصدر مستبعد - لا يُستخدم
}

impl SourceScoringSystem {
    pub fn new() -> Self {
        Self {
            relevance_calculator: RelevanceCalculator::new(),
            authority_evaluator: AuthorityEvaluator::new(),
            freshness_assessor: FreshnessAssessor::new(),
            consensus_checker: ConsensusChecker::new(),
            weights: ScoringWeights::default(),
        }
    }
    
    pub fn with_custom_weights(weights: ScoringWeights) -> Self {
        let mut system = Self::new();
        system.weights = weights;
        system
    }
    
    pub async fn score_sources(
        &self,
        sources: &[IslamicSource],
        query: &ProcessedQuestion,
    ) -> Result<Vec<ScoredSource>> {
        let mut scored_sources = Vec::new();
        
        for source in sources {
            let score = self.calculate_score(source, query).await?;
            let usage_recommendation = self.determine_usage_recommendation(&score, source);
            
            scored_sources.push(ScoredSource {
                source: source.clone(),
                score,
                rank: 0, // سيتم تحديثه بعد الترتيب
                usage_recommendation,
            });
        }
        
        // ترتيب المصادر حسب النتيجة النهائية
        scored_sources.sort_by(|a, b| b.score.final_score.partial_cmp(&a.score.final_score).unwrap());
        
        // تحديث الترتيب
        for (index, scored_source) in scored_sources.iter_mut().enumerate() {
            scored_source.rank = index + 1;
        }
        
        Ok(scored_sources)
    }
    
    pub async fn calculate_score(
        &self,
        source: &IslamicSource,
        query: &ProcessedQuestion,
    ) -> Result<SourceScore> {
        let relevance = self.relevance_calculator.calculate(source, query).await?;
        let authority = self.authority_evaluator.evaluate(source).await?;
        let authenticity = self.calculate_authenticity_score(source).await?;
        let consensus = self.consensus_checker.check_consensus(source, query).await?;
        let freshness = self.freshness_assessor.assess(source).await?;
        
        let final_score = (relevance.score * self.weights.relevance_weight)
            + (authority.score * self.weights.authority_weight)
            + (authenticity.score * self.weights.authenticity_weight)
            + (consensus.score * self.weights.consensus_weight)
            + (freshness.score * self.weights.freshness_weight);
        
        let confidence_level = ConfidenceLevel::from_score(final_score);
        
        let scoring_details = ScoringDetails {
            relevance_factors: relevance.factors,
            authority_factors: authority.factors,
            authenticity_factors: authenticity.factors,
            consensus_factors: consensus.factors,
            penalties: self.calculate_penalties(source),
            bonuses: self.calculate_bonuses(source),
        };
        
        Ok(SourceScore {
            relevance_score: relevance.score,
            authority_score: authority.score,
            authenticity_score: authenticity.score,
            consensus_score: consensus.score,
            freshness_score: freshness.score,
            final_score,
            confidence_level,
            scoring_details,
        })
    }
    
    async fn calculate_authenticity_score(&self, source: &IslamicSource) -> Result<ScoringResult> {
        let mut score = match source.authenticity {
            AuthenticityLevel::Verified => 1.0,
            AuthenticityLevel::Reliable => 0.8,
            AuthenticityLevel::Questionable => 0.5,
            AuthenticityLevel::Unreliable => 0.2,
            AuthenticityLevel::Unknown => 0.4,
        };
        
        let mut factors = vec![format!("مستوى الأصالة: {:?}", source.authenticity)];
        
        // تعديل النتيجة بناءً على نوع المصدر
        match source.content_type {
            SourceType::Quran => {
                score = 1.0; // القرآن دائماً أصيل
                factors.push("القرآن الكريم - أصالة مطلقة".to_string());
            },
            SourceType::SahihHadith => {
                score = score.max(0.9);
                factors.push("حديث صحيح".to_string());
            },
            SourceType::HasanHadith => {
                score = score.max(0.8);
                factors.push("حديث حسن".to_string());
            },
            SourceType::DaifHadith => {
                score = score.min(0.6);
                factors.push("حديث ضعيف".to_string());
            },
            SourceType::MawduHadith => {
                score = 0.1;
                factors.push("حديث موضوع".to_string());
            },
            _ => {}
        }
        
        Ok(ScoringResult { score, factors })
    }
    
    fn determine_usage_recommendation(&self, score: &SourceScore, source: &IslamicSource) -> SourceUsageRecommendation {
        // استبعاد المصادر غير الموثوقة
        if matches!(source.content_type, SourceType::MawduHadith) {
            return SourceUsageRecommendation::Excluded;
        }
        
        match score.confidence_level {
            ConfidenceLevel::VeryHigh => SourceUsageRecommendation::Primary,
            ConfidenceLevel::High => {
                if score.relevance_score > 0.8 {
                    SourceUsageRecommendation::Primary
                } else {
                    SourceUsageRecommendation::Supporting
                }
            },
            ConfidenceLevel::Medium => {
                if matches!(source.content_type, SourceType::DaifHadith) {
                    SourceUsageRecommendation::Cautionary
                } else {
                    SourceUsageRecommendation::Supporting
                }
            },
            ConfidenceLevel::Low => SourceUsageRecommendation::Reference,
            ConfidenceLevel::VeryLow => SourceUsageRecommendation::Excluded,
        }
    }
    
    fn calculate_penalties(&self, source: &IslamicSource) -> Vec<String> {
        let mut penalties = Vec::new();
        
        if matches!(source.content_type, SourceType::DaifHadith) {
            penalties.push("حديث ضعيف - يحتاج تنبيه".to_string());
        }
        
        if matches!(source.authenticity, AuthenticityLevel::Questionable | AuthenticityLevel::Unreliable) {
            penalties.push("مصدر مشكوك في موثوقيته".to_string());
        }
        
        penalties
    }
    
    fn calculate_bonuses(&self, source: &IslamicSource) -> Vec<String> {
        let mut bonuses = Vec::new();
        
        if matches!(source.content_type, SourceType::Quran) {
            bonuses.push("القرآن الكريم - المصدر الأول".to_string());
        }
        
        if matches!(source.content_type, SourceType::SahihHadith) {
            bonuses.push("حديث صحيح - موثوق تماماً".to_string());
        }
        
        if source.author.as_ref().map_or(false, |author| {
            ["البخاري", "مسلم", "ابن كثير", "الطبري", "القرطبي"].contains(&author.as_str())
        }) {
            bonuses.push("مؤلف معتبر ومشهور".to_string());
        }
        
        bonuses
    }
}

/// Relevance calculator for determining how relevant a source is to the query
pub struct RelevanceCalculator {
    semantic_similarity_threshold: f32,
}

#[derive(Debug, Clone)]
pub struct ScoringResult {
    pub score: f32,
    pub factors: Vec<String>,
}

impl RelevanceCalculator {
    pub fn new() -> Self {
        Self {
            semantic_similarity_threshold: 0.3,
        }
    }
    
    pub async fn calculate(&self, source: &IslamicSource, query: &ProcessedQuestion) -> Result<ScoringResult> {
        let mut score = 0.0;
        let mut factors = Vec::new();
        
        // تطابق الكلمات المفتاحية
        let keyword_match_score = self.calculate_keyword_match(&source.text, &query.keywords);
        score += keyword_match_score * 0.3;
        if keyword_match_score > 0.5 {
            factors.push(format!("تطابق كلمات مفتاحية: {:.2}", keyword_match_score));
        }
        
        // تطابق المفاهيم
        let concept_match_score = self.calculate_concept_match(&source.text, &query.concepts);
        score += concept_match_score * 0.4;
        if concept_match_score > 0.5 {
            factors.push(format!("تطابق مفاهيم: {:.2}", concept_match_score));
        }
        
        // تطابق نوع السؤال مع نوع المصدر
        let type_match_score = self.calculate_type_match(&source.content_type, &query.question_type);
        score += type_match_score * 0.3;
        if type_match_score > 0.7 {
            factors.push(format!("تطابق نوع المحتوى: {:.2}", type_match_score));
        }
        
        // تطابق دلالي (إذا كان متاحاً)
        if let Some(query_embedding) = &query.embedding {
            // هنا يمكن حساب التشابه الدلالي
            // let semantic_score = calculate_cosine_similarity(source_embedding, query_embedding);
            // score += semantic_score * 0.2;
        }
        
        Ok(ScoringResult {
            score: score.min(1.0),
            factors,
        })
    }
    
    fn calculate_keyword_match(&self, text: &str, keywords: &[String]) -> f32 {
        if keywords.is_empty() {
            return 0.0;
        }
        
        let text_lower = text.to_lowercase();
        let matches = keywords.iter()
            .filter(|keyword| text_lower.contains(&keyword.to_lowercase()))
            .count();
        
        matches as f32 / keywords.len() as f32
    }
    
    fn calculate_concept_match(&self, text: &str, concepts: &[String]) -> f32 {
        if concepts.is_empty() {
            return 0.0;
        }
        
        let text_lower = text.to_lowercase();
        let matches = concepts.iter()
            .filter(|concept| text_lower.contains(&concept.to_lowercase()))
            .count();
        
        matches as f32 / concepts.len() as f32
    }
    
    fn calculate_type_match(&self, source_type: &SourceType, question_type: &QuestionType) -> f32 {
        match (question_type, source_type) {
            (QuestionType::Tafsir, SourceType::Quran) => 1.0,
            (QuestionType::Tafsir, SourceType::Tafsir) => 0.9,
            (QuestionType::Hadith, SourceType::SahihHadith) => 1.0,
            (QuestionType::Hadith, SourceType::HasanHadith) => 0.9,
            (QuestionType::Hadith, SourceType::DaifHadith) => 0.7,
            (QuestionType::Fiqh, SourceType::FiqhRuling) => 1.0,
            (QuestionType::Fiqh, SourceType::SahihHadith) => 0.8,
            (QuestionType::Aqeedah, SourceType::Quran) => 0.9,
            (QuestionType::Aqeedah, SourceType::SahihHadith) => 0.8,
            (_, SourceType::Quran) => 0.8, // القرآن مناسب لمعظم الأسئلة
            _ => 0.5,
        }
    }
}

/// Authority evaluator for assessing source authority and credibility
pub struct AuthorityEvaluator {
    authority_database: HashMap<String, f32>,
}

impl AuthorityEvaluator {
    pub fn new() -> Self {
        let mut authority_db = HashMap::new();
        
        // علماء التفسير
        authority_db.insert("ابن كثير".to_string(), 1.0);
        authority_db.insert("الطبري".to_string(), 1.0);
        authority_db.insert("القرطبي".to_string(), 0.95);
        authority_db.insert("البغوي".to_string(), 0.9);
        
        // علماء الحديث
        authority_db.insert("البخاري".to_string(), 1.0);
        authority_db.insert("مسلم".to_string(), 1.0);
        authority_db.insert("أبو داود".to_string(), 0.9);
        authority_db.insert("الترمذي".to_string(), 0.9);
        authority_db.insert("النسائي".to_string(), 0.9);
        authority_db.insert("ابن ماجه".to_string(), 0.85);
        
        // علماء الفقه
        authority_db.insert("أبو حنيفة".to_string(), 0.95);
        authority_db.insert("مالك".to_string(), 0.95);
        authority_db.insert("الشافعي".to_string(), 0.95);
        authority_db.insert("أحمد بن حنبل".to_string(), 0.95);
        
        Self {
            authority_database: authority_db,
        }
    }
    
    pub async fn evaluate(&self, source: &IslamicSource) -> Result<ScoringResult> {
        let mut score = 0.5; // نقطة بداية متوسطة
        let mut factors = Vec::new();
        
        // تقييم بناءً على المؤلف
        if let Some(author) = &source.author {
            if let Some(&author_score) = self.authority_database.get(author) {
                score = author_score;
                factors.push(format!("مؤلف معتبر: {} ({:.2})", author, author_score));
            } else {
                factors.push(format!("مؤلف: {} (غير مصنف)", author));
            }
        }
        
        // تقييم بناءً على نوع المصدر
        let source_type_score = match source.content_type {
            SourceType::Quran => 1.0,
            SourceType::SahihHadith => 0.95,
            SourceType::HasanHadith => 0.85,
            SourceType::Tafsir => 0.8,
            SourceType::FiqhRuling => 0.75,
            SourceType::ScholarOpinion => 0.7,
            SourceType::DaifHadith => 0.5,
            SourceType::MawduHadith => 0.1,
            SourceType::IslamicStory => 0.6,
        };
        
        score = score.max(source_type_score);
        factors.push(format!("نوع المصدر: {:?} ({:.2})", source.content_type, source_type_score));
        
        // تقييم بناءً على المرجع
        let reference_score = self.evaluate_reference(&source.reference);
        score = (score + reference_score) / 2.0;
        if reference_score > 0.7 {
            factors.push(format!("مرجع موثوق: {:.2}", reference_score));
        }
        
        Ok(ScoringResult {
            score: score.min(1.0),
            factors,
        })
    }
    
    fn evaluate_reference(&self, reference: &str) -> f32 {
        let reference_lower = reference.to_lowercase();
        
        if reference_lower.contains("صحيح البخاري") || reference_lower.contains("صحيح مسلم") {
            1.0
        } else if reference_lower.contains("سنن") {
            0.85
        } else if reference_lower.contains("مسند") {
            0.8
        } else if reference_lower.contains("تفسير") {
            0.75
        } else {
            0.5
        }
    }
}

/// Freshness assessor for evaluating content recency (less important for Islamic texts)
pub struct FreshnessAssessor;

impl FreshnessAssessor {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn assess(&self, source: &IslamicSource) -> Result<ScoringResult> {
        // بالنسبة للنصوص الإسلامية، الحداثة أقل أهمية
        // النصوص الأقدم قد تكون أكثر موثوقية
        
        let score = match source.content_type {
            SourceType::Quran => 1.0, // القرآن خالد
            SourceType::SahihHadith | SourceType::HasanHadith => 1.0, // الأحاديث الصحيحة خالدة
            _ => 0.8, // المصادر الأخرى
        };
        
        Ok(ScoringResult {
            score,
            factors: vec!["النصوص الإسلامية الأصيلة خالدة".to_string()],
        })
    }
}

/// Consensus checker for evaluating scholarly consensus
pub struct ConsensusChecker;

impl ConsensusChecker {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn check_consensus(&self, source: &IslamicSource, query: &ProcessedQuestion) -> Result<ScoringResult> {
        let mut score = 0.5;
        let mut factors = Vec::new();
        
        // فحص إذا كان الموضوع خلافياً
        if query.is_controversial {
            // للمواضيع الخلافية، نحتاج مصادر متعددة
            score = 0.6;
            factors.push("موضوع خلافي - يحتاج مصادر متعددة".to_string());
        } else {
            // للمواضيع غير الخلافية
            score = 0.8;
            factors.push("موضوع غير خلافي".to_string());
        }
        
        // تعديل النتيجة بناءً على نوع المصدر
        match source.content_type {
            SourceType::Quran => {
                score = 1.0;
                factors.push("القرآن - إجماع مطلق".to_string());
            },
            SourceType::SahihHadith => {
                score = score.max(0.9);
                factors.push("حديث صحيح - إجماع عالي".to_string());
            },
            _ => {}
        }
        
        Ok(ScoringResult { score, factors })
    }
}
use super::*;
use crate::ai_service::{
    question_processor::ProcessedQuestion,
    source_scorer::{ScoredSource, SourceUsageRecommendation},
};
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// Multiple viewpoints system for handling controversial Islamic questions
/// This system automatically detects controversial questions and presents different scholarly opinions
pub struct MultipleViewpointsSystem {
    controversy_detector: ControlversyDetector,
    madhab_classifier: MadhabClassifier,
    viewpoint_aggregator: ViewpointAggregator,
    source_reliability_evaluator: SourceReliabilityEvaluator,
    internal_guidance_generator: InternalGuidanceGenerator,
}

/// Represents different Islamic schools of thought (madhabs)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IslamicMadhab {
    Hanafi,     // الحنفي
    Maliki,     // المالكي
    Shafii,     // الشافعي
    Hanbali,    // الحنبلي
    Zahiri,     // الظاهري
    Shia,       // الشيعي
    Ibadi,      // الإباضي
    General,    // عام - لا ينتمي لمذهب محدد
}

impl IslamicMadhab {
    pub fn to_arabic(&self) -> &'static str {
        match self {
            IslamicMadhab::Hanafi => "الحنفي",
            IslamicMadhab::Maliki => "المالكي",
            IslamicMadhab::Shafii => "الشافعي",
            IslamicMadhab::Hanbali => "الحنبلي",
            IslamicMadhab::Zahiri => "الظاهري",
            IslamicMadhab::Shia => "الشيعي",
            IslamicMadhab::Ibadi => "الإباضي",
            IslamicMadhab::General => "عام",
        }
    }
}

/// Represents a scholarly viewpoint on a controversial issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScholarlyViewpoint {
    pub id: String,
    pub madhab: IslamicMadhab,
    pub position: String,                    // الموقف أو الرأي
    pub evidence: Vec<IslamicSource>,        // الأدلة المدعمة
    pub reasoning: String,                   // التعليل والاستدلال
    pub prominent_scholars: Vec<String>,     // العلماء المؤيدين
    pub strength_level: ViewpointStrength,   // قوة الرأي
    pub conditions: Vec<String>,             // الشروط والضوابط
    pub exceptions: Vec<String>,             // الاستثناءات
    pub modern_applications: Vec<String>,    // التطبيقات المعاصرة
}

/// Strength level of a scholarly viewpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewpointStrength {
    Consensus,      // إجماع
    Majority,       // رأي الجمهور
    Strong,         // رأي قوي
    Moderate,       // رأي متوسط
    Weak,           // رأي ضعيف
    Minority,       // رأي الأقلية
}

impl ViewpointStrength {
    pub fn to_arabic(&self) -> &'static str {
        match self {
            ViewpointStrength::Consensus => "إجماع",
            ViewpointStrength::Majority => "رأي الجمهور",
            ViewpointStrength::Strong => "رأي قوي",
            ViewpointStrength::Moderate => "رأي متوسط",
            ViewpointStrength::Weak => "رأي ضعيف",
            ViewpointStrength::Minority => "رأي الأقلية",
        }
    }
    
    pub fn to_score(&self) -> f32 {
        match self {
            ViewpointStrength::Consensus => 1.0,
            ViewpointStrength::Majority => 0.8,
            ViewpointStrength::Strong => 0.7,
            ViewpointStrength::Moderate => 0.6,
            ViewpointStrength::Weak => 0.4,
            ViewpointStrength::Minority => 0.3,
        }
    }
}

/// Result of multiple viewpoints analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleViewpointsResult {
    pub is_controversial: bool,
    pub controversy_level: ControlversyLevel,
    pub viewpoints: Vec<ScholarlyViewpoint>,
    pub consensus_areas: Vec<String>,           // نقاط الاتفاق
    pub disagreement_areas: Vec<String>,        // نقاط الخلاف
    pub recommended_approach: String,           // المنهج المقترح
    pub internal_guidance: Vec<InternalGuidance>, // التوجيه للمصادر الداخلية
    pub source_reliability_assessment: SourceReliabilityAssessment,
    pub summary: ViewpointsSummary,
}

/// Level of controversy in the question
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlversyLevel {
    None,           // لا يوجد خلاف
    Minor,          // خلاف طفيف
    Moderate,       // خلاف متوسط
    Significant,    // خلاف كبير
    Major,          // خلاف جوهري
}

impl ControlversyLevel {
    pub fn to_arabic(&self) -> &'static str {
        match self {
            ControlversyLevel::None => "لا يوجد خلاف",
            ControlversyLevel::Minor => "خلاف طفيف",
            ControlversyLevel::Moderate => "خلاف متوسط",
            ControlversyLevel::Significant => "خلاف كبير",
            ControlversyLevel::Major => "خلاف جوهري",
        }
    }
}

/// Internal guidance to detailed sources within the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalGuidance {
    pub source_type: InternalSourceType,
    pub reference_path: String,              // مسار المرجع في التطبيق
    pub description: String,                 // وصف المحتوى
    pub relevance_score: f32,               // درجة الصلة
    pub recommended_sections: Vec<String>,   // الأقسام المقترحة
}

/// Types of internal sources within the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InternalSourceType {
    QuranSection,       // قسم في القرآن
    HadithCollection,   // مجموعة أحاديث
    TafsirChapter,      // فصل في التفسير
    FiqhRuling,         // حكم فقهي
    ScholarlyArticle,   // مقال علمي
    ComparativeStudy,   // دراسة مقارنة
}

/// Assessment of source reliability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceReliabilityAssessment {
    pub overall_reliability: f32,           // الموثوقية العامة
    pub source_breakdown: HashMap<String, SourceReliabilityScore>,
    pub reliability_factors: Vec<String>,   // عوامل الموثوقية
    pub warnings: Vec<String>,              // تحذيرات
    pub recommendations: Vec<String>,       // توصيات
}

/// Reliability score for individual sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceReliabilityScore {
    pub score: f32,                         // النتيجة (0.0 - 1.0)
    pub factors: Vec<String>,               // العوامل المؤثرة
    pub classification: ReliabilityClassification,
}

/// Classification of source reliability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReliabilityClassification {
    HighlyReliable,     // موثوق جداً
    Reliable,           // موثوق
    ModeratelyReliable, // موثوق نسبياً
    Questionable,       // مشكوك فيه
    Unreliable,         // غير موثوق
}

impl ReliabilityClassification {
    pub fn to_arabic(&self) -> &'static str {
        match self {
            ReliabilityClassification::HighlyReliable => "موثوق جداً",
            ReliabilityClassification::Reliable => "موثوق",
            ReliabilityClassification::ModeratelyReliable => "موثوق نسبياً",
            ReliabilityClassification::Questionable => "مشكوك فيه",
            ReliabilityClassification::Unreliable => "غير موثوق",
        }
    }
}

/// Summary of different viewpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewpointsSummary {
    pub total_viewpoints: usize,
    pub madhabs_represented: Vec<IslamicMadhab>,
    pub consensus_percentage: f32,           // نسبة الإجماع
    pub main_disagreement: Option<String>,   // الخلاف الرئيسي
    pub practical_recommendation: String,    // التوصية العملية
}

impl MultipleViewpointsSystem {
    pub fn new() -> Self {
        Self {
            controversy_detector: ControlversyDetector::new(),
            madhab_classifier: MadhabClassifier::new(),
            viewpoint_aggregator: ViewpointAggregator::new(),
            source_reliability_evaluator: SourceReliabilityEvaluator::new(),
            internal_guidance_generator: InternalGuidanceGenerator::new(),
        }
    }
    
    /// Main entry point for analyzing multiple viewpoints
    pub async fn analyze_viewpoints(
        &self,
        question: &ProcessedQuestion,
        sources: &[ScoredSource],
    ) -> Result<MultipleViewpointsResult> {
        // 1. Detect if the question is controversial
        let controversy_analysis = self.controversy_detector
            .analyze_controversy(question, sources).await?;
        
        if !controversy_analysis.is_controversial {
            return Ok(MultipleViewpointsResult {
                is_controversial: false,
                controversy_level: ControlversyLevel::None,
                viewpoints: vec![],
                consensus_areas: vec!["هذا الموضوع محل اتفاق بين العلماء".to_string()],
                disagreement_areas: vec![],
                recommended_approach: "يمكن الاعتماد على المصادر المتاحة".to_string(),
                internal_guidance: vec![],
                source_reliability_assessment: SourceReliabilityAssessment {
                    overall_reliability: 0.8,
                    source_breakdown: HashMap::new(),
                    reliability_factors: vec!["موضوع غير خلافي".to_string()],
                    warnings: vec![],
                    recommendations: vec![],
                },
                summary: ViewpointsSummary {
                    total_viewpoints: 1,
                    madhabs_represented: vec![IslamicMadhab::General],
                    consensus_percentage: 100.0,
                    main_disagreement: None,
                    practical_recommendation: "اتباع الرأي المتفق عليه".to_string(),
                },
            });
        }
        
        // 2. Classify sources by madhab
        let madhab_classification = self.madhab_classifier
            .classify_sources_by_madhab(sources).await?;
        
        // 3. Aggregate different viewpoints
        let viewpoints = self.viewpoint_aggregator
            .aggregate_viewpoints(question, &madhab_classification).await?;
        
        // 4. Evaluate source reliability
        let reliability_assessment = self.source_reliability_evaluator
            .evaluate_sources(sources, &viewpoints).await?;
        
        // 5. Generate internal guidance
        let internal_guidance = self.internal_guidance_generator
            .generate_guidance(question, &viewpoints).await?;
        
        // 6. Create summary
        let summary = self.create_viewpoints_summary(&viewpoints);
        
        // 7. Identify consensus and disagreement areas
        let (consensus_areas, disagreement_areas) = self.identify_agreement_areas(&viewpoints);
        
        // 8. Generate recommended approach
        let recommended_approach = self.generate_recommended_approach(&viewpoints, &reliability_assessment);
        
        Ok(MultipleViewpointsResult {
            is_controversial: true,
            controversy_level: controversy_analysis.level,
            viewpoints,
            consensus_areas,
            disagreement_areas,
            recommended_approach,
            internal_guidance,
            source_reliability_assessment: reliability_assessment,
            summary,
        })
    }
    
    fn create_viewpoints_summary(&self, viewpoints: &[ScholarlyViewpoint]) -> ViewpointsSummary {
        let total_viewpoints = viewpoints.len();
        let madhabs_represented: Vec<IslamicMadhab> = viewpoints
            .iter()
            .map(|v| v.madhab.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        
        // Calculate consensus percentage
        let consensus_count = viewpoints
            .iter()
            .filter(|v| matches!(v.strength_level, ViewpointStrength::Consensus))
            .count();
        
        let consensus_percentage = if total_viewpoints > 0 {
            (consensus_count as f32 / total_viewpoints as f32) * 100.0
        } else {
            0.0
        };
        
        // Find main disagreement
        let main_disagreement = if viewpoints.len() > 1 {
            Some("اختلاف في التطبيق العملي للحكم".to_string())
        } else {
            None
        };
        
        // Generate practical recommendation
        let practical_recommendation = if consensus_percentage > 70.0 {
            "اتباع الرأي الراجح مع احترام الخلاف المعتبر".to_string()
        } else {
            "استشارة العلماء المختصين لاختيار الرأي المناسب للحالة".to_string()
        };
        
        ViewpointsSummary {
            total_viewpoints,
            madhabs_represented,
            consensus_percentage,
            main_disagreement,
            practical_recommendation,
        }
    }
    
    fn identify_agreement_areas(&self, viewpoints: &[ScholarlyViewpoint]) -> (Vec<String>, Vec<String>) {
        let mut consensus_areas = Vec::new();
        let mut disagreement_areas = Vec::new();
        
        if viewpoints.is_empty() {
            return (consensus_areas, disagreement_areas);
        }
        
        // Analyze common elements across viewpoints
        let mut common_evidence_types = HashMap::new();
        let mut position_variations = HashSet::new();
        
        for viewpoint in viewpoints {
            position_variations.insert(viewpoint.position.clone());
            
            for evidence in &viewpoint.evidence {
                let evidence_type = format!("{:?}", evidence.content_type);
                *common_evidence_types.entry(evidence_type).or_insert(0) += 1;
            }
        }
        
        // Identify consensus areas
        for (evidence_type, count) in common_evidence_types {
            if count >= (viewpoints.len() as f32 * 0.7) as usize {
                consensus_areas.push(format!("الاتفاق على الاستدلال بـ{}", evidence_type));
            }
        }
        
        if consensus_areas.is_empty() {
            consensus_areas.push("الاتفاق على أهمية الموضوع وضرورة البحث فيه".to_string());
        }
        
        // Identify disagreement areas
        if position_variations.len() > 1 {
            disagreement_areas.push("اختلاف في التطبيق العملي".to_string());
            disagreement_areas.push("تباين في تفسير النصوص".to_string());
        }
        
        (consensus_areas, disagreement_areas)
    }
    
    fn generate_recommended_approach(
        &self,
        viewpoints: &[ScholarlyViewpoint],
        reliability_assessment: &SourceReliabilityAssessment,
    ) -> String {
        if viewpoints.is_empty() {
            return "الرجوع إلى العلماء المختصين".to_string();
        }
        
        let high_reliability_count = reliability_assessment.source_breakdown
            .values()
            .filter(|score| score.score > 0.8)
            .count();
        
        let total_sources = reliability_assessment.source_breakdown.len();
        
        if high_reliability_count as f32 / total_sources as f32 > 0.7 {
            "يمكن الاعتماد على الآراء المدعومة بالمصادر الموثوقة مع مراعاة السياق".to_string()
        } else {
            "يُنصح بالتحقق من مصادر إضافية واستشارة العلماء المعاصرين".to_string()
        }
    }
}

/// Detector for controversial questions
pub struct ControlversyDetector {
    controversial_keywords: HashSet<String>,
    controversial_topics: HashMap<String, ControlversyLevel>,
}

#[derive(Debug, Clone)]
pub struct ControlversyAnalysis {
    pub is_controversial: bool,
    pub level: ControlversyLevel,
    pub indicators: Vec<String>,
    pub confidence: f32,
}

impl ControlversyDetector {
    pub fn new() -> Self {
        let controversial_keywords = [
            "خلاف", "اختلاف", "اختلف", "مذهب", "مذاهب", "آراء", "أقوال",
            "رأي", "قول", "وجه", "احتمال", "جائز", "مكروه", "مستحب",
            "خلافي", "محل نزاع", "محل خلاف", "فيه نظر"
        ].iter().map(|s| s.to_string()).collect();
        
        let mut controversial_topics = HashMap::new();
        
        // Major controversial topics
        controversial_topics.insert("رفع اليدين في الصلاة".to_string(), ControlversyLevel::Moderate);
        controversial_topics.insert("المسح على الخفين".to_string(), ControlversyLevel::Minor);
        controversial_topics.insert("قراءة الفاتحة خلف الإمام".to_string(), ControlversyLevel::Moderate);
        controversial_topics.insert("حكم الموسيقى".to_string(), ControlversyLevel::Significant);
        controversial_topics.insert("التصوير".to_string(), ControlversyLevel::Moderate);
        controversial_topics.insert("الربا".to_string(), ControlversyLevel::Minor);
        controversial_topics.insert("النقاب".to_string(), ControlversyLevel::Significant);
        controversial_topics.insert("صلاة التراويح".to_string(), ControlversyLevel::Minor);
        
        Self {
            controversial_keywords,
            controversial_topics,
        }
    }
    
    pub async fn analyze_controversy(
        &self,
        question: &ProcessedQuestion,
        sources: &[ScoredSource],
    ) -> Result<ControlversyAnalysis> {
        let mut indicators = Vec::new();
        let mut controversy_score = 0.0;
        
        let question_lower = question.normalized_text.to_lowercase();
        
        // Check for controversial keywords
        let keyword_matches: Vec<String> = self.controversial_keywords
            .iter()
            .filter(|keyword| question_lower.contains(&keyword.to_lowercase()))
            .cloned()
            .collect();
        
        if !keyword_matches.is_empty() {
            controversy_score += 0.4;
            indicators.push(format!("كلمات دالة على الخلاف: {}", keyword_matches.join(", ")));
        }
        
        // Check for known controversial topics
        for (topic, level) in &self.controversial_topics {
            if question_lower.contains(&topic.to_lowercase()) {
                controversy_score += match level {
                    ControlversyLevel::Major => 0.8,
                    ControlversyLevel::Significant => 0.6,
                    ControlversyLevel::Moderate => 0.4,
                    ControlversyLevel::Minor => 0.2,
                    ControlversyLevel::None => 0.0,
                };
                indicators.push(format!("موضوع خلافي معروف: {}", topic));
                break;
            }
        }
        
        // Check source diversity (multiple madhabs indicate controversy)
        let madhab_diversity = self.assess_madhab_diversity(sources);
        if madhab_diversity > 0.5 {
            controversy_score += 0.3;
            indicators.push("تنوع في المصادر المذهبية".to_string());
        }
        
        // Check for conflicting source types
        let has_conflicting_sources = self.has_conflicting_sources(sources);
        if has_conflicting_sources {
            controversy_score += 0.2;
            indicators.push("مصادر متضاربة".to_string());
        }
        
        let is_controversial = controversy_score > 0.3;
        let level = if controversy_score > 0.8 {
            ControlversyLevel::Major
        } else if controversy_score > 0.6 {
            ControlversyLevel::Significant
        } else if controversy_score > 0.4 {
            ControlversyLevel::Moderate
        } else if controversy_score > 0.2 {
            ControlversyLevel::Minor
        } else {
            ControlversyLevel::None
        };
        
        Ok(ControlversyAnalysis {
            is_controversial,
            level,
            indicators,
            confidence: controversy_score.min(1.0),
        })
    }
    
    fn assess_madhab_diversity(&self, sources: &[ScoredSource]) -> f32 {
        // Simple heuristic: if we have sources from different types, it might indicate diversity
        let source_types: HashSet<_> = sources
            .iter()
            .map(|s| &s.source.content_type)
            .collect();
        
        // More diverse source types might indicate different perspectives
        match source_types.len() {
            0..=1 => 0.0,
            2 => 0.3,
            3 => 0.5,
            4 => 0.7,
            _ => 0.9,
        }
    }
    
    fn has_conflicting_sources(&self, sources: &[ScoredSource]) -> bool {
        // Check if we have both strong and weak hadiths, which might indicate disagreement
        let has_sahih = sources.iter().any(|s| matches!(s.source.content_type, SourceType::SahihHadith));
        let has_daif = sources.iter().any(|s| matches!(s.source.content_type, SourceType::DaifHadith));
        
        has_sahih && has_daif
    }
}

/// Classifier for Islamic madhabs (schools of thought)
pub struct MadhabClassifier {
    madhab_indicators: HashMap<IslamicMadhab, Vec<String>>,
    scholar_madhab_mapping: HashMap<String, IslamicMadhab>,
}

#[derive(Debug, Clone)]
pub struct MadhabClassification {
    pub sources_by_madhab: HashMap<IslamicMadhab, Vec<ScoredSource>>,
    pub confidence_scores: HashMap<IslamicMadhab, f32>,
}

impl MadhabClassifier {
    pub fn new() -> Self {
        let mut madhab_indicators = HashMap::new();
        
        // Hanafi indicators
        madhab_indicators.insert(IslamicMadhab::Hanafi, vec![
            "أبو حنيفة".to_string(),
            "الحنفي".to_string(),
            "المذهب الحنفي".to_string(),
            "الهداية".to_string(),
            "البدائع".to_string(),
        ]);
        
        // Maliki indicators
        madhab_indicators.insert(IslamicMadhab::Maliki, vec![
            "مالك".to_string(),
            "المالكي".to_string(),
            "المذهب المالكي".to_string(),
            "الموطأ".to_string(),
            "المدونة".to_string(),
        ]);
        
        // Shafii indicators
        madhab_indicators.insert(IslamicMadhab::Shafii, vec![
            "الشافعي".to_string(),
            "المذهب الشافعي".to_string(),
            "الأم".to_string(),
            "المهذب".to_string(),
        ]);
        
        // Hanbali indicators
        madhab_indicators.insert(IslamicMadhab::Hanbali, vec![
            "أحمد".to_string(),
            "الحنبلي".to_string(),
            "المذهب الحنبلي".to_string(),
            "المسند".to_string(),
            "المغني".to_string(),
        ]);
        
        let mut scholar_madhab_mapping = HashMap::new();
        
        // Map famous scholars to their madhabs
        scholar_madhab_mapping.insert("أبو حنيفة".to_string(), IslamicMadhab::Hanafi);
        scholar_madhab_mapping.insert("مالك بن أنس".to_string(), IslamicMadhab::Maliki);
        scholar_madhab_mapping.insert("الشافعي".to_string(), IslamicMadhab::Shafii);
        scholar_madhab_mapping.insert("أحمد بن حنبل".to_string(), IslamicMadhab::Hanbali);
        scholar_madhab_mapping.insert("ابن تيمية".to_string(), IslamicMadhab::Hanbali);
        scholar_madhab_mapping.insert("ابن القيم".to_string(), IslamicMadhab::Hanbali);
        scholar_madhab_mapping.insert("النووي".to_string(), IslamicMadhab::Shafii);
        
        Self {
            madhab_indicators,
            scholar_madhab_mapping,
        }
    }
    
    pub async fn classify_sources_by_madhab(
        &self,
        sources: &[ScoredSource],
    ) -> Result<MadhabClassification> {
        let mut sources_by_madhab: HashMap<IslamicMadhab, Vec<ScoredSource>> = HashMap::new();
        let mut confidence_scores: HashMap<IslamicMadhab, f32> = HashMap::new();
        
        for source in sources {
            let madhab = self.classify_single_source(&source.source).await?;
            let confidence = self.calculate_classification_confidence(&source.source, &madhab);
            
            sources_by_madhab
                .entry(madhab.clone())
                .or_insert_with(Vec::new)
                .push(source.clone());
            
            confidence_scores
                .entry(madhab)
                .and_modify(|e| *e = (*e + confidence) / 2.0)
                .or_insert(confidence);
        }
        
        Ok(MadhabClassification {
            sources_by_madhab,
            confidence_scores,
        })
    }
    
    async fn classify_single_source(&self, source: &IslamicSource) -> Result<IslamicMadhab> {
        // Check author first
        if let Some(author) = &source.author {
            if let Some(madhab) = self.scholar_madhab_mapping.get(author) {
                return Ok(madhab.clone());
            }
        }
        
        // Check reference and text for madhab indicators
        let text_to_check = format!("{} {}", 
            source.reference.to_lowercase(), 
            source.text.to_lowercase()
        );
        
        for (madhab, indicators) in &self.madhab_indicators {
            for indicator in indicators {
                if text_to_check.contains(&indicator.to_lowercase()) {
                    return Ok(madhab.clone());
                }
            }
        }
        
        // Default to General if no specific madhab detected
        Ok(IslamicMadhab::General)
    }
    
    fn calculate_classification_confidence(&self, source: &IslamicSource, madhab: &IslamicMadhab) -> f32 {
        let mut confidence: f32 = 0.5; // Base confidence
        
        // Higher confidence if author is known
        if let Some(author) = &source.author {
            if self.scholar_madhab_mapping.contains_key(author) {
                confidence += 0.4;
            }
        }
        
        // Higher confidence for specific madhab indicators
        if *madhab != IslamicMadhab::General {
            confidence += 0.2;
        }
        
        // Adjust based on source type
        match source.content_type {
            SourceType::FiqhRuling | SourceType::ScholarOpinion => confidence += 0.1,
            SourceType::Quran => confidence = 1.0, // Quran is universal
            _ => {}
        }
        
        confidence.min(1.0)
    }
}

/// Aggregator for different scholarly viewpoints
pub struct ViewpointAggregator;

impl ViewpointAggregator {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn aggregate_viewpoints(
        &self,
        question: &ProcessedQuestion,
        madhab_classification: &MadhabClassification,
    ) -> Result<Vec<ScholarlyViewpoint>> {
        let mut viewpoints = Vec::new();
        
        for (madhab, sources) in &madhab_classification.sources_by_madhab {
            if sources.is_empty() {
                continue;
            }
            
            let viewpoint = self.create_viewpoint_from_sources(
                madhab,
                sources,
                question,
            ).await?;
            
            viewpoints.push(viewpoint);
        }
        
        // Sort viewpoints by strength
        viewpoints.sort_by(|a, b| {
            b.strength_level.to_score()
                .partial_cmp(&a.strength_level.to_score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(viewpoints)
    }
    
    async fn create_viewpoint_from_sources(
        &self,
        madhab: &IslamicMadhab,
        sources: &[ScoredSource],
        question: &ProcessedQuestion,
    ) -> Result<ScholarlyViewpoint> {
        let id = format!("viewpoint_{}_{}", 
            madhab.to_arabic(), 
            chrono::Utc::now().timestamp()
        );
        
        // Extract position from sources
        let position = self.extract_position_from_sources(sources, question);
        
        // Collect evidence
        let evidence: Vec<IslamicSource> = sources
            .iter()
            .map(|s| s.source.clone())
            .collect();
        
        // Generate reasoning
        let reasoning = self.generate_reasoning(madhab, sources, question);
        
        // Extract prominent scholars
        let prominent_scholars = self.extract_scholars(sources);
        
        // Determine strength level
        let strength_level = self.determine_strength_level(sources);
        
        // Extract conditions and exceptions
        let conditions = self.extract_conditions(sources);
        let exceptions = self.extract_exceptions(sources);
        
        // Generate modern applications
        let modern_applications = self.generate_modern_applications(question, madhab);
        
        Ok(ScholarlyViewpoint {
            id,
            madhab: madhab.clone(),
            position,
            evidence,
            reasoning,
            prominent_scholars,
            strength_level,
            conditions,
            exceptions,
            modern_applications,
        })
    }
    
    fn extract_position_from_sources(&self, sources: &[ScoredSource], question: &ProcessedQuestion) -> String {
        // Simple heuristic: use the highest scored source's content as basis for position
        if let Some(best_source) = sources.first() {
            match question.question_type {
                QuestionType::Fiqh => {
                    format!("الحكم الشرعي بناءً على {}", best_source.source.reference)
                },
                QuestionType::Tafsir => {
                    format!("التفسير المعتمد في هذا المذهب")
                },
                _ => {
                    format!("الرأي المعتمد في هذه المسألة")
                }
            }
        } else {
            "لا يوجد موقف واضح".to_string()
        }
    }
    
    fn generate_reasoning(&self, madhab: &IslamicMadhab, sources: &[ScoredSource], question: &ProcessedQuestion) -> String {
        let madhab_name = madhab.to_arabic();
        
        let evidence_types: Vec<String> = sources
            .iter()
            .map(|s| match s.source.content_type {
                SourceType::Quran => "القرآن الكريم",
                SourceType::SahihHadith => "الحديث الصحيح",
                SourceType::HasanHadith => "الحديث الحسن",
                SourceType::Tafsir => "التفسير",
                SourceType::FiqhRuling => "الحكم الفقهي",
                _ => "المصدر الإسلامي",
            })
            .map(|s| s.to_string())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        
        format!(
            "يستند المذهب {} في هذه المسألة إلى {}. وهذا الرأي مبني على فهم النصوص وتطبيق الأصول الفقهية المعتمدة في هذا المذهب.",
            madhab_name,
            evidence_types.join(" و")
        )
    }
    
    fn extract_scholars(&self, sources: &[ScoredSource]) -> Vec<String> {
        sources
            .iter()
            .filter_map(|s| s.source.author.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
    
    fn determine_strength_level(&self, sources: &[ScoredSource]) -> ViewpointStrength {
        let avg_score: f32 = sources
            .iter()
            .map(|s| s.score.final_score)
            .sum::<f32>() / sources.len() as f32;
        
        let has_quran = sources.iter().any(|s| matches!(s.source.content_type, SourceType::Quran));
        let has_sahih_hadith = sources.iter().any(|s| matches!(s.source.content_type, SourceType::SahihHadith));
        
        if has_quran && has_sahih_hadith && avg_score > 0.9 {
            ViewpointStrength::Strong
        } else if has_quran || has_sahih_hadith {
            ViewpointStrength::Moderate
        } else if avg_score > 0.7 {
            ViewpointStrength::Moderate
        } else {
            ViewpointStrength::Weak
        }
    }
    
    fn extract_conditions(&self, sources: &[ScoredSource]) -> Vec<String> {
        // Simple heuristic: look for conditional language in sources
        let mut conditions = Vec::new();
        
        for source in sources {
            let text_lower = source.source.text.to_lowercase();
            if text_lower.contains("إذا") || text_lower.contains("بشرط") || text_lower.contains("عند") {
                conditions.push("يشترط توفر الشروط المذكورة في المصدر".to_string());
                break;
            }
        }
        
        if conditions.is_empty() {
            conditions.push("لا توجد شروط خاصة مذكورة".to_string());
        }
        
        conditions
    }
    
    fn extract_exceptions(&self, sources: &[ScoredSource]) -> Vec<String> {
        let mut exceptions = Vec::new();
        
        for source in sources {
            let text_lower = source.source.text.to_lowercase();
            if text_lower.contains("إلا") || text_lower.contains("غير") || text_lower.contains("ما عدا") {
                exceptions.push("توجد استثناءات مذكورة في المصادر".to_string());
                break;
            }
        }
        
        if exceptions.is_empty() {
            exceptions.push("لا توجد استثناءات خاصة".to_string());
        }
        
        exceptions
    }
    
    fn generate_modern_applications(&self, question: &ProcessedQuestion, madhab: &IslamicMadhab) -> Vec<String> {
        let mut applications = Vec::new();
        
        match question.question_type {
            QuestionType::Fiqh => {
                applications.push("يمكن تطبيق هذا الحكم في الحياة المعاصرة".to_string());
                applications.push("يُنصح بمراجعة العلماء المعاصرين للتطبيق العملي".to_string());
            },
            QuestionType::Aqeedah => {
                applications.push("هذا المفهوم العقدي ثابت عبر العصور".to_string());
            },
            _ => {
                applications.push("يمكن الاستفادة من هذا الرأي في السياق المعاصر".to_string());
            }
        }
        
        applications
    }
}

/// Evaluator for source reliability
pub struct SourceReliabilityEvaluator;

impl SourceReliabilityEvaluator {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn evaluate_sources(
        &self,
        sources: &[ScoredSource],
        viewpoints: &[ScholarlyViewpoint],
    ) -> Result<SourceReliabilityAssessment> {
        let mut source_breakdown = HashMap::new();
        let mut reliability_factors = Vec::new();
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        
        let mut total_reliability = 0.0;
        let mut source_count = 0;
        
        for source in sources {
            let reliability_score = self.calculate_source_reliability(&source.source).await?;
            source_breakdown.insert(source.source.id.clone(), reliability_score.clone());
            
            total_reliability += reliability_score.score;
            source_count += 1;
            
            // Add warnings for unreliable sources
            if matches!(reliability_score.classification, ReliabilityClassification::Questionable | ReliabilityClassification::Unreliable) {
                warnings.push(format!("مصدر مشكوك فيه: {}", source.source.reference));
            }
        }
        
        let overall_reliability = if source_count > 0 {
            total_reliability / source_count as f32
        } else {
            0.0
        };
        
        // Generate reliability factors
        reliability_factors.push(format!("تم تقييم {} مصدر", source_count));
        
        let high_reliability_count = source_breakdown
            .values()
            .filter(|score| score.score > 0.8)
            .count();
        
        if high_reliability_count > 0 {
            reliability_factors.push(format!("{} مصدر عالي الموثوقية", high_reliability_count));
        }
        
        // Generate recommendations
        if overall_reliability < 0.6 {
            recommendations.push("يُنصح بالبحث عن مصادر إضافية أكثر موثوقية".to_string());
        }
        
        if viewpoints.len() > 2 {
            recommendations.push("نظراً لتعدد الآراء، يُنصح باستشارة العلماء المختصين".to_string());
        }
        
        recommendations.push("التحقق من المصادر الأصلية قبل التطبيق العملي".to_string());
        
        Ok(SourceReliabilityAssessment {
            overall_reliability,
            source_breakdown,
            reliability_factors,
            warnings,
            recommendations,
        })
    }
    
    async fn calculate_source_reliability(&self, source: &IslamicSource) -> Result<SourceReliabilityScore> {
        let mut score = 0.5; // Base score
        let mut factors = Vec::new();
        
        // Score based on content type
        let type_score = match source.content_type {
            SourceType::Quran => {
                score = 1.0;
                factors.push("القرآن الكريم - موثوقية مطلقة".to_string());
                1.0
            },
            SourceType::SahihHadith => {
                score += 0.4;
                factors.push("حديث صحيح".to_string());
                0.95
            },
            SourceType::HasanHadith => {
                score += 0.3;
                factors.push("حديث حسن".to_string());
                0.85
            },
            SourceType::Tafsir => {
                score += 0.2;
                factors.push("تفسير معتمد".to_string());
                0.8
            },
            SourceType::FiqhRuling => {
                score += 0.15;
                factors.push("حكم فقهي".to_string());
                0.75
            },
            SourceType::DaifHadith => {
                score -= 0.2;
                factors.push("حديث ضعيف - يحتاج تحقق".to_string());
                0.4
            },
            SourceType::MawduHadith => {
                score = 0.1;
                factors.push("حديث موضوع - غير موثوق".to_string());
                0.1
            },
            _ => 0.6,
        };
        
        // Score based on authenticity level
        let auth_score = match source.authenticity {
            AuthenticityLevel::Verified => {
                score += 0.2;
                factors.push("مصدر محقق".to_string());
                0.2
            },
            AuthenticityLevel::Reliable => {
                score += 0.1;
                factors.push("مصدر موثوق".to_string());
                0.1
            },
            AuthenticityLevel::Questionable => {
                score -= 0.1;
                factors.push("مصدر مشكوك فيه".to_string());
                -0.1
            },
            AuthenticityLevel::Unreliable => {
                score -= 0.3;
                factors.push("مصدر غير موثوق".to_string());
                -0.3
            },
            AuthenticityLevel::Unknown => 0.0,
        };
        
        // Score based on author reputation
        if let Some(author) = &source.author {
            let famous_scholars = [
                "البخاري", "مسلم", "ابن كثير", "الطبري", "القرطبي",
                "النووي", "ابن تيمية", "ابن القيم", "الشافعي", "مالك"
            ];
            
            if famous_scholars.contains(&author.as_str()) {
                score += 0.15;
                factors.push(format!("مؤلف مشهور: {}", author));
            }
        }
        
        score = score.clamp(0.0, 1.0);
        
        let classification = if score > 0.9 {
            ReliabilityClassification::HighlyReliable
        } else if score > 0.7 {
            ReliabilityClassification::Reliable
        } else if score > 0.5 {
            ReliabilityClassification::ModeratelyReliable
        } else if score > 0.3 {
            ReliabilityClassification::Questionable
        } else {
            ReliabilityClassification::Unreliable
        };
        
        Ok(SourceReliabilityScore {
            score,
            factors,
            classification,
        })
    }
}

/// Generator for internal guidance to detailed sources
pub struct InternalGuidanceGenerator;

impl InternalGuidanceGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn generate_guidance(
        &self,
        question: &ProcessedQuestion,
        viewpoints: &[ScholarlyViewpoint],
    ) -> Result<Vec<InternalGuidance>> {
        let mut guidance = Vec::new();
        
        // Generate guidance based on question type
        match question.question_type {
            QuestionType::Fiqh => {
                guidance.extend(self.generate_fiqh_guidance(question, viewpoints).await?);
            },
            QuestionType::Tafsir => {
                guidance.extend(self.generate_tafsir_guidance(question, viewpoints).await?);
            },
            QuestionType::Hadith => {
                guidance.extend(self.generate_hadith_guidance(question, viewpoints).await?);
            },
            QuestionType::Aqeedah => {
                guidance.extend(self.generate_aqeedah_guidance(question, viewpoints).await?);
            },
            _ => {
                guidance.extend(self.generate_general_guidance(question, viewpoints).await?);
            }
        }
        
        // Sort by relevance score
        guidance.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        
        Ok(guidance)
    }
    
    async fn generate_fiqh_guidance(
        &self,
        question: &ProcessedQuestion,
        viewpoints: &[ScholarlyViewpoint],
    ) -> Result<Vec<InternalGuidance>> {
        let mut guidance = Vec::new();
        
        // Guide to comparative fiqh section
        guidance.push(InternalGuidance {
            source_type: InternalSourceType::ComparativeStudy,
            reference_path: "/app/fiqh/comparative-studies".to_string(),
            description: "دراسة مقارنة للآراء الفقهية في هذه المسألة".to_string(),
            relevance_score: 0.9,
            recommended_sections: vec![
                "مقارنة المذاهب الأربعة".to_string(),
                "الأدلة والترجيح".to_string(),
            ],
        });
        
        // Guide to specific madhab sections
        for viewpoint in viewpoints {
            let madhab_path = format!("/app/fiqh/madhabs/{}", viewpoint.madhab.to_arabic());
            guidance.push(InternalGuidance {
                source_type: InternalSourceType::FiqhRuling,
                reference_path: madhab_path,
                description: format!("الأحكام الفقهية في المذهب {}", viewpoint.madhab.to_arabic()),
                relevance_score: 0.8,
                recommended_sections: vec![
                    "الأحكام الأساسية".to_string(),
                    "التطبيقات المعاصرة".to_string(),
                ],
            });
        }
        
        Ok(guidance)
    }
    
    async fn generate_tafsir_guidance(
        &self,
        question: &ProcessedQuestion,
        viewpoints: &[ScholarlyViewpoint],
    ) -> Result<Vec<InternalGuidance>> {
        let mut guidance = Vec::new();
        
        // Guide to tafsir comparison
        guidance.push(InternalGuidance {
            source_type: InternalSourceType::TafsirChapter,
            reference_path: "/app/tafsir/comparative".to_string(),
            description: "مقارنة تفاسير الآية من كتب التفسير المختلفة".to_string(),
            relevance_score: 0.9,
            recommended_sections: vec![
                "تفسير ابن كثير".to_string(),
                "تفسير الطبري".to_string(),
                "تفسير القرطبي".to_string(),
            ],
        });
        
        Ok(guidance)
    }
    
    async fn generate_hadith_guidance(
        &self,
        question: &ProcessedQuestion,
        viewpoints: &[ScholarlyViewpoint],
    ) -> Result<Vec<InternalGuidance>> {
        let mut guidance = Vec::new();
        
        // Guide to hadith verification
        guidance.push(InternalGuidance {
            source_type: InternalSourceType::HadithCollection,
            reference_path: "/app/hadith/verification".to_string(),
            description: "تحقيق الأحاديث ودراسة الأسانيد".to_string(),
            relevance_score: 0.9,
            recommended_sections: vec![
                "درجة الحديث".to_string(),
                "دراسة السند".to_string(),
                "شروح العلماء".to_string(),
            ],
        });
        
        Ok(guidance)
    }
    
    async fn generate_aqeedah_guidance(
        &self,
        question: &ProcessedQuestion,
        viewpoints: &[ScholarlyViewpoint],
    ) -> Result<Vec<InternalGuidance>> {
        let mut guidance = Vec::new();
        
        // Guide to creed studies
        guidance.push(InternalGuidance {
            source_type: InternalSourceType::ScholarlyArticle,
            reference_path: "/app/aqeedah/studies".to_string(),
            description: "دراسات عقدية معاصرة في هذا الموضوع".to_string(),
            relevance_score: 0.8,
            recommended_sections: vec![
                "الأدلة من القرآن والسنة".to_string(),
                "موقف أهل السنة والجماعة".to_string(),
            ],
        });
        
        Ok(guidance)
    }
    
    async fn generate_general_guidance(
        &self,
        question: &ProcessedQuestion,
        viewpoints: &[ScholarlyViewpoint],
    ) -> Result<Vec<InternalGuidance>> {
        let mut guidance = Vec::new();
        
        // General guidance to related topics
        guidance.push(InternalGuidance {
            source_type: InternalSourceType::ScholarlyArticle,
            reference_path: "/app/general/related-topics".to_string(),
            description: "مواضيع ذات صلة في التطبيق".to_string(),
            relevance_score: 0.6,
            recommended_sections: vec![
                "مواضيع مشابهة".to_string(),
                "مراجع إضافية".to_string(),
            ],
        });
        
        Ok(guidance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_controversy_detection() {
        let detector = ControlversyDetector::new();
        let processor = crate::ai_service::question_processor::QuestionProcessor::new();
        
        let controversial_question = processor
            .process_question("ما الخلاف في رفع اليدين في الصلاة؟")
            .await
            .unwrap();
        
        let analysis = detector
            .analyze_controversy(&controversial_question, &[])
            .await
            .unwrap();
        
        assert!(analysis.is_controversial);
        assert!(!analysis.indicators.is_empty());
        assert!(analysis.confidence > 0.3);
    }
    
    #[tokio::test]
    async fn test_madhab_classification() {
        let classifier = MadhabClassifier::new();
        
        let hanafi_source = IslamicSource {
            id: "test_hanafi".to_string(),
            content_type: SourceType::FiqhRuling,
            text: "رأي أبو حنيفة في هذه المسألة".to_string(),
            reference: "الهداية".to_string(),
            author: Some("أبو حنيفة".to_string()),
            authenticity: AuthenticityLevel::Verified,
            language: Language::Arabic,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        };
        
        let madhab = classifier.classify_single_source(&hanafi_source).await.unwrap();
        assert_eq!(madhab, IslamicMadhab::Hanafi);
    }
    
    #[tokio::test]
    async fn test_multiple_viewpoints_system() {
        let system = MultipleViewpointsSystem::new();
        let processor = crate::ai_service::question_processor::QuestionProcessor::new();
        
        let question = processor
            .process_question("ما آراء المذاهب في المسح على الخفين؟")
            .await
            .unwrap();
        
        // Create mock sources
        let sources = vec![
            ScoredSource {
                source: IslamicSource {
                    id: "hanafi_source".to_string(),
                    content_type: SourceType::FiqhRuling,
                    text: "يجوز المسح على الخفين عند الحنفية".to_string(),
                    reference: "الهداية".to_string(),
                    author: Some("أبو حنيفة".to_string()),
                    authenticity: AuthenticityLevel::Verified,
                    language: Language::Arabic,
                    metadata: HashMap::new(),
                    created_at: chrono::Utc::now(),
                },
                score: crate::ai_service::source_scorer::SourceScore {
                    relevance_score: 0.9,
                    authority_score: 0.8,
                    authenticity_score: 0.9,
                    consensus_score: 0.7,
                    freshness_score: 0.8,
                    final_score: 0.84,
                    confidence_level: ConfidenceLevel::High,
                    scoring_details: crate::ai_service::source_scorer::ScoringDetails {
                        relevance_factors: vec![],
                        authority_factors: vec![],
                        authenticity_factors: vec![],
                        consensus_factors: vec![],
                        penalties: vec![],
                        bonuses: vec![],
                    },
                },
                rank: 1,
                usage_recommendation: SourceUsageRecommendation::Primary,
            }
        ];
        
        let result = system.analyze_viewpoints(&question, &sources).await.unwrap();
        
        assert!(result.is_controversial);
        assert!(!result.viewpoints.is_empty());
        assert!(!result.internal_guidance.is_empty());
    }
}
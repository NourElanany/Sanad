use super::*;
use regex::Regex;
use std::collections::HashSet;

/// Anti-hallucination system for detecting and preventing fabricated content
pub struct AntiHallucinationSystem {
    fact_checker: FactChecker,
    source_verifier: SourceVerifier,
    consistency_checker: ConsistencyChecker,
    confidence_assessor: ConfidenceAssessor,
    quran_verifier: QuranVerifier,
    hadith_verifier: HadithContentVerifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HallucinationCheckResult {
    pub is_hallucination_detected: bool,
    pub hallucination_risk_score: f32,
    pub unsupported_claims: Vec<UnsupportedClaim>,
    pub contradictions: Vec<Contradiction>,
    pub fabricated_content: Vec<FabricatedContent>,
    pub confidence_score: f32,
    pub recommendation: ResponseRecommendation,
    pub required_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsupportedClaim {
    pub claim: String,
    pub position: TextPosition,
    pub severity: ClaimSeverity,
    pub suggested_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contradiction {
    pub claim: String,
    pub contradicting_source: IslamicSource,
    pub severity: ContradictionSeverity,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricatedContent {
    pub content: String,
    pub content_type: FabricationType,
    pub confidence: f32,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPosition {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClaimSeverity {
    Critical,    // ادعاء خطير (آية أو حديث مختلق)
    Major,       // ادعاء كبير (حكم فقهي بدون مصدر)
    Minor,       // ادعاء طفيف (معلومة تاريخية)
    Stylistic,   // اختلاف في الأسلوب فقط
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContradictionSeverity {
    Critical,    // تناقض خطير مع النصوص الثابتة
    Major,       // تناقض كبير مع الإجماع
    Minor,       // تناقض طفيف مع بعض الآراء
    Stylistic,   // اختلاف في الأسلوب فقط
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FabricationType {
    FakeAyah,        // آية مختلقة
    FakeHadith,      // حديث مختلق
    FakeQuote,       // قول مختلق لعالم
    FakeRuling,      // حكم فقهي مختلق
    FakeStory,       // قصة مختلقة
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseRecommendation {
    Approve,              // الموافقة على الإجابة
    ApproveWithWarning,   // الموافقة مع تحذير
    RequireRevision,      // تتطلب مراجعة
    RequireSourceCheck,   // تتطلب فحص المصادر
    Reject,              // رفض الإجابة
    RequestHumanReview,  // طلب مراجعة بشرية
}

impl AntiHallucinationSystem {
    pub fn new() -> Self {
        Self {
            fact_checker: FactChecker::new(),
            source_verifier: SourceVerifier::new(),
            consistency_checker: ConsistencyChecker::new(),
            confidence_assessor: ConfidenceAssessor::new(),
            quran_verifier: QuranVerifier::new(),
            hadith_verifier: HadithContentVerifier::new(),
        }
    }
    
    pub async fn check_response(
        &self,
        response_text: &str,
        sources: &[IslamicSource],
        query: &ProcessedQuestion,
    ) -> Result<HallucinationCheckResult> {
        // استخراج الحقائق والادعاءات من الإجابة
        let facts = self.fact_checker.extract_facts(response_text).await?;
        
        // فحص الآيات القرآنية المذكورة
        let quran_check = self.quran_verifier.verify_quran_content(response_text).await?;
        
        // فحص الأحاديث المذكورة
        let hadith_check = self.hadith_verifier.verify_hadith_content(response_text).await?;
        
        // التحقق من دعم المصادر للادعاءات
        let mut unsupported_claims = Vec::new();
        for fact in &facts {
            if !self.source_verifier.verify_fact_support(fact, sources).await? {
                unsupported_claims.push(UnsupportedClaim {
                    claim: fact.claim.clone(),
                    position: fact.position.clone(),
                    severity: self.determine_claim_severity(&fact.claim),
                    suggested_sources: self.suggest_sources_for_claim(&fact.claim).await?,
                });
            }
        }
        
        // فحص التناقضات
        let contradictions = self.consistency_checker
            .check_consistency(response_text, sources).await?;
        
        // جمع المحتوى المختلق
        let mut fabricated_content = Vec::new();
        fabricated_content.extend(quran_check.fabricated_ayahs);
        fabricated_content.extend(hadith_check.fabricated_hadiths);
        
        // حساب مخاطر الاختلاق
        let hallucination_risk = self.calculate_hallucination_risk(
            &unsupported_claims,
            &contradictions,
            &fabricated_content,
        );
        
        // تقييم الثقة العامة
        let confidence = self.confidence_assessor
            .assess_confidence(response_text, sources, query).await?;
        
        // تحديد التوصية
        let recommendation = self.determine_recommendation(
            hallucination_risk,
            confidence,
            &unsupported_claims,
            &fabricated_content,
        );
        
        // تحديد الإجراءات المطلوبة
        let required_actions = self.determine_required_actions(
            &recommendation,
            &unsupported_claims,
            &contradictions,
            &fabricated_content,
        );
        
        Ok(HallucinationCheckResult {
            is_hallucination_detected: hallucination_risk > 0.3,
            hallucination_risk_score: hallucination_risk,
            unsupported_claims,
            contradictions,
            fabricated_content,
            confidence_score: confidence,
            recommendation,
            required_actions,
        })
    }
    
    fn calculate_hallucination_risk(
        &self,
        unsupported_claims: &[UnsupportedClaim],
        contradictions: &[Contradiction],
        fabricated_content: &[FabricatedContent],
    ) -> f32 {
        let mut risk_score = 0.0;
        
        // مخاطر الادعاءات غير المدعومة
        for claim in unsupported_claims {
            let claim_risk = match claim.severity {
                ClaimSeverity::Critical => 0.8,
                ClaimSeverity::Major => 0.6,
                ClaimSeverity::Minor => 0.3,
                ClaimSeverity::Stylistic => 0.1,
            };
            risk_score += claim_risk;
        }
        
        // مخاطر التناقضات
        for contradiction in contradictions {
            let contradiction_risk = match contradiction.severity {
                ContradictionSeverity::Critical => 0.9,
                ContradictionSeverity::Major => 0.7,
                ContradictionSeverity::Minor => 0.4,
                ContradictionSeverity::Stylistic => 0.1,
            };
            risk_score += contradiction_risk;
        }
        
        // مخاطر المحتوى المختلق
        for fabricated in fabricated_content {
            let fabrication_risk = match fabricated.content_type {
                FabricationType::FakeAyah => 1.0,
                FabricationType::FakeHadith => 0.9,
                FabricationType::FakeQuote => 0.7,
                FabricationType::FakeRuling => 0.8,
                FabricationType::FakeStory => 0.5,
            };
            risk_score += fabrication_risk * fabricated.confidence;
        }
        
        // تطبيع النتيجة
        risk_score.min(1.0)
    }
    
    fn determine_claim_severity(&self, claim: &str) -> ClaimSeverity {
        let claim_lower = claim.to_lowercase();
        
        // ادعاءات خطيرة
        if claim_lower.contains("قال الله") || claim_lower.contains("في القرآن") {
            return ClaimSeverity::Critical;
        }
        
        if claim_lower.contains("قال الرسول") || claim_lower.contains("في الحديث") {
            return ClaimSeverity::Critical;
        }
        
        // ادعاءات كبيرة
        if claim_lower.contains("حكم") || claim_lower.contains("فتوى") || claim_lower.contains("إجماع") {
            return ClaimSeverity::Major;
        }
        
        // ادعاءات طفيفة
        if claim_lower.contains("تاريخ") || claim_lower.contains("قصة") {
            return ClaimSeverity::Minor;
        }
        
        ClaimSeverity::Minor
    }
    
    async fn suggest_sources_for_claim(&self, claim: &str) -> Result<Vec<String>> {
        let mut suggestions = Vec::new();
        
        let claim_lower = claim.to_lowercase();
        
        if claim_lower.contains("قرآن") || claim_lower.contains("آية") {
            suggestions.push("المصحف الشريف".to_string());
            suggestions.push("تفسير ابن كثير".to_string());
        }
        
        if claim_lower.contains("حديث") || claim_lower.contains("رسول") {
            suggestions.push("صحيح البخاري".to_string());
            suggestions.push("صحيح مسلم".to_string());
        }
        
        if claim_lower.contains("فقه") || claim_lower.contains("حكم") {
            suggestions.push("كتب الفقه المعتمدة".to_string());
            suggestions.push("فتاوى العلماء المعاصرين".to_string());
        }
        
        Ok(suggestions)
    }
    
    fn determine_recommendation(
        &self,
        hallucination_risk: f32,
        confidence: f32,
        unsupported_claims: &[UnsupportedClaim],
        fabricated_content: &[FabricatedContent],
    ) -> ResponseRecommendation {
        // رفض فوري للمحتوى المختلق الخطير
        if fabricated_content.iter().any(|f| matches!(f.content_type, FabricationType::FakeAyah | FabricationType::FakeHadith)) {
            return ResponseRecommendation::Reject;
        }
        
        // رفض للمخاطر العالية
        if hallucination_risk > 0.7 {
            return ResponseRecommendation::Reject;
        }
        
        // مراجعة بشرية للمخاطر المتوسطة العالية
        if hallucination_risk > 0.5 || confidence < 0.4 {
            return ResponseRecommendation::RequestHumanReview;
        }
        
        // مراجعة مطلوبة للادعاءات الخطيرة غير المدعومة
        if unsupported_claims.iter().any(|c| matches!(c.severity, ClaimSeverity::Critical)) {
            return ResponseRecommendation::RequireSourceCheck;
        }
        
        // مراجعة للمخاطر المتوسطة
        if hallucination_risk > 0.3 {
            return ResponseRecommendation::RequireRevision;
        }
        
        // موافقة مع تحذير للمخاطر المنخفضة
        if hallucination_risk > 0.1 || !unsupported_claims.is_empty() {
            return ResponseRecommendation::ApproveWithWarning;
        }
        
        ResponseRecommendation::Approve
    }
    
    fn determine_required_actions(
        &self,
        recommendation: &ResponseRecommendation,
        unsupported_claims: &[UnsupportedClaim],
        contradictions: &[Contradiction],
        fabricated_content: &[FabricatedContent],
    ) -> Vec<String> {
        let mut actions = Vec::new();
        
        match recommendation {
            ResponseRecommendation::Reject => {
                actions.push("رفض الإجابة وإعادة التوليد".to_string());
            },
            ResponseRecommendation::RequestHumanReview => {
                actions.push("إرسال للمراجعة البشرية".to_string());
            },
            ResponseRecommendation::RequireSourceCheck => {
                actions.push("التحقق من المصادر المذكورة".to_string());
            },
            ResponseRecommendation::RequireRevision => {
                actions.push("مراجعة الإجابة وتصحيحها".to_string());
            },
            ResponseRecommendation::ApproveWithWarning => {
                actions.push("إضافة تحذير للمستخدم".to_string());
            },
            ResponseRecommendation::Approve => {
                // لا حاجة لإجراءات إضافية
            },
        }
        
        // إجراءات محددة للمشاكل المكتشفة
        if !fabricated_content.is_empty() {
            actions.push("إزالة المحتوى المختلق".to_string());
        }
        
        if !unsupported_claims.is_empty() {
            actions.push("إضافة مصادر للادعاءات".to_string());
        }
        
        if !contradictions.is_empty() {
            actions.push("حل التناقضات مع المصادر".to_string());
        }
        
        actions
    }
}

/// Fact checker for extracting and analyzing claims
pub struct FactChecker {
    claim_patterns: Vec<Regex>,
}

#[derive(Debug, Clone)]
pub struct ExtractedFact {
    pub claim: String,
    pub position: TextPosition,
    pub fact_type: FactType,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub enum FactType {
    QuranReference,
    HadithReference,
    ScholarQuote,
    HistoricalFact,
    ReligiousRuling,
    GeneralClaim,
}

impl FactChecker {
    pub fn new() -> Self {
        let claim_patterns = vec![
            Regex::new(r"قال الله تعالى:?\s*[\""]([^\"\"]+)[\""]").unwrap(),
            Regex::new(r"في القرآن:?\s*[\""]([^\"\"]+)[\""]").unwrap(),
            Regex::new(r"قال الرسول:?\s*[\""]([^\"\"]+)[\""]").unwrap(),
            Regex::new(r"في الحديث:?\s*[\""]([^\"\"]+)[\""]").unwrap(),
            Regex::new(r"قال\s+([^:]+):\s*[\""]([^\"\"]+)[\""]").unwrap(),
            Regex::new(r"الحكم\s+في\s+([^:]+):\s*([^.]+)").unwrap(),
        ];
        
        Self { claim_patterns }
    }
    
    pub async fn extract_facts(&self, text: &str) -> Result<Vec<ExtractedFact>> {
        let mut facts = Vec::new();
        
        for (pattern_idx, pattern) in self.claim_patterns.iter().enumerate() {
            for capture in pattern.captures_iter(text) {
                if let Some(claim_match) = capture.get(0) {
                    let fact_type = match pattern_idx {
                        0 | 1 => FactType::QuranReference,
                        2 | 3 => FactType::HadithReference,
                        4 => FactType::ScholarQuote,
                        5 => FactType::ReligiousRuling,
                        _ => FactType::GeneralClaim,
                    };
                    
                    facts.push(ExtractedFact {
                        claim: claim_match.as_str().to_string(),
                        position: TextPosition {
                            start: claim_match.start(),
                            end: claim_match.end(),
                            line: text[..claim_match.start()].matches('\n').count() + 1,
                        },
                        fact_type,
                        confidence: 0.8,
                    });
                }
            }
        }
        
        Ok(facts)
    }
}

/// Source verifier for checking if claims are supported by sources
pub struct SourceVerifier;

impl SourceVerifier {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn verify_fact_support(&self, fact: &ExtractedFact, sources: &[IslamicSource]) -> Result<bool> {
        let claim_lower = fact.claim.to_lowercase();
        
        for source in sources {
            let source_text_lower = source.text.to_lowercase();
            
            // فحص التطابق المباشر
            if source_text_lower.contains(&claim_lower) {
                return Ok(true);
            }
            
            // فحص التطابق الجزئي للآيات والأحاديث
            if matches!(fact.fact_type, FactType::QuranReference | FactType::HadithReference) {
                if self.check_partial_match(&claim_lower, &source_text_lower) {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    fn check_partial_match(&self, claim: &str, source_text: &str) -> bool {
        // إزالة علامات الترقيم والكلمات الشائعة
        let claim_words: HashSet<&str> = claim
            .split_whitespace()
            .filter(|word| word.len() > 2)
            .collect();
        
        let source_words: HashSet<&str> = source_text
            .split_whitespace()
            .filter(|word| word.len() > 2)
            .collect();
        
        let intersection: HashSet<_> = claim_words.intersection(&source_words).collect();
        let union: HashSet<_> = claim_words.union(&source_words).collect();
        
        if union.is_empty() {
            return false;
        }
        
        let similarity = intersection.len() as f32 / union.len() as f32;
        similarity > 0.6 // عتبة التشابه
    }
}

/// Consistency checker for detecting contradictions
pub struct ConsistencyChecker;

impl ConsistencyChecker {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn check_consistency(&self, response_text: &str, sources: &[IslamicSource]) -> Result<Vec<Contradiction>> {
        let mut contradictions = Vec::new();
        
        // هذا مثال مبسط - في التطبيق الحقيقي سيكون أكثر تعقيداً
        for source in sources {
            if let Some(contradiction) = self.find_contradiction(response_text, source).await? {
                contradictions.push(contradiction);
            }
        }
        
        Ok(contradictions)
    }
    
    async fn find_contradiction(&self, response_text: &str, source: &IslamicSource) -> Result<Option<Contradiction>> {
        // فحص التناقضات الأساسية
        // هذا مثال مبسط
        Ok(None)
    }
}

/// Confidence assessor for evaluating overall response confidence
pub struct ConfidenceAssessor;

impl ConfidenceAssessor {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn assess_confidence(
        &self,
        response_text: &str,
        sources: &[IslamicSource],
        query: &ProcessedQuestion,
    ) -> Result<f32> {
        let mut confidence = 0.5;
        
        // عوامل تزيد الثقة
        if !sources.is_empty() {
            confidence += 0.2;
        }
        
        if sources.iter().any(|s| matches!(s.content_type, SourceType::Quran | SourceType::SahihHadith)) {
            confidence += 0.2;
        }
        
        if response_text.len() > 100 && response_text.len() < 1000 {
            confidence += 0.1; // طول مناسب
        }
        
        // عوامل تقلل الثقة
        if query.complexity_level == ComplexityLevel::Scholarly && sources.len() < 3 {
            confidence -= 0.2;
        }
        
        if response_text.contains("لست متأكداً") || response_text.contains("قد يكون") {
            confidence -= 0.1;
        }
        
        Ok(confidence.max(0.0).min(1.0))
    }
}

/// Quran verifier for checking Quranic content
pub struct QuranVerifier;

#[derive(Debug, Clone)]
pub struct QuranVerificationResult {
    pub fabricated_ayahs: Vec<FabricatedContent>,
    pub verified_ayahs: Vec<String>,
}

impl QuranVerifier {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn verify_quran_content(&self, text: &str) -> Result<QuranVerificationResult> {
        let mut fabricated_ayahs = Vec::new();
        let mut verified_ayahs = Vec::new();
        
        // استخراج النصوص التي تبدو كآيات قرآنية
        let quran_pattern = Regex::new(r#"قال الله تعالى:?\s*["""']([^"""']+)["""']"#).unwrap();
        
        for capture in quran_pattern.captures_iter(text) {
            if let Some(ayah_text) = capture.get(1) {
                let ayah = ayah_text.as_str();
                
                // التحقق من وجود الآية في القرآن
                if self.is_valid_ayah(ayah).await? {
                    verified_ayahs.push(ayah.to_string());
                } else {
                    fabricated_ayahs.push(FabricatedContent {
                        content: ayah.to_string(),
                        content_type: FabricationType::FakeAyah,
                        confidence: 0.9,
                        evidence: vec!["لا توجد في المصحف الشريف".to_string()],
                    });
                }
            }
        }
        
        Ok(QuranVerificationResult {
            fabricated_ayahs,
            verified_ayahs,
        })
    }
    
    async fn is_valid_ayah(&self, ayah_text: &str) -> Result<bool> {
        // في التطبيق الحقيقي، سيتم البحث في قاعدة بيانات القرآن
        // هذا مثال مبسط
        Ok(true) // افتراضي للمثال
    }
}

/// Hadith content verifier
pub struct HadithContentVerifier;

#[derive(Debug, Clone)]
pub struct HadithContentVerificationResult {
    pub fabricated_hadiths: Vec<FabricatedContent>,
    pub verified_hadiths: Vec<String>,
}

impl HadithContentVerifier {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn verify_hadith_content(&self, text: &str) -> Result<HadithContentVerificationResult> {
        let mut fabricated_hadiths = Vec::new();
        let mut verified_hadiths = Vec::new();
        
        // استخراج النصوص التي تبدو كأحاديث
        let hadith_pattern = Regex::new(r#"قال الرسول:?\s*["""']([^"""']+)["""']"#).unwrap();
        
        for capture in hadith_pattern.captures_iter(text) {
            if let Some(hadith_text) = capture.get(1) {
                let hadith = hadith_text.as_str();
                
                // التحقق من وجود الحديث
                if self.is_valid_hadith(hadith).await? {
                    verified_hadiths.push(hadith.to_string());
                } else {
                    fabricated_hadiths.push(FabricatedContent {
                        content: hadith.to_string(),
                        content_type: FabricationType::FakeHadith,
                        confidence: 0.8,
                        evidence: vec!["لا يوجد في كتب الحديث المعتمدة".to_string()],
                    });
                }
            }
        }
        
        Ok(HadithContentVerificationResult {
            fabricated_hadiths,
            verified_hadiths,
        })
    }
    
    async fn is_valid_hadith(&self, hadith_text: &str) -> Result<bool> {
        // في التطبيق الحقيقي، سيتم البحث في قاعدة بيانات الأحاديث
        // هذا مثال مبسط
        Ok(true) // افتراضي للمثال
    }
}
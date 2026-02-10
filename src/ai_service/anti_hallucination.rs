use super::*;
use super::question_processor::ProcessedQuestion;
use super::hadith_verifier::HadithGrade as HadithVerifierGrade;
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

// Helper structs for internal processing
#[derive(Debug, Clone)]
struct FiqhRuling {
    ruling: String,
    subject: String,
    position: usize,
}

#[derive(Debug, Clone)]
struct HadithGrade {
    hadith_text: String,
    grade: String,
    position: usize,
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
            let support_level = self.source_verifier.verify_fact_support_detailed(fact, sources).await?;
            if support_level.support_score < 0.3 {
                unsupported_claims.push(UnsupportedClaim {
                    claim: fact.claim.clone(),
                    position: fact.position.clone(),
                    severity: self.determine_claim_severity(&fact.claim, &fact.fact_type),
                    suggested_sources: self.suggest_sources_for_claim(&fact.claim).await?,
                });
            }
        }
        
        // فحص التناقضات المتقدم
        let contradictions = self.consistency_checker
            .check_consistency_advanced(response_text, sources, query).await?;
        
        // فحص الاختلاق المتقدم
        let advanced_fabrication_check = self.detect_advanced_fabrication(response_text, sources).await?;
        
        // جمع المحتوى المختلق
        let mut fabricated_content = Vec::new();
        fabricated_content.extend(quran_check.fabricated_ayahs);
        fabricated_content.extend(hadith_check.fabricated_hadiths);
        fabricated_content.extend(advanced_fabrication_check);
        
        // حساب مخاطر الاختلاق المحسن
        let hallucination_risk = self.calculate_enhanced_hallucination_risk(
            &unsupported_claims,
            &contradictions,
            &fabricated_content,
            response_text,
            query,
        );
        
        // تقييم الثقة العامة
        let confidence = self.confidence_assessor
            .assess_confidence(response_text, sources, query).await?;
        
        // تحديد التوصية المحسنة
        let recommendation = self.determine_enhanced_recommendation(
            hallucination_risk,
            confidence,
            &unsupported_claims,
            &fabricated_content,
            query,
        );
        
        // تحديد الإجراءات المطلوبة المحسنة
        let required_actions = self.determine_enhanced_required_actions(
            &recommendation,
            &unsupported_claims,
            &contradictions,
            &fabricated_content,
            query,
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
    
    async fn detect_advanced_fabrication(&self, response_text: &str, sources: &[IslamicSource]) -> Result<Vec<FabricatedContent>> {
        let mut fabricated_content = Vec::new();
        
        // فحص الأرقام والإحصائيات المشبوهة
        let number_pattern = Regex::new(r#"\d+"#).unwrap();
        for number_match in number_pattern.find_iter(response_text) {
            let number_context = self.get_context_around_match(response_text, number_match.start(), number_match.end());
            if self.is_suspicious_number_claim(&number_context, sources).await? {
                fabricated_content.push(FabricatedContent {
                    content: number_context,
                    content_type: FabricationType::FakeRuling,
                    confidence: 0.7,
                    evidence: vec!["رقم غير مدعوم بالمصادر".to_string()],
                });
            }
        }
        
        // فحص الأسماء والشخصيات المشبوهة
        let name_patterns = vec![
            Regex::new(r#"قال\s+([^:]+):"#).unwrap(),
            Regex::new(r#"ذكر\s+([^:]+)\s+أن"#).unwrap(),
        ];
        
        for pattern in name_patterns {
            for capture in pattern.captures_iter(response_text) {
                if let Some(name_match) = capture.get(1) {
                    let name = name_match.as_str();
                    if !self.is_known_scholar_or_source(name, sources).await? {
                        fabricated_content.push(FabricatedContent {
                            content: format!("قول منسوب إلى: {}", name),
                            content_type: FabricationType::FakeQuote,
                            confidence: 0.8,
                            evidence: vec!["شخصية غير معروفة أو غير مدعومة بالمصادر".to_string()],
                        });
                    }
                }
            }
        }
        
        // فحص الأحكام الفقهية المشبوهة
        let ruling_patterns = vec![
            Regex::new(r"يجب\s+([^.]+)").unwrap(),
            Regex::new(r"يحرم\s+([^.]+)").unwrap(),
            Regex::new(r"يستحب\s+([^.]+)").unwrap(),
        ];
        
        for pattern in ruling_patterns {
            for capture in pattern.captures_iter(response_text) {
                if let Some(ruling_match) = capture.get(1) {
                    let ruling = ruling_match.as_str();
                    if !self.is_ruling_supported_by_sources(ruling, sources).await? {
                        fabricated_content.push(FabricatedContent {
                            content: format!("حكم فقهي: {}", ruling),
                            content_type: FabricationType::FakeRuling,
                            confidence: 0.9,
                            evidence: vec!["حكم غير مدعوم بالمصادر الشرعية".to_string()],
                        });
                    }
                }
            }
        }
        
        Ok(fabricated_content)
    }
    
    fn get_context_around_match(&self, text: &str, start: usize, end: usize) -> String {
        let context_size = 50;
        let text_start = if start >= context_size { start - context_size } else { 0 };
        let text_end = if end + context_size < text.len() { end + context_size } else { text.len() };
        
        text[text_start..text_end].to_string()
    }
    
    async fn is_suspicious_number_claim(&self, context: &str, sources: &[IslamicSource]) -> Result<bool> {
        // فحص إذا كان الرقم مدعوم بالمصادر
        for source in sources {
            if source.text.to_lowercase().contains(&context.to_lowercase()) {
                return Ok(false); // مدعوم
            }
        }
        
        // فحص إذا كان الرقم يبدو مشبوهاً (أرقام كبيرة جداً أو دقيقة جداً)
        let number_pattern = Regex::new(r#"\d+"#).unwrap();
        if let Some(number_match) = number_pattern.find(context) {
            if let Ok(number) = number_match.as_str().parse::<i32>() {
                // أرقام مشبوهة في السياق الإسلامي
                if number > 1000000 || (number > 100 && context.contains("سنة")) {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    async fn is_known_scholar_or_source(&self, name: &str, sources: &[IslamicSource]) -> Result<bool> {
        // قائمة العلماء المعروفين
        let known_scholars = [
            "ابن تيمية", "ابن القيم", "ابن كثير", "الطبري", "القرطبي",
            "البخاري", "مسلم", "أبو داود", "الترمذي", "النسائي", "ابن ماجه",
            "الشافعي", "مالك", "أحمد", "أبو حنيفة", "الألباني", "ابن باز"
        ];
        
        let name_lower = name.to_lowercase();
        
        // فحص في قائمة العلماء المعروفين
        for scholar in &known_scholars {
            if name_lower.contains(&scholar.to_lowercase()) {
                return Ok(true);
            }
        }
        
        // فحص في المصادر المتاحة
        for source in sources {
            if let Some(author) = &source.author {
                if author.to_lowercase().contains(&name_lower) || name_lower.contains(&author.to_lowercase()) {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    async fn is_ruling_supported_by_sources(&self, ruling: &str, sources: &[IslamicSource]) -> Result<bool> {
        let ruling_lower = ruling.to_lowercase();
        
        for source in sources {
            let source_text_lower = source.text.to_lowercase();
            
            // فحص التطابق المباشر
            if source_text_lower.contains(&ruling_lower) {
                return Ok(true);
            }
            
            // فحص التطابق الدلالي للأحكام الفقهية
            if self.check_semantic_ruling_match(&ruling_lower, &source_text_lower) {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    fn check_semantic_ruling_match(&self, ruling: &str, source_text: &str) -> bool {
        // كلمات مفتاحية للأحكام الفقهية
        let ruling_keywords = [
            "واجب", "فرض", "مستحب", "سنة", "مكروه", "حرام", "مباح"
        ];
        
        let ruling_words: HashSet<&str> = ruling.split_whitespace().collect();
        let source_words: HashSet<&str> = source_text.split_whitespace().collect();
        
        // فحص وجود كلمات الأحكام
        let has_ruling_keyword = ruling_keywords.iter()
            .any(|keyword| ruling.contains(keyword) || source_text.contains(keyword));
        
        if !has_ruling_keyword {
            return false;
        }
        
        // حساب التشابه
        let intersection: HashSet<_> = ruling_words.intersection(&source_words).collect();
        let similarity = intersection.len() as f32 / ruling_words.len() as f32;
        
        similarity > 0.4 // عتبة التشابه للأحكام
    }
    
    fn calculate_enhanced_hallucination_risk(
        &self,
        unsupported_claims: &[UnsupportedClaim],
        contradictions: &[Contradiction],
        fabricated_content: &[FabricatedContent],
        response_text: &str,
        query: &ProcessedQuestion,
    ) -> f32 {
        let mut risk_score = 0.0;
        
        // مخاطر الادعاءات غير المدعومة (وزن محسن)
        for claim in unsupported_claims {
            let claim_risk = match claim.severity {
                ClaimSeverity::Critical => 0.9,
                ClaimSeverity::Major => 0.7,
                ClaimSeverity::Minor => 0.4,
                ClaimSeverity::Stylistic => 0.1,
            };
            risk_score += claim_risk;
        }
        
        // مخاطر التناقضات (وزن محسن)
        for contradiction in contradictions {
            let contradiction_risk = match contradiction.severity {
                ContradictionSeverity::Critical => 1.0,
                ContradictionSeverity::Major => 0.8,
                ContradictionSeverity::Minor => 0.5,
                ContradictionSeverity::Stylistic => 0.1,
            };
            risk_score += contradiction_risk;
        }
        
        // مخاطر المحتوى المختلق (وزن محسن)
        for fabricated in fabricated_content {
            let fabrication_risk = match fabricated.content_type {
                FabricationType::FakeAyah => 1.0,
                FabricationType::FakeHadith => 0.95,
                FabricationType::FakeRuling => 0.85,
                FabricationType::FakeQuote => 0.75,
                FabricationType::FakeStory => 0.6,
            };
            risk_score += fabrication_risk * fabricated.confidence;
        }
        
        // عوامل إضافية للمخاطر
        
        // طول الإجابة مقابل التعقيد
        let response_length = response_text.len();
        let expected_length = match query.complexity_level {
            ComplexityLevel::Simple => 200,
            ComplexityLevel::Intermediate => 400,
            ComplexityLevel::Advanced => 600,
            ComplexityLevel::Scholarly => 800,
        };
        
        if response_length > expected_length * 2 {
            risk_score += 0.2; // إجابة طويلة جداً قد تحتوي على اختلاق
        }
        
        // كثرة التفاصيل الدقيقة
        let detail_patterns = [
            r#"\d{4}"#, // سنوات محددة
            r#"\d+\s*%"#, // نسب مئوية
            r#"\d+\s*مرة"#, // أرقام دقيقة
        ];
        
        let mut detail_count = 0;
        for pattern in &detail_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                detail_count += regex.find_iter(response_text).count();
            }
        }
        
        if detail_count > 3 {
            risk_score += 0.3; // كثرة التفاصيل الدقيقة مشبوهة
        }
        
        // تطبيع النتيجة مع وزن للسياق
        let context_weight = match query.question_type {
            QuestionType::Aqeedah | QuestionType::Fiqh => 1.2, // أكثر حساسية
            QuestionType::Hadith => 1.1,
            _ => 1.0,
        };
        
        (risk_score * context_weight).min(1.0)
    }
    
    fn determine_claim_severity(&self, claim: &str, fact_type: &FactType) -> ClaimSeverity {
        let claim_lower = claim.to_lowercase();
        
        // ادعاءات خطيرة بناءً على النوع
        match fact_type {
            FactType::QuranReference => ClaimSeverity::Critical,
            FactType::HadithReference => ClaimSeverity::Critical,
            FactType::ReligiousRuling => ClaimSeverity::Major,
            FactType::ScholarQuote => ClaimSeverity::Major,
            FactType::HistoricalFact => ClaimSeverity::Minor,
            FactType::GeneralClaim => {
                // تحليل إضافي للادعاءات العامة
                if claim_lower.contains("حرام") || claim_lower.contains("واجب") || claim_lower.contains("فرض") {
                    ClaimSeverity::Major
                } else if claim_lower.contains("مستحب") || claim_lower.contains("مكروه") {
                    ClaimSeverity::Minor
                } else {
                    ClaimSeverity::Stylistic
                }
            }
        }
    }
    
    fn determine_enhanced_recommendation(
        &self,
        hallucination_risk: f32,
        confidence: f32,
        unsupported_claims: &[UnsupportedClaim],
        fabricated_content: &[FabricatedContent],
        query: &ProcessedQuestion,
    ) -> ResponseRecommendation {
        // رفض فوري للمحتوى المختلق الخطير
        if fabricated_content.iter().any(|f| {
            matches!(f.content_type, FabricationType::FakeAyah | FabricationType::FakeHadith) 
            && f.confidence > 0.7
        }) {
            return ResponseRecommendation::Reject;
        }
        
        // رفض للمخاطر العالية جداً
        if hallucination_risk > 0.8 {
            return ResponseRecommendation::Reject;
        }
        
        // مراجعة بشرية للمخاطر العالية أو الأسئلة الحساسة
        if hallucination_risk > 0.6 || confidence < 0.3 {
            return ResponseRecommendation::RequestHumanReview;
        }
        
        // اعتبارات خاصة للأسئلة الحساسة
        match query.question_type {
            QuestionType::Aqeedah => {
                if hallucination_risk > 0.4 || confidence < 0.6 {
                    return ResponseRecommendation::RequestHumanReview;
                }
            },
            QuestionType::Fiqh => {
                if hallucination_risk > 0.5 || confidence < 0.5 {
                    return ResponseRecommendation::RequireSourceCheck;
                }
            },
            _ => {}
        }
        
        // مراجعة مطلوبة للادعاءات الخطيرة غير المدعومة
        if unsupported_claims.iter().any(|c| matches!(c.severity, ClaimSeverity::Critical)) {
            return ResponseRecommendation::RequireSourceCheck;
        }
        
        // مراجعة للمخاطر المتوسطة
        if hallucination_risk > 0.4 {
            return ResponseRecommendation::RequireRevision;
        }
        
        // موافقة مع تحذير للمخاطر المنخفضة
        if hallucination_risk > 0.2 || !unsupported_claims.is_empty() {
            return ResponseRecommendation::ApproveWithWarning;
        }
        
        ResponseRecommendation::Approve
    }
    
    fn determine_enhanced_required_actions(
        &self,
        recommendation: &ResponseRecommendation,
        unsupported_claims: &[UnsupportedClaim],
        contradictions: &[Contradiction],
        fabricated_content: &[FabricatedContent],
        query: &ProcessedQuestion,
    ) -> Vec<String> {
        let mut actions = Vec::new();
        
        match recommendation {
            ResponseRecommendation::Reject => {
                actions.push("رفض الإجابة وإعادة التوليد مع مصادر أكثر دقة".to_string());
                if !fabricated_content.is_empty() {
                    actions.push("تنبيه: تم اكتشاف محتوى مختلق - مراجعة النموذج مطلوبة".to_string());
                }
            },
            ResponseRecommendation::RequestHumanReview => {
                actions.push("إرسال للمراجعة البشرية من قبل عالم مختص".to_string());
                if matches!(query.question_type, QuestionType::Aqeedah | QuestionType::Fiqh) {
                    actions.push("أولوية عالية: موضوع حساس يتطلب مراجعة عاجلة".to_string());
                }
            },
            ResponseRecommendation::RequireSourceCheck => {
                actions.push("التحقق من جميع المصادر المذكورة وإضافة مراجع إضافية".to_string());
                actions.push("التأكد من صحة الأحاديث المذكورة".to_string());
            },
            ResponseRecommendation::RequireRevision => {
                actions.push("مراجعة الإجابة وتصحيح النقاط المشكوك فيها".to_string());
                actions.push("إضافة تحذيرات مناسبة للمحتوى غير المؤكد".to_string());
            },
            ResponseRecommendation::ApproveWithWarning => {
                actions.push("إضافة تحذير للمستخدم حول ضرورة التحقق من المصادر".to_string());
                actions.push("إضافة عبارة 'والله أعلم' في نهاية الإجابة".to_string());
            },
            ResponseRecommendation::Approve => {
                actions.push("الموافقة على الإجابة مع إضافة المراجع المناسبة".to_string());
            },
        }
        
        // إجراءات محددة للمشاكل المكتشفة
        if !fabricated_content.is_empty() {
            actions.push("إزالة أو تصحيح المحتوى المختلق المكتشف".to_string());
            for fabricated in fabricated_content {
                actions.push(format!("تحذير: {} - {}", 
                    match fabricated.content_type {
                        FabricationType::FakeAyah => "آية مشكوك فيها",
                        FabricationType::FakeHadith => "حديث مشكوك فيه",
                        FabricationType::FakeQuote => "قول مشكوك فيه",
                        FabricationType::FakeRuling => "حكم مشكوك فيه",
                        FabricationType::FakeStory => "قصة مشكوك فيها",
                    },
                    fabricated.content.chars().take(50).collect::<String>()
                ));
            }
        }
        
        if !unsupported_claims.is_empty() {
            actions.push("إضافة مصادر موثوقة للادعاءات غير المدعومة".to_string());
            let critical_claims = unsupported_claims.iter()
                .filter(|c| matches!(c.severity, ClaimSeverity::Critical))
                .count();
            if critical_claims > 0 {
                actions.push(format!("تحذير: {} ادعاءات خطيرة تحتاج مصادر فورية", critical_claims));
            }
        }
        
        if !contradictions.is_empty() {
            actions.push("حل التناقضات مع المصادر الموثوقة".to_string());
            let critical_contradictions = contradictions.iter()
                .filter(|c| matches!(c.severity, ContradictionSeverity::Critical))
                .count();
            if critical_contradictions > 0 {
                actions.push(format!("تحذير: {} تناقضات خطيرة تحتاج حل فوري", critical_contradictions));
            }
        }
        
        actions
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
    
    async fn suggest_sources_for_claim(&self, claim: &str) -> Result<Vec<String>> {
        let mut suggestions = Vec::new();
        
        let claim_lower = claim.to_lowercase();
        
        if claim_lower.contains("قرآن") || claim_lower.contains("آية") {
            suggestions.push("المصحف الشريف ".to_string());
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
            Regex::new(r#"قال الله تعالى:?\s*[""]([^""]+)[""]"#).unwrap(),
            Regex::new(r#"في القرآن:?\s*[""]([^""]+)[""]"#).unwrap(),
            Regex::new(r#"قال الرسول:?\s*[""]([^""]+)[""]"#).unwrap(),
            Regex::new(r#"في الحديث:?\s*[""]([^""]+)[""]"#).unwrap(),
            Regex::new(r#"قال\s+([^:]+):\s*[""]([^""]+)[""]"#).unwrap(),
            Regex::new(r#"الحكم\s+في\s+([^:]+):\s*([^.]+)"#).unwrap(),
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
pub struct SourceVerifier {
    semantic_similarity_threshold: f32,
    exact_match_boost: f32,
    partial_match_threshold: f32,
}

#[derive(Debug, Clone)]
pub struct SupportLevel {
    pub support_score: f32,
    pub support_type: SupportType,
    pub supporting_sources: Vec<String>,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub enum SupportType {
    ExactMatch,      // تطابق دقيق
    StrongSupport,   // دعم قوي
    WeakSupport,     // دعم ضعيف
    NoSupport,       // لا يوجد دعم
    Contradicted,    // متناقض مع المصادر
}

impl SourceVerifier {
    pub fn new() -> Self {
        Self {
            semantic_similarity_threshold: 0.6,
            exact_match_boost: 1.5,
            partial_match_threshold: 0.4,
        }
    }
    
    pub async fn verify_fact_support(&self, fact: &ExtractedFact, sources: &[IslamicSource]) -> Result<bool> {
        let support_level = self.verify_fact_support_detailed(fact, sources).await?;
        Ok(support_level.support_score >= 0.3)
    }
    
    pub async fn verify_fact_support_detailed(&self, fact: &ExtractedFact, sources: &[IslamicSource]) -> Result<SupportLevel> {
        let claim_lower = fact.claim.to_lowercase();
        let mut best_support_score = 0.0;
        let mut support_type = SupportType::NoSupport;
        let mut supporting_sources = Vec::new();
        let mut confidence_scores = Vec::new();
        
        for source in sources {
            let source_text_lower = source.text.to_lowercase();
            
            // فحص التطابق الدقيق
            if source_text_lower.contains(&claim_lower) {
                let exact_score = 1.0 * self.exact_match_boost;
                if exact_score > best_support_score {
                    best_support_score = exact_score;
                    support_type = SupportType::ExactMatch;
                }
                supporting_sources.push(source.reference.clone());
                confidence_scores.push(0.95);
                continue;
            }
            
            // فحص التطابق الدلالي المتقدم
            let semantic_score = self.calculate_semantic_support(&claim_lower, &source_text_lower, &fact.fact_type);
            if semantic_score > best_support_score {
                best_support_score = semantic_score;
                support_type = if semantic_score > 0.8 {
                    SupportType::StrongSupport
                } else if semantic_score > 0.4 {
                    SupportType::WeakSupport
                } else {
                    SupportType::NoSupport
                };
            }
            
            if semantic_score > self.partial_match_threshold {
                supporting_sources.push(source.reference.clone());
                confidence_scores.push(semantic_score);
            }
            
            // فحص التناقض
            if self.check_contradiction(&claim_lower, &source_text_lower, &fact.fact_type) {
                return Ok(SupportLevel {
                    support_score: 0.0,
                    support_type: SupportType::Contradicted,
                    supporting_sources: vec![source.reference.clone()],
                    confidence: 0.9,
                });
            }
        }
        
        // حساب الثقة الإجمالية
        let overall_confidence = if confidence_scores.is_empty() {
            0.0
        } else {
            confidence_scores.iter().sum::<f32>() / confidence_scores.len() as f32
        };
        
        Ok(SupportLevel {
            support_score: best_support_score.min(1.0),
            support_type,
            supporting_sources,
            confidence: overall_confidence,
        })
    }
    
    fn calculate_semantic_support(&self, claim: &str, source_text: &str, fact_type: &FactType) -> f32 {
        // تحليل دلالي متقدم بناءً على نوع الحقيقة
        match fact_type {
            FactType::QuranReference => self.calculate_quran_support(claim, source_text),
            FactType::HadithReference => self.calculate_hadith_support(claim, source_text),
            FactType::ReligiousRuling => self.calculate_ruling_support(claim, source_text),
            FactType::ScholarQuote => self.calculate_quote_support(claim, source_text),
            FactType::HistoricalFact => self.calculate_historical_support(claim, source_text),
            FactType::GeneralClaim => self.calculate_general_support(claim, source_text),
        }
    }
    
    fn calculate_quran_support(&self, claim: &str, source_text: &str) -> f32 {
        // استخراج النص القرآني من الادعاء
        let quran_patterns = [
            r#"قال الله تعالى:?\s*[""]([^""]+)[""]"#,
            r#"في القرآن:?\s*[""]([^""]+)[""]"#,
        ];
        
        for pattern in &quran_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(capture) = regex.captures(claim) {
                    if let Some(ayah_text) = capture.get(1) {
                        let ayah = ayah_text.as_str();
                        // فحص وجود الآية في المصدر
                        if source_text.contains(ayah) {
                            return 1.0;
                        }
                        // فحص التشابه الجزئي للآيات
                        return self.calculate_ayah_similarity(ayah, source_text);
                    }
                }
            }
        }
        
        self.calculate_word_similarity(claim, source_text)
    }
    
    fn calculate_hadith_support(&self, claim: &str, source_text: &str) -> f32 {
        // استخراج نص الحديث من الادعاء
        let hadith_patterns = [
            r#"قال الرسول:?\s*[""]([^""]+)[""]"#,
            r#"في الحديث:?\s*[""]([^""]+)[""]"#,
        ];
        
        for pattern in &hadith_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(capture) = regex.captures(claim) {
                    if let Some(hadith_text) = capture.get(1) {
                        let hadith = hadith_text.as_str();
                        // فحص وجود الحديث في المصدر
                        if source_text.contains(hadith) {
                            return 1.0;
                        }
                        // فحص التشابه الجزئي للأحاديث
                        return self.calculate_hadith_similarity(hadith, source_text);
                    }
                }
            }
        }
        
        self.calculate_word_similarity(claim, source_text)
    }
    
    fn calculate_ruling_support(&self, claim: &str, source_text: &str) -> f32 {
        // كلمات مفتاحية للأحكام الفقهية
        let ruling_keywords = [
            "واجب", "فرض", "مستحب", "سنة", "مكروه", "حرام", "مباح", "جائز"
        ];
        
        let mut keyword_matches = 0;
        let mut total_keywords = 0;
        
        for keyword in &ruling_keywords {
            if claim.contains(keyword) {
                total_keywords += 1;
                if source_text.contains(keyword) {
                    keyword_matches += 1;
                }
            }
        }
        
        let keyword_score = if total_keywords > 0 {
            keyword_matches as f32 / total_keywords as f32
        } else {
            0.0
        };
        
        // دمج مع التشابه العام
        let word_similarity = self.calculate_word_similarity(claim, source_text);
        (keyword_score * 0.7 + word_similarity * 0.3).min(1.0)
    }
    
    fn calculate_quote_support(&self, claim: &str, source_text: &str) -> f32 {
        // استخراج اسم العالم والقول
        if let Ok(regex) = Regex::new(r#"قال\s+([^:]+):\s*(.+)"#) {
            if let Some(capture) = regex.captures(claim) {
                if let (Some(scholar), Some(quote)) = (capture.get(1), capture.get(2)) {
                    let scholar_name = scholar.as_str();
                    let quote_text = quote.as_str();
                    
                    // فحص وجود العالم في المصدر
                    let scholar_match = source_text.contains(scholar_name);
                    // فحص وجود القول في المصدر
                    let quote_match = self.calculate_word_similarity(quote_text, source_text);
                    
                    if scholar_match && quote_match > 0.6 {
                        return 0.9;
                    } else if scholar_match {
                        return quote_match * 0.7;
                    } else {
                        return quote_match * 0.3;
                    }
                }
            }
        }
        
        self.calculate_word_similarity(claim, source_text)
    }
    
    fn calculate_historical_support(&self, claim: &str, source_text: &str) -> f32 {
        // فحص التواريخ والأحداث التاريخية
        let date_pattern = Regex::new(r#"\d+"#).unwrap();
        let claim_dates: Vec<&str> = date_pattern.find_iter(claim).map(|m| m.as_str()).collect();
        let source_dates: Vec<&str> = date_pattern.find_iter(source_text).map(|m| m.as_str()).collect();
        
        let date_matches = claim_dates.iter()
            .filter(|date| source_dates.contains(date))
            .count();
        
        let date_score = if !claim_dates.is_empty() {
            date_matches as f32 / claim_dates.len() as f32
        } else {
            0.0
        };
        
        let word_similarity = self.calculate_word_similarity(claim, source_text);
        (date_score * 0.4 + word_similarity * 0.6).min(1.0)
    }
    
    fn calculate_general_support(&self, claim: &str, source_text: &str) -> f32 {
        self.calculate_word_similarity(claim, source_text)
    }
    
    fn calculate_ayah_similarity(&self, ayah: &str, source_text: &str) -> f32 {
        // تقسيم الآية إلى كلمات وفحص التطابق
        let ayah_words: HashSet<&str> = ayah.split_whitespace()
            .filter(|word| word.len() > 2)
            .collect();
        
        let source_words: HashSet<&str> = source_text.split_whitespace()
            .filter(|word| word.len() > 2)
            .collect();
        
        if ayah_words.is_empty() {
            return 0.0;
        }
        
        let intersection: HashSet<_> = ayah_words.intersection(&source_words).collect();
        let similarity = intersection.len() as f32 / ayah_words.len() as f32;
        
        // الآيات تحتاج تطابق عالي
        if similarity > 0.8 {
            similarity
        } else if similarity > 0.6 {
            similarity * 0.7
        } else {
            similarity * 0.3
        }
    }
    
    fn calculate_hadith_similarity(&self, hadith: &str, source_text: &str) -> f32 {
        // مشابه لحساب تشابه الآيات لكن مع عتبة أقل
        let hadith_words: HashSet<&str> = hadith.split_whitespace()
            .filter(|word| word.len() > 2)
            .collect();
        
        let source_words: HashSet<&str> = source_text.split_whitespace()
            .filter(|word| word.len() > 2)
            .collect();
        
        if hadith_words.is_empty() {
            return 0.0;
        }
        
        let intersection: HashSet<_> = hadith_words.intersection(&source_words).collect();
        let similarity = intersection.len() as f32 / hadith_words.len() as f32;
        
        // الأحاديث تحتاج تطابق عالي لكن أقل من الآيات
        if similarity > 0.7 {
            similarity
        } else if similarity > 0.5 {
            similarity * 0.8
        } else {
            similarity * 0.4
        }
    }
    
    fn calculate_word_similarity(&self, text1: &str, text2: &str) -> f32 {
        let words1: HashSet<&str> = text1.split_whitespace()
            .filter(|word| word.len() > 2)
            .collect();
        let words2: HashSet<&str> = text2.split_whitespace()
            .filter(|word| word.len() > 2)
            .collect();
        
        if words1.is_empty() && words2.is_empty() {
            return 1.0;
        }
        
        if words1.is_empty() || words2.is_empty() {
            return 0.0;
        }
        
        let intersection: HashSet<_> = words1.intersection(&words2).collect();
        let union: HashSet<_> = words1.union(&words2).collect();
        
        intersection.len() as f32 / union.len() as f32
    }
    
    fn check_contradiction(&self, claim: &str, source_text: &str, fact_type: &FactType) -> bool {
        // فحص التناقضات المباشرة
        let contradiction_pairs = [
            ("حلال", "حرام"),
            ("واجب", "مكروه"),
            ("مستحب", "حرام"),
            ("صحيح", "ضعيف"),
            ("ثابت", "موضوع"),
        ];
        
        for (positive, negative) in &contradiction_pairs {
            if (claim.contains(positive) && source_text.contains(negative)) ||
               (claim.contains(negative) && source_text.contains(positive)) {
                return true;
            }
        }
        
        // فحص تناقضات خاصة بنوع الحقيقة
        match fact_type {
            FactType::ReligiousRuling => {
                self.check_ruling_contradiction(claim, source_text)
            },
            FactType::HadithReference => {
                self.check_hadith_contradiction(claim, source_text)
            },
            _ => false,
        }
    }
    
    fn check_ruling_contradiction(&self, claim: &str, source_text: &str) -> bool {
        // فحص تناقضات الأحكام الفقهية
        let positive_rulings = ["واجب", "فرض", "مستحب", "سنة", "مباح", "جائز"];
        let negative_rulings = ["حرام", "مكروه", "ممنوع"];
        
        let claim_has_positive = positive_rulings.iter().any(|r| claim.contains(r));
        let claim_has_negative = negative_rulings.iter().any(|r| claim.contains(r));
        
        let source_has_positive = positive_rulings.iter().any(|r| source_text.contains(r));
        let source_has_negative = negative_rulings.iter().any(|r| source_text.contains(r));
        
        (claim_has_positive && source_has_negative) || (claim_has_negative && source_has_positive)
    }
    
    fn check_hadith_contradiction(&self, claim: &str, source_text: &str) -> bool {
        // فحص تناقضات درجات الأحاديث
        let strong_grades = ["صحيح", "حسن"];
        let weak_grades = ["ضعيف", "موضوع"];
        
        let claim_strong = strong_grades.iter().any(|g| claim.contains(g));
        let claim_weak = weak_grades.iter().any(|g| claim.contains(g));
        
        let source_strong = strong_grades.iter().any(|g| source_text.contains(g));
        let source_weak = weak_grades.iter().any(|g| source_text.contains(g));
        
        (claim_strong && source_weak) || (claim_weak && source_strong)
    }
}

/// Enhanced consistency checker for detecting contradictions
pub struct ConsistencyChecker {
    contradiction_patterns: Vec<ContradictionPattern>,
    theological_rules: Vec<TheologicalRule>,
}

#[derive(Debug, Clone)]
pub struct ContradictionPattern {
    pub pattern: Regex,
    pub contradiction_type: ContradictionType,
    pub severity: ContradictionSeverity,
}

#[derive(Debug, Clone)]
pub enum ContradictionType {
    DirectOpposition,    // تناقض مباشر
    LogicalInconsistency, // عدم اتساق منطقي
    SourceConflict,      // تضارب في المصادر
    ContextualError,     // خطأ في السياق
}

#[derive(Debug, Clone)]
pub struct TheologicalRule {
    pub rule_name: String,
    pub condition: String,
    pub expected_outcome: String,
    pub violation_severity: ContradictionSeverity,
}

impl ConsistencyChecker {
    pub fn new() -> Self {
        let contradiction_patterns = vec![
            ContradictionPattern {
                pattern: Regex::new(r#"حلال.*حرام|حرام.*حلال"#).unwrap(),
                contradiction_type: ContradictionType::DirectOpposition,
                severity: ContradictionSeverity::Critical,
            },
            ContradictionPattern {
                pattern: Regex::new(r#"واجب.*مكروه|مكروه.*واجب"#).unwrap(),
                contradiction_type: ContradictionType::DirectOpposition,
                severity: ContradictionSeverity::Major,
            },
            ContradictionPattern {
                pattern: Regex::new(r#"صحيح.*ضعيف|ضعيف.*صحيح"#).unwrap(),
                contradiction_type: ContradictionType::SourceConflict,
                severity: ContradictionSeverity::Major,
            },
        ];

        let theological_rules = vec![
            TheologicalRule {
                rule_name: "توحيد الألوهية ".to_string(),
                condition: "عبادة غير الله ".to_string(),
                expected_outcome: "شرك".to_string(),
                violation_severity: ContradictionSeverity::Critical,
            },
            TheologicalRule {
                rule_name: "عصمة القرآن ".to_string(),
                condition: "خطأ في القرآن ".to_string(),
                expected_outcome: "رفض الادعاء ".to_string(),
                violation_severity: ContradictionSeverity::Critical,
            },
        ];

        Self {
            contradiction_patterns,
            theological_rules,
        }
    }
    
    pub async fn check_consistency(&self, response_text: &str, sources: &[IslamicSource]) -> Result<Vec<Contradiction>> {
        let mut contradictions = Vec::new();
        
        // فحص التناقضات الأساسية
        for source in sources {
            if let Some(contradiction) = self.find_contradiction(response_text, source).await? {
                contradictions.push(contradiction);
            }
        }
        
        Ok(contradictions)
    }
    
    pub async fn check_consistency_advanced(
        &self,
        response_text: &str,
        sources: &[IslamicSource],
        query: &ProcessedQuestion,
    ) -> Result<Vec<Contradiction>> {
        let mut contradictions = Vec::new();
        
        // 1. فحص التناقضات المباشرة في النص
        contradictions.extend(self.check_internal_contradictions(response_text).await?);
        
        // 2. فحص التناقضات مع المصادر
        for source in sources {
            if let Some(contradiction) = self.find_advanced_contradiction(response_text, source, query).await? {
                contradictions.push(contradiction);
            }
        }
        
        // 3. فحص انتهاك القواعد اللاهوتية
        contradictions.extend(self.check_theological_violations(response_text).await?);
        
        // 4. فحص التناقضات السياقية
        contradictions.extend(self.check_contextual_contradictions(response_text, query).await?);
        
        // 5. فحص تناقضات المصادر مع بعضها البعض
        contradictions.extend(self.check_source_contradictions(sources).await?);
        
        Ok(contradictions)
    }
    
    async fn check_internal_contradictions(&self, response_text: &str) -> Result<Vec<Contradiction>> {
        let mut contradictions = Vec::new();
        
        // فحص التناقضات باستخدام الأنماط المحددة مسبقاً
        for pattern in &self.contradiction_patterns {
            if pattern.pattern.is_match(response_text) {
                contradictions.push(Contradiction {
                    claim: "تناقض داخلي في النص ".to_string(),
                    contradicting_source: IslamicSource {
                        id: "internal_contradiction".to_string(),
                        content_type: SourceType::ScholarOpinion,
                        text: response_text.to_string(),
                        reference: "النص نفسه ".to_string(),
                        author: None,
                        authenticity: AuthenticityLevel::Questionable,
                        language: Language::Arabic,
                        metadata: HashMap::new(),
                        created_at: chrono::Utc::now(),
                    },
                    severity: pattern.severity.clone(),
                    explanation: format!("تم اكتشاف تناقض من نوع: {:?}", pattern.contradiction_type),
                });
            }
        }
        
        // فحص تناقضات الأحكام الفقهية
        contradictions.extend(self.check_fiqh_contradictions(response_text).await?);
        
        // فحص تناقضات درجات الأحاديث
        contradictions.extend(self.check_hadith_grade_contradictions(response_text).await?);
        
        Ok(contradictions)
    }
    
    async fn check_fiqh_contradictions(&self, response_text: &str) -> Result<Vec<Contradiction>> {
        let mut contradictions = Vec::new();
        
        // استخراج الأحكام الفقهية من النص
        let rulings = self.extract_fiqh_rulings(response_text);
        
        // فحص التناقضات بين الأحكام
        for i in 0..rulings.len() {
            for j in i+1..rulings.len() {
                if self.are_rulings_contradictory(&rulings[i], &rulings[j]) {
                    contradictions.push(Contradiction {
                        claim: format!("تناقض في الأحكام: {} مقابل {}", rulings[i].ruling, rulings[j].ruling),
                        contradicting_source: IslamicSource {
                            id: "fiqh_contradiction".to_string(),
                            content_type: SourceType::FiqhRuling,
                            text: response_text.to_string(),
                            reference: "تناقض داخلي ".to_string(),
                            author: None,
                            authenticity: AuthenticityLevel::Questionable,
                            language: Language::Arabic,
                            metadata: HashMap::new(),
                            created_at: chrono::Utc::now(),
                        },
                        severity: ContradictionSeverity::Major,
                        explanation: "تناقض في الأحكام الفقهية داخل النص ".to_string(),
                    });
                }
            }
        }
        
        Ok(contradictions)
    }
    
    async fn check_hadith_grade_contradictions(&self, response_text: &str) -> Result<Vec<Contradiction>> {
        let mut contradictions = Vec::new();
        
        // استخراج درجات الأحاديث
        let hadith_grades = self.extract_hadith_grades(response_text);
        
        // فحص التناقضات في درجات نفس الحديث
        let mut hadith_map: HashMap<String, Vec<String>> = HashMap::new();
        
        for grade in hadith_grades {
            let hadith_key = self.normalize_hadith_text(&grade.hadith_text);
            hadith_map.entry(hadith_key).or_insert_with(Vec::new).push(grade.grade);
        }
        
        for (hadith_text, grades) in hadith_map {
            if grades.len() > 1 && self.are_grades_contradictory(&grades) {
                contradictions.push(Contradiction {
                    claim: format!("تناقض في درجة الحديث: {:?}", grades),
                    contradicting_source: IslamicSource {
                        id: "hadith_grade_contradiction".to_string(),
                        content_type: SourceType::SahihHadith,
                        text: hadith_text,
                        reference: "تناقض في الدرجة ".to_string(),
                        author: None,
                        authenticity: AuthenticityLevel::Questionable,
                        language: Language::Arabic,
                        metadata: HashMap::new(),
                        created_at: chrono::Utc::now(),
                    },
                    severity: ContradictionSeverity::Major,
                    explanation: "تناقض في درجة صحة نفس الحديث ".to_string(),
                });
            }
        }
        
        Ok(contradictions)
    }
    
    async fn check_theological_violations(&self, response_text: &str) -> Result<Vec<Contradiction>> {
        let mut contradictions = Vec::new();
        
        for rule in &self.theological_rules {
            if response_text.to_lowercase().contains(&rule.condition.to_lowercase()) {
                // فحص إذا كان النص يحتوي على النتيجة المتوقعة
                if !response_text.to_lowercase().contains(&rule.expected_outcome.to_lowercase()) {
                    contradictions.push(Contradiction {
                        claim: format!("انتهاك القاعدة اللاهوتية: {}", rule.rule_name),
                        contradicting_source: IslamicSource {
                            id: "theological_violation".to_string(),
                            content_type: SourceType::ScholarOpinion,
                            text: rule.condition.clone(),
                            reference: format!("القاعدة: {}", rule.rule_name),
                            author: None,
                            authenticity: AuthenticityLevel::Verified,
                            language: Language::Arabic,
                            metadata: HashMap::new(),
                            created_at: chrono::Utc::now(),
                        },
                        severity: rule.violation_severity.clone(),
                        explanation: format!("انتهاك للقاعدة اللاهوتية: {}", rule.rule_name),
                    });
                }
            }
        }
        
        Ok(contradictions)
    }
    
    async fn check_contextual_contradictions(&self, response_text: &str, query: &ProcessedQuestion) -> Result<Vec<Contradiction>> {
        let mut contradictions = Vec::new();
        
        // فحص التناقضات السياقية بناءً على نوع السؤال
        match query.question_type {
            QuestionType::Aqeedah => {
                contradictions.extend(self.check_aqeedah_context(response_text).await?);
            },
            QuestionType::Fiqh => {
                contradictions.extend(self.check_fiqh_context(response_text).await?);
            },
            QuestionType::Hadith => {
                contradictions.extend(self.check_hadith_context(response_text).await?);
            },
            _ => {}
        }
        
        Ok(contradictions)
    }
    
    async fn check_source_contradictions(&self, sources: &[IslamicSource]) -> Result<Vec<Contradiction>> {
        let mut contradictions = Vec::new();
        
        // فحص التناقضات بين المصادر المختلفة
        for i in 0..sources.len() {
            for j in i+1..sources.len() {
                if let Some(contradiction) = self.find_source_to_source_contradiction(&sources[i], &sources[j]).await? {
                    contradictions.push(contradiction);
                }
            }
        }
        
        Ok(contradictions)
    }
    
    async fn find_advanced_contradiction(&self, response_text: &str, source: &IslamicSource, query: &ProcessedQuestion) -> Result<Option<Contradiction>> {
        // فحص متقدم للتناقضات مع المصادر
        let response_lower = response_text.to_lowercase();
        let source_lower = source.text.to_lowercase();
        
        // فحص التناقضات المباشرة
        if let Some(direct_contradiction) = self.find_direct_contradiction(&response_lower, &source_lower) {
            return Ok(Some(Contradiction {
                claim: direct_contradiction,
                contradicting_source: source.clone(),
                severity: ContradictionSeverity::Critical,
                explanation: "تناقض مباشر مع المصدر ".to_string(),
            }));
        }
        
        // فحص التناقضات السياقية
        if let Some(contextual_contradiction) = self.find_contextual_contradiction(&response_lower, &source_lower, query) {
            return Ok(Some(Contradiction {
                claim: contextual_contradiction,
                contradicting_source: source.clone(),
                severity: ContradictionSeverity::Major,
                explanation: "تناقض سياقي مع المصدر ".to_string(),
            }));
        }
        
        Ok(None)
    }
    
    // Helper methods for extracting rulings and grades
    
    fn extract_fiqh_rulings(&self, text: &str) -> Vec<FiqhRuling> {
        let mut rulings = Vec::new();
        
        let ruling_patterns = [
            r#"(\w+)\s+(واجب|فرض|مستحب|سنة|مكروه|حرام|مباح|جائز)"#,
            r#"(واجب|فرض|مستحب|سنة|مكروه|حرام|مباح|جائز)\s+(\w+)"#,
        ];
        
        for pattern in &ruling_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for capture in regex.captures_iter(text) {
                    if let (Some(subject), Some(ruling)) = (capture.get(1), capture.get(2)) {
                        rulings.push(FiqhRuling {
                            ruling: ruling.as_str().to_string(),
                            subject: subject.as_str().to_string(),
                            position: capture.get(0).unwrap().start(),
                        });
                    }
                }
            }
        }
        
        rulings
    }
    
    fn extract_hadith_grades(&self, text: &str) -> Vec<HadithGrade> {
        let mut grades = Vec::new();
        
        let grade_pattern = r#"حديث\s+([^.]+)\s+(صحيح|حسن|ضعيف|موضوع)"#;
        if let Ok(regex) = Regex::new(grade_pattern) {
            for capture in regex.captures_iter(text) {
                if let (Some(hadith_text), Some(grade)) = (capture.get(1), capture.get(2)) {
                    grades.push(HadithGrade {
                        hadith_text: hadith_text.as_str().to_string(),
                        grade: grade.as_str().to_string(),
                        position: capture.get(0).unwrap().start(),
                    });
                }
            }
        }
        
        grades
    }
    
    fn are_rulings_contradictory(&self, ruling1: &FiqhRuling, ruling2: &FiqhRuling) -> bool {
        if ruling1.subject != ruling2.subject {
            return false;
        }
        
        let contradictory_pairs = [
            ("واجب", "حرام"),
            ("فرض", "مكروه"),
            ("مستحب", "حرام"),
            ("حلال", "حرام"),
            ("مباح", "حرام"),
        ];
        
        for (positive, negative) in &contradictory_pairs {
            if (ruling1.ruling == *positive && ruling2.ruling == *negative) ||
               (ruling1.ruling == *negative && ruling2.ruling == *positive) {
                return true;
            }
        }
        
        false
    }
    
    fn are_grades_contradictory(&self, grades: &[String]) -> bool {
        let strong_grades = ["صحيح", "حسن"];
        let weak_grades = ["ضعيف", "موضوع"];
        
        let has_strong = grades.iter().any(|g| strong_grades.contains(&g.as_str()));
        let has_weak = grades.iter().any(|g| weak_grades.contains(&g.as_str()));
        
        has_strong && has_weak
    }
    
    fn normalize_hadith_text(&self, text: &str) -> String {
        // تطبيع نص الحديث للمقارنة
        text.to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    async fn check_aqeedah_context(&self, response_text: &str) -> Result<Vec<Contradiction>> {
        let mut contradictions = Vec::new();
        
        // فحص مخالفات العقيدة
        let aqeedah_violations = [
            ("شرك", "توحيد"),
            ("كفر", "إيمان"),
            ("بدعة", "سنة"),
        ];
        
        for (violation, correct) in &aqeedah_violations {
            if response_text.contains(violation) && !response_text.contains("لا") && !response_text.contains("ليس") {
                // فحص إذا كان السياق يدين الانتهاك
                if !response_text.contains(&format!("لا {}", violation)) && 
                   !response_text.contains(&format!("{} محرم ", violation)) {
                    contradictions.push(Contradiction {
                        claim: format!("ذكر {} بدون إدانة واضحة ", violation),
                        contradicting_source: IslamicSource {
                            id: "aqeedah_context".to_string(),
                            content_type: SourceType::ScholarOpinion,
                            text: format!("العقيدة الصحيحة تتطلب رفض {}", violation),
                            reference: "أصول العقيدة ".to_string(),
                            author: None,
                            authenticity: AuthenticityLevel::Verified,
                            language: Language::Arabic,
                            metadata: HashMap::new(),
                            created_at: chrono::Utc::now(),
                        },
                        severity: ContradictionSeverity::Major,
                        explanation: format!("عدم وضوح الموقف من {}", violation),
                    });
                }
            }
        }
        
        Ok(contradictions)
    }
    
    async fn check_fiqh_context(&self, response_text: &str) -> Result<Vec<Contradiction>> {
        // فحص السياق الفقهي
        Ok(Vec::new()) // مبسط للمثال
    }
    
    async fn check_hadith_context(&self, response_text: &str) -> Result<Vec<Contradiction>> {
        // فحص سياق الأحاديث
        Ok(Vec::new()) // مبسط للمثال
    }
    
    async fn find_source_to_source_contradiction(&self, source1: &IslamicSource, source2: &IslamicSource) -> Result<Option<Contradiction>> {
        // فحص التناقضات بين المصادر
        Ok(None) // مبسط للمثال
    }
    
    fn find_direct_contradiction(&self, response: &str, source: &str) -> Option<String> {
        // فحص التناقضات المباشرة
        None // مبسط للمثال
    }
    
    fn find_contextual_contradiction(&self, response: &str, source: &str, query: &ProcessedQuestion) -> Option<String> {
        // فحص التناقضات السياقية
        None // مبسط للمثال
    }
    
    async fn find_contradiction(&self, response_text: &str, source: &IslamicSource) -> Result<Option<Contradiction>> {
        // الطريقة الأساسية للتوافق مع الكود الموجود
        Ok(None)
    }
}

/// Enhanced confidence assessor for evaluating overall response confidence
pub struct ConfidenceAssessor {
    source_quality_weights: HashMap<SourceType, f32>,
    authenticity_weights: HashMap<AuthenticityLevel, f32>,
    uncertainty_patterns: Vec<Regex>,
    confidence_patterns: Vec<Regex>,
}

impl ConfidenceAssessor {
    pub fn new() -> Self {
        let mut source_weights = HashMap::new();
        source_weights.insert(SourceType::Quran, 1.0);
        source_weights.insert(SourceType::SahihHadith, 0.95);
        source_weights.insert(SourceType::HasanHadith, 0.85);
        source_weights.insert(SourceType::Tafsir, 0.8);
        source_weights.insert(SourceType::FiqhRuling, 0.75);
        source_weights.insert(SourceType::ScholarOpinion, 0.7);
        source_weights.insert(SourceType::DaifHadith, 0.4);
        source_weights.insert(SourceType::MawduHadith, 0.1);
        source_weights.insert(SourceType::IslamicStory, 0.6);

        let mut authenticity_weights = HashMap::new();
        authenticity_weights.insert(AuthenticityLevel::Verified, 1.0);
        authenticity_weights.insert(AuthenticityLevel::Reliable, 0.8);
        authenticity_weights.insert(AuthenticityLevel::Questionable, 0.5);
        authenticity_weights.insert(AuthenticityLevel::Unreliable, 0.2);
        authenticity_weights.insert(AuthenticityLevel::Unknown, 0.3);

        let uncertainty_patterns = vec![
            Regex::new(r#"لست متأكد "#).unwrap(),
            Regex::new(r#"قد يكون "#).unwrap(),
            Regex::new(r"ربما").unwrap(),
            Regex::new(r"يحتمل").unwrap(),
            Regex::new(r#"لا أعلم "#).unwrap(),
            Regex::new(r#"غير واضح "#).unwrap(),
            Regex::new(r#"يحتاج تأكيد "#).unwrap(),
            Regex::new(r#"والله أعلم "#).unwrap(), // هذا إيجابي في السياق الإسلامي
        ];

        let confidence_patterns = vec![
            Regex::new(r#"ثبت في "#).unwrap(),
            Regex::new(r#"صح عن "#).unwrap(),
            Regex::new(r#"أجمع العلماء "#).unwrap(),
            Regex::new(r#"نص صريح "#).unwrap(),
            Regex::new(r#"دليل قاطع "#).unwrap(),
            Regex::new(r#"متفق عليه "#).unwrap(),
        ];

        Self {
            source_quality_weights: source_weights,
            authenticity_weights: authenticity_weights,
            uncertainty_patterns,
            confidence_patterns,
        }
    }
    
    pub async fn assess_confidence(
        &self,
        response_text: &str,
        sources: &[IslamicSource],
        query: &ProcessedQuestion,
    ) -> Result<f32> {
        let mut confidence_score = 0.5; // نقطة البداية
        
        // 1. تقييم جودة المصادر (40% من النتيجة)
        let source_quality = self.calculate_source_quality_score(sources);
        confidence_score += source_quality * 0.4;
        
        // 2. تقييم تطابق المحتوى مع المصادر (25% من النتيجة)
        let content_alignment = self.calculate_content_alignment(response_text, sources).await?;
        confidence_score += content_alignment * 0.25;
        
        // 3. تحليل لغة الإجابة للثقة/عدم الثقة (15% من النتيجة)
        let language_confidence = self.analyze_response_language(response_text);
        confidence_score += language_confidence * 0.15;
        
        // 4. تقييم اكتمال الإجابة (10% من النتيجة)
        let completeness = self.assess_response_completeness(response_text, query);
        confidence_score += completeness * 0.1;
        
        // 5. تقييم التعقيد مقابل المصادر المتاحة (10% من النتيجة)
        let complexity_match = self.assess_complexity_source_match(query, sources);
        confidence_score += complexity_match * 0.1;
        
        // تطبيق عوامل التصحيح
        confidence_score = self.apply_correction_factors(confidence_score, response_text, sources, query);
        
        Ok(confidence_score.max(0.0).min(1.0))
    }
    
    fn calculate_source_quality_score(&self, sources: &[IslamicSource]) -> f32 {
        if sources.is_empty() {
            return 0.0;
        }
        
        let mut total_weight = 0.0;
        let mut weighted_quality = 0.0;
        
        for source in sources {
            let source_weight = self.source_quality_weights
                .get(&source.content_type)
                .unwrap_or(&0.5);
            
            let authenticity_weight = self.authenticity_weights
                .get(&source.authenticity)
                .unwrap_or(&0.3);
            
            let combined_quality = (source_weight + authenticity_weight) / 2.0;
            
            weighted_quality += combined_quality;
            total_weight += 1.0;
        }
        
        if total_weight > 0.0 {
            weighted_quality / total_weight
        } else {
            0.0
        }
    }
    
    async fn calculate_content_alignment(&self, response_text: &str, sources: &[IslamicSource]) -> Result<f32> {
        if sources.is_empty() {
            return Ok(0.2); // ثقة منخفضة بدون مصادر
        }
        
        let response_lower = response_text.to_lowercase();
        let mut alignment_scores = Vec::new();
        
        for source in sources {
            let source_lower = source.text.to_lowercase();
            let alignment = self.calculate_text_similarity(&response_lower, &source_lower);
            alignment_scores.push(alignment);
        }
        
        // أخذ متوسط أعلى 3 نتائج تشابه
        alignment_scores.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let top_scores: Vec<f32> = alignment_scores.into_iter().take(3).collect();
        
        if !top_scores.is_empty() {
            Ok(top_scores.iter().sum::<f32>() / top_scores.len() as f32)
        } else {
            Ok(0.2)
        }
    }
    
    fn calculate_text_similarity(&self, text1: &str, text2: &str) -> f32 {
        let words1: HashSet<&str> = text1.split_whitespace().collect();
        let words2: HashSet<&str> = text2.split_whitespace().collect();
        
        if words1.is_empty() && words2.is_empty() {
            return 1.0;
        }
        
        let intersection: HashSet<_> = words1.intersection(&words2).collect();
        let union: HashSet<_> = words1.union(&words2).collect();
        
        if union.is_empty() {
            0.0
        } else {
            intersection.len() as f32 / union.len() as f32
        }
    }
    
    fn analyze_response_language(&self, response_text: &str) -> f32 {
        let mut language_score = 0.5;
        
        // فحص عبارات عدم الثقة
        let mut uncertainty_count = 0;
        for pattern in &self.uncertainty_patterns {
            uncertainty_count += pattern.find_iter(response_text).count();
        }
        
        // فحص عبارات الثقة
        let mut confidence_count = 0;
        for pattern in &self.confidence_patterns {
            confidence_count += pattern.find_iter(response_text).count();
        }
        
        // تعديل النتيجة بناءً على العبارات
        if confidence_count > 0 {
            language_score += (confidence_count as f32 * 0.1).min(0.3);
        }
        
        if uncertainty_count > 0 {
            // "والله أعلم " إيجابي في السياق الإسلامي
            let positive_uncertainty = response_text.matches("والله أعلم ").count();
            let negative_uncertainty = uncertainty_count - positive_uncertainty;
            
            if positive_uncertainty > 0 {
                language_score += 0.1; // إضافة إيجابية للتواضع العلمي
            }
            
            if negative_uncertainty > 0 {
                language_score -= (negative_uncertainty as f32 * 0.15).min(0.4);
            }
        }
        
        language_score.max(0.0).min(1.0)
    }
    
    fn assess_response_completeness(&self, response_text: &str, query: &ProcessedQuestion) -> f32 {
        let mut completeness: f32 = 0.5;
        
        // تقييم الطول المناسب
        let text_length = response_text.len();
        match query.complexity_level {
            ComplexityLevel::Simple => {
                if text_length >= 100 && text_length <= 500 {
                    completeness += 0.2;
                } else if text_length < 50 {
                    completeness -= 0.3;
                }
            },
            ComplexityLevel::Intermediate => {
                if text_length >= 200 && text_length <= 800 {
                    completeness += 0.2;
                } else if text_length < 100 {
                    completeness -= 0.3;
                }
            },
            ComplexityLevel::Advanced | ComplexityLevel::Scholarly => {
                if text_length >= 300 && text_length <= 1500 {
                    completeness += 0.2;
                } else if text_length < 200 {
                    completeness -= 0.3;
                }
            },
        }
        
        // تقييم تغطية المفاهيم المطلوبة
        let response_lower = response_text.to_lowercase();
        let covered_concepts = query.concepts.iter()
            .filter(|concept| response_lower.contains(&concept.to_lowercase()))
            .count();
        
        if !query.concepts.is_empty() {
            let concept_coverage = covered_concepts as f32 / query.concepts.len() as f32;
            completeness += concept_coverage * 0.3;
        }
        
        completeness.max(0.0).min(1.0)
    }
    
    fn assess_complexity_source_match(&self, query: &ProcessedQuestion, sources: &[IslamicSource]) -> f32 {
        let required_sources = match query.complexity_level {
            ComplexityLevel::Simple => 1,
            ComplexityLevel::Intermediate => 2,
            ComplexityLevel::Advanced => 3,
            ComplexityLevel::Scholarly => 4,
        };
        
        let available_sources = sources.len();
        
        if available_sources >= required_sources {
            1.0
        } else if available_sources == 0 {
            0.0
        } else {
            available_sources as f32 / required_sources as f32
        }
    }
    
    fn apply_correction_factors(
        &self,
        base_confidence: f32,
        response_text: &str,
        sources: &[IslamicSource],
        query: &ProcessedQuestion,
    ) -> f32 {
        let mut adjusted_confidence = base_confidence;
        
        // تصحيح للأسئلة الخلافية
        if query.is_controversial {
            if sources.len() >= 2 && response_text.contains("اختلف") {
                adjusted_confidence += 0.1; // إيجابي لعرض الخلاف
            } else if !response_text.contains("خلاف") && !response_text.contains("اختلف") {
                adjusted_confidence -= 0.2; // سلبي لعدم ذكر الخلاف
            }
        }
        
        // تصحيح للأحاديث الضعيفة
        let weak_hadith_count = sources.iter()
            .filter(|s| matches!(s.content_type, SourceType::DaifHadith))
            .count();
        
        if weak_hadith_count > 0 {
            adjusted_confidence -= (weak_hadith_count as f32 * 0.1).min(0.3);
        }
        
        // تصحيح للمصادر المتضاربة
        let source_types: HashSet<_> = sources.iter().map(|s| &s.content_type).collect();
        if source_types.len() >= 3 {
            adjusted_confidence += 0.1; // تنوع المصادر إيجابي
        }
        
        adjusted_confidence.max(0.0).min(1.0)
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
        let quran_pattern = Regex::new(r#"قال الله تعالى:?\s*[""]([^""]+)[""]"#).unwrap();
        
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
                        evidence: vec!["لا توجد في المصحف الشريف ".to_string()],
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
        let hadith_pattern = Regex::new(r#"قال الرسول:?\s*[""]([^""]+)[""]"#).unwrap();
        
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
                        evidence: vec!["لا يوجد في كتب الحديث المعتمدة ".to_string()],
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    /// Test the complete anti-hallucination pipeline
    #[tokio::test]
    async fn test_anti_hallucination_pipeline() {
        let system = AntiHallucinationSystem::new();
        
        let response_text = "الصلاة هي الركن الثاني من أركان الإسلام ";
        let sources = vec![
            IslamicSource {
                id: "test_source".to_string(),
                content_type: SourceType::SahihHadith,
                text: "بني الإسلام على خمس: شهادة أن لا إله إلا الله وأن محمداً رسول الله وإقام الصلاة ".to_string(),
                reference: "صحيح البخاري ".to_string(),
                author: Some("البخاري ".to_string()),
                authenticity: AuthenticityLevel::Verified,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: chrono::Utc::now(),
            }
        ];
        
        let query = ProcessedQuestion {
            original_text: "ما هي أركان الإسلام ".to_string(),
            normalized_text: "ما هي أركان الإسلام ".to_string(),
            keywords: vec!["أركان".to_string(), "إسلام".to_string()],
            concepts: vec!["إسلام".to_string()],
            question_type: QuestionType::General,
            complexity_level: ComplexityLevel::Simple,
            language: Language::Arabic,
            is_controversial: false,
            requires_multiple_sources: false,
            embedding: None,
        };
        
        let result = system.check_response(response_text, &sources, &query).await;
        
        assert!(result.is_ok());
        let check_result = result.unwrap();
        
        // التحقق من النتائج الأساسية
        assert!(check_result.confidence_score > 0.0);
        assert!(check_result.hallucination_risk_score >= 0.0);
        assert!(check_result.hallucination_risk_score <= 1.0);
        
        println!("Anti-hallucination check completed:");
        println!("Confidence: {:.2}", check_result.confidence_score);
        println!("Hallucination risk: {:.2}", check_result.hallucination_risk_score);
        println!("Recommendation: {:?}", check_result.recommendation);
    }
    
    /// Test confidence assessment with different source qualities
    #[tokio::test]
    async fn test_confidence_with_source_quality() {
        let assessor = ConfidenceAssessor::new();
        
        let response_text = "الصلاة واجبة على كل مسلم ";
        
        // مصادر عالية الجودة
        let high_quality_sources = vec![
            IslamicSource {
                id: "quran_source".to_string(),
                content_type: SourceType::Quran,
                text: "وَأَقِيمُوا الصَّلَاةَ ".to_string(),
                reference: "البقرة: 43".to_string(),
                author: None,
                authenticity: AuthenticityLevel::Verified,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: chrono::Utc::now(),
            }
        ];
        
        // مصادر منخفضة الجودة
        let low_quality_sources = vec![
            IslamicSource {
                id: "weak_source".to_string(),
                content_type: SourceType::DaifHadith,
                text: "حديث ضعيف عن الصلاة ".to_string(),
                reference: "مصدر ضعيف ".to_string(),
                author: None,
                authenticity: AuthenticityLevel::Questionable,
                language: Language::Arabic,
                metadata: HashMap::new(),
                created_at: chrono::Utc::now(),
            }
        ];
        
        let query = ProcessedQuestion {
            original_text: "ما حكم الصلاة ".to_string(),
            normalized_text: "ما حكم الصلاة ".to_string(),
            keywords: vec!["حكم".to_string(), "صلاة".to_string()],
            concepts: vec!["صلاة".to_string()],
            question_type: QuestionType::Fiqh,
            complexity_level: ComplexityLevel::Simple,
            language: Language::Arabic,
            is_controversial: false,
            requires_multiple_sources: false,
            embedding: None,
        };
        
        let high_confidence = assessor.assess_confidence(response_text, &high_quality_sources, &query).await.unwrap();
        let low_confidence = assessor.assess_confidence(response_text, &low_quality_sources, &query).await.unwrap();
        
        assert!(high_confidence > low_confidence, 
                "High quality sources should result in higher confidence: {} vs {}", 
                high_confidence, low_confidence);
        
        println!("High quality confidence: {:.2}", high_confidence);
        println!("Low quality confidence: {:.2}", low_confidence);
    }
}
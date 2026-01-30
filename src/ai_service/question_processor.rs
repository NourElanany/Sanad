use super::*;
use regex::Regex;
use std::collections::{HashSet, HashMap};

/// Question processor for analyzing and preparing user questions
pub struct QuestionProcessor {
    text_normalizer: TextNormalizer,
    keyword_extractor: KeywordExtractor,
    concept_extractor: ConceptExtractor,
    question_classifier: QuestionClassifier,
    out_of_scope_detector: OutOfScopeDetector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedQuestion {
    pub original_text: String,
    pub normalized_text: String,
    pub keywords: Vec<String>,
    pub concepts: Vec<String>,
    pub question_type: QuestionType,
    pub complexity_level: ComplexityLevel,
    pub language: Language,
    pub is_controversial: bool,
    pub requires_multiple_sources: bool,
    pub embedding: Option<Vec<f32>>,
}

impl QuestionProcessor {
    pub fn new() -> Self {
        Self {
            text_normalizer: TextNormalizer::new(),
            keyword_extractor: KeywordExtractor::new(),
            concept_extractor: ConceptExtractor::new(),
            question_classifier: QuestionClassifier::new(),
            out_of_scope_detector: OutOfScopeDetector::new(),
        }
    }
    
    pub async fn process_question(&self, question: &str) -> Result<ProcessedQuestion> {
        // تحليل نطاق السؤال
        let scope_analysis = self.out_of_scope_detector.get_scope_analysis(question);
        
        // التعامل مع الأسئلة خارج النطاق أو الحدودية
        match scope_analysis.scope_status {
            ScopeStatus::OutOfScope => {
                let fallback_response = self.out_of_scope_detector.generate_fallback_response(&scope_analysis);
                return Err(AIServiceError::OutOfScopeQuestion(fallback_response));
            },
            ScopeStatus::Borderline => {
                // يمكن معالجة الأسئلة الحدودية مع تحذير
                // لكن نتركها تمر للمعالجة مع إضافة تحذير لاحقاً
            },
            ScopeStatus::InScope => {
                // السؤال في النطاق، نتابع المعالجة العادية
            }
        }
        
        // تطبيع النص
        let normalized = self.text_normalizer.normalize(question);
        
        // استخراج الكلمات المفتاحية
        let keywords = self.keyword_extractor.extract(&normalized);
        
        // استخراج المفاهيم
        let concepts = self.concept_extractor.extract(&normalized);
        
        // تصنيف السؤال
        let question_type = self.question_classifier.classify(&normalized, &keywords, &concepts);
        
        // تحديد مستوى التعقيد
        let complexity = self.determine_complexity(&normalized, &keywords, &concepts);
        
        // تحديد اللغة
        let language = self.detect_language(question);
        
        // فحص إذا كان السؤال خلافياً
        let is_controversial = self.is_controversial_question(&normalized, &concepts);
        
        // تحديد إذا كان يحتاج مصادر متعددة
        let requires_multiple = self.requires_multiple_sources(&question_type, &concepts);
        
        Ok(ProcessedQuestion {
            original_text: question.to_string(),
            normalized_text: normalized,
            keywords,
            concepts,
            question_type,
            complexity_level: complexity,
            language,
            is_controversial,
            requires_multiple_sources: requires_multiple,
            embedding: None, // سيتم ملؤها لاحقاً بواسطة نظام البحث الدلالي
        })
    }
    
    fn determine_complexity(&self, text: &str, keywords: &[String], concepts: &[String]) -> ComplexityLevel {
        let mut complexity_score = 0;
        
        // عوامل تزيد التعقيد
        if text.len() > 200 { complexity_score += 1; }
        if keywords.len() > 10 { complexity_score += 1; }
        if concepts.len() > 5 { complexity_score += 1; }
        
        // كلمات تدل على التعقيد
        let complex_indicators = [
            "اختلاف", "خلاف", "مذاهب", "تفصيل", "دليل", "حكم", "علة", "قياس"
        ];
        
        for indicator in &complex_indicators {
            if text.contains(indicator) {
                complexity_score += 1;
            }
        }
        
        match complexity_score {
            0..=2 => ComplexityLevel::Simple,
            3..=5 => ComplexityLevel::Intermediate,
            6..=8 => ComplexityLevel::Advanced,
            _ => ComplexityLevel::Scholarly,
        }
    }
    
    fn detect_language(&self, text: &str) -> Language {
        // تحديد اللغة بناءً على الأحرف المستخدمة
        let arabic_chars: usize = text.chars().filter(|c| *c >= '\u{0600}' && *c <= '\u{06FF}').count();
        let total_chars: usize = text.chars().filter(|c| c.is_alphabetic()).count();
        
        if total_chars == 0 {
            return Language::Arabic; // افتراضي
        }
        
        let arabic_ratio = arabic_chars as f32 / total_chars as f32;
        
        if arabic_ratio > 0.5 {
            Language::Arabic
        } else {
            Language::English // يمكن تحسينه لاكتشاف لغات أخرى
        }
    }
    
    fn is_controversial_question(&self, text: &str, concepts: &[String]) -> bool {
        let controversial_topics = [
            "خلاف", "اختلاف", "مذهب", "رأي", "قول", "وجه", "احتمال"
        ];
        
        for topic in &controversial_topics {
            if text.contains(topic) || concepts.iter().any(|c| c.contains(topic)) {
                return true;
            }
        }
        
        false
    }
    
    fn requires_multiple_sources(&self, question_type: &QuestionType, concepts: &[String]) -> bool {
        match question_type {
            QuestionType::Fiqh | QuestionType::Aqeedah => true,
            _ => concepts.len() > 3,
        }
    }
}

/// Text normalizer for Arabic text processing
pub struct TextNormalizer {
    diacritics_regex: Regex,
    punctuation_regex: Regex,
}

impl TextNormalizer {
    pub fn new() -> Self {
        Self {
            diacritics_regex: Regex::new(r"[\u064B-\u0652\u0670\u0640]").unwrap(),
            punctuation_regex: Regex::new(r"[^\w\s\u0600-\u06FF]").unwrap(),
        }
    }
    
    pub fn normalize(&self, text: &str) -> String {
        let mut normalized = text.to_lowercase();
        
        // إزالة التشكيل
        normalized = self.diacritics_regex.replace_all(&normalized, "").to_string();
        
        // توحيد الهمزات
        normalized = normalized
            .replace("أ", "ا")
            .replace("إ", "ا")
            .replace("آ", "ا")
            .replace("ة", "ه")
            .replace("ى", "ي");
        
        // إزالة علامات الترقيم الزائدة
        normalized = self.punctuation_regex.replace_all(&normalized, " ").to_string();
        
        // إزالة المسافات الزائدة
        normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
        
        normalized
    }
}

/// Keyword extractor for Arabic text
pub struct KeywordExtractor {
    stop_words: HashSet<String>,
}

impl KeywordExtractor {
    pub fn new() -> Self {
        let stop_words = [
            "في", "من", "إلى", "على", "عن", "مع", "هذا", "هذه", "ذلك", "تلك",
            "التي", "الذي", "التي", "اللذان", "اللتان", "الذين", "اللواتي",
            "هل", "ما", "متى", "أين", "كيف", "لماذا", "ماذا", "أي", "كم",
            "أن", "إن", "كان", "كانت", "يكون", "تكون", "سوف", "قد", "لقد"
        ].iter().map(|s| s.to_string()).collect();
        
        Self { stop_words }
    }
    
    pub fn extract(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .filter(|word| !self.stop_words.contains(*word))
            .filter(|word| word.len() > 2)
            .map(|word| word.to_string())
            .collect()
    }
}

/// Concept extractor for identifying Islamic concepts
pub struct ConceptExtractor {
    islamic_concepts: HashMap<String, Vec<String>>,
}

impl ConceptExtractor {
    pub fn new() -> Self {
        let mut concepts = HashMap::new();
        
        // مفاهيم العقيدة
        concepts.insert("عقيدة".to_string(), vec![
            "توحيد".to_string(), "شرك".to_string(), "إيمان".to_string(), 
            "كفر".to_string(), "قدر".to_string(), "قضاء".to_string()
        ]);
        
        // مفاهيم الفقه
        concepts.insert("فقه".to_string(), vec![
            "طهارة".to_string(), "صلاة".to_string(), "زكاة".to_string(),
            "صوم".to_string(), "حج".to_string(), "نكاح".to_string(), "طلاق".to_string()
        ]);
        
        // مفاهيم الحديث
        concepts.insert("حديث".to_string(), vec![
            "صحيح".to_string(), "حسن".to_string(), "ضعيف".to_string(),
            "موضوع".to_string(), "سند".to_string(), "متن".to_string()
        ]);
        
        Self {
            islamic_concepts: concepts,
        }
    }
    
    pub fn extract(&self, text: &str) -> Vec<String> {
        let mut found_concepts = Vec::new();
        
        for (category, concepts) in &self.islamic_concepts {
            for concept in concepts {
                if text.contains(concept) {
                    found_concepts.push(concept.clone());
                }
            }
            
            if text.contains(category) {
                found_concepts.push(category.clone());
            }
        }
        
        found_concepts.sort();
        found_concepts.dedup();
        found_concepts
    }
}

/// Question classifier for determining question type
pub struct QuestionClassifier;

impl QuestionClassifier {
    pub fn new() -> Self {
        Self
    }
    
    pub fn classify(&self, text: &str, keywords: &[String], concepts: &[String]) -> QuestionType {
        // تصنيف بناءً على المفاهيم المستخرجة
        if concepts.iter().any(|c| ["توحيد", "شرك", "إيمان", "كفر"].contains(&c.as_str())) {
            return QuestionType::Aqeedah;
        }
        
        if concepts.iter().any(|c| ["صلاة", "زكاة", "صوم", "حج", "طهارة"].contains(&c.as_str())) {
            return QuestionType::Fiqh;
        }
        
        if concepts.iter().any(|c| ["تفسير", "آية", "سورة"].contains(&c.as_str())) {
            return QuestionType::Tafsir;
        }
        
        if concepts.iter().any(|c| ["حديث", "صحيح", "ضعيف", "سند"].contains(&c.as_str())) {
            return QuestionType::Hadith;
        }
        
        if concepts.iter().any(|c| ["سيرة", "غزوة", "صحابة"].contains(&c.as_str())) {
            return QuestionType::Sirah;
        }
        
        if concepts.iter().any(|c| ["أخلاق", "آداب", "سلوك"].contains(&c.as_str())) {
            return QuestionType::Akhlaq;
        }
        
        if keywords.iter().any(|k| ["دعاء", "ذكر", "تسبيح"].contains(&k.as_str())) {
            return QuestionType::Dua;
        }
        
        QuestionType::General
    }
}

/// Enhanced out of scope detector for non-Islamic questions with fallback responses
pub struct OutOfScopeDetector {
    out_of_scope_indicators: HashSet<String>,
    borderline_topics: HashMap<String, String>,
    fallback_responses: HashMap<String, String>,
    islamic_context_keywords: HashSet<String>,
}

impl OutOfScopeDetector {
    pub fn new() -> Self {
        let indicators = [
            // تكنولوجيا ومعلومات
            "برمجة", "كمبيوتر", "تكنولوجيا", "إنترنت", "هاتف", "تطبيق",
            "ذكي", "روبوت", "ذكاء اصطناعي",
            
            // علوم طبيعية
            "فيزياء", "كيمياء", "رياضيات", "جيولوجيا", "فلك", "بيولوجيا",
            
            // طب وصحة (غير الطب النبوي)
            "دواء", "علاج", "مرض", "طبيب", "مستشفى", "عملية جراحية",
            
            // رياضة وترفيه
            "كرة قدم", "رياضة", "لعبة", "فيلم", "مسلسل", "موسيقى", "غناء",
            
            // سياسة واقتصاد
            "سياسة", "حكومة", "انتخابات", "رئيس", "وزير", "اقتصاد", "بورصة",
            "أسهم", "بنك", "فوائد", "ربا", // ربا قد يكون إسلامي
            
            // طبخ وطعام (غير الحلال والحرام)
            "طبخ", "وصفة", "مطعم", "طعام", "أكل",
            
            // سفر وسياحة
            "سفر", "سياحة", "فندق", "طيران", "تذكرة",
            
            // موضة وجمال
            "موضة", "أزياء", "مكياج", "تجميل", "شعر",
        ].iter().map(|s| s.to_string()).collect();

        let mut borderline_topics = HashMap::new();
        borderline_topics.insert("طب".to_string(), "الطب النبوي والعلاج بالقرآن والسنة".to_string());
        borderline_topics.insert("اقتصاد".to_string(), "الاقتصاد الإسلامي والمعاملات المالية".to_string());
        borderline_topics.insert("قانون".to_string(), "الفقه الإسلامي والأحكام الشرعية".to_string());
        borderline_topics.insert("تاريخ".to_string(), "التاريخ الإسلامي والسيرة النبوية".to_string());
        borderline_topics.insert("فلسفة".to_string(), "الفلسفة الإسلامية وعلم الكلام".to_string());
        borderline_topics.insert("علم نفس".to_string(), "التزكية والأخلاق الإسلامية".to_string());

        let mut fallback_responses = HashMap::new();
        fallback_responses.insert("تكنولوجيا".to_string(), 
            "أعتذر، لكن تخصصي في الشؤون الإسلامية. إذا كان لديك سؤال حول الاستخدام الإسلامي للتكنولوجيا أو آدابها في الإسلام، فسأكون سعيداً لمساعدتك.".to_string());
        
        fallback_responses.insert("طب".to_string(),
            "تخصصي في الشؤون الإسلامية وليس الطب. لكن يمكنني مساعدتك في موضوعات الطب النبوي، أو الأحكام الشرعية المتعلقة بالعلاج والصحة.".to_string());
        
        fallback_responses.insert("عام".to_string(),
            "أعتذر، هذا السؤال خارج نطاق تخصصي في الشؤون الإسلامية. أنا هنا لمساعدتك في أمور الدين والفقه والعقيدة والقرآن والسنة. هل لديك سؤال إسلامي يمكنني مساعدتك فيه؟".to_string());

        let islamic_context_keywords = [
            "إسلام", "مسلم", "قرآن", "حديث", "سنة", "فقه", "شريعة", "حلال", "حرام",
            "صلاة", "زكاة", "صوم", "حج", "عمرة", "دعاء", "ذكر", "تسبيح",
            "نبي", "رسول", "صحابة", "تابعين", "علماء", "مذهب", "عقيدة",
            "جنة", "نار", "آخرة", "يوم القيامة", "ملائكة", "جن", "شيطان"
        ].iter().map(|s| s.to_string()).collect();

        Self {
            out_of_scope_indicators: indicators,
            borderline_topics,
            fallback_responses,
            islamic_context_keywords,
        }
    }
    
    pub fn is_out_of_scope(&self, text: &str) -> bool {
        let normalized = text.to_lowercase();
        
        // فحص وجود كلمات إسلامية - إذا وجدت، فالسؤال قد يكون في النطاق
        let has_islamic_context = self.islamic_context_keywords.iter()
            .any(|keyword| normalized.contains(keyword));
        
        if has_islamic_context {
            return false; // لا نرفض الأسئلة التي تحتوي على سياق إسلامي
        }
        
        // فحص المؤشرات الواضحة لخروج عن النطاق
        let out_of_scope_count = self.out_of_scope_indicators.iter()
            .filter(|indicator| normalized.contains(&indicator.to_lowercase()))
            .count();
        
        // إذا كان هناك أكثر من مؤشر واحد، أو مؤشر واحد قوي
        out_of_scope_count > 0
    }
    
    pub fn get_scope_analysis(&self, text: &str) -> ScopeAnalysis {
        let normalized = text.to_lowercase();
        
        // تحليل السياق الإسلامي
        let islamic_keywords: Vec<String> = self.islamic_context_keywords.iter()
            .filter(|keyword| normalized.contains(&keyword.to_lowercase()))
            .cloned()
            .collect();
        
        // تحليل المؤشرات خارج النطاق
        let out_of_scope_keywords: Vec<String> = self.out_of_scope_indicators.iter()
            .filter(|indicator| normalized.contains(&indicator.to_lowercase()))
            .cloned()
            .collect();
        
        // تحليل الموضوعات الحدودية
        let borderline_topics: Vec<String> = self.borderline_topics.keys()
            .filter(|topic| normalized.contains(&topic.to_lowercase()))
            .cloned()
            .collect();
        
        // حساب درجة الانتماء للنطاق الإسلامي
        let islamic_score = islamic_keywords.len() as f32;
        let out_of_scope_score = out_of_scope_keywords.len() as f32;
        let borderline_score = borderline_topics.len() as f32 * 0.5;
        
        let total_score = islamic_score - out_of_scope_score + borderline_score;
        
        let scope_status = if total_score > 0.5 {
            ScopeStatus::InScope
        } else if total_score > -0.5 || !borderline_topics.is_empty() {
            ScopeStatus::Borderline
        } else {
            ScopeStatus::OutOfScope
        };
        
        ScopeAnalysis {
            scope_status,
            islamic_keywords,
            out_of_scope_keywords,
            borderline_topics,
            confidence_score: (total_score + 2.0) / 4.0, // تطبيع بين 0 و 1
            suggested_islamic_angle: self.suggest_islamic_angle(&borderline_topics),
        }
    }
    
    pub fn generate_fallback_response(&self, analysis: &ScopeAnalysis) -> String {
        match analysis.scope_status {
            ScopeStatus::InScope => {
                "هذا السؤال في نطاق تخصصي. سأحاول الإجابة عليه بإذن الله.".to_string()
            },
            ScopeStatus::Borderline => {
                if let Some(suggestion) = &analysis.suggested_islamic_angle {
                    format!(
                        "هذا الموضوع يمكن أن يكون له جانب إسلامي. {}. هل تريد أن أركز على الجانب الإسلامي؟",
                        suggestion
                    )
                } else {
                    "هذا الموضوع قد يكون له جوانب إسلامية. هل يمكنك توضيح السؤال أكثر ليكون في نطاق الشؤون الإسلامية؟".to_string()
                }
            },
            ScopeStatus::OutOfScope => {
                // اختيار الرد المناسب بناءً على الموضوع
                let category = self.categorize_out_of_scope_topic(&analysis.out_of_scope_keywords);
                self.fallback_responses.get(&category)
                    .unwrap_or(&self.fallback_responses["عام"])
                    .clone()
            }
        }
    }
    
    fn suggest_islamic_angle(&self, borderline_topics: &[String]) -> Option<String> {
        for topic in borderline_topics {
            if let Some(islamic_angle) = self.borderline_topics.get(topic) {
                return Some(format!("يمكنني مساعدتك في {}", islamic_angle));
            }
        }
        None
    }
    
    fn categorize_out_of_scope_topic(&self, keywords: &[String]) -> String {
        for keyword in keywords {
            if ["برمجة", "كمبيوتر", "تكنولوجيا"].contains(&keyword.as_str()) {
                return "تكنولوجيا".to_string();
            }
            if ["دواء", "علاج", "طبيب"].contains(&keyword.as_str()) {
                return "طب".to_string();
            }
            if ["رياضة", "لعبة", "فيلم"].contains(&keyword.as_str()) {
                return "ترفيه".to_string();
            }
        }
        "عام".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ScopeAnalysis {
    pub scope_status: ScopeStatus,
    pub islamic_keywords: Vec<String>,
    pub out_of_scope_keywords: Vec<String>,
    pub borderline_topics: Vec<String>,
    pub confidence_score: f32,
    pub suggested_islamic_angle: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeStatus {
    InScope,      // داخل النطاق الإسلامي
    Borderline,   // على الحدود - يمكن أن يكون له جانب إسلامي
    OutOfScope,   // خارج النطاق تماماً
}
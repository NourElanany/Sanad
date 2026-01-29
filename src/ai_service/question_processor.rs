use super::*;
use regex::Regex;
use std::collections::HashSet;

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
        // التحقق من أن السؤال ليس خارج النطاق
        if self.out_of_scope_detector.is_out_of_scope(question) {
            return Err(AIServiceError::OutOfScopeQuestion(
                "السؤال خارج نطاق الاختصاص الإسلامي".to_string()
            ));
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

/// Out of scope detector for non-Islamic questions
pub struct OutOfScopeDetector {
    out_of_scope_indicators: HashSet<String>,
}

impl OutOfScopeDetector {
    pub fn new() -> Self {
        let indicators = [
            "سياسة", "اقتصاد", "رياضة", "فن", "موسيقى", "سينما",
            "طب", "هندسة", "رياضيات", "فيزياء", "كيمياء",
            "برمجة", "تكنولوجيا", "كمبيوتر"
        ].iter().map(|s| s.to_string()).collect();
        
        Self {
            out_of_scope_indicators: indicators,
        }
    }
    
    pub fn is_out_of_scope(&self, text: &str) -> bool {
        let normalized = text.to_lowercase();
        
        for indicator in &self.out_of_scope_indicators {
            if normalized.contains(indicator) {
                return true;
            }
        }
        
        false
    }
}
use crate::models::*;
use regex::Regex;
use std::collections::HashMap;

/// Language detector for automatic language detection
pub struct LanguageDetector {
    // Language patterns for detection
    arabic_pattern: Regex,
    urdu_pattern: Regex,
    persian_pattern: Regex,
    // Common words for each language
    language_keywords: HashMap<SupportedLanguage, Vec<&'static str>>,
}

impl LanguageDetector {
    pub fn new() -> Self {
        let arabic_pattern = Regex::new(r"[\u0600-\u06FF\u0750-\u077F\u08A0-\u08FF\uFB50-\uFDFF\uFE70-\uFEFF]").unwrap();
        let urdu_pattern = Regex::new(r"[\u0600-\u06FF\u0750-\u077F\u08A0-\u08FF\uFB50-\uFDFF\uFE70-\uFEFF]").unwrap();
        let persian_pattern = Regex::new(r"[\u0600-\u06FF\u0750-\u077F\u08A0-\u08FF\uFB50-\uFDFF\uFE70-\uFEFF]").unwrap();

        let mut language_keywords = HashMap::new();
        
        // Arabic keywords
        language_keywords.insert(SupportedLanguage::Arabic, vec![
            "الله", "القرآن", "الصلاة", "المسجد", "الإسلام", "المسلم", "الحمد", "سبحان",
            "أستغفر", "لا إله إلا الله", "محمد", "رسول", "النبي", "الجنة", "النار",
            "الدين", "الإيمان", "التوحيد", "الشهادة", "الزكاة", "الحج", "الصوم"
        ]);

        // English keywords
        language_keywords.insert(SupportedLanguage::English, vec![
            "Allah", "Quran", "prayer", "mosque", "Islam", "Muslim", "praise", "glory",
            "forgiveness", "prophet", "Muhammad", "faith", "belief", "worship", "pilgrimage",
            "fasting", "charity", "paradise", "hell", "religion", "testimony"
        ]);

        // Urdu keywords
        language_keywords.insert(SupportedLanguage::Urdu, vec![
            "اللہ", "قرآن", "نماز", "مسجد", "اسلام", "مسلمان", "حمد", "سبحان",
            "استغفار", "لا الہ الا اللہ", "محمد", "رسول", "نبی", "جنت", "جہنم",
            "دین", "ایمان", "توحید", "شہادت", "زکوٰۃ", "حج", "روزہ"
        ]);

        // Persian keywords
        language_keywords.insert(SupportedLanguage::Persian, vec![
            "خدا", "قرآن", "نماز", "مسجد", "اسلام", "مسلمان", "حمد", "سبحان",
            "استغفار", "لا اله الا الله", "محمد", "رسول", "پیامبر", "بهشت", "جهنم",
            "دین", "ایمان", "توحید", "شهادت", "زکات", "حج", "روزه"
        ]);

        // Turkish keywords
        language_keywords.insert(SupportedLanguage::Turkish, vec![
            "Allah", "Kuran", "namaz", "cami", "İslam", "Müslüman", "hamd", "subhan",
            "istiğfar", "la ilahe illallah", "Muhammed", "resul", "peygamber", "cennet", "cehennem",
            "din", "iman", "tevhid", "şehadet", "zekat", "hac", "oruç"
        ]);

        // Indonesian keywords
        language_keywords.insert(SupportedLanguage::Indonesian, vec![
            "Allah", "Quran", "shalat", "masjid", "Islam", "Muslim", "puji", "subhan",
            "istighfar", "la ilaha illallah", "Muhammad", "rasul", "nabi", "surga", "neraka",
            "agama", "iman", "tauhid", "syahadat", "zakat", "haji", "puasa"
        ]);

        // Malay keywords
        language_keywords.insert(SupportedLanguage::Malay, vec![
            "Allah", "Quran", "solat", "masjid", "Islam", "Muslim", "puji", "subhan",
            "istighfar", "la ilaha illallah", "Muhammad", "rasul", "nabi", "syurga", "neraka",
            "agama", "iman", "tauhid", "syahadat", "zakat", "haji", "puasa"
        ]);

        // Bengali keywords
        language_keywords.insert(SupportedLanguage::Bengali, vec![
            "আল্লাহ", "কুরআন", "নামাজ", "মসজিদ", "ইসলাম", "মুসলিম", "প্রশংসা", "সুবহান",
            "ইস্তিগফার", "লা ইলাহা ইল্লাল্লাহ", "মুহাম্মদ", "রাসূল", "নবী", "জান্নাত", "জাহান্নাম",
            "দীন", "ঈমান", "তাওহীদ", "শাহাদাত", "যাকাত", "হজ", "রোজা"
        ]);

        // French keywords
        language_keywords.insert(SupportedLanguage::French, vec![
            "Allah", "Coran", "prière", "mosquée", "Islam", "musulman", "louange", "gloire",
            "pardon", "prophète", "Muhammad", "foi", "croyance", "adoration", "pèlerinage",
            "jeûne", "charité", "paradis", "enfer", "religion", "témoignage"
        ]);

        // Spanish keywords
        language_keywords.insert(SupportedLanguage::Spanish, vec![
            "Allah", "Corán", "oración", "mezquita", "Islam", "musulmán", "alabanza", "gloria",
            "perdón", "profeta", "Muhammad", "fe", "creencia", "adoración", "peregrinación",
            "ayuno", "caridad", "paraíso", "infierno", "religión", "testimonio"
        ]);

        Self {
            arabic_pattern,
            urdu_pattern,
            persian_pattern,
            language_keywords,
        }
    }

    /// Detect language from text content
    pub fn detect_language(&self, text: &str) -> LanguageDetectionResult {
        let mut scores = HashMap::new();

        // Initialize scores
        for language in SupportedLanguage::all() {
            scores.insert(language, 0.0);
        }

        // Script-based detection
        self.detect_by_script(text, &mut scores);

        // Keyword-based detection
        self.detect_by_keywords(text, &mut scores);

        // Character frequency analysis
        self.detect_by_character_frequency(text, &mut scores);

        // Find the language with highest score
        let mut sorted_scores: Vec<_> = scores.into_iter().collect();
        sorted_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let detected_language = sorted_scores[0].0.clone();
        let confidence = sorted_scores[0].1;

        let alternative_languages = sorted_scores
            .into_iter()
            .skip(1)
            .take(3)
            .collect();

        LanguageDetectionResult {
            detected_language,
            confidence,
            alternative_languages,
        }
    }

    /// Detect language based on script/writing system
    fn detect_by_script(&self, text: &str, scores: &mut HashMap<SupportedLanguage, f32>) {
        let arabic_matches = self.arabic_pattern.find_iter(text).count();
        let total_chars = text.chars().count();

        if total_chars == 0 {
            return;
        }

        let arabic_ratio = arabic_matches as f32 / total_chars as f32;

        if arabic_ratio > 0.5 {
            // High Arabic script presence
            *scores.get_mut(&SupportedLanguage::Arabic).unwrap() += 30.0;
            *scores.get_mut(&SupportedLanguage::Urdu).unwrap() += 25.0;
            *scores.get_mut(&SupportedLanguage::Persian).unwrap() += 20.0;
        } else if arabic_ratio > 0.1 {
            // Some Arabic script presence
            *scores.get_mut(&SupportedLanguage::Arabic).unwrap() += 15.0;
            *scores.get_mut(&SupportedLanguage::Urdu).unwrap() += 10.0;
            *scores.get_mut(&SupportedLanguage::Persian).unwrap() += 8.0;
        } else {
            // Likely Latin script
            *scores.get_mut(&SupportedLanguage::English).unwrap() += 20.0;
            *scores.get_mut(&SupportedLanguage::French).unwrap() += 15.0;
            *scores.get_mut(&SupportedLanguage::Spanish).unwrap() += 15.0;
            *scores.get_mut(&SupportedLanguage::Turkish).unwrap() += 15.0;
            *scores.get_mut(&SupportedLanguage::Indonesian).unwrap() += 15.0;
            *scores.get_mut(&SupportedLanguage::Malay).unwrap() += 15.0;
        }
    }

    /// Detect language based on keywords
    fn detect_by_keywords(&self, text: &str, scores: &mut HashMap<SupportedLanguage, f32>) {
        let text_lower = text.to_lowercase();

        for (language, keywords) in &self.language_keywords {
            let mut keyword_matches = 0;
            for keyword in keywords {
                if text_lower.contains(&keyword.to_lowercase()) {
                    keyword_matches += 1;
                }
            }

            if keyword_matches > 0 {
                let keyword_score = (keyword_matches as f32 / keywords.len() as f32) * 40.0;
                *scores.get_mut(language).unwrap() += keyword_score;
            }
        }
    }

    /// Detect language based on character frequency
    fn detect_by_character_frequency(&self, text: &str, scores: &mut HashMap<SupportedLanguage, f32>) {
        // Common character patterns for different languages
        let char_patterns = [
            // Arabic specific characters
            ('ا', SupportedLanguage::Arabic, 5.0),
            ('ل', SupportedLanguage::Arabic, 4.0),
            ('م', SupportedLanguage::Arabic, 4.0),
            ('ن', SupportedLanguage::Arabic, 3.0),
            ('ر', SupportedLanguage::Arabic, 3.0),
            
            // English specific patterns
            ('e', SupportedLanguage::English, 3.0),
            ('t', SupportedLanguage::English, 2.5),
            ('a', SupportedLanguage::English, 2.5),
            ('o', SupportedLanguage::English, 2.0),
            ('i', SupportedLanguage::English, 2.0),
            
            // Turkish specific characters
            ('ı', SupportedLanguage::Turkish, 5.0),
            ('ğ', SupportedLanguage::Turkish, 4.0),
            ('ş', SupportedLanguage::Turkish, 4.0),
            ('ç', SupportedLanguage::Turkish, 3.0),
            ('ü', SupportedLanguage::Turkish, 3.0),
        ];

        for (ch, language, weight) in char_patterns {
            let count = text.chars().filter(|&c| c == ch).count();
            if count > 0 {
                *scores.get_mut(&language).unwrap() += count as f32 * weight * 0.1;
            }
        }
    }

    /// Detect language from HTTP Accept-Language header
    pub fn detect_from_accept_language(&self, accept_language: &str) -> Option<SupportedLanguage> {
        // Parse Accept-Language header (simplified)
        let languages: Vec<&str> = accept_language
            .split(',')
            .map(|lang| lang.split(';').next().unwrap_or("").trim())
            .collect();

        for lang_code in languages {
            let code = lang_code.split('-').next().unwrap_or("");
            if let Some(language) = SupportedLanguage::from_code(code) {
                return Some(language);
            }
        }

        None
    }

    /// Detect language from user agent or other hints
    pub fn detect_from_hints(&self, hints: &HashMap<String, String>) -> Option<SupportedLanguage> {
        // Check for explicit language hint
        if let Some(lang_hint) = hints.get("language") {
            if let Some(language) = SupportedLanguage::from_code(lang_hint) {
                return Some(language);
            }
        }

        // Check for region/country hints
        if let Some(country) = hints.get("country") {
            match country.to_lowercase().as_str() {
                "sa" | "ae" | "eg" | "jo" | "lb" | "sy" | "iq" | "ye" | "om" | "qa" | "bh" | "kw" => {
                    return Some(SupportedLanguage::Arabic);
                }
                "pk" | "in" => return Some(SupportedLanguage::Urdu),
                "ir" | "af" => return Some(SupportedLanguage::Persian),
                "tr" => return Some(SupportedLanguage::Turkish),
                "id" => return Some(SupportedLanguage::Indonesian),
                "my" | "bn" => return Some(SupportedLanguage::Malay),
                "bd" => return Some(SupportedLanguage::Bengali),
                "fr" => return Some(SupportedLanguage::French),
                "es" | "mx" | "ar" | "co" | "pe" | "ve" | "cl" | "ec" | "gt" | "cu" | "bo" | "do" | "hn" | "py" | "sv" | "ni" | "cr" | "pa" | "uy" => {
                    return Some(SupportedLanguage::Spanish);
                }
                _ => {}
            }
        }

        None
    }
}

impl Default for LanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arabic_detection() {
        let detector = LanguageDetector::new();
        let arabic_text = "بسم الله الرحمن الرحيم. الحمد لله رب العالمين";
        
        let result = detector.detect_language(arabic_text);
        assert_eq!(result.detected_language, SupportedLanguage::Arabic);
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_english_detection() {
        let detector = LanguageDetector::new();
        let english_text = "In the name of Allah, the Most Gracious, the Most Merciful. Praise be to Allah, Lord of the worlds.";
        
        let result = detector.detect_language(english_text);
        assert_eq!(result.detected_language, SupportedLanguage::English);
        assert!(result.confidence > 0.3);
    }

    #[test]
    fn test_accept_language_detection() {
        let detector = LanguageDetector::new();
        
        assert_eq!(
            detector.detect_from_accept_language("ar-SA,ar;q=0.9,en;q=0.8"),
            Some(SupportedLanguage::Arabic)
        );
        
        assert_eq!(
            detector.detect_from_accept_language("en-US,en;q=0.9"),
            Some(SupportedLanguage::English)
        );
        
        assert_eq!(
            detector.detect_from_accept_language("tr-TR,tr;q=0.9"),
            Some(SupportedLanguage::Turkish)
        );
    }

    #[test]
    fn test_country_hints() {
        let detector = LanguageDetector::new();
        let mut hints = HashMap::new();
        
        hints.insert("country".to_string(), "sa".to_string());
        assert_eq!(detector.detect_from_hints(&hints), Some(SupportedLanguage::Arabic));
        
        hints.insert("country".to_string(), "tr".to_string());
        assert_eq!(detector.detect_from_hints(&hints), Some(SupportedLanguage::Turkish));
        
        hints.insert("country".to_string(), "id".to_string());
        assert_eq!(detector.detect_from_hints(&hints), Some(SupportedLanguage::Indonesian));
    }
}
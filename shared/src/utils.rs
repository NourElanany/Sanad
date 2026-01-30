use chrono::{DateTime, Utc, NaiveDate, Datelike};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

/// Utility functions for the Islamic application

/// Calculate SHA-256 hash for content integrity verification
pub fn calculate_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Verify content integrity using SHA-256 hash
pub fn verify_content_integrity(content: &str, expected_hash: &str) -> bool {
    let calculated_hash = calculate_content_hash(content);
    calculated_hash == expected_hash
}

/// Convert Gregorian date to Hijri date (simplified calculation)
/// Note: This is a basic implementation. For production, use a more accurate library
pub fn gregorian_to_hijri(gregorian_date: DateTime<Utc>) -> HijriDate {
    // Simplified Hijri conversion - in production, use a proper Islamic calendar library
    let gregorian_year = gregorian_date.year();
    let gregorian_month = gregorian_date.month();
    let gregorian_day = gregorian_date.day();
    
    // Basic approximation: Hijri year = (Gregorian year - 622) * 1.030684
    let hijri_year = ((gregorian_year - 622) as f64 * 1.030684) as i32;
    
    // This is a very simplified conversion - replace with proper Islamic calendar calculation
    HijriDate {
        year: hijri_year,
        month: gregorian_month as u8,
        day: gregorian_day as u8,
        month_name: get_hijri_month_name(gregorian_month as u8),
    }
}

/// Convert Hijri date to Gregorian date (simplified calculation)
pub fn hijri_to_gregorian(hijri_date: &HijriDate) -> DateTime<Utc> {
    // Simplified conversion - in production, use a proper Islamic calendar library
    let gregorian_year = ((hijri_date.year as f64 / 1.030684) + 622.0) as i32;
    
    // Create a naive date and convert to UTC
    let naive_date = NaiveDate::from_ymd_opt(
        gregorian_year,
        hijri_date.month as u32,
        hijri_date.day as u32,
    ).unwrap_or_else(|| NaiveDate::from_ymd_opt(gregorian_year, 1, 1).unwrap());
    
    DateTime::from_naive_utc_and_offset(
        naive_date.and_hms_opt(0, 0, 0).unwrap(),
        Utc,
    )
}

/// Get Hijri month name in Arabic
pub fn get_hijri_month_name(month: u8) -> String {
    match month {
        1 => "محرم".to_string(),
        2 => "صفر".to_string(),
        3 => "ربيع الأول".to_string(),
        4 => "ربيع الثاني".to_string(),
        5 => "جمادى الأولى".to_string(),
        6 => "جمادى الثانية".to_string(),
        7 => "رجب".to_string(),
        8 => "شعبان".to_string(),
        9 => "رمضان".to_string(),
        10 => "شوال".to_string(),
        11 => "ذو القعدة".to_string(),
        12 => "ذو الحجة".to_string(),
        _ => "غير معروف".to_string(),
    }
}

/// Normalize Arabic text for better search and comparison
pub fn normalize_arabic_text(text: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    
    text.nfc()
        .collect::<String>()
        // Remove diacritics (tashkeel) for better search
        .chars()
        .filter(|&c| !is_arabic_diacritic(c))
        .collect::<String>()
        // Normalize different forms of the same letter
        .replace('ي', "ی")
        .replace('ك', "ک")
        .replace('ة', "ه")
}

/// Check if a character is an Arabic diacritic
fn is_arabic_diacritic(c: char) -> bool {
    matches!(c,
        '\u{064B}' | // Fathatan
        '\u{064C}' | // Dammatan
        '\u{064D}' | // Kasratan
        '\u{064E}' | // Fatha
        '\u{064F}' | // Damma
        '\u{0650}' | // Kasra
        '\u{0651}' | // Shadda
        '\u{0652}' | // Sukun
        '\u{0653}' | // Maddah
        '\u{0654}' | // Hamza above
        '\u{0655}' | // Hamza below
        '\u{0656}' | // Subscript alef
        '\u{0657}' | // Inverted damma
        '\u{0658}' | // Mark noon ghunna
        '\u{0659}' | // Zwarakay
        '\u{065A}' | // Vowel sign small v above
        '\u{065B}' | // Vowel sign inverted small v above
        '\u{065C}' | // Vowel sign dot below
        '\u{065D}' | // Reversed damma
        '\u{065E}' | // Fatha with two dots
        '\u{065F}' | // Wavy hamza below
        '\u{0670}'   // Superscript alef
    )
}

/// Extract Arabic root from a word (simplified implementation)
pub fn extract_arabic_root(word: &str) -> Option<String> {
    // This is a very simplified root extraction
    // In production, use a proper Arabic morphological analyzer
    let normalized = normalize_arabic_text(word);
    
    // Remove common prefixes and suffixes
    let root = normalized
        .trim_start_matches(&['ا', 'ل', 'ب', 'ف', 'ك', 'و'])
        .trim_end_matches(&['ة', 'ه', 'ت', 'ن', 'ي'])
        .to_string();
    
    // Keep only the core consonants (simplified)
    if root.len() >= 3 {
        Some(root.chars().take(3).collect())
    } else {
        None
    }
}

/// Calculate cosine similarity between two vectors
pub fn cosine_similarity(vec1: &[f32], vec2: &[f32]) -> f32 {
    if vec1.len() != vec2.len() {
        return 0.0;
    }
    
    let dot_product: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
    let magnitude1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        0.0
    } else {
        dot_product / (magnitude1 * magnitude2)
    }
}

/// Generate a unique request ID for tracing
pub fn generate_request_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Validate email format
pub fn is_valid_email(email: &str) -> bool {
    if email.len() <= 5 || !email.contains('@') || !email.contains('.') {
        return false;
    }
    
    // Check that email doesn't start or end with @
    if email.starts_with('@') || email.ends_with('@') {
        return false;
    }
    
    // Basic check for @ position
    let at_pos = email.find('@').unwrap();
    at_pos > 0 && at_pos < email.len() - 1
}

/// Sanitize user input to prevent XSS and injection attacks
pub fn sanitize_input(input: &str) -> String {
    input
        .replace('&', "&amp;") // Replace & first to avoid double encoding
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Rate limiting helper - simple in-memory implementation
pub struct RateLimiter {
    requests: HashMap<String, Vec<DateTime<Utc>>>,
    max_requests: usize,
    window_seconds: i64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_seconds: i64) -> Self {
        Self {
            requests: HashMap::new(),
            max_requests,
            window_seconds,
        }
    }
    
    pub fn is_allowed(&mut self, key: &str) -> bool {
        let now = Utc::now();
        let window_start = now - chrono::Duration::seconds(self.window_seconds);
        
        let requests = self.requests.entry(key.to_string()).or_insert_with(Vec::new);
        
        // Remove old requests
        requests.retain(|&timestamp| timestamp > window_start);
        
        // Check if under limit
        if requests.len() < self.max_requests {
            requests.push(now);
            true
        } else {
            false
        }
    }
}

// Re-export the HijriDate struct from models
use crate::models::HijriDate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_hash() {
        let content = "بسم الله الرحمن الرحيم";
        let hash = calculate_content_hash(content);
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 character hex string
    }

    #[test]
    fn test_content_integrity() {
        let content = "الحمد لله رب العالمين";
        let hash = calculate_content_hash(content);
        assert!(verify_content_integrity(content, &hash));
        assert!(!verify_content_integrity("different content", &hash));
    }

    #[test]
    fn test_arabic_normalization() {
        let text = "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ";
        let normalized = normalize_arabic_text(text);
        assert!(!normalized.contains('\u{064E}')); // Should not contain fatha
    }

    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("user@example.com"));
        assert!(!is_valid_email("invalid-email"));
        assert!(!is_valid_email("@example.com"));
    }

    #[test]
    fn test_input_sanitization() {
        let malicious_input = "<script>alert('xss')</script>";
        let sanitized = sanitize_input(malicious_input);
        assert!(!sanitized.contains("<script>"));
        assert!(sanitized.contains("&lt;script&gt;"));
    }
}
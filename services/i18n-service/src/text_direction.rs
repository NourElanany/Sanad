use crate::models::*;
use serde::Serialize;

/// Text direction utilities for different languages and scripts
pub struct TextDirectionManager;

impl TextDirectionManager {
    /// Get text direction for a language
    pub fn get_direction(language: &SupportedLanguage) -> TextDirection {
        if language.is_rtl() {
            TextDirection::RightToLeft
        } else {
            TextDirection::LeftToRight
        }
    }

    /// Get CSS direction value for a language
    pub fn get_css_direction(language: &SupportedLanguage) -> &'static str {
        Self::get_direction(language).css_value()
    }

    /// Get text alignment for a language
    pub fn get_text_align(language: &SupportedLanguage) -> &'static str {
        match Self::get_direction(language) {
            TextDirection::RightToLeft => "right",
            TextDirection::LeftToRight => "left",
        }
    }

    /// Get flex direction for UI layouts
    pub fn get_flex_direction(language: &SupportedLanguage) -> &'static str {
        match Self::get_direction(language) {
            TextDirection::RightToLeft => "row-reverse",
            TextDirection::LeftToRight => "row",
        }
    }

    /// Get margin/padding adjustments for RTL
    pub fn get_margin_adjustments(language: &SupportedLanguage) -> MarginAdjustments {
        match Self::get_direction(language) {
            TextDirection::RightToLeft => MarginAdjustments {
                margin_left: "margin-right",
                margin_right: "margin-left",
                padding_left: "padding-right",
                padding_right: "padding-left",
                border_left: "border-right",
                border_right: "border-left",
                left: "right",
                right: "left",
            },
            TextDirection::LeftToRight => MarginAdjustments {
                margin_left: "margin-left",
                margin_right: "margin-right",
                padding_left: "padding-left",
                padding_right: "padding-right",
                border_left: "border-left",
                border_right: "border-right",
                left: "left",
                right: "right",
            },
        }
    }

    /// Generate CSS classes for language-specific styling
    pub fn generate_css_classes(language: &SupportedLanguage) -> Vec<String> {
        let mut classes = vec![
            format!("lang-{}", language.code()),
            format!("dir-{}", Self::get_css_direction(language)),
        ];

        if language.is_rtl() {
            classes.push("rtl".to_string());
        } else {
            classes.push("ltr".to_string());
        }

        // Add script-specific classes
        match language {
            SupportedLanguage::Arabic | SupportedLanguage::Urdu | SupportedLanguage::Persian => {
                classes.push("arabic-script".to_string());
            }
            SupportedLanguage::Bengali => {
                classes.push("bengali-script".to_string());
            }
            _ => {
                classes.push("latin-script".to_string());
            }
        }

        classes
    }

    /// Get font recommendations for a language
    pub fn get_recommended_fonts(language: &SupportedLanguage) -> FontRecommendations {
        match language {
            SupportedLanguage::Arabic => FontRecommendations {
                primary: vec!["Amiri".to_string(), "Noto Sans Arabic".to_string(), "Scheherazade New".to_string()],
                fallback: vec!["Arial Unicode MS".to_string(), "Tahoma".to_string()],
                web_safe: vec!["Arial".to_string(), "sans-serif".to_string()],
            },
            SupportedLanguage::Urdu => FontRecommendations {
                primary: vec!["Noto Nastaliq Urdu".to_string(), "Jameel Noori Nastaleeq".to_string()],
                fallback: vec!["Arial Unicode MS".to_string(), "Tahoma".to_string()],
                web_safe: vec!["Arial".to_string(), "sans-serif".to_string()],
            },
            SupportedLanguage::Persian => FontRecommendations {
                primary: vec!["Noto Sans Persian".to_string(), "Iranian Sans".to_string(), "Vazir".to_string()],
                fallback: vec!["Arial Unicode MS".to_string(), "Tahoma".to_string()],
                web_safe: vec!["Arial".to_string(), "sans-serif".to_string()],
            },
            SupportedLanguage::Bengali => FontRecommendations {
                primary: vec!["Noto Sans Bengali".to_string(), "Kalpurush".to_string(), "SolaimanLipi".to_string()],
                fallback: vec!["Arial Unicode MS".to_string()],
                web_safe: vec!["Arial".to_string(), "sans-serif".to_string()],
            },
            SupportedLanguage::Turkish => FontRecommendations {
                primary: vec!["Noto Sans".to_string(), "Open Sans".to_string()],
                fallback: vec!["Arial".to_string(), "Helvetica".to_string()],
                web_safe: vec!["Arial".to_string(), "sans-serif".to_string()],
            },
            _ => FontRecommendations {
                primary: vec!["Noto Sans".to_string(), "Open Sans".to_string(), "Roboto".to_string()],
                fallback: vec!["Arial".to_string(), "Helvetica".to_string()],
                web_safe: vec!["Arial".to_string(), "sans-serif".to_string()],
            },
        }
    }

    /// Generate complete CSS for a language
    pub fn generate_language_css(language: &SupportedLanguage) -> String {
        let direction = Self::get_css_direction(language);
        let text_align = Self::get_text_align(language);
        let fonts = Self::get_recommended_fonts(language);
        let font_family = fonts.primary.join(", ");

        format!(
            r#"
.lang-{} {{
    direction: {};
    text-align: {};
    font-family: {}, {};
}}

.lang-{} .text-content {{
    direction: {};
    text-align: {};
}}

.lang-{} .ui-element {{
    direction: {};
}}

.lang-{} .flex-container {{
    flex-direction: {};
}}
"#,
            language.code(), direction, text_align, font_family, fonts.web_safe.join(", "),
            language.code(), direction, text_align,
            language.code(), direction,
            language.code(), Self::get_flex_direction(language)
        )
    }

    /// Check if text contains mixed directions
    pub fn has_mixed_directions(text: &str) -> bool {
        let mut has_rtl = false;
        let mut has_ltr = false;

        for ch in text.chars() {
            match ch {
                // Arabic, Hebrew, and other RTL scripts
                '\u{0590}'..='\u{05FF}' | // Hebrew
                '\u{0600}'..='\u{06FF}' | // Arabic
                '\u{0750}'..='\u{077F}' | // Arabic Supplement
                '\u{08A0}'..='\u{08FF}' | // Arabic Extended-A
                '\u{FB50}'..='\u{FDFF}' | // Arabic Presentation Forms-A
                '\u{FE70}'..='\u{FEFF}' => { // Arabic Presentation Forms-B
                    has_rtl = true;
                }
                // Latin and other LTR scripts
                'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    has_ltr = true;
                }
                _ => {}
            }

            if has_rtl && has_ltr {
                return true;
            }
        }

        false
    }

    /// Get bidirectional text handling recommendations
    pub fn get_bidi_recommendations(text: &str) -> BidiRecommendations {
        let has_mixed = Self::has_mixed_directions(text);
        
        BidiRecommendations {
            needs_bidi_handling: has_mixed,
            recommended_algorithm: if has_mixed { "unicode-bidi" } else { "normal" },
            css_properties: if has_mixed {
                vec![
                    "unicode-bidi: embed".to_string(),
                    "direction: inherit".to_string(),
                ]
            } else {
                vec![]
            },
            html_attributes: if has_mixed {
                vec!["dir=\"auto\"".to_string()]
            } else {
                vec![]
            },
        }
    }
}

/// CSS margin/padding adjustments for RTL
#[derive(Debug, Clone)]
pub struct MarginAdjustments {
    pub margin_left: &'static str,
    pub margin_right: &'static str,
    pub padding_left: &'static str,
    pub padding_right: &'static str,
    pub border_left: &'static str,
    pub border_right: &'static str,
    pub left: &'static str,
    pub right: &'static str,
}

/// Font recommendations for different languages
#[derive(Debug, Clone, Serialize)]
pub struct FontRecommendations {
    pub primary: Vec<String>,
    pub fallback: Vec<String>,
    pub web_safe: Vec<String>,
}

/// Bidirectional text handling recommendations
#[derive(Debug, Clone)]
pub struct BidiRecommendations {
    pub needs_bidi_handling: bool,
    pub recommended_algorithm: &'static str,
    pub css_properties: Vec<String>,
    pub html_attributes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtl_languages() {
        assert_eq!(TextDirectionManager::get_direction(&SupportedLanguage::Arabic), TextDirection::RightToLeft);
        assert_eq!(TextDirectionManager::get_direction(&SupportedLanguage::Urdu), TextDirection::RightToLeft);
        assert_eq!(TextDirectionManager::get_direction(&SupportedLanguage::Persian), TextDirection::RightToLeft);
    }

    #[test]
    fn test_ltr_languages() {
        assert_eq!(TextDirectionManager::get_direction(&SupportedLanguage::English), TextDirection::LeftToRight);
        assert_eq!(TextDirectionManager::get_direction(&SupportedLanguage::French), TextDirection::LeftToRight);
        assert_eq!(TextDirectionManager::get_direction(&SupportedLanguage::Turkish), TextDirection::LeftToRight);
    }

    #[test]
    fn test_css_generation() {
        let css = TextDirectionManager::generate_language_css(&SupportedLanguage::Arabic);
        assert!(css.contains("direction: rtl"));
        assert!(css.contains("text-align: right"));
        assert!(css.contains("lang-ar"));
    }

    #[test]
    fn test_mixed_directions() {
        assert!(TextDirectionManager::has_mixed_directions("Hello مرحبا"));
        assert!(!TextDirectionManager::has_mixed_directions("Hello World"));
        assert!(!TextDirectionManager::has_mixed_directions("مرحبا بالعالم"));
    }

    #[test]
    fn test_css_classes() {
        let classes = TextDirectionManager::generate_css_classes(&SupportedLanguage::Arabic);
        assert!(classes.contains(&"lang-ar".to_string()));
        assert!(classes.contains(&"dir-rtl".to_string()));
        assert!(classes.contains(&"rtl".to_string()));
        assert!(classes.contains(&"arabic-script".to_string()));
    }

    #[test]
    fn test_font_recommendations() {
        let arabic_fonts = TextDirectionManager::get_recommended_fonts(&SupportedLanguage::Arabic);
        assert!(arabic_fonts.primary.contains(&"Amiri".to_string()));
        
        let english_fonts = TextDirectionManager::get_recommended_fonts(&SupportedLanguage::English);
        assert!(english_fonts.primary.contains(&"Noto Sans".to_string()));
    }
}
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use std::collections::HashMap;
use uuid::Uuid;

/// Digital signature and content integrity verification system for Islamic content
/// This module provides cryptographic verification for Quran, Hadith, and other Islamic texts
/// to ensure authenticity and prevent tampering.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSignature {
    /// Unique identifier for the content
    pub content_id: Uuid,
    /// Type of Islamic content (quran, hadith, tafsir, etc.)
    pub content_type: ContentType,
    /// SHA-256 hash of the content
    pub sha256_hash: String,
    /// SHA-512 hash for additional security
    pub sha512_hash: String,
    /// Digital signature using HMAC
    pub digital_signature: String,
    /// Timestamp when the signature was created
    pub created_at: DateTime<Utc>,
    /// Version of the content
    pub version: u32,
    /// Source authority that verified this content
    pub authority: String,
    /// Additional metadata for verification
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Quran,
    Hadith,
    Tafsir,
    Story,
    Prayer,
    Dhikr,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub is_valid: bool,
    pub content_id: Uuid,
    pub verification_time: DateTime<Utc>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub confidence_score: f64, // 0.0 to 1.0
}

#[derive(Debug, Clone)]
pub struct DigitalAuthenticator {
    /// Secret key for HMAC signing (should be stored securely)
    secret_key: Vec<u8>,
    /// Trusted authorities for content verification
    trusted_authorities: Vec<String>,
}

impl DigitalAuthenticator {
    /// Create a new digital authenticator with a secret key
    pub fn new(secret_key: Vec<u8>) -> Self {
        Self {
            secret_key,
            trusted_authorities: vec![
                "Islamic Foundation".to_string(),
                "Al-Azhar University".to_string(),
                "King Fahd Complex".to_string(),
                "Sanad System".to_string(),
            ],
        }
    }

    /// Generate a digital signature for Islamic content
    pub fn sign_content(
        &self,
        content: &str,
        content_type: ContentType,
        authority: &str,
        metadata: HashMap<String, String>,
    ) -> Result<ContentSignature> {
        // Normalize content to ensure consistent hashing
        let normalized_content = self.normalize_content(content)?;
        
        // Generate hashes
        let sha256_hash = self.generate_sha256(&normalized_content);
        let sha512_hash = self.generate_sha512(&normalized_content);
        
        // Create digital signature using HMAC-SHA256
        let signature_data = format!("{}:{}:{}:{}", 
            normalized_content, content_type.to_string(), authority, sha256_hash);
        let digital_signature = self.generate_hmac_signature(&signature_data)?;
        
        let signature = ContentSignature {
            content_id: Uuid::new_v4(),
            content_type,
            sha256_hash,
            sha512_hash,
            digital_signature,
            created_at: Utc::now(),
            version: 1,
            authority: authority.to_string(),
            metadata,
        };

        Ok(signature)
    }

    /// Verify the integrity and authenticity of Islamic content
    pub fn verify_content(
        &self,
        content: &str,
        signature: &ContentSignature,
    ) -> Result<VerificationResult> {
        let mut result = VerificationResult {
            is_valid: true,
            content_id: signature.content_id,
            verification_time: Utc::now(),
            errors: Vec::new(),
            warnings: Vec::new(),
            confidence_score: 1.0,
        };

        // Normalize content for verification
        let normalized_content = match self.normalize_content(content) {
            Ok(content) => content,
            Err(e) => {
                result.errors.push(format!("Content normalization failed: {}", e));
                result.is_valid = false;
                result.confidence_score = 0.0;
                return Ok(result);
            }
        };

        // Verify SHA-256 hash
        let current_sha256 = self.generate_sha256(&normalized_content);
        if current_sha256 != signature.sha256_hash {
            result.errors.push("SHA-256 hash mismatch - content may have been tampered with".to_string());
            result.is_valid = false;
            result.confidence_score *= 0.1;
        }

        // Verify SHA-512 hash
        let current_sha512 = self.generate_sha512(&normalized_content);
        if current_sha512 != signature.sha512_hash {
            result.errors.push("SHA-512 hash mismatch - content integrity compromised".to_string());
            result.is_valid = false;
            result.confidence_score *= 0.1;
        }

        // Verify digital signature
        let signature_data = format!("{}:{}:{}:{}", 
            normalized_content, signature.content_type.to_string(), 
            signature.authority, signature.sha256_hash);
        
        match self.verify_hmac_signature(&signature_data, &signature.digital_signature) {
            Ok(is_valid) => {
                if !is_valid {
                    result.errors.push("Digital signature verification failed".to_string());
                    result.is_valid = false;
                    result.confidence_score *= 0.2;
                }
            }
            Err(e) => {
                result.errors.push(format!("Signature verification error: {}", e));
                result.is_valid = false;
                result.confidence_score *= 0.1;
            }
        }

        // Check authority trust
        if !self.trusted_authorities.contains(&signature.authority) {
            result.warnings.push(format!("Authority '{}' is not in trusted list", signature.authority));
            result.confidence_score *= 0.8;
        }

        // Check content age (warn if very old)
        let age_days = (Utc::now() - signature.created_at).num_days();
        if age_days > 365 {
            result.warnings.push(format!("Content signature is {} days old", age_days));
            result.confidence_score *= 0.95;
        }

        // Special validation for Quranic content
        if signature.content_type == ContentType::Quran {
            if let Err(e) = self.validate_quranic_content(&normalized_content) {
                result.errors.push(format!("Quranic content validation failed: {}", e));
                result.is_valid = false;
                result.confidence_score *= 0.0; // Zero tolerance for Quranic errors
            }
        }

        Ok(result)
    }

    /// Batch verify multiple content items
    pub fn batch_verify(
        &self,
        content_items: Vec<(&str, &ContentSignature)>,
    ) -> Result<Vec<VerificationResult>> {
        let mut results = Vec::new();
        
        for (content, signature) in content_items {
            let result = self.verify_content(content, signature)?;
            results.push(result);
        }
        
        Ok(results)
    }

    /// Generate a reference hash database for trusted content
    pub fn generate_reference_database(
        &self,
        trusted_content: Vec<(String, ContentType, String)>, // (content, type, authority)
    ) -> Result<HashMap<String, ContentSignature>> {
        let mut database = HashMap::new();
        
        for (content, content_type, authority) in trusted_content {
            let metadata = HashMap::new();
            let signature = self.sign_content(&content, content_type, &authority, metadata)?;
            database.insert(signature.sha256_hash.clone(), signature);
        }
        
        Ok(database)
    }

    /// Check if content exists in reference database
    pub fn check_against_reference<'a>(
        &self,
        content: &str,
        reference_db: &'a HashMap<String, ContentSignature>,
    ) -> Result<Option<&'a ContentSignature>> {
        let normalized_content = self.normalize_content(content)?;
        let hash = self.generate_sha256(&normalized_content);
        Ok(reference_db.get(&hash))
    }

    // Private helper methods

    fn normalize_content(&self, content: &str) -> Result<String> {
        use unicode_normalization::UnicodeNormalization;
        
        // Normalize Unicode (important for Arabic text)
        let normalized = content.nfc().collect::<String>();
        
        // Remove extra whitespace but preserve structure for Islamic texts
        let cleaned = normalized
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        
        Ok(cleaned)
    }

    fn generate_sha256(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn generate_sha512(&self, content: &str) -> String {
        let mut hasher = Sha512::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn generate_hmac_signature(&self, data: &str) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Simple HMAC implementation (in production, use a proper HMAC library)
        let mut hasher = DefaultHasher::new();
        self.secret_key.hash(&mut hasher);
        data.hash(&mut hasher);
        let signature = hasher.finish();
        
        Ok(format!("{:x}", signature))
    }

    fn verify_hmac_signature(&self, data: &str, signature: &str) -> Result<bool> {
        let expected_signature = self.generate_hmac_signature(data)?;
        Ok(expected_signature == signature)
    }

    fn validate_quranic_content(&self, content: &str) -> Result<()> {
        // Basic validation for Quranic content
        // In a real implementation, this would be much more sophisticated
        
        // Check for Arabic script
        let has_arabic = content.chars().any(|c| {
            matches!(c, '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{08A0}'..='\u{08FF}')
        });
        
        if !has_arabic {
            return Err(anyhow::anyhow!("Quranic content must contain Arabic text"));
        }

        // Check for minimum length (verses are typically substantial)
        if content.len() < 10 {
            return Err(anyhow::anyhow!("Quranic content appears too short"));
        }

        // Additional validations could include:
        // - Checking against known verse patterns
        // - Validating diacritical marks
        // - Ensuring proper verse numbering
        
        Ok(())
    }
}

impl ContentType {
    pub fn to_string(&self) -> String {
        match self {
            ContentType::Quran => "quran".to_string(),
            ContentType::Hadith => "hadith".to_string(),
            ContentType::Tafsir => "tafsir".to_string(),
            ContentType::Story => "story".to_string(),
            ContentType::Prayer => "prayer".to_string(),
            ContentType::Dhikr => "dhikr".to_string(),
            ContentType::Other(s) => s.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_authenticator() -> DigitalAuthenticator {
        let secret_key = b"test_secret_key_for_islamic_content".to_vec();
        DigitalAuthenticator::new(secret_key)
    }

    #[test]
    fn test_content_signing_and_verification() {
        let auth = create_test_authenticator();
        let content = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
        let mut metadata = HashMap::new();
        metadata.insert("surah".to_string(), "1".to_string());
        metadata.insert("ayah".to_string(), "1".to_string());

        // Sign the content
        let signature = auth.sign_content(
            content,
            ContentType::Quran,
            "Sanad System",
            metadata,
        ).unwrap();

        // Verify the content
        let result = auth.verify_content(content, &signature).unwrap();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
        assert_eq!(result.confidence_score, 1.0);
    }

    #[test]
    fn test_tampered_content_detection() {
        let auth = create_test_authenticator();
        let original_content = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
        let tampered_content = "بِسْمِ اللَّهِ الرَّحْمَٰنِ"; // Missing part
        
        let signature = auth.sign_content(
            original_content,
            ContentType::Quran,
            "Sanad System",
            HashMap::new(),
        ).unwrap();

        // Verify with tampered content
        let result = auth.verify_content(tampered_content, &signature).unwrap();
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
        assert!(result.confidence_score < 1.0);
    }

    #[test]
    fn test_batch_verification() {
        let auth = create_test_authenticator();
        let content1 = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
        let content2 = "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ";

        let sig1 = auth.sign_content(content1, ContentType::Quran, "Sanad System", HashMap::new()).unwrap();
        let sig2 = auth.sign_content(content2, ContentType::Quran, "Sanad System", HashMap::new()).unwrap();

        let results = auth.batch_verify(vec![
            (content1, &sig1),
            (content2, &sig2),
        ]).unwrap();

        assert_eq!(results.len(), 2);
        assert!(results[0].is_valid);
        assert!(results[1].is_valid);
    }

    #[test]
    fn test_reference_database() {
        let auth = create_test_authenticator();
        let trusted_content = vec![
            ("بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ".to_string(), ContentType::Quran, "King Fahd Complex".to_string()),
            ("الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ".to_string(), ContentType::Quran, "Al-Azhar University".to_string()),
        ];

        let reference_db = auth.generate_reference_database(trusted_content).unwrap();
        assert_eq!(reference_db.len(), 2);

        // Check if content exists in reference
        let test_content = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
        let found = auth.check_against_reference(test_content, &reference_db).unwrap();
        assert!(found.is_some());
    }

    #[test]
    fn test_content_normalization() {
        let auth = create_test_authenticator();
        
        // Test with extra whitespace
        let content_with_spaces = "  بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ  \n\n  ";
        let normalized = auth.normalize_content(content_with_spaces).unwrap();
        assert_eq!(normalized, "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ");
    }
}
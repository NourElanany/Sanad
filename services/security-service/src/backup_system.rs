use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::*;

#[derive(Debug, Serialize, Deserialize)]
struct BackupData {
    content_items: Vec<BackupContentItem>,
    metadata: HashMap<String, String>,
    created_at: chrono::DateTime<chrono::Utc>,
    version: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupContentItem {
    pub content_id: Uuid,
    pub content_hash: String,
    pub content_data: Vec<u8>,
    pub content_type: String,
    pub authority: String,
}

pub struct BackupSystem {
    encryption_key: [u8; 32],
}

impl BackupSystem {
    pub async fn new() -> Result<Self> {
        // In production, this should be loaded from secure key management
        let key_material = std::env::var("BACKUP_ENCRYPTION_KEY")
            .unwrap_or_else(|_| "sanad_backup_encryption_key_2024_secure".to_string());
        
        let mut hasher = Sha256::new();
        hasher.update(key_material.as_bytes());
        let key_hash = hasher.finalize();
        
        let mut encryption_key = [0u8; 32];
        encryption_key.copy_from_slice(&key_hash[..32]);

        Ok(Self { encryption_key })
    }

    /// Create an encrypted backup of Islamic content
    pub async fn create_backup(
        &self,
        content_ids: Vec<Uuid>,
        backup_type: BackupType,
        encryption_level: EncryptionLevel,
        metadata: HashMap<String, String>,
    ) -> Result<EncryptedBackup> {
        // Collect content data (in a real implementation, this would fetch from the database)
        let content_items = self.collect_content_items(content_ids.clone()).await?;

        let backup_data = BackupData {
            content_items,
            metadata: metadata.clone(),
            created_at: Utc::now(),
            version: 1,
        };

        // Serialize the backup data
        let serialized_data = serde_json::to_vec(&backup_data)
            .context("Failed to serialize backup data")?;

        // Compress the data
        let compressed_data = self.compress_data(&serialized_data)?;

        // Encrypt the data based on encryption level
        let encrypted_data = match encryption_level {
            EncryptionLevel::Standard => self.encrypt_standard(&compressed_data)?,
            EncryptionLevel::High => self.encrypt_high(&compressed_data)?,
            EncryptionLevel::Maximum => self.encrypt_maximum(&compressed_data)?,
        };

        // Generate encryption hash for integrity verification
        let encryption_hash = self.generate_encryption_hash(&encrypted_data);

        Ok(EncryptedBackup {
            id: Uuid::new_v4(),
            backup_type,
            encryption_level,
            encrypted_data,
            encryption_hash,
            content_manifest: content_ids,
            created_at: Utc::now(),
            metadata,
        })
    }

    /// Verify the integrity of an encrypted backup
    pub async fn verify_backup_integrity(&self, backup: &EncryptedBackup) -> Result<BackupVerificationResult> {
        let mut result = BackupVerificationResult {
            backup_id: backup.id,
            is_valid: true,
            integrity_score: 1.0,
            errors: Vec::new(),
            warnings: Vec::new(),
            verified_at: Utc::now(),
            content_items_verified: 0,
            corrupted_items: Vec::new(),
        };

        // Verify encryption hash
        let current_hash = self.generate_encryption_hash(&backup.encrypted_data);
        if current_hash != backup.encryption_hash {
            result.errors.push("Encryption hash mismatch - backup may be corrupted".to_string());
            result.is_valid = false;
            result.integrity_score *= 0.1;
            return Ok(result);
        }

        // Attempt to decrypt and verify content
        match self.decrypt_backup(backup).await {
            Ok(backup_data) => {
                result.content_items_verified = backup_data.content_items.len();

                // Verify each content item
                for item in &backup_data.content_items {
                    if !self.verify_content_item(item) {
                        result.corrupted_items.push(item.content_id);
                        result.integrity_score *= 0.9;
                    }
                }

                // Check if all expected content is present
                if backup_data.content_items.len() != backup.content_manifest.len() {
                    result.warnings.push(format!(
                        "Content count mismatch: expected {}, found {}",
                        backup.content_manifest.len(),
                        backup_data.content_items.len()
                    ));
                    result.integrity_score *= 0.95;
                }

                // Check backup age
                let age_days = (Utc::now() - backup.created_at).num_days();
                if age_days > 90 {
                    result.warnings.push(format!("Backup is {} days old", age_days));
                    result.integrity_score *= 0.98;
                }
            }
            Err(e) => {
                result.errors.push(format!("Failed to decrypt backup: {}", e));
                result.is_valid = false;
                result.integrity_score = 0.0;
            }
        }

        if !result.corrupted_items.is_empty() {
            result.is_valid = false;
            result.errors.push(format!("{} content items are corrupted", result.corrupted_items.len()));
        }

        Ok(result)
    }

    /// Restore content from an encrypted backup
    pub async fn restore_backup(&self, backup: &EncryptedBackup) -> Result<Vec<BackupContentItem>> {
        let backup_data = self.decrypt_backup(backup).await?;
        Ok(backup_data.content_items)
    }

    // Private helper methods

    async fn collect_content_items(&self, content_ids: Vec<Uuid>) -> Result<Vec<BackupContentItem>> {
        // In a real implementation, this would fetch content from the database
        // For now, we'll create mock data
        let mut items = Vec::new();
        
        for content_id in content_ids {
            // Mock content data - in reality, this would come from the database
            let mock_content = format!("Mock Islamic content for ID: {}", content_id);
            let content_data = mock_content.as_bytes().to_vec();
            let content_hash = self.generate_content_hash(&content_data);

            items.push(BackupContentItem {
                content_id,
                content_hash,
                content_data,
                content_type: "quran".to_string(),
                authority: "Sanad System".to_string(),
            });
        }

        Ok(items)
    }

    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Simple compression using flate2 (in production, consider using better compression)
        use std::io::Write;
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(data)?;
        Ok(encoder.finish()?)
    }

    fn decompress_data(&self, compressed_data: &[u8]) -> Result<Vec<u8>> {
        use std::io::Read;
        let mut decoder = flate2::read::GzDecoder::new(compressed_data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        Ok(decompressed)
    }

    fn encrypt_standard(&self, data: &[u8]) -> Result<Vec<u8>> {
        let key = Key::<Aes256Gcm>::from_slice(&self.encryption_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        let ciphertext = cipher.encrypt(&nonce, data)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // Prepend nonce to ciphertext
        let mut encrypted_data = nonce.to_vec();
        encrypted_data.extend_from_slice(&ciphertext);
        
        Ok(encrypted_data)
    }

    fn encrypt_high(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Double encryption for high security
        let first_pass = self.encrypt_standard(data)?;
        
        // Generate a second key from the first
        let mut hasher = Sha256::new();
        hasher.update(&self.encryption_key);
        hasher.update(b"high_security_salt");
        let second_key_hash = hasher.finalize();
        
        let mut second_key = [0u8; 32];
        second_key.copy_from_slice(&second_key_hash[..32]);
        
        let key = Key::<Aes256Gcm>::from_slice(&second_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        let ciphertext = cipher.encrypt(&nonce, first_pass.as_slice())
            .map_err(|e| anyhow::anyhow!("High encryption failed: {}", e))?;

        let mut encrypted_data = nonce.to_vec();
        encrypted_data.extend_from_slice(&ciphertext);
        
        Ok(encrypted_data)
    }

    fn encrypt_maximum(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Triple encryption with different algorithms for maximum security
        let first_pass = self.encrypt_high(data)?;
        
        // Third encryption layer
        let mut hasher = Sha512::new();
        hasher.update(&self.encryption_key);
        hasher.update(b"maximum_security_salt_2024");
        let third_key_hash = hasher.finalize();
        
        let mut third_key = [0u8; 32];
        third_key.copy_from_slice(&third_key_hash[..32]);
        
        let key = Key::<Aes256Gcm>::from_slice(&third_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        let ciphertext = cipher.encrypt(&nonce, first_pass.as_slice())
            .map_err(|e| anyhow::anyhow!("Maximum encryption failed: {}", e))?;

        let mut encrypted_data = nonce.to_vec();
        encrypted_data.extend_from_slice(&ciphertext);
        
        Ok(encrypted_data)
    }

    async fn decrypt_backup(&self, backup: &EncryptedBackup) -> Result<BackupData> {
        let decrypted_data = match backup.encryption_level {
            EncryptionLevel::Standard => self.decrypt_standard(&backup.encrypted_data)?,
            EncryptionLevel::High => self.decrypt_high(&backup.encrypted_data)?,
            EncryptionLevel::Maximum => self.decrypt_maximum(&backup.encrypted_data)?,
        };

        let decompressed_data = self.decompress_data(&decrypted_data)?;
        let backup_data: BackupData = serde_json::from_slice(&decompressed_data)
            .context("Failed to deserialize backup data")?;

        Ok(backup_data)
    }

    fn decrypt_standard(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        if encrypted_data.len() < 12 {
            return Err(anyhow::anyhow!("Invalid encrypted data length"));
        }

        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let key = Key::<Aes256Gcm>::from_slice(&self.encryption_key);
        let cipher = Aes256Gcm::new(key);
        
        cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))
    }

    fn decrypt_high(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        // Reverse the double encryption
        if encrypted_data.len() < 12 {
            return Err(anyhow::anyhow!("Invalid encrypted data length"));
        }

        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let mut hasher = Sha256::new();
        hasher.update(&self.encryption_key);
        hasher.update(b"high_security_salt");
        let second_key_hash = hasher.finalize();
        
        let mut second_key = [0u8; 32];
        second_key.copy_from_slice(&second_key_hash[..32]);
        
        let key = Key::<Aes256Gcm>::from_slice(&second_key);
        let cipher = Aes256Gcm::new(key);
        
        let first_pass = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("High decryption failed: {}", e))?;

        self.decrypt_standard(&first_pass)
    }

    fn decrypt_maximum(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        // Reverse the triple encryption
        if encrypted_data.len() < 12 {
            return Err(anyhow::anyhow!("Invalid encrypted data length"));
        }

        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let mut hasher = Sha512::new();
        hasher.update(&self.encryption_key);
        hasher.update(b"maximum_security_salt_2024");
        let third_key_hash = hasher.finalize();
        
        let mut third_key = [0u8; 32];
        third_key.copy_from_slice(&third_key_hash[..32]);
        
        let key = Key::<Aes256Gcm>::from_slice(&third_key);
        let cipher = Aes256Gcm::new(key);
        
        let second_pass = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Maximum decryption failed: {}", e))?;

        self.decrypt_high(&second_pass)
    }

    fn generate_encryption_hash(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(&self.encryption_key); // Include key in hash for additional security
        format!("{:x}", hasher.finalize())
    }

    fn generate_content_hash(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    fn verify_content_item(&self, item: &BackupContentItem) -> bool {
        let current_hash = self.generate_content_hash(&item.content_data);
        current_hash == item.content_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_backup_creation_and_verification() {
        let backup_system = BackupSystem::new().await.unwrap();
        let content_ids = vec![Uuid::new_v4(), Uuid::new_v4()];
        let metadata = HashMap::new();

        let backup = backup_system.create_backup(
            content_ids.clone(),
            BackupType::Full,
            EncryptionLevel::Standard,
            metadata,
        ).await.unwrap();

        assert_eq!(backup.content_manifest, content_ids);
        assert!(!backup.encrypted_data.is_empty());
        assert!(!backup.encryption_hash.is_empty());

        let verification_result = backup_system.verify_backup_integrity(&backup).await.unwrap();
        assert!(verification_result.is_valid);
        assert_eq!(verification_result.content_items_verified, 2);
    }

    #[tokio::test]
    async fn test_high_encryption_level() {
        let backup_system = BackupSystem::new().await.unwrap();
        let content_ids = vec![Uuid::new_v4()];
        let metadata = HashMap::new();

        let backup = backup_system.create_backup(
            content_ids,
            BackupType::Full,
            EncryptionLevel::High,
            metadata,
        ).await.unwrap();

        let verification_result = backup_system.verify_backup_integrity(&backup).await.unwrap();
        assert!(verification_result.is_valid);
    }

    #[tokio::test]
    async fn test_maximum_encryption_level() {
        let backup_system = BackupSystem::new().await.unwrap();
        let content_ids = vec![Uuid::new_v4()];
        let metadata = HashMap::new();

        let backup = backup_system.create_backup(
            content_ids,
            BackupType::Full,
            EncryptionLevel::Maximum,
            metadata,
        ).await.unwrap();

        let verification_result = backup_system.verify_backup_integrity(&backup).await.unwrap();
        assert!(verification_result.is_valid);
    }

    #[tokio::test]
    async fn test_corrupted_backup_detection() {
        let backup_system = BackupSystem::new().await.unwrap();
        let content_ids = vec![Uuid::new_v4()];
        let metadata = HashMap::new();

        let mut backup = backup_system.create_backup(
            content_ids,
            BackupType::Full,
            EncryptionLevel::Standard,
            metadata,
        ).await.unwrap();

        // Corrupt the backup data
        backup.encrypted_data[0] ^= 0xFF;

        let verification_result = backup_system.verify_backup_integrity(&backup).await.unwrap();
        assert!(!verification_result.is_valid);
        assert!(!verification_result.errors.is_empty());
    }
}
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use shared::{DigitalAuthenticator, ContentType};
use std::collections::HashMap;
use uuid::Uuid;

use crate::backup_system::BackupSystem;
use crate::models::*;

/// Property-based tests for the digital authentication and security system
/// **Validates: Requirements 12.1, 12.2, 12.3, 12.4, 12.5**
/// **Feature: islamic-app-comprehensive, Property 14: Data Security and Authentication**

// Simple generators for property-based testing
fn arb_content_type() -> impl Strategy<Value = ContentType> {
    prop_oneof![
        Just(ContentType::Quran),
        Just(ContentType::Hadith),
        Just(ContentType::Tafsir),
        Just(ContentType::Story),
        Just(ContentType::Prayer),
        Just(ContentType::Dhikr),
    ]
}

fn arb_authority() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("Sanad System".to_string()),
        Just("Islamic Foundation".to_string()),
        Just("Al-Azhar University".to_string()),
        Just("King Fahd Complex".to_string()),
    ]
}

fn arb_islamic_content() -> impl Strategy<Value = String> {
    // Generate Arabic-like content for testing
    prop_oneof![
        Just("بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ".to_string()),
        Just("الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ".to_string()),
        Just("الرَّحْمَٰنِ الرَّحِيمِ".to_string()),
        Just("مَالِكِ يَوْمِ الدِّينِ".to_string()),
        Just("إِيَّاكَ نَعْبُدُ وَإِيَّاكَ نَسْتَعِينُ".to_string()),
        Just("اهْدِنَا الصِّرَاطَ الْمُسْتَقِيمَ".to_string()),
    ]
}

fn arb_backup_type() -> impl Strategy<Value = BackupType> {
    prop_oneof![
        Just(BackupType::Full),
        Just(BackupType::Incremental),
        Just(BackupType::Differential),
    ]
}

fn arb_encryption_level() -> impl Strategy<Value = EncryptionLevel> {
    prop_oneof![
        Just(EncryptionLevel::Standard),
        Just(EncryptionLevel::High),
        Just(EncryptionLevel::Maximum),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    /// **Property 14.1: Content Signing Determinism**
    /// For any Islamic content, signing it multiple times with the same parameters
    /// should produce identical signatures
    #[test]
    fn prop_content_signing_determinism(
        content in arb_islamic_content(),
        content_type in arb_content_type(),
        authority in arb_authority()
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let secret_key = b"test_secret_key_for_property_testing".to_vec();
            let authenticator = DigitalAuthenticator::new(secret_key);
            let metadata = HashMap::new();

            let signature1 = authenticator.sign_content(
                &content,
                content_type.clone(),
                &authority,
                metadata.clone(),
            ).unwrap();

            let signature2 = authenticator.sign_content(
                &content,
                content_type,
                &authority,
                metadata,
            ).unwrap();

            // Signatures should have identical hashes (deterministic)
            prop_assert_eq!(signature1.sha256_hash, signature2.sha256_hash);
            prop_assert_eq!(signature1.sha512_hash, signature2.sha512_hash);
            prop_assert_eq!(signature1.digital_signature, signature2.digital_signature);
            Ok(())
        })?;
    }

    /// **Property 14.2: Content Verification Consistency**
    /// For any signed content, verification should always succeed with the original content
    #[test]
    fn prop_content_verification_consistency(
        content in arb_islamic_content(),
        content_type in arb_content_type(),
        authority in arb_authority()
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let secret_key = b"test_secret_key_for_property_testing".to_vec();
            let authenticator = DigitalAuthenticator::new(secret_key);
            let metadata = HashMap::new();

            // Sign the original content
            let signature = authenticator.sign_content(
                &content,
                content_type,
                &authority,
                metadata,
            ).unwrap();

            // Verify original content - should always pass
            let original_result = authenticator.verify_content(&content, &signature).unwrap();
            prop_assert!(original_result.is_valid);
            prop_assert_eq!(original_result.confidence_score, 1.0);
            prop_assert!(original_result.errors.is_empty());
            Ok(())
        })?;
    }

    /// **Property 14.3: Tampered Content Detection**
    /// Any modification to content should be detected during verification
    #[test]
    fn prop_tampered_content_detection(
        content in arb_islamic_content(),
        content_type in arb_content_type(),
        authority in arb_authority()
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let secret_key = b"test_secret_key_for_property_testing".to_vec();
            let authenticator = DigitalAuthenticator::new(secret_key);
            let metadata = HashMap::new();

            // Sign the original content
            let signature = authenticator.sign_content(
                &content,
                content_type,
                &authority,
                metadata,
            ).unwrap();

            // Create tampered content by adding a character
            let tampered_content = format!("{}X", content);

            // Verify tampered content - should always fail
            let tampered_result = authenticator.verify_content(&tampered_content, &signature).unwrap();
            prop_assert!(!tampered_result.is_valid);
            prop_assert!(tampered_result.confidence_score < 1.0);
            prop_assert!(!tampered_result.errors.is_empty());
            Ok(())
        })?;
    }

    /// **Property 14.4: Backup Round-Trip Integrity**
    /// For any backup configuration, creating a backup and then restoring it
    /// should yield the original content
    #[test]
    fn prop_backup_round_trip_integrity(
        backup_type in arb_backup_type(),
        encryption_level in arb_encryption_level(),
        content_count in 1..5usize,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let backup_system = BackupSystem::new().await.unwrap();
            let metadata = HashMap::new();
            
            // Generate content IDs
            let content_ids: Vec<Uuid> = (0..content_count).map(|_| Uuid::new_v4()).collect();
            
            // Create backup
            let backup = backup_system.create_backup(
                content_ids.clone(),
                backup_type,
                encryption_level,
                metadata,
            ).await.unwrap();

            // Verify backup integrity
            let verification_result = backup_system.verify_backup_integrity(&backup).await.unwrap();
            prop_assert!(verification_result.is_valid);
            prop_assert_eq!(verification_result.content_items_verified, content_count);
            prop_assert!(verification_result.corrupted_items.is_empty());
            prop_assert!(verification_result.integrity_score > 0.9);

            // Restore backup
            let restored_items = backup_system.restore_backup(&backup).await.unwrap();
            prop_assert_eq!(restored_items.len(), content_count);

            // Verify content IDs match
            for original_id in &content_ids {
                let found = restored_items.iter().any(|item| item.content_id == *original_id);
                prop_assert!(found);
            }
            Ok(())
        })?;
    }

    /// **Property 14.5: Backup Corruption Detection**
    /// Any corruption to backup data should be detected during verification
    #[test]
    fn prop_backup_corruption_detection(
        backup_type in arb_backup_type(),
        encryption_level in arb_encryption_level(),
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let backup_system = BackupSystem::new().await.unwrap();
            let metadata = HashMap::new();
            let content_ids = vec![Uuid::new_v4()];
            
            // Create backup
            let mut backup = backup_system.create_backup(
                content_ids,
                backup_type,
                encryption_level,
                metadata,
            ).await.unwrap();

            // Corrupt the backup data
            if !backup.encrypted_data.is_empty() {
                backup.encrypted_data[0] ^= 0xFF;

                // Verification should detect corruption
                let verification_result = backup_system.verify_backup_integrity(&backup).await.unwrap();
                prop_assert!(!verification_result.is_valid);
                prop_assert!(verification_result.integrity_score < 1.0);
                prop_assert!(!verification_result.errors.is_empty());
            }
            Ok(())
        })?;
    }

    /// **Property 14.6: Encryption Level Security**
    /// Higher encryption levels should produce different encrypted data
    #[test]
    fn prop_encryption_level_security(
        content_count in 1..3usize,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let backup_system = BackupSystem::new().await.unwrap();
            let metadata = HashMap::new();
            let content_ids: Vec<Uuid> = (0..content_count).map(|_| Uuid::new_v4()).collect();

            // Create backups with different encryption levels
            let standard_backup = backup_system.create_backup(
                content_ids.clone(),
                BackupType::Full,
                EncryptionLevel::Standard,
                metadata.clone(),
            ).await.unwrap();

            let high_backup = backup_system.create_backup(
                content_ids.clone(),
                BackupType::Full,
                EncryptionLevel::High,
                metadata,
            ).await.unwrap();

            // Encrypted data should be different
            prop_assert_ne!(standard_backup.encrypted_data.len(), high_backup.encrypted_data.len());

            // But both should verify successfully
            let standard_verification = backup_system.verify_backup_integrity(&standard_backup).await.unwrap();
            let high_verification = backup_system.verify_backup_integrity(&high_backup).await.unwrap();

            prop_assert!(standard_verification.is_valid);
            prop_assert!(high_verification.is_valid);

            // And restore to same content count
            let standard_restored = backup_system.restore_backup(&standard_backup).await.unwrap();
            let high_restored = backup_system.restore_backup(&high_backup).await.unwrap();

            prop_assert_eq!(standard_restored.len(), content_count);
            prop_assert_eq!(high_restored.len(), content_count);
            Ok(())
        })?;
    }

    /// **Property 14.7: Backup Hash Integrity**
    /// The encryption hash should always match the encrypted data
    #[test]
    fn prop_backup_hash_integrity(
        backup_type in arb_backup_type(),
        encryption_level in arb_encryption_level(),
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let backup_system = BackupSystem::new().await.unwrap();
            let metadata = HashMap::new();
            let content_ids = vec![Uuid::new_v4()];

            // Create backup
            let backup = backup_system.create_backup(
                content_ids,
                backup_type,
                encryption_level,
                metadata,
            ).await.unwrap();

            // Hash should not be empty
            prop_assert!(!backup.encryption_hash.is_empty());

            // Verification should pass (hash matches data)
            let verification_result = backup_system.verify_backup_integrity(&backup).await.unwrap();
            prop_assert!(verification_result.is_valid);

            // If we modify the hash, verification should fail
            let mut corrupted_backup = backup.clone();
            corrupted_backup.encryption_hash = "corrupted_hash".to_string();

            let corrupted_verification = backup_system.verify_backup_integrity(&corrupted_backup).await.unwrap();
            prop_assert!(!corrupted_verification.is_valid);
            prop_assert!(!corrupted_verification.errors.is_empty());
            Ok(())
        })?;
    }

    /// **Property 14.8: End-to-End Security Workflow**
    /// The complete workflow of signing content, creating backups, and verifying
    /// should maintain integrity throughout
    #[test]
    fn prop_end_to_end_security_workflow(
        content in arb_islamic_content(),
        content_type in arb_content_type(),
        authority in arb_authority(),
        backup_type in arb_backup_type(),
        encryption_level in arb_encryption_level(),
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let secret_key = b"test_secret_key_for_property_testing".to_vec();
            let authenticator = DigitalAuthenticator::new(secret_key);
            let backup_system = BackupSystem::new().await.unwrap();
            let metadata = HashMap::new();

            // Step 1: Sign content
            let signature = authenticator.sign_content(
                &content,
                content_type,
                &authority,
                metadata.clone(),
            ).unwrap();

            // Verify signature immediately
            let verification = authenticator.verify_content(&content, &signature).unwrap();
            prop_assert!(verification.is_valid);

            // Step 2: Create encrypted backup of signed content
            let content_ids = vec![signature.content_id];
            let backup = backup_system.create_backup(
                content_ids.clone(),
                backup_type,
                encryption_level,
                metadata,
            ).await.unwrap();

            // Step 3: Verify backup integrity
            let backup_verification = backup_system.verify_backup_integrity(&backup).await.unwrap();
            prop_assert!(backup_verification.is_valid);
            prop_assert_eq!(backup_verification.content_items_verified, 1);

            // Step 4: Restore backup
            let restored_items = backup_system.restore_backup(&backup).await.unwrap();
            prop_assert_eq!(restored_items.len(), 1);

            // Step 5: Verify content ID is preserved
            prop_assert_eq!(restored_items[0].content_id, signature.content_id);

            // Step 6: Re-verify original signature
            let final_verification = authenticator.verify_content(&content, &signature).unwrap();
            prop_assert!(final_verification.is_valid);
            prop_assert_eq!(final_verification.confidence_score, 1.0);
            Ok(())
        })?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_property_generators() {
        // Test that our generators produce valid data
        let content = arb_islamic_content().new_tree(&mut proptest::test_runner::TestRunner::default()).unwrap().current();
        assert!(!content.is_empty());
        
        let content_type = arb_content_type().new_tree(&mut proptest::test_runner::TestRunner::default()).unwrap().current();
        match content_type {
            ContentType::Quran | ContentType::Hadith | ContentType::Tafsir | 
            ContentType::Story | ContentType::Prayer | ContentType::Dhikr => {},
            _ => panic!("Invalid content type generated"),
        }
        
        let authority = arb_authority().new_tree(&mut proptest::test_runner::TestRunner::default()).unwrap().current();
        assert!(["Sanad System", "Islamic Foundation", "Al-Azhar University", "King Fahd Complex"]
            .contains(&authority.as_str()));
    }

    #[tokio::test]
    async fn test_backup_generators() {
        let backup_type = arb_backup_type().new_tree(&mut proptest::test_runner::TestRunner::default()).unwrap().current();
        match backup_type {
            BackupType::Full | BackupType::Incremental | BackupType::Differential => {},
        }
        
        let encryption_level = arb_encryption_level().new_tree(&mut proptest::test_runner::TestRunner::default()).unwrap().current();
        match encryption_level {
            EncryptionLevel::Standard | EncryptionLevel::High | EncryptionLevel::Maximum => {},
        }
    }
}
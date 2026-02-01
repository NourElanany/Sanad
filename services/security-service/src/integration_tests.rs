use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

use crate::backup_system::BackupSystem;
use crate::models::*;

/// Integration tests for the encrypted backup system
/// These tests demonstrate the complete workflow of creating, verifying, and restoring backups

#[tokio::test]
async fn test_complete_backup_workflow() -> Result<()> {
    let backup_system = BackupSystem::new().await?;
    
    // Create test content IDs
    let content_ids = vec![
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
    ];
    
    let mut metadata = HashMap::new();
    metadata.insert("description".to_string(), "Test Islamic content backup".to_string());
    metadata.insert("authority".to_string(), "Sanad System".to_string());
    metadata.insert("content_types".to_string(), "quran,hadith,tafsir".to_string());
    
    // Test 1: Create a full backup with standard encryption
    let backup = backup_system.create_backup(
        content_ids.clone(),
        BackupType::Full,
        EncryptionLevel::Standard,
        metadata.clone(),
    ).await?;
    
    assert_eq!(backup.content_manifest, content_ids);
    assert_eq!(backup.backup_type, BackupType::Full);
    assert_eq!(backup.encryption_level, EncryptionLevel::Standard);
    assert!(!backup.encrypted_data.is_empty());
    assert!(!backup.encryption_hash.is_empty());
    
    // Test 2: Verify backup integrity
    let verification_result = backup_system.verify_backup_integrity(&backup).await?;
    assert!(verification_result.is_valid);
    assert_eq!(verification_result.content_items_verified, 3);
    assert!(verification_result.corrupted_items.is_empty());
    assert!(verification_result.integrity_score > 0.9);
    
    // Test 3: Restore backup content
    let restored_items = backup_system.restore_backup(&backup).await?;
    assert_eq!(restored_items.len(), 3);
    
    for (i, item) in restored_items.iter().enumerate() {
        assert_eq!(item.content_id, content_ids[i]);
        assert_eq!(item.content_type, "quran");
        assert_eq!(item.authority, "Sanad System");
        assert!(!item.content_data.is_empty());
        assert!(!item.content_hash.is_empty());
    }
    
    println!("✅ Complete backup workflow test passed");
    Ok(())
}

#[tokio::test]
async fn test_high_security_backup() -> Result<()> {
    let backup_system = BackupSystem::new().await?;
    
    let content_ids = vec![Uuid::new_v4()];
    let mut metadata = HashMap::new();
    metadata.insert("security_level".to_string(), "high".to_string());
    metadata.insert("content_type".to_string(), "quran".to_string());
    
    // Create backup with high encryption
    let backup = backup_system.create_backup(
        content_ids.clone(),
        BackupType::Full,
        EncryptionLevel::High,
        metadata,
    ).await?;
    
    // Verify the backup
    let verification_result = backup_system.verify_backup_integrity(&backup).await?;
    assert!(verification_result.is_valid);
    assert_eq!(verification_result.content_items_verified, 1);
    
    // Restore and verify content
    let restored_items = backup_system.restore_backup(&backup).await?;
    assert_eq!(restored_items.len(), 1);
    assert_eq!(restored_items[0].content_id, content_ids[0]);
    
    println!("✅ High security backup test passed");
    Ok(())
}

#[tokio::test]
async fn test_maximum_security_backup() -> Result<()> {
    let backup_system = BackupSystem::new().await?;
    
    let content_ids = vec![Uuid::new_v4(), Uuid::new_v4()];
    let mut metadata = HashMap::new();
    metadata.insert("security_level".to_string(), "maximum".to_string());
    metadata.insert("content_type".to_string(), "quran_and_hadith".to_string());
    
    // Create backup with maximum encryption
    let backup = backup_system.create_backup(
        content_ids.clone(),
        BackupType::Full,
        EncryptionLevel::Maximum,
        metadata,
    ).await?;
    
    // Verify the backup
    let verification_result = backup_system.verify_backup_integrity(&backup).await?;
    assert!(verification_result.is_valid);
    assert_eq!(verification_result.content_items_verified, 2);
    assert_eq!(verification_result.integrity_score, 1.0);
    
    // Restore and verify content
    let restored_items = backup_system.restore_backup(&backup).await?;
    assert_eq!(restored_items.len(), 2);
    
    println!("✅ Maximum security backup test passed");
    Ok(())
}

#[tokio::test]
async fn test_incremental_backup() -> Result<()> {
    let backup_system = BackupSystem::new().await?;
    
    let content_ids = vec![Uuid::new_v4()];
    let mut metadata = HashMap::new();
    metadata.insert("backup_strategy".to_string(), "incremental".to_string());
    
    // Create incremental backup
    let backup = backup_system.create_backup(
        content_ids.clone(),
        BackupType::Incremental,
        EncryptionLevel::Standard,
        metadata,
    ).await?;
    
    assert_eq!(backup.backup_type, BackupType::Incremental);
    
    // Verify the backup
    let verification_result = backup_system.verify_backup_integrity(&backup).await?;
    assert!(verification_result.is_valid);
    
    println!("✅ Incremental backup test passed");
    Ok(())
}

#[tokio::test]
async fn test_differential_backup() -> Result<()> {
    let backup_system = BackupSystem::new().await?;
    
    let content_ids = vec![Uuid::new_v4(), Uuid::new_v4()];
    let mut metadata = HashMap::new();
    metadata.insert("backup_strategy".to_string(), "differential".to_string());
    
    // Create differential backup
    let backup = backup_system.create_backup(
        content_ids.clone(),
        BackupType::Differential,
        EncryptionLevel::High,
        metadata,
    ).await?;
    
    assert_eq!(backup.backup_type, BackupType::Differential);
    assert_eq!(backup.encryption_level, EncryptionLevel::High);
    
    // Verify the backup
    let verification_result = backup_system.verify_backup_integrity(&backup).await?;
    assert!(verification_result.is_valid);
    assert_eq!(verification_result.content_items_verified, 2);
    
    println!("✅ Differential backup test passed");
    Ok(())
}

#[tokio::test]
async fn test_backup_corruption_detection() -> Result<()> {
    let backup_system = BackupSystem::new().await?;
    
    let content_ids = vec![Uuid::new_v4()];
    let metadata = HashMap::new();
    
    // Create a valid backup
    let mut backup = backup_system.create_backup(
        content_ids,
        BackupType::Full,
        EncryptionLevel::Standard,
        metadata,
    ).await?;
    
    // Corrupt the backup data
    if !backup.encrypted_data.is_empty() {
        backup.encrypted_data[0] ^= 0xFF; // Flip bits to corrupt data
    }
    
    // Verify the corrupted backup
    let verification_result = backup_system.verify_backup_integrity(&backup).await?;
    assert!(!verification_result.is_valid);
    assert!(!verification_result.errors.is_empty());
    assert!(verification_result.integrity_score < 1.0);
    
    // Should contain error about hash mismatch
    let has_hash_error = verification_result.errors.iter()
        .any(|error| error.contains("hash mismatch") || error.contains("corrupted"));
    assert!(has_hash_error);
    
    println!("✅ Backup corruption detection test passed");
    Ok(())
}

#[tokio::test]
async fn test_backup_encryption_hash_integrity() -> Result<()> {
    let backup_system = BackupSystem::new().await?;
    
    let content_ids = vec![Uuid::new_v4()];
    let metadata = HashMap::new();
    
    // Create backup
    let mut backup = backup_system.create_backup(
        content_ids,
        BackupType::Full,
        EncryptionLevel::Standard,
        metadata,
    ).await?;
    
    let original_hash = backup.encryption_hash.clone();
    
    // Tamper with the encryption hash
    backup.encryption_hash = "tampered_hash".to_string();
    
    // Verify should fail due to hash mismatch
    let verification_result = backup_system.verify_backup_integrity(&backup).await?;
    assert!(!verification_result.is_valid);
    assert!(verification_result.errors.iter().any(|e| e.contains("hash mismatch")));
    
    // Restore original hash - should pass
    backup.encryption_hash = original_hash;
    let verification_result = backup_system.verify_backup_integrity(&backup).await?;
    assert!(verification_result.is_valid);
    
    println!("✅ Backup encryption hash integrity test passed");
    Ok(())
}

#[tokio::test]
async fn test_large_backup_performance() -> Result<()> {
    let backup_system = BackupSystem::new().await?;
    
    // Create a larger backup with multiple content items
    let content_ids: Vec<Uuid> = (0..10).map(|_| Uuid::new_v4()).collect();
    let mut metadata = HashMap::new();
    metadata.insert("test_type".to_string(), "performance".to_string());
    metadata.insert("content_count".to_string(), content_ids.len().to_string());
    
    let start_time = std::time::Instant::now();
    
    // Create backup
    let backup = backup_system.create_backup(
        content_ids.clone(),
        BackupType::Full,
        EncryptionLevel::High,
        metadata,
    ).await?;
    
    let creation_time = start_time.elapsed();
    
    // Verify backup
    let verify_start = std::time::Instant::now();
    let verification_result = backup_system.verify_backup_integrity(&backup).await?;
    let verification_time = verify_start.elapsed();
    
    assert!(verification_result.is_valid);
    assert_eq!(verification_result.content_items_verified, 10);
    
    // Performance assertions (reasonable limits for testing)
    assert!(creation_time.as_millis() < 5000, "Backup creation took too long: {:?}", creation_time);
    assert!(verification_time.as_millis() < 3000, "Backup verification took too long: {:?}", verification_time);
    
    println!("✅ Large backup performance test passed");
    println!("   Creation time: {:?}", creation_time);
    println!("   Verification time: {:?}", verification_time);
    println!("   Backup size: {} bytes", backup.encrypted_data.len());
    
    Ok(())
}

/// Run all integration tests
pub async fn run_all_integration_tests() -> Result<()> {
    println!("🔒 Running encrypted backup system integration tests...\n");
    
    test_complete_backup_workflow().await?;
    test_high_security_backup().await?;
    test_maximum_security_backup().await?;
    test_incremental_backup().await?;
    test_differential_backup().await?;
    test_backup_corruption_detection().await?;
    test_backup_encryption_hash_integrity().await?;
    test_large_backup_performance().await?;
    
    println!("\n🎉 All encrypted backup system integration tests passed!");
    Ok(())
}
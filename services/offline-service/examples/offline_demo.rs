use offline_service::models::*;
use offline_service::service::{OfflineService, OfflineServiceBuilder};
use std::path::PathBuf;
use tempfile::TempDir;
use tokio;
use uuid::Uuid;

/// Demonstration of the offline service functionality for the Islamic application
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::init();

    println!("🕌 Sanad Islamic Application - Offline Mode Demo");
    println!("================================================");

    // Create temporary storage for demo
    let temp_dir = TempDir::new()?;
    let storage_path = temp_dir.path().to_path_buf();

    // Configure offline service for Islamic content
    let mut config = OfflineConfig::default();
    config.max_storage_mb = 100; // 100MB for demo
    config.enable_compression = true;
    config.auto_cleanup = true;

    // Build offline service
    let service = OfflineServiceBuilder::new()
        .storage_path(storage_path)
        .config(config)
        .server_url("http://localhost:8080".to_string())
        .build()
        .await?;

    println!("✅ Offline service initialized");

    // Demo 1: Store and retrieve Quran content
    println!("\n📖 Demo 1: Quran Content Offline Storage");
    println!("-----------------------------------------");

    let quran_verses = vec![
        ("quran:1:1", "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ", "Al-Fatiha 1:1"),
        ("quran:1:2", "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ", "Al-Fatiha 1:2"),
        ("quran:1:3", "الرَّحْمَٰنِ الرَّحِيمِ", "Al-Fatiha 1:3"),
        ("quran:2:255", "اللَّهُ لَا إِلَٰهَ إِلَّا هُوَ الْحَيُّ الْقَيُّومُ", "Ayat al-Kursi"),
    ];

    for (content_id, arabic_text, title) in &quran_verses {
        let id = service.store_content(
            OfflineContentType::QuranText,
            content_id.to_string(),
            arabic_text.as_bytes().to_vec(),
            title.to_string(),
            None,
        ).await?;

        println!("📝 Stored: {} - {}", title, id);
    }

    // Retrieve Quran content
    println!("\n🔍 Retrieving Quran content offline:");
    for (content_id, expected_text, title) in &quran_verses {
        if let Some(result) = service.get_content(content_id).await? {
            if let Some(data) = result.data {
                let retrieved_text = String::from_utf8(data)?;
                println!("✅ {}: {} (from cache: {})", title, retrieved_text, result.from_cache);
                assert_eq!(&retrieved_text, expected_text);
            }
        }
    }

    // Demo 2: User bookmarks and reading progress
    println!("\n👤 Demo 2: User Data Management");
    println!("-------------------------------");

    let user_id = Uuid::new_v4();
    println!("User ID: {}", user_id);

    // Store user bookmarks
    let bookmarks = serde_json::json!({
        "bookmarks": [
            {"surah": 1, "ayah": 1, "note": "Opening verse", "date": "2024-01-01"},
            {"surah": 2, "ayah": 255, "note": "Ayat al-Kursi", "date": "2024-01-02"},
            {"surah": 112, "ayah": 1, "note": "Al-Ikhlas", "date": "2024-01-03"}
        ]
    });

    let bookmark_id = service.store_user_bookmarks(user_id, bookmarks.to_string()).await?;
    println!("📚 Stored user bookmarks: {}", bookmark_id);

    // Store reading progress
    let progress = serde_json::json!({
        "current_surah": 2,
        "current_ayah": 100,
        "completion_percentage": 25.5,
        "total_verses_read": 500,
        "last_read_date": "2024-01-15",
        "reading_streak_days": 7
    });

    let progress_id = service.store_reading_progress(user_id, progress.to_string()).await?;
    println!("📊 Stored reading progress: {}", progress_id);

    // Retrieve user data
    if let Some(result) = service.get_user_bookmarks(user_id).await? {
        println!("✅ Retrieved bookmarks: {} characters (from cache: {})", 
                 result.data.as_ref().unwrap().len(), result.from_cache);
    }

    if let Some(result) = service.get_reading_progress(user_id).await? {
        println!("✅ Retrieved progress: {} characters (from cache: {})", 
                 result.data.as_ref().unwrap().len(), result.from_cache);
    }

    // Demo 3: Prayer times with expiry
    println!("\n🕐 Demo 3: Prayer Times with Expiry");
    println!("-----------------------------------");

    let prayer_times = serde_json::json!({
        "location": "New York City",
        "date": "2024-01-15",
        "times": {
            "fajr": "05:30",
            "sunrise": "07:15",
            "dhuhr": "12:15",
            "asr": "15:30",
            "maghrib": "18:00",
            "isha": "19:30"
        },
        "qibla_direction": 58.5
    });

    let prayer_id = service.store_content(
        OfflineContentType::PrayerTimes,
        "prayer_times:40.7128:-74.0060:2024-01-15".to_string(),
        prayer_times.to_string().into_bytes(),
        "NYC Prayer Times".to_string(),
        None,
    ).await?;

    println!("🕐 Stored prayer times: {}", prayer_id);

    // Retrieve prayer times using specialized method
    if let Some(result) = service.get_prayer_times(40.7128, -74.0060, "2024-01-15").await? {
        println!("✅ Retrieved prayer times (from cache: {})", result.from_cache);
        if let Some(data) = result.data {
            let times: serde_json::Value = serde_json::from_str(&data)?;
            println!("   Fajr: {}", times["times"]["fajr"]);
            println!("   Dhuhr: {}", times["times"]["dhuhr"]);
            println!("   Maghrib: {}", times["times"]["maghrib"]);
        }
    }

    // Demo 4: Storage statistics and management
    println!("\n📊 Demo 4: Storage Statistics");
    println!("-----------------------------");

    let stats = service.get_statistics().await;
    println!("Total items: {}", stats.total_items);
    println!("Total size: {:.2} MB", stats.total_size_mb);
    println!("Available space: {:.2} MB", stats.available_space_mb);
    println!("Used space: {:.1}%", stats.used_space_percentage);

    println!("\nItems by type:");
    for (content_type, count) in &stats.items_by_type {
        println!("  {:?}: {}", content_type, count);
    }

    println!("\nItems by priority:");
    for (priority, count) in &stats.items_by_priority {
        println!("  {:?}: {}", priority, count);
    }

    if stats.compression_stats.average_compression_ratio > 1.0 {
        println!("\nCompression statistics:");
        println!("  Average ratio: {:.2}x", stats.compression_stats.average_compression_ratio);
        println!("  Space saved: {:.2} MB ({:.1}%)", 
                 stats.compression_stats.space_saved_mb,
                 stats.compression_stats.space_saved_percentage);
    }

    // Demo 5: Content listing and filtering
    println!("\n📋 Demo 5: Content Management");
    println!("-----------------------------");

    let all_content = service.list_content(None).await;
    println!("All content items: {}", all_content.len());

    let quran_content = service.list_content(Some(OfflineContentType::QuranText)).await;
    println!("Quran content items: {}", quran_content.len());

    let user_content = service.list_content(Some(OfflineContentType::UserBookmarks)).await;
    println!("User bookmark items: {}", user_content.len());

    // Demo 6: Content integrity verification
    println!("\n🔒 Demo 6: Content Integrity");
    println!("----------------------------");

    let corrupted = service.verify_integrity().await?;
    if corrupted.is_empty() {
        println!("✅ All content integrity checks passed");
    } else {
        println!("⚠️  Found {} corrupted items:", corrupted.len());
        for item in &corrupted {
            println!("   - {}", item);
        }
    }

    // Demo 7: Storage optimization
    println!("\n⚡ Demo 7: Storage Optimization");
    println!("-------------------------------");

    let space_saved = service.optimize_storage().await?;
    if space_saved > 0.0 {
        println!("✅ Optimization saved {:.2} MB", space_saved);
    } else {
        println!("ℹ️  No optimization needed");
    }

    // Final statistics
    println!("\n📈 Final Statistics");
    println!("------------------");
    let final_stats = service.get_statistics().await;
    println!("Total items stored: {}", final_stats.total_items);
    println!("Total storage used: {:.2} MB", final_stats.total_size_mb);
    println!("Compression ratio: {:.2}x", final_stats.compression_stats.average_compression_ratio);

    println!("\n🎉 Offline mode demo completed successfully!");
    println!("The Islamic application can now work without internet connection");
    println!("while maintaining all essential content and user data.");

    Ok(())
}
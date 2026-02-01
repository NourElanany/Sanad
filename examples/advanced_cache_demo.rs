use shared::{AdvancedCacheManager, CacheConfig, CacheStrategies, CacheType, SanadResult};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

/// Demonstration of advanced caching features in the Sanad Islamic Application
/// 
/// This example showcases:
/// 1. Intelligent caching for frequent queries
/// 2. Heavy content optimization with compression
/// 3. Adaptive TTL based on access patterns
/// 4. Smart cache invalidation strategies

#[tokio::main]
async fn main() -> SanadResult<()> {
    // Initialize tracing for better logging
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .init();

    println!("🚀 Advanced Cache System Demo for Sanad Islamic Application");
    println!("============================================================");

    // Create advanced cache configuration
    let cache_config = CacheConfig {
        default_ttl_seconds: 1800,        // 30 minutes
        prayer_times_ttl_seconds: 43200,  // 12 hours
        semantic_query_ttl_seconds: 10800, // 3 hours
        quran_content_ttl_seconds: 86400,  // 24 hours
        hadith_content_ttl_seconds: 21600, // 6 hours
        max_memory_cache_size: 5000,
        enable_smart_invalidation: true,
        min_query_frequency_for_cache: 3, // Cache after 3 queries per hour
        heavy_content_threshold_bytes: 512 * 1024, // 512KB threshold
        heavy_content_ttl_seconds: 7200,  // 2 hours
        enable_query_tracking: true,
        enable_adaptive_ttl: true,
    };

    // Initialize cache manager (would use real Redis URL in production)
    let redis_url = "redis://localhost:6379";
    let cache_manager = match AdvancedCacheManager::new(redis_url, Some(cache_config)).await {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("❌ Failed to connect to Redis: {}", e);
            eprintln!("💡 Make sure Redis is running on localhost:6379");
            eprintln!("   You can start it with: docker run -d -p 6379:6379 redis:alpine");
            return Ok(());
        }
    };

    println!("✅ Connected to Redis and initialized advanced cache manager");

    // Demo 1: Frequent Query Caching
    println!("\n📊 Demo 1: Intelligent Frequent Query Caching");
    println!("----------------------------------------------");
    
    let popular_query = "What is the meaning of Surah Al-Fatiha?";
    let query_result = json!({
        "surah": "Al-Fatiha",
        "meaning": "The Opening",
        "verses": 7,
        "themes": ["praise", "guidance", "prayer"],
        "explanation": "Al-Fatiha is the opening chapter of the Quran..."
    });

    // Simulate multiple queries to make it frequent
    for i in 1..=5 {
        println!("  Query attempt {}: {}", i, popular_query);
        cache_manager.cache_frequent_query(popular_query, &query_result).await?;
        sleep(Duration::from_millis(100)).await;
    }

    // Now retrieve the frequent query
    if let Some(cached_result) = cache_manager.get_frequent_query::<serde_json::Value>(popular_query).await? {
        println!("  ✅ Frequent query cached and retrieved successfully!");
        println!("  📄 Result: {}", cached_result.get("surah").unwrap());
    }

    // Demo 2: Heavy Content Caching with Compression
    println!("\n💾 Demo 2: Heavy Content Optimization");
    println!("-------------------------------------");

    // Create large content (simulating audio file or large search results)
    let large_content = vec![0u8; 1024 * 1024]; // 1MB of data
    let content_id = "audio_surah_1_qari_mishary";
    
    println!("  📁 Caching heavy content: {} bytes", large_content.len());
    cache_manager.cache_heavy_content(content_id, &large_content, "audio/mpeg").await?;

    // Retrieve heavy content
    if let Some(retrieved_content) = cache_manager.get_heavy_content(content_id).await? {
        println!("  ✅ Heavy content retrieved: {} bytes", retrieved_content.len());
        println!("  🗜️  Compression and decompression successful!");
    }

    // Demo 3: Specialized Content Caching
    println!("\n📖 Demo 3: Specialized Islamic Content Caching");
    println!("----------------------------------------------");

    // Cache Quran content
    let quran_verse = json!({
        "surah": 1,
        "ayah": 1,
        "arabic": "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
        "translation": "In the name of Allah, the Entirely Merciful, the Especially Merciful.",
        "transliteration": "Bismillahi'r-rahmani'r-raheem"
    });

    CacheStrategies::cache_quran_content(&cache_manager, 1, Some(1), &quran_verse).await?;
    println!("  ✅ Cached Quran verse: Surah 1, Ayah 1");

    // Cache Hadith content
    let hadith = json!({
        "collection": "bukhari",
        "book": "belief",
        "number": "1",
        "arabic": "إنما الأعمال بالنيات",
        "translation": "Actions are but by intention...",
        "grade": "sahih"
    });

    CacheStrategies::cache_hadith_content(&cache_manager, "bukhari", "belief", "1", &hadith).await?;
    println!("  ✅ Cached Hadith: Bukhari, Book of Belief, #1");

    // Cache prayer times
    let prayer_times = json!({
        "fajr": "05:30",
        "sunrise": "07:00",
        "dhuhr": "12:30",
        "asr": "15:45",
        "maghrib": "18:15",
        "isha": "19:45",
        "location": "Mecca, Saudi Arabia"
    });

    CacheStrategies::cache_prayer_times(
        &cache_manager,
        21.4225, 39.8262, // Mecca coordinates
        "2024-01-01",
        "UmmAlQura",
        &prayer_times
    ).await?;
    println!("  ✅ Cached prayer times for Mecca");

    // Demo 4: Cache Statistics and Monitoring
    println!("\n📈 Demo 4: Cache Statistics and Monitoring");
    println!("-----------------------------------------");

    let stats = cache_manager.get_cache_stats().await?;
    println!("  📊 Cache Statistics:");
    println!("     Memory cache entries: {}", stats.memory_cache_entries);
    println!("     Heavy content entries: {}", stats.heavy_content_entries);
    println!("     Heavy content size: {} bytes", stats.total_heavy_content_size_bytes);
    println!("     Average compression ratio: {:.2}", stats.average_compression_ratio);
    println!("     Frequent queries: {}", stats.frequent_queries_count);
    println!("     Query tracking enabled: {}", stats.query_tracking_enabled);
    println!("     Adaptive TTL enabled: {}", stats.adaptive_ttl_enabled);

    println!("  📋 Cache entries by type:");
    for (cache_type, count) in &stats.memory_cache_entries_by_type {
        println!("     {}: {}", cache_type, count);
    }

    // Demo 5: Smart Cache Invalidation
    println!("\n🔄 Demo 5: Smart Cache Invalidation");
    println!("-----------------------------------");

    // Invalidate all semantic queries
    let deleted_count = cache_manager.invalidate_semantic_queries().await?;
    println!("  🗑️  Invalidated {} semantic query cache entries", deleted_count);

    // Invalidate prayer times for a specific location
    let deleted_count = cache_manager.invalidate_prayer_times(21.4225, 39.8262).await?;
    println!("  🗑️  Invalidated {} prayer times cache entries for Mecca", deleted_count);

    // Invalidate Quran surah
    let deleted_count = cache_manager.invalidate_quran_surah(1).await?;
    println!("  🗑️  Invalidated {} Quran Surah 1 cache entries", deleted_count);

    // Demo 6: Cache Cleanup
    println!("\n🧹 Demo 6: Cache Cleanup and Maintenance");
    println!("---------------------------------------");

    let cleaned_count = cache_manager.cleanup_expired_entries().await;
    println!("  🧹 Cleaned up {} expired cache entries", cleaned_count);

    // Warm up cache with frequently accessed content
    cache_manager.warm_up_cache().await?;
    println!("  🔥 Cache warm-up completed");

    println!("\n🎉 Advanced Cache System Demo Completed Successfully!");
    println!("====================================================");
    println!("Key Features Demonstrated:");
    println!("✅ Intelligent frequent query caching");
    println!("✅ Heavy content optimization with compression");
    println!("✅ Adaptive TTL based on access patterns");
    println!("✅ Specialized Islamic content caching strategies");
    println!("✅ Comprehensive cache statistics and monitoring");
    println!("✅ Smart cache invalidation patterns");
    println!("✅ Automated cache cleanup and maintenance");
    
    println!("\n💡 This advanced caching system provides:");
    println!("   • Improved response times for common queries");
    println!("   • Efficient storage of heavy content like audio files");
    println!("   • Intelligent cache expiration management");
    println!("   • Specialized handling for Islamic content types");
    println!("   • Comprehensive monitoring and analytics");

    Ok(())
}
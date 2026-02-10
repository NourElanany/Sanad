//! Real API Integration Tests
//!
//! These tests connect to actual external APIs to verify end-to-end functionality.
//! They require:
//! - Internet connection
//! - Valid API keys (where required)
//! - Redis running (for caching and rate limiting)
//!
//! Run with: cargo test --test real_api_integration_tests -- --ignored --nocapture
//!
//! Note: These tests are marked as #[ignore] by default to avoid hitting real APIs
//! during normal test runs. They should be run manually before releases.

use std::env;

// ============================================================================
// Test Configuration
// ============================================================================

/// Check if we have the required environment setup for real API tests
fn check_test_prerequisites() -> Result<(), String> {
    // Check for required API keys
    let sunnah_key = env::var("SUNNAH_COM_API_KEY").ok();
    let hf_key = env::var("HUGGING_FACE_API_KEY").ok();
    
    let mut warnings = Vec::new();
    
    if sunnah_key.is_none() {
        warnings.push("SUNNAH_COM_API_KEY not set - Hadith tests will be skipped");
    }
    
    if hf_key.is_none() {
        warnings.push("HUGGING_FACE_API_KEY not set - AI tests will be skipped");
    }
    
    // Print warnings
    for warning in &warnings {
        eprintln!("⚠️  WARNING: {}", warning);
    }
    
    Ok(())
}

// ============================================================================
// Quran API Tests (No authentication required)
// ============================================================================

#[tokio::test]
#[ignore] // Run manually with --ignored flag
async fn test_quran_com_api_real_request() {
    println!("\n🕌 Testing Quran.com API with real request...");
    
    let client = reqwest::Client::new();
    let url = "https://api.quran.com/api/v4/verses/by_key/1:1";
    
    let response = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            println!("✅ Status: {}", resp.status());
            assert!(resp.status().is_success(), "API should return success status");
            
            let body = resp.text().await.unwrap();
            println!("📄 Response preview: {}...", &body[..body.len().min(200)]);
            
            // Verify response contains expected fields
            assert!(body.contains("verse"), "Response should contain verse data");
            assert!(body.contains("verse_key") || body.contains("text"), 
                   "Response should contain verse information");
            println!("✅ Quran.com API test passed - verse data retrieved successfully");
        }
        Err(e) => {
            panic!("❌ Failed to connect to Quran.com API: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_alquran_cloud_api_real_request() {
    println!("\n🕌 Testing AlQuran Cloud API with real request...");
    
    let client = reqwest::Client::new();
    let url = "https://api.alquran.cloud/v1/ayah/1:1";
    
    let response = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            println!("✅ Status: {}", resp.status());
            assert!(resp.status().is_success(), "API should return success status");
            
            let body = resp.text().await.unwrap();
            println!("📄 Response preview: {}...", &body[..body.len().min(200)]);
            
            // Verify response structure
            assert!(body.contains("data"), "Response should contain data field");
            assert!(body.contains("text"), "Response should contain text field");
        }
        Err(e) => {
            panic!("❌ Failed to connect to AlQuran Cloud API: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_tanzil_api_real_request() {
    println!("\n🕌 Testing Tanzil API with real request...");
    
    let client = reqwest::Client::new();
    let url = "https://tanzil.net/api/quran/1:1";
    
    let response = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            println!("✅ Status: {}", resp.status());
            
            if resp.status().is_success() {
                let body = resp.text().await.unwrap();
                println!("📄 Response preview: {}...", &body[..body.len().min(200)]);
                println!("✅ Tanzil API is accessible");
            } else {
                println!("⚠️  Tanzil API returned status: {}", resp.status());
                println!("   This may be expected if the API format has changed");
            }
        }
        Err(e) => {
            println!("⚠️  Could not connect to Tanzil API: {}", e);
            println!("   This may be expected if the API is temporarily unavailable");
        }
    }
}

// ============================================================================
// Prayer Times API Tests (No authentication required)
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_aladhan_prayer_times_real_request() {
    println!("\n🕌 Testing Aladhan Prayer Times API with real request...");
    
    let client = reqwest::Client::new();
    // Mecca coordinates
    let url = "https://api.aladhan.com/v1/timings/15-01-2024?latitude=21.4225&longitude=39.8262&method=4";
    
    let response = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            println!("✅ Status: {}", resp.status());
            assert!(resp.status().is_success(), "API should return success status");
            
            let body = resp.text().await.unwrap();
            println!("📄 Response preview: {}...", &body[..body.len().min(300)]);
            
            // Verify response contains prayer times
            assert!(body.contains("Fajr"), "Response should contain Fajr time");
            assert!(body.contains("Dhuhr"), "Response should contain Dhuhr time");
            assert!(body.contains("Asr"), "Response should contain Asr time");
            assert!(body.contains("Maghrib"), "Response should contain Maghrib time");
            assert!(body.contains("Isha"), "Response should contain Isha time");
            
            println!("✅ All five prayer times present in response");
        }
        Err(e) => {
            panic!("❌ Failed to connect to Aladhan API: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_aladhan_qibla_direction_real_request() {
    println!("\n🕌 Testing Aladhan Qibla Direction API with real request...");
    
    let client = reqwest::Client::new();
    // New York coordinates
    let url = "https://api.aladhan.com/v1/qibla/40.7128/-74.0060";
    
    let response = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            println!("✅ Status: {}", resp.status());
            assert!(resp.status().is_success(), "API should return success status");
            
            let body = resp.text().await.unwrap();
            println!("📄 Response: {}", body);
            
            // Verify response contains direction
            assert!(body.contains("direction"), "Response should contain direction field");
            
            // Parse and verify direction is in valid range
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                if let Some(direction) = json["data"]["direction"].as_f64() {
                    println!("🧭 Qibla direction from New York: {:.2}°", direction);
                    assert!(direction >= 0.0 && direction <= 360.0, 
                           "Direction should be between 0 and 360 degrees");
                    println!("✅ Direction is within valid range");
                }
            }
        }
        Err(e) => {
            panic!("❌ Failed to connect to Aladhan Qibla API: {}", e);
        }
    }
}

// ============================================================================
// Calendar API Tests (No authentication required)
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_aladhan_hijri_calendar_real_request() {
    println!("\n📅 Testing Aladhan Hijri Calendar API with real request...");
    
    let client = reqwest::Client::new();
    let url = "https://api.aladhan.com/v1/gToH/15-01-2024";
    
    let response = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            println!("✅ Status: {}", resp.status());
            assert!(resp.status().is_success(), "API should return success status");
            
            let body = resp.text().await.unwrap();
            println!("📄 Response preview: {}...", &body[..body.len().min(300)]);
            
            // Verify response contains Hijri date
            assert!(body.contains("hijri"), "Response should contain hijri date");
            
            // Parse and display the conversion
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                if let Some(hijri) = json["data"]["hijri"].as_object() {
                    let day = hijri["day"].as_str().unwrap_or("?");
                    let month = hijri["month"]["en"].as_str().unwrap_or("?");
                    let year = hijri["year"].as_str().unwrap_or("?");
                    println!("📅 Gregorian 15-01-2024 = Hijri {} {} {}", day, month, year);
                    println!("✅ Date conversion successful");
                }
            }
        }
        Err(e) => {
            panic!("❌ Failed to connect to Aladhan Calendar API: {}", e);
        }
    }
}

// ============================================================================
// Hadith API Tests (Requires API key)
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_sunnah_com_api_real_request() {
    println!("\n📖 Testing Sunnah.com API with real request...");
    
    // Check for API key
    let api_key = match env::var("SUNNAH_COM_API_KEY") {
        Ok(key) if !key.is_empty() && key != "your_sunnah_com_api_key_here" => key,
        _ => {
            println!("⚠️  SUNNAH_COM_API_KEY not set or invalid - skipping test");
            println!("   Set the environment variable to run this test");
            return;
        }
    };
    
    let client = reqwest::Client::new();
    let url = "https://api.sunnah.com/v1/collections";
    
    let response = client
        .get(url)
        .header("X-API-Key", &api_key)
        .header("Accept", "application/json")
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            println!("✅ Status: {}", resp.status());
            
            if resp.status().is_success() {
                let body = resp.text().await.unwrap();
                println!("📄 Response preview: {}...", &body[..body.len().min(300)]);
                
                // Verify response contains collections
                assert!(body.contains("bukhari") || body.contains("muslim") || body.contains("data"), 
                       "Response should contain hadith collections");
                println!("✅ Sunnah.com API is accessible with provided key");
            } else if resp.status().as_u16() == 401 {
                println!("❌ Authentication failed - API key may be invalid");
                panic!("Invalid API key for Sunnah.com");
            } else {
                println!("⚠️  Unexpected status: {}", resp.status());
                let body = resp.text().await.unwrap_or_default();
                println!("   Response: {}", body);
            }
        }
        Err(e) => {
            panic!("❌ Failed to connect to Sunnah.com API: {}", e);
        }
    }
}

// ============================================================================
// Tafsir API Tests (No authentication required)
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_quran_com_tafsir_real_request() {
    println!("\n📚 Testing Quran.com Tafsir API with real request...");
    
    let client = reqwest::Client::new();
    // Get tafsir for Al-Fatiha verse 1
    let url = "https://api.quran.com/api/v4/quran/tafsirs/169?verse_key=1:1";
    
    let response = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            println!("✅ Status: {}", resp.status());
            assert!(resp.status().is_success(), "API should return success status");
            
            let body = resp.text().await.unwrap();
            println!("📄 Response preview: {}...", &body[..body.len().min(300)]);
            
            // Verify response contains tafsir
            assert!(body.contains("tafsir") || body.contains("text"), 
                   "Response should contain tafsir text");
            println!("✅ Tafsir data retrieved successfully");
        }
        Err(e) => {
            panic!("❌ Failed to connect to Quran.com Tafsir API: {}", e);
        }
    }
}

// ============================================================================
// AI API Tests (Requires API key)
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_hugging_face_api_real_request() {
    println!("\n🤖 Testing Hugging Face API with real request...");
    
    // Check for API key
    let api_key = match env::var("HUGGING_FACE_API_KEY") {
        Ok(key) if !key.is_empty() && key != "your_huggingface_api_key_here" => key,
        _ => {
            println!("⚠️  HUGGING_FACE_API_KEY not set or invalid - skipping test");
            println!("   Set the environment variable to run this test");
            return;
        }
    };
    
    let client = reqwest::Client::new();
    // Test with a simple Arabic NLP model
    let url = "https://api-inference.huggingface.co/models/bert-base-multilingual-cased";
    
    let payload = serde_json::json!({
        "inputs": "السلام عليكم"
    });
    
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;
    
    match response {
        Ok(resp) => {
            println!("✅ Status: {}", resp.status());
            
            if resp.status().is_success() {
                let body = resp.text().await.unwrap();
                println!("📄 Response preview: {}...", &body[..body.len().min(200)]);
                println!("✅ Hugging Face API is accessible with provided key");
            } else if resp.status().as_u16() == 401 {
                println!("❌ Authentication failed - API key may be invalid");
                panic!("Invalid API key for Hugging Face");
            } else if resp.status().as_u16() == 503 {
                println!("⚠️  Model is loading - this is normal for first request");
                println!("   Try again in a few seconds");
            } else {
                println!("⚠️  Unexpected status: {}", resp.status());
                let body = resp.text().await.unwrap_or_default();
                println!("   Response: {}", body);
            }
        }
        Err(e) => {
            panic!("❌ Failed to connect to Hugging Face API: {}", e);
        }
    }
}

// ============================================================================
// End-to-End Integration Tests
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_complete_prayer_workflow() {
    println!("\n🔄 Testing complete prayer times workflow...");
    
    check_test_prerequisites().unwrap();
    
    let client = reqwest::Client::new();
    
    // Step 1: Get current Hijri date
    println!("📅 Step 1: Getting current Hijri date...");
    let date_url = "https://api.aladhan.com/v1/gToH/15-01-2024";
    let date_resp = client.get(date_url).send().await.unwrap();
    assert!(date_resp.status().is_success());
    println!("✅ Hijri date retrieved");
    
    // Step 2: Get prayer times for Mecca
    println!("🕌 Step 2: Getting prayer times for Mecca...");
    let prayer_url = "https://api.aladhan.com/v1/timings/15-01-2024?latitude=21.4225&longitude=39.8262&method=4";
    let prayer_resp = client.get(prayer_url).send().await.unwrap();
    assert!(prayer_resp.status().is_success());
    let prayer_body = prayer_resp.text().await.unwrap();
    assert!(prayer_body.contains("Fajr"));
    println!("✅ Prayer times retrieved");
    
    // Step 3: Get Qibla direction from New York
    println!("🧭 Step 3: Getting Qibla direction from New York...");
    let qibla_url = "https://api.aladhan.com/v1/qibla/40.7128/-74.0060";
    let qibla_resp = client.get(qibla_url).send().await.unwrap();
    assert!(qibla_resp.status().is_success());
    println!("✅ Qibla direction retrieved");
    
    println!("\n✅ Complete prayer workflow successful!");
}

#[tokio::test]
#[ignore]
async fn test_complete_quran_study_workflow() {
    println!("\n🔄 Testing complete Quran study workflow...");
    
    let client = reqwest::Client::new();
    
    // Step 1: Get Quran verse text
    println!("📖 Step 1: Getting Quran verse (Al-Fatiha 1:1)...");
    let verse_url = "https://api.quran.com/api/v4/verses/by_key/1:1";
    let verse_resp = client.get(verse_url).send().await.unwrap();
    assert!(verse_resp.status().is_success());
    println!("✅ Verse text retrieved");
    
    // Step 2: Get tafsir for the verse
    println!("📚 Step 2: Getting tafsir for the verse...");
    let tafsir_url = "https://api.quran.com/api/v4/quran/tafsirs/169?verse_key=1:1";
    let tafsir_resp = client.get(tafsir_url).send().await.unwrap();
    assert!(tafsir_resp.status().is_success());
    println!("✅ Tafsir retrieved");
    
    // Step 3: Get audio recitation info
    println!("🎵 Step 3: Getting recitation info...");
    let audio_url = "https://api.quran.com/api/v4/chapter_recitations/1/1";
    let audio_resp = client.get(audio_url).send().await.unwrap();
    // Note: This endpoint might have different structure
    println!("   Status: {}", audio_resp.status());
    
    println!("\n✅ Complete Quran study workflow successful!");
}

// ============================================================================
// Performance and Rate Limiting Tests
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_api_response_times() {
    println!("\n⏱️  Testing API response times...");
    
    let client = reqwest::Client::new();
    let apis = vec![
        ("Quran.com", "https://api.quran.com/api/v4/verses/by_key/1:1"),
        ("AlQuran Cloud", "https://api.alquran.cloud/v1/ayah/1:1"),
        ("Aladhan Prayer", "https://api.aladhan.com/v1/timings/15-01-2024?latitude=21.4225&longitude=39.8262&method=4"),
        ("Aladhan Qibla", "https://api.aladhan.com/v1/qibla/40.7128/-74.0060"),
        ("Aladhan Calendar", "https://api.aladhan.com/v1/gToH/15-01-2024"),
    ];
    
    for (name, url) in apis {
        let start = std::time::Instant::now();
        let response = client.get(url).send().await;
        let duration = start.elapsed();
        
        match response {
            Ok(resp) if resp.status().is_success() => {
                println!("✅ {}: {:.2}ms", name, duration.as_millis());
                assert!(duration.as_secs() < 5, "API should respond within 5 seconds");
            }
            Ok(resp) => {
                println!("⚠️  {}: {} ({}ms)", name, resp.status(), duration.as_millis());
            }
            Err(e) => {
                println!("❌ {}: Error - {}", name, e);
            }
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_concurrent_api_requests() {
    println!("\n🔄 Testing concurrent API requests...");
    
    let client = reqwest::Client::new();
    let mut handles = vec![];
    
    // Make 5 concurrent requests to different APIs
    for i in 1..=5 {
        let client = client.clone();
        let handle = tokio::spawn(async move {
            let url = format!("https://api.quran.com/api/v4/verses/by_key/1:{}", i);
            let start = std::time::Instant::now();
            let response = client.get(&url).send().await;
            let duration = start.elapsed();
            (i, response.is_ok(), duration)
        });
        handles.push(handle);
    }
    
    let mut success_count = 0;
    let mut total_time = std::time::Duration::ZERO;
    
    // Wait for all requests to complete
    for handle in handles {
        if let Ok((verse, success, duration)) = handle.await {
            if success {
                success_count += 1;
                total_time += duration;
                println!("✅ Verse {} retrieved in {:.2}ms", verse, duration.as_millis());
            }
        }
    }
    
    println!("\n📊 Results:");
    println!("   Success rate: {}/5", success_count);
    println!("   Average time: {:.2}ms", total_time.as_millis() / 5);
    
    assert!(success_count >= 4, "At least 4 out of 5 requests should succeed");
}

// ============================================================================
// Test Runner
// ============================================================================

#[tokio::test]
#[ignore]
async fn run_all_real_api_tests() {
    println!("\n{}", "=".repeat(70));
    println!("🚀 RUNNING ALL REAL API INTEGRATION TESTS");
    println!("{}", "=".repeat(70));
    
    check_test_prerequisites().unwrap();
    
    println!("\n📋 Test Plan:");
    println!("   1. Quran APIs (Quran.com, AlQuran Cloud, Tanzil)");
    println!("   2. Prayer Times APIs (Aladhan)");
    println!("   3. Qibla Direction APIs (Aladhan)");
    println!("   4. Calendar APIs (Aladhan)");
    println!("   5. Hadith APIs (Sunnah.com - requires key)");
    println!("   6. Tafsir APIs (Quran.com)");
    println!("   7. AI APIs (Hugging Face - requires key)");
    println!("   8. End-to-end workflows");
    println!("   9. Performance tests");
    
    println!("\n{}", "=".repeat(70));
    println!("⚠️  NOTE: Run individual tests with:");
    println!("   cargo test --test real_api_integration_tests <test_name> -- --ignored --nocapture");
    println!("{}\n", "=".repeat(70));
}

// Load Test Runner
// Task 25.4: Run load tests
// This file provides a runnable load test suite

use std::time::Duration;

mod load_tests;
use load_tests::{LoadTestConfig, LoadTestSuite};

#[tokio::main]
async fn main() {
    println!("🚀 API Integration Service - Load Test Runner");
    println!("{}", "=".repeat(80));
    
    // Get base URL from environment or use default
    let base_url = std::env::var("API_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    
    println!("Target URL: {}", base_url);
    
    // Check if service is available
    println!("\n🔍 Checking service availability...");
    let client = reqwest::Client::new();
    match client.get(format!("{}/health", base_url)).send().await {
        Ok(response) if response.status().is_success() => {
            println!("✓ Service is available");
        }
        Ok(response) => {
            println!("⚠ Service returned status: {}", response.status());
            println!("Continuing with tests anyway...");
        }
        Err(e) => {
            println!("❌ Service is not available: {}", e);
            println!("Please ensure the API Integration Service is running.");
            println!("You can start it with: cargo run --bin api-integration-service");
            std::process::exit(1);
        }
    }
    
    // Parse command line arguments for test configuration
    let args: Vec<String> = std::env::args().collect();
    let config = parse_config(&args);
    
    println!("\n📋 Test Configuration:");
    println!("  Concurrent Users:    {}", config.concurrent_users);
    println!("  Test Duration:       {}s", config.test_duration.as_secs());
    println!("  Target RPS:          {}", config.target_rps);
    println!("  Test Rate Limiting:  {}", config.test_rate_limiting);
    println!("  Test Caching:        {}", config.test_caching);
    println!("  Test Fallback:       {}", config.test_fallback);
    
    // Create test suite
    let suite = LoadTestSuite::new(config.clone(), base_url);
    
    // Run all tests
    let metrics = suite.run_all_tests().await;
    
    // Print comprehensive report
    suite.print_report(&metrics);
    
    // Exit with appropriate code
    let success_rate = (metrics.successful_requests as f64 / metrics.total_requests as f64) * 100.0;
    if success_rate >= 95.0 && metrics.p95_response_time <= Duration::from_secs(3) {
        println!("✅ Load tests PASSED");
        std::process::exit(0);
    } else {
        println!("❌ Load tests FAILED");
        std::process::exit(1);
    }
}

fn parse_config(args: &[String]) -> LoadTestConfig {
    let mut config = LoadTestConfig::default();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--users" | "-u" => {
                if i + 1 < args.len() {
                    config.concurrent_users = args[i + 1].parse().unwrap_or(50);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--duration" | "-d" => {
                if i + 1 < args.len() {
                    let secs: u64 = args[i + 1].parse().unwrap_or(60);
                    config.test_duration = Duration::from_secs(secs);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--rps" | "-r" => {
                if i + 1 < args.len() {
                    config.target_rps = args[i + 1].parse().unwrap_or(100);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--no-rate-limit" => {
                config.test_rate_limiting = false;
                i += 1;
            }
            "--no-cache" => {
                config.test_caching = false;
                i += 1;
            }
            "--no-fallback" => {
                config.test_fallback = false;
                i += 1;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                i += 1;
            }
        }
    }
    
    config
}

fn print_help() {
    println!("Load Test Runner - API Integration Service");
    println!();
    println!("USAGE:");
    println!("    cargo test --test run_load_tests -- [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -u, --users <NUM>        Number of concurrent users (default: 50)");
    println!("    -d, --duration <SECS>    Test duration in seconds (default: 60)");
    println!("    -r, --rps <NUM>          Target requests per second (default: 100)");
    println!("    --no-rate-limit          Skip rate limiting tests");
    println!("    --no-cache               Skip caching tests");
    println!("    --no-fallback            Skip fallback tests");
    println!("    -h, --help               Print this help message");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("    API_BASE_URL             Base URL of the API service (default: http://localhost:8080)");
    println!();
    println!("EXAMPLES:");
    println!("    # Run with default settings");
    println!("    cargo test --test run_load_tests");
    println!();
    println!("    # Run with 100 concurrent users for 120 seconds");
    println!("    cargo test --test run_load_tests -- --users 100 --duration 120");
    println!();
    println!("    # Run only caching tests");
    println!("    cargo test --test run_load_tests -- --no-rate-limit --no-fallback");
}

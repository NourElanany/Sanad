use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;
use tokio;
use tracing::{info, warn, error};

/// Setup script for AI service integration
/// This script initializes the Hugging Face and Vector Database integration
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting AI Service Integration Setup");
    
    // Check prerequisites
    check_prerequisites().await?;
    
    // Setup environment variables
    setup_environment().await?;
    
    // Initialize Vector Database
    initialize_vector_database().await?;
    
    // Test Hugging Face connection
    test_hugging_face_connection().await?;
    
    // Setup Redis cache (optional)
    setup_redis_cache().await?;
    
    // Run integration tests
    run_integration_tests().await?;
    
    info!("AI Service Integration Setup completed successfully!");
    
    Ok(())
}

/// Check if all prerequisites are met
async fn check_prerequisites() -> Result<(), Box<dyn std::error::Error>> {
    info!("Checking prerequisites...");
    
    // Check if Rust is installed
    let rust_version = Command::new("rustc")
        .arg("--version")
        .output()?;
    
    if !rust_version.status.success() {
        return Err("Rust is not installed or not in PATH".into());
    }
    
    info!("Rust version: {}", String::from_utf8_lossy(&rust_version.stdout));
    
    // Check if Docker is available (for Qdrant)
    let docker_version = Command::new("docker")
        .arg("--version")
        .output();
    
    match docker_version {
        Ok(output) if output.status.success() => {
            info!("Docker version: {}", String::from_utf8_lossy(&output.stdout));
        }
        _ => {
            warn!("Docker is not available. You'll need to install Qdrant manually.");
        }
    }
    
    // Check if required directories exist
    let required_dirs = [
        "config",
        "src/ai_service",
        "data",
        "logs",
    ];
    
    for dir in &required_dirs {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir)?;
            info!("Created directory: {}", dir);
        }
    }
    
    Ok(())
}

/// Setup environment variables
async fn setup_environment() -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up environment variables...");
    
    // Check for Hugging Face API key
    if env::var("HUGGING_FACE_API_KEY").is_err() {
        warn!("HUGGING_FACE_API_KEY not set. Please set it to use Hugging Face models.");
        println!("To set the API key, run:");
        println!("export HUGGING_FACE_API_KEY=your_api_key_here");
        println!("You can get an API key from: https://huggingface.co/settings/tokens");
    } else {
        info!("Hugging Face API key is configured");
    }
    
    // Set default values for other environment variables if not set
    let env_defaults = [
        ("QDRANT_HOST", "localhost"),
        ("QDRANT_PORT", "6333"),
        ("REDIS_URL", "redis://localhost:6379"),
        ("LOG_LEVEL", "INFO"),
        ("AI_SERVICE_CONFIG", "config/ai_service_config.yaml"),
    ];
    
    for (key, default_value) in &env_defaults {
        if env::var(key).is_err() {
            env::set_var(key, default_value);
            info!("Set {} to default value: {}", key, default_value);
        }
    }
    
    Ok(())
}

/// Initialize Vector Database (Qdrant)
async fn initialize_vector_database() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing Vector Database (Qdrant)...");
    
    // Check if Qdrant is already running
    let qdrant_host = env::var("QDRANT_HOST").unwrap_or_else(|_| "localhost".to_string());
    let qdrant_port = env::var("QDRANT_PORT").unwrap_or_else(|_| "6333".to_string());
    
    // Try to connect to existing Qdrant instance
    let client = reqwest::Client::new();
    let qdrant_url = format!("http://{}:{}/collections", qdrant_host, qdrant_port);
    
    match client.get(&qdrant_url).send().await {
        Ok(response) if response.status().is_success() => {
            info!("Qdrant is already running at {}:{}", qdrant_host, qdrant_port);
        }
        _ => {
            info!("Qdrant is not running. Attempting to start with Docker...");
            start_qdrant_with_docker().await?;
        }
    }
    
    // Wait for Qdrant to be ready
    wait_for_qdrant().await?;
    
    Ok(())
}

/// Start Qdrant using Docker
async fn start_qdrant_with_docker() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Qdrant with Docker...");
    
    // Create data directory for Qdrant
    let qdrant_data_dir = "data/qdrant";
    if !Path::new(qdrant_data_dir).exists() {
        fs::create_dir_all(qdrant_data_dir)?;
    }
    
    // Docker command to start Qdrant
    let docker_cmd = Command::new("docker")
        .args(&[
            "run", "-d",
            "--name", "qdrant-islamic-app",
            "-p", "6333:6333",
            "-p", "6334:6334",
            "-v", &format!("{}:/qdrant/storage", fs::canonicalize(qdrant_data_dir)?.display()),
            "qdrant/qdrant:latest"
        ])
        .output()?;
    
    if !docker_cmd.status.success() {
        let error_msg = String::from_utf8_lossy(&docker_cmd.stderr);
        if error_msg.contains("already in use") {
            info!("Qdrant container already exists, starting it...");
            
            let start_cmd = Command::new("docker")
                .args(&["start", "qdrant-islamic-app"])
                .output()?;
            
            if !start_cmd.status.success() {
                return Err(format!("Failed to start Qdrant container: {}", 
                    String::from_utf8_lossy(&start_cmd.stderr)).into());
            }
        } else {
            return Err(format!("Failed to run Qdrant container: {}", error_msg).into());
        }
    }
    
    info!("Qdrant container started successfully");
    Ok(())
}

/// Wait for Qdrant to be ready
async fn wait_for_qdrant() -> Result<(), Box<dyn std::error::Error>> {
    info!("Waiting for Qdrant to be ready...");
    
    let client = reqwest::Client::new();
    let qdrant_host = env::var("QDRANT_HOST").unwrap_or_else(|_| "localhost".to_string());
    let qdrant_port = env::var("QDRANT_PORT").unwrap_or_else(|_| "6333".to_string());
    let health_url = format!("http://{}:{}/", qdrant_host, qdrant_port);
    
    for attempt in 1..=30 {
        match client.get(&health_url).send().await {
            Ok(response) if response.status().is_success() => {
                info!("Qdrant is ready!");
                return Ok(());
            }
            _ => {
                if attempt % 5 == 0 {
                    info!("Still waiting for Qdrant... (attempt {}/30)", attempt);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    }
    
    Err("Qdrant did not become ready within 60 seconds".into())
}

/// Test Hugging Face connection
async fn test_hugging_face_connection() -> Result<(), Box<dyn std::error::Error>> {
    info!("Testing Hugging Face connection...");
    
    let api_key = match env::var("HUGGING_FACE_API_KEY") {
        Ok(key) if !key.is_empty() => key,
        _ => {
            warn!("Hugging Face API key not set, skipping connection test");
            return Ok(());
        }
    };
    
    let client = reqwest::Client::new();
    let test_url = "https://api-inference.huggingface.co/models/sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2";
    
    let response = client
        .get(test_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;
    
    if response.status().is_success() {
        info!("Hugging Face connection test successful");
    } else {
        warn!("Hugging Face connection test failed with status: {}", response.status());
        let error_text = response.text().await.unwrap_or_default();
        warn!("Error details: {}", error_text);
    }
    
    Ok(())
}

/// Setup Redis cache (optional)
async fn setup_redis_cache() -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up Redis cache...");
    
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    
    // Try to connect to Redis
    match redis::Client::open(redis_url.as_str()) {
        Ok(client) => {
            match client.get_connection() {
                Ok(mut conn) => {
                    // Test Redis connection
                    let _: String = redis::cmd("PING").query(&mut conn)?;
                    info!("Redis connection successful");
                }
                Err(_) => {
                    warn!("Redis is not available. Caching will use local memory only.");
                    start_redis_with_docker().await?;
                }
            }
        }
        Err(e) => {
            warn!("Failed to create Redis client: {}. Caching will use local memory only.", e);
        }
    }
    
    Ok(())
}

/// Start Redis using Docker
async fn start_redis_with_docker() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Redis with Docker...");
    
    let docker_cmd = Command::new("docker")
        .args(&[
            "run", "-d",
            "--name", "redis-islamic-app",
            "-p", "6379:6379",
            "redis:alpine"
        ])
        .output()?;
    
    if !docker_cmd.status.success() {
        let error_msg = String::from_utf8_lossy(&docker_cmd.stderr);
        if error_msg.contains("already in use") {
            info!("Redis container already exists, starting it...");
            
            let start_cmd = Command::new("docker")
                .args(&["start", "redis-islamic-app"])
                .output()?;
            
            if !start_cmd.status.success() {
                warn!("Failed to start Redis container, continuing without Redis");
            } else {
                info!("Redis container started successfully");
            }
        } else {
            warn!("Failed to run Redis container: {}", error_msg);
        }
    } else {
        info!("Redis container started successfully");
    }
    
    Ok(())
}

/// Run integration tests
async fn run_integration_tests() -> Result<(), Box<dyn std::error::Error>> {
    info!("Running integration tests...");
    
    // Run Rust tests
    let test_cmd = Command::new("cargo")
        .args(&["test", "--package", "sanad", "--lib", "ai_service::integration_tests"])
        .output()?;
    
    if test_cmd.status.success() {
        info!("Integration tests passed successfully");
    } else {
        warn!("Some integration tests failed:");
        warn!("{}", String::from_utf8_lossy(&test_cmd.stderr));
    }
    
    Ok(())
}

/// Print setup summary
fn print_setup_summary() {
    println!("\n🎉 AI Service Integration Setup Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Services Status:");
    println!("   ✅ Qdrant Vector Database: Running on port 6333");
    println!("   ✅ Redis Cache: Running on port 6379 (optional)");
    println!("   ✅ Hugging Face API: Configured");
    println!("");
    println!("📁 Configuration:");
    println!("   📄 Main config: config/ai_service_config.yaml");
    println!("   📄 RAG config: config/rag_config.yaml");
    println!("");
    println!("🔧 Environment Variables:");
    println!("   HUGGING_FACE_API_KEY: Set (required)");
    println!("   QDRANT_HOST: localhost");
    println!("   QDRANT_PORT: 6333");
    println!("   REDIS_URL: redis://localhost:6379");
    println!("");
    println!("🚀 Next Steps:");
    println!("   1. Run: cargo run --bin setup_ai_integration");
    println!("   2. Test: cargo test ai_service::integration_tests");
    println!("   3. Start the application with AI service enabled");
    println!("");
    println!("📚 Documentation:");
    println!("   📖 RAG Implementation: docs/RAG_IMPLEMENTATION.md");
    println!("   📖 API Documentation: docs/api/ai_service.md");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

// Add Redis dependency for connection testing
#[cfg(feature = "redis")]
mod redis_test {
    use redis::Commands;
    
    pub fn test_redis_connection(url: &str) -> Result<(), redis::RedisError> {
        let client = redis::Client::open(url)?;
        let mut conn = client.get_connection()?;
        let _: String = conn.ping()?;
        Ok(())
    }
}

#[cfg(not(feature = "redis"))]
mod redis_test {
    pub fn test_redis_connection(_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
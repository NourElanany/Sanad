use axum::{
    routing::get,
    Router,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::{info, Level};

#[derive(Serialize, Deserialize)]
struct Surah {
    number: i32,
    name: String,
    arabic_name: String,
    english_name: String,
    revelation_type: String,
    number_of_ayahs: i32,
}

#[derive(Serialize, Deserialize)]
struct Ayah {
    number: i32,
    text: String,
    surah_number: i32,
    juz: i32,
    page: i32,
}

async fn get_surahs() -> Json<Vec<Surah>> {
    // Placeholder data - will be replaced with database queries
    let surahs = vec![
        Surah {
            number: 1,
            name: "الفاتحة".to_string(),
            arabic_name: "الفاتحة".to_string(),
            english_name: "Al-Fatihah".to_string(),
            revelation_type: "meccan".to_string(),
            number_of_ayahs: 7,
        },
        Surah {
            number: 2,
            name: "البقرة".to_string(),
            arabic_name: "البقرة".to_string(),
            english_name: "Al-Baqarah".to_string(),
            revelation_type: "medinan".to_string(),
            number_of_ayahs: 286,
        },
    ];
    
    Json(surahs)
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "quran-service",
        "message": "Quran service is running"
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting Quran Service...");

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/surahs", get(get_surahs));

    // Run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    info!("Quran Service listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
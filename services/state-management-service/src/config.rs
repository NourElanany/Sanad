use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub sync_interval_seconds: u64,
    pub compression_enabled: bool,
    pub max_storage_mb: u64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/sanad_state".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            sync_interval_seconds: env::var("SYNC_INTERVAL_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
            compression_enabled: env::var("COMPRESSION_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,
            max_storage_mb: env::var("MAX_STORAGE_MB")
                .unwrap_or_else(|_| "500".to_string())
                .parse()?,
        })
    }
}
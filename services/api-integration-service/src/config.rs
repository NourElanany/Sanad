//! Configuration loading and management
//!
//! This module provides functionality to load service configuration from YAML files
//! with support for environment variable overrides.

use crate::models::*;
use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::Path;

/// Load configuration from a YAML file with environment variable overrides
///
/// # Arguments
/// * `config_path` - Path to the YAML configuration file
///
/// # Returns
/// * `Result<ServiceConfig>` - The loaded and validated configuration
///
/// # Environment Variable Overrides
/// The following environment variables can override YAML configuration:
/// - `SERVICE_NAME` - Override service.name
/// - `SERVICE_PORT` - Override service.port
/// - `SERVICE_HOST` - Override service.host
/// - `REDIS_URL` - Override redis.url
/// - `REDIS_POOL_SIZE` - Override redis.pool_size
/// - `REDIS_CONNECTION_TIMEOUT` - Override redis.connection_timeout
/// - `POSTGRES_URL` or `DATABASE_URL` - Override postgres.url
/// - `POSTGRES_POOL_SIZE` - Override postgres.pool_size
/// - `POSTGRES_CONNECTION_TIMEOUT` - Override postgres.connection_timeout
///
/// # Example
/// ```no_run
/// use api_integration_service::config::load_config;
///
/// let config = load_config("config/api_integration_config.yaml").unwrap();
/// println!("Service: {} on port {}", config.service.name, config.service.port);
/// ```
pub fn load_config<P: AsRef<Path>>(config_path: P) -> Result<ServiceConfig> {
    // Read the YAML file
    let config_content = fs::read_to_string(config_path.as_ref())
        .with_context(|| format!("Failed to read config file: {:?}", config_path.as_ref()))?;
    
    // Parse YAML into ServiceConfig
    let mut config: ServiceConfig = serde_yaml::from_str(&config_content)
        .context("Failed to parse YAML configuration")?;
    
    // Apply environment variable overrides
    apply_env_overrides(&mut config)?;
    
    // Validate the configuration
    validate_config(&config)?;
    
    Ok(config)
}

/// Apply environment variable overrides to the configuration
fn apply_env_overrides(config: &mut ServiceConfig) -> Result<()> {
    // Service overrides
    if let Ok(name) = env::var("SERVICE_NAME") {
        config.service.name = name;
    }
    
    if let Ok(port) = env::var("SERVICE_PORT") {
        config.service.port = port.parse()
            .context("SERVICE_PORT must be a valid u16")?;
    }
    
    if let Ok(host) = env::var("SERVICE_HOST") {
        config.service.host = host;
    }
    
    // Redis overrides
    if let Ok(url) = env::var("REDIS_URL") {
        config.redis.url = url;
    }
    
    if let Ok(pool_size) = env::var("REDIS_POOL_SIZE") {
        config.redis.pool_size = pool_size.parse()
            .context("REDIS_POOL_SIZE must be a valid u32")?;
    }
    
    if let Ok(timeout) = env::var("REDIS_CONNECTION_TIMEOUT") {
        config.redis.connection_timeout = timeout;
    }
    
    // Postgres overrides (support both POSTGRES_URL and DATABASE_URL)
    if let Ok(url) = env::var("POSTGRES_URL").or_else(|_| env::var("DATABASE_URL")) {
        config.postgres.url = url;
    }
    
    if let Ok(pool_size) = env::var("POSTGRES_POOL_SIZE") {
        config.postgres.pool_size = pool_size.parse()
            .context("POSTGRES_POOL_SIZE must be a valid u32")?;
    }
    
    if let Ok(timeout) = env::var("POSTGRES_CONNECTION_TIMEOUT") {
        config.postgres.connection_timeout = timeout;
    }
    
    Ok(())
}

/// Validate the configuration for required fields and valid values
fn validate_config(config: &ServiceConfig) -> Result<()> {
    // Validate service info
    if config.service.name.is_empty() {
        anyhow::bail!("Service name cannot be empty");
    }
    
    if config.service.port == 0 {
        anyhow::bail!("Service port must be greater than 0");
    }
    
    if config.service.host.is_empty() {
        anyhow::bail!("Service host cannot be empty");
    }
    
    // Validate Redis config
    if config.redis.url.is_empty() {
        anyhow::bail!("Redis URL cannot be empty");
    }
    
    if config.redis.pool_size == 0 {
        anyhow::bail!("Redis pool size must be greater than 0");
    }
    
    if config.redis.connection_timeout.is_empty() {
        anyhow::bail!("Redis connection timeout cannot be empty");
    }
    
    // Validate Postgres config
    if config.postgres.url.is_empty() {
        anyhow::bail!("Postgres URL cannot be empty");
    }
    
    if config.postgres.pool_size == 0 {
        anyhow::bail!("Postgres pool size must be greater than 0");
    }
    
    if config.postgres.connection_timeout.is_empty() {
        anyhow::bail!("Postgres connection timeout cannot be empty");
    }
    
    // Validate API configurations
    validate_api_configs(&config.apis)?;
    
    // Validate cache strategies
    validate_cache_config(&config.cache)?;
    
    // Validate health monitor config
    if config.health_monitor.unhealthy_threshold == 0 {
        anyhow::bail!("Unhealthy threshold must be greater than 0");
    }
    
    if config.health_monitor.recovery_threshold == 0 {
        anyhow::bail!("Recovery threshold must be greater than 0");
    }
    
    if config.health_monitor.check_interval.is_empty() {
        anyhow::bail!("Health monitor check interval cannot be empty");
    }
    
    // Validate retry config
    if config.retry.max_attempts == 0 {
        anyhow::bail!("Max retry attempts must be greater than 0");
    }
    
    if config.retry.multiplier <= 0.0 {
        anyhow::bail!("Retry multiplier must be greater than 0");
    }
    
    if config.retry.initial_delay.is_empty() {
        anyhow::bail!("Initial retry delay cannot be empty");
    }
    
    if config.retry.max_delay.is_empty() {
        anyhow::bail!("Max retry delay cannot be empty");
    }
    
    Ok(())
}

/// Validate API configurations
fn validate_api_configs(apis: &ApiConfigs) -> Result<()> {
    // Validate each API category
    validate_api_list(&apis.quran, "quran")?;
    validate_api_list(&apis.hadith, "hadith")?;
    validate_api_list(&apis.prayer_times, "prayer_times")?;
    validate_api_list(&apis.tafsir, "tafsir")?;
    validate_api_list(&apis.calendar, "calendar")?;
    validate_api_list(&apis.qibla, "qibla")?;
    validate_api_list(&apis.ai, "ai")?;
    
    Ok(())
}

/// Validate a list of API configurations
fn validate_api_list(apis: &[ApiConfig], category: &str) -> Result<()> {
    for api in apis {
        if api.name.is_empty() {
            anyhow::bail!("API name cannot be empty in {} category", category);
        }
        
        if api.base_url.is_empty() {
            anyhow::bail!("API base_url cannot be empty for {} in {} category", api.name, category);
        }
        
        if api.priority == 0 {
            anyhow::bail!("API priority must be greater than 0 for {} in {} category", api.name, category);
        }
        
        if api.timeout.is_empty() {
            anyhow::bail!("API timeout cannot be empty for {} in {} category", api.name, category);
        }
        
        // Validate rate limits
        if api.rate_limit.requests_per_minute == 0 {
            anyhow::bail!("Rate limit requests_per_minute must be greater than 0 for {} in {} category", api.name, category);
        }
        
        if api.rate_limit.requests_per_hour == 0 {
            anyhow::bail!("Rate limit requests_per_hour must be greater than 0 for {} in {} category", api.name, category);
        }
        
        if api.rate_limit.requests_per_day == 0 {
            anyhow::bail!("Rate limit requests_per_day must be greater than 0 for {} in {} category", api.name, category);
        }
    }
    
    Ok(())
}

/// Validate cache configuration
fn validate_cache_config(cache: &CacheConfig) -> Result<()> {
    validate_cache_strategy(&cache.strategies.quran_text, "quran_text")?;
    validate_cache_strategy(&cache.strategies.hadith, "hadith")?;
    validate_cache_strategy(&cache.strategies.prayer_times, "prayer_times")?;
    validate_cache_strategy(&cache.strategies.tafsir, "tafsir")?;
    validate_cache_strategy(&cache.strategies.calendar, "calendar")?;
    validate_cache_strategy(&cache.strategies.qibla, "qibla")?;
    validate_cache_strategy(&cache.strategies.ai_response, "ai_response")?;
    
    Ok(())
}

/// Validate a single cache strategy
fn validate_cache_strategy(strategy: &CacheStrategy, name: &str) -> Result<()> {
    if strategy.ttl.is_empty() {
        anyhow::bail!("Cache TTL cannot be empty for {} strategy", name);
    }
    
    if strategy.allow_stale && strategy.stale_ttl.is_none() {
        anyhow::bail!("Stale TTL must be specified when allow_stale is true for {} strategy", name);
    }
    
    if let Some(ref stale_ttl) = strategy.stale_ttl {
        if stale_ttl.is_empty() {
            anyhow::bail!("Stale TTL cannot be empty for {} strategy", name);
        }
    }
    
    Ok(())
}

/// Load configuration from default location with environment variable overrides
///
/// This function looks for configuration in the following order:
/// 1. Path specified in `CONFIG_PATH` environment variable
/// 2. `config/api_integration_config.yaml` (relative to current directory)
/// 3. `/etc/sanad/config/api_integration_config.yaml` (system-wide)
///
/// # Returns
/// * `Result<ServiceConfig>` - The loaded and validated configuration
pub fn load_config_from_default_location() -> Result<ServiceConfig> {
    // Check for CONFIG_PATH environment variable
    if let Ok(config_path) = env::var("CONFIG_PATH") {
        return load_config(&config_path);
    }
    
    // Try local config directory
    let local_config = "config/api_integration_config.yaml";
    if Path::new(local_config).exists() {
        return load_config(local_config);
    }
    
    // Try system-wide config
    let system_config = "/etc/sanad/config/api_integration_config.yaml";
    if Path::new(system_config).exists() {
        return load_config(system_config);
    }
    
    anyhow::bail!(
        "Configuration file not found. Tried:\n\
         - CONFIG_PATH environment variable\n\
         - {}\n\
         - {}",
        local_config,
        system_config
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use serial_test::serial;

    fn create_test_yaml_config() -> String {
        r#"
service:
  name: test-service
  port: 8080
  host: 0.0.0.0

redis:
  url: redis://localhost:6379
  pool_size: 10
  connection_timeout: 5s

postgres:
  url: postgresql://user:pass@localhost:5432/sanad
  pool_size: 20
  connection_timeout: 10s

apis:
  quran:
    - name: quran.com
      base_url: https://api.quran.com/api/v4
      priority: 1
      requires_key: false
      rate_limit:
        requests_per_minute: 60
        requests_per_hour: 1000
        requests_per_day: 10000
      timeout: 10s
  hadith: []
  prayer_times: []
  tafsir: []
  calendar: []
  qibla: []
  ai: []

cache:
  strategies:
    quran_text:
      ttl: 30d
      allow_stale: true
      stale_ttl: 90d
    hadith:
      ttl: 30d
      allow_stale: true
      stale_ttl: 90d
    prayer_times:
      ttl: 1d
      allow_stale: true
      stale_ttl: 7d
    tafsir:
      ttl: 30d
      allow_stale: true
      stale_ttl: 90d
    calendar:
      ttl: 7d
      allow_stale: true
      stale_ttl: 30d
    qibla:
      ttl: 30d
      allow_stale: true
      stale_ttl: 90d
    ai_response:
      ttl: 1h
      allow_stale: false

health_monitor:
  check_interval: 5m
  unhealthy_threshold: 3
  recovery_threshold: 2

retry:
  max_attempts: 3
  initial_delay: 1s
  max_delay: 10s
  multiplier: 2.0
"#.to_string()
    }

    #[test]
    #[serial]
    fn test_load_config_from_yaml() {
        // Clear any environment variables that might interfere
        env::remove_var("SERVICE_NAME");
        env::remove_var("SERVICE_PORT");
        env::remove_var("SERVICE_HOST");
        env::remove_var("REDIS_URL");
        env::remove_var("REDIS_POOL_SIZE");
        env::remove_var("POSTGRES_URL");
        env::remove_var("DATABASE_URL");
        
        let yaml_content = create_test_yaml_config();
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();
        
        let config = load_config(temp_file.path()).unwrap();
        
        assert_eq!(config.service.name, "test-service");
        assert_eq!(config.service.port, 8080);
        assert_eq!(config.service.host, "0.0.0.0");
        assert_eq!(config.redis.url, "redis://localhost:6379");
        assert_eq!(config.redis.pool_size, 10);
        assert_eq!(config.postgres.pool_size, 20);
        assert_eq!(config.health_monitor.unhealthy_threshold, 3);
        assert_eq!(config.retry.max_attempts, 3);
        assert_eq!(config.retry.multiplier, 2.0);
    }

    #[test]
    #[serial]
    fn test_env_override_service_name() {
        // Clear all env vars first
        env::remove_var("SERVICE_NAME");
        env::remove_var("SERVICE_PORT");
        env::remove_var("REDIS_POOL_SIZE");
        
        let yaml_content = create_test_yaml_config();
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();
        
        env::set_var("SERVICE_NAME", "overridden-service");
        let config = load_config(temp_file.path()).unwrap();
        env::remove_var("SERVICE_NAME");
        
        assert_eq!(config.service.name, "overridden-service");
    }

    #[test]
    #[serial]
    fn test_env_override_service_port() {
        // Clear all env vars first
        env::remove_var("SERVICE_NAME");
        env::remove_var("SERVICE_PORT");
        env::remove_var("REDIS_POOL_SIZE");
        
        let yaml_content = create_test_yaml_config();
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();
        
        env::set_var("SERVICE_PORT", "9090");
        let config = load_config(temp_file.path()).unwrap();
        env::remove_var("SERVICE_PORT");
        
        assert_eq!(config.service.port, 9090);
    }

    #[test]
    #[serial]
    fn test_env_override_redis_url() {
        let yaml_content = create_test_yaml_config();
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();
        
        env::set_var("REDIS_URL", "redis://production:6379");
        let config = load_config(temp_file.path()).unwrap();
        env::remove_var("REDIS_URL");
        
        assert_eq!(config.redis.url, "redis://production:6379");
    }

    #[test]
    #[serial]
    fn test_env_override_postgres_url() {
        let yaml_content = create_test_yaml_config();
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();
        
        env::set_var("POSTGRES_URL", "postgresql://prod:pass@prod:5432/sanad");
        let config = load_config(temp_file.path()).unwrap();
        env::remove_var("POSTGRES_URL");
        
        assert_eq!(config.postgres.url, "postgresql://prod:pass@prod:5432/sanad");
    }

    #[test]
    #[serial]
    fn test_env_override_database_url() {
        let yaml_content = create_test_yaml_config();
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();
        
        env::set_var("DATABASE_URL", "postgresql://db:pass@db:5432/sanad");
        let config = load_config(temp_file.path()).unwrap();
        env::remove_var("DATABASE_URL");
        
        assert_eq!(config.postgres.url, "postgresql://db:pass@db:5432/sanad");
    }

    #[test]
    #[serial]
    fn test_multiple_env_overrides() {
        // Clear all env vars first
        env::remove_var("SERVICE_NAME");
        env::remove_var("SERVICE_PORT");
        env::remove_var("REDIS_POOL_SIZE");
        
        let yaml_content = create_test_yaml_config();
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();
        
        env::set_var("SERVICE_NAME", "multi-override");
        env::set_var("SERVICE_PORT", "7777");
        env::set_var("REDIS_POOL_SIZE", "25");
        
        let config = load_config(temp_file.path()).unwrap();
        
        env::remove_var("SERVICE_NAME");
        env::remove_var("SERVICE_PORT");
        env::remove_var("REDIS_POOL_SIZE");
        
        assert_eq!(config.service.name, "multi-override");
        assert_eq!(config.service.port, 7777);
        assert_eq!(config.redis.pool_size, 25);
    }

    #[test]
    fn test_invalid_yaml() {
        let invalid_yaml = "invalid: yaml: content: [";
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(invalid_yaml.as_bytes()).unwrap();
        temp_file.flush().unwrap();
        
        let result = load_config(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_required_field() {
        let incomplete_yaml = r#"
service:
  name: test
  port: 8080
  host: localhost
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(incomplete_yaml.as_bytes()).unwrap();
        temp_file.flush().unwrap();
        
        let result = load_config(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_empty_service_name() {
        let mut config = create_valid_test_config();
        config.service.name = String::new();
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Service name cannot be empty"));
    }

    #[test]
    fn test_validation_zero_port() {
        let mut config = create_valid_test_config();
        config.service.port = 0;
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("port must be greater than 0"));
    }

    #[test]
    fn test_validation_empty_redis_url() {
        let mut config = create_valid_test_config();
        config.redis.url = String::new();
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Redis URL cannot be empty"));
    }

    #[test]
    fn test_validation_zero_unhealthy_threshold() {
        let mut config = create_valid_test_config();
        config.health_monitor.unhealthy_threshold = 0;
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unhealthy threshold must be greater than 0"));
    }

    #[test]
    fn test_validation_zero_retry_attempts() {
        let mut config = create_valid_test_config();
        config.retry.max_attempts = 0;
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Max retry attempts must be greater than 0"));
    }

    #[test]
    fn test_validation_invalid_retry_multiplier() {
        let mut config = create_valid_test_config();
        config.retry.multiplier = 0.0;
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Retry multiplier must be greater than 0"));
    }

    #[test]
    fn test_validation_api_empty_name() {
        let mut config = create_valid_test_config();
        config.apis.quran.push(ApiConfig {
            name: String::new(),
            base_url: "https://api.example.com".to_string(),
            priority: 1,
            requires_key: Some(false),
            rate_limit: RateLimitConfig {
                requests_per_minute: 60,
                requests_per_hour: 1000,
                requests_per_day: 10000,
            },
            timeout: "10s".to_string(),
        });
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("API name cannot be empty"));
    }

    #[test]
    fn test_validation_api_zero_priority() {
        let mut config = create_valid_test_config();
        config.apis.quran.push(ApiConfig {
            name: "test-api".to_string(),
            base_url: "https://api.example.com".to_string(),
            priority: 0,
            requires_key: Some(false),
            rate_limit: RateLimitConfig {
                requests_per_minute: 60,
                requests_per_hour: 1000,
                requests_per_day: 10000,
            },
            timeout: "10s".to_string(),
        });
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("priority must be greater than 0"));
    }

    #[test]
    fn test_validation_cache_empty_ttl() {
        let mut config = create_valid_test_config();
        config.cache.strategies.quran_text.ttl = String::new();
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cache TTL cannot be empty"));
    }

    #[test]
    fn test_validation_cache_stale_without_ttl() {
        let mut config = create_valid_test_config();
        config.cache.strategies.quran_text.allow_stale = true;
        config.cache.strategies.quran_text.stale_ttl = None;
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Stale TTL must be specified"));
    }

    fn create_valid_test_config() -> ServiceConfig {
        ServiceConfig {
            service: ServiceInfo {
                name: "test-service".to_string(),
                port: 8080,
                host: "localhost".to_string(),
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                pool_size: 10,
                connection_timeout: "5s".to_string(),
            },
            postgres: PostgresConfig {
                url: "postgresql://localhost:5432/test".to_string(),
                pool_size: 20,
                connection_timeout: "10s".to_string(),
            },
            apis: ApiConfigs {
                quran: vec![],
                hadith: vec![],
                prayer_times: vec![],
                tafsir: vec![],
                calendar: vec![],
                qibla: vec![],
                ai: vec![],
            },
            cache: CacheConfig {
                strategies: CacheStrategies {
                    quran_text: CacheStrategy {
                        ttl: "30d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("90d".to_string()),
                    },
                    hadith: CacheStrategy {
                        ttl: "30d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("90d".to_string()),
                    },
                    prayer_times: CacheStrategy {
                        ttl: "1d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("7d".to_string()),
                    },
                    tafsir: CacheStrategy {
                        ttl: "30d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("90d".to_string()),
                    },
                    calendar: CacheStrategy {
                        ttl: "7d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("30d".to_string()),
                    },
                    qibla: CacheStrategy {
                        ttl: "30d".to_string(),
                        allow_stale: true,
                        stale_ttl: Some("90d".to_string()),
                    },
                    ai_response: CacheStrategy {
                        ttl: "1h".to_string(),
                        allow_stale: false,
                        stale_ttl: None,
                    },
                },
            },
            health_monitor: HealthMonitorConfig {
                check_interval: "5m".to_string(),
                unhealthy_threshold: 3,
                recovery_threshold: 2,
            },
            retry: RetryConfig {
                max_attempts: 3,
                initial_delay: "1s".to_string(),
                max_delay: "10s".to_string(),
                multiplier: 2.0,
            },
        }
    }
}

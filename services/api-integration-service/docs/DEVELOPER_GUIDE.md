# API Integration Service - Developer Guide

## Table of Contents

1. [Overview](#overview)
2. [Development Setup](#development-setup)
3. [Project Structure](#project-structure)
4. [Adding New API Integrations](#adding-new-api-integrations)
5. [Testing Strategy](#testing-strategy)
6. [Code Style and Standards](#code-style-and-standards)
7. [Debugging and Troubleshooting](#debugging-and-troubleshooting)
8. [Performance Optimization](#performance-optimization)
9. [Contributing Guidelines](#contributing-guidelines)
10. [Common Patterns](#common-patterns)

## Overview

This guide provides comprehensive information for developers working on the API Integration Service. It covers development setup, architecture patterns, testing strategies, and best practices for adding new API integrations.

### Architecture Principles

The service follows these core principles:

1. **Trait-Based Design**: All API clients implement common traits for consistency
2. **Fallback-First**: Every API category has fallback mechanisms
3. **Cache-Aware**: Intelligent caching strategies per data type
4. **Observable**: Comprehensive logging, metrics, and tracing
5. **Testable**: Unit tests, property tests, and integration tests
6. **Resilient**: Retry mechanisms, circuit breakers, and graceful degradation

### Technology Stack

- **Language**: Rust 1.75+
- **Web Framework**: Actix-web 4.x
- **Async Runtime**: Tokio 1.x
- **HTTP Client**: Reqwest 0.11+
- **Caching**: Redis 6.0+
- **Database**: PostgreSQL 13+
- **Serialization**: Serde
- **Testing**: Proptest (property-based), Mockito (mocking)
- **Observability**: Tracing, Prometheus, OpenTelemetry

## Development Setup

### Prerequisites

**Required**:
- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Docker and Docker Compose
- Git
- A code editor (VS Code, IntelliJ IDEA, or Vim)

**Optional**:
- Redis CLI (for debugging)
- PostgreSQL CLI (psql)
- Postman or similar API testing tool

### Initial Setup

**1. Clone the repository**:
```bash
git clone https://github.com/your-org/sanad.git
cd sanad
```

**2. Install Rust toolchain**:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install additional components
rustup component add rustfmt clippy
```

**3. Start dependencies**:
```bash
# Start Redis and PostgreSQL
docker-compose up -d redis postgres

# Verify services are running
docker-compose ps
```

**4. Set up environment variables**:
```bash
# Copy example environment file
cp .env.example .env

# Edit .env and add your API keys
nano .env
```

**5. Run database migrations**:
```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run --database-url postgresql://postgres:postgres@localhost:5432/sanad
```

**6. Build the project**:
```bash
# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release
```

**7. Run the service**:
```bash
# Run in development mode
cargo run --bin api-integration-service

# Or with custom config
CONFIG_PATH=config/api_integration_config.yaml cargo run --bin api-integration-service
```

**8. Verify the service**:
```bash
# Check health endpoint
curl http://localhost:8080/api/v1/health

# Test an endpoint
curl "http://localhost:8080/api/v1/quran/text?surah=1&ayah=1"
```

### Development Tools

**VS Code Extensions**:
- rust-analyzer
- CodeLLDB (debugging)
- Better TOML
- Error Lens
- GitLens

**VS Code Settings** (`.vscode/settings.json`):
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

**IntelliJ IDEA**:
- Install Rust plugin
- Enable "Run Clippy on save"
- Enable "Format on save"


## Project Structure

```
services/api-integration-service/
├── src/
│   ├── main.rs                 # Entry point
│   ├── lib.rs                  # Library root
│   ├── service.rs              # Main service implementation
│   ├── handlers.rs             # HTTP request handlers
│   ├── middleware.rs           # HTTP middleware
│   ├── config.rs               # Configuration management
│   ├── models.rs               # Data models
│   ├── observability.rs        # Logging, metrics, tracing
│   ├── tests/                  # Unit tests
│   │   ├── quran_tests.rs
│   │   ├── hadith_tests.rs
│   │   └── ...
│   └── property_tests/         # Property-based tests
│       ├── api_client_properties.rs
│       ├── cache_properties.rs
│       └── ...
├── tests/
│   └── integration_tests.rs    # Integration tests
├── Cargo.toml                  # Dependencies
├── config/
│   └── api_integration_config.yaml
└── docs/
    ├── API_DOCUMENTATION.md
    ├── DEPLOYMENT_GUIDE.md
    └── DEVELOPER_GUIDE.md

shared/src/api_clients/          # Shared API client library
├── mod.rs                       # Module exports
├── traits.rs                    # Common traits
├── error_handler.rs             # Error handling
├── retry_mechanism.rs           # Retry logic
├── fallback_system.rs           # Fallback mechanisms
├── health_monitor.rs            # Health monitoring
├── cache_manager.rs             # Caching
├── rate_limiter.rs              # Rate limiting
├── api_key_manager.rs           # API key management
├── quran/                       # Quran API clients
│   ├── mod.rs
│   ├── manager.rs
│   ├── quran_com_client.rs
│   ├── alquran_cloud_client.rs
│   ├── tanzil_client.rs
│   ├── everyayah_client.rs
│   ├── tests.rs
│   └── property_tests.rs
├── hadith/                      # Hadith API clients
│   ├── mod.rs
│   ├── manager.rs
│   ├── sunnah_com_client.rs
│   ├── hadith_api_client.rs
│   ├── aladhan_hadith_client.rs
│   ├── tests.rs
│   └── property_tests.rs
├── prayer/                      # Prayer times API clients
├── tafsir/                      # Tafsir API clients
├── calendar/                    # Calendar API clients
├── qibla/                       # Qibla API clients
└── ai/                          # AI/NLP API clients
```

### Key Components

**Service Layer** (`src/service.rs`):
- Main service struct that coordinates all API managers
- Implements business logic
- Handles request routing

**Handlers Layer** (`src/handlers.rs`):
- HTTP request handlers
- Request validation
- Response serialization

**API Clients** (`shared/src/api_clients/`):
- Individual API client implementations
- Manager classes for each API category
- Fallback and retry logic

**Infrastructure** (`shared/src/api_clients/`):
- Cache manager
- Rate limiter
- Health monitor
- Error handler
- Retry mechanism

## Adding New API Integrations

### Step-by-Step Guide

Let's walk through adding a new API integration. We'll use a hypothetical "Islamic Calendar Events API" as an example.

#### Step 1: Define the Trait

**File**: `shared/src/api_clients/traits.rs`

```rust
use async_trait::async_trait;
use crate::api_clients::error::ApiError;

#[async_trait]
pub trait CalendarEventsApiClient: ApiClient {
    /// Get Islamic events for a date range
    async fn get_events(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<IslamicEvent>, ApiError>;
    
    /// Get events for a specific Hijri month
    async fn get_month_events(
        &self,
        hijri_year: i32,
        hijri_month: u8,
    ) -> Result<Vec<IslamicEvent>, ApiError>;
}
```

#### Step 2: Create Data Models

**File**: `shared/src/api_clients/calendar/models.rs`

```rust
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslamicEvent {
    pub date: NaiveDate,
    pub hijri_date: HijriDate,
    pub event_name_ar: String,
    pub event_name_en: String,
    pub event_type: EventType,
    pub description: String,
    pub significance: Significance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    MonthStart,
    Eid,
    SpecialDay,
    HistoricalEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Significance {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HijriDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub month_name_ar: String,
    pub month_name_en: String,
}
```

#### Step 3: Implement the API Client

**File**: `shared/src/api_clients/calendar/islamic_calendar_client.rs`

```rust
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::api_clients::{ApiClient, CalendarEventsApiClient, ApiError, RateLimitConfig};
use super::models::*;

pub struct IslamicCalendarClient {
    base_url: String,
    client: Client,
    api_key: Option<String>,
}

impl IslamicCalendarClient {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        Self {
            base_url,
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl ApiClient for IslamicCalendarClient {
    fn api_name(&self) -> &str {
        "islamic-calendar"
    }
    
    fn priority(&self) -> u8 {
        1  // Primary
    }
    
    async fn is_healthy(&self) -> bool {
        // Implement health check
        let url = format!("{}/health", self.base_url);
        self.client.get(&url)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
    
    fn rate_limit(&self) -> RateLimitConfig {
        RateLimitConfig {
            requests_per_minute: 60,
            requests_per_hour: 1000,
            requests_per_day: 10000,
        }
    }
}

#[async_trait]
impl CalendarEventsApiClient for IslamicCalendarClient {
    async fn get_events(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<IslamicEvent>, ApiError> {
        let url = format!("{}/events", self.base_url);
        
        let mut request = self.client.get(&url)
            .query(&[
                ("start_date", start_date.to_string()),
                ("end_date", end_date.to_string()),
            ]);
        
        // Add API key if available
        if let Some(ref key) = self.api_key {
            request = request.header("X-API-Key", key);
        }
        
        let response = request.send().await
            .map_err(|e| ApiError::Network(e))?;
        
        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("Status: {}", response.status())
            ));
        }
        
        // Parse response
        let api_response: ApiResponse = response.json().await
            .map_err(|e| ApiError::InvalidResponse(
                self.api_name().to_string(),
                e.to_string()
            ))?;
        
        // Convert to our model
        Ok(api_response.events.into_iter()
            .map(|e| e.into())
            .collect())
    }
    
    async fn get_month_events(
        &self,
        hijri_year: i32,
        hijri_month: u8,
    ) -> Result<Vec<IslamicEvent>, ApiError> {
        // Implementation similar to get_events
        todo!()
    }
}

// API response models (specific to this API)
#[derive(Debug, Deserialize)]
struct ApiResponse {
    events: Vec<ApiEvent>,
}

#[derive(Debug, Deserialize)]
struct ApiEvent {
    date: String,
    hijri_date: ApiHijriDate,
    name_ar: String,
    name_en: String,
    event_type: String,
    description: String,
}

// Conversion from API model to our model
impl From<ApiEvent> for IslamicEvent {
    fn from(api_event: ApiEvent) -> Self {
        IslamicEvent {
            date: NaiveDate::parse_from_str(&api_event.date, "%Y-%m-%d").unwrap(),
            hijri_date: api_event.hijri_date.into(),
            event_name_ar: api_event.name_ar,
            event_name_en: api_event.name_en,
            event_type: match api_event.event_type.as_str() {
                "month_start" => EventType::MonthStart,
                "eid" => EventType::Eid,
                "special_day" => EventType::SpecialDay,
                _ => EventType::HistoricalEvent,
            },
            description: api_event.description,
            significance: Significance::Medium,  // Default
        }
    }
}
```


#### Step 4: Create the Manager

**File**: `shared/src/api_clients/calendar/manager.rs`

```rust
use std::sync::Arc;
use crate::api_clients::{
    CalendarEventsApiClient, CacheManager, RateLimiter, ApiError
};
use super::models::*;

pub struct CalendarEventsManager {
    clients: Vec<Box<dyn CalendarEventsApiClient>>,
    cache: Arc<CacheManager>,
    rate_limiter: Arc<RateLimiter>,
}

impl CalendarEventsManager {
    pub fn new(
        clients: Vec<Box<dyn CalendarEventsApiClient>>,
        cache: Arc<CacheManager>,
        rate_limiter: Arc<RateLimiter>,
    ) -> Self {
        Self {
            clients,
            cache,
            rate_limiter,
        }
    }
    
    pub async fn get_events(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<IslamicEvent>, ApiError> {
        // 1. Generate cache key
        let cache_key = format!("calendar:events:{}:{}", start_date, end_date);
        
        // 2. Check cache
        if let Some(cached) = self.cache.get::<Vec<IslamicEvent>>(&cache_key).await? {
            log::debug!("Cache hit for calendar events");
            return Ok(cached);
        }
        
        // 3. Try each client in priority order
        for client in &self.clients {
            // Check if client is healthy
            if !client.is_healthy().await {
                log::warn!("Client {} is unhealthy, skipping", client.api_name());
                continue;
            }
            
            // Check rate limit
            if !self.rate_limiter.check(client.api_name()).await? {
                log::warn!("Rate limit exceeded for {}", client.api_name());
                continue;
            }
            
            // Make request
            match client.get_events(start_date, end_date).await {
                Ok(events) => {
                    // Increment rate limiter
                    self.rate_limiter.increment(client.api_name()).await?;
                    
                    // Cache the result (7 days TTL for calendar events)
                    self.cache.set(
                        &cache_key,
                        &events,
                        Duration::from_secs(7 * 24 * 3600)
                    ).await?;
                    
                    log::info!("Successfully fetched events from {}", client.api_name());
                    return Ok(events);
                }
                Err(e) => {
                    log::warn!("Client {} failed: {}", client.api_name(), e);
                    continue;
                }
            }
        }
        
        // 4. All clients failed, try stale cache
        if let Some(stale) = self.cache.get_expired::<Vec<IslamicEvent>>(&cache_key).await? {
            log::warn!("Serving stale cache for calendar events");
            return Ok(stale);
        }
        
        // 5. Everything failed
        Err(ApiError::AllApisFailed)
    }
}
```

#### Step 5: Add Unit Tests

**File**: `shared/src/api_clients/calendar/tests.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    
    #[tokio::test]
    async fn test_get_events_success() {
        // Mock API response
        let _m = mock("GET", "/events")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("start_date".into(), "2024-01-01".into()),
                mockito::Matcher::UrlEncoded("end_date".into(), "2024-12-31".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "events": [
                    {
                        "date": "2024-03-10",
                        "hijri_date": {
                            "year": 1445,
                            "month": 9,
                            "day": 1,
                            "month_name_ar": "رمضان",
                            "month_name_en": "Ramadan"
                        },
                        "name_ar": "بداية رمضان",
                        "name_en": "Start of Ramadan",
                        "event_type": "month_start",
                        "description": "The beginning of Ramadan"
                    }
                ]
            }"#)
            .create();
        
        let client = IslamicCalendarClient::new(server_url(), None);
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        
        let result = client.get_events(start_date, end_date).await;
        
        assert!(result.is_ok());
        let events = result.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_name_en, "Start of Ramadan");
    }
    
    #[tokio::test]
    async fn test_get_events_api_error() {
        let _m = mock("GET", "/events")
            .with_status(500)
            .create();
        
        let client = IslamicCalendarClient::new(server_url(), None);
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        
        let result = client.get_events(start_date, end_date).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            ApiError::ApiError(api, _) => assert_eq!(api, "islamic-calendar"),
            _ => panic!("Expected ApiError"),
        }
    }
    
    #[tokio::test]
    async fn test_health_check() {
        let _m = mock("GET", "/health")
            .with_status(200)
            .create();
        
        let client = IslamicCalendarClient::new(server_url(), None);
        let is_healthy = client.is_healthy().await;
        
        assert!(is_healthy);
    }
}
```

#### Step 6: Add Property-Based Tests

**File**: `shared/src/api_clients/calendar/property_tests.rs`

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    // Property: Date range should always be valid (start <= end)
    proptest! {
        #[test]
        fn date_range_validation(
            start_year in 2000i32..2100,
            start_month in 1u32..=12,
            start_day in 1u32..=28,
            days_diff in 0u32..365,
        ) {
            let start_date = NaiveDate::from_ymd_opt(
                start_year,
                start_month,
                start_day
            ).unwrap();
            
            let end_date = start_date + Duration::days(days_diff as i64);
            
            // Property: start_date should always be <= end_date
            prop_assert!(start_date <= end_date);
        }
    }
    
    // Property: Events should be within requested date range
    proptest! {
        #[test]
        fn events_within_date_range(
            start_year in 2000i32..2100,
            start_month in 1u32..=12,
            start_day in 1u32..=28,
            days_diff in 1u32..365,
        ) {
            // This would be tested with a mock that returns events
            // Property: All returned events should have dates within [start_date, end_date]
            
            let start_date = NaiveDate::from_ymd_opt(
                start_year,
                start_month,
                start_day
            ).unwrap();
            
            let end_date = start_date + Duration::days(days_diff as i64);
            
            // Mock implementation would go here
            // For each event in result:
            //   prop_assert!(event.date >= start_date && event.date <= end_date);
        }
    }
}
```

#### Step 7: Integrate into Service

**File**: `src/service.rs`

```rust
use shared::api_clients::calendar::{CalendarEventsManager, IslamicCalendarClient};

pub struct ApiIntegrationService {
    // ... existing managers
    calendar_events_manager: CalendarEventsManager,
    // ...
}

impl ApiIntegrationService {
    pub async fn new(config: ServiceConfig) -> Result<Self> {
        // ... existing initialization
        
        // Initialize calendar events clients
        let calendar_clients: Vec<Box<dyn CalendarEventsApiClient>> = vec![
            Box::new(IslamicCalendarClient::new(
                config.apis.calendar_events.base_url.clone(),
                api_key_manager.get_key("islamic-calendar").ok(),
            )),
            // Add more clients as fallbacks
        ];
        
        let calendar_events_manager = CalendarEventsManager::new(
            calendar_clients,
            cache_manager.clone(),
            rate_limiter.clone(),
        );
        
        Ok(Self {
            // ... existing fields
            calendar_events_manager,
            // ...
        })
    }
    
    pub async fn get_islamic_events(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<IslamicEvent>> {
        self.calendar_events_manager
            .get_events(start_date, end_date)
            .await
            .map_err(|e| e.into())
    }
}
```

#### Step 8: Add HTTP Handler

**File**: `src/handlers.rs`

```rust
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetEventsRequest {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize)]
pub struct GetEventsResponse {
    pub events: Vec<IslamicEvent>,
}

pub async fn get_islamic_events(
    service: web::Data<ApiIntegrationService>,
    query: web::Query<GetEventsRequest>,
) -> Result<HttpResponse, ApiError> {
    // Parse dates
    let start_date = NaiveDate::parse_from_str(&query.start_date, "%Y-%m-%d")
        .map_err(|_| ApiError::ValidationError("Invalid start_date format".into()))?;
    
    let end_date = NaiveDate::parse_from_str(&query.end_date, "%Y-%m-%d")
        .map_err(|_| ApiError::ValidationError("Invalid end_date format".into()))?;
    
    // Validate date range
    if start_date > end_date {
        return Err(ApiError::ValidationError("start_date must be <= end_date".into()));
    }
    
    // Get events
    let events = service.get_islamic_events(start_date, end_date).await?;
    
    // Return response
    Ok(HttpResponse::Ok().json(ApiResponse::success(GetEventsResponse { events })))
}
```

#### Step 9: Add Route

**File**: `src/main.rs`

```rust
HttpServer::new(move || {
    App::new()
        .app_data(web::Data::new(service.clone()))
        .service(
            web::scope("/api/v1")
                // ... existing routes
                .route("/calendar/events", web::get().to(handlers::get_islamic_events))
        )
})
```

#### Step 10: Update Configuration

**File**: `config/api_integration_config.yaml`

```yaml
apis:
  calendar_events:
    - name: islamic-calendar
      base_url: https://api.islamic-calendar.com/v1
      priority: 1
      requires_key: false
      rate_limit:
        requests_per_minute: 60
        requests_per_hour: 1000
        requests_per_day: 10000
      timeout: 10s

cache:
  strategies:
    calendar_events:
      ttl: 7d
      allow_stale: true
      stale_ttl: 30d
```


## Testing Strategy

### Testing Pyramid

The service uses a comprehensive testing strategy with three levels:

```
        /\
       /  \
      / E2E\          Integration Tests (few)
     /______\
    /        \
   / Property\        Property-Based Tests (some)
  /__________\
 /            \
/  Unit Tests  \     Unit Tests (many)
/________________\
```

### Unit Tests

**Purpose**: Test specific examples and edge cases

**Location**: `src/tests/` and inline `#[cfg(test)]` modules

**Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_key_generation() {
        let request = QuranTextRequest {
            surah: 1,
            ayah: Some(1),
            translation: Some("en.sahih".to_string()),
            reciter: None,
        };
        
        let key = request.cache_key();
        assert_eq!(key, "quran:1:1:en.sahih");
    }
    
    #[tokio::test]
    async fn test_rate_limiter_allows_within_limit() {
        let rate_limiter = RateLimiter::new(redis_client);
        
        // First request should be allowed
        let allowed = rate_limiter.check("test-api").await.unwrap();
        assert!(allowed);
        
        // Increment counter
        rate_limiter.increment("test-api").await.unwrap();
    }
    
    #[tokio::test]
    async fn test_fallback_to_secondary_api() {
        // Mock primary API to fail
        let _m1 = mock("GET", "/quran/1/1")
            .with_status(500)
            .create();
        
        // Mock secondary API to succeed
        let _m2 = mock("GET", "/v1/ayah/1:1")
            .with_status(200)
            .with_body(r#"{"text":"..."}"#)
            .create();
        
        let manager = QuranApiManager::new(/* ... */);
        let result = manager.get_text(request).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().source, "secondary-api");
    }
}
```

**Running Unit Tests**:
```bash
# Run all unit tests
cargo test --lib

# Run specific test
cargo test test_cache_key_generation

# Run with output
cargo test -- --nocapture

# Run with specific number of threads
cargo test -- --test-threads=1
```

### Property-Based Tests

**Purpose**: Test universal properties across many inputs

**Library**: Proptest

**Location**: `src/property_tests/`

**Example**:
```rust
use proptest::prelude::*;

proptest! {
    // Property: Cache key should be deterministic
    #[test]
    fn cache_key_deterministic(
        surah in 1u8..=114,
        ayah in 1u16..300,
        translation in "[a-z]{2}\\.[a-z]+",
    ) {
        let request1 = QuranTextRequest {
            surah,
            ayah: Some(ayah),
            translation: Some(translation.clone()),
            reciter: None,
        };
        
        let request2 = QuranTextRequest {
            surah,
            ayah: Some(ayah),
            translation: Some(translation),
            reciter: None,
        };
        
        // Property: Same request should generate same cache key
        prop_assert_eq!(request1.cache_key(), request2.cache_key());
    }
    
    // Property: Rate limiter should never allow more than limit
    #[test]
    fn rate_limiter_enforces_limit(
        requests in 1usize..200,
    ) {
        // Make 'requests' number of requests
        // Property: Allowed requests should never exceed configured limit
        
        let limit = 100;
        let mut allowed_count = 0;
        
        for _ in 0..requests {
            if rate_limiter.check("test-api").await.unwrap() {
                allowed_count += 1;
                rate_limiter.increment("test-api").await.unwrap();
            }
        }
        
        prop_assert!(allowed_count <= limit);
    }
    
    // Property: Fallback should always try APIs in priority order
    #[test]
    fn fallback_respects_priority(
        num_clients in 2usize..5,
    ) {
        // Create clients with different priorities
        // Property: Clients should be tried in ascending priority order
        
        let mut tried_order = Vec::new();
        
        // Mock all clients to fail and record order
        // ...
        
        // Verify order matches priority
        for i in 1..tried_order.len() {
            prop_assert!(tried_order[i-1] <= tried_order[i]);
        }
    }
}
```

**Running Property Tests**:
```bash
# Run property tests
cargo test --lib property_tests

# Run with more iterations
PROPTEST_CASES=1000 cargo test property_tests

# Run with specific seed (for reproducibility)
PROPTEST_SEED=12345 cargo test property_tests
```

### Integration Tests

**Purpose**: Test end-to-end flows with real dependencies

**Location**: `tests/integration_tests.rs`

**Example**:
```rust
#[tokio::test]
#[ignore]  // Ignore by default, run explicitly
async fn test_quran_api_integration() {
    // This test requires real API keys and network access
    let service = ApiIntegrationService::new(test_config()).await.unwrap();
    
    let request = QuranTextRequest {
        surah: 1,
        ayah: Some(1),
        translation: Some("en.sahih".to_string()),
        reciter: None,
    };
    
    let response = service.get_quran_text(request).await.unwrap();
    
    assert_eq!(response.surah, 1);
    assert_eq!(response.ayah, 1);
    assert!(!response.text_arabic.is_empty());
    assert!(response.text_translation.is_some());
}

#[tokio::test]
async fn test_cache_integration() {
    let service = ApiIntegrationService::new(test_config()).await.unwrap();
    
    // First request (cache miss)
    let start = Instant::now();
    let response1 = service.get_quran_text(request.clone()).await.unwrap();
    let duration1 = start.elapsed();
    
    // Second request (cache hit)
    let start = Instant::now();
    let response2 = service.get_quran_text(request).await.unwrap();
    let duration2 = start.elapsed();
    
    // Cache hit should be significantly faster
    assert!(duration2 < duration1 / 2);
    assert_eq!(response1.text_arabic, response2.text_arabic);
}
```

**Running Integration Tests**:
```bash
# Run integration tests (requires dependencies)
cargo test --test integration_tests

# Run ignored tests (requires API keys)
cargo test --test integration_tests -- --ignored

# Run all tests including ignored
cargo test --test integration_tests -- --include-ignored
```

### Test Coverage

**Measure coverage**:
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# View report
open coverage/index.html
```

**Coverage Goals**:
- Overall: 80% minimum
- Critical paths: 95% minimum
- Error handling: 90% minimum

## Code Style and Standards

### Rust Style Guide

Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

**Formatting**:
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

**Linting**:
```bash
# Run clippy
cargo clippy

# Run clippy with all features
cargo clippy --all-features

# Deny warnings
cargo clippy -- -D warnings
```

### Naming Conventions

**Modules**: `snake_case`
```rust
mod api_clients;
mod rate_limiter;
```

**Structs/Enums**: `PascalCase`
```rust
struct QuranApiClient;
enum ApiError;
```

**Functions/Variables**: `snake_case`
```rust
fn get_quran_text() -> Result<QuranTextResponse>;
let cache_key = request.cache_key();
```

**Constants**: `SCREAMING_SNAKE_CASE`
```rust
const MAX_RETRY_ATTEMPTS: u32 = 3;
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);
```

**Traits**: `PascalCase` (often with `able` suffix)
```rust
trait ApiClient;
trait Cacheable;
```

### Documentation

**Module documentation**:
```rust
//! This module provides API clients for Quran data sources.
//!
//! It includes clients for:
//! - Quran.com API
//! - AlQuran Cloud API
//! - Tanzil API
//! - EveryAyah API
```

**Function documentation**:
```rust
/// Retrieves Quran text for a specific verse.
///
/// # Arguments
///
/// * `surah` - Surah number (1-114)
/// * `ayah` - Ayah number within the surah
/// * `translation` - Optional translation identifier
///
/// # Returns
///
/// Returns `QuranTextResponse` containing the Arabic text and optional translation.
///
/// # Errors
///
/// Returns `ApiError` if:
/// - Surah or ayah number is invalid
/// - All API clients fail
/// - Network error occurs
///
/// # Examples
///
/// ```
/// let request = QuranTextRequest {
///     surah: 1,
///     ayah: Some(1),
///     translation: Some("en.sahih".to_string()),
///     reciter: None,
/// };
///
/// let response = client.get_text(request).await?;
/// println!("{}", response.text_arabic);
/// ```
pub async fn get_text(&self, request: QuranTextRequest) -> Result<QuranTextResponse, ApiError> {
    // Implementation
}
```

### Error Handling

**Use Result types**:
```rust
// Good
pub async fn get_data(&self) -> Result<Data, ApiError> {
    // ...
}

// Bad - don't panic in library code
pub async fn get_data(&self) -> Data {
    // ...
    .unwrap()  // Don't do this!
}
```

**Provide context**:
```rust
// Good
.map_err(|e| ApiError::Network(format!("Failed to connect to {}: {}", api_name, e)))?

// Bad
.map_err(|e| ApiError::Network(e.to_string()))?
```

**Use custom error types**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Rate limit exceeded for API: {0}")]
    RateLimitExceeded(String),
    
    #[error("Invalid response from API {0}: {1}")]
    InvalidResponse(String, String),
}
```

### Logging

**Use structured logging**:
```rust
use tracing::{info, warn, error, debug};

// Good - structured
info!(
    api = "quran.com",
    surah = 1,
    ayah = 1,
    cache_status = "HIT",
    "Quran text request completed"
);

// Bad - unstructured
println!("Request completed for surah 1 ayah 1");
```

**Log levels**:
- `error!`: Errors that require immediate attention
- `warn!`: Warnings that should be investigated
- `info!`: Important informational messages
- `debug!`: Detailed debugging information
- `trace!`: Very detailed tracing information


## Debugging and Troubleshooting

### Debug Logging

**Enable debug logging**:
```bash
# Set log level
export LOG_LEVEL=debug

# Or in .env
LOG_LEVEL=debug

# Run service
cargo run
```

**Filter logs by module**:
```bash
# Only show logs from api_clients module
RUST_LOG=api_integration_service::api_clients=debug cargo run

# Multiple modules
RUST_LOG=api_integration_service::api_clients=debug,api_integration_service::handlers=info cargo run
```

### Using the Debugger

**VS Code** (with CodeLLDB extension):

**.vscode/launch.json**:
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug API Integration Service",
      "cargo": {
        "args": [
          "build",
          "--bin=api-integration-service",
          "--package=api-integration-service"
        ],
        "filter": {
          "name": "api-integration-service",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}",
      "env": {
        "RUST_LOG": "debug",
        "REDIS_URL": "redis://localhost:6379",
        "POSTGRES_URL": "postgresql://postgres:postgres@localhost:5432/sanad"
      }
    }
  ]
}
```

**IntelliJ IDEA**:
1. Right-click on `main.rs`
2. Select "Debug 'api-integration-service'"
3. Set breakpoints by clicking in the gutter

### Common Issues and Solutions

#### Issue: "Connection refused" to Redis/PostgreSQL

**Diagnosis**:
```bash
# Check if services are running
docker-compose ps

# Check logs
docker-compose logs redis
docker-compose logs postgres
```

**Solution**:
```bash
# Start services
docker-compose up -d redis postgres

# Verify connectivity
redis-cli -h localhost ping
psql -h localhost -U postgres -d sanad -c "SELECT 1"
```

#### Issue: "API key not found"

**Diagnosis**:
```bash
# Check environment variables
env | grep API_KEY

# Check .env file
cat .env
```

**Solution**:
```bash
# Set API keys in .env
echo "SUNNAH_COM_API_KEY=your_key" >> .env
echo "HUGGING_FACE_API_KEY=your_key" >> .env

# Restart service
cargo run
```

#### Issue: High memory usage

**Diagnosis**:
```bash
# Monitor memory usage
cargo build --release
valgrind --tool=massif ./target/release/api-integration-service

# Or use heaptrack
heaptrack ./target/release/api-integration-service
```

**Solution**:
- Reduce cache size
- Reduce connection pool sizes
- Check for memory leaks
- Use `Arc` instead of `Clone` for large data

#### Issue: Slow response times

**Diagnosis**:
```bash
# Enable tracing
RUST_LOG=trace cargo run

# Use flamegraph for profiling
cargo install flamegraph
cargo flamegraph --bin api-integration-service
```

**Solution**:
- Check external API response times
- Optimize database queries
- Increase cache hit rate
- Use connection pooling

### Performance Profiling

**CPU Profiling**:
```bash
# Install perf (Linux)
sudo apt-get install linux-tools-common

# Profile the service
perf record -g ./target/release/api-integration-service

# View results
perf report
```

**Flamegraph**:
```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --bin api-integration-service

# Open flamegraph.svg in browser
```

**Benchmarking**:
```rust
// benches/api_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_cache_key_generation(c: &mut Criterion) {
    c.bench_function("cache_key_generation", |b| {
        b.iter(|| {
            let request = QuranTextRequest {
                surah: black_box(1),
                ayah: Some(black_box(1)),
                translation: Some(black_box("en.sahih".to_string())),
                reciter: None,
            };
            request.cache_key()
        });
    });
}

criterion_group!(benches, benchmark_cache_key_generation);
criterion_main!(benches);
```

```bash
# Run benchmarks
cargo bench
```

## Performance Optimization

### Caching Strategies

**1. Aggressive Caching for Static Data**:
```rust
// Quran text never changes - cache for 30 days
cache.set(&key, &data, Duration::from_secs(30 * 24 * 3600)).await?;
```

**2. Short TTL for Dynamic Data**:
```rust
// Prayer times change daily - cache for 1 day
cache.set(&key, &data, Duration::from_secs(24 * 3600)).await?;
```

**3. Stale Cache as Fallback**:
```rust
// Serve stale cache if all APIs fail
if let Some(stale) = cache.get_expired(&key).await? {
    return Ok(stale);
}
```

### Connection Pooling

**Redis Connection Pool**:
```rust
let redis_pool = RedisPool::builder()
    .max_size(10)
    .min_idle(Some(2))
    .connection_timeout(Duration::from_secs(5))
    .build(redis_url)?;
```

**PostgreSQL Connection Pool**:
```rust
let pg_pool = PgPoolOptions::new()
    .max_connections(20)
    .min_connections(5)
    .acquire_timeout(Duration::from_secs(10))
    .connect(&database_url)
    .await?;
```

### Async Best Practices

**1. Use `tokio::spawn` for concurrent operations**:
```rust
// Bad - sequential
let result1 = api1.request().await?;
let result2 = api2.request().await?;

// Good - concurrent
let (result1, result2) = tokio::join!(
    api1.request(),
    api2.request()
);
```

**2. Use `tokio::select!` for racing operations**:
```rust
tokio::select! {
    result = primary_api.request() => {
        // Use primary result
    }
    result = secondary_api.request() => {
        // Use secondary result
    }
    _ = tokio::time::sleep(Duration::from_secs(5)) => {
        // Timeout
    }
}
```

**3. Avoid blocking operations**:
```rust
// Bad - blocks async runtime
std::thread::sleep(Duration::from_secs(1));

// Good - async sleep
tokio::time::sleep(Duration::from_secs(1)).await;
```

### Memory Optimization

**1. Use `Arc` for shared data**:
```rust
// Good - shared ownership without cloning
let cache = Arc::new(CacheManager::new(redis));
let service1 = Service::new(cache.clone());
let service2 = Service::new(cache.clone());
```

**2. Use streaming for large responses**:
```rust
// Good - stream large responses
let stream = response.bytes_stream();
while let Some(chunk) = stream.next().await {
    process_chunk(chunk?);
}
```

**3. Limit cache size**:
```rust
// Implement LRU eviction
if cache.size() > MAX_CACHE_SIZE {
    cache.evict_lru().await?;
}
```

## Contributing Guidelines

### Workflow

1. **Create a branch**:
```bash
git checkout -b feature/add-new-api-client
```

2. **Make changes**:
- Write code
- Add tests
- Update documentation

3. **Run checks**:
```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Run tests
cargo test

# Check documentation
cargo doc --no-deps --open
```

4. **Commit changes**:
```bash
git add .
git commit -m "feat: add new API client for Islamic Calendar"
```

5. **Push and create PR**:
```bash
git push origin feature/add-new-api-client
# Create pull request on GitHub
```

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(quran): add Tanzil API client

Implement Tanzil API client as tertiary fallback for Quran text.
Includes unit tests and property tests.

Closes #123
```

```
fix(cache): fix cache key collision for similar requests

Cache keys were not unique for requests with different translations.
Added translation to cache key generation.

Fixes #456
```

### Pull Request Checklist

- [ ] Code follows style guidelines
- [ ] All tests pass
- [ ] New tests added for new functionality
- [ ] Documentation updated
- [ ] Commit messages follow convention
- [ ] No merge conflicts
- [ ] CI/CD pipeline passes

### Code Review Guidelines

**For Reviewers**:
- Check code quality and style
- Verify tests are comprehensive
- Ensure documentation is clear
- Look for potential bugs or edge cases
- Suggest improvements

**For Authors**:
- Respond to all comments
- Make requested changes
- Re-request review after changes
- Be open to feedback

## Common Patterns

### Pattern 1: API Client with Fallback

```rust
pub async fn get_data(&self, request: Request) -> Result<Response> {
    // 1. Check cache
    if let Some(cached) = self.cache.get(&request.cache_key()).await? {
        return Ok(cached);
    }
    
    // 2. Try each client in priority order
    for client in &self.clients {
        if !client.is_healthy().await {
            continue;
        }
        
        if !self.rate_limiter.check(client.api_name()).await? {
            continue;
        }
        
        match client.request(request.clone()).await {
            Ok(response) => {
                self.cache.set(&request.cache_key(), &response, TTL).await?;
                return Ok(response);
            }
            Err(e) => {
                log::warn!("Client {} failed: {}", client.api_name(), e);
                continue;
            }
        }
    }
    
    // 3. Try stale cache
    if let Some(stale) = self.cache.get_expired(&request.cache_key()).await? {
        return Ok(stale);
    }
    
    // 4. All failed
    Err(ApiError::AllApisFailed)
}
```

### Pattern 2: Retry with Exponential Backoff

```rust
pub async fn request_with_retry<F, T>(&self, f: F) -> Result<T>
where
    F: Fn() -> BoxFuture<'static, Result<T>>,
{
    let mut attempt = 0;
    let mut delay = self.initial_delay;
    
    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt >= self.max_attempts => return Err(e),
            Err(e) if !e.is_retryable() => return Err(e),
            Err(_) => {
                attempt += 1;
                tokio::time::sleep(delay).await;
                delay = (delay * 2).min(self.max_delay);
            }
        }
    }
}
```

### Pattern 3: Circuit Breaker

```rust
pub struct CircuitBreaker {
    state: Arc<RwLock<CircuitState>>,
    failure_threshold: u32,
    success_threshold: u32,
    timeout: Duration,
}

enum CircuitState {
    Closed { failures: u32 },
    Open { opened_at: Instant },
    HalfOpen { successes: u32 },
}

impl CircuitBreaker {
    pub async fn call<F, T>(&self, f: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        // Check state
        let state = self.state.read().await;
        match *state {
            CircuitState::Open { opened_at } => {
                if opened_at.elapsed() > self.timeout {
                    drop(state);
                    self.transition_to_half_open().await;
                } else {
                    return Err(ApiError::CircuitOpen);
                }
            }
            _ => {}
        }
        drop(state);
        
        // Execute function
        match f.await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(e) => {
                self.on_failure().await;
                Err(e)
            }
        }
    }
}
```

---

## Additional Resources

### Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Actix-web Documentation](https://actix.rs/docs/)

### Tools
- [Rust Analyzer](https://rust-analyzer.github.io/)
- [Clippy](https://github.com/rust-lang/rust-clippy)
- [Rustfmt](https://github.com/rust-lang/rustfmt)
- [Cargo Watch](https://github.com/watchexec/cargo-watch)

### Community
- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [r/rust](https://www.reddit.com/r/rust/)

---

**Last Updated**: 2024-01-15  
**Version**: 1.0.0  
**Maintained by**: Sanad Development Team


# Design Document: Official Islamic APIs Integration

## Overview

يصف هذا المستند التصميم الشامل لنظام تكامل الـ APIs الإسلامية الرسمية في مشروع Sanad. النظام مبني على معمارية microservices باستخدام Rust، ويوفر طبقة تجريد موحدة للوصول إلى APIs متعددة مع دعم كامل للـ caching، rate limiting، error handling، وآليات fallback.

سيتم تنظيم النظام في خدمة مركزية (`api-integration-service`) تدير جميع الاتصالات بالـ APIs الخارجية، مع مكتبة مشتركة (`shared/api_clients`) يمكن استخدامها من قبل الخدمات الأخرى.

## Verified Official API Sources

**CRITICAL**: All API sources have been verified for authenticity and official status. See requirements.md for detailed verification information.

### Primary APIs to Integrate

1. **Quran APIs**:
   - Quran.com / Quran Foundation API (Primary) - ✅ Official
   - Tanzil.net (Secondary) - ✅ Official verified text
   - AlQuran Cloud API (Tertiary) - ✅ Community trusted
   - EveryAyah.com (Audio) - ✅ Verified reciters
   - IslamHouse QuranEnc.com - ✅ Officially supervised

2. **Hadith APIs**:
   - Sunnah.com (Primary) - ✅ Official with authenticated chains
   - IslamHouse HadeethEnc.com - ✅ Officially supervised

3. **Prayer Times & Qibla**:
   - AlAdhan API (Primary) - ✅ Official Islamic Network
   - Islamic Finder (Secondary) - ✅ Widely trusted

4. **Tafsir**:
   - Quran.com Tafsir API - ✅ Official Quran Foundation

5. **Calendar**:
   - AlAdhan Hijri Calendar - ✅ Official
   - Islamic Finder Calendar - ✅ Verified

6. **AI/NLP** (Technical processing only):
   - Hugging Face Arabic Models - ✅ For language processing only
   - Note: NOT used for Islamic rulings or fatwas

## Architecture

### High-Level Architecture

```mermaid
graph TB
    subgraph "Frontend Layer"
        Mobile[Mobile App - Flutter]
        Web[Web App - Next.js]
    end
    
    subgraph "Gateway Layer"
        Gateway[API Gateway]
    end
    
    subgraph "Integration Service"
        IntegrationService[API Integration Service]
        APIRouter[API Router]
        RateLimiter[Rate Limiter]
        CacheManager[Cache Manager]
        HealthMonitor[Health Monitor]
    end
    
    subgraph "API Clients"
        QuranClients[Quran API Clients]
        HadithClients[Hadith API Clients]
        PrayerClients[Prayer Times Clients]
        TafsirClients[Tafsir Clients]
        CalendarClients[Calendar Clients]
        QiblaClients[Qibla Clients]
        AIClients[AI/NLP Clients]
    end
    
    subgraph "External APIs"
        QuranAPIs[Quran.com, Alquran Cloud, Tanzil, Everyayah]
        HadithAPIs[Sunnah.com, Hadith API, Aladhan]
        PrayerAPIs[Aladhan, Islamic Finder]
        TafsirAPIs[Quran.com Tafsir, Others]
        CalendarAPIs[Aladhan Calendar, Islamic Finder]
        QiblaAPIs[Aladhan Qibla, Islamic Finder]
        AIAPIs[Hugging Face, OpenAI]
    end
    
    subgraph "Storage Layer"
        Redis[(Redis Cache)]
        Postgres[(PostgreSQL)]
    end
    
    Mobile --> Gateway
    Web --> Gateway
    Gateway --> IntegrationService
    IntegrationService --> APIRouter
    APIRouter --> RateLimiter
    RateLimiter --> CacheManager
    CacheManager --> QuranClients
    CacheManager --> HadithClients
    CacheManager --> PrayerClients
    CacheManager --> TafsirClients
    CacheManager --> CalendarClients
    CacheManager --> QiblaClients
    CacheManager --> AIClients
    
    QuranClients --> QuranAPIs
    HadithClients --> HadithAPIs
    PrayerClients --> PrayerAPIs
    TafsirClients --> TafsirAPIs
    CalendarClients --> CalendarAPIs
    QiblaClients --> QiblaAPIs
    AIClients --> AIAPIs
    
    CacheManager --> Redis
    IntegrationService --> Postgres
    HealthMonitor --> QuranAPIs
    HealthMonitor --> HadithAPIs
    HealthMonitor --> PrayerAPIs
```

### Service Architecture

النظام يتكون من الطبقات التالية:

1. **API Router Layer**: توجيه الطلبات إلى API clients المناسبة
2. **Rate Limiting Layer**: التحكم في معدل الطلبات لكل API
3. **Caching Layer**: تخزين مؤقت ذكي للاستجابات
4. **Client Layer**: عملاء HTTP لكل API خارجي
5. **Fallback Layer**: آليات بديلة عند فشل APIs
6. **Monitoring Layer**: مراقبة صحة وأداء APIs

## Components and Interfaces

### 1. API Integration Service

الخدمة الرئيسية التي تنسق جميع عمليات التكامل.

```rust
// services/api-integration-service/src/service.rs

pub struct ApiIntegrationService {
    quran_manager: QuranApiManager,
    hadith_manager: HadithApiManager,
    prayer_manager: PrayerTimesApiManager,
    tafsir_manager: TafsirApiManager,
    calendar_manager: CalendarApiManager,
    qibla_manager: QiblaApiManager,
    ai_manager: AiApiManager,
    cache_manager: Arc<CacheManager>,
    rate_limiter: Arc<RateLimiter>,
    health_monitor: Arc<HealthMonitor>,
    config: ServiceConfig,
}

impl ApiIntegrationService {
    pub async fn new(config: ServiceConfig) -> Result<Self>;
    
    // Quran operations
    pub async fn get_quran_text(&self, request: QuranTextRequest) -> Result<QuranTextResponse>;
    pub async fn get_quran_audio(&self, request: QuranAudioRequest) -> Result<QuranAudioResponse>;
    
    // Hadith operations
    pub async fn search_hadith(&self, request: HadithSearchRequest) -> Result<HadithSearchResponse>;
    pub async fn get_hadith_by_id(&self, request: HadithByIdRequest) -> Result<HadithResponse>;
    
    // Prayer times operations
    pub async fn get_prayer_times(&self, request: PrayerTimesRequest) -> Result<PrayerTimesResponse>;
    
    // Tafsir operations
    pub async fn get_tafsir(&self, request: TafsirRequest) -> Result<TafsirResponse>;
    
    // Calendar operations
    pub async fn convert_date(&self, request: DateConversionRequest) -> Result<DateConversionResponse>;
    pub async fn get_islamic_events(&self, request: IslamicEventsRequest) -> Result<IslamicEventsResponse>;
    
    // Qibla operations
    pub async fn get_qibla_direction(&self, request: QiblaRequest) -> Result<QiblaResponse>;
    
    // AI operations
    pub async fn process_ai_query(&self, request: AiQueryRequest) -> Result<AiQueryResponse>;
    
    // Health check
    pub async fn health_check(&self) -> HealthStatus;
}
```

### 2. API Client Trait

واجهة موحدة لجميع API clients.

```rust
// shared/src/api_clients/mod.rs

#[async_trait]
pub trait ApiClient: Send + Sync {
    type Request;
    type Response;
    
    /// Get the API name for logging and monitoring
    fn api_name(&self) -> &str;
    
    /// Get the priority level (lower is higher priority)
    fn priority(&self) -> u8;
    
    /// Check if the API is currently healthy
    async fn is_healthy(&self) -> bool;
    
    /// Make a request to the API
    async fn request(&self, req: Self::Request) -> Result<Self::Response, ApiError>;
    
    /// Get rate limit information
    fn rate_limit(&self) -> RateLimitConfig;
}

pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub requests_per_day: u32,
}
```

### 3. Quran API Manager

إدارة جميع Quran APIs مع fallback.

```rust
// shared/src/api_clients/quran/manager.rs

pub struct QuranApiManager {
    clients: Vec<Box<dyn QuranApiClient>>,
    cache: Arc<CacheManager>,
    rate_limiter: Arc<RateLimiter>,
}

#[async_trait]
pub trait QuranApiClient: ApiClient<Request = QuranTextRequest, Response = QuranTextResponse> {
    async fn get_surah(&self, surah_number: u8) -> Result<SurahData>;
    async fn get_ayah(&self, surah: u8, ayah: u16) -> Result<AyahData>;
    async fn get_page(&self, page: u16) -> Result<PageData>;
}

// Implementations for each API
pub struct QuranComClient { /* ... */ }
pub struct AlquranCloudClient { /* ... */ }
pub struct TanzilClient { /* ... */ }
pub struct EveryayahClient { /* ... */ }

impl QuranApiManager {
    pub async fn get_text(&self, request: QuranTextRequest) -> Result<QuranTextResponse> {
        // 1. Check cache
        if let Some(cached) = self.cache.get(&request.cache_key()).await? {
            return Ok(cached);
        }
        
        // 2. Try primary API
        for client in &self.clients {
            if !client.is_healthy().await {
                continue;
            }
            
            // Check rate limit
            if !self.rate_limiter.check(client.api_name()).await? {
                continue;
            }
            
            match client.request(request.clone()).await {
                Ok(response) => {
                    // Cache the response
                    self.cache.set(&request.cache_key(), &response, Duration::from_secs(86400)).await?;
                    return Ok(response);
                }
                Err(e) => {
                    log::warn!("API {} failed: {}", client.api_name(), e);
                    continue;
                }
            }
        }
        
        // 3. All APIs failed, try expired cache
        if let Some(cached) = self.cache.get_expired(&request.cache_key()).await? {
            log::warn!("Serving expired cache for Quran request");
            return Ok(cached);
        }
        
        Err(ApiError::AllApisFailed)
    }
}
```

### 4. Rate Limiter

نظام التحكم في معدل الطلبات.

```rust
// shared/src/rate_limiter.rs

pub struct RateLimiter {
    redis: Arc<RedisClient>,
    configs: HashMap<String, RateLimitConfig>,
}

impl RateLimiter {
    pub async fn check(&self, api_name: &str) -> Result<bool> {
        let config = self.configs.get(api_name)
            .ok_or(ApiError::UnknownApi)?;
        
        let now = SystemTime::now();
        let minute_key = format!("ratelimit:{}:minute:{}", api_name, now.minute());
        let hour_key = format!("ratelimit:{}:hour:{}", api_name, now.hour());
        let day_key = format!("ratelimit:{}:day:{}", api_name, now.day());
        
        // Check all time windows
        let minute_count: u32 = self.redis.get(&minute_key).await?.unwrap_or(0);
        let hour_count: u32 = self.redis.get(&hour_key).await?.unwrap_or(0);
        let day_count: u32 = self.redis.get(&day_key).await?.unwrap_or(0);
        
        if minute_count >= config.requests_per_minute {
            return Ok(false);
        }
        if hour_count >= config.requests_per_hour {
            return Ok(false);
        }
        if day_count >= config.requests_per_day {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    pub async fn increment(&self, api_name: &str) -> Result<()> {
        let now = SystemTime::now();
        let minute_key = format!("ratelimit:{}:minute:{}", api_name, now.minute());
        let hour_key = format!("ratelimit:{}:hour:{}", api_name, now.hour());
        let day_key = format!("ratelimit:{}:day:{}", api_name, now.day());
        
        // Increment all counters with appropriate TTL
        self.redis.incr_with_ttl(&minute_key, 60).await?;
        self.redis.incr_with_ttl(&hour_key, 3600).await?;
        self.redis.incr_with_ttl(&day_key, 86400).await?;
        
        Ok(())
    }
}
```

### 5. Cache Manager

نظام التخزين المؤقت الذكي.

```rust
// shared/src/cache_manager.rs

pub struct CacheManager {
    redis: Arc<RedisClient>,
    strategies: HashMap<CacheCategory, CacheStrategy>,
}

pub enum CacheCategory {
    QuranText,      // Static, long TTL
    QuranAudio,     // Static, long TTL
    Hadith,         // Static, long TTL
    PrayerTimes,    // Dynamic, daily TTL
    Tafsir,         // Static, long TTL
    Calendar,       // Semi-static, weekly TTL
    Qibla,          // Static per location, long TTL
    AiResponse,     // Dynamic, short TTL
}

pub struct CacheStrategy {
    pub ttl: Duration,
    pub allow_stale: bool,
    pub stale_ttl: Duration,
}

impl CacheManager {
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let value: Option<String> = self.redis.get(key).await?;
        match value {
            Some(v) => Ok(Some(serde_json::from_str(&v)?)),
            None => Ok(None),
        }
    }
    
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<()> {
        let serialized = serde_json::to_string(value)?;
        self.redis.set_with_ttl(key, &serialized, ttl.as_secs() as usize).await?;
        Ok(())
    }
    
    pub async fn get_expired<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let stale_key = format!("{}:stale", key);
        let value: Option<String> = self.redis.get(&stale_key).await?;
        match value {
            Some(v) => Ok(Some(serde_json::from_str(&v)?)),
            None => Ok(None),
        }
    }
    
    pub async fn set_with_stale<T: Serialize>(&self, key: &str, value: &T, strategy: &CacheStrategy) -> Result<()> {
        let serialized = serde_json::to_string(value)?;
        
        // Set primary cache
        self.redis.set_with_ttl(key, &serialized, strategy.ttl.as_secs() as usize).await?;
        
        // Set stale cache if allowed
        if strategy.allow_stale {
            let stale_key = format!("{}:stale", key);
            self.redis.set_with_ttl(&stale_key, &serialized, strategy.stale_ttl.as_secs() as usize).await?;
        }
        
        Ok(())
    }
}
```

### 6. Health Monitor

مراقبة صحة جميع APIs.

```rust
// shared/src/health_monitor.rs

pub struct HealthMonitor {
    api_status: Arc<RwLock<HashMap<String, ApiHealthStatus>>>,
    redis: Arc<RedisClient>,
    check_interval: Duration,
}

pub struct ApiHealthStatus {
    pub api_name: String,
    pub is_healthy: bool,
    pub last_check: SystemTime,
    pub last_success: Option<SystemTime>,
    pub last_failure: Option<SystemTime>,
    pub success_rate: f64,
    pub avg_response_time: Duration,
    pub consecutive_failures: u32,
}

impl HealthMonitor {
    pub async fn start_monitoring(&self) {
        loop {
            self.check_all_apis().await;
            tokio::time::sleep(self.check_interval).await;
        }
    }
    
    async fn check_all_apis(&self) {
        // Check each API category
        self.check_quran_apis().await;
        self.check_hadith_apis().await;
        self.check_prayer_apis().await;
        self.check_tafsir_apis().await;
        self.check_calendar_apis().await;
        self.check_qibla_apis().await;
        self.check_ai_apis().await;
    }
    
    async fn check_api(&self, client: &dyn ApiClient) -> bool {
        let start = Instant::now();
        let is_healthy = client.is_healthy().await;
        let duration = start.elapsed();
        
        let mut status_map = self.api_status.write().await;
        let status = status_map.entry(client.api_name().to_string())
            .or_insert_with(|| ApiHealthStatus::new(client.api_name()));
        
        status.last_check = SystemTime::now();
        status.is_healthy = is_healthy;
        
        if is_healthy {
            status.last_success = Some(SystemTime::now());
            status.consecutive_failures = 0;
            status.update_response_time(duration);
        } else {
            status.last_failure = Some(SystemTime::now());
            status.consecutive_failures += 1;
        }
        
        status.update_success_rate();
        
        // Persist to Redis
        self.persist_status(status).await;
        
        is_healthy
    }
    
    pub async fn get_status(&self, api_name: &str) -> Option<ApiHealthStatus> {
        let status_map = self.api_status.read().await;
        status_map.get(api_name).cloned()
    }
    
    pub async fn get_all_status(&self) -> HashMap<String, ApiHealthStatus> {
        let status_map = self.api_status.read().await;
        status_map.clone()
    }
}
```

### 7. API Key Manager

إدارة آمنة لمفاتيح APIs.

```rust
// shared/src/api_key_manager.rs

pub struct ApiKeyManager {
    keys: Arc<RwLock<HashMap<String, ApiKey>>>,
    secrets_client: Option<SecretsManagerClient>,
}

pub struct ApiKey {
    pub api_name: String,
    pub key: String,
    pub key_type: ApiKeyType,
    pub created_at: SystemTime,
    pub expires_at: Option<SystemTime>,
    pub is_active: bool,
}

pub enum ApiKeyType {
    Header(String),      // e.g., "X-API-Key"
    QueryParam(String),  // e.g., "api_key"
    Bearer,              // Authorization: Bearer <token>
    Basic(String),       // Basic auth with username
}

impl ApiKeyManager {
    pub async fn load_keys(&mut self) -> Result<()> {
        // Load from environment variables
        self.load_from_env()?;
        
        // Load from secrets manager if configured
        if let Some(client) = &self.secrets_client {
            self.load_from_secrets_manager(client).await?;
        }
        
        Ok(())
    }
    
    pub fn get_key(&self, api_name: &str) -> Result<ApiKey> {
        let keys = self.keys.read().unwrap();
        keys.get(api_name)
            .cloned()
            .ok_or(ApiError::ApiKeyNotFound(api_name.to_string()))
    }
    
    pub fn inject_key(&self, api_name: &str, request: &mut reqwest::Request) -> Result<()> {
        let key = self.get_key(api_name)?;
        
        if !key.is_active {
            return Err(ApiError::ApiKeyInactive(api_name.to_string()));
        }
        
        if let Some(expires_at) = key.expires_at {
            if SystemTime::now() > expires_at {
                return Err(ApiError::ApiKeyExpired(api_name.to_string()));
            }
        }
        
        match key.key_type {
            ApiKeyType::Header(ref header_name) => {
                request.headers_mut().insert(
                    header_name.parse().unwrap(),
                    key.key.parse().unwrap()
                );
            }
            ApiKeyType::QueryParam(ref param_name) => {
                let url = request.url_mut();
                url.query_pairs_mut().append_pair(param_name, &key.key);
            }
            ApiKeyType::Bearer => {
                request.headers_mut().insert(
                    "Authorization",
                    format!("Bearer {}", key.key).parse().unwrap()
                );
            }
            ApiKeyType::Basic(ref username) => {
                let credentials = base64::encode(format!("{}:{}", username, key.key));
                request.headers_mut().insert(
                    "Authorization",
                    format!("Basic {}", credentials).parse().unwrap()
                );
            }
        }
        
        Ok(())
    }
}
```

## Data Models

### Request/Response Models

```rust
// shared/src/api_clients/models.rs

// Quran Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuranTextRequest {
    pub surah: u8,
    pub ayah: Option<u16>,
    pub translation: Option<String>,
    pub reciter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuranTextResponse {
    pub surah: u8,
    pub ayah: u16,
    pub text_arabic: String,
    pub text_translation: Option<String>,
    pub audio_url: Option<String>,
    pub source: String,
}

// Hadith Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HadithSearchRequest {
    pub query: String,
    pub collection: Option<String>,
    pub book: Option<String>,
    pub language: String,
    pub limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HadithSearchResponse {
    pub results: Vec<HadithResult>,
    pub total: usize,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HadithResult {
    pub id: String,
    pub collection: String,
    pub book: String,
    pub hadith_number: String,
    pub text_arabic: String,
    pub text_translation: Option<String>,
    pub grade: Option<String>,
    pub narrator: String,
    pub source: String,
}

// Prayer Times Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerTimesRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub date: NaiveDate,
    pub calculation_method: CalculationMethod,
    pub madhab: Madhab,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CalculationMethod {
    MWL,           // Muslim World League
    ISNA,          // Islamic Society of North America
    Egypt,         // Egyptian General Authority of Survey
    Makkah,        // Umm Al-Qura University, Makkah
    Karachi,       // University of Islamic Sciences, Karachi
    Tehran,        // Institute of Geophysics, University of Tehran
    Jafari,        // Shia Ithna-Ashari, Leva Institute, Qum
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Madhab {
    Shafi,
    Hanafi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerTimesResponse {
    pub date: NaiveDate,
    pub fajr: NaiveTime,
    pub sunrise: NaiveTime,
    pub dhuhr: NaiveTime,
    pub asr: NaiveTime,
    pub maghrib: NaiveTime,
    pub isha: NaiveTime,
    pub source: String,
}

// Tafsir Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TafsirRequest {
    pub surah: u8,
    pub ayah: u16,
    pub tafsir_id: Option<String>,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TafsirResponse {
    pub surah: u8,
    pub ayah: u16,
    pub tafsirs: Vec<TafsirEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TafsirEntry {
    pub tafsir_id: String,
    pub tafsir_name: String,
    pub scholar: String,
    pub text: String,
    pub language: String,
    pub source: String,
}

// Calendar Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateConversionRequest {
    pub date: NaiveDate,
    pub direction: ConversionDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversionDirection {
    GregorianToHijri,
    HijriToGregorian,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateConversionResponse {
    pub gregorian: NaiveDate,
    pub hijri: HijriDate,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HijriDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub month_name_ar: String,
    pub month_name_en: String,
}

// Qibla Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QiblaRequest {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QiblaResponse {
    pub direction: f64,  // Degrees from North
    pub distance_km: f64,
    pub source: String,
}

// AI Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiQueryRequest {
    pub query: String,
    pub context: Option<String>,
    pub language: String,
    pub max_tokens: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiQueryResponse {
    pub response: String,
    pub sources: Vec<String>,
    pub confidence: f64,
    pub model: String,
}
```

### Error Models

```rust
// shared/src/api_clients/error.rs

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Rate limit exceeded for API: {0}")]
    RateLimitExceeded(String),
    
    #[error("API key not found: {0}")]
    ApiKeyNotFound(String),
    
    #[error("API key inactive: {0}")]
    ApiKeyInactive(String),
    
    #[error("API key expired: {0}")]
    ApiKeyExpired(String),
    
    #[error("Invalid response from API {0}: {1}")]
    InvalidResponse(String, String),
    
    #[error("API {0} returned error: {1}")]
    ApiError(String, String),
    
    #[error("All APIs failed for request")]
    AllApisFailed,
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Unknown API: {0}")]
    UnknownApi(String),
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*



### Property 1: API Client Initialization Completeness
*For any* API category (Quran, Hadith, Prayer Times, Tafsir, Calendar, Qibla, AI), when the Integration_Service initializes, all configured clients for that category should be present and properly configured with their respective endpoints and authentication.

**Validates: Requirements 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1, 8.1**

### Property 2: Fallback Chain Execution
*For any* API request, if the primary API fails, the system should attempt secondary APIs in priority order until one succeeds or all fail, and each attempt should be logged.

**Validates: Requirements 1.2, 3.3, 6.4, 11.4, 12.1**

### Property 3: Response Validation Consistency
*For any* API response, the Response_Validator should verify that the response structure matches the expected schema and that all required fields are present and valid before returning to the caller.

**Validates: Requirements 1.4, 2.4, 3.4, 4.4, 5.4, 6.3**

### Property 4: Cache Storage with Appropriate TTL
*For any* validated API response, the Cache_Manager should store it with a TTL appropriate to the data type (static data gets longer TTL, dynamic data gets shorter TTL), and the cache key should be deterministic based on request parameters.

**Validates: Requirements 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5**

### Property 5: Parallel API Querying
*For any* hadith search request, the system should query all configured hadith APIs in parallel (not sequentially), and the total time should not exceed the slowest individual API call time plus overhead.

**Validates: Requirements 2.2**

### Property 6: Deduplication of Merged Results
*For any* set of API responses containing duplicate entries (based on content hash or reference), the system should return only unique entries, and the count of unique results should be less than or equal to the sum of all results.

**Validates: Requirements 2.3**

### Property 7: Prayer Times Chronological Ordering
*For any* prayer times response, the five prayer times (Fajr, Dhuhr, Asr, Maghrib, Isha) should be in chronological order, with each time being later than the previous one.

**Validates: Requirements 3.4**

### Property 8: Tafsir Organization by Scholar and Language
*For any* tafsir response with multiple sources, the results should be organized (grouped or sorted) by scholar name and language, making it easy to find tafsir from a specific scholar or in a specific language.

**Validates: Requirements 4.3**

### Property 9: Date Conversion Round Trip
*For any* valid Gregorian date, converting it to Hijri and then back to Gregorian should produce the original date (or a date within acceptable margin due to calculation differences).

**Validates: Requirements 5.2**

### Property 10: Qibla Direction Valid Range
*For any* location coordinates, the calculated qibla direction should be within the valid range of 0-360 degrees (inclusive).

**Validates: Requirements 6.3**

### Property 11: API Key Injection
*For any* API request, the API_Key_Manager should inject the appropriate API key in the correct format (header, query param, or bearer token) based on the API's requirements, and the request should contain the key before being sent.

**Validates: Requirements 8.2**

### Property 12: API Key Confidentiality
*For any* log entry or error message, it should not contain actual API key values (only masked versions like "key_***"), ensuring keys are never exposed in logs.

**Validates: Requirements 8.4**

### Property 13: Rate Limit Enforcement
*For any* API with configured rate limits, the number of requests sent within any time window (minute, hour, day) should never exceed the configured limit for that window.

**Validates: Requirements 9.2, 9.3, 9.5**

### Property 14: Cache-First Behavior
*For any* request with valid cached data, the system should return the cached data without making an external API call, and the response time should be significantly faster than an API call.

**Validates: Requirements 10.1, 10.2**

### Property 15: Cache Update on Miss
*For any* request with expired or missing cache, the system should fetch from the API and update the cache, so that a subsequent identical request finds valid cached data.

**Validates: Requirements 10.3**

### Property 16: TTL Strategy Differentiation
*For any* two data types with different volatility (e.g., Quran text vs Prayer times), the static data should have a longer TTL than the dynamic data in the cache.

**Validates: Requirements 10.4**

### Property 17: Error Categorization
*For any* API error, the Error_Handler should categorize it into one of the defined categories (network, authentication, rate limit, server error), and the category should be included in the error response.

**Validates: Requirements 11.1**

### Property 18: Retry with Exponential Backoff
*For any* network error, the Retry_Mechanism should attempt up to 3 retries, and the delay between retries should increase exponentially (e.g., 1s, 2s, 4s).

**Validates: Requirements 11.2**

### Property 19: Stale Cache as Last Resort
*For any* request where all external APIs fail, if expired cached data exists, the system should return it with a warning indicator rather than failing completely.

**Validates: Requirements 12.2**

### Property 20: Local Calculation Fallback
*For any* prayer times or qibla request where all APIs fail and no cache exists, the system should attempt local calculation using astronomical formulas, and the result should be within reasonable accuracy.

**Validates: Requirements 12.3**

### Property 21: Fallback Event Logging
*For any* fallback event (switching from primary to secondary API, using stale cache, or local calculation), the system should log the event with timestamp, reason, and which fallback was used.

**Validates: Requirements 12.4**

### Property 22: Primary API Recovery Detection
*For any* API that was marked unhealthy and later recovers, the Health_Monitor should detect the recovery within one health check cycle and restore it as the primary source for new requests.

**Validates: Requirements 12.5**

### Property 23: Periodic Health Checks
*For any* configured API, the Health_Monitor should perform a health check at regular intervals (every 5 minutes), and the time between consecutive checks should not deviate significantly from the configured interval.

**Validates: Requirements 13.1**

### Property 24: Health Status Tracking
*For any* API, the Health_Monitor should track response times and success rates over time, and these metrics should be accessible via the monitoring endpoint.

**Validates: Requirements 13.4, 13.5**

### Property 25: Automatic Fallback on Unhealthy Status
*For any* API marked as unhealthy, new requests should automatically use fallback APIs without attempting the unhealthy API, until it's marked healthy again.

**Validates: Requirements 13.3**

## Error Handling

### Error Categories

1. **Network Errors**: Connection timeouts, DNS failures, network unreachable
   - Strategy: Retry with exponential backoff (3 attempts)
   - Fallback: Try next API in priority list

2. **Authentication Errors**: Invalid API key, expired token, unauthorized
   - Strategy: Log error, check key validity, notify administrators
   - Fallback: Try alternative API with different credentials

3. **Rate Limit Errors**: Too many requests, quota exceeded
   - Strategy: Queue request or return rate limit error to client
   - Fallback: Try alternative API or serve from cache

4. **Server Errors**: 500, 502, 503, 504 responses
   - Strategy: Retry once after short delay
   - Fallback: Try next API in priority list

5. **Validation Errors**: Invalid response format, missing required fields
   - Strategy: Log error with response details
   - Fallback: Try next API in priority list

6. **Timeout Errors**: Request takes too long
   - Strategy: Cancel request, try with shorter timeout
   - Fallback: Try faster API or serve from cache

### Error Response Format

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_code: String,
    pub error_message: String,
    pub error_category: ErrorCategory,
    pub timestamp: SystemTime,
    pub request_id: String,
    pub retry_after: Option<Duration>,
    pub fallback_used: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorCategory {
    Network,
    Authentication,
    RateLimit,
    ServerError,
    Validation,
    Timeout,
    Unknown,
}
```

### Retry Strategy

```rust
pub struct RetryStrategy {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(10),
            multiplier: 2.0,
        }
    }
}

impl RetryStrategy {
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        let delay_secs = self.initial_delay.as_secs_f64() * self.multiplier.powi(attempt as i32);
        let delay_secs = delay_secs.min(self.max_delay.as_secs_f64());
        Duration::from_secs_f64(delay_secs)
    }
}
```

## Testing Strategy

### Dual Testing Approach

يتطلب هذا المشروع نهج اختبار مزدوج يجمع بين:

1. **Unit Tests**: للتحقق من أمثلة محددة، حالات حدية، وشروط الأخطاء
2. **Property-Based Tests**: للتحقق من الخصائص العامة عبر جميع المدخلات

كلا النوعين ضروري ومكمل للآخر لضمان تغطية شاملة.

### Unit Testing Focus

يجب أن تركز اختبارات الوحدة على:

- **أمثلة محددة**: اختبار سيناريوهات واقعية محددة (مثل: طلب سورة البقرة، البحث عن حديث معين)
- **نقاط التكامل**: التحقق من التكامل بين المكونات (API Client → Cache Manager → Response)
- **حالات حدية**: اختبار حالات خاصة (API غير متاح، cache ممتلئ، rate limit مستنفد)
- **شروط الأخطاء**: التحقق من معالجة الأخطاء المختلفة

### Property-Based Testing Focus

يجب أن تركز اختبارات الخصائص على:

- **الخصائص العامة**: التحقق من القواعد التي تنطبق على جميع المدخلات
- **التغطية الشاملة**: اختبار آلاف المدخلات العشوائية تلقائياً
- **اكتشاف حالات غير متوقعة**: العثور على أخطاء في سيناريوهات لم نفكر فيها

### Property Test Configuration

- **عدد التكرارات**: 100 تكرار كحد أدنى لكل اختبار خاصية
- **مكتبة الاختبار**: استخدام `proptest` أو `quickcheck` في Rust
- **التوثيق**: كل اختبار خاصية يجب أن يشير إلى الخاصية في وثيقة التصميم
- **صيغة التوثيق**: `// Feature: official-apis-integration, Property {number}: {property_text}`

### Test Organization

```
services/api-integration-service/
├── src/
│   ├── lib.rs
│   ├── service.rs
│   ├── tests/              # Unit tests
│   │   ├── quran_tests.rs
│   │   ├── hadith_tests.rs
│   │   ├── prayer_tests.rs
│   │   ├── cache_tests.rs
│   │   └── rate_limit_tests.rs
│   └── property_tests/     # Property-based tests
│       ├── api_client_properties.rs
│       ├── cache_properties.rs
│       ├── fallback_properties.rs
│       └── validation_properties.rs
└── tests/
    └── integration_tests.rs  # Integration tests with real APIs
```

### Example Property Test

```rust
// Feature: official-apis-integration, Property 9: Date Conversion Round Trip
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn date_conversion_round_trip(
            year in 2000i32..2100,
            month in 1u32..=12,
            day in 1u32..=28  // Safe range for all months
        ) {
            let gregorian = NaiveDate::from_ymd(year, month, day);
            
            // Convert to Hijri
            let hijri = convert_to_hijri(gregorian).unwrap();
            
            // Convert back to Gregorian
            let gregorian_back = convert_to_gregorian(hijri).unwrap();
            
            // Should be within 1 day due to calculation differences
            let diff = (gregorian - gregorian_back).num_days().abs();
            prop_assert!(diff <= 1, "Round trip conversion diff too large: {} days", diff);
        }
    }
}
```

### Integration Testing

اختبارات التكامل يجب أن:

1. تستخدم حسابات اختبار حقيقية للـ APIs (حيث متاح)
2. تتحقق من الاتصال الفعلي بالـ APIs الخارجية
3. تختبر سيناريوهات end-to-end كاملة
4. تُشغل بشكل منفصل عن اختبارات الوحدة (باستخدام feature flags)

### Mock APIs for Testing

لتسهيل الاختبار، سيتم إنشاء mock APIs محلية:

```rust
// tests/mocks/mock_quran_api.rs

pub struct MockQuranApi {
    responses: HashMap<String, QuranTextResponse>,
    failure_rate: f64,
    latency: Duration,
}

impl MockQuranApi {
    pub fn new() -> Self {
        Self {
            responses: Self::load_test_data(),
            failure_rate: 0.0,
            latency: Duration::from_millis(100),
        }
    }
    
    pub fn with_failure_rate(mut self, rate: f64) -> Self {
        self.failure_rate = rate;
        self
    }
    
    pub fn with_latency(mut self, latency: Duration) -> Self {
        self.latency = latency;
        self
    }
}

#[async_trait]
impl QuranApiClient for MockQuranApi {
    async fn get_ayah(&self, surah: u8, ayah: u16) -> Result<AyahData> {
        // Simulate latency
        tokio::time::sleep(self.latency).await;
        
        // Simulate failures
        if rand::random::<f64>() < self.failure_rate {
            return Err(ApiError::Network("Simulated failure".into()));
        }
        
        // Return mock data
        let key = format!("{}:{}", surah, ayah);
        self.responses.get(&key)
            .cloned()
            .ok_or(ApiError::NotFound)
    }
}
```

### Test Coverage Goals

- **Overall Coverage**: 80% minimum
- **Critical Paths**: 95% minimum (API clients, cache, rate limiter, fallback)
- **Error Handling**: 90% minimum
- **Property Tests**: All 25 properties must have corresponding tests

### Continuous Testing

- اختبارات الوحدة تُشغل في كل commit
- اختبارات الخصائص تُشغل في كل pull request
- اختبارات التكامل تُشغل يومياً أو قبل الإصدار
- مراقبة التغطية في CI/CD pipeline

## API-Specific Implementation Details

### Quran.com API

```rust
pub struct QuranComClient {
    base_url: String,
    client: reqwest::Client,
    api_key: Option<String>,
}

impl QuranComClient {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            base_url: "https://api.quran.com/api/v4".to_string(),
            client: reqwest::Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl QuranApiClient for QuranComClient {
    fn api_name(&self) -> &str { "quran.com" }
    fn priority(&self) -> u8 { 1 }  // Primary
    
    async fn get_ayah(&self, surah: u8, ayah: u16) -> Result<AyahData> {
        let url = format!("{}/verses/by_key/{}:{}", self.base_url, surah, ayah);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("Status: {}", response.status())
            ));
        }
        
        let data: QuranComResponse = response.json().await?;
        Ok(data.into())
    }
}
```

### Sunnah.com API

```rust
pub struct SunnahComClient {
    base_url: String,
    client: reqwest::Client,
    api_key: String,
}

impl SunnahComClient {
    pub fn new(api_key: String) -> Self {
        Self {
            base_url: "https://api.sunnah.com/v1".to_string(),
            client: reqwest::Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl HadithApiClient for SunnahComClient {
    fn api_name(&self) -> &str { "sunnah.com" }
    fn priority(&self) -> u8 { 1 }  // Primary
    
    async fn search(&self, query: &str) -> Result<Vec<HadithResult>> {
        let url = format!("{}/hadiths", self.base_url);
        let response = self.client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .query(&[("q", query)])
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("Status: {}", response.status())
            ));
        }
        
        let data: SunnahComResponse = response.json().await?;
        Ok(data.hadiths.into_iter().map(|h| h.into()).collect())
    }
}
```

### Aladhan API

```rust
pub struct AladhanClient {
    base_url: String,
    client: reqwest::Client,
}

impl AladhanClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.aladhan.com/v1".to_string(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl PrayerTimesApiClient for AladhanClient {
    fn api_name(&self) -> &str { "aladhan" }
    fn priority(&self) -> u8 { 1 }  // Primary
    
    async fn get_times(&self, request: &PrayerTimesRequest) -> Result<PrayerTimesResponse> {
        let url = format!("{}/timings/{}", self.base_url, request.date.format("%d-%m-%Y"));
        let response = self.client
            .get(&url)
            .query(&[
                ("latitude", request.latitude.to_string()),
                ("longitude", request.longitude.to_string()),
                ("method", format!("{:?}", request.calculation_method)),
                ("school", format!("{:?}", request.madhab)),
            ])
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(ApiError::ApiError(
                self.api_name().to_string(),
                format!("Status: {}", response.status())
            ));
        }
        
        let data: AladhanResponse = response.json().await?;
        Ok(data.into())
    }
}
```

## Configuration

### Service Configuration

```yaml
# config/api_integration_config.yaml

service:
  name: api-integration-service
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
      rate_limit:
        per_minute: 60
        per_hour: 1000
        per_day: 10000
      timeout: 10s
      
    - name: alquran.cloud
      base_url: https://api.alquran.cloud/v1
      priority: 2
      rate_limit:
        per_minute: 30
        per_hour: 500
        per_day: 5000
      timeout: 10s
      
  hadith:
    - name: sunnah.com
      base_url: https://api.sunnah.com/v1
      priority: 1
      requires_key: true
      rate_limit:
        per_minute: 30
        per_hour: 500
        per_day: 5000
      timeout: 15s
      
  prayer_times:
    - name: aladhan
      base_url: https://api.aladhan.com/v1
      priority: 1
      rate_limit:
        per_minute: 60
        per_hour: 1000
        per_day: 10000
      timeout: 5s

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
  unhealthy_threshold: 3  # consecutive failures
  recovery_threshold: 2   # consecutive successes

retry:
  max_attempts: 3
  initial_delay: 1s
  max_delay: 10s
  multiplier: 2.0
```

## Deployment Considerations

### Environment Variables

```bash
# API Keys (stored in secrets manager in production)
QURAN_COM_API_KEY=
SUNNAH_COM_API_KEY=
ISLAMIC_FINDER_API_KEY=
HUGGING_FACE_API_KEY=
OPENAI_API_KEY=  # Optional

# Redis
REDIS_URL=redis://localhost:6379

# PostgreSQL
DATABASE_URL=postgresql://user:pass@localhost:5432/sanad

# Service Configuration
SERVICE_PORT=8080
LOG_LEVEL=info
ENVIRONMENT=production
```

### Docker Deployment

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin api-integration-service

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api-integration-service /usr/local/bin/
COPY config/ /etc/sanad/config/
EXPOSE 8080
CMD ["api-integration-service"]
```

### Monitoring and Observability

- **Metrics**: Prometheus metrics for API calls, cache hits/misses, error rates
- **Logging**: Structured logging with correlation IDs
- **Tracing**: Distributed tracing with OpenTelemetry
- **Alerts**: Alert on high error rates, API failures, rate limit approaching

## Security Considerations

1. **API Key Storage**: Store keys in environment variables or secrets manager, never in code
2. **Key Rotation**: Support hot-reloading of API keys without service restart
3. **Rate Limiting**: Enforce rate limits to prevent abuse and comply with API terms
4. **Input Validation**: Validate all inputs before sending to external APIs
5. **Output Sanitization**: Sanitize API responses before caching or returning to clients
6. **TLS/SSL**: Use HTTPS for all external API calls
7. **Audit Logging**: Log all API calls with timestamps and request IDs for audit trails

## Performance Optimization

1. **Connection Pooling**: Reuse HTTP connections to external APIs
2. **Parallel Requests**: Query multiple APIs in parallel when appropriate
3. **Caching Strategy**: Aggressive caching for static data, smart invalidation for dynamic data
4. **Compression**: Enable gzip compression for API responses
5. **Timeout Configuration**: Set appropriate timeouts to prevent hanging requests
6. **Circuit Breaker**: Implement circuit breaker pattern for failing APIs
7. **Request Batching**: Batch multiple requests where APIs support it

## Future Enhancements

1. **GraphQL API**: Expose a GraphQL interface for flexible querying
2. **WebSocket Support**: Real-time updates for prayer times and notifications
3. **Offline Support**: Enhanced offline capabilities with local databases
4. **Multi-Region**: Deploy in multiple regions for lower latency
5. **API Versioning**: Support multiple API versions for backward compatibility
6. **Custom API Plugins**: Allow adding custom API integrations via plugins
7. **Machine Learning**: Use ML to predict API failures and optimize fallback strategies

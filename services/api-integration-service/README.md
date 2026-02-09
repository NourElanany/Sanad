# API Integration Service

## Overview

The API Integration Service provides a unified interface for integrating with multiple official Islamic APIs. It implements a comprehensive system for managing external API connections with built-in support for:

- **Multiple API Sources**: Integrates with verified official APIs for Quran, Hadith, Prayer Times, Tafsir, Calendar, Qibla, and AI/NLP services
- **Intelligent Caching**: Smart caching strategies with different TTLs based on data volatility
- **Rate Limiting**: Per-API rate limiting to comply with terms of service
- **Fallback Mechanisms**: Automatic fallback to secondary APIs when primary APIs fail
- **Health Monitoring**: Continuous monitoring of API health and performance
- **Error Handling**: Comprehensive error handling with retry mechanisms

## Verified Official API Sources

All API sources have been verified for authenticity and official status:

### Quran APIs
- **Quran.com / Quran Foundation API** - ✅ Official
- **Tanzil.net** - ✅ Official verified text
- **AlQuran Cloud API** - ✅ Community trusted
- **EveryAyah.com** - ✅ Verified reciters
- **IslamHouse QuranEnc.com** - ✅ Officially supervised

### Hadith APIs
- **Sunnah.com** - ✅ Official with authenticated chains
- **IslamHouse HadeethEnc.com** - ✅ Officially supervised

### Prayer Times & Qibla
- **AlAdhan API** - ✅ Official Islamic Network
- **Islamic Finder** - ✅ Widely trusted

### Tafsir
- **Quran.com Tafsir API** - ✅ Official Quran Foundation

### Calendar
- **AlAdhan Hijri Calendar** - ✅ Official
- **Islamic Finder Calendar** - ✅ Verified

### AI/NLP (Technical processing only)
- **Hugging Face Arabic Models** - ✅ For language processing only
- Note: NOT used for Islamic rulings or fatwas

## Architecture

The service is organized into several layers:

1. **API Router Layer**: Routes requests to appropriate API clients
2. **Rate Limiting Layer**: Controls request rates per API
3. **Caching Layer**: Intelligent caching with TTL strategies
4. **Client Layer**: HTTP clients for each external API
5. **Fallback Layer**: Automatic fallback mechanisms
6. **Monitoring Layer**: Health monitoring and metrics

## Core Traits

### ApiClient
Base trait for all API clients providing:
- API name and priority
- Health check capability
- Rate limit configuration

### Specialized Traits
- `QuranApiClient`: Quran text and audio
- `HadithApiClient`: Hadith search and retrieval
- `PrayerTimesApiClient`: Prayer times calculation
- `TafsirApiClient`: Quran interpretation
- `CalendarApiClient`: Hijri calendar conversions
- `QiblaApiClient`: Qibla direction calculation
- `AiApiClient`: AI-powered language processing

## Usage

```rust
use api_integration_service::{ApiIntegrationService, ServiceConfig};

// Load configuration
let config = ServiceConfig::from_file("config/api_integration_config.yaml")?;

// Create service instance
let service = ApiIntegrationService::new(config).await?;

// Get Quran text
let request = QuranTextRequest {
    surah: 1,
    ayah: Some(1),
    translation: Some("en.sahih".to_string()),
    reciter: None,
};
let response = service.get_quran_text(request).await?;

// Search hadith
let request = HadithSearchRequest {
    query: "prayer".to_string(),
    collection: None,
    book: None,
    language: "en".to_string(),
    limit: 10,
};
let response = service.search_hadith(request).await?;

// Get prayer times
let request = PrayerTimesRequest {
    latitude: 21.4225,
    longitude: 39.8262,
    date: chrono::Local::now().date_naive(),
    calculation_method: CalculationMethod::Makkah,
    madhab: Madhab::Shafi,
};
let response = service.get_prayer_times(request).await?;
```

## Configuration

The service is configured via YAML file. See `config/api_integration_config.yaml` for a complete example.

Key configuration sections:
- `service`: Service name, port, and host
- `redis`: Redis connection for caching
- `postgres`: PostgreSQL connection for persistence
- `apis`: Configuration for each API category
- `cache`: Cache strategies per data type
- `health_monitor`: Health check settings
- `retry`: Retry strategy configuration

## Environment Variables

API keys should be stored in environment variables:

```bash
QURAN_COM_API_KEY=your_key_here
SUNNAH_COM_API_KEY=your_key_here
ISLAMIC_FINDER_API_KEY=your_key_here
HUGGING_FACE_API_KEY=your_key_here
```

## Development Status

This service is currently under development. Implementation is organized into tasks:

- ✅ Task 1: Project structure and shared libraries (COMPLETE)
- ⏳ Task 2: API Key Manager
- ⏳ Task 3: Rate Limiter
- ⏳ Task 4: Cache Manager
- ⏳ Task 6-13: API Client Implementations
- ⏳ Task 15-17: Error Handling, Fallback, Health Monitoring
- ⏳ Task 18: Main Integration Service
- ⏳ Task 20: HTTP Handlers and Routes

## Testing

The service includes comprehensive testing:

- **Unit Tests**: Specific examples and edge cases
- **Property-Based Tests**: Universal properties across all inputs
- **Integration Tests**: Real API connectivity tests

Run tests:
```bash
cargo test --package api-integration-service
```

## License

See the main project LICENSE file.

# Implementation Plan: Official Islamic APIs Integration

## Overview

هذه الخطة تحول التصميم إلى سلسلة من المهام القابلة للتنفيذ لبناء نظام تكامل شامل مع جميع الـ APIs الإسلامية الرسمية. سيتم البناء بشكل تدريجي، بدءاً من البنية الأساسية، ثم إضافة API clients، ثم الميزات المتقدمة مثل caching وrate limiting وfallback mechanisms.

## Tasks

- [x] 1. Setup project structure and shared libraries
  - Create `services/api-integration-service` directory with Cargo.toml
  - Create `shared/src/api_clients` module for shared API client code
  - Define core traits: `ApiClient`, `QuranApiClient`, `HadithApiClient`, etc.
  - Set up dependencies: reqwest, tokio, serde, redis, sqlx
  - _Requirements: 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1_

- [x] 2. Implement API Key Manager
  - [x] 2.1 Create ApiKeyManager struct with key storage
    - Implement key loading from environment variables
    - Implement key loading from secrets manager (optional)
    - Support different key types: Header, QueryParam, Bearer, Basic
    - _Requirements: 8.1_
  
  - [x] 2.2 Write property test for API key injection
    - **Property 11: API Key Injection**
    - **Validates: Requirements 8.2**
  
  - [x] 2.3 Write property test for API key confidentiality
    - **Property 12: API Key Confidentiality**
    - **Validates: Requirements 8.4**
  
  - [x] 2.4 Implement key injection into HTTP requests
    - Add method to inject keys based on ApiKeyType
    - Handle key expiration and validation
    - _Requirements: 8.2, 8.3_
  
  - [x] 2.5 Write unit tests for key rotation
    - Test hot-reloading of API keys
    - Test error handling for invalid/expired keys
    - _Requirements: 8.5_

- [x] 3. Implement Rate Limiter
  - [x] 3.1 Create RateLimiter struct with Redis backend
    - Implement rate limit checking for minute/hour/day windows
    - Implement counter increment with TTL
    - Support per-API rate limit configuration
    - _Requirements: 9.1, 9.2, 9.5_
  
  - [x] 3.2 Write property test for rate limit enforcement
    - **Property 13: Rate Limit Enforcement**
    - **Validates: Requirements 9.2, 9.3, 9.5**
  
  - [x] 3.3 Implement rate limit exceeded handling
    - Queue requests or return rate limit error
    - Log warnings when approaching limits
    - _Requirements: 9.3, 9.4_
  
  - [x] 3.4 Write unit tests for rate limiting edge cases
    - Test boundary conditions (exactly at limit)
    - Test concurrent requests
    - _Requirements: 9.2, 9.3_

- [x] 4. Implement Cache Manager
  - [x] 4.1 Create CacheManager struct with Redis backend
    - Implement get/set operations with TTL
    - Support different cache strategies per data type
    - Implement stale cache support
    - _Requirements: 10.1, 10.2, 10.3_
  
  - [x] 4.2 Write property test for cache-first behavior
    - **Property 14: Cache-First Behavior**
    - **Validates: Requirements 10.1, 10.2**
  
  - [x] 4.3 Write property test for cache update on miss
    - **Property 15: Cache Update on Miss**
    - **Validates: Requirements 10.3**
  
  - [x] 4.4 Write property test for TTL strategy differentiation
    - **Property 16: TTL Strategy Differentiation**
    - **Validates: Requirements 10.4**
  
  - [x] 4.5 Implement LRU eviction policy
    - Track cache usage and evict least recently used entries
    - _Requirements: 10.5_
  
  - [x] 4.6 Write unit tests for cache operations
    - Test cache hit/miss scenarios
    - Test TTL expiration
    - Test stale cache retrieval
    - _Requirements: 10.1, 10.2, 10.3_

- [x] 5. Checkpoint - Ensure core infrastructure tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 6. Implement Quran API Clients
  - [x] 6.1 Create QuranComClient
    - Implement get_surah, get_ayah, get_page methods
    - Handle authentication and error responses
    - _Requirements: 1.1_
  
  - [x] 6.2 Create AlquranCloudClient
    - Implement same interface as QuranComClient
    - _Requirements: 1.1_
  
  - [x] 6.3 Create TanzilClient
    - Implement Quran text fetching from Tanzil API
    - _Requirements: 1.1_
  
  - [x] 6.4 Create EveryayahClient
    - Implement audio recitation fetching
    - _Requirements: 1.3_
  
  - [x] 6.5 Create QuranApiManager
    - Implement fallback logic between multiple Quran APIs
    - Integrate with CacheManager and RateLimiter
    - _Requirements: 1.2, 1.5_
  
  - [x] 6.6 Write property test for fallback chain execution
    - **Property 2: Fallback Chain Execution**
    - **Validates: Requirements 1.2**
  
  - [x] 6.7 Write property test for response validation
    - **Property 3: Response Validation Consistency**
    - **Validates: Requirements 1.4**
  
  - [x] 6.8 Write unit tests for Quran API clients
    - Test successful requests
    - Test error handling
    - Test audio fetching
    - _Requirements: 1.1, 1.3, 1.4_

- [x] 7. Implement Hadith API Clients
  - [x] 7.1 Create SunnahComClient
    - Implement search and get_by_id methods
    - Handle API key authentication
    - _Requirements: 2.1_
  
  - [x] 7.2 Create HadithApiClient
    - Implement hadith search and retrieval
    - _Requirements: 2.1_
  
  - [x] 7.3 Create AladhanHadithClient
    - Implement hadith fetching from Aladhan
    - _Requirements: 2.1_
  
  - [x] 7.4 Create HadithApiManager
    - Implement parallel querying of multiple hadith APIs
    - Implement result merging and deduplication
    - _Requirements: 2.2, 2.3_
  
  - [x] 7.5 Write property test for parallel API querying
    - **Property 5: Parallel API Querying**
    - **Validates: Requirements 2.2**
  
  - [x] 7.6 Write property test for deduplication
    - **Property 6: Deduplication of Merged Results**
    - **Validates: Requirements 2.3**
  
  - [x] 7.7 Write unit tests for Hadith API clients
    - Test search functionality
    - Test result merging
    - Test deduplication logic
    - _Requirements: 2.2, 2.3, 2.4_

- [x] 8. Implement Prayer Times API Clients
  - [x] 8.1 Create AladhanPrayerClient
    - Implement prayer times calculation with location and method
    - Support different calculation methods and madhabs
    - _Requirements: 3.1, 3.2_
  
  - [x] 8.2 Create IslamicFinderPrayerClient
    - Implement same interface as AladhanPrayerClient
    - _Requirements: 3.1_
  
  - [x] 8.3 Create PrayerTimesApiManager
    - Implement fallback between prayer times APIs
    - Implement local calculation as last resort
    - _Requirements: 3.3_
  
  - [x] 8.4 Write property test for prayer times chronological ordering
    - **Property 7: Prayer Times Chronological Ordering**
    - **Validates: Requirements 3.4**
  
  - [x] 8.5 Write unit tests for prayer times APIs
    - Test different calculation methods
    - Test different madhabs
    - Test fallback to local calculation
    - _Requirements: 3.2, 3.3, 3.4_

- [x] 9. Checkpoint - Ensure API clients tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 10. Implement Tafsir API Clients
  - [x] 10.1 Create QuranComTafsirClient
    - Implement tafsir fetching for specific verses
    - Support multiple tafsir sources
    - _Requirements: 4.1, 4.2_
  
  - [x] 10.2 Create TafsirApiManager
    - Implement fetching from multiple tafsir sources
    - Organize results by scholar and language
    - _Requirements: 4.3_
  
  - [x] 10.3 Write property test for tafsir organization
    - **Property 8: Tafsir Organization by Scholar and Language**
    - **Validates: Requirements 4.3**
  
  - [x] 10.4 Write unit tests for Tafsir API clients
    - Test verse reference validation
    - Test multi-source fetching
    - Test organization by scholar/language
    - _Requirements: 4.2, 4.3, 4.4_

- [x] 11. Implement Calendar API Clients
  - [x] 11.1 Create AladhanCalendarClient
    - Implement Gregorian to Hijri conversion
    - Implement Hijri to Gregorian conversion
    - Implement Islamic events fetching
    - _Requirements: 5.1, 5.2, 5.3_
  
  - [x] 11.2 Create IslamicFinderCalendarClient
    - Implement same interface as AladhanCalendarClient
    - _Requirements: 5.1_
  
  - [x] 11.3 Create CalendarApiManager
    - Implement fallback between calendar APIs
    - _Requirements: 5.2_
  
  - [x] 11.4 Write property test for date conversion round trip
    - **Property 9: Date Conversion Round Trip**
    - **Validates: Requirements 5.2**
  
  - [x] 11.5 Write unit tests for Calendar API clients
    - Test date conversions
    - Test Islamic events fetching
    - Test date format validation
    - _Requirements: 5.2, 5.3, 5.4_

- [x] 12. Implement Qibla API Clients
  - [x] 12.1 Create AladhanQiblaClient
    - Implement qibla direction calculation from coordinates
    - _Requirements: 6.1, 6.2_
  
  - [x] 12.2 Create IslamicFinderQiblaClient
    - Implement same interface as AladhanQiblaClient
    - _Requirements: 6.1_
  
  - [x] 12.3 Implement local qibla calculation
    - Use astronomical formulas as fallback
    - _Requirements: 6.4_
  
  - [x] 12.4 Create QiblaApiManager
    - Implement fallback to local calculation
    - _Requirements: 6.4_
  
  - [x] 12.5 Write property test for qibla direction valid range
    - **Property 10: Qibla Direction Valid Range**
    - **Validates: Requirements 6.3**
  
  - [x] 12.6 Write property test for local calculation fallback
    - **Property 20: Local Calculation Fallback**
    - **Validates: Requirements 12.3**
  
  - [x] 12.7 Write unit tests for Qibla API clients
    - Test direction calculation
    - Test fallback to local calculation
    - Test direction range validation
    - _Requirements: 6.2, 6.3, 6.4_

- [x] 13. Implement AI/NLP API Clients
  - [x] 13.1 Create HuggingFaceClient
    - Implement query processing with Arabic NLP models
    - Handle authentication and rate limiting
    - _Requirements: 7.1, 7.2_
  
  - [x] 13.2 Create OpenAIClient (optional)
    - Implement query processing with OpenAI API
    - _Requirements: 7.1, 7.2_
  
  - [x] 13.3 Create AiApiManager
    - Implement fallback between AI services
    - Implement response validation and filtering
    - _Requirements: 7.2_
  
  - [x] 13.4 Write unit tests for AI API clients
    - Test query processing
    - Test error handling when services unavailable
    - Test response caching
    - _Requirements: 7.2, 7.4, 7.5_

- [x] 14. Checkpoint - Ensure all API clients are integrated
  - Ensure all tests pass, ask the user if questions arise.

- [x] 15. Implement Error Handling System
  - [x] 15.1 Create ErrorHandler with error categorization
    - Categorize errors: Network, Authentication, RateLimit, ServerError, Validation, Timeout
    - Implement user-friendly error messages
    - _Requirements: 11.1, 11.5_
  
  - [x] 15.2 Implement RetryMechanism with exponential backoff
    - Retry network errors up to 3 times
    - Use exponential backoff strategy
    - _Requirements: 11.2_
  
  - [x] 15.3 Write property test for error categorization
    - **Property 17: Error Categorization**
    - **Validates: Requirements 11.1**
  
  - [x] 15.4 Write property test for retry with exponential backoff
    - **Property 18: Retry with Exponential Backoff**
    - **Validates: Requirements 11.2**
  
  - [x] 15.5 Write unit tests for error handling
    - Test different error types
    - Test retry logic
    - Test authentication error handling
    - _Requirements: 11.1, 11.2, 11.3_

- [x] 16. Implement Fallback System
  - [x] 16.1 Create FallbackSystem with priority-based API selection
    - Implement automatic switching to secondary APIs
    - Implement stale cache serving as last resort
    - Implement local calculation fallback where applicable
    - _Requirements: 12.1, 12.2, 12.3_
  
  - [x] 16.2 Write property test for stale cache as last resort
    - **Property 19: Stale Cache as Last Resort**
    - **Validates: Requirements 12.2**
  
  - [x] 16.3 Write property test for fallback event logging
    - **Property 21: Fallback Event Logging**
    - **Validates: Requirements 12.4**
  
  - [x] 16.4 Write unit tests for fallback mechanisms
    - Test priority-based fallback
    - Test stale cache serving
    - Test local calculation fallback
    - _Requirements: 12.1, 12.2, 12.3, 12.4_

- [x] 17. Implement Health Monitor
  - [x] 17.1 Create HealthMonitor with periodic health checks
    - Check health of all APIs every 5 minutes
    - Track response times and success rates
    - Mark APIs as healthy/unhealthy based on consecutive failures
    - _Requirements: 13.1, 13.2, 13.4_
  
  - [x] 17.2 Implement automatic recovery detection
    - Detect when unhealthy APIs recover
    - Restore recovered APIs as primary sources
    - _Requirements: 12.5, 13.2_
  
  - [x] 17.3 Create health metrics endpoint
    - Expose health status for all APIs
    - Expose response times and success rates
    - _Requirements: 13.5_
  
  - [x] 17.4 Write property test for periodic health checks
    - **Property 23: Periodic Health Checks**
    - **Validates: Requirements 13.1**
  
  - [x] 17.5 Write property test for primary API recovery detection
    - **Property 22: Primary API Recovery Detection**
    - **Validates: Requirements 12.5**
  
  - [x] 17.6 Write property test for automatic fallback on unhealthy status
    - **Property 25: Automatic Fallback on Unhealthy Status**
    - **Validates: Requirements 13.3**
  
  - [x] 17.7 Write unit tests for health monitoring
    - Test health check execution
    - Test unhealthy marking and alerts
    - Test recovery detection
    - Test metrics tracking
    - _Requirements: 13.1, 13.2, 13.3, 13.4_

- [x] 18. Implement Main Integration Service
  - [x] 18.1 Create ApiIntegrationService struct
    - Initialize all API managers
    - Initialize cache, rate limiter, health monitor
    - Load configuration from YAML file
    - _Requirements: 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1_
  
  - [x] 18.2 Implement service methods for all API categories
    - Implement get_quran_text, get_quran_audio
    - Implement search_hadith, get_hadith_by_id
    - Implement get_prayer_times
    - Implement get_tafsir
    - Implement convert_date, get_islamic_events
    - Implement get_qibla_direction
    - Implement process_ai_query
    - _Requirements: All requirements_
  
  - [x] 18.3 Implement health_check endpoint
    - Return overall service health
    - Return individual API health status
    - _Requirements: 13.5_
  
  - [x] 18.4 Write property test for API client initialization completeness
    - **Property 1: API Client Initialization Completeness**
    - **Validates: Requirements 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1, 8.1**
  
  - [x] 18.5 Write integration tests for end-to-end flows
    - Test complete request flow from service to API and back
    - Test caching integration
    - Test rate limiting integration
    - Test fallback integration
    - _Requirements: All requirements_

- [x] 19. Checkpoint - Ensure integration service tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 20. Implement HTTP Handlers and Routes
  - [x] 20.1 Create HTTP handlers using Actix-web or Axum
    - Create handlers for all service methods
    - Implement request validation
    - Implement response serialization
    - _Requirements: All requirements_
  
  - [x] 20.2 Set up routes and middleware
    - Configure routes for all endpoints
    - Add logging middleware
    - Add error handling middleware
    - _Requirements: All requirements_
  
  - [x] 20.3 Write integration tests for HTTP endpoints
    - Test all endpoints with valid requests
    - Test error responses
    - Test rate limiting via HTTP
    - _Requirements: All requirements_

- [x] 21. Create Configuration Management
  - [x] 21.1 Create configuration structs
    - Define ServiceConfig, ApiConfig, CacheConfig, etc.
    - Implement configuration loading from YAML
    - Support environment variable overrides
    - _Requirements: 8.1, 9.1_
  
  - [x] 21.2 Create example configuration files
    - Create config/api_integration_config.yaml
    - Create .env.example with API keys template
    - Document all configuration options
    - _Requirements: 14.1, 14.2_
  
  - [x] 21.3 Write unit tests for configuration loading
    - Test YAML parsing
    - Test environment variable overrides
    - Test validation of required fields
    - _Requirements: 8.1_

- [x] 22. Implement Logging and Monitoring
  - [x] 22.1 Set up structured logging
    - Use tracing crate for structured logging
    - Add correlation IDs to all requests
    - Log all API calls with timing
    - _Requirements: 12.4, 13.4_
  
  - [x] 22.2 Set up Prometheus metrics
    - Add metrics for API calls, cache hits/misses, error rates
    - Add metrics for response times
    - Add metrics for rate limit usage
    - _Requirements: 13.4, 13.5_
  
  - [x] 22.3 Set up OpenTelemetry tracing
    - Add distributed tracing for request flows
    - Trace API calls to external services
    - _Requirements: 13.5_
  
  - [x] 22.4 Write unit tests for metrics collection
    - Test metric increments
    - Test metric labels
    - _Requirements: 13.4_

- [x] 23. Create Documentation
  - [x] 23.1 Write API documentation
    - Document all endpoints with examples
    - Document request/response formats
    - Document error codes and messages
    - _Requirements: 14.1, 14.2, 14.3_
  
  - [x] 23.2 Write deployment documentation
    - Document environment variables
    - Document Docker deployment
    - Document configuration options
    - _Requirements: 14.1_
  
  - [x] 23.3 Write developer guide
    - Document how to add new API integrations
    - Document testing strategy
    - Document troubleshooting guide
    - _Requirements: 14.1, 14.5_

- [x] 24. Create Docker and Deployment Files
  - [x] 24.1 Create Dockerfile
    - Multi-stage build for optimized image
    - Include configuration files
    - _Requirements: All requirements_
  
  - [x] 24.2 Create docker-compose.yml
    - Include api-integration-service
    - Include Redis
    - Include PostgreSQL
    - _Requirements: All requirements_
  
  - [x] 24.3 Create Kubernetes manifests (optional)
    - Create deployment, service, configmap
    - Create secrets for API keys
    - _Requirements: All requirements_

- [x] 25. Final Integration Testing
  - [x] 25.1 Run all property-based tests
    - Verify all 25 properties pass with 100+ iterations
    - _Requirements: All requirements_
  
  - [x] 25.2 Run all unit tests
    - Verify 100%+ code coverage
    - _Requirements: All requirements_
  
  - [x] 25.3 Run integration tests with real APIs
    - Test with actual API keys (test accounts)
    - Verify end-to-end functionality
    - _Requirements: All requirements_
  
  - [x] 25.4 Run load tests
    - Test rate limiting under load
    - Test caching performance
    - Test fallback mechanisms under failure scenarios
    - _Requirements: 9.1, 10.1, 12.1_

- [x] 26. Final Checkpoint - Production Readiness
  - Ensure all tests pass, ask the user if questions arise.
  - Verify all documentation is complete
  - Verify all configuration is production-ready
  - Verify monitoring and logging are working

## Notes

- All tasks are required for comprehensive implementation
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties (minimum 100 iterations each)
- Unit tests validate specific examples and edge cases
- Integration tests verify actual API connectivity
- All API keys should be stored securely in environment variables or secrets manager
- Rate limiting must be configured according to each API's terms of service
- Caching strategies should be optimized based on data type (static vs dynamic)
- Fallback mechanisms ensure service continuity even when APIs fail
- Health monitoring provides observability into API status and performance

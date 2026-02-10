# Property Test Compilation Fixes - Task 25.1

## Summary of API Changes

### 1. ApiClient Trait Changes
**Old API:**
```rust
trait ApiClient {
    type Request;
    type Response;
    async fn request(&self, req: Self::Request) -> Result<Self::Response, ApiError>;
    fn rate_limit(&self) -> RateLimitConfig;
}
```

**New API:**
```rust
trait ApiClient: Send + Sync + Debug {
    fn api_name(&self) -> &str;
    fn priority(&self) -> u8;
    async fn is_healthy(&self) -> bool;
    fn rate_limit(&self) -> RateLimitConfig;
}
```

**Impact:** Associated types `Request` and `Response` removed. The `request` method is no longer part of the base trait.

### 2. CacheManager Constructor Changes
**Old API:**
```rust
impl CacheManager {
    pub fn new(redis: Arc<RedisClient>, strategies: HashMap<...>) -> Self
}
```

**New API:**
```rust
impl CacheManager {
    pub async fn new(redis_url: &str) -> Result<Self, ApiError>
    pub async fn with_strategies(redis_url: &str, strategies: HashMap<...>) -> Result<Self, ApiError>
}
```

**Impact:** Constructor is now async and takes redis_url string instead of client.

### 3. RateLimiter Constructor Changes
**Old API:**
```rust
impl RateLimiter {
    pub fn new(redis: Arc<RedisClient>, configs: HashMap<...>) -> Self
}
```

**New API:**
```rust
impl RateLimiter {
    pub async fn new(redis_url: &str, configs: HashMap<...>) -> Result<Self, ApiError>
}
```

**Impact:** Constructor is now async and takes redis_url string instead of client.

### 4. ApiError Clone Trait
**Issue:** ApiError enum didn't implement Clone, but property tests need it.

**Fix:** Added Clone derive and converted `reqwest::Error` from `#[from]` to manual From impl.

### 5. FallbackSystem API Changes
**Old API:**
```rust
execute_with_fallback<T: ApiClient>(
    clients: Vec<Box<dyn ApiClient<Request=R, Response=Res>>>,
    request: R
) -> Result<Res, ApiError>
```

**New API:**
```rust
execute_with_fallback<T, Res, F, Fut>(
    clients: &[Arc<T>],
    request_fn: F,
    cache_key: Option<&str>,
    request_id: String
) -> Result<(Res, Option<FallbackEvent>), ApiError>
where
    T: ApiClient + ?Sized,
    F: Fn(Arc<T>) -> Fut,
    Fut: Future<Output = Result<Res, ApiError>>
```

**Impact:** Now uses a request function closure instead of request object.

### 6. Private Methods Being Accessed
Several tests access private methods that should be public or tested differently:
- `cache_key()` methods in various managers
- `validate_response()` in AI manager
- `filter_sources()` in AI service

## Files Requiring Fixes

### High Priority (Core Infrastructure)
1. `shared/src/api_clients/fallback_system_property_tests.rs` - Remove Request/Response types
2. `shared/src/api_clients/health_monitor_property_tests.rs` - Remove Request/Response types
3. `shared/src/api_clients/error_handler_property_tests.rs` - ApiError Clone issues
4. `shared/src/api_clients/cache_manager_property_tests.rs` - Constructor changes
5. `shared/src/api_clients/rate_limiter_property_tests.rs` - Constructor changes

### Medium Priority (API Clients)
6. `shared/src/api_clients/qibla/property_tests.rs` - Constructor and cache_key issues
7. `shared/src/api_clients/qibla/tests.rs` - Constructor and cache_key issues
8. `shared/src/api_clients/ai/tests.rs` - Constructor, cache_key, and private method issues
9. `shared/src/api_clients/ai/manager.rs` - Constructor usage

### Low Priority (Other Services)
10. `src/ai_service/tests.rs` - Type resolution issues
11. `src/ai_service/multiple_viewpoints_tests.rs` - Type resolution issues
12. `src/ai_service/ai_answer_quality_tests.rs` - Return type issues

## Fix Strategy

### Phase 1: Core Trait Fixes (DONE)
- ✅ Add Clone to ApiError
- ✅ Convert reqwest::Error to String in ApiError

### Phase 2: Mock Client Updates
- Update all MockApiClient implementations to remove Request/Response types
- Add Debug derive to all mock clients
- Implement rate_limit() method on all mocks

### Phase 3: Constructor Updates
- Update all CacheManager::new() calls to be async with redis_url
- Update all RateLimiter::new() calls to be async with redis_url
- Wrap in tokio runtime where needed

### Phase 4: Fallback System Updates
- Rewrite tests to use request_fn closure pattern
- Update return type handling for (Result, Option<FallbackEvent>)

### Phase 5: Private Method Access
- Make cache_key() methods public where appropriate
- Rewrite tests to avoid accessing private methods
- Test through public API instead

### Phase 6: Integration Test Updates
- Update service integration tests
- Fix AI service test imports

## Estimated Fixes

- **Fallback System Property Tests**: ~50 lines to rewrite
- **Health Monitor Property Tests**: ~40 lines to rewrite
- **Error Handler Property Tests**: ~30 lines (mostly done with Clone fix)
- **Cache Manager Property Tests**: ~20 lines for constructor updates
- **Rate Limiter Property Tests**: ~20 lines for constructor updates
- **Qibla Tests**: ~60 lines for constructor and method access
- **AI Tests**: ~80 lines for constructor and method access
- **Other Service Tests**: ~50 lines for type imports

**Total**: ~350 lines of test code to update

## Testing Plan

After fixes:
1. Run `cargo test --package shared --lib` to verify shared library tests
2. Run `cargo test --lib` to verify all library tests
3. Run property tests with 100+ iterations
4. Verify all 25 properties pass

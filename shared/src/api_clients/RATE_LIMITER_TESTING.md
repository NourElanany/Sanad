# Rate Limiter Testing Guide

## Overview

The Rate Limiter implementation includes three types of tests:

1. **Basic Unit Tests** - Tests that don't require Redis (test data structures and logic)
2. **Integration Tests** - Tests that require a running Redis instance
3. **Property-Based Tests** - Tests that verify universal properties across many inputs

## Running Tests

### Basic Unit Tests (No Redis Required)

These tests verify the `RateLimitUsage` struct and its methods:

```bash
cargo test --package shared --lib api_clients::rate_limiter::tests -- --nocapture
```

### Integration Tests (Requires Redis)

These tests require a running Redis instance on `localhost:6379`.

**Start Redis:**
```bash
# Using Docker
docker run -d -p 6379:6379 redis:latest

# Or using Redis installed locally
redis-server
```

**Run Integration Tests:**
```bash
cargo test --package shared --lib api_clients::rate_limiter::unit_tests -- --nocapture
```

### Property-Based Tests (Requires Redis)

Property-based tests verify that the rate limiter enforces limits correctly across 100+ random test cases:

```bash
cargo test --package shared --lib api_clients::rate_limiter::property_tests -- --nocapture
```

## Test Coverage

### Basic Unit Tests
- ✅ `test_rate_limit_usage_is_exceeded` - Verify exceeded detection
- ✅ `test_rate_limit_usage_not_exceeded` - Verify not exceeded detection
- ✅ `test_max_usage_percentage` - Verify percentage calculation

### Integration Unit Tests (Requires Redis)
- ✅ `test_exactly_at_limit` - Boundary condition at exact limit
- ✅ `test_concurrent_requests` - Concurrent request handling
- ✅ `test_check_without_increment` - Idempotent check operations
- ✅ `test_increment_without_check` - Direct increment operations
- ✅ `test_unknown_api_error` - Error handling for unknown APIs
- ✅ `test_check_and_increment_or_error` - Error-based rate limiting
- ✅ `test_time_until_reset` - Reset time calculation
- ✅ `test_is_approaching_limit` - Threshold detection
- ✅ `test_multiple_time_windows` - Multiple window enforcement
- ✅ `test_reset_functionality` - Counter reset
- ✅ `test_usage_statistics` - Usage statistics accuracy
- ✅ `test_zero_limit` - Zero limit edge case
- ✅ `test_very_high_limit` - Very high limit handling

### Property-Based Tests (Requires Redis)
- ✅ **Property 13: Rate Limit Enforcement** - Validates Requirements 9.2, 9.3, 9.5
  - `property_rate_limit_enforcement_minute` - Minute window enforcement
  - `property_rate_limit_enforcement_multiple_windows` - Multiple window enforcement
  - `property_rate_limit_idempotent_check` - Idempotent check behavior
  - `property_rate_limit_usage_percentage` - Percentage calculation correctness
  - `property_rate_limit_reset_clears_counters` - Reset functionality

## CI/CD Integration

For CI/CD pipelines, ensure Redis is available:

```yaml
# GitHub Actions example
services:
  redis:
    image: redis:latest
    ports:
      - 6379:6379
    options: >-
      --health-cmd "redis-cli ping"
      --health-interval 10s
      --health-timeout 5s
      --health-retries 5
```

## Test Isolation

Each test uses a unique API name (with random suffix) to ensure test isolation and prevent interference between concurrent test runs.

## Expected Test Results

All tests should pass when Redis is available. If Redis is not available, integration and property-based tests will fail with connection errors.

## Troubleshooting

### Redis Connection Errors

If you see errors like "Failed to connect to Redis", ensure:
1. Redis is running on `localhost:6379`
2. No firewall is blocking the connection
3. Redis is accepting connections (check `redis-cli ping`)

### Test Timeouts

If tests timeout, it may indicate:
1. Redis is slow or overloaded
2. Network latency issues
3. Too many concurrent tests running

### Flaky Tests

The tests use random API names to avoid conflicts. If you see flaky behavior:
1. Ensure Redis is not being used by other processes
2. Check Redis memory limits
3. Verify system resources are sufficient

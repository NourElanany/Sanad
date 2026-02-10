# Load Testing Guide - API Integration Service

## Overview

This guide describes the load testing implementation for the API Integration Service, covering:
- **Rate Limiting Under Load** (Requirement 9.1)
- **Caching Performance** (Requirement 10.1)
- **Fallback Mechanisms Under Failure** (Requirement 12.1)

## Test Implementation

### Test Files

- `tests/load_tests.rs` - Core load testing framework
- `tests/run_load_tests.rs` - Executable load test runner

### Test Scenarios

#### 1. Rate Limiting Under Load

**Objective**: Verify that rate limiting prevents exceeding API limits under high load.

**Test Method**:
- Generate high concurrent load (10x target RPS)
- Monitor for 429 (Too Many Requests) responses
- Verify rate limiting is enforced correctly

**Success Criteria**:
- Rate limiting triggers when limits are exceeded
- Requests are properly queued or rejected
- No API limits are violated

#### 2. Caching Performance

**Objective**: Verify that caching reduces API calls and improves performance.

**Test Method**:
- Make initial requests (cache miss scenario)
- Make repeated requests (cache hit scenario)
- Make concurrent requests to same resources
- Compare response times

**Success Criteria**:
- Cache hits are significantly faster than cache misses (>50% improvement)
- Cache hit rate is high for repeated requests (>80%)
- Cached responses are under 100ms

#### 3. Fallback Mechanisms Under Failure

**Objective**: Verify that fallback mechanisms maintain service availability when APIs fail.

**Test Method**:
- Simulate primary API failures
- Test multiple API failures
- Test stale cache fallback
- Monitor fallback usage

**Success Criteria**:
- Fallback APIs are used when primary fails
- Stale cache is served when all APIs fail
- Service remains available during failures
- Fallback events are properly logged

## Running Load Tests

### Prerequisites

1. **Start the API Integration Service**:
   ```bash
   cd services/api-integration-service
   cargo run --release
   ```

2. **Ensure Redis is running**:
   ```bash
   docker run -d -p 6379:6379 redis:latest
   ```

3. **Ensure PostgreSQL is running** (if needed):
   ```bash
   docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=password postgres:latest
   ```

### Running Tests

#### Basic Load Test

Run with default settings (50 concurrent users, 60 seconds):

```bash
cd services/api-integration-service
cargo test --test run_load_tests --release
```

#### Custom Configuration

Run with custom parameters:

```bash
# 100 concurrent users for 120 seconds
cargo test --test run_load_tests --release -- --users 100 --duration 120

# High load test: 200 users, 300 seconds, 500 RPS target
cargo test --test run_load_tests --release -- --users 200 --duration 300 --rps 500

# Quick test: 20 users, 30 seconds
cargo test --test run_load_tests --release -- --users 20 --duration 30
```

#### Selective Tests

Run specific test categories:

```bash
# Only rate limiting tests
cargo test --test run_load_tests --release -- --no-cache --no-fallback

# Only caching tests
cargo test --test run_load_tests --release -- --no-rate-limit --no-fallback

# Only fallback tests
cargo test --test run_load_tests --release -- --no-rate-limit --no-cache
```

#### Custom Service URL

Test against a different service instance:

```bash
API_BASE_URL=http://api.example.com cargo test --test run_load_tests --release
```

### Command Line Options

```
OPTIONS:
    -u, --users <NUM>        Number of concurrent users (default: 50)
    -d, --duration <SECS>    Test duration in seconds (default: 60)
    -r, --rps <NUM>          Target requests per second (default: 100)
    --no-rate-limit          Skip rate limiting tests
    --no-cache               Skip caching tests
    --no-fallback            Skip fallback tests
    -h, --help               Print help message

ENVIRONMENT VARIABLES:
    API_BASE_URL             Base URL of the API service (default: http://localhost:8080)
```

## Understanding Test Results

### Sample Output

```
🚀 Starting Load Tests for API Integration Service
Configuration: LoadTestConfig { concurrent_users: 50, test_duration: 60s, target_rps: 100, ... }
================================================================================

📊 Test 1: Rate Limiting Under Load
  Testing rate limiting with 50 concurrent users
  ✓ Completed 1000 requests in 10.23s
  ✓ Rate limited: 150 requests (15.0%)

💾 Test 2: Caching Performance
  Testing caching with repeated requests
  → Testing cache miss scenario...
  → Testing cache hit scenario...
  ✓ Cache miss avg: 245.32ms
  ✓ Cache hit avg: 12.45ms
  ✓ Performance improvement: 94.9%
  → Testing concurrent cache hits...

🔄 Test 3: Fallback Mechanisms Under Failure
  Testing fallback mechanisms
  → Testing primary API failure scenario...
  ✓ Fallback triggered: 23 times
  → Testing multiple API failures...
  → Testing stale cache fallback...


================================================================================
📊 LOAD TEST RESULTS SUMMARY
================================================================================

📈 Request Statistics:
  Total Requests:      1500
  Successful:          1425 (95.0%)
  Failed:              75 (5.0%)
  Rate Limited:        150 (10.0%)

💾 Caching Statistics:
  Cached Responses:    850 (56.7%)

🔄 Fallback Statistics:
  Fallback Used:       45 (3.0%)

⏱️  Response Time Statistics:
  Average:             125.45ms
  Minimum:             8.23ms
  Maximum:             2845.67ms
  P50 (Median):        98.34ms
  P95:                 456.78ms
  P99:                 1234.56ms

🚀 Throughput:
  Requests/Second:     25.00


================================================================================

✅ Validation Checks:
  ✓ Success rate: 95.0% (>= 95%)
  ✓ Rate limiting: Active (150 requests limited)
  ✓ Caching: Active (56.7% cache hit rate)
  ✓ Fallback: Active (45 fallback responses)
  ✓ Response time: P95 456.78ms (<= 3000ms)


✅ Load tests PASSED
```

### Metrics Explained

#### Request Statistics
- **Total Requests**: Total number of HTTP requests made
- **Successful**: Requests that returned 2xx status codes
- **Failed**: Requests that failed or returned error codes
- **Rate Limited**: Requests that received 429 (Too Many Requests)

#### Caching Statistics
- **Cached Responses**: Requests served from cache (indicated by `X-Cache-Status: HIT` header)
- **Cache Hit Rate**: Percentage of requests served from cache

#### Fallback Statistics
- **Fallback Used**: Number of times fallback mechanisms were triggered
- **Fallback Rate**: Percentage of requests that used fallback

#### Response Time Statistics
- **Average**: Mean response time across all requests
- **Minimum**: Fastest response time
- **Maximum**: Slowest response time
- **P50 (Median)**: 50% of requests completed faster than this
- **P95**: 95% of requests completed faster than this
- **P99**: 99% of requests completed faster than this

#### Throughput
- **Requests/Second**: Average number of requests processed per second

### Success Criteria

Tests are considered **PASSED** if:
1. ✅ Success rate >= 95%
2. ✅ P95 response time <= 3000ms
3. ✅ Rate limiting is active (when load exceeds limits)
4. ✅ Caching provides performance improvement
5. ✅ Fallback mechanisms work when needed

## Performance Benchmarks

### Expected Performance

| Metric | Target | Acceptable | Poor |
|--------|--------|------------|------|
| Success Rate | >= 99% | >= 95% | < 95% |
| P95 Response Time | <= 1000ms | <= 3000ms | > 3000ms |
| Cache Hit Rate | >= 80% | >= 50% | < 50% |
| Cache Performance Improvement | >= 90% | >= 50% | < 50% |
| Throughput | >= 100 RPS | >= 50 RPS | < 50 RPS |

### Typical Results

With default configuration (50 concurrent users):
- **Success Rate**: 95-99%
- **P95 Response Time**: 200-500ms
- **Cache Hit Rate**: 60-80%
- **Throughput**: 80-120 RPS

## Troubleshooting

### Service Not Available

**Error**: `Service is not available`

**Solution**:
1. Ensure the service is running: `cargo run --release`
2. Check the service is listening on the correct port
3. Verify no firewall is blocking connections

### High Failure Rate

**Error**: Success rate < 95%

**Possible Causes**:
1. Service is overloaded - reduce concurrent users
2. Database connection issues - check PostgreSQL
3. Redis connection issues - check Redis
4. External API failures - check API status

**Solutions**:
- Reduce load: `--users 20 --duration 30`
- Check service logs for errors
- Verify all dependencies are running

### Slow Response Times

**Error**: P95 response time > 3000ms

**Possible Causes**:
1. Cache not working properly
2. Database queries are slow
3. External APIs are slow
4. System resources exhausted

**Solutions**:
- Verify Redis is running and accessible
- Check database query performance
- Monitor system resources (CPU, memory)
- Reduce concurrent load

### Rate Limiting Not Triggered

**Warning**: Rate limiting not triggered

**Explanation**: This is not necessarily an error. It means the load was below the configured rate limits.

**To Test Rate Limiting**:
- Increase load: `--users 100 --rps 500`
- Reduce rate limits in configuration
- Focus on single endpoint

### No Cache Hits

**Warning**: No cache hits detected

**Possible Causes**:
1. Cache is disabled
2. Redis is not running
3. Cache TTL is too short
4. Requests are not identical

**Solutions**:
- Verify Redis is running: `redis-cli ping`
- Check cache configuration
- Ensure repeated requests use same parameters

## Integration with CI/CD

### GitHub Actions Example

```yaml
name: Load Tests

on:
  schedule:
    - cron: '0 2 * * *'  # Run daily at 2 AM
  workflow_dispatch:      # Allow manual trigger

jobs:
  load-tests:
    runs-on: ubuntu-latest
    
    services:
      redis:
        image: redis:latest
        ports:
          - 6379:6379
      
      postgres:
        image: postgres:latest
        env:
          POSTGRES_PASSWORD: password
        ports:
          - 5432:5432
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Start API Service
        run: |
          cd services/api-integration-service
          cargo build --release
          cargo run --release &
          sleep 10  # Wait for service to start
      
      - name: Run Load Tests
        run: |
          cd services/api-integration-service
          cargo test --test run_load_tests --release -- --users 50 --duration 60
      
      - name: Upload Results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: load-test-results
          path: services/api-integration-service/load-test-results.txt
```

## Best Practices

### 1. Start Small
Begin with low load and gradually increase:
```bash
# Start with 10 users
cargo test --test run_load_tests --release -- --users 10 --duration 30

# Increase to 50 users
cargo test --test run_load_tests --release -- --users 50 --duration 60

# Scale to 100 users
cargo test --test run_load_tests --release -- --users 100 --duration 120
```

### 2. Monitor System Resources
While running load tests, monitor:
- CPU usage: `top` or `htop`
- Memory usage: `free -h`
- Network: `netstat -an | grep 8080`
- Redis: `redis-cli info stats`

### 3. Run During Off-Peak Hours
For production testing, run during low-traffic periods to minimize impact.

### 4. Use Realistic Data
Ensure test data represents actual usage patterns:
- Mix of different API endpoints
- Realistic request parameters
- Varied user behaviors

### 5. Document Results
Keep records of load test results to track performance over time:
```bash
cargo test --test run_load_tests --release 2>&1 | tee load-test-$(date +%Y%m%d-%H%M%S).log
```

## Advanced Scenarios

### Stress Testing

Push the system to its limits:
```bash
cargo test --test run_load_tests --release -- --users 500 --duration 300 --rps 1000
```

### Endurance Testing

Test system stability over extended periods:
```bash
cargo test --test run_load_tests --release -- --users 100 --duration 3600  # 1 hour
```

### Spike Testing

Test system response to sudden load increases:
1. Start with low load
2. Quickly increase to high load
3. Monitor recovery

### Soak Testing

Test for memory leaks and resource exhaustion:
```bash
cargo test --test run_load_tests --release -- --users 50 --duration 7200  # 2 hours
```

## Conclusion

Load testing is essential for ensuring the API Integration Service can handle production traffic while maintaining:
- ✅ Rate limiting enforcement
- ✅ Caching performance
- ✅ Fallback reliability
- ✅ Overall system stability

Regular load testing helps identify performance bottlenecks and ensures the system meets SLA requirements.

## Related Documentation

- [API Documentation](docs/API_DOCUMENTATION.md)
- [Deployment Guide](docs/DEPLOYMENT_GUIDE.md)
- [Developer Guide](docs/DEVELOPER_GUIDE.md)
- [Configuration Guide](../config/CONFIGURATION_GUIDE.md)

# Structured Logging Implementation Guide

## Overview

This document describes the structured logging implementation for the API Integration Service, which provides comprehensive observability for all API calls with correlation IDs and timing information.

## Features

### 1. Structured Logging with Tracing

The service uses the `tracing` crate for structured logging, which provides:
- Hierarchical span-based logging
- Structured fields for easy filtering and analysis
- Integration with observability tools (Prometheus, Jaeger, etc.)
- Performance-optimized logging

### 2. Correlation IDs

Every request is assigned a unique correlation ID that:
- Tracks requests across the entire service lifecycle
- Propagates through all API calls to external services
- Appears in all log entries related to the request
- Is returned in response headers for client-side tracking

**Header Name**: `X-Request-ID`

**Generation**:
- If the client provides an `X-Request-ID` header, it is preserved
- Otherwise, a new UUID v4 is generated

### 3. Request Context

The `RequestContext` provides task-local storage for:
- Correlation ID
- User ID (for authenticated requests)
- Request start time
- Other request metadata

**Usage**:
```rust
use api_integration_service::{RequestContext, with_context};

// Create a context
let context = RequestContext::new("correlation-id-123".to_string());

// Run code within the context
with_context(context, async {
    // Correlation ID is available throughout this scope
    let id = current_correlation_id();
    // ... your code here
}).await;
```

### 4. API Call Logging

The `log_api_call` function provides automatic logging for API calls:

```rust
use shared::api_clients::{log_api_call, LogApiResult};

// Start logging an API call
let logger = log_api_call("quran.com", "get_ayah", Some("correlation-id"));

// Make the API call
let result = make_api_call().await;

// Log the result (automatically logs success or failure)
result.log_result(logger)
```

**Logged Information**:
- API name
- Operation name
- Correlation ID
- Duration (in milliseconds)
- Status (success, failure, cached, fallback)
- Error details (for failures)

## Log Levels

The service uses the following log levels:

- **ERROR**: API failures, server errors, critical issues
- **WARN**: Fallbacks, rate limit warnings, client errors (4xx)
- **INFO**: Successful requests, API calls, service lifecycle events
- **DEBUG**: Detailed debugging information, metrics
- **TRACE**: Very detailed tracing information

## Log Format

All logs are structured with the following fields:

### Request Logs
```json
{
  "timestamp": "2024-01-15T10:30:45.123Z",
  "level": "INFO",
  "request_id": "550e8400-e29b-41d4-a716-446655440000",
  "method": "GET",
  "uri": "/api/quran/1/1",
  "message": "Incoming request"
}
```

### API Call Logs
```json
{
  "timestamp": "2024-01-15T10:30:45.234Z",
  "level": "INFO",
  "request_id": "550e8400-e29b-41d4-a716-446655440000",
  "api_name": "quran.com",
  "operation": "get_ayah",
  "duration_ms": 145,
  "status": "success",
  "message": "API call completed successfully"
}
```

### Response Logs
```json
{
  "timestamp": "2024-01-15T10:30:45.345Z",
  "level": "INFO",
  "request_id": "550e8400-e29b-41d4-a716-446655440000",
  "method": "GET",
  "uri": "/api/quran/1/1",
  "status": 200,
  "duration_ms": 234,
  "message": "Request completed successfully"
}
```

## Middleware Stack

The service applies middleware in the following order (outermost to innermost):

1. **CORS Layer**: Handles cross-origin requests
2. **Security Headers**: Adds security headers to responses
3. **Timeout**: Enforces request timeouts
4. **Metrics**: Collects request metrics
5. **Logging**: Logs requests and responses
6. **Request ID**: Generates/preserves correlation IDs and sets up request context
7. **Tower Trace Layer**: Additional HTTP tracing

## Configuration

### Log Level

Set the log level via environment variable:
```bash
RUST_LOG=info  # Options: error, warn, info, debug, trace
```

### Structured Logging Format

The service uses the default `tracing_subscriber` format with:
- Thread IDs enabled
- File and line numbers enabled
- Target disabled (to reduce noise)

### Customization

To customize logging in `main.rs`:
```rust
tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .with_target(false)
    .with_thread_ids(true)
    .with_file(true)
    .with_line_number(true)
    .json()  // Optional: Use JSON format
    .init();
```

## Best Practices

### 1. Always Use Correlation IDs

When making API calls, always pass the correlation ID:
```rust
let correlation_id = current_correlation_id();
let logger = log_api_call("api_name", "operation", correlation_id.as_deref());
```

### 2. Log at Appropriate Levels

- Use `error!` for failures that require attention
- Use `warn!` for degraded service (fallbacks, rate limits)
- Use `info!` for normal operations
- Use `debug!` for detailed debugging
- Use `trace!` for very detailed tracing

### 3. Include Structured Fields

Always include relevant structured fields:
```rust
info!(
    request_id = %correlation_id,
    api_name = %api_name,
    operation = %operation,
    duration_ms = %duration.as_millis(),
    "API call completed"
);
```

### 4. Don't Log Sensitive Data

Never log:
- API keys or tokens
- User passwords
- Personal information (PII)
- Credit card numbers

### 5. Use the LogApiResult Trait

For automatic result logging:
```rust
let result = api_call().await.log_result(logger);
```

## Integration with Observability Tools

### Prometheus Metrics

The service exposes metrics for:
- Request count by endpoint and status
- Request duration histograms
- API call counts and durations
- Cache hit/miss rates
- Rate limit usage

### Distributed Tracing

The service supports distributed tracing with:
- OpenTelemetry integration
- Span propagation across services
- Correlation ID propagation

### Log Aggregation

Logs can be aggregated using:
- ELK Stack (Elasticsearch, Logstash, Kibana)
- Grafana Loki
- CloudWatch Logs
- Datadog

## Examples

### Example 1: Basic API Call Logging

```rust
use shared::api_clients::{log_api_call, LogApiResult};

async fn fetch_ayah(surah: u8, ayah: u16) -> Result<AyahData, ApiError> {
    let correlation_id = current_correlation_id();
    let logger = log_api_call("quran.com", "get_ayah", correlation_id.as_deref());
    
    let result = make_api_request(surah, ayah).await;
    result.log_result(logger)
}
```

### Example 2: Fallback Logging

```rust
let logger = log_api_call("quran.com", "get_ayah", Some("correlation-id"));

match primary_api.get_ayah(1, 1).await {
    Ok(data) => {
        logger.success(&data);
        Ok(data)
    }
    Err(e) => {
        logger.fallback("alquran.cloud", "primary API failed");
        fallback_api.get_ayah(1, 1).await
    }
}
```

### Example 3: Cached Response Logging

```rust
let logger = log_api_call("quran.com", "get_ayah", Some("correlation-id"));

if let Some(cached) = cache.get(&key).await? {
    return Ok(logger.cached(&cached));
}

let result = api.get_ayah(1, 1).await?;
cache.set(&key, &result).await?;
logger.success(&result)
```

## Troubleshooting

### Logs Not Appearing

1. Check the `RUST_LOG` environment variable
2. Ensure `tracing_subscriber` is initialized in `main.rs`
3. Verify log level is appropriate for the messages

### Correlation IDs Missing

1. Ensure `request_id_middleware` is applied
2. Check that the middleware is in the correct order
3. Verify the request context is being set up

### Performance Issues

1. Reduce log level in production (use `info` or `warn`)
2. Disable file/line numbers if not needed
3. Use async logging for high-throughput scenarios

## Future Enhancements

1. **Sampling**: Implement log sampling for high-volume endpoints
2. **Dynamic Log Levels**: Allow changing log levels without restart
3. **Structured Metrics**: Enhanced Prometheus metrics integration
4. **Trace Visualization**: Integration with Jaeger or Zipkin
5. **Log Rotation**: Automatic log file rotation and archival

## References

- [Tracing Documentation](https://docs.rs/tracing/)
- [OpenTelemetry](https://opentelemetry.io/)
- [Structured Logging Best Practices](https://www.honeycomb.io/blog/structured-logging-and-your-team)

# AI Service Integration Guide

This guide covers the complete setup and usage of the AI service integration for the Islamic application, including Hugging Face models and Qdrant vector database integration with comprehensive caching and fallback strategies.

## Overview

The AI service integration provides:

- **Hugging Face Integration**: Connection to Islamic-specialized AI models
- **Vector Database**: Qdrant for semantic search and RAG operations
- **Intelligent Caching**: Multi-level caching with Redis and local fallback
- **Error Handling**: Comprehensive error handling with circuit breakers
- **Fallback Strategies**: Multiple fallback mechanisms for high availability
- **Rate Limiting**: Adaptive rate limiting to prevent API abuse
- **Monitoring**: Health checks and metrics collection

## Quick Start

### Prerequisites

- **Rust** (latest stable version)
- **Docker** and **Docker Compose**
- **Hugging Face API Key** (get from [huggingface.co/settings/tokens](https://huggingface.co/settings/tokens))

### Automated Setup

#### Linux/macOS
```bash
# Make the script executable
chmod +x scripts/setup_ai_integration.sh

# Run the setup script
./scripts/setup_ai_integration.sh
```

#### Windows (PowerShell)
```powershell
# Run the setup script
.\scripts\setup_ai_integration.ps1
```

### Manual Setup

1. **Start Required Services**
   ```bash
   # Start Qdrant and Redis
   docker-compose -f docker-compose.ai-services.yml up -d qdrant redis
   ```

2. **Set Environment Variables**
   ```bash
   export HUGGING_FACE_API_KEY="your_api_key_here"
   export QDRANT_HOST="localhost"
   export QDRANT_PORT="6333"
   export REDIS_URL="redis://localhost:6379"
   ```

3. **Build and Test**
   ```bash
   # Build the project
   cargo build --release
   
   # Run tests
   cargo test ai_service
   
   # Run integration example
   cargo run --example ai_service_integration
   ```

## Configuration

### Main Configuration File

The main configuration is in `config/ai_service_config.yaml`:

```yaml
# Hugging Face Configuration
hugging_face:
  api_key: "${HUGGING_FACE_API_KEY}"
  base_url: "https://api-inference.huggingface.co"
  timeout_seconds: 30
  max_retries: 3
  requests_per_minute: 60
  default_model: "microsoft/DialoGPT-medium"
  embedding_model: "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2"
  
  islamic_models:
    - name: "Arabic Islamic General"
      model_id: "aubmindlab/bert-base-arabertv02"
      specialization: "General"
      language: "Arabic"
      priority: 1

# Vector Database Configuration
vector_database:
  host: "localhost"
  port: 6333
  collection_name: "islamic_sources"
  vector_size: 384
  distance_metric: "Cosine"

# Cache Configuration
cache:
  enable_response_cache: true
  enable_embedding_cache: true
  response_cache_ttl_seconds: 1800
  embedding_cache_ttl_seconds: 7200
  redis:
    url: "redis://localhost:6379"

# Fallback Configuration
fallback:
  enable_fallback: true
  fallback_models:
    - "aubmindlab/bert-base-arabertv02"
    - "CAMeL-Lab/bert-base-arabic-camelbert-mix"
  enable_offline_mode: true
  offline_responses:
    default: "عذراً، الخدمة غير متاحة حالياً. يرجى المحاولة لاحقاً أو استشارة العلماء المختصين."
```

### Environment Variables

Create a `.env` file in the project root:

```env
# Required
HUGGING_FACE_API_KEY=your_api_key_here

# Optional (with defaults)
QDRANT_HOST=localhost
QDRANT_PORT=6333
REDIS_URL=redis://localhost:6379
LOG_LEVEL=INFO
AI_SERVICE_CONFIG=config/ai_service_config.yaml
RUST_LOG=info
```

## Usage

### Basic Usage

```rust
use sanad::ai_service::{
    config::AIServiceConfig,
    service_manager::AIServiceManager,
    integration_service::RAGProcessingRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = AIServiceConfig::from_file("config/ai_service_config.yaml")?;
    
    // Initialize service manager
    let service_manager = AIServiceManager::new(config)?;
    let init_result = service_manager.initialize().await?;
    
    if !init_result.success {
        eprintln!("Failed to initialize services: {:?}", init_result.services_failed);
        return Err("Initialization failed".into());
    }
    
    // Get integration service
    let mut service = service_manager.get_integration_service().await
        .ok_or("Integration service not available")?;
    
    // Process a question
    let request = RAGProcessingRequest {
        question: "ما هي أركان الإسلام؟".to_string(),
        context: None,
        max_sources: Some(5),
        similarity_threshold: Some(0.7),
        preferred_source_types: Some(vec!["quran".to_string(), "hadith".to_string()]),
        language: Some("Arabic".to_string()),
        user_id: Some("user123".to_string()),
    };
    
    let response = service.process_rag_request(request).await?;
    
    println!("Answer: {}", response.answer);
    println!("Confidence: {:.2}", response.confidence);
    println!("Sources: {}", response.sources.len());
    println!("Processing time: {}ms", response.processing_time_ms);
    
    Ok(())
}
```

### Advanced Usage with Error Handling

```rust
use sanad::ai_service::{
    service_manager::AIServiceManager,
    error_handler::{ErrorHandler, ErrorHandlerConfig, ErrorContext},
    integration_service::RAGProcessingRequest,
};

async fn process_with_error_handling(
    service_manager: &AIServiceManager,
    question: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut error_handler = ErrorHandler::new(ErrorHandlerConfig::default());
    let mut attempt = 1;
    const MAX_ATTEMPTS: u32 = 3;
    
    loop {
        let context = ErrorContext::new(
            "rag_processing".to_string(),
            "integration_service".to_string(),
        );
        
        match service_manager.get_integration_service().await {
            Some(mut service) => {
                let request = RAGProcessingRequest {
                    question: question.to_string(),
                    context: None,
                    max_sources: Some(5),
                    similarity_threshold: Some(0.7),
                    preferred_source_types: None,
                    language: Some("Arabic".to_string()),
                    user_id: None,
                };
                
                match service.process_rag_request(request).await {
                    Ok(response) => {
                        error_handler.record_result("integration_service", true);
                        return Ok(response.answer);
                    }
                    Err(e) => {
                        error_handler.record_result("integration_service", false);
                        
                        if attempt >= MAX_ATTEMPTS {
                            return Err(e.into());
                        }
                        
                        let recovery_action = error_handler.handle_error(&e, &context, attempt).await;
                        
                        match error_handler.execute_recovery_action(recovery_action, &context).await {
                            Ok(Some(fallback_response)) => {
                                return Ok(fallback_response);
                            }
                            Ok(None) => {
                                // Retry
                                attempt += 1;
                                continue;
                            }
                            Err(recovery_error) => {
                                return Err(recovery_error.into());
                            }
                        }
                    }
                }
            }
            None => {
                return Err("Integration service not available".into());
            }
        }
    }
}
```

## Architecture

### Service Components

```
┌─────────────────────────────────────────────────────────────┐
│                    AI Service Manager                        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │ Hugging Face    │  │ Vector Database │  │ Cache Manager   │ │
│  │ Client          │  │ Client          │  │                 │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │ Error Handler   │  │ Circuit Breaker │  │ Rate Limiter    │ │
│  │                 │  │                 │  │                 │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│                Integration Service                           │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

```
User Request
     │
     ▼
┌─────────────────┐
│ Service Manager │
└─────────────────┘
     │
     ▼
┌─────────────────┐    ┌─────────────────┐
│ Error Handler   │───▶│ Circuit Breaker │
└─────────────────┘    └─────────────────┘
     │
     ▼
┌─────────────────┐    ┌─────────────────┐
│ Cache Check     │───▶│ Cache Hit?      │
└─────────────────┘    └─────────────────┘
     │                          │
     ▼ (Cache Miss)              ▼ (Cache Hit)
┌─────────────────┐         ┌─────────────────┐
│ Vector Search   │         │ Return Cached   │
└─────────────────┘         └─────────────────┘
     │
     ▼
┌─────────────────┐
│ Hugging Face    │
│ Generation      │
└─────────────────┘
     │
     ▼
┌─────────────────┐
│ Response        │
│ Validation      │
└─────────────────┘
     │
     ▼
┌─────────────────┐
│ Cache Store     │
└─────────────────┘
     │
     ▼
   Response
```

## Error Handling and Fallbacks

### Error Types and Strategies

| Error Type | Retry Strategy | Fallback Strategy |
|------------|----------------|-------------------|
| Network Error | Exponential backoff (3 attempts) | Use cache |
| Rate Limit | Linear backoff (5 attempts) | Service degradation |
| Model Loading | Fixed delay (3 attempts) | Different model |
| Authentication | No retry | Fail gracefully |
| Service Unavailable | Exponential backoff (2 attempts) | Offline response |

### Circuit Breaker

The circuit breaker protects against cascading failures:

- **Closed**: Normal operation
- **Open**: Service is failing, reject requests
- **Half-Open**: Testing if service recovered

Configuration:
```yaml
circuit_breaker:
  failure_threshold: 5      # Trip after 5 failures
  recovery_timeout_seconds: 60  # Wait 60s before testing
  half_open_max_calls: 3    # Allow 3 test calls
```

### Fallback Responses

When all else fails, the system provides contextual offline responses:

```yaml
offline_responses:
  default: "عذراً، الخدمة غير متاحة حالياً. يرجى المحاولة لاحقاً أو استشارة العلماء المختصين."
  network_error: "حدث خطأ في الاتصال بالشبكة. يرجى التحقق من اتصالك بالإنترنت والمحاولة مرة أخرى."
  service_unavailable: "الخدمة غير متاحة مؤقتاً. نعمل على حل المشكلة. يرجى المحاولة لاحقاً."
```

## Caching Strategy

### Multi-Level Caching

1. **L1 Cache**: In-memory local cache (fastest)
2. **L2 Cache**: Redis distributed cache (shared)
3. **L3 Cache**: Vector database (persistent)

### Cache Types

- **Query Cache**: Stores search results (1 hour TTL)
- **Response Cache**: Stores generated responses (30 minutes TTL)
- **Embedding Cache**: Stores text embeddings (2 hours TTL)

### Cache Keys

Cache keys are generated using content hashing:
```rust
fn generate_cache_key(request: &RAGProcessingRequest) -> String {
    let mut hasher = DefaultHasher::new();
    request.question.hash(&mut hasher);
    request.context.hash(&mut hasher);
    request.max_sources.hash(&mut hasher);
    format!("rag_request_{:x}", hasher.finish())
}
```

## Monitoring and Metrics

### Health Checks

The service provides comprehensive health checks:

```rust
let health_status = service_manager.get_health_status().await;
println!("Overall: {:?}", health_status.overall_status);
println!("Hugging Face: {:?}", health_status.hugging_face_status);
println!("Vector DB: {:?}", health_status.vector_db_status);
println!("Cache: {:?}", health_status.cache_status);
```

### Metrics Collection

Available metrics:
- Total requests
- Success/failure rates
- Average response time
- Cache hit rates
- Fallback usage rates
- Circuit breaker trips

### Monitoring Setup

Start monitoring services:
```bash
docker-compose -f docker-compose.ai-services.yml --profile monitoring up -d
```

Access monitoring:
- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3000 (admin/admin)

## Performance Optimization

### Connection Pooling

Configure connection pools for better performance:

```yaml
production:
  connection_pools:
    hugging_face_pool_size: 10
    vector_db_pool_size: 20
    redis_pool_size: 15
  
  performance:
    max_concurrent_requests: 100
    request_timeout_seconds: 45
    batch_processing_size: 50
```

### Batch Processing

For bulk operations, use batch processing:

```rust
let documents = vec![/* your documents */];
let result = vector_db_client.index_documents_batch(documents).await?;
println!("Indexed {} documents successfully", result.successful_count);
```

## Troubleshooting

### Common Issues

#### 1. Hugging Face API Key Issues
```
Error: authentication failed
```
**Solution**: Check your API key in the `.env` file and ensure it's valid.

#### 2. Qdrant Connection Issues
```
Error: Failed to connect to Qdrant
```
**Solution**: Ensure Qdrant is running:
```bash
docker-compose -f docker-compose.ai-services.yml up -d qdrant
```

#### 3. Redis Connection Issues
```
Error: Redis connection failed
```
**Solution**: The system will fall back to local caching. To fix:
```bash
docker-compose -f docker-compose.ai-services.yml up -d redis
```

#### 4. Model Loading Timeout
```
Error: Model did not become ready within 120 seconds
```
**Solution**: Some models take time to load on first use. Wait and retry.

### Debug Mode

Enable debug mode for detailed logging:

```yaml
development:
  enable_debug_mode: true
```

Or set environment variable:
```bash
export RUST_LOG=debug
```

### Log Analysis

Check service logs:
```bash
# View all service logs
docker-compose -f docker-compose.ai-services.yml logs -f

# View specific service logs
docker-compose -f docker-compose.ai-services.yml logs -f qdrant
docker-compose -f docker-compose.ai-services.yml logs -f redis
```

## Security Considerations

### API Key Management

- Store API keys in environment variables, never in code
- Use different keys for development and production
- Rotate keys regularly
- Monitor API key usage

### Network Security

- Use HTTPS for all external API calls
- Implement proper firewall rules
- Use VPN for production deployments
- Enable Redis AUTH if exposed

### Data Privacy

- Encrypt sensitive data at rest
- Use TLS for data in transit
- Implement proper access controls
- Log access attempts

## Production Deployment

### Docker Production Setup

```yaml
# docker-compose.prod.yml
version: '3.8'
services:
  qdrant:
    image: qdrant/qdrant:latest
    restart: always
    volumes:
      - qdrant_prod_data:/qdrant/storage
    environment:
      - QDRANT__SERVICE__HTTP_PORT=6333
      - QDRANT__LOG_LEVEL=WARN
    deploy:
      resources:
        limits:
          memory: 2G
          cpus: '1.0'

  redis:
    image: redis:7-alpine
    restart: always
    command: redis-server --requirepass ${REDIS_PASSWORD}
    volumes:
      - redis_prod_data:/data
    deploy:
      resources:
        limits:
          memory: 512M
          cpus: '0.5'
```

### Environment Configuration

Production environment variables:
```env
# Production settings
HUGGING_FACE_API_KEY=prod_api_key_here
QDRANT_HOST=qdrant-prod.internal
REDIS_URL=redis://:password@redis-prod.internal:6379
LOG_LEVEL=WARN
RUST_LOG=warn

# Security
ENABLE_REQUEST_VALIDATION=true
ENABLE_RESPONSE_SANITIZATION=true
MAX_REQUEST_SIZE_MB=10
ENABLE_AUDIT_LOGGING=true
```

### Scaling Considerations

- Use multiple Qdrant nodes for high availability
- Implement Redis clustering for cache scaling
- Use load balancers for API distribution
- Monitor resource usage and scale accordingly

## API Reference

### Service Manager

```rust
impl AIServiceManager {
    pub fn new(config: AIServiceConfig) -> Result<Self>;
    pub async fn initialize(&self) -> Result<InitializationResult>;
    pub async fn get_health_status(&self) -> ServiceHealth;
    pub async fn get_metrics(&self) -> ServiceMetrics;
    pub async fn get_integration_service(&self) -> Option<IntegrationService>;
    pub async fn shutdown(&self) -> Result<()>;
}
```

### Integration Service

```rust
impl IntegrationService {
    pub async fn process_rag_request(&mut self, request: RAGProcessingRequest) -> Result<RAGProcessingResponse>;
    pub async fn index_content(&mut self, content: IslamicSource) -> Result<()>;
    pub async fn health_check(&self) -> Result<ServiceHealthStatus>;
}
```

### Error Handler

```rust
impl ErrorHandler {
    pub fn new(config: ErrorHandlerConfig) -> Self;
    pub async fn handle_error(&mut self, error: &AIServiceError, context: &ErrorContext, attempt: u32) -> RecoveryAction;
    pub async fn execute_recovery_action(&mut self, action: RecoveryAction, context: &ErrorContext) -> Result<Option<String>>;
    pub fn record_result(&mut self, service: &str, success: bool);
}
```

## Contributing

### Development Setup

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run the test suite
6. Submit a pull request

### Testing

Run all tests:
```bash
# Unit tests
cargo test ai_service --lib

# Integration tests
cargo test ai_service::integration_tests --lib

# Property-based tests
cargo test ai_service::property_tests --lib

# Example tests
cargo run --example ai_service_integration
```

### Code Style

Follow Rust conventions:
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Add documentation for public APIs
- Write comprehensive tests

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Support

For support and questions:

1. Check the troubleshooting section
2. Review the logs for error details
3. Open an issue on GitHub
4. Contact the development team

---

**Note**: This integration is designed specifically for Islamic content and applications. The models and configurations are optimized for Arabic text and Islamic concepts.
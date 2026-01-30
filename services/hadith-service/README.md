# Hadith Service

The Hadith Service is a comprehensive microservice for managing and searching Islamic prophetic traditions (Hadiths) with advanced features including authenticity grading, thematic classification, and semantic search capabilities.

## Features

### Core Functionality
- **Comprehensive Hadith Management**: Store and manage Hadiths with complete metadata
- **Authenticity Grading**: Support for Sahih, Hasan, Daif, and Mawdu classifications
- **Sanad Management**: Chain of narration tracking with integrity verification
- **Scholar Information**: Detailed scholar profiles with credibility scoring
- **Book Organization**: Hierarchical organization by books and chapters

### Advanced Search
- **Text Search**: Full-text search with Arabic language support
- **Semantic Search**: Meaning-based search capabilities
- **Narrator Search**: Search by narrator names and chains
- **Theme Search**: Topical and thematic classification search
- **Exact Search**: Precise text matching

### Data Integrity
- **Content Verification**: SHA-256 hash verification for all Islamic content
- **Digital Signatures**: Cryptographic integrity for authentic texts
- **Audit Trail**: Complete tracking of content changes and updates

### API Endpoints

#### Hadith Management
- `GET /api/v1/hadiths/{id}` - Get Hadith by ID
- `GET /api/v1/hadiths/number/{number}/book/{book}` - Get Hadith by number and book
- `POST /api/v1/hadiths` - Create new Hadith
- `GET /api/v1/hadiths` - List Hadiths with filtering

#### Search
- `GET /api/v1/search` - Advanced Hadith search
- `GET /api/v1/search/suggestions` - Get search suggestions

#### Books and Organization
- `GET /api/v1/books` - List all Hadith books
- `POST /api/v1/books` - Create new Hadith book
- `GET /api/v1/books/{book}/hadiths` - Get Hadiths by book
- `GET /api/v1/books/{id}/chapters` - Get book chapters

#### Topics and Themes
- `GET /api/v1/topics/{topic}` - Get Hadiths by topic

#### Analytics
- `GET /api/v1/analytics` - Get Hadith analytics and statistics

#### System
- `GET /health` - Health check
- `POST /api/v1/integrity/verify` - Verify content integrity

## Requirements Validation

This service validates the following requirements:

### المتطلبات 3.1، 3.2، 3.3، 3.4، 3.5، 3.6

- **3.1**: Complete Hadith text display with chain of narration
- **3.2**: Authenticity grading system (Sahih, Hasan, Daif, Mawdu)
- **3.3**: Source linking to authentic Hadith collections
- **3.4**: Scholarly explanations with verified scholars
- **3.5**: Thematic classification and topical organization
- **3.6**: Advanced search across Hadith texts and metadata

## Architecture

### Models
- **Hadith**: Core Hadith model with integrity verification
- **Sanad**: Chain of narration with authenticity grading
- **Scholar**: Scholar information with credibility scoring
- **HadithBook**: Book metadata and organization
- **HadithChapter**: Chapter organization within books
- **HadithExplanation**: Scholarly explanations and commentary

### Services
- **HadithService**: Business logic and orchestration
- **HadithRepository**: Data access and persistence
- **Handlers**: HTTP API endpoints and request handling

### Database Schema
- PostgreSQL with advanced indexing for Arabic text
- Full-text search capabilities with Arabic language support
- GIN indexes for array fields (themes, keywords, narrators)
- Integrity constraints and triggers for data consistency

## Testing

### Unit Tests
- Model validation and integrity verification
- Business logic testing
- API endpoint testing
- Error handling and edge cases

### Property-Based Tests
- **Comprehensive Thematic Classification**: Validates Requirements 3.5
- **Thematic Search Functionality**: Validates search and filtering
- **Classification Consistency**: Ensures consistent categorization

### Integration Tests
- End-to-end service functionality
- Database integration
- API endpoint integration
- Performance testing with large datasets

## Usage

### Starting the Service

```bash
# Set environment variables
export SANAD_DATABASE_URL="postgresql://user:password@localhost/sanad"
export SANAD_SERVER_PORT=3003

# Run the service
cargo run
```

### Example API Calls

```bash
# Search for Hadiths about faith
curl "http://localhost:3003/api/v1/search?q=إيمان&type=text&limit=10"

# Get a specific Hadith
curl "http://localhost:3003/api/v1/hadiths/1"

# Get Hadiths by topic
curl "http://localhost:3003/api/v1/topics/عقيدة"

# Get all Hadith books
curl "http://localhost:3003/api/v1/books"
```

### Configuration

The service uses the shared configuration system with the following key settings:

```yaml
database:
  url: "postgresql://localhost/sanad"
  max_connections: 10
  
server:
  host: "0.0.0.0"
  port: 3003
  
security:
  jwt_secret: "your-secret-key"
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests
cargo test --test integration_test

# Run property-based tests
cargo test prop_

# Run verification script
./scripts/verify_integration.ps1
```

### Code Quality

- **Linting**: `cargo clippy`
- **Formatting**: `cargo fmt`
- **Security Audit**: `cargo audit`

## Performance

- **Search Performance**: Optimized for sub-second response times
- **Concurrent Requests**: Supports high concurrency with connection pooling
- **Memory Usage**: Efficient memory management with streaming for large datasets
- **Caching**: Redis integration for frequently accessed data

## Security

- **Content Integrity**: SHA-256 verification for all Islamic texts
- **Input Validation**: Comprehensive input sanitization
- **Rate Limiting**: Protection against abuse
- **Authentication**: JWT-based authentication support
- **CORS**: Configurable cross-origin resource sharing

## Monitoring

- **Health Checks**: Built-in health check endpoints
- **Metrics**: Performance and usage metrics
- **Logging**: Structured logging with tracing support
- **Error Tracking**: Comprehensive error reporting

## Deployment

### Docker

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/hadith-service /usr/local/bin/
EXPOSE 3003
CMD ["hadith-service"]
```

### Environment Variables

- `SANAD_DATABASE_URL`: PostgreSQL connection string
- `SANAD_SERVER_HOST`: Server host (default: 0.0.0.0)
- `SANAD_SERVER_PORT`: Server port (default: 3003)
- `SANAD_REDIS_URL`: Redis connection string for caching
- `SANAD_LOGGING_LEVEL`: Log level (debug, info, warn, error)

## Contributing

1. Follow the existing code style and patterns
2. Add tests for new functionality
3. Update documentation for API changes
4. Ensure all tests pass before submitting
5. Use meaningful commit messages

## License

This project is part of the Sanad Islamic Application Platform.
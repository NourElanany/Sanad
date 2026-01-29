# Quran Service

## Overview

The Quran Service is a core microservice in the Sanad Islamic Application that provides comprehensive access to the Quran, including Surahs, Ayahs, and Tafsir (interpretations). It implements robust data integrity verification using SHA-256 hashing and provides a RESTful API for all Quran-related operations.

## Features

### Core Data Models

- **Surah**: Represents a chapter of the Quran with metadata
- **Ayah**: Represents a verse with integrity verification
- **Tafsir**: Represents interpretations with source attribution
- **TafsirSource**: Represents interpretation sources and authors

### Data Integrity

- SHA-256 hash verification for all Quranic text
- Content integrity checking endpoints
- Tamper detection for critical Islamic content

### API Endpoints

#### Health & Status
- `GET /health` - Service health check

#### Surahs
- `GET /surahs` - Get all Surahs
- `GET /surahs/{surah_number}` - Get specific Surah
- `GET /surahs/{surah_number}/ayahs` - Get all Ayahs in a Surah

#### Ayahs
- `GET /surahs/{surah_number}/ayahs/{ayah_number}` - Get specific Ayah
- `GET /surahs/{surah_number}/ayahs/{ayah_number}/navigation` - Get navigation info
- `GET /ayahs/range` - Get range of Ayahs (for Khatma planning)

#### Tafsir
- `GET /surahs/{surah_number}/ayahs/{ayah_number}/tafsir` - Get Tafsir for Ayah
- `GET /tafsir/sources` - Get all Tafsir sources

#### Search & Statistics
- `GET /search` - Search in Quran text with advanced options
  - Query params: `q`, `type` (text/semantic/root/exact), `surahs`, `revelation`, `juz`, `limit`, `offset`
- `GET /search/advanced` - Advanced search with filters
- `GET /search/suggestions` - Get search suggestions
- `GET /statistics` - Get Quran statistics
- `POST /integrity/verify` - Verify content integrity

#### Translations & Recitations
- `GET /surahs/{surah_number}/ayahs/{ayah_number}/translations` - Get translations
- `GET /recitation/styles` - Get available recitation styles

#### Navigation & Organization
- `GET /surahs/revelation/{revelation_type}` - Get Surahs by revelation type
- `GET /juz/{juz_number}/ayahs` - Get Ayahs by Juz (1-30)
- `GET /pages/{page_number}/ayahs` - Get Ayahs by page (1-604)

## Architecture

### Layers

1. **Handlers** (`handlers.rs`) - HTTP request/response handling
2. **Service** (`service.rs`) - Business logic and validation
3. **Repository** (`repository.rs`) - Database operations
4. **Models** (`models.rs`) - Data structures and domain logic

### Database Schema

The service uses PostgreSQL with the following key tables:
- `surahs` - Surah metadata
- `ayahs` - Ayah text with integrity hashes
- `tafsir_sources` - Interpretation sources
- `tafsir` - Interpretation entries
- `translations` - Multi-language translations
- `recitation_styles` - Available recitation styles (Qira'at)

### Key Features Implemented

#### 1. Content Integrity Verification
```rust
impl Ayah {
    pub fn verify_integrity(&self) -> bool {
        let calculated_hash = Self::calculate_text_hash(&self.text);
        calculated_hash == self.text_hash
    }
}
```

#### 2. Comprehensive Search
- Multiple search types: text, semantic, root-based, exact
- Full-text search using PostgreSQL's Arabic text search
- Relevance scoring with `ts_rank`
- Advanced filtering by Surah, revelation type, Juz, page range
- Search suggestions and auto-completion
- Pagination support

#### 3. Navigation Support
- Previous/Next Ayah navigation
- Cross-Surah navigation
- Ayah range queries for reading plans

#### 4. Tafsir Integration
- Multiple interpretation sources
- Source attribution and metadata
- Filtered queries by source

## Usage Examples

### Get a Surah with Ayahs
```bash
curl "http://localhost:8081/surahs/1?include_ayahs=true"
```

### Search with Different Types
```bash
# Text search
curl "http://localhost:8081/search?q=الحمد&type=text&limit=10"

# Exact phrase search
curl "http://localhost:8081/search?q=بسم الله&type=exact"

# Search in specific revelation type
curl "http://localhost:8081/search?q=صلاة&revelation=medinan"
```

### Get Translations
```bash
curl "http://localhost:8081/surahs/1/ayahs/1/translations?languages=en,ur"
```

### Get Ayahs by Juz or Page
```bash
# Get all Ayahs in Juz 1
curl "http://localhost:8081/juz/1/ayahs"

# Get all Ayahs on page 1
curl "http://localhost:8081/pages/1/ayahs"
```

### Advanced Search
```bash
curl "http://localhost:8081/search/advanced?q=رحمة&surahs=1,2,3&page_range=1-50"
```

### Get Tafsir for an Ayah
```bash
curl "http://localhost:8081/surahs/1/ayahs/1/tafsir"
```

### Verify Content Integrity
```bash
curl -X POST "http://localhost:8081/integrity/verify"
```

## Configuration

The service requires the following environment variables:
- `DATABASE_URL` - PostgreSQL connection string
- Default: `postgresql://postgres:password@localhost:5432/sanad`

## Testing

The service includes comprehensive unit tests:

```bash
cargo test -p quran-service
```

### Test Coverage
- Model integrity verification
- Hash calculation consistency
- Serialization/deserialization
- API parameter parsing
- Business logic validation

## Database Migrations

The service includes SQL migrations in `database/migrations/`:
- `001_initial_schema.sql` - Core database schema
- `002_sample_data.sql` - Sample data for development
- `003_quran_enhancements.sql` - Translations and recitation styles

## Dependencies

Key dependencies include:
- `axum` - Web framework
- `sqlx` - Database operations
- `serde` - Serialization
- `sha2` - Hash calculation
- `uuid` - Unique identifiers
- `chrono` - Date/time handling

## Security Features

1. **Content Integrity**: SHA-256 hashing prevents tampering
2. **Input Validation**: Query parameter validation and sanitization
3. **SQL Injection Prevention**: Parameterized queries with sqlx
4. **Error Handling**: Comprehensive error types and responses

## Performance Considerations

1. **Database Indexing**: Optimized indexes for search and retrieval
2. **Pagination**: Prevents large result sets
3. **Connection Pooling**: Efficient database connection management
4. **Async Operations**: Non-blocking I/O throughout

## Future Enhancements

1. **Caching**: Redis integration for frequently accessed content
2. **Vector Search**: Semantic search capabilities
3. **Audio Integration**: Recitation and pronunciation features
4. **Multi-language**: Translation support
5. **Bookmarking**: User-specific content management

## Compliance

This service implements the requirements from the Sanad specification:
- **Requirement 1.1**: Complete Quran display with proper formatting ✅
- **Requirement 1.2**: Accurate verse numbering and navigation ✅
- **Requirement 1.3**: Multiple recitation styles support ✅
- **Requirement 1.4**: Advanced search capabilities ✅
- **Requirement 2.1-2.3**: Tafsir integration and source attribution ✅
- **Requirement 12.3**: Content integrity verification ✅

The implementation ensures 100% accuracy of Quranic text through cryptographic verification and maintains the sanctity of Islamic content through robust security measures.
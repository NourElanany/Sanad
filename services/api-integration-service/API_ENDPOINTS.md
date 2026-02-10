# API Integration Service - HTTP Endpoints

This document describes all available HTTP endpoints for the API Integration Service.

## Base URL

```
http://localhost:8080/api/v1
```

## Response Format

All endpoints return responses in the following format:

```json
{
  "success": true,
  "data": { ... },
  "error": null,
  "request_id": "uuid-v4"
}
```

Error responses:

```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error message",
    "category": "Network|Authentication|RateLimit|ServerError|Validation|Timeout|Unknown"
  },
  "request_id": "uuid-v4"
}
```

## Endpoints

### 1. Quran Endpoints

#### Get Quran Text

```http
GET /api/v1/quran/text
```

**Query Parameters:**
- `surah` (required): Surah number (1-114)
- `ayah` (optional): Ayah number
- `translation` (optional): Translation identifier
- `reciter` (optional): Reciter name for audio

**Example Request:**
```bash
curl "http://localhost:8080/api/v1/quran/text?surah=1&ayah=1&translation=en.sahih"
```

**Example Response:**
```json
{
  "success": true,
  "data": {
    "surah": 1,
    "ayah": 1,
    "text_arabic": "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
    "text_translation": "In the name of Allah, the Entirely Merciful, the Especially Merciful.",
    "audio_url": null,
    "source": "quran.com"
  },
  "error": null,
  "request_id": "..."
}
```

#### Get Quran Audio

```http
GET /api/v1/quran/audio
```

**Query Parameters:**
- `surah` (required): Surah number (1-114)
- `ayah` (required): Ayah number
- `reciter` (required): Reciter name

**Example Request:**
```bash
curl "http://localhost:8080/api/v1/quran/audio?surah=1&ayah=1&reciter=mishary"
```

**Example Response:**
```json
{
  "success": true,
  "data": {
    "surah": 1,
    "ayah": 1,
    "audio_url": "https://everyayah.com/data/...",
    "reciter": "mishary",
    "source": "everyayah.com"
  },
  "error": null,
  "request_id": "..."
}
```

### 2. Hadith Endpoints

#### Search Hadith

```http
GET /api/v1/hadith/search
```

**Query Parameters:**
- `query` (required): Search query text
- `collection` (optional): Collection filter (e.g., "bukhari", "muslim")
- `book` (optional): Book filter
- `language` (optional): Language code (default: "en")
- `limit` (optional): Maximum results (default: 10, max: 100)

**Example Request:**
```bash
curl "http://localhost:8080/api/v1/hadith/search?query=prayer&collection=bukhari&limit=5"
```

**Example Response:**
```json
{
  "success": true,
  "data": {
    "results": [
      {
        "id": "1",
        "collection": "bukhari",
        "book": "Book of Prayer",
        "hadith_number": "500",
        "text_arabic": "...",
        "text_translation": "...",
        "grade": "Sahih",
        "narrator": "Abu Huraira",
        "source": "sunnah.com"
      }
    ],
    "total": 150,
    "sources": ["sunnah.com"]
  },
  "error": null,
  "request_id": "..."
}
```

#### Get Hadith by ID

```http
GET /api/v1/hadith/:collection/:id
```

**Path Parameters:**
- `collection`: Collection name (e.g., "bukhari", "muslim")
- `id`: Hadith ID within the collection

**Example Request:**
```bash
curl "http://localhost:8080/api/v1/hadith/bukhari/1"
```

**Example Response:**
```json
{
  "success": true,
  "data": {
    "hadith": {
      "id": "1",
      "collection": "bukhari",
      "book": "Book of Revelation",
      "hadith_number": "1",
      "text_arabic": "...",
      "text_translation": "...",
      "grade": "Sahih",
      "narrator": "Umar bin Al-Khattab",
      "source": "sunnah.com"
    }
  },
  "error": null,
  "request_id": "..."
}
```

### 3. Prayer Times Endpoint

#### Get Prayer Times

```http
POST /api/v1/prayer-times
```

**Request Body:**
```json
{
  "latitude": 40.7128,
  "longitude": -74.0060,
  "date": "2024-01-15",
  "calculation_method": "ISNA",
  "madhab": "Shafi"
}
```

**Calculation Methods:**
- `MWL`: Muslim World League
- `ISNA`: Islamic Society of North America
- `Egypt`: Egyptian General Authority of Survey
- `Makkah`: Umm Al-Qura University, Makkah
- `Karachi`: University of Islamic Sciences, Karachi
- `Tehran`: Institute of Geophysics, University of Tehran
- `Jafari`: Shia Ithna-Ashari, Leva Institute, Qum

**Madhab:**
- `Shafi`: Shafi school (earlier Asr time)
- `Hanafi`: Hanafi school (later Asr time)

**Example Response:**
```json
{
  "success": true,
  "data": {
    "date": "2024-01-15",
    "fajr": "05:45:00",
    "sunrise": "07:20:00",
    "dhuhr": "12:10:00",
    "asr": "14:45:00",
    "maghrib": "17:00:00",
    "isha": "18:30:00",
    "source": "aladhan"
  },
  "error": null,
  "request_id": "..."
}
```

### 4. Tafsir Endpoint

#### Get Tafsir

```http
GET /api/v1/tafsir
```

**Query Parameters:**
- `surah` (required): Surah number (1-114)
- `ayah` (required): Ayah number
- `tafsir_id` (optional): Specific tafsir source
- `language` (optional): Language code (default: "en")

**Example Request:**
```bash
curl "http://localhost:8080/api/v1/tafsir?surah=1&ayah=1&language=en"
```

**Example Response:**
```json
{
  "success": true,
  "data": {
    "surah": 1,
    "ayah": 1,
    "tafsirs": [
      {
        "tafsir_id": "ibn-kathir",
        "tafsir_name": "Tafsir Ibn Kathir",
        "scholar": "Ibn Kathir",
        "text": "...",
        "language": "en",
        "source": "quran.com"
      }
    ]
  },
  "error": null,
  "request_id": "..."
}
```

### 5. Calendar Endpoints

#### Convert Date

```http
POST /api/v1/calendar/convert
```

**Request Body:**
```json
{
  "date": "2024-01-15",
  "direction": "GregorianToHijri"
}
```

**Conversion Directions:**
- `GregorianToHijri`: Convert Gregorian to Hijri
- `HijriToGregorian`: Convert Hijri to Gregorian

**Example Response:**
```json
{
  "success": true,
  "data": {
    "gregorian": "2024-01-15",
    "hijri": {
      "year": 1445,
      "month": 7,
      "day": 3,
      "month_name_ar": "رجب",
      "month_name_en": "Rajab"
    },
    "source": "aladhan"
  },
  "error": null,
  "request_id": "..."
}
```

#### Get Islamic Events

```http
POST /api/v1/calendar/events
```

**Request Body:**
```json
{
  "start_date": "2024-01-01",
  "end_date": "2024-12-31"
}
```

**Example Response:**
```json
{
  "success": true,
  "data": {
    "events": [
      {
        "date": "2024-03-10",
        "hijri_date": {
          "year": 1445,
          "month": 9,
          "day": 1,
          "month_name_ar": "رمضان",
          "month_name_en": "Ramadan"
        },
        "event_name_ar": "بداية رمضان",
        "event_name_en": "Start of Ramadan",
        "description": "The beginning of the holy month of Ramadan"
      }
    ]
  },
  "error": null,
  "request_id": "..."
}
```

### 6. Qibla Endpoint

#### Get Qibla Direction

```http
POST /api/v1/qibla
```

**Request Body:**
```json
{
  "latitude": 40.7128,
  "longitude": -74.0060
}
```

**Example Response:**
```json
{
  "success": true,
  "data": {
    "direction": 58.48,
    "distance_km": 9842.5,
    "source": "aladhan"
  },
  "error": null,
  "request_id": "..."
}
```

### 7. AI Endpoint

#### Process AI Query

```http
POST /api/v1/ai/query
```

**Request Body:**
```json
{
  "query": "What are the pillars of Islam?",
  "context": "Educational context",
  "language": "en",
  "max_tokens": 500
}
```

**Example Response:**
```json
{
  "success": true,
  "data": {
    "response": "The five pillars of Islam are...",
    "sources": ["quran.com", "sunnah.com"],
    "confidence": 0.95,
    "model": "huggingface-arabic-bert"
  },
  "error": null,
  "request_id": "..."
}
```

### 8. Health Check Endpoint

#### Get Health Status

```http
GET /api/v1/health
```

**Example Response:**
```json
{
  "success": true,
  "data": {
    "overall_status": "Healthy",
    "apis": [
      {
        "api_name": "quran.com",
        "is_healthy": true,
        "last_check": "2024-01-15T12:00:00Z",
        "last_success": "2024-01-15T12:00:00Z",
        "last_failure": null,
        "success_rate": 0.99,
        "avg_response_time": {
          "secs": 0,
          "nanos": 250000000
        },
        "consecutive_failures": 0
      }
    ],
    "timestamp": "2024-01-15T12:00:00Z"
  },
  "error": null,
  "request_id": "..."
}
```

## Error Codes

| Code | Description | HTTP Status |
|------|-------------|-------------|
| `INVALID_SURAH` | Surah number out of range (1-114) | 400 |
| `INVALID_LATITUDE` | Latitude out of range (-90 to 90) | 400 |
| `INVALID_LONGITUDE` | Longitude out of range (-180 to 180) | 400 |
| `EMPTY_QUERY` | Search query is empty | 400 |
| `INVALID_DATE_RANGE` | End date before start date | 400 |
| `VALIDATION_ERROR` | General validation error | 400 |
| `AUTHENTICATION_ERROR` | API key invalid or expired | 401 |
| `NOT_FOUND` | Resource not found | 404 |
| `RATE_LIMIT_EXCEEDED` | Too many requests | 429 |
| `NETWORK_ERROR` | Network connectivity issue | 502 |
| `INVALID_RESPONSE` | Invalid response from external API | 502 |
| `API_ERROR` | External API returned error | 502 |
| `ALL_APIS_FAILED` | All fallback APIs failed | 503 |
| `TIMEOUT` | Request timeout | 504 |
| `INTERNAL_ERROR` | Internal server error | 500 |

## Rate Limiting

The service implements rate limiting per API according to their terms of service. When rate limits are exceeded, the service will:

1. Return a 429 status code
2. Include a `retry_after` field in the error response
3. Automatically use fallback APIs if available

## Caching

The service implements intelligent caching with different TTL strategies:

- **Quran text**: 30 days (static content)
- **Hadith**: 30 days (static content)
- **Prayer times**: 1 day (daily updates)
- **Tafsir**: 30 days (static content)
- **Calendar**: 7 days (weekly updates)
- **Qibla**: 30 days (location-based, static)
- **AI responses**: 1 hour (dynamic content)

## Fallback Mechanisms

When the primary API fails, the service automatically:

1. Tries secondary APIs in priority order
2. Serves stale cache if all APIs fail
3. Uses local calculations where applicable (prayer times, qibla)
4. Returns appropriate error if all fallbacks fail

## Authentication

Some endpoints require API keys for external services. Set these as environment variables:

```bash
QURAN_COM_API_KEY=your_key_here
SUNNAH_COM_API_KEY=your_key_here
ISLAMIC_FINDER_API_KEY=your_key_here
HUGGING_FACE_API_KEY=your_key_here
```

## Running the Service

### Development

```bash
cargo run --bin api-integration-service
```

### Production

```bash
cargo build --release --bin api-integration-service
./target/release/api-integration-service
```

### With Custom Configuration

```bash
CONFIG_PATH=/path/to/config.yaml cargo run --bin api-integration-service
```

### Environment Variables

- `PORT`: Server port (default: 8080)
- `HOST`: Server host (default: 0.0.0.0)
- `REDIS_URL`: Redis connection URL
- `DATABASE_URL`: PostgreSQL connection URL
- `CONFIG_PATH`: Path to configuration file

## Testing

Run the handler tests:

```bash
cargo test --lib handlers
```

Run all tests:

```bash
cargo test
```

## Monitoring

The service exposes health metrics at `/api/v1/health` which can be used for:

- Kubernetes liveness/readiness probes
- Load balancer health checks
- Monitoring dashboards
- Alerting systems

## Examples

### Complete cURL Examples

```bash
# Get Quran text
curl "http://localhost:8080/api/v1/quran/text?surah=1&ayah=1"

# Search hadith
curl "http://localhost:8080/api/v1/hadith/search?query=prayer&limit=5"

# Get prayer times
curl -X POST "http://localhost:8080/api/v1/prayer-times" \
  -H "Content-Type: application/json" \
  -d '{
    "latitude": 40.7128,
    "longitude": -74.0060,
    "date": "2024-01-15",
    "calculation_method": "ISNA",
    "madhab": "Shafi"
  }'

# Get qibla direction
curl -X POST "http://localhost:8080/api/v1/qibla" \
  -H "Content-Type: application/json" \
  -d '{
    "latitude": 40.7128,
    "longitude": -74.0060
  }'

# Health check
curl "http://localhost:8080/api/v1/health"
```

## Support

For issues or questions, please refer to the main project documentation or open an issue on the project repository.

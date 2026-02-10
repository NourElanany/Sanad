# API Integration Service - Complete API Documentation

## Table of Contents

1. [Overview](#overview)
2. [Authentication](#authentication)
3. [Base URL and Versioning](#base-url-and-versioning)
4. [Response Format](#response-format)
5. [Error Handling](#error-handling)
6. [Quran Endpoints](#quran-endpoints)
7. [Hadith Endpoints](#hadith-endpoints)
8. [Prayer Times Endpoints](#prayer-times-endpoints)
9. [Tafsir Endpoints](#tafsir-endpoints)
10. [Calendar Endpoints](#calendar-endpoints)
11. [Qibla Endpoints](#qibla-endpoints)
12. [AI Assistant Endpoints](#ai-assistant-endpoints)
13. [Health and Monitoring](#health-and-monitoring)
14. [Rate Limiting](#rate-limiting)
15. [Caching Behavior](#caching-behavior)
16. [Fallback Mechanisms](#fallback-mechanisms)
17. [Code Examples](#code-examples)

## Overview

The API Integration Service provides a unified REST API for accessing multiple official Islamic data sources. It aggregates data from verified APIs including Quran.com, Sunnah.com, AlAdhan, and others, providing:

- **Unified Interface**: Single API for multiple data sources
- **Automatic Fallback**: Seamless failover between API providers
- **Intelligent Caching**: Optimized caching strategies per data type
- **Rate Limiting**: Built-in rate limiting to comply with upstream APIs
- **Health Monitoring**: Real-time health status of all integrated APIs

### Verified Official Sources

All integrated APIs have been verified for authenticity:

**Quran**: Quran.com (Official), Tanzil.net (Official), AlQuran Cloud (Verified)
**Hadith**: Sunnah.com (Official), IslamHouse (Official)
**Prayer Times**: AlAdhan (Official), Islamic Finder (Verified)
**Tafsir**: Quran.com Tafsir (Official)
**Calendar**: AlAdhan Calendar (Official)
**Qibla**: AlAdhan Qibla (Official)

## Authentication

### API Keys

Most endpoints do not require authentication. However, some features require API keys:


- **Hadith Search**: Requires Sunnah.com API key (set via `SUNNAH_COM_API_KEY`)
- **AI Features**: Requires Hugging Face API key (set via `HUGGING_FACE_API_KEY`)

API keys are managed server-side and do not need to be included in client requests.

### Request Headers

All requests should include:

```http
Content-Type: application/json
Accept: application/json
```

Optional headers:
```http
X-Request-ID: <unique-request-id>  # For request tracking
X-Client-Version: <client-version>  # For analytics
```

## Base URL and Versioning

### Base URL

```
Production: https://api.sanad.app/api/v1
Development: http://localhost:8080/api/v1
```

### API Versioning

The API uses URL-based versioning (`/api/v1`). Major version changes will be announced with migration guides.

## Response Format

### Success Response

All successful responses follow this structure:

```json
{
  "success": true,
  "data": {
    // Response data specific to the endpoint
  },
  "error": null,
  "request_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-01-15T10:30:00Z",
  "source": "quran.com"  // API source that provided the data
}
```

### Error Response

Error responses follow this structure:

```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "INVALID_SURAH",
    "message": "Surah number must be between 1 and 114",
    "category": "Validation",
    "details": {
      "field": "surah",
      "value": 115,
      "constraint": "1-114"
    }
  },
  "request_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

## Error Handling

### Error Categories

| Category | Description | HTTP Status | Retry? |
|----------|-------------|-------------|--------|
| `Validation` | Invalid request parameters | 400 | No |
| `Authentication` | Invalid or missing API key | 401 | No |
| `NotFound` | Resource not found | 404 | No |
| `RateLimit` | Rate limit exceeded | 429 | Yes, after delay |
| `Network` | Network connectivity issue | 502 | Yes |
| `ServerError` | External API error | 502 | Yes |
| `Timeout` | Request timeout | 504 | Yes |
| `AllApisFailed` | All fallback APIs failed | 503 | Yes |

### Error Codes


#### Validation Errors (400)

| Code | Description | Solution |
|------|-------------|----------|
| `INVALID_SURAH` | Surah number out of range (1-114) | Use valid surah number |
| `INVALID_AYAH` | Ayah number invalid for surah | Check ayah count for surah |
| `INVALID_LATITUDE` | Latitude out of range (-90 to 90) | Provide valid latitude |
| `INVALID_LONGITUDE` | Longitude out of range (-180 to 180) | Provide valid longitude |
| `EMPTY_QUERY` | Search query is empty | Provide non-empty query |
| `INVALID_DATE` | Date format invalid | Use YYYY-MM-DD format |
| `INVALID_DATE_RANGE` | End date before start date | Correct date range |
| `MISSING_REQUIRED_FIELD` | Required field missing | Include all required fields |

#### Service Errors (5xx)

| Code | Description | Retry Strategy |
|------|-------------|----------------|
| `NETWORK_ERROR` | Network connectivity issue | Retry with exponential backoff |
| `TIMEOUT` | Request timeout | Retry with shorter timeout |
| `API_ERROR` | External API returned error | Automatic fallback to secondary API |
| `INVALID_RESPONSE` | Invalid response from API | Automatic fallback to secondary API |
| `ALL_APIS_FAILED` | All APIs failed | Retry after delay, check status page |
| `RATE_LIMIT_EXCEEDED` | Rate limit exceeded | Wait for retry_after seconds |

### Retry Strategy

For retryable errors, the service implements exponential backoff:

- **Attempt 1**: Immediate
- **Attempt 2**: Wait 1 second
- **Attempt 3**: Wait 2 seconds
- **Attempt 4**: Wait 4 seconds (if applicable)

Maximum retry attempts: 3

## Quran Endpoints

### Get Quran Text

Retrieve Quran text with optional translation and audio.

**Endpoint**: `GET /api/v1/quran/text`

**Query Parameters**:

| Parameter | Type | Required | Description | Example |
|-----------|------|----------|-------------|---------|
| `surah` | integer | Yes | Surah number (1-114) | `1` |
| `ayah` | integer | No | Ayah number (omit for full surah) | `1` |
| `translation` | string | No | Translation identifier | `en.sahih` |
| `reciter` | string | No | Reciter name for audio URL | `mishary` |

**Available Translations**:
- `en.sahih` - Sahih International
- `en.pickthall` - Pickthall
- `en.yusufali` - Yusuf Ali
- `ar.muyassar` - Tafsir Al-Muyassar (Arabic)
- `ur.jalandhry` - Jalandhry (Urdu)
- `fr.hamidullah` - Hamidullah (French)

**Example Request**:
```bash
curl "http://localhost:8080/api/v1/quran/text?surah=1&ayah=1&translation=en.sahih"
```

**Example Response**:
```json
{
  "success": true,
  "data": {
    "surah": 1,
    "surah_name_arabic": "الفاتحة",
    "surah_name_english": "Al-Fatihah",
    "ayah": 1,
    "text_arabic": "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
    "text_translation": "In the name of Allah, the Entirely Merciful, the Especially Merciful.",
    "audio_url": null,
    "source": "quran.com"
  },
  "error": null,
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```


**Get Full Surah**:
```bash
curl "http://localhost:8080/api/v1/quran/text?surah=1&translation=en.sahih"
```

Returns all ayahs in the surah as an array.

### Get Quran Audio

Retrieve audio recitation URL for a specific ayah.

**Endpoint**: `GET /api/v1/quran/audio`

**Query Parameters**:

| Parameter | Type | Required | Description | Example |
|-----------|------|----------|-------------|---------|
| `surah` | integer | Yes | Surah number (1-114) | `1` |
| `ayah` | integer | Yes | Ayah number | `1` |
| `reciter` | string | Yes | Reciter identifier | `mishary` |

**Available Reciters**:
- `mishary` - Mishary Rashid Alafasy
- `husary` - Mahmoud Khalil Al-Hussary
- `sudais` - Abdurrahman As-Sudais
- `minshawi` - Mohamed Siddiq El-Minshawi
- `ghamadi` - Saad Al-Ghamadi
- `ajmy` - Ahmed ibn Ali al-Ajmy

**Example Request**:
```bash
curl "http://localhost:8080/api/v1/quran/audio?surah=1&ayah=1&reciter=mishary"
```

**Example Response**:
```json
{
  "success": true,
  "data": {
    "surah": 1,
    "ayah": 1,
    "audio_url": "https://everyayah.com/data/Alafasy_128kbps/001001.mp3",
    "reciter": "mishary",
    "reciter_name": "Mishary Rashid Alafasy",
    "format": "mp3",
    "bitrate": "128kbps",
    "source": "everyayah.com"
  },
  "error": null,
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

## Hadith Endpoints

### Search Hadith

Search for hadith across multiple collections.

**Endpoint**: `GET /api/v1/hadith/search`

**Query Parameters**:

| Parameter | Type | Required | Description | Example |
|-----------|------|----------|-------------|---------|
| `query` | string | Yes | Search query text | `prayer` |
| `collection` | string | No | Filter by collection | `bukhari` |
| `book` | string | No | Filter by book | `Book of Prayer` |
| `language` | string | No | Language code (default: `en`) | `en` |
| `limit` | integer | No | Max results (default: 10, max: 100) | `20` |

**Available Collections**:
- `bukhari` - Sahih al-Bukhari
- `muslim` - Sahih Muslim
- `abudawud` - Sunan Abu Dawud
- `tirmidhi` - Jami` at-Tirmidhi
- `nasai` - Sunan an-Nasa'i
- `ibnmajah` - Sunan Ibn Majah
- `malik` - Muwatta Malik
- `ahmad` - Musnad Ahmad

**Example Request**:
```bash
curl "http://localhost:8080/api/v1/hadith/search?query=prayer&collection=bukhari&limit=5"
```

**Example Response**:
```json
{
  "success": true,
  "data": {
    "results": [
      {
        "id": "bukhari:500",
        "collection": "bukhari",
        "book": "Book of Prayer",
        "chapter": "Chapter: The obligation of prayer",
        "hadith_number": "500",
        "text_arabic": "...",
        "text_translation": "Narrated Abu Huraira: The Prophet said, 'The prayer in congregation is twenty-seven times superior to the prayer offered by person alone.'",
        "grade": "Sahih",
        "narrator": "Abu Huraira",
        "chain": "Abu Huraira → ... → Al-Bukhari",
        "source": "sunnah.com"
      }
    ],
    "total": 150,
    "page": 1,
    "limit": 5,
    "sources": ["sunnah.com"]
  },
  "error": null,
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```


### Get Hadith by ID

Retrieve a specific hadith by collection and ID.

**Endpoint**: `GET /api/v1/hadith/:collection/:id`

**Path Parameters**:

| Parameter | Type | Description | Example |
|-----------|------|-------------|---------|
| `collection` | string | Collection name | `bukhari` |
| `id` | string | Hadith ID within collection | `1` |

**Example Request**:
```bash
curl "http://localhost:8080/api/v1/hadith/bukhari/1"
```

**Example Response**:
```json
{
  "success": true,
  "data": {
    "hadith": {
      "id": "bukhari:1",
      "collection": "bukhari",
      "book": "Book of Revelation",
      "chapter": "Chapter: How the Divine Inspiration started",
      "hadith_number": "1",
      "text_arabic": "إِنَّمَا الأَعْمَالُ بِالنِّيَّاتِ...",
      "text_translation": "Narrated 'Umar bin Al-Khattab: I heard Allah's Messenger saying, 'The reward of deeds depends upon the intentions...'",
      "grade": "Sahih",
      "narrator": "Umar bin Al-Khattab",
      "chain": "Umar bin Al-Khattab → Alqama → Muhammad → Yahya → Al-Bukhari",
      "commentary": "This is one of the most important hadiths in Islam...",
      "source": "sunnah.com"
    }
  },
  "error": null,
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

## Prayer Times Endpoints

### Get Prayer Times

Calculate prayer times for a specific location and date.

**Endpoint**: `POST /api/v1/prayer-times`

**Request Body**:

| Field | Type | Required | Description | Example |
|-------|------|----------|-------------|---------|
| `latitude` | float | Yes | Latitude (-90 to 90) | `21.4225` |
| `longitude` | float | Yes | Longitude (-180 to 180) | `39.8262` |
| `date` | string | Yes | Date (YYYY-MM-DD) | `2024-01-15` |
| `calculation_method` | string | Yes | Calculation method | `Makkah` |
| `madhab` | string | Yes | Madhab for Asr time | `Shafi` |

**Calculation Methods**:

| Method | Description | Region |
|--------|-------------|--------|
| `MWL` | Muslim World League | Europe, Americas |
| `ISNA` | Islamic Society of North America | North America |
| `Egypt` | Egyptian General Authority of Survey | Egypt, Middle East |
| `Makkah` | Umm Al-Qura University | Saudi Arabia |
| `Karachi` | University of Islamic Sciences | Pakistan, Bangladesh |
| `Tehran` | Institute of Geophysics | Iran |
| `Jafari` | Shia Ithna-Ashari | Shia communities |

**Madhab Options**:
- `Shafi`: Shafi, Maliki, Hanbali schools (earlier Asr time)
- `Hanafi`: Hanafi school (later Asr time)

**Example Request**:
```bash
curl -X POST "http://localhost:8080/api/v1/prayer-times" \
  -H "Content-Type: application/json" \
  -d '{
    "latitude": 21.4225,
    "longitude": 39.8262,
    "date": "2024-01-15",
    "calculation_method": "Makkah",
    "madhab": "Shafi"
  }'
```

**Example Response**:
```json
{
  "success": true,
  "data": {
    "date": "2024-01-15",
    "hijri_date": {
      "year": 1445,
      "month": 7,
      "day": 3,
      "month_name_ar": "رجب",
      "month_name_en": "Rajab"
    },
    "location": {
      "latitude": 21.4225,
      "longitude": 39.8262,
      "city": "Makkah",
      "country": "Saudi Arabia"
    },
    "times": {
      "fajr": "05:15:00",
      "sunrise": "06:35:00",
      "dhuhr": "12:20:00",
      "asr": "15:40:00",
      "maghrib": "18:05:00",
      "isha": "19:35:00"
    },
    "calculation_method": "Makkah",
    "madhab": "Shafi",
    "source": "aladhan"
  },
  "error": null,
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```


## Tafsir Endpoints

### Get Tafsir

Retrieve Quranic interpretation (tafsir) for a specific verse.

**Endpoint**: `GET /api/v1/tafsir`

**Query Parameters**:

| Parameter | Type | Required | Description | Example |
|-----------|------|----------|-------------|---------|
| `surah` | integer | Yes | Surah number (1-114) | `1` |
| `ayah` | integer | Yes | Ayah number | `1` |
| `tafsir_id` | string | No | Specific tafsir source | `ibn-kathir` |
| `language` | string | No | Language code (default: `en`) | `en` |

**Available Tafsir Sources**:

| ID | Name | Scholar | Language | Description |
|----|------|---------|----------|-------------|
| `ibn-kathir` | Tafsir Ibn Kathir | Ibn Kathir | en, ar | Classical comprehensive tafsir |
| `al-jalalayn` | Tafsir al-Jalalayn | Jalaluddin | en, ar | Concise classical tafsir |
| `al-tabari` | Tafsir al-Tabari | Al-Tabari | ar | Extensive classical tafsir |
| `al-qurtubi` | Tafsir al-Qurtubi | Al-Qurtubi | ar | Jurisprudential tafsir |
| `al-saadi` | Tafsir al-Sa'di | Al-Sa'di | ar | Modern simplified tafsir |
| `maarif-quran` | Ma'ariful Quran | Mufti Shafi | en, ur | Contemporary comprehensive |

**Example Request**:
```bash
curl "http://localhost:8080/api/v1/tafsir?surah=1&ayah=1&language=en"
```

**Example Response**:
```json
{
  "success": true,
  "data": {
    "surah": 1,
    "ayah": 1,
    "text_arabic": "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
    "tafsirs": [
      {
        "tafsir_id": "ibn-kathir",
        "tafsir_name": "Tafsir Ibn Kathir",
        "scholar": "Ibn Kathir",
        "text": "The Basmalah is the first verse of Surah Al-Fatihah. It begins with the name of Allah, the Most Gracious, the Most Merciful...",
        "language": "en",
        "source": "quran.com"
      },
      {
        "tafsir_id": "al-jalalayn",
        "tafsir_name": "Tafsir al-Jalalayn",
        "scholar": "Jalaluddin al-Mahalli and Jalaluddin al-Suyuti",
        "text": "In the Name of God, the Merciful, the Compassionate: this is a blessed verse...",
        "language": "en",
        "source": "quran.com"
      }
    ]
  },
  "error": null,
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

## Calendar Endpoints

### Convert Date

Convert between Gregorian and Hijri calendars.

**Endpoint**: `POST /api/v1/calendar/convert`

**Request Body**:

| Field | Type | Required | Description | Example |
|-------|------|----------|-------------|---------|
| `date` | string | Yes | Date to convert (YYYY-MM-DD) | `2024-01-15` |
| `direction` | string | Yes | Conversion direction | `GregorianToHijri` |

**Conversion Directions**:
- `GregorianToHijri`: Convert Gregorian to Hijri
- `HijriToGregorian`: Convert Hijri to Gregorian

**Example Request (Gregorian to Hijri)**:
```bash
curl -X POST "http://localhost:8080/api/v1/calendar/convert" \
  -H "Content-Type: application/json" \
  -d '{
    "date": "2024-01-15",
    "direction": "GregorianToHijri"
  }'
```

**Example Response**:
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
      "month_name_en": "Rajab",
      "weekday_ar": "الإثنين",
      "weekday_en": "Monday"
    },
    "calculation_method": "Umm Al-Qura",
    "source": "aladhan"
  },
  "error": null,
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```


### Get Islamic Events

Retrieve Islamic events for a date range.

**Endpoint**: `POST /api/v1/calendar/events`

**Request Body**:

| Field | Type | Required | Description | Example |
|-------|------|----------|-------------|---------|
| `start_date` | string | Yes | Start date (YYYY-MM-DD) | `2024-01-01` |
| `end_date` | string | Yes | End date (YYYY-MM-DD) | `2024-12-31` |

**Example Request**:
```bash
curl -X POST "http://localhost:8080/api/v1/calendar/events" \
  -H "Content-Type: application/json" \
  -d '{
    "start_date": "2024-01-01",
    "end_date": "2024-12-31"
  }'
```

**Example Response**:
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
        "event_type": "month_start",
        "description": "The beginning of the holy month of Ramadan, the month of fasting.",
        "significance": "high"
      },
      {
        "date": "2024-04-09",
        "hijri_date": {
          "year": 1445,
          "month": 10,
          "day": 1,
          "month_name_ar": "شوال",
          "month_name_en": "Shawwal"
        },
        "event_name_ar": "عيد الفطر",
        "event_name_en": "Eid al-Fitr",
        "event_type": "eid",
        "description": "The festival of breaking the fast, celebrated after Ramadan.",
        "significance": "high"
      }
    ],
    "total": 15,
    "source": "aladhan"
  },
  "error": null,
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

## Qibla Endpoints

### Get Qibla Direction

Calculate the Qibla direction (direction to Makkah) from any location.

**Endpoint**: `POST /api/v1/qibla`

**Request Body**:

| Field | Type | Required | Description | Example |
|-------|------|----------|-------------|---------|
| `latitude` | float | Yes | Latitude (-90 to 90) | `40.7128` |
| `longitude` | float | Yes | Longitude (-180 to 180) | `-74.0060` |

**Example Request**:
```bash
curl -X POST "http://localhost:8080/api/v1/qibla" \
  -H "Content-Type: application/json" \
  -d '{
    "latitude": 40.7128,
    "longitude": -74.0060
  }'
```

**Example Response**:
```json
{
  "success": true,
  "data": {
    "location": {
      "latitude": 40.7128,
      "longitude": -74.0060,
      "city": "New York",
      "country": "United States"
    },
    "qibla": {
      "direction": 58.48,
      "direction_compass": "ENE",
      "distance_km": 9842.5,
      "distance_miles": 6116.2
    },
    "kaaba": {
      "latitude": 21.4225,
      "longitude": 39.8262
    },
    "calculation_method": "Great Circle",
    "source": "aladhan"
  },
  "error": null,
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Direction Interpretation**:
- `direction`: Degrees from North (0-360)
- `direction_compass`: Compass direction (N, NE, E, SE, S, SW, W, NW)
- `0°` = North
- `90°` = East
- `180°` = South
- `270°` = West

## AI Assistant Endpoints

### Process AI Query

Process a natural language query using AI/NLP models.

**Endpoint**: `POST /api/v1/ai/query`

**Request Body**:

| Field | Type | Required | Description | Example |
|-------|------|----------|-------------|---------|
| `query` | string | Yes | User's question | `What are the pillars of Islam?` |
| `context` | string | No | Additional context | `Educational context` |
| `language` | string | No | Language code (default: `en`) | `en` |
| `max_tokens` | integer | No | Max response length (default: 500) | `500` |

**Important Notes**:
- AI is used ONLY for language processing and search
- NOT used for Islamic rulings or fatwas
- All content comes from verified traditional sources
- Responses include source citations


**Example Request**:
```bash
curl -X POST "http://localhost:8080/api/v1/ai/query" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "What are the pillars of Islam?",
    "context": "Educational context",
    "language": "en",
    "max_tokens": 500
  }'
```

**Example Response**:
```json
{
  "success": true,
  "data": {
    "query": "What are the pillars of Islam?",
    "response": "The five pillars of Islam are the fundamental acts of worship that form the foundation of a Muslim's faith and practice:\n\n1. Shahada (Declaration of Faith): Testifying that there is no god but Allah and Muhammad is His messenger.\n\n2. Salah (Prayer): Performing the five daily prayers.\n\n3. Zakat (Charity): Giving a portion of one's wealth to those in need.\n\n4. Sawm (Fasting): Fasting during the month of Ramadan.\n\n5. Hajj (Pilgrimage): Making the pilgrimage to Makkah at least once in a lifetime if able.",
    "sources": [
      {
        "type": "hadith",
        "reference": "Sahih al-Bukhari 8",
        "text": "Islam is based on five principles..."
      },
      {
        "type": "hadith",
        "reference": "Sahih Muslim 16",
        "text": "The Messenger of Allah said: Islam is built upon five..."
      }
    ],
    "confidence": 0.95,
    "model": "huggingface-arabic-bert",
    "language": "en"
  },
  "error": null,
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

## Health and Monitoring

### Health Check

Get the health status of the service and all integrated APIs.

**Endpoint**: `GET /api/v1/health`

**Example Request**:
```bash
curl "http://localhost:8080/api/v1/health"
```

**Example Response**:
```json
{
  "success": true,
  "data": {
    "overall_status": "Healthy",
    "service_uptime": "72h 15m 30s",
    "apis": [
      {
        "api_name": "quran.com",
        "category": "quran",
        "is_healthy": true,
        "priority": 1,
        "last_check": "2024-01-15T12:00:00Z",
        "last_success": "2024-01-15T12:00:00Z",
        "last_failure": null,
        "success_rate": 0.99,
        "avg_response_time": {
          "secs": 0,
          "nanos": 250000000
        },
        "consecutive_failures": 0,
        "total_requests": 15420,
        "failed_requests": 154
      },
      {
        "api_name": "sunnah.com",
        "category": "hadith",
        "is_healthy": false,
        "priority": 1,
        "last_check": "2024-01-15T12:00:00Z",
        "last_success": "2024-01-15T11:45:00Z",
        "last_failure": "2024-01-15T12:00:00Z",
        "success_rate": 0.95,
        "avg_response_time": {
          "secs": 1,
          "nanos": 500000000
        },
        "consecutive_failures": 3,
        "total_requests": 8230,
        "failed_requests": 411,
        "error": "Connection timeout"
      }
    ],
    "cache_status": {
      "connected": true,
      "hit_rate": 0.85,
      "total_keys": 45230
    },
    "rate_limiter_status": {
      "connected": true,
      "active_limits": 7
    },
    "timestamp": "2024-01-15T12:00:00Z"
  },
  "error": null,
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### Metrics Endpoint

Get Prometheus-compatible metrics.

**Endpoint**: `GET /metrics`

**Example Response**:
```
# HELP api_requests_total Total number of API requests
# TYPE api_requests_total counter
api_requests_total{api="quran.com",status="success"} 15266
api_requests_total{api="quran.com",status="error"} 154
api_requests_total{api="sunnah.com",status="success"} 7819
api_requests_total{api="sunnah.com",status="error"} 411

# HELP api_response_time_seconds API response time in seconds
# TYPE api_response_time_seconds histogram
api_response_time_seconds_bucket{api="quran.com",le="0.1"} 12000
api_response_time_seconds_bucket{api="quran.com",le="0.5"} 15200
api_response_time_seconds_bucket{api="quran.com",le="1.0"} 15400
api_response_time_seconds_bucket{api="quran.com",le="+Inf"} 15420

# HELP cache_hits_total Total number of cache hits
# TYPE cache_hits_total counter
cache_hits_total{category="quran_text"} 38450
cache_hits_total{category="hadith"} 12340
cache_hits_total{category="prayer_times"} 8920

# HELP cache_misses_total Total number of cache misses
# TYPE cache_misses_total counter
cache_misses_total{category="quran_text"} 2340
cache_misses_total{category="hadith"} 1890
cache_misses_total{category="prayer_times"} 3210
```


## Rate Limiting

### Rate Limit Headers

All responses include rate limit information in headers:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 995
X-RateLimit-Reset: 1705320000
```

### Rate Limit Response

When rate limit is exceeded (HTTP 429):

```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit exceeded for API: quran.com",
    "category": "RateLimit",
    "retry_after": 60,
    "details": {
      "api": "quran.com",
      "limit": 60,
      "window": "minute",
      "reset_at": "2024-01-15T12:01:00Z"
    }
  },
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### Rate Limits by Endpoint

| Endpoint Category | Requests/Minute | Requests/Hour | Requests/Day |
|-------------------|-----------------|---------------|--------------|
| Quran Text | 60 | 1000 | 10000 |
| Quran Audio | 60 | 1000 | 10000 |
| Hadith Search | 30 | 500 | 5000 |
| Prayer Times | 60 | 1000 | 10000 |
| Tafsir | 60 | 1000 | 10000 |
| Calendar | 60 | 1000 | 10000 |
| Qibla | 60 | 1000 | 10000 |
| AI Query | 30 | 500 | 5000 |

**Note**: These are service-level limits. The service manages upstream API limits automatically.

## Caching Behavior

### Cache Strategy by Data Type

| Data Type | TTL | Stale Cache | Description |
|-----------|-----|-------------|-------------|
| Quran Text | 30 days | 90 days | Static content, long cache |
| Quran Audio | 30 days | 90 days | Static URLs, long cache |
| Hadith | 30 days | 90 days | Static content, long cache |
| Prayer Times | 1 day | 7 days | Daily updates, short cache |
| Tafsir | 30 days | 90 days | Static content, long cache |
| Calendar | 7 days | 30 days | Semi-static, medium cache |
| Qibla | 30 days | 90 days | Static per location, long cache |
| AI Response | 1 hour | None | Dynamic content, short cache |

### Cache Headers

Responses include cache information:

```http
X-Cache-Status: HIT
X-Cache-Age: 3600
X-Cache-TTL: 86400
```

**Cache Status Values**:
- `HIT`: Served from cache
- `MISS`: Fetched from API
- `STALE`: Served from expired cache (fallback)
- `BYPASS`: Cache bypassed

### Cache Invalidation

Caches are automatically invalidated based on TTL. Manual invalidation is not supported via public API.

## Fallback Mechanisms

### Automatic Fallback

The service implements automatic fallback when APIs fail:

1. **Primary API**: First attempt
2. **Secondary API**: If primary fails
3. **Tertiary API**: If secondary fails
4. **Stale Cache**: If all APIs fail
5. **Local Calculation**: For prayer times and qibla (if applicable)

### Fallback Indicators

Responses include fallback information:

```json
{
  "success": true,
  "data": { ... },
  "fallback_used": "secondary_api",
  "fallback_reason": "Primary API timeout",
  "source": "alquran.cloud"
}
```

### API Priority Order

**Quran**:
1. Quran.com (Primary)
2. AlQuran Cloud (Secondary)
3. Tanzil (Tertiary)

**Hadith**:
1. Sunnah.com (Primary)
2. IslamHouse (Secondary)

**Prayer Times**:
1. AlAdhan (Primary)
2. Islamic Finder (Secondary)
3. Local Calculation (Fallback)

**Tafsir**:
1. Quran.com Tafsir (Primary)

**Calendar**:
1. AlAdhan Calendar (Primary)
2. Islamic Finder Calendar (Secondary)

**Qibla**:
1. AlAdhan Qibla (Primary)
2. Islamic Finder Qibla (Secondary)
3. Local Calculation (Fallback)

## Code Examples

### JavaScript/TypeScript

```typescript
// Using fetch API
async function getQuranText(surah: number, ayah: number): Promise<QuranResponse> {
  const response = await fetch(
    `http://localhost:8080/api/v1/quran/text?surah=${surah}&ayah=${ayah}&translation=en.sahih`
  );
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error.message);
  }
  
  return response.json();
}

// Using axios
import axios from 'axios';

async function searchHadith(query: string, limit: number = 10) {
  try {
    const response = await axios.get('http://localhost:8080/api/v1/hadith/search', {
      params: { query, limit }
    });
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      console.error('API Error:', error.response?.data.error);
    }
    throw error;
  }
}

// Get prayer times
async function getPrayerTimes(lat: number, lon: number, date: string) {
  const response = await fetch('http://localhost:8080/api/v1/prayer-times', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      latitude: lat,
      longitude: lon,
      date: date,
      calculation_method: 'ISNA',
      madhab: 'Shafi'
    })
  });
  
  return response.json();
}
```


### Python

```python
import requests
from typing import Dict, Any

class SanadAPIClient:
    def __init__(self, base_url: str = "http://localhost:8080/api/v1"):
        self.base_url = base_url
        self.session = requests.Session()
    
    def get_quran_text(self, surah: int, ayah: int = None, translation: str = "en.sahih") -> Dict[str, Any]:
        """Get Quran text with optional translation."""
        params = {"surah": surah, "translation": translation}
        if ayah:
            params["ayah"] = ayah
        
        response = self.session.get(f"{self.base_url}/quran/text", params=params)
        response.raise_for_status()
        return response.json()
    
    def search_hadith(self, query: str, collection: str = None, limit: int = 10) -> Dict[str, Any]:
        """Search for hadith."""
        params = {"query": query, "limit": limit}
        if collection:
            params["collection"] = collection
        
        response = self.session.get(f"{self.base_url}/hadith/search", params=params)
        response.raise_for_status()
        return response.json()
    
    def get_prayer_times(self, lat: float, lon: float, date: str, 
                        method: str = "ISNA", madhab: str = "Shafi") -> Dict[str, Any]:
        """Get prayer times for a location."""
        data = {
            "latitude": lat,
            "longitude": lon,
            "date": date,
            "calculation_method": method,
            "madhab": madhab
        }
        
        response = self.session.post(f"{self.base_url}/prayer-times", json=data)
        response.raise_for_status()
        return response.json()
    
    def get_qibla_direction(self, lat: float, lon: float) -> Dict[str, Any]:
        """Get Qibla direction for a location."""
        data = {"latitude": lat, "longitude": lon}
        
        response = self.session.post(f"{self.base_url}/qibla", json=data)
        response.raise_for_status()
        return response.json()

# Usage example
client = SanadAPIClient()

# Get Quran verse
verse = client.get_quran_text(surah=1, ayah=1)
print(verse['data']['text_arabic'])

# Search hadith
hadiths = client.search_hadith(query="prayer", collection="bukhari", limit=5)
for hadith in hadiths['data']['results']:
    print(f"{hadith['collection']}:{hadith['hadith_number']} - {hadith['text_translation'][:100]}...")

# Get prayer times
times = client.get_prayer_times(lat=40.7128, lon=-74.0060, date="2024-01-15")
print(f"Fajr: {times['data']['times']['fajr']}")
```

### Rust

```rust
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<ApiError>,
    request_id: String,
}

#[derive(Debug, Deserialize)]
struct ApiError {
    code: String,
    message: String,
    category: String,
}

#[derive(Debug, Deserialize)]
struct QuranTextData {
    surah: u8,
    ayah: u16,
    text_arabic: String,
    text_translation: Option<String>,
    source: String,
}

#[derive(Debug, Serialize)]
struct PrayerTimesRequest {
    latitude: f64,
    longitude: f64,
    date: String,
    calculation_method: String,
    madhab: String,
}

#[derive(Debug, Deserialize)]
struct PrayerTimesData {
    date: String,
    times: PrayerTimes,
    source: String,
}

#[derive(Debug, Deserialize)]
struct PrayerTimes {
    fajr: String,
    sunrise: String,
    dhuhr: String,
    asr: String,
    maghrib: String,
    isha: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let base_url = "http://localhost:8080/api/v1";
    
    // Get Quran text
    let quran_response: ApiResponse<QuranTextData> = client
        .get(&format!("{}/quran/text", base_url))
        .query(&[("surah", "1"), ("ayah", "1"), ("translation", "en.sahih")])
        .send()
        .await?
        .json()
        .await?;
    
    if let Some(data) = quran_response.data {
        println!("Arabic: {}", data.text_arabic);
        println!("Translation: {}", data.text_translation.unwrap_or_default());
    }
    
    // Get prayer times
    let prayer_request = PrayerTimesRequest {
        latitude: 40.7128,
        longitude: -74.0060,
        date: "2024-01-15".to_string(),
        calculation_method: "ISNA".to_string(),
        madhab: "Shafi".to_string(),
    };
    
    let prayer_response: ApiResponse<PrayerTimesData> = client
        .post(&format!("{}/prayer-times", base_url))
        .json(&prayer_request)
        .send()
        .await?
        .json()
        .await?;
    
    if let Some(data) = prayer_response.data {
        println!("Fajr: {}", data.times.fajr);
        println!("Dhuhr: {}", data.times.dhuhr);
        println!("Asr: {}", data.times.asr);
    }
    
    Ok(())
}
```

### Flutter/Dart

```dart
import 'dart:convert';
import 'package:http/http.dart' as http;

class SanadApiClient {
  final String baseUrl;
  
  SanadApiClient({this.baseUrl = 'http://localhost:8080/api/v1'});
  
  Future<Map<String, dynamic>> getQuranText({
    required int surah,
    int? ayah,
    String translation = 'en.sahih',
  }) async {
    final params = {
      'surah': surah.toString(),
      'translation': translation,
      if (ayah != null) 'ayah': ayah.toString(),
    };
    
    final uri = Uri.parse('$baseUrl/quran/text').replace(queryParameters: params);
    final response = await http.get(uri);
    
    if (response.statusCode == 200) {
      return json.decode(response.body);
    } else {
      throw Exception('Failed to load Quran text');
    }
  }
  
  Future<Map<String, dynamic>> searchHadith({
    required String query,
    String? collection,
    int limit = 10,
  }) async {
    final params = {
      'query': query,
      'limit': limit.toString(),
      if (collection != null) 'collection': collection,
    };
    
    final uri = Uri.parse('$baseUrl/hadith/search').replace(queryParameters: params);
    final response = await http.get(uri);
    
    if (response.statusCode == 200) {
      return json.decode(response.body);
    } else {
      throw Exception('Failed to search hadith');
    }
  }
  
  Future<Map<String, dynamic>> getPrayerTimes({
    required double latitude,
    required double longitude,
    required String date,
    String calculationMethod = 'ISNA',
    String madhab = 'Shafi',
  }) async {
    final uri = Uri.parse('$baseUrl/prayer-times');
    final response = await http.post(
      uri,
      headers: {'Content-Type': 'application/json'},
      body: json.encode({
        'latitude': latitude,
        'longitude': longitude,
        'date': date,
        'calculation_method': calculationMethod,
        'madhab': madhab,
      }),
    );
    
    if (response.statusCode == 200) {
      return json.decode(response.body);
    } else {
      throw Exception('Failed to get prayer times');
    }
  }
}

// Usage
void main() async {
  final client = SanadApiClient();
  
  // Get Quran verse
  final verse = await client.getQuranText(surah: 1, ayah: 1);
  print(verse['data']['text_arabic']);
  
  // Search hadith
  final hadiths = await client.searchHadith(query: 'prayer', limit: 5);
  print('Found ${hadiths['data']['total']} hadiths');
  
  // Get prayer times
  final times = await client.getPrayerTimes(
    latitude: 40.7128,
    longitude: -74.0060,
    date: '2024-01-15',
  );
  print('Fajr: ${times['data']['times']['fajr']}');
}
```

---

## Support and Resources

### Documentation
- **Configuration Guide**: See `config/CONFIGURATION_GUIDE.md`
- **Deployment Guide**: See `DEPLOYMENT_GUIDE.md`
- **Developer Guide**: See `DEVELOPER_GUIDE.md`

### API Status
- **Status Page**: https://status.sanad.app (if available)
- **Health Endpoint**: `GET /api/v1/health`

### Support Channels
- **GitHub Issues**: For bug reports and feature requests
- **Documentation**: For detailed guides and examples
- **Community Forum**: For discussions and questions

### Rate Limit Increases
For higher rate limits, please contact the development team with:
- Use case description
- Expected request volume
- Current limitations

---

**API Version**: 1.0.0  
**Last Updated**: 2024-01-15  
**Maintained by**: Sanad Development Team


# Backend Services Integration Documentation

## Overview

This document describes the comprehensive backend services integration implemented for both the Flutter mobile app and Next.js web application. The integration includes HTTP clients, JWT authentication, API endpoints configuration, error handling with retry mechanisms, and network connectivity monitoring.

## Architecture

### Flutter Mobile App

```
┌─────────────────────────────────────────┐
│         Application Layer               │
├─────────────────────────────────────────┤
│         DioClient (HTTP Client)         │
│  ┌───────────────────────────────────┐  │
│  │  Interceptors:                    │  │
│  │  - ConnectivityInterceptor        │  │
│  │  - AuthInterceptor                │  │
│  │  - RetryInterceptor               │  │
│  │  - LoggingInterceptor             │  │
│  └───────────────────────────────────┘  │
├─────────────────────────────────────────┤
│         Services Layer                  │
│  ┌──────────────┐  ┌─────────────────┐ │
│  │ AuthService  │  │ Connectivity    │ │
│  │              │  │ Service         │ │
│  └──────────────┘  └─────────────────┘ │
├─────────────────────────────────────────┤
│         Backend Services                │
│  (Rust Microservices)                   │
└─────────────────────────────────────────┘
```

### Next.js Web App

```
┌─────────────────────────────────────────┐
│         Application Layer               │
├─────────────────────────────────────────┤
│      ApiClient (Axios Client)           │
│  ┌───────────────────────────────────┐  │
│  │  Interceptors:                    │  │
│  │  - Request Interceptor            │  │
│  │  - Response Interceptor           │  │
│  │  - Auth Token Management          │  │
│  │  - Error Handling                 │  │
│  └───────────────────────────────────┘  │
├─────────────────────────────────────────┤
│         Services Layer                  │
│  ┌──────────────┐  ┌─────────────────┐ │
│  │ authService  │  │ connectivity    │ │
│  │              │  │ Service         │ │
│  └──────────────┘  └─────────────────┘ │
├─────────────────────────────────────────┤
│         Backend Services                │
│  (Rust Microservices)                   │
└─────────────────────────────────────────┘
```

## Features Implemented

### 1. HTTP Client Configuration

#### Flutter (Dio)
- **Location**: `frontend/mobile/lib/core/network/dio_client.dart`
- **Features**:
  - Configurable base URL and timeouts
  - Automatic request/response logging in debug mode
  - Type-safe HTTP methods (GET, POST, PUT, PATCH, DELETE)
  - Custom error handling with specific exception types
  - Platform-specific headers

#### Next.js (Axios)
- **Location**: `frontend/nextjs-app/src/lib/api/axios-client.ts`
- **Features**:
  - Singleton pattern for consistent instance
  - TypeScript support with generics
  - Request/response interceptors
  - Automatic error transformation
  - Queue management for failed requests during token refresh

### 2. JWT Authentication

#### Token Management
Both platforms implement secure JWT token management:

- **Access Token**: Short-lived token for API requests
- **Refresh Token**: Long-lived token for obtaining new access tokens
- **Automatic Refresh**: Tokens are automatically refreshed when expired
- **Secure Storage**: 
  - Flutter: `flutter_secure_storage` with platform-specific encryption
  - Next.js: `localStorage` (consider upgrading to httpOnly cookies for production)

#### Authentication Flow

```
┌─────────┐         ┌─────────┐         ┌─────────┐
│  User   │         │  Client │         │ Backend │
└────┬────┘         └────┬────┘         └────┬────┘
     │                   │                   │
     │  Login Request    │                   │
     ├──────────────────>│                   │
     │                   │  POST /auth/login │
     │                   ├──────────────────>│
     │                   │                   │
     │                   │  Access + Refresh │
     │                   │<──────────────────┤
     │                   │                   │
     │  Login Success    │                   │
     │<──────────────────┤                   │
     │                   │                   │
     │  API Request      │                   │
     ├──────────────────>│                   │
     │                   │  GET /api/data    │
     │                   │  + Bearer Token   │
     │                   ├──────────────────>│
     │                   │                   │
     │                   │  401 Unauthorized │
     │                   │<──────────────────┤
     │                   │                   │
     │                   │  POST /auth/refresh
     │                   ├──────────────────>│
     │                   │                   │
     │                   │  New Access Token │
     │                   │<──────────────────┤
     │                   │                   │
     │                   │  Retry GET /api/data
     │                   ├──────────────────>│
     │                   │                   │
     │  Data Response    │  200 OK + Data    │
     │<──────────────────┤<──────────────────┤
     │                   │                   │
```

### 3. API Endpoints Configuration

Centralized endpoint configuration for all backend services:

#### Flutter
- **Location**: `frontend/mobile/lib/core/network/api_endpoints.dart`

#### Next.js
- **Location**: `frontend/nextjs-app/src/lib/api/endpoints.ts`

#### Supported Services
- Authentication Service
- Quran Service (Surahs, Ayahs, Tafsir, Translations, Audio)
- Hadith Service (Collections, Books, Search)
- Prayer Times Service (Daily, Monthly, Qibla, Hijri Calendar)
- AI Service (RAG System, Streaming, Multiple Viewpoints)
- Audio Analysis Service (Tajweed Analysis, Progress Tracking)
- Search Service (Semantic Search, Advanced Filters)
- User Service (Profile, Preferences, Bookmarks, Statistics)
- Stories Service
- Notification Service
- Khatma Service
- Customization Service
- Widgets Service

### 4. Error Handling

#### Exception Hierarchy

```
NetworkException (Base)
├── BadRequestException (400)
├── UnauthorizedException (401)
├── ForbiddenException (403)
├── NotFoundException (404)
├── ValidationException (422)
├── RateLimitException (429)
└── ServerException (5xx)
```

#### Error Handling Features
- Specific exception types for different HTTP status codes
- Detailed error messages from backend
- Validation errors with field-specific messages
- Automatic error logging in debug mode

### 5. Retry Mechanism

#### Flutter RetryInterceptor
- **Location**: `frontend/mobile/lib/core/network/interceptors/retry_interceptor.dart`
- **Features**:
  - Exponential backoff strategy
  - Configurable max retries (default: 3)
  - Jitter to prevent thundering herd
  - Retry only on specific error types:
    - Connection timeouts
    - Server errors (5xx)
    - Rate limiting (429)
    - Request timeout (408)
  - Maximum delay cap (30 seconds)

#### Retry Algorithm

```dart
delay = initialDelay * (backoffMultiplier ^ retryCount) + jitter
jitter = random(0, delay * 0.2)
finalDelay = min(delay, 30000ms)
```

Example retry delays:
- Retry 1: 500ms + jitter
- Retry 2: 1000ms + jitter
- Retry 3: 2000ms + jitter

### 6. Network Connectivity Monitoring

#### Flutter ConnectivityService
- **Location**: `frontend/mobile/lib/core/network/services/connectivity_service.dart`
- **Features**:
  - Real-time connectivity status monitoring
  - Stream-based status updates
  - WiFi vs Mobile data detection
  - Automatic status updates on network changes

#### Next.js ConnectivityService
- **Location**: `frontend/nextjs-app/src/lib/services/connectivity-service.ts`
- **Features**:
  - Browser online/offline event monitoring
  - Connection type detection (if supported)
  - Slow connection detection
  - Downlink speed estimation
  - Subscribe/unsubscribe pattern for status updates

#### Connectivity Status

```typescript
enum ConnectivityStatus {
  CONNECTED = 'connected',
  DISCONNECTED = 'disconnected',
  UNKNOWN = 'unknown',
}
```

## Usage Examples

### Flutter

#### Making API Requests

```dart
// Initialize DioClient
final dioClient = DioClient();

// GET request
try {
  final response = await dioClient.get<Map<String, dynamic>>(
    ApiEndpoints.surahs,
  );
  print('Surahs: $response');
} on NetworkException catch (e) {
  print('Error: ${e.message}');
}

// POST request with data
try {
  final response = await dioClient.post<Map<String, dynamic>>(
    ApiEndpoints.login,
    data: {
      'email': 'user@example.com',
      'password': 'password123',
    },
  );
  print('Login successful: $response');
} on UnauthorizedException catch (e) {
  print('Invalid credentials: ${e.message}');
}
```

#### Authentication

```dart
// Initialize AuthService
final authService = AuthService();
authService.init();

// Login
final result = await authService.login(
  email: 'user@example.com',
  password: 'password123',
);

if (result.success) {
  print('Logged in as: ${result.userId}');
} else {
  print('Login failed: ${result.error}');
}

// Check authentication status
final isAuth = await authService.isAuthenticated();
print('Is authenticated: $isAuth');

// Logout
await authService.logout();
```

#### Connectivity Monitoring

```dart
// Initialize ConnectivityService
final connectivityService = ConnectivityService();
await connectivityService.init();

// Check current status
final isConnected = await connectivityService.isConnected();
print('Connected: $isConnected');

// Listen to status changes
connectivityService.onConnectivityChanged.listen((status) {
  print('Connectivity changed: ${status.message}');
  
  if (status.isConnected) {
    // Sync data
  } else {
    // Show offline message
  }
});
```

### Next.js

#### Making API Requests

```typescript
import { apiClient } from '@/lib/api/axios-client';
import { API_ENDPOINTS } from '@/lib/api/endpoints';

// GET request
try {
  const surahs = await apiClient.get(API_ENDPOINTS.QURAN.SURAHS);
  console.log('Surahs:', surahs);
} catch (error) {
  console.error('Error:', error);
}

// POST request with data
try {
  const result = await apiClient.post(API_ENDPOINTS.AUTH.LOGIN, {
    email: 'user@example.com',
    password: 'password123',
  });
  console.log('Login successful:', result);
} catch (error) {
  console.error('Login failed:', error);
}
```

#### Authentication

```typescript
import { authService } from '@/lib/services/auth-service';

// Login
const result = await authService.login(
  'user@example.com',
  'password123'
);

if (result.success) {
  console.log('Logged in as:', result.userId);
} else {
  console.error('Login failed:', result.error);
}

// Check authentication status
const isAuth = authService.isAuthenticated();
console.log('Is authenticated:', isAuth);

// Logout
authService.logout();
```

#### Connectivity Monitoring

```typescript
import { connectivityService, ConnectivityStatus } from '@/lib/services/connectivity-service';

// Check current status
const isConnected = await connectivityService.isConnected();
console.log('Connected:', isConnected);

// Subscribe to status changes
const unsubscribe = connectivityService.subscribe((status) => {
  console.log('Connectivity changed:', status);
  
  if (status === ConnectivityStatus.CONNECTED) {
    // Sync data
  } else {
    // Show offline message
  }
});

// Unsubscribe when component unmounts
unsubscribe();
```

## Testing

### Flutter Tests

Run tests:
```bash
cd frontend/mobile
flutter test
```

Test files:
- `test/core/network/dio_client_test.dart`
- `test/core/services/auth_service_test.dart`
- `test/core/services/connectivity_service_test.dart`

### Next.js Tests

Run tests:
```bash
cd frontend/nextjs-app
npm test
```

Test files:
- `src/lib/api/__tests__/axios-client.test.ts`
- `src/lib/services/__tests__/auth-service.test.ts`

## Configuration

### Flutter

Edit `frontend/mobile/lib/core/config/app_config.dart`:

```dart
class AppConfig {
  static String get apiBaseUrl => _apiBaseUrl;
  static const int apiTimeout = 30000; // 30 seconds
  static const int connectTimeout = 15000; // 15 seconds
}
```

### Next.js

Create `.env.local` file:

```env
NEXT_PUBLIC_API_BASE_URL=https://api.sanad.app
NEXT_PUBLIC_WS_BASE_URL=wss://api.sanad.app
NEXT_PUBLIC_APP_VERSION=1.0.0
```

## Security Considerations

### Token Storage
- **Flutter**: Uses `flutter_secure_storage` with platform-specific encryption
  - Android: EncryptedSharedPreferences
  - iOS: Keychain with `first_unlock_this_device` accessibility
- **Next.js**: Currently uses localStorage (consider upgrading to httpOnly cookies)

### HTTPS Only
- All API requests use HTTPS
- WebSocket connections use WSS (secure WebSocket)

### Token Expiration
- Access tokens expire after a short period (typically 15-60 minutes)
- Refresh tokens expire after a longer period (typically 7-30 days)
- Tokens are automatically refreshed before expiration (5-minute buffer)

### Request Validation
- All requests include app version and platform headers
- Authentication tokens are validated on every request
- Invalid tokens trigger automatic logout

## Performance Optimizations

### Request Caching
- Implement caching strategies for frequently accessed data
- Use ETags for conditional requests
- Cache static content (Quran text, translations)

### Connection Pooling
- Dio and Axios automatically manage connection pools
- Reuse connections for better performance

### Retry Strategy
- Exponential backoff prevents server overload
- Jitter prevents thundering herd problem
- Maximum retry limit prevents infinite loops

## Troubleshooting

### Common Issues

#### 1. Connection Timeout
**Symptom**: Requests fail with timeout error
**Solution**: 
- Check internet connection
- Verify API base URL is correct
- Increase timeout values if needed

#### 2. 401 Unauthorized
**Symptom**: Requests fail with 401 status
**Solution**:
- Check if user is logged in
- Verify token is not expired
- Clear tokens and login again

#### 3. Token Refresh Loop
**Symptom**: Continuous token refresh attempts
**Solution**:
- Check refresh token validity
- Verify refresh endpoint is working
- Clear all tokens and login again

#### 4. Network Detection Issues
**Symptom**: App doesn't detect network changes
**Solution**:
- Verify connectivity service is initialized
- Check platform permissions
- Test on real device (not simulator)

## Future Enhancements

### Planned Features
1. **Request Queuing**: Queue requests when offline and sync when online
2. **GraphQL Support**: Add GraphQL client for more efficient data fetching
3. **WebSocket Integration**: Real-time updates for prayer times and notifications
4. **Request Cancellation**: Cancel in-flight requests when navigating away
5. **Request Deduplication**: Prevent duplicate requests for same resource
6. **Advanced Caching**: Implement cache invalidation strategies
7. **Offline Mode**: Full offline support with local database sync

### Security Enhancements
1. **Certificate Pinning**: Pin SSL certificates for added security
2. **Request Signing**: Sign requests with HMAC for integrity
3. **Rate Limiting**: Client-side rate limiting to prevent abuse
4. **Biometric Authentication**: Add fingerprint/face ID for sensitive operations

## Requirements Validation

This implementation satisfies the following requirements from the spec:

✅ **Requirement 14.1**: JWT token-based authentication implemented
✅ **Requirement 14.2**: Integration with all existing Backend Services
✅ **Requirement 14.3**: CRDT Sync foundation (token management and API structure)
✅ **Requirement 14.4**: Network connectivity changes handled gracefully
✅ **Requirement 14.5**: Sensitive data encrypted in local storage (Flutter)

## Conclusion

The backend services integration provides a robust, secure, and performant foundation for both the Flutter mobile app and Next.js web application. The implementation includes comprehensive error handling, automatic token refresh, retry mechanisms, and network monitoring to ensure a smooth user experience even in challenging network conditions.

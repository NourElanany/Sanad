# Riverpod State Management Integration Guide

## Overview

This directory contains the comprehensive Riverpod state management system for the Sanad Islamic Application. All providers are integrated with caching, offline support, error handling, and optimistic updates.

## Core Providers

### 1. App State Provider (`app_state_provider.dart`)
- **Purpose**: Global application state management
- **Features**:
  - Connectivity monitoring
  - Loading state management
  - Error state handling
  - User session management

### 2. Cache Provider (`cache_provider.dart`)
- **Purpose**: Intelligent data caching with TTL
- **Features**:
  - Configurable TTL (Time To Live)
  - Automatic size management (100MB default)
  - Expired item cleanup
  - JSON serialization support

### 3. Offline Provider (`offline_provider.dart`)
- **Purpose**: Offline operation queue management
- **Features**:
  - Operation queuing when offline
  - Automatic retry with backoff
  - Maximum retry limit (3 attempts)
  - Persistent queue storage

### 4. Error Handler Provider (`error_handler_provider.dart`)
- **Purpose**: Centralized error handling
- **Features**:
  - Automatic error type detection
  - User-friendly Arabic messages
  - Error history tracking
  - Visual error indicators

### 5. Optimistic Update Provider (`optimistic_update_provider.dart`)
- **Purpose**: Optimistic UI updates
- **Features**:
  - Immediate UI updates
  - Automatic rollback on error
  - Success/error callbacks
  - Pending operation tracking

## Feature Providers

### Quran Provider (`quran_provider.dart`)
**Integrated with**: Cache, Offline, Error Handler

**Usage Example**:
```dart
// Load surahs with cache and offline support
final quranState = ref.watch(quranIndexProvider);
ref.read(quranIndexProvider.notifier).loadSurahs();

// Add bookmark (works offline)
await ref.read(quranIndexProvider.notifier).addBookmark(
  surahNumber: 1,
  ayahNumber: 5,
  pageNumber: 1,
  note: 'Important verse',
);

// Load page with cache
await ref.read(quranProvider.notifier).loadPage(1);
```

### Dashboard Provider (`dashboard_provider.dart`)
**Integrated with**: Cache, Offline, Error Handler

**Usage Example**:
```dart
// Load dashboard with cache
await ref.read(dashboardNotifierProvider.notifier).loadDashboardData(
  latitude: 24.7136,
  longitude: 46.6753,
);

// Update wird progress (works offline)
await ref.read(dashboardNotifierProvider.notifier).updateWirdProgress(
  pageNumber: 5,
  completed: true,
);

// Refresh data
await ref.read(dashboardNotifierProvider.notifier).refresh(
  latitude: 24.7136,
  longitude: 46.6753,
);
```

## Integration Pattern

All feature providers follow this pattern:

```dart
final featureProvider = StateNotifierProvider<FeatureNotifier, FeatureState>((ref) {
  final service = ref.watch(featureServiceProvider);
  final cacheService = ref.watch(configuredCacheServiceProvider);
  final offlineManager = ref.watch(offlineManagerProvider.notifier);
  final errorHandler = ref.watch(errorHandlerProvider.notifier);
  final isOnline = ref.watch(isOnlineProvider);
  
  return FeatureNotifier(
    service,
    cacheService,
    offlineManager,
    errorHandler,
    isOnline,
  );
});
```

## Caching Strategy

### Cache TTL Guidelines

| Data Type | TTL | Reason |
|-----------|-----|--------|
| Quran content (surahs, pages) | 30 days | Static content, rarely changes |
| Prayer times | 24 hours | Changes daily |
| User bookmarks | 1 hour | User-specific, frequently updated |
| Dashboard data | 15 minutes | Dynamic content |
| Search results | 5 minutes | Frequently changing |

### Cache Keys Convention

```dart
// Static content
'quran_surahs'
'quran_page_{pageNumber}'
'quran_juzs'

// User-specific content
'quran_bookmarks'
'reading_progress'
'user_preferences'

// Location-specific content
'prayer_times_{latitude}_{longitude}'
'qibla_direction_{latitude}_{longitude}'

// Time-sensitive content
'dashboard_data'
'daily_content'
'hijri_date'
```

## Offline Operations

### Supported Operations

1. **add_bookmark**: Add Quran bookmark
2. **delete_bookmark**: Remove bookmark
3. **update_reading_progress**: Update reading position
4. **update_wird_progress**: Update daily wird completion
5. **add_favorite**: Add to favorites
6. **remove_favorite**: Remove from favorites

### Processing Offline Queue

```dart
// Automatically process when online
ref.listen(isOnlineProvider, (previous, next) {
  if (next && !previous) {
    // Connection restored, process queue
    ref.read(offlineManagerProvider.notifier).processPendingOperations(
      (operation) async {
        switch (operation.operation) {
          case 'add_bookmark':
            await quranService.addBookmark(operation.data);
            break;
          case 'update_reading_progress':
            await quranService.updateReadingProgress(operation.data);
            break;
          // ... handle other operations
        }
      },
    );
  }
});
```

## Error Handling

### Error Types

- `network`: Connection issues → Show retry option
- `authentication`: Auth failures → Redirect to login
- `validation`: Invalid data → Show validation errors
- `notFound`: Resource not found → Show not found message
- `serverError`: Server-side errors → Show try again later
- `unknown`: Unexpected errors → Show generic error

### Displaying Errors

```dart
// In UI
final errorState = ref.watch(errorHandlerProvider);
if (errorState.currentError != null) {
  showErrorSnackbar(context, errorState.currentError!);
}

// Or use error boundary
Consumer(
  builder: (context, ref, child) {
    ref.listen(errorHandlerProvider, (previous, next) {
      if (next.currentError != null) {
        showErrorSnackbar(context, next.currentError!);
      }
    });
    return child!;
  },
  child: YourWidget(),
);
```

## Optimistic Updates

### When to Use

✅ **Use for**:
- Adding bookmarks
- Updating reading progress
- Marking favorites
- Simple CRUD operations

❌ **Avoid for**:
- Complex transactions
- Payment operations
- Critical data updates

### Example

```dart
final operation = createOptimisticOperation<List<String>>(
  id: 'add_bookmark_${surahId}_${ayahId}',
  optimisticData: [...currentBookmarks, newBookmarkId],
  serverOperation: () => apiService.addBookmark(surahId, ayahId),
  onSuccess: (result) {
    showSuccessMessage(context, 'تمت إضافة العلامة المرجعية');
  },
  onError: (error) {
    showErrorSnackbar(context, AppError.fromException(error));
  },
  onRollback: () {
    print('Bookmark addition rolled back');
  },
);

await ref.read(bookmarkOptimisticProvider(userId).notifier)
    .executeOptimistic(operation);
```

## Performance Optimization

### 1. Selective Watching

```dart
// ❌ Bad: Watches entire state
final state = ref.watch(quranProvider);

// ✅ Good: Watches specific field
final isLoading = ref.watch(quranProvider.select((state) => state.isLoading));
```

### 2. Auto-dispose

```dart
// For temporary data
final searchResultsProvider = StateNotifierProvider.autoDispose<
    SearchNotifier,
    SearchState
>((ref) {
  return SearchNotifier(ref.read(searchServiceProvider));
});
```

### 3. Family Providers

```dart
// For parameterized providers
final surahProvider = FutureProvider.family<Surah, int>((ref, surahNumber) async {
  return ref.read(quranProvider.notifier).loadSurah(surahNumber);
});
```

## Testing

### Unit Tests

```dart
test('Cache service stores and retrieves data', () async {
  final box = await Hive.openBox('test_cache');
  final cacheService = CacheService(box, const CacheConfig());
  
  await cacheService.put('test_key', {'data': 'value'});
  final result = cacheService.get('test_key', (json) => json);
  
  expect(result, {'data': 'value'});
  
  await box.close();
});
```

### Widget Tests

```dart
testWidgets('Shows cached data when offline', (tester) async {
  await tester.pumpWidget(
    ProviderScope(
      overrides: [
        isOnlineProvider.overrideWithValue(false),
        configuredCacheServiceProvider.overrideWithValue(mockCacheService),
      ],
      child: MyApp(),
    ),
  );
  
  expect(find.text('Cached Content'), findsOneWidget);
});
```

## Initialization

Add to `main.dart`:

```dart
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'core/providers/cache_provider.dart';
import 'core/providers/offline_provider.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Initialize Hive
  await Hive.initFlutter();
  
  // Initialize cache
  await initializeCache();
  
  // Initialize offline storage
  await initializeOfflineStorage();
  
  runApp(
    ProviderScope(
      child: MyApp(),
    ),
  );
}
```

## Monitoring and Debugging

### Provider Observer

```dart
class MyProviderObserver extends ProviderObserver {
  @override
  void didUpdateProvider(
    ProviderBase provider,
    Object? previousValue,
    Object? newValue,
    ProviderContainer container,
  ) {
    debugPrint('Provider updated: ${provider.name ?? provider.runtimeType}');
  }
}

// In main.dart
runApp(
  ProviderScope(
    observers: [MyProviderObserver()],
    child: MyApp(),
  ),
);
```

## Best Practices

1. **Always use cache for frequently accessed data**
2. **Queue operations when offline instead of failing**
3. **Use optimistic updates for better UX**
4. **Handle errors centrally with ErrorHandler**
5. **Set appropriate TTL based on data volatility**
6. **Clean up cache periodically**
7. **Test offline scenarios thoroughly**
8. **Monitor cache size and performance**
9. **Use selective watching to avoid unnecessary rebuilds**
10. **Implement proper error recovery strategies**

## Troubleshooting

### Cache not working
- Check if Hive is initialized
- Verify cache key naming
- Check TTL configuration
- Ensure JSON serialization is correct

### Offline queue not processing
- Verify connectivity monitoring is working
- Check operation handler implementation
- Ensure queue is not cleared prematurely
- Check retry count limits

### Errors not displaying
- Verify ErrorHandler is initialized
- Check error listener setup
- Ensure error types are correctly mapped
- Verify UI error display logic

## Summary

This Riverpod state management system provides:

✅ **Robust caching** with TTL and size management  
✅ **Offline-first architecture** with operation queuing  
✅ **Optimistic updates** with automatic rollback  
✅ **Comprehensive error handling** with user-friendly messages  
✅ **Performance optimization** with selective watching  
✅ **Type-safe state management** with Riverpod  
✅ **Testable architecture** with provider overrides  

All providers are production-ready and follow best practices for Flutter state management.

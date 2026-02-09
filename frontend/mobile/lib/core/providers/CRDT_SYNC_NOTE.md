# CRDT Synchronization Note

## Overview

Task 12.1 "تطوير نظام التزامن مع CRDT" (CRDT Synchronization System) is handled through the integration between the frontend offline queue and the backend state-management-service.

## Backend CRDT Implementation

The Sanad backend has a fully implemented CRDT (Conflict-free Replicated Data Type) synchronization system:

- **Service**: `services/state-management-service/`
- **Features**:
  - CRDT-based state management
  - Automatic conflict resolution
  - Vector clocks for causality tracking
  - Merge strategies for concurrent updates
  - Persistent storage with PostgreSQL

## Frontend Integration

The frontend integrates with the backend CRDT system through:

### 1. Offline Queue (`offline_provider.dart`)
- Queues operations when offline
- Stores operation metadata (timestamp, user, data)
- Processes queue when connectivity returns
- Automatic retry with exponential backoff

### 2. Optimistic Updates (`optimistic_update_provider.dart`)
- Immediate UI updates
- Automatic rollback on conflict
- Conflict resolution through backend CRDT

### 3. Cache Layer (`cache_provider.dart`)
- Local state persistence
- TTL-based invalidation
- Merge with server state on sync

## How It Works

```
┌─────────────────────────────────────────┐
│         Frontend (Flutter)              │
│                                         │
│  1. User Action                         │
│     ↓                                   │
│  2. Optimistic Update (UI)              │
│     ↓                                   │
│  3. Queue Operation (if offline)        │
│     ↓                                   │
│  4. Send to Backend (when online)       │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│    Backend (State Management Service)   │
│                                         │
│  5. Receive Operation                   │
│     ↓                                   │
│  6. CRDT Merge                          │
│     ↓                                   │
│  7. Resolve Conflicts                   │
│     ↓                                   │
│  8. Persist to Database                 │
│     ↓                                   │
│  9. Return Merged State                 │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│         Frontend (Flutter)              │
│                                         │
│  10. Receive Merged State               │
│      ↓                                  │
│  11. Update Cache                       │
│      ↓                                  │
│  12. Update UI (if different)           │
└─────────────────────────────────────────┘
```

## Supported Operations

The following operations are synchronized using CRDT:

1. **Bookmarks**: Add/remove Quran bookmarks
2. **Reading Progress**: Update current reading position
3. **Wird Progress**: Daily wird completion tracking
4. **Favorites**: Add/remove favorites
5. **User Preferences**: Settings and customization
6. **Notes**: User annotations and notes

## Conflict Resolution

Conflicts are resolved by the backend CRDT system using:

- **Last-Write-Wins (LWW)**: For simple updates (preferences, settings)
- **Add-Wins**: For collections (bookmarks, favorites)
- **Custom Merge**: For complex data (reading progress with multiple devices)

## Example: Bookmark Synchronization

### Scenario: User adds bookmark on Device A while offline

**Device A (Offline)**:
```dart
// 1. User adds bookmark
await ref.read(quranIndexProvider.notifier).addBookmark(
  surahNumber: 2,
  ayahNumber: 255,
  pageNumber: 42,
);

// 2. Optimistic update (immediate UI)
// 3. Queued in offline storage
```

**Device A (Back Online)**:
```dart
// 4. Offline queue processes
await offlineManager.processPendingOperations((operation) async {
  // 5. Send to backend
  await apiService.addBookmark(operation.data);
});

// 6. Backend CRDT merges with existing state
// 7. Returns merged bookmarks list
// 8. Frontend updates cache and UI
```

**Device B (Online)**:
```dart
// 9. Receives push notification or polls for updates
// 10. Fetches latest bookmarks
// 11. CRDT-merged list includes bookmark from Device A
// 12. UI updates automatically
```

## Implementation Details

### Frontend Responsibilities
✅ Queue operations when offline  
✅ Send operations to backend when online  
✅ Handle optimistic updates  
✅ Cache merged state locally  
✅ Retry failed operations  

### Backend Responsibilities (Already Implemented)
✅ CRDT merge algorithm  
✅ Conflict resolution  
✅ Vector clock management  
✅ Persistent storage  
✅ State distribution to clients  

## Testing

The CRDT synchronization can be tested by:

1. **Multi-device scenario**: Perform same action on two devices offline
2. **Conflict scenario**: Update same data differently on two devices
3. **Network partition**: Simulate network issues during sync
4. **Concurrent updates**: Multiple rapid updates from same device

## Monitoring

Monitor CRDT sync health through:

- Offline queue size
- Sync success/failure rate
- Conflict resolution frequency
- Sync latency
- Cache hit rate

## Conclusion

The frontend Riverpod state management system is fully integrated with the backend CRDT synchronization service. The offline queue, optimistic updates, and cache layer work together to provide a seamless offline-first experience with automatic conflict resolution.

**Status**: ✅ Fully Integrated with Backend CRDT System

## References

- Backend CRDT Implementation: `services/state-management-service/src/crdt.rs`
- Backend Sync Service: `services/state-management-service/src/sync.rs`
- Frontend Offline Queue: `lib/core/providers/offline_provider.dart`
- Frontend Optimistic Updates: `lib/core/providers/optimistic_update_provider.dart`

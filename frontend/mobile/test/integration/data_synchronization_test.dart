import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import '../../lib/core/services/crdt_sync_service.dart';
import '../../lib/core/services/local_storage_service.dart';
import '../../lib/core/services/connectivity_service.dart';
import '../../lib/core/services/auth_service.dart';
import '../../lib/core/network/dio_client.dart';

/// Integration tests for data synchronization using CRDT
/// **Validates: Requirements 20.4**
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Data Synchronization Integration Tests', () {
    late CRDTSyncService syncService;
    late LocalStorageService localStorageService;
    late ConnectivityService connectivityService;
    late AuthService authService;
    late DioClient dioClient;

    setUpAll(() async {
      dioClient = DioClient(baseUrl: 'https://api.sanad.app');
      authService = AuthService(dioClient);
      localStorageService = await LocalStorageService.init();
      connectivityService = ConnectivityService();
      syncService = CRDTSyncService(
        dioClient: dioClient,
        localStorage: localStorageService,
        connectivity: connectivityService,
      );

      // Login for authenticated tests
      await authService.login(
        email: 'sync_test@example.com',
        password: 'SyncTest123!',
      );
    });

    tearDown(() async {
      await localStorageService.clear();
    });

    group('Basic Synchronization', () {
      test('should sync local changes to server', () async {
        // Arrange - Make local changes
        await localStorageService.updateReadingProgress(
          surahNumber: 2,
          ayahNumber: 100,
          timestamp: DateTime.now(),
        );

        await localStorageService.addBookmark(
          surahNumber: 18,
          ayahNumber: 10,
          note: 'Test bookmark',
          timestamp: DateTime.now(),
        );

        // Act
        final syncResult = await syncService.syncToServer();

        // Assert
        expect(syncResult.success, isTrue);
        expect(syncResult.itemsSynced, equals(2));
        expect(syncResult.conflicts, isEmpty);

        // Verify on server
        final serverProgress = await syncService.fetchServerProgress();
        expect(serverProgress.lastReadSurah, equals(2));
        expect(serverProgress.lastReadAyah, equals(100));
      });

      test('should sync server changes to local', () async {
        // Arrange - Make changes on server
        await syncService.updateServerProgress(
          surahNumber: 3,
          ayahNumber: 50,
        );

        await syncService.addServerBookmark(
          surahNumber: 36,
          ayahNumber: 1,
          note: 'Server bookmark',
        );

        // Act
        final syncResult = await syncService.syncFromServer();

        // Assert
        expect(syncResult.success, isTrue);
        expect(syncResult.itemsSynced, equals(2));

        // Verify locally
        final localProgress = await localStorageService.getReadingProgress();
        expect(localProgress.lastReadSurah, equals(3));
        expect(localProgress.lastReadAyah, equals(50));

        final bookmarks = await localStorageService.getBookmarks();
        expect(
          bookmarks.any((b) => b.surahNumber == 36 && b.ayahNumber == 1),
          isTrue,
        );
      });

      test('should perform bidirectional sync', () async {
        // Arrange - Changes on both sides
        await localStorageService.updateReadingProgress(
          surahNumber: 4,
          ayahNumber: 75,
          timestamp: DateTime.now(),
        );

        await syncService.addServerBookmark(
          surahNumber: 55,
          ayahNumber: 1,
          note: 'Server bookmark',
        );

        // Act
        final syncResult = await syncService.performFullSync();

        // Assert
        expect(syncResult.success, isTrue);
        expect(syncResult.itemsSynced, greaterThanOrEqualTo(2));

        // Verify both sides are in sync
        final localProgress = await localStorageService.getReadingProgress();
        final serverProgress = await syncService.fetchServerProgress();
        
        expect(localProgress.lastReadSurah, equals(serverProgress.lastReadSurah));
        expect(localProgress.lastReadAyah, equals(serverProgress.lastReadAyah));
      });
    });

    group('Conflict Resolution', () {
      test('should resolve conflicts using Last-Write-Wins', () async {
        // Arrange - Create conflicting changes
        final olderTime = DateTime.now().subtract(const Duration(hours: 1));
        final newerTime = DateTime.now();

        // Local change (newer)
        await localStorageService.updateReadingProgress(
          surahNumber: 5,
          ayahNumber: 100,
          timestamp: newerTime,
        );

        // Server change (older)
        await syncService.updateServerProgress(
          surahNumber: 5,
          ayahNumber: 80,
          timestamp: olderTime,
        );

        // Act
        final syncResult = await syncService.performFullSync();

        // Assert - Should keep newer local change
        expect(syncResult.conflicts.length, equals(1));
        expect(syncResult.conflicts.first.resolution, equals('last-write-wins'));
        
        final finalProgress = await localStorageService.getReadingProgress();
        expect(finalProgress.lastReadAyah, equals(100));

        final serverProgress = await syncService.fetchServerProgress();
        expect(serverProgress.lastReadAyah, equals(100));
      });

      test('should merge non-conflicting changes', () async {
        // Arrange
        // Local: Update progress
        await localStorageService.updateReadingProgress(
          surahNumber: 6,
          ayahNumber: 50,
          timestamp: DateTime.now(),
        );

        // Server: Add bookmark (different data)
        await syncService.addServerBookmark(
          surahNumber: 67,
          ayahNumber: 1,
          note: 'Server bookmark',
        );

        // Act
        final syncResult = await syncService.performFullSync();

        // Assert - Both changes should be preserved
        expect(syncResult.success, isTrue);
        expect(syncResult.conflicts, isEmpty);

        final localProgress = await localStorageService.getReadingProgress();
        expect(localProgress.lastReadSurah, equals(6));

        final bookmarks = await localStorageService.getBookmarks();
        expect(
          bookmarks.any((b) => b.surahNumber == 67),
          isTrue,
        );
      });

      test('should handle bookmark conflicts', () async {
        // Arrange - Same bookmark modified on both sides
        final bookmarkId = 'bookmark_123';
        
        await localStorageService.updateBookmark(
          id: bookmarkId,
          note: 'Local note',
          timestamp: DateTime.now(),
        );

        await syncService.updateServerBookmark(
          id: bookmarkId,
          note: 'Server note',
          timestamp: DateTime.now().subtract(const Duration(minutes: 5)),
        );

        // Act
        final syncResult = await syncService.performFullSync();

        // Assert - Should keep local (newer) version
        expect(syncResult.conflicts.length, equals(1));
        
        final bookmark = await localStorageService.getBookmark(bookmarkId);
        expect(bookmark!.note, equals('Local note'));
      });

      test('should handle deletion conflicts', () async {
        // Arrange
        final bookmarkId = 'bookmark_to_delete';
        
        // Create bookmark
        await localStorageService.addBookmark(
          id: bookmarkId,
          surahNumber: 7,
          ayahNumber: 1,
          note: 'Test',
          timestamp: DateTime.now(),
        );
        
        await syncService.performFullSync();

        // Delete locally
        await localStorageService.deleteBookmark(
          id: bookmarkId,
          timestamp: DateTime.now(),
        );

        // Update on server
        await syncService.updateServerBookmark(
          id: bookmarkId,
          note: 'Updated note',
          timestamp: DateTime.now().subtract(const Duration(minutes: 1)),
        );

        // Act
        final syncResult = await syncService.performFullSync();

        // Assert - Deletion should win
        expect(syncResult.conflicts.length, equals(1));
        expect(syncResult.conflicts.first.resolution, equals('deletion-wins'));
        
        final bookmark = await localStorageService.getBookmark(bookmarkId);
        expect(bookmark, isNull);
      });
    });

    group('Offline Queue Management', () {
      test('should queue operations when offline', () async {
        // Arrange
        connectivityService.setOfflineMode(true);

        // Act - Make changes while offline
        await localStorageService.updateReadingProgress(
          surahNumber: 8,
          ayahNumber: 25,
          timestamp: DateTime.now(),
        );

        await localStorageService.addBookmark(
          surahNumber: 89,
          ayahNumber: 1,
          note: 'Offline bookmark',
          timestamp: DateTime.now(),
        );

        // Assert
        final queuedOps = await syncService.getQueuedOperations();
        expect(queuedOps.length, equals(2));
        expect(queuedOps[0].type, equals('update_progress'));
        expect(queuedOps[1].type, equals('add_bookmark'));

        // Cleanup
        connectivityService.setOfflineMode(false);
      });

      test('should process queued operations when back online', () async {
        // Arrange - Queue operations
        connectivityService.setOfflineMode(true);

        await localStorageService.updateReadingProgress(
          surahNumber: 9,
          ayahNumber: 50,
          timestamp: DateTime.now(),
        );

        await localStorageService.addBookmark(
          surahNumber: 90,
          ayahNumber: 1,
          note: 'Queued bookmark',
          timestamp: DateTime.now(),
        );

        // Act - Go online and sync
        connectivityService.setOfflineMode(false);
        final syncResult = await syncService.processQueuedOperations();

        // Assert
        expect(syncResult.success, isTrue);
        expect(syncResult.itemsSynced, equals(2));
        
        final queuedOps = await syncService.getQueuedOperations();
        expect(queuedOps, isEmpty);

        // Verify on server
        final serverProgress = await syncService.fetchServerProgress();
        expect(serverProgress.lastReadSurah, equals(9));
      });

      test('should retry failed operations', () async {
        // Arrange
        connectivityService.setOfflineMode(true);

        await localStorageService.updateReadingProgress(
          surahNumber: 10,
          ayahNumber: 30,
          timestamp: DateTime.now(),
        );

        connectivityService.setOfflineMode(false);

        // Simulate server error
        var attemptCount = 0;
        syncService.onSyncAttempt = () {
          attemptCount++;
          if (attemptCount < 3) {
            throw Exception('Server error');
          }
        };

        // Act
        final syncResult = await syncService.processQueuedOperations();

        // Assert - Should retry and succeed
        expect(attemptCount, equals(3));
        expect(syncResult.success, isTrue);
      });

      test('should preserve operation order', () async {
        // Arrange
        connectivityService.setOfflineMode(true);

        final operations = <Map<String, dynamic>>[];
        
        for (var i = 1; i <= 5; i++) {
          await localStorageService.updateReadingProgress(
            surahNumber: i,
            ayahNumber: i * 10,
            timestamp: DateTime.now().add(Duration(seconds: i)),
          );
          
          operations.add({
            'surah': i,
            'ayah': i * 10,
          });
        }

        // Act
        connectivityService.setOfflineMode(false);
        
        final processedOps = <Map<String, dynamic>>[];
        syncService.onOperationProcessed = (op) {
          processedOps.add(op.data);
        };

        await syncService.processQueuedOperations();

        // Assert - Operations should be processed in order
        expect(processedOps.length, equals(5));
        for (var i = 0; i < 5; i++) {
          expect(processedOps[i]['surah'], equals(operations[i]['surah']));
          expect(processedOps[i]['ayah'], equals(operations[i]['ayah']));
        }
      });
    });

    group('CRDT Operations', () {
      test('should handle concurrent updates using CRDT', () async {
        // Arrange - Simulate two devices making concurrent changes
        final device1SyncService = CRDTSyncService(
          dioClient: dioClient,
          localStorage: localStorageService,
          connectivity: connectivityService,
        );

        final device2SyncService = CRDTSyncService(
          dioClient: dioClient,
          localStorage: await LocalStorageService.init(),
          connectivity: connectivityService,
        );

        // Act - Both devices update different fields
        await device1SyncService.updateLocalData({
          'reading_progress': {'surah': 11, 'ayah': 50},
        });

        await device2SyncService.updateLocalData({
          'daily_goal': {'pages': 5, 'minutes': 30},
        });

        // Sync both
        await device1SyncService.performFullSync();
        await device2SyncService.performFullSync();

        // Assert - Both changes should be preserved
        final device1Data = await device1SyncService.getLocalData();
        final device2Data = await device2SyncService.getLocalData();

        expect(device1Data['reading_progress']['surah'], equals(11));
        expect(device1Data['daily_goal']['pages'], equals(5));

        expect(device2Data['reading_progress']['surah'], equals(11));
        expect(device2Data['daily_goal']['pages'], equals(5));
      });

      test('should merge sets correctly', () async {
        // Arrange - Two devices add different bookmarks
        final device1Bookmarks = {'bookmark_1', 'bookmark_2'};
        final device2Bookmarks = {'bookmark_3', 'bookmark_4'};

        await syncService.addLocalBookmarks(device1Bookmarks);
        
        // Simulate device 2
        await syncService.addServerBookmarks(device2Bookmarks);

        // Act
        await syncService.performFullSync();

        // Assert - All bookmarks should be present
        final allBookmarks = await syncService.getAllBookmarks();
        expect(allBookmarks.length, equals(4));
        expect(allBookmarks.containsAll(device1Bookmarks), isTrue);
        expect(allBookmarks.containsAll(device2Bookmarks), isTrue);
      });

      test('should handle counter increments', () async {
        // Arrange - Track reading statistics
        await syncService.incrementLocalCounter('pages_read', 5);
        await syncService.incrementServerCounter('pages_read', 3);

        // Act
        await syncService.performFullSync();

        // Assert - Counters should be merged (5 + 3 = 8)
        final localCounter = await syncService.getLocalCounter('pages_read');
        final serverCounter = await syncService.getServerCounter('pages_read');

        expect(localCounter, equals(8));
        expect(serverCounter, equals(8));
      });
    });

    group('Sync Performance', () {
      test('should batch sync operations efficiently', () async {
        // Arrange - Make many changes
        for (var i = 1; i <= 100; i++) {
          await localStorageService.addBookmark(
            surahNumber: i % 114 + 1,
            ayahNumber: i,
            note: 'Bookmark $i',
            timestamp: DateTime.now(),
          );
        }

        // Act
        final startTime = DateTime.now();
        final syncResult = await syncService.performFullSync();
        final duration = DateTime.now().difference(startTime);

        // Assert - Should complete in reasonable time
        expect(syncResult.success, isTrue);
        expect(syncResult.itemsSynced, equals(100));
        expect(duration.inSeconds, lessThan(10)); // Should take less than 10 seconds
      });

      test('should use delta sync for efficiency', () async {
        // Arrange - Initial sync
        await syncService.performFullSync();
        final initialSyncSize = syncService.lastSyncSize;

        // Make small change
        await localStorageService.updateReadingProgress(
          surahNumber: 12,
          ayahNumber: 10,
          timestamp: DateTime.now(),
        );

        // Act - Delta sync
        final syncResult = await syncService.performDeltaSync();

        // Assert - Should only sync changed data
        expect(syncResult.success, isTrue);
        expect(syncService.lastSyncSize, lessThan(initialSyncSize / 10));
      });

      test('should compress large sync payloads', () async {
        // Arrange - Create large dataset
        for (var i = 1; i <= 50; i++) {
          await localStorageService.addBookmark(
            surahNumber: i,
            ayahNumber: 1,
            note: 'Long note ' * 100, // Large note
            timestamp: DateTime.now(),
          );
        }

        // Act
        final syncResult = await syncService.performFullSync(compress: true);

        // Assert
        expect(syncResult.success, isTrue);
        expect(syncResult.compressionRatio, greaterThan(2.0)); // At least 2x compression
      });
    });

    group('Sync Status and Monitoring', () {
      test('should track sync status', () async {
        // Arrange
        var statusUpdates = <String>[];
        
        syncService.onStatusChanged.listen((status) {
          statusUpdates.add(status);
        });

        // Act
        await syncService.performFullSync();

        // Assert
        expect(statusUpdates, contains('syncing'));
        expect(statusUpdates, contains('completed'));
      });

      test('should report sync progress', () async {
        // Arrange
        var progressUpdates = <double>[];
        
        syncService.onProgressChanged.listen((progress) {
          progressUpdates.add(progress);
        });

        // Create data to sync
        for (var i = 1; i <= 10; i++) {
          await localStorageService.addBookmark(
            surahNumber: i,
            ayahNumber: 1,
            note: 'Test',
            timestamp: DateTime.now(),
          );
        }

        // Act
        await syncService.performFullSync();

        // Assert
        expect(progressUpdates.isNotEmpty, isTrue);
        expect(progressUpdates.first, lessThan(progressUpdates.last));
        expect(progressUpdates.last, equals(1.0));
      });

      test('should maintain sync history', () async {
        // Act - Perform multiple syncs
        await syncService.performFullSync();
        await Future.delayed(const Duration(seconds: 1));
        await syncService.performFullSync();
        await Future.delayed(const Duration(seconds: 1));
        await syncService.performFullSync();

        // Assert
        final syncHistory = await syncService.getSyncHistory();
        expect(syncHistory.length, greaterThanOrEqualTo(3));
        
        for (var entry in syncHistory) {
          expect(entry.timestamp, isNotNull);
          expect(entry.success, isTrue);
          expect(entry.itemsSynced, greaterThanOrEqualTo(0));
        }
      });
    });
  });
}

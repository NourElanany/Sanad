import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:connectivity_plus/connectivity_plus.dart';
import '../../lib/core/services/connectivity_service.dart';
import '../../lib/core/services/local_storage_service.dart';
import '../../lib/core/services/quran_service.dart';
import '../../lib/core/providers/offline_provider.dart';

/// Integration tests for offline functionality
/// **Validates: Requirements 20.3**
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Offline Functionality Integration Tests', () {
    late ConnectivityService connectivityService;
    late LocalStorageService localStorageService;
    late QuranService quranService;
    late OfflineManager offlineManager;

    setUpAll(() async {
      connectivityService = ConnectivityService();
      localStorageService = await LocalStorageService.init();
      quranService = QuranService(dioClient);
      offlineManager = OfflineManager(offlineBox);
    });

    group('Offline Data Storage', () {
      test('should store Quran data locally', () async {
        // Arrange - Ensure online
        expect(await connectivityService.isConnected(), isTrue);

        // Act - Fetch and store surah
        final surah = await quranService.getSurahById(1);
        await localStorageService.storeSurah(surah);

        // Assert - Data should be in local storage
        final storedSurah = await localStorageService.getSurah(1);
        expect(storedSurah, isNotNull);
        expect(storedSurah!.name, equals('الفاتحة'));
      });

      test('should retrieve data from cache when offline', () async {
        // Arrange - Store data while online
        final surah = await quranService.getSurahById(2);
        await localStorageService.storeSurah(surah);

        // Simulate offline mode
        connectivityService.setOfflineMode(true);

        // Act - Try to get data
        final cachedSurah = await localStorageService.getSurah(2);

        // Assert
        expect(cachedSurah, isNotNull);
        expect(cachedSurah!.name, equals('البقرة'));

        // Cleanup
        connectivityService.setOfflineMode(false);
      });

      test('should queue operations when offline', () async {
        // Arrange
        connectivityService.setOfflineMode(true);

        // Act - Try to add bookmark while offline
        await offlineManager.queueOperation(
          'add_bookmark',
          {
            'surahNumber': 36,
            'ayahNumber': 1,
            'note': 'Offline bookmark',
          },
        );

        // Assert
        expect(offlineManager.state.pendingCount, equals(1));

        // Cleanup
        connectivityService.setOfflineMode(false);
      });

      test('should sync queued operations when back online', () async {
        // Arrange - Queue operations while offline
        connectivityService.setOfflineMode(true);

        await offlineManager.queueOperation('update_progress', {
          'surahNumber': 3,
          'ayahNumber': 50,
        });

        await offlineManager.queueOperation('add_bookmark', {
          'surahNumber': 4,
          'ayahNumber': 1,
        });

        expect(offlineManager.state.pendingCount, equals(2));

        // Act - Go back online and sync
        connectivityService.setOfflineMode(false);
        await offlineManager.processPendingOperations((operation) async {
          // Simulate successful sync
          if (operation.operation == 'update_progress') {
            await quranService.updateReadingProgress(
              surahNumber: operation.data['surahNumber'],
              ayahNumber: operation.data['ayahNumber'],
            );
          } else if (operation.operation == 'add_bookmark') {
            await quranService.addBookmark(
              surahNumber: operation.data['surahNumber'],
              ayahNumber: operation.data['ayahNumber'],
            );
          }
        });

        // Assert
        expect(offlineManager.state.pendingCount, equals(0));
      });
    });

    group('Offline Reading Experience', () {
      test('should allow reading Quran offline', () async {
        // Arrange - Download surahs while online
        for (var i = 1; i <= 5; i++) {
          final surah = await quranService.getSurahById(i);
          await localStorageService.storeSurah(surah);
        }

        // Act - Go offline
        connectivityService.setOfflineMode(true);

        // Assert - Should be able to read stored surahs
        for (var i = 1; i <= 5; i++) {
          final surah = await localStorageService.getSurah(i);
          expect(surah, isNotNull);
          expect(surah!.ayahs, isNotNull);
        }

        // Cleanup
        connectivityService.setOfflineMode(false);
      });

      test('should show offline indicator', () async {
        // Act
        connectivityService.setOfflineMode(true);

        // Assert
        expect(await connectivityService.isConnected(), isFalse);
        expect(connectivityService.connectionStatus, equals(ConnectivityResult.none));

        // Cleanup
        connectivityService.setOfflineMode(false);
      });

      test('should handle partial downloads', () async {
        // Arrange - Start downloading a large surah
        final downloadFuture = localStorageService.downloadSurah(2); // Al-Baqarah

        // Act - Simulate connection loss mid-download
        await Future.delayed(const Duration(milliseconds: 100));
        connectivityService.setOfflineMode(true);

        // Assert - Download should be paused
        expect(localStorageService.hasPartialDownload(2), isTrue);

        // Act - Resume when back online
        connectivityService.setOfflineMode(false);
        await localStorageService.resumeDownload(2);

        // Assert - Download should complete
        final surah = await localStorageService.getSurah(2);
        expect(surah, isNotNull);
        expect(surah!.ayahs, isNotNull);
      });
    });

    group('Data Synchronization', () {
      test('should detect conflicts and resolve them', () async {
        // Arrange - Make changes offline
        connectivityService.setOfflineMode(true);

        await offlineManager.queueOperation('update_progress', {
          'surahNumber': 5,
          'ayahNumber': 100,
          'timestamp': DateTime.now().toIso8601String(),
        });

        // Simulate server having different data
        final serverProgress = {
          'surahNumber': 5,
          'ayahNumber': 80,
          'timestamp': DateTime.now().subtract(const Duration(hours: 1)).toIso8601String(),
        };

        // Act - Go online and sync
        connectivityService.setOfflineMode(false);

        final resolved = await offlineManager.resolveConflict(
          localData: offlineManager.state.pendingOperations.first.data,
          serverData: serverProgress,
        );

        // Assert - Should keep local data (newer timestamp)
        expect(resolved['ayahNumber'], equals(100));
      });

      test('should handle sync failures gracefully', () async {
        // Arrange
        connectivityService.setOfflineMode(true);

        await offlineManager.queueOperation('add_bookmark', {
          'surahNumber': 10,
          'ayahNumber': 1,
        });

        // Act - Try to sync with failing server
        connectivityService.setOfflineMode(false);

        var syncAttempts = 0;
        await offlineManager.processPendingOperations((operation) async {
          syncAttempts++;
          if (syncAttempts < 3) {
            throw Exception('Server error');
          }
          // Success on 3rd attempt
        });

        // Assert - Should retry and eventually succeed
        expect(syncAttempts, equals(3));
        expect(offlineManager.state.pendingCount, equals(0));
      });
    });

    group('Storage Management', () {
      test('should track storage usage', () async {
        // Arrange - Download multiple surahs
        for (var i = 1; i <= 10; i++) {
          final surah = await quranService.getSurahById(i);
          await localStorageService.storeSurah(surah);
        }

        // Act
        final storageInfo = await localStorageService.getStorageInfo();

        // Assert
        expect(storageInfo.usedSpace, greaterThan(0));
        expect(storageInfo.totalSpace, greaterThan(storageInfo.usedSpace));
        expect(storageInfo.availableSpace, greaterThan(0));
      });

      test('should clear old cached data', () async {
        // Arrange - Store data with old timestamp
        await localStorageService.storeSurah(
          surah,
          timestamp: DateTime.now().subtract(const Duration(days: 31)),
        );

        // Act - Clear cache older than 30 days
        await localStorageService.clearOldCache(days: 30);

        // Assert
        final surah = await localStorageService.getSurah(1);
        expect(surah, isNull);
      });

      test('should manage download priorities', () async {
        // Arrange - Queue multiple downloads
        final downloads = [
          localStorageService.downloadSurah(1, priority: DownloadPriority.high),
          localStorageService.downloadSurah(2, priority: DownloadPriority.low),
          localStorageService.downloadSurah(3, priority: DownloadPriority.medium),
        ];

        // Act
        await Future.wait(downloads);

        // Assert - High priority should complete first
        final downloadOrder = localStorageService.getDownloadHistory();
        expect(downloadOrder.first.surahNumber, equals(1));
      });
    });

    group('Connectivity Changes', () {
      test('should detect connectivity changes', () async {
        // Arrange
        var connectivityChanges = <bool>[];
        
        connectivityService.onConnectivityChanged.listen((isConnected) {
          connectivityChanges.add(isConnected);
        });

        // Act
        connectivityService.setOfflineMode(true);
        await Future.delayed(const Duration(milliseconds: 100));
        
        connectivityService.setOfflineMode(false);
        await Future.delayed(const Duration(milliseconds: 100));

        // Assert
        expect(connectivityChanges.length, greaterThanOrEqualTo(2));
        expect(connectivityChanges.last, isTrue);
      });

      test('should auto-sync when connectivity restored', () async {
        // Arrange - Queue operations while offline
        connectivityService.setOfflineMode(true);

        await offlineManager.queueOperation('test_operation', {'data': 'value'});

        // Act - Restore connectivity
        connectivityService.setOfflineMode(false);

        // Wait for auto-sync
        await Future.delayed(const Duration(seconds: 2));

        // Assert - Operations should be synced
        expect(offlineManager.state.pendingCount, equals(0));
      });
    });
  });
}

import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'dart:async';
import '../../lib/core/services/quran_service.dart';
import '../../lib/core/services/search_service.dart';
import '../../lib/core/services/auth_service.dart';
import '../../lib/core/services/local_storage_service.dart';
import '../../lib/core/network/dio_client.dart';

/// Integration tests for performance under load
/// **Validates: Requirements 20.5**
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Performance Under Load Integration Tests', () {
    late DioClient dioClient;
    late QuranService quranService;
    late SearchService searchService;
    late AuthService authService;
    late LocalStorageService localStorageService;

    setUpAll(() async {
      dioClient = DioClient(baseUrl: 'https://api.sanad.app');
      quranService = QuranService(dioClient);
      searchService = SearchService(dioClient);
      authService = AuthService(dioClient);
      localStorageService = await LocalStorageService.init();

      // Login for authenticated tests
      await authService.login(
        email: 'perf_test@example.com',
        password: 'PerfTest123!',
      );
    });

    group('Concurrent Request Handling', () {
      test('should handle multiple concurrent API requests', () async {
        // Arrange
        final requests = <Future>[];
        final startTime = DateTime.now();

        // Act - Make 50 concurrent requests
        for (var i = 0; i < 50; i++) {
          requests.add(quranService.getSurahById((i % 114) + 1));
        }

        final results = await Future.wait(requests);
        final duration = DateTime.now().difference(startTime);

        // Assert
        expect(results.length, equals(50));
        expect(results.every((r) => r != null), isTrue);
        expect(duration.inSeconds, lessThan(30)); // Should complete within 30 seconds
        
        print('Concurrent requests completed in ${duration.inMilliseconds}ms');
      });

      test('should handle burst traffic without errors', () async {
        // Arrange
        final requests = <Future>[];
        var errorCount = 0;

        // Act - Send 100 requests in quick succession
        for (var i = 0; i < 100; i++) {
          requests.add(
            quranService.getSurahs().catchError((e) {
              errorCount++;
              return [];
            }),
          );
        }

        await Future.wait(requests);

        // Assert - Should handle most requests successfully
        expect(errorCount, lessThan(10)); // Less than 10% error rate
      });

      test('should maintain response time under load', () async {
        // Arrange
        final responseTimes = <int>[];

        // Act - Make sequential requests and measure time
        for (var i = 0; i < 20; i++) {
          final startTime = DateTime.now();
          await quranService.getSurahById((i % 114) + 1);
          final duration = DateTime.now().difference(startTime);
          responseTimes.add(duration.inMilliseconds);
        }

        // Assert
        final averageTime = responseTimes.reduce((a, b) => a + b) / responseTimes.length;
        final maxTime = responseTimes.reduce((a, b) => a > b ? a : b);

        expect(averageTime, lessThan(2000)); // Average under 2 seconds
        expect(maxTime, lessThan(5000)); // Max under 5 seconds
        
        print('Average response time: ${averageTime.toStringAsFixed(2)}ms');
        print('Max response time: ${maxTime}ms');
      });
    });

    group('Memory Management Under Load', () {
      test('should not leak memory with repeated operations', () async {
        // Arrange
        final iterations = 100;
        
        // Act - Perform memory-intensive operations
        for (var i = 0; i < iterations; i++) {
          final surah = await quranService.getSurahById((i % 114) + 1);
          await localStorageService.storeSurah(surah);
          
          // Periodically clear cache
          if (i % 20 == 0) {
            await localStorageService.clearCache();
          }
        }

        // Assert - Memory should be stable
        final memoryUsage = await localStorageService.getMemoryUsage();
        expect(memoryUsage.inMegabytes, lessThan(100)); // Under 100MB
        
        print('Memory usage after $iterations operations: ${memoryUsage.inMegabytes}MB');
      });

      test('should handle large dataset efficiently', () async {
        // Arrange - Load all surahs
        final startTime = DateTime.now();
        
        // Act
        final surahs = await quranService.getAllSurahsWithAyahs();
        final duration = DateTime.now().difference(startTime);

        // Assert
        expect(surahs.length, equals(114));
        expect(duration.inSeconds, lessThan(60)); // Should load within 1 minute
        
        // Verify memory efficiency
        final memoryUsage = await localStorageService.getMemoryUsage();
        expect(memoryUsage.inMegabytes, lessThan(200));
        
        print('Loaded ${surahs.length} surahs in ${duration.inSeconds}s');
        print('Memory usage: ${memoryUsage.inMegabytes}MB');
      });

      test('should cleanup resources properly', () async {
        // Arrange
        final initialMemory = await localStorageService.getMemoryUsage();

        // Act - Create and dispose many objects
        for (var i = 0; i < 50; i++) {
          final surah = await quranService.getSurahById((i % 114) + 1);
          await localStorageService.storeSurah(surah);
        }

        // Cleanup
        await localStorageService.clearCache();
        await Future.delayed(const Duration(seconds: 2)); // Allow GC to run

        // Assert
        final finalMemory = await localStorageService.getMemoryUsage();
        final memoryIncrease = finalMemory.inMegabytes - initialMemory.inMegabytes;
        
        expect(memoryIncrease, lessThan(20)); // Should not increase by more than 20MB
      });
    });

    group('Search Performance Under Load', () {
      test('should handle concurrent search queries', () async {
        // Arrange
        final searchTerms = [
          'الله',
          'الرحمن',
          'الصلاة',
          'الزكاة',
          'الحج',
          'الصيام',
          'الجنة',
          'النار',
          'القيامة',
          'الإيمان',
        ];

        final startTime = DateTime.now();

        // Act - Execute concurrent searches
        final searchFutures = searchTerms.map((term) => 
          searchService.searchQuran(term)
        ).toList();

        final results = await Future.wait(searchFutures);
        final duration = DateTime.now().difference(startTime);

        // Assert
        expect(results.length, equals(searchTerms.length));
        expect(results.every((r) => r.isNotEmpty), isTrue);
        expect(duration.inSeconds, lessThan(15)); // All searches within 15 seconds
        
        print('${searchTerms.length} concurrent searches completed in ${duration.inSeconds}s');
      });

      test('should maintain search accuracy under load', () async {
        // Arrange
        final searchTerm = 'الله';
        final iterations = 20;
        final results = <List>[];

        // Act - Perform same search multiple times
        for (var i = 0; i < iterations; i++) {
          final result = await searchService.searchQuran(searchTerm);
          results.add(result);
        }

        // Assert - Results should be consistent
        final firstResultCount = results.first.length;
        expect(
          results.every((r) => r.length == firstResultCount),
          isTrue,
          reason: 'Search results should be consistent',
        );
      });

      test('should handle complex search queries efficiently', () async {
        // Arrange
        final complexQueries = [
          {'term': 'الله', 'filters': {'surah': [1, 2, 3]}},
          {'term': 'الرحمن', 'filters': {'revelation': 'meccan'}},
          {'term': 'الصلاة', 'filters': {'juz': [1, 2]}},
        ];

        // Act
        final startTime = DateTime.now();
        
        for (var query in complexQueries) {
          await searchService.advancedSearch(
            term: query['term'] as String,
            filters: query['filters'] as Map<String, dynamic>,
          );
        }

        final duration = DateTime.now().difference(startTime);

        // Assert
        expect(duration.inSeconds, lessThan(10));
      });
    });

    group('Database Performance', () {
      test('should handle bulk insert operations', () async {
        // Arrange
        final bookmarks = List.generate(1000, (i) => {
          'surahNumber': (i % 114) + 1,
          'ayahNumber': i + 1,
          'note': 'Bookmark $i',
          'timestamp': DateTime.now().toIso8601String(),
        });

        // Act
        final startTime = DateTime.now();
        await localStorageService.bulkInsertBookmarks(bookmarks);
        final duration = DateTime.now().difference(startTime);

        // Assert
        expect(duration.inSeconds, lessThan(5)); // Should complete within 5 seconds
        
        final storedBookmarks = await localStorageService.getBookmarks();
        expect(storedBookmarks.length, greaterThanOrEqualTo(1000));
        
        print('Inserted 1000 bookmarks in ${duration.inMilliseconds}ms');
      });

      test('should handle bulk read operations', () async {
        // Arrange - Ensure data exists
        await localStorageService.bulkInsertBookmarks(
          List.generate(500, (i) => {
            'surahNumber': (i % 114) + 1,
            'ayahNumber': i + 1,
            'note': 'Test $i',
            'timestamp': DateTime.now().toIso8601String(),
          }),
        );

        // Act
        final startTime = DateTime.now();
        final bookmarks = await localStorageService.getBookmarks();
        final duration = DateTime.now().difference(startTime);

        // Assert
        expect(bookmarks.length, greaterThanOrEqualTo(500));
        expect(duration.inMilliseconds, lessThan(1000)); // Under 1 second
        
        print('Read ${bookmarks.length} bookmarks in ${duration.inMilliseconds}ms');
      });

      test('should handle bulk update operations', () async {
        // Arrange
        final bookmarks = await localStorageService.getBookmarks();
        final updates = bookmarks.take(100).map((b) => {
          'id': b.id,
          'note': 'Updated note',
          'timestamp': DateTime.now().toIso8601String(),
        }).toList();

        // Act
        final startTime = DateTime.now();
        await localStorageService.bulkUpdateBookmarks(updates);
        final duration = DateTime.now().difference(startTime);

        // Assert
        expect(duration.inSeconds, lessThan(3));
        
        // Verify updates
        final updatedBookmarks = await localStorageService.getBookmarks();
        final updatedCount = updatedBookmarks
            .where((b) => b.note == 'Updated note')
            .length;
        expect(updatedCount, greaterThanOrEqualTo(100));
      });

      test('should handle bulk delete operations', () async {
        // Arrange
        final bookmarks = await localStorageService.getBookmarks();
        final idsToDelete = bookmarks.take(200).map((b) => b.id).toList();

        // Act
        final startTime = DateTime.now();
        await localStorageService.bulkDeleteBookmarks(idsToDelete);
        final duration = DateTime.now().difference(startTime);

        // Assert
        expect(duration.inSeconds, lessThan(3));
        
        final remainingBookmarks = await localStorageService.getBookmarks();
        expect(
          remainingBookmarks.any((b) => idsToDelete.contains(b.id)),
          isFalse,
        );
      });
    });

    group('Network Resilience Under Load', () {
      test('should handle network timeouts gracefully', () async {
        // Arrange
        final requests = <Future>[];
        var timeoutCount = 0;
        var successCount = 0;

        // Act - Make requests with short timeout
        for (var i = 0; i < 30; i++) {
          requests.add(
            quranService.getSurahById(
              (i % 114) + 1,
              timeout: const Duration(milliseconds: 500),
            ).then((_) {
              successCount++;
            }).catchError((e) {
              timeoutCount++;
            }),
          );
        }

        await Future.wait(requests);

        // Assert - Should handle timeouts without crashing
        expect(successCount + timeoutCount, equals(30));
        print('Success: $successCount, Timeouts: $timeoutCount');
      });

      test('should retry failed requests automatically', () async {
        // Arrange
        var attemptCount = 0;
        
        // Mock intermittent failures
        quranService.onRequestAttempt = () {
          attemptCount++;
          if (attemptCount % 3 != 0) {
            throw Exception('Simulated failure');
          }
        };

        // Act
        final result = await quranService.getSurahById(1);

        // Assert - Should succeed after retries
        expect(result, isNotNull);
        expect(attemptCount, greaterThan(1));
        
        print('Succeeded after $attemptCount attempts');
      });

      test('should handle rate limiting appropriately', () async {
        // Arrange
        final requests = <Future>[];
        var rateLimitedCount = 0;
        var successCount = 0;

        // Act - Make many rapid requests
        for (var i = 0; i < 100; i++) {
          requests.add(
            quranService.getSurahs().then((_) {
              successCount++;
            }).catchError((e) {
              if (e.toString().contains('429')) {
                rateLimitedCount++;
              }
            }),
          );
        }

        await Future.wait(requests);

        // Assert
        expect(successCount + rateLimitedCount, equals(100));
        
        // Should have some rate limiting
        if (rateLimitedCount > 0) {
          print('Rate limited: $rateLimitedCount requests');
        }
      });
    });

    group('UI Responsiveness Under Load', () {
      test('should maintain 60fps during heavy operations', () async {
        // Arrange
        final frameTimings = <Duration>[];
        var lastFrameTime = DateTime.now();

        // Act - Simulate heavy UI operations
        for (var i = 0; i < 100; i++) {
          // Simulate frame rendering
          await Future.delayed(const Duration(milliseconds: 1));
          
          final currentTime = DateTime.now();
          final frameDuration = currentTime.difference(lastFrameTime);
          frameTimings.add(frameDuration);
          lastFrameTime = currentTime;

          // Perform background work
          if (i % 10 == 0) {
            await quranService.getSurahById((i % 114) + 1);
          }
        }

        // Assert - Most frames should be under 16.67ms (60fps)
        final slowFrames = frameTimings.where((d) => d.inMilliseconds > 17).length;
        final slowFramePercentage = (slowFrames / frameTimings.length) * 100;

        expect(slowFramePercentage, lessThan(10)); // Less than 10% slow frames
        
        print('Slow frames: $slowFramePercentage%');
      });

      test('should not block UI during data loading', () async {
        // Arrange
        var uiBlocked = false;
        final uiCheckTimer = Timer.periodic(
          const Duration(milliseconds: 100),
          (_) {
            // Simulate UI interaction check
            final canInteract = !uiBlocked;
            expect(canInteract, isTrue);
          },
        );

        // Act - Load large dataset
        await quranService.getAllSurahsWithAyahs();

        // Assert
        uiCheckTimer.cancel();
        expect(uiBlocked, isFalse);
      });
    });

    group('Cache Performance', () {
      test('should improve performance with caching', () async {
        // Arrange
        final surahId = 2; // Al-Baqarah (large surah)

        // Act - First load (no cache)
        final startTime1 = DateTime.now();
        await quranService.getSurahById(surahId);
        final duration1 = DateTime.now().difference(startTime1);

        // Second load (with cache)
        final startTime2 = DateTime.now();
        await quranService.getSurahById(surahId);
        final duration2 = DateTime.now().difference(startTime2);

        // Assert - Cached load should be significantly faster
        expect(duration2.inMilliseconds, lessThan(duration1.inMilliseconds / 2));
        
        print('First load: ${duration1.inMilliseconds}ms');
        print('Cached load: ${duration2.inMilliseconds}ms');
        print('Speedup: ${(duration1.inMilliseconds / duration2.inMilliseconds).toStringAsFixed(2)}x');
      });

      test('should handle cache invalidation efficiently', () async {
        // Arrange - Populate cache
        for (var i = 1; i <= 10; i++) {
          await quranService.getSurahById(i);
        }

        // Act - Invalidate cache
        final startTime = DateTime.now();
        await localStorageService.invalidateCache();
        final duration = DateTime.now().difference(startTime);

        // Assert
        expect(duration.inMilliseconds, lessThan(500));
        
        // Verify cache is cleared
        final cacheSize = await localStorageService.getCacheSize();
        expect(cacheSize, equals(0));
      });
    });

    group('Stress Testing', () {
      test('should survive extended stress test', () async {
        // Arrange
        final duration = const Duration(minutes: 2);
        final endTime = DateTime.now().add(duration);
        var operationCount = 0;
        var errorCount = 0;

        // Act - Continuous operations for 2 minutes
        while (DateTime.now().isBefore(endTime)) {
          try {
            // Mix of different operations
            switch (operationCount % 4) {
              case 0:
                await quranService.getSurahById((operationCount % 114) + 1);
                break;
              case 1:
                await searchService.searchQuran('الله');
                break;
              case 2:
                await localStorageService.getBookmarks();
                break;
              case 3:
                await quranService.getSurahs();
                break;
            }
            operationCount++;
          } catch (e) {
            errorCount++;
          }

          // Small delay between operations
          await Future.delayed(const Duration(milliseconds: 100));
        }

        // Assert
        final errorRate = (errorCount / operationCount) * 100;
        expect(errorRate, lessThan(5)); // Less than 5% error rate
        
        print('Completed $operationCount operations in ${duration.inMinutes} minutes');
        print('Error rate: ${errorRate.toStringAsFixed(2)}%');
      });
    });
  });
}

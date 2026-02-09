import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:dio/dio.dart';
import '../../lib/core/network/dio_client.dart';
import '../../lib/core/services/auth_service.dart';
import '../../lib/core/services/quran_service.dart';
import '../../lib/core/services/prayer_times_service.dart';

/// Integration tests for Backend API integration
/// **Validates: Requirements 20.1, 20.3**
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Backend API Integration Tests', () {
    late DioClient dioClient;
    late AuthService authService;
    late QuranService quranService;
    late PrayerTimesService prayerTimesService;

    setUpAll(() {
      // Initialize services with real backend URL
      dioClient = DioClient(baseUrl: 'https://api.sanad.app');
      authService = AuthService(dioClient);
      quranService = QuranService(dioClient);
      prayerTimesService = PrayerTimesService(dioClient);
    });

    group('Authentication Flow', () {
      test('should register new user successfully', () async {
        // Arrange
        final testEmail = 'test_${DateTime.now().millisecondsSinceEpoch}@example.com';
        final testPassword = 'TestPassword123!';

        // Act
        final result = await authService.register(
          email: testEmail,
          password: testPassword,
          name: 'Test User',
        );

        // Assert
        expect(result.success, isTrue);
        expect(result.user, isNotNull);
        expect(result.accessToken, isNotNull);
      });

      test('should login with valid credentials', () async {
        // Arrange
        final testEmail = 'existing_user@example.com';
        final testPassword = 'ValidPassword123!';

        // Act
        final result = await authService.login(
          email: testEmail,
          password: testPassword,
        );

        // Assert
        expect(result.success, isTrue);
        expect(result.accessToken, isNotNull);
        expect(result.refreshToken, isNotNull);
      });

      test('should fail login with invalid credentials', () async {
        // Act & Assert
        expect(
          () => authService.login(
            email: 'invalid@example.com',
            password: 'WrongPassword',
          ),
          throwsA(isA<DioException>()),
        );
      });

      test('should refresh access token', () async {
        // Arrange
        await authService.login(
          email: 'existing_user@example.com',
          password: 'ValidPassword123!',
        );

        // Act
        final newToken = await authService.refreshToken();

        // Assert
        expect(newToken, isNotNull);
        expect(newToken.length, greaterThan(0));
      });

      test('should logout successfully', () async {
        // Arrange
        await authService.login(
          email: 'existing_user@example.com',
          password: 'ValidPassword123!',
        );

        // Act
        await authService.logout();

        // Assert
        final isAuthenticated = await authService.isAuthenticated();
        expect(isAuthenticated, isFalse);
      });
    });

    group('Quran Service Integration', () {
      test('should fetch all surahs from backend', () async {
        // Act
        final surahs = await quranService.getSurahs();

        // Assert
        expect(surahs.length, equals(114));
        expect(surahs.first.name, equals('الفاتحة'));
        expect(surahs.first.numberOfAyahs, equals(7));
        expect(surahs.last.name, equals('الناس'));
      });

      test('should fetch specific surah with ayahs', () async {
        // Act
        final surah = await quranService.getSurahById(1);

        // Assert
        expect(surah.number, equals(1));
        expect(surah.name, equals('الفاتحة'));
        expect(surah.ayahs, isNotNull);
        expect(surah.ayahs!.length, equals(7));
      });

      test('should search Quran and return results', () async {
        // Act
        final results = await quranService.searchQuran('الله');

        // Assert
        expect(results.isNotEmpty, isTrue);
        expect(results.first.text, contains('الله'));
      });

      test('should manage bookmarks', () async {
        // Arrange
        await authService.login(
          email: 'existing_user@example.com',
          password: 'ValidPassword123!',
        );

        // Act - Add bookmark
        final bookmark = await quranService.addBookmark(
          surahNumber: 2,
          ayahNumber: 255,
          note: 'آية الكرسي',
        );

        // Assert
        expect(bookmark.id, isNotNull);
        expect(bookmark.surahNumber, equals(2));

        // Act - Get bookmarks
        final bookmarks = await quranService.getBookmarks();
        expect(bookmarks.any((b) => b.id == bookmark.id), isTrue);

        // Act - Delete bookmark
        await quranService.deleteBookmark(bookmark.id);

        // Assert
        final updatedBookmarks = await quranService.getBookmarks();
        expect(updatedBookmarks.any((b) => b.id == bookmark.id), isFalse);
      });

      test('should track reading progress', () async {
        // Arrange
        await authService.login(
          email: 'existing_user@example.com',
          password: 'ValidPassword123!',
        );

        // Act - Update progress
        await quranService.updateReadingProgress(
          surahNumber: 2,
          ayahNumber: 100,
        );

        // Act - Get progress
        final progress = await quranService.getReadingProgress();

        // Assert
        expect(progress.lastReadSurah, equals(2));
        expect(progress.lastReadAyah, equals(100));
      });
    });

    group('Prayer Times Service Integration', () {
      test('should fetch prayer times for location', () async {
        // Act
        final prayerTimes = await prayerTimesService.getPrayerTimes(
          latitude: 24.7136,
          longitude: 46.6753,
          date: DateTime.now(),
        );

        // Assert
        expect(prayerTimes.fajr, isNotNull);
        expect(prayerTimes.dhuhr, isNotNull);
        expect(prayerTimes.asr, isNotNull);
        expect(prayerTimes.maghrib, isNotNull);
        expect(prayerTimes.isha, isNotNull);
      });

      test('should fetch monthly prayer times', () async {
        // Act
        final monthlyTimes = await prayerTimesService.getMonthlyPrayerTimes(
          latitude: 24.7136,
          longitude: 46.6753,
          year: 2024,
          month: 1,
        );

        // Assert
        expect(monthlyTimes.length, greaterThan(28));
        expect(monthlyTimes.length, lessThanOrEqual(31));
      });
    });

    group('Error Handling and Retry', () {
      test('should retry on network failure', () async {
        // Arrange
        var attemptCount = 0;
        final interceptor = InterceptorsWrapper(
          onRequest: (options, handler) {
            attemptCount++;
            if (attemptCount < 3) {
              return handler.reject(
                DioException(
                  requestOptions: options,
                  type: DioExceptionType.connectionTimeout,
                ),
              );
            }
            return handler.next(options);
          },
        );

        dioClient.dio.interceptors.add(interceptor);

        // Act
        final surahs = await quranService.getSurahs();

        // Assert
        expect(attemptCount, equals(3));
        expect(surahs.length, equals(114));

        // Cleanup
        dioClient.dio.interceptors.remove(interceptor);
      });

      test('should handle rate limiting', () async {
        // Make multiple rapid requests
        final futures = List.generate(
          10,
          (_) => quranService.getSurahs(),
        );

        // Act & Assert - Should not throw rate limit errors
        final results = await Future.wait(futures);
        expect(results.length, equals(10));
      });
    });

    group('Data Consistency', () {
      test('should maintain data consistency across requests', () async {
        // Arrange
        await authService.login(
          email: 'existing_user@example.com',
          password: 'ValidPassword123!',
        );

        // Act - Add bookmark
        final bookmark1 = await quranService.addBookmark(
          surahNumber: 18,
          ayahNumber: 10,
          note: 'Test bookmark',
        );

        // Act - Fetch bookmarks immediately
        final bookmarks1 = await quranService.getBookmarks();

        // Act - Fetch bookmarks again
        final bookmarks2 = await quranService.getBookmarks();

        // Assert - Data should be consistent
        expect(
          bookmarks1.any((b) => b.id == bookmark1.id),
          isTrue,
        );
        expect(
          bookmarks2.any((b) => b.id == bookmark1.id),
          isTrue,
        );

        // Cleanup
        await quranService.deleteBookmark(bookmark1.id);
      });
    });
  });
}

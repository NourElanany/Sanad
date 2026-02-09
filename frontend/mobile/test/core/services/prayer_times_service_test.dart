import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/mockito.dart';
import 'package:mockito/annotations.dart';
import 'package:dio/dio.dart';
import '../../../lib/core/services/prayer_times_service.dart';
import '../../../lib/core/network/dio_client.dart';

@GenerateMocks([DioClient])
void main() {
  group('PrayerTimesService', () {
    late PrayerTimesService service;
    late MockDioClient mockDioClient;

    setUp(() {
      mockDioClient = MockDioClient();
      service = PrayerTimesService(mockDioClient);
    });

    group('getPrayerTimes', () {
      test('should return prayer times for valid location', () async {
        // Arrange
        final mockResponse = {
          'data': {
            'fajr': '05:30',
            'sunrise': '06:45',
            'dhuhr': '12:30',
            'asr': '15:45',
            'maghrib': '18:15',
            'isha': '19:30',
          },
        };

        when(mockDioClient.get(
          any,
          queryParameters: anyNamed('queryParameters'),
        )).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        final result = await service.getPrayerTimes(
          latitude: 24.7136,
          longitude: 46.6753,
          date: DateTime(2024, 1, 15),
        );

        // Assert
        expect(result.fajr, equals('05:30'));
        expect(result.dhuhr, equals('12:30'));
        expect(result.maghrib, equals('18:15'));
      });

      test('should throw exception for invalid coordinates', () async {
        // Arrange
        when(mockDioClient.get(
          any,
          queryParameters: anyNamed('queryParameters'),
        )).thenThrow(DioException(
          requestOptions: RequestOptions(path: ''),
          type: DioExceptionType.badResponse,
          response: Response(
            statusCode: 400,
            requestOptions: RequestOptions(path: ''),
          ),
        ));

        // Act & Assert
        expect(
          () => service.getPrayerTimes(
            latitude: 200.0, // Invalid latitude
            longitude: 46.6753,
            date: DateTime.now(),
          ),
          throwsException,
        );
      });

      test('should cache prayer times for same location and date', () async {
        // Arrange
        final mockResponse = {
          'data': {
            'fajr': '05:30',
            'sunrise': '06:45',
            'dhuhr': '12:30',
            'asr': '15:45',
            'maghrib': '18:15',
            'isha': '19:30',
          },
        };

        when(mockDioClient.get(
          any,
          queryParameters: anyNamed('queryParameters'),
        )).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        await service.getPrayerTimes(
          latitude: 24.7136,
          longitude: 46.6753,
          date: DateTime(2024, 1, 15),
        );

        await service.getPrayerTimes(
          latitude: 24.7136,
          longitude: 46.6753,
          date: DateTime(2024, 1, 15),
        );

        // Assert - Should only call API once due to caching
        verify(mockDioClient.get(
          any,
          queryParameters: anyNamed('queryParameters'),
        )).called(1);
      });
    });

    group('getNextPrayer', () {
      test('should return next prayer correctly', () {
        // Arrange
        final prayerTimes = PrayerTimes(
          fajr: '05:30',
          sunrise: '06:45',
          dhuhr: '12:30',
          asr: '15:45',
          maghrib: '18:15',
          isha: '19:30',
          date: DateTime(2024, 1, 15),
        );

        final currentTime = DateTime(2024, 1, 15, 14, 0); // 2:00 PM

        // Act
        final nextPrayer = service.getNextPrayer(prayerTimes, currentTime);

        // Assert
        expect(nextPrayer.name, equals('asr'));
        expect(nextPrayer.time, equals('15:45'));
      });

      test('should return fajr as next prayer after isha', () {
        // Arrange
        final prayerTimes = PrayerTimes(
          fajr: '05:30',
          sunrise: '06:45',
          dhuhr: '12:30',
          asr: '15:45',
          maghrib: '18:15',
          isha: '19:30',
          date: DateTime(2024, 1, 15),
        );

        final currentTime = DateTime(2024, 1, 15, 20, 0); // 8:00 PM

        // Act
        final nextPrayer = service.getNextPrayer(prayerTimes, currentTime);

        // Assert
        expect(nextPrayer.name, equals('fajr'));
        expect(nextPrayer.isNextDay, isTrue);
      });
    });

    group('calculateTimeRemaining', () {
      test('should calculate time remaining correctly', () {
        // Arrange
        final prayerTime = DateTime(2024, 1, 15, 15, 45);
        final currentTime = DateTime(2024, 1, 15, 14, 30);

        // Act
        final remaining = service.calculateTimeRemaining(prayerTime, currentTime);

        // Assert
        expect(remaining.inHours, equals(1));
        expect(remaining.inMinutes, equals(75));
      });

      test('should return zero for past prayer times', () {
        // Arrange
        final prayerTime = DateTime(2024, 1, 15, 12, 0);
        final currentTime = DateTime(2024, 1, 15, 14, 0);

        // Act
        final remaining = service.calculateTimeRemaining(prayerTime, currentTime);

        // Assert
        expect(remaining.inSeconds, equals(0));
      });
    });

    group('getMonthlyPrayerTimes', () {
      test('should return prayer times for entire month', () async {
        // Arrange
        when(mockDioClient.get(
          any,
          queryParameters: anyNamed('queryParameters'),
        )).thenAnswer((_) async => Response(
              data: {
                'data': List.generate(
                  30,
                  (index) => {
                    'date': DateTime(2024, 1, index + 1).toIso8601String(),
                    'fajr': '05:30',
                    'dhuhr': '12:30',
                    'asr': '15:45',
                    'maghrib': '18:15',
                    'isha': '19:30',
                  },
                ),
              },
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        final result = await service.getMonthlyPrayerTimes(
          latitude: 24.7136,
          longitude: 46.6753,
          year: 2024,
          month: 1,
        );

        // Assert
        expect(result.length, equals(30));
        expect(result.first.date.day, equals(1));
        expect(result.last.date.day, equals(30));
      });
    });
  });
}

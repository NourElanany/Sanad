import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/mockito.dart';
import 'package:mockito/annotations.dart';
import 'package:dio/dio.dart';
import '../../../lib/core/services/quran_service.dart';
import '../../../lib/core/network/dio_client.dart';
import '../../../lib/features/quran/data/models/surah_model.dart';

@GenerateMocks([DioClient])
void main() {
  group('QuranService', () {
    late QuranService service;
    late MockDioClient mockDioClient;

    setUp(() {
      mockDioClient = MockDioClient();
      service = QuranService(mockDioClient);
    });

    group('getSurahs', () {
      test('should return list of all surahs', () async {
        // Arrange
        final mockResponse = {
          'data': [
            {
              'number': 1,
              'name': 'الفاتحة',
              'englishName': 'Al-Fatihah',
              'numberOfAyahs': 7,
              'revelationType': 'Meccan',
            },
            {
              'number': 2,
              'name': 'البقرة',
              'englishName': 'Al-Baqarah',
              'numberOfAyahs': 286,
              'revelationType': 'Medinan',
            },
          ],
        };

        when(mockDioClient.get(any)).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        final result = await service.getSurahs();

        // Assert
        expect(result.length, equals(2));
        expect(result.first.name, equals('الفاتحة'));
        expect(result.first.numberOfAyahs, equals(7));
        expect(result.last.name, equals('البقرة'));
      });

      test('should cache surahs list', () async {
        // Arrange
        final mockResponse = {
          'data': [
            {
              'number': 1,
              'name': 'الفاتحة',
              'englishName': 'Al-Fatihah',
              'numberOfAyahs': 7,
              'revelationType': 'Meccan',
            },
          ],
        };

        when(mockDioClient.get(any)).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        await service.getSurahs();
        await service.getSurahs();

        // Assert - Should only call API once
        verify(mockDioClient.get(any)).called(1);
      });
    });

    group('getSurahById', () {
      test('should return specific surah with ayahs', () async {
        // Arrange
        final mockResponse = {
          'data': {
            'number': 1,
            'name': 'الفاتحة',
            'englishName': 'Al-Fatihah',
            'numberOfAyahs': 7,
            'revelationType': 'Meccan',
            'ayahs': [
              {
                'number': 1,
                'text': 'بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ',
                'numberInSurah': 1,
              },
              {
                'number': 2,
                'text': 'الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ',
                'numberInSurah': 2,
              },
            ],
          },
        };

        when(mockDioClient.get(any)).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        final result = await service.getSurahById(1);

        // Assert
        expect(result.number, equals(1));
        expect(result.name, equals('الفاتحة'));
        expect(result.ayahs?.length, equals(2));
      });

      test('should throw exception for invalid surah number', () async {
        // Arrange
        when(mockDioClient.get(any)).thenThrow(DioException(
          requestOptions: RequestOptions(path: ''),
          type: DioExceptionType.badResponse,
          response: Response(
            statusCode: 404,
            requestOptions: RequestOptions(path: ''),
          ),
        ));

        // Act & Assert
        expect(
          () => service.getSurahById(115), // Invalid surah number
          throwsException,
        );
      });
    });

    group('getAyahsByJuz', () {
      test('should return ayahs for specific juz', () async {
        // Arrange
        final mockResponse = {
          'data': {
            'juzNumber': 1,
            'ayahs': List.generate(
              148,
              (index) => {
                'number': index + 1,
                'text': 'آية رقم ${index + 1}',
                'surah': {'number': 1, 'name': 'الفاتحة'},
              },
            ),
          },
        };

        when(mockDioClient.get(any)).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        final result = await service.getAyahsByJuz(1);

        // Assert
        expect(result.length, equals(148));
      });
    });

    group('searchQuran', () {
      test('should return search results for Arabic query', () async {
        // Arrange
        final mockResponse = {
          'data': {
            'results': [
              {
                'ayah': {
                  'number': 255,
                  'text': 'اللَّهُ لَا إِلَٰهَ إِلَّا هُوَ الْحَيُّ الْقَيُّومُ',
                  'surah': {'number': 2, 'name': 'البقرة'},
                  'numberInSurah': 255,
                },
                'relevance': 0.95,
              },
            ],
            'total': 1,
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
        final result = await service.searchQuran('الله');

        // Assert
        expect(result.length, equals(1));
        expect(result.first.text, contains('اللَّهُ'));
      });

      test('should return empty list for no matches', () async {
        // Arrange
        final mockResponse = {
          'data': {
            'results': [],
            'total': 0,
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
        final result = await service.searchQuran('xyz123');

        // Assert
        expect(result.isEmpty, isTrue);
      });
    });

    group('getBookmarks', () {
      test('should return user bookmarks', () async {
        // Arrange
        final mockResponse = {
          'data': [
            {
              'id': '1',
              'surahNumber': 2,
              'ayahNumber': 255,
              'note': 'آية الكرسي',
              'createdAt': '2024-01-15T10:00:00Z',
            },
            {
              'id': '2',
              'surahNumber': 18,
              'ayahNumber': 10,
              'note': 'سورة الكهف',
              'createdAt': '2024-01-14T10:00:00Z',
            },
          ],
        };

        when(mockDioClient.get(any)).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        final result = await service.getBookmarks();

        // Assert
        expect(result.length, equals(2));
        expect(result.first.surahNumber, equals(2));
        expect(result.first.note, equals('آية الكرسي'));
      });
    });

    group('addBookmark', () {
      test('should add new bookmark successfully', () async {
        // Arrange
        final mockResponse = {
          'data': {
            'id': '3',
            'surahNumber': 36,
            'ayahNumber': 1,
            'note': 'سورة يس',
            'createdAt': '2024-01-15T12:00:00Z',
          },
        };

        when(mockDioClient.post(
          any,
          data: anyNamed('data'),
        )).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 201,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        final result = await service.addBookmark(
          surahNumber: 36,
          ayahNumber: 1,
          note: 'سورة يس',
        );

        // Assert
        expect(result.id, equals('3'));
        expect(result.surahNumber, equals(36));
      });
    });

    group('deleteBookmark', () {
      test('should delete bookmark successfully', () async {
        // Arrange
        when(mockDioClient.delete(any)).thenAnswer((_) async => Response(
              data: {'success': true},
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act & Assert
        expect(
          () => service.deleteBookmark('1'),
          returnsNormally,
        );
      });
    });

    group('getReadingProgress', () {
      test('should return user reading progress', () async {
        // Arrange
        final mockResponse = {
          'data': {
            'lastReadSurah': 2,
            'lastReadAyah': 150,
            'completionPercentage': 15.5,
            'totalAyahsRead': 950,
            'updatedAt': '2024-01-15T10:00:00Z',
          },
        };

        when(mockDioClient.get(any)).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        final result = await service.getReadingProgress();

        // Assert
        expect(result.lastReadSurah, equals(2));
        expect(result.completionPercentage, equals(15.5));
        expect(result.totalAyahsRead, equals(950));
      });
    });

    group('updateReadingProgress', () {
      test('should update reading progress successfully', () async {
        // Arrange
        when(mockDioClient.put(
          any,
          data: anyNamed('data'),
        )).thenAnswer((_) async => Response(
              data: {'success': true},
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act & Assert
        expect(
          () => service.updateReadingProgress(
            surahNumber: 2,
            ayahNumber: 200,
          ),
          returnsNormally,
        );
      });
    });
  });
}

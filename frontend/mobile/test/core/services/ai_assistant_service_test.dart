import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/mockito.dart';
import 'package:mockito/annotations.dart';
import 'package:dio/dio.dart';
import '../../../lib/core/services/ai_assistant_service.dart';
import '../../../lib/core/network/dio_client.dart';

@GenerateMocks([DioClient])
void main() {
  group('AIAssistantService', () {
    late AIAssistantService service;
    late MockDioClient mockDioClient;

    setUp(() {
      mockDioClient = MockDioClient();
      service = AIAssistantService(mockDioClient);
    });

    group('askQuestion', () {
      test('should return AI response with sources', () async {
        // Arrange
        final mockResponse = {
          'data': {
            'answer': 'الصلاة في الطائرة جائزة مع مراعاة الشروط',
            'sources': [
              {
                'type': 'hadith',
                'reference': 'صحيح البخاري',
                'text': 'حديث عن الصلاة في السفر',
                'authenticity': 'sahih',
              },
              {
                'type': 'fatwa',
                'reference': 'فتاوى اللجنة الدائمة',
                'text': 'فتوى عن الصلاة في الطائرة',
              },
            ],
            'confidence': 0.95,
          },
        };

        when(mockDioClient.post(
          any,
          data: anyNamed('data'),
        )).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        final result = await service.askQuestion('ما حكم الصلاة في الطائرة؟');

        // Assert
        expect(result.answer, contains('الصلاة في الطائرة'));
        expect(result.sources.length, equals(2));
        expect(result.sources.first.type, equals('hadith'));
        expect(result.confidence, equals(0.95));
      });

      test('should handle streaming responses', () async {
        // Arrange
        final streamController = StreamController<String>();
        
        when(mockDioClient.post(
          any,
          data: anyNamed('data'),
          options: anyNamed('options'),
        )).thenAnswer((_) async {
          streamController.add('الصلاة ');
          streamController.add('في ');
          streamController.add('الطائرة ');
          streamController.add('جائزة');
          streamController.close();
          
          return Response(
            data: streamController.stream,
            statusCode: 200,
            requestOptions: RequestOptions(path: ''),
          );
        });

        // Act
        final stream = service.askQuestionStream('سؤال');
        final chunks = await stream.toList();

        // Assert
        expect(chunks.length, equals(4));
        expect(chunks.join(), equals('الصلاة في الطائرة جائزة'));
      });

      test('should validate sources authenticity', () async {
        // Arrange
        final mockResponse = {
          'data': {
            'answer': 'إجابة',
            'sources': [
              {
                'type': 'hadith',
                'reference': 'مصدر ضعيف',
                'authenticity': 'weak',
              },
            ],
            'confidence': 0.6,
          },
        };

        when(mockDioClient.post(
          any,
          data: anyNamed('data'),
        )).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        final result = await service.askQuestion('سؤال');

        // Assert
        expect(result.hasWeakSources, isTrue);
        expect(result.confidence, lessThan(0.8));
      });

      test('should handle empty or invalid questions', () async {
        // Act & Assert
        expect(
          () => service.askQuestion(''),
          throwsArgumentError,
        );

        expect(
          () => service.askQuestion('   '),
          throwsArgumentError,
        );
      });

      test('should retry on network failure', () async {
        // Arrange
        var attemptCount = 0;
        
        when(mockDioClient.post(
          any,
          data: anyNamed('data'),
        )).thenAnswer((_) async {
          attemptCount++;
          if (attemptCount < 3) {
            throw DioException(
              requestOptions: RequestOptions(path: ''),
              type: DioExceptionType.connectionTimeout,
            );
          }
          return Response(
            data: {
              'data': {
                'answer': 'إجابة',
                'sources': [],
                'confidence': 0.9,
              },
            },
            statusCode: 200,
            requestOptions: RequestOptions(path: ''),
          );
        });

        // Act
        final result = await service.askQuestion('سؤال');

        // Assert
        expect(attemptCount, equals(3));
        expect(result.answer, equals('إجابة'));
      });
    });

    group('getConversationHistory', () {
      test('should return conversation history', () async {
        // Arrange
        final mockResponse = {
          'data': [
            {
              'id': '1',
              'question': 'ما حكم الصيام؟',
              'answer': 'الصيام واجب',
              'timestamp': '2024-01-15T10:00:00Z',
            },
            {
              'id': '2',
              'question': 'ما حكم الزكاة؟',
              'answer': 'الزكاة ركن من أركان الإسلام',
              'timestamp': '2024-01-15T11:00:00Z',
            },
          ],
        };

        when(mockDioClient.get(any)).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        final result = await service.getConversationHistory();

        // Assert
        expect(result.length, equals(2));
        expect(result.first.question, equals('ما حكم الصيام؟'));
      });
    });

    group('clearConversation', () {
      test('should clear conversation history', () async {
        // Arrange
        when(mockDioClient.delete(any)).thenAnswer((_) async => Response(
              data: {'success': true},
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act & Assert
        expect(
          () => service.clearConversation(),
          returnsNormally,
        );
      });
    });

    group('reportIssue', () {
      test('should report incorrect answer', () async {
        // Arrange
        when(mockDioClient.post(
          any,
          data: anyNamed('data'),
        )).thenAnswer((_) async => Response(
              data: {'success': true},
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        await service.reportIssue(
          conversationId: '123',
          issueType: 'incorrect_answer',
          description: 'الإجابة غير دقيقة',
        );

        // Assert
        verify(mockDioClient.post(
          any,
          data: anyNamed('data'),
        )).called(1);
      });
    });

    group('getSuggestedQuestions', () {
      test('should return suggested questions', () async {
        // Arrange
        final mockResponse = {
          'data': [
            'ما هي أركان الإسلام؟',
            'كيف أتوضأ؟',
            'ما هي شروط الصلاة؟',
          ],
        };

        when(mockDioClient.get(any)).thenAnswer((_) async => Response(
              data: mockResponse,
              statusCode: 200,
              requestOptions: RequestOptions(path: ''),
            ));

        // Act
        final result = await service.getSuggestedQuestions();

        // Assert
        expect(result.length, equals(3));
        expect(result.first, equals('ما هي أركان الإسلام؟'));
      });
    });
  });
}

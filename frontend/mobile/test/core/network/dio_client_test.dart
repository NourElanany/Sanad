import 'package:flutter_test/flutter_test.dart';
import 'package:dio/dio.dart';
import 'package:mockito/mockito.dart';
import 'package:mockito/annotations.dart';
import 'package:sanad_mobile/core/network/dio_client.dart';

@GenerateMocks([Dio])
import 'dio_client_test.mocks.dart';

void main() {
  group('DioClient', () {
    late DioClient dioClient;

    setUp(() {
      dioClient = DioClient();
    });

    test('should create Dio instance with correct base options', () {
      final dio = dioClient.dio;
      
      expect(dio.options.baseUrl, isNotEmpty);
      expect(dio.options.headers['Content-Type'], equals('application/json'));
      expect(dio.options.headers['Accept'], equals('application/json'));
    });

    test('should have interceptors configured', () {
      final dio = dioClient.dio;
      
      // Should have at least connectivity, auth, and retry interceptors
      expect(dio.interceptors.length, greaterThanOrEqualTo(3));
    });

    group('Error Handling', () {
      test('should throw NetworkException on connection timeout', () {
        final error = DioException(
          requestOptions: RequestOptions(path: '/test'),
          type: DioExceptionType.connectionTimeout,
        );

        expect(
          () => dioClient.get('/test'),
          throwsA(isA<NetworkException>()),
        );
      });

      test('should throw UnauthorizedException on 401 status', () {
        final error = DioException(
          requestOptions: RequestOptions(path: '/test'),
          type: DioExceptionType.badResponse,
          response: Response(
            requestOptions: RequestOptions(path: '/test'),
            statusCode: 401,
            data: {'message': 'Unauthorized'},
          ),
        );

        expect(
          () => throw dioClient._handleError(error),
          throwsA(isA<UnauthorizedException>()),
        );
      });

      test('should throw NotFoundException on 404 status', () {
        final error = DioException(
          requestOptions: RequestOptions(path: '/test'),
          type: DioExceptionType.badResponse,
          response: Response(
            requestOptions: RequestOptions(path: '/test'),
            statusCode: 404,
            data: {'message': 'Not found'},
          ),
        );

        expect(
          () => throw dioClient._handleError(error),
          throwsA(isA<NotFoundException>()),
        );
      });

      test('should throw ValidationException on 422 status', () {
        final error = DioException(
          requestOptions: RequestOptions(path: '/test'),
          type: DioExceptionType.badResponse,
          response: Response(
            requestOptions: RequestOptions(path: '/test'),
            statusCode: 422,
            data: {
              'message': 'Validation failed',
              'errors': {'email': 'Invalid email'},
            },
          ),
        );

        expect(
          () => throw dioClient._handleError(error),
          throwsA(isA<ValidationException>()),
        );
      });

      test('should throw ServerException on 500 status', () {
        final error = DioException(
          requestOptions: RequestOptions(path: '/test'),
          type: DioExceptionType.badResponse,
          response: Response(
            requestOptions: RequestOptions(path: '/test'),
            statusCode: 500,
            data: {'message': 'Internal server error'},
          ),
        );

        expect(
          () => throw dioClient._handleError(error),
          throwsA(isA<ServerException>()),
        );
      });
    });
  });

  group('NetworkException', () {
    test('should create exception with message and status code', () {
      final exception = NetworkException(
        message: 'Test error',
        statusCode: 400,
      );

      expect(exception.message, equals('Test error'));
      expect(exception.statusCode, equals(400));
      expect(exception.toString(), contains('NetworkException'));
      expect(exception.toString(), contains('Test error'));
    });
  });

  group('ValidationException', () {
    test('should create exception with errors map', () {
      final exception = ValidationException(
        message: 'Validation failed',
        statusCode: 422,
        errors: {'email': 'Invalid email', 'password': 'Too short'},
      );

      expect(exception.message, equals('Validation failed'));
      expect(exception.statusCode, equals(422));
      expect(exception.errors, isNotNull);
      expect(exception.errors!['email'], equals('Invalid email'));
      expect(exception.toString(), contains('Validation failed'));
    });
  });
}

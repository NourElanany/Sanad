import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:mockito/mockito.dart';
import 'package:mockito/annotations.dart';

// Import providers
import 'package:sanad_app/core/providers/cache_provider.dart';
import 'package:sanad_app/core/providers/offline_provider.dart';
import 'package:sanad_app/core/providers/error_handler_provider.dart';
import 'package:sanad_app/core/providers/optimistic_update_provider.dart';

@GenerateMocks([Box])
void main() {
  group('Cache Provider Tests', () {
    late Box mockBox;
    late CacheService cacheService;

    setUp(() {
      mockBox = MockBox();
      cacheService = CacheService(mockBox, const CacheConfig());
    });

    test('should store and retrieve data from cache', () async {
      // Arrange
      final testData = {'key': 'value'};
      when(mockBox.put(any, any)).thenAnswer((_) async => {});
      when(mockBox.get(any)).thenReturn(null);

      // Act
      await cacheService.put('test_key', testData);

      // Assert
      verify(mockBox.put('test_key', any)).called(1);
    });

    test('should return null for expired cache items', () {
      // Arrange
      final expiredData = {
        'data': {'key': 'value'},
        'cachedAt': DateTime.now().subtract(const Duration(days: 2)).toIso8601String(),
        'ttl': 86400, // 1 day in seconds
      };
      when(mockBox.get('test_key')).thenReturn(jsonEncode(expiredData));

      // Act
      final result = cacheService.get<Map<String, dynamic>>(
        'test_key',
        (json) => Map<String, dynamic>.from(json),
      );

      // Assert
      expect(result, isNull);
    });

    test('should check if key exists and is not expired', () {
      // Arrange
      final validData = {
        'data': {'key': 'value'},
        'cachedAt': DateTime.now().toIso8601String(),
        'ttl': 86400,
      };
      when(mockBox.get('test_key')).thenReturn(jsonEncode(validData));

      // Act
      final exists = cacheService.has('test_key');

      // Assert
      expect(exists, isTrue);
    });

    test('should clear all cache', () async {
      // Arrange
      when(mockBox.clear()).thenAnswer((_) async => 0);

      // Act
      await cacheService.clear();

      // Assert
      verify(mockBox.clear()).called(1);
    });
  });

  group('Offline Provider Tests', () {
    late Box mockBox;
    late OfflineManager offlineManager;

    setUp(() {
      mockBox = MockBox();
      when(mockBox.keys).thenReturn([]);
      offlineManager = OfflineManager(mockBox);
    });

    test('should queue operation when offline', () async {
      // Arrange
      when(mockBox.put(any, any)).thenAnswer((_) async => {});

      // Act
      await offlineManager.queueOperation('test_operation', {'data': 'value'});

      // Assert
      expect(offlineManager.state.pendingCount, equals(1));
      verify(mockBox.put(any, any)).called(1);
    });

    test('should process pending operations', () async {
      // Arrange
      final operation = OfflineQueueItem(
        id: '1',
        operation: 'test_operation',
        data: {'data': 'value'},
        createdAt: DateTime.now(),
      );
      
      when(mockBox.keys).thenReturn(['1']);
      when(mockBox.get('1')).thenReturn(jsonEncode(operation.toJson()));
      when(mockBox.delete(any)).thenAnswer((_) async => {});

      // Reload to get the operation
      offlineManager = OfflineManager(mockBox);
      await Future.delayed(const Duration(milliseconds: 100));

      // Act
      await offlineManager.processPendingOperations((op) async {
        // Simulate successful processing
      });

      // Assert
      expect(offlineManager.state.pendingCount, equals(0));
    });

    test('should retry failed operations up to 3 times', () async {
      // Arrange
      final operation = OfflineQueueItem(
        id: '1',
        operation: 'test_operation',
        data: {'data': 'value'},
        createdAt: DateTime.now(),
        retryCount: 2,
      );
      
      when(mockBox.keys).thenReturn(['1']);
      when(mockBox.get('1')).thenReturn(jsonEncode(operation.toJson()));
      when(mockBox.put(any, any)).thenAnswer((_) async => {});
      when(mockBox.delete(any)).thenAnswer((_) async => {});

      offlineManager = OfflineManager(mockBox);
      await Future.delayed(const Duration(milliseconds: 100));

      // Act
      await offlineManager.processPendingOperations((op) async {
        throw Exception('Test error');
      });

      // Assert - Should be removed after 3rd retry
      expect(offlineManager.state.pendingCount, equals(0));
    });
  });

  group('Error Handler Provider Tests', () {
    test('should create AppError from DioException', () {
      // Arrange
      final dioError = DioException(
        requestOptions: RequestOptions(path: '/test'),
        type: DioExceptionType.connectionTimeout,
      );

      // Act
      final appError = AppError.fromException(dioError);

      // Assert
      expect(appError.type, equals(ErrorType.network));
      expect(appError.message, contains('انتهت مهلة الاتصال'));
    });

    test('should handle authentication errors', () {
      // Arrange
      final dioError = DioException(
        requestOptions: RequestOptions(path: '/test'),
        type: DioExceptionType.badResponse,
        response: Response(
          requestOptions: RequestOptions(path: '/test'),
          statusCode: 401,
        ),
      );

      // Act
      final appError = AppError.fromException(dioError);

      // Assert
      expect(appError.type, equals(ErrorType.authentication));
      expect(appError.statusCode, equals(401));
    });

    test('should provide user-friendly messages', () {
      // Arrange
      final networkError = AppError(
        type: ErrorType.network,
        message: 'Connection failed',
      );

      // Act
      final message = networkError.userFriendlyMessage;

      // Assert
      expect(message, contains('اتصالك بالإنترنت'));
    });
  });

  group('Optimistic Update Provider Tests', () {
    test('should apply optimistic update immediately', () async {
      // Arrange
      final container = ProviderContainer();
      final notifier = OptimisticUpdateManager<List<String>>([]);
      
      final operation = OptimisticOperation<List<String>>(
        id: 'test_op',
        optimisticData: ['item1', 'item2'],
        serverOperation: () async {
          await Future.delayed(const Duration(milliseconds: 100));
          return ['item1', 'item2', 'item3'];
        },
      );

      // Act
      final future = notifier.executeOptimistic(operation);
      
      // Assert - Optimistic data should be applied immediately
      expect(notifier.state.data, equals(['item1', 'item2']));
      expect(notifier.state.isProcessing, isTrue);
      
      await future;
      
      // Assert - Server data should replace optimistic data
      expect(notifier.state.data, equals(['item1', 'item2', 'item3']));
      expect(notifier.state.isProcessing, isFalse);
      
      container.dispose();
    });

    test('should rollback on error', () async {
      // Arrange
      final notifier = OptimisticUpdateManager<List<String>>(['initial']);
      
      final operation = OptimisticOperation<List<String>>(
        id: 'test_op',
        optimisticData: ['optimistic'],
        serverOperation: () async {
          throw Exception('Server error');
        },
      );

      // Act
      try {
        await notifier.executeOptimistic(operation);
      } catch (e) {
        // Expected to throw
      }

      // Assert - Should rollback to initial data
      expect(notifier.state.data, equals(['initial']));
      expect(notifier.state.error, isNotNull);
    });

    test('should track pending operations', () async {
      // Arrange
      final notifier = OptimisticUpdateManager<List<String>>([]);
      
      final operation = OptimisticOperation<List<String>>(
        id: 'test_op',
        optimisticData: ['item'],
        serverOperation: () async {
          await Future.delayed(const Duration(milliseconds: 100));
          return ['item'];
        },
      );

      // Act
      final future = notifier.executeOptimistic(operation);
      
      // Assert - Operation should be pending
      expect(notifier.isOperationPending('test_op'), isTrue);
      
      await future;
      
      // Assert - Operation should no longer be pending
      expect(notifier.isOperationPending('test_op'), isFalse);
    });
  });

  group('Integration Tests', () {
    test('should integrate cache with offline queue', () async {
      // Arrange
      final cacheBox = await Hive.openBox('test_cache');
      final offlineBox = await Hive.openBox('test_offline');
      
      final cacheService = CacheService(cacheBox, const CacheConfig());
      final offlineManager = OfflineManager(offlineBox);

      // Act - Cache some data
      await cacheService.put('test_data', {'value': 'cached'});
      
      // Act - Queue an operation
      await offlineManager.queueOperation('sync_data', {'value': 'queued'});

      // Assert
      expect(cacheService.has('test_data'), isTrue);
      expect(offlineManager.state.pendingCount, equals(1));

      // Cleanup
      await cacheBox.close();
      await offlineBox.close();
    });
  });
}

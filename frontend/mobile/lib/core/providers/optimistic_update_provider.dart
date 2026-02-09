import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dart:async';

/// Optimistic update operation
class OptimisticOperation<T> {
  final String id;
  final T optimisticData;
  final Future<T> Function() serverOperation;
  final void Function(T)? onSuccess;
  final void Function(dynamic)? onError;
  final void Function()? onRollback;

  OptimisticOperation({
    required this.id,
    required this.optimisticData,
    required this.serverOperation,
    this.onSuccess,
    this.onError,
    this.onRollback,
  });
}

/// Optimistic update state
class OptimisticUpdateState<T> {
  final T? data;
  final bool isProcessing;
  final dynamic error;
  final List<String> pendingOperations;

  const OptimisticUpdateState({
    this.data,
    this.isProcessing = false,
    this.error,
    this.pendingOperations = const [],
  });

  OptimisticUpdateState<T> copyWith({
    T? data,
    bool? isProcessing,
    dynamic error,
    List<String>? pendingOperations,
  }) {
    return OptimisticUpdateState<T>(
      data: data ?? this.data,
      isProcessing: isProcessing ?? this.isProcessing,
      error: error,
      pendingOperations: pendingOperations ?? this.pendingOperations,
    );
  }
}

/// Optimistic update manager
class OptimisticUpdateManager<T> extends StateNotifier<OptimisticUpdateState<T>> {
  final Map<String, T> _rollbackData = {};

  OptimisticUpdateManager(T? initialData)
      : super(OptimisticUpdateState<T>(data: initialData));

  /// Execute optimistic update
  Future<void> executeOptimistic(OptimisticOperation<T> operation) async {
    // Store current data for rollback
    if (state.data != null) {
      _rollbackData[operation.id] = state.data as T;
    }

    // Apply optimistic update immediately
    state = state.copyWith(
      data: operation.optimisticData,
      isProcessing: true,
      pendingOperations: [...state.pendingOperations, operation.id],
    );

    try {
      // Execute server operation
      final result = await operation.serverOperation();

      // Update with server result
      state = state.copyWith(
        data: result,
        isProcessing: false,
        error: null,
        pendingOperations: state.pendingOperations
            .where((id) => id != operation.id)
            .toList(),
      );

      // Call success callback
      operation.onSuccess?.call(result);

      // Clean up rollback data
      _rollbackData.remove(operation.id);
    } catch (error) {
      // Rollback to previous state
      final rollbackValue = _rollbackData[operation.id];
      if (rollbackValue != null) {
        state = state.copyWith(
          data: rollbackValue,
          isProcessing: false,
          error: error,
          pendingOperations: state.pendingOperations
              .where((id) => id != operation.id)
              .toList(),
        );
      }

      // Call error callback
      operation.onError?.call(error);

      // Call rollback callback
      operation.onRollback?.call();

      // Clean up
      _rollbackData.remove(operation.id);

      rethrow;
    }
  }

  /// Update data directly (non-optimistic)
  void updateData(T data) {
    state = state.copyWith(data: data);
  }

  /// Clear error
  void clearError() {
    state = state.copyWith(error: null);
  }

  /// Check if operation is pending
  bool isOperationPending(String operationId) {
    return state.pendingOperations.contains(operationId);
  }
}

/// Example: Bookmark optimistic update provider
final bookmarkOptimisticProvider = StateNotifierProvider.family<
    OptimisticUpdateManager<List<String>>,
    OptimisticUpdateState<List<String>>,
    String>((ref, userId) {
  return OptimisticUpdateManager<List<String>>([]);
});

/// Example: Reading progress optimistic update provider
final readingProgressOptimisticProvider = StateNotifierProvider.family<
    OptimisticUpdateManager<Map<String, dynamic>>,
    OptimisticUpdateState<Map<String, dynamic>>,
    String>((ref, userId) {
  return OptimisticUpdateManager<Map<String, dynamic>>({});
});

/// Helper function to create optimistic operation
OptimisticOperation<T> createOptimisticOperation<T>({
  required String id,
  required T optimisticData,
  required Future<T> Function() serverOperation,
  void Function(T)? onSuccess,
  void Function(dynamic)? onError,
  void Function()? onRollback,
}) {
  return OptimisticOperation<T>(
    id: id,
    optimisticData: optimisticData,
    serverOperation: serverOperation,
    onSuccess: onSuccess,
    onError: onError,
    onRollback: onRollback,
  );
}

/// Example usage:
/// 
/// ```dart
/// // Add bookmark optimistically
/// final operation = createOptimisticOperation<List<String>>(
///   id: 'add_bookmark_${surahId}_${ayahId}',
///   optimisticData: [...currentBookmarks, newBookmarkId],
///   serverOperation: () => apiService.addBookmark(surahId, ayahId),
///   onSuccess: (result) => print('Bookmark added successfully'),
///   onError: (error) => showErrorSnackbar(context, error),
///   onRollback: () => print('Bookmark addition rolled back'),
/// );
/// 
/// await ref.read(bookmarkOptimisticProvider(userId).notifier)
///     .executeOptimistic(operation);
/// ```

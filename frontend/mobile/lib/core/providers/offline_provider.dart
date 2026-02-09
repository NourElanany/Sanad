import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'dart:convert';

/// Offline queue item for pending operations
class OfflineQueueItem {
  final String id;
  final String operation;
  final Map<String, dynamic> data;
  final DateTime createdAt;
  final int retryCount;

  OfflineQueueItem({
    required this.id,
    required this.operation,
    required this.data,
    required this.createdAt,
    this.retryCount = 0,
  });

  OfflineQueueItem copyWith({
    int? retryCount,
  }) {
    return OfflineQueueItem(
      id: id,
      operation: operation,
      data: data,
      createdAt: createdAt,
      retryCount: retryCount ?? this.retryCount,
    );
  }

  Map<String, dynamic> toJson() => {
        'id': id,
        'operation': operation,
        'data': data,
        'createdAt': createdAt.toIso8601String(),
        'retryCount': retryCount,
      };

  factory OfflineQueueItem.fromJson(Map<String, dynamic> json) {
    return OfflineQueueItem(
      id: json['id'],
      operation: json['operation'],
      data: Map<String, dynamic>.from(json['data']),
      createdAt: DateTime.parse(json['createdAt']),
      retryCount: json['retryCount'] ?? 0,
    );
  }
}

/// Offline state
class OfflineState {
  final List<OfflineQueueItem> pendingOperations;
  final bool isSyncing;
  final String? syncError;

  const OfflineState({
    this.pendingOperations = const [],
    this.isSyncing = false,
    this.syncError,
  });

  OfflineState copyWith({
    List<OfflineQueueItem>? pendingOperations,
    bool? isSyncing,
    String? syncError,
  }) {
    return OfflineState(
      pendingOperations: pendingOperations ?? this.pendingOperations,
      isSyncing: isSyncing ?? this.isSyncing,
      syncError: syncError,
    );
  }

  int get pendingCount => pendingOperations.length;
  bool get hasPending => pendingOperations.isNotEmpty;
}

/// Offline manager for handling offline operations
class OfflineManager extends StateNotifier<OfflineState> {
  final Box _offlineBox;

  OfflineManager(this._offlineBox) : super(const OfflineState()) {
    _loadPendingOperations();
  }

  /// Load pending operations from storage
  Future<void> _loadPendingOperations() async {
    final operations = <OfflineQueueItem>[];
    for (var key in _offlineBox.keys) {
      try {
        final json = jsonDecode(_offlineBox.get(key));
        operations.add(OfflineQueueItem.fromJson(json));
      } catch (e) {
        // Invalid item, remove it
        await _offlineBox.delete(key);
      }
    }

    state = state.copyWith(pendingOperations: operations);
  }

  /// Add operation to offline queue
  Future<void> queueOperation(
    String operation,
    Map<String, dynamic> data,
  ) async {
    final item = OfflineQueueItem(
      id: DateTime.now().millisecondsSinceEpoch.toString(),
      operation: operation,
      data: data,
      createdAt: DateTime.now(),
    );

    await _offlineBox.put(item.id, jsonEncode(item.toJson()));

    state = state.copyWith(
      pendingOperations: [...state.pendingOperations, item],
    );
  }

  /// Process pending operations
  Future<void> processPendingOperations(
    Future<void> Function(OfflineQueueItem) processor,
  ) async {
    if (state.isSyncing || state.pendingOperations.isEmpty) return;

    state = state.copyWith(isSyncing: true, syncError: null);

    final operations = List<OfflineQueueItem>.from(state.pendingOperations);
    final processed = <String>[];
    String? error;

    for (var operation in operations) {
      try {
        await processor(operation);
        processed.add(operation.id);
        await _offlineBox.delete(operation.id);
      } catch (e) {
        error = e.toString();
        // Increment retry count
        final updated = operation.copyWith(retryCount: operation.retryCount + 1);
        
        // Remove if max retries exceeded
        if (updated.retryCount >= 3) {
          processed.add(operation.id);
          await _offlineBox.delete(operation.id);
        } else {
          await _offlineBox.put(updated.id, jsonEncode(updated.toJson()));
        }
      }
    }

    final remaining = operations
        .where((op) => !processed.contains(op.id))
        .toList();

    state = state.copyWith(
      pendingOperations: remaining,
      isSyncing: false,
      syncError: error,
    );
  }

  /// Clear all pending operations
  Future<void> clearPending() async {
    await _offlineBox.clear();
    state = state.copyWith(pendingOperations: []);
  }

  /// Remove specific operation
  Future<void> removeOperation(String id) async {
    await _offlineBox.delete(id);
    state = state.copyWith(
      pendingOperations: state.pendingOperations
          .where((op) => op.id != id)
          .toList(),
    );
  }
}

/// Offline box provider
final offlineBoxProvider = Provider<Box>((ref) {
  return Hive.box('offline_queue');
});

/// Offline manager provider
final offlineManagerProvider = StateNotifierProvider<OfflineManager, OfflineState>((ref) {
  final box = ref.watch(offlineBoxProvider);
  return OfflineManager(box);
});

/// Initialize offline storage
Future<void> initializeOfflineStorage() async {
  await Hive.openBox('offline_queue');
}

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'dart:convert';
import 'dart:async';
import 'package:connectivity_plus/connectivity_plus.dart';
import 'package:dio/dio.dart';

/// CRDT Synchronization Provider
/// Implements conflict-free replicated data type synchronization
/// with the backend state-management-service

/// Connection quality assessment
class ConnectionQuality {
  final double bandwidthMbps;
  final int latencyMs;
  final double stabilityScore; // 0.0 to 1.0
  final DateTime lastAssessed;

  const ConnectionQuality({
    required this.bandwidthMbps,
    required this.latencyMs,
    required this.stabilityScore,
    required this.lastAssessed,
  });

  ConnectionQuality copyWith({
    double? bandwidthMbps,
    int? latencyMs,
    double? stabilityScore,
    DateTime? lastAssessed,
  }) {
    return ConnectionQuality(
      bandwidthMbps: bandwidthMbps ?? this.bandwidthMbps,
      latencyMs: latencyMs ?? this.latencyMs,
      stabilityScore: stabilityScore ?? this.stabilityScore,
      lastAssessed: lastAssessed ?? this.lastAssessed,
    );
  }

  Map<String, dynamic> toJson() => {
        'bandwidthMbps': bandwidthMbps,
        'latencyMs': latencyMs,
        'stabilityScore': stabilityScore,
        'lastAssessed': lastAssessed.toIso8601String(),
      };

  factory ConnectionQuality.fromJson(Map<String, dynamic> json) {
    return ConnectionQuality(
      bandwidthMbps: json['bandwidthMbps']?.toDouble() ?? 10.0,
      latencyMs: json['latencyMs'] ?? 100,
      stabilityScore: json['stabilityScore']?.toDouble() ?? 0.8,
      lastAssessed: DateTime.parse(json['lastAssessed']),
    );
  }

  // Default quality for initial state
  factory ConnectionQuality.defaultQuality() {
    return ConnectionQuality(
      bandwidthMbps: 10.0,
      latencyMs: 100,
      stabilityScore: 0.8,
      lastAssessed: DateTime.now(),
    );
  }
}

/// Sync priority levels
enum SyncPriority {
  critical, // Prayer times, khatma progress
  high, // Reading progress, bookmarks
  normal, // Notes, preferences
  low, // Historical data, analytics
}

/// Sync operation types
enum SyncOperationType {
  bookmarkAdd,
  bookmarkUpdate,
  bookmarkDelete,
  progressUpdate,
  noteAdd,
  noteUpdate,
  noteDelete,
  preferenceUpdate,
  fullSync,
}

/// Sync operation for queue
class SyncOperation {
  final String id;
  final SyncOperationType type;
  final Map<String, dynamic> data;
  final SyncPriority priority;
  final DateTime createdAt;
  final int retryCount;
  final Map<String, int> versionVector;

  SyncOperation({
    required this.id,
    required this.type,
    required this.data,
    required this.priority,
    required this.createdAt,
    this.retryCount = 0,
    Map<String, int>? versionVector,
  }) : versionVector = versionVector ?? {};

  SyncOperation copyWith({
    int? retryCount,
    Map<String, int>? versionVector,
  }) {
    return SyncOperation(
      id: id,
      type: type,
      data: data,
      priority: priority,
      createdAt: createdAt,
      retryCount: retryCount ?? this.retryCount,
      versionVector: versionVector ?? this.versionVector,
    );
  }

  Map<String, dynamic> toJson() => {
        'id': id,
        'type': type.toString(),
        'data': data,
        'priority': priority.toString(),
        'createdAt': createdAt.toIso8601String(),
        'retryCount': retryCount,
        'versionVector': versionVector,
      };

  factory SyncOperation.fromJson(Map<String, dynamic> json) {
    return SyncOperation(
      id: json['id'],
      type: SyncOperationType.values.firstWhere(
        (e) => e.toString() == json['type'],
      ),
      data: Map<String, dynamic>.from(json['data']),
      priority: SyncPriority.values.firstWhere(
        (e) => e.toString() == json['priority'],
      ),
      createdAt: DateTime.parse(json['createdAt']),
      retryCount: json['retryCount'] ?? 0,
      versionVector: Map<String, int>.from(json['versionVector'] ?? {}),
    );
  }
}

/// CRDT Sync State
class CRDTSyncState {
  final List<SyncOperation> pendingOperations;
  final List<SyncOperation> priorityOperations;
  final bool isSyncing;
  final DateTime? lastSyncTime;
  final ConnectionQuality connectionQuality;
  final String? syncError;
  final Map<String, int> localVersionVector;
  final int syncedItemsCount;
  final int conflictsResolvedCount;

  const CRDTSyncState({
    this.pendingOperations = const [],
    this.priorityOperations = const [],
    this.isSyncing = false,
    this.lastSyncTime,
    required this.connectionQuality,
    this.syncError,
    this.localVersionVector = const {},
    this.syncedItemsCount = 0,
    this.conflictsResolvedCount = 0,
  });

  CRDTSyncState copyWith({
    List<SyncOperation>? pendingOperations,
    List<SyncOperation>? priorityOperations,
    bool? isSyncing,
    DateTime? lastSyncTime,
    ConnectionQuality? connectionQuality,
    String? syncError,
    Map<String, int>? localVersionVector,
    int? syncedItemsCount,
    int? conflictsResolvedCount,
  }) {
    return CRDTSyncState(
      pendingOperations: pendingOperations ?? this.pendingOperations,
      priorityOperations: priorityOperations ?? this.priorityOperations,
      isSyncing: isSyncing ?? this.isSyncing,
      lastSyncTime: lastSyncTime ?? this.lastSyncTime,
      connectionQuality: connectionQuality ?? this.connectionQuality,
      syncError: syncError,
      localVersionVector: localVersionVector ?? this.localVersionVector,
      syncedItemsCount: syncedItemsCount ?? this.syncedItemsCount,
      conflictsResolvedCount:
          conflictsResolvedCount ?? this.conflictsResolvedCount,
    );
  }

  int get totalPendingCount =>
      pendingOperations.length + priorityOperations.length;
  bool get hasPending => totalPendingCount > 0;
  bool get isHealthy => syncError == null && !isSyncing;
}

/// CRDT Sync Manager
class CRDTSyncManager extends StateNotifier<CRDTSyncState> {
  final Box _syncBox;
  final Box _dataBox;
  final Dio _dio;
  final String deviceId;
  Timer? _periodicSyncTimer;
  Timer? _qualityMonitorTimer;
  StreamSubscription<ConnectivityResult>? _connectivitySubscription;

  CRDTSyncManager(
    this._syncBox,
    this._dataBox,
    this._dio,
    this.deviceId,
  ) : super(CRDTSyncState(
          connectionQuality: ConnectionQuality.defaultQuality(),
        )) {
    _loadState();
    _startAdaptiveSync();
    _startConnectionMonitoring();
  }

  /// Load state from storage
  Future<void> _loadState() async {
    try {
      // Load pending operations
      final pendingOps = <SyncOperation>[];
      final priorityOps = <SyncOperation>[];

      for (var key in _syncBox.keys) {
        try {
          final json = jsonDecode(_syncBox.get(key));
          final op = SyncOperation.fromJson(json);

          if (op.priority == SyncPriority.critical) {
            priorityOps.add(op);
          } else {
            pendingOps.add(op);
          }
        } catch (e) {
          // Invalid operation, remove it
          await _syncBox.delete(key);
        }
      }

      // Load version vector
      final versionVectorJson = _dataBox.get('version_vector');
      final versionVector = versionVectorJson != null
          ? Map<String, int>.from(jsonDecode(versionVectorJson))
          : <String, int>{};

      // Load connection quality
      final qualityJson = _dataBox.get('connection_quality');
      final quality = qualityJson != null
          ? ConnectionQuality.fromJson(jsonDecode(qualityJson))
          : ConnectionQuality.defaultQuality();

      // Load last sync time
      final lastSyncStr = _dataBox.get('last_sync_time');
      final lastSync =
          lastSyncStr != null ? DateTime.parse(lastSyncStr) : null;

      state = state.copyWith(
        pendingOperations: pendingOps,
        priorityOperations: priorityOps,
        localVersionVector: versionVector,
        connectionQuality: quality,
        lastSyncTime: lastSync,
      );
    } catch (e) {
      // Error loading state, start fresh
      state = CRDTSyncState(
        connectionQuality: ConnectionQuality.defaultQuality(),
      );
    }
  }

  /// Queue a sync operation
  Future<void> queueOperation(
    SyncOperationType type,
    Map<String, dynamic> data,
    SyncPriority priority,
  ) async {
    final operation = SyncOperation(
      id: '${DateTime.now().millisecondsSinceEpoch}_${type.toString()}',
      type: type,
      data: data,
      priority: priority,
      createdAt: DateTime.now(),
      versionVector: Map.from(state.localVersionVector),
    );

    // Increment local version
    final newVersionVector = Map<String, int>.from(state.localVersionVector);
    newVersionVector[deviceId] = (newVersionVector[deviceId] ?? 0) + 1;

    // Save to storage
    await _syncBox.put(operation.id, jsonEncode(operation.toJson()));
    await _dataBox.put('version_vector', jsonEncode(newVersionVector));

    // Update state
    if (priority == SyncPriority.critical) {
      state = state.copyWith(
        priorityOperations: [...state.priorityOperations, operation],
        localVersionVector: newVersionVector,
      );

      // Trigger immediate sync for critical operations
      _syncImmediately();
    } else {
      state = state.copyWith(
        pendingOperations: [...state.pendingOperations, operation],
        localVersionVector: newVersionVector,
      );
    }
  }

  /// Start adaptive sync based on connection quality
  void _startAdaptiveSync() {
    _periodicSyncTimer?.cancel();

    final syncInterval = _calculateAdaptiveSyncInterval();
    _periodicSyncTimer = Timer.periodic(
      Duration(seconds: syncInterval),
      (_) => _syncPeriodically(),
    );
  }

  /// Calculate sync interval based on connection quality
  int _calculateAdaptiveSyncInterval() {
    const baseInterval = 30; // seconds

    // Adjust based on connection quality
    final qualityMultiplier =
        state.connectionQuality.stabilityScore > 0.8
            ? 1.0
            : state.connectionQuality.stabilityScore > 0.5
                ? 1.5
                : 2.0;

    // Adjust based on bandwidth
    final bandwidthMultiplier =
        state.connectionQuality.bandwidthMbps > 5.0
            ? 1.0
            : state.connectionQuality.bandwidthMbps > 1.0
                ? 1.2
                : 1.5;

    return (baseInterval * qualityMultiplier * bandwidthMultiplier).toInt();
  }

  /// Start connection quality monitoring
  void _startConnectionMonitoring() {
    // Monitor connectivity changes
    _connectivitySubscription = Connectivity()
        .onConnectivityChanged
        .listen((ConnectivityResult result) {
      if (result != ConnectivityResult.none) {
        _assessConnectionQuality();
        _syncImmediately();
      }
    });

    // Periodic quality assessment
    _qualityMonitorTimer?.cancel();
    _qualityMonitorTimer = Timer.periodic(
      const Duration(seconds: 30),
      (_) => _assessConnectionQuality(),
    );
  }

  /// Assess connection quality
  Future<void> _assessConnectionQuality() async {
    try {
      final startTime = DateTime.now();

      // Ping backend to measure latency
      await _dio.get('/api/health/ping');

      final latency = DateTime.now().difference(startTime).inMilliseconds;

      // Estimate bandwidth (simplified)
      final bandwidth = latency < 50
          ? 10.0
          : latency < 100
              ? 5.0
              : latency < 200
                  ? 2.0
                  : 1.0;

      // Calculate stability score based on recent performance
      final stability = latency < 100 ? 0.9 : latency < 200 ? 0.7 : 0.5;

      final quality = ConnectionQuality(
        bandwidthMbps: bandwidth,
        latencyMs: latency,
        stabilityScore: stability,
        lastAssessed: DateTime.now(),
      );

      state = state.copyWith(connectionQuality: quality);

      // Save quality assessment
      await _dataBox.put(
        'connection_quality',
        jsonEncode(quality.toJson()),
      );

      // Restart adaptive sync with new interval
      _startAdaptiveSync();
    } catch (e) {
      // Connection assessment failed, assume poor quality
      final quality = ConnectionQuality(
        bandwidthMbps: 1.0,
        latencyMs: 500,
        stabilityScore: 0.3,
        lastAssessed: DateTime.now(),
      );

      state = state.copyWith(connectionQuality: quality);
    }
  }

  /// Sync immediately (for critical operations)
  Future<void> _syncImmediately() async {
    if (state.isSyncing) return;

    await _performSync(priorityOnly: true);
  }

  /// Periodic sync
  Future<void> _syncPeriodically() async {
    if (state.isSyncing) return;

    await _performSync(priorityOnly: false);
  }

  /// Perform synchronization
  Future<void> _performSync({required bool priorityOnly}) async {
    if (state.isSyncing) return;

    state = state.copyWith(isSyncing: true, syncError: null);

    try {
      final operations = priorityOnly
          ? state.priorityOperations
          : [...state.priorityOperations, ...state.pendingOperations];

      if (operations.isEmpty) {
        state = state.copyWith(isSyncing: false);
        return;
      }

      // Sort by priority and timestamp
      operations.sort((a, b) {
        final priorityCompare = a.priority.index.compareTo(b.priority.index);
        if (priorityCompare != 0) return priorityCompare;
        return a.createdAt.compareTo(b.createdAt);
      });

      final processed = <String>[];
      int syncedCount = 0;
      int conflictsResolved = 0;

      for (var operation in operations) {
        try {
          // Send operation to backend
          final response = await _dio.post(
            '/api/state/sync',
            data: {
              'device_id': deviceId,
              'operation': operation.toJson(),
              'version_vector': state.localVersionVector,
            },
          );

          // Process response
          if (response.statusCode == 200) {
            final result = response.data;

            // Update version vector from server
            if (result['version_vector'] != null) {
              final serverVector =
                  Map<String, int>.from(result['version_vector']);
              await _mergeVersionVectors(serverVector);
            }

            // Check for conflicts
            if (result['conflicts_resolved'] != null) {
              conflictsResolved += result['conflicts_resolved'] as int;
            }

            processed.add(operation.id);
            syncedCount++;

            // Remove from storage
            await _syncBox.delete(operation.id);
          }
        } catch (e) {
          // Operation failed, increment retry count
          final updated = operation.copyWith(
            retryCount: operation.retryCount + 1,
          );

          // Remove if max retries exceeded
          if (updated.retryCount >= 3) {
            processed.add(operation.id);
            await _syncBox.delete(operation.id);
          } else {
            // Update retry count in storage
            await _syncBox.put(updated.id, jsonEncode(updated.toJson()));
          }
        }
      }

      // Update state
      final remainingPriority = state.priorityOperations
          .where((op) => !processed.contains(op.id))
          .toList();

      final remainingPending = state.pendingOperations
          .where((op) => !processed.contains(op.id))
          .toList();

      // Save last sync time
      final now = DateTime.now();
      await _dataBox.put('last_sync_time', now.toIso8601String());

      state = state.copyWith(
        priorityOperations: remainingPriority,
        pendingOperations: remainingPending,
        isSyncing: false,
        lastSyncTime: now,
        syncedItemsCount: state.syncedItemsCount + syncedCount,
        conflictsResolvedCount:
            state.conflictsResolvedCount + conflictsResolved,
      );
    } catch (e) {
      state = state.copyWith(
        isSyncing: false,
        syncError: e.toString(),
      );
    }
  }

  /// Merge version vectors (CRDT operation)
  Future<void> _mergeVersionVectors(Map<String, int> serverVector) async {
    final merged = Map<String, int>.from(state.localVersionVector);

    for (var entry in serverVector.entries) {
      final localVersion = merged[entry.key] ?? 0;
      merged[entry.key] = localVersion > entry.value ? localVersion : entry.value;
    }

    await _dataBox.put('version_vector', jsonEncode(merged));
    state = state.copyWith(localVersionVector: merged);
  }

  /// Force full sync
  Future<void> forceFullSync() async {
    await queueOperation(
      SyncOperationType.fullSync,
      {'timestamp': DateTime.now().toIso8601String()},
      SyncPriority.high,
    );

    await _performSync(priorityOnly: false);
  }

  /// Clear all pending operations
  Future<void> clearPending() async {
    await _syncBox.clear();
    state = state.copyWith(
      pendingOperations: [],
      priorityOperations: [],
    );
  }

  /// Get sync statistics
  Map<String, dynamic> getSyncStats() {
    return {
      'device_id': deviceId,
      'last_sync': state.lastSyncTime?.toIso8601String(),
      'pending_operations': state.totalPendingCount,
      'synced_items': state.syncedItemsCount,
      'conflicts_resolved': state.conflictsResolvedCount,
      'connection_quality': state.connectionQuality.toJson(),
      'is_syncing': state.isSyncing,
      'sync_error': state.syncError,
    };
  }

  @override
  void dispose() {
    _periodicSyncTimer?.cancel();
    _qualityMonitorTimer?.cancel();
    _connectivitySubscription?.cancel();
    super.dispose();
  }
}

/// CRDT Sync Box Provider
final crdtSyncBoxProvider = Provider<Box>((ref) {
  return Hive.box('crdt_sync');
});

/// CRDT Data Box Provider
final crdtDataBoxProvider = Provider<Box>((ref) {
  return Hive.box('crdt_data');
});

/// Device ID Provider
final deviceIdProvider = Provider<String>((ref) {
  // In production, this should be a unique device identifier
  return 'device_${DateTime.now().millisecondsSinceEpoch}';
});

/// CRDT Sync Manager Provider
final crdtSyncManagerProvider =
    StateNotifierProvider<CRDTSyncManager, CRDTSyncState>((ref) {
  final syncBox = ref.watch(crdtSyncBoxProvider);
  final dataBox = ref.watch(crdtDataBoxProvider);
  final dio = Dio(); // In production, use configured Dio instance
  final deviceId = ref.watch(deviceIdProvider);

  return CRDTSyncManager(syncBox, dataBox, dio, deviceId);
});

/// Initialize CRDT sync storage
Future<void> initializeCRDTSync() async {
  await Hive.openBox('crdt_sync');
  await Hive.openBox('crdt_data');
}

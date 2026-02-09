import 'package:dio/dio.dart';
import 'dart:convert';
import 'conflict_resolution_service.dart';

/// Personal data types that can be synchronized
enum PersonalDataType {
  bookmarks,
  readingProgress,
  personalNotes,
  userPreferences,
  khatmaProgress,
}

/// Sync status for personal data
class PersonalDataSyncStatus {
  final PersonalDataType dataType;
  final bool isSynced;
  final DateTime? lastSyncTime;
  final int pendingChanges;
  final String? error;

  const PersonalDataSyncStatus({
    required this.dataType,
    required this.isSynced,
    this.lastSyncTime,
    this.pendingChanges = 0,
    this.error,
  });
}

/// Service for synchronizing personal data with backend
class PersonalDataSyncService {
  final Dio _dio;
  final String deviceId;
  final ConflictResolutionService _conflictResolver;

  PersonalDataSyncService({
    required Dio dio,
    required this.deviceId,
  })  : _dio = dio,
        _conflictResolver = ConflictResolutionService();

  /// Sync bookmarks with backend
  Future<Map<String, dynamic>> syncBookmarks({
    required Map<String, dynamic> localBookmarks,
    required Map<String, int> versionVector,
  }) async {
    try {
      final response = await _dio.post(
        '/api/state/sync/bookmarks',
        data: {
          'device_id': deviceId,
          'bookmarks': localBookmarks,
          'version_vector': versionVector,
        },
      );

      if (response.statusCode == 200) {
        final serverData = response.data;

        // Check for conflicts
        if (serverData['has_conflicts'] == true) {
          // Resolve conflicts using set union strategy
          final resolved = _conflictResolver.resolveConflict(
            dataType: 'bookmarks',
            localData: localBookmarks,
            remoteData: serverData['bookmarks'],
            strategy: ConflictResolutionStrategy.setUnion,
          );

          return {
            'bookmarks': resolved.resolvedData,
            'version_vector': serverData['version_vector'],
            'conflicts_resolved': resolved.hadConflict ? 1 : 0,
          };
        }

        return {
          'bookmarks': serverData['bookmarks'],
          'version_vector': serverData['version_vector'],
          'conflicts_resolved': 0,
        };
      }

      throw Exception('Sync failed: ${response.statusCode}');
    } catch (e) {
      throw Exception('Bookmark sync error: $e');
    }
  }

  /// Sync reading progress with backend
  Future<Map<String, dynamic>> syncReadingProgress({
    required Map<String, dynamic> localProgress,
    required Map<String, int> versionVector,
  }) async {
    try {
      final response = await _dio.post(
        '/api/state/sync/reading-progress',
        data: {
          'device_id': deviceId,
          'reading_progress': localProgress,
          'version_vector': versionVector,
        },
      );

      if (response.statusCode == 200) {
        final serverData = response.data;

        // Check for conflicts
        if (serverData['has_conflicts'] == true) {
          // Resolve conflicts using max value strategy
          final resolved = _conflictResolver.resolveConflict(
            dataType: 'reading_progress',
            localData: localProgress,
            remoteData: serverData['reading_progress'],
            strategy: ConflictResolutionStrategy.maxValue,
          );

          return {
            'reading_progress': resolved.resolvedData,
            'version_vector': serverData['version_vector'],
            'conflicts_resolved': resolved.hadConflict ? 1 : 0,
          };
        }

        return {
          'reading_progress': serverData['reading_progress'],
          'version_vector': serverData['version_vector'],
          'conflicts_resolved': 0,
        };
      }

      throw Exception('Sync failed: ${response.statusCode}');
    } catch (e) {
      throw Exception('Reading progress sync error: $e');
    }
  }

  /// Sync personal notes with backend
  Future<Map<String, dynamic>> syncPersonalNotes({
    required Map<String, dynamic> localNotes,
    required Map<String, int> versionVector,
  }) async {
    try {
      final response = await _dio.post(
        '/api/state/sync/personal-notes',
        data: {
          'device_id': deviceId,
          'personal_notes': localNotes,
          'version_vector': versionVector,
        },
      );

      if (response.statusCode == 200) {
        final serverData = response.data;

        // Check for conflicts
        if (serverData['has_conflicts'] == true) {
          // Resolve conflicts using custom strategy
          final resolved = _conflictResolver.resolveConflict(
            dataType: 'personal_notes',
            localData: localNotes,
            remoteData: serverData['personal_notes'],
            strategy: ConflictResolutionStrategy.custom,
          );

          return {
            'personal_notes': resolved.resolvedData,
            'version_vector': serverData['version_vector'],
            'conflicts_resolved': resolved.hadConflict ? 1 : 0,
          };
        }

        return {
          'personal_notes': serverData['personal_notes'],
          'version_vector': serverData['version_vector'],
          'conflicts_resolved': 0,
        };
      }

      throw Exception('Sync failed: ${response.statusCode}');
    } catch (e) {
      throw Exception('Personal notes sync error: $e');
    }
  }

  /// Sync user preferences with backend
  Future<Map<String, dynamic>> syncUserPreferences({
    required Map<String, dynamic> localPreferences,
    required Map<String, int> versionVector,
  }) async {
    try {
      final response = await _dio.post(
        '/api/state/sync/preferences',
        data: {
          'device_id': deviceId,
          'preferences': localPreferences,
          'version_vector': versionVector,
        },
      );

      if (response.statusCode == 200) {
        final serverData = response.data;

        // Check for conflicts
        if (serverData['has_conflicts'] == true) {
          // Resolve conflicts using last-write-wins strategy
          final resolved = _conflictResolver.resolveConflict(
            dataType: 'user_preferences',
            localData: localPreferences,
            remoteData: serverData['preferences'],
            strategy: ConflictResolutionStrategy.lastWriteWins,
          );

          return {
            'preferences': resolved.resolvedData,
            'version_vector': serverData['version_vector'],
            'conflicts_resolved': resolved.hadConflict ? 1 : 0,
          };
        }

        return {
          'preferences': serverData['preferences'],
          'version_vector': serverData['version_vector'],
          'conflicts_resolved': 0,
        };
      }

      throw Exception('Sync failed: ${response.statusCode}');
    } catch (e) {
      throw Exception('Preferences sync error: $e');
    }
  }

  /// Sync khatma progress with backend
  Future<Map<String, dynamic>> syncKhatmaProgress({
    required Map<String, dynamic> localKhatmaProgress,
    required Map<String, int> versionVector,
  }) async {
    try {
      final response = await _dio.post(
        '/api/state/sync/khatma-progress',
        data: {
          'device_id': deviceId,
          'khatma_progress': localKhatmaProgress,
          'version_vector': versionVector,
        },
      );

      if (response.statusCode == 200) {
        final serverData = response.data;

        // Check for conflicts
        if (serverData['has_conflicts'] == true) {
          // Resolve conflicts using custom khatma strategy
          final resolved = _conflictResolver.resolveConflict(
            dataType: 'khatma_plan',
            localData: localKhatmaProgress,
            remoteData: serverData['khatma_progress'],
            strategy: ConflictResolutionStrategy.custom,
          );

          return {
            'khatma_progress': resolved.resolvedData,
            'version_vector': serverData['version_vector'],
            'conflicts_resolved': resolved.hadConflict ? 1 : 0,
          };
        }

        return {
          'khatma_progress': serverData['khatma_progress'],
          'version_vector': serverData['version_vector'],
          'conflicts_resolved': 0,
        };
      }

      throw Exception('Sync failed: ${response.statusCode}');
    } catch (e) {
      throw Exception('Khatma progress sync error: $e');
    }
  }

  /// Perform full sync of all personal data
  Future<Map<String, dynamic>> performFullSync({
    required Map<String, dynamic> allLocalData,
    required Map<String, int> versionVector,
  }) async {
    try {
      final response = await _dio.post(
        '/api/state/sync/full',
        data: {
          'device_id': deviceId,
          'user_data': allLocalData,
          'version_vector': versionVector,
        },
      );

      if (response.statusCode == 200) {
        final serverData = response.data;
        final mergedData = <String, dynamic>{};
        int totalConflicts = 0;

        // Merge each data type
        for (var dataType in PersonalDataType.values) {
          final typeKey = _getDataTypeKey(dataType);

          if (allLocalData.containsKey(typeKey) &&
              serverData['user_data'].containsKey(typeKey)) {
            final resolved = _conflictResolver.resolveConflict(
              dataType: typeKey,
              localData: allLocalData[typeKey],
              remoteData: serverData['user_data'][typeKey],
            );

            mergedData[typeKey] = resolved.resolvedData;
            if (resolved.hadConflict) {
              totalConflicts++;
            }
          } else if (serverData['user_data'].containsKey(typeKey)) {
            mergedData[typeKey] = serverData['user_data'][typeKey];
          } else if (allLocalData.containsKey(typeKey)) {
            mergedData[typeKey] = allLocalData[typeKey];
          }
        }

        return {
          'user_data': mergedData,
          'version_vector': serverData['version_vector'],
          'conflicts_resolved': totalConflicts,
          'sync_time': DateTime.now().toIso8601String(),
        };
      }

      throw Exception('Full sync failed: ${response.statusCode}');
    } catch (e) {
      throw Exception('Full sync error: $e');
    }
  }

  /// Get sync status for all personal data types
  Future<List<PersonalDataSyncStatus>> getSyncStatus() async {
    try {
      final response = await _dio.get(
        '/api/state/sync/status',
        queryParameters: {'device_id': deviceId},
      );

      if (response.statusCode == 200) {
        final statusData = response.data as List;

        return statusData.map((item) {
          return PersonalDataSyncStatus(
            dataType: _parseDataType(item['data_type']),
            isSynced: item['is_synced'],
            lastSyncTime: item['last_sync_time'] != null
                ? DateTime.parse(item['last_sync_time'])
                : null,
            pendingChanges: item['pending_changes'] ?? 0,
          );
        }).toList();
      }

      throw Exception('Failed to get sync status');
    } catch (e) {
      // Return default status on error
      return PersonalDataType.values
          .map((type) => PersonalDataSyncStatus(
                dataType: type,
                isSynced: false,
                error: e.toString(),
              ))
          .toList();
    }
  }

  /// Get data type key for API
  String _getDataTypeKey(PersonalDataType type) {
    switch (type) {
      case PersonalDataType.bookmarks:
        return 'bookmarks';
      case PersonalDataType.readingProgress:
        return 'reading_progress';
      case PersonalDataType.personalNotes:
        return 'personal_notes';
      case PersonalDataType.userPreferences:
        return 'user_preferences';
      case PersonalDataType.khatmaProgress:
        return 'khatma_progress';
    }
  }

  /// Parse data type from string
  PersonalDataType _parseDataType(String typeStr) {
    switch (typeStr) {
      case 'bookmarks':
        return PersonalDataType.bookmarks;
      case 'reading_progress':
        return PersonalDataType.readingProgress;
      case 'personal_notes':
        return PersonalDataType.personalNotes;
      case 'user_preferences':
        return PersonalDataType.userPreferences;
      case 'khatma_progress':
        return PersonalDataType.khatmaProgress;
      default:
        return PersonalDataType.bookmarks;
    }
  }

  /// Check if data needs sync
  bool needsSync(DateTime? lastSyncTime, int pendingChanges) {
    if (pendingChanges > 0) return true;

    if (lastSyncTime == null) return true;

    // Sync if more than 5 minutes since last sync
    return DateTime.now().difference(lastSyncTime).inMinutes > 5;
  }
}

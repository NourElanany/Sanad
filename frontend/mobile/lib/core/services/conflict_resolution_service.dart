import 'dart:convert';

/// Conflict resolution strategies for CRDT synchronization
enum ConflictResolutionStrategy {
  lastWriteWins, // For user preferences, display settings
  setUnion, // For bookmarks, favorite surahs
  maxValue, // For reading progress, khatma completion
  custom, // For complex data requiring custom logic
}

/// Conflict resolution result
class ConflictResolutionResult<T> {
  final T resolvedData;
  final bool hadConflict;
  final String? resolutionStrategy;
  final Map<String, dynamic>? metadata;

  const ConflictResolutionResult({
    required this.resolvedData,
    required this.hadConflict,
    this.resolutionStrategy,
    this.metadata,
  });
}

/// Service for resolving data conflicts using CRDT strategies
class ConflictResolutionService {
  /// Resolve conflict between local and remote data
  ConflictResolutionResult<Map<String, dynamic>> resolveConflict({
    required String dataType,
    required Map<String, dynamic> localData,
    required Map<String, dynamic> remoteData,
    ConflictResolutionStrategy? strategy,
  }) {
    // Determine strategy based on data type if not provided
    final resolveStrategy = strategy ?? _getStrategyForDataType(dataType);

    switch (resolveStrategy) {
      case ConflictResolutionStrategy.lastWriteWins:
        return _resolveLastWriteWins(localData, remoteData);

      case ConflictResolutionStrategy.setUnion:
        return _resolveSetUnion(localData, remoteData);

      case ConflictResolutionStrategy.maxValue:
        return _resolveMaxValue(localData, remoteData);

      case ConflictResolutionStrategy.custom:
        return _resolveCustom(dataType, localData, remoteData);
    }
  }

  /// Get appropriate strategy for data type
  ConflictResolutionStrategy _getStrategyForDataType(String dataType) {
    switch (dataType) {
      case 'user_preferences':
      case 'display_settings':
      case 'notification_settings':
        return ConflictResolutionStrategy.lastWriteWins;

      case 'bookmarks':
      case 'favorites':
      case 'tags':
        return ConflictResolutionStrategy.setUnion;

      case 'reading_progress':
      case 'khatma_progress':
      case 'completion_percentage':
        return ConflictResolutionStrategy.maxValue;

      default:
        return ConflictResolutionStrategy.custom;
    }
  }

  /// Last-Write-Wins resolution
  /// Compares timestamps and keeps the most recent version
  ConflictResolutionResult<Map<String, dynamic>> _resolveLastWriteWins(
    Map<String, dynamic> localData,
    Map<String, dynamic> remoteData,
  ) {
    final localTimestamp = _extractTimestamp(localData);
    final remoteTimestamp = _extractTimestamp(remoteData);

    final hadConflict = localTimestamp != remoteTimestamp;

    // If timestamps are equal, use device_id as tiebreaker
    if (localTimestamp == remoteTimestamp) {
      final localDeviceId = localData['device_id'] as String? ?? '';
      final remoteDeviceId = remoteData['device_id'] as String? ?? '';

      final resolved = localDeviceId.compareTo(remoteDeviceId) > 0
          ? localData
          : remoteData;

      return ConflictResolutionResult(
        resolvedData: resolved,
        hadConflict: hadConflict,
        resolutionStrategy: 'last_write_wins_device_tiebreak',
        metadata: {
          'local_timestamp': localTimestamp.toIso8601String(),
          'remote_timestamp': remoteTimestamp.toIso8601String(),
          'winner': resolved == localData ? 'local' : 'remote',
        },
      );
    }

    final resolved =
        localTimestamp.isAfter(remoteTimestamp) ? localData : remoteData;

    return ConflictResolutionResult(
      resolvedData: resolved,
      hadConflict: hadConflict,
      resolutionStrategy: 'last_write_wins',
      metadata: {
        'local_timestamp': localTimestamp.toIso8601String(),
        'remote_timestamp': remoteTimestamp.toIso8601String(),
        'winner': resolved == localData ? 'local' : 'remote',
      },
    );
  }

  /// Set Union resolution
  /// Merges two sets by taking the union of all elements
  ConflictResolutionResult<Map<String, dynamic>> _resolveSetUnion(
    Map<String, dynamic> localData,
    Map<String, dynamic> remoteData,
  ) {
    final merged = Map<String, dynamic>.from(localData);
    bool hadConflict = false;

    // Merge all keys from remote data
    for (var entry in remoteData.entries) {
      if (entry.value is List) {
        // Merge lists (sets)
        final localList = (merged[entry.key] as List?) ?? [];
        final remoteList = entry.value as List;

        // Create union of both lists
        final union = <dynamic>{...localList, ...remoteList}.toList();

        if (union.length != localList.length) {
          hadConflict = true;
        }

        merged[entry.key] = union;
      } else if (entry.value is Map) {
        // Recursively merge maps
        final localMap =
            (merged[entry.key] as Map<String, dynamic>?) ?? {};
        final remoteMap = entry.value as Map<String, dynamic>;

        final result = _resolveSetUnion(localMap, remoteMap);
        merged[entry.key] = result.resolvedData;

        if (result.hadConflict) {
          hadConflict = true;
        }
      } else {
        // For primitive values, check if they differ
        if (!merged.containsKey(entry.key) ||
            merged[entry.key] != entry.value) {
          hadConflict = true;
          // Use last-write-wins for primitive values
          final localTimestamp = _extractTimestamp(localData);
          final remoteTimestamp = _extractTimestamp(remoteData);

          if (remoteTimestamp.isAfter(localTimestamp)) {
            merged[entry.key] = entry.value;
          }
        }
      }
    }

    return ConflictResolutionResult(
      resolvedData: merged,
      hadConflict: hadConflict,
      resolutionStrategy: 'set_union',
      metadata: {
        'local_items': localData.length,
        'remote_items': remoteData.length,
        'merged_items': merged.length,
      },
    );
  }

  /// Max Value resolution
  /// Takes the maximum value for numeric fields
  ConflictResolutionResult<Map<String, dynamic>> _resolveMaxValue(
    Map<String, dynamic> localData,
    Map<String, dynamic> remoteData,
  ) {
    final merged = Map<String, dynamic>.from(localData);
    bool hadConflict = false;

    for (var entry in remoteData.entries) {
      if (entry.value is num) {
        final localValue = (merged[entry.key] as num?) ?? 0;
        final remoteValue = entry.value as num;

        if (remoteValue > localValue) {
          merged[entry.key] = remoteValue;
          hadConflict = true;
        }
      } else if (entry.value is Map) {
        // Recursively resolve nested maps
        final localMap =
            (merged[entry.key] as Map<String, dynamic>?) ?? {};
        final remoteMap = entry.value as Map<String, dynamic>;

        final result = _resolveMaxValue(localMap, remoteMap);
        merged[entry.key] = result.resolvedData;

        if (result.hadConflict) {
          hadConflict = true;
        }
      } else {
        // For non-numeric values, use last-write-wins
        if (!merged.containsKey(entry.key)) {
          merged[entry.key] = entry.value;
        } else if (merged[entry.key] != entry.value) {
          hadConflict = true;
          final localTimestamp = _extractTimestamp(localData);
          final remoteTimestamp = _extractTimestamp(remoteData);

          if (remoteTimestamp.isAfter(localTimestamp)) {
            merged[entry.key] = entry.value;
          }
        }
      }
    }

    return ConflictResolutionResult(
      resolvedData: merged,
      hadConflict: hadConflict,
      resolutionStrategy: 'max_value',
      metadata: {
        'fields_compared': remoteData.length,
        'conflicts_found': hadConflict ? 1 : 0,
      },
    );
  }

  /// Custom resolution for complex data types
  ConflictResolutionResult<Map<String, dynamic>> _resolveCustom(
    String dataType,
    Map<String, dynamic> localData,
    Map<String, dynamic> remoteData,
  ) {
    switch (dataType) {
      case 'reading_progress_detailed':
        return _resolveReadingProgress(localData, remoteData);

      case 'khatma_plan':
        return _resolveKhatmaPlan(localData, remoteData);

      case 'personal_notes':
        return _resolvePersonalNotes(localData, remoteData);

      default:
        // Fallback to last-write-wins
        return _resolveLastWriteWins(localData, remoteData);
    }
  }

  /// Custom resolution for reading progress
  /// Takes the furthest progress for each surah
  ConflictResolutionResult<Map<String, dynamic>> _resolveReadingProgress(
    Map<String, dynamic> localData,
    Map<String, dynamic> remoteData,
  ) {
    final merged = Map<String, dynamic>.from(localData);
    bool hadConflict = false;

    // Merge surah progress
    final localSurahs =
        (localData['quran_progress'] as Map<String, dynamic>?) ?? {};
    final remoteSurahs =
        (remoteData['quran_progress'] as Map<String, dynamic>?) ?? {};

    final mergedSurahs = Map<String, dynamic>.from(localSurahs);

    for (var entry in remoteSurahs.entries) {
      final surahNumber = entry.key;
      final remoteProgress = entry.value as Map<String, dynamic>;

      if (!mergedSurahs.containsKey(surahNumber)) {
        mergedSurahs[surahNumber] = remoteProgress;
        hadConflict = true;
      } else {
        final localProgress =
            mergedSurahs[surahNumber] as Map<String, dynamic>;

        // Compare last_ayah_read
        final localAyah = localProgress['last_ayah_read'] as int? ?? 0;
        final remoteAyah = remoteProgress['last_ayah_read'] as int? ?? 0;

        if (remoteAyah > localAyah) {
          mergedSurahs[surahNumber] = remoteProgress;
          hadConflict = true;
        } else if (remoteAyah == localAyah) {
          // If same ayah, use latest timestamp
          final localTime = DateTime.parse(
              localProgress['last_read_at'] as String);
          final remoteTime = DateTime.parse(
              remoteProgress['last_read_at'] as String);

          if (remoteTime.isAfter(localTime)) {
            mergedSurahs[surahNumber] = remoteProgress;
            hadConflict = true;
          }
        }
      }
    }

    merged['quran_progress'] = mergedSurahs;

    return ConflictResolutionResult(
      resolvedData: merged,
      hadConflict: hadConflict,
      resolutionStrategy: 'reading_progress_max',
      metadata: {
        'local_surahs': localSurahs.length,
        'remote_surahs': remoteSurahs.length,
        'merged_surahs': mergedSurahs.length,
      },
    );
  }

  /// Custom resolution for khatma plans
  ConflictResolutionResult<Map<String, dynamic>> _resolveKhatmaPlan(
    Map<String, dynamic> localData,
    Map<String, dynamic> remoteData,
  ) {
    // Take the plan with more completed portions
    final localCompleted = localData['completed_portions'] as int? ?? 0;
    final remoteCompleted = remoteData['completed_portions'] as int? ?? 0;

    final hadConflict = localCompleted != remoteCompleted;

    if (remoteCompleted > localCompleted) {
      return ConflictResolutionResult(
        resolvedData: remoteData,
        hadConflict: hadConflict,
        resolutionStrategy: 'khatma_max_completion',
        metadata: {
          'local_completed': localCompleted,
          'remote_completed': remoteCompleted,
          'winner': 'remote',
        },
      );
    } else if (localCompleted > remoteCompleted) {
      return ConflictResolutionResult(
        resolvedData: localData,
        hadConflict: hadConflict,
        resolutionStrategy: 'khatma_max_completion',
        metadata: {
          'local_completed': localCompleted,
          'remote_completed': remoteCompleted,
          'winner': 'local',
        },
      );
    } else {
      // Same completion, use last-write-wins
      return _resolveLastWriteWins(localData, remoteData);
    }
  }

  /// Custom resolution for personal notes
  ConflictResolutionResult<Map<String, dynamic>> _resolvePersonalNotes(
    Map<String, dynamic> localData,
    Map<String, dynamic> remoteData,
  ) {
    final merged = Map<String, dynamic>.from(localData);
    bool hadConflict = false;

    final localNotes = (localData['notes'] as Map<String, dynamic>?) ?? {};
    final remoteNotes = (remoteData['notes'] as Map<String, dynamic>?) ?? {};

    final mergedNotes = Map<String, dynamic>.from(localNotes);

    for (var entry in remoteNotes.entries) {
      final noteId = entry.key;
      final remoteNote = entry.value as Map<String, dynamic>;

      if (!mergedNotes.containsKey(noteId)) {
        mergedNotes[noteId] = remoteNote;
        hadConflict = true;
      } else {
        final localNote = mergedNotes[noteId] as Map<String, dynamic>;

        // Compare updated_at timestamps
        final localTime =
            DateTime.parse(localNote['updated_at'] as String);
        final remoteTime =
            DateTime.parse(remoteNote['updated_at'] as String);

        if (remoteTime.isAfter(localTime)) {
          mergedNotes[noteId] = remoteNote;
          hadConflict = true;
        }
      }
    }

    merged['notes'] = mergedNotes;

    return ConflictResolutionResult(
      resolvedData: merged,
      hadConflict: hadConflict,
      resolutionStrategy: 'notes_last_write_wins',
      metadata: {
        'local_notes': localNotes.length,
        'remote_notes': remoteNotes.length,
        'merged_notes': mergedNotes.length,
      },
    );
  }

  /// Extract timestamp from data
  DateTime _extractTimestamp(Map<String, dynamic> data) {
    // Try common timestamp fields
    final timestampFields = [
      'updated_at',
      'last_modified',
      'timestamp',
      'last_read_at',
      'created_at',
    ];

    for (var field in timestampFields) {
      if (data.containsKey(field)) {
        try {
          return DateTime.parse(data[field] as String);
        } catch (e) {
          // Invalid timestamp, try next field
          continue;
        }
      }
    }

    // No timestamp found, use epoch
    return DateTime.fromMillisecondsSinceEpoch(0);
  }

  /// Merge version vectors (CRDT operation)
  Map<String, int> mergeVersionVectors(
    Map<String, int> local,
    Map<String, int> remote,
  ) {
    final merged = Map<String, int>.from(local);

    for (var entry in remote.entries) {
      final localVersion = merged[entry.key] ?? 0;
      merged[entry.key] =
          localVersion > entry.value ? localVersion : entry.value;
    }

    return merged;
  }

  /// Check if version vectors are concurrent (conflicting)
  bool areVersionsConcurrent(
    Map<String, int> local,
    Map<String, int> remote,
  ) {
    bool localDominates = true;
    bool remoteDominates = true;

    // Check all devices in local vector
    for (var entry in local.entries) {
      final remoteVersion = remote[entry.key] ?? 0;
      if (entry.value < remoteVersion) {
        localDominates = false;
      }
    }

    // Check all devices in remote vector
    for (var entry in remote.entries) {
      final localVersion = local[entry.key] ?? 0;
      if (entry.value < localVersion) {
        remoteDominates = false;
      }
    }

    // Concurrent if neither dominates
    return !localDominates && !remoteDominates;
  }
}

import 'dart:convert';
import 'dart:io';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:path_provider/path_provider.dart';
import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';

/// Backup metadata
class BackupMetadata {
  final String backupId;
  final DateTime createdAt;
  final String deviceId;
  final String appVersion;
  final int dataSize;
  final String checksum;
  final Map<String, int> versionVector;

  BackupMetadata({
    required this.backupId,
    required this.createdAt,
    required this.deviceId,
    required this.appVersion,
    required this.dataSize,
    required this.checksum,
    required this.versionVector,
  });

  Map<String, dynamic> toJson() => {
        'backup_id': backupId,
        'created_at': createdAt.toIso8601String(),
        'device_id': deviceId,
        'app_version': appVersion,
        'data_size': dataSize,
        'checksum': checksum,
        'version_vector': versionVector,
      };

  factory BackupMetadata.fromJson(Map<String, dynamic> json) {
    return BackupMetadata(
      backupId: json['backup_id'],
      createdAt: DateTime.parse(json['created_at']),
      deviceId: json['device_id'],
      appVersion: json['app_version'],
      dataSize: json['data_size'],
      checksum: json['checksum'],
      versionVector: Map<String, int>.from(json['version_vector']),
    );
  }
}

/// Backup result
class BackupResult {
  final bool success;
  final String? backupPath;
  final BackupMetadata? metadata;
  final String? error;

  const BackupResult({
    required this.success,
    this.backupPath,
    this.metadata,
    this.error,
  });
}

/// Restore result
class RestoreResult {
  final bool success;
  final int itemsRestored;
  final BackupMetadata? metadata;
  final String? error;

  const RestoreResult({
    required this.success,
    required this.itemsRestored,
    this.metadata,
    this.error,
  });
}

/// Service for backing up and restoring user data
class BackupRestoreService {
  final String deviceId;
  final String appVersion;

  BackupRestoreService({
    required this.deviceId,
    required this.appVersion,
  });

  /// Create a full backup of all user data
  Future<BackupResult> createBackup({
    bool includeCache = false,
    bool compress = true,
  }) async {
    try {
      // Collect all data to backup
      final backupData = await _collectBackupData(includeCache);

      // Create backup metadata
      final backupId =
          'backup_${DateTime.now().millisecondsSinceEpoch}';
      final dataJson = jsonEncode(backupData);
      final dataBytes = utf8.encode(dataJson);

      // Calculate checksum
      final checksum = sha256.convert(dataBytes).toString();

      // Get version vector
      final versionVector = await _getVersionVector();

      final metadata = BackupMetadata(
        backupId: backupId,
        createdAt: DateTime.now(),
        deviceId: deviceId,
        appVersion: appVersion,
        dataSize: dataBytes.length,
        checksum: checksum,
        versionVector: versionVector,
      );

      // Prepare backup package
      final backupPackage = {
        'metadata': metadata.toJson(),
        'data': backupData,
      };

      final packageJson = jsonEncode(backupPackage);
      var packageBytes = utf8.encode(packageJson);

      // Compress if requested
      if (compress) {
        packageBytes = _compressData(packageBytes);
      }

      // Save to file
      final backupPath = await _saveBackupToFile(
        backupId,
        packageBytes,
        compress,
      );

      return BackupResult(
        success: true,
        backupPath: backupPath,
        metadata: metadata,
      );
    } catch (e) {
      return BackupResult(
        success: false,
        error: 'Backup failed: $e',
      );
    }
  }

  /// Restore data from a backup file
  Future<RestoreResult> restoreFromBackup(
    String backupPath, {
    bool verifyChecksum = true,
    bool mergeWithExisting = true,
  }) async {
    try {
      // Read backup file
      final file = File(backupPath);
      if (!await file.exists()) {
        return RestoreResult(
          success: false,
          itemsRestored: 0,
          error: 'Backup file not found',
        );
      }

      var backupBytes = await file.readAsBytes();

      // Decompress if needed
      if (backupPath.endsWith('.gz')) {
        backupBytes = _decompressData(backupBytes);
      }

      // Parse backup package
      final packageJson = utf8.decode(backupBytes);
      final backupPackage = jsonDecode(packageJson) as Map<String, dynamic>;

      final metadata =
          BackupMetadata.fromJson(backupPackage['metadata']);
      final backupData =
          backupPackage['data'] as Map<String, dynamic>;

      // Verify checksum if requested
      if (verifyChecksum) {
        final dataJson = jsonEncode(backupData);
        final dataBytes = utf8.encode(dataJson);
        final checksum = sha256.convert(dataBytes).toString();

        if (checksum != metadata.checksum) {
          return RestoreResult(
            success: false,
            itemsRestored: 0,
            error: 'Checksum verification failed',
          );
        }
      }

      // Restore data
      final itemsRestored = await _restoreBackupData(
        backupData,
        mergeWithExisting,
      );

      return RestoreResult(
        success: true,
        itemsRestored: itemsRestored,
        metadata: metadata,
      );
    } catch (e) {
      return RestoreResult(
        success: false,
        itemsRestored: 0,
        error: 'Restore failed: $e',
      );
    }
  }

  /// Create automatic backup (scheduled)
  Future<BackupResult> createAutoBackup() async {
    // Check if auto backup is needed
    final lastBackup = await _getLastBackupTime();
    final now = DateTime.now();

    if (lastBackup != null &&
        now.difference(lastBackup).inHours < 24) {
      // Backup not needed yet
      return BackupResult(
        success: true,
        error: 'Auto backup not needed yet',
      );
    }

    // Create backup
    final result = await createBackup(
      includeCache: false,
      compress: true,
    );

    if (result.success) {
      await _saveLastBackupTime(now);

      // Clean up old backups (keep last 7)
      await _cleanupOldBackups(keepCount: 7);
    }

    return result;
  }

  /// List available backups
  Future<List<BackupMetadata>> listBackups() async {
    try {
      final backupDir = await _getBackupDirectory();
      final backups = <BackupMetadata>[];

      if (!await backupDir.exists()) {
        return backups;
      }

      final files = backupDir.listSync();
      for (var file in files) {
        if (file is File &&
            (file.path.endsWith('.backup') ||
                file.path.endsWith('.backup.gz'))) {
          try {
            var bytes = await file.readAsBytes();

            if (file.path.endsWith('.gz')) {
              bytes = _decompressData(bytes);
            }

            final json = utf8.decode(bytes);
            final package = jsonDecode(json) as Map<String, dynamic>;
            final metadata =
                BackupMetadata.fromJson(package['metadata']);

            backups.add(metadata);
          } catch (e) {
            // Invalid backup file, skip
            continue;
          }
        }
      }

      // Sort by creation date (newest first)
      backups.sort((a, b) => b.createdAt.compareTo(a.createdAt));

      return backups;
    } catch (e) {
      return [];
    }
  }

  /// Delete a backup
  Future<bool> deleteBackup(String backupId) async {
    try {
      final backupDir = await _getBackupDirectory();
      final files = backupDir.listSync();

      for (var file in files) {
        if (file is File && file.path.contains(backupId)) {
          await file.delete();
          return true;
        }
      }

      return false;
    } catch (e) {
      return false;
    }
  }

  /// Export backup to external storage
  Future<String?> exportBackup(String backupId) async {
    try {
      final backupDir = await _getBackupDirectory();
      final files = backupDir.listSync();

      for (var file in files) {
        if (file is File && file.path.contains(backupId)) {
          // Get external storage directory
          final externalDir = await getExternalStorageDirectory();
          if (externalDir == null) return null;

          final exportPath =
              '${externalDir.path}/sanad_backup_$backupId.backup.gz';
          await file.copy(exportPath);

          return exportPath;
        }
      }

      return null;
    } catch (e) {
      return null;
    }
  }

  /// Import backup from external storage
  Future<RestoreResult> importBackup(String externalPath) async {
    try {
      final file = File(externalPath);
      if (!await file.exists()) {
        return RestoreResult(
          success: false,
          itemsRestored: 0,
          error: 'Import file not found',
        );
      }

      // Copy to internal backup directory
      final backupDir = await _getBackupDirectory();
      final fileName = externalPath.split('/').last;
      final internalPath = '${backupDir.path}/$fileName';

      await file.copy(internalPath);

      // Restore from imported backup
      return await restoreFromBackup(internalPath);
    } catch (e) {
      return RestoreResult(
        success: false,
        itemsRestored: 0,
        error: 'Import failed: $e',
      );
    }
  }

  /// Collect all data for backup
  Future<Map<String, dynamic>> _collectBackupData(
      bool includeCache) async {
    final data = <String, dynamic>{};

    // Collect from all Hive boxes
    final boxNames = [
      'crdt_data',
      'crdt_sync',
      'offline_queue',
      'user_preferences',
      'bookmarks',
      'reading_progress',
      'personal_notes',
    ];

    if (includeCache) {
      boxNames.add('app_cache');
    }

    for (var boxName in boxNames) {
      try {
        final box = await Hive.openBox(boxName);
        final boxData = <String, dynamic>{};

        for (var key in box.keys) {
          boxData[key.toString()] = box.get(key);
        }

        data[boxName] = boxData;
      } catch (e) {
        // Box doesn't exist or error, skip
        continue;
      }
    }

    return data;
  }

  /// Restore data from backup
  Future<int> _restoreBackupData(
    Map<String, dynamic> backupData,
    bool mergeWithExisting,
  ) async {
    int itemsRestored = 0;

    for (var entry in backupData.entries) {
      final boxName = entry.key;
      final boxData = entry.value as Map<String, dynamic>;

      try {
        final box = await Hive.openBox(boxName);

        if (!mergeWithExisting) {
          // Clear existing data
          await box.clear();
        }

        // Restore data
        for (var dataEntry in boxData.entries) {
          await box.put(dataEntry.key, dataEntry.value);
          itemsRestored++;
        }
      } catch (e) {
        // Error restoring box, continue with others
        continue;
      }
    }

    return itemsRestored;
  }

  /// Get version vector from CRDT data
  Future<Map<String, int>> _getVersionVector() async {
    try {
      final box = await Hive.openBox('crdt_data');
      final versionVectorJson = box.get('version_vector');

      if (versionVectorJson != null) {
        return Map<String, int>.from(jsonDecode(versionVectorJson));
      }
    } catch (e) {
      // Error reading version vector
    }

    return {};
  }

  /// Save backup to file
  Future<String> _saveBackupToFile(
    String backupId,
    List<int> data,
    bool compressed,
  ) async {
    final backupDir = await _getBackupDirectory();

    if (!await backupDir.exists()) {
      await backupDir.create(recursive: true);
    }

    final extension = compressed ? '.backup.gz' : '.backup';
    final filePath = '${backupDir.path}/$backupId$extension';

    final file = File(filePath);
    await file.writeAsBytes(data);

    return filePath;
  }

  /// Get backup directory
  Future<Directory> _getBackupDirectory() async {
    final appDir = await getApplicationDocumentsDirectory();
    return Directory('${appDir.path}/backups');
  }

  /// Compress data using gzip
  List<int> _compressData(List<int> data) {
    return GZipEncoder().encode(data)!;
  }

  /// Decompress data using gzip
  List<int> _decompressData(List<int> data) {
    return GZipDecoder().decodeBytes(data);
  }

  /// Get last backup time
  Future<DateTime?> _getLastBackupTime() async {
    try {
      final box = await Hive.openBox('crdt_data');
      final lastBackupStr = box.get('last_backup_time');

      if (lastBackupStr != null) {
        return DateTime.parse(lastBackupStr);
      }
    } catch (e) {
      // Error reading last backup time
    }

    return null;
  }

  /// Save last backup time
  Future<void> _saveLastBackupTime(DateTime time) async {
    try {
      final box = await Hive.openBox('crdt_data');
      await box.put('last_backup_time', time.toIso8601String());
    } catch (e) {
      // Error saving last backup time
    }
  }

  /// Clean up old backups
  Future<void> _cleanupOldBackups({int keepCount = 7}) async {
    try {
      final backups = await listBackups();

      if (backups.length <= keepCount) {
        return;
      }

      // Delete oldest backups
      final toDelete = backups.skip(keepCount);
      for (var backup in toDelete) {
        await deleteBackup(backup.backupId);
      }
    } catch (e) {
      // Error cleaning up backups
    }
  }
}

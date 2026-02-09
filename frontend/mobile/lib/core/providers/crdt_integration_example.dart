import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:dio/dio.dart';
import 'crdt_sync_provider.dart';
import '../services/personal_data_sync_service.dart';
import '../services/backup_restore_service.dart';

/// Example integration of CRDT sync system
/// This file demonstrates how to use the CRDT synchronization system

/// Example: Bookmark Management with CRDT Sync
class BookmarkManager {
  final CRDTSyncManager syncManager;
  final PersonalDataSyncService syncService;

  BookmarkManager({
    required this.syncManager,
    required this.syncService,
  });

  /// Add a bookmark with automatic sync
  Future<void> addBookmark({
    required int surahNumber,
    required int ayahNumber,
    required int pageNumber,
  }) async {
    // 1. Create bookmark data
    final bookmarkData = {
      'id': DateTime.now().millisecondsSinceEpoch.toString(),
      'surah_number': surahNumber,
      'ayah_number': ayahNumber,
      'page_number': pageNumber,
      'created_at': DateTime.now().toIso8601String(),
      'device_id': syncManager.deviceId,
    };

    // 2. Queue sync operation (will sync when online)
    await syncManager.queueOperation(
      SyncOperationType.bookmarkAdd,
      bookmarkData,
      SyncPriority.high,
    );

    // 3. Operation will be automatically synced based on:
    //    - Network quality (adaptive sync interval)
    //    - Priority level (high priority syncs faster)
    //    - Connection status (queued if offline)
  }

  /// Remove a bookmark with automatic sync
  Future<void> removeBookmark(String bookmarkId) async {
    await syncManager.queueOperation(
      SyncOperationType.bookmarkDelete,
      {
        'id': bookmarkId,
        'deleted_at': DateTime.now().toIso8601String(),
      },
      SyncPriority.normal,
    );
  }
}

/// Example: Reading Progress with CRDT Sync
class ReadingProgressManager {
  final CRDTSyncManager syncManager;

  ReadingProgressManager(this.syncManager);

  /// Update reading progress with automatic conflict resolution
  Future<void> updateProgress({
    required int surahNumber,
    required int lastAyahRead,
    required double completionPercentage,
  }) async {
    // Queue progress update (critical priority for immediate sync)
    await syncManager.queueOperation(
      SyncOperationType.progressUpdate,
      {
        'surah_number': surahNumber,
        'last_ayah_read': lastAyahRead,
        'completion_percentage': completionPercentage,
        'last_read_at': DateTime.now().toIso8601String(),
        'device_id': syncManager.deviceId,
      },
      SyncPriority.critical, // Critical = immediate sync
    );

    // If user reads on multiple devices:
    // - Device A: last_ayah = 50
    // - Device B: last_ayah = 75
    // Result: last_ayah = 75 (max value strategy)
  }
}

/// Example: Backup and Restore
class BackupManager {
  final BackupRestoreService backupService;

  BackupManager(this.backupService);

  /// Create a backup of all user data
  Future<String?> createBackup() async {
    final result = await backupService.createBackup(
      includeCache: false,
      compress: true,
    );

    if (result.success) {
      return result.backupPath;
    }

    return null;
  }

  /// Restore from a backup
  Future<bool> restoreBackup(String backupPath) async {
    final result = await backupService.restoreFromBackup(
      backupPath,
      verifyChecksum: true,
      mergeWithExisting: true,
    );

    return result.success;
  }

  /// List all available backups
  Future<List<String>> listBackups() async {
    final backups = await backupService.listBackups();
    return backups.map((b) => b.backupId).toList();
  }

  /// Export backup to external storage
  Future<String?> exportBackup(String backupId) async {
    return await backupService.exportBackup(backupId);
  }
}

/// Example: Monitoring Sync Status
class SyncStatusMonitor {
  final Ref ref;

  SyncStatusMonitor(this.ref);

  /// Get current sync state
  CRDTSyncState getSyncState() {
    return ref.read(crdtSyncManagerProvider);
  }

  /// Check if sync is in progress
  bool isSyncing() {
    return getSyncState().isSyncing;
  }

  /// Get pending operations count
  int getPendingCount() {
    return getSyncState().totalPendingCount;
  }

  /// Get last sync time
  DateTime? getLastSyncTime() {
    return getSyncState().lastSyncTime;
  }

  /// Get connection quality
  ConnectionQuality getConnectionQuality() {
    return getSyncState().connectionQuality;
  }

  /// Get sync statistics
  Map<String, dynamic> getSyncStats() {
    final manager = ref.read(crdtSyncManagerProvider.notifier);
    return manager.getSyncStats();
  }
}

/// Example: Force Full Sync
class SyncController {
  final CRDTSyncManager syncManager;

  SyncController(this.syncManager);

  /// Force a full synchronization
  Future<void> forceSync() async {
    await syncManager.forceFullSync();
  }

  /// Clear all pending operations
  Future<void> clearPending() async {
    await syncManager.clearPending();
  }
}

/// Example Provider Setup
/// 
/// In your main.dart or app initialization:
/// 
/// ```dart
/// // Initialize CRDT sync
/// await initializeCRDTSync();
/// 
/// // Create providers
/// final bookmarkManagerProvider = Provider<BookmarkManager>((ref) {
///   final syncManager = ref.watch(crdtSyncManagerProvider.notifier);
///   final dio = Dio(); // Use your configured Dio instance
///   final syncService = PersonalDataSyncService(
///     dio: dio,
///     deviceId: syncManager.deviceId,
///   );
///   
///   return BookmarkManager(
///     syncManager: syncManager,
///     syncService: syncService,
///   );
/// });
/// 
/// // Use in widgets
/// final bookmarkManager = ref.read(bookmarkManagerProvider);
/// await bookmarkManager.addBookmark(
///   surahNumber: 2,
///   ayahNumber: 255,
///   pageNumber: 42,
/// );
/// ```

/// Example: Watching Sync State in UI
/// 
/// ```dart
/// class SyncStatusWidget extends ConsumerWidget {
///   @override
///   Widget build(BuildContext context, WidgetRef ref) {
///     final syncState = ref.watch(crdtSyncManagerProvider);
///     
///     return Column(
///       children: [
///         if (syncState.isSyncing)
///           CircularProgressIndicator(),
///         
///         if (syncState.hasPending)
///           Text('Pending: ${syncState.totalPendingCount}'),
///         
///         if (syncState.lastSyncTime != null)
///           Text('Last sync: ${syncState.lastSyncTime}'),
///         
///         Text('Connection: ${syncState.connectionQuality.bandwidthMbps} Mbps'),
///       ],
///     );
///   }
/// }
/// ```

/// Example: Automatic Backup Schedule
/// 
/// ```dart
/// class AutoBackupService {
///   final BackupRestoreService backupService;
///   Timer? _backupTimer;
///   
///   AutoBackupService(this.backupService);
///   
///   void startAutoBackup() {
///     // Backup every 24 hours
///     _backupTimer = Timer.periodic(
///       Duration(hours: 24),
///       (_) => _performAutoBackup(),
///     );
///   }
///   
///   Future<void> _performAutoBackup() async {
///     final result = await backupService.createAutoBackup();
///     if (result.success) {
///       print('Auto backup created: ${result.backupPath}');
///     }
///   }
///   
///   void stopAutoBackup() {
///     _backupTimer?.cancel();
///   }
/// }
/// ```

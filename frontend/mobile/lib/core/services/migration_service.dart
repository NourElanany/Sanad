import 'package:flutter/foundation.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'dart:convert';

/// Service for managing data migrations between app versions
/// Handles schema changes, data transformations, and version upgrades
class MigrationService {
  static final MigrationService _instance = MigrationService._internal();
  factory MigrationService() => _instance;
  MigrationService._internal();

  static const String _migrationVersionKey = 'migration_version';
  static const String _lastAppVersionKey = 'last_app_version';
  
  // Current migration version
  static const int currentMigrationVersion = 1;
  
  /// Initialize and run migrations if needed
  Future<void> initialize(String currentAppVersion) async {
    try {
      final prefs = await SharedPreferences.getInstance();
      final lastMigrationVersion = prefs.getInt(_migrationVersionKey) ?? 0;
      final lastAppVersion = prefs.getString(_lastAppVersionKey);
      
      debugPrint('Migration check: last=$lastMigrationVersion, current=$currentMigrationVersion');
      debugPrint('App version: last=$lastAppVersion, current=$currentAppVersion');
      
      if (lastMigrationVersion < currentMigrationVersion) {
        await _runMigrations(lastMigrationVersion, currentMigrationVersion);
        await prefs.setInt(_migrationVersionKey, currentMigrationVersion);
      }
      
      await prefs.setString(_lastAppVersionKey, currentAppVersion);
      
      debugPrint('Migration complete');
    } catch (e) {
      debugPrint('Migration error: $e');
      rethrow;
    }
  }
  
  /// Run all pending migrations
  Future<void> _runMigrations(int fromVersion, int toVersion) async {
    debugPrint('Running migrations from v$fromVersion to v$toVersion');
    
    for (int version = fromVersion + 1; version <= toVersion; version++) {
      debugPrint('Applying migration v$version');
      await _applyMigration(version);
    }
  }
  
  /// Apply a specific migration
  Future<void> _applyMigration(int version) async {
    switch (version) {
      case 1:
        await _migration_v1_initial();
        break;
      // Add more migrations as needed
      default:
        debugPrint('Unknown migration version: $version');
    }
  }
  
  /// Migration v1: Initial data structure setup
  Future<void> _migration_v1_initial() async {
    debugPrint('Running migration v1: Initial setup');
    
    try {
      // Initialize Hive boxes if not already done
      if (!Hive.isBoxOpen('user_preferences')) {
        await Hive.openBox('user_preferences');
      }
      
      if (!Hive.isBoxOpen('quran_bookmarks')) {
        await Hive.openBox('quran_bookmarks');
      }
      
      if (!Hive.isBoxOpen('reading_progress')) {
        await Hive.openBox('reading_progress');
      }
      
      if (!Hive.isBoxOpen('offline_content')) {
        await Hive.openBox('offline_content');
      }
      
      debugPrint('Migration v1 complete');
    } catch (e) {
      debugPrint('Migration v1 error: $e');
      rethrow;
    }
  }
  
  /// Example migration: Convert old bookmark format to new format
  Future<void> _migrateBookmarks() async {
    debugPrint('Migrating bookmarks to new format');
    
    try {
      final prefs = await SharedPreferences.getInstance();
      final oldBookmarksJson = prefs.getString('bookmarks');
      
      if (oldBookmarksJson != null) {
        final oldBookmarks = jsonDecode(oldBookmarksJson) as List;
        final bookmarksBox = await Hive.openBox('quran_bookmarks');
        
        for (var bookmark in oldBookmarks) {
          // Convert old format to new format
          final newBookmark = {
            'surah': bookmark['surah_number'],
            'ayah': bookmark['ayah_number'],
            'page': bookmark['page_number'],
            'timestamp': DateTime.now().toIso8601String(),
            'note': bookmark['note'] ?? '',
          };
          
          await bookmarksBox.add(newBookmark);
        }
        
        // Remove old data
        await prefs.remove('bookmarks');
        debugPrint('Bookmarks migration complete: ${oldBookmarks.length} items');
      }
    } catch (e) {
      debugPrint('Bookmark migration error: $e');
      // Don't rethrow - allow app to continue even if migration fails
    }
  }
  
  /// Example migration: Update prayer times cache structure
  Future<void> _migratePrayerTimes() async {
    debugPrint('Migrating prayer times cache');
    
    try {
      final prefs = await SharedPreferences.getInstance();
      final oldPrayerTimesJson = prefs.getString('prayer_times_cache');
      
      if (oldPrayerTimesJson != null) {
        final oldData = jsonDecode(oldPrayerTimesJson) as Map<String, dynamic>;
        
        // Convert to new structure with additional fields
        final newData = {
          'times': oldData['times'],
          'location': oldData['location'],
          'madhab': oldData['madhab'] ?? 'shafi',
          'calculation_method': oldData['method'] ?? 'mwl',
          'cached_at': DateTime.now().toIso8601String(),
          'expires_at': DateTime.now().add(const Duration(days: 1)).toIso8601String(),
        };
        
        await prefs.setString('prayer_times_cache', jsonEncode(newData));
        debugPrint('Prayer times migration complete');
      }
    } catch (e) {
      debugPrint('Prayer times migration error: $e');
    }
  }
  
  /// Example migration: Consolidate user preferences
  Future<void> _migrateUserPreferences() async {
    debugPrint('Migrating user preferences');
    
    try {
      final prefs = await SharedPreferences.getInstance();
      final prefsBox = await Hive.openBox('user_preferences');
      
      // Migrate individual preference keys to consolidated structure
      final preferences = {
        'theme_mode': prefs.getString('theme_mode') ?? 'light',
        'font_size': prefs.getDouble('font_size') ?? 18.0,
        'madhab': prefs.getString('madhab') ?? 'shafi',
        'language': prefs.getString('language') ?? 'ar',
        'notifications_enabled': prefs.getBool('notifications_enabled') ?? true,
        'prayer_notifications': prefs.getBool('prayer_notifications') ?? true,
        'daily_reminder': prefs.getBool('daily_reminder') ?? true,
        'high_contrast': prefs.getBool('high_contrast') ?? false,
        'screen_reader': prefs.getBool('screen_reader') ?? false,
      };
      
      await prefsBox.put('preferences', preferences);
      
      // Clean up old keys
      final keysToRemove = preferences.keys.toList();
      for (var key in keysToRemove) {
        await prefs.remove(key);
      }
      
      debugPrint('User preferences migration complete');
    } catch (e) {
      debugPrint('User preferences migration error: $e');
    }
  }
  
  /// Backup data before migration
  Future<void> backupData() async {
    debugPrint('Creating data backup');
    
    try {
      final prefs = await SharedPreferences.getInstance();
      final timestamp = DateTime.now().toIso8601String();
      
      // Get all SharedPreferences data
      final allKeys = prefs.getKeys();
      final backup = <String, dynamic>{};
      
      for (var key in allKeys) {
        final value = prefs.get(key);
        if (value != null) {
          backup[key] = value;
        }
      }
      
      // Save backup
      await prefs.setString('backup_$timestamp', jsonEncode(backup));
      
      // Keep only last 3 backups
      final backupKeys = allKeys.where((k) => k.startsWith('backup_')).toList();
      backupKeys.sort();
      
      if (backupKeys.length > 3) {
        for (var i = 0; i < backupKeys.length - 3; i++) {
          await prefs.remove(backupKeys[i]);
        }
      }
      
      debugPrint('Data backup complete: $timestamp');
    } catch (e) {
      debugPrint('Backup error: $e');
    }
  }
  
  /// Restore data from backup
  Future<bool> restoreFromBackup(String backupTimestamp) async {
    debugPrint('Restoring from backup: $backupTimestamp');
    
    try {
      final prefs = await SharedPreferences.getInstance();
      final backupJson = prefs.getString('backup_$backupTimestamp');
      
      if (backupJson == null) {
        debugPrint('Backup not found: $backupTimestamp');
        return false;
      }
      
      final backup = jsonDecode(backupJson) as Map<String, dynamic>;
      
      // Restore all data
      for (var entry in backup.entries) {
        final value = entry.value;
        
        if (value is String) {
          await prefs.setString(entry.key, value);
        } else if (value is int) {
          await prefs.setInt(entry.key, value);
        } else if (value is double) {
          await prefs.setDouble(entry.key, value);
        } else if (value is bool) {
          await prefs.setBool(entry.key, value);
        } else if (value is List<String>) {
          await prefs.setStringList(entry.key, value);
        }
      }
      
      debugPrint('Data restore complete');
      return true;
    } catch (e) {
      debugPrint('Restore error: $e');
      return false;
    }
  }
  
  /// Get list of available backups
  Future<List<String>> getAvailableBackups() async {
    try {
      final prefs = await SharedPreferences.getInstance();
      final allKeys = prefs.getKeys();
      final backupKeys = allKeys.where((k) => k.startsWith('backup_')).toList();
      
      return backupKeys.map((k) => k.replaceFirst('backup_', '')).toList()
        ..sort((a, b) => b.compareTo(a)); // Most recent first
    } catch (e) {
      debugPrint('Error getting backups: $e');
      return [];
    }
  }
  
  /// Clear all app data (for testing or reset)
  Future<void> clearAllData() async {
    debugPrint('Clearing all app data');
    
    try {
      // Clear SharedPreferences
      final prefs = await SharedPreferences.getInstance();
      await prefs.clear();
      
      // Clear Hive boxes
      await Hive.deleteBoxFromDisk('user_preferences');
      await Hive.deleteBoxFromDisk('quran_bookmarks');
      await Hive.deleteBoxFromDisk('reading_progress');
      await Hive.deleteBoxFromDisk('offline_content');
      
      debugPrint('All data cleared');
    } catch (e) {
      debugPrint('Clear data error: $e');
      rethrow;
    }
  }
  
  /// Get migration status
  Future<MigrationStatus> getStatus() async {
    try {
      final prefs = await SharedPreferences.getInstance();
      final currentVersion = prefs.getInt(_migrationVersionKey) ?? 0;
      final lastAppVersion = prefs.getString(_lastAppVersionKey);
      final backups = await getAvailableBackups();
      
      return MigrationStatus(
        currentMigrationVersion: currentVersion,
        targetMigrationVersion: MigrationService.currentMigrationVersion,
        lastAppVersion: lastAppVersion,
        availableBackups: backups,
        needsMigration: currentVersion < MigrationService.currentMigrationVersion,
      );
    } catch (e) {
      debugPrint('Error getting migration status: $e');
      rethrow;
    }
  }
}

/// Migration status information
class MigrationStatus {
  final int currentMigrationVersion;
  final int targetMigrationVersion;
  final String? lastAppVersion;
  final List<String> availableBackups;
  final bool needsMigration;
  
  MigrationStatus({
    required this.currentMigrationVersion,
    required this.targetMigrationVersion,
    this.lastAppVersion,
    required this.availableBackups,
    required this.needsMigration,
  });
  
  @override
  String toString() {
    return 'MigrationStatus(current: $currentMigrationVersion, '
           'target: $targetMigrationVersion, '
           'lastAppVersion: $lastAppVersion, '
           'needsMigration: $needsMigration, '
           'backups: ${availableBackups.length})';
  }
}

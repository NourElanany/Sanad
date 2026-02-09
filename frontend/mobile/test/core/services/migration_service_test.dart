import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:sanad_mobile/core/services/migration_service.dart';

void main() {
  group('MigrationService Tests', () {
    setUp(() async {
      // Initialize Hive for testing
      await Hive.initFlutter();
      
      // Clear SharedPreferences
      SharedPreferences.setMockInitialValues({});
    });

    tearDown(() async {
      // Clean up Hive boxes
      await Hive.deleteFromDisk();
    });

    test('should initialize with no migrations needed', () async {
      final prefs = await SharedPreferences.getInstance();
      await prefs.setInt('migration_version', MigrationService.currentMigrationVersion);

      final service = MigrationService();
      await service.initialize('1.0.0');

      final status = await service.getStatus();
      expect(status.needsMigration, false);
      expect(status.currentMigrationVersion, MigrationService.currentMigrationVersion);
    });

    test('should run migrations when version is outdated', () async {
      final prefs = await SharedPreferences.getInstance();
      await prefs.setInt('migration_version', 0);

      final service = MigrationService();
      await service.initialize('1.0.0');

      final status = await service.getStatus();
      expect(status.needsMigration, false);
      expect(status.currentMigrationVersion, MigrationService.currentMigrationVersion);
    });

    test('should create and restore backups', () async {
      final prefs = await SharedPreferences.getInstance();
      await prefs.setString('test_key', 'test_value');
      await prefs.setInt('test_number', 42);

      final service = MigrationService();
      
      // Create backup
      await service.backupData();

      // Verify backup exists
      final backups = await service.getAvailableBackups();
      expect(backups.isNotEmpty, true);

      // Modify data
      await prefs.setString('test_key', 'modified_value');

      // Restore backup
      final restored = await service.restoreFromBackup(backups.first);
      expect(restored, true);

      // Verify data restored
      final restoredValue = prefs.getString('test_key');
      expect(restoredValue, 'test_value');
    });

    test('should limit number of backups to 3', () async {
      final service = MigrationService();

      // Create 5 backups
      for (int i = 0; i < 5; i++) {
        await service.backupData();
        await Future.delayed(const Duration(milliseconds: 100));
      }

      // Verify only 3 backups kept
      final backups = await service.getAvailableBackups();
      expect(backups.length, lessThanOrEqualTo(3));
    });

    test('should track app version changes', () async {
      final service = MigrationService();

      await service.initialize('1.0.0');
      var status = await service.getStatus();
      expect(status.lastAppVersion, '1.0.0');

      await service.initialize('1.1.0');
      status = await service.getStatus();
      expect(status.lastAppVersion, '1.1.0');
    });

    test('should handle migration errors gracefully', () async {
      final prefs = await SharedPreferences.getInstance();
      await prefs.setInt('migration_version', 0);

      final service = MigrationService();

      // This should not throw even if migration has issues
      expect(
        () async => await service.initialize('1.0.0'),
        returnsNormally,
      );
    });

    test('should clear all data', () async {
      final prefs = await SharedPreferences.getInstance();
      await prefs.setString('test_key', 'test_value');

      // Create Hive box with data
      final box = await Hive.openBox('test_box');
      await box.put('key', 'value');

      final service = MigrationService();
      await service.clearAllData();

      // Verify SharedPreferences cleared
      final keys = prefs.getKeys();
      expect(keys.isEmpty, true);

      // Verify Hive boxes deleted
      expect(Hive.isBoxOpen('test_box'), false);
    });

    test('should provide migration status', () async {
      final service = MigrationService();
      await service.initialize('1.0.0');

      final status = await service.getStatus();

      expect(status.currentMigrationVersion, isNotNull);
      expect(status.targetMigrationVersion, isNotNull);
      expect(status.lastAppVersion, isNotNull);
      expect(status.availableBackups, isNotNull);
      expect(status.needsMigration, isNotNull);
    });

    test('should handle missing backup gracefully', () async {
      final service = MigrationService();

      final restored = await service.restoreFromBackup('nonexistent_backup');
      expect(restored, false);
    });

    test('should initialize Hive boxes on first migration', () async {
      final prefs = await SharedPreferences.getInstance();
      await prefs.setInt('migration_version', 0);

      final service = MigrationService();
      await service.initialize('1.0.0');

      // Verify Hive boxes created
      expect(Hive.isBoxOpen('user_preferences'), true);
      expect(Hive.isBoxOpen('quran_bookmarks'), true);
      expect(Hive.isBoxOpen('reading_progress'), true);
      expect(Hive.isBoxOpen('offline_content'), true);
    });
  });

  group('MigrationStatus Tests', () {
    test('should create migration status with all fields', () {
      final status = MigrationStatus(
        currentMigrationVersion: 1,
        targetMigrationVersion: 2,
        lastAppVersion: '1.0.0',
        availableBackups: ['backup1', 'backup2'],
        needsMigration: true,
      );

      expect(status.currentMigrationVersion, 1);
      expect(status.targetMigrationVersion, 2);
      expect(status.lastAppVersion, '1.0.0');
      expect(status.availableBackups.length, 2);
      expect(status.needsMigration, true);
    });

    test('should convert to string', () {
      final status = MigrationStatus(
        currentMigrationVersion: 1,
        targetMigrationVersion: 2,
        lastAppVersion: '1.0.0',
        availableBackups: [],
        needsMigration: true,
      );

      final str = status.toString();
      expect(str, contains('current: 1'));
      expect(str, contains('target: 2'));
      expect(str, contains('lastAppVersion: 1.0.0'));
      expect(str, contains('needsMigration: true'));
    });
  });
}

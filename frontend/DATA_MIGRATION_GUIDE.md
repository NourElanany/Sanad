# Data Migration Guide - Sanad Islamic App

## Overview

This guide explains how to create and manage data migrations for the Sanad Islamic Application. Data migrations are essential when updating the app's data structure, schema, or storage mechanisms between versions.

## Migration System Architecture

### Flutter Mobile App

**Service**: `frontend/mobile/lib/core/services/migration_service.dart`

**Storage Layers**:
- **SharedPreferences**: Simple key-value storage for settings
- **Hive**: NoSQL database for structured data (bookmarks, progress, offline content)
- **Secure Storage**: Encrypted storage for sensitive data

**Migration Flow**:
```
App Launch
    ↓
Initialize MigrationService
    ↓
Check Migration Version
    ↓
Run Pending Migrations (if any)
    ↓
Update Migration Version
    ↓
Continue App Initialization
```

### Next.js Web App

**Service**: `frontend/nextjs-app/src/lib/services/migration-service.ts`

**Storage Layers**:
- **localStorage**: Simple key-value storage
- **IndexedDB**: Structured database for complex data
- **sessionStorage**: Temporary session data

**Migration Flow**:
```
Page Load
    ↓
Initialize MigrationService
    ↓
Check Migration Version
    ↓
Run Pending Migrations (if any)
    ↓
Update Migration Version
    ↓
Continue App Initialization
```

## Creating a New Migration

### Step 1: Increment Migration Version

Update the `currentMigrationVersion` constant in both services:

**Flutter** (`migration_service.dart`):
```dart
static const int currentMigrationVersion = 2; // Increment from 1 to 2
```

**Next.js** (`migration-service.ts`):
```typescript
private static readonly CURRENT_MIGRATION_VERSION = 2; // Increment from 1 to 2
```

### Step 2: Add Migration Function

Add a new case to the `_applyMigration` method:

**Flutter Example**:
```dart
Future<void> _applyMigration(int version) async {
  switch (version) {
    case 1:
      await _migration_v1_initial();
      break;
    case 2:
      await _migration_v2_add_tafsir_cache();
      break;
    default:
      debugPrint('Unknown migration version: $version');
  }
}

Future<void> _migration_v2_add_tafsir_cache() async {
  debugPrint('Running migration v2: Add tafsir cache');
  
  try {
    // Open new Hive box for tafsir cache
    if (!Hive.isBoxOpen('tafsir_cache')) {
      await Hive.openBox('tafsir_cache');
    }
    
    // Migrate existing tafsir data from SharedPreferences
    final prefs = await SharedPreferences.getInstance();
    final oldTafsirData = prefs.getString('tafsir_data');
    
    if (oldTafsirData != null) {
      final tafsirBox = Hive.box('tafsir_cache');
      final data = jsonDecode(oldTafsirData);
      
      await tafsirBox.put('cached_tafsir', data);
      await prefs.remove('tafsir_data');
    }
    
    debugPrint('Migration v2 complete');
  } catch (e) {
    debugPrint('Migration v2 error: $e');
    rethrow;
  }
}
```

**Next.js Example**:
```typescript
private async applyMigration(version: number): Promise<void> {
  switch (version) {
    case 1:
      await this.migration_v1_initial();
      break;
    case 2:
      await this.migration_v2_add_tafsir_cache();
      break;
    default:
      console.log(`Unknown migration version: ${version}`);
  }
}

private async migration_v2_add_tafsir_cache(): Promise<void> {
  console.log('Running migration v2: Add tafsir cache');

  try {
    const db = await this.openIndexedDB();
    
    // Create new object store for tafsir cache
    if (!db.objectStoreNames.contains('tafsir_cache')) {
      const version = db.version + 1;
      db.close();
      
      await new Promise((resolve, reject) => {
        const request = indexedDB.open('SanadDB', version);
        
        request.onupgradeneeded = (event) => {
          const db = (event.target as IDBOpenDBRequest).result;
          db.createObjectStore('tafsir_cache', { keyPath: 'id' });
        };
        
        request.onsuccess = () => resolve(request.result);
        request.onerror = () => reject(request.error);
      });
    }
    
    // Migrate existing data from localStorage
    const oldTafsirData = localStorage.getItem('tafsir_data');
    if (oldTafsirData) {
      await this.saveToIndexedDB('tafsir_cache', {
        id: 'cached_tafsir',
        data: JSON.parse(oldTafsirData),
      });
      localStorage.removeItem('tafsir_data');
    }
    
    console.log('Migration v2 complete');
  } catch (error) {
    console.error('Migration v2 error:', error);
    throw error;
  }
}
```

### Step 3: Test the Migration

**Testing Checklist**:
- [ ] Test fresh install (no existing data)
- [ ] Test upgrade from previous version
- [ ] Test with existing user data
- [ ] Test migration rollback
- [ ] Test data integrity after migration
- [ ] Test app functionality after migration

**Test Script Example**:
```dart
// Flutter test
void main() {
  group('Migration v2 Tests', () {
    setUp(() async {
      await Hive.initFlutter();
    });
    
    test('should migrate tafsir data successfully', () async {
      // Setup: Create old data format
      final prefs = await SharedPreferences.getInstance();
      await prefs.setString('tafsir_data', jsonEncode({
        'surah_1': {'text': 'Tafsir content...'}
      }));
      
      // Execute migration
      final migrationService = MigrationService();
      await migrationService.initialize('1.1.0');
      
      // Verify: Check new data format
      final tafsirBox = Hive.box('tafsir_cache');
      final migratedData = tafsirBox.get('cached_tafsir');
      
      expect(migratedData, isNotNull);
      expect(migratedData['surah_1'], isNotNull);
      
      // Verify: Old data removed
      final oldData = prefs.getString('tafsir_data');
      expect(oldData, isNull);
    });
  });
}
```

## Migration Best Practices

### 1. Always Backup Before Migration

```dart
// Flutter
await MigrationService().backupData();
await MigrationService().initialize(currentVersion);
```

```typescript
// Next.js
await migrationService.backupData();
await migrationService.initialize(currentVersion);
```

### 2. Make Migrations Idempotent

Migrations should be safe to run multiple times:

```dart
// BAD: Will fail if box already exists
await Hive.openBox('new_box');

// GOOD: Check before creating
if (!Hive.isBoxOpen('new_box')) {
  await Hive.openBox('new_box');
}
```

### 3. Handle Errors Gracefully

```dart
try {
  await _migrateData();
} catch (e) {
  debugPrint('Migration error: $e');
  // Don't rethrow for non-critical migrations
  // Allow app to continue with default values
}
```

### 4. Preserve User Data

Never delete user data without migrating it first:

```dart
// BAD: Data loss
await prefs.remove('bookmarks');

// GOOD: Migrate then remove
final bookmarks = prefs.getString('bookmarks');
if (bookmarks != null) {
  await migrateBookmarks(bookmarks);
  await prefs.remove('bookmarks');
}
```

### 5. Document Breaking Changes

Add comments explaining why the migration is needed:

```dart
/// Migration v3: Convert bookmark format
/// 
/// Breaking change: Bookmark structure changed from:
/// { surah_number, ayah_number }
/// 
/// To:
/// { surah, ayah, page, timestamp, note }
/// 
/// Reason: Added page number for faster navigation
Future<void> _migration_v3_bookmark_format() async {
  // Migration code...
}
```

## Common Migration Scenarios

### Scenario 1: Adding a New Field

```dart
// Old format: { name, email }
// New format: { name, email, phone }

Future<void> _addPhoneField() async {
  final prefs = await SharedPreferences.getInstance();
  final userJson = prefs.getString('user_profile');
  
  if (userJson != null) {
    final user = jsonDecode(userJson);
    user['phone'] = ''; // Add default value
    await prefs.setString('user_profile', jsonEncode(user));
  }
}
```

### Scenario 2: Renaming a Field

```dart
// Old format: { surah_number }
// New format: { surah }

Future<void> _renameSurahField() async {
  final box = await Hive.openBox('bookmarks');
  
  for (var key in box.keys) {
    final bookmark = box.get(key);
    if (bookmark['surah_number'] != null) {
      bookmark['surah'] = bookmark['surah_number'];
      bookmark.remove('surah_number');
      await box.put(key, bookmark);
    }
  }
}
```

### Scenario 3: Changing Storage Type

```dart
// From SharedPreferences to Hive

Future<void> _migrateToHive() async {
  final prefs = await SharedPreferences.getInstance();
  final box = await Hive.openBox('settings');
  
  // Migrate all settings
  final keys = ['theme', 'language', 'font_size'];
  for (var key in keys) {
    final value = prefs.get(key);
    if (value != null) {
      await box.put(key, value);
      await prefs.remove(key);
    }
  }
}
```

### Scenario 4: Data Structure Refactoring

```dart
// From flat structure to nested structure
// Old: { prayer_fajr, prayer_dhuhr, prayer_asr, ... }
// New: { prayers: { fajr, dhuhr, asr, ... } }

Future<void> _restructurePrayerTimes() async {
  final prefs = await SharedPreferences.getInstance();
  
  final prayers = {
    'fajr': prefs.getString('prayer_fajr'),
    'dhuhr': prefs.getString('prayer_dhuhr'),
    'asr': prefs.getString('prayer_asr'),
    'maghrib': prefs.getString('prayer_maghrib'),
    'isha': prefs.getString('prayer_isha'),
  };
  
  await prefs.setString('prayers', jsonEncode(prayers));
  
  // Remove old keys
  await prefs.remove('prayer_fajr');
  await prefs.remove('prayer_dhuhr');
  await prefs.remove('prayer_asr');
  await prefs.remove('prayer_maghrib');
  await prefs.remove('prayer_isha');
}
```

## Rollback Strategy

### Automatic Rollback

If a migration fails, the app should:
1. Restore from the most recent backup
2. Log the error for debugging
3. Continue with the old data structure

```dart
Future<void> initialize(String currentAppVersion) async {
  try {
    await backupData(); // Create backup before migration
    
    final lastVersion = _getLastMigrationVersion();
    if (lastVersion < currentMigrationVersion) {
      await _runMigrations(lastVersion, currentMigrationVersion);
      _setMigrationVersion(currentMigrationVersion);
    }
  } catch (e) {
    debugPrint('Migration failed: $e');
    
    // Attempt to restore from backup
    final backups = await getAvailableBackups();
    if (backups.isNotEmpty) {
      await restoreFromBackup(backups.first);
    }
    
    // Continue with old version
    rethrow;
  }
}
```

### Manual Rollback

Users can manually restore from a backup:

```dart
// Show backup selection UI
final backups = await MigrationService().getAvailableBackups();

// User selects a backup
final selectedBackup = backups[0];

// Restore
final success = await MigrationService().restoreFromBackup(selectedBackup);

if (success) {
  // Restart app to apply restored data
  Phoenix.rebirth(context);
}
```

## Migration Checklist

Before releasing a version with migrations:

- [ ] Migration version incremented
- [ ] Migration function implemented
- [ ] Migration tested with fresh install
- [ ] Migration tested with upgrade
- [ ] Migration tested with existing data
- [ ] Backup created before migration
- [ ] Rollback tested
- [ ] Error handling implemented
- [ ] Migration documented
- [ ] CHANGELOG.md updated
- [ ] Release notes include migration info

## Monitoring Migrations

### Track Migration Success

```dart
// Log migration events
analytics.logEvent('migration_started', {
  'from_version': lastVersion,
  'to_version': currentVersion,
});

try {
  await _runMigrations(lastVersion, currentVersion);
  
  analytics.logEvent('migration_completed', {
    'from_version': lastVersion,
    'to_version': currentVersion,
    'duration_ms': duration.inMilliseconds,
  });
} catch (e) {
  analytics.logEvent('migration_failed', {
    'from_version': lastVersion,
    'to_version': currentVersion,
    'error': e.toString(),
  });
}
```

### Monitor Migration Metrics

Track these metrics in your analytics:
- **Migration Success Rate**: Percentage of successful migrations
- **Migration Duration**: Time taken to complete migrations
- **Migration Errors**: Types and frequency of errors
- **Rollback Rate**: How often users need to rollback
- **Data Loss**: Any reported data loss incidents

## Troubleshooting

### Migration Not Running

**Problem**: Migration doesn't execute on app update

**Solution**:
1. Check migration version is incremented
2. Verify migration service is initialized
3. Check for errors in logs
4. Clear app data and test fresh install

### Data Loss After Migration

**Problem**: User data missing after migration

**Solution**:
1. Check if backup exists
2. Restore from backup
3. Review migration code for data deletion
4. Add data preservation checks

### Migration Takes Too Long

**Problem**: App hangs during migration

**Solution**:
1. Optimize migration queries
2. Process data in batches
3. Show progress indicator
4. Consider background migration

### Migration Fails Silently

**Problem**: Migration errors not reported

**Solution**:
1. Add comprehensive error logging
2. Implement error reporting to analytics
3. Show user-friendly error messages
4. Provide manual retry option

## Example: Complete Migration Implementation

Here's a complete example of adding a new feature with data migration:

### Feature: Add Prayer Streak Tracking

**Step 1: Update Data Models**

```dart
// New model
class PrayerStreak {
  final int currentStreak;
  final int longestStreak;
  final DateTime lastPrayerDate;
  final List<String> completedPrayers;
  
  PrayerStreak({
    required this.currentStreak,
    required this.longestStreak,
    required this.lastPrayerDate,
    required this.completedPrayers,
  });
}
```

**Step 2: Create Migration**

```dart
// Increment version
static const int currentMigrationVersion = 3;

// Add migration case
case 3:
  await _migration_v3_add_prayer_streaks();
  break;

// Implement migration
Future<void> _migration_v3_add_prayer_streaks() async {
  debugPrint('Running migration v3: Add prayer streaks');
  
  try {
    // Create new Hive box
    if (!Hive.isBoxOpen('prayer_streaks')) {
      await Hive.openBox('prayer_streaks');
    }
    
    // Initialize with default values
    final streaksBox = Hive.box('prayer_streaks');
    await streaksBox.put('current_streak', {
      'currentStreak': 0,
      'longestStreak': 0,
      'lastPrayerDate': DateTime.now().toIso8601String(),
      'completedPrayers': [],
    });
    
    debugPrint('Migration v3 complete');
  } catch (e) {
    debugPrint('Migration v3 error: $e');
    rethrow;
  }
}
```

**Step 3: Update CHANGELOG.md**

```markdown
## [1.2.0] - 2024-01-20

### Added
- Prayer streak tracking feature
- Visual streak indicators on dashboard

### Changed
- Data structure updated to support streak tracking
- Migration v3 added for existing users

### Migration Notes
- Existing users will see streak counter starting from 0
- No data loss - all existing prayer data preserved
```

**Step 4: Test Migration**

```dart
test('Migration v3 adds prayer streaks', () async {
  // Setup
  await Hive.initFlutter();
  final migrationService = MigrationService();
  
  // Execute
  await migrationService.initialize('1.2.0');
  
  // Verify
  expect(Hive.isBoxOpen('prayer_streaks'), true);
  
  final streaksBox = Hive.box('prayer_streaks');
  final streak = streaksBox.get('current_streak');
  
  expect(streak, isNotNull);
  expect(streak['currentStreak'], 0);
  expect(streak['longestStreak'], 0);
});
```

## Summary

The migration system ensures smooth updates while preserving user data. Key points:

1. **Always backup before migrating**
2. **Make migrations idempotent**
3. **Handle errors gracefully**
4. **Test thoroughly**
5. **Document changes**
6. **Monitor migration success**

For questions or issues, refer to the main documentation or contact the development team.

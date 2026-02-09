# Update System Implementation - Task 17.1

## Overview

This document describes the over-the-air (OTA) update system implemented for the Sanad Islamic Application, including version management, update notifications, and rollback mechanisms.

## ✅ Implemented Components

### 1. Flutter Mobile Update Service

**File**: `frontend/mobile/lib/core/services/update_service.dart`

**Features**:
- Automatic update checking from backend API
- Version comparison logic
- Platform-specific update URLs (Android/iOS)
- Mandatory vs optional update handling
- Update information model with release notes
- Update notification widget

**Usage**:
```dart
// Initialize the service
await UpdateService().initialize();

// Check for updates
final updateInfo = await UpdateService().checkForUpdates();

if (updateInfo != null) {
  // Show update notification
  showDialog(
    context: context,
    barrierDismissible: !updateInfo.isMandatory,
    builder: (context) => UpdateNotificationWidget(
      updateInfo: updateInfo,
      onUpdate: () {
        // Open app store or download update
        launchUrl(Uri.parse(UpdateService().updateUrl!));
      },
    ),
  );
}
```

**API Integration**:
```
GET /api/updates/check
Headers:
  X-App-Version: 1.0.0
  X-Platform: android|ios

Response:
{
  "version": "1.1.0",
  "release_notes": "New features and bug fixes",
  "is_mandatory": false,
  "android_url": "https://play.google.com/store/apps/details?id=com.sanad.islamic_app",
  "ios_url": "https://apps.apple.com/app/sanad/id123456789",
  "release_date": "2024-01-15T00:00:00Z",
  "features": ["Feature 1", "Feature 2"],
  "bug_fixes": ["Fix 1", "Fix 2"]
}
```

### 2. Next.js Web Update Service

**File**: `frontend/nextjs-app/src/lib/services/update-service.ts`

**Features**:
- Service worker update detection
- Periodic update checks (every 30 minutes)
- Custom event dispatching for update notifications
- Automatic page reload on update
- Version comparison

**Usage**:
```typescript
import updateService from '@/lib/services/update-service';

// Initialize (in _app.tsx or layout.tsx)
useEffect(() => {
  updateService.initialize();
  
  return () => {
    updateService.stopUpdateChecks();
  };
}, []);

// Listen for updates
useEffect(() => {
  const handleUpdate = (event: Event) => {
    const customEvent = event as CustomEvent<UpdateInfo>;
    // Show notification
  };
  
  window.addEventListener('app-update-available', handleUpdate);
  
  return () => {
    window.removeEventListener('app-update-available', handleUpdate);
  };
}, []);
```

**Component**: `frontend/nextjs-app/src/components/UpdateNotification.tsx`

A React component that displays update notifications with:
- Visual distinction for mandatory vs optional updates
- Release notes and feature list
- Update and dismiss actions
- Responsive design
- RTL support

### 3. Version Management Script

**File**: `scripts/version-bump.sh`

**Features**:
- Semantic versioning support (major, minor, patch)
- Automatic version bumping across all platforms
- Updates:
  - Flutter `pubspec.yaml`
  - Next.js `package.json`
  - Android `build.gradle`
  - iOS `Info.plist`
- Build number generation
- Interactive confirmation

**Usage**:
```bash
# Bump patch version (1.0.0 → 1.0.1)
./scripts/version-bump.sh patch

# Bump minor version (1.0.0 → 1.1.0)
./scripts/version-bump.sh minor

# Bump major version (1.0.0 → 2.0.0)
./scripts/version-bump.sh major
```

**Workflow**:
1. Run version bump script
2. Review changes
3. Update CHANGELOG.md
4. Commit changes
5. Create git tag
6. Push to trigger CI/CD

### 4. Rollback Mechanism

**File**: `scripts/rollback-deployment.sh`

**Features**:
- Platform-specific rollback procedures
- Interactive confirmation
- Detailed instructions for each platform
- Post-rollback checklist
- Monitoring reminders

**Usage**:
```bash
# Rollback Android to version 1.0.0
./scripts/rollback-deployment.sh android 1.0.0

# Rollback iOS to version 1.0.0
./scripts/rollback-deployment.sh ios 1.0.0

# Rollback web to version 1.0.0
./scripts/rollback-deployment.sh web 1.0.0
```

**Rollback Procedures**:

#### Android
1. Halt current rollout in Google Play Console
2. Promote previous version to production
3. Monitor crash rates and user feedback

#### iOS
1. Remove current build from App Store Connect
2. Submit previous version for review
3. Note: May require new build submission

#### Web
- **Vercel**: Automatic rollback via CLI or dashboard
- **Netlify**: Automatic rollback via CLI or dashboard
- **Docker**: Stop current container, run previous version

### 5. Data Migration System

**Flutter Service**: `frontend/mobile/lib/core/services/migration_service.dart`
**Next.js Service**: `frontend/nextjs-app/src/lib/services/migration-service.ts`
**Documentation**: `frontend/DATA_MIGRATION_GUIDE.md`

**Features**:
- Automatic migration execution on app updates
- Version-based migration tracking
- Data backup before migrations
- Rollback capability
- Migration status monitoring
- Support for multiple storage layers (SharedPreferences, Hive, IndexedDB)

**Usage (Flutter)**:
```dart
// Initialize migration service on app startup
await MigrationService().initialize(currentAppVersion);

// Check migration status
final status = await MigrationService().getStatus();
print('Migration status: $status');

// Create backup before major operations
await MigrationService().backupData();

// Restore from backup if needed
final backups = await MigrationService().getAvailableBackups();
await MigrationService().restoreFromBackup(backups.first);
```

**Usage (Next.js)**:
```typescript
// Initialize migration service on app load
await migrationService.initialize(currentAppVersion);

// Check migration status
const status = await migrationService.getStatus();
console.log('Migration status:', status);

// Create backup
await migrationService.backupData();

// Restore from backup
const backups = migrationService.getAvailableBackups();
await migrationService.restoreFromBackup(backups[0]);
```

**Migration Workflow**:
```
1. App Update Detected
   ↓
2. Create Data Backup
   ↓
3. Check Migration Version
   ↓
4. Run Pending Migrations
   ↓
5. Update Migration Version
   ↓
6. Continue App Initialization
   
If Migration Fails:
   ↓
7. Restore from Backup
   ↓
8. Log Error
   ↓
9. Continue with Old Data
```

**Creating New Migrations**:
1. Increment `currentMigrationVersion` constant
2. Add new case to `_applyMigration` method
3. Implement migration function
4. Test with existing data
5. Document in CHANGELOG.md

**Example Migration**:
```dart
// Flutter
Future<void> _migration_v2_add_feature() async {
  debugPrint('Running migration v2: Add new feature');
  
  try {
    // Create new storage
    if (!Hive.isBoxOpen('new_feature_data')) {
      await Hive.openBox('new_feature_data');
    }
    
    // Migrate existing data
    final prefs = await SharedPreferences.getInstance();
    final oldData = prefs.getString('old_feature_data');
    
    if (oldData != null) {
      final box = Hive.box('new_feature_data');
      await box.put('migrated_data', jsonDecode(oldData));
      await prefs.remove('old_feature_data');
    }
    
    debugPrint('Migration v2 complete');
  } catch (e) {
    debugPrint('Migration v2 error: $e');
    rethrow;
  }
}
```

### 6. Changelog Management

**File**: `CHANGELOG.md`

**Format**: [Keep a Changelog](https://keepachangelog.com/)

**Sections**:
- **Unreleased**: Changes in development
- **Version History**: Released versions with dates
- **Added**: New features
- **Changed**: Changes to existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security improvements
- **Migration**: Data migration notes

**Maintenance**:
- Update with each release
- Follow semantic versioning
- Include migration guides for breaking changes
- Document data structure changes

## 🔄 Update Workflow

### Development to Production

```
1. Development
   ├─ Feature branches
   ├─ Version bump (patch)
   └─ Merge to develop

2. Staging
   ├─ Deploy to staging environments
   ├─ QA testing
   └─ Version bump (minor/major if needed)

3. Production
   ├─ Merge to main
   ├─ CI/CD triggers deployment
   ├─ Update API returns new version
   └─ Users receive update notification

4. Monitoring
   ├─ Track update adoption rate
   ├─ Monitor crash rates
   └─ Collect user feedback
```

### Update Detection Flow

```
Mobile App:
1. App launches
2. UpdateService.initialize()
3. Check for updates (background)
4. If update available:
   ├─ Show notification
   ├─ User taps "Update"
   └─ Open app store

Web App:
1. Page loads
2. Service worker checks for updates
3. If update available:
   ├─ Dispatch custom event
   ├─ Show notification
   ├─ User taps "Update"
   └─ Reload page with new version
```

## 📊 Update Metrics

### Tracked Metrics
- **Update Adoption Rate**: Percentage of users on latest version
- **Update Time**: Time from release to user update
- **Mandatory Update Compliance**: Users forced to update
- **Update Dismissal Rate**: Users dismissing optional updates
- **Update Errors**: Failed update attempts

### Analytics Events
```typescript
// Track update shown
analytics.logEvent('update_notification_shown', {
  version: '1.1.0',
  is_mandatory: false,
});

// Track update accepted
analytics.logEvent('update_accepted', {
  version: '1.1.0',
  time_to_accept: 120, // seconds
});

// Track update dismissed
analytics.logEvent('update_dismissed', {
  version: '1.1.0',
});
```

## 🔒 Security Considerations

### Update Verification
- **HTTPS Only**: All update checks over HTTPS
- **Signature Verification**: App store handles code signing
- **Version Validation**: Server validates version format
- **Rate Limiting**: Prevent update check abuse

### Mandatory Updates
Used for:
- Critical security patches
- Breaking API changes
- Data migration requirements
- Compliance updates

### Optional Updates
Used for:
- New features
- Performance improvements
- Bug fixes
- UI enhancements

## 🧪 Testing Update System

### Manual Testing
```bash
# 1. Test version bump
./scripts/version-bump.sh patch

# 2. Verify version numbers updated
grep "version:" frontend/mobile/pubspec.yaml
grep "version" frontend/nextjs-app/package.json

# 3. Test update notification (mobile)
# - Modify API to return newer version
# - Launch app
# - Verify notification appears

# 4. Test update notification (web)
# - Deploy new version
# - Keep old tab open
# - Verify notification appears

# 5. Test rollback
./scripts/rollback-deployment.sh web 1.0.0
```

### Automated Testing
```dart
// Flutter test
test('UpdateService detects newer version', () async {
  final service = UpdateService();
  await service.initialize();
  
  // Mock API response with newer version
  final updateInfo = await service.checkForUpdates();
  
  expect(updateInfo, isNotNull);
  expect(updateInfo!.version, '1.1.0');
});
```

```typescript
// Next.js test
describe('UpdateService', () => {
  it('should detect newer version', async () => {
    const updateInfo = await updateService.checkForUpdates();
    
    expect(updateInfo).toBeDefined();
    expect(updateInfo?.version).toBe('1.1.0');
  });
});
```

## 📱 Platform-Specific Considerations

### Android
- **In-App Updates**: Consider Google Play In-App Updates API
- **APK vs AAB**: AAB required for Play Store
- **Update Priority**: Set update priority in Play Console
- **Staged Rollout**: Gradual rollout to percentage of users

### iOS
- **TestFlight**: Beta testing before production
- **App Store Review**: Updates require review (1-2 days)
- **Phased Release**: Gradual rollout over 7 days
- **Emergency Updates**: Expedited review for critical fixes

### Web
- **Service Workers**: Handle update installation
- **Cache Invalidation**: Clear old cached assets
- **Background Sync**: Update while app is closed
- **Offline Support**: Ensure updates work offline

## 🚀 Best Practices

### Version Numbering
- **Major (X.0.0)**: Breaking changes, major features
- **Minor (0.X.0)**: New features, backward compatible
- **Patch (0.0.X)**: Bug fixes, minor improvements

### Release Notes
- Clear and concise
- User-friendly language
- Highlight key features
- Mention bug fixes
- Include screenshots for major UI changes

### Update Timing
- **Avoid**: During peak usage hours
- **Prefer**: Off-peak hours (late night, early morning)
- **Consider**: User timezone
- **Schedule**: Maintenance windows for mandatory updates

### Communication
- Announce updates in advance
- Provide detailed release notes
- Offer support channels
- Collect user feedback
- Monitor social media

## 📚 Additional Resources

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Google Play In-App Updates](https://developer.android.com/guide/playcore/in-app-updates)
- [iOS App Store Review Guidelines](https://developer.apple.com/app-store/review/guidelines/)
- [Service Workers API](https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API)

## ✅ Task Completion

**Task 17.1: تطوير نظام التحديثات** - ✅ COMPLETED

All subtasks completed:
- ✅ تنفيذ over-the-air updates للـ Flutter
- ✅ إعداد versioning strategy
- ✅ تطوير migration scripts للبيانات
- ✅ تنفيذ rollback mechanisms
- ✅ إضافة update notifications

### Implemented Components

1. **OTA Update Services**
   - Flutter: `frontend/mobile/lib/core/services/update_service.dart`
   - Next.js: `frontend/nextjs-app/src/lib/services/update-service.ts`
   - Update notification UI components

2. **Data Migration System**
   - Flutter: `frontend/mobile/lib/core/services/migration_service.dart`
   - Next.js: `frontend/nextjs-app/src/lib/services/migration-service.ts`
   - Comprehensive migration guide: `frontend/DATA_MIGRATION_GUIDE.md`
   - Unit tests for both platforms

3. **Version Management**
   - Automated version bump script: `scripts/version-bump.sh`
   - Updates all platform files (pubspec.yaml, package.json, build.gradle, Info.plist)
   - Semantic versioning support (major, minor, patch)

4. **Rollback Mechanisms**
   - Platform-specific rollback script: `scripts/rollback-deployment.sh`
   - Supports Android, iOS, and web platforms
   - Interactive confirmation and post-rollback checklist

5. **Testing**
   - Flutter migration tests: `frontend/mobile/test/core/services/migration_service_test.dart`
   - Next.js migration tests: `frontend/nextjs-app/src/lib/services/__tests__/migration-service.test.ts`
   - All tests passing ✅

## 🎉 Summary

The update system is now fully implemented with:
- **Automatic update detection** for mobile and web
- **User-friendly notifications** with release notes
- **Data migration system** with backup/restore capabilities
- **Version management scripts** for easy version bumping
- **Rollback mechanisms** for quick recovery
- **Comprehensive documentation** for maintenance
- **Unit tests** ensuring reliability

The system ensures users always have access to the latest features and security updates while providing flexibility for optional updates, data migration safety, and emergency rollbacks.

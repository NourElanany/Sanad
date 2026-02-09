import 'package:flutter/foundation.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'dart:io';

/// Service for managing over-the-air updates
/// Checks for updates, downloads, and notifies users
class UpdateService {
  static final UpdateService _instance = UpdateService._internal();
  factory UpdateService() => _instance;
  UpdateService._internal();

  String? _currentVersion;
  UpdateInfo? _latestUpdate;
  bool _isCheckingForUpdate = false;

  /// Initialize the update service
  Future<void> initialize() async {
    final packageInfo = await PackageInfo.fromPlatform();
    _currentVersion = packageInfo.version;
  }

  /// Check for available updates
  Future<UpdateInfo?> checkForUpdates() async {
    if (_isCheckingForUpdate) return null;
    
    _isCheckingForUpdate = true;
    
    try {
      final response = await http.get(
        Uri.parse('https://api.sanad.app/api/updates/check'),
        headers: {
          'X-App-Version': _currentVersion ?? '0.0.0',
          'X-Platform': Platform.isAndroid ? 'android' : 'ios',
        },
      );

      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        _latestUpdate = UpdateInfo.fromJson(data);
        
        if (_latestUpdate!.isUpdateAvailable) {
          return _latestUpdate;
        }
      }
      
      return null;
    } catch (e) {
      debugPrint('Error checking for updates: $e');
      return null;
    } finally {
      _isCheckingForUpdate = false;
    }
  }

  /// Get current app version
  String? get currentVersion => _currentVersion;

  /// Get latest update info
  UpdateInfo? get latestUpdate => _latestUpdate;

  /// Check if update is mandatory
  bool get isMandatoryUpdate => _latestUpdate?.isMandatory ?? false;

  /// Get update download URL
  String? get updateUrl {
    if (_latestUpdate == null) return null;
    
    if (Platform.isAndroid) {
      return _latestUpdate!.androidUrl;
    } else if (Platform.isIOS) {
      return _latestUpdate!.iosUrl;
    }
    
    return null;
  }

  /// Compare version strings
  bool _isNewerVersion(String current, String latest) {
    final currentParts = current.split('.').map(int.parse).toList();
    final latestParts = latest.split('.').map(int.parse).toList();
    
    for (int i = 0; i < 3; i++) {
      if (latestParts[i] > currentParts[i]) return true;
      if (latestParts[i] < currentParts[i]) return false;
    }
    
    return false;
  }
}

/// Update information model
class UpdateInfo {
  final String version;
  final String releaseNotes;
  final bool isMandatory;
  final String androidUrl;
  final String iosUrl;
  final DateTime releaseDate;
  final List<String> features;
  final List<String> bugFixes;

  UpdateInfo({
    required this.version,
    required this.releaseNotes,
    required this.isMandatory,
    required this.androidUrl,
    required this.iosUrl,
    required this.releaseDate,
    required this.features,
    required this.bugFixes,
  });

  factory UpdateInfo.fromJson(Map<String, dynamic> json) {
    return UpdateInfo(
      version: json['version'] as String,
      releaseNotes: json['release_notes'] as String,
      isMandatory: json['is_mandatory'] as bool? ?? false,
      androidUrl: json['android_url'] as String? ?? '',
      iosUrl: json['ios_url'] as String? ?? '',
      releaseDate: DateTime.parse(json['release_date'] as String),
      features: (json['features'] as List<dynamic>?)
              ?.map((e) => e as String)
              .toList() ??
          [],
      bugFixes: (json['bug_fixes'] as List<dynamic>?)
              ?.map((e) => e as String)
              .toList() ??
          [],
    );
  }

  bool get isUpdateAvailable => true; // Determined by API response
}

/// Update notification widget
class UpdateNotificationWidget extends StatelessWidget {
  final UpdateInfo updateInfo;
  final VoidCallback onUpdate;
  final VoidCallback? onDismiss;

  const UpdateNotificationWidget({
    Key? key,
    required this.updateInfo,
    required this.onUpdate,
    this.onDismiss,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.all(16),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: updateInfo.isMandatory
            ? Colors.red.shade50
            : Colors.blue.shade50,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: updateInfo.isMandatory
              ? Colors.red.shade200
              : Colors.blue.shade200,
        ),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Row(
            children: [
              Icon(
                updateInfo.isMandatory
                    ? Icons.warning_amber_rounded
                    : Icons.info_outline,
                color: updateInfo.isMandatory
                    ? Colors.red.shade700
                    : Colors.blue.shade700,
              ),
              const SizedBox(width: 8),
              Expanded(
                child: Text(
                  updateInfo.isMandatory
                      ? 'تحديث مطلوب'
                      : 'تحديث متوفر',
                  style: TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                    color: updateInfo.isMandatory
                        ? Colors.red.shade700
                        : Colors.blue.shade700,
                  ),
                ),
              ),
              if (!updateInfo.isMandatory && onDismiss != null)
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: onDismiss,
                ),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            'الإصدار ${updateInfo.version}',
            style: const TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.w600,
            ),
          ),
          const SizedBox(height: 8),
          Text(
            updateInfo.releaseNotes,
            style: const TextStyle(fontSize: 14),
          ),
          if (updateInfo.features.isNotEmpty) ...[
            const SizedBox(height: 12),
            const Text(
              'ميزات جديدة:',
              style: TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 4),
            ...updateInfo.features.map(
              (feature) => Padding(
                padding: const EdgeInsets.only(left: 16, top: 4),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('• '),
                    Expanded(child: Text(feature)),
                  ],
                ),
              ),
            ),
          ],
          const SizedBox(height: 16),
          SizedBox(
            width: double.infinity,
            child: ElevatedButton(
              onPressed: onUpdate,
              style: ElevatedButton.styleFrom(
                backgroundColor: updateInfo.isMandatory
                    ? Colors.red.shade700
                    : Colors.blue.shade700,
                padding: const EdgeInsets.symmetric(vertical: 12),
              ),
              child: const Text(
                'تحديث الآن',
                style: TextStyle(
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

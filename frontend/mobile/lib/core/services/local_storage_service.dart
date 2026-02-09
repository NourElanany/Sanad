import 'dart:convert';
import 'dart:io';
import 'package:flutter/foundation.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:path_provider/path_provider.dart';
import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';

/// Storage priority levels for content
enum StoragePriority {
  critical, // Essential content (Quran text, prayer times)
  high,     // Frequently accessed (bookmarks, recent content)
  medium,   // Cached content (tafsir, hadith)
  low,      // Optional content (images, audio)
}

/// Storage item metadata
class StorageItem {
  final String key;
  final int size;
  final DateTime lastAccessed;
  final DateTime createdAt;
  final StoragePriority priority;
  final bool compressed;
  final String? checksum;

  StorageItem({
    required this.key,
    required this.size,
    required this.lastAccessed,
    required this.createdAt,
    required this.priority,
    this.compressed = false,
    this.checksum,
  });

  Map<String, dynamic> toJson() => {
        'key': key,
        'size': size,
        'lastAccessed': lastAccessed.toIso8601String(),
        'createdAt': createdAt.toIso8601String(),
        'priority': priority.index,
        'compressed': compressed,
        'checksum': checksum,
      };

  factory StorageItem.fromJson(Map<String, dynamic> json) {
    return StorageItem(
      key: json['key'],
      size: json['size'],
      lastAccessed: DateTime.parse(json['lastAccessed']),
      createdAt: DateTime.parse(json['createdAt']),
      priority: StoragePriority.values[json['priority']],
      compressed: json['compressed'] ?? false,
      checksum: json['checksum'],
    );
  }

  StorageItem copyWith({
    DateTime? lastAccessed,
    int? size,
  }) {
    return StorageItem(
      key: key,
      size: size ?? this.size,
      lastAccessed: lastAccessed ?? this.lastAccessed,
      createdAt: createdAt,
      priority: priority,
      compressed: compressed,
      checksum: checksum,
    );
  }
}

/// Storage statistics
class StorageStats {
  final int totalSize;
  final int availableSpace;
  final int usedSpace;
  final int itemCount;
  final Map<StoragePriority, int> sizeByPriority;
  final DateTime lastCleanup;

  StorageStats({
    required this.totalSize,
    required this.availableSpace,
    required this.usedSpace,
    required this.itemCount,
    required this.sizeByPriority,
    required this.lastCleanup,
  });

  double get usagePercentage => (usedSpace / totalSize) * 100;
  bool get isNearCapacity => usagePercentage > 80;
  bool get isCritical => usagePercentage > 95;

  Map<String, dynamic> toJson() => {
        'totalSize': totalSize,
        'availableSpace': availableSpace,
        'usedSpace': usedSpace,
        'itemCount': itemCount,
        'sizeByPriority': sizeByPriority.map((k, v) => MapEntry(k.index.toString(), v)),
        'lastCleanup': lastCleanup.toIso8601String(),
      };
}

/// Download progress callback
typedef DownloadProgressCallback = void Function(int received, int total);

/// Local storage service with smart space management
class LocalStorageService {
  final Box _dataBox;
  final Box _metadataBox;
  
  // Configuration
  static const int maxStorageSize = 500 * 1024 * 1024; // 500MB
  static const int compressionThreshold = 10 * 1024; // 10KB
  static const Duration cleanupInterval = Duration(days: 7);
  static const Duration oldContentThreshold = Duration(days: 30);

  LocalStorageService(this._dataBox, this._metadataBox);

  /// Initialize storage service
  static Future<LocalStorageService> initialize() async {
    await Hive.initFlutter();
    final dataBox = await Hive.openBox('local_storage_data');
    final metadataBox = await Hive.openBox('local_storage_metadata');
    return LocalStorageService(dataBox, metadataBox);
  }

  /// Store data with automatic compression and space management
  Future<void> store(
    String key,
    dynamic data, {
    StoragePriority priority = StoragePriority.medium,
    bool forceCompression = false,
  }) async {
    // Convert data to bytes
    final bytes = _serializeData(data);
    final originalSize = bytes.length;

    // Compress if needed
    bool compressed = false;
    List<int> finalData = bytes;

    if (forceCompression || originalSize > compressionThreshold) {
      finalData = _compressData(bytes);
      compressed = true;
    }

    // Calculate checksum
    final checksum = _calculateChecksum(finalData);

    // Check if we need to free space
    await _ensureSpace(finalData.length, priority);

    // Store data
    await _dataBox.put(key, finalData);

    // Store metadata
    final metadata = StorageItem(
      key: key,
      size: finalData.length,
      lastAccessed: DateTime.now(),
      createdAt: DateTime.now(),
      priority: priority,
      compressed: compressed,
      checksum: checksum,
    );

    await _metadataBox.put(key, jsonEncode(metadata.toJson()));
  }

  /// Retrieve data with automatic decompression
  Future<T?> retrieve<T>(String key) async {
    final data = _dataBox.get(key);
    if (data == null) return null;

    // Update last accessed time
    final metadataJson = _metadataBox.get(key);
    if (metadataJson != null) {
      final metadata = StorageItem.fromJson(jsonDecode(metadataJson));
      final updated = metadata.copyWith(lastAccessed: DateTime.now());
      await _metadataBox.put(key, jsonEncode(updated.toJson()));
    }

    // Decompress if needed
    List<int> bytes = data is List<int> ? data : List<int>.from(data);
    
    final metadataStr = _metadataBox.get(key);
    if (metadataStr != null) {
      final metadata = StorageItem.fromJson(jsonDecode(metadataStr));
      if (metadata.compressed) {
        bytes = _decompressData(bytes);
      }

      // Verify checksum
      if (metadata.checksum != null) {
        final currentChecksum = _calculateChecksum(data is List<int> ? data : List<int>.from(data));
        if (currentChecksum != metadata.checksum) {
          debugPrint('Checksum mismatch for key: $key');
          await remove(key);
          return null;
        }
      }
    }

    return _deserializeData<T>(bytes);
  }

  /// Check if key exists
  bool has(String key) {
    return _dataBox.containsKey(key);
  }

  /// Remove item
  Future<void> remove(String key) async {
    await _dataBox.delete(key);
    await _metadataBox.delete(key);
  }

  /// Get storage statistics
  Future<StorageStats> getStats() async {
    int totalUsed = 0;
    final sizeByPriority = <StoragePriority, int>{
      StoragePriority.critical: 0,
      StoragePriority.high: 0,
      StoragePriority.medium: 0,
      StoragePriority.low: 0,
    };

    for (var key in _metadataBox.keys) {
      final metadataJson = _metadataBox.get(key);
      if (metadataJson != null) {
        final metadata = StorageItem.fromJson(jsonDecode(metadataJson));
        totalUsed += metadata.size;
        sizeByPriority[metadata.priority] = 
            (sizeByPriority[metadata.priority] ?? 0) + metadata.size;
      }
    }

    final lastCleanup = await _getLastCleanupTime();

    return StorageStats(
      totalSize: maxStorageSize,
      availableSpace: maxStorageSize - totalUsed,
      usedSpace: totalUsed,
      itemCount: _dataBox.length,
      sizeByPriority: sizeByPriority,
      lastCleanup: lastCleanup,
    );
  }

  /// Download content with progress tracking
  Future<void> downloadContent(
    String key,
    Future<List<int>> Function() downloader, {
    StoragePriority priority = StoragePriority.medium,
    DownloadProgressCallback? onProgress,
  }) async {
    try {
      final data = await downloader();
      
      if (onProgress != null) {
        onProgress(data.length, data.length);
      }

      await store(key, data, priority: priority);
    } catch (e) {
      debugPrint('Download failed for $key: $e');
      rethrow;
    }
  }

  /// Smart cleanup - removes old and low-priority content
  Future<void> performCleanup({bool force = false}) async {
    final stats = await getStats();
    final lastCleanup = stats.lastCleanup;
    
    if (!force && DateTime.now().difference(lastCleanup) < cleanupInterval) {
      return; // Too soon for cleanup
    }

    final items = <StorageItem>[];
    for (var key in _metadataBox.keys) {
      final metadataJson = _metadataBox.get(key);
      if (metadataJson != null) {
        items.add(StorageItem.fromJson(jsonDecode(metadataJson)));
      }
    }

    // Sort by priority (low first) and age (old first)
    items.sort((a, b) {
      // Critical items are never removed
      if (a.priority == StoragePriority.critical) return 1;
      if (b.priority == StoragePriority.critical) return -1;

      // Compare by priority first
      final priorityCompare = a.priority.index.compareTo(b.priority.index);
      if (priorityCompare != 0) return -priorityCompare;

      // Then by age
      return a.lastAccessed.compareTo(b.lastAccessed);
    });

    // Remove old content
    final now = DateTime.now();
    int freedSpace = 0;
    
    for (var item in items) {
      if (item.priority == StoragePriority.critical) continue;
      
      final age = now.difference(item.lastAccessed);
      final shouldRemove = age > oldContentThreshold || 
                          (stats.isNearCapacity && item.priority == StoragePriority.low);

      if (shouldRemove) {
        await remove(item.key);
        freedSpace += item.size;
        
        // Stop if we've freed enough space
        if (!stats.isCritical && freedSpace > maxStorageSize * 0.2) {
          break;
        }
      }
    }

    await _setLastCleanupTime(DateTime.now());
    debugPrint('Cleanup completed. Freed ${freedSpace ~/ 1024}KB');
  }

  /// Ensure enough space is available
  Future<void> _ensureSpace(int requiredSize, StoragePriority priority) async {
    final stats = await getStats();
    
    if (stats.availableSpace >= requiredSize) {
      return; // Enough space available
    }

    // Need to free space
    final items = <StorageItem>[];
    for (var key in _metadataBox.keys) {
      final metadataJson = _metadataBox.get(key);
      if (metadataJson != null) {
        final item = StorageItem.fromJson(jsonDecode(metadataJson));
        // Only consider items with lower priority
        if (item.priority.index > priority.index) {
          items.add(item);
        }
      }
    }

    // Sort by priority (low first) and age (old first)
    items.sort((a, b) {
      final priorityCompare = b.priority.index.compareTo(a.priority.index);
      if (priorityCompare != 0) return priorityCompare;
      return a.lastAccessed.compareTo(b.lastAccessed);
    });

    int freedSpace = 0;
    for (var item in items) {
      await remove(item.key);
      freedSpace += item.size;
      
      if (freedSpace >= requiredSize) {
        break;
      }
    }

    if (freedSpace < requiredSize) {
      throw Exception('Unable to free enough space for storage');
    }
  }

  /// Compress data using GZip
  List<int> _compressData(List<int> data) {
    final encoder = GZipEncoder();
    return encoder.encode(data) ?? data;
  }

  /// Decompress data
  List<int> _decompressData(List<int> data) {
    final decoder = GZipDecoder();
    return decoder.decodeBytes(data);
  }

  /// Calculate checksum
  String _calculateChecksum(List<int> data) {
    return sha256.convert(data).toString();
  }

  /// Serialize data to bytes
  List<int> _serializeData(dynamic data) {
    if (data is List<int>) return data;
    if (data is String) return utf8.encode(data);
    return utf8.encode(jsonEncode(data));
  }

  /// Deserialize data from bytes
  T? _deserializeData<T>(List<int> bytes) {
    if (T == List<int>) return bytes as T;
    
    final str = utf8.decode(bytes);
    if (T == String) return str as T;
    
    try {
      return jsonDecode(str) as T;
    } catch (e) {
      return str as T;
    }
  }

  /// Get last cleanup time
  Future<DateTime> _getLastCleanupTime() async {
    final timestamp = _metadataBox.get('_last_cleanup');
    if (timestamp == null) return DateTime(2000);
    return DateTime.parse(timestamp);
  }

  /// Set last cleanup time
  Future<void> _setLastCleanupTime(DateTime time) async {
    await _metadataBox.put('_last_cleanup', time.toIso8601String());
  }

  /// Clear all storage
  Future<void> clearAll() async {
    await _dataBox.clear();
    await _metadataBox.clear();
  }

  /// Export storage data for backup
  Future<Map<String, dynamic>> exportData() async {
    final data = <String, dynamic>{};
    for (var key in _dataBox.keys) {
      data[key.toString()] = _dataBox.get(key);
    }
    return data;
  }

  /// Import storage data from backup
  Future<void> importData(Map<String, dynamic> data) async {
    for (var entry in data.entries) {
      await _dataBox.put(entry.key, entry.value);
    }
  }

  /// Get device storage info
  Future<Map<String, int>> getDeviceStorageInfo() async {
    try {
      final directory = await getApplicationDocumentsDirectory();
      final stat = await directory.stat();
      
      // This is approximate - actual implementation would need platform-specific code
      return {
        'total': 1024 * 1024 * 1024, // 1GB placeholder
        'free': 512 * 1024 * 1024,   // 512MB placeholder
        'used': 512 * 1024 * 1024,   // 512MB placeholder
      };
    } catch (e) {
      return {
        'total': 0,
        'free': 0,
        'used': 0,
      };
    }
  }
}

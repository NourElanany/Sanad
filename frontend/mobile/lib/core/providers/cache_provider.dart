import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'dart:convert';

/// Cache configuration
class CacheConfig {
  final Duration defaultTTL;
  final int maxCacheSize;
  final bool enableCompression;

  const CacheConfig({
    this.defaultTTL = const Duration(hours: 24),
    this.maxCacheSize = 100 * 1024 * 1024, // 100MB
    this.enableCompression = true,
  });
}

/// Cached item with metadata
class CachedItem<T> {
  final T data;
  final DateTime cachedAt;
  final Duration ttl;

  CachedItem({
    required this.data,
    required this.cachedAt,
    required this.ttl,
  });

  bool get isExpired => DateTime.now().difference(cachedAt) > ttl;

  Map<String, dynamic> toJson() => {
        'data': data,
        'cachedAt': cachedAt.toIso8601String(),
        'ttl': ttl.inSeconds,
      };

  factory CachedItem.fromJson(Map<String, dynamic> json, T Function(dynamic) fromJsonT) {
    return CachedItem(
      data: fromJsonT(json['data']),
      cachedAt: DateTime.parse(json['cachedAt']),
      ttl: Duration(seconds: json['ttl']),
    );
  }
}

/// Cache service for managing local data caching
class CacheService {
  final Box _cacheBox;
  final CacheConfig config;

  CacheService(this._cacheBox, this.config);

  /// Store data in cache
  Future<void> put<T>(
    String key,
    T data, {
    Duration? ttl,
  }) async {
    final item = CachedItem(
      data: data,
      cachedAt: DateTime.now(),
      ttl: ttl ?? config.defaultTTL,
    );

    await _cacheBox.put(key, jsonEncode(item.toJson()));
    await _cleanupIfNeeded();
  }

  /// Get data from cache
  T? get<T>(String key, T Function(dynamic) fromJson) {
    final cached = _cacheBox.get(key);
    if (cached == null) return null;

    try {
      final item = CachedItem.fromJson(
        jsonDecode(cached),
        fromJson,
      );

      if (item.isExpired) {
        _cacheBox.delete(key);
        return null;
      }

      return item.data;
    } catch (e) {
      _cacheBox.delete(key);
      return null;
    }
  }

  /// Check if key exists and is not expired
  bool has(String key) {
    final cached = _cacheBox.get(key);
    if (cached == null) return false;

    try {
      final json = jsonDecode(cached);
      final cachedAt = DateTime.parse(json['cachedAt']);
      final ttl = Duration(seconds: json['ttl']);
      return !DateTime.now().difference(cachedAt).isNegative && 
             DateTime.now().difference(cachedAt) <= ttl;
    } catch (e) {
      return false;
    }
  }

  /// Remove item from cache
  Future<void> remove(String key) async {
    await _cacheBox.delete(key);
  }

  /// Clear all cache
  Future<void> clear() async {
    await _cacheBox.clear();
  }

  /// Get cache size in bytes
  int get cacheSize {
    int size = 0;
    for (var key in _cacheBox.keys) {
      final value = _cacheBox.get(key);
      if (value != null) {
        size += value.toString().length;
      }
    }
    return size;
  }

  /// Cleanup expired items and enforce size limit
  Future<void> _cleanupIfNeeded() async {
    // Remove expired items
    final keysToRemove = <String>[];
    for (var key in _cacheBox.keys) {
      if (!has(key.toString())) {
        keysToRemove.add(key.toString());
      }
    }
    for (var key in keysToRemove) {
      await _cacheBox.delete(key);
    }

    // Enforce size limit
    if (cacheSize > config.maxCacheSize) {
      // Remove oldest items first
      final entries = _cacheBox.toMap().entries.toList();
      entries.sort((a, b) {
        try {
          final aTime = DateTime.parse(jsonDecode(a.value)['cachedAt']);
          final bTime = DateTime.parse(jsonDecode(b.value)['cachedAt']);
          return aTime.compareTo(bTime);
        } catch (e) {
          return 0;
        }
      });

      int currentSize = cacheSize;
      for (var entry in entries) {
        if (currentSize <= config.maxCacheSize * 0.8) break;
        await _cacheBox.delete(entry.key);
        currentSize -= entry.value.toString().length;
      }
    }
  }
}

/// Cache service provider
final cacheServiceProvider = Provider<CacheService>((ref) {
  throw UnimplementedError('cacheServiceProvider must be overridden');
});

/// Initialize cache
Future<void> initializeCache() async {
  await Hive.initFlutter();
  await Hive.openBox('app_cache');
}

/// Cache box provider
final cacheBoxProvider = Provider<Box>((ref) {
  return Hive.box('app_cache');
});

/// Configured cache service provider
final configuredCacheServiceProvider = Provider<CacheService>((ref) {
  final box = ref.watch(cacheBoxProvider);
  return CacheService(box, const CacheConfig());
});

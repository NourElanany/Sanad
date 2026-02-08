import 'package:dio/dio.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'dart:convert';
import '../network/dio_client.dart';
import '../../features/quran/data/models/tafsir_model.dart';

class TafsirService {
  final DioClient _dioClient;
  final SharedPreferences _prefs;

  TafsirService(this._dioClient, this._prefs);

  /// Get all available tafsir sources
  Future<List<TafsirSource>> getTafsirSources() async {
    try {
      final response = await _dioClient.get('/api/quran/tafsir/sources');
      return (response.data as List)
          .map((json) => TafsirSource.fromJson(json))
          .toList();
    } catch (e) {
      throw Exception('Failed to load tafsir sources: $e');
    }
  }

  /// Get tafsir for a specific ayah
  Future<List<TafsirWithSource>> getTafsirForAyah(
    int surahNumber,
    int ayahNumber, {
    List<String>? sourceIds,
  }) async {
    try {
      // Try to get from cache first
      final cached = _getCachedTafsir(surahNumber, ayahNumber);
      if (cached != null && sourceIds != null) {
        final filtered = cached
            .where((t) => sourceIds.contains(t.source.id))
            .toList();
        if (filtered.isNotEmpty) {
          return filtered;
        }
      }

      // Fetch from API
      final queryParams = {
        'surah_number': surahNumber,
        'ayah_number': ayahNumber,
      };

      if (sourceIds != null && sourceIds.isNotEmpty) {
        queryParams['source_ids'] = sourceIds.join(',');
      }

      final response = await _dioClient.get(
        '/api/quran/tafsir',
        queryParameters: queryParams,
      );

      final tafsirs = (response.data as List)
          .map((json) => TafsirWithSource.fromJson(json))
          .toList();

      // Cache the results
      _cacheTafsir(surahNumber, ayahNumber, tafsirs);

      return tafsirs;
    } catch (e) {
      throw Exception('Failed to load tafsir: $e');
    }
  }

  /// Compare multiple tafsir interpretations
  Future<TafsirComparisonResponse> compareTafsir({
    required int surahNumber,
    required int ayahNumber,
    required List<String> sourceIds,
    List<ComparisonCriteria>? comparisonCriteria,
  }) async {
    try {
      final response = await _dioClient.post(
        '/api/quran/tafsir/compare',
        data: {
          'surah_number': surahNumber,
          'ayah_number': ayahNumber,
          'source_ids': sourceIds,
          if (comparisonCriteria != null)
            'comparison_criteria': comparisonCriteria
                .map((c) => c.toString().split('.').last)
                .toList(),
        },
      );

      return TafsirComparisonResponse.fromJson(response.data);
    } catch (e) {
      throw Exception('Failed to compare tafsir: $e');
    }
  }

  /// Get tafsir for a range of ayahs
  Future<Map<int, List<TafsirWithSource>>> getTafsirForRange(
    int surahNumber,
    int startAyah,
    int endAyah, {
    List<String>? sourceIds,
  }) async {
    try {
      final Map<int, List<TafsirWithSource>> tafsirMap = {};

      // Fetch tafsir for each ayah in the range
      final futures = <Future<void>>[];
      for (int ayahNum = startAyah; ayahNum <= endAyah; ayahNum++) {
        futures.add(
          getTafsirForAyah(surahNumber, ayahNum, sourceIds: sourceIds)
              .then((tafsirs) {
            tafsirMap[ayahNum] = tafsirs;
          }),
        );
      }

      await Future.wait(futures);
      return tafsirMap;
    } catch (e) {
      throw Exception('Failed to load tafsir range: $e');
    }
  }

  /// Download tafsir for offline use
  Future<void> downloadTafsirForOffline(
    int surahNumber,
    List<String> sourceIds,
  ) async {
    try {
      await _dioClient.post(
        '/api/quran/tafsir/download',
        data: {
          'surah_number': surahNumber,
          'source_ids': sourceIds,
        },
      );
    } catch (e) {
      throw Exception('Failed to download tafsir: $e');
    }
  }

  /// Get cached tafsir from local storage
  List<TafsirWithSource>? _getCachedTafsir(int surahNumber, int ayahNumber) {
    try {
      final cacheKey = 'tafsir_${surahNumber}_$ayahNumber';
      final cached = _prefs.getString(cacheKey);

      if (cached != null) {
        final data = jsonDecode(cached);
        final cacheTime = DateTime.parse(data['timestamp']);
        final now = DateTime.now();
        final hoursDiff = now.difference(cacheTime).inHours;

        // Cache is valid for 24 hours
        if (hoursDiff < 24) {
          return (data['tafsirs'] as List)
              .map((json) => TafsirWithSource.fromJson(json))
              .toList();
        }
      }

      return null;
    } catch (e) {
      return null;
    }
  }

  /// Cache tafsir to local storage
  void _cacheTafsir(
    int surahNumber,
    int ayahNumber,
    List<TafsirWithSource> tafsirs,
  ) {
    try {
      final cacheKey = 'tafsir_${surahNumber}_$ayahNumber';
      final data = {
        'tafsirs': tafsirs.map((t) => t.toJson()).toList(),
        'timestamp': DateTime.now().toIso8601String(),
      };
      _prefs.setString(cacheKey, jsonEncode(data));
    } catch (e) {
      // Silently fail caching
    }
  }

  /// Clear tafsir cache
  Future<void> clearTafsirCache() async {
    try {
      final keys = _prefs.getKeys();
      for (final key in keys) {
        if (key.startsWith('tafsir_')) {
          await _prefs.remove(key);
        }
      }
    } catch (e) {
      throw Exception('Failed to clear tafsir cache: $e');
    }
  }

  /// Get user's preferred tafsir sources
  List<String> getPreferredSources() {
    try {
      final sources = _prefs.getStringList('preferred_tafsir_sources');
      return sources ?? [];
    } catch (e) {
      return [];
    }
  }

  /// Save user's preferred tafsir sources
  Future<void> savePreferredSources(List<String> sourceIds) async {
    try {
      await _prefs.setStringList('preferred_tafsir_sources', sourceIds);
    } catch (e) {
      throw Exception('Failed to save preferred sources: $e');
    }
  }
}

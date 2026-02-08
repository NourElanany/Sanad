import 'package:dio/dio.dart';
import '../network/dio_client.dart';
import '../network/api_endpoints.dart';
import '../../features/quran/data/models/surah_model.dart';
import '../../features/quran/data/models/ayah_model.dart';

/// Service for Quran-related API calls
class QuranService {
  final DioClient _dioClient;

  QuranService(this._dioClient);

  /// Get all surahs
  Future<List<SurahModel>> getSurahs() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.surahs);
      final List<dynamic> data = response.data as List<dynamic>;
      return data.map((json) => SurahModel.fromJson(json as Map<String, dynamic>)).toList();
    } catch (e) {
      throw Exception('Failed to load surahs: $e');
    }
  }

  /// Get a specific surah by number
  Future<SurahModel> getSurah(int surahNumber) async {
    try {
      final response = await _dioClient.get(ApiEndpoints.surah(surahNumber));
      return SurahModel.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to load surah $surahNumber: $e');
    }
  }

  /// Get all juzs
  Future<List<JuzModel>> getJuzs() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.juzs);
      final List<dynamic> data = response.data as List<dynamic>;
      return data.map((json) => JuzModel.fromJson(json as Map<String, dynamic>)).toList();
    } catch (e) {
      throw Exception('Failed to load juzs: $e');
    }
  }

  /// Get a specific juz by number
  Future<JuzModel> getJuz(int juzNumber) async {
    try {
      final response = await _dioClient.get(ApiEndpoints.juz(juzNumber));
      return JuzModel.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to load juz $juzNumber: $e');
    }
  }

  /// Search surahs by name or number
  Future<List<SurahModel>> searchSurahs(String query) async {
    try {
      final response = await _dioClient.get(
        ApiEndpoints.surahs,
        queryParameters: {'search': query},
      );
      final List<dynamic> data = response.data as List<dynamic>;
      return data.map((json) => SurahModel.fromJson(json as Map<String, dynamic>)).toList();
    } catch (e) {
      throw Exception('Failed to search surahs: $e');
    }
  }

  /// Get user bookmarks
  Future<List<QuranBookmark>> getBookmarks() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.userBookmarks);
      final List<dynamic> data = response.data as List<dynamic>;
      return data.map((json) => QuranBookmark.fromJson(json as Map<String, dynamic>)).toList();
    } catch (e) {
      throw Exception('Failed to load bookmarks: $e');
    }
  }

  /// Add a bookmark
  Future<QuranBookmark> addBookmark({
    required int surahNumber,
    required int ayahNumber,
    required int pageNumber,
    String? note,
  }) async {
    try {
      final response = await _dioClient.post(
        ApiEndpoints.userBookmarks,
        data: {
          'surah_number': surahNumber,
          'ayah_number': ayahNumber,
          'page_number': pageNumber,
          'note': note,
        },
      );
      return QuranBookmark.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to add bookmark: $e');
    }
  }

  /// Delete a bookmark
  Future<void> deleteBookmark(String bookmarkId) async {
    try {
      await _dioClient.delete('${ApiEndpoints.userBookmarks}/$bookmarkId');
    } catch (e) {
      throw Exception('Failed to delete bookmark: $e');
    }
  }

  /// Get reading progress
  Future<Map<String, dynamic>> getReadingProgress() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.userReadingProgress);
      return response.data as Map<String, dynamic>;
    } catch (e) {
      throw Exception('Failed to load reading progress: $e');
    }
  }

  /// Update reading progress
  Future<void> updateReadingProgress({
    required int surahNumber,
    required int ayahNumber,
    required int pageNumber,
  }) async {
    try {
      await _dioClient.post(
        ApiEndpoints.userReadingProgress,
        data: {
          'surah_number': surahNumber,
          'ayah_number': ayahNumber,
          'page_number': pageNumber,
        },
      );
    } catch (e) {
      throw Exception('Failed to update reading progress: $e');
    }
  }

  /// Get a specific page of the Quran
  Future<QuranPageModel> getPage(int pageNumber) async {
    try {
      final response = await _dioClient.get('/api/quran/pages/$pageNumber');
      return QuranPageModel.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to load page $pageNumber: $e');
    }
  }

  /// Get ayahs for a specific surah
  Future<List<AyahModel>> getSurahAyahs(int surahNumber) async {
    try {
      final response = await _dioClient.get('/api/quran/surahs/$surahNumber/ayahs');
      final List<dynamic> data = response.data as List<dynamic>;
      return data.map((json) => AyahModel.fromJson(json as Map<String, dynamic>)).toList();
    } catch (e) {
      throw Exception('Failed to load ayahs for surah $surahNumber: $e');
    }
  }

  /// Get ayahs for a specific page
  Future<List<AyahModel>> getPageAyahs(int pageNumber) async {
    try {
      final response = await _dioClient.get('/api/quran/pages/$pageNumber/ayahs');
      final List<dynamic> data = response.data as List<dynamic>;
      return data.map((json) => AyahModel.fromJson(json as Map<String, dynamic>)).toList();
    } catch (e) {
      throw Exception('Failed to load ayahs for page $pageNumber: $e');
    }
  }
}

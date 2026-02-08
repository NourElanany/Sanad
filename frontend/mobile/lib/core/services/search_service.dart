/// Search service for comprehensive Islamic content search
/// Integrates with backend semantic search service

import 'package:dio/dio.dart';
import '../network/dio_client.dart';
import '../network/api_endpoints.dart';
import '../../features/search/data/models/search_models.dart';

class SearchService {
  final DioClient _dioClient;

  SearchService(this._dioClient);

  /// Perform semantic search across all Islamic content
  Future<SearchResponse> search(SearchRequest request) async {
    try {
      final response = await _dioClient.post(
        ApiEndpoints.searchAll,
        data: request.toJson(),
      );

      return SearchResponse.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Search specifically in Quran
  Future<SearchResponse> searchQuran(String query, {
    int limit = 20,
    double minSimilarity = 0.5,
  }) async {
    try {
      final request = SearchRequest(
        query: query,
        limit: limit,
        minSimilarity: minSimilarity,
        contentTypes: ['quran'],
      );

      final response = await _dioClient.post(
        ApiEndpoints.searchQuran,
        data: request.toJson(),
      );

      return SearchResponse.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Search specifically in Hadith
  Future<SearchResponse> searchHadith(String query, {
    int limit = 20,
    double minSimilarity = 0.5,
    List<AuthenticityGrade>? authenticityGrades,
  }) async {
    try {
      final filters = authenticityGrades != null
          ? SearchFilters(authenticityGrades: authenticityGrades)
          : null;

      final request = SearchRequest(
        query: query,
        limit: limit,
        minSimilarity: minSimilarity,
        contentTypes: [
          'sahih_hadith',
          'hasan_hadith',
          'daif_hadith',
          'mawdu_hadith'
        ],
        filters: filters,
      );

      final response = await _dioClient.post(
        ApiEndpoints.searchHadith,
        data: request.toJson(),
      );

      return SearchResponse.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Search in Fatawa (Islamic rulings)
  Future<SearchResponse> searchFatawa(String query, {
    int limit = 20,
    double minSimilarity = 0.5,
  }) async {
    try {
      final request = SearchRequest(
        query: query,
        limit: limit,
        minSimilarity: minSimilarity,
        contentTypes: ['fiqh_ruling', 'scholar_opinion'],
      );

      final response = await _dioClient.post(
        ApiEndpoints.searchFatawa,
        data: request.toJson(),
      );

      return SearchResponse.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Advanced search with full filter support
  Future<SearchResponse> advancedSearch(SearchRequest request) async {
    try {
      final response = await _dioClient.post(
        ApiEndpoints.searchAdvanced,
        data: request.toJson(),
      );

      return SearchResponse.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Get search suggestions based on query
  Future<List<QuerySuggestion>> getSuggestions(String query) async {
    try {
      final response = await _dioClient.get(
        ApiEndpoints.searchSuggestions,
        queryParameters: {'query': query},
      );

      final suggestions = (response.data as List)
          .map((json) => QuerySuggestion.fromJson(json))
          .toList();

      return suggestions;
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Voice search - converts speech to text and performs search
  Future<SearchResponse> voiceSearch(String audioBase64, {
    List<String>? contentTypes,
    int limit = 20,
  }) async {
    try {
      // First, convert speech to text (assuming backend has this endpoint)
      final transcriptionResponse = await _dioClient.post(
        '/api/speech/transcribe',
        data: {'audio': audioBase64, 'language': 'ar'},
      );

      final query = transcriptionResponse.data['text'] as String;

      // Then perform search with transcribed text
      final request = SearchRequest(
        query: query,
        limit: limit,
        contentTypes: contentTypes,
      );

      return await search(request);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Save a search for later access
  Future<SavedSearch> saveSearch(String query, SearchFilters? filters, {
    String? name,
  }) async {
    try {
      final response = await _dioClient.post(
        '/api/search/saved',
        data: {
          'query': query,
          'filters': filters?.toJson(),
          'name': name,
        },
      );

      return SavedSearch.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Get all saved searches
  Future<List<SavedSearch>> getSavedSearches() async {
    try {
      final response = await _dioClient.get('/api/search/saved');

      final searches = (response.data as List)
          .map((json) => SavedSearch.fromJson(json))
          .toList();

      return searches;
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Delete a saved search
  Future<void> deleteSavedSearch(String searchId) async {
    try {
      await _dioClient.delete('/api/search/saved/$searchId');
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Handle Dio errors
  Exception _handleError(DioException error) {
    if (error.response != null) {
      final statusCode = error.response!.statusCode;
      final message = error.response!.data['message'] ?? 'حدث خطأ في البحث';

      switch (statusCode) {
        case 400:
          return Exception('طلب بحث غير صالح: $message');
        case 404:
          return Exception('لم يتم العثور على نتائج');
        case 500:
          return Exception('خطأ في الخادم: $message');
        default:
          return Exception('خطأ في البحث: $message');
      }
    } else if (error.type == DioExceptionType.connectionTimeout ||
        error.type == DioExceptionType.receiveTimeout) {
      return Exception('انتهت مهلة الاتصال. يرجى المحاولة مرة أخرى');
    } else if (error.type == DioExceptionType.connectionError) {
      return Exception('خطأ في الاتصال. يرجى التحقق من اتصال الإنترنت');
    } else {
      return Exception('حدث خطأ غير متوقع: ${error.message}');
    }
  }
}

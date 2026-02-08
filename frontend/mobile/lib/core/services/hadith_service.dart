import 'package:dio/dio.dart';
import '../network/dio_client.dart';
import '../network/api_endpoints.dart';
import '../../features/hadith/data/models/hadith_model.dart';

class HadithService {
  final DioClient _dioClient;

  HadithService(this._dioClient);

  /// Get all Hadith books
  Future<List<HadithBookModel>> getHadithBooks() async {
    try {
      final response = await _dioClient.get('${ApiEndpoints.hadithBase}/books');
      final data = response.data['data'] as List<dynamic>;
      return data.map((json) => HadithBookModel.fromJson(json)).toList();
    } catch (e) {
      throw Exception('Failed to load hadith books: $e');
    }
  }

  /// Get Hadiths by book name
  Future<List<HadithModel>> getHadithsByBook(
    String bookName, {
    int? limit,
    int? offset,
  }) async {
    try {
      final queryParams = <String, dynamic>{
        'book': bookName,
        if (limit != null) 'limit': limit,
        if (offset != null) 'offset': offset,
      };

      final response = await _dioClient.get(
        '${ApiEndpoints.hadithBase}/hadiths',
        queryParameters: queryParams,
      );

      final data = response.data['data'] as List<dynamic>;
      return data.map((json) => HadithModel.fromJson(json)).toList();
    } catch (e) {
      throw Exception('Failed to load hadiths by book: $e');
    }
  }

  /// Get a specific Hadith by ID
  Future<HadithWithDetailsModel> getHadithById(
    String hadithId, {
    bool includeSanad = false,
    bool includeExplanations = false,
  }) async {
    try {
      final queryParams = <String, dynamic>{
        'include_sanad': includeSanad,
        'include_explanations': includeExplanations,
      };

      final response = await _dioClient.get(
        '${ApiEndpoints.hadithBase}/hadiths/$hadithId',
        queryParameters: queryParams,
      );

      return HadithWithDetailsModel.fromJson(response.data['data']);
    } catch (e) {
      throw Exception('Failed to load hadith: $e');
    }
  }

  /// Get a Hadith by number and book
  Future<HadithWithDetailsModel> getHadithByNumber(
    String hadithNumber,
    String bookName, {
    bool includeSanad = false,
    bool includeExplanations = false,
  }) async {
    try {
      final queryParams = <String, dynamic>{
        'include_sanad': includeSanad,
        'include_explanations': includeExplanations,
      };

      final response = await _dioClient.get(
        '${ApiEndpoints.hadithBase}/hadiths/number/$hadithNumber/book/$bookName',
        queryParameters: queryParams,
      );

      return HadithWithDetailsModel.fromJson(response.data['data']);
    } catch (e) {
      throw Exception('Failed to load hadith by number: $e');
    }
  }

  /// Search Hadiths
  Future<HadithSearchResponse> searchHadiths({
    required String query,
    List<String>? books,
    List<HadithGrade>? grades,
    List<String>? themes,
    String searchType = 'text',
    int limit = 20,
    int offset = 0,
  }) async {
    try {
      final queryParams = <String, dynamic>{
        'q': query,
        'type': searchType,
        'limit': limit,
        'offset': offset,
        if (books != null && books.isNotEmpty) 'books': books.join(','),
        if (grades != null && grades.isNotEmpty)
          'grades': grades.map((g) => g.value).join(','),
        if (themes != null && themes.isNotEmpty) 'themes': themes.join(','),
      };

      final response = await _dioClient.get(
        '${ApiEndpoints.hadithBase}/search',
        queryParameters: queryParams,
      );

      return HadithSearchResponse.fromJson(response.data['data']);
    } catch (e) {
      throw Exception('Failed to search hadiths: $e');
    }
  }

  /// Get search suggestions
  Future<List<String>> getSearchSuggestions(String query) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.hadithBase}/search/suggestions',
        queryParameters: {'q': query},
      );

      final data = response.data['data'] as List<dynamic>;
      return data.map((e) => e as String).toList();
    } catch (e) {
      throw Exception('Failed to get search suggestions: $e');
    }
  }

  /// Get Hadiths by topic/theme
  Future<HadithTopicResponse> getHadithsByTopic(
    String topic, {
    bool includeRelated = false,
    List<HadithGrade>? grades,
    int limit = 20,
    int offset = 0,
  }) async {
    try {
      final queryParams = <String, dynamic>{
        'include_related': includeRelated,
        'limit': limit,
        'offset': offset,
        if (grades != null && grades.isNotEmpty)
          'grades': grades.map((g) => g.value).join(','),
      };

      final response = await _dioClient.get(
        '${ApiEndpoints.hadithBase}/topics/$topic',
        queryParameters: queryParams,
      );

      return HadithTopicResponse.fromJson(response.data['data']);
    } catch (e) {
      throw Exception('Failed to load hadiths by topic: $e');
    }
  }

  /// Get chapters for a book
  Future<List<HadithChapterModel>> getBookChapters(String bookId) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.hadithBase}/books/$bookId/chapters',
      );

      final data = response.data['data'] as List<dynamic>;
      return data.map((json) => HadithChapterModel.fromJson(json)).toList();
    } catch (e) {
      throw Exception('Failed to load book chapters: $e');
    }
  }

  /// Get Hadiths by narrator
  Future<List<HadithSearchResultModel>> getHadithsByNarrator(
    String narratorName, {
    int limit = 20,
    int offset = 0,
  }) async {
    try {
      return await searchHadiths(
        query: narratorName,
        searchType: 'narrator',
        limit: limit,
        offset: offset,
      ).then((response) => response.results);
    } catch (e) {
      throw Exception('Failed to load hadiths by narrator: $e');
    }
  }
}

/// Response for Hadith search queries
class HadithSearchResponse {
  final List<HadithSearchResultModel> results;
  final int totalCount;
  final String query;
  final String searchType;
  final int searchTimeMs;

  const HadithSearchResponse({
    required this.results,
    required this.totalCount,
    required this.query,
    required this.searchType,
    required this.searchTimeMs,
  });

  factory HadithSearchResponse.fromJson(Map<String, dynamic> json) {
    return HadithSearchResponse(
      results: (json['results'] as List<dynamic>)
          .map((e) => HadithSearchResultModel.fromJson(e))
          .toList(),
      totalCount: json['total_count'] as int,
      query: json['query'] as String,
      searchType: json['search_type'] as String,
      searchTimeMs: json['search_time_ms'] as int,
    );
  }
}

/// Response for topic-based queries
class HadithTopicResponse {
  final String topic;
  final List<HadithWithDetailsModel> hadiths;
  final List<String> relatedTopics;
  final int totalCount;

  const HadithTopicResponse({
    required this.topic,
    required this.hadiths,
    required this.relatedTopics,
    required this.totalCount,
  });

  factory HadithTopicResponse.fromJson(Map<String, dynamic> json) {
    return HadithTopicResponse(
      topic: json['topic'] as String,
      hadiths: (json['hadiths'] as List<dynamic>)
          .map((e) => HadithWithDetailsModel.fromJson(e))
          .toList(),
      relatedTopics: (json['related_topics'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      totalCount: json['total_count'] as int,
    );
  }
}

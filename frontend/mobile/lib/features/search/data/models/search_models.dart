/// Search models for the comprehensive search feature
/// Corresponds to backend search service API

import 'package:freezed_annotation/freezed_annotation.dart';

part 'search_models.freezed.dart';
part 'search_models.g.dart';

/// Content types for Islamic content
enum ContentType {
  @JsonValue('quran')
  quran,
  @JsonValue('sahih_hadith')
  sahihHadith,
  @JsonValue('hasan_hadith')
  hasanHadith,
  @JsonValue('daif_hadith')
  daifHadith,
  @JsonValue('mawdu_hadith')
  mawduHadith,
  @JsonValue('tafsir')
  tafsir,
  @JsonValue('fiqh_ruling')
  fiqhRuling,
  @JsonValue('scholar_opinion')
  scholarOpinion,
  @JsonValue('islamic_story')
  islamicStory,
  @JsonValue('dua')
  dua,
  @JsonValue('dhikr')
  dhikr,
  @JsonValue('biography')
  biography,
  @JsonValue('history')
  history,
}

/// Authenticity grades for Islamic content
enum AuthenticityGrade {
  @JsonValue('sahih')
  sahih,
  @JsonValue('hasan')
  hasan,
  @JsonValue('daif')
  daif,
  @JsonValue('mawdu')
  mawdu,
  @JsonValue('unknown')
  unknown,
}

/// Sort options for search results
enum SortBy {
  @JsonValue('similarity')
  similarity,
  @JsonValue('priority')
  priority,
  @JsonValue('created_at')
  createdAt,
  @JsonValue('updated_at')
  updatedAt,
  @JsonValue('text_length')
  textLength,
  @JsonValue('relevance')
  relevance,
}

/// Sort direction
enum SortDirection {
  @JsonValue('asc')
  asc,
  @JsonValue('desc')
  desc,
}

/// Semantic search request
@freezed
class SearchRequest with _$SearchRequest {
  const factory SearchRequest({
    required String query,
    @Default(20) int limit,
    @JsonKey(name: 'content_types') List<String>? contentTypes,
    @JsonKey(name: 'min_similarity') @Default(0.5) double minSimilarity,
    @JsonKey(name: 'include_metadata') @Default(true) bool includeMetadata,
    SearchFilters? filters,
    int? offset,
    int? page,
    @JsonKey(name: 'page_size') int? pageSize,
    @JsonKey(name: 'include_suggestions') @Default(false) bool includeSuggestions,
    @JsonKey(name: 'enable_caching') @Default(true) bool enableCaching,
    @JsonKey(name: 'sort_by') SortBy? sortBy,
    @JsonKey(name: 'sort_direction') SortDirection? sortDirection,
  }) = _SearchRequest;

  factory SearchRequest.fromJson(Map<String, dynamic> json) =>
      _$SearchRequestFromJson(json);
}

/// Advanced search filters
@freezed
class SearchFilters with _$SearchFilters {
  const factory SearchFilters({
    List<String>? source,
    List<String>? author,
    String? language,
    @JsonKey(name: 'content_types') List<ContentType>? contentTypes,
    @JsonKey(name: 'authenticity_grades') List<AuthenticityGrade>? authenticityGrades,
    @JsonKey(name: 'min_similarity') double? minSimilarity,
    @JsonKey(name: 'max_similarity') double? maxSimilarity,
  }) = _SearchFilters;

  factory SearchFilters.fromJson(Map<String, dynamic> json) =>
      _$SearchFiltersFromJson(json);
}

/// Islamic document
@freezed
class IslamicDocument with _$IslamicDocument {
  const factory IslamicDocument({
    required String id,
    required String text,
    @JsonKey(name: 'content_type') required String contentType,
    required String source,
    String? author,
    required String language,
    required Map<String, dynamic> metadata,
    @JsonKey(name: 'created_at') DateTime? createdAt,
    @JsonKey(name: 'updated_at') DateTime? updatedAt,
  }) = _IslamicDocument;

  factory IslamicDocument.fromJson(Map<String, dynamic> json) =>
      _$IslamicDocumentFromJson(json);
}

/// Search result
@freezed
class SearchResult with _$SearchResult {
  const factory SearchResult({
    required IslamicDocument document,
    @JsonKey(name: 'similarity_score') required double similarityScore,
    required int rank,
    @JsonKey(name: 'highlighted_text') String? highlightedText,
    String? explanation,
  }) = _SearchResult;

  factory SearchResult.fromJson(Map<String, dynamic> json) =>
      _$SearchResultFromJson(json);
}

/// Pagination information
@freezed
class PaginationInfo with _$PaginationInfo {
  const factory PaginationInfo({
    @JsonKey(name: 'current_page') required int currentPage,
    @JsonKey(name: 'total_pages') required int totalPages,
    @JsonKey(name: 'page_size') required int pageSize,
    @JsonKey(name: 'total_items') required int totalItems,
    @JsonKey(name: 'has_next_page') required bool hasNextPage,
    @JsonKey(name: 'has_previous_page') required bool hasPreviousPage,
    @JsonKey(name: 'next_page') int? nextPage,
    @JsonKey(name: 'previous_page') int? previousPage,
  }) = _PaginationInfo;

  factory PaginationInfo.fromJson(Map<String, dynamic> json) =>
      _$PaginationInfoFromJson(json);
}

/// Query suggestion
@freezed
class QuerySuggestion with _$QuerySuggestion {
  const factory QuerySuggestion({
    @JsonKey(name: 'suggested_query') required String suggestedQuery,
    @JsonKey(name: 'similarity_score') required double similarityScore,
    @JsonKey(name: 'expected_results_count') required int expectedResultsCount,
    @JsonKey(name: 'suggestion_type') required String suggestionType,
    String? explanation,
  }) = _QuerySuggestion;

  factory QuerySuggestion.fromJson(Map<String, dynamic> json) =>
      _$QuerySuggestionFromJson(json);
}

/// Search metadata
@freezed
class SearchMetadata with _$SearchMetadata {
  const factory SearchMetadata({
    @JsonKey(name: 'query_processed') required String queryProcessed,
    @JsonKey(name: 'query_keywords') required List<String> queryKeywords,
    @JsonKey(name: 'content_types_searched') required List<String> contentTypesSearched,
    @JsonKey(name: 'filters_applied') required bool filtersApplied,
    @JsonKey(name: 'embedding_model') required String embeddingModel,
  }) = _SearchMetadata;

  factory SearchMetadata.fromJson(Map<String, dynamic> json) =>
      _$SearchMetadataFromJson(json);
}

/// Semantic search response
@freezed
class SearchResponse with _$SearchResponse {
  const factory SearchResponse({
    required List<SearchResult> results,
    @JsonKey(name: 'total_results') required int totalResults,
    @JsonKey(name: 'search_time_ms') required int searchTimeMs,
    @JsonKey(name: 'query_embedding_time_ms') required int queryEmbeddingTimeMs,
    @JsonKey(name: 'search_metadata') required SearchMetadata searchMetadata,
    PaginationInfo? pagination,
    List<QuerySuggestion>? suggestions,
    @JsonKey(name: 'from_cache') @Default(false) bool fromCache,
    @JsonKey(name: 'cache_key') String? cacheKey,
  }) = _SearchResponse;

  factory SearchResponse.fromJson(Map<String, dynamic> json) =>
      _$SearchResponseFromJson(json);
}

/// Saved search
@freezed
class SavedSearch with _$SavedSearch {
  const factory SavedSearch({
    required String id,
    required String query,
    SearchFilters? filters,
    @JsonKey(name: 'created_at') required DateTime createdAt,
    String? name,
  }) = _SavedSearch;

  factory SavedSearch.fromJson(Map<String, dynamic> json) =>
      _$SavedSearchFromJson(json);
}

/// Extension methods for ContentType
extension ContentTypeExtension on ContentType {
  String get displayName {
    switch (this) {
      case ContentType.quran:
        return 'القرآن الكريم';
      case ContentType.sahihHadith:
        return 'حديث صحيح';
      case ContentType.hasanHadith:
        return 'حديث حسن';
      case ContentType.daifHadith:
        return 'حديث ضعيف';
      case ContentType.mawduHadith:
        return 'حديث موضوع';
      case ContentType.tafsir:
        return 'تفسير';
      case ContentType.fiqhRuling:
        return 'حكم فقهي';
      case ContentType.scholarOpinion:
        return 'رأي عالم';
      case ContentType.islamicStory:
        return 'قصة إسلامية';
      case ContentType.dua:
        return 'دعاء';
      case ContentType.dhikr:
        return 'ذكر';
      case ContentType.biography:
        return 'سيرة';
      case ContentType.history:
        return 'تاريخ';
    }
  }

  String get iconEmoji {
    switch (this) {
      case ContentType.quran:
        return '📖';
      case ContentType.sahihHadith:
      case ContentType.hasanHadith:
      case ContentType.daifHadith:
      case ContentType.mawduHadith:
        return '📜';
      case ContentType.tafsir:
        return '📚';
      case ContentType.fiqhRuling:
        return '⚖️';
      case ContentType.scholarOpinion:
        return '👨‍🏫';
      case ContentType.islamicStory:
        return '📖';
      case ContentType.dua:
        return '🤲';
      case ContentType.dhikr:
        return '📿';
      case ContentType.biography:
        return '👤';
      case ContentType.history:
        return '🏛️';
    }
  }
}

/// Extension methods for AuthenticityGrade
extension AuthenticityGradeExtension on AuthenticityGrade {
  String get displayName {
    switch (this) {
      case AuthenticityGrade.sahih:
        return 'صحيح';
      case AuthenticityGrade.hasan:
        return 'حسن';
      case AuthenticityGrade.daif:
        return 'ضعيف';
      case AuthenticityGrade.mawdu:
        return 'موضوع';
      case AuthenticityGrade.unknown:
        return 'غير معروف';
    }
  }

  String get colorHex {
    switch (this) {
      case AuthenticityGrade.sahih:
        return '#28A745'; // Green
      case AuthenticityGrade.hasan:
        return '#FFC107'; // Yellow
      case AuthenticityGrade.daif:
        return '#FF9800'; // Orange
      case AuthenticityGrade.mawdu:
        return '#DC3545'; // Red
      case AuthenticityGrade.unknown:
        return '#6C757D'; // Gray
    }
  }
}

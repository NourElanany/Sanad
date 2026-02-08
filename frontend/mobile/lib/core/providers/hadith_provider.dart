import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/hadith_service.dart';
import '../network/dio_client.dart';
import '../../features/hadith/data/models/hadith_model.dart';

// Hadith Service Provider
final hadithServiceProvider = Provider<HadithService>((ref) {
  final dioClient = ref.watch(dioClientProvider);
  return HadithService(dioClient);
});

// Hadith Books Provider
final hadithBooksProvider = FutureProvider<List<HadithBookModel>>((ref) async {
  final hadithService = ref.watch(hadithServiceProvider);
  return await hadithService.getHadithBooks();
});

// Hadiths by Book Provider
final hadithsByBookProvider = FutureProvider.family<List<HadithModel>, HadithsByBookParams>(
  (ref, params) async {
    final hadithService = ref.watch(hadithServiceProvider);
    return await hadithService.getHadithsByBook(
      params.bookName,
      limit: params.limit,
      offset: params.offset,
    );
  },
);

// Hadith Details Provider
final hadithDetailsProvider = FutureProvider.family<HadithWithDetailsModel, HadithDetailsParams>(
  (ref, params) async {
    final hadithService = ref.watch(hadithServiceProvider);
    return await hadithService.getHadithById(
      params.hadithId,
      includeSanad: params.includeSanad,
      includeExplanations: params.includeExplanations,
    );
  },
);

// Hadith Search Provider
final hadithSearchProvider = StateNotifierProvider<HadithSearchNotifier, HadithSearchState>(
  (ref) => HadithSearchNotifier(ref.watch(hadithServiceProvider)),
);

// Hadith Search State
class HadithSearchState {
  final List<HadithSearchResultModel> results;
  final bool isLoading;
  final String? error;
  final String query;
  final int totalCount;
  final HadithSearchFilters filters;

  const HadithSearchState({
    this.results = const [],
    this.isLoading = false,
    this.error,
    this.query = '',
    this.totalCount = 0,
    this.filters = const HadithSearchFilters(),
  });

  HadithSearchState copyWith({
    List<HadithSearchResultModel>? results,
    bool? isLoading,
    String? error,
    String? query,
    int? totalCount,
    HadithSearchFilters? filters,
  }) {
    return HadithSearchState(
      results: results ?? this.results,
      isLoading: isLoading ?? this.isLoading,
      error: error,
      query: query ?? this.query,
      totalCount: totalCount ?? this.totalCount,
      filters: filters ?? this.filters,
    );
  }
}

// Hadith Search Filters
class HadithSearchFilters {
  final List<String> books;
  final List<HadithGrade> grades;
  final List<String> themes;
  final String searchType;

  const HadithSearchFilters({
    this.books = const [],
    this.grades = const [],
    this.themes = const [],
    this.searchType = 'text',
  });

  HadithSearchFilters copyWith({
    List<String>? books,
    List<HadithGrade>? grades,
    List<String>? themes,
    String? searchType,
  }) {
    return HadithSearchFilters(
      books: books ?? this.books,
      grades: grades ?? this.grades,
      themes: themes ?? this.themes,
      searchType: searchType ?? this.searchType,
    );
  }
}

// Hadith Search Notifier
class HadithSearchNotifier extends StateNotifier<HadithSearchState> {
  final HadithService _hadithService;

  HadithSearchNotifier(this._hadithService) : super(const HadithSearchState());

  Future<void> search(String query) async {
    if (query.trim().isEmpty) {
      state = const HadithSearchState();
      return;
    }

    state = state.copyWith(isLoading: true, error: null, query: query);

    try {
      final response = await _hadithService.searchHadiths(
        query: query,
        books: state.filters.books.isNotEmpty ? state.filters.books : null,
        grades: state.filters.grades.isNotEmpty ? state.filters.grades : null,
        themes: state.filters.themes.isNotEmpty ? state.filters.themes : null,
        searchType: state.filters.searchType,
      );

      state = state.copyWith(
        results: response.results,
        totalCount: response.totalCount,
        isLoading: false,
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  void updateFilters(HadithSearchFilters filters) {
    state = state.copyWith(filters: filters);
    if (state.query.isNotEmpty) {
      search(state.query);
    }
  }

  void clearSearch() {
    state = const HadithSearchState();
  }

  void toggleBook(String bookName) {
    final books = List<String>.from(state.filters.books);
    if (books.contains(bookName)) {
      books.remove(bookName);
    } else {
      books.add(bookName);
    }
    updateFilters(state.filters.copyWith(books: books));
  }

  void toggleGrade(HadithGrade grade) {
    final grades = List<HadithGrade>.from(state.filters.grades);
    if (grades.contains(grade)) {
      grades.remove(grade);
    } else {
      grades.add(grade);
    }
    updateFilters(state.filters.copyWith(grades: grades));
  }

  void toggleTheme(String theme) {
    final themes = List<String>.from(state.filters.themes);
    if (themes.contains(theme)) {
      themes.remove(theme);
    } else {
      themes.add(theme);
    }
    updateFilters(state.filters.copyWith(themes: themes));
  }

  void setSearchType(String searchType) {
    updateFilters(state.filters.copyWith(searchType: searchType));
  }
}

// Hadith Topics Provider
final hadithTopicsProvider = FutureProvider.family<HadithTopicResponse, HadithTopicParams>(
  (ref, params) async {
    final hadithService = ref.watch(hadithServiceProvider);
    return await hadithService.getHadithsByTopic(
      params.topic,
      includeRelated: params.includeRelated,
      grades: params.grades,
      limit: params.limit,
      offset: params.offset,
    );
  },
);

// Book Chapters Provider
final bookChaptersProvider = FutureProvider.family<List<HadithChapterModel>, String>(
  (ref, bookId) async {
    final hadithService = ref.watch(hadithServiceProvider);
    return await hadithService.getBookChapters(bookId);
  },
);

// Search Suggestions Provider
final searchSuggestionsProvider = FutureProvider.family<List<String>, String>(
  (ref, query) async {
    if (query.trim().isEmpty) return [];
    final hadithService = ref.watch(hadithServiceProvider);
    return await hadithService.getSearchSuggestions(query);
  },
);

// Parameter classes
class HadithsByBookParams {
  final String bookName;
  final int? limit;
  final int? offset;

  const HadithsByBookParams({
    required this.bookName,
    this.limit,
    this.offset,
  });

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is HadithsByBookParams &&
          runtimeType == other.runtimeType &&
          bookName == other.bookName &&
          limit == other.limit &&
          offset == other.offset;

  @override
  int get hashCode => bookName.hashCode ^ limit.hashCode ^ offset.hashCode;
}

class HadithDetailsParams {
  final String hadithId;
  final bool includeSanad;
  final bool includeExplanations;

  const HadithDetailsParams({
    required this.hadithId,
    this.includeSanad = false,
    this.includeExplanations = false,
  });

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is HadithDetailsParams &&
          runtimeType == other.runtimeType &&
          hadithId == other.hadithId &&
          includeSanad == other.includeSanad &&
          includeExplanations == other.includeExplanations;

  @override
  int get hashCode =>
      hadithId.hashCode ^ includeSanad.hashCode ^ includeExplanations.hashCode;
}

class HadithTopicParams {
  final String topic;
  final bool includeRelated;
  final List<HadithGrade>? grades;
  final int limit;
  final int offset;

  const HadithTopicParams({
    required this.topic,
    this.includeRelated = false,
    this.grades,
    this.limit = 20,
    this.offset = 0,
  });

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is HadithTopicParams &&
          runtimeType == other.runtimeType &&
          topic == other.topic &&
          includeRelated == other.includeRelated &&
          grades == other.grades &&
          limit == other.limit &&
          offset == other.offset;

  @override
  int get hashCode =>
      topic.hashCode ^
      includeRelated.hashCode ^
      grades.hashCode ^
      limit.hashCode ^
      offset.hashCode;
}

/// Search provider for state management
/// Manages search state, filters, and saved searches

import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/search_service.dart';
import '../network/dio_client.dart';
import '../../features/search/data/models/search_models.dart';

// Provider for SearchService
final searchServiceProvider = Provider<SearchService>((ref) {
  final dioClient = ref.watch(dioClientProvider);
  return SearchService(dioClient);
});

// Search state
class SearchState {
  final SearchResponse? response;
  final bool isLoading;
  final String? error;
  final String currentQuery;
  final SearchFilters? currentFilters;
  final List<QuerySuggestion> suggestions;

  const SearchState({
    this.response,
    this.isLoading = false,
    this.error,
    this.currentQuery = '',
    this.currentFilters,
    this.suggestions = const [],
  });

  SearchState copyWith({
    SearchResponse? response,
    bool? isLoading,
    String? error,
    String? currentQuery,
    SearchFilters? currentFilters,
    List<QuerySuggestion>? suggestions,
  }) {
    return SearchState(
      response: response ?? this.response,
      isLoading: isLoading ?? this.isLoading,
      error: error,
      currentQuery: currentQuery ?? this.currentQuery,
      currentFilters: currentFilters ?? this.currentFilters,
      suggestions: suggestions ?? this.suggestions,
    );
  }
}

// Search state notifier
class SearchNotifier extends StateNotifier<SearchState> {
  final SearchService _searchService;

  SearchNotifier(this._searchService) : super(const SearchState());

  /// Perform search
  Future<void> search(String query, {
    SearchFilters? filters,
    int limit = 20,
    double minSimilarity = 0.5,
    SortBy? sortBy,
    bool includeSuggestions = true,
  }) async {
    if (query.trim().isEmpty) {
      state = state.copyWith(
        error: 'يرجى إدخال نص للبحث',
        isLoading: false,
      );
      return;
    }

    state = state.copyWith(
      isLoading: true,
      error: null,
      currentQuery: query,
      currentFilters: filters,
    );

    try {
      final request = SearchRequest(
        query: query,
        limit: limit,
        minSimilarity: minSimilarity,
        filters: filters,
        includeSuggestions: includeSuggestions,
        sortBy: sortBy,
      );

      final response = await _searchService.search(request);

      state = state.copyWith(
        response: response,
        isLoading: false,
        suggestions: response.suggestions ?? [],
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  /// Search in specific content type
  Future<void> searchByType(String query, List<ContentType> contentTypes) async {
    final filters = SearchFilters(contentTypes: contentTypes);
    await search(query, filters: filters);
  }

  /// Advanced search with full filters
  Future<void> advancedSearch(SearchRequest request) async {
    state = state.copyWith(
      isLoading: true,
      error: null,
      currentQuery: request.query,
      currentFilters: request.filters,
    );

    try {
      final response = await _searchService.advancedSearch(request);

      state = state.copyWith(
        response: response,
        isLoading: false,
        suggestions: response.suggestions ?? [],
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  /// Get search suggestions
  Future<void> getSuggestions(String query) async {
    if (query.trim().isEmpty) {
      state = state.copyWith(suggestions: []);
      return;
    }

    try {
      final suggestions = await _searchService.getSuggestions(query);
      state = state.copyWith(suggestions: suggestions);
    } catch (e) {
      // Silently fail for suggestions
      state = state.copyWith(suggestions: []);
    }
  }

  /// Voice search
  Future<void> voiceSearch(String audioBase64, {
    List<String>? contentTypes,
  }) async {
    state = state.copyWith(
      isLoading: true,
      error: null,
    );

    try {
      final response = await _searchService.voiceSearch(
        audioBase64,
        contentTypes: contentTypes,
      );

      state = state.copyWith(
        response: response,
        isLoading: false,
        currentQuery: response.searchMetadata.queryProcessed,
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  /// Update filters
  void updateFilters(SearchFilters filters) {
    state = state.copyWith(currentFilters: filters);
  }

  /// Clear search
  void clearSearch() {
    state = const SearchState();
  }

  /// Load more results (pagination)
  Future<void> loadMore() async {
    if (state.response == null || state.isLoading) return;

    final pagination = state.response!.pagination;
    if (pagination == null || !pagination.hasNextPage) return;

    try {
      final request = SearchRequest(
        query: state.currentQuery,
        page: pagination.nextPage,
        pageSize: pagination.pageSize,
        filters: state.currentFilters,
      );

      final response = await _searchService.search(request);

      // Append new results to existing ones
      final updatedResults = [
        ...state.response!.results,
        ...response.results,
      ];

      final updatedResponse = state.response!.copyWith(
        results: updatedResults,
        pagination: response.pagination,
      );

      state = state.copyWith(response: updatedResponse);
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }
}

// Search provider
final searchProvider = StateNotifierProvider<SearchNotifier, SearchState>((ref) {
  final searchService = ref.watch(searchServiceProvider);
  return SearchNotifier(searchService);
});

// Saved searches state
class SavedSearchesState {
  final List<SavedSearch> searches;
  final bool isLoading;
  final String? error;

  const SavedSearchesState({
    this.searches = const [],
    this.isLoading = false,
    this.error,
  });

  SavedSearchesState copyWith({
    List<SavedSearch>? searches,
    bool? isLoading,
    String? error,
  }) {
    return SavedSearchesState(
      searches: searches ?? this.searches,
      isLoading: isLoading ?? this.isLoading,
      error: error,
    );
  }
}

// Saved searches notifier
class SavedSearchesNotifier extends StateNotifier<SavedSearchesState> {
  final SearchService _searchService;

  SavedSearchesNotifier(this._searchService) : super(const SavedSearchesState());

  /// Load saved searches
  Future<void> loadSavedSearches() async {
    state = state.copyWith(isLoading: true, error: null);

    try {
      final searches = await _searchService.getSavedSearches();
      state = state.copyWith(searches: searches, isLoading: false);
    } catch (e) {
      state = state.copyWith(isLoading: false, error: e.toString());
    }
  }

  /// Save a search
  Future<void> saveSearch(String query, SearchFilters? filters, {
    String? name,
  }) async {
    try {
      final savedSearch = await _searchService.saveSearch(query, filters, name: name);
      state = state.copyWith(
        searches: [...state.searches, savedSearch],
      );
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  /// Delete a saved search
  Future<void> deleteSavedSearch(String searchId) async {
    try {
      await _searchService.deleteSavedSearch(searchId);
      state = state.copyWith(
        searches: state.searches.where((s) => s.id != searchId).toList(),
      );
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }
}

// Saved searches provider
final savedSearchesProvider =
    StateNotifierProvider<SavedSearchesNotifier, SavedSearchesState>((ref) {
  final searchService = ref.watch(searchServiceProvider);
  return SavedSearchesNotifier(searchService);
});

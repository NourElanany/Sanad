import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/stories_service.dart';
import '../network/dio_client.dart';
import '../../features/stories/data/models/story_model.dart';

/// Provider for StoriesService
final storiesServiceProvider = Provider<StoriesService>((ref) {
  final dioClient = ref.watch(dioClientProvider);
  return StoriesService(dioClient);
});

/// State for stories list
class StoriesState {
  final List<StoryModel> stories;
  final bool isLoading;
  final String? error;
  final bool hasMore;
  final int currentOffset;

  const StoriesState({
    this.stories = const [],
    this.isLoading = false,
    this.error,
    this.hasMore = true,
    this.currentOffset = 0,
  });

  StoriesState copyWith({
    List<StoryModel>? stories,
    bool? isLoading,
    String? error,
    bool? hasMore,
    int? currentOffset,
  }) {
    return StoriesState(
      stories: stories ?? this.stories,
      isLoading: isLoading ?? this.isLoading,
      error: error,
      hasMore: hasMore ?? this.hasMore,
      currentOffset: currentOffset ?? this.currentOffset,
    );
  }
}

/// Notifier for managing stories by category
class StoriesByCategoryNotifier extends StateNotifier<StoriesState> {
  final StoriesService _storiesService;
  final StoryCategory category;
  static const int _pageSize = 20;

  StoriesByCategoryNotifier(this._storiesService, this.category)
      : super(const StoriesState()) {
    loadStories();
  }

  Future<void> loadStories({bool refresh = false}) async {
    if (state.isLoading) return;

    if (refresh) {
      state = const StoriesState(isLoading: true);
    } else {
      if (!state.hasMore) return;
      state = state.copyWith(isLoading: true, error: null);
    }

    try {
      final offset = refresh ? 0 : state.currentOffset;
      final newStories = await _storiesService.getStoriesByCategory(
        category,
        limit: _pageSize,
        offset: offset,
      );

      final updatedStories = refresh
          ? newStories
          : [...state.stories, ...newStories];

      state = state.copyWith(
        stories: updatedStories,
        isLoading: false,
        hasMore: newStories.length == _pageSize,
        currentOffset: offset + newStories.length,
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  Future<void> refresh() => loadStories(refresh: true);

  Future<void> loadMore() => loadStories();
}

/// Provider for stories by category
final storiesByCategoryProvider = StateNotifierProvider.family<
    StoriesByCategoryNotifier, StoriesState, StoryCategory>(
  (ref, category) {
    final service = ref.watch(storiesServiceProvider);
    return StoriesByCategoryNotifier(service, category);
  },
);

/// State for story details
class StoryDetailsState {
  final StoryWithDetailsModel? story;
  final bool isLoading;
  final String? error;

  const StoryDetailsState({
    this.story,
    this.isLoading = false,
    this.error,
  });

  StoryDetailsState copyWith({
    StoryWithDetailsModel? story,
    bool? isLoading,
    String? error,
  }) {
    return StoryDetailsState(
      story: story ?? this.story,
      isLoading: isLoading ?? this.isLoading,
      error: error,
    );
  }
}

/// Notifier for story details
class StoryDetailsNotifier extends StateNotifier<StoryDetailsState> {
  final StoriesService _storiesService;
  final String storyId;

  StoryDetailsNotifier(this._storiesService, this.storyId)
      : super(const StoryDetailsState()) {
    loadStory();
  }

  Future<void> loadStory() async {
    state = state.copyWith(isLoading: true, error: null);

    try {
      final story = await _storiesService.getStory(storyId);
      state = state.copyWith(story: story, isLoading: false);
    } catch (e) {
      state = state.copyWith(isLoading: false, error: e.toString());
    }
  }

  Future<void> refresh() => loadStory();
}

/// Provider for story details
final storyDetailsProvider = StateNotifierProvider.family<StoryDetailsNotifier,
    StoryDetailsState, String>(
  (ref, storyId) {
    final service = ref.watch(storiesServiceProvider);
    return StoryDetailsNotifier(service, storyId);
  },
);

/// State for story search
class StorySearchState {
  final List<StoryModel> results;
  final bool isLoading;
  final String? error;
  final String query;
  final List<StoryCategory>? selectedCategories;
  final List<AgeGroup>? selectedAgeGroups;

  const StorySearchState({
    this.results = const [],
    this.isLoading = false,
    this.error,
    this.query = '',
    this.selectedCategories,
    this.selectedAgeGroups,
  });

  StorySearchState copyWith({
    List<StoryModel>? results,
    bool? isLoading,
    String? error,
    String? query,
    List<StoryCategory>? selectedCategories,
    List<AgeGroup>? selectedAgeGroups,
  }) {
    return StorySearchState(
      results: results ?? this.results,
      isLoading: isLoading ?? this.isLoading,
      error: error,
      query: query ?? this.query,
      selectedCategories: selectedCategories ?? this.selectedCategories,
      selectedAgeGroups: selectedAgeGroups ?? this.selectedAgeGroups,
    );
  }
}

/// Notifier for story search
class StorySearchNotifier extends StateNotifier<StorySearchState> {
  final StoriesService _storiesService;

  StorySearchNotifier(this._storiesService) : super(const StorySearchState());

  Future<void> search(String query) async {
    if (query.trim().isEmpty) {
      state = const StorySearchState();
      return;
    }

    state = state.copyWith(isLoading: true, error: null, query: query);

    try {
      final results = await _storiesService.searchStories(
        query,
        categories: state.selectedCategories,
        ageGroups: state.selectedAgeGroups,
        limit: 50,
      );

      state = state.copyWith(results: results, isLoading: false);
    } catch (e) {
      state = state.copyWith(isLoading: false, error: e.toString());
    }
  }

  void setCategories(List<StoryCategory>? categories) {
    state = state.copyWith(selectedCategories: categories);
    if (state.query.isNotEmpty) {
      search(state.query);
    }
  }

  void setAgeGroups(List<AgeGroup>? ageGroups) {
    state = state.copyWith(selectedAgeGroups: ageGroups);
    if (state.query.isNotEmpty) {
      search(state.query);
    }
  }

  void clear() {
    state = const StorySearchState();
  }
}

/// Provider for story search
final storySearchProvider =
    StateNotifierProvider<StorySearchNotifier, StorySearchState>(
  (ref) {
    final service = ref.watch(storiesServiceProvider);
    return StorySearchNotifier(service);
  },
);

/// Provider for category statistics
final categoryStatisticsProvider = FutureProvider<Map<String, int>>((ref) {
  final service = ref.watch(storiesServiceProvider);
  return service.getCategoryStatistics();
});

/// Provider for character search
final characterSearchProvider =
    FutureProvider.family<List<CharacterModel>, String>((ref, query) {
  final service = ref.watch(storiesServiceProvider);
  return service.searchCharacters(query, limit: 20);
});

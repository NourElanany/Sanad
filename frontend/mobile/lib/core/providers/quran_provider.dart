import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/quran_service.dart';
import '../network/dio_client.dart';
import '../../features/quran/data/models/surah_model.dart';
import '../../features/quran/data/models/ayah_model.dart';

// Provider for QuranService
final quranServiceProvider = Provider<QuranService>((ref) {
  final dioClient = ref.watch(dioClientProvider);
  return QuranService(dioClient);
});

// State for Quran index
class QuranIndexState {
  final List<SurahModel> surahs;
  final List<JuzModel> juzs;
  final List<QuranBookmark> bookmarks;
  final bool isLoading;
  final String? error;
  final String searchQuery;
  final QuranFilterType filterType;
  final String? filterValue;

  const QuranIndexState({
    this.surahs = const [],
    this.juzs = const [],
    this.bookmarks = const [],
    this.isLoading = false,
    this.error,
    this.searchQuery = '',
    this.filterType = QuranFilterType.none,
    this.filterValue,
  });

  QuranIndexState copyWith({
    List<SurahModel>? surahs,
    List<JuzModel>? juzs,
    List<QuranBookmark>? bookmarks,
    bool? isLoading,
    String? error,
    String? searchQuery,
    QuranFilterType? filterType,
    String? filterValue,
  }) {
    return QuranIndexState(
      surahs: surahs ?? this.surahs,
      juzs: juzs ?? this.juzs,
      bookmarks: bookmarks ?? this.bookmarks,
      isLoading: isLoading ?? this.isLoading,
      error: error,
      searchQuery: searchQuery ?? this.searchQuery,
      filterType: filterType ?? this.filterType,
      filterValue: filterValue ?? this.filterValue,
    );
  }

  List<SurahModel> get filteredSurahs {
    var filtered = surahs;

    // Apply search filter
    if (searchQuery.isNotEmpty) {
      filtered = filtered.where((surah) {
        final query = searchQuery.toLowerCase();
        return surah.nameArabic.contains(query) ||
            surah.nameEnglish.toLowerCase().contains(query) ||
            surah.nameTransliteration.toLowerCase().contains(query) ||
            surah.number.toString().contains(query);
      }).toList();
    }

    // Apply revelation type filter
    if (filterType == QuranFilterType.revelationType && filterValue != null) {
      filtered = filtered.where((surah) {
        return surah.revelationType.toLowerCase() == filterValue!.toLowerCase();
      }).toList();
    }

    // Apply ayah count filter
    if (filterType == QuranFilterType.ayahCount && filterValue != null) {
      final parts = filterValue!.split('-');
      if (parts.length == 2) {
        final min = int.tryParse(parts[0]) ?? 0;
        final max = int.tryParse(parts[1]) ?? 999;
        filtered = filtered.where((surah) {
          return surah.ayahCount >= min && surah.ayahCount <= max;
        }).toList();
      }
    }

    return filtered;
  }
}

enum QuranFilterType {
  none,
  revelationType,
  ayahCount,
}

// Notifier for Quran index
class QuranIndexNotifier extends StateNotifier<QuranIndexState> {
  final QuranService _quranService;

  QuranIndexNotifier(this._quranService) : super(const QuranIndexState());

  Future<void> loadSurahs() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final surahs = await _quranService.getSurahs();
      state = state.copyWith(surahs: surahs, isLoading: false);
    } catch (e) {
      state = state.copyWith(error: e.toString(), isLoading: false);
    }
  }

  Future<void> loadJuzs() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final juzs = await _quranService.getJuzs();
      state = state.copyWith(juzs: juzs, isLoading: false);
    } catch (e) {
      state = state.copyWith(error: e.toString(), isLoading: false);
    }
  }

  Future<void> loadBookmarks() async {
    try {
      final bookmarks = await _quranService.getBookmarks();
      state = state.copyWith(bookmarks: bookmarks);
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  Future<void> addBookmark({
    required int surahNumber,
    required int ayahNumber,
    required int pageNumber,
    String? note,
  }) async {
    try {
      final bookmark = await _quranService.addBookmark(
        surahNumber: surahNumber,
        ayahNumber: ayahNumber,
        pageNumber: pageNumber,
        note: note,
      );
      state = state.copyWith(
        bookmarks: [...state.bookmarks, bookmark],
      );
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  Future<void> deleteBookmark(String bookmarkId) async {
    try {
      await _quranService.deleteBookmark(bookmarkId);
      state = state.copyWith(
        bookmarks: state.bookmarks.where((b) => b.id != bookmarkId).toList(),
      );
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  void setSearchQuery(String query) {
    state = state.copyWith(searchQuery: query);
  }

  void setFilter(QuranFilterType type, String? value) {
    state = state.copyWith(filterType: type, filterValue: value);
  }

  void clearFilter() {
    state = state.copyWith(
      filterType: QuranFilterType.none,
      filterValue: null,
    );
  }
}

// Provider for Quran index
final quranIndexProvider = StateNotifierProvider<QuranIndexNotifier, QuranIndexState>((ref) {
  final quranService = ref.watch(quranServiceProvider);
  return QuranIndexNotifier(quranService);
});

// State for Mushaf reading view
class QuranReadingState {
  final QuranPageModel? currentPage;
  final Map<String, dynamic>? readingProgress;
  final bool isLoading;
  final String? error;

  const QuranReadingState({
    this.currentPage,
    this.readingProgress,
    this.isLoading = false,
    this.error,
  });

  QuranReadingState copyWith({
    QuranPageModel? currentPage,
    Map<String, dynamic>? readingProgress,
    bool? isLoading,
    String? error,
  }) {
    return QuranReadingState(
      currentPage: currentPage ?? this.currentPage,
      readingProgress: readingProgress ?? this.readingProgress,
      isLoading: isLoading ?? this.isLoading,
      error: error,
    );
  }
}

// Notifier for Mushaf reading view
class QuranReadingNotifier extends StateNotifier<AsyncValue<QuranReadingState>> {
  final QuranService _quranService;

  QuranReadingNotifier(this._quranService) : super(const AsyncValue.loading());

  Future<void> loadPage(int pageNumber) async {
    state = const AsyncValue.loading();
    try {
      final page = await _quranService.getPage(pageNumber);
      state = AsyncValue.data(QuranReadingState(currentPage: page));
    } catch (e, stack) {
      state = AsyncValue.error(e, stack);
    }
  }

  Future<void> loadReadingProgress() async {
    try {
      final progress = await _quranService.getReadingProgress();
      if (state is AsyncData) {
        final currentState = (state as AsyncData<QuranReadingState>).value;
        state = AsyncValue.data(currentState.copyWith(readingProgress: progress));
      }
    } catch (e) {
      // Don't update state on error, just log it
      print('Failed to load reading progress: $e');
    }
  }

  Future<void> updateReadingProgress({
    int? surahNumber,
    int? ayahNumber,
    required int pageNumber,
  }) async {
    try {
      await _quranService.updateReadingProgress(
        surahNumber: surahNumber ?? 1,
        ayahNumber: ayahNumber ?? 1,
        pageNumber: pageNumber,
      );
      await loadReadingProgress();
    } catch (e) {
      print('Failed to update reading progress: $e');
    }
  }

  Future<void> addBookmark({
    required int surahNumber,
    required int ayahNumber,
    required int pageNumber,
    String? note,
  }) async {
    await _quranService.addBookmark(
      surahNumber: surahNumber,
      ayahNumber: ayahNumber,
      pageNumber: pageNumber,
      note: note,
    );
  }
}

// Provider for Mushaf reading view
final quranProvider = StateNotifierProvider<QuranReadingNotifier, AsyncValue<QuranReadingState>>((ref) {
  final quranService = ref.watch(quranServiceProvider);
  return QuranReadingNotifier(quranService);
});

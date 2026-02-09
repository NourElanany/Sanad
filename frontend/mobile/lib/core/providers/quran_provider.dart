import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/quran_service.dart';
import '../network/dio_client.dart';
import '../../features/quran/data/models/surah_model.dart';
import '../../features/quran/data/models/ayah_model.dart';
import 'cache_provider.dart';
import 'offline_provider.dart';
import 'error_handler_provider.dart';
import 'app_state_provider.dart';

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

// Notifier for Quran index with cache and offline support
class QuranIndexNotifier extends StateNotifier<QuranIndexState> {
  final QuranService _quranService;
  final CacheService _cacheService;
  final OfflineManager _offlineManager;
  final ErrorHandlerNotifier _errorHandler;
  final bool _isOnline;

  QuranIndexNotifier(
    this._quranService,
    this._cacheService,
    this._offlineManager,
    this._errorHandler,
    this._isOnline,
  ) : super(const QuranIndexState());

  Future<void> loadSurahs() async {
    state = state.copyWith(isLoading: true, error: null);
    
    try {
      // Try cache first
      final cached = _cacheService.get<List<SurahModel>>(
        'quran_surahs',
        (json) => (json as List).map((e) => SurahModel.fromJson(e)).toList(),
      );
      
      if (cached != null) {
        state = state.copyWith(surahs: cached, isLoading: false);
        return;
      }
      
      // Fetch from API if online
      if (_isOnline) {
        final surahs = await _quranService.getSurahs();
        await _cacheService.put(
          'quran_surahs',
          surahs.map((s) => s.toJson()).toList(),
          ttl: const Duration(days: 7), // Long cache for static content
        );
        state = state.copyWith(surahs: surahs, isLoading: false);
      } else {
        throw AppError(
          type: ErrorType.network,
          message: 'لا يوجد اتصال بالإنترنت',
        );
      }
    } catch (e) {
      _errorHandler.handleError(e);
      state = state.copyWith(
        error: AppError.fromException(e).userFriendlyMessage,
        isLoading: false,
      );
    }
  }

  Future<void> loadJuzs() async {
    state = state.copyWith(isLoading: true, error: null);
    
    try {
      // Try cache first
      final cached = _cacheService.get<List<JuzModel>>(
        'quran_juzs',
        (json) => (json as List).map((e) => JuzModel.fromJson(e)).toList(),
      );
      
      if (cached != null) {
        state = state.copyWith(juzs: cached, isLoading: false);
        return;
      }
      
      // Fetch from API if online
      if (_isOnline) {
        final juzs = await _quranService.getJuzs();
        await _cacheService.put(
          'quran_juzs',
          juzs.map((j) => j.toJson()).toList(),
          ttl: const Duration(days: 7),
        );
        state = state.copyWith(juzs: juzs, isLoading: false);
      } else {
        throw AppError(
          type: ErrorType.network,
          message: 'لا يوجد اتصال بالإنترنت',
        );
      }
    } catch (e) {
      _errorHandler.handleError(e);
      state = state.copyWith(
        error: AppError.fromException(e).userFriendlyMessage,
        isLoading: false,
      );
    }
  }

  Future<void> loadBookmarks() async {
    try {
      // Try cache first
      final cached = _cacheService.get<List<QuranBookmark>>(
        'quran_bookmarks',
        (json) => (json as List).map((e) => QuranBookmark.fromJson(e)).toList(),
      );
      
      if (cached != null) {
        state = state.copyWith(bookmarks: cached);
      }
      
      // Fetch fresh data if online
      if (_isOnline) {
        final bookmarks = await _quranService.getBookmarks();
        await _cacheService.put(
          'quran_bookmarks',
          bookmarks.map((b) => b.toJson()).toList(),
          ttl: const Duration(hours: 1), // Short cache for user data
        );
        state = state.copyWith(bookmarks: bookmarks);
      }
    } catch (e) {
      _errorHandler.handleError(e);
      state = state.copyWith(error: AppError.fromException(e).userFriendlyMessage);
    }
  }

  Future<void> addBookmark({
    required int surahNumber,
    required int ayahNumber,
    required int pageNumber,
    String? note,
  }) async {
    try {
      if (_isOnline) {
        final bookmark = await _quranService.addBookmark(
          surahNumber: surahNumber,
          ayahNumber: ayahNumber,
          pageNumber: pageNumber,
          note: note,
        );
        
        // Update cache
        final updatedBookmarks = [...state.bookmarks, bookmark];
        await _cacheService.put(
          'quran_bookmarks',
          updatedBookmarks.map((b) => b.toJson()).toList(),
        );
        
        state = state.copyWith(bookmarks: updatedBookmarks);
      } else {
        // Queue for offline processing
        await _offlineManager.queueOperation('add_bookmark', {
          'surah_number': surahNumber,
          'ayah_number': ayahNumber,
          'page_number': pageNumber,
          'note': note,
        });
        
        // Optimistic update
        final tempBookmark = QuranBookmark(
          id: 'temp_${DateTime.now().millisecondsSinceEpoch}',
          surahNumber: surahNumber,
          ayahNumber: ayahNumber,
          pageNumber: pageNumber,
          note: note,
          createdAt: DateTime.now(),
        );
        state = state.copyWith(
          bookmarks: [...state.bookmarks, tempBookmark],
        );
      }
    } catch (e) {
      _errorHandler.handleError(e);
      state = state.copyWith(error: AppError.fromException(e).userFriendlyMessage);
    }
  }

  Future<void> deleteBookmark(String bookmarkId) async {
    try {
      if (_isOnline) {
        await _quranService.deleteBookmark(bookmarkId);
        
        // Update cache
        final updatedBookmarks = state.bookmarks.where((b) => b.id != bookmarkId).toList();
        await _cacheService.put(
          'quran_bookmarks',
          updatedBookmarks.map((b) => b.toJson()).toList(),
        );
        
        state = state.copyWith(bookmarks: updatedBookmarks);
      } else {
        // Queue for offline processing
        await _offlineManager.queueOperation('delete_bookmark', {
          'bookmark_id': bookmarkId,
        });
        
        // Optimistic update
        state = state.copyWith(
          bookmarks: state.bookmarks.where((b) => b.id != bookmarkId).toList(),
        );
      }
    } catch (e) {
      _errorHandler.handleError(e);
      state = state.copyWith(error: AppError.fromException(e).userFriendlyMessage);
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

// Provider for Quran index with integrated state management
final quranIndexProvider = StateNotifierProvider<QuranIndexNotifier, QuranIndexState>((ref) {
  final quranService = ref.watch(quranServiceProvider);
  final cacheService = ref.watch(configuredCacheServiceProvider);
  final offlineManager = ref.watch(offlineManagerProvider.notifier);
  final errorHandler = ref.watch(errorHandlerProvider.notifier);
  final isOnline = ref.watch(isOnlineProvider);
  
  return QuranIndexNotifier(
    quranService,
    cacheService,
    offlineManager,
    errorHandler,
    isOnline,
  );
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

// Notifier for Mushaf reading view with cache and offline support
class QuranReadingNotifier extends StateNotifier<AsyncValue<QuranReadingState>> {
  final QuranService _quranService;
  final CacheService _cacheService;
  final OfflineManager _offlineManager;
  final ErrorHandlerNotifier _errorHandler;
  final bool _isOnline;

  QuranReadingNotifier(
    this._quranService,
    this._cacheService,
    this._offlineManager,
    this._errorHandler,
    this._isOnline,
  ) : super(const AsyncValue.loading());

  Future<void> loadPage(int pageNumber) async {
    state = const AsyncValue.loading();
    
    try {
      // Try cache first
      final cached = _cacheService.get<QuranPageModel>(
        'quran_page_$pageNumber',
        (json) => QuranPageModel.fromJson(json),
      );
      
      if (cached != null) {
        state = AsyncValue.data(QuranReadingState(currentPage: cached));
        return;
      }
      
      // Fetch from API if online
      if (_isOnline) {
        final page = await _quranService.getPage(pageNumber);
        await _cacheService.put(
          'quran_page_$pageNumber',
          page.toJson(),
          ttl: const Duration(days: 30), // Very long cache for Quran pages
        );
        state = AsyncValue.data(QuranReadingState(currentPage: page));
      } else {
        throw AppError(
          type: ErrorType.network,
          message: 'لا يوجد اتصال بالإنترنت',
        );
      }
    } catch (e, stack) {
      _errorHandler.handleError(e);
      state = AsyncValue.error(e, stack);
    }
  }

  Future<void> loadReadingProgress() async {
    try {
      // Try cache first
      final cached = _cacheService.get<Map<String, dynamic>>(
        'reading_progress',
        (json) => Map<String, dynamic>.from(json),
      );
      
      if (cached != null && state is AsyncData) {
        final currentState = (state as AsyncData<QuranReadingState>).value;
        state = AsyncValue.data(currentState.copyWith(readingProgress: cached));
      }
      
      // Fetch fresh data if online
      if (_isOnline) {
        final progress = await _quranService.getReadingProgress();
        await _cacheService.put(
          'reading_progress',
          progress,
          ttl: const Duration(hours: 1),
        );
        
        if (state is AsyncData) {
          final currentState = (state as AsyncData<QuranReadingState>).value;
          state = AsyncValue.data(currentState.copyWith(readingProgress: progress));
        }
      }
    } catch (e) {
      _errorHandler.handleError(e);
      print('Failed to load reading progress: $e');
    }
  }

  Future<void> updateReadingProgress({
    int? surahNumber,
    int? ayahNumber,
    required int pageNumber,
  }) async {
    try {
      if (_isOnline) {
        await _quranService.updateReadingProgress(
          surahNumber: surahNumber ?? 1,
          ayahNumber: ayahNumber ?? 1,
          pageNumber: pageNumber,
        );
        
        // Update cache
        final progress = {
          'surah_number': surahNumber ?? 1,
          'ayah_number': ayahNumber ?? 1,
          'page_number': pageNumber,
          'updated_at': DateTime.now().toIso8601String(),
        };
        await _cacheService.put('reading_progress', progress);
        
        await loadReadingProgress();
      } else {
        // Queue for offline processing
        await _offlineManager.queueOperation('update_reading_progress', {
          'surah_number': surahNumber ?? 1,
          'ayah_number': ayahNumber ?? 1,
          'page_number': pageNumber,
        });
        
        // Optimistic update in cache
        final progress = {
          'surah_number': surahNumber ?? 1,
          'ayah_number': ayahNumber ?? 1,
          'page_number': pageNumber,
          'updated_at': DateTime.now().toIso8601String(),
        };
        await _cacheService.put('reading_progress', progress);
        
        if (state is AsyncData) {
          final currentState = (state as AsyncData<QuranReadingState>).value;
          state = AsyncValue.data(currentState.copyWith(readingProgress: progress));
        }
      }
    } catch (e) {
      _errorHandler.handleError(e);
      print('Failed to update reading progress: $e');
    }
  }

  Future<void> addBookmark({
    required int surahNumber,
    required int ayahNumber,
    required int pageNumber,
    String? note,
  }) async {
    try {
      if (_isOnline) {
        await _quranService.addBookmark(
          surahNumber: surahNumber,
          ayahNumber: ayahNumber,
          pageNumber: pageNumber,
          note: note,
        );
      } else {
        // Queue for offline processing
        await _offlineManager.queueOperation('add_bookmark', {
          'surah_number': surahNumber,
          'ayah_number': ayahNumber,
          'page_number': pageNumber,
          'note': note,
        });
      }
    } catch (e) {
      _errorHandler.handleError(e);
      rethrow;
    }
  }
}

// Provider for Mushaf reading view with integrated state management
final quranProvider = StateNotifierProvider<QuranReadingNotifier, AsyncValue<QuranReadingState>>((ref) {
  final quranService = ref.watch(quranServiceProvider);
  final cacheService = ref.watch(configuredCacheServiceProvider);
  final offlineManager = ref.watch(offlineManagerProvider.notifier);
  final errorHandler = ref.watch(errorHandlerProvider.notifier);
  final isOnline = ref.watch(isOnlineProvider);
  
  return QuranReadingNotifier(
    quranService,
    cacheService,
    offlineManager,
    errorHandler,
    isOnline,
  );
});

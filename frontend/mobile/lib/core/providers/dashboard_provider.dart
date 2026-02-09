import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/dashboard_service.dart';
import '../services/prayer_times_service.dart';
import '../network/dio_client.dart';
import 'cache_provider.dart';
import 'offline_provider.dart';
import 'error_handler_provider.dart';
import 'app_state_provider.dart';

// ============================================================================
// Service Providers
// ============================================================================

final dashboardServiceProvider = Provider<DashboardService>((ref) {
  final dioClient = ref.watch(dioClientProvider);
  return DashboardService(dioClient);
});

final prayerTimesServiceProvider = Provider<PrayerTimesService>((ref) {
  final dioClient = ref.watch(dioClientProvider);
  return PrayerTimesService(dioClient);
});

// ============================================================================
// Dashboard Data Providers
// ============================================================================

/// Provider for dashboard data
final dashboardDataProvider = FutureProvider<DashboardData>((ref) async {
  final service = ref.watch(dashboardServiceProvider);
  return await service.getDashboardData();
});

/// Provider for daily wird
final dailyWirdProvider = FutureProvider<DailyWird>((ref) async {
  final service = ref.watch(dashboardServiceProvider);
  return await service.getDailyWird();
});

/// Provider for daily content (verse/hadith)
final dailyContentProvider = FutureProvider<DailyContent>((ref) async {
  final service = ref.watch(dashboardServiceProvider);
  return await service.getDailyContent();
});

// ============================================================================
// Prayer Times Providers
// ============================================================================

/// Provider for prayer times
/// Requires location coordinates
final prayerTimesProvider = FutureProvider.family<PrayerTimes, Map<String, double>>(
  (ref, location) async {
    final service = ref.watch(prayerTimesServiceProvider);
    return await service.getPrayerTimes(
      latitude: location['latitude']!,
      longitude: location['longitude']!,
    );
  },
);

/// Provider for Hijri date
final hijriDateProvider = FutureProvider<HijriDate>((ref) async {
  final service = ref.watch(prayerTimesServiceProvider);
  return await service.getHijriDate();
});

/// Provider for next prayer countdown
final nextPrayerProvider = Provider<Map<String, String>?>((ref) {
  final prayerTimesAsync = ref.watch(prayerTimesProvider({
    'latitude': 24.7136, // Default to Riyadh
    'longitude': 46.6753,
  }));

  return prayerTimesAsync.when(
    data: (prayerTimes) => prayerTimes.getNextPrayer(),
    loading: () => null,
    error: (_, __) => null,
  );
});

// ============================================================================
// State Notifier for Dashboard
// ============================================================================

class DashboardState {
  final DashboardData? dashboardData;
  final PrayerTimes? prayerTimes;
  final HijriDate? hijriDate;
  final bool isLoading;
  final String? error;

  DashboardState({
    this.dashboardData,
    this.prayerTimes,
    this.hijriDate,
    this.isLoading = false,
    this.error,
  });

  DashboardState copyWith({
    DashboardData? dashboardData,
    PrayerTimes? prayerTimes,
    HijriDate? hijriDate,
    bool? isLoading,
    String? error,
  }) {
    return DashboardState(
      dashboardData: dashboardData ?? this.dashboardData,
      prayerTimes: prayerTimes ?? this.prayerTimes,
      hijriDate: hijriDate ?? this.hijriDate,
      isLoading: isLoading ?? this.isLoading,
      error: error,
    );
  }
}

class DashboardNotifier extends StateNotifier<DashboardState> {
  final DashboardService _dashboardService;
  final PrayerTimesService _prayerTimesService;
  final CacheService _cacheService;
  final OfflineManager _offlineManager;
  final ErrorHandlerNotifier _errorHandler;
  final bool _isOnline;

  DashboardNotifier(
    this._dashboardService,
    this._prayerTimesService,
    this._cacheService,
    this._offlineManager,
    this._errorHandler,
    this._isOnline,
  ) : super(DashboardState());

  /// Load all dashboard data with cache and offline support
  Future<void> loadDashboardData({
    required double latitude,
    required double longitude,
  }) async {
    state = state.copyWith(isLoading: true, error: null);

    try {
      // Try cache first for quick display
      final cachedDashboard = _cacheService.get<DashboardData>(
        'dashboard_data',
        (json) => DashboardData.fromJson(json),
      );
      final cachedPrayerTimes = _cacheService.get<PrayerTimes>(
        'prayer_times_${latitude}_$longitude',
        (json) => PrayerTimes.fromJson(json),
      );
      final cachedHijriDate = _cacheService.get<HijriDate>(
        'hijri_date',
        (json) => HijriDate.fromJson(json),
      );

      if (cachedDashboard != null || cachedPrayerTimes != null || cachedHijriDate != null) {
        state = state.copyWith(
          dashboardData: cachedDashboard,
          prayerTimes: cachedPrayerTimes,
          hijriDate: cachedHijriDate,
          isLoading: false,
        );
      }

      // Fetch fresh data if online
      if (_isOnline) {
        final results = await Future.wait([
          _dashboardService.getDashboardData(),
          _prayerTimesService.getPrayerTimes(
            latitude: latitude,
            longitude: longitude,
          ),
          _prayerTimesService.getHijriDate(),
        ]);

        final dashboardData = results[0] as DashboardData;
        final prayerTimes = results[1] as PrayerTimes;
        final hijriDate = results[2] as HijriDate;

        // Cache the fresh data
        await _cacheService.put(
          'dashboard_data',
          dashboardData.toJson(),
          ttl: const Duration(minutes: 15), // Short cache for dynamic data
        );
        await _cacheService.put(
          'prayer_times_${latitude}_$longitude',
          prayerTimes.toJson(),
          ttl: const Duration(hours: 24), // Daily cache for prayer times
        );
        await _cacheService.put(
          'hijri_date',
          hijriDate.toJson(),
          ttl: const Duration(hours: 24), // Daily cache for Hijri date
        );

        state = state.copyWith(
          dashboardData: dashboardData,
          prayerTimes: prayerTimes,
          hijriDate: hijriDate,
          isLoading: false,
        );
      } else if (cachedDashboard == null && cachedPrayerTimes == null) {
        // No cache and offline
        throw AppError(
          type: ErrorType.network,
          message: 'لا يوجد اتصال بالإنترنت',
        );
      }
    } catch (e) {
      _errorHandler.handleError(e);
      state = state.copyWith(
        isLoading: false,
        error: AppError.fromException(e).userFriendlyMessage,
      );
    }
  }

  /// Refresh dashboard data
  Future<void> refresh({
    required double latitude,
    required double longitude,
  }) async {
    await loadDashboardData(latitude: latitude, longitude: longitude);
  }

  /// Update daily wird progress with optimistic updates
  Future<void> updateWirdProgress({
    required int pageNumber,
    required bool completed,
  }) async {
    try {
      if (_isOnline) {
        final updatedWird = await _dashboardService.updateDailyWird(
          pageNumber: pageNumber,
          completed: completed,
        );

        if (state.dashboardData != null) {
          final updatedDashboard = DashboardData(
            dailyWird: updatedWird,
            dailyContent: state.dashboardData!.dailyContent,
            lastUpdated: DateTime.now(),
          );
          
          // Update cache
          await _cacheService.put(
            'dashboard_data',
            updatedDashboard.toJson(),
          );
          
          state = state.copyWith(dashboardData: updatedDashboard);
        }
      } else {
        // Queue for offline processing
        await _offlineManager.queueOperation('update_wird_progress', {
          'page_number': pageNumber,
          'completed': completed,
        });
        
        // Optimistic update
        if (state.dashboardData != null) {
          final optimisticWird = state.dashboardData!.dailyWird.copyWith(
            completedPages: completed
                ? [...state.dashboardData!.dailyWird.completedPages, pageNumber]
                : state.dashboardData!.dailyWird.completedPages
                    .where((p) => p != pageNumber)
                    .toList(),
          );
          
          final updatedDashboard = DashboardData(
            dailyWird: optimisticWird,
            dailyContent: state.dashboardData!.dailyContent,
            lastUpdated: DateTime.now(),
          );
          
          state = state.copyWith(dashboardData: updatedDashboard);
        }
      }
    } catch (e) {
      _errorHandler.handleError(e);
      state = state.copyWith(error: AppError.fromException(e).userFriendlyMessage);
    }
  }
}

/// Provider for dashboard notifier with integrated state management
final dashboardNotifierProvider =
    StateNotifierProvider<DashboardNotifier, DashboardState>((ref) {
  final dashboardService = ref.watch(dashboardServiceProvider);
  final prayerTimesService = ref.watch(prayerTimesServiceProvider);
  final cacheService = ref.watch(configuredCacheServiceProvider);
  final offlineManager = ref.watch(offlineManagerProvider.notifier);
  final errorHandler = ref.watch(errorHandlerProvider.notifier);
  final isOnline = ref.watch(isOnlineProvider);
  
  return DashboardNotifier(
    dashboardService,
    prayerTimesService,
    cacheService,
    offlineManager,
    errorHandler,
    isOnline,
  );
});

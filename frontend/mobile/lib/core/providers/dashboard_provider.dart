import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/dashboard_service.dart';
import '../services/prayer_times_service.dart';
import '../network/dio_client.dart';

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

  DashboardNotifier(this._dashboardService, this._prayerTimesService)
      : super(DashboardState());

  /// Load all dashboard data
  Future<void> loadDashboardData({
    required double latitude,
    required double longitude,
  }) async {
    state = state.copyWith(isLoading: true, error: null);

    try {
      // Load all data in parallel
      final results = await Future.wait([
        _dashboardService.getDashboardData(),
        _prayerTimesService.getPrayerTimes(
          latitude: latitude,
          longitude: longitude,
        ),
        _prayerTimesService.getHijriDate(),
      ]);

      state = state.copyWith(
        dashboardData: results[0] as DashboardData,
        prayerTimes: results[1] as PrayerTimes,
        hijriDate: results[2] as HijriDate,
        isLoading: false,
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
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

  /// Update daily wird progress
  Future<void> updateWirdProgress({
    required int pageNumber,
    required bool completed,
  }) async {
    try {
      final updatedWird = await _dashboardService.updateDailyWird(
        pageNumber: pageNumber,
        completed: completed,
      );

      if (state.dashboardData != null) {
        state = state.copyWith(
          dashboardData: DashboardData(
            dailyWird: updatedWird,
            dailyContent: state.dashboardData!.dailyContent,
            lastUpdated: DateTime.now(),
          ),
        );
      }
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }
}

/// Provider for dashboard notifier
final dashboardNotifierProvider =
    StateNotifierProvider<DashboardNotifier, DashboardState>((ref) {
  final dashboardService = ref.watch(dashboardServiceProvider);
  final prayerTimesService = ref.watch(prayerTimesServiceProvider);
  return DashboardNotifier(dashboardService, prayerTimesService);
});

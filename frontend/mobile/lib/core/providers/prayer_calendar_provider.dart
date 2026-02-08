import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/prayer_calendar_service.dart';
import '../network/dio_client.dart';
import '../../features/prayer_calendar/data/models/calendar_day_model.dart';

/// Provider for DioClient
final dioClientProvider = Provider<DioClient>((ref) {
  return DioClient();
});

/// Provider for PrayerCalendarService
final prayerCalendarServiceProvider = Provider<PrayerCalendarService>((ref) {
  final dioClient = ref.watch(dioClientProvider);
  return PrayerCalendarService(dioClient);
});

/// State for monthly calendar
class MonthlyCalendarState {
  final MonthlyCalendarModel? calendar;
  final bool isLoading;
  final String? error;

  MonthlyCalendarState({
    this.calendar,
    this.isLoading = false,
    this.error,
  });

  MonthlyCalendarState copyWith({
    MonthlyCalendarModel? calendar,
    bool? isLoading,
    String? error,
  }) {
    return MonthlyCalendarState(
      calendar: calendar ?? this.calendar,
      isLoading: isLoading ?? this.isLoading,
      error: error,
    );
  }
}

/// Notifier for monthly calendar
class MonthlyCalendarNotifier extends StateNotifier<MonthlyCalendarState> {
  final PrayerCalendarService _service;
  double? _latitude;
  double? _longitude;
  String? _calculationMethod;

  MonthlyCalendarNotifier(this._service) : super(MonthlyCalendarState());

  /// Set location for prayer times calculation
  void setLocation(double latitude, double longitude) {
    _latitude = latitude;
    _longitude = longitude;
  }

  /// Set calculation method
  void setCalculationMethod(String method) {
    _calculationMethod = method;
  }

  /// Load monthly calendar
  Future<void> loadMonthlyCalendar(int hijriYear, int hijriMonth) async {
    if (_latitude == null || _longitude == null) {
      state = state.copyWith(
        error: 'Location not set. Please enable location services.',
      );
      return;
    }

    state = state.copyWith(isLoading: true, error: null);

    try {
      final calendar = await _service.getMonthlyCalendar(
        latitude: _latitude!,
        longitude: _longitude!,
        hijriYear: hijriYear,
        hijriMonth: hijriMonth,
        calculationMethod: _calculationMethod,
      );

      state = state.copyWith(
        calendar: calendar,
        isLoading: false,
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  /// Navigate to next month
  Future<void> nextMonth() async {
    if (state.calendar == null) return;

    final currentMonth = state.calendar!.hijriMonth.monthNumber;
    final currentYear = state.calendar!.hijriYear;

    int nextMonth = currentMonth + 1;
    int nextYear = currentYear;

    if (nextMonth > 12) {
      nextMonth = 1;
      nextYear++;
    }

    await loadMonthlyCalendar(nextYear, nextMonth);
  }

  /// Navigate to previous month
  Future<void> previousMonth() async {
    if (state.calendar == null) return;

    final currentMonth = state.calendar!.hijriMonth.monthNumber;
    final currentYear = state.calendar!.hijriYear;

    int prevMonth = currentMonth - 1;
    int prevYear = currentYear;

    if (prevMonth < 1) {
      prevMonth = 12;
      prevYear--;
    }

    await loadMonthlyCalendar(prevYear, prevMonth);
  }

  /// Export calendar to iCal
  Future<String> exportToICal() async {
    if (state.calendar == null || _latitude == null || _longitude == null) {
      throw Exception('Calendar not loaded or location not set');
    }

    return await _service.exportCalendarToICal(
      latitude: _latitude!,
      longitude: _longitude!,
      hijriYear: state.calendar!.hijriYear,
      hijriMonth: state.calendar!.hijriMonth.monthNumber,
    );
  }

  /// Get shareable link
  Future<String> getShareableLink() async {
    if (state.calendar == null || _latitude == null || _longitude == null) {
      throw Exception('Calendar not loaded or location not set');
    }

    return await _service.getShareableLink(
      latitude: _latitude!,
      longitude: _longitude!,
      hijriYear: state.calendar!.hijriYear,
      hijriMonth: state.calendar!.hijriMonth.monthNumber,
    );
  }
}

/// Provider for monthly calendar
final monthlyCalendarProvider =
    StateNotifierProvider<MonthlyCalendarNotifier, MonthlyCalendarState>((ref) {
  final service = ref.watch(prayerCalendarServiceProvider);
  return MonthlyCalendarNotifier(service);
});

/// Provider for Islamic events
final islamicEventsProvider = FutureProvider.autoDispose
    .family<List<IslamicEventModel>, Map<String, int?>>((ref, params) async {
  final service = ref.watch(prayerCalendarServiceProvider);
  return await service.getIslamicEvents(
    hijriMonth: params['hijriMonth'],
    hijriYear: params['hijriYear'],
    importanceLevel: params['importanceLevel'],
  );
});

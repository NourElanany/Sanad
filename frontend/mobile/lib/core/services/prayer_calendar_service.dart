import 'package:dio/dio.dart';
import '../network/dio_client.dart';
import '../network/api_endpoints.dart';
import '../../features/prayer_calendar/data/models/calendar_day_model.dart';

/// Prayer Calendar Service for monthly prayer times
class PrayerCalendarService {
  final DioClient _dioClient;

  PrayerCalendarService(this._dioClient);

  /// Get monthly prayer times calendar
  Future<MonthlyCalendarModel> getMonthlyCalendar({
    required double latitude,
    required double longitude,
    required int hijriYear,
    required int hijriMonth,
    String? calculationMethod,
  }) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.prayerCalendar}/$hijriYear/$hijriMonth',
        queryParameters: {
          'latitude': latitude,
          'longitude': longitude,
          if (calculationMethod != null) 'method': calculationMethod,
        },
      );

      return MonthlyCalendarModel.fromJson(response.data['data']);
    } catch (e) {
      throw Exception('Failed to fetch monthly calendar: $e');
    }
  }

  /// Get prayer times for a specific date range
  Future<List<CalendarDayModel>> getPrayerTimesRange({
    required double latitude,
    required double longitude,
    required DateTime startDate,
    required DateTime endDate,
    String? calculationMethod,
  }) async {
    try {
      final response = await _dioClient.get(
        ApiEndpoints.prayerTimesRange,
        queryParameters: {
          'latitude': latitude,
          'longitude': longitude,
          'start_date': startDate.toIso8601String().split('T')[0],
          'end_date': endDate.toIso8601String().split('T')[0],
          if (calculationMethod != null) 'method': calculationMethod,
        },
      );

      return (response.data['data'] as List)
          .map((json) => CalendarDayModel.fromJson(json))
          .toList();
    } catch (e) {
      throw Exception('Failed to fetch prayer times range: $e');
    }
  }

  /// Get Islamic events for a specific month
  Future<List<IslamicEventModel>> getIslamicEvents({
    int? hijriMonth,
    int? hijriYear,
    int? importanceLevel,
  }) async {
    try {
      final response = await _dioClient.get(
        ApiEndpoints.islamicEvents,
        queryParameters: {
          if (hijriMonth != null) 'hijri_month': hijriMonth,
          if (hijriYear != null) 'hijri_year': hijriYear,
          if (importanceLevel != null) 'importance_level': importanceLevel,
        },
      );

      return (response.data['data'] as List)
          .map((json) => IslamicEventModel.fromJson(json))
          .toList();
    } catch (e) {
      throw Exception('Failed to fetch Islamic events: $e');
    }
  }

  /// Export calendar to iCal format
  Future<String> exportCalendarToICal({
    required double latitude,
    required double longitude,
    required int hijriYear,
    required int hijriMonth,
  }) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.prayerCalendar}/$hijriYear/$hijriMonth/export',
        queryParameters: {
          'latitude': latitude,
          'longitude': longitude,
          'format': 'ical',
        },
        options: Options(responseType: ResponseType.plain),
      );

      return response.data as String;
    } catch (e) {
      throw Exception('Failed to export calendar: $e');
    }
  }

  /// Get shareable calendar link
  Future<String> getShareableLink({
    required double latitude,
    required double longitude,
    required int hijriYear,
    required int hijriMonth,
  }) async {
    try {
      final response = await _dioClient.post(
        '${ApiEndpoints.prayerCalendar}/share',
        data: {
          'latitude': latitude,
          'longitude': longitude,
          'hijri_year': hijriYear,
          'hijri_month': hijriMonth,
        },
      );

      return response.data['data']['share_url'] as String;
    } catch (e) {
      throw Exception('Failed to generate shareable link: $e');
    }
  }
}

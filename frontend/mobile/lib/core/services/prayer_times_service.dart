import 'package:dio/dio.dart';
import '../network/dio_client.dart';
import '../network/api_endpoints.dart';

/// Model for Prayer Times
class PrayerTimes {
  final String fajr;
  final String sunrise;
  final String dhuhr;
  final String asr;
  final String maghrib;
  final String isha;
  final DateTime date;
  final String location;

  PrayerTimes({
    required this.fajr,
    required this.sunrise,
    required this.dhuhr,
    required this.asr,
    required this.maghrib,
    required this.isha,
    required this.date,
    required this.location,
  });

  factory PrayerTimes.fromJson(Map<String, dynamic> json) {
    return PrayerTimes(
      fajr: json['fajr'] ?? '',
      sunrise: json['sunrise'] ?? '',
      dhuhr: json['dhuhr'] ?? '',
      asr: json['asr'] ?? '',
      maghrib: json['maghrib'] ?? '',
      isha: json['isha'] ?? '',
      date: DateTime.parse(json['date'] ?? DateTime.now().toIso8601String()),
      location: json['location'] ?? '',
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'fajr': fajr,
      'sunrise': sunrise,
      'dhuhr': dhuhr,
      'asr': asr,
      'maghrib': maghrib,
      'isha': isha,
      'date': date.toIso8601String(),
      'location': location,
    };
  }

  /// Get the next prayer name and time
  Map<String, String> getNextPrayer() {
    final now = DateTime.now();
    final prayers = [
      {'name': 'الفجر', 'time': fajr},
      {'name': 'الشروق', 'time': sunrise},
      {'name': 'الظهر', 'time': dhuhr},
      {'name': 'العصر', 'time': asr},
      {'name': 'المغرب', 'time': maghrib},
      {'name': 'العشاء', 'time': isha},
    ];

    for (var prayer in prayers) {
      final prayerTime = _parseTime(prayer['time']!);
      if (prayerTime.isAfter(now)) {
        return {'name': prayer['name']!, 'time': prayer['time']!};
      }
    }

    // If all prayers passed, return Fajr of next day
    return {'name': 'الفجر', 'time': fajr};
  }

  DateTime _parseTime(String time) {
    final parts = time.split(':');
    final now = DateTime.now();
    return DateTime(
      now.year,
      now.month,
      now.day,
      int.parse(parts[0]),
      int.parse(parts[1]),
    );
  }
}

/// Model for Hijri Date
class HijriDate {
  final int day;
  final int month;
  final int year;
  final String monthName;
  final String weekday;

  HijriDate({
    required this.day,
    required this.month,
    required this.year,
    required this.monthName,
    required this.weekday,
  });

  factory HijriDate.fromJson(Map<String, dynamic> json) {
    return HijriDate(
      day: json['day'] ?? 1,
      month: json['month'] ?? 1,
      year: json['year'] ?? 1445,
      monthName: json['month_name'] ?? '',
      weekday: json['weekday'] ?? '',
    );
  }

  String get formatted => '$weekday، $day $monthName $year هـ';
}

/// Prayer Times Service
class PrayerTimesService {
  final DioClient _dioClient;

  PrayerTimesService(this._dioClient);

  /// Get prayer times for a specific location
  Future<PrayerTimes> getPrayerTimes({
    required double latitude,
    required double longitude,
    String? madhab,
  }) async {
    try {
      final response = await _dioClient.get(
        ApiEndpoints.prayerTimes,
        queryParameters: {
          'latitude': latitude,
          'longitude': longitude,
          if (madhab != null) 'madhab': madhab,
        },
      );

      return PrayerTimes.fromJson(response.data);
    } catch (e) {
      throw Exception('Failed to fetch prayer times: $e');
    }
  }

  /// Get Hijri date for today
  Future<HijriDate> getHijriDate() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.hijriDate);
      return HijriDate.fromJson(response.data);
    } catch (e) {
      throw Exception('Failed to fetch Hijri date: $e');
    }
  }

  /// Get monthly prayer times
  Future<List<PrayerTimes>> getMonthlyPrayerTimes({
    required double latitude,
    required double longitude,
    required int month,
    required int year,
    String? madhab,
  }) async {
    try {
      final response = await _dioClient.get(
        ApiEndpoints.monthlyPrayerTimes,
        queryParameters: {
          'latitude': latitude,
          'longitude': longitude,
          'month': month,
          'year': year,
          if (madhab != null) 'madhab': madhab,
        },
      );

      return (response.data as List)
          .map((json) => PrayerTimes.fromJson(json))
          .toList();
    } catch (e) {
      throw Exception('Failed to fetch monthly prayer times: $e');
    }
  }
}

import 'package:dio/dio.dart';
import '../network/dio_client.dart';
import '../network/api_endpoints.dart';

/// Model for Daily Wird (Daily Reading)
class DailyWird {
  final int totalPages;
  final int completedPages;
  final double progressPercentage;
  final List<int> completedPageNumbers;

  DailyWird({
    required this.totalPages,
    required this.completedPages,
    required this.progressPercentage,
    required this.completedPageNumbers,
  });

  factory DailyWird.fromJson(Map<String, dynamic> json) {
    return DailyWird(
      totalPages: json['total_pages'] ?? 10,
      completedPages: json['completed_pages'] ?? 0,
      progressPercentage: (json['progress_percentage'] ?? 0.0).toDouble(),
      completedPageNumbers: List<int>.from(json['completed_page_numbers'] ?? []),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'total_pages': totalPages,
      'completed_pages': completedPages,
      'progress_percentage': progressPercentage,
      'completed_page_numbers': completedPageNumbers,
    };
  }
}

/// Model for Daily Verse/Hadith
class DailyContent {
  final String id;
  final String type; // 'verse' or 'hadith'
  final String arabicText;
  final String translation;
  final String reference;
  final String? tafsir;

  DailyContent({
    required this.id,
    required this.type,
    required this.arabicText,
    required this.translation,
    required this.reference,
    this.tafsir,
  });

  factory DailyContent.fromJson(Map<String, dynamic> json) {
    return DailyContent(
      id: json['id'] ?? '',
      type: json['type'] ?? 'verse',
      arabicText: json['arabic_text'] ?? '',
      translation: json['translation'] ?? '',
      reference: json['reference'] ?? '',
      tafsir: json['tafsir'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'type': type,
      'arabic_text': arabicText,
      'translation': translation,
      'reference': reference,
      if (tafsir != null) 'tafsir': tafsir,
    };
  }
}

/// Model for Dashboard Data
class DashboardData {
  final DailyWird dailyWird;
  final DailyContent dailyContent;
  final DateTime lastUpdated;

  DashboardData({
    required this.dailyWird,
    required this.dailyContent,
    required this.lastUpdated,
  });

  factory DashboardData.fromJson(Map<String, dynamic> json) {
    return DashboardData(
      dailyWird: DailyWird.fromJson(json['daily_wird'] ?? {}),
      dailyContent: DailyContent.fromJson(json['daily_content'] ?? {}),
      lastUpdated: DateTime.parse(
        json['last_updated'] ?? DateTime.now().toIso8601String(),
      ),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'daily_wird': dailyWird.toJson(),
      'daily_content': dailyContent.toJson(),
      'last_updated': lastUpdated.toIso8601String(),
    };
  }
}

/// Dashboard Service
class DashboardService {
  final DioClient _dioClient;

  DashboardService(this._dioClient);

  /// Get dashboard data
  Future<DashboardData> getDashboardData() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.dashboard);
      return DashboardData.fromJson(response.data);
    } catch (e) {
      throw Exception('Failed to fetch dashboard data: $e');
    }
  }

  /// Get daily wird progress
  Future<DailyWird> getDailyWird() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.dailyWird);
      return DailyWird.fromJson(response.data);
    } catch (e) {
      throw Exception('Failed to fetch daily wird: $e');
    }
  }

  /// Update daily wird progress
  Future<DailyWird> updateDailyWird({
    required int pageNumber,
    required bool completed,
  }) async {
    try {
      final response = await _dioClient.post(
        ApiEndpoints.updateDailyWird,
        data: {
          'page_number': pageNumber,
          'completed': completed,
        },
      );
      return DailyWird.fromJson(response.data);
    } catch (e) {
      throw Exception('Failed to update daily wird: $e');
    }
  }

  /// Get daily content (verse or hadith)
  Future<DailyContent> getDailyContent() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.dailyContent);
      return DailyContent.fromJson(response.data);
    } catch (e) {
      throw Exception('Failed to fetch daily content: $e');
    }
  }

  /// Get user statistics
  Future<Map<String, dynamic>> getUserStatistics() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.userStatistics);
      return response.data;
    } catch (e) {
      throw Exception('Failed to fetch user statistics: $e');
    }
  }
}

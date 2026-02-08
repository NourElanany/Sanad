import 'package:dio/dio.dart';
import '../../features/statistics/data/models/statistics_model.dart';
import '../network/dio_client.dart';
import '../network/api_endpoints.dart';

/// Service for fetching and managing statistics data
class StatisticsService {
  final DioClient _dioClient;

  StatisticsService(this._dioClient);

  /// Get comprehensive statistics dashboard
  Future<StatisticsDashboard> getStatisticsDashboard({
    int? timePeriodDays,
  }) async {
    try {
      final queryParams = <String, dynamic>{};
      if (timePeriodDays != null) {
        queryParams['time_period_days'] = timePeriodDays;
      }

      final response = await _dioClient.get(
        ApiEndpoints.statisticsDashboard,
        queryParameters: queryParams,
      );

      return StatisticsDashboard.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to fetch statistics dashboard: $e');
    }
  }

  /// Get Khatma statistics
  Future<KhatmaStatistics> getKhatmaStatistics() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.khatmaStatistics);
      return KhatmaStatistics.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to fetch Khatma statistics: $e');
    }
  }

  /// Get reading statistics
  Future<ReadingStatistics> getReadingStatistics({
    int? days,
  }) async {
    try {
      final queryParams = <String, dynamic>{};
      if (days != null) {
        queryParams['days'] = days;
      }

      final response = await _dioClient.get(
        ApiEndpoints.readingStatistics,
        queryParameters: queryParams,
      );

      return ReadingStatistics.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to fetch reading statistics: $e');
    }
  }

  /// Get recitation improvement statistics
  Future<RecitationStatistics> getRecitationStatistics() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.recitationStatistics);
      return RecitationStatistics.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to fetch recitation statistics: $e');
    }
  }

  /// Get weekly comparison data
  Future<WeeklyComparison> getWeeklyComparison() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.weeklyComparison);
      return WeeklyComparison.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to fetch weekly comparison: $e');
    }
  }

  /// Get monthly comparison data
  Future<MonthlyComparison> getMonthlyComparison() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.monthlyComparison);
      return MonthlyComparison.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to fetch monthly comparison: $e');
    }
  }

  /// Get personal goals
  Future<List<PersonalGoal>> getPersonalGoals() async {
    try {
      final response = await _dioClient.get(ApiEndpoints.personalGoals);
      return (response.data as List<dynamic>)
          .map((e) => PersonalGoal.fromJson(e as Map<String, dynamic>))
          .toList();
    } catch (e) {
      throw Exception('Failed to fetch personal goals: $e');
    }
  }

  /// Create a new personal goal
  Future<PersonalGoal> createGoal(CreateGoalRequest request) async {
    try {
      final response = await _dioClient.post(
        ApiEndpoints.personalGoals,
        data: request.toJson(),
      );

      return PersonalGoal.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to create goal: $e');
    }
  }

  /// Update goal progress
  Future<PersonalGoal> updateGoalProgress(
    String goalId,
    int currentValue,
  ) async {
    try {
      final response = await _dioClient.put(
        '${ApiEndpoints.personalGoals}/$goalId',
        data: {'current_value': currentValue},
      );

      return PersonalGoal.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to update goal progress: $e');
    }
  }

  /// Delete a personal goal
  Future<void> deleteGoal(String goalId) async {
    try {
      await _dioClient.delete('${ApiEndpoints.personalGoals}/$goalId');
    } catch (e) {
      throw Exception('Failed to delete goal: $e');
    }
  }

  /// Get daily reading minutes for a specific date range
  Future<List<DailyReadingData>> getDailyReadingData({
    required DateTime startDate,
    required DateTime endDate,
  }) async {
    try {
      final response = await _dioClient.get(
        ApiEndpoints.dailyReadingData,
        queryParameters: {
          'start_date': startDate.toIso8601String(),
          'end_date': endDate.toIso8601String(),
        },
      );

      return (response.data as List<dynamic>)
          .map((e) => DailyReadingData.fromJson(e as Map<String, dynamic>))
          .toList();
    } catch (e) {
      throw Exception('Failed to fetch daily reading data: $e');
    }
  }

  /// Get recitation score history
  Future<List<RecitationScoreData>> getRecitationScoreHistory({
    int? limit,
  }) async {
    try {
      final queryParams = <String, dynamic>{};
      if (limit != null) {
        queryParams['limit'] = limit;
      }

      final response = await _dioClient.get(
        ApiEndpoints.recitationScoreHistory,
        queryParameters: queryParams,
      );

      return (response.data as List<dynamic>)
          .map((e) => RecitationScoreData.fromJson(e as Map<String, dynamic>))
          .toList();
    } catch (e) {
      throw Exception('Failed to fetch recitation score history: $e');
    }
  }
}

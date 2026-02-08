import 'package:dio/dio.dart';
import '../../features/achievements/data/models/achievement_model.dart';
import '../network/dio_client.dart';
import '../network/api_endpoints.dart';

/// Service for managing achievements, badges, levels, and challenges
class AchievementsService {
  final DioClient _dioClient;

  AchievementsService(this._dioClient);

  /// Get achievements dashboard with all data
  Future<AchievementsDashboard> getAchievementsDashboard() async {
    try {
      final response = await _dioClient.get(
        ApiEndpoints.achievementsDashboard,
      );
      return AchievementsDashboard.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Get all achievements (locked and unlocked)
  Future<List<Achievement>> getAllAchievements({
    AchievementCategory? category,
    AchievementTier? tier,
    bool? isUnlocked,
  }) async {
    try {
      final queryParams = <String, dynamic>{};
      if (category != null) {
        queryParams['category'] = category.toString().split('.').last;
      }
      if (tier != null) {
        queryParams['tier'] = tier.toString().split('.').last;
      }
      if (isUnlocked != null) {
        queryParams['is_unlocked'] = isUnlocked;
      }

      final response = await _dioClient.get(
        ApiEndpoints.achievements,
        queryParameters: queryParams,
      );
      
      return (response.data as List<dynamic>)
          .map((json) => Achievement.fromJson(json as Map<String, dynamic>))
          .toList();
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Get specific achievement details
  Future<Achievement> getAchievement(String achievementId) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.achievements}/$achievementId',
      );
      return Achievement.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Get user level and points information
  Future<UserLevel> getUserLevel() async {
    try {
      final response = await _dioClient.get(
        ApiEndpoints.userLevel,
      );
      return UserLevel.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Get active challenges (daily and weekly)
  Future<List<Challenge>> getActiveChallenges({
    ChallengeType? type,
  }) async {
    try {
      final queryParams = <String, dynamic>{};
      if (type != null) {
        queryParams['type'] = type.toString().split('.').last;
      }

      final response = await _dioClient.get(
        ApiEndpoints.challenges,
        queryParameters: queryParams,
      );
      
      return (response.data as List<dynamic>)
          .map((json) => Challenge.fromJson(json as Map<String, dynamic>))
          .toList();
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Get specific challenge details
  Future<Challenge> getChallenge(String challengeId) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.challenges}/$challengeId',
      );
      return Challenge.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Update challenge progress
  Future<Challenge> updateChallengeProgress(
    String challengeId,
    int progressValue,
  ) async {
    try {
      final response = await _dioClient.post(
        '${ApiEndpoints.challenges}/$challengeId/progress',
        data: {
          'progress_value': progressValue,
        },
      );
      return Challenge.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Get achievement statistics
  Future<AchievementStats> getAchievementStats() async {
    try {
      final response = await _dioClient.get(
        ApiEndpoints.achievementStats,
      );
      return AchievementStats.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Get motivational reminders
  Future<List<MotivationalReminder>> getReminders({
    bool? isActive,
  }) async {
    try {
      final queryParams = <String, dynamic>{};
      if (isActive != null) {
        queryParams['is_active'] = isActive;
      }

      final response = await _dioClient.get(
        ApiEndpoints.achievementReminders,
        queryParameters: queryParams,
      );
      
      return (response.data as List<dynamic>)
          .map((json) => MotivationalReminder.fromJson(json as Map<String, dynamic>))
          .toList();
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Create or update a reminder
  Future<MotivationalReminder> saveReminder(MotivationalReminder reminder) async {
    try {
      final response = await _dioClient.post(
        ApiEndpoints.achievementReminders,
        data: reminder.toJson(),
      );
      return MotivationalReminder.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Delete a reminder
  Future<void> deleteReminder(String reminderId) async {
    try {
      await _dioClient.delete(
        '${ApiEndpoints.achievementReminders}/$reminderId',
      );
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Share achievement on social media
  Future<Map<String, dynamic>> shareAchievement(
    ShareAchievementRequest request,
  ) async {
    try {
      final response = await _dioClient.post(
        ApiEndpoints.shareAchievement,
        data: request.toJson(),
      );
      return response.data as Map<String, dynamic>;
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Get achievement unlock history
  Future<List<AchievementUnlockNotification>> getUnlockHistory({
    int? limit,
    DateTime? since,
  }) async {
    try {
      final queryParams = <String, dynamic>{};
      if (limit != null) {
        queryParams['limit'] = limit;
      }
      if (since != null) {
        queryParams['since'] = since.toIso8601String();
      }

      final response = await _dioClient.get(
        ApiEndpoints.achievementUnlockHistory,
        queryParameters: queryParams,
      );
      
      return (response.data as List<dynamic>)
          .map((json) => AchievementUnlockNotification.fromJson(json as Map<String, dynamic>))
          .toList();
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Manually trigger achievement check (for testing/debugging)
  Future<List<Achievement>> checkAchievements() async {
    try {
      final response = await _dioClient.post(
        ApiEndpoints.checkAchievements,
      );
      
      return (response.data as List<dynamic>)
          .map((json) => Achievement.fromJson(json as Map<String, dynamic>))
          .toList();
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  /// Get leaderboard (if social features are enabled)
  Future<List<Map<String, dynamic>>> getLeaderboard({
    String? timeframe, // 'daily', 'weekly', 'monthly', 'all_time'
    int? limit,
  }) async {
    try {
      final queryParams = <String, dynamic>{};
      if (timeframe != null) {
        queryParams['timeframe'] = timeframe;
      }
      if (limit != null) {
        queryParams['limit'] = limit;
      }

      final response = await _dioClient.get(
        ApiEndpoints.achievementLeaderboard,
        queryParameters: queryParams,
      );
      
      return (response.data as List<dynamic>)
          .map((json) => json as Map<String, dynamic>)
          .toList();
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  Exception _handleError(DioException error) {
    if (error.response != null) {
      final statusCode = error.response!.statusCode;
      final message = error.response!.data['message'] ?? 'Unknown error occurred';
      
      switch (statusCode) {
        case 400:
          return Exception('Bad request: $message');
        case 401:
          return Exception('Unauthorized: Please login again');
        case 404:
          return Exception('Not found: $message');
        case 500:
          return Exception('Server error: $message');
        default:
          return Exception('Error: $message');
      }
    } else {
      return Exception('Network error: ${error.message}');
    }
  }
}

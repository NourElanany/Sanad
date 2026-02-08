import 'package:dio/dio.dart';
import '../network/dio_client.dart';
import '../network/api_endpoints.dart';
import '../../features/stories/data/models/story_model.dart';

/// Service for managing Islamic stories
class StoriesService {
  final DioClient _dioClient;

  StoriesService(this._dioClient);

  /// Get stories by category
  Future<List<StoryModel>> getStoriesByCategory(
    StoryCategory category, {
    int? limit,
    int? offset,
  }) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.stories}/category/${category.name}',
        queryParameters: {
          if (limit != null) 'limit': limit,
          if (offset != null) 'offset': offset,
        },
      );

      final data = response.data['data'] as List;
      return data.map((json) => StoryModel.fromJson(json)).toList();
    } catch (e) {
      throw Exception('Failed to get stories by category: $e');
    }
  }

  /// Get a single story by ID
  Future<StoryWithDetailsModel> getStory(
    String storyId, {
    bool includeDetails = true,
  }) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.stories}/$storyId',
        queryParameters: {
          'include_details': includeDetails,
        },
      );

      return StoryWithDetailsModel.fromJson(response.data['data']);
    } catch (e) {
      throw Exception('Failed to get story: $e');
    }
  }

  /// Search stories
  Future<List<StoryModel>> searchStories(
    String query, {
    List<StoryCategory>? categories,
    List<AgeGroup>? ageGroups,
    List<AuthenticityLevel>? authenticityLevels,
    int? limit,
    int? offset,
  }) async {
    try {
      final response = await _dioClient.get(
        ApiEndpoints.stories,
        queryParameters: {
          'query': query,
          if (categories != null)
            'categories': categories.map((c) => c.name).join(','),
          if (ageGroups != null)
            'age_groups': ageGroups.map((a) => a.name).join(','),
          if (authenticityLevels != null)
            'authenticity_levels':
                authenticityLevels.map((a) => a.name).join(','),
          if (limit != null) 'limit': limit,
          if (offset != null) 'offset': offset,
        },
      );

      final results = response.data['data']['results'] as List;
      return results.map((json) => StoryModel.fromJson(json['story'])).toList();
    } catch (e) {
      throw Exception('Failed to search stories: $e');
    }
  }

  /// Get stories by character
  Future<List<StoryModel>> getStoriesByCharacter(
    String characterName, {
    CharacterType? characterType,
    int? limit,
    int? offset,
  }) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.stories}/character/$characterName',
        queryParameters: {
          if (characterType != null) 'character_type': characterType.name,
          if (limit != null) 'limit': limit,
          if (offset != null) 'offset': offset,
        },
      );

      final stories = response.data['data']['stories'] as List;
      return stories
          .map((json) => StoryModel.fromJson(json['story']))
          .toList();
    } catch (e) {
      throw Exception('Failed to get stories by character: $e');
    }
  }

  /// Get stories by theme
  Future<List<StoryModel>> getStoriesByTheme(
    String theme, {
    LessonType? lessonType,
    MoralCategory? moralCategory,
    AgeGroup? ageGroup,
    int? limit,
    int? offset,
  }) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.stories}/theme/$theme',
        queryParameters: {
          if (lessonType != null) 'lesson_type': lessonType.name,
          if (moralCategory != null) 'moral_category': moralCategory.name,
          if (ageGroup != null) 'age_group': ageGroup.name,
          if (limit != null) 'limit': limit,
          if (offset != null) 'offset': offset,
        },
      );

      final stories = response.data['data']['stories'] as List;
      return stories
          .map((json) => StoryModel.fromJson(json['story']))
          .toList();
    } catch (e) {
      throw Exception('Failed to get stories by theme: $e');
    }
  }

  /// Get story lessons
  Future<List<LessonInStoryModel>> getStoryLessons(String storyId) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.stories}/$storyId/lessons',
      );

      final lessons = response.data['data'] as List;
      return lessons.map((json) => LessonInStoryModel.fromJson(json)).toList();
    } catch (e) {
      throw Exception('Failed to get story lessons: $e');
    }
  }

  /// Get story sources
  Future<List<StorySourceModel>> getStorySources(String storyId) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.stories}/$storyId/sources',
      );

      final sources = response.data['data'] as List;
      return sources.map((json) => StorySourceModel.fromJson(json)).toList();
    } catch (e) {
      throw Exception('Failed to get story sources: $e');
    }
  }

  /// Get characters
  Future<List<CharacterModel>> searchCharacters(
    String query, {
    CharacterType? characterType,
    TimePeriod? historicalPeriod,
    int? limit,
    int? offset,
  }) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.stories}/characters/search',
        queryParameters: {
          'query': query,
          if (characterType != null) 'character_type': characterType.name,
          if (historicalPeriod != null)
            'historical_period': historicalPeriod.name,
          if (limit != null) 'limit': limit,
          if (offset != null) 'offset': offset,
        },
      );

      final characters = response.data['data'] as List;
      return characters.map((json) => CharacterModel.fromJson(json)).toList();
    } catch (e) {
      throw Exception('Failed to search characters: $e');
    }
  }

  /// Get category statistics
  Future<Map<String, int>> getCategoryStatistics() async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.stories}/analytics/categories',
      );

      final stats = response.data['data'] as Map<String, dynamic>;
      return stats.map((key, value) => MapEntry(key, value as int));
    } catch (e) {
      throw Exception('Failed to get category statistics: $e');
    }
  }

  /// Verify story integrity
  Future<bool> verifyStoryIntegrity(String storyId) async {
    try {
      final response = await _dioClient.get(
        '${ApiEndpoints.stories}/$storyId/integrity',
      );

      return response.data['data']['is_valid'] as bool;
    } catch (e) {
      throw Exception('Failed to verify story integrity: $e');
    }
  }
}

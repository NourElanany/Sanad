import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../features/achievements/data/models/achievement_model.dart';
import '../services/achievements_service.dart';
import '../network/dio_client.dart';

/// Provider for DioClient
final dioClientProvider = Provider<DioClient>((ref) {
  return DioClient();
});

/// Provider for AchievementsService
final achievementsServiceProvider = Provider<AchievementsService>((ref) {
  final dioClient = ref.watch(dioClientProvider);
  return AchievementsService(dioClient);
});

/// State for achievements dashboard
class AchievementsDashboardState {
  final AchievementsDashboard? dashboard;
  final bool isLoading;
  final String? error;

  AchievementsDashboardState({
    this.dashboard,
    this.isLoading = false,
    this.error,
  });

  AchievementsDashboardState copyWith({
    AchievementsDashboard? dashboard,
    bool? isLoading,
    String? error,
  }) {
    return AchievementsDashboardState(
      dashboard: dashboard ?? this.dashboard,
      isLoading: isLoading ?? this.isLoading,
      error: error,
    );
  }
}

/// Notifier for achievements dashboard
class AchievementsDashboardNotifier extends StateNotifier<AchievementsDashboardState> {
  final AchievementsService _service;

  AchievementsDashboardNotifier(this._service) : super(AchievementsDashboardState());

  Future<void> loadDashboard() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final dashboard = await _service.getAchievementsDashboard();
      state = state.copyWith(
        dashboard: dashboard,
        isLoading: false,
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  Future<void> refresh() async {
    await loadDashboard();
  }
}

/// Provider for achievements dashboard
final achievementsDashboardProvider = StateNotifierProvider<AchievementsDashboardNotifier, AchievementsDashboardState>((ref) {
  final service = ref.watch(achievementsServiceProvider);
  return AchievementsDashboardNotifier(service);
});

/// State for achievements list
class AchievementsListState {
  final List<Achievement> achievements;
  final bool isLoading;
  final String? error;
  final AchievementCategory? filterCategory;
  final AchievementTier? filterTier;
  final bool? filterUnlocked;

  AchievementsListState({
    this.achievements = const [],
    this.isLoading = false,
    this.error,
    this.filterCategory,
    this.filterTier,
    this.filterUnlocked,
  });

  AchievementsListState copyWith({
    List<Achievement>? achievements,
    bool? isLoading,
    String? error,
    AchievementCategory? filterCategory,
    AchievementTier? filterTier,
    bool? filterUnlocked,
  }) {
    return AchievementsListState(
      achievements: achievements ?? this.achievements,
      isLoading: isLoading ?? this.isLoading,
      error: error,
      filterCategory: filterCategory ?? this.filterCategory,
      filterTier: filterTier ?? this.filterTier,
      filterUnlocked: filterUnlocked ?? this.filterUnlocked,
    );
  }
}

/// Notifier for achievements list
class AchievementsListNotifier extends StateNotifier<AchievementsListState> {
  final AchievementsService _service;

  AchievementsListNotifier(this._service) : super(AchievementsListState());

  Future<void> loadAchievements() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final achievements = await _service.getAllAchievements(
        category: state.filterCategory,
        tier: state.filterTier,
        isUnlocked: state.filterUnlocked,
      );
      state = state.copyWith(
        achievements: achievements,
        isLoading: false,
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  void setFilter({
    AchievementCategory? category,
    AchievementTier? tier,
    bool? isUnlocked,
  }) {
    state = state.copyWith(
      filterCategory: category,
      filterTier: tier,
      filterUnlocked: isUnlocked,
    );
    loadAchievements();
  }

  void clearFilters() {
    state = AchievementsListState();
    loadAchievements();
  }
}

/// Provider for achievements list
final achievementsListProvider = StateNotifierProvider<AchievementsListNotifier, AchievementsListState>((ref) {
  final service = ref.watch(achievementsServiceProvider);
  return AchievementsListNotifier(service);
});

/// State for challenges
class ChallengesState {
  final List<Challenge> challenges;
  final bool isLoading;
  final String? error;

  ChallengesState({
    this.challenges = const [],
    this.isLoading = false,
    this.error,
  });

  ChallengesState copyWith({
    List<Challenge>? challenges,
    bool? isLoading,
    String? error,
  }) {
    return ChallengesState(
      challenges: challenges ?? this.challenges,
      isLoading: isLoading ?? this.isLoading,
      error: error,
    );
  }
}

/// Notifier for challenges
class ChallengesNotifier extends StateNotifier<ChallengesState> {
  final AchievementsService _service;

  ChallengesNotifier(this._service) : super(ChallengesState());

  Future<void> loadChallenges({ChallengeType? type}) async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final challenges = await _service.getActiveChallenges(type: type);
      state = state.copyWith(
        challenges: challenges,
        isLoading: false,
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  Future<void> updateProgress(String challengeId, int progressValue) async {
    try {
      final updatedChallenge = await _service.updateChallengeProgress(
        challengeId,
        progressValue,
      );
      
      // Update the challenge in the list
      final updatedChallenges = state.challenges.map((challenge) {
        if (challenge.id == challengeId) {
          return updatedChallenge;
        }
        return challenge;
      }).toList();
      
      state = state.copyWith(challenges: updatedChallenges);
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  Future<void> refresh() async {
    await loadChallenges();
  }
}

/// Provider for challenges
final challengesProvider = StateNotifierProvider<ChallengesNotifier, ChallengesState>((ref) {
  final service = ref.watch(achievementsServiceProvider);
  return ChallengesNotifier(service);
});

/// Provider for user level
final userLevelProvider = FutureProvider<UserLevel>((ref) async {
  final service = ref.watch(achievementsServiceProvider);
  return service.getUserLevel();
});

/// Provider for achievement stats
final achievementStatsProvider = FutureProvider<AchievementStats>((ref) async {
  final service = ref.watch(achievementsServiceProvider);
  return service.getAchievementStats();
});

/// Provider for motivational reminders
final remindersProvider = FutureProvider<List<MotivationalReminder>>((ref) async {
  final service = ref.watch(achievementsServiceProvider);
  return service.getReminders(isActive: true);
});

/// Provider for unlock history
final unlockHistoryProvider = FutureProvider<List<AchievementUnlockNotification>>((ref) async {
  final service = ref.watch(achievementsServiceProvider);
  return service.getUnlockHistory(limit: 20);
});

/// Provider for leaderboard
final leaderboardProvider = FutureProvider.family<List<Map<String, dynamic>>, String>((ref, timeframe) async {
  final service = ref.watch(achievementsServiceProvider);
  return service.getLeaderboard(timeframe: timeframe, limit: 50);
});

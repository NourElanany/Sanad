import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../features/statistics/data/models/statistics_model.dart';
import '../services/statistics_service.dart';
import '../network/dio_client.dart';

// Provider for StatisticsService
final statisticsServiceProvider = Provider<StatisticsService>((ref) {
  final dioClient = ref.watch(dioClientProvider);
  return StatisticsService(dioClient);
});

// Provider for statistics dashboard
final statisticsDashboardProvider = FutureProvider.autoDispose<StatisticsDashboard>((ref) async {
  final service = ref.watch(statisticsServiceProvider);
  return await service.getStatisticsDashboard();
});

// Provider for Khatma statistics
final khatmaStatisticsProvider = FutureProvider.autoDispose<KhatmaStatistics>((ref) async {
  final service = ref.watch(statisticsServiceProvider);
  return await service.getKhatmaStatistics();
});

// Provider for reading statistics
final readingStatisticsProvider = FutureProvider.autoDispose<ReadingStatistics>((ref) async {
  final service = ref.watch(statisticsServiceProvider);
  return await service.getReadingStatistics();
});

// Provider for recitation statistics
final recitationStatisticsProvider = FutureProvider.autoDispose<RecitationStatistics>((ref) async {
  final service = ref.watch(statisticsServiceProvider);
  return await service.getRecitationStatistics();
});

// Provider for weekly comparison
final weeklyComparisonProvider = FutureProvider.autoDispose<WeeklyComparison>((ref) async {
  final service = ref.watch(statisticsServiceProvider);
  return await service.getWeeklyComparison();
});

// Provider for monthly comparison
final monthlyComparisonProvider = FutureProvider.autoDispose<MonthlyComparison>((ref) async {
  final service = ref.watch(statisticsServiceProvider);
  return await service.getMonthlyComparison();
});

// Provider for personal goals
final personalGoalsProvider = FutureProvider.autoDispose<List<PersonalGoal>>((ref) async {
  final service = ref.watch(statisticsServiceProvider);
  return await service.getPersonalGoals();
});

// State notifier for managing statistics state
class StatisticsNotifier extends StateNotifier<AsyncValue<StatisticsDashboard>> {
  final StatisticsService _service;

  StatisticsNotifier(this._service) : super(const AsyncValue.loading()) {
    loadStatistics();
  }

  Future<void> loadStatistics({int? timePeriodDays}) async {
    state = const AsyncValue.loading();
    try {
      final dashboard = await _service.getStatisticsDashboard(
        timePeriodDays: timePeriodDays,
      );
      state = AsyncValue.data(dashboard);
    } catch (error, stackTrace) {
      state = AsyncValue.error(error, stackTrace);
    }
  }

  Future<void> refresh() async {
    await loadStatistics();
  }
}

// Provider for statistics notifier
final statisticsNotifierProvider = StateNotifierProvider<StatisticsNotifier, AsyncValue<StatisticsDashboard>>((ref) {
  final service = ref.watch(statisticsServiceProvider);
  return StatisticsNotifier(service);
});

// State notifier for managing personal goals
class PersonalGoalsNotifier extends StateNotifier<AsyncValue<List<PersonalGoal>>> {
  final StatisticsService _service;

  PersonalGoalsNotifier(this._service) : super(const AsyncValue.loading()) {
    loadGoals();
  }

  Future<void> loadGoals() async {
    state = const AsyncValue.loading();
    try {
      final goals = await _service.getPersonalGoals();
      state = AsyncValue.data(goals);
    } catch (error, stackTrace) {
      state = AsyncValue.error(error, stackTrace);
    }
  }

  Future<void> createGoal(CreateGoalRequest request) async {
    try {
      await _service.createGoal(request);
      await loadGoals(); // Reload goals after creating
    } catch (error) {
      // Handle error
      rethrow;
    }
  }

  Future<void> updateGoalProgress(String goalId, int currentValue) async {
    try {
      await _service.updateGoalProgress(goalId, currentValue);
      await loadGoals(); // Reload goals after updating
    } catch (error) {
      // Handle error
      rethrow;
    }
  }

  Future<void> deleteGoal(String goalId) async {
    try {
      await _service.deleteGoal(goalId);
      await loadGoals(); // Reload goals after deleting
    } catch (error) {
      // Handle error
      rethrow;
    }
  }

  Future<void> refresh() async {
    await loadGoals();
  }
}

// Provider for personal goals notifier
final personalGoalsNotifierProvider = StateNotifierProvider<PersonalGoalsNotifier, AsyncValue<List<PersonalGoal>>>((ref) {
  final service = ref.watch(statisticsServiceProvider);
  return PersonalGoalsNotifier(service);
});

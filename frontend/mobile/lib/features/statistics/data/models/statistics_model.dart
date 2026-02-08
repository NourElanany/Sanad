import 'package:uuid/uuid.dart';

/// Statistics dashboard data model
class StatisticsDashboard {
  final String userId;
  final KhatmaStatistics khatmaStats;
  final ReadingStatistics readingStats;
  final RecitationStatistics recitationStats;
  final WeeklyComparison weeklyComparison;
  final MonthlyComparison monthlyComparison;
  final List<PersonalGoal> personalGoals;
  final DateTime generatedAt;

  StatisticsDashboard({
    required this.userId,
    required this.khatmaStats,
    required this.readingStats,
    required this.recitationStats,
    required this.weeklyComparison,
    required this.monthlyComparison,
    required this.personalGoals,
    required this.generatedAt,
  });

  factory StatisticsDashboard.fromJson(Map<String, dynamic> json) {
    return StatisticsDashboard(
      userId: json['user_id'] as String,
      khatmaStats: KhatmaStatistics.fromJson(json['khatma_stats'] as Map<String, dynamic>),
      readingStats: ReadingStatistics.fromJson(json['reading_stats'] as Map<String, dynamic>),
      recitationStats: RecitationStatistics.fromJson(json['recitation_stats'] as Map<String, dynamic>),
      weeklyComparison: WeeklyComparison.fromJson(json['weekly_comparison'] as Map<String, dynamic>),
      monthlyComparison: MonthlyComparison.fromJson(json['monthly_comparison'] as Map<String, dynamic>),
      personalGoals: (json['personal_goals'] as List<dynamic>)
          .map((e) => PersonalGoal.fromJson(e as Map<String, dynamic>))
          .toList(),
      generatedAt: DateTime.parse(json['generated_at'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'user_id': userId,
      'khatma_stats': khatmaStats.toJson(),
      'reading_stats': readingStats.toJson(),
      'recitation_stats': recitationStats.toJson(),
      'weekly_comparison': weeklyComparison.toJson(),
      'monthly_comparison': monthlyComparison.toJson(),
      'personal_goals': personalGoals.map((e) => e.toJson()).toList(),
      'generated_at': generatedAt.toIso8601String(),
    };
  }
}

/// Khatma completion statistics
class KhatmaStatistics {
  final int totalCompleted;
  final int currentProgress; // percentage 0-100
  final List<KhatmaCompletionData> completionHistory;
  final double averageCompletionDays;
  final int currentStreak;
  final int longestStreak;

  KhatmaStatistics({
    required this.totalCompleted,
    required this.currentProgress,
    required this.completionHistory,
    required this.averageCompletionDays,
    required this.currentStreak,
    required this.longestStreak,
  });

  factory KhatmaStatistics.fromJson(Map<String, dynamic> json) {
    return KhatmaStatistics(
      totalCompleted: json['total_completed'] as int,
      currentProgress: json['current_progress'] as int,
      completionHistory: (json['completion_history'] as List<dynamic>)
          .map((e) => KhatmaCompletionData.fromJson(e as Map<String, dynamic>))
          .toList(),
      averageCompletionDays: (json['average_completion_days'] as num).toDouble(),
      currentStreak: json['current_streak'] as int,
      longestStreak: json['longest_streak'] as int,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'total_completed': totalCompleted,
      'current_progress': currentProgress,
      'completion_history': completionHistory.map((e) => e.toJson()).toList(),
      'average_completion_days': averageCompletionDays,
      'current_streak': currentStreak,
      'longest_streak': longestStreak,
    };
  }
}

/// Individual Khatma completion data
class KhatmaCompletionData {
  final String khatmaId;
  final DateTime completionDate;
  final int durationDays;
  final double consistencyScore;

  KhatmaCompletionData({
    required this.khatmaId,
    required this.completionDate,
    required this.durationDays,
    required this.consistencyScore,
  });

  factory KhatmaCompletionData.fromJson(Map<String, dynamic> json) {
    return KhatmaCompletionData(
      khatmaId: json['khatma_id'] as String,
      completionDate: DateTime.parse(json['completion_date'] as String),
      durationDays: json['duration_days'] as int,
      consistencyScore: (json['consistency_score'] as num).toDouble(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'khatma_id': khatmaId,
      'completion_date': completionDate.toIso8601String(),
      'duration_days': durationDays,
      'consistency_score': consistencyScore,
    };
  }
}

/// Reading time statistics
class ReadingStatistics {
  final int totalMinutesToday;
  final int totalMinutesWeek;
  final int totalMinutesMonth;
  final double averageDailyMinutes;
  final List<DailyReadingData> dailyReadingHistory;
  final int pagesReadTotal;
  final int surahsCompleted;

  ReadingStatistics({
    required this.totalMinutesToday,
    required this.totalMinutesWeek,
    required this.totalMinutesMonth,
    required this.averageDailyMinutes,
    required this.dailyReadingHistory,
    required this.pagesReadTotal,
    required this.surahsCompleted,
  });

  factory ReadingStatistics.fromJson(Map<String, dynamic> json) {
    return ReadingStatistics(
      totalMinutesToday: json['total_minutes_today'] as int,
      totalMinutesWeek: json['total_minutes_week'] as int,
      totalMinutesMonth: json['total_minutes_month'] as int,
      averageDailyMinutes: (json['average_daily_minutes'] as num).toDouble(),
      dailyReadingHistory: (json['daily_reading_history'] as List<dynamic>)
          .map((e) => DailyReadingData.fromJson(e as Map<String, dynamic>))
          .toList(),
      pagesReadTotal: json['pages_read_total'] as int,
      surahsCompleted: json['surahs_completed'] as int,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'total_minutes_today': totalMinutesToday,
      'total_minutes_week': totalMinutesWeek,
      'total_minutes_month': totalMinutesMonth,
      'average_daily_minutes': averageDailyMinutes,
      'daily_reading_history': dailyReadingHistory.map((e) => e.toJson()).toList(),
      'pages_read_total': pagesReadTotal,
      'surahs_completed': surahsCompleted,
    };
  }
}

/// Daily reading data point
class DailyReadingData {
  final DateTime date;
  final int minutes;
  final int pagesRead;
  final double readingSpeedWpm;

  DailyReadingData({
    required this.date,
    required this.minutes,
    required this.pagesRead,
    required this.readingSpeedWpm,
  });

  factory DailyReadingData.fromJson(Map<String, dynamic> json) {
    return DailyReadingData(
      date: DateTime.parse(json['date'] as String),
      minutes: json['minutes'] as int,
      pagesRead: json['pages_read'] as int,
      readingSpeedWpm: (json['reading_speed_wpm'] as num).toDouble(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'date': date.toIso8601String(),
      'minutes': minutes,
      'pages_read': pagesRead,
      'reading_speed_wpm': readingSpeedWpm,
    };
  }
}

/// Recitation improvement statistics
class RecitationStatistics {
  final double currentScore;
  final double averageScore;
  final double improvementPercentage;
  final List<RecitationScoreData> scoreHistory;
  final int totalRecitations;
  final Map<String, int> errorTypeFrequency;
  final List<String> topImprovementAreas;

  RecitationStatistics({
    required this.currentScore,
    required this.averageScore,
    required this.improvementPercentage,
    required this.scoreHistory,
    required this.totalRecitations,
    required this.errorTypeFrequency,
    required this.topImprovementAreas,
  });

  factory RecitationStatistics.fromJson(Map<String, dynamic> json) {
    return RecitationStatistics(
      currentScore: (json['current_score'] as num).toDouble(),
      averageScore: (json['average_score'] as num).toDouble(),
      improvementPercentage: (json['improvement_percentage'] as num).toDouble(),
      scoreHistory: (json['score_history'] as List<dynamic>)
          .map((e) => RecitationScoreData.fromJson(e as Map<String, dynamic>))
          .toList(),
      totalRecitations: json['total_recitations'] as int,
      errorTypeFrequency: Map<String, int>.from(json['error_type_frequency'] as Map),
      topImprovementAreas: List<String>.from(json['top_improvement_areas'] as List),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'current_score': currentScore,
      'average_score': averageScore,
      'improvement_percentage': improvementPercentage,
      'score_history': scoreHistory.map((e) => e.toJson()).toList(),
      'total_recitations': totalRecitations,
      'error_type_frequency': errorTypeFrequency,
      'top_improvement_areas': topImprovementAreas,
    };
  }
}

/// Recitation score data point
class RecitationScoreData {
  final DateTime date;
  final double score;
  final int errorCount;
  final String surahName;

  RecitationScoreData({
    required this.date,
    required this.score,
    required this.errorCount,
    required this.surahName,
  });

  factory RecitationScoreData.fromJson(Map<String, dynamic> json) {
    return RecitationScoreData(
      date: DateTime.parse(json['date'] as String),
      score: (json['score'] as num).toDouble(),
      errorCount: json['error_count'] as int,
      surahName: json['surah_name'] as String,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'date': date.toIso8601String(),
      'score': score,
      'error_count': errorCount,
      'surah_name': surahName,
    };
  }
}

/// Weekly comparison data
class WeeklyComparison {
  final int currentWeekMinutes;
  final int previousWeekMinutes;
  final double changePercentage;
  final String trend; // 'improving', 'stable', 'declining'
  final List<WeeklyDataPoint> weeklyData;

  WeeklyComparison({
    required this.currentWeekMinutes,
    required this.previousWeekMinutes,
    required this.changePercentage,
    required this.trend,
    required this.weeklyData,
  });

  factory WeeklyComparison.fromJson(Map<String, dynamic> json) {
    return WeeklyComparison(
      currentWeekMinutes: json['current_week_minutes'] as int,
      previousWeekMinutes: json['previous_week_minutes'] as int,
      changePercentage: (json['change_percentage'] as num).toDouble(),
      trend: json['trend'] as String,
      weeklyData: (json['weekly_data'] as List<dynamic>)
          .map((e) => WeeklyDataPoint.fromJson(e as Map<String, dynamic>))
          .toList(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'current_week_minutes': currentWeekMinutes,
      'previous_week_minutes': previousWeekMinutes,
      'change_percentage': changePercentage,
      'trend': trend,
      'weekly_data': weeklyData.map((e) => e.toJson()).toList(),
    };
  }
}

/// Weekly data point
class WeeklyDataPoint {
  final DateTime weekStart;
  final int totalMinutes;
  final int sessionsCount;
  final double averageScore;

  WeeklyDataPoint({
    required this.weekStart,
    required this.totalMinutes,
    required this.sessionsCount,
    required this.averageScore,
  });

  factory WeeklyDataPoint.fromJson(Map<String, dynamic> json) {
    return WeeklyDataPoint(
      weekStart: DateTime.parse(json['week_start'] as String),
      totalMinutes: json['total_minutes'] as int,
      sessionsCount: json['sessions_count'] as int,
      averageScore: (json['average_score'] as num).toDouble(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'week_start': weekStart.toIso8601String(),
      'total_minutes': totalMinutes,
      'sessions_count': sessionsCount,
      'average_score': averageScore,
    };
  }
}

/// Monthly comparison data
class MonthlyComparison {
  final int currentMonthMinutes;
  final int previousMonthMinutes;
  final double changePercentage;
  final String trend;
  final List<MonthlyDataPoint> monthlyData;

  MonthlyComparison({
    required this.currentMonthMinutes,
    required this.previousMonthMinutes,
    required this.changePercentage,
    required this.trend,
    required this.monthlyData,
  });

  factory MonthlyComparison.fromJson(Map<String, dynamic> json) {
    return MonthlyComparison(
      currentMonthMinutes: json['current_month_minutes'] as int,
      previousMonthMinutes: json['previous_month_minutes'] as int,
      changePercentage: (json['change_percentage'] as num).toDouble(),
      trend: json['trend'] as String,
      monthlyData: (json['monthly_data'] as List<dynamic>)
          .map((e) => MonthlyDataPoint.fromJson(e as Map<String, dynamic>))
          .toList(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'current_month_minutes': currentMonthMinutes,
      'previous_month_minutes': previousMonthMinutes,
      'change_percentage': changePercentage,
      'trend': trend,
      'monthly_data': monthlyData.map((e) => e.toJson()).toList(),
    };
  }
}

/// Monthly data point
class MonthlyDataPoint {
  final DateTime monthStart;
  final int totalMinutes;
  final int khatmasCompleted;
  final double consistencyScore;

  MonthlyDataPoint({
    required this.monthStart,
    required this.totalMinutes,
    required this.khatmasCompleted,
    required this.consistencyScore,
  });

  factory MonthlyDataPoint.fromJson(Map<String, dynamic> json) {
    return MonthlyDataPoint(
      monthStart: DateTime.parse(json['month_start'] as String),
      totalMinutes: json['total_minutes'] as int,
      khatmasCompleted: json['khatmas_completed'] as int,
      consistencyScore: (json['consistency_score'] as num).toDouble(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'month_start': monthStart.toIso8601String(),
      'total_minutes': totalMinutes,
      'khatmas_completed': khatmasCompleted,
      'consistency_score': consistencyScore,
    };
  }
}

/// Personal goal
class PersonalGoal {
  final String id;
  final String title;
  final String description;
  final GoalType type;
  final int targetValue;
  final int currentValue;
  final DateTime deadline;
  final bool isCompleted;
  final double progressPercentage;

  PersonalGoal({
    required this.id,
    required this.title,
    required this.description,
    required this.type,
    required this.targetValue,
    required this.currentValue,
    required this.deadline,
    required this.isCompleted,
    required this.progressPercentage,
  });

  factory PersonalGoal.fromJson(Map<String, dynamic> json) {
    return PersonalGoal(
      id: json['id'] as String,
      title: json['title'] as String,
      description: json['description'] as String,
      type: GoalType.values.firstWhere(
        (e) => e.toString().split('.').last == json['type'],
        orElse: () => GoalType.custom,
      ),
      targetValue: json['target_value'] as int,
      currentValue: json['current_value'] as int,
      deadline: DateTime.parse(json['deadline'] as String),
      isCompleted: json['is_completed'] as bool,
      progressPercentage: (json['progress_percentage'] as num).toDouble(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'title': title,
      'description': description,
      'type': type.toString().split('.').last,
      'target_value': targetValue,
      'current_value': currentValue,
      'deadline': deadline.toIso8601String(),
      'is_completed': isCompleted,
      'progress_percentage': progressPercentage,
    };
  }
}

/// Goal types
enum GoalType {
  dailyReading,
  weeklyReading,
  monthlyReading,
  khatmaCompletion,
  recitationImprovement,
  consistencyStreak,
  custom,
}

/// Create goal request
class CreateGoalRequest {
  final String title;
  final String description;
  final GoalType type;
  final int targetValue;
  final DateTime deadline;

  CreateGoalRequest({
    required this.title,
    required this.description,
    required this.type,
    required this.targetValue,
    required this.deadline,
  });

  Map<String, dynamic> toJson() {
    return {
      'title': title,
      'description': description,
      'type': type.toString().split('.').last,
      'target_value': targetValue,
      'deadline': deadline.toIso8601String(),
    };
  }
}

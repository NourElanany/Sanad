/// Achievement and rewards system data models
/// Supports badges, points, levels, challenges, and social sharing

/// Achievement badge model
class Achievement {
  final String id;
  final String titleAr;
  final String titleEn;
  final String descriptionAr;
  final String descriptionEn;
  final AchievementCategory category;
  final AchievementTier tier;
  final String iconName;
  final int pointsReward;
  final bool isUnlocked;
  final DateTime? unlockedAt;
  final double progress; // 0.0 to 1.0
  final int currentValue;
  final int targetValue;
  final List<String> requirements;

  Achievement({
    required this.id,
    required this.titleAr,
    required this.titleEn,
    required this.descriptionAr,
    required this.descriptionEn,
    required this.category,
    required this.tier,
    required this.iconName,
    required this.pointsReward,
    required this.isUnlocked,
    this.unlockedAt,
    required this.progress,
    required this.currentValue,
    required this.targetValue,
    required this.requirements,
  });

  factory Achievement.fromJson(Map<String, dynamic> json) {
    return Achievement(
      id: json['id'] as String,
      titleAr: json['title_ar'] as String,
      titleEn: json['title_en'] as String,
      descriptionAr: json['description_ar'] as String,
      descriptionEn: json['description_en'] as String,
      category: AchievementCategory.values.firstWhere(
        (e) => e.toString().split('.').last == json['category'],
        orElse: () => AchievementCategory.general,
      ),
      tier: AchievementTier.values.firstWhere(
        (e) => e.toString().split('.').last == json['tier'],
        orElse: () => AchievementTier.bronze,
      ),
      iconName: json['icon_name'] as String,
      pointsReward: json['points_reward'] as int,
      isUnlocked: json['is_unlocked'] as bool,
      unlockedAt: json['unlocked_at'] != null
          ? DateTime.parse(json['unlocked_at'] as String)
          : null,
      progress: (json['progress'] as num).toDouble(),
      currentValue: json['current_value'] as int,
      targetValue: json['target_value'] as int,
      requirements: List<String>.from(json['requirements'] as List),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'title_ar': titleAr,
      'title_en': titleEn,
      'description_ar': descriptionAr,
      'description_en': descriptionEn,
      'category': category.toString().split('.').last,
      'tier': tier.toString().split('.').last,
      'icon_name': iconName,
      'points_reward': pointsReward,
      'is_unlocked': isUnlocked,
      'unlocked_at': unlockedAt?.toIso8601String(),
      'progress': progress,
      'current_value': currentValue,
      'target_value': targetValue,
      'requirements': requirements,
    };
  }

  Achievement copyWith({
    bool? isUnlocked,
    DateTime? unlockedAt,
    double? progress,
    int? currentValue,
  }) {
    return Achievement(
      id: id,
      titleAr: titleAr,
      titleEn: titleEn,
      descriptionAr: descriptionAr,
      descriptionEn: descriptionEn,
      category: category,
      tier: tier,
      iconName: iconName,
      pointsReward: pointsReward,
      isUnlocked: isUnlocked ?? this.isUnlocked,
      unlockedAt: unlockedAt ?? this.unlockedAt,
      progress: progress ?? this.progress,
      currentValue: currentValue ?? this.currentValue,
      targetValue: targetValue,
      requirements: requirements,
    );
  }
}

/// Achievement categories
enum AchievementCategory {
  quranReading,
  khatmaCompletion,
  recitation,
  consistency,
  learning,
  prayer,
  general,
}

/// Achievement tiers (difficulty/rarity)
enum AchievementTier {
  bronze,
  silver,
  gold,
  platinum,
  diamond,
}

/// User level and points system
class UserLevel {
  final String userId;
  final int currentLevel;
  final int totalPoints;
  final int pointsInCurrentLevel;
  final int pointsRequiredForNextLevel;
  final double progressToNextLevel; // 0.0 to 1.0
  final String levelTitle;
  final String levelTitleAr;
  final List<String> unlockedPerks;
  final DateTime lastUpdated;

  UserLevel({
    required this.userId,
    required this.currentLevel,
    required this.totalPoints,
    required this.pointsInCurrentLevel,
    required this.pointsRequiredForNextLevel,
    required this.progressToNextLevel,
    required this.levelTitle,
    required this.levelTitleAr,
    required this.unlockedPerks,
    required this.lastUpdated,
  });

  factory UserLevel.fromJson(Map<String, dynamic> json) {
    return UserLevel(
      userId: json['user_id'] as String,
      currentLevel: json['current_level'] as int,
      totalPoints: json['total_points'] as int,
      pointsInCurrentLevel: json['points_in_current_level'] as int,
      pointsRequiredForNextLevel: json['points_required_for_next_level'] as int,
      progressToNextLevel: (json['progress_to_next_level'] as num).toDouble(),
      levelTitle: json['level_title'] as String,
      levelTitleAr: json['level_title_ar'] as String,
      unlockedPerks: List<String>.from(json['unlocked_perks'] as List),
      lastUpdated: DateTime.parse(json['last_updated'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'user_id': userId,
      'current_level': currentLevel,
      'total_points': totalPoints,
      'points_in_current_level': pointsInCurrentLevel,
      'points_required_for_next_level': pointsRequiredForNextLevel,
      'progress_to_next_level': progressToNextLevel,
      'level_title': levelTitle,
      'level_title_ar': levelTitleAr,
      'unlocked_perks': unlockedPerks,
      'last_updated': lastUpdated.toIso8601String(),
    };
  }
}

/// Daily and weekly challenges
class Challenge {
  final String id;
  final String titleAr;
  final String titleEn;
  final String descriptionAr;
  final String descriptionEn;
  final ChallengeType type;
  final ChallengeDifficulty difficulty;
  final int pointsReward;
  final int targetValue;
  final int currentProgress;
  final double progressPercentage;
  final DateTime startDate;
  final DateTime endDate;
  final bool isCompleted;
  final DateTime? completedAt;
  final String iconName;
  final List<String> requirements;

  Challenge({
    required this.id,
    required this.titleAr,
    required this.titleEn,
    required this.descriptionAr,
    required this.descriptionEn,
    required this.type,
    required this.difficulty,
    required this.pointsReward,
    required this.targetValue,
    required this.currentProgress,
    required this.progressPercentage,
    required this.startDate,
    required this.endDate,
    required this.isCompleted,
    this.completedAt,
    required this.iconName,
    required this.requirements,
  });

  factory Challenge.fromJson(Map<String, dynamic> json) {
    return Challenge(
      id: json['id'] as String,
      titleAr: json['title_ar'] as String,
      titleEn: json['title_en'] as String,
      descriptionAr: json['description_ar'] as String,
      descriptionEn: json['description_en'] as String,
      type: ChallengeType.values.firstWhere(
        (e) => e.toString().split('.').last == json['type'],
        orElse: () => ChallengeType.daily,
      ),
      difficulty: ChallengeDifficulty.values.firstWhere(
        (e) => e.toString().split('.').last == json['difficulty'],
        orElse: () => ChallengeDifficulty.easy,
      ),
      pointsReward: json['points_reward'] as int,
      targetValue: json['target_value'] as int,
      currentProgress: json['current_progress'] as int,
      progressPercentage: (json['progress_percentage'] as num).toDouble(),
      startDate: DateTime.parse(json['start_date'] as String),
      endDate: DateTime.parse(json['end_date'] as String),
      isCompleted: json['is_completed'] as bool,
      completedAt: json['completed_at'] != null
          ? DateTime.parse(json['completed_at'] as String)
          : null,
      iconName: json['icon_name'] as String,
      requirements: List<String>.from(json['requirements'] as List),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'title_ar': titleAr,
      'title_en': titleEn,
      'description_ar': descriptionAr,
      'description_en': descriptionEn,
      'type': type.toString().split('.').last,
      'difficulty': difficulty.toString().split('.').last,
      'points_reward': pointsReward,
      'target_value': targetValue,
      'current_progress': currentProgress,
      'progress_percentage': progressPercentage,
      'start_date': startDate.toIso8601String(),
      'end_date': endDate.toIso8601String(),
      'is_completed': isCompleted,
      'completed_at': completedAt?.toIso8601String(),
      'icon_name': iconName,
      'requirements': requirements,
    };
  }

  bool get isExpired => DateTime.now().isAfter(endDate);
  
  Duration get timeRemaining => endDate.difference(DateTime.now());
}

/// Challenge types
enum ChallengeType {
  daily,
  weekly,
  special,
}

/// Challenge difficulty levels
enum ChallengeDifficulty {
  easy,
  medium,
  hard,
  expert,
}

/// Achievements dashboard summary
class AchievementsDashboard {
  final String userId;
  final UserLevel userLevel;
  final List<Achievement> recentAchievements;
  final List<Achievement> inProgressAchievements;
  final List<Challenge> activeChallenges;
  final AchievementStats stats;
  final List<MotivationalReminder> reminders;
  final DateTime generatedAt;

  AchievementsDashboard({
    required this.userId,
    required this.userLevel,
    required this.recentAchievements,
    required this.inProgressAchievements,
    required this.activeChallenges,
    required this.stats,
    required this.reminders,
    required this.generatedAt,
  });

  factory AchievementsDashboard.fromJson(Map<String, dynamic> json) {
    return AchievementsDashboard(
      userId: json['user_id'] as String,
      userLevel: UserLevel.fromJson(json['user_level'] as Map<String, dynamic>),
      recentAchievements: (json['recent_achievements'] as List<dynamic>)
          .map((e) => Achievement.fromJson(e as Map<String, dynamic>))
          .toList(),
      inProgressAchievements: (json['in_progress_achievements'] as List<dynamic>)
          .map((e) => Achievement.fromJson(e as Map<String, dynamic>))
          .toList(),
      activeChallenges: (json['active_challenges'] as List<dynamic>)
          .map((e) => Challenge.fromJson(e as Map<String, dynamic>))
          .toList(),
      stats: AchievementStats.fromJson(json['stats'] as Map<String, dynamic>),
      reminders: (json['reminders'] as List<dynamic>)
          .map((e) => MotivationalReminder.fromJson(e as Map<String, dynamic>))
          .toList(),
      generatedAt: DateTime.parse(json['generated_at'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'user_id': userId,
      'user_level': userLevel.toJson(),
      'recent_achievements': recentAchievements.map((e) => e.toJson()).toList(),
      'in_progress_achievements': inProgressAchievements.map((e) => e.toJson()).toList(),
      'active_challenges': activeChallenges.map((e) => e.toJson()).toList(),
      'stats': stats.toJson(),
      'reminders': reminders.map((e) => e.toJson()).toList(),
      'generated_at': generatedAt.toIso8601String(),
    };
  }
}

/// Achievement statistics
class AchievementStats {
  final int totalAchievements;
  final int unlockedAchievements;
  final int lockedAchievements;
  final double completionPercentage;
  final int totalChallengesCompleted;
  final int currentStreak;
  final int longestStreak;
  final Map<AchievementCategory, int> achievementsByCategory;
  final Map<AchievementTier, int> achievementsByTier;

  AchievementStats({
    required this.totalAchievements,
    required this.unlockedAchievements,
    required this.lockedAchievements,
    required this.completionPercentage,
    required this.totalChallengesCompleted,
    required this.currentStreak,
    required this.longestStreak,
    required this.achievementsByCategory,
    required this.achievementsByTier,
  });

  factory AchievementStats.fromJson(Map<String, dynamic> json) {
    return AchievementStats(
      totalAchievements: json['total_achievements'] as int,
      unlockedAchievements: json['unlocked_achievements'] as int,
      lockedAchievements: json['locked_achievements'] as int,
      completionPercentage: (json['completion_percentage'] as num).toDouble(),
      totalChallengesCompleted: json['total_challenges_completed'] as int,
      currentStreak: json['current_streak'] as int,
      longestStreak: json['longest_streak'] as int,
      achievementsByCategory: (json['achievements_by_category'] as Map<String, dynamic>).map(
        (key, value) => MapEntry(
          AchievementCategory.values.firstWhere(
            (e) => e.toString().split('.').last == key,
            orElse: () => AchievementCategory.general,
          ),
          value as int,
        ),
      ),
      achievementsByTier: (json['achievements_by_tier'] as Map<String, dynamic>).map(
        (key, value) => MapEntry(
          AchievementTier.values.firstWhere(
            (e) => e.toString().split('.').last == key,
            orElse: () => AchievementTier.bronze,
          ),
          value as int,
        ),
      ),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'total_achievements': totalAchievements,
      'unlocked_achievements': unlockedAchievements,
      'locked_achievements': lockedAchievements,
      'completion_percentage': completionPercentage,
      'total_challenges_completed': totalChallengesCompleted,
      'current_streak': currentStreak,
      'longest_streak': longestStreak,
      'achievements_by_category': achievementsByCategory.map(
        (key, value) => MapEntry(key.toString().split('.').last, value),
      ),
      'achievements_by_tier': achievementsByTier.map(
        (key, value) => MapEntry(key.toString().split('.').last, value),
      ),
    };
  }
}

/// Motivational reminder
class MotivationalReminder {
  final String id;
  final String messageAr;
  final String messageEn;
  final ReminderType type;
  final DateTime scheduledFor;
  final bool isActive;
  final String? relatedAchievementId;
  final String? relatedChallengeId;

  MotivationalReminder({
    required this.id,
    required this.messageAr,
    required this.messageEn,
    required this.type,
    required this.scheduledFor,
    required this.isActive,
    this.relatedAchievementId,
    this.relatedChallengeId,
  });

  factory MotivationalReminder.fromJson(Map<String, dynamic> json) {
    return MotivationalReminder(
      id: json['id'] as String,
      messageAr: json['message_ar'] as String,
      messageEn: json['message_en'] as String,
      type: ReminderType.values.firstWhere(
        (e) => e.toString().split('.').last == json['type'],
        orElse: () => ReminderType.general,
      ),
      scheduledFor: DateTime.parse(json['scheduled_for'] as String),
      isActive: json['is_active'] as bool,
      relatedAchievementId: json['related_achievement_id'] as String?,
      relatedChallengeId: json['related_challenge_id'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'message_ar': messageAr,
      'message_en': messageEn,
      'type': type.toString().split('.').last,
      'scheduled_for': scheduledFor.toIso8601String(),
      'is_active': isActive,
      'related_achievement_id': relatedAchievementId,
      'related_challenge_id': relatedChallengeId,
    };
  }
}

/// Reminder types
enum ReminderType {
  achievementProgress,
  challengeDeadline,
  streakMaintenance,
  levelUp,
  general,
}

/// Share achievement request
class ShareAchievementRequest {
  final String achievementId;
  final SharePlatform platform;
  final String? customMessage;

  ShareAchievementRequest({
    required this.achievementId,
    required this.platform,
    this.customMessage,
  });

  Map<String, dynamic> toJson() {
    return {
      'achievement_id': achievementId,
      'platform': platform.toString().split('.').last,
      'custom_message': customMessage,
    };
  }
}

/// Social sharing platforms
enum SharePlatform {
  twitter,
  facebook,
  whatsapp,
  telegram,
  instagram,
  clipboard,
}

/// Achievement unlock notification
class AchievementUnlockNotification {
  final Achievement achievement;
  final int pointsEarned;
  final bool leveledUp;
  final int? newLevel;
  final DateTime unlockedAt;

  AchievementUnlockNotification({
    required this.achievement,
    required this.pointsEarned,
    required this.leveledUp,
    this.newLevel,
    required this.unlockedAt,
  });

  factory AchievementUnlockNotification.fromJson(Map<String, dynamic> json) {
    return AchievementUnlockNotification(
      achievement: Achievement.fromJson(json['achievement'] as Map<String, dynamic>),
      pointsEarned: json['points_earned'] as int,
      leveledUp: json['leveled_up'] as bool,
      newLevel: json['new_level'] as int?,
      unlockedAt: DateTime.parse(json['unlocked_at'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'achievement': achievement.toJson(),
      'points_earned': pointsEarned,
      'leveled_up': leveledUp,
      'new_level': newLevel,
      'unlocked_at': unlockedAt.toIso8601String(),
    };
  }
}

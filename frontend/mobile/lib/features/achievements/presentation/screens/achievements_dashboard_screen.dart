import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/providers/achievements_provider.dart';
import '../../../../core/widgets/islamic_loading_indicator.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../widgets/user_level_card.dart';
import '../widgets/achievement_card.dart';
import '../widgets/challenge_card.dart';
import '../widgets/achievement_stats_card.dart';
import '../widgets/reminder_card.dart';

/// Achievements and rewards dashboard screen
class AchievementsDashboardScreen extends ConsumerStatefulWidget {
  const AchievementsDashboardScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<AchievementsDashboardScreen> createState() => _AchievementsDashboardScreenState();
}

class _AchievementsDashboardScreenState extends ConsumerState<AchievementsDashboardScreen> {
  @override
  void initState() {
    super.initState();
    // Load dashboard data on init
    Future.microtask(() {
      ref.read(achievementsDashboardProvider.notifier).loadDashboard();
      ref.read(challengesProvider.notifier).loadChallenges();
    });
  }

  @override
  Widget build(BuildContext context) {
    final dashboardState = ref.watch(achievementsDashboardProvider);
    final challengesState = ref.watch(challengesProvider);

    return Scaffold(
      backgroundColor: const Color(0xFFF8F9FA),
      appBar: AppBar(
        title: const Text(
          'الإنجازات والمكافآت',
          style: TextStyle(
            fontFamily: 'Tajawal',
            fontWeight: FontWeight.bold,
          ),
        ),
        backgroundColor: const Color(0xFF1B365D),
        elevation: 0,
        actions: [
          IconButton(
            icon: const Icon(Icons.leaderboard),
            onPressed: () {
              Navigator.pushNamed(context, '/achievements/leaderboard');
            },
            tooltip: 'لوحة المتصدرين',
          ),
          IconButton(
            icon: const Icon(Icons.history),
            onPressed: () {
              Navigator.pushNamed(context, '/achievements/history');
            },
            tooltip: 'سجل الإنجازات',
          ),
        ],
      ),
      body: RefreshIndicator(
        onRefresh: () async {
          await ref.read(achievementsDashboardProvider.notifier).refresh();
          await ref.read(challengesProvider.notifier).refresh();
        },
        child: dashboardState.isLoading && dashboardState.dashboard == null
            ? const Center(child: IslamicLoadingIndicator())
            : dashboardState.error != null
                ? _buildErrorState(dashboardState.error!)
                : dashboardState.dashboard == null
                    ? const Center(child: Text('لا توجد بيانات'))
                    : _buildDashboardContent(
                        dashboardState.dashboard!,
                        challengesState.challenges,
                      ),
      ),
    );
  }

  Widget _buildErrorState(String error) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Icon(
            Icons.error_outline,
            size: 64,
            color: Color(0xFFDC3545),
          ),
          const SizedBox(height: 16),
          Text(
            'حدث خطأ',
            style: const TextStyle(
              fontSize: 20,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
            ),
          ),
          const SizedBox(height: 8),
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 32),
            child: Text(
              error,
              textAlign: TextAlign.center,
              style: const TextStyle(
                fontSize: 14,
                color: Color(0xFF666666),
                fontFamily: 'Tajawal',
              ),
            ),
          ),
          const SizedBox(height: 24),
          ElevatedButton(
            onPressed: () {
              ref.read(achievementsDashboardProvider.notifier).refresh();
            },
            style: ElevatedButton.styleFrom(
              backgroundColor: const Color(0xFF1B365D),
              padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 12),
            ),
            child: const Text(
              'إعادة المحاولة',
              style: TextStyle(
                fontFamily: 'Tajawal',
                fontSize: 16,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildDashboardContent(
    dynamic dashboard,
    List<dynamic> challenges,
  ) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // User Level Card
          UserLevelCard(userLevel: dashboard.userLevel),
          const SizedBox(height: 16),

          // Achievement Stats Card
          AchievementStatsCard(stats: dashboard.stats),
          const SizedBox(height: 24),

          // Active Challenges Section
          _buildSectionHeader(
            'التحديات النشطة',
            Icons.emoji_events,
            onViewAll: () {
              Navigator.pushNamed(context, '/achievements/challenges');
            },
          ),
          const SizedBox(height: 12),
          if (challenges.isEmpty)
            const IslamicCard(
              child: Padding(
                padding: EdgeInsets.all(24),
                child: Center(
                  child: Text(
                    'لا توجد تحديات نشطة حالياً',
                    style: TextStyle(
                      fontSize: 16,
                      color: Color(0xFF666666),
                      fontFamily: 'Tajawal',
                    ),
                  ),
                ),
              ),
            )
          else
            ...challenges.take(3).map((challenge) => Padding(
                  padding: const EdgeInsets.only(bottom: 12),
                  child: ChallengeCard(challenge: challenge),
                )),
          const SizedBox(height: 24),

          // Recent Achievements Section
          _buildSectionHeader(
            'الإنجازات الأخيرة',
            Icons.stars,
            onViewAll: () {
              Navigator.pushNamed(context, '/achievements/all');
            },
          ),
          const SizedBox(height: 12),
          if (dashboard.recentAchievements.isEmpty)
            const IslamicCard(
              child: Padding(
                padding: EdgeInsets.all(24),
                child: Center(
                  child: Text(
                    'لم تحصل على إنجازات بعد',
                    style: TextStyle(
                      fontSize: 16,
                      color: Color(0xFF666666),
                      fontFamily: 'Tajawal',
                    ),
                  ),
                ),
              ),
            )
          else
            ...dashboard.recentAchievements.map((achievement) => Padding(
                  padding: const EdgeInsets.only(bottom: 12),
                  child: AchievementCard(achievement: achievement),
                )),
          const SizedBox(height: 24),

          // In Progress Achievements Section
          _buildSectionHeader(
            'إنجازات قيد التقدم',
            Icons.trending_up,
            onViewAll: () {
              Navigator.pushNamed(context, '/achievements/in-progress');
            },
          ),
          const SizedBox(height: 12),
          if (dashboard.inProgressAchievements.isEmpty)
            const IslamicCard(
              child: Padding(
                padding: EdgeInsets.all(24),
                child: Center(
                  child: Text(
                    'لا توجد إنجازات قيد التقدم',
                    style: TextStyle(
                      fontSize: 16,
                      color: Color(0xFF666666),
                      fontFamily: 'Tajawal',
                    ),
                  ),
                ),
              ),
            )
          else
            ...dashboard.inProgressAchievements.take(3).map((achievement) => Padding(
                  padding: const EdgeInsets.only(bottom: 12),
                  child: AchievementCard(achievement: achievement),
                )),
          const SizedBox(height: 24),

          // Motivational Reminders Section
          if (dashboard.reminders.isNotEmpty) ...[
            _buildSectionHeader(
              'تذكيرات تحفيزية',
              Icons.notifications_active,
            ),
            const SizedBox(height: 12),
            ...dashboard.reminders.take(2).map((reminder) => Padding(
                  padding: const EdgeInsets.only(bottom: 12),
                  child: ReminderCard(reminder: reminder),
                )),
            const SizedBox(height: 24),
          ],
        ],
      ),
    );
  }

  Widget _buildSectionHeader(
    String title,
    IconData icon, {
    VoidCallback? onViewAll,
  }) {
    return Row(
      children: [
        Icon(
          icon,
          color: const Color(0xFFB8860B),
          size: 24,
        ),
        const SizedBox(width: 8),
        Expanded(
          child: Text(
            title,
            style: const TextStyle(
              fontSize: 20,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
              color: Color(0xFF1B365D),
            ),
          ),
        ),
        if (onViewAll != null)
          TextButton(
            onPressed: onViewAll,
            child: const Text(
              'عرض الكل',
              style: TextStyle(
                fontSize: 14,
                fontFamily: 'Tajawal',
                color: Color(0xFF1B365D),
              ),
            ),
          ),
      ],
    );
  }
}

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/providers/statistics_provider.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_loading_indicator.dart';
import '../widgets/khatma_completion_chart.dart';
import '../widgets/reading_minutes_chart.dart';
import '../widgets/recitation_improvement_chart.dart';
import '../widgets/comparison_cards.dart';
import '../widgets/personal_goals_section.dart';
import '../widgets/statistics_summary_cards.dart';

/// Statistics Dashboard Screen
/// Displays comprehensive statistics including:
/// - Khatma completion charts
/// - Daily reading minutes
/// - Recitation improvement metrics
/// - Weekly and monthly comparisons
/// - Personal goals tracking
class StatisticsDashboardScreen extends ConsumerWidget {
  const StatisticsDashboardScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final statisticsAsync = ref.watch(statisticsNotifierProvider);

    return Scaffold(
      backgroundColor: AppColors.backgroundPrimary,
      appBar: AppBar(
        title: Text(
          'الإحصائيات',
          style: AppTextStyles.h5.copyWith(
            color: Colors.white,
            fontWeight: FontWeight.bold,
          ),
        ),
        backgroundColor: AppColors.primaryMain,
        elevation: 0,
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh, color: Colors.white),
            onPressed: () {
              ref.read(statisticsNotifierProvider.notifier).refresh();
            },
            tooltip: 'تحديث',
          ),
        ],
      ),
      body: statisticsAsync.when(
        data: (dashboard) => RefreshIndicator(
          onRefresh: () async {
            await ref.read(statisticsNotifierProvider.notifier).refresh();
          },
          child: SingleChildScrollView(
            physics: const AlwaysScrollableScrollPhysics(),
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                // Summary Cards
                StatisticsSummaryCards(dashboard: dashboard),
                const SizedBox(height: 24),

                // Khatma Completion Chart
                _buildSectionHeader(context, 'الختمات المكتملة', Icons.book),
                const SizedBox(height: 12),
                KhatmaCompletionChart(
                  khatmaStats: dashboard.khatmaStats,
                ),
                const SizedBox(height: 24),

                // Reading Minutes Chart
                _buildSectionHeader(context, 'دقائق القراءة اليومية', Icons.schedule),
                const SizedBox(height: 12),
                ReadingMinutesChart(
                  readingStats: dashboard.readingStats,
                ),
                const SizedBox(height: 24),

                // Recitation Improvement Chart
                _buildSectionHeader(context, 'تحسن التلاوة', Icons.trending_up),
                const SizedBox(height: 12),
                RecitationImprovementChart(
                  recitationStats: dashboard.recitationStats,
                ),
                const SizedBox(height: 24),

                // Weekly and Monthly Comparisons
                _buildSectionHeader(context, 'المقارنات', Icons.compare_arrows),
                const SizedBox(height: 12),
                ComparisonCards(
                  weeklyComparison: dashboard.weeklyComparison,
                  monthlyComparison: dashboard.monthlyComparison,
                ),
                const SizedBox(height: 24),

                // Personal Goals
                _buildSectionHeader(context, 'الأهداف الشخصية', Icons.flag),
                const SizedBox(height: 12),
                PersonalGoalsSection(
                  goals: dashboard.personalGoals,
                ),
                const SizedBox(height: 24),
              ],
            ),
          ),
        ),
        loading: () => const Center(
          child: IslamicLoadingIndicator(),
        ),
        error: (error, stack) => Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(
                Icons.error_outline,
                size: 64,
                color: AppColors.statusError,
              ),
              const SizedBox(height: 16),
              Text(
                'حدث خطأ في تحميل الإحصائيات',
                style: AppTextStyles.body1.copyWith(
                  color: AppColors.textSecondary,
                ),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 8),
              Text(
                error.toString(),
                style: AppTextStyles.body2.copyWith(
                  color: AppColors.textDisabled,
                ),
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 24),
              ElevatedButton.icon(
                onPressed: () {
                  ref.read(statisticsNotifierProvider.notifier).refresh();
                },
                icon: const Icon(Icons.refresh),
                label: const Text('إعادة المحاولة'),
                style: ElevatedButton.styleFrom(
                  backgroundColor: AppColors.primaryMain,
                  foregroundColor: Colors.white,
                  padding: const EdgeInsets.symmetric(
                    horizontal: 24,
                    vertical: 12,
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildSectionHeader(BuildContext context, String title, IconData icon) {
    return Row(
      children: [
        Container(
          padding: const EdgeInsets.all(8),
          decoration: BoxDecoration(
            color: AppColors.primaryMain.withOpacity(0.1),
            borderRadius: BorderRadius.circular(8),
          ),
          child: Icon(
            icon,
            color: AppColors.primaryMain,
            size: 24,
          ),
        ),
        const SizedBox(width: 12),
        Text(
          title,
          style: AppTextStyles.h6.copyWith(
            color: AppColors.textPrimary,
            fontWeight: FontWeight.bold,
          ),
        ),
      ],
    );
  }
}

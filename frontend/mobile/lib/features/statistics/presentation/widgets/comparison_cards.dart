import 'package:flutter/material.dart';
import '../../data/models/statistics_model.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Cards displaying weekly and monthly comparisons
class ComparisonCards extends StatelessWidget {
  final WeeklyComparison weeklyComparison;
  final MonthlyComparison monthlyComparison;

  const ComparisonCards({
    Key? key,
    required this.weeklyComparison,
    required this.monthlyComparison,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // Weekly Comparison
        _buildComparisonCard(
          context,
          title: 'المقارنة الأسبوعية',
          icon: Icons.calendar_view_week,
          currentValue: weeklyComparison.currentWeekMinutes,
          previousValue: weeklyComparison.previousWeekMinutes,
          changePercentage: weeklyComparison.changePercentage,
          trend: weeklyComparison.trend,
        ),
        const SizedBox(height: 16),

        // Monthly Comparison
        _buildComparisonCard(
          context,
          title: 'المقارنة الشهرية',
          icon: Icons.calendar_month,
          currentValue: monthlyComparison.currentMonthMinutes,
          previousValue: monthlyComparison.previousMonthMinutes,
          changePercentage: monthlyComparison.changePercentage,
          trend: monthlyComparison.trend,
        ),
      ],
    );
  }

  Widget _buildComparisonCard(
    BuildContext context, {
    required String title,
    required IconData icon,
    required int currentValue,
    required int previousValue,
    required double changePercentage,
    required String trend,
  }) {
    final isImproving = trend == 'improving';
    final isStable = trend == 'stable';
    final trendColor = isImproving
        ? AppColors.statusSuccess
        : isStable
            ? AppColors.accentGold
            : AppColors.statusError;
    final trendIcon = isImproving
        ? Icons.trending_up
        : isStable
            ? Icons.trending_flat
            : Icons.trending_down;

    return IslamicCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Header
          Row(
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
                style: AppTextStyles.subtitle1.copyWith(
                  color: AppColors.textPrimary,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),

          // Comparison Values
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              // Current Period
              Column(
                children: [
                  Text(
                    'الحالي',
                    style: AppTextStyles.caption.copyWith(
                      color: AppColors.textSecondary,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    '$currentValue د',
                    style: AppTextStyles.h5.copyWith(
                      color: AppColors.primaryMain,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ],
              ),

              // Trend Indicator
              Container(
                padding: const EdgeInsets.symmetric(
                  horizontal: 16,
                  vertical: 8,
                ),
                decoration: BoxDecoration(
                  color: trendColor.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(20),
                ),
                child: Row(
                  children: [
                    Icon(
                      trendIcon,
                      color: trendColor,
                      size: 20,
                    ),
                    const SizedBox(width: 4),
                    Text(
                      '${changePercentage >= 0 ? '+' : ''}${changePercentage.toStringAsFixed(1)}%',
                      style: AppTextStyles.subtitle2.copyWith(
                        color: trendColor,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ],
                ),
              ),

              // Previous Period
              Column(
                children: [
                  Text(
                    'السابق',
                    style: AppTextStyles.caption.copyWith(
                      color: AppColors.textSecondary,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    '$previousValue د',
                    style: AppTextStyles.h5.copyWith(
                      color: AppColors.textSecondary,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ],
              ),
            ],
          ),
          const SizedBox(height: 16),

          // Trend Message
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: trendColor.withOpacity(0.05),
              borderRadius: BorderRadius.circular(8),
              border: Border.all(
                color: trendColor.withOpacity(0.2),
                width: 1,
              ),
            ),
            child: Row(
              children: [
                Icon(
                  _getTrendMessageIcon(trend),
                  color: trendColor,
                  size: 20,
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    _getTrendMessage(trend, changePercentage),
                    style: AppTextStyles.body2.copyWith(
                      color: AppColors.textPrimary,
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  IconData _getTrendMessageIcon(String trend) {
    switch (trend) {
      case 'improving':
        return Icons.celebration;
      case 'stable':
        return Icons.info_outline;
      case 'declining':
        return Icons.warning_amber;
      default:
        return Icons.info_outline;
    }
  }

  String _getTrendMessage(String trend, double changePercentage) {
    switch (trend) {
      case 'improving':
        return 'ممتاز! أنت تتحسن بشكل ملحوظ. استمر على هذا المنوال!';
      case 'stable':
        return 'أداء ثابت. حاول زيادة وقت القراءة قليلاً.';
      case 'declining':
        return 'انخفض وقت القراءة. حاول العودة إلى روتينك المعتاد.';
      default:
        return '';
    }
  }
}

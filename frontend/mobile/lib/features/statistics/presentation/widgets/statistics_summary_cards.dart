import 'package:flutter/material.dart';
import '../../data/models/statistics_model.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Summary cards displaying key statistics at a glance
class StatisticsSummaryCards extends StatelessWidget {
  final StatisticsDashboard dashboard;

  const StatisticsSummaryCards({
    Key? key,
    required this.dashboard,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Row(
          children: [
            Expanded(
              child: _buildSummaryCard(
                title: 'الختمات',
                value: dashboard.khatmaStats.totalCompleted.toString(),
                icon: Icons.book,
                color: AppColors.primaryMain,
                subtitle: 'مكتملة',
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: _buildSummaryCard(
                title: 'القراءة اليوم',
                value: '${dashboard.readingStats.totalMinutesToday}',
                icon: Icons.schedule,
                color: AppColors.secondaryMain,
                subtitle: 'دقيقة',
              ),
            ),
          ],
        ),
        const SizedBox(height: 12),
        Row(
          children: [
            Expanded(
              child: _buildSummaryCard(
                title: 'التلاوة',
                value: '${dashboard.recitationStats.currentScore.toStringAsFixed(0)}%',
                icon: Icons.mic,
                color: AppColors.accentGold,
                subtitle: 'النتيجة',
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: _buildSummaryCard(
                title: 'السلسلة',
                value: dashboard.khatmaStats.currentStreak.toString(),
                icon: Icons.local_fire_department,
                color: AppColors.statusWarning,
                subtitle: 'يوم',
              ),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildSummaryCard({
    required String title,
    required String value,
    required IconData icon,
    required Color color,
    required String subtitle,
  }) {
    return IslamicCard(
      padding: const EdgeInsets.all(16),
      child: Column(
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(
                title,
                style: AppTextStyles.body2.copyWith(
                  color: AppColors.textSecondary,
                ),
              ),
              Icon(
                icon,
                color: color,
                size: 20,
              ),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            value,
            style: AppTextStyles.h4.copyWith(
              color: color,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            subtitle,
            style: AppTextStyles.caption.copyWith(
              color: AppColors.textDisabled,
            ),
          ),
        ],
      ),
    );
  }
}

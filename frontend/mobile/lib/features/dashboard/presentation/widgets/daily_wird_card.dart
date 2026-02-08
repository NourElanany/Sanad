import 'package:flutter/material.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../../../core/services/dashboard_service.dart';

class DailyWirdCard extends StatelessWidget {
  final DailyWird dailyWird;
  final VoidCallback? onTap;

  const DailyWirdCard({
    Key? key,
    required this.dailyWird,
    this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final progressPercentage = dailyWird.progressPercentage;
    final completedPages = dailyWird.completedPages;
    final totalPages = dailyWird.totalPages;

    return IslamicCard(
      onTap: onTap,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header
          Row(
            children: [
              Container(
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: AppColors.secondary.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Icon(
                  Icons.menu_book,
                  color: AppColors.secondary,
                  size: 24,
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'وردك اليومي',
                      style: AppTextStyles.h6.copyWith(
                        color: AppColors.primary,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      '$completedPages من $totalPages صفحات',
                      style: AppTextStyles.bodyMedium.copyWith(
                        color: AppColors.textSecondary,
                      ),
                    ),
                  ],
                ),
              ),
              // Percentage
              Container(
                padding: const EdgeInsets.symmetric(
                  horizontal: 12,
                  vertical: 6,
                ),
                decoration: BoxDecoration(
                  color: _getProgressColor(progressPercentage).withOpacity(0.1),
                  borderRadius: BorderRadius.circular(20),
                ),
                child: Text(
                  '${progressPercentage.toStringAsFixed(0)}%',
                  style: AppTextStyles.bodyMedium.copyWith(
                    color: _getProgressColor(progressPercentage),
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),

          // Progress Bar
          ClipRRect(
            borderRadius: BorderRadius.circular(8),
            child: LinearProgressIndicator(
              value: progressPercentage / 100,
              minHeight: 12,
              backgroundColor: AppColors.backgroundSecondary,
              valueColor: AlwaysStoppedAnimation<Color>(
                _getProgressColor(progressPercentage),
              ),
            ),
          ),
          const SizedBox(height: 12),

          // Motivational Message
          if (progressPercentage < 100)
            Row(
              children: [
                Icon(
                  Icons.emoji_events_outlined,
                  size: 16,
                  color: AppColors.accent,
                ),
                const SizedBox(width: 6),
                Expanded(
                  child: Text(
                    _getMotivationalMessage(progressPercentage),
                    style: AppTextStyles.bodySmall.copyWith(
                      color: AppColors.textSecondary,
                      fontStyle: FontStyle.italic,
                    ),
                  ),
                ),
              ],
            )
          else
            Row(
              children: [
                Icon(
                  Icons.check_circle,
                  size: 16,
                  color: AppColors.success,
                ),
                const SizedBox(width: 6),
                Expanded(
                  child: Text(
                    'ما شاء الله! أكملت وردك اليومي 🎉',
                    style: AppTextStyles.bodySmall.copyWith(
                      color: AppColors.success,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                ),
              ],
            ),
        ],
      ),
    );
  }

  Color _getProgressColor(double percentage) {
    if (percentage >= 100) return AppColors.success;
    if (percentage >= 70) return AppColors.secondary;
    if (percentage >= 40) return AppColors.accent;
    return AppColors.warning;
  }

  String _getMotivationalMessage(double percentage) {
    if (percentage >= 70) return 'أحسنت! أنت قريب من إتمام وردك';
    if (percentage >= 40) return 'استمر! أنت في منتصف الطريق';
    if (percentage >= 20) return 'بداية موفقة! واصل القراءة';
    return 'ابدأ وردك اليومي الآن';
  }
}

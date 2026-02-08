import 'package:flutter/material.dart';
import '../../data/models/statistics_model.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Section displaying personal goals with progress
class PersonalGoalsSection extends StatelessWidget {
  final List<PersonalGoal> goals;

  const PersonalGoalsSection({
    Key? key,
    required this.goals,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    if (goals.isEmpty) {
      return IslamicCard(
        child: Center(
          child: Padding(
            padding: const EdgeInsets.all(32),
            child: Column(
              children: [
                Icon(
                  Icons.flag_outlined,
                  size: 64,
                  color: AppColors.textDisabled,
                ),
                const SizedBox(height: 16),
                Text(
                  'لا توجد أهداف شخصية بعد',
                  style: AppTextStyles.body1.copyWith(
                    color: AppColors.textSecondary,
                  ),
                ),
                const SizedBox(height: 8),
                Text(
                  'أضف أهدافاً لتتبع تقدمك',
                  style: AppTextStyles.body2.copyWith(
                    color: AppColors.textDisabled,
                  ),
                ),
              ],
            ),
          ),
        ),
      );
    }

    return Column(
      children: goals.map((goal) => _buildGoalCard(context, goal)).toList(),
    );
  }

  Widget _buildGoalCard(BuildContext context, PersonalGoal goal) {
    final daysRemaining = goal.deadline.difference(DateTime.now()).inDays;
    final isOverdue = daysRemaining < 0;
    final isNearDeadline = daysRemaining <= 7 && daysRemaining >= 0;

    return Padding(
      padding: const EdgeInsets.only(bottom: 12),
      child: IslamicCard(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Goal Header
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.all(8),
                  decoration: BoxDecoration(
                    color: goal.isCompleted
                        ? AppColors.statusSuccess.withOpacity(0.1)
                        : AppColors.primaryMain.withOpacity(0.1),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Icon(
                    goal.isCompleted ? Icons.check_circle : _getGoalIcon(goal.type),
                    color: goal.isCompleted
                        ? AppColors.statusSuccess
                        : AppColors.primaryMain,
                    size: 24,
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        goal.title,
                        style: AppTextStyles.subtitle1.copyWith(
                          color: AppColors.textPrimary,
                          fontWeight: FontWeight.bold,
                          decoration: goal.isCompleted
                              ? TextDecoration.lineThrough
                              : null,
                        ),
                      ),
                      if (goal.description.isNotEmpty) ...[
                        const SizedBox(height: 4),
                        Text(
                          goal.description,
                          style: AppTextStyles.body2.copyWith(
                            color: AppColors.textSecondary,
                          ),
                          maxLines: 2,
                          overflow: TextOverflow.ellipsis,
                        ),
                      ],
                    ],
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),

            // Progress Bar
            Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      '${goal.currentValue} / ${goal.targetValue}',
                      style: AppTextStyles.body2.copyWith(
                        color: AppColors.textPrimary,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                    Text(
                      '${goal.progressPercentage.toStringAsFixed(0)}%',
                      style: AppTextStyles.body2.copyWith(
                        color: AppColors.primaryMain,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 8),
                ClipRRect(
                  borderRadius: BorderRadius.circular(4),
                  child: LinearProgressIndicator(
                    value: goal.progressPercentage / 100,
                    backgroundColor: AppColors.textDisabled.withOpacity(0.2),
                    valueColor: AlwaysStoppedAnimation<Color>(
                      goal.isCompleted
                          ? AppColors.statusSuccess
                          : AppColors.primaryMain,
                    ),
                    minHeight: 8,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),

            // Deadline Info
            Container(
              padding: const EdgeInsets.all(8),
              decoration: BoxDecoration(
                color: isOverdue
                    ? AppColors.statusError.withOpacity(0.1)
                    : isNearDeadline
                        ? AppColors.statusWarning.withOpacity(0.1)
                        : AppColors.textDisabled.withOpacity(0.05),
                borderRadius: BorderRadius.circular(6),
              ),
              child: Row(
                children: [
                  Icon(
                    isOverdue
                        ? Icons.error_outline
                        : isNearDeadline
                            ? Icons.warning_amber
                            : Icons.calendar_today,
                    size: 16,
                    color: isOverdue
                        ? AppColors.statusError
                        : isNearDeadline
                            ? AppColors.statusWarning
                            : AppColors.textSecondary,
                  ),
                  const SizedBox(width: 8),
                  Text(
                    isOverdue
                        ? 'متأخر ${daysRemaining.abs()} يوم'
                        : isNearDeadline
                            ? 'باقي $daysRemaining أيام'
                            : 'الموعد النهائي: ${_formatDate(goal.deadline)}',
                    style: AppTextStyles.caption.copyWith(
                      color: isOverdue
                          ? AppColors.statusError
                          : isNearDeadline
                              ? AppColors.statusWarning
                              : AppColors.textSecondary,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  IconData _getGoalIcon(GoalType type) {
    switch (type) {
      case GoalType.dailyReading:
        return Icons.today;
      case GoalType.weeklyReading:
        return Icons.date_range;
      case GoalType.monthlyReading:
        return Icons.calendar_month;
      case GoalType.khatmaCompletion:
        return Icons.book;
      case GoalType.recitationImprovement:
        return Icons.mic;
      case GoalType.consistencyStreak:
        return Icons.local_fire_department;
      case GoalType.custom:
        return Icons.flag;
    }
  }

  String _formatDate(DateTime date) {
    return '${date.day}/${date.month}/${date.year}';
  }
}

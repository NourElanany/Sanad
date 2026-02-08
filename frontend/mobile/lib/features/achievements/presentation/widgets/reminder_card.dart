import 'package:flutter/material.dart';
import '../../../achievements/data/models/achievement_model.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Widget to display a motivational reminder
class ReminderCard extends StatelessWidget {
  final MotivationalReminder reminder;
  final VoidCallback? onDismiss;

  const ReminderCard({
    Key? key,
    required this.reminder,
    this.onDismiss,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      child: Row(
        children: [
          // Reminder Icon
          Container(
            width: 48,
            height: 48,
            decoration: BoxDecoration(
              color: _getReminderColor().withOpacity(0.2),
              borderRadius: BorderRadius.circular(12),
            ),
            child: Icon(
              _getReminderIcon(),
              color: _getReminderColor(),
              size: 24,
            ),
          ),
          const SizedBox(width: 12),
          // Reminder Message
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  reminder.messageAr,
                  style: const TextStyle(
                    fontSize: 14,
                    fontFamily: 'Tajawal',
                    color: Color(0xFF1A1A1A),
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  _formatScheduledTime(),
                  style: const TextStyle(
                    fontSize: 12,
                    color: Color(0xFF666666),
                    fontFamily: 'Tajawal',
                  ),
                ),
              ],
            ),
          ),
          // Dismiss Button
          if (onDismiss != null)
            IconButton(
              icon: const Icon(Icons.close),
              color: const Color(0xFF666666),
              iconSize: 20,
              onPressed: onDismiss,
            ),
        ],
      ),
    );
  }

  Color _getReminderColor() {
    switch (reminder.type) {
      case ReminderType.achievementProgress:
        return const Color(0xFF17A2B8);
      case ReminderType.challengeDeadline:
        return const Color(0xFFFFC107);
      case ReminderType.streakMaintenance:
        return const Color(0xFFFF6B6B);
      case ReminderType.levelUp:
        return const Color(0xFFB8860B);
      case ReminderType.general:
        return const Color(0xFF6F42C1);
    }
  }

  IconData _getReminderIcon() {
    switch (reminder.type) {
      case ReminderType.achievementProgress:
        return Icons.trending_up;
      case ReminderType.challengeDeadline:
        return Icons.timer;
      case ReminderType.streakMaintenance:
        return Icons.local_fire_department;
      case ReminderType.levelUp:
        return Icons.arrow_upward;
      case ReminderType.general:
        return Icons.notifications_active;
    }
  }

  String _formatScheduledTime() {
    final now = DateTime.now();
    final scheduled = reminder.scheduledFor;
    final difference = scheduled.difference(now);

    if (difference.isNegative) {
      return 'الآن';
    } else if (difference.inMinutes < 60) {
      return 'خلال ${difference.inMinutes} دقيقة';
    } else if (difference.inHours < 24) {
      return 'خلال ${difference.inHours} ساعة';
    } else {
      return 'خلال ${difference.inDays} يوم';
    }
  }
}

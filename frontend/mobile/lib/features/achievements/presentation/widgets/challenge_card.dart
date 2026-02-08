import 'package:flutter/material.dart';
import '../../../achievements/data/models/achievement_model.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Widget to display a daily or weekly challenge
class ChallengeCard extends StatelessWidget {
  final Challenge challenge;
  final VoidCallback? onTap;

  const ChallengeCard({
    Key? key,
    required this.challenge,
    this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      onTap: onTap,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Row(
            children: [
              // Challenge Icon
              Container(
                width: 48,
                height: 48,
                decoration: BoxDecoration(
                  color: _getDifficultyColor().withOpacity(0.2),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Icon(
                  _getChallengeIcon(),
                  color: _getDifficultyColor(),
                  size: 24,
                ),
              ),
              const SizedBox(width: 12),
              // Challenge Info
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        Expanded(
                          child: Text(
                            challenge.titleAr,
                            style: const TextStyle(
                              fontSize: 16,
                              fontWeight: FontWeight.bold,
                              fontFamily: 'Tajawal',
                            ),
                          ),
                        ),
                        _buildTypeBadge(),
                      ],
                    ),
                    const SizedBox(height: 4),
                    Row(
                      children: [
                        Icon(
                          Icons.timer_outlined,
                          size: 16,
                          color: challenge.isExpired
                              ? const Color(0xFFDC3545)
                              : const Color(0xFF666666),
                        ),
                        const SizedBox(width: 4),
                        Text(
                          _getTimeRemaining(),
                          style: TextStyle(
                            fontSize: 12,
                            color: challenge.isExpired
                                ? const Color(0xFFDC3545)
                                : const Color(0xFF666666),
                            fontFamily: 'Tajawal',
                          ),
                        ),
                        const Spacer(),
                        const Icon(
                          Icons.stars,
                          color: Color(0xFFB8860B),
                          size: 16,
                        ),
                        const SizedBox(width: 4),
                        Text(
                          '${challenge.pointsReward} نقطة',
                          style: const TextStyle(
                            fontSize: 12,
                            fontWeight: FontWeight.bold,
                            color: Color(0xFFB8860B),
                            fontFamily: 'Tajawal',
                          ),
                        ),
                      ],
                    ),
                  ],
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          // Description
          Text(
            challenge.descriptionAr,
            style: const TextStyle(
              fontSize: 14,
              color: Color(0xFF666666),
              fontFamily: 'Tajawal',
            ),
          ),
          const SizedBox(height: 12),
          // Progress
          Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    'التقدم',
                    style: const TextStyle(
                      fontSize: 14,
                      fontWeight: FontWeight.bold,
                      fontFamily: 'Tajawal',
                    ),
                  ),
                  Text(
                    '${challenge.currentProgress} / ${challenge.targetValue}',
                    style: const TextStyle(
                      fontSize: 14,
                      fontWeight: FontWeight.bold,
                      fontFamily: 'Tajawal',
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 8),
              ClipRRect(
                borderRadius: BorderRadius.circular(8),
                child: LinearProgressIndicator(
                  value: challenge.progressPercentage / 100,
                  backgroundColor: const Color(0xFFE0E0E0),
                  valueColor: AlwaysStoppedAnimation<Color>(
                    challenge.isCompleted
                        ? const Color(0xFF28A745)
                        : _getDifficultyColor(),
                  ),
                  minHeight: 10,
                ),
              ),
              const SizedBox(height: 4),
              Text(
                '${challenge.progressPercentage.toStringAsFixed(0)}% مكتمل',
                textAlign: TextAlign.center,
                style: const TextStyle(
                  fontSize: 12,
                  color: Color(0xFF666666),
                  fontFamily: 'Tajawal',
                ),
              ),
            ],
          ),
          // Completion Badge
          if (challenge.isCompleted) ...[
            const SizedBox(height: 12),
            Container(
              padding: const EdgeInsets.symmetric(vertical: 8),
              decoration: BoxDecoration(
                color: const Color(0xFF28A745).withOpacity(0.1),
                borderRadius: BorderRadius.circular(8),
                border: Border.all(
                  color: const Color(0xFF28A745),
                  width: 1,
                ),
              ),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  const Icon(
                    Icons.check_circle,
                    color: Color(0xFF28A745),
                    size: 20,
                  ),
                  const SizedBox(width: 8),
                  const Text(
                    'تم إكمال التحدي!',
                    style: TextStyle(
                      fontSize: 14,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFF28A745),
                      fontFamily: 'Tajawal',
                    ),
                  ),
                ],
              ),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildTypeBadge() {
    String typeText;
    Color typeColor;

    switch (challenge.type) {
      case ChallengeType.daily:
        typeText = 'يومي';
        typeColor = const Color(0xFF17A2B8);
        break;
      case ChallengeType.weekly:
        typeText = 'أسبوعي';
        typeColor = const Color(0xFF6F42C1);
        break;
      case ChallengeType.special:
        typeText = 'خاص';
        typeColor = const Color(0xFFFFD700);
        break;
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: typeColor.withOpacity(0.2),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: typeColor,
          width: 1,
        ),
      ),
      child: Text(
        typeText,
        style: TextStyle(
          fontSize: 10,
          fontWeight: FontWeight.bold,
          color: typeColor,
          fontFamily: 'Tajawal',
        ),
      ),
    );
  }

  Color _getDifficultyColor() {
    switch (challenge.difficulty) {
      case ChallengeDifficulty.easy:
        return const Color(0xFF28A745);
      case ChallengeDifficulty.medium:
        return const Color(0xFFFFC107);
      case ChallengeDifficulty.hard:
        return const Color(0xFFFF6B6B);
      case ChallengeDifficulty.expert:
        return const Color(0xFF6F42C1);
    }
  }

  IconData _getChallengeIcon() {
    // Map icon names to Flutter icons
    switch (challenge.iconName) {
      case 'book':
        return Icons.menu_book;
      case 'target':
        return Icons.track_changes;
      case 'fire':
        return Icons.local_fire_department;
      case 'star':
        return Icons.star;
      case 'trophy':
        return Icons.emoji_events;
      default:
        return Icons.flag;
    }
  }

  String _getTimeRemaining() {
    if (challenge.isExpired) {
      return 'منتهي';
    }

    final remaining = challenge.timeRemaining;

    if (remaining.inDays > 0) {
      return 'باقي ${remaining.inDays} يوم';
    } else if (remaining.inHours > 0) {
      return 'باقي ${remaining.inHours} ساعة';
    } else if (remaining.inMinutes > 0) {
      return 'باقي ${remaining.inMinutes} دقيقة';
    } else {
      return 'ينتهي قريباً';
    }
  }
}

import 'package:flutter/material.dart';
import 'package:share_plus/share_plus.dart';
import '../../../achievements/data/models/achievement_model.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Widget to display an achievement badge
class AchievementCard extends StatelessWidget {
  final Achievement achievement;
  final VoidCallback? onTap;

  const AchievementCard({
    Key? key,
    required this.achievement,
    this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      onTap: onTap ?? () => _showAchievementDetails(context),
      child: Opacity(
        opacity: achievement.isUnlocked ? 1.0 : 0.6,
        child: Row(
          children: [
            // Achievement Icon
            Container(
              width: 64,
              height: 64,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                gradient: _getTierGradient(),
                boxShadow: achievement.isUnlocked
                    ? [
                        BoxShadow(
                          color: _getTierColor().withOpacity(0.3),
                          blurRadius: 12,
                          offset: const Offset(0, 4),
                        ),
                      ]
                    : null,
              ),
              child: Center(
                child: Icon(
                  _getAchievementIcon(),
                  color: Colors.white,
                  size: 32,
                ),
              ),
            ),
            const SizedBox(width: 16),
            // Achievement Info
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Expanded(
                        child: Text(
                          achievement.titleAr,
                          style: TextStyle(
                            fontSize: 16,
                            fontWeight: FontWeight.bold,
                            fontFamily: 'Tajawal',
                            color: achievement.isUnlocked
                                ? const Color(0xFF1A1A1A)
                                : const Color(0xFF666666),
                          ),
                        ),
                      ),
                      _buildTierBadge(),
                    ],
                  ),
                  const SizedBox(height: 4),
                  Text(
                    achievement.descriptionAr,
                    style: const TextStyle(
                      fontSize: 14,
                      color: Color(0xFF666666),
                      fontFamily: 'Tajawal',
                    ),
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                  ),
                  const SizedBox(height: 8),
                  // Progress Bar
                  if (!achievement.isUnlocked) ...[
                    ClipRRect(
                      borderRadius: BorderRadius.circular(4),
                      child: LinearProgressIndicator(
                        value: achievement.progress,
                        backgroundColor: const Color(0xFFE0E0E0),
                        valueColor: AlwaysStoppedAnimation<Color>(
                          _getTierColor(),
                        ),
                        minHeight: 6,
                      ),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      '${achievement.currentValue} / ${achievement.targetValue}',
                      style: const TextStyle(
                        fontSize: 12,
                        color: Color(0xFF666666),
                        fontFamily: 'Tajawal',
                      ),
                    ),
                  ],
                  // Points Reward
                  Row(
                    children: [
                      const Icon(
                        Icons.stars,
                        color: Color(0xFFB8860B),
                        size: 16,
                      ),
                      const SizedBox(width: 4),
                      Text(
                        '${achievement.pointsReward} نقطة',
                        style: const TextStyle(
                          fontSize: 14,
                          fontWeight: FontWeight.bold,
                          color: Color(0xFFB8860B),
                          fontFamily: 'Tajawal',
                        ),
                      ),
                      if (achievement.isUnlocked && achievement.unlockedAt != null) ...[
                        const Spacer(),
                        Text(
                          _formatUnlockDate(achievement.unlockedAt!),
                          style: const TextStyle(
                            fontSize: 12,
                            color: Color(0xFF666666),
                            fontFamily: 'Tajawal',
                          ),
                        ),
                      ],
                    ],
                  ),
                ],
              ),
            ),
            // Share Button (if unlocked)
            if (achievement.isUnlocked) ...[
              const SizedBox(width: 8),
              IconButton(
                icon: const Icon(Icons.share),
                color: const Color(0xFF1B365D),
                onPressed: () => _shareAchievement(context),
              ),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildTierBadge() {
    final tierName = _getTierName();
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: _getTierColor().withOpacity(0.2),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: _getTierColor(),
          width: 1,
        ),
      ),
      child: Text(
        tierName,
        style: TextStyle(
          fontSize: 10,
          fontWeight: FontWeight.bold,
          color: _getTierColor(),
          fontFamily: 'Tajawal',
        ),
      ),
    );
  }

  LinearGradient _getTierGradient() {
    switch (achievement.tier) {
      case AchievementTier.bronze:
        return const LinearGradient(
          colors: [Color(0xFFCD7F32), Color(0xFF8B4513)],
        );
      case AchievementTier.silver:
        return const LinearGradient(
          colors: [Color(0xFFC0C0C0), Color(0xFF808080)],
        );
      case AchievementTier.gold:
        return const LinearGradient(
          colors: [Color(0xFFFFD700), Color(0xFFB8860B)],
        );
      case AchievementTier.platinum:
        return const LinearGradient(
          colors: [Color(0xFFE5E4E2), Color(0xFF9C9C9C)],
        );
      case AchievementTier.diamond:
        return const LinearGradient(
          colors: [Color(0xFFB9F2FF), Color(0xFF4A90E2)],
        );
    }
  }

  Color _getTierColor() {
    switch (achievement.tier) {
      case AchievementTier.bronze:
        return const Color(0xFFCD7F32);
      case AchievementTier.silver:
        return const Color(0xFFC0C0C0);
      case AchievementTier.gold:
        return const Color(0xFFFFD700);
      case AchievementTier.platinum:
        return const Color(0xFFE5E4E2);
      case AchievementTier.diamond:
        return const Color(0xFFB9F2FF);
    }
  }

  String _getTierName() {
    switch (achievement.tier) {
      case AchievementTier.bronze:
        return 'برونزي';
      case AchievementTier.silver:
        return 'فضي';
      case AchievementTier.gold:
        return 'ذهبي';
      case AchievementTier.platinum:
        return 'بلاتيني';
      case AchievementTier.diamond:
        return 'ماسي';
    }
  }

  IconData _getAchievementIcon() {
    // Map icon names to Flutter icons
    switch (achievement.iconName) {
      case 'book':
        return Icons.menu_book;
      case 'star':
        return Icons.star;
      case 'trophy':
        return Icons.emoji_events;
      case 'fire':
        return Icons.local_fire_department;
      case 'target':
        return Icons.track_changes;
      case 'prayer':
        return Icons.mosque;
      case 'quran':
        return Icons.auto_stories;
      default:
        return Icons.emoji_events;
    }
  }

  String _formatUnlockDate(DateTime date) {
    final now = DateTime.now();
    final difference = now.difference(date);

    if (difference.inDays == 0) {
      return 'اليوم';
    } else if (difference.inDays == 1) {
      return 'أمس';
    } else if (difference.inDays < 7) {
      return 'منذ ${difference.inDays} أيام';
    } else if (difference.inDays < 30) {
      return 'منذ ${(difference.inDays / 7).floor()} أسابيع';
    } else {
      return 'منذ ${(difference.inDays / 30).floor()} شهر';
    }
  }

  void _showAchievementDetails(BuildContext context) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (context) => Container(
        decoration: const BoxDecoration(
          color: Colors.white,
          borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
        ),
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            // Achievement Icon
            Container(
              width: 100,
              height: 100,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                gradient: _getTierGradient(),
              ),
              child: Center(
                child: Icon(
                  _getAchievementIcon(),
                  color: Colors.white,
                  size: 48,
                ),
              ),
            ),
            const SizedBox(height: 16),
            Text(
              achievement.titleAr,
              style: const TextStyle(
                fontSize: 24,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 8),
            Text(
              achievement.descriptionAr,
              style: const TextStyle(
                fontSize: 16,
                color: Color(0xFF666666),
                fontFamily: 'Tajawal',
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 16),
            // Requirements
            if (achievement.requirements.isNotEmpty) ...[
              const Align(
                alignment: Alignment.centerRight,
                child: Text(
                  'المتطلبات:',
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    fontFamily: 'Tajawal',
                  ),
                ),
              ),
              const SizedBox(height: 8),
              ...achievement.requirements.map((req) => Padding(
                    padding: const EdgeInsets.only(bottom: 4),
                    child: Row(
                      children: [
                        const Icon(
                          Icons.check_circle_outline,
                          size: 20,
                          color: Color(0xFF28A745),
                        ),
                        const SizedBox(width: 8),
                        Expanded(
                          child: Text(
                            req,
                            style: const TextStyle(
                              fontSize: 14,
                              fontFamily: 'Tajawal',
                            ),
                          ),
                        ),
                      ],
                    ),
                  )),
              const SizedBox(height: 16),
            ],
            // Action Buttons
            if (achievement.isUnlocked)
              ElevatedButton.icon(
                onPressed: () {
                  Navigator.pop(context);
                  _shareAchievement(context);
                },
                icon: const Icon(Icons.share),
                label: const Text(
                  'مشاركة الإنجاز',
                  style: TextStyle(fontFamily: 'Tajawal'),
                ),
                style: ElevatedButton.styleFrom(
                  backgroundColor: const Color(0xFF1B365D),
                  minimumSize: const Size(double.infinity, 48),
                ),
              )
            else
              ElevatedButton(
                onPressed: () => Navigator.pop(context),
                style: ElevatedButton.styleFrom(
                  backgroundColor: const Color(0xFF666666),
                  minimumSize: const Size(double.infinity, 48),
                ),
                child: const Text(
                  'إغلاق',
                  style: TextStyle(fontFamily: 'Tajawal'),
                ),
              ),
          ],
        ),
      ),
    );
  }

  void _shareAchievement(BuildContext context) {
    final shareText = '''
🎉 حصلت على إنجاز جديد في تطبيق سند!

${achievement.titleAr}
${achievement.descriptionAr}

⭐ ${achievement.pointsReward} نقطة

#سند #إنجازات_إسلامية
''';

    Share.share(shareText);
  }
}

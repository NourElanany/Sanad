import 'package:flutter/material.dart';
import '../../../achievements/data/models/achievement_model.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Widget to display user level and progress
class UserLevelCard extends StatelessWidget {
  final UserLevel userLevel;

  const UserLevelCard({
    Key? key,
    required this.userLevel,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      child: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              const Color(0xFF1B365D),
              const Color(0xFF2E4A6B),
            ],
          ),
          borderRadius: BorderRadius.circular(16),
        ),
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Row(
              children: [
                // Level Badge
                Container(
                  width: 80,
                  height: 80,
                  decoration: BoxDecoration(
                    shape: BoxShape.circle,
                    color: const Color(0xFFB8860B).withOpacity(0.2),
                    border: Border.all(
                      color: const Color(0xFFB8860B),
                      width: 3,
                    ),
                  ),
                  child: Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Text(
                          '${userLevel.currentLevel}',
                          style: const TextStyle(
                            fontSize: 32,
                            fontWeight: FontWeight.bold,
                            color: Colors.white,
                            fontFamily: 'Tajawal',
                          ),
                        ),
                        const Text(
                          'المستوى',
                          style: TextStyle(
                            fontSize: 12,
                            color: Colors.white70,
                            fontFamily: 'Tajawal',
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(width: 16),
                // Level Info
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        userLevel.levelTitleAr,
                        style: const TextStyle(
                          fontSize: 24,
                          fontWeight: FontWeight.bold,
                          color: Colors.white,
                          fontFamily: 'Tajawal',
                        ),
                      ),
                      const SizedBox(height: 4),
                      Row(
                        children: [
                          const Icon(
                            Icons.stars,
                            color: Color(0xFFB8860B),
                            size: 20,
                          ),
                          const SizedBox(width: 4),
                          Text(
                            '${userLevel.totalPoints} نقطة',
                            style: const TextStyle(
                              fontSize: 16,
                              color: Colors.white70,
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
            const SizedBox(height: 20),
            // Progress to Next Level
            Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      'التقدم للمستوى التالي',
                      style: const TextStyle(
                        fontSize: 14,
                        color: Colors.white70,
                        fontFamily: 'Tajawal',
                      ),
                    ),
                    Text(
                      '${userLevel.pointsInCurrentLevel} / ${userLevel.pointsRequiredForNextLevel}',
                      style: const TextStyle(
                        fontSize: 14,
                        color: Colors.white,
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
                    value: userLevel.progressToNextLevel,
                    backgroundColor: Colors.white24,
                    valueColor: const AlwaysStoppedAnimation<Color>(
                      Color(0xFFB8860B),
                    ),
                    minHeight: 12,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  '${(userLevel.progressToNextLevel * 100).toStringAsFixed(0)}% مكتمل',
                  textAlign: TextAlign.center,
                  style: const TextStyle(
                    fontSize: 12,
                    color: Colors.white70,
                    fontFamily: 'Tajawal',
                  ),
                ),
              ],
            ),
            // Unlocked Perks
            if (userLevel.unlockedPerks.isNotEmpty) ...[
              const SizedBox(height: 16),
              const Divider(color: Colors.white24),
              const SizedBox(height: 12),
              const Text(
                'المزايا المفتوحة:',
                style: TextStyle(
                  fontSize: 14,
                  color: Colors.white70,
                  fontFamily: 'Tajawal',
                ),
              ),
              const SizedBox(height: 8),
              Wrap(
                spacing: 8,
                runSpacing: 8,
                children: userLevel.unlockedPerks.map((perk) {
                  return Container(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 12,
                      vertical: 6,
                    ),
                    decoration: BoxDecoration(
                      color: const Color(0xFFB8860B).withOpacity(0.2),
                      borderRadius: BorderRadius.circular(16),
                      border: Border.all(
                        color: const Color(0xFFB8860B),
                        width: 1,
                      ),
                    ),
                    child: Text(
                      perk,
                      style: const TextStyle(
                        fontSize: 12,
                        color: Colors.white,
                        fontFamily: 'Tajawal',
                      ),
                    ),
                  );
                }).toList(),
              ),
            ],
          ],
        ),
      ),
    );
  }
}

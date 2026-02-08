import 'package:flutter/material.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../data/models/story_model.dart';

/// Card for displaying a story category
class StoryCategoryCard extends StatelessWidget {
  final StoryCategory category;
  final VoidCallback onTap;

  const StoryCategoryCard({
    Key? key,
    required this.category,
    required this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      onTap: onTap,
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Text(
            category.icon,
            style: const TextStyle(fontSize: 48),
          ),
          const SizedBox(height: 12),
          Text(
            category.arabicName,
            textAlign: TextAlign.center,
            style: const TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
            ),
          ),
        ],
      ),
    );
  }
}

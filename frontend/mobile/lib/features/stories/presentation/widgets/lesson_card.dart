import 'package:flutter/material.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../data/models/story_model.dart';

/// Card for displaying a lesson from a story
class LessonCard extends StatelessWidget {
  final LessonModel lesson;
  final double relevanceScore;
  final String? explanation;

  const LessonCard({
    Key? key,
    required this.lesson,
    required this.relevanceScore,
    this.explanation,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(
                _getLessonIcon(),
                color: _getLessonColor(),
                size: 24,
              ),
              const SizedBox(width: 8),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      lesson.arabicTitle,
                      style: const TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.bold,
                        fontFamily: 'Tajawal',
                      ),
                    ),
                    const SizedBox(height: 2),
                    Row(
                      children: [
                        _buildBadge(
                          lesson.lessonTypeArabic,
                          Colors.blue,
                        ),
                        const SizedBox(width: 4),
                        _buildBadge(
                          lesson.moralCategoryArabic,
                          Colors.purple,
                        ),
                      ],
                    ),
                  ],
                ),
              ),
              _buildRelevanceScore(relevanceScore),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            lesson.description,
            style: const TextStyle(
              fontSize: 14,
              fontFamily: 'Tajawal',
            ),
          ),
          if (explanation != null) ...[
            const SizedBox(height: 8),
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: Colors.amber.withOpacity(0.1),
                borderRadius: BorderRadius.circular(8),
                border: Border.all(
                  color: Colors.amber.withOpacity(0.3),
                ),
              ),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Icon(
                    Icons.lightbulb,
                    size: 16,
                    color: Colors.amber,
                  ),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Text(
                      explanation!,
                      style: const TextStyle(
                        fontSize: 13,
                        fontFamily: 'Tajawal',
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ],
          if (lesson.practicalApplication != null) ...[
            const SizedBox(height: 8),
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: Colors.green.withOpacity(0.1),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Icon(
                    Icons.check_circle,
                    size: 16,
                    color: Colors.green,
                  ),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text(
                          'التطبيق العملي:',
                          style: TextStyle(
                            fontSize: 12,
                            fontWeight: FontWeight.bold,
                            color: Colors.green,
                            fontFamily: 'Tajawal',
                          ),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          lesson.practicalApplication!,
                          style: const TextStyle(
                            fontSize: 13,
                            fontFamily: 'Tajawal',
                          ),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ],
          if (lesson.relatedVerses.isNotEmpty ||
              lesson.relatedHadiths.isNotEmpty) ...[
            const SizedBox(height: 12),
            const Divider(),
            const SizedBox(height: 8),
            if (lesson.relatedVerses.isNotEmpty) ...[
              _buildReferences(
                'آيات قرآنية',
                lesson.relatedVerses,
                Icons.menu_book,
                Colors.green,
              ),
              if (lesson.relatedHadiths.isNotEmpty) const SizedBox(height: 8),
            ],
            if (lesson.relatedHadiths.isNotEmpty) ...[
              _buildReferences(
                'أحاديث نبوية',
                lesson.relatedHadiths,
                Icons.format_quote,
                Colors.blue,
              ),
            ],
          ],
        ],
      ),
    );
  }

  IconData _getLessonIcon() {
    switch (lesson.lessonType) {
      case LessonType.moral:
        return Icons.favorite;
      case LessonType.spiritual:
        return Icons.spa;
      case LessonType.practical:
        return Icons.build;
      case LessonType.historical:
        return Icons.history;
      case LessonType.theological:
        return Icons.auto_stories;
      case LessonType.social:
        return Icons.people;
    }
  }

  Color _getLessonColor() {
    switch (lesson.lessonType) {
      case LessonType.moral:
        return Colors.pink;
      case LessonType.spiritual:
        return Colors.purple;
      case LessonType.practical:
        return Colors.orange;
      case LessonType.historical:
        return Colors.brown;
      case LessonType.theological:
        return Colors.indigo;
      case LessonType.social:
        return Colors.teal;
    }
  }

  Widget _buildBadge(String label, Color color) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Text(
        label,
        style: TextStyle(
          fontSize: 10,
          fontWeight: FontWeight.bold,
          color: color,
          fontFamily: 'Tajawal',
        ),
      ),
    );
  }

  Widget _buildRelevanceScore(double score) {
    final percentage = (score / 10.0 * 100).toInt();
    final color = score >= 8.0
        ? Colors.green
        : score >= 6.0
            ? Colors.orange
            : Colors.grey;

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(Icons.star, size: 14, color: color),
          const SizedBox(width: 4),
          Text(
            '$percentage%',
            style: TextStyle(
              fontSize: 12,
              fontWeight: FontWeight.bold,
              color: color,
              fontFamily: 'Tajawal',
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildReferences(
    String title,
    List<String> references,
    IconData icon,
    Color color,
  ) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Icon(icon, size: 14, color: color),
            const SizedBox(width: 4),
            Text(
              title,
              style: TextStyle(
                fontSize: 12,
                fontWeight: FontWeight.bold,
                color: color,
                fontFamily: 'Tajawal',
              ),
            ),
          ],
        ),
        const SizedBox(height: 4),
        Wrap(
          spacing: 4,
          runSpacing: 4,
          children: references.map((ref) {
            return Container(
              padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
              decoration: BoxDecoration(
                color: color.withOpacity(0.1),
                borderRadius: BorderRadius.circular(8),
                border: Border.all(color: color.withOpacity(0.3)),
              ),
              child: Text(
                ref,
                style: TextStyle(
                  fontSize: 11,
                  color: color,
                  fontFamily: 'Tajawal',
                ),
              ),
            );
          }).toList(),
        ),
      ],
    );
  }
}

import 'package:flutter/material.dart';
import '../../data/models/recording_models.dart';

/// Card widget for displaying improvement recommendations
class RecommendationCard extends StatefulWidget {
  final Recommendation recommendation;

  const RecommendationCard({
    Key? key,
    required this.recommendation,
  }) : super(key: key);

  @override
  State<RecommendationCard> createState() => _RecommendationCardState();
}

class _RecommendationCardState extends State<RecommendationCard> {
  bool _isExpanded = false;

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: _getPriorityColor(widget.recommendation.priority).withOpacity(0.3),
          width: 1.5,
        ),
      ),
      child: Column(
        children: [
          InkWell(
            onTap: () => setState(() => _isExpanded = !_isExpanded),
            borderRadius: BorderRadius.circular(12),
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                children: [
                  // Category icon
                  Container(
                    padding: const EdgeInsets.all(8),
                    decoration: BoxDecoration(
                      color: _getCategoryColor(widget.recommendation.category).withOpacity(0.1),
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Icon(
                      _getCategoryIcon(widget.recommendation.category),
                      color: _getCategoryColor(widget.recommendation.category),
                      size: 24,
                    ),
                  ),
                  const SizedBox(width: 12),

                  // Recommendation info
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            Expanded(
                              child: Text(
                                _getCategoryLabel(widget.recommendation.category),
                                style: const TextStyle(
                                  fontSize: 16,
                                  fontWeight: FontWeight.bold,
                                  fontFamily: 'Tajawal',
                                  color: Color(0xFF1A1A1A),
                                ),
                              ),
                            ),
                            Container(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 8,
                                vertical: 4,
                              ),
                              decoration: BoxDecoration(
                                color: _getPriorityColor(widget.recommendation.priority)
                                    .withOpacity(0.2),
                                borderRadius: BorderRadius.circular(8),
                              ),
                              child: Text(
                                _getPriorityLabel(widget.recommendation.priority),
                                style: TextStyle(
                                  fontSize: 11,
                                  fontWeight: FontWeight.bold,
                                  fontFamily: 'Tajawal',
                                  color: _getPriorityColor(widget.recommendation.priority),
                                ),
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 4),
                        Text(
                          widget.recommendation.description,
                          style: const TextStyle(
                            fontSize: 14,
                            fontFamily: 'Tajawal',
                            color: Color(0xFF666666),
                          ),
                          maxLines: _isExpanded ? null : 2,
                          overflow: _isExpanded ? null : TextOverflow.ellipsis,
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(width: 8),

                  // Expand icon
                  Icon(
                    _isExpanded ? Icons.expand_less : Icons.expand_more,
                    color: const Color(0xFF666666),
                  ),
                ],
              ),
            ),
          ),

          // Expanded content
          if (_isExpanded)
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: const Color(0xFFF8F9FA),
                borderRadius: const BorderRadius.only(
                  bottomLeft: Radius.circular(12),
                  bottomRight: Radius.circular(12),
                ),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // Specific advice
                  Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Icon(
                        Icons.tips_and_updates,
                        color: Color(0xFFB8860B),
                        size: 20,
                      ),
                      const SizedBox(width: 8),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            const Text(
                              'نصيحة محددة',
                              style: TextStyle(
                                fontSize: 14,
                                fontWeight: FontWeight.bold,
                                fontFamily: 'Tajawal',
                                color: Color(0xFF1A1A1A),
                              ),
                            ),
                            const SizedBox(height: 4),
                            Text(
                              widget.recommendation.specificAdvice,
                              style: const TextStyle(
                                fontSize: 14,
                                fontFamily: 'Tajawal',
                                color: Color(0xFF666666),
                              ),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),

                  // Practice exercises
                  if (widget.recommendation.practiceExercises.isNotEmpty) ...[
                    const SizedBox(height: 16),
                    Row(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Icon(
                          Icons.fitness_center,
                          color: Color(0xFF2D5A27),
                          size: 20,
                        ),
                        const SizedBox(width: 8),
                        Expanded(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              const Text(
                                'تمارين مقترحة',
                                style: TextStyle(
                                  fontSize: 14,
                                  fontWeight: FontWeight.bold,
                                  fontFamily: 'Tajawal',
                                  color: Color(0xFF1A1A1A),
                                ),
                              ),
                              const SizedBox(height: 8),
                              ...widget.recommendation.practiceExercises
                                  .asMap()
                                  .entries
                                  .map((entry) {
                                return Padding(
                                  padding: const EdgeInsets.only(bottom: 6),
                                  child: Row(
                                    crossAxisAlignment: CrossAxisAlignment.start,
                                    children: [
                                      Container(
                                        margin: const EdgeInsets.only(top: 6),
                                        width: 6,
                                        height: 6,
                                        decoration: const BoxDecoration(
                                          color: Color(0xFF2D5A27),
                                          shape: BoxShape.circle,
                                        ),
                                      ),
                                      const SizedBox(width: 8),
                                      Expanded(
                                        child: Text(
                                          entry.value,
                                          style: const TextStyle(
                                            fontSize: 13,
                                            fontFamily: 'Tajawal',
                                            color: Color(0xFF666666),
                                          ),
                                        ),
                                      ),
                                    ],
                                  ),
                                );
                              }).toList(),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ],
                ],
              ),
            ),
        ],
      ),
    );
  }

  Color _getPriorityColor(String priority) {
    switch (priority.toLowerCase()) {
      case 'high':
        return const Color(0xFFDC3545); // Red
      case 'medium':
        return const Color(0xFFFFC107); // Yellow
      case 'low':
        return const Color(0xFF17A2B8); // Blue
      default:
        return const Color(0xFF666666); // Gray
    }
  }

  String _getPriorityLabel(String priority) {
    switch (priority.toLowerCase()) {
      case 'high':
        return 'عالي';
      case 'medium':
        return 'متوسط';
      case 'low':
        return 'منخفض';
      default:
        return 'غير محدد';
    }
  }

  Color _getCategoryColor(String category) {
    switch (category.toLowerCase()) {
      case 'pronunciation':
        return const Color(0xFF1B365D); // Navy
      case 'timing':
        return const Color(0xFFB8860B); // Gold
      case 'tajweed':
        return const Color(0xFF2D5A27); // Green
      case 'fluency':
        return const Color(0xFF17A2B8); // Blue
      case 'general':
        return const Color(0xFF666666); // Gray
      default:
        return const Color(0xFF666666);
    }
  }

  IconData _getCategoryIcon(String category) {
    switch (category.toLowerCase()) {
      case 'pronunciation':
        return Icons.record_voice_over;
      case 'timing':
        return Icons.timer;
      case 'tajweed':
        return Icons.check_circle;
      case 'fluency':
        return Icons.trending_up;
      case 'general':
        return Icons.info;
      default:
        return Icons.help;
    }
  }

  String _getCategoryLabel(String category) {
    switch (category.toLowerCase()) {
      case 'pronunciation':
        return 'النطق';
      case 'timing':
        return 'التوقيت';
      case 'tajweed':
        return 'التجويد';
      case 'fluency':
        return 'الطلاقة';
      case 'general':
        return 'عام';
      default:
        return category;
    }
  }
}

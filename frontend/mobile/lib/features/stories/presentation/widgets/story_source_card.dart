import 'package:flutter/material.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../data/models/story_model.dart';

/// Card for displaying a story source/reference
class StorySourceCard extends StatelessWidget {
  final StorySourceModel source;

  const StorySourceCard({
    Key? key,
    required this.source,
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
                _getSourceIcon(),
                color: _getSourceColor(),
                size: 24,
              ),
              const SizedBox(width: 8),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      source.arabicSourceName,
                      style: const TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.bold,
                        fontFamily: 'Tajawal',
                      ),
                    ),
                    if (source.author != null) ...[
                      const SizedBox(height: 2),
                      Text(
                        source.author!,
                        style: const TextStyle(
                          fontSize: 12,
                          color: Colors.grey,
                          fontFamily: 'Tajawal',
                        ),
                      ),
                    ],
                  ],
                ),
              ),
              if (source.isPrimarySource)
                Container(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 8,
                    vertical: 4,
                  ),
                  decoration: BoxDecoration(
                    color: Colors.green.withOpacity(0.1),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: const Text(
                    'مصدر أساسي',
                    style: TextStyle(
                      fontSize: 10,
                      fontWeight: FontWeight.bold,
                      color: Colors.green,
                      fontFamily: 'Tajawal',
                    ),
                  ),
                ),
            ],
          ),
          const SizedBox(height: 8),
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: Colors.grey.withOpacity(0.05),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Row(
              children: [
                const Icon(Icons.bookmark, size: 16, color: Colors.grey),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    source.reference,
                    style: const TextStyle(
                      fontSize: 14,
                      fontFamily: 'Tajawal',
                    ),
                  ),
                ),
              ],
            ),
          ),
          if (source.authenticityGrade != null) ...[
            const SizedBox(height: 8),
            Row(
              children: [
                const Icon(Icons.verified, size: 16, color: Colors.blue),
                const SizedBox(width: 4),
                Text(
                  'درجة الصحة: ${source.authenticityGrade}',
                  style: const TextStyle(
                    fontSize: 12,
                    color: Colors.blue,
                    fontFamily: 'Tajawal',
                  ),
                ),
              ],
            ),
          ],
          const SizedBox(height: 8),
          Row(
            children: [
              _buildScoreIndicator(source.credibilityScore),
              const Spacer(),
              _buildVerificationBadge(source.verificationStatus),
            ],
          ),
          if (source.notes != null) ...[
            const SizedBox(height: 8),
            Text(
              source.notes!,
              style: const TextStyle(
                fontSize: 12,
                color: Colors.grey,
                fontStyle: FontStyle.italic,
                fontFamily: 'Tajawal',
              ),
            ),
          ],
        ],
      ),
    );
  }

  IconData _getSourceIcon() {
    switch (source.sourceType) {
      case SourceType.quran:
        return Icons.menu_book;
      case SourceType.hadith:
        return Icons.format_quote;
      case SourceType.historicalBook:
        return Icons.history_edu;
      case SourceType.biography:
        return Icons.person;
      case SourceType.tafsir:
        return Icons.description;
      case SourceType.scholarlyWork:
        return Icons.school;
    }
  }

  Color _getSourceColor() {
    switch (source.sourceType) {
      case SourceType.quran:
        return Colors.green;
      case SourceType.hadith:
        return Colors.blue;
      case SourceType.historicalBook:
        return Colors.brown;
      case SourceType.biography:
        return Colors.purple;
      case SourceType.tafsir:
        return Colors.teal;
      case SourceType.scholarlyWork:
        return Colors.indigo;
    }
  }

  Widget _buildScoreIndicator(double score) {
    final percentage = (score / 10.0 * 100).toInt();
    final color = score >= 8.0
        ? Colors.green
        : score >= 6.0
            ? Colors.orange
            : Colors.red;

    return Row(
      children: [
        const Text(
          'المصداقية: ',
          style: TextStyle(
            fontSize: 12,
            fontFamily: 'Tajawal',
          ),
        ),
        Container(
          width: 60,
          height: 6,
          decoration: BoxDecoration(
            color: Colors.grey.withOpacity(0.2),
            borderRadius: BorderRadius.circular(3),
          ),
          child: FractionallySizedBox(
            alignment: Alignment.centerRight,
            widthFactor: score / 10.0,
            child: Container(
              decoration: BoxDecoration(
                color: color,
                borderRadius: BorderRadius.circular(3),
              ),
            ),
          ),
        ),
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
    );
  }

  Widget _buildVerificationBadge(VerificationStatus status) {
    final color = status == VerificationStatus.verified
        ? Colors.green
        : status == VerificationStatus.unverified
            ? Colors.orange
            : Colors.red;

    final icon = status == VerificationStatus.verified
        ? Icons.check_circle
        : status == VerificationStatus.unverified
            ? Icons.help
            : Icons.warning;

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: 14, color: color),
          const SizedBox(width: 4),
          Text(
            status.arabicName,
            style: TextStyle(
              fontSize: 10,
              fontWeight: FontWeight.bold,
              color: color,
              fontFamily: 'Tajawal',
            ),
          ),
        ],
      ),
    );
  }
}

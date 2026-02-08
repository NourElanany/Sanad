import 'package:flutter/material.dart';
import '../../../../core/theme/app_theme.dart';
import '../../../ai_assistant/data/models/ai_message_model.dart';

/// Source citation card widget
class SourceCard extends StatelessWidget {
  final SourceModel source;
  final VoidCallback? onTap;
  final bool isExpanded;

  const SourceCard({
    Key? key,
    required this.source,
    this.onTap,
    this.isExpanded = false,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    if (isExpanded) {
      return _buildExpandedCard(context);
    }

    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(12),
      child: Container(
        padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(
          color: _getSourceColor().withOpacity(0.1),
          borderRadius: BorderRadius.circular(12),
          border: Border.all(
            color: _getSourceColor().withOpacity(0.3),
            width: 1,
          ),
        ),
        child: Row(
          children: [
            // Source icon
            Container(
              padding: const EdgeInsets.all(8),
              decoration: BoxDecoration(
                color: _getSourceColor(),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Icon(
                _getSourceIcon(),
                color: Colors.white,
                size: 16,
              ),
            ),
            const SizedBox(width: 12),

            // Source info
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    source.title,
                    style: const TextStyle(
                      fontSize: 13,
                      fontWeight: FontWeight.bold,
                    ),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                  const SizedBox(height: 2),
                  Text(
                    source.reference,
                    style: TextStyle(
                      fontSize: 11,
                      color: Colors.grey.shade600,
                    ),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                ],
              ),
            ),

            // Confidence indicator
            if (source.confidence != null)
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                decoration: BoxDecoration(
                  color: _getConfidenceColor(),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Text(
                  '${(source.confidence! * 100).toInt()}%',
                  style: const TextStyle(
                    fontSize: 10,
                    fontWeight: FontWeight.bold,
                    color: Colors.white,
                  ),
                ),
              ),

            // Arrow icon
            if (onTap != null) ...[
              const SizedBox(width: 8),
              Icon(
                Icons.arrow_forward_ios,
                size: 12,
                color: Colors.grey.shade400,
              ),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildExpandedCard(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(top: 8),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: const BorderRadius.vertical(top: Radius.circular(24)),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          // Handle bar
          Container(
            margin: const EdgeInsets.only(top: 12),
            width: 40,
            height: 4,
            decoration: BoxDecoration(
              color: Colors.grey.shade300,
              borderRadius: BorderRadius.circular(2),
            ),
          ),

          // Content
          Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                // Header
                Row(
                  children: [
                    Container(
                      padding: const EdgeInsets.all(12),
                      decoration: BoxDecoration(
                        color: _getSourceColor(),
                        borderRadius: BorderRadius.circular(12),
                      ),
                      child: Icon(
                        _getSourceIcon(),
                        color: Colors.white,
                        size: 24,
                      ),
                    ),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            _getSourceTypeLabel(),
                            style: TextStyle(
                              fontSize: 12,
                              color: _getSourceColor(),
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                          const SizedBox(height: 4),
                          Text(
                            source.title,
                            style: const TextStyle(
                              fontSize: 18,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                        ],
                      ),
                    ),
                  ],
                ),

                const SizedBox(height: 16),

                // Reference
                Container(
                  padding: const EdgeInsets.all(12),
                  decoration: BoxDecoration(
                    color: AppColors.background,
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Row(
                    children: [
                      Icon(
                        Icons.bookmark_outline,
                        size: 16,
                        color: AppColors.accent,
                      ),
                      const SizedBox(width: 8),
                      Expanded(
                        child: Text(
                          source.reference,
                          style: TextStyle(
                            fontSize: 13,
                            color: AppColors.textSecondary,
                          ),
                        ),
                      ),
                    ],
                  ),
                ),

                // Excerpt
                if (source.excerpt != null) ...[
                  const SizedBox(height: 16),
                  Container(
                    padding: const EdgeInsets.all(16),
                    decoration: BoxDecoration(
                      color: _getSourceColor().withOpacity(0.05),
                      borderRadius: BorderRadius.circular(12),
                      border: Border.all(
                        color: _getSourceColor().withOpacity(0.2),
                      ),
                    ),
                    child: Text(
                      source.excerpt!,
                      style: const TextStyle(
                        fontSize: 14,
                        height: 1.8,
                      ),
                      textDirection: TextDirection.rtl,
                    ),
                  ),
                ],

                // Confidence
                if (source.confidence != null) ...[
                  const SizedBox(height: 16),
                  Row(
                    children: [
                      const Text(
                        'مستوى الثقة:',
                        style: TextStyle(
                          fontSize: 13,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(width: 8),
                      Expanded(
                        child: ClipRRect(
                          borderRadius: BorderRadius.circular(4),
                          child: LinearProgressIndicator(
                            value: source.confidence,
                            backgroundColor: Colors.grey.shade200,
                            valueColor: AlwaysStoppedAnimation<Color>(
                              _getConfidenceColor(),
                            ),
                            minHeight: 8,
                          ),
                        ),
                      ),
                      const SizedBox(width: 8),
                      Text(
                        '${(source.confidence! * 100).toInt()}%',
                        style: TextStyle(
                          fontSize: 13,
                          fontWeight: FontWeight.bold,
                          color: _getConfidenceColor(),
                        ),
                      ),
                    ],
                  ),
                ],

                // Action buttons
                const SizedBox(height: 24),
                Row(
                  children: [
                    Expanded(
                      child: OutlinedButton.icon(
                        onPressed: () {
                          // TODO: Navigate to source
                          Navigator.pop(context);
                        },
                        icon: const Icon(Icons.open_in_new),
                        label: const Text('عرض المصدر الكامل'),
                        style: OutlinedButton.styleFrom(
                          foregroundColor: _getSourceColor(),
                          side: BorderSide(color: _getSourceColor()),
                          padding: const EdgeInsets.symmetric(vertical: 12),
                        ),
                      ),
                    ),
                    const SizedBox(width: 12),
                    OutlinedButton(
                      onPressed: () => Navigator.pop(context),
                      style: OutlinedButton.styleFrom(
                        padding: const EdgeInsets.symmetric(
                          horizontal: 16,
                          vertical: 12,
                        ),
                      ),
                      child: const Text('إغلاق'),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Color _getSourceColor() {
    switch (source.type.toLowerCase()) {
      case 'quran':
        return const Color(0xFF2D5A27); // Green
      case 'hadith':
        return const Color(0xFF1B365D); // Navy
      case 'fatwa':
        return const Color(0xFFB8860B); // Gold
      case 'tafsir':
        return const Color(0xFF6B4423); // Brown
      default:
        return Colors.grey;
    }
  }

  IconData _getSourceIcon() {
    switch (source.type.toLowerCase()) {
      case 'quran':
        return Icons.menu_book;
      case 'hadith':
        return Icons.format_quote;
      case 'fatwa':
        return Icons.gavel;
      case 'tafsir':
        return Icons.library_books;
      default:
        return Icons.article;
    }
  }

  String _getSourceTypeLabel() {
    switch (source.type.toLowerCase()) {
      case 'quran':
        return 'القرآن الكريم';
      case 'hadith':
        return 'الحديث النبوي';
      case 'fatwa':
        return 'فتوى';
      case 'tafsir':
        return 'التفسير';
      default:
        return 'مصدر';
    }
  }

  Color _getConfidenceColor() {
    if (source.confidence == null) return Colors.grey;
    
    if (source.confidence! >= 0.8) {
      return Colors.green;
    } else if (source.confidence! >= 0.6) {
      return Colors.orange;
    } else {
      return Colors.red;
    }
  }
}

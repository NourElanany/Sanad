/// Search suggestions list widget
/// Requirements: 8.1

import 'package:flutter/material.dart';
import '../../../../core/theme/app_theme.dart';
import '../../data/models/search_models.dart';

class SearchSuggestionsList extends StatelessWidget {
  final List<QuerySuggestion> suggestions;
  final Function(QuerySuggestion) onSuggestionTap;

  const SearchSuggestionsList({
    Key? key,
    required this.suggestions,
    required this.onSuggestionTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: suggestions.length,
      itemBuilder: (context, index) {
        final suggestion = suggestions[index];
        return _SuggestionTile(
          suggestion: suggestion,
          onTap: () => onSuggestionTap(suggestion),
        );
      },
    );
  }
}

class _SuggestionTile extends StatelessWidget {
  final QuerySuggestion suggestion;
  final VoidCallback onTap;

  const _SuggestionTile({
    required this.suggestion,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(8),
      child: Container(
        padding: const EdgeInsets.all(12),
        margin: const EdgeInsets.only(bottom: 8),
        decoration: BoxDecoration(
          color: AppTheme.background.secondary,
          borderRadius: BorderRadius.circular(8),
          border: Border.all(
            color: AppTheme.primary.main.withOpacity(0.1),
          ),
        ),
        child: Row(
          children: [
            // Suggestion type icon
            Container(
              padding: const EdgeInsets.all(8),
              decoration: BoxDecoration(
                color: _getSuggestionTypeColor().withOpacity(0.1),
                borderRadius: BorderRadius.circular(6),
              ),
              child: Icon(
                _getSuggestionTypeIcon(),
                size: 20,
                color: _getSuggestionTypeColor(),
              ),
            ),
            const SizedBox(width: 12),

            // Suggestion text and metadata
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    suggestion.suggestedQuery,
                    style: TextStyle(
                      fontSize: 15,
                      fontWeight: FontWeight.w600,
                      color: AppTheme.text.primary,
                    ),
                    textDirection: TextDirection.rtl,
                  ),
                  if (suggestion.explanation != null) ...[
                    const SizedBox(height: 4),
                    Text(
                      suggestion.explanation!,
                      style: TextStyle(
                        fontSize: 12,
                        color: AppTheme.text.secondary,
                      ),
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                    ),
                  ],
                  const SizedBox(height: 4),
                  Row(
                    children: [
                      Icon(
                        Icons.article,
                        size: 12,
                        color: AppTheme.text.secondary,
                      ),
                      const SizedBox(width: 4),
                      Text(
                        '${suggestion.expectedResultsCount} نتيجة متوقعة',
                        style: TextStyle(
                          fontSize: 11,
                          color: AppTheme.text.secondary,
                        ),
                      ),
                      const SizedBox(width: 12),
                      Icon(
                        Icons.star,
                        size: 12,
                        color: AppTheme.accent.gold,
                      ),
                      const SizedBox(width: 4),
                      Text(
                        '${(suggestion.similarityScore * 100).toInt()}%',
                        style: TextStyle(
                          fontSize: 11,
                          color: AppTheme.text.secondary,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),

            // Arrow icon
            Icon(
              Icons.arrow_forward_ios,
              size: 16,
              color: AppTheme.text.secondary,
            ),
          ],
        ),
      ),
    );
  }

  IconData _getSuggestionTypeIcon() {
    switch (suggestion.suggestionType.toLowerCase()) {
      case 'synonym':
        return Icons.swap_horiz;
      case 'conceptual':
        return Icons.lightbulb_outline;
      case 'morphological':
        return Icons.text_fields;
      case 'popular':
        return Icons.trending_up;
      case 'correction':
        return Icons.spellcheck;
      default:
        return Icons.search;
    }
  }

  Color _getSuggestionTypeColor() {
    switch (suggestion.suggestionType.toLowerCase()) {
      case 'synonym':
        return AppTheme.primary.main;
      case 'conceptual':
        return AppTheme.accent.gold;
      case 'morphological':
        return AppTheme.secondary.main;
      case 'popular':
        return AppTheme.status.success;
      case 'correction':
        return AppTheme.status.warning;
      default:
        return AppTheme.text.secondary;
    }
  }
}

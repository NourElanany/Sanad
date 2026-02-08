/// Search sorting bottom sheet
/// Requirements: 8.3

import 'package:flutter/material.dart';
import '../../../../core/theme/app_theme.dart';
import '../../data/models/search_models.dart';

class SearchSortSheet extends StatelessWidget {
  final SortBy currentSort;
  final SortDirection currentDirection;
  final Function(SortBy, SortDirection) onSortChanged;

  const SearchSortSheet({
    Key? key,
    required this.currentSort,
    required this.currentDirection,
    required this.onSortChanged,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: const BorderRadius.vertical(top: Radius.circular(20)),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          // Handle
          Container(
            width: 40,
            height: 4,
            decoration: BoxDecoration(
              color: Colors.grey.shade300,
              borderRadius: BorderRadius.circular(2),
            ),
          ),
          const SizedBox(height: 20),
          
          // Title
          Text(
            'ترتيب النتائج',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              color: AppTheme.text.primary,
            ),
          ),
          const SizedBox(height: 20),

          // Sort options
          _buildSortOption(
            context,
            SortBy.similarity,
            'الأكثر صلة',
            Icons.star,
            'ترتيب حسب درجة التطابق',
          ),
          _buildSortOption(
            context,
            SortBy.relevance,
            'الصلة',
            Icons.trending_up,
            'ترتيب حسب الصلة بالموضوع',
          ),
          _buildSortOption(
            context,
            SortBy.createdAt,
            'الأحدث',
            Icons.access_time,
            'ترتيب حسب تاريخ الإضافة',
          ),
          _buildSortOption(
            context,
            SortBy.priority,
            'الأولوية',
            Icons.priority_high,
            'ترتيب حسب الأولوية',
          ),
          
          const SizedBox(height: 10),
        ],
      ),
    );
  }

  Widget _buildSortOption(
    BuildContext context,
    SortBy sortBy,
    String label,
    IconData icon,
    String description,
  ) {
    final isSelected = currentSort == sortBy;
    
    return InkWell(
      onTap: () {
        final newDirection = isSelected && currentDirection == SortDirection.desc
            ? SortDirection.asc
            : SortDirection.desc;
        onSortChanged(sortBy, newDirection);
        Navigator.pop(context);
      },
      child: Container(
        margin: const EdgeInsets.only(bottom: 12),
        padding: const EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: isSelected ? AppTheme.primary.main.withOpacity(0.05) : Colors.transparent,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(
            color: isSelected ? AppTheme.primary.main : Colors.grey.shade300,
            width: isSelected ? 2 : 1,
          ),
        ),
        child: Row(
          children: [
            Container(
              padding: const EdgeInsets.all(8),
              decoration: BoxDecoration(
                color: isSelected
                    ? AppTheme.primary.main
                    : Colors.grey.shade200,
                borderRadius: BorderRadius.circular(8),
              ),
              child: Icon(
                icon,
                size: 20,
                color: isSelected ? Colors.white : Colors.grey.shade600,
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    label,
                    style: TextStyle(
                      fontSize: 16,
                      fontWeight: isSelected ? FontWeight.bold : FontWeight.w600,
                      color: isSelected ? AppTheme.primary.main : AppTheme.text.primary,
                    ),
                  ),
                  const SizedBox(height: 2),
                  Text(
                    description,
                    style: TextStyle(
                      fontSize: 12,
                      color: AppTheme.text.secondary,
                    ),
                  ),
                ],
              ),
            ),
            if (isSelected) ...[
              const SizedBox(width: 8),
              Icon(
                currentDirection == SortDirection.desc
                    ? Icons.arrow_downward
                    : Icons.arrow_upward,
                size: 20,
                color: AppTheme.primary.main,
              ),
            ],
          ],
        ),
      ),
    );
  }
}

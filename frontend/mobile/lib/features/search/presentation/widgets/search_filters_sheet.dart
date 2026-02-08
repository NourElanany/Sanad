/// Advanced search filters bottom sheet
/// Requirements: 8.2

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/theme/app_theme.dart';
import '../../../../core/providers/search_provider.dart';
import '../../data/models/search_models.dart';

class SearchFiltersSheet extends ConsumerStatefulWidget {
  const SearchFiltersSheet({Key? key}) : super(key: key);

  @override
  ConsumerState<SearchFiltersSheet> createState() => _SearchFiltersSheetState();
}

class _SearchFiltersSheetState extends ConsumerState<SearchFiltersSheet> {
  Set<ContentType> _selectedContentTypes = {};
  Set<AuthenticityGrade> _selectedAuthenticityGrades = {};
  double _minSimilarity = 0.5;
  SortBy _sortBy = SortBy.similarity;

  @override
  void initState() {
    super.initState();
    // Load current filters
    final currentFilters = ref.read(searchProvider).currentFilters;
    if (currentFilters != null) {
      _selectedContentTypes = currentFilters.contentTypes?.toSet() ?? {};
      _selectedAuthenticityGrades = currentFilters.authenticityGrades?.toSet() ?? {};
      _minSimilarity = currentFilters.minSimilarity ?? 0.5;
    }
  }

  void _applyFilters() {
    final filters = SearchFilters(
      contentTypes: _selectedContentTypes.isEmpty ? null : _selectedContentTypes.toList(),
      authenticityGrades: _selectedAuthenticityGrades.isEmpty
          ? null
          : _selectedAuthenticityGrades.toList(),
      minSimilarity: _minSimilarity,
    );

    ref.read(searchProvider.notifier).updateFilters(filters);

    // Re-run search with new filters
    final currentQuery = ref.read(searchProvider).currentQuery;
    if (currentQuery.isNotEmpty) {
      ref.read(searchProvider.notifier).search(
        currentQuery,
        filters: filters,
        sortBy: _sortBy,
      );
    }

    Navigator.pop(context);
  }

  void _clearFilters() {
    setState(() {
      _selectedContentTypes.clear();
      _selectedAuthenticityGrades.clear();
      _minSimilarity = 0.5;
      _sortBy = SortBy.similarity;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: const BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
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
              color: AppTheme.text.secondary.withOpacity(0.3),
              borderRadius: BorderRadius.circular(2),
            ),
          ),

          // Header
          Padding(
            padding: const EdgeInsets.all(20),
            child: Row(
              children: [
                Text(
                  'فلاتر البحث',
                  style: TextStyle(
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                    color: AppTheme.primary.main,
                  ),
                ),
                const Spacer(),
                TextButton(
                  onPressed: _clearFilters,
                  child: const Text('مسح الكل'),
                ),
              ],
            ),
          ),

          // Filters content
          Expanded(
            child: SingleChildScrollView(
              padding: const EdgeInsets.symmetric(horizontal: 20),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // Content types
                  _buildSectionTitle('نوع المحتوى'),
                  const SizedBox(height: 12),
                  _buildContentTypeFilters(),

                  const SizedBox(height: 24),

                  // Authenticity grades (for hadith)
                  _buildSectionTitle('درجة الصحة (للأحاديث)'),
                  const SizedBox(height: 12),
                  _buildAuthenticityFilters(),

                  const SizedBox(height: 24),

                  // Minimum similarity
                  _buildSectionTitle('الحد الأدنى للتطابق'),
                  const SizedBox(height: 12),
                  _buildSimilaritySlider(),

                  const SizedBox(height: 24),

                  // Sort by
                  _buildSectionTitle('ترتيب النتائج'),
                  const SizedBox(height: 12),
                  _buildSortOptions(),

                  const SizedBox(height: 32),
                ],
              ),
            ),
          ),

          // Apply button
          Container(
            padding: const EdgeInsets.all(20),
            decoration: BoxDecoration(
              color: Colors.white,
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withOpacity(0.05),
                  blurRadius: 10,
                  offset: const Offset(0, -2),
                ),
              ],
            ),
            child: SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: _applyFilters,
                style: ElevatedButton.styleFrom(
                  padding: const EdgeInsets.symmetric(vertical: 16),
                  backgroundColor: AppTheme.primary.main,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                ),
                child: const Text(
                  'تطبيق الفلاتر',
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSectionTitle(String title) {
    return Text(
      title,
      style: TextStyle(
        fontSize: 16,
        fontWeight: FontWeight.w600,
        color: AppTheme.text.primary,
      ),
    );
  }

  Widget _buildContentTypeFilters() {
    return Wrap(
      spacing: 8,
      runSpacing: 8,
      children: ContentType.values.map((type) {
        final isSelected = _selectedContentTypes.contains(type);
        return FilterChip(
          label: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(type.iconEmoji),
              const SizedBox(width: 6),
              Text(type.displayName),
            ],
          ),
          selected: isSelected,
          onSelected: (selected) {
            setState(() {
              if (selected) {
                _selectedContentTypes.add(type);
              } else {
                _selectedContentTypes.remove(type);
              }
            });
          },
          selectedColor: AppTheme.primary.main.withOpacity(0.2),
          checkmarkColor: AppTheme.primary.main,
          labelStyle: TextStyle(
            color: isSelected ? AppTheme.primary.main : AppTheme.text.secondary,
            fontWeight: isSelected ? FontWeight.w600 : FontWeight.normal,
          ),
        );
      }).toList(),
    );
  }

  Widget _buildAuthenticityFilters() {
    return Wrap(
      spacing: 8,
      runSpacing: 8,
      children: AuthenticityGrade.values.map((grade) {
        final isSelected = _selectedAuthenticityGrades.contains(grade);
        final color = _parseColor(grade.colorHex);
        return FilterChip(
          label: Text(grade.displayName),
          selected: isSelected,
          onSelected: (selected) {
            setState(() {
              if (selected) {
                _selectedAuthenticityGrades.add(grade);
              } else {
                _selectedAuthenticityGrades.remove(grade);
              }
            });
          },
          selectedColor: color.withOpacity(0.2),
          checkmarkColor: color,
          labelStyle: TextStyle(
            color: isSelected ? color : AppTheme.text.secondary,
            fontWeight: isSelected ? FontWeight.w600 : FontWeight.normal,
          ),
        );
      }).toList(),
    );
  }

  Widget _buildSimilaritySlider() {
    return Column(
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(
              'منخفض',
              style: TextStyle(
                fontSize: 12,
                color: AppTheme.text.secondary,
              ),
            ),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
              decoration: BoxDecoration(
                color: AppTheme.primary.main.withOpacity(0.1),
                borderRadius: BorderRadius.circular(6),
              ),
              child: Text(
                '${(_minSimilarity * 100).toInt()}%',
                style: TextStyle(
                  fontSize: 14,
                  fontWeight: FontWeight.w600,
                  color: AppTheme.primary.main,
                ),
              ),
            ),
            Text(
              'عالي',
              style: TextStyle(
                fontSize: 12,
                color: AppTheme.text.secondary,
              ),
            ),
          ],
        ),
        Slider(
          value: _minSimilarity,
          min: 0.3,
          max: 0.9,
          divisions: 12,
          activeColor: AppTheme.primary.main,
          onChanged: (value) {
            setState(() => _minSimilarity = value);
          },
        ),
      ],
    );
  }

  Widget _buildSortOptions() {
    return Column(
      children: [
        _buildSortOption(SortBy.similarity, 'التطابق', Icons.star),
        _buildSortOption(SortBy.relevance, 'الصلة', Icons.trending_up),
        _buildSortOption(SortBy.priority, 'الأولوية', Icons.priority_high),
        _buildSortOption(SortBy.createdAt, 'التاريخ', Icons.calendar_today),
      ],
    );
  }

  Widget _buildSortOption(SortBy sortBy, String label, IconData icon) {
    final isSelected = _sortBy == sortBy;
    return InkWell(
      onTap: () => setState(() => _sortBy = sortBy),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        margin: const EdgeInsets.only(bottom: 8),
        decoration: BoxDecoration(
          color: isSelected
              ? AppTheme.primary.main.withOpacity(0.1)
              : AppTheme.background.secondary,
          borderRadius: BorderRadius.circular(8),
          border: Border.all(
            color: isSelected
                ? AppTheme.primary.main
                : AppTheme.primary.main.withOpacity(0.1),
          ),
        ),
        child: Row(
          children: [
            Icon(
              icon,
              size: 20,
              color: isSelected ? AppTheme.primary.main : AppTheme.text.secondary,
            ),
            const SizedBox(width: 12),
            Text(
              label,
              style: TextStyle(
                fontSize: 14,
                fontWeight: isSelected ? FontWeight.w600 : FontWeight.normal,
                color: isSelected ? AppTheme.primary.main : AppTheme.text.primary,
              ),
            ),
            const Spacer(),
            if (isSelected)
              Icon(
                Icons.check_circle,
                size: 20,
                color: AppTheme.primary.main,
              ),
          ],
        ),
      ),
    );
  }

  Color _parseColor(String hex) {
    return Color(int.parse(hex.substring(1), radix: 16) + 0xFF000000);
  }
}

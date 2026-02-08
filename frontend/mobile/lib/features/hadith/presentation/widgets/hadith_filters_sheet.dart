import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/providers/hadith_provider.dart';
import '../../data/models/hadith_model.dart';
import '../../../../core/theme/app_theme.dart';

class HadithFiltersSheet extends ConsumerWidget {
  const HadithFiltersSheet({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final searchState = ref.watch(hadithSearchProvider);
    final hadithBooksAsync = ref.watch(hadithBooksProvider);

    return Container(
      padding: const EdgeInsets.all(24),
      decoration: const BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.vertical(top: Radius.circular(24)),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Header
          Row(
            children: [
              const Text(
                'فلاتر البحث',
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Tajawal',
                ),
              ),
              const Spacer(),
              IconButton(
                icon: const Icon(Icons.close),
                onPressed: () => Navigator.pop(context),
              ),
            ],
          ),
          const SizedBox(height: 24),

          // Search Type
          const Text(
            'نوع البحث',
            style: TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
            ),
          ),
          const SizedBox(height: 12),
          Wrap(
            spacing: 8,
            runSpacing: 8,
            children: [
              _buildSearchTypeChip(
                context,
                ref,
                'text',
                'نصي',
                searchState.filters.searchType == 'text',
              ),
              _buildSearchTypeChip(
                context,
                ref,
                'semantic',
                'دلالي',
                searchState.filters.searchType == 'semantic',
              ),
              _buildSearchTypeChip(
                context,
                ref,
                'narrator',
                'راوي',
                searchState.filters.searchType == 'narrator',
              ),
              _buildSearchTypeChip(
                context,
                ref,
                'theme',
                'موضوع',
                searchState.filters.searchType == 'theme',
              ),
            ],
          ),
          const SizedBox(height: 24),

          // Authenticity Grades
          const Text(
            'درجة الصحة',
            style: TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
            ),
          ),
          const SizedBox(height: 12),
          Wrap(
            spacing: 8,
            runSpacing: 8,
            children: HadithGrade.values.map((grade) {
              final isSelected = searchState.filters.grades.contains(grade);
              return _buildGradeChip(context, ref, grade, isSelected);
            }).toList(),
          ),
          const SizedBox(height: 24),

          // Books
          const Text(
            'المجموعات',
            style: TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
            ),
          ),
          const SizedBox(height: 12),
          hadithBooksAsync.when(
            data: (books) => Wrap(
              spacing: 8,
              runSpacing: 8,
              children: books.map((book) {
                final isSelected = searchState.filters.books.contains(book.name);
                return _buildBookChip(context, ref, book.name, book.arabicName, isSelected);
              }).toList(),
            ),
            loading: () => const Center(child: CircularProgressIndicator()),
            error: (_, __) => const Text('خطأ في تحميل المجموعات'),
          ),
          const SizedBox(height: 24),

          // Themes
          const Text(
            'المواضيع',
            style: TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
            ),
          ),
          const SizedBox(height: 12),
          Wrap(
            spacing: 8,
            runSpacing: 8,
            children: [
              'عقيدة',
              'عبادة',
              'معاملات',
              'أسرة',
              'أخلاق',
              'تاريخ',
              'نبوءات',
              'فقه',
            ].map((theme) {
              final isSelected = searchState.filters.themes.contains(theme);
              return _buildThemeChip(context, ref, theme, isSelected);
            }).toList(),
          ),
          const SizedBox(height: 24),

          // Apply Button
          ElevatedButton(
            onPressed: () => Navigator.pop(context),
            style: ElevatedButton.styleFrom(
              backgroundColor: AppTheme.primaryColor,
              padding: const EdgeInsets.symmetric(vertical: 16),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(12),
              ),
            ),
            child: const Text(
              'تطبيق الفلاتر',
              style: TextStyle(
                fontSize: 16,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSearchTypeChip(
    BuildContext context,
    WidgetRef ref,
    String type,
    String label,
    bool isSelected,
  ) {
    return FilterChip(
      label: Text(label),
      selected: isSelected,
      onSelected: (_) {
        ref.read(hadithSearchProvider.notifier).setSearchType(type);
      },
      selectedColor: AppTheme.primaryColor.withOpacity(0.2),
      checkmarkColor: AppTheme.primaryColor,
      labelStyle: TextStyle(
        fontFamily: 'Tajawal',
        color: isSelected ? AppTheme.primaryColor : Colors.grey[700],
        fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
      ),
    );
  }

  Widget _buildGradeChip(
    BuildContext context,
    WidgetRef ref,
    HadithGrade grade,
    bool isSelected,
  ) {
    Color color;
    switch (grade) {
      case HadithGrade.sahih:
        color = Colors.green;
        break;
      case HadithGrade.hasan:
        color = Colors.amber;
        break;
      case HadithGrade.daif:
        color = Colors.orange;
        break;
      case HadithGrade.mawdu:
        color = Colors.red;
        break;
    }

    return FilterChip(
      label: Text(grade.arabicName),
      selected: isSelected,
      onSelected: (_) {
        ref.read(hadithSearchProvider.notifier).toggleGrade(grade);
      },
      selectedColor: color.withOpacity(0.2),
      checkmarkColor: color,
      labelStyle: TextStyle(
        fontFamily: 'Tajawal',
        color: isSelected ? color : Colors.grey[700],
        fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
      ),
    );
  }

  Widget _buildBookChip(
    BuildContext context,
    WidgetRef ref,
    String bookName,
    String arabicName,
    bool isSelected,
  ) {
    return FilterChip(
      label: Text(arabicName),
      selected: isSelected,
      onSelected: (_) {
        ref.read(hadithSearchProvider.notifier).toggleBook(bookName);
      },
      selectedColor: AppTheme.secondaryColor.withOpacity(0.2),
      checkmarkColor: AppTheme.secondaryColor,
      labelStyle: TextStyle(
        fontFamily: 'Tajawal',
        color: isSelected ? AppTheme.secondaryColor : Colors.grey[700],
        fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
      ),
    );
  }

  Widget _buildThemeChip(
    BuildContext context,
    WidgetRef ref,
    String theme,
    bool isSelected,
  ) {
    return FilterChip(
      label: Text(theme),
      selected: isSelected,
      onSelected: (_) {
        ref.read(hadithSearchProvider.notifier).toggleTheme(theme);
      },
      selectedColor: AppTheme.accentGold.withOpacity(0.2),
      checkmarkColor: AppTheme.accentGold,
      labelStyle: TextStyle(
        fontFamily: 'Tajawal',
        color: isSelected ? AppTheme.accentGold : Colors.grey[700],
        fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
      ),
    );
  }
}

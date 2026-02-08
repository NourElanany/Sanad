import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/providers/quran_provider.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_button.dart';

/// Bottom sheet for Quran filtering options
class QuranFilterSheet extends ConsumerStatefulWidget {
  const QuranFilterSheet({Key? key}) : super(key: key);

  @override
  ConsumerState<QuranFilterSheet> createState() => _QuranFilterSheetState();
}

class _QuranFilterSheetState extends ConsumerState<QuranFilterSheet> {
  QuranFilterType _selectedFilterType = QuranFilterType.none;
  String? _selectedFilterValue;

  @override
  void initState() {
    super.initState();
    final state = ref.read(quranIndexProvider);
    _selectedFilterType = state.filterType;
    _selectedFilterValue = state.filterValue;
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: AppColors.backgroundPaper,
        borderRadius: const BorderRadius.vertical(
          top: Radius.circular(24),
        ),
      ),
      child: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Header
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    'فلاتر البحث',
                    style: AppTextStyles.h5.copyWith(
                      color: AppColors.textPrimary,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  IconButton(
                    icon: Icon(
                      Icons.close,
                      color: AppColors.textSecondary,
                    ),
                    onPressed: () => Navigator.of(context).pop(),
                  ),
                ],
              ),
              const SizedBox(height: 24),
              
              // Filter options
              _buildFilterOption(
                title: 'نوع السورة',
                filterType: QuranFilterType.revelationType,
                children: [
                  _buildFilterChip('مكية', 'meccan'),
                  _buildFilterChip('مدنية', 'medinan'),
                ],
              ),
              const SizedBox(height: 16),
              
              _buildFilterOption(
                title: 'عدد الآيات',
                filterType: QuranFilterType.ayahCount,
                children: [
                  _buildFilterChip('قصيرة (1-20)', '1-20'),
                  _buildFilterChip('متوسطة (21-100)', '21-100'),
                  _buildFilterChip('طويلة (100+)', '100-999'),
                ],
              ),
              const SizedBox(height: 32),
              
              // Action buttons
              Row(
                children: [
                  Expanded(
                    child: OutlinedButton(
                      onPressed: () {
                        setState(() {
                          _selectedFilterType = QuranFilterType.none;
                          _selectedFilterValue = null;
                        });
                        ref.read(quranIndexProvider.notifier).clearFilter();
                        Navigator.of(context).pop();
                      },
                      style: OutlinedButton.styleFrom(
                        side: BorderSide(color: AppColors.primaryMain),
                        padding: const EdgeInsets.symmetric(vertical: 16),
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(12),
                        ),
                      ),
                      child: Text(
                        'مسح الفلاتر',
                        style: AppTextStyles.button.copyWith(
                          color: AppColors.primaryMain,
                        ),
                      ),
                    ),
                  ),
                  const SizedBox(width: 16),
                  Expanded(
                    child: IslamicButton(
                      text: 'تطبيق',
                      onPressed: () {
                        ref.read(quranIndexProvider.notifier).setFilter(
                          _selectedFilterType,
                          _selectedFilterValue,
                        );
                        Navigator.of(context).pop();
                      },
                      type: IslamicButtonType.primary,
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildFilterOption({
    required String title,
    required QuranFilterType filterType,
    required List<Widget> children,
  }) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          title,
          style: AppTextStyles.subtitle1.copyWith(
            color: AppColors.textPrimary,
            fontWeight: FontWeight.w600,
          ),
        ),
        const SizedBox(height: 12),
        Wrap(
          spacing: 8,
          runSpacing: 8,
          children: children,
        ),
      ],
    );
  }

  Widget _buildFilterChip(String label, String value) {
    final isSelected = _selectedFilterValue == value;
    
    return FilterChip(
      label: Text(label),
      selected: isSelected,
      onSelected: (selected) {
        setState(() {
          if (selected) {
            _selectedFilterType = QuranFilterType.revelationType;
            if (value == 'meccan' || value == 'medinan') {
              _selectedFilterType = QuranFilterType.revelationType;
            } else {
              _selectedFilterType = QuranFilterType.ayahCount;
            }
            _selectedFilterValue = value;
          } else {
            _selectedFilterType = QuranFilterType.none;
            _selectedFilterValue = null;
          }
        });
      },
      selectedColor: AppColors.primaryMain.withOpacity(0.2),
      checkmarkColor: AppColors.primaryMain,
      labelStyle: AppTextStyles.body2.copyWith(
        color: isSelected ? AppColors.primaryMain : AppColors.textSecondary,
        fontWeight: isSelected ? FontWeight.w600 : FontWeight.normal,
      ),
      side: BorderSide(
        color: isSelected ? AppColors.primaryMain : AppColors.textDisabled,
      ),
    );
  }
}

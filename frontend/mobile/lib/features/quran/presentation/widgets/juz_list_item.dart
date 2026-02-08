import 'package:flutter/material.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../data/models/surah_model.dart';

/// List item widget for displaying a Juz
class JuzListItem extends StatelessWidget {
  final JuzModel juz;
  final VoidCallback onTap;

  const JuzListItem({
    Key? key,
    required this.juz,
    required this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      onTap: onTap,
      padding: const EdgeInsets.all(16),
      child: Row(
        children: [
          // Juz number badge
          _buildNumberBadge(),
          const SizedBox(width: 16),
          
          // Juz info
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Juz title
                Text(
                  'الجزء ${_arabicNumber(juz.number)}',
                  style: AppTextStyles.h6.copyWith(
                    color: AppColors.textQuranic,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 8),
                
                // Start position
                Row(
                  children: [
                    Icon(
                      Icons.play_arrow,
                      size: 16,
                      color: AppColors.secondaryMain,
                    ),
                    const SizedBox(width: 4),
                    Text(
                      'من سورة ${juz.startSurah} آية ${juz.startAyah}',
                      style: AppTextStyles.body2.copyWith(
                        color: AppColors.textSecondary,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 4),
                
                // End position
                Row(
                  children: [
                    Icon(
                      Icons.stop,
                      size: 16,
                      color: AppColors.statusError,
                    ),
                    const SizedBox(width: 4),
                    Text(
                      'إلى سورة ${juz.endSurah} آية ${juz.endAyah}',
                      style: AppTextStyles.body2.copyWith(
                        color: AppColors.textSecondary,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 4),
                
                // Page range
                Text(
                  'الصفحات ${juz.pageStart} - ${juz.pageEnd}',
                  style: AppTextStyles.caption.copyWith(
                    color: AppColors.textSecondary,
                  ),
                ),
              ],
            ),
          ),
          
          // Arrow icon
          Icon(
            Icons.arrow_forward_ios,
            size: 16,
            color: AppColors.textSecondary,
          ),
        ],
      ),
    );
  }

  Widget _buildNumberBadge() {
    return Container(
      width: 56,
      height: 56,
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: [
            AppColors.secondaryMain,
            AppColors.secondaryLight,
          ],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
        borderRadius: BorderRadius.circular(12),
        boxShadow: [
          BoxShadow(
            color: AppColors.secondaryMain.withOpacity(0.3),
            blurRadius: 8,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(
              'جزء',
              style: AppTextStyles.caption.copyWith(
                color: Colors.white,
                fontSize: 10,
              ),
            ),
            Text(
              juz.number.toString(),
              style: AppTextStyles.h6.copyWith(
                color: Colors.white,
                fontWeight: FontWeight.bold,
              ),
            ),
          ],
        ),
      ),
    );
  }

  String _arabicNumber(int number) {
    const arabicNumbers = [
      'الأول', 'الثاني', 'الثالث', 'الرابع', 'الخامس',
      'السادس', 'السابع', 'الثامن', 'التاسع', 'العاشر',
      'الحادي عشر', 'الثاني عشر', 'الثالث عشر', 'الرابع عشر', 'الخامس عشر',
      'السادس عشر', 'السابع عشر', 'الثامن عشر', 'التاسع عشر', 'العشرون',
      'الحادي والعشرون', 'الثاني والعشرون', 'الثالث والعشرون', 'الرابع والعشرون',
      'الخامس والعشرون', 'السادس والعشرون', 'السابع والعشرون', 'الثامن والعشرون',
      'التاسع والعشرون', 'الثلاثون'
    ];
    
    if (number >= 1 && number <= 30) {
      return arabicNumbers[number - 1];
    }
    return number.toString();
  }
}

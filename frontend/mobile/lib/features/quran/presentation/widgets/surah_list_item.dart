import 'package:flutter/material.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../data/models/surah_model.dart';

/// List item widget for displaying a Surah
class SurahListItem extends StatelessWidget {
  final SurahModel surah;
  final VoidCallback onTap;

  const SurahListItem({
    Key? key,
    required this.surah,
    required this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      onTap: onTap,
      padding: const EdgeInsets.all(16),
      child: Row(
        children: [
          // Surah number badge
          _buildNumberBadge(),
          const SizedBox(width: 16),
          
          // Surah info
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Arabic name
                Text(
                  surah.nameArabic,
                  style: AppTextStyles.h6.copyWith(
                    color: AppColors.textQuranic,
                    fontWeight: FontWeight.bold,
                    fontFamily: 'KFGQPC Uthman Taha Naskh',
                  ),
                ),
                const SizedBox(height: 4),
                
                // English name and transliteration
                Text(
                  surah.nameEnglish,
                  style: AppTextStyles.body2.copyWith(
                    color: AppColors.textSecondary,
                  ),
                ),
                const SizedBox(height: 4),
                
                // Revelation type and ayah count
                Row(
                  children: [
                    _buildRevelationTypeBadge(),
                    const SizedBox(width: 8),
                    Text(
                      '${surah.ayahCount} آية',
                      style: AppTextStyles.caption.copyWith(
                        color: AppColors.textSecondary,
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
            color: AppColors.textSecondary,
          ),
        ],
      ),
    );
  }

  Widget _buildNumberBadge() {
    return Container(
      width: 48,
      height: 48,
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: [
            AppColors.primaryMain,
            AppColors.primaryLight,
          ],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
        borderRadius: BorderRadius.circular(12),
        boxShadow: [
          BoxShadow(
            color: AppColors.primaryMain.withOpacity(0.3),
            blurRadius: 8,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Center(
        child: Text(
          surah.number.toString(),
          style: AppTextStyles.h6.copyWith(
            color: Colors.white,
            fontWeight: FontWeight.bold,
          ),
        ),
      ),
    );
  }

  Widget _buildRevelationTypeBadge() {
    final isMeccan = surah.isMeccan;
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: isMeccan
            ? AppColors.secondaryMain.withOpacity(0.1)
            : AppColors.primaryMain.withOpacity(0.1),
        borderRadius: BorderRadius.circular(4),
        border: Border.all(
          color: isMeccan ? AppColors.secondaryMain : AppColors.primaryMain,
          width: 1,
        ),
      ),
      child: Text(
        isMeccan ? 'مكية' : 'مدنية',
        style: AppTextStyles.caption.copyWith(
          color: isMeccan ? AppColors.secondaryMain : AppColors.primaryMain,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }
}

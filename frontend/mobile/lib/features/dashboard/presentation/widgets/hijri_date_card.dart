import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../../../core/services/prayer_times_service.dart';

class HijriDateCard extends StatelessWidget {
  final HijriDate hijriDate;

  const HijriDateCard({
    Key? key,
    required this.hijriDate,
  }) : super(key: key);

  String _getGregorianDate() {
    final now = DateTime.now();
    final formatter = DateFormat('EEEE، d MMMM yyyy', 'ar');
    return formatter.format(now);
  }

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      child: Row(
        children: [
          // Calendar Icon
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              gradient: LinearGradient(
                colors: [
                  AppColors.accent.withOpacity(0.2),
                  AppColors.accent.withOpacity(0.1),
                ],
                begin: Alignment.topLeft,
                end: Alignment.bottomRight,
              ),
              borderRadius: BorderRadius.circular(12),
            ),
            child: Icon(
              Icons.calendar_today,
              color: AppColors.accent,
              size: 28,
            ),
          ),
          const SizedBox(width: 16),

          // Dates
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Hijri Date
                Text(
                  hijriDate.formatted,
                  style: AppTextStyles.bodyLarge.copyWith(
                    color: AppColors.primary,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 4),

                // Gregorian Date
                Text(
                  _getGregorianDate() + ' م',
                  style: AppTextStyles.bodyMedium.copyWith(
                    color: AppColors.textSecondary,
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

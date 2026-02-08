import 'package:flutter/material.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';

class CalendarLegendWidget extends StatelessWidget {
  const CalendarLegendWidget({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: AppColors.backgroundSecondary,
        borderRadius: BorderRadius.circular(12),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'دليل الألوان:',
            style: AppTextStyles.bodySmall.copyWith(
              color: AppColors.textSecondary,
              fontWeight: FontWeight.w600,
            ),
          ),
          const SizedBox(height: 8),
          Wrap(
            spacing: 16,
            runSpacing: 8,
            children: [
              _buildLegendItem(
                color: AppColors.primary.withOpacity(0.1),
                borderColor: AppColors.accent,
                label: 'اليوم الحالي',
              ),
              _buildLegendItem(
                color: AppColors.secondary.withOpacity(0.1),
                borderColor: AppColors.secondary.withOpacity(0.3),
                label: 'يوم الجمعة',
              ),
              _buildLegendItem(
                color: AppColors.success.withOpacity(0.1),
                borderColor: AppColors.success.withOpacity(0.3),
                label: 'عيد',
              ),
              _buildLegendItem(
                color: AppColors.accent.withOpacity(0.05),
                borderColor: AppColors.accent.withOpacity(0.3),
                label: 'مناسبة إسلامية',
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildLegendItem({
    required Color color,
    required Color borderColor,
    required String label,
  }) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: 16,
          height: 16,
          decoration: BoxDecoration(
            color: color,
            borderRadius: BorderRadius.circular(4),
            border: Border.all(color: borderColor, width: 1),
          ),
        ),
        const SizedBox(width: 6),
        Text(
          label,
          style: AppTextStyles.caption.copyWith(
            color: AppColors.textSecondary,
          ),
        ),
      ],
    );
  }
}

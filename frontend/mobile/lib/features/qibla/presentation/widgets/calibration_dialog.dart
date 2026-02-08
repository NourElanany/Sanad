import 'package:flutter/material.dart';
import 'package:lottie/lottie.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/widgets/islamic_button.dart';

/// Dialog showing compass calibration instructions
class CalibrationDialog extends StatelessWidget {
  const CalibrationDialog({super.key});

  @override
  Widget build(BuildContext context) {
    return Dialog(
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(24),
      ),
      child: Container(
        padding: const EdgeInsets.all(24),
        decoration: BoxDecoration(
          color: AppColors.backgroundPaper,
          borderRadius: BorderRadius.circular(24),
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            // Title
            Row(
              children: [
                Icon(
                  Icons.settings_backup_restore,
                  color: AppColors.primaryMain,
                  size: 28,
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Text(
                    'معايرة البوصلة',
                    style: TextStyle(
                      fontFamily: 'Tajawal',
                      fontSize: 22,
                      fontWeight: FontWeight.bold,
                      color: AppColors.textPrimary,
                    ),
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: () => Navigator.of(context).pop(),
                  color: AppColors.textSecondary,
                ),
              ],
            ),

            const SizedBox(height: 24),

            // Animation placeholder (would use Lottie animation in production)
            Container(
              width: 200,
              height: 200,
              decoration: BoxDecoration(
                color: AppColors.primaryMain.withOpacity(0.1),
                borderRadius: BorderRadius.circular(16),
              ),
              child: Icon(
                Icons.phone_android,
                size: 80,
                color: AppColors.primaryMain.withOpacity(0.5),
              ),
            ),

            const SizedBox(height: 24),

            // Instructions
            _buildInstructionStep(
              number: '1',
              text: 'أمسك الجهاز بشكل مستوٍ أمامك',
            ),
            const SizedBox(height: 12),
            _buildInstructionStep(
              number: '2',
              text: 'حرك الجهاز على شكل رقم 8 في الهواء',
            ),
            const SizedBox(height: 12),
            _buildInstructionStep(
              number: '3',
              text: 'كرر الحركة عدة مرات حتى تتحسن الدقة',
            ),

            const SizedBox(height: 24),

            // Tips
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: AppColors.accentGold.withOpacity(0.1),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(
                  color: AppColors.accentGold.withOpacity(0.3),
                ),
              ),
              child: Row(
                children: [
                  Icon(
                    Icons.lightbulb_outline,
                    color: AppColors.accentGold,
                    size: 24,
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Text(
                      'ابتعد عن الأجهزة الإلكترونية والمعادن للحصول على أفضل دقة',
                      style: TextStyle(
                        fontFamily: 'Tajawal',
                        fontSize: 14,
                        color: AppColors.textSecondary,
                      ),
                    ),
                  ),
                ],
              ),
            ),

            const SizedBox(height: 24),

            // Close button
            IslamicButton(
              text: 'فهمت',
              onPressed: () => Navigator.of(context).pop(),
              type: IslamicButtonType.primary,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildInstructionStep({
    required String number,
    required String text,
  }) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Container(
          width: 32,
          height: 32,
          decoration: BoxDecoration(
            color: AppColors.primaryMain,
            shape: BoxShape.circle,
          ),
          child: Center(
            child: Text(
              number,
              style: const TextStyle(
                fontFamily: 'Tajawal',
                fontSize: 16,
                fontWeight: FontWeight.bold,
                color: Colors.white,
              ),
            ),
          ),
        ),
        const SizedBox(width: 12),
        Expanded(
          child: Padding(
            padding: const EdgeInsets.only(top: 6),
            child: Text(
              text,
              style: TextStyle(
                fontFamily: 'Tajawal',
                fontSize: 16,
                color: AppColors.textPrimary,
              ),
            ),
          ),
        ),
      ],
    );
  }
}

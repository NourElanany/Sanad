import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_button.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Screen for selecting app theme and display preferences
class ThemeSelectionScreen extends StatefulWidget {
  const ThemeSelectionScreen({super.key});

  @override
  State<ThemeSelectionScreen> createState() => _ThemeSelectionScreenState();
}

class _ThemeSelectionScreenState extends State<ThemeSelectionScreen> {
  String _selectedTheme = 'light';
  String _selectedFontSize = 'medium';
  bool _enableAnimations = true;

  final List<Map<String, dynamic>> _themes = [
    {
      'id': 'light',
      'name': 'الوضع النهاري',
      'icon': Icons.light_mode,
      'description': 'خلفية فاتحة مريحة للعين',
    },
    {
      'id': 'dark',
      'name': 'الوضع الليلي',
      'icon': Icons.dark_mode,
      'description': 'خلفية داكنة للقراءة الليلية',
    },
    {
      'id': 'auto',
      'name': 'تلقائي',
      'icon': Icons.brightness_auto,
      'description': 'يتغير حسب وقت اليوم',
    },
  ];

  final List<Map<String, String>> _fontSizes = [
    {'id': 'small', 'name': 'صغير'},
    {'id': 'medium', 'name': 'متوسط'},
    {'id': 'large', 'name': 'كبير'},
    {'id': 'xlarge', 'name': 'كبير جداً'},
  ];

  void _finish() {
    // Save preferences
    // TODO: Save to local storage and backend
    
    // Navigate to home screen
    context.go('/home');
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: AppColors.backgroundPrimary,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        leading: IconButton(
          icon: const Icon(Icons.arrow_forward, color: AppColors.primaryMain),
          onPressed: () => context.pop(),
        ),
      ),
      body: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Header
              Text(
                'تخصيص المظهر',
                style: AppTextStyles.h3.copyWith(
                  color: AppColors.textPrimary,
                  fontWeight: FontWeight.w700,
                ),
                textAlign: TextAlign.center,
              ),

              const SizedBox(height: 12),

              Text(
                'اختر الإعدادات المناسبة لك',
                style: AppTextStyles.body1.copyWith(
                  color: AppColors.textSecondary,
                ),
                textAlign: TextAlign.center,
              ),

              const SizedBox(height: 40),

              // Settings
              Expanded(
                child: SingleChildScrollView(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      // Theme selection
                      Text(
                        'الثيم',
                        style: AppTextStyles.subtitle1.copyWith(
                          color: AppColors.textPrimary,
                          fontWeight: FontWeight.w600,
                        ),
                      ),

                      const SizedBox(height: 16),

                      ...List.generate(_themes.length, (index) {
                        final theme = _themes[index];
                        final isSelected = _selectedTheme == theme['id'];

                        return Padding(
                          padding: const EdgeInsets.only(bottom: 12),
                          child: IslamicCard(
                            padding: EdgeInsets.zero,
                            onTap: () {
                              setState(() {
                                _selectedTheme = theme['id'] as String;
                              });
                            },
                            child: Container(
                              decoration: BoxDecoration(
                                borderRadius: BorderRadius.circular(16),
                                border: Border.all(
                                  color: isSelected
                                      ? AppColors.primaryMain
                                      : Colors.transparent,
                                  width: 2,
                                ),
                              ),
                              padding: const EdgeInsets.all(16),
                              child: Row(
                                children: [
                                  Container(
                                    width: 48,
                                    height: 48,
                                    decoration: BoxDecoration(
                                      color: isSelected
                                          ? AppColors.primaryMain.withOpacity(0.1)
                                          : AppColors.backgroundSecondary,
                                      borderRadius: BorderRadius.circular(12),
                                    ),
                                    child: Icon(
                                      theme['icon'] as IconData,
                                      color: isSelected
                                          ? AppColors.primaryMain
                                          : AppColors.textSecondary,
                                    ),
                                  ),
                                  const SizedBox(width: 16),
                                  Expanded(
                                    child: Column(
                                      crossAxisAlignment: CrossAxisAlignment.start,
                                      children: [
                                        Text(
                                          theme['name'] as String,
                                          style: AppTextStyles.body1.copyWith(
                                            color: AppColors.textPrimary,
                                            fontWeight: FontWeight.w600,
                                          ),
                                        ),
                                        const SizedBox(height: 2),
                                        Text(
                                          theme['description'] as String,
                                          style: AppTextStyles.body2.copyWith(
                                            color: AppColors.textSecondary,
                                          ),
                                        ),
                                      ],
                                    ),
                                  ),
                                  if (isSelected)
                                    const Icon(
                                      Icons.check_circle,
                                      color: AppColors.primaryMain,
                                    ),
                                ],
                              ),
                            ),
                          ),
                        );
                      }),

                      const SizedBox(height: 32),

                      // Font size selection
                      Text(
                        'حجم الخط',
                        style: AppTextStyles.subtitle1.copyWith(
                          color: AppColors.textPrimary,
                          fontWeight: FontWeight.w600,
                        ),
                      ),

                      const SizedBox(height: 16),

                      IslamicCard(
                        padding: const EdgeInsets.all(16),
                        child: Row(
                          mainAxisAlignment: MainAxisAlignment.spaceAround,
                          children: List.generate(_fontSizes.length, (index) {
                            final fontSize = _fontSizes[index];
                            final isSelected = _selectedFontSize == fontSize['id'];

                            return GestureDetector(
                              onTap: () {
                                setState(() {
                                  _selectedFontSize = fontSize['id']!;
                                });
                              },
                              child: Container(
                                padding: const EdgeInsets.symmetric(
                                  horizontal: 16,
                                  vertical: 12,
                                ),
                                decoration: BoxDecoration(
                                  color: isSelected
                                      ? AppColors.primaryMain
                                      : Colors.transparent,
                                  borderRadius: BorderRadius.circular(8),
                                ),
                                child: Text(
                                  fontSize['name']!,
                                  style: AppTextStyles.body2.copyWith(
                                    color: isSelected
                                        ? Colors.white
                                        : AppColors.textSecondary,
                                    fontWeight: isSelected
                                        ? FontWeight.w600
                                        : FontWeight.w400,
                                  ),
                                ),
                              ),
                            );
                          }),
                        ),
                      ),

                      const SizedBox(height: 32),

                      // Animations toggle
                      IslamicCard(
                        padding: const EdgeInsets.all(16),
                        child: Row(
                          children: [
                            Icon(
                              Icons.animation,
                              color: AppColors.primaryMain,
                            ),
                            const SizedBox(width: 16),
                            Expanded(
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  Text(
                                    'الحركات والتأثيرات',
                                    style: AppTextStyles.body1.copyWith(
                                      color: AppColors.textPrimary,
                                      fontWeight: FontWeight.w600,
                                    ),
                                  ),
                                  const SizedBox(height: 2),
                                  Text(
                                    'تفعيل الرسوم المتحركة',
                                    style: AppTextStyles.body2.copyWith(
                                      color: AppColors.textSecondary,
                                    ),
                                  ),
                                ],
                              ),
                            ),
                            Switch(
                              value: _enableAnimations,
                              onChanged: (value) {
                                setState(() {
                                  _enableAnimations = value;
                                });
                              },
                              activeColor: AppColors.primaryMain,
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 24),

              // Finish button
              IslamicButton(
                text: 'ابدأ الاستخدام',
                onPressed: _finish,
                type: IslamicButtonType.primary,
                icon: Icons.check,
              ),
            ],
          ),
        ),
      ),
    );
  }
}

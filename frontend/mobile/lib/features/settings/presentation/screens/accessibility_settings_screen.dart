import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../../../core/providers/accessibility_provider.dart';

/// Accessibility settings screen
class AccessibilitySettingsScreen extends ConsumerWidget {
  const AccessibilitySettingsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final screenReader = ref.watch(screenReaderProvider);
    final highContrast = ref.watch(highContrastProvider);
    final voiceNavigation = ref.watch(voiceNavigationProvider);
    final textScale = ref.watch(textScaleProvider);
    final reduceAnimations = ref.watch(reduceAnimationsProvider);
    final keyboardShortcuts = ref.watch(keyboardShortcutsProvider);

    return Scaffold(
      backgroundColor: AppColors.backgroundPrimary,
      appBar: AppBar(
        backgroundColor: AppColors.primaryMain,
        elevation: 0,
        leading: IconButton(
          icon: const Icon(Icons.arrow_forward, color: Colors.white),
          onPressed: () => context.pop(),
          tooltip: 'رجوع',
        ),
        title: Text(
          'إمكانية الوصول',
          style: AppTextStyles.h5.copyWith(
            color: Colors.white,
            fontWeight: FontWeight.w700,
          ),
        ),
        centerTitle: true,
      ),
      body: SafeArea(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Screen Reader Section
              _buildSectionHeader('قارئ الشاشة'),
              const SizedBox(height: 12),
              Semantics(
                label: 'تفعيل قارئ الشاشة للمساعدة في قراءة محتوى التطبيق',
                child: IslamicCard(
                  padding: const EdgeInsets.all(16),
                  child: Row(
                    children: [
                      const Icon(
                        Icons.accessibility_new,
                        color: AppColors.primaryMain,
                        size: 28,
                      ),
                      const SizedBox(width: 16),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              'تفعيل قارئ الشاشة',
                              style: AppTextStyles.body1.copyWith(
                                color: AppColors.textPrimary,
                                fontWeight: FontWeight.w600,
                              ),
                            ),
                            const SizedBox(height: 4),
                            Text(
                              'قراءة محتوى الشاشة بصوت عالٍ',
                              style: AppTextStyles.body2.copyWith(
                                color: AppColors.textSecondary,
                              ),
                            ),
                          ],
                        ),
                      ),
                      Switch(
                        value: screenReader,
                        onChanged: (value) async {
                          await ref.read(screenReaderProvider.notifier).toggle();
                          if (value) {
                            await ref.read(screenReaderProvider.notifier).announce(
                              'تم تفعيل قارئ الشاشة',
                            );
                          }
                        },
                        activeColor: AppColors.primaryMain,
                      ),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 32),

              // Visual Settings Section
              _buildSectionHeader('الإعدادات البصرية'),
              const SizedBox(height: 12),
              
              Semantics(
                label: 'تفعيل وضع التباين العالي لتحسين الرؤية',
                child: IslamicCard(
                  padding: const EdgeInsets.all(16),
                  child: Row(
                    children: [
                      const Icon(
                        Icons.contrast,
                        color: AppColors.primaryMain,
                        size: 28,
                      ),
                      const SizedBox(width: 16),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              'وضع التباين العالي',
                              style: AppTextStyles.body1.copyWith(
                                color: AppColors.textPrimary,
                                fontWeight: FontWeight.w600,
                              ),
                            ),
                            const SizedBox(height: 4),
                            Text(
                              'ألوان عالية التباين لتحسين الرؤية',
                              style: AppTextStyles.body2.copyWith(
                                color: AppColors.textSecondary,
                              ),
                            ),
                          ],
                        ),
                      ),
                      Switch(
                        value: highContrast,
                        onChanged: (value) async {
                          await ref.read(highContrastProvider.notifier).toggle();
                          if (mounted) {
                            ScaffoldMessenger.of(context).showSnackBar(
                              SnackBar(
                                content: Text(
                                  value
                                      ? 'تم تفعيل وضع التباين العالي'
                                      : 'تم إيقاف وضع التباين العالي',
                                ),
                                duration: const Duration(seconds: 2),
                              ),
                            );
                          }
                        },
                        activeColor: AppColors.primaryMain,
                      ),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 12),

              // Text Scaling
              Semantics(
                label: 'تكبير حجم النصوص. الحجم الحالي: ${(textScale * 100).toInt()}%',
                child: IslamicCard(
                  padding: const EdgeInsets.all(16),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        children: [
                          const Icon(
                            Icons.text_fields,
                            color: AppColors.primaryMain,
                            size: 28,
                          ),
                          const SizedBox(width: 16),
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text(
                                  'حجم النصوص',
                                  style: AppTextStyles.body1.copyWith(
                                    color: AppColors.textPrimary,
                                    fontWeight: FontWeight.w600,
                                  ),
                                ),
                                const SizedBox(height: 4),
                                Text(
                                  '${(textScale * 100).toInt()}%',
                                  style: AppTextStyles.body2.copyWith(
                                    color: AppColors.textSecondary,
                                  ),
                                ),
                              ],
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 16),
                      Row(
                        children: [
                          Expanded(
                            child: Slider(
                              value: textScale,
                              min: 0.8,
                              max: 2.0,
                              divisions: 12,
                              label: '${(textScale * 100).toInt()}%',
                              onChanged: (value) {
                                ref.read(textScaleProvider.notifier).setScale(value);
                              },
                              activeColor: AppColors.primaryMain,
                            ),
                          ),
                          IconButton(
                            icon: const Icon(Icons.refresh),
                            onPressed: () {
                              ref.read(textScaleProvider.notifier).reset();
                            },
                            tooltip: 'إعادة تعيين',
                          ),
                        ],
                      ),
                      const SizedBox(height: 8),
                      Text(
                        'مثال على النص',
                        style: AppTextStyles.body1.copyWith(
                          fontSize: 16 * textScale,
                        ),
                      ),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 12),

              Semantics(
                label: 'تقليل الحركات والتأثيرات البصرية',
                child: IslamicCard(
                  padding: const EdgeInsets.all(16),
                  child: Row(
                    children: [
                      const Icon(
                        Icons.animation,
                        color: AppColors.primaryMain,
                        size: 28,
                      ),
                      const SizedBox(width: 16),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              'تقليل الحركات',
                              style: AppTextStyles.body1.copyWith(
                                color: AppColors.textPrimary,
                                fontWeight: FontWeight.w600,
                              ),
                            ),
                            const SizedBox(height: 4),
                            Text(
                              'تقليل الرسوم المتحركة والتأثيرات',
                              style: AppTextStyles.body2.copyWith(
                                color: AppColors.textSecondary,
                              ),
                            ),
                          ],
                        ),
                      ),
                      Switch(
                        value: reduceAnimations,
                        onChanged: (value) {
                          ref.read(reduceAnimationsProvider.notifier).toggle();
                        },
                        activeColor: AppColors.primaryMain,
                      ),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 32),

              // Navigation Section
              _buildSectionHeader('التنقل'),
              const SizedBox(height: 12),
              
              Semantics(
                label: 'تفعيل التنقل الصوتي باستخدام الأوامر الصوتية',
                child: IslamicCard(
                  padding: const EdgeInsets.all(16),
                  child: Row(
                    children: [
                      const Icon(
                        Icons.mic,
                        color: AppColors.primaryMain,
                        size: 28,
                      ),
                      const SizedBox(width: 16),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              'التنقل الصوتي',
                              style: AppTextStyles.body1.copyWith(
                                color: AppColors.textPrimary,
                                fontWeight: FontWeight.w600,
                              ),
                            ),
                            const SizedBox(height: 4),
                            Text(
                              'استخدام الأوامر الصوتية للتنقل',
                              style: AppTextStyles.body2.copyWith(
                                color: AppColors.textSecondary,
                              ),
                            ),
                          ],
                        ),
                      ),
                      Switch(
                        value: voiceNavigation,
                        onChanged: (value) {
                          ref.read(voiceNavigationProvider.notifier).toggle();
                        },
                        activeColor: AppColors.primaryMain,
                      ),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 12),

              Semantics(
                label: 'تفعيل اختصارات لوحة المفاتيح',
                child: IslamicCard(
                  padding: const EdgeInsets.all(16),
                  child: Row(
                    children: [
                      const Icon(
                        Icons.keyboard,
                        color: AppColors.primaryMain,
                        size: 28,
                      ),
                      const SizedBox(width: 16),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              'اختصارات لوحة المفاتيح',
                              style: AppTextStyles.body1.copyWith(
                                color: AppColors.textPrimary,
                                fontWeight: FontWeight.w600,
                              ),
                            ),
                            const SizedBox(height: 4),
                            Text(
                              'استخدام اختصارات لوحة المفاتيح',
                              style: AppTextStyles.body2.copyWith(
                                color: AppColors.textSecondary,
                              ),
                            ),
                          ],
                        ),
                      ),
                      Switch(
                        value: keyboardShortcuts,
                        onChanged: (value) {
                          ref.read(keyboardShortcutsProvider.notifier).toggle();
                        },
                        activeColor: AppColors.primaryMain,
                      ),
                    ],
                  ),
                ),
              ),

              const SizedBox(height: 32),

              // Help Section
              _buildSectionHeader('المساعدة'),
              const SizedBox(height: 12),
              
              IslamicCard(
                padding: const EdgeInsets.all(16),
                onTap: () => _showKeyboardShortcutsDialog(context),
                child: Row(
                  children: [
                    const Icon(
                      Icons.help_outline,
                      color: AppColors.primaryMain,
                      size: 28,
                    ),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Text(
                        'عرض اختصارات لوحة المفاتيح',
                        style: AppTextStyles.body1.copyWith(
                          color: AppColors.textPrimary,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                    ),
                    const Icon(Icons.arrow_back_ios, size: 16),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildSectionHeader(String title) {
    return Semantics(
      header: true,
      child: Text(
        title,
        style: AppTextStyles.subtitle1.copyWith(
          color: AppColors.textPrimary,
          fontWeight: FontWeight.w700,
        ),
      ),
    );
  }

  void _showKeyboardShortcutsDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('اختصارات لوحة المفاتيح'),
        content: SingleChildScrollView(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              _buildShortcutItem('Ctrl + H', 'الصفحة الرئيسية'),
              _buildShortcutItem('Ctrl + Q', 'القرآن الكريم'),
              _buildShortcutItem('Ctrl + A', 'المساعد الذكي'),
              _buildShortcutItem('Ctrl + S', 'البحث'),
              _buildShortcutItem('Ctrl + P', 'مواقيت الصلاة'),
              _buildShortcutItem('Ctrl + K', 'القبلة'),
              _buildShortcutItem('Ctrl + ,', 'الإعدادات'),
              _buildShortcutItem('Esc', 'رجوع'),
              _buildShortcutItem('Ctrl + +', 'تكبير النص'),
              _buildShortcutItem('Ctrl + -', 'تصغير النص'),
              _buildShortcutItem('Ctrl + 0', 'إعادة تعيين حجم النص'),
            ],
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('إغلاق'),
          ),
        ],
      ),
    );
  }

  Widget _buildShortcutItem(String shortcut, String description) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: Row(
        children: [
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
            decoration: BoxDecoration(
              color: AppColors.backgroundSecondary,
              borderRadius: BorderRadius.circular(6),
              border: Border.all(color: AppColors.primaryMain.withOpacity(0.3)),
            ),
            child: Text(
              shortcut,
              style: AppTextStyles.body2.copyWith(
                fontWeight: FontWeight.w600,
                fontFamily: 'monospace',
              ),
            ),
          ),
          const SizedBox(width: 16),
          Expanded(
            child: Text(
              description,
              style: AppTextStyles.body2,
            ),
          ),
        ],
      ),
    );
  }
}

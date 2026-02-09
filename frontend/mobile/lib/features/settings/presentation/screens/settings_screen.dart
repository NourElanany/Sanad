import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_button.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../../../core/providers/preferences_provider.dart';
import '../../../../core/services/preferences_service.dart';

/// Settings screen for managing user preferences
class SettingsScreen extends ConsumerWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final prefsService = ref.watch(preferencesServiceProvider);
    final madhab = ref.watch(madhabProvider);
    final theme = ref.watch(themeProvider);
    final fontSize = ref.watch(fontSizeProvider);
    final animations = ref.watch(animationsProvider);
    final notifications = ref.watch(notificationsProvider);
    final language = ref.watch(languageProvider);

    return Scaffold(
      backgroundColor: AppColors.backgroundPrimary,
      appBar: AppBar(
        backgroundColor: AppColors.primaryMain,
        elevation: 0,
        leading: IconButton(
          icon: const Icon(Icons.arrow_forward, color: Colors.white),
          onPressed: () => context.pop(),
        ),
        title: Text(
          'الإعدادات',
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
              // Prayer Settings Section
              _buildSectionHeader('إعدادات الصلاة'),
              const SizedBox(height: 12),
              IslamicCard(
                padding: const EdgeInsets.all(16),
                onTap: () => _showMadhabPicker(context, ref, madhab),
                child: Row(
                  children: [
                    const Icon(Icons.mosque, color: AppColors.primaryMain),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            'المذهب الفقهي',
                            style: AppTextStyles.body1.copyWith(
                              color: AppColors.textPrimary,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                          const SizedBox(height: 4),
                          Text(
                            madhab ?? 'غير محدد',
                            style: AppTextStyles.body2.copyWith(
                              color: AppColors.textSecondary,
                            ),
                          ),
                        ],
                      ),
                    ),
                    const Icon(Icons.arrow_back_ios, size: 16),
                  ],
                ),
              ),

              const SizedBox(height: 32),

              // Display Settings Section
              _buildSectionHeader('إعدادات العرض'),
              const SizedBox(height: 12),
              
              IslamicCard(
                padding: const EdgeInsets.all(16),
                onTap: () => _showThemePicker(context, ref, theme),
                child: Row(
                  children: [
                    Icon(
                      theme == 'dark' ? Icons.dark_mode : Icons.light_mode,
                      color: AppColors.primaryMain,
                    ),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            'الثيم',
                            style: AppTextStyles.body1.copyWith(
                              color: AppColors.textPrimary,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                          const SizedBox(height: 4),
                          Text(
                            _getThemeName(theme),
                            style: AppTextStyles.body2.copyWith(
                              color: AppColors.textSecondary,
                            ),
                          ),
                        ],
                      ),
                    ),
                    const Icon(Icons.arrow_back_ios, size: 16),
                  ],
                ),
              ),

              const SizedBox(height: 12),

              IslamicCard(
                padding: const EdgeInsets.all(16),
                onTap: () => _showFontSizePicker(context, ref, fontSize),
                child: Row(
                  children: [
                    const Icon(Icons.text_fields, color: AppColors.primaryMain),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            'حجم الخط',
                            style: AppTextStyles.body1.copyWith(
                              color: AppColors.textPrimary,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                          const SizedBox(height: 4),
                          Text(
                            _getFontSizeName(fontSize),
                            style: AppTextStyles.body2.copyWith(
                              color: AppColors.textSecondary,
                            ),
                          ),
                        ],
                      ),
                    ),
                    const Icon(Icons.arrow_back_ios, size: 16),
                  ],
                ),
              ),

              const SizedBox(height: 12),

              IslamicCard(
                padding: const EdgeInsets.all(16),
                child: Row(
                  children: [
                    const Icon(Icons.animation, color: AppColors.primaryMain),
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
                          const SizedBox(height: 4),
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
                      value: animations,
                      onChanged: (value) async {
                        ref.read(animationsProvider.notifier).state = value;
                        await prefsService.setEnableAnimations(value);
                      },
                      activeColor: AppColors.primaryMain,
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 32),

              // Notification Settings Section
              _buildSectionHeader('إعدادات الإشعارات'),
              const SizedBox(height: 12),
              
              IslamicCard(
                padding: const EdgeInsets.all(16),
                child: Row(
                  children: [
                    const Icon(Icons.notifications, color: AppColors.primaryMain),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            'تنبيهات الصلاة',
                            style: AppTextStyles.body1.copyWith(
                              color: AppColors.textPrimary,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                          const SizedBox(height: 4),
                          Text(
                            'تلقي تنبيهات مواقيت الصلاة',
                            style: AppTextStyles.body2.copyWith(
                              color: AppColors.textSecondary,
                            ),
                          ),
                        ],
                      ),
                    ),
                    Switch(
                      value: notifications,
                      onChanged: (value) async {
                        ref.read(notificationsProvider.notifier).state = value;
                        await prefsService.setEnableNotifications(value);
                      },
                      activeColor: AppColors.primaryMain,
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 32),

              // Accessibility Section
              _buildSectionHeader('إمكانية الوصول'),
              const SizedBox(height: 12),
              
              IslamicCard(
                padding: const EdgeInsets.all(16),
                onTap: () => context.push('/accessibility'),
                child: Row(
                  children: [
                    const Icon(Icons.accessibility_new, color: AppColors.primaryMain),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            'إعدادات إمكانية الوصول',
                            style: AppTextStyles.body1.copyWith(
                              color: AppColors.textPrimary,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                          const SizedBox(height: 4),
                          Text(
                            'قارئ الشاشة، التباين العالي، والمزيد',
                            style: AppTextStyles.body2.copyWith(
                              color: AppColors.textSecondary,
                            ),
                          ),
                        ],
                      ),
                    ),
                    const Icon(Icons.arrow_back_ios, size: 16),
                  ),
                ),
              ),

              const SizedBox(height: 32),

              // Data Management Section
              _buildSectionHeader('إدارة البيانات'),
              const SizedBox(height: 12),
              
              IslamicCard(
                padding: const EdgeInsets.all(16),
                onTap: () => _showBackupDialog(context, prefsService),
                child: Row(
                  children: [
                    const Icon(Icons.backup, color: AppColors.primaryMain),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Text(
                        'نسخ احتياطي للإعدادات',
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

              const SizedBox(height: 12),

              IslamicCard(
                padding: const EdgeInsets.all(16),
                onTap: () => _showRestoreDialog(context, prefsService),
                child: Row(
                  children: [
                    const Icon(Icons.restore, color: AppColors.primaryMain),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Text(
                        'استعادة الإعدادات',
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

              const SizedBox(height: 12),

              IslamicCard(
                padding: const EdgeInsets.all(16),
                onTap: () => _showResetDialog(context, prefsService, ref),
                child: Row(
                  children: [
                    const Icon(Icons.refresh, color: AppColors.statusError),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Text(
                        'إعادة تعيين الإعدادات',
                        style: AppTextStyles.body1.copyWith(
                          color: AppColors.statusError,
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
    return Text(
      title,
      style: AppTextStyles.subtitle1.copyWith(
        color: AppColors.textPrimary,
        fontWeight: FontWeight.w700,
      ),
    );
  }

  String _getThemeName(String theme) {
    switch (theme) {
      case 'light':
        return 'الوضع النهاري';
      case 'dark':
        return 'الوضع الليلي';
      case 'auto':
        return 'تلقائي';
      default:
        return theme;
    }
  }

  String _getFontSizeName(String fontSize) {
    switch (fontSize) {
      case 'small':
        return 'صغير';
      case 'medium':
        return 'متوسط';
      case 'large':
        return 'كبير';
      case 'xlarge':
        return 'كبير جداً';
      default:
        return fontSize;
    }
  }

  void _showMadhabPicker(BuildContext context, WidgetRef ref, String? currentMadhab) {
    // Implementation for madhab picker
    // Similar to the onboarding madhab selection
  }

  void _showThemePicker(BuildContext context, WidgetRef ref, String currentTheme) {
    // Implementation for theme picker
  }

  void _showFontSizePicker(BuildContext context, WidgetRef ref, String currentFontSize) {
    // Implementation for font size picker
  }

  void _showBackupDialog(BuildContext context, PreferencesService prefsService) {
    final backup = prefsService.backupPreferences();
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('نسخ احتياطي للإعدادات'),
        content: SelectableText(backup),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('إغلاق'),
          ),
        ],
      ),
    );
  }

  void _showRestoreDialog(BuildContext context, PreferencesService prefsService) {
    // Implementation for restore dialog
  }

  void _showResetDialog(BuildContext context, PreferencesService prefsService, WidgetRef ref) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('إعادة تعيين الإعدادات'),
        content: const Text('هل أنت متأكد من إعادة تعيين جميع الإعدادات إلى القيم الافتراضية؟'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('إلغاء'),
          ),
          TextButton(
            onPressed: () async {
              await prefsService.resetToDefaults();
              Navigator.pop(context);
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(content: Text('تم إعادة تعيين الإعدادات')),
              );
            },
            child: const Text('تأكيد', style: TextStyle(color: AppColors.statusError)),
          ),
        ],
      ),
    );
  }
}

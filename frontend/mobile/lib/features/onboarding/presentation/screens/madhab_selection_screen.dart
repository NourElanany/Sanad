import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_button.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Screen for selecting Islamic jurisprudence school (madhab)
class MadhabSelectionScreen extends StatefulWidget {
  const MadhabSelectionScreen({super.key});

  @override
  State<MadhabSelectionScreen> createState() => _MadhabSelectionScreenState();
}

class _MadhabSelectionScreenState extends State<MadhabSelectionScreen> {
  String? _selectedMadhab;

  final List<Map<String, String>> _madhabs = [
    {
      'id': 'hanafi',
      'name': 'الحنفي',
      'description': 'مذهب الإمام أبي حنيفة النعمان',
    },
    {
      'id': 'maliki',
      'name': 'المالكي',
      'description': 'مذهب الإمام مالك بن أنس',
    },
    {
      'id': 'shafii',
      'name': 'الشافعي',
      'description': 'مذهب الإمام محمد بن إدريس الشافعي',
    },
    {
      'id': 'hanbali',
      'name': 'الحنبلي',
      'description': 'مذهب الإمام أحمد بن حنبل',
    },
    {
      'id': 'jafari',
      'name': 'الجعفري',
      'description': 'مذهب الإمام جعفر الصادق',
    },
  ];

  void _continue() {
    if (_selectedMadhab == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('الرجاء اختيار المذهب الفقهي'),
          backgroundColor: AppColors.statusError,
        ),
      );
      return;
    }

    // Save madhab preference
    // TODO: Save to local storage and backend
    
    // Navigate to theme selection
    context.go('/onboarding/theme');
  }

  void _skip() {
    // Navigate to theme selection
    context.go('/onboarding/theme');
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
        actions: [
          TextButton(
            onPressed: _skip,
            child: Text(
              'تخطي',
              style: AppTextStyles.body1.copyWith(
                color: AppColors.primaryMain,
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
        ],
      ),
      body: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Header
              Text(
                'اختر المذهب الفقهي',
                style: AppTextStyles.h3.copyWith(
                  color: AppColors.textPrimary,
                  fontWeight: FontWeight.w700,
                ),
                textAlign: TextAlign.center,
              ),

              const SizedBox(height: 12),

              Text(
                'سيتم استخدامه لحساب مواقيت الصلاة والفتاوى',
                style: AppTextStyles.body1.copyWith(
                  color: AppColors.textSecondary,
                ),
                textAlign: TextAlign.center,
              ),

              const SizedBox(height: 40),

              // Madhab options
              Expanded(
                child: ListView.separated(
                  itemCount: _madhabs.length,
                  separatorBuilder: (context, index) => const SizedBox(height: 12),
                  itemBuilder: (context, index) {
                    final madhab = _madhabs[index];
                    final isSelected = _selectedMadhab == madhab['id'];

                    return IslamicCard(
                      padding: EdgeInsets.zero,
                      onTap: () {
                        setState(() {
                          _selectedMadhab = madhab['id'];
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
                        padding: const EdgeInsets.all(20),
                        child: Row(
                          children: [
                            // Radio button
                            Container(
                              width: 24,
                              height: 24,
                              decoration: BoxDecoration(
                                shape: BoxShape.circle,
                                border: Border.all(
                                  color: isSelected
                                      ? AppColors.primaryMain
                                      : AppColors.textSecondary,
                                  width: 2,
                                ),
                                color: isSelected
                                    ? AppColors.primaryMain
                                    : Colors.transparent,
                              ),
                              child: isSelected
                                  ? const Icon(
                                      Icons.check,
                                      size: 16,
                                      color: Colors.white,
                                    )
                                  : null,
                            ),

                            const SizedBox(width: 16),

                            // Text content
                            Expanded(
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                children: [
                                  Text(
                                    madhab['name']!,
                                    style: AppTextStyles.subtitle1.copyWith(
                                      color: AppColors.textPrimary,
                                      fontWeight: FontWeight.w600,
                                    ),
                                  ),
                                  const SizedBox(height: 4),
                                  Text(
                                    madhab['description']!,
                                    style: AppTextStyles.body2.copyWith(
                                      color: AppColors.textSecondary,
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          ],
                        ),
                      ),
                    );
                  },
                ),
              ),

              const SizedBox(height: 24),

              // Continue button
              IslamicButton(
                text: 'متابعة',
                onPressed: _continue,
                type: IslamicButtonType.primary,
                icon: Icons.arrow_back,
              ),
            ],
          ),
        ),
      ),
    );
  }
}

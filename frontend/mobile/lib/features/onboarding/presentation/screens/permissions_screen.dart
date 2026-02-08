import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:permission_handler/permission_handler.dart';

import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_button.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Screen for requesting necessary permissions
class PermissionsScreen extends StatefulWidget {
  const PermissionsScreen({super.key});

  @override
  State<PermissionsScreen> createState() => _PermissionsScreenState();
}

class _PermissionsScreenState extends State<PermissionsScreen> {
  bool _locationGranted = false;
  bool _microphoneGranted = false;
  bool _notificationsGranted = false;

  @override
  void initState() {
    super.initState();
    _checkPermissions();
  }

  Future<void> _checkPermissions() async {
    final locationStatus = await Permission.location.status;
    final microphoneStatus = await Permission.microphone.status;
    final notificationStatus = await Permission.notification.status;

    setState(() {
      _locationGranted = locationStatus.isGranted;
      _microphoneGranted = microphoneStatus.isGranted;
      _notificationsGranted = notificationStatus.isGranted;
    });
  }

  Future<void> _requestLocationPermission() async {
    final status = await Permission.location.request();
    setState(() {
      _locationGranted = status.isGranted;
    });
  }

  Future<void> _requestMicrophonePermission() async {
    final status = await Permission.microphone.request();
    setState(() {
      _microphoneGranted = status.isGranted;
    });
  }

  Future<void> _requestNotificationPermission() async {
    final status = await Permission.notification.request();
    setState(() {
      _notificationsGranted = status.isGranted;
    });
  }

  void _continue() {
    // Navigate to madhab selection
    context.go('/onboarding/madhab');
  }

  void _skip() {
    // Navigate to madhab selection
    context.go('/onboarding/madhab');
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: AppColors.backgroundPrimary,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
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
                'الأذونات المطلوبة',
                style: AppTextStyles.h3.copyWith(
                  color: AppColors.textPrimary,
                  fontWeight: FontWeight.w700,
                ),
                textAlign: TextAlign.center,
              ),

              const SizedBox(height: 12),

              Text(
                'نحتاج بعض الأذونات لتقديم أفضل تجربة لك',
                style: AppTextStyles.body1.copyWith(
                  color: AppColors.textSecondary,
                ),
                textAlign: TextAlign.center,
              ),

              const SizedBox(height: 40),

              // Permission cards
              Expanded(
                child: SingleChildScrollView(
                  child: Column(
                    children: [
                      _buildPermissionCard(
                        icon: Icons.location_on,
                        title: 'الموقع',
                        description: 'لحساب مواقيت الصلاة الدقيقة حسب موقعك',
                        isGranted: _locationGranted,
                        onRequest: _requestLocationPermission,
                      ),

                      const SizedBox(height: 16),

                      _buildPermissionCard(
                        icon: Icons.mic,
                        title: 'الميكروفون',
                        description: 'لتسجيل تلاوتك وتحليل التجويد',
                        isGranted: _microphoneGranted,
                        onRequest: _requestMicrophonePermission,
                      ),

                      const SizedBox(height: 16),

                      _buildPermissionCard(
                        icon: Icons.notifications,
                        title: 'الإشعارات',
                        description: 'لتذكيرك بمواقيت الصلاة والأذكار',
                        isGranted: _notificationsGranted,
                        onRequest: _requestNotificationPermission,
                      ),
                    ],
                  ),
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

  Widget _buildPermissionCard({
    required IconData icon,
    required String title,
    required String description,
    required bool isGranted,
    required VoidCallback onRequest,
  }) {
    return IslamicCard(
      padding: const EdgeInsets.all(20),
      child: Row(
        children: [
          // Icon
          Container(
            width: 56,
            height: 56,
            decoration: BoxDecoration(
              color: isGranted
                  ? AppColors.statusSuccess.withOpacity(0.1)
                  : AppColors.primaryMain.withOpacity(0.1),
              borderRadius: BorderRadius.circular(16),
            ),
            child: Icon(
              icon,
              color: isGranted ? AppColors.statusSuccess : AppColors.primaryMain,
              size: 28,
            ),
          ),

          const SizedBox(width: 16),

          // Text content
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  title,
                  style: AppTextStyles.subtitle1.copyWith(
                    color: AppColors.textPrimary,
                    fontWeight: FontWeight.w600,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  description,
                  style: AppTextStyles.body2.copyWith(
                    color: AppColors.textSecondary,
                  ),
                ),
              ],
            ),
          ),

          const SizedBox(width: 12),

          // Action button
          if (isGranted)
            Icon(
              Icons.check_circle,
              color: AppColors.statusSuccess,
              size: 28,
            )
          else
            TextButton(
              onPressed: onRequest,
              style: TextButton.styleFrom(
                padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                backgroundColor: AppColors.primaryMain,
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(8),
                ),
              ),
              child: Text(
                'السماح',
                style: AppTextStyles.body2.copyWith(
                  color: Colors.white,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
        ],
      ),
    );
  }
}

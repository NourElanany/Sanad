import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dart:math' as math;
import '../../../../core/providers/qibla_provider.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/widgets/islamic_button.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../widgets/compass_widget.dart';
import '../widgets/qibla_info_card.dart';
import '../widgets/calibration_dialog.dart';

/// Main screen for Qibla compass with AR visualization
class QiblaCompassScreen extends ConsumerStatefulWidget {
  const QiblaCompassScreen({super.key});

  @override
  ConsumerState<QiblaCompassScreen> createState() =>
      _QiblaCompassScreenState();
}

class _QiblaCompassScreenState extends ConsumerState<QiblaCompassScreen> {
  @override
  void initState() {
    super.initState();
    // Initialize Qibla compass when screen loads
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(qiblaProvider.notifier).initialize();
    });
  }

  @override
  Widget build(BuildContext context) {
    final qiblaState = ref.watch(qiblaProvider);
    final isNightMode = qiblaState.isNightMode;

    return Scaffold(
      backgroundColor: isNightMode
          ? AppColors.primaryDark
          : AppColors.backgroundPrimary,
      appBar: AppBar(
        title: const Text(
          'بوصلة القبلة',
          style: TextStyle(
            fontFamily: 'Tajawal',
            fontWeight: FontWeight.bold,
          ),
        ),
        centerTitle: true,
        backgroundColor: isNightMode
            ? AppColors.primaryDark
            : AppColors.primaryMain,
        actions: [
          // Night mode toggle
          IconButton(
            icon: Icon(
              isNightMode ? Icons.light_mode : Icons.dark_mode,
              color: Colors.white,
            ),
            onPressed: () {
              ref.read(qiblaProvider.notifier).toggleNightMode();
            },
            tooltip: isNightMode ? 'الوضع النهاري' : 'الوضع الليلي',
          ),
          // Refresh button
          IconButton(
            icon: const Icon(Icons.refresh, color: Colors.white),
            onPressed: qiblaState.isLoading
                ? null
                : () {
                    ref.read(qiblaProvider.notifier).refresh();
                  },
            tooltip: 'تحديث الموقع',
          ),
        ],
      ),
      body: _buildBody(context, qiblaState, isNightMode),
    );
  }

  Widget _buildBody(BuildContext context, dynamic qiblaState, bool isNightMode) {
    if (qiblaState.isLoading) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            CircularProgressIndicator(
              valueColor: AlwaysStoppedAnimation<Color>(
                isNightMode ? AppColors.accentGold : AppColors.primaryMain,
              ),
            ),
            const SizedBox(height: 16),
            Text(
              'جاري تحديد موقعك...',
              style: TextStyle(
                fontFamily: 'Tajawal',
                fontSize: 16,
                color: isNightMode ? Colors.white70 : AppColors.textSecondary,
              ),
            ),
          ],
        ),
      );
    }

    if (qiblaState.error != null) {
      return Center(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(
                Icons.error_outline,
                size: 64,
                color: isNightMode ? Colors.red[300] : Colors.red,
              ),
              const SizedBox(height: 16),
              Text(
                qiblaState.error!,
                textAlign: TextAlign.center,
                style: TextStyle(
                  fontFamily: 'Tajawal',
                  fontSize: 16,
                  color: isNightMode ? Colors.white : AppColors.textPrimary,
                ),
              ),
              const SizedBox(height: 24),
              IslamicButton(
                text: 'إعادة المحاولة',
                onPressed: () {
                  ref.read(qiblaProvider.notifier).initialize();
                },
                type: IslamicButtonType.primary,
              ),
            ],
          ),
        ),
      );
    }

    if (qiblaState.qiblaData == null) {
      return const Center(
        child: Text('لا توجد بيانات'),
      );
    }

    return SingleChildScrollView(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            // Calibration warning if needed
            if (!qiblaState.compassState.calibration.isCalibrated)
              _buildCalibrationWarning(context, qiblaState, isNightMode),

            const SizedBox(height: 16),

            // Main compass widget
            CompassWidget(
              compassState: qiblaState.compassState,
              isNightMode: isNightMode,
            ),

            const SizedBox(height: 24),

            // Qibla information card
            QiblaInfoCard(
              qiblaData: qiblaState.qiblaData!,
              isNightMode: isNightMode,
            ),

            const SizedBox(height: 16),

            // Direction indicator
            _buildDirectionIndicator(qiblaState, isNightMode),

            const SizedBox(height: 24),

            // Calibration button
            IslamicButton(
              text: 'معايرة البوصلة',
              icon: Icons.settings_backup_restore,
              onPressed: () {
                _showCalibrationDialog(context);
              },
              type: IslamicButtonType.secondary,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildCalibrationWarning(
    BuildContext context,
    dynamic qiblaState,
    bool isNightMode,
  ) {
    return IslamicCard(
      padding: const EdgeInsets.all(16),
      backgroundColor: isNightMode
          ? Colors.orange[900]!.withOpacity(0.3)
          : Colors.orange[50],
      child: Row(
        children: [
          Icon(
            Icons.warning_amber_rounded,
            color: isNightMode ? Colors.orange[300] : Colors.orange[700],
            size: 32,
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  'تحتاج البوصلة إلى معايرة',
                  style: TextStyle(
                    fontFamily: 'Tajawal',
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    color: isNightMode ? Colors.white : AppColors.textPrimary,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  qiblaState.compassState.calibration.message,
                  style: TextStyle(
                    fontFamily: 'Tajawal',
                    fontSize: 14,
                    color: isNightMode
                        ? Colors.white70
                        : AppColors.textSecondary,
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildDirectionIndicator(dynamic qiblaState, bool isNightMode) {
    final relativeDirection = qiblaState.compassState.relativeDirection;
    final isPointingToQibla = qiblaState.compassState.isPointingToQibla;

    String directionText;
    IconData directionIcon;
    Color directionColor;

    if (isPointingToQibla) {
      directionText = 'أنت تتجه نحو القبلة ✓';
      directionIcon = Icons.check_circle;
      directionColor = isNightMode ? Colors.green[300]! : Colors.green;
    } else if (relativeDirection > 0) {
      directionText = 'اتجه يميناً ${relativeDirection.abs().toStringAsFixed(0)}°';
      directionIcon = Icons.arrow_forward;
      directionColor = isNightMode ? AppColors.accentGold : AppColors.primaryMain;
    } else {
      directionText = 'اتجه يساراً ${relativeDirection.abs().toStringAsFixed(0)}°';
      directionIcon = Icons.arrow_back;
      directionColor = isNightMode ? AppColors.accentGold : AppColors.primaryMain;
    }

    return IslamicCard(
      padding: const EdgeInsets.all(20),
      backgroundColor: isNightMode
          ? AppColors.primaryMain.withOpacity(0.3)
          : AppColors.backgroundSecondary,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            directionIcon,
            color: directionColor,
            size: 32,
          ),
          const SizedBox(width: 12),
          Text(
            directionText,
            style: TextStyle(
              fontFamily: 'Tajawal',
              fontSize: 20,
              fontWeight: FontWeight.bold,
              color: directionColor,
            ),
          ),
        ],
      ),
    );
  }

  void _showCalibrationDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => const CalibrationDialog(),
    );
  }
}

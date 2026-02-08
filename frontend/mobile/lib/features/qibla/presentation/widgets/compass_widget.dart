import 'package:flutter/material.dart';
import 'dart:math' as math;
import '../../data/models/qibla_model.dart';
import '../../../../core/theme/app_colors.dart';

/// Animated compass widget with AR-like visualization
class CompassWidget extends StatefulWidget {
  final CompassState compassState;
  final bool isNightMode;

  const CompassWidget({
    super.key,
    required this.compassState,
    this.isNightMode = false,
  });

  @override
  State<CompassWidget> createState() => _CompassWidgetState();
}

class _CompassWidgetState extends State<CompassWidget>
    with SingleTickerProviderStateMixin {
  late AnimationController _pulseController;
  late Animation<double> _pulseAnimation;

  @override
  void initState() {
    super.initState();
    _pulseController = AnimationController(
      duration: const Duration(milliseconds: 1500),
      vsync: this,
    )..repeat(reverse: true);

    _pulseAnimation = Tween<double>(begin: 0.9, end: 1.1).animate(
      CurvedAnimation(parent: _pulseController, curve: Curves.easeInOut),
    );
  }

  @override
  void dispose() {
    _pulseController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      width: double.infinity,
      height: 400,
      decoration: BoxDecoration(
        color: widget.isNightMode
            ? AppColors.primaryDark.withOpacity(0.5)
            : AppColors.backgroundPaper,
        borderRadius: BorderRadius.circular(24),
        boxShadow: [
          BoxShadow(
            color: widget.isNightMode
                ? Colors.black.withOpacity(0.5)
                : AppColors.primaryMain.withOpacity(0.1),
            blurRadius: 20,
            offset: const Offset(0, 8),
          ),
        ],
      ),
      child: Stack(
        alignment: Alignment.center,
        children: [
          // Background gradient
          Container(
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(24),
              gradient: RadialGradient(
                colors: widget.isNightMode
                    ? [
                        AppColors.primaryMain.withOpacity(0.3),
                        AppColors.primaryDark.withOpacity(0.1),
                      ]
                    : [
                        AppColors.accentGold.withOpacity(0.1),
                        Colors.transparent,
                      ],
              ),
            ),
          ),

          // Compass rose
          Transform.rotate(
            angle: -widget.compassState.heading * math.pi / 180,
            child: _buildCompassRose(),
          ),

          // Qibla direction indicator (fixed)
          Transform.rotate(
            angle: widget.compassState.relativeDirection * math.pi / 180,
            child: _buildQiblaIndicator(),
          ),

          // Center Kaaba icon
          _buildCenterIcon(),

          // Heading text at top
          Positioned(
            top: 20,
            child: _buildHeadingText(),
          ),
        ],
      ),
    );
  }

  Widget _buildCompassRose() {
    return SizedBox(
      width: 300,
      height: 300,
      child: CustomPaint(
        painter: CompassRosePainter(
          isNightMode: widget.isNightMode,
        ),
      ),
    );
  }

  Widget _buildQiblaIndicator() {
    final isPointingToQibla = widget.compassState.isPointingToQibla;

    return AnimatedBuilder(
      animation: _pulseAnimation,
      builder: (context, child) {
        return Transform.scale(
          scale: isPointingToQibla ? _pulseAnimation.value : 1.0,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              // Arrow pointing to Qibla
              Icon(
                Icons.navigation,
                size: 60,
                color: isPointingToQibla
                    ? (widget.isNightMode ? Colors.green[300] : Colors.green)
                    : (widget.isNightMode
                        ? AppColors.accentGold
                        : AppColors.primaryMain),
              ),
              const SizedBox(height: 8),
              // Kaaba text
              Container(
                padding: const EdgeInsets.symmetric(
                  horizontal: 12,
                  vertical: 6,
                ),
                decoration: BoxDecoration(
                  color: isPointingToQibla
                      ? (widget.isNightMode
                          ? Colors.green[900]!.withOpacity(0.8)
                          : Colors.green.withOpacity(0.2))
                      : (widget.isNightMode
                          ? AppColors.primaryMain.withOpacity(0.8)
                          : AppColors.accentGold.withOpacity(0.2)),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Text(
                  'الكعبة',
                  style: TextStyle(
                    fontFamily: 'Tajawal',
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    color: widget.isNightMode
                        ? Colors.white
                        : AppColors.textPrimary,
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  Widget _buildCenterIcon() {
    return Container(
      width: 80,
      height: 80,
      decoration: BoxDecoration(
        shape: BoxShape.circle,
        color: widget.isNightMode
            ? AppColors.primaryMain.withOpacity(0.5)
            : AppColors.backgroundPaper,
        border: Border.all(
          color: widget.isNightMode
              ? AppColors.accentGold
              : AppColors.primaryMain,
          width: 3,
        ),
        boxShadow: [
          BoxShadow(
            color: widget.isNightMode
                ? AppColors.accentGold.withOpacity(0.3)
                : AppColors.primaryMain.withOpacity(0.2),
            blurRadius: 12,
            spreadRadius: 2,
          ),
        ],
      ),
      child: Icon(
        Icons.mosque,
        size: 40,
        color: widget.isNightMode
            ? AppColors.accentGold
            : AppColors.primaryMain,
      ),
    );
  }

  Widget _buildHeadingText() {
    final heading = widget.compassState.heading.toStringAsFixed(0);
    final direction = _getCardinalDirection(widget.compassState.heading);

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      decoration: BoxDecoration(
        color: widget.isNightMode
            ? AppColors.primaryMain.withOpacity(0.8)
            : AppColors.backgroundPaper,
        borderRadius: BorderRadius.circular(20),
        border: Border.all(
          color: widget.isNightMode
              ? AppColors.accentGold.withOpacity(0.5)
              : AppColors.primaryMain.withOpacity(0.3),
        ),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            '$heading°',
            style: TextStyle(
              fontFamily: 'Tajawal',
              fontSize: 24,
              fontWeight: FontWeight.bold,
              color: widget.isNightMode
                  ? AppColors.accentGold
                  : AppColors.primaryMain,
            ),
          ),
          const SizedBox(width: 8),
          Text(
            direction,
            style: TextStyle(
              fontFamily: 'Tajawal',
              fontSize: 18,
              color: widget.isNightMode ? Colors.white70 : AppColors.textSecondary,
            ),
          ),
        ],
      ),
    );
  }

  String _getCardinalDirection(double heading) {
    if (heading >= 337.5 || heading < 22.5) return 'شمال';
    if (heading >= 22.5 && heading < 67.5) return 'شمال شرق';
    if (heading >= 67.5 && heading < 112.5) return 'شرق';
    if (heading >= 112.5 && heading < 157.5) return 'جنوب شرق';
    if (heading >= 157.5 && heading < 202.5) return 'جنوب';
    if (heading >= 202.5 && heading < 247.5) return 'جنوب غرب';
    if (heading >= 247.5 && heading < 292.5) return 'غرب';
    return 'شمال غرب';
  }
}

/// Custom painter for compass rose
class CompassRosePainter extends CustomPainter {
  final bool isNightMode;

  CompassRosePainter({required this.isNightMode});

  @override
  void paint(Canvas canvas, Size size) {
    final center = Offset(size.width / 2, size.height / 2);
    final radius = size.width / 2;

    // Draw outer circle
    final outerCirclePaint = Paint()
      ..color = (isNightMode
              ? AppColors.accentGold
              : AppColors.primaryMain)
          .withOpacity(0.3)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2;
    canvas.drawCircle(center, radius - 10, outerCirclePaint);

    // Draw cardinal directions
    final textPainter = TextPainter(
      textDirection: TextDirection.rtl,
      textAlign: TextAlign.center,
    );

    final directions = ['ش', 'ق', 'ج', 'غ']; // N, E, S, W in Arabic
    final angles = [0, 90, 180, 270];

    for (int i = 0; i < 4; i++) {
      final angle = angles[i] * math.pi / 180;
      final x = center.dx + (radius - 40) * math.sin(angle);
      final y = center.dy - (radius - 40) * math.cos(angle);

      textPainter.text = TextSpan(
        text: directions[i],
        style: TextStyle(
          fontFamily: 'Tajawal',
          fontSize: 24,
          fontWeight: FontWeight.bold,
          color: isNightMode ? Colors.white : AppColors.primaryMain,
        ),
      );
      textPainter.layout();
      textPainter.paint(
        canvas,
        Offset(x - textPainter.width / 2, y - textPainter.height / 2),
      );
    }

    // Draw tick marks
    final tickPaint = Paint()
      ..color = (isNightMode ? AppColors.accentGold : AppColors.primaryMain)
          .withOpacity(0.5)
      ..strokeWidth = 2;

    for (int i = 0; i < 360; i += 10) {
      final angle = i * math.pi / 180;
      final isMajor = i % 30 == 0;
      final tickLength = isMajor ? 15.0 : 8.0;

      final startX = center.dx + (radius - 10) * math.sin(angle);
      final startY = center.dy - (radius - 10) * math.cos(angle);
      final endX = center.dx + (radius - 10 - tickLength) * math.sin(angle);
      final endY = center.dy - (radius - 10 - tickLength) * math.cos(angle);

      canvas.drawLine(
        Offset(startX, startY),
        Offset(endX, endY),
        tickPaint,
      );
    }
  }

  @override
  bool shouldRepaint(CompassRosePainter oldDelegate) =>
      oldDelegate.isNightMode != isNightMode;
}

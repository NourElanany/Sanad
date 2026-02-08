import 'package:flutter/material.dart';
import '../theme/app_colors.dart';

/// Islamic-themed loading indicator with various styles
class IslamicLoadingIndicator extends StatelessWidget {
  final double size;
  final Color? color;
  final double strokeWidth;

  const IslamicLoadingIndicator({
    Key? key,
    this.size = 40,
    this.color,
    this.strokeWidth = 3,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: size,
      height: size,
      child: CircularProgressIndicator(
        strokeWidth: strokeWidth,
        valueColor: AlwaysStoppedAnimation<Color>(
          color ?? AppColors.primaryMain,
        ),
      ),
    );
  }
}

/// Islamic-themed loading indicator with text
class IslamicLoadingWithText extends StatelessWidget {
  final String text;
  final double size;
  final Color? color;

  const IslamicLoadingWithText({
    Key? key,
    required this.text,
    this.size = 40,
    this.color,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        IslamicLoadingIndicator(
          size: size,
          color: color,
        ),
        const SizedBox(height: 16),
        Text(
          text,
          style: TextStyle(
            fontSize: 16,
            fontWeight: FontWeight.w500,
            fontFamily: 'Tajawal',
            color: color ?? AppColors.textSecondary,
          ),
          textAlign: TextAlign.center,
        ),
      ],
    );
  }
}

/// Islamic-themed shimmer loading effect
class IslamicShimmer extends StatefulWidget {
  final double width;
  final double height;
  final double borderRadius;

  const IslamicShimmer({
    Key? key,
    this.width = double.infinity,
    this.height = 100,
    this.borderRadius = 12,
  }) : super(key: key);

  @override
  State<IslamicShimmer> createState() => _IslamicShimmerState();
}

class _IslamicShimmerState extends State<IslamicShimmer>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _animation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1500),
    )..repeat();

    _animation = Tween<double>(begin: -1.0, end: 2.0).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _animation,
      builder: (context, child) {
        return Container(
          width: widget.width,
          height: widget.height,
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(widget.borderRadius),
            gradient: LinearGradient(
              begin: Alignment.centerLeft,
              end: Alignment.centerRight,
              colors: [
                AppColors.backgroundSecondary,
                AppColors.backgroundPaper,
                AppColors.backgroundSecondary,
              ],
              stops: [
                _animation.value - 0.3,
                _animation.value,
                _animation.value + 0.3,
              ],
            ),
          ),
        );
      },
    );
  }
}

/// Islamic-themed pulsing loading indicator
class IslamicPulsingIndicator extends StatefulWidget {
  final double size;
  final Color? color;

  const IslamicPulsingIndicator({
    Key? key,
    this.size = 60,
    this.color,
  }) : super(key: key);

  @override
  State<IslamicPulsingIndicator> createState() =>
      _IslamicPulsingIndicatorState();
}

class _IslamicPulsingIndicatorState extends State<IslamicPulsingIndicator>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _animation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1000),
    )..repeat(reverse: true);

    _animation = Tween<double>(begin: 0.8, end: 1.2).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _animation,
      builder: (context, child) {
        return Transform.scale(
          scale: _animation.value,
          child: Container(
            width: widget.size,
            height: widget.size,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              color: (widget.color ?? AppColors.primaryMain).withOpacity(0.3),
            ),
            child: Center(
              child: Container(
                width: widget.size * 0.6,
                height: widget.size * 0.6,
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  color: widget.color ?? AppColors.primaryMain,
                ),
              ),
            ),
          ),
        );
      },
    );
  }
}

/// Full-screen Islamic loading overlay
class IslamicLoadingOverlay extends StatelessWidget {
  final String? message;

  const IslamicLoadingOverlay({
    Key? key,
    this.message,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.black.withOpacity(0.5),
      child: Center(
        child: Container(
          padding: const EdgeInsets.all(32),
          decoration: BoxDecoration(
            color: AppColors.backgroundPaper,
            borderRadius: BorderRadius.circular(16),
            boxShadow: [
              BoxShadow(
                color: Colors.black.withOpacity(0.2),
                blurRadius: 20,
                offset: const Offset(0, 10),
              ),
            ],
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const IslamicPulsingIndicator(),
              if (message != null) ...[
                const SizedBox(height: 24),
                Text(
                  message!,
                  style: const TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.w500,
                    fontFamily: 'Tajawal',
                    color: AppColors.textPrimary,
                  ),
                  textAlign: TextAlign.center,
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }
}

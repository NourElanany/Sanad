import 'package:flutter/material.dart';
import '../theme/app_colors.dart';

/// Islamic-themed card component with customizable elevation and styling
class IslamicCard extends StatelessWidget {
  final Widget child;
  final EdgeInsetsGeometry? padding;
  final VoidCallback? onTap;
  final bool elevated;
  final Color? backgroundColor;
  final double? borderRadius;
  final Border? border;

  const IslamicCard({
    Key? key,
    required this.child,
    this.padding,
    this.onTap,
    this.elevated = true,
    this.backgroundColor,
    this.borderRadius,
    this.border,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: backgroundColor ?? AppColors.backgroundPaper,
        borderRadius: BorderRadius.circular(borderRadius ?? 16),
        border: border ??
            Border.all(
              color: AppColors.primaryMain.withOpacity(0.1),
              width: 1,
            ),
        boxShadow: elevated
            ? [
                BoxShadow(
                  color: AppColors.primaryMain.withOpacity(0.08),
                  blurRadius: 16,
                  offset: const Offset(0, 4),
                ),
              ]
            : null,
      ),
      child: Material(
        color: Colors.transparent,
        child: InkWell(
          onTap: onTap,
          borderRadius: BorderRadius.circular(borderRadius ?? 16),
          child: Padding(
            padding: padding ?? const EdgeInsets.all(20),
            child: child,
          ),
        ),
      ),
    );
  }
}

/// Islamic-themed card with header section
class IslamicCardWithHeader extends StatelessWidget {
  final String title;
  final Widget child;
  final IconData? icon;
  final Widget? trailing;
  final EdgeInsetsGeometry? padding;
  final VoidCallback? onTap;
  final bool elevated;

  const IslamicCardWithHeader({
    Key? key,
    required this.title,
    required this.child,
    this.icon,
    this.trailing,
    this.padding,
    this.onTap,
    this.elevated = true,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      padding: EdgeInsets.zero,
      onTap: onTap,
      elevated: elevated,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Header
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: AppColors.primaryMain.withOpacity(0.05),
              borderRadius: const BorderRadius.only(
                topLeft: Radius.circular(16),
                topRight: Radius.circular(16),
              ),
            ),
            child: Row(
              children: [
                if (icon != null) ...[
                  Icon(
                    icon,
                    color: AppColors.accentGold,
                    size: 24,
                  ),
                  const SizedBox(width: 12),
                ],
                Expanded(
                  child: Text(
                    title,
                    style: const TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.w600,
                      fontFamily: 'Tajawal',
                      color: AppColors.textPrimary,
                    ),
                  ),
                ),
                if (trailing != null) trailing!,
              ],
            ),
          ),
          // Content
          Padding(
            padding: padding ?? const EdgeInsets.all(20),
            child: child,
          ),
        ],
      ),
    );
  }
}

/// Islamic-themed gradient card
class IslamicGradientCard extends StatelessWidget {
  final Widget child;
  final EdgeInsetsGeometry? padding;
  final VoidCallback? onTap;
  final List<Color>? gradientColors;
  final double? borderRadius;

  const IslamicGradientCard({
    Key? key,
    required this.child,
    this.padding,
    this.onTap,
    this.gradientColors,
    this.borderRadius,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: gradientColors ??
              [
                AppColors.primaryMain,
                AppColors.primaryLight,
              ],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
        borderRadius: BorderRadius.circular(borderRadius ?? 16),
        boxShadow: [
          BoxShadow(
            color: AppColors.primaryMain.withOpacity(0.3),
            blurRadius: 16,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Material(
        color: Colors.transparent,
        child: InkWell(
          onTap: onTap,
          borderRadius: BorderRadius.circular(borderRadius ?? 16),
          child: Padding(
            padding: padding ?? const EdgeInsets.all(20),
            child: DefaultTextStyle(
              style: const TextStyle(
                color: Colors.white,
                fontFamily: 'Tajawal',
              ),
              child: child,
            ),
          ),
        ),
      ),
    );
  }
}

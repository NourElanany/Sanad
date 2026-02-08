import 'package:flutter/material.dart';
import '../theme/app_colors.dart';

/// Types of Islamic-themed buttons
enum IslamicButtonType {
  primary,
  secondary,
  outlined,
  text,
  gradient,
}

/// Islamic-themed button component with customizable styles
class IslamicButton extends StatelessWidget {
  final String text;
  final VoidCallback? onPressed;
  final IslamicButtonType type;
  final IconData? icon;
  final bool isLoading;
  final double? width;
  final double? height;
  final EdgeInsetsGeometry? padding;

  const IslamicButton({
    Key? key,
    required this.text,
    this.onPressed,
    this.type = IslamicButtonType.primary,
    this.icon,
    this.isLoading = false,
    this.width,
    this.height,
    this.padding,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    if (type == IslamicButtonType.gradient) {
      return _buildGradientButton(context);
    }

    return SizedBox(
      width: width,
      height: height ?? 56,
      child: _buildButton(context),
    );
  }

  Widget _buildButton(BuildContext context) {
    switch (type) {
      case IslamicButtonType.primary:
        return ElevatedButton(
          onPressed: isLoading ? null : onPressed,
          style: ElevatedButton.styleFrom(
            backgroundColor: AppColors.primaryMain,
            foregroundColor: Colors.white,
            elevation: 4,
            shadowColor: AppColors.primaryMain.withOpacity(0.3),
            padding: padding ?? const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(12),
            ),
          ),
          child: _buildButtonContent(),
        );

      case IslamicButtonType.secondary:
        return ElevatedButton(
          onPressed: isLoading ? null : onPressed,
          style: ElevatedButton.styleFrom(
            backgroundColor: AppColors.secondaryMain,
            foregroundColor: Colors.white,
            elevation: 4,
            shadowColor: AppColors.secondaryMain.withOpacity(0.3),
            padding: padding ?? const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(12),
            ),
          ),
          child: _buildButtonContent(),
        );

      case IslamicButtonType.outlined:
        return OutlinedButton(
          onPressed: isLoading ? null : onPressed,
          style: OutlinedButton.styleFrom(
            foregroundColor: AppColors.primaryMain,
            side: BorderSide(color: AppColors.primaryMain, width: 1.5),
            padding: padding ?? const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(12),
            ),
          ),
          child: _buildButtonContent(),
        );

      case IslamicButtonType.text:
        return TextButton(
          onPressed: isLoading ? null : onPressed,
          style: TextButton.styleFrom(
            foregroundColor: AppColors.primaryMain,
            padding: padding ?? const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
          ),
          child: _buildButtonContent(),
        );

      default:
        return const SizedBox.shrink();
    }
  }

  Widget _buildGradientButton(BuildContext context) {
    return Container(
      width: width,
      height: height ?? 56,
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: [
            AppColors.primaryMain,
            AppColors.primaryLight,
          ],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
        borderRadius: BorderRadius.circular(12),
        boxShadow: [
          BoxShadow(
            color: AppColors.primaryMain.withOpacity(0.3),
            blurRadius: 8,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Material(
        color: Colors.transparent,
        child: InkWell(
          onTap: isLoading ? null : onPressed,
          borderRadius: BorderRadius.circular(12),
          child: Padding(
            padding: padding ?? const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
            child: _buildButtonContent(forceWhite: true),
          ),
        ),
      ),
    );
  }

  Widget _buildButtonContent({bool forceWhite = false}) {
    if (isLoading) {
      return SizedBox(
        height: 20,
        width: 20,
        child: CircularProgressIndicator(
          strokeWidth: 2,
          valueColor: AlwaysStoppedAnimation<Color>(
            forceWhite ? Colors.white : AppColors.primaryMain,
          ),
        ),
      );
    }

    return Row(
      mainAxisSize: MainAxisSize.min,
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        if (icon != null) ...[
          Icon(
            icon,
            size: 20,
            color: forceWhite ? Colors.white : null,
          ),
          const SizedBox(width: 8),
        ],
        Text(
          text,
          style: TextStyle(
            fontSize: 16,
            fontWeight: FontWeight.w600,
            fontFamily: 'Tajawal',
            color: forceWhite ? Colors.white : null,
          ),
        ),
      ],
    );
  }
}

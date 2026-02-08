import 'package:flutter/material.dart';
import '../theme/app_colors.dart';
import 'islamic_button.dart';

/// Islamic-themed modal dialog
class IslamicModal {
  /// Show a basic dialog
  static Future<T?> showDialog<T>({
    required BuildContext context,
    required String title,
    required Widget content,
    List<IslamicModalAction>? actions,
    bool barrierDismissible = true,
  }) {
    return showGeneralDialog<T>(
      context: context,
      barrierDismissible: barrierDismissible,
      barrierLabel: MaterialLocalizations.of(context).modalBarrierDismissLabel,
      barrierColor: Colors.black54,
      transitionDuration: const Duration(milliseconds: 300),
      pageBuilder: (context, animation, secondaryAnimation) {
        return Center(
          child: Material(
            color: Colors.transparent,
            child: Container(
              margin: const EdgeInsets.symmetric(horizontal: 24),
              padding: const EdgeInsets.all(24),
              decoration: BoxDecoration(
                color: AppColors.backgroundPaper,
                borderRadius: BorderRadius.circular(20),
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
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  // Title
                  Text(
                    title,
                    style: const TextStyle(
                      fontSize: 20,
                      fontWeight: FontWeight.w600,
                      fontFamily: 'Tajawal',
                      color: AppColors.textPrimary,
                    ),
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 16),
                  // Content
                  content,
                  // Actions
                  if (actions != null && actions.isNotEmpty) ...[
                    const SizedBox(height: 24),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.end,
                      children: actions
                          .map((action) => Padding(
                                padding: const EdgeInsets.only(left: 8),
                                child: IslamicButton(
                                  text: action.label,
                                  type: action.isPrimary
                                      ? IslamicButtonType.primary
                                      : IslamicButtonType.text,
                                  onPressed: () {
                                    if (action.onPressed != null) {
                                      action.onPressed!();
                                    }
                                    if (action.dismissOnPress) {
                                      Navigator.of(context).pop();
                                    }
                                  },
                                ),
                              ))
                          .toList(),
                    ),
                  ],
                ],
              ),
            ),
          ),
        );
      },
      transitionBuilder: (context, animation, secondaryAnimation, child) {
        return ScaleTransition(
          scale: CurvedAnimation(
            parent: animation,
            curve: Curves.easeOutBack,
          ),
          child: FadeTransition(
            opacity: animation,
            child: child,
          ),
        );
      },
    );
  }

  /// Show a confirmation dialog
  static Future<bool?> showConfirmation({
    required BuildContext context,
    required String title,
    required String message,
    String confirmText = 'تأكيد',
    String cancelText = 'إلغاء',
    bool isDangerous = false,
  }) {
    return showDialog<bool>(
      context: context,
      title: title,
      content: Text(
        message,
        style: const TextStyle(
          fontSize: 16,
          fontFamily: 'Tajawal',
          color: AppColors.textSecondary,
        ),
        textAlign: TextAlign.center,
      ),
      actions: [
        IslamicModalAction(
          label: cancelText,
          isPrimary: false,
          dismissOnPress: true,
          onPressed: () => Navigator.of(context).pop(false),
        ),
        IslamicModalAction(
          label: confirmText,
          isPrimary: true,
          dismissOnPress: true,
          onPressed: () => Navigator.of(context).pop(true),
        ),
      ],
    );
  }

  /// Show a success dialog
  static Future<void> showSuccess({
    required BuildContext context,
    required String title,
    required String message,
    String buttonText = 'حسناً',
  }) {
    return showDialog(
      context: context,
      title: title,
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: 80,
            height: 80,
            decoration: BoxDecoration(
              color: AppColors.statusSuccess.withOpacity(0.1),
              shape: BoxShape.circle,
            ),
            child: const Icon(
              Icons.check_circle,
              color: AppColors.statusSuccess,
              size: 48,
            ),
          ),
          const SizedBox(height: 16),
          Text(
            message,
            style: const TextStyle(
              fontSize: 16,
              fontFamily: 'Tajawal',
              color: AppColors.textSecondary,
            ),
            textAlign: TextAlign.center,
          ),
        ],
      ),
      actions: [
        IslamicModalAction(
          label: buttonText,
          isPrimary: true,
          dismissOnPress: true,
        ),
      ],
    );
  }

  /// Show an error dialog
  static Future<void> showError({
    required BuildContext context,
    required String title,
    required String message,
    String buttonText = 'حسناً',
  }) {
    return showDialog(
      context: context,
      title: title,
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(
            width: 80,
            height: 80,
            decoration: BoxDecoration(
              color: AppColors.statusError.withOpacity(0.1),
              shape: BoxShape.circle,
            ),
            child: const Icon(
              Icons.error,
              color: AppColors.statusError,
              size: 48,
            ),
          ),
          const SizedBox(height: 16),
          Text(
            message,
            style: const TextStyle(
              fontSize: 16,
              fontFamily: 'Tajawal',
              color: AppColors.textSecondary,
            ),
            textAlign: TextAlign.center,
          ),
        ],
      ),
      actions: [
        IslamicModalAction(
          label: buttonText,
          isPrimary: true,
          dismissOnPress: true,
        ),
      ],
    );
  }
}

/// Modal action model
class IslamicModalAction {
  final String label;
  final VoidCallback? onPressed;
  final bool isPrimary;
  final bool dismissOnPress;

  const IslamicModalAction({
    required this.label,
    this.onPressed,
    this.isPrimary = false,
    this.dismissOnPress = false,
  });
}

/// Islamic-themed bottom sheet
class IslamicBottomSheet {
  /// Show a basic bottom sheet
  static Future<T?> show<T>({
    required BuildContext context,
    required Widget child,
    String? title,
    bool isDismissible = true,
    bool enableDrag = true,
  }) {
    return showModalBottomSheet<T>(
      context: context,
      isDismissible: isDismissible,
      enableDrag: enableDrag,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (context) {
        return Container(
          decoration: const BoxDecoration(
            color: AppColors.backgroundPaper,
            borderRadius: BorderRadius.only(
              topLeft: Radius.circular(24),
              topRight: Radius.circular(24),
            ),
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              // Handle
              Container(
                margin: const EdgeInsets.only(top: 12),
                width: 40,
                height: 4,
                decoration: BoxDecoration(
                  color: AppColors.textDisabled,
                  borderRadius: BorderRadius.circular(2),
                ),
              ),
              // Title
              if (title != null) ...[
                Padding(
                  padding: const EdgeInsets.all(20),
                  child: Text(
                    title,
                    style: const TextStyle(
                      fontSize: 20,
                      fontWeight: FontWeight.w600,
                      fontFamily: 'Tajawal',
                      color: AppColors.textPrimary,
                    ),
                  ),
                ),
                Divider(
                  color: AppColors.primaryMain.withOpacity(0.1),
                  thickness: 1,
                  height: 1,
                ),
              ],
              // Content
              Flexible(
                child: SingleChildScrollView(
                  padding: const EdgeInsets.all(20),
                  child: child,
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  /// Show a list bottom sheet
  static Future<T?> showList<T>({
    required BuildContext context,
    required String title,
    required List<IslamicBottomSheetItem<T>> items,
  }) {
    return show<T>(
      context: context,
      title: title,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: items
            .map((item) => ListTile(
                  leading: item.icon != null
                      ? Icon(
                          item.icon,
                          color: AppColors.primaryMain,
                        )
                      : null,
                  title: Text(
                    item.title,
                    style: const TextStyle(
                      fontSize: 16,
                      fontFamily: 'Tajawal',
                      color: AppColors.textPrimary,
                    ),
                  ),
                  subtitle: item.subtitle != null
                      ? Text(
                          item.subtitle!,
                          style: const TextStyle(
                            fontSize: 14,
                            fontFamily: 'Tajawal',
                            color: AppColors.textSecondary,
                          ),
                        )
                      : null,
                  onTap: () {
                    Navigator.of(context).pop(item.value);
                  },
                ))
            .toList(),
      ),
    );
  }
}

/// Bottom sheet item model
class IslamicBottomSheetItem<T> {
  final String title;
  final String? subtitle;
  final IconData? icon;
  final T value;

  const IslamicBottomSheetItem({
    required this.title,
    this.subtitle,
    this.icon,
    required this.value,
  });
}

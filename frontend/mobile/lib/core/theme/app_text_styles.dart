import 'package:flutter/material.dart';

import 'app_colors.dart';

/// Typography styles for the application
class AppTextStyles {
  AppTextStyles._();
  
  // Regular text font family
  static const String regularFontFamily = 'Tajawal';
  
  // Quranic text font family
  static const String quranicFontFamily = 'KFGQPC';
  
  // Heading styles
  static const TextStyle h1 = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 48,
    fontWeight: FontWeight.w700,
    color: AppColors.textPrimary,
    height: 1.2,
  );
  
  static const TextStyle h2 = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 40,
    fontWeight: FontWeight.w700,
    color: AppColors.textPrimary,
    height: 1.2,
  );
  
  static const TextStyle h3 = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 36,
    fontWeight: FontWeight.w600,
    color: AppColors.textPrimary,
    height: 1.3,
  );
  
  static const TextStyle h4 = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 32,
    fontWeight: FontWeight.w600,
    color: AppColors.textPrimary,
    height: 1.3,
  );
  
  static const TextStyle h5 = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 28,
    fontWeight: FontWeight.w600,
    color: AppColors.textPrimary,
    height: 1.4,
  );
  
  static const TextStyle h6 = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 24,
    fontWeight: FontWeight.w600,
    color: AppColors.textPrimary,
    height: 1.4,
  );
  
  // Subtitle styles
  static const TextStyle subtitle1 = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 20,
    fontWeight: FontWeight.w500,
    color: AppColors.textPrimary,
    height: 1.5,
  );
  
  static const TextStyle subtitle2 = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 18,
    fontWeight: FontWeight.w500,
    color: AppColors.textSecondary,
    height: 1.5,
  );
  
  // Body text styles
  static const TextStyle body1 = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 16,
    fontWeight: FontWeight.w400,
    color: AppColors.textPrimary,
    height: 1.6,
  );
  
  static const TextStyle body2 = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 14,
    fontWeight: FontWeight.w400,
    color: AppColors.textSecondary,
    height: 1.6,
  );
  
  // Caption style
  static const TextStyle caption = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 12,
    fontWeight: FontWeight.w400,
    color: AppColors.textSecondary,
    height: 1.4,
  );
  
  // Quranic text styles
  static const TextStyle quranicSmall = TextStyle(
    fontFamily: quranicFontFamily,
    fontSize: 18,
    fontWeight: FontWeight.w400,
    color: AppColors.textQuranic,
    height: 2.0,
  );
  
  static const TextStyle quranicMedium = TextStyle(
    fontFamily: quranicFontFamily,
    fontSize: 24,
    fontWeight: FontWeight.w400,
    color: AppColors.textQuranic,
    height: 2.0,
  );
  
  static const TextStyle quranicLarge = TextStyle(
    fontFamily: quranicFontFamily,
    fontSize: 32,
    fontWeight: FontWeight.w400,
    color: AppColors.textQuranic,
    height: 2.2,
  );
  
  static const TextStyle quranicXLarge = TextStyle(
    fontFamily: quranicFontFamily,
    fontSize: 40,
    fontWeight: FontWeight.w400,
    color: AppColors.textQuranic,
    height: 2.2,
  );
  
  // Button text style
  static const TextStyle button = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 16,
    fontWeight: FontWeight.w600,
    color: Colors.white,
    height: 1.2,
  );
  
  // Link text style
  static const TextStyle link = TextStyle(
    fontFamily: regularFontFamily,
    fontSize: 16,
    fontWeight: FontWeight.w500,
    color: AppColors.primaryMain,
    decoration: TextDecoration.underline,
    height: 1.5,
  );
}

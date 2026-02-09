import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'app_text_styles.dart';

/// High contrast theme for accessibility
class HighContrastTheme {
  HighContrastTheme._();

  // High contrast colors
  static const Color _primaryColor = Color(0xFF000000); // Pure black
  static const Color _secondaryColor = Color(0xFFFFFFFF); // Pure white
  static const Color _accentColor = Color(0xFFFFD700); // Bright gold
  static const Color _errorColor = Color(0xFFFF0000); // Pure red
  static const Color _successColor = Color(0xFF00FF00); // Pure green
  static const Color _warningColor = Color(0xFFFFFF00); // Pure yellow
  static const Color _infoColor = Color(0xFF00FFFF); // Pure cyan

  /// High contrast light theme
  static ThemeData get lightTheme {
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.light,
      
      // Color scheme with maximum contrast
      colorScheme: const ColorScheme.light(
        primary: _primaryColor,
        onPrimary: _secondaryColor,
        primaryContainer: _primaryColor,
        secondary: _primaryColor,
        onSecondary: _secondaryColor,
        secondaryContainer: _primaryColor,
        tertiary: _accentColor,
        error: _errorColor,
        background: _secondaryColor,
        onBackground: _primaryColor,
        surface: _secondaryColor,
        onSurface: _primaryColor,
      ),
      
      // Scaffold
      scaffoldBackgroundColor: _secondaryColor,
      
      // App bar with high contrast
      appBarTheme: AppBarTheme(
        backgroundColor: _primaryColor,
        foregroundColor: _secondaryColor,
        elevation: 4,
        centerTitle: true,
        systemOverlayStyle: SystemUiOverlayStyle.light,
        titleTextStyle: AppTextStyles.h6.copyWith(
          color: _secondaryColor,
          fontWeight: FontWeight.w700,
        ),
        iconTheme: const IconThemeData(
          color: _secondaryColor,
          size: 28,
        ),
      ),
      
      // Card with strong borders
      cardTheme: CardThemeData(
        color: _secondaryColor,
        elevation: 4,
        shadowColor: _primaryColor.withOpacity(0.3),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(
            color: _primaryColor,
            width: 3,
          ),
        ),
        margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      ),
      
      // Elevated button with high contrast
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: _primaryColor,
          foregroundColor: _secondaryColor,
          elevation: 6,
          shadowColor: _primaryColor.withOpacity(0.5),
          padding: const EdgeInsets.symmetric(horizontal: 28, vertical: 18),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
            side: const BorderSide(
              color: _primaryColor,
              width: 2,
            ),
          ),
          textStyle: AppTextStyles.body1.copyWith(
            fontWeight: FontWeight.w700,
            fontSize: 18,
          ),
        ),
      ),
      
      // Text button
      textButtonTheme: TextButtonThemeData(
        style: TextButton.styleFrom(
          foregroundColor: _primaryColor,
          padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 14),
          textStyle: AppTextStyles.body1.copyWith(
            fontWeight: FontWeight.w700,
            fontSize: 18,
          ),
        ),
      ),
      
      // Outlined button
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          foregroundColor: _primaryColor,
          side: const BorderSide(color: _primaryColor, width: 3),
          padding: const EdgeInsets.symmetric(horizontal: 28, vertical: 18),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
          textStyle: AppTextStyles.body1.copyWith(
            fontWeight: FontWeight.w700,
            fontSize: 18,
          ),
        ),
      ),
      
      // Input decoration with strong borders
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: _secondaryColor,
        contentPadding: const EdgeInsets.symmetric(horizontal: 20, vertical: 18),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: _primaryColor, width: 3),
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: _primaryColor, width: 3),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: _accentColor, width: 4),
        ),
        errorBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
          borderSide: const BorderSide(color: _errorColor, width: 3),
        ),
        labelStyle: AppTextStyles.body1.copyWith(
          color: _primaryColor,
          fontWeight: FontWeight.w600,
        ),
        hintStyle: AppTextStyles.body1.copyWith(
          color: _primaryColor.withOpacity(0.6),
          fontWeight: FontWeight.w600,
        ),
      ),
      
      // Icon theme with larger size
      iconTheme: const IconThemeData(
        color: _primaryColor,
        size: 28,
      ),
      
      // Divider with strong contrast
      dividerTheme: const DividerThemeData(
        color: _primaryColor,
        thickness: 2,
        space: 2,
      ),
      
      // Bottom navigation bar
      bottomNavigationBarTheme: BottomNavigationBarThemeData(
        backgroundColor: _secondaryColor,
        selectedItemColor: _primaryColor,
        unselectedItemColor: _primaryColor.withOpacity(0.6),
        selectedLabelStyle: AppTextStyles.caption.copyWith(
          fontWeight: FontWeight.w700,
          fontSize: 14,
        ),
        unselectedLabelStyle: AppTextStyles.caption.copyWith(
          fontSize: 14,
        ),
        type: BottomNavigationBarType.fixed,
        elevation: 8,
      ),
      
      // Text theme with bold weights
      textTheme: TextTheme(
        displayLarge: AppTextStyles.h1.copyWith(fontWeight: FontWeight.w700),
        displayMedium: AppTextStyles.h2.copyWith(fontWeight: FontWeight.w700),
        displaySmall: AppTextStyles.h3.copyWith(fontWeight: FontWeight.w700),
        headlineMedium: AppTextStyles.h4.copyWith(fontWeight: FontWeight.w700),
        headlineSmall: AppTextStyles.h5.copyWith(fontWeight: FontWeight.w700),
        titleLarge: AppTextStyles.h6.copyWith(fontWeight: FontWeight.w700),
        titleMedium: AppTextStyles.subtitle1.copyWith(fontWeight: FontWeight.w600),
        titleSmall: AppTextStyles.subtitle2.copyWith(fontWeight: FontWeight.w600),
        bodyLarge: AppTextStyles.body1.copyWith(fontWeight: FontWeight.w600),
        bodyMedium: AppTextStyles.body2.copyWith(fontWeight: FontWeight.w600),
        bodySmall: AppTextStyles.caption.copyWith(fontWeight: FontWeight.w600),
      ),
      
      // Font family
      fontFamily: 'Tajawal',
    );
  }

  /// High contrast dark theme
  static ThemeData get darkTheme {
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.dark,
      
      // Color scheme with maximum contrast (inverted)
      colorScheme: const ColorScheme.dark(
        primary: _secondaryColor,
        onPrimary: _primaryColor,
        primaryContainer: _secondaryColor,
        secondary: _secondaryColor,
        onSecondary: _primaryColor,
        secondaryContainer: _secondaryColor,
        tertiary: _accentColor,
        error: _errorColor,
        background: _primaryColor,
        onBackground: _secondaryColor,
        surface: _primaryColor,
        onSurface: _secondaryColor,
      ),
      
      // Scaffold
      scaffoldBackgroundColor: _primaryColor,
      
      // App bar
      appBarTheme: AppBarTheme(
        backgroundColor: _primaryColor,
        foregroundColor: _secondaryColor,
        elevation: 4,
        centerTitle: true,
        systemOverlayStyle: SystemUiOverlayStyle.light,
        titleTextStyle: AppTextStyles.h6.copyWith(
          color: _secondaryColor,
          fontWeight: FontWeight.w700,
        ),
        iconTheme: const IconThemeData(
          color: _secondaryColor,
          size: 28,
        ),
      ),
      
      // Font family
      fontFamily: 'Tajawal',
    );
  }
}

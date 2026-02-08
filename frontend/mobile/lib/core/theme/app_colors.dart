import 'package:flutter/material.dart';

/// Islamic-themed color palette
class AppColors {
  AppColors._();
  
  // Primary colors (Deep Navy)
  static const Color primaryMain = Color(0xFF1B365D);
  static const Color primaryLight = Color(0xFF2E4A6B);
  static const Color primaryDark = Color(0xFF0F1F35);
  
  // Secondary colors (Emerald Green)
  static const Color secondaryMain = Color(0xFF2D5A27);
  static const Color secondaryLight = Color(0xFF4A7C59);
  static const Color secondaryDark = Color(0xFF1A3318);
  
  // Accent colors (Muted Gold)
  static const Color accentGold = Color(0xFFB8860B);
  static const Color accentLightGold = Color(0xFFDAA520);
  
  // Background colors
  static const Color backgroundPrimary = Color(0xFFFEFEFE);
  static const Color backgroundSecondary = Color(0xFFF8F9FA);
  static const Color backgroundPaper = Color(0xFFFFFFFF);
  
  // Text colors
  static const Color textPrimary = Color(0xFF1A1A1A);
  static const Color textSecondary = Color(0xFF666666);
  static const Color textDisabled = Color(0xFFCCCCCC);
  static const Color textQuranic = Color(0xFF0F1F35);
  
  // Status colors
  static const Color statusSuccess = Color(0xFF28A745);
  static const Color statusWarning = Color(0xFFFFC107);
  static const Color statusError = Color(0xFFDC3545);
  static const Color statusInfo = Color(0xFF17A2B8);
  
  // Hadith authenticity colors
  static const Color hadithSahih = Color(0xFF28A745); // Green for Sahih
  static const Color hadithHasan = Color(0xFFFFC107); // Yellow for Hasan
  static const Color hadithDaif = Color(0xFFDC3545); // Red for Daif
  
  // Prayer time colors
  static const Color prayerFajr = Color(0xFF4A90E2);
  static const Color prayerDhuhr = Color(0xFFF5A623);
  static const Color prayerAsr = Color(0xFFE67E22);
  static const Color prayerMaghrib = Color(0xFF9B59B6);
  static const Color prayerIsha = Color(0xFF34495E);
  
  // Gradient colors
  static const LinearGradient primaryGradient = LinearGradient(
    colors: [primaryMain, primaryLight],
    begin: Alignment.topLeft,
    end: Alignment.bottomRight,
  );
  
  static const LinearGradient secondaryGradient = LinearGradient(
    colors: [secondaryMain, secondaryLight],
    begin: Alignment.topLeft,
    end: Alignment.bottomRight,
  );
  
  static const LinearGradient goldGradient = LinearGradient(
    colors: [accentGold, accentLightGold],
    begin: Alignment.topLeft,
    end: Alignment.bottomRight,
  );
  
  // Shimmer colors for loading states
  static const Color shimmerBase = Color(0xFFE0E0E0);
  static const Color shimmerHighlight = Color(0xFFF5F5F5);
}

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// Service for managing accessibility features
class AccessibilityService {
  static const String _keyScreenReaderEnabled = 'accessibility_screen_reader';
  static const String _keyHighContrastEnabled = 'accessibility_high_contrast';
  static const String _keyVoiceNavigationEnabled = 'accessibility_voice_navigation';
  static const String _keyTextScaleFactor = 'accessibility_text_scale';
  static const String _keyReduceAnimations = 'accessibility_reduce_animations';
  static const String _keyKeyboardShortcutsEnabled = 'accessibility_keyboard_shortcuts';

  final SharedPreferences _prefs;

  AccessibilityService(this._prefs);

  // Screen Reader Support
  bool get isScreenReaderEnabled => _prefs.getBool(_keyScreenReaderEnabled) ?? false;
  
  Future<void> setScreenReaderEnabled(bool enabled) async {
    await _prefs.setBool(_keyScreenReaderEnabled, enabled);
    if (enabled) {
      await _announceScreenReaderEnabled();
    }
  }

  Future<void> _announceScreenReaderEnabled() async {
    await SystemChannels.accessibility.invokeMethod(
      'announce',
      'تم تفعيل قارئ الشاشة',
    );
  }

  // High Contrast Mode
  bool get isHighContrastEnabled => _prefs.getBool(_keyHighContrastEnabled) ?? false;
  
  Future<void> setHighContrastEnabled(bool enabled) async {
    await _prefs.setBool(_keyHighContrastEnabled, enabled);
  }

  // Voice Navigation
  bool get isVoiceNavigationEnabled => _prefs.getBool(_keyVoiceNavigationEnabled) ?? false;
  
  Future<void> setVoiceNavigationEnabled(bool enabled) async {
    await _prefs.setBool(_keyVoiceNavigationEnabled, enabled);
  }

  // Text Scaling
  double get textScaleFactor => _prefs.getDouble(_keyTextScaleFactor) ?? 1.0;
  
  Future<void> setTextScaleFactor(double factor) async {
    // Clamp between 0.8 and 2.0
    final clampedFactor = factor.clamp(0.8, 2.0);
    await _prefs.setDouble(_keyTextScaleFactor, clampedFactor);
  }

  // Reduce Animations
  bool get shouldReduceAnimations => _prefs.getBool(_keyReduceAnimations) ?? false;
  
  Future<void> setReduceAnimations(bool reduce) async {
    await _prefs.setBool(_keyReduceAnimations, reduce);
  }

  // Keyboard Shortcuts
  bool get areKeyboardShortcutsEnabled => _prefs.getBool(_keyKeyboardShortcutsEnabled) ?? true;
  
  Future<void> setKeyboardShortcutsEnabled(bool enabled) async {
    await _prefs.setBool(_keyKeyboardShortcutsEnabled, enabled);
  }

  // Semantic Announcements
  Future<void> announce(String message) async {
    if (isScreenReaderEnabled) {
      await SystemChannels.accessibility.invokeMethod('announce', message);
    }
  }

  // Get recommended text scale based on system settings
  double getRecommendedTextScale(BuildContext context) {
    final mediaQuery = MediaQuery.of(context);
    final systemTextScale = mediaQuery.textScaleFactor;
    final userTextScale = textScaleFactor;
    
    // Combine system and user preferences
    return systemTextScale * userTextScale;
  }

  // Check if device has accessibility features enabled
  bool hasSystemAccessibilityEnabled(BuildContext context) {
    final mediaQuery = MediaQuery.of(context);
    return mediaQuery.accessibleNavigation || 
           mediaQuery.boldText || 
           mediaQuery.highContrast;
  }

  // Reset all accessibility settings
  Future<void> resetToDefaults() async {
    await Future.wait([
      _prefs.remove(_keyScreenReaderEnabled),
      _prefs.remove(_keyHighContrastEnabled),
      _prefs.remove(_keyVoiceNavigationEnabled),
      _prefs.remove(_keyTextScaleFactor),
      _prefs.remove(_keyReduceAnimations),
      _prefs.remove(_keyKeyboardShortcutsEnabled),
    ]);
  }
}

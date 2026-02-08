import 'package:shared_preferences/shared_preferences.dart';
import 'dart:convert';

/// Service for managing user preferences
class PreferencesService {
  static const String _keyOnboardingCompleted = 'onboarding_completed';
  static const String _keyMadhab = 'madhab';
  static const String _keyTheme = 'theme';
  static const String _keyFontSize = 'font_size';
  static const String _keyEnableAnimations = 'enable_animations';
  static const String _keyEnableNotifications = 'enable_notifications';
  static const String _keyLanguage = 'language';
  static const String _keyAllPreferences = 'all_preferences';

  final SharedPreferences _prefs;

  PreferencesService(this._prefs);

  // Onboarding
  Future<void> setOnboardingCompleted(bool completed) async {
    await _prefs.setBool(_keyOnboardingCompleted, completed);
  }

  bool getOnboardingCompleted() {
    return _prefs.getBool(_keyOnboardingCompleted) ?? false;
  }

  // Madhab
  Future<void> setMadhab(String madhab) async {
    await _prefs.setString(_keyMadhab, madhab);
  }

  String? getMadhab() {
    return _prefs.getString(_keyMadhab);
  }

  // Theme
  Future<void> setTheme(String theme) async {
    await _prefs.setString(_keyTheme, theme);
  }

  String getTheme() {
    return _prefs.getString(_keyTheme) ?? 'light';
  }

  // Font Size
  Future<void> setFontSize(String fontSize) async {
    await _prefs.setString(_keyFontSize, fontSize);
  }

  String getFontSize() {
    return _prefs.getString(_keyFontSize) ?? 'medium';
  }

  // Animations
  Future<void> setEnableAnimations(bool enable) async {
    await _prefs.setBool(_keyEnableAnimations, enable);
  }

  bool getEnableAnimations() {
    return _prefs.getBool(_keyEnableAnimations) ?? true;
  }

  // Notifications
  Future<void> setEnableNotifications(bool enable) async {
    await _prefs.setBool(_keyEnableNotifications, enable);
  }

  bool getEnableNotifications() {
    return _prefs.getBool(_keyEnableNotifications) ?? true;
  }

  // Language
  Future<void> setLanguage(String language) async {
    await _prefs.setString(_keyLanguage, language);
  }

  String getLanguage() {
    return _prefs.getString(_keyLanguage) ?? 'ar';
  }

  // Get all preferences as JSON
  Map<String, dynamic> getAllPreferences() {
    return {
      'onboarding_completed': getOnboardingCompleted(),
      'madhab': getMadhab(),
      'theme': getTheme(),
      'font_size': getFontSize(),
      'enable_animations': getEnableAnimations(),
      'enable_notifications': getEnableNotifications(),
      'language': getLanguage(),
    };
  }

  // Save all preferences from JSON
  Future<void> setAllPreferences(Map<String, dynamic> preferences) async {
    if (preferences.containsKey('onboarding_completed')) {
      await setOnboardingCompleted(preferences['onboarding_completed']);
    }
    if (preferences.containsKey('madhab')) {
      await setMadhab(preferences['madhab']);
    }
    if (preferences.containsKey('theme')) {
      await setTheme(preferences['theme']);
    }
    if (preferences.containsKey('font_size')) {
      await setFontSize(preferences['font_size']);
    }
    if (preferences.containsKey('enable_animations')) {
      await setEnableAnimations(preferences['enable_animations']);
    }
    if (preferences.containsKey('enable_notifications')) {
      await setEnableNotifications(preferences['enable_notifications']);
    }
    if (preferences.containsKey('language')) {
      await setLanguage(preferences['language']);
    }
  }

  // Backup preferences to JSON string
  String backupPreferences() {
    final prefs = getAllPreferences();
    return jsonEncode(prefs);
  }

  // Restore preferences from JSON string
  Future<void> restorePreferences(String jsonString) async {
    try {
      final Map<String, dynamic> prefs = jsonDecode(jsonString);
      await setAllPreferences(prefs);
    } catch (e) {
      throw Exception('Failed to restore preferences: $e');
    }
  }

  // Reset all preferences to defaults
  Future<void> resetToDefaults() async {
    await _prefs.clear();
  }

  // Sync preferences with backend (placeholder for future implementation)
  Future<void> syncWithBackend() async {
    // TODO: Implement backend sync
    // This will be implemented when backend integration is ready
    final prefs = getAllPreferences();
    // await apiService.syncPreferences(prefs);
  }
}

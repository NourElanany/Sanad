import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../services/preferences_service.dart';

/// Provider for SharedPreferences instance
final sharedPreferencesProvider = Provider<SharedPreferences>((ref) {
  throw UnimplementedError('SharedPreferences must be initialized in main()');
});

/// Provider for PreferencesService
final preferencesServiceProvider = Provider<PreferencesService>((ref) {
  final prefs = ref.watch(sharedPreferencesProvider);
  return PreferencesService(prefs);
});

/// Provider for onboarding completion status
final onboardingCompletedProvider = Provider<bool>((ref) {
  final prefsService = ref.watch(preferencesServiceProvider);
  return prefsService.getOnboardingCompleted();
});

/// Provider for madhab selection
final madhabProvider = StateProvider<String?>((ref) {
  final prefsService = ref.watch(preferencesServiceProvider);
  return prefsService.getMadhab();
});

/// Provider for theme selection
final themeProvider = StateProvider<String>((ref) {
  final prefsService = ref.watch(preferencesServiceProvider);
  return prefsService.getTheme();
});

/// Provider for font size
final fontSizeProvider = StateProvider<String>((ref) {
  final prefsService = ref.watch(preferencesServiceProvider);
  return prefsService.getFontSize();
});

/// Provider for animations setting
final animationsProvider = StateProvider<bool>((ref) {
  final prefsService = ref.watch(preferencesServiceProvider);
  return prefsService.getEnableAnimations();
});

/// Provider for notifications setting
final notificationsProvider = StateProvider<bool>((ref) {
  final prefsService = ref.watch(preferencesServiceProvider);
  return prefsService.getEnableNotifications();
});

/// Provider for language selection
final languageProvider = StateProvider<String>((ref) {
  final prefsService = ref.watch(preferencesServiceProvider);
  return prefsService.getLanguage();
});

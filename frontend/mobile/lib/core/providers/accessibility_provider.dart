import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../services/accessibility_service.dart';

/// Provider for SharedPreferences
final sharedPreferencesProvider = Provider<SharedPreferences>((ref) {
  throw UnimplementedError('SharedPreferences must be overridden');
});

/// Provider for AccessibilityService
final accessibilityServiceProvider = Provider<AccessibilityService>((ref) {
  final prefs = ref.watch(sharedPreferencesProvider);
  return AccessibilityService(prefs);
});

/// Provider for screen reader state
final screenReaderProvider = StateNotifierProvider<ScreenReaderNotifier, bool>((ref) {
  final service = ref.watch(accessibilityServiceProvider);
  return ScreenReaderNotifier(service);
});

class ScreenReaderNotifier extends StateNotifier<bool> {
  final AccessibilityService _service;

  ScreenReaderNotifier(this._service) : super(_service.isScreenReaderEnabled);

  Future<void> toggle() async {
    final newValue = !state;
    await _service.setScreenReaderEnabled(newValue);
    state = newValue;
  }

  Future<void> announce(String message) async {
    await _service.announce(message);
  }
}

/// Provider for high contrast mode state
final highContrastProvider = StateNotifierProvider<HighContrastNotifier, bool>((ref) {
  final service = ref.watch(accessibilityServiceProvider);
  return HighContrastNotifier(service);
});

class HighContrastNotifier extends StateNotifier<bool> {
  final AccessibilityService _service;

  HighContrastNotifier(this._service) : super(_service.isHighContrastEnabled);

  Future<void> toggle() async {
    final newValue = !state;
    await _service.setHighContrastEnabled(newValue);
    state = newValue;
  }
}

/// Provider for voice navigation state
final voiceNavigationProvider = StateNotifierProvider<VoiceNavigationNotifier, bool>((ref) {
  final service = ref.watch(accessibilityServiceProvider);
  return VoiceNavigationNotifier(service);
});

class VoiceNavigationNotifier extends StateNotifier<bool> {
  final AccessibilityService _service;

  VoiceNavigationNotifier(this._service) : super(_service.isVoiceNavigationEnabled);

  Future<void> toggle() async {
    final newValue = !state;
    await _service.setVoiceNavigationEnabled(newValue);
    state = newValue;
  }
}

/// Provider for text scale factor
final textScaleProvider = StateNotifierProvider<TextScaleNotifier, double>((ref) {
  final service = ref.watch(accessibilityServiceProvider);
  return TextScaleNotifier(service);
});

class TextScaleNotifier extends StateNotifier<double> {
  final AccessibilityService _service;

  TextScaleNotifier(this._service) : super(_service.textScaleFactor);

  Future<void> setScale(double scale) async {
    await _service.setTextScaleFactor(scale);
    state = scale;
  }

  Future<void> increase() async {
    final newScale = (state + 0.1).clamp(0.8, 2.0);
    await setScale(newScale);
  }

  Future<void> decrease() async {
    final newScale = (state - 0.1).clamp(0.8, 2.0);
    await setScale(newScale);
  }

  Future<void> reset() async {
    await setScale(1.0);
  }
}

/// Provider for reduce animations state
final reduceAnimationsProvider = StateNotifierProvider<ReduceAnimationsNotifier, bool>((ref) {
  final service = ref.watch(accessibilityServiceProvider);
  return ReduceAnimationsNotifier(service);
});

class ReduceAnimationsNotifier extends StateNotifier<bool> {
  final AccessibilityService _service;

  ReduceAnimationsNotifier(this._service) : super(_service.shouldReduceAnimations);

  Future<void> toggle() async {
    final newValue = !state;
    await _service.setReduceAnimations(newValue);
    state = newValue;
  }
}

/// Provider for keyboard shortcuts state
final keyboardShortcutsProvider = StateNotifierProvider<KeyboardShortcutsNotifier, bool>((ref) {
  final service = ref.watch(accessibilityServiceProvider);
  return KeyboardShortcutsNotifier(service);
});

class KeyboardShortcutsNotifier extends StateNotifier<bool> {
  final AccessibilityService _service;

  KeyboardShortcutsNotifier(this._service) : super(_service.areKeyboardShortcutsEnabled);

  Future<void> toggle() async {
    final newValue = !state;
    await _service.setKeyboardShortcutsEnabled(newValue);
    state = newValue;
  }
}

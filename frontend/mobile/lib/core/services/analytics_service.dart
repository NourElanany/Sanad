import 'package:firebase_analytics/firebase_analytics.dart';
import 'package:firebase_crashlytics/firebase_crashlytics.dart';
import 'package:flutter/foundation.dart';

/// Analytics service for tracking user behavior and app performance
class AnalyticsService {
  static final AnalyticsService _instance = AnalyticsService._internal();
  factory AnalyticsService() => _instance;
  AnalyticsService._internal();

  late FirebaseAnalytics _analytics;
  late FirebaseCrashlytics _crashlytics;

  /// Initialize analytics services
  Future<void> initialize() async {
    _analytics = FirebaseAnalytics.instance;
    _crashlytics = FirebaseCrashlytics.instance;

    // Enable analytics collection
    await _analytics.setAnalyticsCollectionEnabled(true);

    // Configure Crashlytics
    FlutterError.onError = _crashlytics.recordFlutterFatalError;
    PlatformDispatcher.instance.onError = (error, stack) {
      _crashlytics.recordError(error, stack, fatal: true);
      return true;
    };
  }

  /// Get Firebase Analytics instance
  FirebaseAnalytics get analytics => _analytics;

  /// Set user ID for analytics
  Future<void> setUserId(String userId) async {
    await _analytics.setUserId(id: userId);
    await _crashlytics.setUserIdentifier(userId);
  }

  /// Set user properties
  Future<void> setUserProperty(String name, String value) async {
    await _analytics.setUserProperty(name: name, value: value);
  }

  /// Clear user data
  Future<void> clearUser() async {
    await _analytics.setUserId(id: null);
    await _crashlytics.setUserIdentifier('');
  }

  /// Log screen view
  Future<void> logScreenView(String screenName) async {
    await _analytics.logScreenView(
      screenName: screenName,
      screenClass: screenName,
    );
  }

  /// Log custom event
  Future<void> logEvent(String name, {Map<String, dynamic>? parameters}) async {
    await _analytics.logEvent(
      name: name,
      parameters: parameters,
    );
  }

  // ==================== Islamic App Specific Events ====================

  /// Track Quran reading
  Future<void> trackQuranReading(int surahNumber, int ayahNumber) async {
    await logEvent('quran_reading', parameters: {
      'surah_number': surahNumber,
      'ayah_number': ayahNumber,
    });
  }

  /// Track prayer time view
  Future<void> trackPrayerTimeView(String prayerName) async {
    await logEvent('prayer_time_view', parameters: {
      'prayer_name': prayerName,
    });
  }

  /// Track AI assistant question
  Future<void> trackAIQuestion(String question, int questionLength) async {
    await logEvent('ai_question', parameters: {
      'question_length': questionLength,
      'has_voice_input': false,
    });
  }

  /// Track recitation analysis
  Future<void> trackRecitationAnalysis(int surahNumber, int ayahStart, int ayahEnd) async {
    await logEvent('recitation_analysis', parameters: {
      'surah_number': surahNumber,
      'ayah_start': ayahStart,
      'ayah_end': ayahEnd,
    });
  }

  /// Track search
  Future<void> trackSearch(String searchTerm, int resultCount) async {
    await logEvent('search', parameters: {
      'search_term': searchTerm,
      'result_count': resultCount,
    });
  }

  /// Track feature usage
  Future<void> trackFeatureUsage(String featureName) async {
    await logEvent('feature_usage', parameters: {
      'feature_name': featureName,
    });
  }

  /// Track Khatma progress
  Future<void> trackKhatmaProgress(int khatmaId, double progress) async {
    await logEvent('khatma_progress', parameters: {
      'khatma_id': khatmaId,
      'progress': progress,
    });
  }

  /// Track bookmark creation
  Future<void> trackBookmarkCreated(int surahNumber, int ayahNumber) async {
    await logEvent('bookmark_created', parameters: {
      'surah_number': surahNumber,
      'ayah_number': ayahNumber,
    });
  }

  /// Track Tafsir view
  Future<void> trackTafsirView(int surahNumber, int ayahNumber, String tafsirSource) async {
    await logEvent('tafsir_view', parameters: {
      'surah_number': surahNumber,
      'ayah_number': ayahNumber,
      'tafsir_source': tafsirSource,
    });
  }

  /// Track Hadith view
  Future<void> trackHadithView(String collection, int hadithNumber) async {
    await logEvent('hadith_view', parameters: {
      'collection': collection,
      'hadith_number': hadithNumber,
    });
  }

  /// Track Qibla compass usage
  Future<void> trackQiblaCompassUsage() async {
    await logEvent('qibla_compass_usage');
  }

  /// Track offline content download
  Future<void> trackOfflineDownload(String contentType, int sizeInMB) async {
    await logEvent('offline_download', parameters: {
      'content_type': contentType,
      'size_mb': sizeInMB,
    });
  }

  // ==================== Error Tracking ====================

  /// Record error
  Future<void> recordError(
    dynamic exception,
    StackTrace? stackTrace, {
    String? reason,
    bool fatal = false,
  }) async {
    await _crashlytics.recordError(
      exception,
      stackTrace,
      reason: reason,
      fatal: fatal,
    );
  }

  /// Log message
  Future<void> log(String message) async {
    await _crashlytics.log(message);
  }

  /// Set custom key
  Future<void> setCustomKey(String key, dynamic value) async {
    await _crashlytics.setCustomKey(key, value);
  }

  // ==================== Performance Monitoring ====================

  /// Start performance trace
  Future<void> startTrace(String traceName) async {
    // Implementation would use Firebase Performance Monitoring
    // This is a placeholder for the actual implementation
    debugPrint('Starting trace: $traceName');
  }

  /// Stop performance trace
  Future<void> stopTrace(String traceName) async {
    // Implementation would use Firebase Performance Monitoring
    debugPrint('Stopping trace: $traceName');
  }
}

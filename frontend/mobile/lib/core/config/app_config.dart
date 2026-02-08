import 'package:flutter/foundation.dart';

/// Application configuration based on build flavor
class AppConfig {
  static late AppEnvironment _environment;
  static late String _apiBaseUrl;
  
  /// Initialize app configuration
  static Future<void> init() async {
    // Determine environment from build flavor
    const flavor = String.fromEnvironment('FLAVOR', defaultValue: 'development');
    
    switch (flavor) {
      case 'production':
        _environment = AppEnvironment.production;
        _apiBaseUrl = 'https://api.sanad.app';
        break;
      case 'staging':
        _environment = AppEnvironment.staging;
        _apiBaseUrl = 'https://staging-api.sanad.app';
        break;
      case 'development':
      default:
        _environment = AppEnvironment.development;
        _apiBaseUrl = 'https://dev-api.sanad.app';
        break;
    }
    
    if (kDebugMode) {
      print('🚀 App initialized with environment: ${_environment.name}');
      print('🌐 API Base URL: $_apiBaseUrl');
    }
  }
  
  /// Get current environment
  static AppEnvironment get environment => _environment;
  
  /// Get API base URL
  static String get apiBaseUrl => _apiBaseUrl;
  
  /// Check if running in production
  static bool get isProduction => _environment == AppEnvironment.production;
  
  /// Check if running in development
  static bool get isDevelopment => _environment == AppEnvironment.development;
  
  /// Check if running in staging
  static bool get isStaging => _environment == AppEnvironment.staging;
  
  /// API endpoints
  static const String quranServicePath = '/api/quran';
  static const String hadithServicePath = '/api/hadith';
  static const String prayerTimesServicePath = '/api/prayer-times';
  static const String aiServicePath = '/api/ai';
  static const String audioAnalysisServicePath = '/api/audio';
  static const String searchServicePath = '/api/search';
  static const String userServicePath = '/api/user';
  static const String authServicePath = '/api/auth';
  
  /// WebSocket endpoints
  static String get wsBaseUrl => _apiBaseUrl.replaceFirst('https://', 'wss://');
  static const String aiStreamPath = '/ai/stream';
  
  /// App constants
  static const String appName = 'Sanad';
  static const String appVersion = '1.0.0';
  static const int apiTimeout = 30000; // 30 seconds
  static const int connectTimeout = 15000; // 15 seconds
  
  /// Storage keys
  static const String accessTokenKey = 'access_token';
  static const String refreshTokenKey = 'refresh_token';
  static const String userIdKey = 'user_id';
  static const String userPreferencesKey = 'user_preferences';
  static const String lastReadPositionKey = 'last_read_position';
  static const String bookmarksKey = 'bookmarks';
  static const String offlineContentKey = 'offline_content';
  
  /// Feature flags
  static const bool enableAIAssistant = true;
  static const bool enableRecitationAnalysis = true;
  static const bool enableOfflineMode = true;
  static const bool enablePushNotifications = true;
  static const bool enableAnalytics = true;
}

/// Application environment
enum AppEnvironment {
  development,
  staging,
  production,
}

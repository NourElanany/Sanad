/// Environment configuration for Flutter app
/// Manages different environments (development, staging, production)

enum Environment {
  development,
  staging,
  production,
}

class EnvironmentConfig {
  final Environment environment;
  final String apiBaseUrl;
  final String appName;
  final bool enableLogging;
  final bool enableAnalytics;
  final bool enableCrashlytics;
  final String firebaseProjectId;
  final String sentryDsn;
  
  const EnvironmentConfig({
    required this.environment,
    required this.apiBaseUrl,
    required this.appName,
    required this.enableLogging,
    required this.enableAnalytics,
    required this.enableCrashlytics,
    required this.firebaseProjectId,
    required this.sentryDsn,
  });

  /// Development environment configuration
  static const development = EnvironmentConfig(
    environment: Environment.development,
    apiBaseUrl: 'http://localhost:8080',
    appName: 'Sanad (Dev)',
    enableLogging: true,
    enableAnalytics: false,
    enableCrashlytics: false,
    firebaseProjectId: 'sanad-dev',
    sentryDsn: '',
  );

  /// Staging environment configuration
  static const staging = EnvironmentConfig(
    environment: Environment.staging,
    apiBaseUrl: 'https://staging-api.sanad.app',
    appName: 'Sanad (Staging)',
    enableLogging: true,
    enableAnalytics: true,
    enableCrashlytics: true,
    firebaseProjectId: 'sanad-staging',
    sentryDsn: 'https://xxx@xxx.ingest.sentry.io/xxx',
  );

  /// Production environment configuration
  static const production = EnvironmentConfig(
    environment: Environment.production,
    apiBaseUrl: 'https://api.sanad.app',
    appName: 'Sanad',
    enableLogging: false,
    enableAnalytics: true,
    enableCrashlytics: true,
    firebaseProjectId: 'sanad-production',
    sentryDsn: 'https://xxx@xxx.ingest.sentry.io/xxx',
  );

  /// Current environment (set during app initialization)
  static late EnvironmentConfig current;

  /// Initialize environment based on build flavor
  static void initialize(Environment env) {
    switch (env) {
      case Environment.development:
        current = development;
        break;
      case Environment.staging:
        current = staging;
        break;
      case Environment.production:
        current = production;
        break;
    }
  }

  /// Check if current environment is development
  bool get isDevelopment => environment == Environment.development;

  /// Check if current environment is staging
  bool get isStaging => environment == Environment.staging;

  /// Check if current environment is production
  bool get isProduction => environment == Environment.production;

  /// Get API endpoint URL
  String getApiEndpoint(String path) {
    return '$apiBaseUrl$path';
  }

  @override
  String toString() {
    return 'EnvironmentConfig(environment: $environment, apiBaseUrl: $apiBaseUrl)';
  }
}

/// Feature flags for enabling/disabling features per environment
class FeatureFlags {
  static bool get enableAIAssistant => true;
  static bool get enableRecitationAnalysis => true;
  static bool get enableOfflineMode => true;
  static bool get enableAdvancedSearch => true;
  static bool get enableQiblaCompass => true;
  static bool get enablePrayerNotifications => true;
  
  /// Debug features (only in development)
  static bool get enableDebugMenu => EnvironmentConfig.current.isDevelopment;
  static bool get enablePerformanceOverlay => EnvironmentConfig.current.isDevelopment;
  static bool get enableNetworkInspector => EnvironmentConfig.current.isDevelopment;
}

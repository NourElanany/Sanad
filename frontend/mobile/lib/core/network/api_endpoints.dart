import '../config/app_config.dart';

/// API endpoints configuration for all backend services
class ApiEndpoints {
  // Base paths
  static const String quranBase = AppConfig.quranServicePath;
  static const String hadithBase = AppConfig.hadithServicePath;
  static const String prayerTimesBase = AppConfig.prayerTimesServicePath;
  static const String aiBase = AppConfig.aiServicePath;
  static const String audioAnalysisBase = AppConfig.audioAnalysisServicePath;
  static const String searchBase = AppConfig.searchServicePath;
  static const String userBase = AppConfig.userServicePath;
  static const String authBase = AppConfig.authServicePath;
  
  // ============================================================================
  // Authentication Endpoints
  // ============================================================================
  
  static const String login = '$authBase/login';
  static const String register = '$authBase/register';
  static const String refreshToken = '$authBase/refresh';
  static const String logout = '$authBase/logout';
  static const String forgotPassword = '$authBase/forgot-password';
  static const String resetPassword = '$authBase/reset-password';
  static const String verifyEmail = '$authBase/verify-email';
  
  // ============================================================================
  // Quran Service Endpoints
  // ============================================================================
  
  static const String surahs = '$quranBase/surahs';
  static String surah(int surahNumber) => '$quranBase/surahs/$surahNumber';
  static String ayah(int surahNumber, int ayahNumber) => 
      '$quranBase/surahs/$surahNumber/ayahs/$ayahNumber';
  static String surahAyahs(int surahNumber) => 
      '$quranBase/surahs/$surahNumber/ayahs';
  static const String juzs = '$quranBase/juzs';
  static String juz(int juzNumber) => '$quranBase/juzs/$juzNumber';
  static const String pages = '$quranBase/pages';
  static String page(int pageNumber) => '$quranBase/pages/$pageNumber';
  
  // Tafsir endpoints
  static const String tafsirs = '$quranBase/tafsirs';
  static String tafsir(int tafsirId) => '$quranBase/tafsirs/$tafsirId';
  static String ayahTafsir(int surahNumber, int ayahNumber, int tafsirId) =>
      '$quranBase/surahs/$surahNumber/ayahs/$ayahNumber/tafsir/$tafsirId';
  static String compareTafsirs(int surahNumber, int ayahNumber) =>
      '$quranBase/surahs/$surahNumber/ayahs/$ayahNumber/tafsir/compare';
  
  // Translation endpoints
  static const String translations = '$quranBase/translations';
  static String translation(int translationId) => 
      '$quranBase/translations/$translationId';
  static String ayahTranslation(int surahNumber, int ayahNumber, int translationId) =>
      '$quranBase/surahs/$surahNumber/ayahs/$ayahNumber/translation/$translationId';
  
  // Audio endpoints
  static String surahAudio(int surahNumber, int reciterId) =>
      '$quranBase/surahs/$surahNumber/audio/$reciterId';
  static String ayahAudio(int surahNumber, int ayahNumber, int reciterId) =>
      '$quranBase/surahs/$surahNumber/ayahs/$ayahNumber/audio/$reciterId';
  static const String reciters = '$quranBase/reciters';
  
  // ============================================================================
  // Hadith Service Endpoints
  // ============================================================================
  
  static const String hadithCollections = '$hadithBase/collections';
  static String hadithCollection(String collectionId) => 
      '$hadithBase/collections/$collectionId';
  static String hadithBooks(String collectionId) => 
      '$hadithBase/collections/$collectionId/books';
  static String hadithBook(String collectionId, int bookNumber) =>
      '$hadithBase/collections/$collectionId/books/$bookNumber';
  static String hadith(String collectionId, int hadithNumber) =>
      '$hadithBase/collections/$collectionId/hadiths/$hadithNumber';
  static const String hadithSearch = '$hadithBase/search';
  static const String hadithByNarrator = '$hadithBase/narrators';
  static const String hadithByTopic = '$hadithBase/topics';
  
  // ============================================================================
  // Prayer Times Service Endpoints
  // ============================================================================
  
  static const String prayerTimes = '$prayerTimesBase/times';
  static const String prayerTimesDaily = '$prayerTimesBase/times/daily';
  static const String prayerTimesMonthly = '$prayerTimesBase/times/monthly';
  static const String monthlyPrayerTimes = '$prayerTimesBase/times/monthly';
  static const String prayerTimesYearly = '$prayerTimesBase/times/yearly';
  static const String prayerTimesRange = '$prayerTimesBase/times/range';
  static const String prayerCalendar = '$prayerTimesBase/calendar';
  static const String qiblaDirection = '$prayerTimesBase/qibla';
  static const String hijriCalendar = '$prayerTimesBase/hijri';
  static const String hijriDate = '$prayerTimesBase/hijri/today';
  static const String hijriToGregorian = '$prayerTimesBase/hijri/convert';
  static const String islamicEvents = '$prayerTimesBase/events';
  
  // ============================================================================
  // AI Service Endpoints (RAG System)
  // ============================================================================
  
  static const String aiAsk = '$aiBase/ask';
  static const String aiStream = '$aiBase/stream';
  static const String aiHistory = '$aiBase/history';
  static const String aiVerifySources = '$aiBase/verify-sources';
  static const String aiMultipleViewpoints = '$aiBase/multiple-viewpoints';
  static const String aiFeedback = '$aiBase/feedback';
  
  // ============================================================================
  // Audio Analysis Service Endpoints (Tajweed)
  // ============================================================================
  
  static const String analyzeRecitation = '$audioAnalysisBase/analyze';
  static const String recitationHistory = '$audioAnalysisBase/history';
  static const String recitationProgress = '$audioAnalysisBase/progress';
  static const String recitationComparison = '$audioAnalysisBase/compare';
  static const String tajweedRules = '$audioAnalysisBase/tajweed-rules';
  static const String improvementSuggestions = '$audioAnalysisBase/suggestions';
  
  // ============================================================================
  // Search Service Endpoints (Semantic Search)
  // ============================================================================
  
  static const String searchAll = '$searchBase/search';
  static const String searchQuran = '$searchBase/quran';
  static const String searchHadith = '$searchBase/hadith';
  static const String searchFatawa = '$searchBase/fatawa';
  static const String searchAdvanced = '$searchBase/advanced';
  static const String searchSuggestions = '$searchBase/suggestions';
  
  // ============================================================================
  // User Service Endpoints
  // ============================================================================
  
  static const String userProfile = '$userBase/profile';
  static const String userPreferences = '$userBase/preferences';
  static const String userBookmarks = '$userBase/bookmarks';
  static const String userReadingProgress = '$userBase/reading-progress';
  static const String userKhatma = '$userBase/khatma';
  static const String userStatistics = '$userBase/statistics';
  static const String userAchievements = '$userBase/achievements';
  static const String userNotifications = '$userBase/notifications';
  
  // Dashboard endpoints
  static const String dashboard = '$userBase/dashboard';
  static const String dailyWird = '$userBase/daily-wird';
  static const String updateDailyWird = '$userBase/daily-wird/update';
  static const String dailyContent = '$userBase/daily-content';
  
  // ============================================================================
  // Offline Sync Endpoints
  // ============================================================================
  
  static const String syncData = '$userBase/sync';
  static const String syncStatus = '$userBase/sync/status';
  static const String downloadContent = '$userBase/download';
  
  // ============================================================================
  // Stories Service Endpoints
  // ============================================================================
  
  static const String storiesBase = '/api/stories';
  static const String stories = '$storiesBase/stories';
  static String story(String storyId) => '$storiesBase/stories/$storyId';
  static const String storiesCategories = '$storiesBase/categories';
  static String storiesByCategory(String category) => 
      '$storiesBase/categories/$category/stories';
  
  // ============================================================================
  // Notification Service Endpoints
  // ============================================================================
  
  static const String notificationsBase = '/api/notifications';
  static const String notifications = '$notificationsBase/notifications';
  static const String notificationSettings = '$notificationsBase/settings';
  static const String dhikrReminders = '$notificationsBase/dhikr';
  static const String prayerReminders = '$notificationsBase/prayer';
  
  // ============================================================================
  // Khatma Service Endpoints
  // ============================================================================
  
  static const String khatmaBase = '/api/khatma';
  static const String khatmas = '$khatmaBase/khatmas';
  static String khatma(String khatmaId) => '$khatmaBase/khatmas/$khatmaId';
  static const String khatmaPlans = '$khatmaBase/plans';
  static const String khatmaProgress = '$khatmaBase/progress';
  static const String khatmaReminders = '$khatmaBase/reminders';
  
  // ============================================================================
  // Customization Service Endpoints
  // ============================================================================
  
  static const String customizationBase = '/api/customization';
  static const String themes = '$customizationBase/themes';
  static const String fonts = '$customizationBase/fonts';
  static const String layouts = '$customizationBase/layouts';
  
  // ============================================================================
  // Widgets Service Endpoints
  // ============================================================================
  
  static const String widgetsBase = '/api/widgets';
  static const String widgets = '$widgetsBase/widgets';
  static String widget(String widgetId) => '$widgetsBase/widgets/$widgetId';
  static const String widgetConfigurations = '$widgetsBase/configurations';
  
  // ============================================================================
  // Statistics Service Endpoints
  // ============================================================================
  
  static const String statisticsBase = '/api/statistics';
  static const String statisticsDashboard = '$statisticsBase/dashboard';
  static const String khatmaStatistics = '$statisticsBase/khatma';
  static const String readingStatistics = '$statisticsBase/reading';
  static const String recitationStatistics = '$statisticsBase/recitation';
  static const String weeklyComparison = '$statisticsBase/weekly';
  static const String monthlyComparison = '$statisticsBase/monthly';
  static const String personalGoals = '$statisticsBase/goals';
  static const String dailyReadingData = '$statisticsBase/daily-reading';
  static const String recitationScoreHistory = '$statisticsBase/recitation-history';
  
  // ============================================================================
  // Achievements Service Endpoints
  // ============================================================================
  
  static const String achievementsBase = '/api/achievements';
  static const String achievementsDashboard = '$achievementsBase/dashboard';
  static const String achievements = '$achievementsBase/achievements';
  static String achievement(String achievementId) => '$achievementsBase/achievements/$achievementId';
  static const String userLevel = '$achievementsBase/level';
  static const String challenges = '$achievementsBase/challenges';
  static String challenge(String challengeId) => '$achievementsBase/challenges/$challengeId';
  static const String achievementStats = '$achievementsBase/stats';
  static const String achievementReminders = '$achievementsBase/reminders';
  static const String shareAchievement = '$achievementsBase/share';
  static const String achievementUnlockHistory = '$achievementsBase/unlock-history';
  static const String checkAchievements = '$achievementsBase/check';
  static const String achievementLeaderboard = '$achievementsBase/leaderboard';
}

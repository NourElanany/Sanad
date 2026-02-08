/**
 * API endpoints configuration for all backend services
 */
export const API_ENDPOINTS = {
  // ============================================================================
  // Authentication Endpoints
  // ============================================================================
  AUTH: {
    LOGIN: '/api/auth/login',
    REGISTER: '/api/auth/register',
    REFRESH: '/api/auth/refresh',
    LOGOUT: '/api/auth/logout',
    FORGOT_PASSWORD: '/api/auth/forgot-password',
    RESET_PASSWORD: '/api/auth/reset-password',
    VERIFY_EMAIL: '/api/auth/verify-email',
  },

  // ============================================================================
  // Quran Service Endpoints
  // ============================================================================
  QURAN: {
    SURAHS: '/api/quran/surahs',
    SURAH: (surahNumber: number) => `/api/quran/surahs/${surahNumber}`,
    AYAH: (surahNumber: number, ayahNumber: number) => 
      `/api/quran/surahs/${surahNumber}/ayahs/${ayahNumber}`,
    SURAH_AYAHS: (surahNumber: number) => `/api/quran/surahs/${surahNumber}/ayahs`,
    JUZS: '/api/quran/juzs',
    JUZ: (juzNumber: number) => `/api/quran/juzs/${juzNumber}`,
    PAGES: '/api/quran/pages',
    PAGE: (pageNumber: number) => `/api/quran/pages/${pageNumber}`,
    
    // Tafsir
    TAFSIRS: '/api/quran/tafsirs',
    TAFSIR: (tafsirId: number) => `/api/quran/tafsirs/${tafsirId}`,
    AYAH_TAFSIR: (surahNumber: number, ayahNumber: number, tafsirId: number) =>
      `/api/quran/surahs/${surahNumber}/ayahs/${ayahNumber}/tafsir/${tafsirId}`,
    COMPARE_TAFSIRS: (surahNumber: number, ayahNumber: number) =>
      `/api/quran/surahs/${surahNumber}/ayahs/${ayahNumber}/tafsir/compare`,
    
    // Translation
    TRANSLATIONS: '/api/quran/translations',
    TRANSLATION: (translationId: number) => `/api/quran/translations/${translationId}`,
    AYAH_TRANSLATION: (surahNumber: number, ayahNumber: number, translationId: number) =>
      `/api/quran/surahs/${surahNumber}/ayahs/${ayahNumber}/translation/${translationId}`,
    
    // Audio
    SURAH_AUDIO: (surahNumber: number, reciterId: number) =>
      `/api/quran/surahs/${surahNumber}/audio/${reciterId}`,
    AYAH_AUDIO: (surahNumber: number, ayahNumber: number, reciterId: number) =>
      `/api/quran/surahs/${surahNumber}/ayahs/${ayahNumber}/audio/${reciterId}`,
    RECITERS: '/api/quran/reciters',
  },

  // ============================================================================
  // Hadith Service Endpoints
  // ============================================================================
  HADITH: {
    COLLECTIONS: '/api/hadith/collections',
    COLLECTION: (collectionId: string) => `/api/hadith/collections/${collectionId}`,
    BOOKS: (collectionId: string) => `/api/hadith/collections/${collectionId}/books`,
    BOOK: (collectionId: string, bookNumber: number) =>
      `/api/hadith/collections/${collectionId}/books/${bookNumber}`,
    HADITH: (collectionId: string, hadithNumber: number) =>
      `/api/hadith/collections/${collectionId}/hadiths/${hadithNumber}`,
    SEARCH: '/api/hadith/search',
    BY_NARRATOR: '/api/hadith/narrators',
    BY_TOPIC: '/api/hadith/topics',
  },

  // ============================================================================
  // Prayer Times Service Endpoints
  // ============================================================================
  PRAYER_TIMES: '/api/prayer-times/times',
  PRAYER_TIMES_DAILY: '/api/prayer-times/times/daily',
  MONTHLY_PRAYER_TIMES: '/api/prayer-times/times/monthly',
  PRAYER_TIMES_YEARLY: '/api/prayer-times/times/yearly',
  QIBLA: '/api/prayer-times/qibla',
  HIJRI: '/api/prayer-times/hijri',
  HIJRI_DATE: '/api/prayer-times/hijri/today',
  HIJRI_CONVERT: '/api/prayer-times/hijri/convert',
  EVENTS: '/api/prayer-times/events',

  // ============================================================================
  // AI Service Endpoints (RAG System)
  // ============================================================================
  AI: {
    ASK: '/api/ai/ask',
    STREAM: '/api/ai/stream',
    HISTORY: '/api/ai/history',
    VERIFY_SOURCES: '/api/ai/verify-sources',
    MULTIPLE_VIEWPOINTS: '/api/ai/multiple-viewpoints',
    FEEDBACK: '/api/ai/feedback',
  },

  // ============================================================================
  // Audio Analysis Service Endpoints (Tajweed)
  // ============================================================================
  AUDIO: {
    ANALYZE: '/api/audio/analyze',
    HISTORY: '/api/audio/history',
    PROGRESS: '/api/audio/progress',
    COMPARISON: '/api/audio/compare',
    TAJWEED_RULES: '/api/audio/tajweed-rules',
    SUGGESTIONS: '/api/audio/suggestions',
  },

  // ============================================================================
  // Search Service Endpoints (Semantic Search)
  // ============================================================================
  SEARCH: {
    ALL: '/api/search/search',
    QURAN: '/api/search/quran',
    HADITH: '/api/search/hadith',
    FATAWA: '/api/search/fatawa',
    ADVANCED: '/api/search/advanced',
    SUGGESTIONS: '/api/search/suggestions',
  },

  // ============================================================================
  // User Service Endpoints
  // ============================================================================
  USER_PROFILE: '/api/user/profile',
  USER_PREFERENCES: '/api/user/preferences',
  USER_BOOKMARKS: '/api/user/bookmarks',
  USER_READING_PROGRESS: '/api/user/reading-progress',
  USER_KHATMA: '/api/user/khatma',
  USER_STATISTICS: '/api/user/statistics',
  USER_ACHIEVEMENTS: '/api/user/achievements',
  USER_NOTIFICATIONS: '/api/user/notifications',
  
  // Dashboard endpoints
  DASHBOARD: '/api/user/dashboard',
  DAILY_WIRD: '/api/user/daily-wird',
  UPDATE_DAILY_WIRD: '/api/user/daily-wird/update',
  DAILY_CONTENT: '/api/user/daily-content',

  // ============================================================================
  // Offline Sync Endpoints
  // ============================================================================
  SYNC: {
    DATA: '/api/user/sync',
    STATUS: '/api/user/sync/status',
    DOWNLOAD: '/api/user/download',
  },

  // ============================================================================
  // Stories Service Endpoints
  // ============================================================================
  STORIES: {
    ALL: '/api/stories/stories',
    STORY: (storyId: string) => `/api/stories/stories/${storyId}`,
    CATEGORIES: '/api/stories/categories',
    BY_CATEGORY: (category: string) => `/api/stories/categories/${category}/stories`,
  },

  // ============================================================================
  // Notification Service Endpoints
  // ============================================================================
  NOTIFICATIONS: {
    ALL: '/api/notifications/notifications',
    SETTINGS: '/api/notifications/settings',
    DHIKR: '/api/notifications/dhikr',
    PRAYER: '/api/notifications/prayer',
  },

  // ============================================================================
  // Khatma Service Endpoints
  // ============================================================================
  KHATMA: {
    ALL: '/api/khatma/khatmas',
    KHATMA: (khatmaId: string) => `/api/khatma/khatmas/${khatmaId}`,
    PLANS: '/api/khatma/plans',
    PROGRESS: '/api/khatma/progress',
    REMINDERS: '/api/khatma/reminders',
  },

  // ============================================================================
  // Customization Service Endpoints
  // ============================================================================
  CUSTOMIZATION: {
    THEMES: '/api/customization/themes',
    FONTS: '/api/customization/fonts',
    LAYOUTS: '/api/customization/layouts',
  },

  // ============================================================================
  // Widgets Service Endpoints
  // ============================================================================
  WIDGETS: {
    ALL: '/api/widgets/widgets',
    WIDGET: (widgetId: string) => `/api/widgets/widgets/${widgetId}`,
    CONFIGURATIONS: '/api/widgets/configurations',
  },
} as const;

/**
 * Helper function to build URL with query parameters
 */
export function buildUrl(endpoint: string, params?: Record<string, any>): string {
  if (!params || Object.keys(params).length === 0) {
    return endpoint;
  }

  const queryString = Object.entries(params)
    .filter(([_, value]) => value !== undefined && value !== null)
    .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`)
    .join('&');

  return queryString ? `${endpoint}?${queryString}` : endpoint;
}

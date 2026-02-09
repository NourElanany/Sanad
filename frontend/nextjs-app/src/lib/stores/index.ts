/**
 * Central export for all Zustand stores
 * Provides easy access to all state management stores
 */

// Export stores
export { useQuranStore } from './quran-store';
export { usePrayerTimesStore } from './prayer-times-store';
export { useAIAssistantStore } from './ai-assistant-store';
export { useSettingsStore } from './settings-store';

// Export selectors from Quran store
export {
  selectSurahs,
  selectJuzs,
  selectBookmarks,
  selectReadingProgress,
  selectCurrentPage,
  selectCurrentSurah,
  selectSurahByNumber,
  selectBookmarksBySurah,
  selectCachedPage,
  selectCachedSurahAyahs,
} from './quran-store';

// Export selectors from Prayer Times store
export {
  selectPrayerTimes,
  selectHijriDate,
  selectNextPrayer,
  selectLocation,
  selectMadhab,
  selectMonthlyPrayerTimes,
  selectFormattedHijriDate,
  selectTimeUntilNextPrayer,
} from './prayer-times-store';

// Export selectors from AI Assistant store
export {
  selectSessions,
  selectCurrentSessionId,
  selectCurrentMessages,
  selectStreaming,
  selectIsRecording,
  selectRecordingDuration,
  selectCurrentSession,
  selectHasActiveSession,
  selectMessageCount,
  selectLastMessage,
} from './ai-assistant-store';

// Export selectors from Settings store
export {
  selectOnboardingCompleted,
  selectMadhab as selectSettingsMadhab,
  selectLanguage,
  selectDisplay,
  selectNotifications,
  selectAudio,
  selectPrivacy,
  selectOfflineMode,
  selectAutoDownload,
  selectDownloadQuality,
  selectTheme,
  selectFontSize,
  selectEnableAnimations,
  selectHighContrast,
  selectIsRTL,
  selectRecitationVolume,
  selectEffectsVolume,
  selectPreferredReciter,
} from './settings-store';

// Export types
export type { UserPreferences } from '../services/preferences-service';
export type { PrayerTimes, HijriDate, NextPrayer } from '../services/prayer-times-service';
export type { AIMessage, ChatSession, SourceModel } from '../services/ai-assistant-service';
export type { Surah, Juz, QuranBookmark, ReadingProgress, QuranPage, Ayah } from '@/types/quran';

/**
 * Zustand Store for Settings/Preferences State Management
 * Handles user preferences, theme, notifications, and app settings
 * 
 * Requirements: 19.1, 19.2, 19.3, 19.4, 19.5
 */

import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import { devtools } from 'zustand/middleware';
import { PreferencesService, type UserPreferences } from '../services/preferences-service';

// ============================================================================
// Types
// ============================================================================

interface NotificationSettings {
  prayerTimes: boolean;
  dailyReminders: boolean;
  khatmaProgress: boolean;
  achievements: boolean;
}

interface DisplaySettings {
  theme: 'light' | 'dark' | 'auto';
  fontSize: 'small' | 'medium' | 'large' | 'xlarge';
  fontFamily: 'tajawal' | 'alexandria' | 'system';
  quranFontSize: 'small' | 'medium' | 'large' | 'xlarge';
  enableAnimations: boolean;
  highContrast: boolean;
}

interface AudioSettings {
  recitationVolume: number;
  effectsVolume: number;
  enableVoiceNavigation: boolean;
  preferredReciter: string;
}

interface PrivacySettings {
  shareStatistics: boolean;
  enableAnalytics: boolean;
  saveHistory: boolean;
}

interface SettingsState {
  // User Preferences
  onboardingCompleted: boolean;
  madhab: string;
  language: 'ar' | 'en';
  
  // Display Settings
  display: DisplaySettings;
  
  // Notification Settings
  notifications: NotificationSettings;
  
  // Audio Settings
  audio: AudioSettings;
  
  // Privacy Settings
  privacy: PrivacySettings;
  
  // Offline Settings
  offlineMode: boolean;
  autoDownload: boolean;
  downloadQuality: 'low' | 'medium' | 'high';
  
  // UI State
  loading: boolean;
  error: string | null;
  
  // Actions
  loadPreferences: () => void;
  updateOnboarding: (completed: boolean) => void;
  updateMadhab: (madhab: string) => void;
  updateLanguage: (language: 'ar' | 'en') => void;
  updateDisplay: (settings: Partial<DisplaySettings>) => void;
  updateNotifications: (settings: Partial<NotificationSettings>) => void;
  updateAudio: (settings: Partial<AudioSettings>) => void;
  updatePrivacy: (settings: Partial<PrivacySettings>) => void;
  updateOfflineSettings: (settings: {
    offlineMode?: boolean;
    autoDownload?: boolean;
    downloadQuality?: 'low' | 'medium' | 'high';
  }) => void;
  
  // Utility
  exportSettings: () => string;
  importSettings: (jsonString: string) => boolean;
  resetToDefaults: () => void;
  syncWithBackend: () => Promise<void>;
  clearError: () => void;
}

// ============================================================================
// Initial State
// ============================================================================

const initialState = {
  onboardingCompleted: false,
  madhab: 'shafi',
  language: 'ar' as const,
  
  display: {
    theme: 'light' as const,
    fontSize: 'medium' as const,
    fontFamily: 'tajawal' as const,
    quranFontSize: 'medium' as const,
    enableAnimations: true,
    highContrast: false,
  },
  
  notifications: {
    prayerTimes: true,
    dailyReminders: true,
    khatmaProgress: true,
    achievements: true,
  },
  
  audio: {
    recitationVolume: 80,
    effectsVolume: 50,
    enableVoiceNavigation: false,
    preferredReciter: 'abdul-basit',
  },
  
  privacy: {
    shareStatistics: true,
    enableAnalytics: true,
    saveHistory: true,
  },
  
  offlineMode: false,
  autoDownload: false,
  downloadQuality: 'medium' as const,
  
  loading: false,
  error: null,
};

// ============================================================================
// Store Implementation
// ============================================================================

export const useSettingsStore = create<SettingsState>()(
  devtools(
    persist(
      (set, get) => ({
        ...initialState,

        // Load preferences from PreferencesService
        loadPreferences: () => {
          try {
            const prefs = PreferencesService.getPreferences();
            set({
              onboardingCompleted: prefs.onboardingCompleted,
              madhab: prefs.madhab || 'shafi',
              language: prefs.language,
              display: {
                ...get().display,
                theme: prefs.theme,
                fontSize: prefs.fontSize,
                enableAnimations: prefs.enableAnimations,
              },
              notifications: {
                ...get().notifications,
                prayerTimes: prefs.enableNotifications,
              },
            });
          } catch (error: any) {
            set({ error: error.message });
          }
        },

        // Update onboarding status
        updateOnboarding: (completed: boolean) => {
          set({ onboardingCompleted: completed });
          PreferencesService.setOnboardingCompleted(completed);
        },

        // Update madhab
        updateMadhab: (madhab: string) => {
          set({ madhab });
          PreferencesService.setMadhab(madhab);
        },

        // Update language
        updateLanguage: (language: 'ar' | 'en') => {
          set({ language });
          PreferencesService.setLanguage(language);
          
          // Update document direction
          if (typeof document !== 'undefined') {
            document.documentElement.dir = language === 'ar' ? 'rtl' : 'ltr';
            document.documentElement.lang = language;
          }
        },

        // Update display settings
        updateDisplay: (settings: Partial<DisplaySettings>) => {
          const newDisplay = { ...get().display, ...settings };
          set({ display: newDisplay });
          
          // Apply theme immediately
          if (settings.theme && typeof document !== 'undefined') {
            const theme = settings.theme === 'auto'
              ? window.matchMedia('(prefers-color-scheme: dark)').matches
                ? 'dark'
                : 'light'
              : settings.theme;
            
            document.documentElement.classList.remove('light', 'dark');
            document.documentElement.classList.add(theme);
          }
          
          // Sync with PreferencesService
          if (settings.theme) PreferencesService.setTheme(settings.theme);
          if (settings.fontSize) PreferencesService.setFontSize(settings.fontSize);
          if (settings.enableAnimations !== undefined) {
            PreferencesService.setEnableAnimations(settings.enableAnimations);
          }
        },

        // Update notification settings
        updateNotifications: (settings: Partial<NotificationSettings>) => {
          const newNotifications = { ...get().notifications, ...settings };
          set({ notifications: newNotifications });
          
          // Sync with PreferencesService
          if (settings.prayerTimes !== undefined) {
            PreferencesService.setEnableNotifications(settings.prayerTimes);
          }
        },

        // Update audio settings
        updateAudio: (settings: Partial<AudioSettings>) => {
          const newAudio = { ...get().audio, ...settings };
          set({ audio: newAudio });
        },

        // Update privacy settings
        updatePrivacy: (settings: Partial<PrivacySettings>) => {
          const newPrivacy = { ...get().privacy, ...settings };
          set({ privacy: newPrivacy });
        },

        // Update offline settings
        updateOfflineSettings: (settings) => {
          set({
            offlineMode: settings.offlineMode ?? get().offlineMode,
            autoDownload: settings.autoDownload ?? get().autoDownload,
            downloadQuality: settings.downloadQuality ?? get().downloadQuality,
          });
        },

        // Export settings as JSON
        exportSettings: () => {
          const state = get();
          const settings = {
            onboardingCompleted: state.onboardingCompleted,
            madhab: state.madhab,
            language: state.language,
            display: state.display,
            notifications: state.notifications,
            audio: state.audio,
            privacy: state.privacy,
            offlineMode: state.offlineMode,
            autoDownload: state.autoDownload,
            downloadQuality: state.downloadQuality,
          };
          return JSON.stringify(settings, null, 2);
        },

        // Import settings from JSON
        importSettings: (jsonString: string) => {
          try {
            const settings = JSON.parse(jsonString);
            set({
              ...settings,
              loading: false,
              error: null,
            });
            return true;
          } catch (error: any) {
            set({ error: 'Failed to import settings: Invalid JSON' });
            return false;
          }
        },

        // Reset to defaults
        resetToDefaults: () => {
          set(initialState);
          PreferencesService.resetToDefaults();
        },

        // Sync with backend
        syncWithBackend: async () => {
          set({ loading: true, error: null });
          try {
            await PreferencesService.syncWithBackend();
            set({ loading: false });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Clear error
        clearError: () => set({ error: null }),
      }),
      {
        name: 'settings-storage',
        storage: createJSONStorage(() => localStorage),
        // Persist all settings
        partialize: (state) => ({
          onboardingCompleted: state.onboardingCompleted,
          madhab: state.madhab,
          language: state.language,
          display: state.display,
          notifications: state.notifications,
          audio: state.audio,
          privacy: state.privacy,
          offlineMode: state.offlineMode,
          autoDownload: state.autoDownload,
          downloadQuality: state.downloadQuality,
        }),
      }
    ),
    {
      name: 'SettingsStore',
    }
  )
);

// ============================================================================
// Selectors
// ============================================================================

export const selectOnboardingCompleted = (state: SettingsState) => state.onboardingCompleted;
export const selectMadhab = (state: SettingsState) => state.madhab;
export const selectLanguage = (state: SettingsState) => state.language;
export const selectDisplay = (state: SettingsState) => state.display;
export const selectNotifications = (state: SettingsState) => state.notifications;
export const selectAudio = (state: SettingsState) => state.audio;
export const selectPrivacy = (state: SettingsState) => state.privacy;
export const selectOfflineMode = (state: SettingsState) => state.offlineMode;
export const selectAutoDownload = (state: SettingsState) => state.autoDownload;
export const selectDownloadQuality = (state: SettingsState) => state.downloadQuality;
export const selectLoading = (state: SettingsState) => state.loading;
export const selectError = (state: SettingsState) => state.error;

// Computed selectors
export const selectTheme = (state: SettingsState) => state.display.theme;
export const selectFontSize = (state: SettingsState) => state.display.fontSize;
export const selectEnableAnimations = (state: SettingsState) => state.display.enableAnimations;
export const selectHighContrast = (state: SettingsState) => state.display.highContrast;

export const selectIsRTL = (state: SettingsState) => state.language === 'ar';

export const selectRecitationVolume = (state: SettingsState) => state.audio.recitationVolume;
export const selectEffectsVolume = (state: SettingsState) => state.audio.effectsVolume;
export const selectPreferredReciter = (state: SettingsState) => state.audio.preferredReciter;

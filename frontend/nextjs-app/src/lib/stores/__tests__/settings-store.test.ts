/**
 * Unit tests for Settings Store
 * Tests preferences management, theme switching, and persistence
 */

import { renderHook, act } from '@testing-library/react';
import { useSettingsStore } from '../settings-store';
import { PreferencesService } from '../../services/preferences-service';

// Mock the PreferencesService
jest.mock('../../services/preferences-service');

describe('Settings Store', () => {
  const mockPreferences = {
    onboardingCompleted: true,
    madhab: 'shafi',
    theme: 'light' as const,
    fontSize: 'medium' as const,
    enableAnimations: true,
    enableNotifications: true,
    language: 'ar' as const,
  };

  beforeEach(() => {
    // Reset store before each test
    useSettingsStore.getState().resetToDefaults();
    jest.clearAllMocks();
    
    // Mock document
    Object.defineProperty(document, 'documentElement', {
      value: {
        classList: {
          add: jest.fn(),
          remove: jest.fn(),
        },
        dir: '',
        lang: '',
      },
      writable: true,
    });
  });

  describe('loadPreferences', () => {
    it('should load preferences from PreferencesService', () => {
      (PreferencesService.getPreferences as jest.Mock).mockReturnValue(mockPreferences);

      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.loadPreferences();
      });

      expect(result.current.onboardingCompleted).toBe(true);
      expect(result.current.madhab).toBe('shafi');
      expect(result.current.language).toBe('ar');
    });

    it('should handle errors gracefully', () => {
      const error = new Error('Failed to load preferences');
      (PreferencesService.getPreferences as jest.Mock).mockImplementation(() => {
        throw error;
      });

      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.loadPreferences();
      });

      expect(result.current.error).toBe(error.message);
    });
  });

  describe('updateOnboarding', () => {
    it('should update onboarding status', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateOnboarding(true);
      });

      expect(result.current.onboardingCompleted).toBe(true);
      expect(PreferencesService.setOnboardingCompleted).toHaveBeenCalledWith(true);
    });
  });

  describe('updateMadhab', () => {
    it('should update madhab', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateMadhab('hanafi');
      });

      expect(result.current.madhab).toBe('hanafi');
      expect(PreferencesService.setMadhab).toHaveBeenCalledWith('hanafi');
    });
  });

  describe('updateLanguage', () => {
    it('should update language and document attributes', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateLanguage('en');
      });

      expect(result.current.language).toBe('en');
      expect(PreferencesService.setLanguage).toHaveBeenCalledWith('en');
      expect(document.documentElement.dir).toBe('ltr');
      expect(document.documentElement.lang).toBe('en');
    });

    it('should set RTL for Arabic', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateLanguage('ar');
      });

      expect(document.documentElement.dir).toBe('rtl');
      expect(document.documentElement.lang).toBe('ar');
    });
  });

  describe('updateDisplay', () => {
    it('should update display settings', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateDisplay({
          theme: 'dark',
          fontSize: 'large',
          enableAnimations: false,
        });
      });

      expect(result.current.display.theme).toBe('dark');
      expect(result.current.display.fontSize).toBe('large');
      expect(result.current.display.enableAnimations).toBe(false);
    });

    it('should apply theme to document', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateDisplay({ theme: 'dark' });
      });

      expect(document.documentElement.classList.remove).toHaveBeenCalledWith('light', 'dark');
      expect(document.documentElement.classList.add).toHaveBeenCalledWith('dark');
    });

    it('should handle auto theme', () => {
      // Mock matchMedia
      Object.defineProperty(window, 'matchMedia', {
        writable: true,
        value: jest.fn().mockImplementation(query => ({
          matches: query === '(prefers-color-scheme: dark)',
          media: query,
          onchange: null,
          addListener: jest.fn(),
          removeListener: jest.fn(),
          addEventListener: jest.fn(),
          removeEventListener: jest.fn(),
          dispatchEvent: jest.fn(),
        })),
      });

      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateDisplay({ theme: 'auto' });
      });

      expect(document.documentElement.classList.add).toHaveBeenCalledWith('dark');
    });

    it('should sync with PreferencesService', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateDisplay({
          theme: 'dark',
          fontSize: 'large',
          enableAnimations: false,
        });
      });

      expect(PreferencesService.setTheme).toHaveBeenCalledWith('dark');
      expect(PreferencesService.setFontSize).toHaveBeenCalledWith('large');
      expect(PreferencesService.setEnableAnimations).toHaveBeenCalledWith(false);
    });
  });

  describe('updateNotifications', () => {
    it('should update notification settings', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateNotifications({
          prayerTimes: false,
          dailyReminders: false,
        });
      });

      expect(result.current.notifications.prayerTimes).toBe(false);
      expect(result.current.notifications.dailyReminders).toBe(false);
    });

    it('should sync with PreferencesService', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateNotifications({ prayerTimes: false });
      });

      expect(PreferencesService.setEnableNotifications).toHaveBeenCalledWith(false);
    });
  });

  describe('updateAudio', () => {
    it('should update audio settings', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateAudio({
          recitationVolume: 50,
          effectsVolume: 30,
          enableVoiceNavigation: true,
        });
      });

      expect(result.current.audio.recitationVolume).toBe(50);
      expect(result.current.audio.effectsVolume).toBe(30);
      expect(result.current.audio.enableVoiceNavigation).toBe(true);
    });
  });

  describe('updatePrivacy', () => {
    it('should update privacy settings', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updatePrivacy({
          shareStatistics: false,
          enableAnalytics: false,
        });
      });

      expect(result.current.privacy.shareStatistics).toBe(false);
      expect(result.current.privacy.enableAnalytics).toBe(false);
    });
  });

  describe('updateOfflineSettings', () => {
    it('should update offline settings', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateOfflineSettings({
          offlineMode: true,
          autoDownload: true,
          downloadQuality: 'high',
        });
      });

      expect(result.current.offlineMode).toBe(true);
      expect(result.current.autoDownload).toBe(true);
      expect(result.current.downloadQuality).toBe('high');
    });
  });

  describe('exportSettings', () => {
    it('should export settings as JSON', () => {
      const { result } = renderHook(() => useSettingsStore());

      // Set some settings
      act(() => {
        result.current.updateMadhab('hanafi');
        result.current.updateLanguage('en');
      });

      const exported = result.current.exportSettings();
      const parsed = JSON.parse(exported);

      expect(parsed.madhab).toBe('hanafi');
      expect(parsed.language).toBe('en');
    });
  });

  describe('importSettings', () => {
    it('should import settings from JSON', () => {
      const { result } = renderHook(() => useSettingsStore());

      const settingsJson = JSON.stringify({
        madhab: 'maliki',
        language: 'en',
        display: {
          theme: 'dark',
          fontSize: 'large',
        },
      });

      act(() => {
        const success = result.current.importSettings(settingsJson);
        expect(success).toBe(true);
      });

      expect(result.current.madhab).toBe('maliki');
      expect(result.current.language).toBe('en');
    });

    it('should handle invalid JSON', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        const success = result.current.importSettings('invalid json');
        expect(success).toBe(false);
      });

      expect(result.current.error).toContain('Invalid JSON');
    });
  });

  describe('resetToDefaults', () => {
    it('should reset all settings to defaults', () => {
      const { result } = renderHook(() => useSettingsStore());

      // Set some custom settings
      act(() => {
        result.current.updateMadhab('hanafi');
        result.current.updateLanguage('en');
        result.current.updateDisplay({ theme: 'dark' });
      });

      // Reset
      act(() => {
        result.current.resetToDefaults();
      });

      expect(result.current.madhab).toBe('shafi');
      expect(result.current.language).toBe('ar');
      expect(result.current.display.theme).toBe('light');
      expect(PreferencesService.resetToDefaults).toHaveBeenCalled();
    });
  });

  describe('syncWithBackend', () => {
    it('should sync settings with backend', async () => {
      (PreferencesService.syncWithBackend as jest.Mock).mockResolvedValue(undefined);

      const { result } = renderHook(() => useSettingsStore());

      await act(async () => {
        await result.current.syncWithBackend();
      });

      expect(result.current.loading).toBe(false);
      expect(PreferencesService.syncWithBackend).toHaveBeenCalled();
    });

    it('should handle sync errors', async () => {
      const error = new Error('Sync failed');
      (PreferencesService.syncWithBackend as jest.Mock).mockRejectedValue(error);

      const { result } = renderHook(() => useSettingsStore());

      await act(async () => {
        await result.current.syncWithBackend();
      });

      expect(result.current.error).toBe(error.message);
      expect(result.current.loading).toBe(false);
    });
  });

  describe('selectors', () => {
    it('should select theme', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateDisplay({ theme: 'dark' });
      });

      expect(result.current.display.theme).toBe('dark');
    });

    it('should determine RTL based on language', () => {
      const { result } = renderHook(() => useSettingsStore());

      act(() => {
        result.current.updateLanguage('ar');
      });

      expect(result.current.language).toBe('ar');

      act(() => {
        result.current.updateLanguage('en');
      });

      expect(result.current.language).toBe('en');
    });
  });
});

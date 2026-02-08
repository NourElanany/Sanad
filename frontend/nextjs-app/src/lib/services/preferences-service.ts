/**
 * Service for managing user preferences in localStorage
 */

export interface UserPreferences {
  onboardingCompleted: boolean;
  madhab?: string;
  theme: 'light' | 'dark' | 'auto';
  fontSize: 'small' | 'medium' | 'large' | 'xlarge';
  enableAnimations: boolean;
  enableNotifications: boolean;
  language: 'ar' | 'en';
}

const DEFAULT_PREFERENCES: UserPreferences = {
  onboardingCompleted: false,
  theme: 'light',
  fontSize: 'medium',
  enableAnimations: true,
  enableNotifications: true,
  language: 'ar',
};

const STORAGE_KEY = 'userPreferences';

export class PreferencesService {
  /**
   * Get all user preferences
   */
  static getPreferences(): UserPreferences {
    if (typeof window === 'undefined') {
      return DEFAULT_PREFERENCES;
    }

    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (!stored) {
        return DEFAULT_PREFERENCES;
      }

      const parsed = JSON.parse(stored);
      return { ...DEFAULT_PREFERENCES, ...parsed };
    } catch (error) {
      console.error('Failed to load preferences:', error);
      return DEFAULT_PREFERENCES;
    }
  }

  /**
   * Save all user preferences
   */
  static setPreferences(preferences: Partial<UserPreferences>): void {
    if (typeof window === 'undefined') {
      return;
    }

    try {
      const current = this.getPreferences();
      const updated = { ...current, ...preferences };
      localStorage.setItem(STORAGE_KEY, JSON.stringify(updated));
    } catch (error) {
      console.error('Failed to save preferences:', error);
    }
  }

  /**
   * Get onboarding completion status
   */
  static getOnboardingCompleted(): boolean {
    return this.getPreferences().onboardingCompleted;
  }

  /**
   * Set onboarding completion status
   */
  static setOnboardingCompleted(completed: boolean): void {
    this.setPreferences({ onboardingCompleted: completed });
  }

  /**
   * Get madhab selection
   */
  static getMadhab(): string | undefined {
    return this.getPreferences().madhab;
  }

  /**
   * Set madhab selection
   */
  static setMadhab(madhab: string): void {
    this.setPreferences({ madhab });
  }

  /**
   * Get theme preference
   */
  static getTheme(): 'light' | 'dark' | 'auto' {
    return this.getPreferences().theme;
  }

  /**
   * Set theme preference
   */
  static setTheme(theme: 'light' | 'dark' | 'auto'): void {
    this.setPreferences({ theme });
  }

  /**
   * Get font size preference
   */
  static getFontSize(): 'small' | 'medium' | 'large' | 'xlarge' {
    return this.getPreferences().fontSize;
  }

  /**
   * Set font size preference
   */
  static setFontSize(fontSize: 'small' | 'medium' | 'large' | 'xlarge'): void {
    this.setPreferences({ fontSize });
  }

  /**
   * Get animations preference
   */
  static getEnableAnimations(): boolean {
    return this.getPreferences().enableAnimations;
  }

  /**
   * Set animations preference
   */
  static setEnableAnimations(enable: boolean): void {
    this.setPreferences({ enableAnimations: enable });
  }

  /**
   * Get notifications preference
   */
  static getEnableNotifications(): boolean {
    return this.getPreferences().enableNotifications;
  }

  /**
   * Set notifications preference
   */
  static setEnableNotifications(enable: boolean): void {
    this.setPreferences({ enableNotifications: enable });
  }

  /**
   * Get language preference
   */
  static getLanguage(): 'ar' | 'en' {
    return this.getPreferences().language;
  }

  /**
   * Set language preference
   */
  static setLanguage(language: 'ar' | 'en'): void {
    this.setPreferences({ language });
  }

  /**
   * Export preferences as JSON string for backup
   */
  static exportPreferences(): string {
    const prefs = this.getPreferences();
    return JSON.stringify(prefs, null, 2);
  }

  /**
   * Import preferences from JSON string
   */
  static importPreferences(jsonString: string): boolean {
    try {
      const prefs = JSON.parse(jsonString);
      this.setPreferences(prefs);
      return true;
    } catch (error) {
      console.error('Failed to import preferences:', error);
      return false;
    }
  }

  /**
   * Reset all preferences to defaults
   */
  static resetToDefaults(): void {
    if (typeof window === 'undefined') {
      return;
    }

    localStorage.removeItem(STORAGE_KEY);
  }

  /**
   * Sync preferences with backend (placeholder for future implementation)
   */
  static async syncWithBackend(): Promise<void> {
    // TODO: Implement backend sync
    // const prefs = this.getPreferences();
    // await apiClient.post('/api/preferences/sync', prefs);
  }
}

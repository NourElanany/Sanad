/**
 * Accessibility Service for managing accessibility features
 */

export interface AccessibilitySettings {
  screenReaderEnabled: boolean;
  highContrastEnabled: boolean;
  voiceNavigationEnabled: boolean;
  textScaleFactor: number;
  reduceAnimations: boolean;
  keyboardShortcutsEnabled: boolean;
}

const STORAGE_KEY = 'sanad_accessibility_settings';

const DEFAULT_SETTINGS: AccessibilitySettings = {
  screenReaderEnabled: false,
  highContrastEnabled: false,
  voiceNavigationEnabled: false,
  textScaleFactor: 1.0,
  reduceAnimations: false,
  keyboardShortcutsEnabled: true,
};

class AccessibilityService {
  private settings: AccessibilitySettings;
  private listeners: Set<(settings: AccessibilitySettings) => void> = new Set();

  constructor() {
    this.settings = this.loadSettings();
    this.applySettings();
  }

  /**
   * Load settings from localStorage
   */
  private loadSettings(): AccessibilitySettings {
    if (typeof window === 'undefined') return DEFAULT_SETTINGS;

    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        return { ...DEFAULT_SETTINGS, ...JSON.parse(stored) };
      }
    } catch (error) {
      console.error('Failed to load accessibility settings:', error);
    }

    return DEFAULT_SETTINGS;
  }

  /**
   * Save settings to localStorage
   */
  private saveSettings(): void {
    if (typeof window === 'undefined') return;

    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(this.settings));
      this.notifyListeners();
    } catch (error) {
      console.error('Failed to save accessibility settings:', error);
    }
  }

  /**
   * Apply settings to the document
   */
  private applySettings(): void {
    if (typeof window === 'undefined') return;

    const root = document.documentElement;

    // Apply text scale
    root.style.fontSize = `${this.settings.textScaleFactor * 16}px`;

    // Apply high contrast
    if (this.settings.highContrastEnabled) {
      root.classList.add('high-contrast');
    } else {
      root.classList.remove('high-contrast');
    }

    // Apply reduce animations
    if (this.settings.reduceAnimations) {
      root.classList.add('reduce-animations');
    } else {
      root.classList.remove('reduce-animations');
    }

    // Set ARIA live region for screen reader announcements
    if (this.settings.screenReaderEnabled) {
      this.ensureAriaLiveRegion();
    }
  }

  /**
   * Ensure ARIA live region exists for announcements
   */
  private ensureAriaLiveRegion(): void {
    if (typeof window === 'undefined') return;

    let liveRegion = document.getElementById('aria-live-region');
    if (!liveRegion) {
      liveRegion = document.createElement('div');
      liveRegion.id = 'aria-live-region';
      liveRegion.setAttribute('role', 'status');
      liveRegion.setAttribute('aria-live', 'polite');
      liveRegion.setAttribute('aria-atomic', 'true');
      liveRegion.style.position = 'absolute';
      liveRegion.style.left = '-10000px';
      liveRegion.style.width = '1px';
      liveRegion.style.height = '1px';
      liveRegion.style.overflow = 'hidden';
      document.body.appendChild(liveRegion);
    }
  }

  /**
   * Get current settings
   */
  getSettings(): AccessibilitySettings {
    return { ...this.settings };
  }

  /**
   * Update settings
   */
  updateSettings(updates: Partial<AccessibilitySettings>): void {
    this.settings = { ...this.settings, ...updates };
    this.saveSettings();
    this.applySettings();
  }

  /**
   * Toggle screen reader
   */
  toggleScreenReader(): void {
    this.updateSettings({
      screenReaderEnabled: !this.settings.screenReaderEnabled,
    });

    if (this.settings.screenReaderEnabled) {
      this.announce('تم تفعيل قارئ الشاشة');
    }
  }

  /**
   * Toggle high contrast mode
   */
  toggleHighContrast(): void {
    this.updateSettings({
      highContrastEnabled: !this.settings.highContrastEnabled,
    });
  }

  /**
   * Toggle voice navigation
   */
  toggleVoiceNavigation(): void {
    this.updateSettings({
      voiceNavigationEnabled: !this.settings.voiceNavigationEnabled,
    });
  }

  /**
   * Set text scale factor
   */
  setTextScaleFactor(factor: number): void {
    const clampedFactor = Math.max(0.8, Math.min(2.0, factor));
    this.updateSettings({ textScaleFactor: clampedFactor });
  }

  /**
   * Increase text size
   */
  increaseTextSize(): void {
    const newFactor = Math.min(2.0, this.settings.textScaleFactor + 0.1);
    this.setTextScaleFactor(newFactor);
  }

  /**
   * Decrease text size
   */
  decreaseTextSize(): void {
    const newFactor = Math.max(0.8, this.settings.textScaleFactor - 0.1);
    this.setTextScaleFactor(newFactor);
  }

  /**
   * Reset text size
   */
  resetTextSize(): void {
    this.setTextScaleFactor(1.0);
  }

  /**
   * Toggle reduce animations
   */
  toggleReduceAnimations(): void {
    this.updateSettings({
      reduceAnimations: !this.settings.reduceAnimations,
    });
  }

  /**
   * Toggle keyboard shortcuts
   */
  toggleKeyboardShortcuts(): void {
    this.updateSettings({
      keyboardShortcutsEnabled: !this.settings.keyboardShortcutsEnabled,
    });
  }

  /**
   * Announce message to screen reader
   */
  announce(message: string): void {
    if (!this.settings.screenReaderEnabled) return;
    if (typeof window === 'undefined') return;

    const liveRegion = document.getElementById('aria-live-region');
    if (liveRegion) {
      liveRegion.textContent = message;
      // Clear after announcement
      setTimeout(() => {
        liveRegion.textContent = '';
      }, 1000);
    }
  }

  /**
   * Subscribe to settings changes
   */
  subscribe(listener: (settings: AccessibilitySettings) => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  /**
   * Notify all listeners
   */
  private notifyListeners(): void {
    this.listeners.forEach((listener) => listener(this.settings));
  }

  /**
   * Reset to default settings
   */
  resetToDefaults(): void {
    this.settings = { ...DEFAULT_SETTINGS };
    this.saveSettings();
    this.applySettings();
  }

  /**
   * Check if system has accessibility features enabled
   */
  hasSystemAccessibilityEnabled(): boolean {
    if (typeof window === 'undefined') return false;

    return (
      window.matchMedia('(prefers-reduced-motion: reduce)').matches ||
      window.matchMedia('(prefers-contrast: high)').matches
    );
  }
}

// Export singleton instance
export const accessibilityService = new AccessibilityService();

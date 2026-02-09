/**
 * Keyboard Shortcuts Service for managing keyboard navigation
 */

export interface KeyboardShortcut {
  key: string;
  ctrl?: boolean;
  alt?: boolean;
  shift?: boolean;
  action: () => void;
  description: string;
}

class KeyboardShortcutsService {
  private shortcuts: Map<string, KeyboardShortcut> = new Map();
  private isEnabled = true;
  private isListening = false;

  /**
   * Register a keyboard shortcut
   */
  register(shortcut: KeyboardShortcut): void {
    const key = this.getShortcutKey(shortcut);
    this.shortcuts.set(key, shortcut);
  }

  /**
   * Unregister a keyboard shortcut
   */
  unregister(key: string, ctrl?: boolean, alt?: boolean, shift?: boolean): void {
    const shortcutKey = this.getShortcutKeyFromParams(key, ctrl, alt, shift);
    this.shortcuts.delete(shortcutKey);
  }

  /**
   * Get shortcut key string
   */
  private getShortcutKey(shortcut: KeyboardShortcut): string {
    return this.getShortcutKeyFromParams(
      shortcut.key,
      shortcut.ctrl,
      shortcut.alt,
      shortcut.shift
    );
  }

  /**
   * Get shortcut key string from parameters
   */
  private getShortcutKeyFromParams(
    key: string,
    ctrl?: boolean,
    alt?: boolean,
    shift?: boolean
  ): string {
    const parts: string[] = [];
    if (ctrl) parts.push('ctrl');
    if (alt) parts.push('alt');
    if (shift) parts.push('shift');
    parts.push(key.toLowerCase());
    return parts.join('+');
  }

  /**
   * Handle keyboard event
   */
  private handleKeyDown = (event: KeyboardEvent): void => {
    if (!this.isEnabled) return;

    // Don't trigger shortcuts when typing in input fields
    const target = event.target as HTMLElement;
    if (
      target.tagName === 'INPUT' ||
      target.tagName === 'TEXTAREA' ||
      target.isContentEditable
    ) {
      return;
    }

    const key = this.getShortcutKeyFromParams(
      event.key,
      event.ctrlKey || event.metaKey,
      event.altKey,
      event.shiftKey
    );

    const shortcut = this.shortcuts.get(key);
    if (shortcut) {
      event.preventDefault();
      shortcut.action();
    }
  };

  /**
   * Start listening for keyboard events
   */
  startListening(): void {
    if (this.isListening) return;
    if (typeof window === 'undefined') return;

    window.addEventListener('keydown', this.handleKeyDown);
    this.isListening = true;
  }

  /**
   * Stop listening for keyboard events
   */
  stopListening(): void {
    if (!this.isListening) return;
    if (typeof window === 'undefined') return;

    window.removeEventListener('keydown', this.handleKeyDown);
    this.isListening = false;
  }

  /**
   * Enable shortcuts
   */
  enable(): void {
    this.isEnabled = true;
  }

  /**
   * Disable shortcuts
   */
  disable(): void {
    this.isEnabled = false;
  }

  /**
   * Get all registered shortcuts
   */
  getAllShortcuts(): KeyboardShortcut[] {
    return Array.from(this.shortcuts.values());
  }

  /**
   * Clear all shortcuts
   */
  clearAll(): void {
    this.shortcuts.clear();
  }

  /**
   * Register default application shortcuts
   */
  registerDefaultShortcuts(router: any, accessibilityService: any): void {
    // Navigation shortcuts
    this.register({
      key: 'h',
      ctrl: true,
      description: 'الصفحة الرئيسية',
      action: () => router.push('/dashboard'),
    });

    this.register({
      key: 'q',
      ctrl: true,
      description: 'القرآن الكريم',
      action: () => router.push('/quran'),
    });

    this.register({
      key: 'a',
      ctrl: true,
      description: 'المساعد الذكي',
      action: () => router.push('/ai-assistant'),
    });

    this.register({
      key: 's',
      ctrl: true,
      description: 'البحث',
      action: () => router.push('/search'),
    });

    this.register({
      key: 'p',
      ctrl: true,
      description: 'مواقيت الصلاة',
      action: () => router.push('/dashboard'),
    });

    this.register({
      key: 'k',
      ctrl: true,
      description: 'القبلة',
      action: () => router.push('/qibla'),
    });

    this.register({
      key: ',',
      ctrl: true,
      description: 'الإعدادات',
      action: () => router.push('/settings'),
    });

    // Back navigation
    this.register({
      key: 'Escape',
      description: 'رجوع',
      action: () => router.back(),
    });

    // Text scaling shortcuts
    this.register({
      key: '=',
      ctrl: true,
      description: 'تكبير النص',
      action: () => accessibilityService.increaseTextSize(),
    });

    this.register({
      key: '-',
      ctrl: true,
      description: 'تصغير النص',
      action: () => accessibilityService.decreaseTextSize(),
    });

    this.register({
      key: '0',
      ctrl: true,
      description: 'إعادة تعيين حجم النص',
      action: () => accessibilityService.resetTextSize(),
    });

    // Accessibility shortcuts
    this.register({
      key: 'r',
      ctrl: true,
      shift: true,
      description: 'تبديل قارئ الشاشة',
      action: () => accessibilityService.toggleScreenReader(),
    });

    this.register({
      key: 'c',
      ctrl: true,
      shift: true,
      description: 'تبديل وضع التباين العالي',
      action: () => accessibilityService.toggleHighContrast(),
    });

    this.register({
      key: 'v',
      ctrl: true,
      shift: true,
      description: 'تبديل التنقل الصوتي',
      action: () => accessibilityService.toggleVoiceNavigation(),
    });
  }

  /**
   * Get shortcut display string
   */
  getShortcutDisplay(shortcut: KeyboardShortcut): string {
    const parts: string[] = [];
    if (shortcut.ctrl) parts.push('Ctrl');
    if (shortcut.alt) parts.push('Alt');
    if (shortcut.shift) parts.push('Shift');
    parts.push(shortcut.key.toUpperCase());
    return parts.join(' + ');
  }
}

// Export singleton instance
export const keyboardShortcutsService = new KeyboardShortcutsService();

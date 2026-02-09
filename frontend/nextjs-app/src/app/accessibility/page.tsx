'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import {
  accessibilityService,
  AccessibilitySettings,
} from '@/lib/services/accessibility-service';
import { keyboardShortcutsService } from '@/lib/services/keyboard-shortcuts-service';

export default function AccessibilityPage() {
  const router = useRouter();
  const [settings, setSettings] = useState<AccessibilitySettings>(
    accessibilityService.getSettings()
  );
  const [showShortcuts, setShowShortcuts] = useState(false);

  useEffect(() => {
    const unsubscribe = accessibilityService.subscribe(setSettings);
    return unsubscribe;
  }, []);

  const handleToggleScreenReader = () => {
    accessibilityService.toggleScreenReader();
  };

  const handleToggleHighContrast = () => {
    accessibilityService.toggleHighContrast();
  };

  const handleToggleVoiceNavigation = () => {
    accessibilityService.toggleVoiceNavigation();
  };

  const handleTextScaleChange = (value: number) => {
    accessibilityService.setTextScaleFactor(value);
  };

  const handleToggleReduceAnimations = () => {
    accessibilityService.toggleReduceAnimations();
  };

  const handleToggleKeyboardShortcuts = () => {
    accessibilityService.toggleKeyboardShortcuts();
  };

  const shortcuts = keyboardShortcutsService.getAllShortcuts();

  return (
    <div className="min-h-screen bg-[#FEFEFE]" dir="rtl">
      {/* Header */}
      <header className="bg-[#1B365D] text-white shadow-lg">
        <div className="container mx-auto px-4 py-6">
          <div className="flex items-center justify-between">
            <button
              onClick={() => router.back()}
              className="p-2 hover:bg-white/10 rounded-lg transition-colors"
              aria-label="رجوع"
            >
              <svg
                className="w-6 h-6"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 5l7 7-7 7"
                />
              </svg>
            </button>
            <h1 className="text-2xl font-bold">إمكانية الوصول</h1>
            <div className="w-10" />
          </div>
        </div>
      </header>

      {/* Content */}
      <main className="container mx-auto px-4 py-8 max-w-4xl">
        {/* Screen Reader Section */}
        <section className="mb-8">
          <h2 className="text-xl font-bold text-[#1A1A1A] mb-4">قارئ الشاشة</h2>
          <div className="bg-white rounded-2xl shadow-md border border-[#1B365D]/10 p-6">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-4">
                <div className="w-12 h-12 bg-[#1B365D]/10 rounded-full flex items-center justify-center">
                  <svg
                    className="w-6 h-6 text-[#1B365D]"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                    />
                  </svg>
                </div>
                <div>
                  <h3 className="font-semibold text-[#1A1A1A]">
                    تفعيل قارئ الشاشة
                  </h3>
                  <p className="text-sm text-[#666666]">
                    قراءة محتوى الشاشة بصوت عالٍ
                  </p>
                </div>
              </div>
              <button
                onClick={handleToggleScreenReader}
                className={`relative inline-flex h-8 w-14 items-center rounded-full transition-colors ${
                  settings.screenReaderEnabled ? 'bg-[#1B365D]' : 'bg-gray-300'
                }`}
                role="switch"
                aria-checked={settings.screenReaderEnabled}
                aria-label="تبديل قارئ الشاشة"
              >
                <span
                  className={`inline-block h-6 w-6 transform rounded-full bg-white transition-transform ${
                    settings.screenReaderEnabled ? 'translate-x-1' : 'translate-x-7'
                  }`}
                />
              </button>
            </div>
          </div>
        </section>

        {/* Visual Settings Section */}
        <section className="mb-8">
          <h2 className="text-xl font-bold text-[#1A1A1A] mb-4">
            الإعدادات البصرية
          </h2>

          {/* High Contrast */}
          <div className="bg-white rounded-2xl shadow-md border border-[#1B365D]/10 p-6 mb-4">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-4">
                <div className="w-12 h-12 bg-[#1B365D]/10 rounded-full flex items-center justify-center">
                  <svg
                    className="w-6 h-6 text-[#1B365D]"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                    />
                  </svg>
                </div>
                <div>
                  <h3 className="font-semibold text-[#1A1A1A]">
                    وضع التباين العالي
                  </h3>
                  <p className="text-sm text-[#666666]">
                    ألوان عالية التباين لتحسين الرؤية
                  </p>
                </div>
              </div>
              <button
                onClick={handleToggleHighContrast}
                className={`relative inline-flex h-8 w-14 items-center rounded-full transition-colors ${
                  settings.highContrastEnabled ? 'bg-[#1B365D]' : 'bg-gray-300'
                }`}
                role="switch"
                aria-checked={settings.highContrastEnabled}
                aria-label="تبديل وضع التباين العالي"
              >
                <span
                  className={`inline-block h-6 w-6 transform rounded-full bg-white transition-transform ${
                    settings.highContrastEnabled ? 'translate-x-1' : 'translate-x-7'
                  }`}
                />
              </button>
            </div>
          </div>

          {/* Text Scaling */}
          <div className="bg-white rounded-2xl shadow-md border border-[#1B365D]/10 p-6 mb-4">
            <div className="flex items-center gap-4 mb-4">
              <div className="w-12 h-12 bg-[#1B365D]/10 rounded-full flex items-center justify-center">
                <svg
                  className="w-6 h-6 text-[#1B365D]"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129"
                  />
                </svg>
              </div>
              <div className="flex-1">
                <h3 className="font-semibold text-[#1A1A1A]">حجم النصوص</h3>
                <p className="text-sm text-[#666666]">
                  {Math.round(settings.textScaleFactor * 100)}%
                </p>
              </div>
            </div>
            <div className="flex items-center gap-4">
              <input
                type="range"
                min="0.8"
                max="2.0"
                step="0.1"
                value={settings.textScaleFactor}
                onChange={(e) => handleTextScaleChange(parseFloat(e.target.value))}
                className="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-[#1B365D]"
                aria-label="حجم النصوص"
              />
              <button
                onClick={() => accessibilityService.resetTextSize()}
                className="px-4 py-2 bg-[#1B365D]/10 text-[#1B365D] rounded-lg hover:bg-[#1B365D]/20 transition-colors"
                aria-label="إعادة تعيين حجم النص"
              >
                إعادة تعيين
              </button>
            </div>
            <div className="mt-4 p-4 bg-[#F8F9FA] rounded-lg">
              <p style={{ fontSize: `${settings.textScaleFactor}rem` }}>
                مثال على النص
              </p>
            </div>
          </div>

          {/* Reduce Animations */}
          <div className="bg-white rounded-2xl shadow-md border border-[#1B365D]/10 p-6">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-4">
                <div className="w-12 h-12 bg-[#1B365D]/10 rounded-full flex items-center justify-center">
                  <svg
                    className="w-6 h-6 text-[#1B365D]"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
                    />
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                  </svg>
                </div>
                <div>
                  <h3 className="font-semibold text-[#1A1A1A]">تقليل الحركات</h3>
                  <p className="text-sm text-[#666666]">
                    تقليل الرسوم المتحركة والتأثيرات
                  </p>
                </div>
              </div>
              <button
                onClick={handleToggleReduceAnimations}
                className={`relative inline-flex h-8 w-14 items-center rounded-full transition-colors ${
                  settings.reduceAnimations ? 'bg-[#1B365D]' : 'bg-gray-300'
                }`}
                role="switch"
                aria-checked={settings.reduceAnimations}
                aria-label="تبديل تقليل الحركات"
              >
                <span
                  className={`inline-block h-6 w-6 transform rounded-full bg-white transition-transform ${
                    settings.reduceAnimations ? 'translate-x-1' : 'translate-x-7'
                  }`}
                />
              </button>
            </div>
          </div>
        </section>

        {/* Navigation Section */}
        <section className="mb-8">
          <h2 className="text-xl font-bold text-[#1A1A1A] mb-4">التنقل</h2>

          {/* Voice Navigation */}
          <div className="bg-white rounded-2xl shadow-md border border-[#1B365D]/10 p-6 mb-4">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-4">
                <div className="w-12 h-12 bg-[#1B365D]/10 rounded-full flex items-center justify-center">
                  <svg
                    className="w-6 h-6 text-[#1B365D]"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
                    />
                  </svg>
                </div>
                <div>
                  <h3 className="font-semibold text-[#1A1A1A]">التنقل الصوتي</h3>
                  <p className="text-sm text-[#666666]">
                    استخدام الأوامر الصوتية للتنقل
                  </p>
                </div>
              </div>
              <button
                onClick={handleToggleVoiceNavigation}
                className={`relative inline-flex h-8 w-14 items-center rounded-full transition-colors ${
                  settings.voiceNavigationEnabled ? 'bg-[#1B365D]' : 'bg-gray-300'
                }`}
                role="switch"
                aria-checked={settings.voiceNavigationEnabled}
                aria-label="تبديل التنقل الصوتي"
              >
                <span
                  className={`inline-block h-6 w-6 transform rounded-full bg-white transition-transform ${
                    settings.voiceNavigationEnabled
                      ? 'translate-x-1'
                      : 'translate-x-7'
                  }`}
                />
              </button>
            </div>
          </div>

          {/* Keyboard Shortcuts */}
          <div className="bg-white rounded-2xl shadow-md border border-[#1B365D]/10 p-6">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-4">
                <div className="w-12 h-12 bg-[#1B365D]/10 rounded-full flex items-center justify-center">
                  <svg
                    className="w-6 h-6 text-[#1B365D]"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"
                    />
                  </svg>
                </div>
                <div>
                  <h3 className="font-semibold text-[#1A1A1A]">
                    اختصارات لوحة المفاتيح
                  </h3>
                  <p className="text-sm text-[#666666]">
                    استخدام اختصارات لوحة المفاتيح
                  </p>
                </div>
              </div>
              <button
                onClick={handleToggleKeyboardShortcuts}
                className={`relative inline-flex h-8 w-14 items-center rounded-full transition-colors ${
                  settings.keyboardShortcutsEnabled
                    ? 'bg-[#1B365D]'
                    : 'bg-gray-300'
                }`}
                role="switch"
                aria-checked={settings.keyboardShortcutsEnabled}
                aria-label="تبديل اختصارات لوحة المفاتيح"
              >
                <span
                  className={`inline-block h-6 w-6 transform rounded-full bg-white transition-transform ${
                    settings.keyboardShortcutsEnabled
                      ? 'translate-x-1'
                      : 'translate-x-7'
                  }`}
                />
              </button>
            </div>
          </div>
        </section>

        {/* Help Section */}
        <section>
          <h2 className="text-xl font-bold text-[#1A1A1A] mb-4">المساعدة</h2>
          <div className="bg-white rounded-2xl shadow-md border border-[#1B365D]/10 p-6">
            <button
              onClick={() => setShowShortcuts(!showShortcuts)}
              className="w-full flex items-center justify-between"
            >
              <div className="flex items-center gap-4">
                <div className="w-12 h-12 bg-[#1B365D]/10 rounded-full flex items-center justify-center">
                  <svg
                    className="w-6 h-6 text-[#1B365D]"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                  </svg>
                </div>
                <h3 className="font-semibold text-[#1A1A1A]">
                  عرض اختصارات لوحة المفاتيح
                </h3>
              </div>
              <svg
                className={`w-5 h-5 text-[#666666] transition-transform ${
                  showShortcuts ? 'rotate-180' : ''
                }`}
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M19 9l-7 7-7-7"
                />
              </svg>
            </button>

            {showShortcuts && shortcuts.length > 0 && (
              <div className="mt-6 space-y-3">
                {shortcuts.map((shortcut, index) => (
                  <div
                    key={index}
                    className="flex items-center justify-between py-2 border-t border-gray-200"
                  >
                    <span className="text-sm text-[#666666]">
                      {shortcut.description}
                    </span>
                    <kbd className="px-3 py-1 bg-[#F8F9FA] border border-gray-300 rounded text-sm font-mono">
                      {keyboardShortcutsService.getShortcutDisplay(shortcut)}
                    </kbd>
                  </div>
                ))}
              </div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}

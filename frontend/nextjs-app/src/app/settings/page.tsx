'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { IslamicCard } from '@/components/ui/IslamicCard';
import { PreferencesService, UserPreferences } from '@/lib/services/preferences-service';

const madhabs = [
  { id: 'hanafi', name: 'الحنفي' },
  { id: 'maliki', name: 'المالكي' },
  { id: 'shafii', name: 'الشافعي' },
  { id: 'hanbali', name: 'الحنبلي' },
  { id: 'jafari', name: 'الجعفري' },
];

const themes = [
  { id: 'light', name: 'الوضع النهاري', icon: '☀️' },
  { id: 'dark', name: 'الوضع الليلي', icon: '🌙' },
  { id: 'auto', name: 'تلقائي', icon: '🔄' },
];

const fontSizes = [
  { id: 'small', name: 'صغير' },
  { id: 'medium', name: 'متوسط' },
  { id: 'large', name: 'كبير' },
  { id: 'xlarge', name: 'كبير جداً' },
];

export default function SettingsPage() {
  const router = useRouter();
  const [preferences, setPreferences] = useState<UserPreferences>(
    PreferencesService.getPreferences()
  );
  const [showBackup, setShowBackup] = useState(false);
  const [backupData, setBackupData] = useState('');

  useEffect(() => {
    setPreferences(PreferencesService.getPreferences());
  }, []);

  const updatePreference = (key: keyof UserPreferences, value: any) => {
    const updated = { ...preferences, [key]: value };
    setPreferences(updated);
    PreferencesService.setPreferences({ [key]: value });
  };

  const handleBackup = () => {
    const backup = PreferencesService.exportPreferences();
    setBackupData(backup);
    setShowBackup(true);
  };

  const handleRestore = () => {
    const input = prompt('الصق بيانات النسخ الاحتياطي:');
    if (input) {
      const success = PreferencesService.importPreferences(input);
      if (success) {
        setPreferences(PreferencesService.getPreferences());
        alert('تم استعادة الإعدادات بنجاح');
      } else {
        alert('فشل في استعادة الإعدادات');
      }
    }
  };

  const handleReset = () => {
    if (confirm('هل أنت متأكد من إعادة تعيين جميع الإعدادات؟')) {
      PreferencesService.resetToDefaults();
      setPreferences(PreferencesService.getPreferences());
      alert('تم إعادة تعيين الإعدادات');
    }
  };

  return (
    <div className="min-h-screen bg-background-primary">
      {/* Header */}
      <div className="bg-gradient-to-r from-primary-main to-primary-light text-white py-6 px-6">
        <div className="max-w-4xl mx-auto flex items-center gap-4">
          <button
            onClick={() => router.back()}
            className="text-white hover:text-white/80"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
          </button>
          <h1 className="text-3xl font-bold">الإعدادات</h1>
        </div>
      </div>

      {/* Content */}
      <div className="max-w-4xl mx-auto px-6 py-8 space-y-8">
        {/* Prayer Settings */}
        <section>
          <h2 className="text-2xl font-bold text-text-primary mb-4">
            إعدادات الصلاة
          </h2>
          <IslamicCard>
            <div className="space-y-2">
              <label className="block text-sm font-semibold text-text-primary">
                المذهب الفقهي
              </label>
              <select
                value={preferences.madhab || ''}
                onChange={(e) => updatePreference('madhab', e.target.value)}
                className="w-full px-4 py-3 rounded-lg border border-gray-300 focus:border-primary-main focus:ring-2 focus:ring-primary-main/20 outline-none"
              >
                <option value="">اختر المذهب</option>
                {madhabs.map((madhab) => (
                  <option key={madhab.id} value={madhab.id}>
                    {madhab.name}
                  </option>
                ))}
              </select>
            </div>
          </IslamicCard>
        </section>

        {/* Display Settings */}
        <section>
          <h2 className="text-2xl font-bold text-text-primary mb-4">
            إعدادات العرض
          </h2>
          <div className="space-y-4">
            <IslamicCard>
              <div className="space-y-2">
                <label className="block text-sm font-semibold text-text-primary">
                  الثيم
                </label>
                <div className="grid grid-cols-3 gap-3">
                  {themes.map((theme) => (
                    <button
                      key={theme.id}
                      onClick={() => updatePreference('theme', theme.id)}
                      className={`p-4 rounded-lg border-2 transition-all ${
                        preferences.theme === theme.id
                          ? 'border-primary-main bg-primary-main/5'
                          : 'border-gray-200 hover:border-primary-main/50'
                      }`}
                    >
                      <div className="text-3xl mb-2">{theme.icon}</div>
                      <div className="text-sm font-semibold">{theme.name}</div>
                    </button>
                  ))}
                </div>
              </div>
            </IslamicCard>

            <IslamicCard>
              <div className="space-y-2">
                <label className="block text-sm font-semibold text-text-primary">
                  حجم الخط
                </label>
                <div className="flex gap-2">
                  {fontSizes.map((fontSize) => (
                    <button
                      key={fontSize.id}
                      onClick={() => updatePreference('fontSize', fontSize.id)}
                      className={`flex-1 px-4 py-3 rounded-lg font-semibold transition-all ${
                        preferences.fontSize === fontSize.id
                          ? 'bg-primary-main text-white'
                          : 'bg-background-secondary text-text-secondary hover:bg-background-secondary/80'
                      }`}
                    >
                      {fontSize.name}
                    </button>
                  ))}
                </div>
              </div>
            </IslamicCard>

            <IslamicCard>
              <div className="flex items-center justify-between">
                <div>
                  <h3 className="font-semibold text-text-primary">
                    الحركات والتأثيرات
                  </h3>
                  <p className="text-sm text-text-secondary">
                    تفعيل الرسوم المتحركة
                  </p>
                </div>
                <button
                  onClick={() =>
                    updatePreference('enableAnimations', !preferences.enableAnimations)
                  }
                  className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                    preferences.enableAnimations ? 'bg-primary-main' : 'bg-gray-300'
                  }`}
                >
                  <span
                    className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                      preferences.enableAnimations ? 'translate-x-1' : 'translate-x-6'
                    }`}
                  />
                </button>
              </div>
            </IslamicCard>
          </div>
        </section>

        {/* Notification Settings */}
        <section>
          <h2 className="text-2xl font-bold text-text-primary mb-4">
            إعدادات الإشعارات
          </h2>
          <IslamicCard>
            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-semibold text-text-primary">
                  تنبيهات الصلاة
                </h3>
                <p className="text-sm text-text-secondary">
                  تلقي تنبيهات مواقيت الصلاة
                </p>
              </div>
              <button
                onClick={() =>
                  updatePreference('enableNotifications', !preferences.enableNotifications)
                }
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  preferences.enableNotifications ? 'bg-primary-main' : 'bg-gray-300'
                }`}
              >
                <span
                  className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    preferences.enableNotifications ? 'translate-x-1' : 'translate-x-6'
                  }`}
                />
              </button>
            </div>
          </IslamicCard>
        </section>

        {/* Data Management */}
        <section>
          <h2 className="text-2xl font-bold text-text-primary mb-4">
            إدارة البيانات
          </h2>
          <div className="space-y-3">
            <IslamicCard
              onClick={handleBackup}
              className="cursor-pointer hover:shadow-islamic-lg transition-all"
            >
              <div className="flex items-center gap-4">
                <div className="text-2xl">💾</div>
                <div className="flex-1">
                  <h3 className="font-semibold text-text-primary">
                    نسخ احتياطي للإعدادات
                  </h3>
                  <p className="text-sm text-text-secondary">
                    احفظ نسخة من إعداداتك
                  </p>
                </div>
                <svg className="w-5 h-5 text-text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
                </svg>
              </div>
            </IslamicCard>

            <IslamicCard
              onClick={handleRestore}
              className="cursor-pointer hover:shadow-islamic-lg transition-all"
            >
              <div className="flex items-center gap-4">
                <div className="text-2xl">📥</div>
                <div className="flex-1">
                  <h3 className="font-semibold text-text-primary">
                    استعادة الإعدادات
                  </h3>
                  <p className="text-sm text-text-secondary">
                    استرجع نسخة احتياطية سابقة
                  </p>
                </div>
                <svg className="w-5 h-5 text-text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
                </svg>
              </div>
            </IslamicCard>

            <IslamicCard
              onClick={handleReset}
              className="cursor-pointer hover:shadow-islamic-lg transition-all"
            >
              <div className="flex items-center gap-4">
                <div className="text-2xl text-status-error">🔄</div>
                <div className="flex-1">
                  <h3 className="font-semibold text-status-error">
                    إعادة تعيين الإعدادات
                  </h3>
                  <p className="text-sm text-text-secondary">
                    استرجع الإعدادات الافتراضية
                  </p>
                </div>
                <svg className="w-5 h-5 text-text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
                </svg>
              </div>
            </IslamicCard>
          </div>
        </section>
      </div>

      {/* Backup Modal */}
      {showBackup && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
          <div className="bg-white rounded-xl p-6 max-w-2xl w-full max-h-[80vh] overflow-auto">
            <h3 className="text-2xl font-bold text-text-primary mb-4">
              نسخ احتياطي للإعدادات
            </h3>
            <p className="text-text-secondary mb-4">
              احفظ هذا النص في مكان آمن لاستعادة إعداداتك لاحقاً
            </p>
            <textarea
              value={backupData}
              readOnly
              className="w-full h-64 p-4 border border-gray-300 rounded-lg font-mono text-sm"
            />
            <div className="flex gap-3 mt-4">
              <button
                onClick={() => {
                  navigator.clipboard.writeText(backupData);
                  alert('تم نسخ البيانات');
                }}
                className="flex-1 bg-primary-main text-white py-3 rounded-lg font-semibold hover:bg-primary-light transition-colors"
              >
                نسخ
              </button>
              <button
                onClick={() => setShowBackup(false)}
                className="flex-1 bg-background-secondary text-text-primary py-3 rounded-lg font-semibold hover:bg-background-secondary/80 transition-colors"
              >
                إغلاق
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

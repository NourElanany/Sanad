'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { IslamicButton } from '@/components/ui/IslamicButton';
import { IslamicCard } from '@/components/ui/IslamicCard';
import { PreferencesService } from '@/lib/services/preferences-service';

interface Madhab {
  id: string;
  name: string;
  description: string;
}

const madhabs: Madhab[] = [
  {
    id: 'hanafi',
    name: 'الحنفي',
    description: 'مذهب الإمام أبي حنيفة النعمان',
  },
  {
    id: 'maliki',
    name: 'المالكي',
    description: 'مذهب الإمام مالك بن أنس',
  },
  {
    id: 'shafii',
    name: 'الشافعي',
    description: 'مذهب الإمام محمد بن إدريس الشافعي',
  },
  {
    id: 'hanbali',
    name: 'الحنبلي',
    description: 'مذهب الإمام أحمد بن حنبل',
  },
  {
    id: 'jafari',
    name: 'الجعفري',
    description: 'مذهب الإمام جعفر الصادق',
  },
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

export default function PreferencesPage() {
  const router = useRouter();
  const [selectedMadhab, setSelectedMadhab] = useState<string>('');
  const [selectedTheme, setSelectedTheme] = useState<string>('light');
  const [selectedFontSize, setSelectedFontSize] = useState<string>('medium');
  const [enableNotifications, setEnableNotifications] = useState(true);
  const [enableAnimations, setEnableAnimations] = useState(true);

  const handleFinish = () => {
    if (!selectedMadhab) {
      alert('الرجاء اختيار المذهب الفقهي');
      return;
    }

    // Save preferences using PreferencesService
    PreferencesService.setPreferences({
      madhab: selectedMadhab,
      theme: selectedTheme as 'light' | 'dark' | 'auto',
      fontSize: selectedFontSize as 'small' | 'medium' | 'large' | 'xlarge',
      enableNotifications: enableNotifications,
      enableAnimations: enableAnimations,
      onboardingCompleted: true,
    });

    // Navigate to home
    router.push('/');
  };

  const requestNotificationPermission = async () => {
    if ('Notification' in window) {
      const permission = await Notification.requestPermission();
      setEnableNotifications(permission === 'granted');
    }
  };

  return (
    <div className="min-h-screen bg-background-primary">
      {/* Header */}
      <div className="bg-gradient-to-r from-primary-main to-primary-light text-white py-8 px-6">
        <div className="max-w-4xl mx-auto">
          <h1 className="text-3xl font-bold text-center mb-2">
            تخصيص التطبيق
          </h1>
          <p className="text-white/90 text-center">
            اختر الإعدادات المناسبة لك
          </p>
        </div>
      </div>

      {/* Content */}
      <div className="max-w-4xl mx-auto px-6 py-8 space-y-8">
        {/* Madhab Selection */}
        <section>
          <h2 className="text-2xl font-bold text-text-primary mb-4">
            المذهب الفقهي
          </h2>
          <p className="text-text-secondary mb-6">
            سيتم استخدامه لحساب مواقيت الصلاة والفتاوى
          </p>
          <div className="space-y-3">
            {madhabs.map((madhab) => (
              <IslamicCard
                key={madhab.id}
                onClick={() => setSelectedMadhab(madhab.id)}
                className={`cursor-pointer transition-all ${
                  selectedMadhab === madhab.id
                    ? 'ring-2 ring-primary-main'
                    : 'hover:shadow-lg'
                }`}
              >
                <div className="flex items-center gap-4">
                  <div
                    className={`w-6 h-6 rounded-full border-2 flex items-center justify-center ${
                      selectedMadhab === madhab.id
                        ? 'border-primary-main bg-primary-main'
                        : 'border-text-secondary'
                    }`}
                  >
                    {selectedMadhab === madhab.id && (
                      <svg
                        className="w-4 h-4 text-white"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                      >
                        <path
                          fillRule="evenodd"
                          d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                          clipRule="evenodd"
                        />
                      </svg>
                    )}
                  </div>
                  <div className="flex-1">
                    <h3 className="font-semibold text-text-primary">
                      {madhab.name}
                    </h3>
                    <p className="text-sm text-text-secondary">
                      {madhab.description}
                    </p>
                  </div>
                </div>
              </IslamicCard>
            ))}
          </div>
        </section>

        {/* Theme Selection */}
        <section>
          <h2 className="text-2xl font-bold text-text-primary mb-4">
            الثيم
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {themes.map((theme) => (
              <IslamicCard
                key={theme.id}
                onClick={() => setSelectedTheme(theme.id)}
                className={`cursor-pointer transition-all ${
                  selectedTheme === theme.id
                    ? 'ring-2 ring-primary-main'
                    : 'hover:shadow-lg'
                }`}
              >
                <div className="text-center">
                  <div className="text-4xl mb-2">{theme.icon}</div>
                  <h3 className="font-semibold text-text-primary">
                    {theme.name}
                  </h3>
                </div>
              </IslamicCard>
            ))}
          </div>
        </section>

        {/* Font Size Selection */}
        <section>
          <h2 className="text-2xl font-bold text-text-primary mb-4">
            حجم الخط
          </h2>
          <IslamicCard>
            <div className="flex justify-around gap-2">
              {fontSizes.map((fontSize) => (
                <button
                  key={fontSize.id}
                  onClick={() => setSelectedFontSize(fontSize.id)}
                  className={`px-6 py-3 rounded-lg font-semibold transition-all ${
                    selectedFontSize === fontSize.id
                      ? 'bg-primary-main text-white'
                      : 'bg-background-secondary text-text-secondary hover:bg-background-secondary/80'
                  }`}
                >
                  {fontSize.name}
                </button>
              ))}
            </div>
          </IslamicCard>
        </section>

        {/* Additional Settings */}
        <section>
          <h2 className="text-2xl font-bold text-text-primary mb-4">
            إعدادات إضافية
          </h2>
          <div className="space-y-3">
            <IslamicCard>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-4">
                  <div className="text-2xl">🔔</div>
                  <div>
                    <h3 className="font-semibold text-text-primary">
                      الإشعارات
                    </h3>
                    <p className="text-sm text-text-secondary">
                      تلقي تنبيهات مواقيت الصلاة
                    </p>
                  </div>
                </div>
                <button
                  onClick={requestNotificationPermission}
                  className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                    enableNotifications ? 'bg-primary-main' : 'bg-gray-300'
                  }`}
                >
                  <span
                    className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                      enableNotifications ? 'translate-x-1' : 'translate-x-6'
                    }`}
                  />
                </button>
              </div>
            </IslamicCard>

            <IslamicCard>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-4">
                  <div className="text-2xl">✨</div>
                  <div>
                    <h3 className="font-semibold text-text-primary">
                      الحركات والتأثيرات
                    </h3>
                    <p className="text-sm text-text-secondary">
                      تفعيل الرسوم المتحركة
                    </p>
                  </div>
                </div>
                <button
                  onClick={() => setEnableAnimations(!enableAnimations)}
                  className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                    enableAnimations ? 'bg-primary-main' : 'bg-gray-300'
                  }`}
                >
                  <span
                    className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                      enableAnimations ? 'translate-x-1' : 'translate-x-6'
                    }`}
                  />
                </button>
              </div>
            </IslamicCard>
          </div>
        </section>

        {/* Finish Button */}
        <div className="pt-4">
          <button
            onClick={handleFinish}
            className="w-full bg-primary-main text-white font-bold py-4 px-8 rounded-xl hover:bg-primary-light transition-all duration-200 shadow-lg hover:shadow-xl transform hover:scale-105"
          >
            ابدأ الاستخدام
          </button>
        </div>
      </div>
    </div>
  );
}

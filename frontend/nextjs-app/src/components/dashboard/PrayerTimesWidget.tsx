'use client';

import { useEffect, useState } from 'react';
import { PrayerTimes, PrayerTimesService } from '@/lib/services/prayer-times-service';

interface PrayerTimesWidgetProps {
  prayerTimes: PrayerTimes;
  onTap?: () => void;
}

export function PrayerTimesWidget({ prayerTimes, onTap }: PrayerTimesWidgetProps) {
  const [timeRemaining, setTimeRemaining] = useState({ hours: 0, minutes: 0, seconds: 0 });
  const [nextPrayer, setNextPrayer] = useState({ name: '', time: '' });
  const [notificationsEnabled, setNotificationsEnabled] = useState(false);
  const [isExpanded, setIsExpanded] = useState(false);

  useEffect(() => {
    const updateCountdown = () => {
      const next = PrayerTimesService.getNextPrayer(prayerTimes);
      setNextPrayer({ name: next.name, time: next.time });
      setTimeRemaining(next.timeRemaining);
    };

    updateCountdown();
    const interval = setInterval(updateCountdown, 1000);

    // Check notification permission
    if ('Notification' in window) {
      setNotificationsEnabled(Notification.permission === 'granted');
    }

    return () => clearInterval(interval);
  }, [prayerTimes]);

  const toggleNotifications = async () => {
    if (!('Notification' in window)) {
      alert('المتصفح لا يدعم الإشعارات');
      return;
    }

    if (notificationsEnabled) {
      setNotificationsEnabled(false);
      // TODO: Disable notifications
    } else {
      const permission = await Notification.requestPermission();
      if (permission === 'granted') {
        setNotificationsEnabled(true);
        // TODO: Schedule prayer notifications
        new Notification('تم تفعيل تنبيهات الصلاة', {
          body: 'سيتم إرسال تنبيهات عند حلول أوقات الصلاة',
          icon: '/icons/mosque.png',
        });
      }
    }
  };

  const prayers = [
    { name: 'الفجر', time: prayerTimes.fajr, icon: '🌅' },
    { name: 'الشروق', time: prayerTimes.sunrise, icon: '☀️' },
    { name: 'الظهر', time: prayerTimes.dhuhr, icon: '🌞' },
    { name: 'العصر', time: prayerTimes.asr, icon: '🌤️' },
    { name: 'المغرب', time: prayerTimes.maghrib, icon: '🌆' },
    { name: 'العشاء', time: prayerTimes.isha, icon: '🌙' },
  ];

  return (
    <div
      className="bg-white rounded-2xl shadow-lg border border-primary/10 p-6 cursor-pointer hover:shadow-xl transition-all"
      onClick={onTap}
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <div className="bg-gradient-to-br from-primary to-primary/70 p-3 rounded-xl">
            <span className="text-2xl">🕌</span>
          </div>
          <div>
            <p className="text-sm text-gray-600">الصلاة القادمة</p>
            <h3 className="text-xl font-bold text-primary">{nextPrayer.name}</h3>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <button
            onClick={(e) => {
              e.stopPropagation();
              toggleNotifications();
            }}
            className="p-2 rounded-lg hover:bg-gray-100 transition-colors"
            title={notificationsEnabled ? 'إيقاف التنبيهات' : 'تفعيل التنبيهات'}
          >
            <span className="text-2xl">
              {notificationsEnabled ? '🔔' : '🔕'}
            </span>
          </button>
          <div className="text-2xl font-bold text-accent">
            {nextPrayer.time}
          </div>
        </div>
      </div>

      {/* Countdown */}
      <div className="bg-gradient-to-br from-primary/10 to-secondary/10 rounded-xl p-4 mb-4 border border-primary/20">
        <div className="flex items-center justify-center gap-2">
          <span className="text-lg">⏰</span>
          <span className="text-gray-700">باقي</span>
          <span className="text-2xl font-bold text-primary font-mono">
            {String(timeRemaining.hours).padStart(2, '0')}:
            {String(timeRemaining.minutes).padStart(2, '0')}:
            {String(timeRemaining.seconds).padStart(2, '0')}
          </span>
        </div>
      </div>

      {/* Expand/Collapse button */}
      <button
        onClick={(e) => {
          e.stopPropagation();
          setIsExpanded(!isExpanded);
        }}
        className="w-full flex items-center justify-center gap-2 py-2 text-primary font-semibold hover:bg-primary/5 rounded-lg transition-colors"
      >
        <span className="text-sm">
          {isExpanded ? 'إخفاء المواقيت' : 'عرض جميع المواقيت'}
        </span>
        <span className="text-lg">
          {isExpanded ? '▲' : '▼'}
        </span>
      </button>

      {/* All Prayer Times (expandable) */}
      {isExpanded && (
        <div className="mt-4 space-y-2 animate-fadeIn">
          {prayers.map((prayer) => {
            const isNext = prayer.name === nextPrayer.name;
            return (
              <div
                key={prayer.name}
                className={`flex items-center justify-between p-3 rounded-lg transition-colors ${
                  isNext
                    ? 'bg-accent/10 border border-accent/30'
                    : 'bg-gray-50'
                }`}
              >
                <div className="flex items-center gap-3">
                  <span className="text-xl">{prayer.icon}</span>
                  <span
                    className={`font-medium ${
                      isNext ? 'text-accent font-bold' : 'text-gray-900'
                    }`}
                  >
                    {prayer.name}
                  </span>
                </div>
                <span
                  className={`font-semibold font-mono ${
                    isNext ? 'text-accent' : 'text-gray-900'
                  }`}
                >
                  {prayer.time}
                </span>
              </div>
            );
          })}
        </div>
      )}

      {/* Location */}
      <div className="flex items-center gap-1 text-sm text-gray-500 mt-3">
        <span>📍</span>
        <span>{prayerTimes.location}</span>
      </div>
    </div>
  );
}

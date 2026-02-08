'use client';

import { useEffect, useState } from 'react';
import { PrayerTimes, PrayerTimesService } from '@/lib/services/prayer-times-service';

interface PrayerTimesCardProps {
  prayerTimes: PrayerTimes;
}

export function PrayerTimesCard({ prayerTimes }: PrayerTimesCardProps) {
  const [timeRemaining, setTimeRemaining] = useState({ hours: 0, minutes: 0, seconds: 0 });
  const [nextPrayer, setNextPrayer] = useState({ name: '', time: '' });

  useEffect(() => {
    const updateCountdown = () => {
      const next = PrayerTimesService.getNextPrayer(prayerTimes);
      setNextPrayer({ name: next.name, time: next.time });
      setTimeRemaining(next.timeRemaining);
    };

    updateCountdown();
    const interval = setInterval(updateCountdown, 1000);

    return () => clearInterval(interval);
  }, [prayerTimes]);

  const prayers = [
    { name: 'الفجر', time: prayerTimes.fajr, icon: '🌅' },
    { name: 'الشروق', time: prayerTimes.sunrise, icon: '☀️' },
    { name: 'الظهر', time: prayerTimes.dhuhr, icon: '🌞' },
    { name: 'العصر', time: prayerTimes.asr, icon: '🌤️' },
    { name: 'المغرب', time: prayerTimes.maghrib, icon: '🌆' },
    { name: 'العشاء', time: prayerTimes.isha, icon: '🌙' },
  ];

  return (
    <div className="bg-white rounded-2xl shadow-lg border border-primary/10 p-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <div className="bg-primary/10 p-3 rounded-xl">
            <span className="text-2xl">🕌</span>
          </div>
          <div>
            <p className="text-sm text-gray-600">الصلاة القادمة</p>
            <h3 className="text-xl font-bold text-primary">{nextPrayer.name}</h3>
          </div>
        </div>
        <div className="text-2xl font-bold text-accent">
          {nextPrayer.time}
        </div>
      </div>

      {/* Countdown */}
      <div className="bg-gradient-to-br from-primary/10 to-secondary/10 rounded-xl p-4 mb-4">
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

      {/* All Prayer Times */}
      <div className="grid grid-cols-6 gap-2 mb-3">
        {prayers.map((prayer) => (
          <div key={prayer.name} className="text-center">
            <div className="text-xl mb-1">{prayer.icon}</div>
            <p className="text-xs text-gray-600 mb-1">{prayer.name}</p>
            <p className="text-sm font-semibold text-gray-900 font-mono">
              {prayer.time}
            </p>
          </div>
        ))}
      </div>

      {/* Location */}
      <div className="flex items-center gap-1 text-sm text-gray-500">
        <span>📍</span>
        <span>{prayerTimes.location}</span>
      </div>
    </div>
  );
}

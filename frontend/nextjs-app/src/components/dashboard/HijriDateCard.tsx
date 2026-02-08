'use client';

import { HijriDate, PrayerTimesService } from '@/lib/services/prayer-times-service';

interface HijriDateCardProps {
  hijriDate: HijriDate;
}

export function HijriDateCard({ hijriDate }: HijriDateCardProps) {
  const getGregorianDate = () => {
    const now = new Date();
    const options: Intl.DateTimeFormatOptions = {
      weekday: 'long',
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    };
    return new Intl.DateTimeFormat('ar-SA', options).format(now);
  };

  return (
    <div className="bg-white rounded-2xl shadow-lg border border-primary/10 p-6">
      <div className="flex items-center gap-4">
        {/* Calendar Icon */}
        <div className="bg-gradient-to-br from-accent/20 to-accent/10 p-4 rounded-xl">
          <span className="text-3xl">📅</span>
        </div>

        {/* Dates */}
        <div className="flex-1">
          {/* Hijri Date */}
          <p className="text-lg font-bold text-primary mb-1">
            {PrayerTimesService.formatHijriDate(hijriDate)}
          </p>

          {/* Gregorian Date */}
          <p className="text-sm text-gray-600">
            {getGregorianDate()} م
          </p>
        </div>
      </div>
    </div>
  );
}

'use client';

import { useState, useEffect } from 'react';
import { PrayerTimesService, PrayerTimes, HijriDate, NextPrayer } from '@/lib/services/prayer-times-service';

export default function PrayerTimesClient() {
  const [prayerTimes, setPrayerTimes] = useState<PrayerTimes | null>(null);
  const [hijriDate, setHijriDate] = useState<HijriDate | null>(null);
  const [nextPrayer, setNextPrayer] = useState<NextPrayer | null>(null);
  const [monthlyTimes, setMonthlyTimes] = useState<PrayerTimes[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [location, setLocation] = useState<{ latitude: number; longitude: number } | null>(null);
  const [showMonthly, setShowMonthly] = useState(false);

  useEffect(() => {
    loadPrayerTimes();
    loadHijriDate();
  }, []);

  useEffect(() => {
    if (prayerTimes) {
      const next = PrayerTimesService.getNextPrayer(prayerTimes);
      setNextPrayer(next);

      // Update countdown every second
      const interval = setInterval(() => {
        const updated = PrayerTimesService.getNextPrayer(prayerTimes);
        setNextPrayer(updated);
      }, 1000);

      return () => clearInterval(interval);
    }
  }, [prayerTimes]);

  const loadPrayerTimes = async () => {
    setIsLoading(true);
    setError(null);

    try {
      // Get user location
      const position = await new Promise<GeolocationPosition>((resolve, reject) => {
        navigator.geolocation.getCurrentPosition(resolve, reject);
      });

      const { latitude, longitude } = position.coords;
      setLocation({ latitude, longitude });

      // Get prayer times
      const times = await PrayerTimesService.getPrayerTimes(latitude, longitude);
      setPrayerTimes(times);
    } catch (err) {
      setError('فشل تحميل مواقيت الصلاة. يرجى السماح بالوصول إلى الموقع.');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const loadHijriDate = async () => {
    try {
      const date = await PrayerTimesService.getHijriDate();
      setHijriDate(date);
    } catch (err) {
      console.error('Failed to load Hijri date:', err);
    }
  };

  const loadMonthlyTimes = async () => {
    if (!location) return;

    try {
      const now = new Date();
      const times = await PrayerTimesService.getMonthlyPrayerTimes(
        location.latitude,
        location.longitude,
        now.getMonth() + 1,
        now.getFullYear()
      );
      setMonthlyTimes(times);
      setShowMonthly(true);
    } catch (err) {
      console.error('Failed to load monthly times:', err);
    }
  };

  const getPrayerIcon = (prayerName: string) => {
    switch (prayerName) {
      case 'الفجر':
        return '🌅';
      case 'الشروق':
        return '☀️';
      case 'الظهر':
        return '🌞';
      case 'العصر':
        return '🌤️';
      case 'المغرب':
        return '🌆';
      case 'العشاء':
        return '🌙';
      default:
        return '🕌';
    }
  };

  if (isLoading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-16 w-16 border-b-2 border-[#1B365D] mx-auto mb-4"></div>
          <p className="text-gray-600">جاري تحميل مواقيت الصلاة...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center max-w-md mx-auto p-6">
          <div className="text-red-500 text-6xl mb-4">⚠️</div>
          <h2 className="text-2xl font-bold text-gray-800 mb-2">حدث خطأ</h2>
          <p className="text-gray-600 mb-6">{error}</p>
          <button
            onClick={loadPrayerTimes}
            className="bg-[#1B365D] text-white px-6 py-3 rounded-lg hover:bg-[#2E4A6B] transition-colors"
          >
            إعادة المحاولة
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-b from-[#1B365D] to-[#2E4A6B]" dir="rtl">
      {/* Header */}
      <header className="bg-[#1B365D] text-white shadow-lg">
        <div className="container mx-auto px-4 py-6">
          <h1 className="text-3xl font-bold text-center mb-4 font-['Tajawal']">
            مواقيت الصلاة
          </h1>
          {hijriDate && (
            <p className="text-center text-lg opacity-90">
              {PrayerTimesService.formatHijriDate(hijriDate)}
            </p>
          )}
          {prayerTimes && (
            <p className="text-center text-sm opacity-75 mt-2">
              📍 {prayerTimes.location}
            </p>
          )}
        </div>
      </header>

      <main className="container mx-auto px-4 py-8">
        {/* Next Prayer Card */}
        {nextPrayer && (
          <div className="bg-white rounded-2xl shadow-2xl p-8 mb-8 text-center">
            <h2 className="text-2xl font-bold text-gray-800 mb-4 font-['Tajawal']">
              الصلاة القادمة
            </h2>
            <div className="text-6xl mb-4">{getPrayerIcon(nextPrayer.name)}</div>
            <h3 className="text-4xl font-bold text-[#1B365D] mb-4">
              {nextPrayer.name}
            </h3>
            <p className="text-3xl text-gray-700 mb-6">{nextPrayer.time}</p>
            <div className="flex justify-center gap-4 text-2xl font-bold text-[#B8860B]">
              <div className="flex flex-col items-center">
                <span className="text-4xl">{String(nextPrayer.timeRemaining.hours).padStart(2, '0')}</span>
                <span className="text-sm text-gray-600">ساعة</span>
              </div>
              <span className="text-4xl">:</span>
              <div className="flex flex-col items-center">
                <span className="text-4xl">{String(nextPrayer.timeRemaining.minutes).padStart(2, '0')}</span>
                <span className="text-sm text-gray-600">دقيقة</span>
              </div>
              <span className="text-4xl">:</span>
              <div className="flex flex-col items-center">
                <span className="text-4xl">{String(nextPrayer.timeRemaining.seconds).padStart(2, '0')}</span>
                <span className="text-sm text-gray-600">ثانية</span>
              </div>
            </div>
          </div>
        )}

        {/* Today's Prayer Times */}
        {prayerTimes && (
          <div className="bg-white rounded-2xl shadow-xl p-6 mb-8">
            <h2 className="text-2xl font-bold text-gray-800 mb-6 text-center font-['Tajawal']">
              مواقيت اليوم
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
              {[
                { name: 'الفجر', time: prayerTimes.fajr },
                { name: 'الشروق', time: prayerTimes.sunrise },
                { name: 'الظهر', time: prayerTimes.dhuhr },
                { name: 'العصر', time: prayerTimes.asr },
                { name: 'المغرب', time: prayerTimes.maghrib },
                { name: 'العشاء', time: prayerTimes.isha },
              ].map((prayer) => (
                <div
                  key={prayer.name}
                  className={`p-4 rounded-lg text-center transition-all ${
                    nextPrayer?.name === prayer.name
                      ? 'bg-[#1B365D] text-white shadow-lg scale-105'
                      : 'bg-gray-50 hover:bg-gray-100'
                  }`}
                >
                  <div className="text-3xl mb-2">{getPrayerIcon(prayer.name)}</div>
                  <h3 className="font-bold text-lg mb-1">{prayer.name}</h3>
                  <p className="text-xl font-semibold">{prayer.time}</p>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Monthly Calendar Button */}
        <div className="text-center mb-8">
          <button
            onClick={loadMonthlyTimes}
            className="bg-white text-[#1B365D] px-8 py-4 rounded-lg font-bold text-lg hover:bg-gray-100 transition-colors shadow-lg"
          >
            📅 عرض التقويم الشهري
          </button>
        </div>

        {/* Monthly Calendar */}
        {showMonthly && monthlyTimes.length > 0 && (
          <div className="bg-white rounded-2xl shadow-xl p-6">
            <h2 className="text-2xl font-bold text-gray-800 mb-6 text-center font-['Tajawal']">
              التقويم الشهري
            </h2>
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="bg-[#1B365D] text-white">
                    <th className="p-3 text-right">التاريخ</th>
                    <th className="p-3">الفجر</th>
                    <th className="p-3">الشروق</th>
                    <th className="p-3">الظهر</th>
                    <th className="p-3">العصر</th>
                    <th className="p-3">المغرب</th>
                    <th className="p-3">العشاء</th>
                  </tr>
                </thead>
                <tbody>
                  {monthlyTimes.map((day, index) => (
                    <tr
                      key={index}
                      className={`border-b ${
                        index % 2 === 0 ? 'bg-gray-50' : 'bg-white'
                      } hover:bg-blue-50`}
                    >
                      <td className="p-3 font-semibold">{day.date}</td>
                      <td className="p-3 text-center">{day.fajr}</td>
                      <td className="p-3 text-center">{day.sunrise}</td>
                      <td className="p-3 text-center">{day.dhuhr}</td>
                      <td className="p-3 text-center">{day.asr}</td>
                      <td className="p-3 text-center">{day.maghrib}</td>
                      <td className="p-3 text-center">{day.isha}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}
      </main>
    </div>
  );
}

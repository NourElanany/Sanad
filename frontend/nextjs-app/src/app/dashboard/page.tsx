'use client';

import { useEffect, useState } from 'react';
import { PrayerTimesCard } from '@/components/dashboard/PrayerTimesCard';
import { HijriDateCard } from '@/components/dashboard/HijriDateCard';
import { DailyWirdCard } from '@/components/dashboard/DailyWirdCard';
import { DailyContentCard } from '@/components/dashboard/DailyContentCard';
import { QuickActionsCard } from '@/components/dashboard/QuickActionsCard';
import { PrayerTimesService, PrayerTimes, HijriDate } from '@/lib/services/prayer-times-service';
import { DashboardService, DashboardData } from '@/lib/services/dashboard-service';
import { IslamicLoading } from '@/components/ui/IslamicLoading';

export default function DashboardPage() {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [prayerTimes, setPrayerTimes] = useState<PrayerTimes | null>(null);
  const [hijriDate, setHijriDate] = useState<HijriDate | null>(null);
  const [dashboardData, setDashboardData] = useState<DashboardData | null>(null);

  useEffect(() => {
    loadDashboardData();
  }, []);

  const loadDashboardData = async () => {
    setLoading(true);
    setError(null);

    try {
      // Get user location (default to Riyadh for now)
      const latitude = 24.7136;
      const longitude = 46.6753;

      // Load all data in parallel
      const [prayerTimesData, hijriDateData, dashboardDataResult] = await Promise.all([
        PrayerTimesService.getPrayerTimes(latitude, longitude),
        PrayerTimesService.getHijriDate(),
        DashboardService.getDashboardData(),
      ]);

      setPrayerTimes(prayerTimesData);
      setHijriDate(hijriDateData);
      setDashboardData(dashboardDataResult);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'حدث خطأ في تحميل البيانات');
    } finally {
      setLoading(false);
    }
  };

  const quickActions = [
    {
      title: 'المساعد الذكي',
      icon: '🤖',
      color: '#1B365D',
      onTap: () => {
        // TODO: Navigate to AI assistant
        console.log('Navigate to AI assistant');
      },
    },
    {
      title: 'القبلة',
      icon: '🧭',
      color: '#2D5A27',
      onTap: () => {
        // TODO: Navigate to Qibla compass
        console.log('Navigate to Qibla');
      },
    },
    {
      title: 'الأذكار',
      icon: '📿',
      color: '#B8860B',
      onTap: () => {
        // TODO: Navigate to Adhkar
        console.log('Navigate to Adhkar');
      },
    },
  ];

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <IslamicLoading />
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
        <div className="text-center">
          <div className="text-6xl mb-4">⚠️</div>
          <h2 className="text-2xl font-bold text-gray-800 mb-2">
            حدث خطأ في تحميل البيانات
          </h2>
          <p className="text-gray-600 mb-6">{error}</p>
          <button
            onClick={loadDashboardData}
            className="px-6 py-3 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors"
          >
            إعادة المحاولة
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow-sm sticky top-0 z-10">
        <div className="max-w-7xl mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <button className="text-primary">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
                </svg>
              </button>
              <h1 className="text-xl font-bold text-primary">
                السلام عليكم، مستخدم
              </h1>
            </div>
            <div className="flex items-center gap-2">
              <button className="p-2 text-primary hover:bg-gray-100 rounded-lg">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                </svg>
              </button>
              <button className="p-2 text-primary hover:bg-gray-100 rounded-lg">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 py-6">
        <div className="space-y-6">
          {/* Hijri Date Card */}
          {hijriDate && <HijriDateCard hijriDate={hijriDate} />}

          {/* Prayer Times Card */}
          {prayerTimes && <PrayerTimesCard prayerTimes={prayerTimes} />}

          {/* Daily Wird Card */}
          {dashboardData?.dailyWird && (
            <DailyWirdCard
              dailyWird={dashboardData.dailyWird}
              onTap={() => console.log('Navigate to Quran reading')}
            />
          )}

          {/* Daily Content Card */}
          {dashboardData?.dailyContent && (
            <DailyContentCard
              dailyContent={dashboardData.dailyContent}
              onTap={() => console.log('Navigate to tafsir/explanation')}
            />
          )}

          {/* Quick Actions Card */}
          <QuickActionsCard actions={quickActions} />
        </div>
      </main>
    </div>
  );
}

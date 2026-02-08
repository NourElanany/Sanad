'use client';

import { PrayerTimesWidget } from '@/components/dashboard/PrayerTimesWidget';
import { KhatmaProgressWidget } from '@/components/dashboard/KhatmaProgressWidget';
import { DailyVerseWidget } from '@/components/dashboard/DailyVerseWidget';
import { WeatherWidget } from '@/components/dashboard/WeatherWidget';
import { EnhancedQuickActionsWidget } from '@/components/dashboard/EnhancedQuickActionsWidget';

export default function WidgetsDemoPage() {
  // Mock data
  const mockPrayerTimes = {
    fajr: '05:15',
    sunrise: '06:45',
    dhuhr: '12:30',
    asr: '15:45',
    maghrib: '18:15',
    isha: '19:45',
    location: 'الرياض، السعودية',
  };

  return (
    <div className="min-h-screen bg-gray-50 py-8 px-4">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-primary mb-2">
            الودجات التفاعلية
          </h1>
          <p className="text-gray-600">
            عرض توضيحي للودجات التفاعلية في تطبيق سند الإسلامي
          </p>
        </div>

        {/* Widgets Grid */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Prayer Times Widget */}
          <div>
            <h2 className="text-xl font-bold text-primary mb-3">
              1. ودجة مواقيت الصلاة مع التنبيهات
            </h2>
            <PrayerTimesWidget
              prayerTimes={mockPrayerTimes}
              onTap={() => console.log('Prayer times tapped')}
            />
          </div>

          {/* Khatma Progress Widget */}
          <div>
            <h2 className="text-xl font-bold text-primary mb-3">
              2. ودجة تقدم الختمة
            </h2>
            <KhatmaProgressWidget
              onTap={() => console.log('Khatma progress tapped')}
            />
          </div>

          {/* Daily Verse Widget */}
          <div>
            <h2 className="text-xl font-bold text-primary mb-3">
              3. ودجة آية اليوم مع التفسير
            </h2>
            <DailyVerseWidget
              onTap={() => console.log('Daily verse tapped')}
            />
          </div>

          {/* Weather Widget */}
          <div>
            <h2 className="text-xl font-bold text-primary mb-3">
              4. ودجة الطقس للصيام والصلاة
            </h2>
            <WeatherWidget
              onTap={() => console.log('Weather tapped')}
            />
          </div>

          {/* Enhanced Quick Actions Widget */}
          <div className="lg:col-span-2">
            <h2 className="text-xl font-bold text-primary mb-3">
              5. ودجة الوصول السريع المحسّنة
            </h2>
            <EnhancedQuickActionsWidget />
          </div>
        </div>

        {/* Features List */}
        <div className="mt-12 bg-white rounded-2xl shadow-lg border border-primary/10 p-8">
          <h2 className="text-2xl font-bold text-primary mb-6">
            الميزات المُنفذة
          </h2>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h3 className="text-lg font-bold text-secondary mb-3">
                ✅ ودجة مواقيت الصلاة
              </h3>
              <ul className="space-y-2 text-gray-700">
                <li>• عد تنازلي للصلاة القادمة</li>
                <li>• تفعيل/إيقاف التنبيهات</li>
                <li>• عرض جميع المواقيت (قابل للتوسيع)</li>
                <li>• تمييز الصلاة القادمة</li>
                <li>• رسوم متحركة سلسة</li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-bold text-secondary mb-3">
                ✅ ودجة تقدم الختمة
              </h3>
              <ul className="space-y-2 text-gray-700">
                <li>• مؤشر دائري للتقدم</li>
                <li>• شارة الختمات المكتملة</li>
                <li>• إحصائيات (الصفحات المتبقية، المعدل اليومي)</li>
                <li>• تاريخ الإتمام المتوقع</li>
                <li>• زر متابعة القراءة</li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-bold text-secondary mb-3">
                ✅ ودجة آية اليوم
              </h3>
              <ul className="space-y-2 text-gray-700">
                <li>• نص عربي بخط أميري</li>
                <li>• ترجمة إنجليزية</li>
                <li>• تفسير مختصر (دائم الظهور)</li>
                <li>• تفسير كامل (قابل للتوسيع)</li>
                <li>• زر المشاركة</li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-bold text-secondary mb-3">
                ✅ ودجة الطقس
              </h3>
              <ul className="space-y-2 text-gray-700">
                <li>• درجة الحرارة والحالة</li>
                <li>• الرطوبة وسرعة الرياح</li>
                <li>• أوقات الشروق والغروب</li>
                <li>• نصائح إسلامية حسب الطقس</li>
                <li>• ألوان ديناميكية</li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-bold text-secondary mb-3">
                ✅ ودجة الوصول السريع
              </h3>
              <ul className="space-y-2 text-gray-700">
                <li>• 8 أزرار إجراءات سريعة</li>
                <li>• دعم الشارات للإشعارات</li>
                <li>• خلفيات متدرجة</li>
                <li>• تخطيط شبكي (4 أعمدة)</li>
                <li>• إجراءات قابلة للتخصيص</li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-bold text-secondary mb-3">
                ✅ خدمة الإشعارات
              </h3>
              <ul className="space-y-2 text-gray-700">
                <li>• جدولة إشعارات الصلاة</li>
                <li>• معالجة الأذونات</li>
                <li>• أصوات مخصصة (الأذان)</li>
                <li>• تخزين التفضيلات</li>
                <li>• إشعارات يومية متكررة</li>
              </ul>
            </div>
          </div>
        </div>

        {/* Technical Details */}
        <div className="mt-8 bg-white rounded-2xl shadow-lg border border-primary/10 p-8">
          <h2 className="text-2xl font-bold text-primary mb-6">
            التفاصيل التقنية
          </h2>
          
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div>
              <h3 className="text-lg font-bold text-accent mb-3">
                Flutter Mobile
              </h3>
              <ul className="space-y-2 text-gray-700 text-sm">
                <li>• Riverpod لإدارة الحالة</li>
                <li>• flutter_local_notifications</li>
                <li>• AnimationController</li>
                <li>• CustomPainter للرسوم</li>
                <li>• SharedPreferences</li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-bold text-accent mb-3">
                Next.js Web
              </h3>
              <ul className="space-y-2 text-gray-700 text-sm">
                <li>• React Hooks (useState, useEffect)</li>
                <li>• Web Notifications API</li>
                <li>• Tailwind CSS</li>
                <li>• SVG للرسوم</li>
                <li>• Web Share API</li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-bold text-accent mb-3">
                نظام التصميم
              </h3>
              <ul className="space-y-2 text-gray-700 text-sm">
                <li>• ألوان إسلامية حديثة</li>
                <li>• خطوط عربية (تجوال، أميري)</li>
                <li>• دعم RTL كامل</li>
                <li>• رسوم متحركة سلسة</li>
                <li>• تصميم متجاوب</li>
              </ul>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="mt-8 text-center text-gray-600">
          <p>تم تنفيذ المهمة 4.1: تنفيذ الودجات التفاعلية ✅</p>
          <p className="text-sm mt-2">
            جميع الودجات جاهزة للتكامل مع الخدمات الخلفية
          </p>
        </div>
      </div>
    </div>
  );
}

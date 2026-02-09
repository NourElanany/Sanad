'use client';

import { useState } from 'react';
import { usePerformanceMonitoring, measureAsyncOperation } from '@/lib/hooks/usePerformanceMonitoring';
import { OptimizedImage } from '@/components/ui/OptimizedImage';
import { LazyLoadingList } from '@/components/ui/LazyLoadingList';
import { fadeInVariants, slideUpVariants, staggerContainerVariants, staggerItemVariants } from '@/lib/utils/animations';

/**
 * Performance Optimization Demo Page
 * Demonstrates all performance features and optimizations
 */
export default function PerformanceDemoPage() {
  const metrics = usePerformanceMonitoring('PerformanceDemoPage', {
    enabled: true,
    logToConsole: true,
  });

  const [operationTime, setOperationTime] = useState<number | null>(null);

  const handleMeasureOperation = async () => {
    const result = await measureAsyncOperation(
      'Heavy Operation',
      async () => {
        // Simulate heavy computation
        await new Promise(resolve => setTimeout(resolve, 500));
        
        let sum = 0;
        for (let i = 0; i < 1000000; i++) {
          sum += i;
        }
        
        return sum;
      }
    );
    
    setOperationTime(500); // Approximate time
  };

  return (
    <div className="min-h-screen bg-[#FEFEFE] p-8" dir="rtl">
      <div className="max-w-6xl mx-auto space-y-8">
        {/* Header */}
        <div className="animate-fade-in-down">
          <h1 className="text-4xl font-bold text-[#1B365D] mb-2 font-['Tajawal']">
            عرض تحسينات الأداء
          </h1>
          <p className="text-gray-600 font-['Tajawal']">
            أمثلة على جميع ميزات تحسين الأداء والرسوم المتحركة
          </p>
        </div>

        {/* Performance Metrics Card */}
        <div className="bg-white rounded-2xl shadow-lg p-6 animate-fade-in-up stagger-1">
          <h2 className="text-2xl font-bold text-[#1B365D] mb-4 font-['Tajawal']">
            مقاييس الأداء
          </h2>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
            <MetricCard label="FPS الحالي" value={metrics.fps.toFixed(1)} />
            <MetricCard label="وقت التحميل" value={`${metrics.loadTime.toFixed(0)}ms`} />
            <MetricCard label="وقت العرض" value={`${metrics.renderTime.toFixed(2)}ms`} />
            {metrics.memoryUsage && (
              <MetricCard label="استخدام الذاكرة" value={`${metrics.memoryUsage}MB`} />
            )}
            <MetricCard 
              label="الحالة" 
              value={metrics.fps >= 55 ? '✅ ممتاز' : '⚠️ يحتاج تحسين'} 
            />
          </div>
        </div>

        {/* Optimized Image Example */}
        <div className="bg-white rounded-2xl shadow-lg p-6 animate-fade-in-up stagger-2">
          <h2 className="text-2xl font-bold text-[#1B365D] mb-4 font-['Tajawal']">
            تحميل الصور المحسّن
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <OptimizedImage
                src="https://via.placeholder.com/400x300"
                alt="صورة تجريبية"
                width={400}
                height={300}
                className="rounded-lg"
              />
              <p className="text-sm text-gray-600 mt-2 font-['Tajawal']">
                صورة محسّنة مع Lazy Loading و Blur Placeholder
              </p>
            </div>
            <div>
              <OptimizedImage
                src="https://via.placeholder.com/400x300/1B365D/FFFFFF"
                alt="صورة تجريبية 2"
                width={400}
                height={300}
                className="rounded-lg"
              />
              <p className="text-sm text-gray-600 mt-2 font-['Tajawal']">
                تحميل تلقائي عند الظهور في الشاشة
              </p>
            </div>
          </div>
        </div>

        {/* Animation Examples */}
        <div className="bg-white rounded-2xl shadow-lg p-6 animate-fade-in-up stagger-3">
          <h2 className="text-2xl font-bold text-[#1B365D] mb-4 font-['Tajawal']">
            الرسوم المتحركة السلسة
          </h2>
          <div className="space-y-4">
            {[1, 2, 3, 4, 5].map((item, index) => (
              <div
                key={item}
                className={`p-4 bg-gradient-to-r from-[#1B365D]/10 to-[#2D5A27]/10 rounded-lg animate-fade-in-up stagger-${index + 1}`}
              >
                <p className="font-['Tajawal']">
                  عنصر متحرك رقم {item} - يظهر بتأخير {index * 50}ms
                </p>
              </div>
            ))}
          </div>
        </div>

        {/* Lazy Loading Example */}
        <div className="bg-white rounded-2xl shadow-lg p-6 animate-fade-in-up stagger-4">
          <h2 className="text-2xl font-bold text-[#1B365D] mb-4 font-['Tajawal']">
            التحميل التدريجي (Lazy Loading)
          </h2>
          <div className="h-96 overflow-hidden">
            <LazyLoadingList<string>
              onLoadMore={async (page, pageSize) => {
                // Simulate API call
                await new Promise(resolve => setTimeout(resolve, 500));
                return Array.from(
                  { length: pageSize },
                  (_, i) => `عنصر ${page * pageSize + i + 1}`
                );
              }}
              renderItem={(item, index) => (
                <div className="p-4 bg-white border border-[#1B365D]/20 rounded-lg hover-lift">
                  <p className="font-['Tajawal']">{item}</p>
                </div>
              )}
              pageSize={10}
              enableAnimation={true}
            />
          </div>
        </div>

        {/* Performance Measurement */}
        <div className="bg-white rounded-2xl shadow-lg p-6 animate-fade-in-up stagger-5">
          <h2 className="text-2xl font-bold text-[#1B365D] mb-4 font-['Tajawal']">
            قياس أداء العمليات
          </h2>
          <button
            onClick={handleMeasureOperation}
            className="px-6 py-3 bg-[#1B365D] text-white rounded-lg hover:bg-[#2E4A6B] smooth-transition font-['Tajawal']"
          >
            قياس عملية ثقيلة
          </button>
          {operationTime !== null && (
            <p className="mt-4 text-gray-600 font-['Tajawal']">
              وقت التنفيذ: {operationTime}ms
            </p>
          )}
        </div>

        {/* Hover Effects Demo */}
        <div className="bg-white rounded-2xl shadow-lg p-6 animate-fade-in-up stagger-6">
          <h2 className="text-2xl font-bold text-[#1B365D] mb-4 font-['Tajawal']">
            تأثيرات التفاعل
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="p-6 bg-gradient-to-br from-[#1B365D] to-[#2E4A6B] text-white rounded-lg hover-lift cursor-pointer">
              <h3 className="font-bold mb-2 font-['Tajawal']">تأثير الرفع</h3>
              <p className="text-sm font-['Tajawal']">مرر الماوس لرؤية التأثير</p>
            </div>
            <div className="p-6 bg-gradient-to-br from-[#2D5A27] to-[#4A7C59] text-white rounded-lg hover-scale cursor-pointer">
              <h3 className="font-bold mb-2 font-['Tajawal']">تأثير التكبير</h3>
              <p className="text-sm font-['Tajawal']">مرر الماوس لرؤية التأثير</p>
            </div>
            <div className="p-6 bg-gradient-to-br from-[#B8860B] to-[#DAA520] text-white rounded-lg smooth-transition hover:shadow-2xl cursor-pointer">
              <h3 className="font-bold mb-2 font-['Tajawal']">تأثير الظل</h3>
              <p className="text-sm font-['Tajawal']">مرر الماوس لرؤية التأثير</p>
            </div>
          </div>
        </div>

        {/* Loading Skeleton Demo */}
        <div className="bg-white rounded-2xl shadow-lg p-6 animate-fade-in-up stagger-7">
          <h2 className="text-2xl font-bold text-[#1B365D] mb-4 font-['Tajawal']">
            هيكل التحميل (Skeleton)
          </h2>
          <div className="space-y-4">
            <div className="skeleton h-12 w-full"></div>
            <div className="skeleton h-12 w-3/4"></div>
            <div className="skeleton h-12 w-1/2"></div>
          </div>
        </div>
      </div>
    </div>
  );
}

function MetricCard({ label, value }: { label: string; value: string }) {
  return (
    <div className="p-4 bg-gray-50 rounded-lg">
      <p className="text-sm text-gray-600 mb-1 font-['Tajawal']">{label}</p>
      <p className="text-2xl font-bold text-[#1B365D] font-['Tajawal']">{value}</p>
    </div>
  );
}

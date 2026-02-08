'use client';

import { DailyContent } from '@/lib/services/dashboard-service';

interface DailyContentCardProps {
  dailyContent: DailyContent;
  onTap?: () => void;
}

export function DailyContentCard({ dailyContent, onTap }: DailyContentCardProps) {
  const isVerse = dailyContent.type === 'verse';

  return (
    <div className="bg-white rounded-2xl shadow-lg border border-primary/10 p-6">
      {/* Header */}
      <div className="flex items-center gap-3 mb-4">
        <div
          className={`p-3 rounded-xl ${
            isVerse ? 'bg-primary/10' : 'bg-accent/10'
          }`}
        >
          <span className="text-2xl">{isVerse ? '📜' : '📿'}</span>
        </div>
        <h3 className="text-xl font-bold text-primary">
          {isVerse ? '💎 آية اليوم' : '📿 حديث اليوم'}
        </h3>
      </div>

      {/* Arabic Text */}
      <div className="bg-gray-50 rounded-xl p-4 mb-4 border border-primary/10">
        <p
          className="text-xl text-center leading-loose text-primary"
          style={{ fontFamily: 'Amiri, serif' }}
          dir="rtl"
        >
          {dailyContent.arabicText}
        </p>
      </div>

      {/* Translation */}
      {dailyContent.translation && (
        <p className="text-gray-700 text-center mb-4 leading-relaxed">
          {dailyContent.translation}
        </p>
      )}

      {/* Reference */}
      <div className="bg-accent/10 rounded-lg px-4 py-2 mb-4">
        <div className="flex items-center justify-center gap-2">
          <span>🔖</span>
          <p className="text-sm font-semibold text-accent">
            {dailyContent.reference}
          </p>
        </div>
      </div>

      {/* Action Button */}
      <button
        onClick={onTap}
        className="w-full py-3 border-2 border-primary/30 rounded-lg hover:bg-primary/5 transition-colors flex items-center justify-center gap-2"
      >
        <span>📚</span>
        <span className="font-semibold text-primary">
          {isVerse ? 'اقرأ التفسير' : 'اقرأ الشرح'}
        </span>
      </button>
    </div>
  );
}

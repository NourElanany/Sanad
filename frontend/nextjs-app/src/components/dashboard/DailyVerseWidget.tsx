'use client';

import { useState } from 'react';

interface DailyVerse {
  arabicText: string;
  translation: string;
  reference: string;
  briefTafsir: string;
  fullTafsir: string;
  tafsirSource: string;
}

interface DailyVerseWidgetProps {
  dailyVerse?: DailyVerse;
  onTap?: () => void;
}

export function DailyVerseWidget({ dailyVerse, onTap }: DailyVerseWidgetProps) {
  const [isExpanded, setIsExpanded] = useState(false);

  const verse: DailyVerse = dailyVerse || {
    arabicText: 'وَمَن يَتَّقِ اللَّهَ يَجْعَل لَّهُ مَخْرَجًا',
    translation: 'And whoever fears Allah - He will make for him a way out',
    reference: 'سورة الطلاق: 2',
    briefTafsir:
      'من يتق الله في جميع أموره، يجعل له مخرجًا من كل ضيق وكرب في الدنيا والآخرة.',
    fullTafsir:
      'من يتق الله في جميع أموره، يجعل له مخرجًا من كل ضيق وكرب في الدنيا والآخرة، ويرزقه من حيث لا يحتسب. وهذا وعد من الله تعالى لمن اتقاه بأن يجعل له فرجًا ومخرجًا من كل أمر يضيق عليه، وأن يرزقه من جهة لا تخطر بباله. والتقوى هي امتثال أوامر الله واجتناب نواهيه، وهي سبب كل خير في الدنيا والآخرة.',
    tafsirSource: 'تفسير السعدي',
  };

  const handleShare = () => {
    if (navigator.share) {
      navigator.share({
        title: 'آية اليوم',
        text: `${verse.arabicText}\n\n${verse.translation}\n\n${verse.reference}`,
      });
    } else {
      // Fallback: copy to clipboard
      navigator.clipboard.writeText(
        `${verse.arabicText}\n\n${verse.translation}\n\n${verse.reference}`
      );
      alert('تم نسخ الآية');
    }
  };

  return (
    <div className="bg-white rounded-2xl shadow-lg border border-primary/10 p-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <div className="bg-gradient-to-br from-accent to-accent/70 p-3 rounded-xl">
            <span className="text-2xl">✨</span>
          </div>
          <div>
            <h3 className="text-xl font-bold text-primary">آية اليوم</h3>
            <p className="text-sm text-gray-600">{verse.reference}</p>
          </div>
        </div>
        <button
          onClick={handleShare}
          className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          title="مشاركة"
        >
          <span className="text-xl">📤</span>
        </button>
      </div>

      {/* Arabic Text */}
      <div className="bg-gradient-to-br from-primary/5 to-accent/5 border border-primary/10 rounded-xl p-4 mb-4">
        <p
          className="text-2xl text-primary text-center leading-loose font-semibold"
          style={{ fontFamily: 'Amiri, serif' }}
          dir="rtl"
        >
          {verse.arabicText}
        </p>
      </div>

      {/* Translation */}
      <p className="text-center text-gray-600 italic mb-4">
        {verse.translation}
      </p>

      {/* Brief Tafsir */}
      <div className="bg-secondary/10 rounded-lg p-4 mb-3">
        <div className="flex items-center gap-2 mb-2">
          <span className="text-lg">💡</span>
          <span className="text-sm font-bold text-secondary">التفسير المختصر</span>
        </div>
        <p className="text-gray-900 leading-relaxed">{verse.briefTafsir}</p>
      </div>

      {/* Expandable Full Tafsir */}
      {isExpanded && (
        <div className="bg-gray-50 rounded-lg p-4 mb-3 animate-fadeIn">
          <div className="flex items-center gap-2 mb-2">
            <span className="text-lg">📚</span>
            <span className="text-sm font-bold text-primary">التفسير الكامل</span>
          </div>
          <p className="text-gray-900 leading-relaxed mb-3">{verse.fullTafsir}</p>
          <div className="flex items-center gap-1 text-sm text-gray-600 italic">
            <span>📖</span>
            <span>المصدر: {verse.tafsirSource}</span>
          </div>
        </div>
      )}

      {/* Expand/Collapse Button */}
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="w-full border border-primary/20 hover:bg-primary/5 rounded-lg py-2.5 flex items-center justify-center gap-2 text-primary font-semibold transition-colors"
      >
        <span>{isExpanded ? 'إخفاء التفسير الكامل' : 'عرض التفسير الكامل'}</span>
        <span className={`transform transition-transform ${isExpanded ? 'rotate-180' : ''}`}>
          ▼
        </span>
      </button>
    </div>
  );
}

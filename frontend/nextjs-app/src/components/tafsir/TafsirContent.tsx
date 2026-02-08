'use client';

import React, { useState } from 'react';
import type { TafsirWithSource, TafsirDisplayPreferences } from '@/types/tafsir';

interface TafsirContentProps {
  tafsirs: TafsirWithSource[];
  preferences: TafsirDisplayPreferences;
  onLayoutChange: (layout: 'stacked' | 'side-by-side' | 'tabbed') => void;
}

export const TafsirContent: React.FC<TafsirContentProps> = ({
  tafsirs,
  preferences,
  onLayoutChange,
}) => {
  const [activeTabIndex, setActiveTabIndex] = useState(0);

  const getFontSizeClass = () => {
    const sizes = {
      small: 'text-sm',
      medium: 'text-base',
      large: 'text-lg',
    };
    return sizes[preferences.font_size];
  };

  const getReadingTime = (wordCount: number) => {
    const minutes = Math.ceil(wordCount / 200);
    return `⏱️ ${minutes} دقيقة قراءة`;
  };

  if (tafsirs.length === 0) {
    return (
      <div className="text-center py-12 text-gray-500">
        <p className="text-lg">لا توجد تفاسير متاحة</p>
        <p className="text-sm mt-2">الرجاء اختيار مصادر التفسير</p>
      </div>
    );
  }

  // Stacked Layout
  if (preferences.layout === 'stacked') {
    return (
      <div className="space-y-6">
        <div className="flex justify-end gap-2 mb-4">
          <button
            onClick={() => onLayoutChange('stacked')}
            className="px-3 py-1 bg-[#1B365D] text-white rounded-lg text-sm"
          >
            📚 متتالي
          </button>
          <button
            onClick={() => onLayoutChange('side-by-side')}
            className="px-3 py-1 bg-gray-200 text-gray-700 rounded-lg text-sm hover:bg-gray-300"
          >
            ⚖️ جنباً إلى جنب
          </button>
          <button
            onClick={() => onLayoutChange('tabbed')}
            className="px-3 py-1 bg-gray-200 text-gray-700 rounded-lg text-sm hover:bg-gray-300"
          >
            📑 تبويبات
          </button>
        </div>

        {tafsirs.map(({ tafsir, source }) => (
          <div key={tafsir.id} className="bg-white border-2 border-gray-200 rounded-xl p-6 shadow-sm">
            {/* Source Header */}
            <div className="flex items-center justify-between mb-4 pb-4 border-b border-gray-200">
              <div>
                <h4 className="text-xl font-bold text-[#1B365D] font-['Tajawal']" dir="rtl">
                  {source.name}
                </h4>
                <p className="text-sm text-gray-600" dir="rtl">
                  {source.author}
                </p>
              </div>
              <div className="text-right">
                <div className="text-sm text-gray-500">
                  {getReadingTime(tafsir.word_count)}
                </div>
                <div className="text-sm font-bold text-[#B8860B]">
                  ⭐ {source.credibility_score.toFixed(1)}/10
                </div>
              </div>
            </div>

            {/* Tafsir Text */}
            <div
              className={`${getFontSizeClass()} leading-loose text-gray-800 font-['Tajawal']`}
              dir="rtl"
            >
              {tafsir.text}
            </div>

            {/* Themes */}
            {preferences.show_themes && tafsir.themes.length > 0 && (
              <div className="mt-4 pt-4 border-t border-gray-200">
                <h5 className="text-sm font-bold text-gray-700 mb-2" dir="rtl">
                  🏷️ المواضيع:
                </h5>
                <div className="flex flex-wrap gap-2">
                  {tafsir.themes.map((theme, index) => (
                    <span
                      key={index}
                      className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm"
                    >
                      {theme}
                    </span>
                  ))}
                </div>
              </div>
            )}

            {/* Cross References */}
            {preferences.show_cross_references && tafsir.cross_references.length > 0 && (
              <div className="mt-4 pt-4 border-t border-gray-200">
                <h5 className="text-sm font-bold text-gray-700 mb-2" dir="rtl">
                  🔗 المراجع المرتبطة:
                </h5>
                <div className="flex flex-wrap gap-2">
                  {tafsir.cross_references.map((ref, index) => (
                    <a
                      key={index}
                      href={`#${ref}`}
                      className="px-3 py-1 bg-green-100 text-green-800 rounded-lg text-sm hover:bg-green-200 transition-colors"
                    >
                      📖 {ref}
                    </a>
                  ))}
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    );
  }

  // Side-by-Side Layout
  if (preferences.layout === 'side-by-side') {
    return (
      <div>
        <div className="flex justify-end gap-2 mb-4">
          <button
            onClick={() => onLayoutChange('stacked')}
            className="px-3 py-1 bg-gray-200 text-gray-700 rounded-lg text-sm hover:bg-gray-300"
          >
            📚 متتالي
          </button>
          <button
            onClick={() => onLayoutChange('side-by-side')}
            className="px-3 py-1 bg-[#1B365D] text-white rounded-lg text-sm"
          >
            ⚖️ جنباً إلى جنب
          </button>
          <button
            onClick={() => onLayoutChange('tabbed')}
            className="px-3 py-1 bg-gray-200 text-gray-700 rounded-lg text-sm hover:bg-gray-300"
          >
            📑 تبويبات
          </button>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {tafsirs.map(({ tafsir, source }) => (
            <div key={tafsir.id} className="bg-white border-2 border-gray-200 rounded-xl p-4 shadow-sm">
              <div className="mb-3 pb-3 border-b border-gray-200">
                <h4 className="text-lg font-bold text-[#1B365D] font-['Tajawal']" dir="rtl">
                  {source.name}
                </h4>
                <p className="text-xs text-gray-600" dir="rtl">
                  {source.author}
                </p>
              </div>

              <div
                className={`${getFontSizeClass()} leading-relaxed text-gray-800 font-['Tajawal']`}
                dir="rtl"
              >
                {tafsir.text}
              </div>

              {preferences.show_themes && tafsir.themes.length > 0 && (
                <div className="mt-3 pt-3 border-t border-gray-200">
                  <div className="flex flex-wrap gap-1">
                    {tafsir.themes.slice(0, 3).map((theme, index) => (
                      <span
                        key={index}
                        className="px-2 py-0.5 bg-blue-100 text-blue-800 rounded-full text-xs"
                      >
                        {theme}
                      </span>
                    ))}
                  </div>
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
    );
  }

  // Tabbed Layout
  return (
    <div>
      <div className="flex justify-end gap-2 mb-4">
        <button
          onClick={() => onLayoutChange('stacked')}
          className="px-3 py-1 bg-gray-200 text-gray-700 rounded-lg text-sm hover:bg-gray-300"
        >
          📚 متتالي
        </button>
        <button
          onClick={() => onLayoutChange('side-by-side')}
          className="px-3 py-1 bg-gray-200 text-gray-700 rounded-lg text-sm hover:bg-gray-300"
        >
          ⚖️ جنباً إلى جنب
        </button>
        <button
          onClick={() => onLayoutChange('tabbed')}
          className="px-3 py-1 bg-[#1B365D] text-white rounded-lg text-sm"
        >
          📑 تبويبات
        </button>
      </div>

      {/* Tabs */}
      <div className="flex gap-2 mb-4 overflow-x-auto">
        {tafsirs.map(({ source }, index) => (
          <button
            key={source.id}
            onClick={() => setActiveTabIndex(index)}
            className={`px-4 py-2 rounded-lg whitespace-nowrap transition-colors ${
              activeTabIndex === index
                ? 'bg-[#1B365D] text-white'
                : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
            }`}
          >
            {source.name}
          </button>
        ))}
      </div>

      {/* Active Tab Content */}
      {tafsirs[activeTabIndex] && (
        <div className="bg-white border-2 border-gray-200 rounded-xl p-6 shadow-sm">
          <div className="flex items-center justify-between mb-4 pb-4 border-b border-gray-200">
            <div>
              <h4 className="text-xl font-bold text-[#1B365D] font-['Tajawal']" dir="rtl">
                {tafsirs[activeTabIndex].source.name}
              </h4>
              <p className="text-sm text-gray-600" dir="rtl">
                {tafsirs[activeTabIndex].source.author}
              </p>
            </div>
            <div className="text-right">
              <div className="text-sm text-gray-500">
                {getReadingTime(tafsirs[activeTabIndex].tafsir.word_count)}
              </div>
              <div className="text-sm font-bold text-[#B8860B]">
                ⭐ {tafsirs[activeTabIndex].source.credibility_score.toFixed(1)}/10
              </div>
            </div>
          </div>

          <div
            className={`${getFontSizeClass()} leading-loose text-gray-800 font-['Tajawal']`}
            dir="rtl"
          >
            {tafsirs[activeTabIndex].tafsir.text}
          </div>

          {preferences.show_themes && tafsirs[activeTabIndex].tafsir.themes.length > 0 && (
            <div className="mt-4 pt-4 border-t border-gray-200">
              <h5 className="text-sm font-bold text-gray-700 mb-2" dir="rtl">
                🏷️ المواضيع:
              </h5>
              <div className="flex flex-wrap gap-2">
                {tafsirs[activeTabIndex].tafsir.themes.map((theme, index) => (
                  <span
                    key={index}
                    className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm"
                  >
                    {theme}
                  </span>
                ))}
              </div>
            </div>
          )}

          {preferences.show_cross_references &&
            tafsirs[activeTabIndex].tafsir.cross_references.length > 0 && (
              <div className="mt-4 pt-4 border-t border-gray-200">
                <h5 className="text-sm font-bold text-gray-700 mb-2" dir="rtl">
                  🔗 المراجع المرتبطة:
                </h5>
                <div className="flex flex-wrap gap-2">
                  {tafsirs[activeTabIndex].tafsir.cross_references.map((ref, index) => (
                    <a
                      key={index}
                      href={`#${ref}`}
                      className="px-3 py-1 bg-green-100 text-green-800 rounded-lg text-sm hover:bg-green-200 transition-colors"
                    >
                      📖 {ref}
                    </a>
                  ))}
                </div>
              </div>
            )}
        </div>
      )}
    </div>
  );
};

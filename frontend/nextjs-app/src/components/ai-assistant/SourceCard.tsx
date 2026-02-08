'use client';

import { useState } from 'react';
import { Source } from '@/types/ai-assistant';

interface SourceCardProps {
  source: Source;
}

export function SourceCard({ source }: SourceCardProps) {
  const [isExpanded, setIsExpanded] = useState(false);

  const getSourceColor = () => {
    switch (source.type) {
      case 'quran':
        return {
          bg: 'bg-green-50',
          border: 'border-green-200',
          text: 'text-green-700',
          icon: 'bg-green-600',
        };
      case 'hadith':
        return {
          bg: 'bg-blue-50',
          border: 'border-blue-200',
          text: 'text-blue-700',
          icon: 'bg-blue-600',
        };
      case 'fatwa':
        return {
          bg: 'bg-amber-50',
          border: 'border-amber-200',
          text: 'text-amber-700',
          icon: 'bg-amber-600',
        };
      case 'tafsir':
        return {
          bg: 'bg-purple-50',
          border: 'border-purple-200',
          text: 'text-purple-700',
          icon: 'bg-purple-600',
        };
      default:
        return {
          bg: 'bg-gray-50',
          border: 'border-gray-200',
          text: 'text-gray-700',
          icon: 'bg-gray-600',
        };
    }
  };

  const getSourceIcon = () => {
    switch (source.type) {
      case 'quran':
        return (
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
        );
      case 'hadith':
        return (
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z" />
          </svg>
        );
      case 'fatwa':
        return (
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0M6 7l3 9M6 7l6-2m6 2l3-1m-3 1l-3 9a5.002 5.002 0 006.001 0M18 7l3 9m-3-9l-6-2m0-2v2m0 16V5m0 16H9m3 0h3" />
          </svg>
        );
      case 'tafsir':
        return (
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 14v3m4-3v3m4-3v3M3 21h18M3 10h18M3 7l9-4 9 4M4 10h16v11H4V10z" />
          </svg>
        );
      default:
        return null;
    }
  };

  const getSourceTypeLabel = () => {
    switch (source.type) {
      case 'quran':
        return 'القرآن الكريم';
      case 'hadith':
        return 'الحديث النبوي';
      case 'fatwa':
        return 'فتوى';
      case 'tafsir':
        return 'التفسير';
      default:
        return 'مصدر';
    }
  };

  const getConfidenceColor = () => {
    if (!source.confidence) return 'bg-gray-400';
    if (source.confidence >= 0.8) return 'bg-green-500';
    if (source.confidence >= 0.6) return 'bg-yellow-500';
    return 'bg-red-500';
  };

  const colors = getSourceColor();

  return (
    <div
      className={`${colors.bg} ${colors.border} border rounded-xl overflow-hidden transition-all hover:shadow-md cursor-pointer`}
      onClick={() => setIsExpanded(!isExpanded)}
    >
      <div className="p-4">
        <div className="flex items-start gap-3">
          {/* Icon */}
          <div className={`${colors.icon} p-2 rounded-lg text-white flex-shrink-0`}>
            {getSourceIcon()}
          </div>

          {/* Content */}
          <div className="flex-1 min-w-0">
            <div className="flex items-start justify-between gap-2">
              <div className="flex-1">
                <p className={`text-xs font-semibold ${colors.text} mb-1`}>
                  {getSourceTypeLabel()}
                </p>
                <h4 className="font-bold text-gray-900 text-sm mb-1">
                  {source.title}
                </h4>
                <p className="text-xs text-gray-600">{source.reference}</p>
              </div>

              {/* Confidence Badge */}
              {source.confidence !== undefined && (
                <div className="flex items-center gap-1 flex-shrink-0">
                  <div className={`${getConfidenceColor()} w-2 h-2 rounded-full`} />
                  <span className="text-xs font-semibold text-gray-600">
                    {Math.round(source.confidence * 100)}%
                  </span>
                </div>
              )}
            </div>

            {/* Excerpt (when expanded) */}
            {isExpanded && source.excerpt && (
              <div className="mt-3 pt-3 border-t border-gray-200">
                <p className="text-sm text-gray-700 leading-relaxed" dir="rtl">
                  {source.excerpt}
                </p>
              </div>
            )}
          </div>

          {/* Expand Icon */}
          <svg
            className={`w-5 h-5 text-gray-400 flex-shrink-0 transition-transform ${
              isExpanded ? 'rotate-180' : ''
            }`}
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M19 9l-7 7-7-7"
            />
          </svg>
        </div>

        {/* Action Buttons (when expanded) */}
        {isExpanded && (
          <div className="mt-4 flex gap-2">
            <button
              onClick={(e) => {
                e.stopPropagation();
                // TODO: Navigate to source
              }}
              className={`flex-1 px-4 py-2 ${colors.text} border ${colors.border} rounded-lg hover:bg-white transition-colors text-sm font-semibold`}
            >
              عرض المصدر الكامل
            </button>
          </div>
        )}
      </div>
    </div>
  );
}

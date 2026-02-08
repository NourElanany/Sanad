import Link from 'next/link';
import type { HadithSearchResponse } from '@/types/hadith';
import { getGradeArabicName, getGradeColor } from '@/types/hadith';

interface HadithSearchResultsProps {
  results: HadithSearchResponse;
}

export function HadithSearchResults({ results }: HadithSearchResultsProps) {
  if (results.results.length === 0) {
    return (
      <div className="text-center py-12">
        <svg
          className="mx-auto h-16 w-16 text-gray-400"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
        <h3 className="mt-4 text-lg font-medium text-gray-900 font-tajawal">
          لا توجد نتائج
        </h3>
        <p className="mt-2 text-sm text-gray-500 font-tajawal">
          جرب البحث بكلمات مختلفة أو قم بتعديل الفلاتر
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {/* Results Header */}
      <div className="flex items-center justify-between mb-6">
        <div className="text-sm text-gray-600 font-tajawal">
          <span className="font-bold text-gray-900">{results.total_count}</span> نتيجة
          للبحث عن "<span className="font-bold">{results.query}</span>"
        </div>
        <div className="text-xs text-gray-500 font-tajawal">
          وقت البحث: {results.search_time_ms} مللي ثانية
        </div>
      </div>

      {/* Results List */}
      <div className="space-y-4">
        {results.results.map((result) => (
          <Link
            key={result.hadith.id}
            href={`/hadith/${result.hadith.id}`}
            className="block"
          >
            <div className="bg-white border border-gray-200 rounded-lg p-6 hover:shadow-lg transition-shadow">
              {/* Header */}
              <div className="flex items-start justify-between mb-4">
                <div className="flex-1">
                  <h3 className="text-base font-bold text-gray-900 font-tajawal" dir="rtl">
                    {result.book.arabic_name}
                  </h3>
                  <p className="text-sm text-gray-600 font-tajawal mt-1" dir="rtl">
                    {result.book.author_arabic_name}
                  </p>
                </div>
                <span
                  className="inline-flex items-center px-3 py-1 rounded-full text-xs font-bold text-white"
                  style={{ backgroundColor: getGradeColor(result.hadith.grade) }}
                >
                  {getGradeArabicName(result.hadith.grade)}
                </span>
              </div>

              {/* Hadith Text */}
              <div className="mb-4">
                <p
                  className="text-base leading-loose text-gray-800 font-amiri"
                  dir="rtl"
                  dangerouslySetInnerHTML={{ __html: result.highlighted_text }}
                />
              </div>

              {/* Metadata */}
              <div className="flex items-center justify-between text-sm text-gray-600">
                <div className="flex items-center gap-4">
                  <div className="flex items-center gap-1">
                    <svg
                      className="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                      />
                    </svg>
                    <span className="font-tajawal" dir="rtl">
                      {result.hadith.narrator}
                    </span>
                  </div>
                  <div className="flex items-center gap-1">
                    <span className="font-tajawal">رقم {result.hadith.hadith_number}</span>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  {result.matching_criteria.length > 0 && (
                    <div className="flex gap-1">
                      {result.matching_criteria.map((criteria, index) => (
                        <span
                          key={index}
                          className="inline-flex items-center px-2 py-1 rounded text-xs bg-blue-100 text-blue-800 font-tajawal"
                        >
                          {criteria}
                        </span>
                      ))}
                    </div>
                  )}
                  <span className="text-xs text-gray-500">
                    {(result.relevance_score * 100).toFixed(0)}% تطابق
                  </span>
                </div>
              </div>
            </div>
          </Link>
        ))}
      </div>
    </div>
  );
}

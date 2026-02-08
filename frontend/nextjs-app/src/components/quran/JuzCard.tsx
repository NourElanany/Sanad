/**
 * Juz Card Component
 */
import Link from 'next/link';
import type { Juz } from '@/types/quran';

interface JuzCardProps {
  juz: Juz;
}

const arabicNumbers = [
  'الأول', 'الثاني', 'الثالث', 'الرابع', 'الخامس',
  'السادس', 'السابع', 'الثامن', 'التاسع', 'العاشر',
  'الحادي عشر', 'الثاني عشر', 'الثالث عشر', 'الرابع عشر', 'الخامس عشر',
  'السادس عشر', 'السابع عشر', 'الثامن عشر', 'التاسع عشر', 'العشرون',
  'الحادي والعشرون', 'الثاني والعشرون', 'الثالث والعشرون', 'الرابع والعشرون',
  'الخامس والعشرون', 'السادس والعشرون', 'السابع والعشرون', 'الثامن والعشرون',
  'التاسع والعشرون', 'الثلاثون'
];

export function JuzCard({ juz }: JuzCardProps) {
  const arabicNumber = arabicNumbers[juz.number - 1] || juz.number.toString();

  return (
    <Link href={`/quran/juz/${juz.number}`}>
      <div className="bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 p-6 border border-gray-100 hover:border-[#2D5A27] cursor-pointer group">
        <div className="flex items-start gap-4">
          {/* Juz Number Badge */}
          <div className="flex-shrink-0">
            <div className="w-16 h-16 rounded-xl bg-gradient-to-br from-[#2D5A27] to-[#4A7C59] flex flex-col items-center justify-center shadow-lg group-hover:scale-110 transition-transform">
              <span className="text-white text-xs">جزء</span>
              <span className="text-white font-bold text-lg">
                {juz.number}
              </span>
            </div>
          </div>

          {/* Juz Info */}
          <div className="flex-1 min-w-0">
            {/* Title */}
            <h3 className="text-xl font-bold text-[#0F1F35] mb-3">
              الجزء {arabicNumber}
            </h3>

            {/* Start Position */}
            <div className="flex items-center gap-2 mb-2">
              <svg
                className="w-4 h-4 text-green-600"
                fill="currentColor"
                viewBox="0 0 20 20"
              >
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z"
                  clipRule="evenodd"
                />
              </svg>
              <span className="text-sm text-gray-600">
                من سورة {juz.start_surah} آية {juz.start_ayah}
              </span>
            </div>

            {/* End Position */}
            <div className="flex items-center gap-2 mb-2">
              <svg
                className="w-4 h-4 text-red-600"
                fill="currentColor"
                viewBox="0 0 20 20"
              >
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zM8 7a1 1 0 00-1 1v4a1 1 0 001 1h4a1 1 0 001-1V8a1 1 0 00-1-1H8z"
                  clipRule="evenodd"
                />
              </svg>
              <span className="text-sm text-gray-600">
                إلى سورة {juz.end_surah} آية {juz.end_ayah}
              </span>
            </div>

            {/* Page Range */}
            <div className="text-xs text-gray-500 mt-2">
              الصفحات {juz.page_start} - {juz.page_end}
            </div>
          </div>

          {/* Arrow Icon */}
          <div className="flex-shrink-0">
            <svg
              className="w-6 h-6 text-gray-400 group-hover:text-[#2D5A27] group-hover:translate-x-1 transition-all"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 19l-7-7 7-7"
              />
            </svg>
          </div>
        </div>
      </div>
    </Link>
  );
}

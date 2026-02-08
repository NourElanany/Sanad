/**
 * Surah Card Component
 */
import Link from 'next/link';
import type { Surah } from '@/types/quran';

interface SurahCardProps {
  surah: Surah;
}

export function SurahCard({ surah }: SurahCardProps) {
  const isMeccan = surah.revelation_type === 'Meccan';

  return (
    <Link href={`/quran/surah/${surah.number}`}>
      <div className="bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 p-6 border border-gray-100 hover:border-[#1B365D] cursor-pointer group">
        <div className="flex items-start gap-4">
          {/* Surah Number Badge */}
          <div className="flex-shrink-0">
            <div className="w-14 h-14 rounded-xl bg-gradient-to-br from-[#1B365D] to-[#2E4A6B] flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform">
              <span className="text-white font-bold text-lg">
                {surah.number}
              </span>
            </div>
          </div>

          {/* Surah Info */}
          <div className="flex-1 min-w-0">
            {/* Arabic Name */}
            <h3 className="text-2xl font-bold text-[#0F1F35] mb-2 font-arabic">
              {surah.name_arabic}
            </h3>

            {/* English Name */}
            <p className="text-gray-600 mb-2">
              {surah.name_english}
            </p>

            {/* Metadata */}
            <div className="flex items-center gap-3 flex-wrap">
              {/* Revelation Type Badge */}
              <span
                className={`inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold ${
                  isMeccan
                    ? 'bg-green-100 text-green-800 border border-green-300'
                    : 'bg-blue-100 text-blue-800 border border-blue-300'
                }`}
              >
                {isMeccan ? 'مكية' : 'مدنية'}
              </span>

              {/* Ayah Count */}
              <span className="text-sm text-gray-500">
                {surah.ayah_count} آية
              </span>

              {/* Juz Info */}
              <span className="text-sm text-gray-500">
                الجزء {surah.juz_start}
                {surah.juz_end !== surah.juz_start && `-${surah.juz_end}`}
              </span>
            </div>
          </div>

          {/* Arrow Icon */}
          <div className="flex-shrink-0">
            <svg
              className="w-6 h-6 text-gray-400 group-hover:text-[#1B365D] group-hover:translate-x-1 transition-all"
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

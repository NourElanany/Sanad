import Link from 'next/link';
import type { HadithBook } from '@/types/hadith';
import {
  getBookTypeArabicName,
  getAuthenticityLevelArabicName,
} from '@/types/hadith';

interface HadithBookCardProps {
  book: HadithBook;
}

export function HadithBookCard({ book }: HadithBookCardProps) {
  const getAuthenticityColor = () => {
    switch (book.authenticity_level) {
      case 'highest':
        return 'bg-green-100 text-green-800 border-green-300';
      case 'high':
        return 'bg-lime-100 text-lime-800 border-lime-300';
      case 'moderate':
        return 'bg-amber-100 text-amber-800 border-amber-300';
      case 'variable':
        return 'bg-orange-100 text-orange-800 border-orange-300';
      default:
        return 'bg-gray-100 text-gray-800 border-gray-300';
    }
  };

  return (
    <Link href={`/hadith/book/${book.id}`}>
      <div className="bg-white border border-gray-200 rounded-2xl p-6 hover:shadow-xl transition-all duration-300 cursor-pointer h-full flex flex-col">
        {/* Header with badges */}
        <div className="flex items-start justify-between mb-4">
          <span className="inline-flex items-center px-3 py-1 rounded-full text-xs font-bold bg-[#2D5A27] bg-opacity-10 text-[#2D5A27] font-tajawal">
            {getBookTypeArabicName(book.book_type)}
          </span>
          <span
            className={`inline-flex items-center gap-1 px-3 py-1 rounded-full text-xs font-bold border ${getAuthenticityColor()} font-tajawal`}
          >
            <svg
              className="w-3 h-3"
              fill="currentColor"
              viewBox="0 0 20 20"
            >
              <path
                fillRule="evenodd"
                d="M6.267 3.455a3.066 3.066 0 001.745-.723 3.066 3.066 0 013.976 0 3.066 3.066 0 001.745.723 3.066 3.066 0 012.812 2.812c.051.643.304 1.254.723 1.745a3.066 3.066 0 010 3.976 3.066 3.066 0 00-.723 1.745 3.066 3.066 0 01-2.812 2.812 3.066 3.066 0 00-1.745.723 3.066 3.066 0 01-3.976 0 3.066 3.066 0 00-1.745-.723 3.066 3.066 0 01-2.812-2.812 3.066 3.066 0 00-.723-1.745 3.066 3.066 0 010-3.976 3.066 3.066 0 00.723-1.745 3.066 3.066 0 012.812-2.812zm7.44 5.252a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                clipRule="evenodd"
              />
            </svg>
            {getAuthenticityLevelArabicName(book.authenticity_level)}
          </span>
        </div>

        {/* Book title */}
        <h3 className="text-xl font-bold text-gray-900 mb-3 font-tajawal leading-relaxed" dir="rtl">
          {book.arabic_name}
        </h3>

        {/* Author */}
        <div className="flex items-center gap-2 mb-4 text-gray-700">
          <svg
            className="w-4 h-4 flex-shrink-0"
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
          <span className="text-sm font-tajawal" dir="rtl">
            {book.author_arabic_name}
          </span>
        </div>

        {/* Description */}
        {book.description && (
          <p className="text-sm text-gray-600 mb-4 line-clamp-2 leading-relaxed" dir="rtl">
            {book.description}
          </p>
        )}

        {/* Footer with stats */}
        <div className="mt-auto pt-4 border-t border-gray-100 flex items-center justify-between">
          <div className="flex items-center gap-4 text-sm text-gray-600">
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
                  d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
                />
              </svg>
              <span className="font-tajawal">{book.total_hadiths} حديث</span>
            </div>
            {book.compilation_year && (
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
                    d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
                  />
                </svg>
                <span className="font-tajawal">{book.compilation_year} هـ</span>
              </div>
            )}
          </div>
          <svg
            className="w-5 h-5 text-[#1B365D]"
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
    </Link>
  );
}

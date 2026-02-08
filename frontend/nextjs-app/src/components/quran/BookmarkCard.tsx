/**
 * Bookmark Card Component
 */
'use client';

import { useState } from 'react';
import Link from 'next/link';
import type { QuranBookmark } from '@/types/quran';

interface BookmarkCardProps {
  bookmark: QuranBookmark;
  onDelete: () => void;
}

export function BookmarkCard({ bookmark, onDelete }: BookmarkCardProps) {
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffInDays = Math.floor(
      (now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24)
    );

    if (diffInDays === 0) return 'اليوم';
    if (diffInDays === 1) return 'أمس';
    if (diffInDays < 7) return `منذ ${diffInDays} أيام`;
    
    return date.toLocaleDateString('ar-SA', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
  };

  const handleDelete = () => {
    setShowDeleteConfirm(false);
    onDelete();
  };

  return (
    <div className="bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 p-6 border border-gray-100 hover:border-[#B8860B] relative group">
      <Link href={`/quran/surah/${bookmark.surah_number}?ayah=${bookmark.ayah_number}`}>
        <div className="cursor-pointer">
          {/* Bookmark Icon */}
          <div className="flex items-start gap-4 mb-4">
            <div className="flex-shrink-0">
              <div className="w-12 h-12 rounded-lg bg-gradient-to-br from-[#B8860B] to-[#DAA520] flex items-center justify-center shadow-md">
                <svg
                  className="w-6 h-6 text-white"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path d="M5 4a2 2 0 012-2h6a2 2 0 012 2v14l-5-2.5L5 18V4z" />
                </svg>
              </div>
            </div>

            {/* Bookmark Info */}
            <div className="flex-1 min-w-0">
              <h3 className="text-lg font-bold text-[#0F1F35] mb-1">
                سورة {bookmark.surah_number} - آية {bookmark.ayah_number}
              </h3>
              <p className="text-sm text-gray-600 mb-2">
                صفحة {bookmark.page_number}
              </p>
              
              {bookmark.note && (
                <p className="text-sm text-gray-500 italic line-clamp-2 mb-2">
                  {bookmark.note}
                </p>
              )}
              
              <p className="text-xs text-gray-400">
                {formatDate(bookmark.created_at)}
              </p>
            </div>
          </div>
        </div>
      </Link>

      {/* Delete Button */}
      <button
        onClick={(e) => {
          e.stopPropagation();
          setShowDeleteConfirm(true);
        }}
        className="absolute top-4 left-4 p-2 rounded-lg bg-red-50 text-red-600 hover:bg-red-100 transition-colors opacity-0 group-hover:opacity-100"
        title="حذف العلامة المرجعية"
      >
        <svg
          className="w-5 h-5"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
          />
        </svg>
      </button>

      {/* Delete Confirmation Modal */}
      {showDeleteConfirm && (
        <div
          className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
          onClick={() => setShowDeleteConfirm(false)}
        >
          <div
            className="bg-white rounded-xl p-6 max-w-sm mx-4 shadow-2xl"
            onClick={(e) => e.stopPropagation()}
          >
            <h3 className="text-xl font-bold text-gray-800 mb-2">
              حذف العلامة المرجعية
            </h3>
            <p className="text-gray-600 mb-6">
              هل أنت متأكد من حذف هذه العلامة المرجعية؟
            </p>
            <div className="flex gap-3">
              <button
                onClick={() => setShowDeleteConfirm(false)}
                className="flex-1 px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors"
              >
                إلغاء
              </button>
              <button
                onClick={handleDelete}
                className="flex-1 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
              >
                حذف
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

/**
 * Quran Filters Component
 */
'use client';

import { useState } from 'react';
import type { QuranFilters } from '@/types/quran';

interface QuranFiltersProps {
  filters: QuranFilters;
  onChange: (filters: QuranFilters) => void;
}

export function QuranFilters({ filters, onChange }: QuranFiltersProps) {
  const [showFilters, setShowFilters] = useState(false);

  const hasActiveFilters =
    filters.revelationType !== 'all' || filters.ayahCountRange !== 'all';

  return (
    <div className="relative">
      {/* Filter Toggle Button */}
      <button
        onClick={() => setShowFilters(!showFilters)}
        className={`flex items-center gap-2 px-4 py-2 rounded-lg transition-colors ${
          hasActiveFilters
            ? 'bg-[#B8860B] text-white'
            : 'bg-white text-[#1B365D] hover:bg-gray-100'
        }`}
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
            d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"
          />
        </svg>
        <span>فلاتر البحث</span>
        {hasActiveFilters && (
          <span className="bg-white text-[#B8860B] text-xs font-bold px-2 py-0.5 rounded-full">
            نشط
          </span>
        )}
      </button>

      {/* Filter Panel */}
      {showFilters && (
        <div className="absolute top-full left-0 right-0 mt-2 bg-white rounded-xl shadow-2xl p-6 z-20 border border-gray-200">
          <div className="grid md:grid-cols-2 gap-6">
            {/* Revelation Type Filter */}
            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-3">
                نوع السورة
              </label>
              <div className="flex flex-wrap gap-2">
                <button
                  onClick={() =>
                    onChange({ ...filters, revelationType: 'all' })
                  }
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                    filters.revelationType === 'all'
                      ? 'bg-[#1B365D] text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  الكل
                </button>
                <button
                  onClick={() =>
                    onChange({ ...filters, revelationType: 'meccan' })
                  }
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                    filters.revelationType === 'meccan'
                      ? 'bg-green-600 text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  مكية
                </button>
                <button
                  onClick={() =>
                    onChange({ ...filters, revelationType: 'medinan' })
                  }
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                    filters.revelationType === 'medinan'
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  مدنية
                </button>
              </div>
            </div>

            {/* Ayah Count Filter */}
            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-3">
                عدد الآيات
              </label>
              <div className="flex flex-wrap gap-2">
                <button
                  onClick={() =>
                    onChange({ ...filters, ayahCountRange: 'all' })
                  }
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                    filters.ayahCountRange === 'all'
                      ? 'bg-[#1B365D] text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  الكل
                </button>
                <button
                  onClick={() =>
                    onChange({ ...filters, ayahCountRange: '1-20' })
                  }
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                    filters.ayahCountRange === '1-20'
                      ? 'bg-[#B8860B] text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  قصيرة (1-20)
                </button>
                <button
                  onClick={() =>
                    onChange({ ...filters, ayahCountRange: '21-100' })
                  }
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                    filters.ayahCountRange === '21-100'
                      ? 'bg-[#B8860B] text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  متوسطة (21-100)
                </button>
                <button
                  onClick={() =>
                    onChange({ ...filters, ayahCountRange: '100-999' })
                  }
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                    filters.ayahCountRange === '100-999'
                      ? 'bg-[#B8860B] text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  طويلة (100+)
                </button>
              </div>
            </div>
          </div>

          {/* Clear Filters Button */}
          {hasActiveFilters && (
            <div className="mt-4 pt-4 border-t border-gray-200">
              <button
                onClick={() =>
                  onChange({
                    revelationType: 'all',
                    ayahCountRange: 'all',
                    searchQuery: filters.searchQuery,
                  })
                }
                className="text-sm text-red-600 hover:text-red-700 font-medium"
              >
                مسح جميع الفلاتر
              </button>
            </div>
          )}
        </div>
      )}

      {/* Backdrop */}
      {showFilters && (
        <div
          className="fixed inset-0 z-10"
          onClick={() => setShowFilters(false)}
        />
      )}
    </div>
  );
}

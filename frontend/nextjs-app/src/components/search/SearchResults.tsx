'use client';

/**
 * Search Results Display
 * Requirements: 8.1, 8.2, 8.3, 8.4, 8.5
 */

import { useState } from 'react';
import type { SearchResponse, SortBy, SortDirection } from '@/types/search';
import { SearchResultCard } from './SearchResultCard';

interface SearchResultsProps {
  response: SearchResponse | null;
  isLoading: boolean;
  error: string | null;
  onLoadMore: () => void;
  onRetry: () => void;
  onSortChange?: (sortBy: SortBy, direction: SortDirection) => void;
}

export function SearchResults({
  response,
  isLoading,
  error,
  onLoadMore,
  onRetry,
  onSortChange,
}: SearchResultsProps) {
  const [sortBy, setSortBy] = useState<SortBy>('similarity' as SortBy);
  const [sortDirection, setSortDirection] = useState<SortDirection>('desc' as SortDirection);
  const [showSortMenu, setShowSortMenu] = useState(false);

  const handleSortChange = (newSortBy: SortBy) => {
    const newDirection = sortBy === newSortBy && sortDirection === 'desc' ? 'asc' : 'desc';
    setSortBy(newSortBy);
    setSortDirection(newDirection as SortDirection);
    onSortChange?.(newSortBy, newDirection as SortDirection);
    setShowSortMenu(false);
  };

  const getSortLabel = (sort: SortBy): string => {
    const labels: Record<SortBy, string> = {
      similarity: 'الأكثر صلة',
      priority: 'الأولوية',
      created_at: 'الأحدث',
      updated_at: 'آخر تحديث',
      text_length: 'الطول',
      relevance: 'الصلة',
    };
    return labels[sort];
  };
  if (isLoading && !response) {
    return (
      <div className="flex justify-center items-center py-20">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-[#1B365D]" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-20">
        <svg
          className="w-16 h-16 text-red-500 mx-auto mb-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
        <h3 className="text-lg font-semibold text-gray-900 mb-2">حدث خطأ</h3>
        <p className="text-gray-600 mb-4">{error}</p>
        <button
          onClick={onRetry}
          className="bg-[#1B365D] text-white px-6 py-2 rounded-lg hover:bg-[#2E4A6B] transition-colors"
        >
          إعادة المحاولة
        </button>
      </div>
    );
  }

  if (!response) {
    return (
      <div className="text-center py-20">
        <svg
          className="w-20 h-20 text-gray-300 mx-auto mb-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
          />
        </svg>
        <h3 className="text-xl font-semibold text-gray-900 mb-2">
          ابحث في القرآن والحديث والفتاوى
        </h3>
        <p className="text-gray-600">استخدم البحث الذكي للعثور على المحتوى الإسلامي</p>
      </div>
    );
  }

  if (response.results.length === 0) {
    return (
      <div className="text-center py-20">
        <svg
          className="w-16 h-16 text-gray-300 mx-auto mb-4"
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
        <h3 className="text-lg font-semibold text-gray-900 mb-2">لم يتم العثور على نتائج</h3>
        <p className="text-gray-600">جرب كلمات بحث مختلفة أو قم بتعديل الفلاتر</p>
      </div>
    );
  }

  return (
    <div>
      {/* Results Header with Sorting */}
      <div className="flex items-center justify-between mb-6 pb-4 border-b">
        <div className="flex items-center gap-4">
          <span className="text-sm font-semibold text-gray-600">
            النتائج: {response.total_results}
          </span>
          {response.from_cache && (
            <span className="bg-[#B8860B]/10 text-[#B8860B] px-3 py-1 rounded-full text-xs font-semibold flex items-center gap-1">
              <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                <path d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z" />
              </svg>
              سريع
            </span>
          )}
        </div>
        
        <div className="flex items-center gap-3">
          <span className="text-xs text-gray-500">{response.search_time_ms}ms</span>
          
          {/* Sort Dropdown */}
          <div className="relative">
            <button
              onClick={() => setShowSortMenu(!showSortMenu)}
              className="flex items-center gap-2 px-4 py-2 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors text-sm font-medium"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 4h13M3 8h9m-9 4h6m4 0l4-4m0 0l4 4m-4-4v12" />
              </svg>
              <span>ترتيب: {getSortLabel(sortBy)}</span>
              <svg className={`w-4 h-4 transition-transform ${showSortMenu ? 'rotate-180' : ''}`} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
              </svg>
            </button>

            {showSortMenu && (
              <div className="absolute left-0 mt-2 w-48 bg-white rounded-lg shadow-xl border border-gray-200 z-10">
                <div className="py-2">
                  {(['similarity', 'relevance', 'created_at', 'priority'] as SortBy[]).map((sort) => (
                    <button
                      key={sort}
                      onClick={() => handleSortChange(sort)}
                      className={`w-full px-4 py-2 text-right hover:bg-gray-50 flex items-center justify-between text-sm ${
                        sortBy === sort ? 'bg-[#1B365D]/5 text-[#1B365D] font-semibold' : 'text-gray-700'
                      }`}
                    >
                      <span>{getSortLabel(sort)}</span>
                      {sortBy === sort && (
                        <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                          <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                        </svg>
                      )}
                    </button>
                  ))}
                </div>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Results List */}
      <div className="space-y-4">
        {response.results.map((result, index) => (
          <SearchResultCard key={`${result.document.id}-${index}`} result={result} />
        ))}
      </div>

      {/* Load More */}
      {response.pagination?.has_next_page && (
        <div className="mt-8 text-center">
          <button
            onClick={onLoadMore}
            disabled={isLoading}
            className="bg-white border-2 border-[#1B365D] text-[#1B365D] px-8 py-3 rounded-lg hover:bg-[#1B365D] hover:text-white transition-colors font-semibold disabled:opacity-50"
          >
            {isLoading ? 'جاري التحميل...' : 'تحميل المزيد'}
          </button>
        </div>
      )}
    </div>
  );
}

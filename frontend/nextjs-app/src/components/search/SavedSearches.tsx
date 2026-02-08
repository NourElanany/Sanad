'use client';

/**
 * Saved Searches Component
 * Requirements: 8.5
 */

import { useState, useEffect } from 'react';
import { SearchService } from '@/lib/services/search-service';
import type { SavedSearch } from '@/types/search';

interface SavedSearchesProps {
  onSearchSelect: (search: SavedSearch) => void;
  onClose: () => void;
}

export function SavedSearches({ onSearchSelect, onClose }: SavedSearchesProps) {
  const [searches, setSearches] = useState<SavedSearch[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    loadSavedSearches();
  }, []);

  const loadSavedSearches = async () => {
    try {
      const data = await SearchService.getSavedSearches();
      setSearches(data);
    } catch (error) {
      console.error('Failed to load saved searches:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleDelete = async (searchId: string) => {
    if (!confirm('هل تريد حذف هذا البحث المحفوظ؟')) return;

    try {
      await SearchService.deleteSavedSearch(searchId);
      setSearches(searches.filter((s) => s.id !== searchId));
    } catch (error) {
      alert('فشل حذف البحث');
    }
  };

  if (isLoading) {
    return (
      <div className="bg-white rounded-xl border border-gray-200 p-6">
        <div className="animate-pulse space-y-4">
          <div className="h-4 bg-gray-200 rounded w-3/4" />
          <div className="h-4 bg-gray-200 rounded w-1/2" />
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-xl border border-gray-200 p-6 sticky top-24">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-bold text-[#1B365D]">البحثات المحفوظة</h3>
        <button
          onClick={onClose}
          className="text-gray-400 hover:text-gray-600 transition-colors"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>

      {searches.length === 0 ? (
        <div className="text-center py-8">
          <svg
            className="w-12 h-12 text-gray-300 mx-auto mb-3"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z"
            />
          </svg>
          <p className="text-sm text-gray-600">لا توجد بحثات محفوظة</p>
        </div>
      ) : (
        <div className="space-y-3">
          {searches.map((search) => (
            <div
              key={search.id}
              className="border border-gray-200 rounded-lg p-3 hover:border-[#1B365D] transition-colors"
            >
              <button
                onClick={() => onSearchSelect(search)}
                className="w-full text-right"
              >
                <p className="font-medium text-gray-900 mb-1">
                  {search.name || search.query}
                </p>
                {search.name && (
                  <p className="text-sm text-gray-600 mb-2">{search.query}</p>
                )}
                <p className="text-xs text-gray-500">
                  {new Date(search.created_at).toLocaleDateString('ar-SA')}
                </p>
              </button>
              <button
                onClick={() => handleDelete(search.id)}
                className="mt-2 text-xs text-red-600 hover:text-red-700 transition-colors"
              >
                حذف
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

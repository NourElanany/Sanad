'use client';

/**
 * Advanced Search Filters Component
 * Requirements: 8.2
 */

import { useState } from 'react';
import type { SearchFilters as SearchFiltersType, SortBy } from '@/types/search';
import { ContentType, AuthenticityGrade } from '@/types/search';

interface SearchFiltersProps {
  filters?: SearchFiltersType;
  sortBy?: SortBy;
  onFiltersChange: (filters: SearchFiltersType) => void;
  onSortChange: (sortBy: SortBy) => void;
  onApply: () => void;
  onClear: () => void;
}

export function SearchFilters({
  filters,
  sortBy,
  onFiltersChange,
  onSortChange,
  onApply,
  onClear,
}: SearchFiltersProps) {
  const [selectedContentTypes, setSelectedContentTypes] = useState<ContentType[]>(
    filters?.content_types || []
  );
  const [selectedAuthenticityGrades, setSelectedAuthenticityGrades] = useState<
    AuthenticityGrade[]
  >(filters?.authenticity_grades || []);
  const [minSimilarity, setMinSimilarity] = useState(filters?.min_similarity || 0.5);

  const handleApply = () => {
    onFiltersChange({
      content_types: selectedContentTypes.length > 0 ? selectedContentTypes : undefined,
      authenticity_grades:
        selectedAuthenticityGrades.length > 0 ? selectedAuthenticityGrades : undefined,
      min_similarity: minSimilarity,
    });
    onApply();
  };

  return (
    <div className="bg-white rounded-xl border border-gray-200 p-6 sticky top-24">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-bold text-[#1B365D]">فلاتر البحث</h3>
        <button
          onClick={onClear}
          className="text-sm text-gray-600 hover:text-[#1B365D] transition-colors"
        >
          مسح الكل
        </button>
      </div>

      {/* Content Types */}
      <div className="mb-6">
        <h4 className="text-sm font-semibold text-gray-700 mb-3">نوع المحتوى</h4>
        <div className="space-y-2">
          {Object.values(ContentType).map((type) => (
            <label key={type} className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={selectedContentTypes.includes(type)}
                onChange={(e) => {
                  if (e.target.checked) {
                    setSelectedContentTypes([...selectedContentTypes, type]);
                  } else {
                    setSelectedContentTypes(
                      selectedContentTypes.filter((t) => t !== type)
                    );
                  }
                }}
                className="rounded border-gray-300 text-[#1B365D] focus:ring-[#1B365D]"
              />
              <span className="text-sm text-gray-700">{type}</span>
            </label>
          ))}
        </div>
      </div>

      {/* Authenticity Grades */}
      <div className="mb-6">
        <h4 className="text-sm font-semibold text-gray-700 mb-3">درجة الصحة</h4>
        <div className="space-y-2">
          {Object.values(AuthenticityGrade).map((grade) => (
            <label key={grade} className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={selectedAuthenticityGrades.includes(grade)}
                onChange={(e) => {
                  if (e.target.checked) {
                    setSelectedAuthenticityGrades([...selectedAuthenticityGrades, grade]);
                  } else {
                    setSelectedAuthenticityGrades(
                      selectedAuthenticityGrades.filter((g) => g !== grade)
                    );
                  }
                }}
                className="rounded border-gray-300 text-[#1B365D] focus:ring-[#1B365D]"
              />
              <span className="text-sm text-gray-700">{grade}</span>
            </label>
          ))}
        </div>
      </div>

      {/* Similarity Threshold */}
      <div className="mb-6">
        <h4 className="text-sm font-semibold text-gray-700 mb-3">
          الحد الأدنى للتطابق: {Math.round(minSimilarity * 100)}%
        </h4>
        <input
          type="range"
          min="0.3"
          max="0.9"
          step="0.05"
          value={minSimilarity}
          onChange={(e) => setMinSimilarity(parseFloat(e.target.value))}
          className="w-full"
        />
      </div>

      {/* Apply Button */}
      <button
        onClick={handleApply}
        className="w-full bg-[#1B365D] text-white py-3 rounded-lg hover:bg-[#2E4A6B] transition-colors font-semibold"
      >
        تطبيق الفلاتر
      </button>
    </div>
  );
}

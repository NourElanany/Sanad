'use client';

import type { QuerySuggestion } from '@/types/search';

interface SearchSuggestionsProps {
  suggestions: QuerySuggestion[];
  onSuggestionClick: (suggestion: QuerySuggestion) => void;
}

export function SearchSuggestions({
  suggestions,
  onSuggestionClick,
}: SearchSuggestionsProps) {
  return (
    <div className="absolute top-full left-0 right-0 mt-2 bg-white rounded-lg shadow-lg border border-gray-200 max-h-96 overflow-y-auto z-20">
      {suggestions.map((suggestion, index) => (
        <button
          key={index}
          onClick={() => onSuggestionClick(suggestion)}
          className="w-full text-right px-4 py-3 hover:bg-gray-50 transition-colors border-b border-gray-100 last:border-b-0"
        >
          <div className="flex items-center justify-between">
            <span className="text-gray-900">{suggestion.suggested_query}</span>
            <span className="text-xs text-gray-500">
              {suggestion.expected_results_count} نتيجة
            </span>
          </div>
          {suggestion.explanation && (
            <p className="text-sm text-gray-600 mt-1">{suggestion.explanation}</p>
          )}
        </button>
      ))}
    </div>
  );
}

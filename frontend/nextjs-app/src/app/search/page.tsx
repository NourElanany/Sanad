'use client';

/**
 * Comprehensive Search Page
 * Requirements: 8.1, 8.2, 8.3, 8.4, 8.5
 */

import { useState, useEffect, useCallback } from 'react';
import { SearchBar } from '@/components/search/SearchBar';
import { SearchFilters } from '@/components/search/SearchFilters';
import { SearchResults } from '@/components/search/SearchResults';
import { SearchSuggestions } from '@/components/search/SearchSuggestions';
import { SavedSearches } from '@/components/search/SavedSearches';
import { SearchService } from '@/lib/services/search-service';
import type {
  SearchRequest,
  SearchResponse,
  SearchFilters as SearchFiltersType,
  QuerySuggestion,
  SortBy,
} from '@/types/search';

export default function SearchPage() {
  const [query, setQuery] = useState('');
  const [response, setResponse] = useState<SearchResponse | null>(null);
  const [suggestions, setSuggestions] = useState<QuerySuggestion[]>([]);
  const [filters, setFilters] = useState<SearchFiltersType | undefined>();
  const [sortBy, setSortBy] = useState<SortBy | undefined>();
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showFilters, setShowFilters] = useState(false);
  const [showSavedSearches, setShowSavedSearches] = useState(false);
  const [showSuggestions, setShowSuggestions] = useState(false);

  // Debounced suggestions
  useEffect(() => {
    if (query.length >= 3) {
      const timer = setTimeout(async () => {
        try {
          const suggestions = await SearchService.getSuggestions(query);
          setSuggestions(suggestions);
          setShowSuggestions(true);
        } catch (error) {
          console.error('Failed to get suggestions:', error);
        }
      }, 300);

      return () => clearTimeout(timer);
    } else {
      setSuggestions([]);
      setShowSuggestions(false);
    }
  }, [query]);

  const performSearch = useCallback(
    async (searchQuery?: string) => {
      const q = searchQuery || query;
      if (!q.trim()) {
        setError('يرجى إدخال نص للبحث');
        return;
      }

      setIsLoading(true);
      setError(null);
      setShowSuggestions(false);

      try {
        const request: SearchRequest = {
          query: q,
          limit: 20,
          filters,
          sort_by: sortBy,
          include_suggestions: true,
        };

        const result = await SearchService.search(request);
        setResponse(result);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'حدث خطأ في البحث');
      } finally {
        setIsLoading(false);
      }
    },
    [query, filters, sortBy]
  );

  const handleVoiceSearch = async (audioBlob: Blob) => {
    setIsLoading(true);
    setError(null);

    try {
      const result = await SearchService.voiceSearch(audioBlob);
      setResponse(result);
      setQuery(result.search_metadata.query_processed);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'فشل البحث الصوتي');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSuggestionClick = (suggestion: QuerySuggestion) => {
    setQuery(suggestion.suggested_query);
    performSearch(suggestion.suggested_query);
  };

  const handleSaveSearch = async () => {
    if (!query.trim()) return;

    try {
      await SearchService.saveSearch(query, filters);
      alert('تم حفظ البحث بنجاح');
    } catch (err) {
      alert('فشل حفظ البحث');
    }
  };

  const handleLoadMore = async () => {
    if (!response?.pagination?.has_next_page || isLoading) return;

    setIsLoading(true);

    try {
      const request: SearchRequest = {
        query,
        page: response.pagination.next_page,
        page_size: response.pagination.page_size,
        filters,
        sort_by: sortBy,
      };

      const result = await SearchService.search(request);

      // Append new results
      setResponse({
        ...result,
        results: [...response.results, ...result.results],
      });
    } catch (err) {
      setError(err instanceof Error ? err.message : 'فشل تحميل المزيد');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50" dir="rtl">
      {/* Header */}
      <header className="bg-white shadow-sm sticky top-0 z-10">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex items-center justify-between mb-4">
            <h1 className="text-2xl font-bold text-[#1B365D]">البحث الشامل</h1>
            <div className="flex gap-2">
              <button
                onClick={() => setShowSavedSearches(!showSavedSearches)}
                className="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                title="البحثات المحفوظة"
              >
                <svg
                  className="w-6 h-6 text-[#1B365D]"
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
              </button>
              {query && (
                <button
                  onClick={handleSaveSearch}
                  className="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                  title="حفظ البحث"
                >
                  <svg
                    className="w-6 h-6 text-[#B8860B]"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M12 4v16m8-8H4"
                    />
                  </svg>
                </button>
              )}
            </div>
          </div>

          {/* Search Bar */}
          <SearchBar
            value={query}
            onChange={setQuery}
            onSearch={performSearch}
            onVoiceSearch={handleVoiceSearch}
            onFilterClick={() => setShowFilters(!showFilters)}
            hasActiveFilters={!!filters}
            isLoading={isLoading}
          />

          {/* Suggestions */}
          {showSuggestions && suggestions.length > 0 && (
            <SearchSuggestions
              suggestions={suggestions}
              onSuggestionClick={handleSuggestionClick}
            />
          )}
        </div>
      </header>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
        <div className="flex gap-6">
          {/* Filters Sidebar */}
          {showFilters && (
            <aside className="w-80 flex-shrink-0">
              <SearchFilters
                filters={filters}
                sortBy={sortBy}
                onFiltersChange={setFilters}
                onSortChange={setSortBy}
                onApply={() => performSearch()}
                onClear={() => {
                  setFilters(undefined);
                  setSortBy(undefined);
                }}
              />
            </aside>
          )}

          {/* Results */}
          <main className="flex-1">
            <SearchResults
              response={response}
              isLoading={isLoading}
              error={error}
              onLoadMore={handleLoadMore}
              onRetry={() => performSearch()}
            />
          </main>

          {/* Saved Searches Sidebar */}
          {showSavedSearches && (
            <aside className="w-80 flex-shrink-0">
              <SavedSearches
                onSearchSelect={(search) => {
                  setQuery(search.query);
                  setFilters(search.filters);
                  performSearch(search.query);
                }}
                onClose={() => setShowSavedSearches(false)}
              />
            </aside>
          )}
        </div>
      </div>
    </div>
  );
}

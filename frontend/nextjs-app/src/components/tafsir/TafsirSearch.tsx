'use client';

import React, { useState } from 'react';
import { TafsirService } from '@/lib/services/tafsir-service';
import type {
  TafsirSearchResponse,
  TafsirSearchCriteria,
  TafsirSourceType,
  ScholarlyAuthentication,
} from '@/types/tafsir';

export const TafsirSearch: React.FC = () => {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<TafsirSearchResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const [searchCriteria, setSearchCriteria] = useState<TafsirSearchCriteria[]>([
    TafsirSearchCriteria.TextContent,
  ]);
  
  const [filters, setFilters] = useState({
    sourceTypes: [] as TafsirSourceType[],
    authLevels: [] as ScholarlyAuthentication[],
    languages: [] as string[],
  });

  const handleSearch = async () => {
    if (!query.trim()) return;

    try {
      setLoading(true);
      setError(null);

      const response = await TafsirService.searchTafsir({
        query: query.trim(),
        search_criteria: searchCriteria,
        source_filters: {
          source_types: filters.sourceTypes.length > 0 ? filters.sourceTypes : undefined,
          authentication_levels: filters.authLevels.length > 0 ? filters.authLevels : undefined,
          languages: filters.languages.length > 0 ? filters.languages : undefined,
        },
        limit: 20,
        offset: 0,
      });

      setResults(response);
    } catch (err) {
      setError('فشل البحث في التفاسير');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const toggleCriteria = (criterion: TafsirSearchCriteria) => {
    if (searchCriteria.includes(criterion)) {
      setSearchCriteria(searchCriteria.filter((c) => c !== criterion));
    } else {
      setSearchCriteria([...searchCriteria, criterion]);
    }
  };

  const toggleSourceType = (type: TafsirSourceType) => {
    if (filters.sourceTypes.includes(type)) {
      setFilters({
        ...filters,
        sourceTypes: filters.sourceTypes.filter((t) => t !== type),
      });
    } else {
      setFilters({
        ...filters,
        sourceTypes: [...filters.sourceTypes, type],
      });
    }
  };

  const getCriteriaLabel = (criterion: TafsirSearchCriteria) => {
    const labels = {
      [TafsirSearchCriteria.TextContent]: '📝 النص',
      [TafsirSearchCriteria.Themes]: '🏷️ المواضيع',
      [TafsirSearchCriteria.CrossReferences]: '🔗 المراجع',
      [TafsirSearchCriteria.AuthorName]: '✍️ المؤلف',
      [TafsirSearchCriteria.Methodology]: '📚 المنهجية',
    };
    return labels[criterion];
  };

  const getSourceTypeLabel = (type: TafsirSourceType) => {
    const labels = {
      [TafsirSourceType.Classical]: '📚 كلاسيكي',
      [TafsirSourceType.Contemporary]: '📖 معاصر',
      [TafsirSourceType.Linguistic]: '🔤 لغوي',
      [TafsirSourceType.Thematic]: '🎯 موضوعي',
      [TafsirSourceType.Sectarian]: '🕌 مذهبي',
    };
    return labels[type];
  };

  return (
    <div className="space-y-6">
      {/* Search Input */}
      <div className="bg-gray-50 rounded-lg p-4">
        <div className="flex gap-2 mb-4">
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
            placeholder="ابحث في التفاسير..."
            className="flex-1 px-4 py-3 border-2 border-gray-300 rounded-lg focus:border-[#1B365D] focus:outline-none font-['Tajawal']"
            dir="rtl"
          />
          <button
            onClick={handleSearch}
            disabled={loading || !query.trim()}
            className="px-6 py-3 bg-[#1B365D] text-white rounded-lg hover:bg-[#0F1F35] disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
          >
            {loading ? '⏳' : '🔍'} بحث
          </button>
        </div>

        {/* Search Criteria */}
        <div className="mb-4">
          <h5 className="text-sm font-bold text-gray-700 mb-2 font-['Tajawal']" dir="rtl">
            البحث في:
          </h5>
          <div className="flex flex-wrap gap-2">
            {Object.values(TafsirSearchCriteria).map((criterion) => (
              <button
                key={criterion}
                onClick={() => toggleCriteria(criterion)}
                className={`px-3 py-1 rounded-lg text-sm transition-colors ${
                  searchCriteria.includes(criterion)
                    ? 'bg-[#1B365D] text-white'
                    : 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-100'
                }`}
              >
                {getCriteriaLabel(criterion)}
              </button>
            ))}
          </div>
        </div>

        {/* Source Type Filters */}
        <div>
          <h5 className="text-sm font-bold text-gray-700 mb-2 font-['Tajawal']" dir="rtl">
            نوع المصدر:
          </h5>
          <div className="flex flex-wrap gap-2">
            {Object.values(TafsirSourceType).map((type) => (
              <button
                key={type}
                onClick={() => toggleSourceType(type)}
                className={`px-3 py-1 rounded-lg text-sm transition-colors ${
                  filters.sourceTypes.includes(type)
                    ? 'bg-[#2D5A27] text-white'
                    : 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-100'
                }`}
              >
                {getSourceTypeLabel(type)}
              </button>
            ))}
          </div>
        </div>
      </div>

      {/* Loading State */}
      {loading && (
        <div className="flex items-center justify-center py-12">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-[#1B365D]"></div>
        </div>
      )}

      {/* Error State */}
      {error && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-4 text-red-700">
          {error}
        </div>
      )}

      {/* Results */}
      {results && !loading && (
        <div className="space-y-4">
          {/* Results Header */}
          <div className="flex items-center justify-between">
            <h4 className="text-lg font-bold text-[#1B365D] font-['Tajawal']" dir="rtl">
              📊 النتائج: {results.total_count} نتيجة
            </h4>
            <span className="text-sm text-gray-600">
              ⏱️ {results.search_time_ms}ms
            </span>
          </div>

          {/* Facets */}
          {results.facets && (
            <div className="bg-gray-50 rounded-lg p-4">
              <h5 className="text-sm font-bold text-gray-700 mb-3 font-['Tajawal']" dir="rtl">
                تصنيف النتائج:
              </h5>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                {results.facets.source_types.length > 0 && (
                  <div>
                    <p className="text-xs text-gray-600 mb-1" dir="rtl">
                      حسب النوع:
                    </p>
                    {results.facets.source_types.map((facet) => (
                      <div key={facet.value} className="text-sm text-gray-700" dir="rtl">
                        {facet.value}: {facet.count}
                      </div>
                    ))}
                  </div>
                )}
                {results.facets.authors.length > 0 && (
                  <div>
                    <p className="text-xs text-gray-600 mb-1" dir="rtl">
                      حسب المؤلف:
                    </p>
                    {results.facets.authors.slice(0, 3).map((facet) => (
                      <div key={facet.value} className="text-sm text-gray-700" dir="rtl">
                        {facet.value}: {facet.count}
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>
          )}

          {/* Search Results */}
          <div className="space-y-4">
            {results.results.map((result, index) => (
              <div
                key={index}
                className="bg-white border-2 border-gray-200 rounded-xl p-6 hover:border-[#1B365D] transition-colors"
              >
                {/* Result Header */}
                <div className="flex items-start justify-between mb-3">
                  <div className="flex-1">
                    <h5 className="text-lg font-bold text-[#1B365D] font-['Tajawal']" dir="rtl">
                      {result.source.name}
                    </h5>
                    <p className="text-sm text-gray-600" dir="rtl">
                      {result.source.author} • سورة {result.surah.name} آية {result.ayah.ayah_number}
                    </p>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-bold text-[#B8860B]">
                      ⭐ {result.source.credibility_score.toFixed(1)}
                    </div>
                    <div className="text-xs text-gray-500">
                      صلة: {(result.relevance_score * 100).toFixed(0)}%
                    </div>
                  </div>
                </div>

                {/* Highlighted Text */}
                <div
                  className="text-gray-800 leading-relaxed mb-3 font-['Tajawal']"
                  dir="rtl"
                  dangerouslySetInnerHTML={{ __html: result.highlighted_text }}
                />

                {/* Matching Criteria */}
                {result.matching_criteria.length > 0 && (
                  <div className="flex flex-wrap gap-2">
                    {result.matching_criteria.map((criterion, critIndex) => (
                      <span
                        key={critIndex}
                        className="px-2 py-1 bg-blue-100 text-blue-800 rounded text-xs"
                      >
                        {criterion}
                      </span>
                    ))}
                  </div>
                )}
              </div>
            ))}
          </div>

          {/* No Results */}
          {results.results.length === 0 && (
            <div className="text-center py-12 text-gray-500">
              <p className="text-lg">لم يتم العثور على نتائج</p>
              <p className="text-sm mt-2">جرب استخدام كلمات مفتاحية مختلفة</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

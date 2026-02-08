'use client';

import { useState, useEffect } from 'react';
import { HadithService } from '@/lib/services/hadith-service';
import { HadithBookCard } from '@/components/hadith/HadithBookCard';
import { HadithSearchBar } from '@/components/hadith/HadithSearchBar';
import { HadithFilters } from '@/components/hadith/HadithFilters';
import { HadithSearchResults } from '@/components/hadith/HadithSearchResults';
import type {
  HadithBook,
  HadithSearchResponse,
  HadithSearchFilters,
} from '@/types/hadith';

export default function HadithLibraryPage() {
  const [activeTab, setActiveTab] = useState<'collections' | 'topics' | 'narrators'>('collections');
  const [books, setBooks] = useState<HadithBook[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [searchResults, setSearchResults] = useState<HadithSearchResponse | null>(null);
  const [filters, setFilters] = useState<HadithSearchFilters>({});
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showFilters, setShowFilters] = useState(false);

  // Load hadith books on mount
  useEffect(() => {
    loadBooks();
  }, []);

  const loadBooks = async () => {
    try {
      setIsLoading(true);
      const booksData = await HadithService.getHadithBooks();
      setBooks(booksData);
    } catch (err) {
      setError('فشل تحميل مجموعات الأحاديث');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSearch = async (query: string) => {
    if (!query.trim()) {
      setSearchResults(null);
      return;
    }

    try {
      setIsLoading(true);
      setError(null);
      const results = await HadithService.searchHadiths(query, filters);
      setSearchResults(results);
      setSearchQuery(query);
    } catch (err) {
      setError('فشل البحث في الأحاديث');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleClearSearch = () => {
    setSearchQuery('');
    setSearchResults(null);
  };

  const handleFilterChange = (newFilters: HadithSearchFilters) => {
    setFilters(newFilters);
    if (searchQuery) {
      handleSearch(searchQuery);
    }
  };

  const topics = [
    'عقيدة',
    'عبادة',
    'معاملات',
    'أسرة',
    'أخلاق',
    'تاريخ',
    'نبوءات',
    'فقه',
  ];

  const narrators = [
    'أبو هريرة',
    'عائشة',
    'ابن عمر',
    'أنس بن مالك',
    'جابر بن عبد الله',
    'أبو سعيد الخدري',
  ];

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <h1 className="text-3xl font-bold text-gray-900 text-center font-tajawal">
            مكتبة الأحاديث النبوية
          </h1>
        </div>
      </div>

      {/* Search Bar */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
        <div className="flex gap-4">
          <div className="flex-1">
            <HadithSearchBar
              onSearch={handleSearch}
              onClear={handleClearSearch}
              placeholder="ابحث في الأحاديث..."
            />
          </div>
          <button
            onClick={() => setShowFilters(!showFilters)}
            className="px-6 py-3 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors flex items-center gap-2"
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
            <span className="font-tajawal">الفلاتر</span>
          </button>
        </div>

        {/* Filters Panel */}
        {showFilters && (
          <div className="mt-4">
            <HadithFilters
              filters={filters}
              books={books}
              onFilterChange={handleFilterChange}
            />
          </div>
        )}
      </div>

      {/* Content */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
        {searchResults ? (
          <HadithSearchResults results={searchResults} />
        ) : (
          <>
            {/* Tabs */}
            <div className="border-b border-gray-200 mb-6">
              <nav className="-mb-px flex space-x-8 space-x-reverse" dir="rtl">
                <button
                  onClick={() => setActiveTab('collections')}
                  className={`${
                    activeTab === 'collections'
                      ? 'border-[#1B365D] text-[#1B365D]'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  } whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm font-tajawal`}
                >
                  المجموعات
                </button>
                <button
                  onClick={() => setActiveTab('topics')}
                  className={`${
                    activeTab === 'topics'
                      ? 'border-[#1B365D] text-[#1B365D]'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  } whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm font-tajawal`}
                >
                  المواضيع
                </button>
                <button
                  onClick={() => setActiveTab('narrators')}
                  className={`${
                    activeTab === 'narrators'
                      ? 'border-[#1B365D] text-[#1B365D]'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  } whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm font-tajawal`}
                >
                  الرواة
                </button>
              </nav>
            </div>

            {/* Tab Content */}
            {isLoading ? (
              <div className="flex justify-center items-center py-12">
                <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-[#1B365D]"></div>
              </div>
            ) : error ? (
              <div className="text-center py-12">
                <div className="text-red-500 text-lg font-tajawal">{error}</div>
              </div>
            ) : (
              <>
                {activeTab === 'collections' && (
                  <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {books.map((book) => (
                      <HadithBookCard key={book.id} book={book} />
                    ))}
                  </div>
                )}

                {activeTab === 'topics' && (
                  <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                    {topics.map((topic) => (
                      <a
                        key={topic}
                        href={`/hadith/topic/${encodeURIComponent(topic)}`}
                        className="bg-white border border-gray-200 rounded-lg p-6 hover:shadow-lg transition-shadow text-center"
                      >
                        <h3 className="text-lg font-bold text-gray-900 font-tajawal">
                          {topic}
                        </h3>
                      </a>
                    ))}
                  </div>
                )}

                {activeTab === 'narrators' && (
                  <div className="space-y-3">
                    {narrators.map((narrator) => (
                      <a
                        key={narrator}
                        href={`/hadith/narrator/${encodeURIComponent(narrator)}`}
                        className="block bg-white border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
                      >
                        <div className="flex items-center gap-4">
                          <div className="w-12 h-12 bg-[#1B365D] rounded-full flex items-center justify-center">
                            <svg
                              className="w-6 h-6 text-white"
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
                          </div>
                          <div className="flex-1">
                            <h3 className="text-lg font-bold text-gray-900 font-tajawal">
                              {narrator}
                            </h3>
                          </div>
                          <svg
                            className="w-5 h-5 text-gray-400"
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
                      </a>
                    ))}
                  </div>
                )}
              </>
            )}
          </>
        )}
      </div>
    </div>
  );
}

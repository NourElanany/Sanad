'use client';

/**
 * Quran Index Page - Main page for browsing Quran content
 * Implements Requirements 5.1 and 5.2
 */
import { useState, useEffect, useMemo } from 'react';
import { QuranService } from '@/lib/services/quran-service';
import type { Surah, Juz, QuranBookmark, QuranFilters } from '@/types/quran';
import { SurahList } from '@/components/quran/SurahList';
import { JuzList } from '@/components/quran/JuzList';
import { BookmarkList } from '@/components/quran/BookmarkList';
import { QuranSearchBar } from '@/components/quran/QuranSearchBar';
import { QuranFilters as QuranFiltersComponent } from '@/components/quran/QuranFilters';

type TabType = 'surahs' | 'juzs' | 'bookmarks';

export default function QuranIndexPage() {
  const [activeTab, setActiveTab] = useState<TabType>('surahs');
  const [surahs, setSurahs] = useState<Surah[]>([]);
  const [juzs, setJuzs] = useState<Juz[]>([]);
  const [bookmarks, setBookmarks] = useState<QuranBookmark[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  const [filters, setFilters] = useState<QuranFilters>({
    revelationType: 'all',
    ayahCountRange: 'all',
    searchQuery: '',
  });

  // Load data on mount
  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const [surahsData, juzsData, bookmarksData] = await Promise.all([
        QuranService.getSurahs(),
        QuranService.getJuzs(),
        QuranService.getBookmarks().catch(() => []), // Bookmarks may fail if not authenticated
      ]);
      
      setSurahs(surahsData);
      setJuzs(juzsData);
      setBookmarks(bookmarksData);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load data');
    } finally {
      setIsLoading(false);
    }
  };

  // Filter surahs based on current filters
  const filteredSurahs = useMemo(() => {
    let filtered = surahs;

    // Apply search filter
    if (filters.searchQuery) {
      const query = filters.searchQuery.toLowerCase();
      filtered = filtered.filter(
        (surah) =>
          surah.name_arabic.includes(query) ||
          surah.name_english.toLowerCase().includes(query) ||
          surah.name_transliteration.toLowerCase().includes(query) ||
          surah.number.toString().includes(query)
      );
    }

    // Apply revelation type filter
    if (filters.revelationType !== 'all') {
      filtered = filtered.filter(
        (surah) =>
          surah.revelation_type.toLowerCase() === filters.revelationType
      );
    }

    // Apply ayah count filter
    if (filters.ayahCountRange !== 'all') {
      const [min, max] = filters.ayahCountRange.split('-').map(Number);
      filtered = filtered.filter(
        (surah) => surah.ayah_count >= min && surah.ayah_count <= max
      );
    }

    return filtered;
  }, [surahs, filters]);

  const handleDeleteBookmark = async (bookmarkId: string) => {
    try {
      await QuranService.deleteBookmark(bookmarkId);
      setBookmarks(bookmarks.filter((b) => b.id !== bookmarkId));
    } catch (err) {
      console.error('Failed to delete bookmark:', err);
    }
  };

  if (isLoading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-16 w-16 border-b-2 border-[#1B365D] mx-auto mb-4"></div>
          <p className="text-gray-600">جاري التحميل...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center max-w-md mx-auto p-6">
          <div className="text-red-500 text-6xl mb-4">⚠️</div>
          <h2 className="text-2xl font-bold text-gray-800 mb-2">حدث خطأ</h2>
          <p className="text-gray-600 mb-6">{error}</p>
          <button
            onClick={loadData}
            className="bg-[#1B365D] text-white px-6 py-3 rounded-lg hover:bg-[#2E4A6B] transition-colors"
          >
            إعادة المحاولة
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50" dir="rtl">
      {/* Header */}
      <header className="bg-[#1B365D] text-white shadow-lg">
        <div className="container mx-auto px-4 py-6">
          <h1 className="text-3xl font-bold text-center mb-6">القرآن الكريم</h1>
          
          {/* Search Bar */}
          <QuranSearchBar
            value={filters.searchQuery}
            onChange={(query) => setFilters({ ...filters, searchQuery: query })}
          />
          
          {/* Filters */}
          <QuranFiltersComponent
            filters={filters}
            onChange={setFilters}
          />
        </div>
      </header>

      {/* Tabs */}
      <div className="bg-white border-b border-gray-200 sticky top-0 z-10">
        <div className="container mx-auto px-4">
          <div className="flex space-x-reverse space-x-8">
            <button
              onClick={() => setActiveTab('surahs')}
              className={`py-4 px-6 font-semibold border-b-2 transition-colors ${
                activeTab === 'surahs'
                  ? 'border-[#B8860B] text-[#1B365D]'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              السور ({filteredSurahs.length})
            </button>
            <button
              onClick={() => setActiveTab('juzs')}
              className={`py-4 px-6 font-semibold border-b-2 transition-colors ${
                activeTab === 'juzs'
                  ? 'border-[#B8860B] text-[#1B365D]'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              الأجزاء ({juzs.length})
            </button>
            <button
              onClick={() => setActiveTab('bookmarks')}
              className={`py-4 px-6 font-semibold border-b-2 transition-colors ${
                activeTab === 'bookmarks'
                  ? 'border-[#B8860B] text-[#1B365D]'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              المفضلة ({bookmarks.length})
            </button>
          </div>
        </div>
      </div>

      {/* Content */}
      <main className="container mx-auto px-4 py-8">
        {activeTab === 'surahs' && (
          <SurahList surahs={filteredSurahs} />
        )}
        {activeTab === 'juzs' && (
          <JuzList juzs={juzs} />
        )}
        {activeTab === 'bookmarks' && (
          <BookmarkList
            bookmarks={bookmarks}
            onDelete={handleDeleteBookmark}
          />
        )}
      </main>
    </div>
  );
}

'use client';

import { useState, useEffect } from 'react';
import { storiesService } from '@/lib/services/stories-service';
import type { Story, StoryCategory } from '@/types/stories';

type TabType = 'prophets' | 'companions' | 'successors' | 'all';

const categoryMap: Record<TabType, StoryCategory | undefined> = {
  prophets: 'prophets',
  companions: 'companions',
  successors: 'successors',
  all: undefined,
};

export default function StoriesLibraryClient() {
  const [activeTab, setActiveTab] = useState<TabType>('all');
  const [stories, setStories] = useState<Story[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');

  useEffect(() => {
    loadStories();
  }, [activeTab]);

  const loadStories = async () => {
    setIsLoading(true);
    setError(null);

    try {
      const category = categoryMap[activeTab];
      let storiesData: Story[];

      if (category) {
        storiesData = await storiesService.getStoriesByCategory(category, {
          limit: 50,
          offset: 0,
        });
      } else {
        // Load all stories
        const [prophets, companions, successors] = await Promise.all([
          storiesService.getStoriesByCategory('prophets', { limit: 20 }),
          storiesService.getStoriesByCategory('companions', { limit: 20 }),
          storiesService.getStoriesByCategory('successors', { limit: 20 }),
        ]);
        storiesData = [...prophets, ...companions, ...successors];
      }

      setStories(storiesData);
    } catch (err) {
      setError('فشل تحميل القصص');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSearch = async () => {
    if (!searchQuery.trim()) {
      loadStories();
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const results = await storiesService.searchStories(searchQuery, {}, { limit: 50 });
      setStories(results.stories);
    } catch (err) {
      setError('فشل البحث في القصص');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const getCategoryIcon = (category: string) => {
    switch (category) {
      case 'prophets':
        return '📖';
      case 'companions':
        return '⭐';
      case 'successors':
        return '🌟';
      default:
        return '📚';
    }
  };

  const getCategoryColor = (category: string) => {
    switch (category) {
      case 'prophets':
        return 'bg-blue-100 text-blue-800';
      case 'companions':
        return 'bg-green-100 text-green-800';
      case 'successors':
        return 'bg-purple-100 text-purple-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  return (
    <div className="min-h-screen bg-gray-50" dir="rtl">
      {/* Header */}
      <header className="bg-[#1B365D] text-white shadow-lg">
        <div className="container mx-auto px-4 py-6">
          <h1 className="text-3xl font-bold text-center mb-6 font-['Tajawal']">
            مكتبة القصص الإسلامية
          </h1>

          {/* Search Bar */}
          <div className="max-w-2xl mx-auto">
            <div className="flex gap-2">
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
                placeholder="ابحث في القصص..."
                className="flex-1 px-4 py-3 rounded-lg text-gray-900 focus:outline-none focus:ring-2 focus:ring-[#B8860B]"
              />
              <button
                onClick={handleSearch}
                className="px-6 py-3 bg-[#B8860B] hover:bg-[#DAA520] rounded-lg transition-colors font-semibold"
              >
                بحث
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Tabs */}
      <div className="bg-white border-b border-gray-200 sticky top-0 z-10">
        <div className="container mx-auto px-4">
          <div className="flex space-x-reverse space-x-8">
            <button
              onClick={() => setActiveTab('all')}
              className={`py-4 px-6 font-semibold border-b-2 transition-colors ${
                activeTab === 'all'
                  ? 'border-[#B8860B] text-[#1B365D]'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              جميع القصص
            </button>
            <button
              onClick={() => setActiveTab('prophets')}
              className={`py-4 px-6 font-semibold border-b-2 transition-colors ${
                activeTab === 'prophets'
                  ? 'border-[#B8860B] text-[#1B365D]'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              📖 قصص الأنبياء
            </button>
            <button
              onClick={() => setActiveTab('companions')}
              className={`py-4 px-6 font-semibold border-b-2 transition-colors ${
                activeTab === 'companions'
                  ? 'border-[#B8860B] text-[#1B365D]'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              ⭐ قصص الصحابة
            </button>
            <button
              onClick={() => setActiveTab('successors')}
              className={`py-4 px-6 font-semibold border-b-2 transition-colors ${
                activeTab === 'successors'
                  ? 'border-[#B8860B] text-[#1B365D]'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              🌟 قصص التابعين
            </button>
          </div>
        </div>
      </div>

      {/* Content */}
      <main className="container mx-auto px-4 py-8">
        {isLoading && (
          <div className="flex justify-center items-center py-12">
            <div className="animate-spin rounded-full h-16 w-16 border-b-2 border-[#1B365D]"></div>
          </div>
        )}

        {error && (
          <div className="text-center py-12">
            <div className="text-red-500 text-6xl mb-4">⚠️</div>
            <p className="text-red-600 text-lg">{error}</p>
            <button
              onClick={loadStories}
              className="mt-4 px-6 py-3 bg-[#1B365D] text-white rounded-lg hover:bg-[#2E4A6B] transition-colors"
            >
              إعادة المحاولة
            </button>
          </div>
        )}

        {!isLoading && !error && stories.length === 0 && (
          <div className="text-center py-12">
            <div className="text-gray-400 text-6xl mb-4">📚</div>
            <p className="text-gray-600 text-lg">لا توجد قصص متاحة</p>
          </div>
        )}

        {!isLoading && !error && stories.length > 0 && (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {stories.map((story) => (
              <a
                key={story.id}
                href={`/stories/${story.id}`}
                className="bg-white rounded-lg shadow-md hover:shadow-xl transition-shadow overflow-hidden"
              >
                <div className="p-6">
                  <div className="flex items-start justify-between mb-4">
                    <div className="flex-1">
                      <h3 className="text-xl font-bold text-gray-900 mb-2 font-['Tajawal']">
                        {story.title_arabic}
                      </h3>
                      {story.title_english && (
                        <p className="text-sm text-gray-500 mb-2">
                          {story.title_english}
                        </p>
                      )}
                    </div>
                    <span className="text-3xl">
                      {getCategoryIcon(story.category)}
                    </span>
                  </div>

                  <p className="text-gray-600 mb-4 line-clamp-3 leading-relaxed">
                    {story.summary}
                  </p>

                  <div className="flex flex-wrap gap-2 mb-4">
                    <span
                      className={`px-3 py-1 rounded-full text-xs font-semibold ${getCategoryColor(
                        story.category
                      )}`}
                    >
                      {story.category === 'prophets' && 'قصص الأنبياء'}
                      {story.category === 'companions' && 'قصص الصحابة'}
                      {story.category === 'successors' && 'قصص التابعين'}
                    </span>
                    {story.age_group && (
                      <span className="px-3 py-1 rounded-full text-xs font-semibold bg-orange-100 text-orange-800">
                        {story.age_group}
                      </span>
                    )}
                  </div>

                  <div className="flex items-center justify-between text-sm text-gray-500">
                    <span>📖 {story.word_count || 0} كلمة</span>
                    {story.authenticity_level && (
                      <span
                        className={`font-semibold ${
                          story.authenticity_level === 'sahih'
                            ? 'text-green-600'
                            : story.authenticity_level === 'hasan'
                            ? 'text-yellow-600'
                            : 'text-gray-600'
                        }`}
                      >
                        {story.authenticity_level === 'sahih' && '✓ صحيح'}
                        {story.authenticity_level === 'hasan' && '~ حسن'}
                        {story.authenticity_level === 'weak' && '⚠ ضعيف'}
                      </span>
                    )}
                  </div>
                </div>
              </a>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}

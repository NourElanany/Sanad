'use client';

import React, { useState, useEffect } from 'react';
import { TafsirService } from '@/lib/services/tafsir-service';
import type { TafsirWithSource, TafsirSource, TafsirDisplayPreferences } from '@/types/tafsir';
import { TafsirSourceSelector } from './TafsirSourceSelector';
import { TafsirContent } from './TafsirContent';
import { TafsirComparison } from './TafsirComparison';
import { TafsirSearch } from './TafsirSearch';

interface TafsirViewerProps {
  surahNumber: number;
  ayahNumber: number;
  arabicText: string;
  onClose?: () => void;
}

export const TafsirViewer: React.FC<TafsirViewerProps> = ({
  surahNumber,
  ayahNumber,
  arabicText,
  onClose,
}) => {
  const [tafsirs, setTafsirs] = useState<TafsirWithSource[]>([]);
  const [sources, setSources] = useState<TafsirSource[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'view' | 'compare' | 'search'>('view');
  
  const [preferences, setPreferences] = useState<TafsirDisplayPreferences>({
    selected_sources: [],
    layout: 'stacked',
    show_cross_references: true,
    show_themes: true,
    font_size: 'medium',
  });

  // Load tafsir sources on mount
  useEffect(() => {
    loadTafsirSources();
  }, []);

  // Load tafsir when sources are selected
  useEffect(() => {
    if (preferences.selected_sources.length > 0) {
      loadTafsir();
    }
  }, [surahNumber, ayahNumber, preferences.selected_sources]);

  const loadTafsirSources = async () => {
    try {
      const sourcesData = await TafsirService.getTafsirSources();
      setSources(sourcesData);
      
      // Auto-select top 2 highly credible sources
      const topSources = sourcesData
        .filter((s) => s.credibility_score >= 8.0)
        .slice(0, 2)
        .map((s) => s.id);
      
      setPreferences((prev) => ({
        ...prev,
        selected_sources: topSources,
      }));
    } catch (err) {
      setError('Failed to load tafsir sources');
      console.error(err);
    }
  };

  const loadTafsir = async () => {
    try {
      setLoading(true);
      setError(null);

      // Try to get from cache first
      const cached = TafsirService.getCachedTafsir(surahNumber, ayahNumber);
      if (cached) {
        const filtered = cached.filter((t) =>
          preferences.selected_sources.includes(t.source.id)
        );
        if (filtered.length > 0) {
          setTafsirs(filtered);
          setLoading(false);
          return;
        }
      }

      // Fetch from API
      const data = await TafsirService.getTafsirForAyah(
        surahNumber,
        ayahNumber,
        preferences.selected_sources
      );
      
      setTafsirs(data);
      
      // Cache the results
      TafsirService.cacheTafsir(surahNumber, ayahNumber, data);
    } catch (err) {
      setError('Failed to load tafsir');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleSourcesChange = (sourceIds: string[]) => {
    setPreferences((prev) => ({
      ...prev,
      selected_sources: sourceIds,
    }));
  };

  const handleLayoutChange = (layout: 'stacked' | 'side-by-side' | 'tabbed') => {
    setPreferences((prev) => ({
      ...prev,
      layout,
    }));
  };

  const handleDownloadForOffline = async () => {
    try {
      await TafsirService.downloadTafsirForOffline(
        surahNumber,
        preferences.selected_sources
      );
      alert('تم تحميل التفاسير للعمل دون اتصال');
    } catch (err) {
      alert('فشل تحميل التفاسير');
      console.error(err);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
      <div className="bg-white rounded-2xl shadow-2xl max-w-6xl w-full max-h-[90vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="bg-gradient-to-r from-[#1B365D] to-[#2D5A27] text-white p-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-2xl font-bold font-['Tajawal']">
              التفسير - سورة {surahNumber} آية {ayahNumber}
            </h2>
            {onClose && (
              <button
                onClick={onClose}
                className="text-white hover:text-gray-200 transition-colors"
                aria-label="إغلاق"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            )}
          </div>
          
          {/* Arabic Text */}
          <div className="bg-white bg-opacity-10 rounded-lg p-4 mb-4">
            <p className="text-center text-2xl leading-loose font-['KFGQPC_Uthman_Taha_Naskh']" dir="rtl">
              {arabicText}
            </p>
          </div>

          {/* Tabs */}
          <div className="flex gap-2">
            <button
              onClick={() => setActiveTab('view')}
              className={`px-4 py-2 rounded-lg transition-colors ${
                activeTab === 'view'
                  ? 'bg-white text-[#1B365D]'
                  : 'bg-white bg-opacity-20 text-white hover:bg-opacity-30'
              }`}
            >
              عرض التفاسير
            </button>
            <button
              onClick={() => setActiveTab('compare')}
              className={`px-4 py-2 rounded-lg transition-colors ${
                activeTab === 'compare'
                  ? 'bg-white text-[#1B365D]'
                  : 'bg-white bg-opacity-20 text-white hover:bg-opacity-30'
              }`}
            >
              مقارنة التفاسير
            </button>
            <button
              onClick={() => setActiveTab('search')}
              className={`px-4 py-2 rounded-lg transition-colors ${
                activeTab === 'search'
                  ? 'bg-white text-[#1B365D]'
                  : 'bg-white bg-opacity-20 text-white hover:bg-opacity-30'
              }`}
            >
              البحث في التفاسير
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6">
          {/* Source Selector */}
          <div className="mb-6">
            <TafsirSourceSelector
              sources={sources}
              selectedSources={preferences.selected_sources}
              onSourcesChange={handleSourcesChange}
              onDownloadOffline={handleDownloadForOffline}
            />
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

          {/* Content Tabs */}
          {!loading && !error && (
            <>
              {activeTab === 'view' && (
                <TafsirContent
                  tafsirs={tafsirs}
                  preferences={preferences}
                  onLayoutChange={handleLayoutChange}
                />
              )}
              
              {activeTab === 'compare' && (
                <TafsirComparison
                  surahNumber={surahNumber}
                  ayahNumber={ayahNumber}
                  selectedSources={preferences.selected_sources}
                />
              )}
              
              {activeTab === 'search' && (
                <TafsirSearch />
              )}
            </>
          )}
        </div>
      </div>
    </div>
  );
};

'use client';

import { useEffect, useState, useRef } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { QuranService } from '@/lib/services/quran-service';
import type { QuranPage, Ayah } from '@/types/quran';
import MushafPageView from '@/components/quran/MushafPageView';
import AyahOptionsModal from '@/components/quran/AyahOptionsModal';

/**
 * Mushaf View Page - High-quality Quran reading interface
 * 
 * Features:
 * - Page-based Quran display with high-quality typography
 * - Smooth page navigation with keyboard shortcuts
 * - Zoom and pan functionality for text
 * - Verse highlighting on click
 * - Automatic reading position saving
 */
export default function MushafPage() {
  const params = useParams();
  const router = useRouter();
  const pageNumber = parseInt(params.page as string) || 1;
  
  const [currentPage, setCurrentPage] = useState<QuranPage | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedAyah, setSelectedAyah] = useState<Ayah | null>(null);
  const [showControls, setShowControls] = useState(true);
  const [fontSize, setFontSize] = useState(24);
  const [scale, setScale] = useState(1);
  
  const containerRef = useRef<HTMLDivElement>(null);
  const controlsTimeoutRef = useRef<NodeJS.Timeout>();
  
  const TOTAL_PAGES = 604;

  useEffect(() => {
    loadPage(pageNumber);
  }, [pageNumber]);

  useEffect(() => {
    // Keyboard navigation
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'ArrowLeft' && pageNumber < TOTAL_PAGES) {
        navigateToPage(pageNumber + 1);
      } else if (e.key === 'ArrowRight' && pageNumber > 1) {
        navigateToPage(pageNumber - 1);
      } else if (e.key === 'Home') {
        navigateToPage(1);
      } else if (e.key === 'End') {
        navigateToPage(TOTAL_PAGES);
      } else if (e.key === '+' || e.key === '=') {
        adjustFontSize(2);
      } else if (e.key === '-' || e.key === '_') {
        adjustFontSize(-2);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [pageNumber]);

  const loadPage = async (page: number) => {
    setLoading(true);
    setError(null);
    try {
      const pageData = await QuranService.getPage(page);
      setCurrentPage(pageData);
      saveReadingPosition(page);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load page');
    } finally {
      setLoading(false);
    }
  };

  const saveReadingPosition = async (page: number) => {
    try {
      await QuranService.updateReadingProgress({
        surah_number: currentPage?.surah_number || 1,
        ayah_number: currentPage?.ayahs[0]?.number_in_surah || 1,
        page_number: page,
      });
    } catch (err) {
      console.error('Failed to save reading position:', err);
    }
  };

  const navigateToPage = (page: number) => {
    if (page >= 1 && page <= TOTAL_PAGES) {
      router.push(`/quran/mushaf/${page}`);
    }
  };

  const handleAyahClick = (ayah: Ayah) => {
    setSelectedAyah(ayah);
  };

  const adjustFontSize = (delta: number) => {
    setFontSize(prev => Math.max(16, Math.min(40, prev + delta)));
  };

  const adjustScale = (delta: number) => {
    setScale(prev => Math.max(1, Math.min(3, prev + delta)));
  };

  const toggleControls = () => {
    setShowControls(prev => !prev);
    
    // Auto-hide controls after 3 seconds
    if (controlsTimeoutRef.current) {
      clearTimeout(controlsTimeoutRef.current);
    }
    controlsTimeoutRef.current = setTimeout(() => {
      setShowControls(false);
    }, 3000);
  };

  const handlePageJump = () => {
    const page = prompt(`الانتقال إلى صفحة (1-${TOTAL_PAGES}):`);
    const pageNum = parseInt(page || '');
    if (pageNum && pageNum >= 1 && pageNum <= TOTAL_PAGES) {
      navigateToPage(pageNum);
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-[#FEFEFE] flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-16 w-16 border-b-2 border-[#1B365D] mx-auto"></div>
          <p className="mt-4 text-[#1B365D] font-tajawal">جاري التحميل...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-[#FEFEFE] flex items-center justify-center">
        <div className="text-center">
          <div className="text-red-500 text-5xl mb-4">⚠️</div>
          <p className="text-[#1B365D] font-tajawal mb-4">{error}</p>
          <button
            onClick={() => loadPage(pageNumber)}
            className="px-6 py-2 bg-[#1B365D] text-white rounded-lg hover:bg-[#2E4A6B] transition-colors font-tajawal"
          >
            إعادة المحاولة
          </button>
        </div>
      </div>
    );
  }

  return (
    <div 
      ref={containerRef}
      className="min-h-screen bg-[#FEFEFE] relative"
      onClick={toggleControls}
    >
      {/* Top Controls */}
      {showControls && (
        <div className="fixed top-0 left-0 right-0 z-50 bg-gradient-to-b from-black/60 to-transparent">
          <div className="container mx-auto px-4 py-3 flex items-center justify-between">
            <button
              onClick={(e) => {
                e.stopPropagation();
                router.back();
              }}
              className="text-white hover:text-gray-200 transition-colors"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
              </svg>
            </button>
            
            <div className="text-white font-tajawal font-bold">
              صفحة {pageNumber} من {TOTAL_PAGES}
            </div>
            
            <button
              onClick={(e) => {
                e.stopPropagation();
                // Show bookmarks
              }}
              className="text-white hover:text-gray-200 transition-colors"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z" />
              </svg>
            </button>
          </div>
        </div>
      )}

      {/* Main Content */}
      <div className="container mx-auto px-4 py-20">
        <div 
          className="max-w-4xl mx-auto"
          style={{ transform: `scale(${scale})`, transformOrigin: 'top center' }}
        >
          {currentPage && (
            <MushafPageView
              page={currentPage}
              selectedAyahNumber={selectedAyah?.number}
              fontSize={fontSize}
              onAyahClick={handleAyahClick}
            />
          )}
        </div>
      </div>

      {/* Bottom Controls */}
      {showControls && (
        <div className="fixed bottom-0 left-0 right-0 z-50 bg-gradient-to-t from-black/60 to-transparent">
          <div className="container mx-auto px-4 py-3 flex items-center justify-center gap-4">
            <button
              onClick={(e) => {
                e.stopPropagation();
                adjustFontSize(-2);
              }}
              className="text-white hover:text-gray-200 transition-colors p-2"
              title="تصغير الخط"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 12H4" />
              </svg>
            </button>
            
            <button
              onClick={(e) => {
                e.stopPropagation();
                adjustFontSize(2);
              }}
              className="text-white hover:text-gray-200 transition-colors p-2"
              title="تكبير الخط"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
              </svg>
            </button>
            
            <button
              onClick={(e) => {
                e.stopPropagation();
                adjustScale(-0.1);
              }}
              className="text-white hover:text-gray-200 transition-colors p-2"
              title="تصغير الصفحة"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0zM13 10H7" />
              </svg>
            </button>
            
            <button
              onClick={(e) => {
                e.stopPropagation();
                adjustScale(0.1);
              }}
              className="text-white hover:text-gray-200 transition-colors p-2"
              title="تكبير الصفحة"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0zM10 7v3m0 0v3m0-3h3m-3 0H7" />
              </svg>
            </button>
            
            <button
              onClick={(e) => {
                e.stopPropagation();
                handlePageJump();
              }}
              className="text-white hover:text-gray-200 transition-colors p-2"
              title="الانتقال إلى صفحة"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
            </button>
            
            <button
              onClick={(e) => {
                e.stopPropagation();
                navigateToPage(pageNumber - 1);
              }}
              disabled={pageNumber <= 1}
              className="text-white hover:text-gray-200 transition-colors p-2 disabled:opacity-50 disabled:cursor-not-allowed"
              title="الصفحة السابقة"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
              </svg>
            </button>
            
            <button
              onClick={(e) => {
                e.stopPropagation();
                navigateToPage(pageNumber + 1);
              }}
              disabled={pageNumber >= TOTAL_PAGES}
              className="text-white hover:text-gray-200 transition-colors p-2 disabled:opacity-50 disabled:cursor-not-allowed"
              title="الصفحة التالية"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
              </svg>
            </button>
          </div>
        </div>
      )}

      {/* Ayah Options Modal */}
      {selectedAyah && (
        <AyahOptionsModal
          ayah={selectedAyah}
          onClose={() => setSelectedAyah(null)}
        />
      )}
    </div>
  );
}

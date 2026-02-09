/**
 * Zustand Store for Quran State Management
 * Handles Quran data, bookmarks, reading progress, and caching
 * 
 * Requirements: 19.1, 19.2, 19.3, 19.4, 19.5
 */

import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import { devtools } from 'zustand/middleware';
import { QuranService } from '../services/quran-service';
import type { Surah, Juz, QuranBookmark, ReadingProgress, QuranPage, Ayah } from '@/types/quran';

// ============================================================================
// Types
// ============================================================================

interface QuranState {
  // Data
  surahs: Surah[];
  juzs: Juz[];
  bookmarks: QuranBookmark[];
  readingProgress: ReadingProgress | null;
  currentPage: QuranPage | null;
  currentSurah: Surah | null;
  
  // UI State
  loading: boolean;
  error: string | null;
  
  // Cache
  cachedPages: Map<number, QuranPage>;
  cachedSurahAyahs: Map<number, Ayah[]>;
  
  // Actions
  fetchSurahs: () => Promise<void>;
  fetchSurah: (surahNumber: number) => Promise<void>;
  fetchJuzs: () => Promise<void>;
  fetchBookmarks: () => Promise<void>;
  fetchReadingProgress: () => Promise<void>;
  fetchPage: (pageNumber: number) => Promise<void>;
  fetchSurahAyahs: (surahNumber: number) => Promise<void>;
  
  // Optimistic Updates
  addBookmark: (data: {
    surah_number: number;
    ayah_number: number;
    page_number: number;
    note?: string;
  }) => Promise<void>;
  deleteBookmark: (bookmarkId: string) => Promise<void>;
  updateReadingProgress: (data: {
    surah_number: number;
    ayah_number: number;
    page_number: number;
  }) => Promise<void>;
  
  // Utility
  clearError: () => void;
  reset: () => void;
}

// ============================================================================
// Initial State
// ============================================================================

const initialState = {
  surahs: [],
  juzs: [],
  bookmarks: [],
  readingProgress: null,
  currentPage: null,
  currentSurah: null,
  loading: false,
  error: null,
  cachedPages: new Map<number, QuranPage>(),
  cachedSurahAyahs: new Map<number, Ayah[]>(),
};

// ============================================================================
// Store Implementation
// ============================================================================

export const useQuranStore = create<QuranState>()(
  devtools(
    persist(
      (set, get) => ({
        ...initialState,

        // Fetch all surahs
        fetchSurahs: async () => {
          // Return cached data if available
          if (get().surahs.length > 0) {
            return;
          }

          set({ loading: true, error: null });
          try {
            const surahs = await QuranService.getSurahs();
            set({ surahs, loading: false });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Fetch a specific surah
        fetchSurah: async (surahNumber: number) => {
          set({ loading: true, error: null });
          try {
            const surah = await QuranService.getSurah(surahNumber);
            set({ currentSurah: surah, loading: false });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Fetch all juzs
        fetchJuzs: async () => {
          // Return cached data if available
          if (get().juzs.length > 0) {
            return;
          }

          set({ loading: true, error: null });
          try {
            const juzs = await QuranService.getJuzs();
            set({ juzs, loading: false });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Fetch user bookmarks
        fetchBookmarks: async () => {
          set({ loading: true, error: null });
          try {
            const bookmarks = await QuranService.getBookmarks();
            set({ bookmarks, loading: false });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Fetch reading progress
        fetchReadingProgress: async () => {
          set({ loading: true, error: null });
          try {
            const progress = await QuranService.getReadingProgress();
            set({ readingProgress: progress, loading: false });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Fetch a specific page with caching
        fetchPage: async (pageNumber: number) => {
          // Check cache first
          const cached = get().cachedPages.get(pageNumber);
          if (cached) {
            set({ currentPage: cached });
            return;
          }

          set({ loading: true, error: null });
          try {
            const page = await QuranService.getPage(pageNumber);
            
            // Update cache
            const newCache = new Map(get().cachedPages);
            newCache.set(pageNumber, page);
            
            // Limit cache size to 20 pages
            if (newCache.size > 20) {
              const firstKey = newCache.keys().next().value;
              newCache.delete(firstKey);
            }
            
            set({ 
              currentPage: page, 
              cachedPages: newCache,
              loading: false 
            });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Fetch ayahs for a surah with caching
        fetchSurahAyahs: async (surahNumber: number) => {
          // Check cache first
          const cached = get().cachedSurahAyahs.get(surahNumber);
          if (cached) {
            return;
          }

          set({ loading: true, error: null });
          try {
            const ayahs = await QuranService.getSurahAyahs(surahNumber);
            
            // Update cache
            const newCache = new Map(get().cachedSurahAyahs);
            newCache.set(surahNumber, ayahs);
            
            // Limit cache size to 10 surahs
            if (newCache.size > 10) {
              const firstKey = newCache.keys().next().value;
              newCache.delete(firstKey);
            }
            
            set({ 
              cachedSurahAyahs: newCache,
              loading: false 
            });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Add bookmark with optimistic update
        addBookmark: async (data) => {
          // Optimistic update
          const tempBookmark: QuranBookmark = {
            id: `temp-${Date.now()}`,
            ...data,
            created_at: new Date().toISOString(),
          };
          
          set({ 
            bookmarks: [...get().bookmarks, tempBookmark],
            error: null 
          });

          try {
            const bookmark = await QuranService.addBookmark(data);
            
            // Replace temp bookmark with real one
            set({
              bookmarks: get().bookmarks.map(b => 
                b.id === tempBookmark.id ? bookmark : b
              ),
            });
          } catch (error: any) {
            // Rollback on error
            set({
              bookmarks: get().bookmarks.filter(b => b.id !== tempBookmark.id),
              error: error.message,
            });
          }
        },

        // Delete bookmark with optimistic update
        deleteBookmark: async (bookmarkId: string) => {
          // Store for potential rollback
          const previousBookmarks = get().bookmarks;
          
          // Optimistic update
          set({
            bookmarks: get().bookmarks.filter(b => b.id !== bookmarkId),
            error: null,
          });

          try {
            await QuranService.deleteBookmark(bookmarkId);
          } catch (error: any) {
            // Rollback on error
            set({
              bookmarks: previousBookmarks,
              error: error.message,
            });
          }
        },

        // Update reading progress with optimistic update
        updateReadingProgress: async (data) => {
          // Optimistic update
          const newProgress: ReadingProgress = {
            ...data,
            last_read_at: new Date().toISOString(),
          };
          
          set({ 
            readingProgress: newProgress,
            error: null 
          });

          try {
            await QuranService.updateReadingProgress(data);
          } catch (error: any) {
            // Note: We don't rollback reading progress as it's less critical
            set({ error: error.message });
          }
        },

        // Clear error
        clearError: () => set({ error: null }),

        // Reset store
        reset: () => set(initialState),
      }),
      {
        name: 'quran-storage',
        storage: createJSONStorage(() => localStorage),
        // Only persist essential data
        partialize: (state) => ({
          surahs: state.surahs,
          juzs: state.juzs,
          bookmarks: state.bookmarks,
          readingProgress: state.readingProgress,
        }),
      }
    ),
    {
      name: 'QuranStore',
    }
  )
);

// ============================================================================
// Selectors (for optimized re-renders)
// ============================================================================

export const selectSurahs = (state: QuranState) => state.surahs;
export const selectJuzs = (state: QuranState) => state.juzs;
export const selectBookmarks = (state: QuranState) => state.bookmarks;
export const selectReadingProgress = (state: QuranState) => state.readingProgress;
export const selectCurrentPage = (state: QuranState) => state.currentPage;
export const selectCurrentSurah = (state: QuranState) => state.currentSurah;
export const selectLoading = (state: QuranState) => state.loading;
export const selectError = (state: QuranState) => state.error;

// Memoized selectors
export const selectSurahByNumber = (surahNumber: number) => (state: QuranState) =>
  state.surahs.find(s => s.number === surahNumber);

export const selectBookmarksBySurah = (surahNumber: number) => (state: QuranState) =>
  state.bookmarks.filter(b => b.surah_number === surahNumber);

export const selectCachedPage = (pageNumber: number) => (state: QuranState) =>
  state.cachedPages.get(pageNumber);

export const selectCachedSurahAyahs = (surahNumber: number) => (state: QuranState) =>
  state.cachedSurahAyahs.get(surahNumber);

/**
 * Unit tests for Quran Store
 * Tests state management, caching, and optimistic updates
 */

import { renderHook, act, waitFor } from '@testing-library/react';
import { useQuranStore } from '../quran-store';
import { QuranService } from '../../services/quran-service';
import type { Surah, QuranBookmark, ReadingProgress } from '@/types/quran';

// Mock the QuranService
jest.mock('../../services/quran-service');

describe('Quran Store', () => {
  const mockSurahs: Surah[] = [
    {
      number: 1,
      name_arabic: 'الفاتحة',
      name_english: 'Al-Fatihah',
      name_transliteration: 'Al-Fatihah',
      ayah_count: 7,
      revelation_type: 'Meccan',
      revelation_order: 5,
      juz_start: 1,
      juz_end: 1,
      page_start: 1,
      page_end: 1,
    },
    {
      number: 2,
      name_arabic: 'البقرة',
      name_english: 'Al-Baqarah',
      name_transliteration: 'Al-Baqarah',
      ayah_count: 286,
      revelation_type: 'Medinan',
      revelation_order: 87,
      juz_start: 1,
      juz_end: 3,
      page_start: 2,
      page_end: 49,
    },
  ];

  const mockBookmark: QuranBookmark = {
    id: '1',
    surah_number: 2,
    ayah_number: 255,
    page_number: 42,
    note: 'Ayat Al-Kursi',
    created_at: '2024-01-01T00:00:00Z',
  };

  const mockProgress: ReadingProgress = {
    surah_number: 2,
    ayah_number: 100,
    page_number: 15,
    last_read_at: '2024-01-01T00:00:00Z',
  };

  beforeEach(() => {
    // Reset store before each test
    useQuranStore.getState().reset();
    jest.clearAllMocks();
  });

  describe('fetchSurahs', () => {
    it('should fetch and store surahs', async () => {
      (QuranService.getSurahs as jest.Mock).mockResolvedValue(mockSurahs);

      const { result } = renderHook(() => useQuranStore());

      await act(async () => {
        await result.current.fetchSurahs();
      });

      expect(result.current.surahs).toEqual(mockSurahs);
      expect(result.current.loading).toBe(false);
      expect(result.current.error).toBeNull();
    });

    it('should use cached surahs on subsequent calls', async () => {
      (QuranService.getSurahs as jest.Mock).mockResolvedValue(mockSurahs);

      const { result } = renderHook(() => useQuranStore());

      // First call
      await act(async () => {
        await result.current.fetchSurahs();
      });

      expect(QuranService.getSurahs).toHaveBeenCalledTimes(1);

      // Second call should use cache
      await act(async () => {
        await result.current.fetchSurahs();
      });

      expect(QuranService.getSurahs).toHaveBeenCalledTimes(1); // Still 1
      expect(result.current.surahs).toEqual(mockSurahs);
    });

    it('should handle errors', async () => {
      const error = new Error('Failed to fetch surahs');
      (QuranService.getSurahs as jest.Mock).mockRejectedValue(error);

      const { result } = renderHook(() => useQuranStore());

      await act(async () => {
        await result.current.fetchSurahs();
      });

      expect(result.current.error).toBe(error.message);
      expect(result.current.loading).toBe(false);
      expect(result.current.surahs).toEqual([]);
    });
  });

  describe('addBookmark', () => {
    it('should add bookmark with optimistic update', async () => {
      const newBookmarkData = {
        surah_number: 2,
        ayah_number: 255,
        page_number: 42,
        note: 'Ayat Al-Kursi',
      };

      (QuranService.addBookmark as jest.Mock).mockResolvedValue(mockBookmark);

      const { result } = renderHook(() => useQuranStore());

      await act(async () => {
        await result.current.addBookmark(newBookmarkData);
      });

      await waitFor(() => {
        expect(result.current.bookmarks).toHaveLength(1);
        expect(result.current.bookmarks[0]).toEqual(mockBookmark);
      });
    });

    it('should rollback on error', async () => {
      const newBookmarkData = {
        surah_number: 2,
        ayah_number: 255,
        page_number: 42,
      };

      const error = new Error('Failed to add bookmark');
      (QuranService.addBookmark as jest.Mock).mockRejectedValue(error);

      const { result } = renderHook(() => useQuranStore());

      await act(async () => {
        await result.current.addBookmark(newBookmarkData);
      });

      await waitFor(() => {
        expect(result.current.bookmarks).toHaveLength(0);
        expect(result.current.error).toBe(error.message);
      });
    });
  });

  describe('deleteBookmark', () => {
    it('should delete bookmark with optimistic update', async () => {
      (QuranService.deleteBookmark as jest.Mock).mockResolvedValue(undefined);

      const { result } = renderHook(() => useQuranStore());

      // Set initial bookmark
      act(() => {
        useQuranStore.setState({ bookmarks: [mockBookmark] });
      });

      await act(async () => {
        await result.current.deleteBookmark(mockBookmark.id);
      });

      expect(result.current.bookmarks).toHaveLength(0);
    });

    it('should rollback on error', async () => {
      const error = new Error('Failed to delete bookmark');
      (QuranService.deleteBookmark as jest.Mock).mockRejectedValue(error);

      const { result } = renderHook(() => useQuranStore());

      // Set initial bookmark
      act(() => {
        useQuranStore.setState({ bookmarks: [mockBookmark] });
      });

      await act(async () => {
        await result.current.deleteBookmark(mockBookmark.id);
      });

      await waitFor(() => {
        expect(result.current.bookmarks).toHaveLength(1);
        expect(result.current.error).toBe(error.message);
      });
    });
  });

  describe('updateReadingProgress', () => {
    it('should update reading progress with optimistic update', async () => {
      const progressData = {
        surah_number: 2,
        ayah_number: 100,
        page_number: 15,
      };

      (QuranService.updateReadingProgress as jest.Mock).mockResolvedValue(undefined);

      const { result } = renderHook(() => useQuranStore());

      await act(async () => {
        await result.current.updateReadingProgress(progressData);
      });

      expect(result.current.readingProgress).toMatchObject(progressData);
    });

    it('should not rollback on error (less critical)', async () => {
      const progressData = {
        surah_number: 2,
        ayah_number: 100,
        page_number: 15,
      };

      const error = new Error('Failed to update progress');
      (QuranService.updateReadingProgress as jest.Mock).mockRejectedValue(error);

      const { result } = renderHook(() => useQuranStore());

      await act(async () => {
        await result.current.updateReadingProgress(progressData);
      });

      // Progress should still be updated locally
      expect(result.current.readingProgress).toMatchObject(progressData);
      expect(result.current.error).toBe(error.message);
    });
  });

  describe('caching', () => {
    it('should cache pages', async () => {
      const mockPage = {
        page_number: 1,
        ayahs: [],
        juz_number: 1,
        surah_number: 1,
        surah_name: 'Al-Fatihah',
      };

      (QuranService.getPage as jest.Mock).mockResolvedValue(mockPage);

      const { result } = renderHook(() => useQuranStore());

      // First fetch
      await act(async () => {
        await result.current.fetchPage(1);
      });

      expect(QuranService.getPage).toHaveBeenCalledTimes(1);
      expect(result.current.currentPage).toEqual(mockPage);

      // Second fetch should use cache
      await act(async () => {
        await result.current.fetchPage(1);
      });

      expect(QuranService.getPage).toHaveBeenCalledTimes(1); // Still 1
    });

    it('should limit cache size to 20 pages', async () => {
      const { result } = renderHook(() => useQuranStore());

      // Mock pages
      for (let i = 1; i <= 25; i++) {
        const mockPage = {
          page_number: i,
          ayahs: [],
          juz_number: 1,
          surah_number: 1,
          surah_name: 'Test',
        };
        (QuranService.getPage as jest.Mock).mockResolvedValue(mockPage);

        await act(async () => {
          await result.current.fetchPage(i);
        });
      }

      // Cache should only have 20 pages
      expect(result.current.cachedPages.size).toBe(20);
    });
  });

  describe('selectors', () => {
    it('should select surah by number', () => {
      const { result } = renderHook(() => useQuranStore());

      act(() => {
        useQuranStore.setState({ surahs: mockSurahs });
      });

      const surah = result.current.surahs.find(s => s.number === 2);
      expect(surah).toEqual(mockSurahs[1]);
    });

    it('should select bookmarks by surah', () => {
      const bookmarks = [
        { ...mockBookmark, id: '1', surah_number: 2 },
        { ...mockBookmark, id: '2', surah_number: 3 },
        { ...mockBookmark, id: '3', surah_number: 2 },
      ];

      const { result } = renderHook(() => useQuranStore());

      act(() => {
        useQuranStore.setState({ bookmarks });
      });

      const surah2Bookmarks = result.current.bookmarks.filter(
        b => b.surah_number === 2
      );
      expect(surah2Bookmarks).toHaveLength(2);
    });
  });

  describe('reset', () => {
    it('should reset store to initial state', () => {
      const { result } = renderHook(() => useQuranStore());

      // Set some state
      act(() => {
        useQuranStore.setState({
          surahs: mockSurahs,
          bookmarks: [mockBookmark],
          readingProgress: mockProgress,
        });
      });

      // Reset
      act(() => {
        result.current.reset();
      });

      expect(result.current.surahs).toEqual([]);
      expect(result.current.bookmarks).toEqual([]);
      expect(result.current.readingProgress).toBeNull();
    });
  });
});

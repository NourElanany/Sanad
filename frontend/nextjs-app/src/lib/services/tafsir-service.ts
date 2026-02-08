/**
 * Tafsir service for API interactions
 * Handles tafsir retrieval, comparison, search, and offline caching
 */
import { apiClient } from '../api/axios-client';
import type {
  TafsirSource,
  Tafsir,
  TafsirWithSource,
  TafsirComparisonRequest,
  TafsirComparisonResponse,
  TafsirSearchRequest,
  TafsirSearchResponse,
} from '@/types/tafsir';

export class TafsirService {
  /**
   * Get all available tafsir sources
   */
  static async getTafsirSources(): Promise<TafsirSource[]> {
    try {
      return await apiClient.get<TafsirSource[]>('/api/quran/tafsir/sources');
    } catch (error) {
      console.error('Failed to fetch tafsir sources:', error);
      throw new Error('Failed to load tafsir sources');
    }
  }

  /**
   * Get tafsir for a specific ayah
   */
  static async getTafsirForAyah(
    surahNumber: number,
    ayahNumber: number,
    sourceIds?: string[]
  ): Promise<TafsirWithSource[]> {
    try {
      const params: any = {
        surah_number: surahNumber,
        ayah_number: ayahNumber,
      };
      
      if (sourceIds && sourceIds.length > 0) {
        params.source_ids = sourceIds.join(',');
      }

      return await apiClient.get<TafsirWithSource[]>('/api/quran/tafsir', {
        params,
      });
    } catch (error) {
      console.error(`Failed to fetch tafsir for ${surahNumber}:${ayahNumber}:`, error);
      throw new Error('Failed to load tafsir');
    }
  }

  /**
   * Compare multiple tafsir interpretations side-by-side
   */
  static async compareTafsir(
    request: TafsirComparisonRequest
  ): Promise<TafsirComparisonResponse> {
    try {
      return await apiClient.post<TafsirComparisonResponse>(
        '/api/quran/tafsir/compare',
        request
      );
    } catch (error) {
      console.error('Failed to compare tafsir:', error);
      throw new Error('Failed to compare tafsir interpretations');
    }
  }

  /**
   * Search within tafsir content
   */
  static async searchTafsir(
    request: TafsirSearchRequest
  ): Promise<TafsirSearchResponse> {
    try {
      return await apiClient.post<TafsirSearchResponse>(
        '/api/quran/tafsir/search',
        request
      );
    } catch (error) {
      console.error('Failed to search tafsir:', error);
      throw new Error('Failed to search tafsir');
    }
  }

  /**
   * Get tafsir for a range of ayahs
   */
  static async getTafsirForRange(
    surahNumber: number,
    startAyah: number,
    endAyah: number,
    sourceIds?: string[]
  ): Promise<Map<number, TafsirWithSource[]>> {
    try {
      const tafsirMap = new Map<number, TafsirWithSource[]>();
      
      // Fetch tafsir for each ayah in the range
      const promises = [];
      for (let ayahNum = startAyah; ayahNum <= endAyah; ayahNum++) {
        promises.push(
          this.getTafsirForAyah(surahNumber, ayahNum, sourceIds).then(
            (tafsirs) => ({ ayahNum, tafsirs })
          )
        );
      }

      const results = await Promise.all(promises);
      results.forEach(({ ayahNum, tafsirs }) => {
        tafsirMap.set(ayahNum, tafsirs);
      });

      return tafsirMap;
    } catch (error) {
      console.error('Failed to fetch tafsir range:', error);
      throw new Error('Failed to load tafsir range');
    }
  }

  /**
   * Download tafsir for offline use
   */
  static async downloadTafsirForOffline(
    surahNumber: number,
    sourceIds: string[]
  ): Promise<void> {
    try {
      await apiClient.post('/api/quran/tafsir/download', {
        surah_number: surahNumber,
        source_ids: sourceIds,
      });
    } catch (error) {
      console.error('Failed to download tafsir for offline:', error);
      throw new Error('Failed to download tafsir');
    }
  }

  /**
   * Get cached tafsir from local storage
   */
  static getCachedTafsir(
    surahNumber: number,
    ayahNumber: number
  ): TafsirWithSource[] | null {
    try {
      const cacheKey = `tafsir_${surahNumber}_${ayahNumber}`;
      const cached = localStorage.getItem(cacheKey);
      
      if (cached) {
        const data = JSON.parse(cached);
        // Check if cache is still valid (24 hours)
        const cacheTime = new Date(data.timestamp);
        const now = new Date();
        const hoursDiff = (now.getTime() - cacheTime.getTime()) / (1000 * 60 * 60);
        
        if (hoursDiff < 24) {
          return data.tafsirs;
        }
      }
      
      return null;
    } catch (error) {
      console.error('Failed to get cached tafsir:', error);
      return null;
    }
  }

  /**
   * Cache tafsir to local storage
   */
  static cacheTafsir(
    surahNumber: number,
    ayahNumber: number,
    tafsirs: TafsirWithSource[]
  ): void {
    try {
      const cacheKey = `tafsir_${surahNumber}_${ayahNumber}`;
      const data = {
        tafsirs,
        timestamp: new Date().toISOString(),
      };
      localStorage.setItem(cacheKey, JSON.stringify(data));
    } catch (error) {
      console.error('Failed to cache tafsir:', error);
    }
  }

  /**
   * Clear tafsir cache
   */
  static clearTafsirCache(): void {
    try {
      const keys = Object.keys(localStorage);
      keys.forEach((key) => {
        if (key.startsWith('tafsir_')) {
          localStorage.removeItem(key);
        }
      });
    } catch (error) {
      console.error('Failed to clear tafsir cache:', error);
    }
  }
}

/**
 * Quran service for API interactions
 */
import { apiClient } from '../api/axios-client';
import { API_ENDPOINTS } from '../api/endpoints';
import type { Surah, Juz, QuranBookmark, ReadingProgress, QuranPage, Ayah } from '@/types/quran';

export class QuranService {
  /**
   * Get all surahs
   */
  static async getSurahs(): Promise<Surah[]> {
    try {
      return await apiClient.get<Surah[]>(API_ENDPOINTS.QURAN.SURAHS);
    } catch (error) {
      console.error('Failed to fetch surahs:', error);
      throw new Error('Failed to load surahs');
    }
  }

  /**
   * Get a specific surah by number
   */
  static async getSurah(surahNumber: number): Promise<Surah> {
    try {
      return await apiClient.get<Surah>(
        API_ENDPOINTS.QURAN.SURAH(surahNumber)
      );
    } catch (error) {
      console.error(`Failed to fetch surah ${surahNumber}:`, error);
      throw new Error(`Failed to load surah ${surahNumber}`);
    }
  }

  /**
   * Get all juzs
   */
  static async getJuzs(): Promise<Juz[]> {
    try {
      return await apiClient.get<Juz[]>(API_ENDPOINTS.QURAN.JUZS);
    } catch (error) {
      console.error('Failed to fetch juzs:', error);
      throw new Error('Failed to load juzs');
    }
  }

  /**
   * Get a specific juz by number
   */
  static async getJuz(juzNumber: number): Promise<Juz> {
    try {
      return await apiClient.get<Juz>(
        API_ENDPOINTS.QURAN.JUZ(juzNumber)
      );
    } catch (error) {
      console.error(`Failed to fetch juz ${juzNumber}:`, error);
      throw new Error(`Failed to load juz ${juzNumber}`);
    }
  }

  /**
   * Search surahs by name or number
   */
  static async searchSurahs(query: string): Promise<Surah[]> {
    try {
      return await apiClient.get<Surah[]>(API_ENDPOINTS.QURAN.SURAHS, {
        params: { search: query },
      });
    } catch (error) {
      console.error('Failed to search surahs:', error);
      throw new Error('Failed to search surahs');
    }
  }

  /**
   * Get user bookmarks
   */
  static async getBookmarks(): Promise<QuranBookmark[]> {
    try {
      return await apiClient.get<QuranBookmark[]>(
        API_ENDPOINTS.USER_BOOKMARKS
      );
    } catch (error) {
      console.error('Failed to fetch bookmarks:', error);
      throw new Error('Failed to load bookmarks');
    }
  }

  /**
   * Add a bookmark
   */
  static async addBookmark(data: {
    surah_number: number;
    ayah_number: number;
    page_number: number;
    note?: string;
  }): Promise<QuranBookmark> {
    try {
      return await apiClient.post<QuranBookmark>(
        API_ENDPOINTS.USER_BOOKMARKS,
        data
      );
    } catch (error) {
      console.error('Failed to add bookmark:', error);
      throw new Error('Failed to add bookmark');
    }
  }

  /**
   * Delete a bookmark
   */
  static async deleteBookmark(bookmarkId: string): Promise<void> {
    try {
      await apiClient.delete(`${API_ENDPOINTS.USER_BOOKMARKS}/${bookmarkId}`);
    } catch (error) {
      console.error('Failed to delete bookmark:', error);
      throw new Error('Failed to delete bookmark');
    }
  }

  /**
   * Get reading progress
   */
  static async getReadingProgress(): Promise<ReadingProgress> {
    try {
      return await apiClient.get<ReadingProgress>(
        API_ENDPOINTS.USER_READING_PROGRESS
      );
    } catch (error) {
      console.error('Failed to fetch reading progress:', error);
      throw new Error('Failed to load reading progress');
    }
  }

  /**
   * Update reading progress
   */
  static async updateReadingProgress(data: {
    surah_number: number;
    ayah_number: number;
    page_number: number;
  }): Promise<void> {
    try {
      await apiClient.post(API_ENDPOINTS.USER_READING_PROGRESS, data);
    } catch (error) {
      console.error('Failed to update reading progress:', error);
      throw new Error('Failed to update reading progress');
    }
  }

  /**
   * Get a specific page of the Quran
   */
  static async getPage(pageNumber: number): Promise<QuranPage> {
    try {
      return await apiClient.get<QuranPage>(`/api/quran/pages/${pageNumber}`);
    } catch (error) {
      console.error(`Failed to fetch page ${pageNumber}:`, error);
      throw new Error(`Failed to load page ${pageNumber}`);
    }
  }

  /**
   * Get ayahs for a specific surah
   */
  static async getSurahAyahs(surahNumber: number): Promise<Ayah[]> {
    try {
      return await apiClient.get<Ayah[]>(`/api/quran/surahs/${surahNumber}/ayahs`);
    } catch (error) {
      console.error(`Failed to fetch ayahs for surah ${surahNumber}:`, error);
      throw new Error(`Failed to load ayahs for surah ${surahNumber}`);
    }
  }

  /**
   * Get ayahs for a specific page
   */
  static async getPageAyahs(pageNumber: number): Promise<Ayah[]> {
    try {
      return await apiClient.get<Ayah[]>(`/api/quran/pages/${pageNumber}/ayahs`);
    } catch (error) {
      console.error(`Failed to fetch ayahs for page ${pageNumber}:`, error);
      throw new Error(`Failed to load ayahs for page ${pageNumber}`);
    }
  }
}


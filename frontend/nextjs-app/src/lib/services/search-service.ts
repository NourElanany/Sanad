/**
 * Search service for comprehensive Islamic content search
 * Integrates with backend semantic search service
 */

import { apiClient } from '../api/axios-client';
import type {
  SearchRequest,
  SearchResponse,
  QuerySuggestion,
  SavedSearch,
  AuthenticityGrade,
} from '@/types/search';

export class SearchService {
  /**
   * Perform semantic search across all Islamic content
   */
  static async search(request: SearchRequest): Promise<SearchResponse> {
    try {
      return await apiClient.post<SearchResponse>('/api/search/search', request);
    } catch (error) {
      console.error('Failed to perform search:', error);
      throw new Error('فشل البحث. يرجى المحاولة مرة أخرى');
    }
  }

  /**
   * Search specifically in Quran
   */
  static async searchQuran(
    query: string,
    limit: number = 20,
    minSimilarity: number = 0.5
  ): Promise<SearchResponse> {
    try {
      const request: SearchRequest = {
        query,
        limit,
        min_similarity: minSimilarity,
        content_types: ['quran'],
      };

      return await apiClient.post<SearchResponse>('/api/search/quran', request);
    } catch (error) {
      console.error('Failed to search Quran:', error);
      throw new Error('فشل البحث في القرآن الكريم');
    }
  }

  /**
   * Search specifically in Hadith
   */
  static async searchHadith(
    query: string,
    limit: number = 20,
    minSimilarity: number = 0.5,
    authenticityGrades?: AuthenticityGrade[]
  ): Promise<SearchResponse> {
    try {
      const request: SearchRequest = {
        query,
        limit,
        min_similarity: minSimilarity,
        content_types: [
          'sahih_hadith',
          'hasan_hadith',
          'daif_hadith',
          'mawdu_hadith',
        ],
        filters: authenticityGrades
          ? { authenticity_grades: authenticityGrades }
          : undefined,
      };

      return await apiClient.post<SearchResponse>('/api/search/hadith', request);
    } catch (error) {
      console.error('Failed to search Hadith:', error);
      throw new Error('فشل البحث في الأحاديث');
    }
  }

  /**
   * Search in Fatawa (Islamic rulings)
   */
  static async searchFatawa(
    query: string,
    limit: number = 20,
    minSimilarity: number = 0.5
  ): Promise<SearchResponse> {
    try {
      const request: SearchRequest = {
        query,
        limit,
        min_similarity: minSimilarity,
        content_types: ['fiqh_ruling', 'scholar_opinion'],
      };

      return await apiClient.post<SearchResponse>('/api/search/fatawa', request);
    } catch (error) {
      console.error('Failed to search Fatawa:', error);
      throw new Error('فشل البحث في الفتاوى');
    }
  }

  /**
   * Advanced search with full filter support
   */
  static async advancedSearch(request: SearchRequest): Promise<SearchResponse> {
    try {
      return await apiClient.post<SearchResponse>('/api/search/advanced', request);
    } catch (error) {
      console.error('Failed to perform advanced search:', error);
      throw new Error('فشل البحث المتقدم');
    }
  }

  /**
   * Get search suggestions based on query
   */
  static async getSuggestions(query: string): Promise<QuerySuggestion[]> {
    try {
      return await apiClient.get<QuerySuggestion[]>('/api/search/suggestions', {
        params: { query },
      });
    } catch (error) {
      console.error('Failed to get suggestions:', error);
      return [];
    }
  }

  /**
   * Voice search - converts speech to text and performs search
   */
  static async voiceSearch(
    audioBlob: Blob,
    contentTypes?: string[],
    limit: number = 20
  ): Promise<SearchResponse> {
    try {
      // Convert blob to base64
      const base64Audio = await this.blobToBase64(audioBlob);

      // First, convert speech to text
      const transcriptionResponse = await apiClient.post<{ text: string }>(
        '/api/speech/transcribe',
        {
          audio: base64Audio,
          language: 'ar',
        }
      );

      const query = transcriptionResponse.text;

      // Then perform search with transcribed text
      const request: SearchRequest = {
        query,
        limit,
        content_types: contentTypes,
      };

      return await this.search(request);
    } catch (error) {
      console.error('Failed to perform voice search:', error);
      throw new Error('فشل البحث الصوتي');
    }
  }

  /**
   * Save a search for later access
   */
  static async saveSearch(
    query: string,
    filters?: SearchRequest['filters'],
    name?: string
  ): Promise<SavedSearch> {
    try {
      return await apiClient.post<SavedSearch>('/api/search/saved', {
        query,
        filters,
        name,
      });
    } catch (error) {
      console.error('Failed to save search:', error);
      throw new Error('فشل حفظ البحث');
    }
  }

  /**
   * Get all saved searches
   */
  static async getSavedSearches(): Promise<SavedSearch[]> {
    try {
      return await apiClient.get<SavedSearch[]>('/api/search/saved');
    } catch (error) {
      console.error('Failed to get saved searches:', error);
      throw new Error('فشل تحميل البحثات المحفوظة');
    }
  }

  /**
   * Delete a saved search
   */
  static async deleteSavedSearch(searchId: string): Promise<void> {
    try {
      await apiClient.delete(`/api/search/saved/${searchId}`);
    } catch (error) {
      console.error('Failed to delete saved search:', error);
      throw new Error('فشل حذف البحث');
    }
  }

  /**
   * Helper: Convert Blob to base64
   */
  private static blobToBase64(blob: Blob): Promise<string> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onloadend = () => {
        const base64 = reader.result as string;
        // Remove data URL prefix
        const base64Data = base64.split(',')[1];
        resolve(base64Data);
      };
      reader.onerror = reject;
      reader.readAsDataURL(blob);
    });
  }
}

import { axiosClient } from '../api/axios-client';
import type {
  Hadith,
  HadithBook,
  HadithChapter,
  HadithWithDetails,
  HadithSearchResponse,
  HadithTopicResponse,
  HadithSearchFilters,
  HadithGrade,
} from '@/types/hadith';

const HADITH_BASE_URL = '/api/v1';

export class HadithService {
  /**
   * Get all Hadith books
   */
  static async getHadithBooks(): Promise<HadithBook[]> {
    try {
      const response = await axiosClient.get(`${HADITH_BASE_URL}/books`);
      return response.data.data;
    } catch (error) {
      console.error('Failed to fetch hadith books:', error);
      throw new Error('Failed to load hadith books');
    }
  }

  /**
   * Get Hadiths by book name
   */
  static async getHadithsByBook(
    bookName: string,
    limit?: number,
    offset?: number
  ): Promise<Hadith[]> {
    try {
      const params: Record<string, any> = { book: bookName };
      if (limit) params.limit = limit;
      if (offset) params.offset = offset;

      const response = await axiosClient.get(`${HADITH_BASE_URL}/hadiths`, { params });
      return response.data.data;
    } catch (error) {
      console.error('Failed to fetch hadiths by book:', error);
      throw new Error('Failed to load hadiths');
    }
  }

  /**
   * Get a specific Hadith by ID
   */
  static async getHadithById(
    hadithId: string,
    includeSanad: boolean = false,
    includeExplanations: boolean = false
  ): Promise<HadithWithDetails> {
    try {
      const params = {
        include_sanad: includeSanad,
        include_explanations: includeExplanations,
      };

      const response = await axiosClient.get(
        `${HADITH_BASE_URL}/hadiths/${hadithId}`,
        { params }
      );
      return response.data.data;
    } catch (error) {
      console.error('Failed to fetch hadith:', error);
      throw new Error('Failed to load hadith details');
    }
  }

  /**
   * Get a Hadith by number and book
   */
  static async getHadithByNumber(
    hadithNumber: string,
    bookName: string,
    includeSanad: boolean = false,
    includeExplanations: boolean = false
  ): Promise<HadithWithDetails> {
    try {
      const params = {
        include_sanad: includeSanad,
        include_explanations: includeExplanations,
      };

      const response = await axiosClient.get(
        `${HADITH_BASE_URL}/hadiths/number/${hadithNumber}/book/${bookName}`,
        { params }
      );
      return response.data.data;
    } catch (error) {
      console.error('Failed to fetch hadith by number:', error);
      throw new Error('Failed to load hadith');
    }
  }

  /**
   * Search Hadiths
   */
  static async searchHadiths(
    query: string,
    filters?: HadithSearchFilters,
    limit: number = 20,
    offset: number = 0
  ): Promise<HadithSearchResponse> {
    try {
      const params: Record<string, any> = {
        q: query,
        type: filters?.searchType || 'text',
        limit,
        offset,
      };

      if (filters?.books && filters.books.length > 0) {
        params.books = filters.books.join(',');
      }

      if (filters?.grades && filters.grades.length > 0) {
        params.grades = filters.grades.join(',');
      }

      if (filters?.themes && filters.themes.length > 0) {
        params.themes = filters.themes.join(',');
      }

      const response = await axiosClient.get(`${HADITH_BASE_URL}/search`, { params });
      return response.data.data;
    } catch (error) {
      console.error('Failed to search hadiths:', error);
      throw new Error('Failed to search hadiths');
    }
  }

  /**
   * Get search suggestions
   */
  static async getSearchSuggestions(query: string): Promise<string[]> {
    try {
      const response = await axiosClient.get(`${HADITH_BASE_URL}/search/suggestions`, {
        params: { q: query },
      });
      return response.data.data;
    } catch (error) {
      console.error('Failed to get search suggestions:', error);
      return [];
    }
  }

  /**
   * Get Hadiths by topic/theme
   */
  static async getHadithsByTopic(
    topic: string,
    includeRelated: boolean = false,
    grades?: HadithGrade[],
    limit: number = 20,
    offset: number = 0
  ): Promise<HadithTopicResponse> {
    try {
      const params: Record<string, any> = {
        include_related: includeRelated,
        limit,
        offset,
      };

      if (grades && grades.length > 0) {
        params.grades = grades.join(',');
      }

      const response = await axiosClient.get(
        `${HADITH_BASE_URL}/topics/${topic}`,
        { params }
      );
      return response.data.data;
    } catch (error) {
      console.error('Failed to fetch hadiths by topic:', error);
      throw new Error('Failed to load hadiths by topic');
    }
  }

  /**
   * Get chapters for a book
   */
  static async getBookChapters(bookId: string): Promise<HadithChapter[]> {
    try {
      const response = await axiosClient.get(`${HADITH_BASE_URL}/books/${bookId}/chapters`);
      return response.data.data;
    } catch (error) {
      console.error('Failed to fetch book chapters:', error);
      throw new Error('Failed to load book chapters');
    }
  }

  /**
   * Get Hadiths by narrator
   */
  static async getHadithsByNarrator(
    narratorName: string,
    limit: number = 20,
    offset: number = 0
  ): Promise<HadithSearchResponse> {
    try {
      return await this.searchHadiths(
        narratorName,
        { searchType: 'narrator' },
        limit,
        offset
      );
    } catch (error) {
      console.error('Failed to fetch hadiths by narrator:', error);
      throw new Error('Failed to load hadiths by narrator');
    }
  }
}

import { axiosClient } from '../api/axios-client';
import type {
  Story,
  StoryWithDetails,
  StorySearchResponse,
  Character,
  LessonInStory,
  StorySource,
  StoryCategory,
  PaginationOptions,
  StoryFilters,
  CharacterFilterOptions,
  ThemeFilterOptions,
  CharacterFilters,
} from '@/types/stories';

/**
 * Service for managing Islamic stories
 */
class StoriesService {
  private readonly baseUrl = '/api/stories';

  /**
   * Get stories by category
   */
  async getStoriesByCategory(
    category: StoryCategory,
    options?: PaginationOptions
  ): Promise<Story[]> {
    try {
      const response = await axiosClient.get(
        `${this.baseUrl}/category/${category}`,
        {
          params: {
            limit: options?.limit,
            offset: options?.offset,
          },
        }
      );
      return response.data.data;
    } catch (error) {
      console.error('Error fetching stories by category:', error);
      throw error;
    }
  }

  /**
   * Get a single story by ID
   */
  async getStory(
    storyId: string,
    includeDetails: boolean = true
  ): Promise<StoryWithDetails> {
    try {
      const response = await axiosClient.get(`${this.baseUrl}/stories/${storyId}`, {
        params: {
          include_details: includeDetails,
        },
      });
      return response.data.data;
    } catch (error) {
      console.error('Error fetching story:', error);
      throw error;
    }
  }

  /**
   * Search stories
   */
  async searchStories(
    query: string,
    filters?: StoryFilters,
    options?: PaginationOptions
  ): Promise<StorySearchResponse> {
    try {
      const response = await axiosClient.get(`${this.baseUrl}/stories`, {
        params: {
          query,
          categories: filters?.categories?.join(','),
          age_groups: filters?.ageGroups?.join(','),
          authenticity_levels: filters?.authenticityLevels?.join(','),
          time_periods: filters?.timePeriods?.join(','),
          themes: filters?.themes?.join(','),
          limit: options?.limit,
          offset: options?.offset,
        },
      });
      return response.data.data;
    } catch (error) {
      console.error('Error searching stories:', error);
      throw error;
    }
  }

  /**
   * Get stories by character
   */
  async getStoriesByCharacter(
    characterName: string,
    options?: CharacterFilterOptions
  ): Promise<Story[]> {
    try {
      const response = await axiosClient.get(
        `${this.baseUrl}/character/${encodeURIComponent(characterName)}`,
        {
          params: {
            character_type: options?.characterType,
            include_related: options?.includeRelated,
            limit: options?.limit,
            offset: options?.offset,
          },
        }
      );
      return response.data.data.stories.map((item: any) => item.story);
    } catch (error) {
      console.error('Error fetching stories by character:', error);
      throw error;
    }
  }

  /**
   * Get stories by theme
   */
  async getStoriesByTheme(
    theme: string,
    options?: ThemeFilterOptions
  ): Promise<Story[]> {
    try {
      const response = await axiosClient.get(
        `${this.baseUrl}/theme/${encodeURIComponent(theme)}`,
        {
          params: {
            lesson_type: options?.lessonType,
            moral_category: options?.moralCategory,
            age_group: options?.ageGroup,
            limit: options?.limit,
            offset: options?.offset,
          },
        }
      );
      return response.data.data.stories.map((item: any) => item.story);
    } catch (error) {
      console.error('Error fetching stories by theme:', error);
      throw error;
    }
  }

  /**
   * Get story lessons
   */
  async getStoryLessons(storyId: string): Promise<LessonInStory[]> {
    try {
      const response = await axiosClient.get(
        `${this.baseUrl}/stories/${storyId}/lessons`
      );
      return response.data.data;
    } catch (error) {
      console.error('Error fetching story lessons:', error);
      throw error;
    }
  }

  /**
   * Get story sources
   */
  async getStorySources(storyId: string): Promise<StorySource[]> {
    try {
      const response = await axiosClient.get(
        `${this.baseUrl}/stories/${storyId}/sources`
      );
      return response.data.data;
    } catch (error) {
      console.error('Error fetching story sources:', error);
      throw error;
    }
  }

  /**
   * Search characters
   */
  async searchCharacters(
    query: string,
    filters?: CharacterFilters
  ): Promise<Character[]> {
    try {
      const response = await axiosClient.get(`${this.baseUrl}/characters/search`, {
        params: {
          query,
          character_type: filters?.characterType,
          historical_period: filters?.historicalPeriod,
          limit: filters?.limit,
          offset: filters?.offset,
        },
      });
      return response.data.data;
    } catch (error) {
      console.error('Error searching characters:', error);
      throw error;
    }
  }

  /**
   * Get category statistics
   */
  async getCategoryStatistics(): Promise<Record<string, number>> {
    try {
      const response = await axiosClient.get(
        `${this.baseUrl}/analytics/categories`
      );
      return response.data.data;
    } catch (error) {
      console.error('Error fetching category statistics:', error);
      throw error;
    }
  }

  /**
   * Verify story integrity
   */
  async verifyStoryIntegrity(storyId: string): Promise<boolean> {
    try {
      const response = await axiosClient.get(
        `${this.baseUrl}/stories/${storyId}/integrity`
      );
      return response.data.data.is_valid;
    } catch (error) {
      console.error('Error verifying story integrity:', error);
      throw error;
    }
  }
}

export const storiesService = new StoriesService();

import { axiosClient } from '../api/axios-client';
import type {
  StatisticsDashboard,
  KhatmaStatistics,
  ReadingStatistics,
  RecitationStatistics,
  WeeklyComparison,
  MonthlyComparison,
  PersonalGoal,
  CreateGoalRequest,
  DailyReadingData,
  RecitationScoreData,
} from '@/types/statistics';

const STATISTICS_BASE = '/api/statistics';

export const statisticsService = {
  /**
   * Get comprehensive statistics dashboard
   */
  async getStatisticsDashboard(timePeriodDays?: number): Promise<StatisticsDashboard> {
    const params = timePeriodDays ? { time_period_days: timePeriodDays } : {};
    const response = await axiosClient.get<StatisticsDashboard>(
      `${STATISTICS_BASE}/dashboard`,
      { params }
    );
    return response.data;
  },

  /**
   * Get Khatma statistics
   */
  async getKhatmaStatistics(): Promise<KhatmaStatistics> {
    const response = await axiosClient.get<KhatmaStatistics>(
      `${STATISTICS_BASE}/khatma`
    );
    return response.data;
  },

  /**
   * Get reading statistics
   */
  async getReadingStatistics(days?: number): Promise<ReadingStatistics> {
    const params = days ? { days } : {};
    const response = await axiosClient.get<ReadingStatistics>(
      `${STATISTICS_BASE}/reading`,
      { params }
    );
    return response.data;
  },

  /**
   * Get recitation improvement statistics
   */
  async getRecitationStatistics(): Promise<RecitationStatistics> {
    const response = await axiosClient.get<RecitationStatistics>(
      `${STATISTICS_BASE}/recitation`
    );
    return response.data;
  },

  /**
   * Get weekly comparison data
   */
  async getWeeklyComparison(): Promise<WeeklyComparison> {
    const response = await axiosClient.get<WeeklyComparison>(
      `${STATISTICS_BASE}/weekly`
    );
    return response.data;
  },

  /**
   * Get monthly comparison data
   */
  async getMonthlyComparison(): Promise<MonthlyComparison> {
    const response = await axiosClient.get<MonthlyComparison>(
      `${STATISTICS_BASE}/monthly`
    );
    return response.data;
  },

  /**
   * Get personal goals
   */
  async getPersonalGoals(): Promise<PersonalGoal[]> {
    const response = await axiosClient.get<PersonalGoal[]>(
      `${STATISTICS_BASE}/goals`
    );
    return response.data;
  },

  /**
   * Create a new personal goal
   */
  async createGoal(request: CreateGoalRequest): Promise<PersonalGoal> {
    const response = await axiosClient.post<PersonalGoal>(
      `${STATISTICS_BASE}/goals`,
      request
    );
    return response.data;
  },

  /**
   * Update goal progress
   */
  async updateGoalProgress(goalId: string, currentValue: number): Promise<PersonalGoal> {
    const response = await axiosClient.put<PersonalGoal>(
      `${STATISTICS_BASE}/goals/${goalId}`,
      { current_value: currentValue }
    );
    return response.data;
  },

  /**
   * Delete a personal goal
   */
  async deleteGoal(goalId: string): Promise<void> {
    await axiosClient.delete(`${STATISTICS_BASE}/goals/${goalId}`);
  },

  /**
   * Get daily reading minutes for a specific date range
   */
  async getDailyReadingData(
    startDate: Date,
    endDate: Date
  ): Promise<DailyReadingData[]> {
    const response = await axiosClient.get<DailyReadingData[]>(
      `${STATISTICS_BASE}/daily-reading`,
      {
        params: {
          start_date: startDate.toISOString(),
          end_date: endDate.toISOString(),
        },
      }
    );
    return response.data;
  },

  /**
   * Get recitation score history
   */
  async getRecitationScoreHistory(limit?: number): Promise<RecitationScoreData[]> {
    const params = limit ? { limit } : {};
    const response = await axiosClient.get<RecitationScoreData[]>(
      `${STATISTICS_BASE}/recitation-history`,
      { params }
    );
    return response.data;
  },
};

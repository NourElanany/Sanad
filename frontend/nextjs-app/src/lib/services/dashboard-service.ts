import { axiosClient } from '../api/axios-client';
import { API_ENDPOINTS } from '../api/endpoints';

export interface DailyWird {
  totalPages: number;
  completedPages: number;
  progressPercentage: number;
  completedPageNumbers: number[];
}

export interface DailyContent {
  id: string;
  type: 'verse' | 'hadith';
  arabicText: string;
  translation: string;
  reference: string;
  tafsir?: string;
}

export interface DashboardData {
  dailyWird: DailyWird;
  dailyContent: DailyContent;
  lastUpdated: string;
}

export interface UserStatistics {
  totalReadingMinutes: number;
  completedKhatmas: number;
  currentStreak: number;
  totalAyahsRead: number;
  recitationScore: number;
}

export class DashboardService {
  /**
   * Get complete dashboard data
   */
  static async getDashboardData(): Promise<DashboardData> {
    const response = await axiosClient.get<DashboardData>(
      API_ENDPOINTS.DASHBOARD
    );
    return response.data;
  }

  /**
   * Get daily wird progress
   */
  static async getDailyWird(): Promise<DailyWird> {
    const response = await axiosClient.get<DailyWird>(
      API_ENDPOINTS.DAILY_WIRD
    );
    return response.data;
  }

  /**
   * Update daily wird progress
   */
  static async updateDailyWird(
    pageNumber: number,
    completed: boolean
  ): Promise<DailyWird> {
    const response = await axiosClient.post<DailyWird>(
      API_ENDPOINTS.UPDATE_DAILY_WIRD,
      {
        page_number: pageNumber,
        completed,
      }
    );
    return response.data;
  }

  /**
   * Get daily content (verse or hadith)
   */
  static async getDailyContent(): Promise<DailyContent> {
    const response = await axiosClient.get<DailyContent>(
      API_ENDPOINTS.DAILY_CONTENT
    );
    return response.data;
  }

  /**
   * Get user statistics
   */
  static async getUserStatistics(): Promise<UserStatistics> {
    const response = await axiosClient.get<UserStatistics>(
      API_ENDPOINTS.USER_STATISTICS
    );
    return response.data;
  }

  /**
   * Get progress color based on percentage
   */
  static getProgressColor(percentage: number): string {
    if (percentage >= 100) return '#28A745'; // success
    if (percentage >= 70) return '#2D5A27'; // secondary
    if (percentage >= 40) return '#B8860B'; // accent
    return '#FFC107'; // warning
  }

  /**
   * Get motivational message based on progress
   */
  static getMotivationalMessage(percentage: number): string {
    if (percentage >= 100) return 'ما شاء الله! أكملت وردك اليومي 🎉';
    if (percentage >= 70) return 'أحسنت! أنت قريب من إتمام وردك';
    if (percentage >= 40) return 'استمر! أنت في منتصف الطريق';
    if (percentage >= 20) return 'بداية موفقة! واصل القراءة';
    return 'ابدأ وردك اليومي الآن';
  }
}

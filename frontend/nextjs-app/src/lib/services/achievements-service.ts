/**
 * Achievements service for Next.js web app
 * Handles all achievements, badges, levels, and challenges API calls
 */

import axiosClient from '../api/axios-client';
import {
  Achievement,
  AchievementCategory,
  AchievementTier,
  AchievementsDashboard,
  AchievementStats,
  AchievementUnlockNotification,
  Challenge,
  ChallengeType,
  LeaderboardEntry,
  MotivationalReminder,
  ShareAchievementRequest,
  UserLevel,
} from '@/types/achievements';

const ACHIEVEMENTS_BASE = '/api/achievements';

export class AchievementsService {
  /**
   * Get achievements dashboard with all data
   */
  static async getAchievementsDashboard(): Promise<AchievementsDashboard> {
    const response = await axiosClient.get<AchievementsDashboard>(
      `${ACHIEVEMENTS_BASE}/dashboard`
    );
    return response.data;
  }

  /**
   * Get all achievements (locked and unlocked)
   */
  static async getAllAchievements(params?: {
    category?: AchievementCategory;
    tier?: AchievementTier;
    isUnlocked?: boolean;
  }): Promise<Achievement[]> {
    const response = await axiosClient.get<Achievement[]>(
      `${ACHIEVEMENTS_BASE}/achievements`,
      { params }
    );
    return response.data;
  }

  /**
   * Get specific achievement details
   */
  static async getAchievement(achievementId: string): Promise<Achievement> {
    const response = await axiosClient.get<Achievement>(
      `${ACHIEVEMENTS_BASE}/achievements/${achievementId}`
    );
    return response.data;
  }

  /**
   * Get user level and points information
   */
  static async getUserLevel(): Promise<UserLevel> {
    const response = await axiosClient.get<UserLevel>(
      `${ACHIEVEMENTS_BASE}/level`
    );
    return response.data;
  }

  /**
   * Get active challenges (daily and weekly)
   */
  static async getActiveChallenges(params?: {
    type?: ChallengeType;
  }): Promise<Challenge[]> {
    const response = await axiosClient.get<Challenge[]>(
      `${ACHIEVEMENTS_BASE}/challenges`,
      { params }
    );
    return response.data;
  }

  /**
   * Get specific challenge details
   */
  static async getChallenge(challengeId: string): Promise<Challenge> {
    const response = await axiosClient.get<Challenge>(
      `${ACHIEVEMENTS_BASE}/challenges/${challengeId}`
    );
    return response.data;
  }

  /**
   * Update challenge progress
   */
  static async updateChallengeProgress(
    challengeId: string,
    progressValue: number
  ): Promise<Challenge> {
    const response = await axiosClient.post<Challenge>(
      `${ACHIEVEMENTS_BASE}/challenges/${challengeId}/progress`,
      { progress_value: progressValue }
    );
    return response.data;
  }

  /**
   * Get achievement statistics
   */
  static async getAchievementStats(): Promise<AchievementStats> {
    const response = await axiosClient.get<AchievementStats>(
      `${ACHIEVEMENTS_BASE}/stats`
    );
    return response.data;
  }

  /**
   * Get motivational reminders
   */
  static async getReminders(params?: {
    isActive?: boolean;
  }): Promise<MotivationalReminder[]> {
    const response = await axiosClient.get<MotivationalReminder[]>(
      `${ACHIEVEMENTS_BASE}/reminders`,
      { params }
    );
    return response.data;
  }

  /**
   * Create or update a reminder
   */
  static async saveReminder(
    reminder: MotivationalReminder
  ): Promise<MotivationalReminder> {
    const response = await axiosClient.post<MotivationalReminder>(
      `${ACHIEVEMENTS_BASE}/reminders`,
      reminder
    );
    return response.data;
  }

  /**
   * Delete a reminder
   */
  static async deleteReminder(reminderId: string): Promise<void> {
    await axiosClient.delete(`${ACHIEVEMENTS_BASE}/reminders/${reminderId}`);
  }

  /**
   * Share achievement on social media
   */
  static async shareAchievement(
    request: ShareAchievementRequest
  ): Promise<{ shareUrl?: string; message: string }> {
    const response = await axiosClient.post<{
      shareUrl?: string;
      message: string;
    }>(`${ACHIEVEMENTS_BASE}/share`, request);
    return response.data;
  }

  /**
   * Get achievement unlock history
   */
  static async getUnlockHistory(params?: {
    limit?: number;
    since?: string;
  }): Promise<AchievementUnlockNotification[]> {
    const response = await axiosClient.get<AchievementUnlockNotification[]>(
      `${ACHIEVEMENTS_BASE}/unlock-history`,
      { params }
    );
    return response.data;
  }

  /**
   * Manually trigger achievement check (for testing/debugging)
   */
  static async checkAchievements(): Promise<Achievement[]> {
    const response = await axiosClient.post<Achievement[]>(
      `${ACHIEVEMENTS_BASE}/check`
    );
    return response.data;
  }

  /**
   * Get leaderboard (if social features are enabled)
   */
  static async getLeaderboard(params?: {
    timeframe?: 'daily' | 'weekly' | 'monthly' | 'all_time';
    limit?: number;
  }): Promise<LeaderboardEntry[]> {
    const response = await axiosClient.get<LeaderboardEntry[]>(
      `${ACHIEVEMENTS_BASE}/leaderboard`,
      { params }
    );
    return response.data;
  }
}

export default AchievementsService;

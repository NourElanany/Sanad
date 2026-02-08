/**
 * Achievement and rewards system types for Next.js web app
 */

export enum AchievementCategory {
  QuranReading = 'quranReading',
  KhatmaCompletion = 'khatmaCompletion',
  Recitation = 'recitation',
  Consistency = 'consistency',
  Learning = 'learning',
  Prayer = 'prayer',
  General = 'general',
}

export enum AchievementTier {
  Bronze = 'bronze',
  Silver = 'silver',
  Gold = 'gold',
  Platinum = 'platinum',
  Diamond = 'diamond',
}

export interface Achievement {
  id: string;
  titleAr: string;
  titleEn: string;
  descriptionAr: string;
  descriptionEn: string;
  category: AchievementCategory;
  tier: AchievementTier;
  iconName: string;
  pointsReward: number;
  isUnlocked: boolean;
  unlockedAt?: string;
  progress: number; // 0.0 to 1.0
  currentValue: number;
  targetValue: number;
  requirements: string[];
}

export interface UserLevel {
  userId: string;
  currentLevel: number;
  totalPoints: number;
  pointsInCurrentLevel: number;
  pointsRequiredForNextLevel: number;
  progressToNextLevel: number; // 0.0 to 1.0
  levelTitle: string;
  levelTitleAr: string;
  unlockedPerks: string[];
  lastUpdated: string;
}

export enum ChallengeType {
  Daily = 'daily',
  Weekly = 'weekly',
  Special = 'special',
}

export enum ChallengeDifficulty {
  Easy = 'easy',
  Medium = 'medium',
  Hard = 'hard',
  Expert = 'expert',
}

export interface Challenge {
  id: string;
  titleAr: string;
  titleEn: string;
  descriptionAr: string;
  descriptionEn: string;
  type: ChallengeType;
  difficulty: ChallengeDifficulty;
  pointsReward: number;
  targetValue: number;
  currentProgress: number;
  progressPercentage: number;
  startDate: string;
  endDate: string;
  isCompleted: boolean;
  completedAt?: string;
  iconName: string;
  requirements: string[];
}

export interface AchievementStats {
  totalAchievements: number;
  unlockedAchievements: number;
  lockedAchievements: number;
  completionPercentage: number;
  totalChallengesCompleted: number;
  currentStreak: number;
  longestStreak: number;
  achievementsByCategory: Record<AchievementCategory, number>;
  achievementsByTier: Record<AchievementTier, number>;
}

export enum ReminderType {
  AchievementProgress = 'achievementProgress',
  ChallengeDeadline = 'challengeDeadline',
  StreakMaintenance = 'streakMaintenance',
  LevelUp = 'levelUp',
  General = 'general',
}

export interface MotivationalReminder {
  id: string;
  messageAr: string;
  messageEn: string;
  type: ReminderType;
  scheduledFor: string;
  isActive: boolean;
  relatedAchievementId?: string;
  relatedChallengeId?: string;
}

export interface AchievementsDashboard {
  userId: string;
  userLevel: UserLevel;
  recentAchievements: Achievement[];
  inProgressAchievements: Achievement[];
  activeChallenges: Challenge[];
  stats: AchievementStats;
  reminders: MotivationalReminder[];
  generatedAt: string;
}

export interface AchievementUnlockNotification {
  achievement: Achievement;
  pointsEarned: number;
  leveledUp: boolean;
  newLevel?: number;
  unlockedAt: string;
}

export enum SharePlatform {
  Twitter = 'twitter',
  Facebook = 'facebook',
  WhatsApp = 'whatsapp',
  Telegram = 'telegram',
  Instagram = 'instagram',
  Clipboard = 'clipboard',
}

export interface ShareAchievementRequest {
  achievementId: string;
  platform: SharePlatform;
  customMessage?: string;
}

export interface LeaderboardEntry {
  userId: string;
  username: string;
  level: number;
  totalPoints: number;
  rank: number;
  avatarUrl?: string;
}

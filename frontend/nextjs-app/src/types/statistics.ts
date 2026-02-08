// Statistics Dashboard Types

export interface StatisticsDashboard {
  userId: string;
  khatmaStats: KhatmaStatistics;
  readingStats: ReadingStatistics;
  recitationStats: RecitationStatistics;
  weeklyComparison: WeeklyComparison;
  monthlyComparison: MonthlyComparison;
  personalGoals: PersonalGoal[];
  generatedAt: string;
}

export interface KhatmaStatistics {
  totalCompleted: number;
  currentProgress: number;
  completionHistory: KhatmaCompletionData[];
  averageCompletionDays: number;
  currentStreak: number;
  longestStreak: number;
}

export interface KhatmaCompletionData {
  khatmaId: string;
  completionDate: string;
  durationDays: number;
  consistencyScore: number;
}

export interface ReadingStatistics {
  totalMinutesToday: number;
  totalMinutesWeek: number;
  totalMinutesMonth: number;
  averageDailyMinutes: number;
  dailyReadingHistory: DailyReadingData[];
  pagesReadTotal: number;
  surahsCompleted: number;
}

export interface DailyReadingData {
  date: string;
  minutes: number;
  pagesRead: number;
  readingSpeedWpm: number;
}

export interface RecitationStatistics {
  currentScore: number;
  averageScore: number;
  improvementPercentage: number;
  scoreHistory: RecitationScoreData[];
  totalRecitations: number;
  errorTypeFrequency: Record<string, number>;
  topImprovementAreas: string[];
}

export interface RecitationScoreData {
  date: string;
  score: number;
  errorCount: number;
  surahName: string;
}

export interface WeeklyComparison {
  currentWeekMinutes: number;
  previousWeekMinutes: number;
  changePercentage: number;
  trend: 'improving' | 'stable' | 'declining';
  weeklyData: WeeklyDataPoint[];
}

export interface WeeklyDataPoint {
  weekStart: string;
  totalMinutes: number;
  sessionsCount: number;
  averageScore: number;
}

export interface MonthlyComparison {
  currentMonthMinutes: number;
  previousMonthMinutes: number;
  changePercentage: number;
  trend: 'improving' | 'stable' | 'declining';
  monthlyData: MonthlyDataPoint[];
}

export interface MonthlyDataPoint {
  monthStart: string;
  totalMinutes: number;
  khatmasCompleted: number;
  consistencyScore: number;
}

export interface PersonalGoal {
  id: string;
  title: string;
  description: string;
  type: GoalType;
  targetValue: number;
  currentValue: number;
  deadline: string;
  isCompleted: boolean;
  progressPercentage: number;
}

export enum GoalType {
  DailyReading = 'dailyReading',
  WeeklyReading = 'weeklyReading',
  MonthlyReading = 'monthlyReading',
  KhatmaCompletion = 'khatmaCompletion',
  RecitationImprovement = 'recitationImprovement',
  ConsistencyStreak = 'consistencyStreak',
  Custom = 'custom',
}

export interface CreateGoalRequest {
  title: string;
  description: string;
  type: GoalType;
  targetValue: number;
  deadline: string;
}

'use client';

import { DailyWird, DashboardService } from '@/lib/services/dashboard-service';

interface DailyWirdCardProps {
  dailyWird: DailyWird;
  onTap?: () => void;
}

export function DailyWirdCard({ dailyWird, onTap }: DailyWirdCardProps) {
  const progressPercentage = dailyWird.progressPercentage;
  const completedPages = dailyWird.completedPages;
  const totalPages = dailyWird.totalPages;
  const progressColor = DashboardService.getProgressColor(progressPercentage);
  const motivationalMessage = DashboardService.getMotivationalMessage(progressPercentage);

  return (
    <div
      className="bg-white rounded-2xl shadow-lg border border-primary/10 p-6 cursor-pointer hover:shadow-xl transition-shadow"
      onClick={onTap}
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <div className="bg-secondary/10 p-3 rounded-xl">
            <span className="text-2xl">📖</span>
          </div>
          <div>
            <h3 className="text-xl font-bold text-primary">وردك اليومي</h3>
            <p className="text-sm text-gray-600">
              {completedPages} من {totalPages} صفحات
            </p>
          </div>
        </div>
        <div
          className="px-3 py-1 rounded-full text-sm font-bold"
          style={{
            backgroundColor: `${progressColor}20`,
            color: progressColor,
          }}
        >
          {progressPercentage.toFixed(0)}%
        </div>
      </div>

      {/* Progress Bar */}
      <div className="mb-3">
        <div className="h-3 bg-gray-200 rounded-full overflow-hidden">
          <div
            className="h-full transition-all duration-500 ease-out rounded-full"
            style={{
              width: `${progressPercentage}%`,
              backgroundColor: progressColor,
            }}
          />
        </div>
      </div>

      {/* Motivational Message */}
      <div className="flex items-center gap-2 text-sm">
        <span>{progressPercentage >= 100 ? '✅' : '🏆'}</span>
        <p
          className={progressPercentage >= 100 ? 'text-green-600 font-semibold' : 'text-gray-600 italic'}
        >
          {motivationalMessage}
        </p>
      </div>
    </div>
  );
}

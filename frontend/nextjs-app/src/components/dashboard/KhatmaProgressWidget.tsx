'use client';

import { useEffect, useState } from 'react';

interface KhatmaProgress {
  currentPage: number;
  totalPages: number;
  completedKhatmas: number;
  startDate?: Date;
  estimatedEndDate?: Date;
  dailyAverage: number;
}

interface KhatmaProgressWidgetProps {
  khatmaProgress?: KhatmaProgress;
  onTap?: () => void;
}

export function KhatmaProgressWidget({ khatmaProgress, onTap }: KhatmaProgressWidgetProps) {
  const [progress, setProgress] = useState<KhatmaProgress>({
    currentPage: 245,
    totalPages: 604,
    completedKhatmas: 3,
    dailyAverage: 5.4,
    startDate: new Date(Date.now() - 45 * 24 * 60 * 60 * 1000),
    estimatedEndDate: new Date(Date.now() + 60 * 24 * 60 * 60 * 1000),
  });

  useEffect(() => {
    if (khatmaProgress) {
      setProgress(khatmaProgress);
    }
  }, [khatmaProgress]);

  const progressPercentage = (progress.currentPage / progress.totalPages) * 100;
  const remainingPages = progress.totalPages - progress.currentPage;

  const getProgressColor = (percentage: number): string => {
    if (percentage >= 75) return '#28A745';
    if (percentage >= 50) return '#2D5A27';
    if (percentage >= 25) return '#B8860B';
    return '#1B365D';
  };

  const formatDate = (date: Date): string => {
    const months = [
      'يناير', 'فبراير', 'مارس', 'أبريل', 'مايو', 'يونيو',
      'يوليو', 'أغسطس', 'سبتمبر', 'أكتوبر', 'نوفمبر', 'ديسمبر'
    ];
    return `${date.getDate()} ${months[date.getMonth()]} ${date.getFullYear()}`;
  };

  const progressColor = getProgressColor(progressPercentage);

  return (
    <div
      className="bg-white rounded-2xl shadow-lg border border-primary/10 p-6 cursor-pointer hover:shadow-xl transition-all"
      onClick={onTap}
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <div className="bg-gradient-to-br from-secondary to-secondary/70 p-3 rounded-xl">
            <span className="text-2xl">📖</span>
          </div>
          <div>
            <h3 className="text-xl font-bold text-primary">ختمة القرآن الكريم</h3>
            <p className="text-sm text-gray-600">
              الصفحة {progress.currentPage} من {progress.totalPages}
            </p>
          </div>
        </div>
        {progress.completedKhatmas > 0 && (
          <div className="flex items-center gap-2 px-3 py-1.5 bg-accent/10 border border-accent/30 rounded-full">
            <span className="text-lg">🏆</span>
            <span className="text-sm font-bold text-accent">
              {progress.completedKhatmas}
            </span>
          </div>
        )}
      </div>

      {/* Circular Progress */}
      <div className="flex justify-center mb-6">
        <div className="relative w-36 h-36">
          <svg className="w-full h-full transform -rotate-90">
            {/* Background circle */}
            <circle
              cx="72"
              cy="72"
              r="64"
              stroke="#F8F9FA"
              strokeWidth="12"
              fill="none"
            />
            {/* Progress circle */}
            <circle
              cx="72"
              cy="72"
              r="64"
              stroke={progressColor}
              strokeWidth="12"
              fill="none"
              strokeDasharray={`${2 * Math.PI * 64}`}
              strokeDashoffset={`${2 * Math.PI * 64 * (1 - progressPercentage / 100)}`}
              strokeLinecap="round"
              className="transition-all duration-500"
            />
          </svg>
          <div className="absolute inset-0 flex flex-col items-center justify-center">
            <span className="text-3xl font-bold text-primary">
              {progressPercentage.toFixed(1)}%
            </span>
            <span className="text-sm text-gray-600">مكتمل</span>
          </div>
        </div>
      </div>

      {/* Statistics */}
      <div className="grid grid-cols-2 gap-3 mb-4">
        <div className="bg-primary/10 rounded-lg p-3 text-center">
          <div className="text-2xl mb-1">📄</div>
          <p className="text-xs text-gray-600 mb-1">متبقي</p>
          <p className="text-lg font-bold text-primary">
            {remainingPages} <span className="text-xs font-normal">صفحة</span>
          </p>
        </div>
        <div className="bg-secondary/10 rounded-lg p-3 text-center">
          <div className="text-2xl mb-1">📈</div>
          <p className="text-xs text-gray-600 mb-1">المعدل اليومي</p>
          <p className="text-lg font-bold text-secondary">
            {progress.dailyAverage.toFixed(1)} <span className="text-xs font-normal">صفحة</span>
          </p>
        </div>
      </div>

      {/* Estimated completion */}
      {progress.estimatedEndDate && (
        <div className="bg-accent/10 rounded-lg p-3 mb-4 flex items-center gap-2">
          <span className="text-lg">📅</span>
          <span className="text-sm text-gray-900">
            الإتمام المتوقع: {formatDate(progress.estimatedEndDate)}
          </span>
        </div>
      )}

      {/* Action button */}
      <button
        onClick={(e) => {
          e.stopPropagation();
          onTap?.();
        }}
        className="w-full bg-secondary hover:bg-secondary/90 text-white font-semibold py-3 rounded-xl flex items-center justify-center gap-2 transition-colors"
      >
        <span className="text-lg">▶️</span>
        <span>متابعة القراءة</span>
      </button>
    </div>
  );
}

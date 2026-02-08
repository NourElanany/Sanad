'use client';

import { useEffect, useState } from 'react';
import { AchievementsService } from '@/lib/services/achievements-service';
import {
  AchievementsDashboard,
  Achievement,
  Challenge,
  AchievementTier,
} from '@/types/achievements';

export default function AchievementsPage() {
  const [dashboard, setDashboard] = useState<AchievementsDashboard | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'overview' | 'achievements' | 'challenges'>('overview');

  useEffect(() => {
    loadDashboard();
  }, []);

  const loadDashboard = async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await AchievementsService.getAchievementsDashboard();
      setDashboard(data);
    } catch (err) {
      setError('فشل تحميل البيانات. يرجى المحاولة مرة أخرى.');
      console.error('Error loading achievements dashboard:', err);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-16 w-16 border-b-2 border-[#1B365D] mx-auto"></div>
          <p className="mt-4 text-gray-600 font-tajawal">جاري التحميل...</p>
        </div>
      </div>
    );
  }

  if (error || !dashboard) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="text-red-500 text-6xl mb-4">⚠️</div>
          <h2 className="text-2xl font-bold text-gray-800 mb-2 font-tajawal">حدث خطأ</h2>
          <p className="text-gray-600 mb-4 font-tajawal">{error}</p>
          <button
            onClick={loadDashboard}
            className="bg-[#1B365D] text-white px-6 py-2 rounded-lg hover:bg-[#2E4A6B] transition-colors font-tajawal"
          >
            إعادة المحاولة
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50" dir="rtl">
      {/* Header */}
      <header className="bg-[#1B365D] text-white shadow-lg">
        <div className="container mx-auto px-4 py-6">
          <h1 className="text-3xl font-bold font-tajawal">الإنجازات والمكافآت</h1>
          <p className="text-gray-200 mt-2 font-tajawal">تتبع تقدمك واحصل على المكافآت</p>
        </div>
      </header>

      {/* User Level Card */}
      <div className="container mx-auto px-4 py-8">
        <UserLevelCard userLevel={dashboard.userLevel} />

        {/* Tabs */}
        <div className="mt-8 bg-white rounded-lg shadow-md overflow-hidden">
          <div className="flex border-b">
            <button
              onClick={() => setActiveTab('overview')}
              className={`flex-1 py-4 px-6 font-tajawal font-semibold transition-colors ${
                activeTab === 'overview'
                  ? 'bg-[#1B365D] text-white'
                  : 'text-gray-600 hover:bg-gray-50'
              }`}
            >
              نظرة عامة
            </button>
            <button
              onClick={() => setActiveTab('achievements')}
              className={`flex-1 py-4 px-6 font-tajawal font-semibold transition-colors ${
                activeTab === 'achievements'
                  ? 'bg-[#1B365D] text-white'
                  : 'text-gray-600 hover:bg-gray-50'
              }`}
            >
              الإنجازات
            </button>
            <button
              onClick={() => setActiveTab('challenges')}
              className={`flex-1 py-4 px-6 font-tajawal font-semibold transition-colors ${
                activeTab === 'challenges'
                  ? 'bg-[#1B365D] text-white'
                  : 'text-gray-600 hover:bg-gray-50'
              }`}
            >
              التحديات
            </button>
          </div>

          <div className="p-6">
            {activeTab === 'overview' && <OverviewTab dashboard={dashboard} />}
            {activeTab === 'achievements' && (
              <AchievementsTab
                recent={dashboard.recentAchievements}
                inProgress={dashboard.inProgressAchievements}
              />
            )}
            {activeTab === 'challenges' && (
              <ChallengesTab challenges={dashboard.activeChallenges} />
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

function UserLevelCard({ userLevel }: { userLevel: any }) {
  return (
    <div className="bg-gradient-to-br from-[#1B365D] to-[#2E4A6B] rounded-2xl shadow-xl p-8 text-white">
      <div className="flex items-center gap-6">
        {/* Level Badge */}
        <div className="w-24 h-24 rounded-full border-4 border-[#B8860B] bg-[#B8860B]/20 flex flex-col items-center justify-center">
          <div className="text-4xl font-bold font-tajawal">{userLevel.currentLevel}</div>
          <div className="text-sm opacity-70 font-tajawal">المستوى</div>
        </div>

        {/* Level Info */}
        <div className="flex-1">
          <h2 className="text-3xl font-bold font-tajawal mb-2">{userLevel.levelTitleAr}</h2>
          <div className="flex items-center gap-2 text-lg">
            <span className="text-[#B8860B]">⭐</span>
            <span className="font-tajawal">{userLevel.totalPoints} نقطة</span>
          </div>
        </div>
      </div>

      {/* Progress Bar */}
      <div className="mt-6">
        <div className="flex justify-between text-sm mb-2 font-tajawal">
          <span>التقدم للمستوى التالي</span>
          <span>
            {userLevel.pointsInCurrentLevel} / {userLevel.pointsRequiredForNextLevel}
          </span>
        </div>
        <div className="w-full bg-white/20 rounded-full h-3">
          <div
            className="bg-[#B8860B] h-3 rounded-full transition-all duration-500"
            style={{ width: `${userLevel.progressToNextLevel * 100}%` }}
          ></div>
        </div>
        <div className="text-center text-sm mt-2 font-tajawal">
          {(userLevel.progressToNextLevel * 100).toFixed(0)}% مكتمل
        </div>
      </div>
    </div>
  );
}

function OverviewTab({ dashboard }: { dashboard: AchievementsDashboard }) {
  return (
    <div className="space-y-6">
      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard
          icon="🏆"
          label="الإنجازات"
          value={`${dashboard.stats.unlockedAchievements}/${dashboard.stats.totalAchievements}`}
          color="text-[#B8860B]"
        />
        <StatCard
          icon="✅"
          label="التحديات"
          value={dashboard.stats.totalChallengesCompleted.toString()}
          color="text-green-600"
        />
        <StatCard
          icon="🔥"
          label="السلسلة الحالية"
          value={`${dashboard.stats.currentStreak} يوم`}
          color="text-red-500"
        />
        <StatCard
          icon="📈"
          label="أطول سلسلة"
          value={`${dashboard.stats.longestStreak} يوم`}
          color="text-purple-600"
        />
      </div>

      {/* Recent Achievements */}
      <div>
        <h3 className="text-xl font-bold mb-4 font-tajawal">الإنجازات الأخيرة</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {dashboard.recentAchievements.map((achievement) => (
            <AchievementCard key={achievement.id} achievement={achievement} />
          ))}
        </div>
      </div>

      {/* Active Challenges */}
      <div>
        <h3 className="text-xl font-bold mb-4 font-tajawal">التحديات النشطة</h3>
        <div className="space-y-4">
          {dashboard.activeChallenges.map((challenge) => (
            <ChallengeCard key={challenge.id} challenge={challenge} />
          ))}
        </div>
      </div>
    </div>
  );
}

function AchievementsTab({
  recent,
  inProgress,
}: {
  recent: Achievement[];
  inProgress: Achievement[];
}) {
  return (
    <div className="space-y-8">
      <div>
        <h3 className="text-xl font-bold mb-4 font-tajawal">الإنجازات الأخيرة</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {recent.map((achievement) => (
            <AchievementCard key={achievement.id} achievement={achievement} />
          ))}
        </div>
      </div>

      <div>
        <h3 className="text-xl font-bold mb-4 font-tajawal">إنجازات قيد التقدم</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {inProgress.map((achievement) => (
            <AchievementCard key={achievement.id} achievement={achievement} />
          ))}
        </div>
      </div>
    </div>
  );
}

function ChallengesTab({ challenges }: { challenges: Challenge[] }) {
  return (
    <div className="space-y-4">
      {challenges.map((challenge) => (
        <ChallengeCard key={challenge.id} challenge={challenge} />
      ))}
    </div>
  );
}

function StatCard({
  icon,
  label,
  value,
  color,
}: {
  icon: string;
  label: string;
  value: string;
  color: string;
}) {
  return (
    <div className="bg-white border border-gray-200 rounded-lg p-6 text-center">
      <div className="text-4xl mb-2">{icon}</div>
      <div className={`text-2xl font-bold ${color} font-tajawal`}>{value}</div>
      <div className="text-sm text-gray-600 mt-1 font-tajawal">{label}</div>
    </div>
  );
}

function AchievementCard({ achievement }: { achievement: Achievement }) {
  const getTierColor = (tier: AchievementTier) => {
    switch (tier) {
      case AchievementTier.Bronze:
        return 'from-orange-400 to-orange-600';
      case AchievementTier.Silver:
        return 'from-gray-300 to-gray-500';
      case AchievementTier.Gold:
        return 'from-yellow-400 to-yellow-600';
      case AchievementTier.Platinum:
        return 'from-gray-200 to-gray-400';
      case AchievementTier.Diamond:
        return 'from-blue-300 to-blue-500';
    }
  };

  return (
    <div
      className={`bg-white border border-gray-200 rounded-lg p-6 ${
        achievement.isUnlocked ? '' : 'opacity-60'
      }`}
    >
      <div className="flex items-start gap-4">
        <div
          className={`w-16 h-16 rounded-full bg-gradient-to-br ${getTierColor(
            achievement.tier
          )} flex items-center justify-center text-white text-2xl`}
        >
          🏆
        </div>
        <div className="flex-1">
          <h4 className="font-bold text-lg font-tajawal">{achievement.titleAr}</h4>
          <p className="text-sm text-gray-600 mt-1 font-tajawal">
            {achievement.descriptionAr}
          </p>
          {!achievement.isUnlocked && (
            <div className="mt-3">
              <div className="w-full bg-gray-200 rounded-full h-2">
                <div
                  className="bg-[#1B365D] h-2 rounded-full"
                  style={{ width: `${achievement.progress * 100}%` }}
                ></div>
              </div>
              <p className="text-xs text-gray-500 mt-1 font-tajawal">
                {achievement.currentValue} / {achievement.targetValue}
              </p>
            </div>
          )}
          <div className="flex items-center gap-2 mt-2">
            <span className="text-[#B8860B]">⭐</span>
            <span className="text-sm font-bold text-[#B8860B] font-tajawal">
              {achievement.pointsReward} نقطة
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}

function ChallengeCard({ challenge }: { challenge: Challenge }) {
  return (
    <div className="bg-white border border-gray-200 rounded-lg p-6">
      <div className="flex items-start justify-between mb-4">
        <div>
          <h4 className="font-bold text-lg font-tajawal">{challenge.titleAr}</h4>
          <p className="text-sm text-gray-600 mt-1 font-tajawal">
            {challenge.descriptionAr}
          </p>
        </div>
        <span className="bg-blue-100 text-blue-800 text-xs font-semibold px-3 py-1 rounded-full font-tajawal">
          {challenge.type === 'daily' ? 'يومي' : 'أسبوعي'}
        </span>
      </div>

      <div className="mb-4">
        <div className="flex justify-between text-sm mb-2 font-tajawal">
          <span>التقدم</span>
          <span>
            {challenge.currentProgress} / {challenge.targetValue}
          </span>
        </div>
        <div className="w-full bg-gray-200 rounded-full h-3">
          <div
            className="bg-green-500 h-3 rounded-full"
            style={{ width: `${challenge.progressPercentage}%` }}
          ></div>
        </div>
      </div>

      <div className="flex items-center justify-between text-sm">
        <span className="text-gray-600 font-tajawal">⏰ ينتهي قريباً</span>
        <span className="text-[#B8860B] font-bold font-tajawal">
          ⭐ {challenge.pointsReward} نقطة
        </span>
      </div>
    </div>
  );
}

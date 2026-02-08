'use client';

import React, { useState } from 'react';
import { RecitationAnalysis, RecordingMetadata, TajweedError, Recommendation } from '@/types/recording';
import {
  CheckCircle,
  AlertCircle,
  Info,
  Lightbulb,
  Play,
  Share2,
  Mic,
  TrendingUp,
  ChevronDown,
  ChevronUp,
} from 'lucide-react';

interface AnalysisResultsProps {
  analysis: RecitationAnalysis;
  metadata: RecordingMetadata;
  onRecordAgain?: () => void;
  onViewProgress?: () => void;
}

export const AnalysisResults: React.FC<AnalysisResultsProps> = ({
  analysis,
  metadata,
  onRecordAgain,
  onViewProgress,
}) => {
  const getScoreColor = (score: number): string => {
    if (score >= 90) return 'text-green-600';
    if (score >= 80) return 'text-green-700';
    if (score >= 70) return 'text-yellow-600';
    if (score >= 60) return 'text-yellow-500';
    return 'text-red-600';
  };

  const getScoreBgColor = (score: number): string => {
    if (score >= 90) return 'bg-green-600';
    if (score >= 80) return 'bg-green-700';
    if (score >= 70) return 'bg-yellow-600';
    if (score >= 60) return 'bg-yellow-500';
    return 'bg-red-600';
  };

  const getScoreLabel = (score: number): string => {
    if (score >= 90) return 'ممتاز';
    if (score >= 80) return 'جيد جداً';
    if (score >= 70) return 'جيد';
    if (score >= 60) return 'مقبول';
    return 'يحتاج تحسين';
  };

  return (
    <div className="space-y-6">
      {/* Overall Score */}
      <div className="bg-gradient-to-br from-navy to-navy-light rounded-2xl shadow-xl p-8">
        <h2 className="text-xl font-bold text-white text-center mb-6 font-tajawal">
          النتيجة الإجمالية
        </h2>
        <div className="flex items-center justify-center">
          <div className="relative w-36 h-36">
            <svg className="w-full h-full transform -rotate-90">
              <circle
                cx="72"
                cy="72"
                r="64"
                stroke="rgba(255,255,255,0.2)"
                strokeWidth="12"
                fill="none"
              />
              <circle
                cx="72"
                cy="72"
                r="64"
                stroke="white"
                strokeWidth="12"
                fill="none"
                strokeDasharray={`${(analysis.overallScore / 100) * 402} 402`}
                strokeLinecap="round"
              />
            </svg>
            <div className="absolute inset-0 flex flex-col items-center justify-center">
              <span className="text-4xl font-bold text-white font-tajawal">
                {analysis.overallScore.toFixed(1)}%
              </span>
              <span className="text-sm text-white/90 font-tajawal">
                {getScoreLabel(analysis.overallScore)}
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* Recording Info */}
      <div className="bg-white rounded-2xl shadow-md p-6 space-y-4">
        <InfoRow icon={<CheckCircle className="w-5 h-5" />} label="السورة" value={`سورة رقم ${metadata.surahNumber}`} />
        <div className="border-t border-gray-200" />
        <InfoRow
          icon={<CheckCircle className="w-5 h-5" />}
          label="الآيات"
          value={`من ${metadata.ayahStart} إلى ${metadata.ayahEnd}`}
        />
        <div className="border-t border-gray-200" />
        <InfoRow
          icon={<CheckCircle className="w-5 h-5" />}
          label="المدة"
          value={formatDuration(metadata.duration)}
        />
        <div className="border-t border-gray-200" />
        <InfoRow
          icon={<CheckCircle className="w-5 h-5" />}
          label="التاريخ"
          value={formatDate(analysis.analyzedAt)}
        />
      </div>

      {/* Detailed Scores */}
      <div className="bg-white rounded-2xl shadow-md p-6 space-y-4">
        <h3 className="text-lg font-bold text-navy font-tajawal mb-4">التقييم التفصيلي</h3>
        <ScoreBar label="دقة النطق" score={analysis.detailedScores.pronunciationAccuracy} />
        <ScoreBar label="دقة التوقيت" score={analysis.detailedScores.timingAccuracy} />
        <ScoreBar label="التزام التجويد" score={analysis.detailedScores.tajweedCompliance} />
        <ScoreBar label="الطلاقة" score={analysis.detailedScores.fluency} />
        <ScoreBar label="الوضوح" score={analysis.detailedScores.clarity} />
        <ScoreBar label="الإيقاع" score={analysis.detailedScores.rhythm} />
      </div>

      {/* Errors */}
      {analysis.errors.length > 0 && (
        <div className="bg-white rounded-2xl shadow-md p-6 space-y-4">
          <div className="flex items-center gap-2">
            <h3 className="text-lg font-bold text-navy font-tajawal">الأخطاء المكتشفة</h3>
            <span className="px-2 py-1 bg-gold/20 text-gold text-sm font-bold rounded-lg font-tajawal">
              {analysis.errors.length}
            </span>
          </div>
          <div className="space-y-3">
            {analysis.errors.map((error, index) => (
              <ErrorCard key={index} error={error} />
            ))}
          </div>
        </div>
      )}

      {/* Recommendations */}
      {analysis.recommendations.length > 0 && (
        <div className="bg-white rounded-2xl shadow-md p-6 space-y-4">
          <div className="flex items-center gap-2">
            <h3 className="text-lg font-bold text-navy font-tajawal">توصيات التحسين</h3>
            <span className="px-2 py-1 bg-gold/20 text-gold text-sm font-bold rounded-lg font-tajawal">
              {analysis.recommendations.length}
            </span>
          </div>
          <div className="space-y-3">
            {analysis.recommendations.map((rec, index) => (
              <RecommendationCard key={index} recommendation={rec} />
            ))}
          </div>
        </div>
      )}

      {/* Action Buttons */}
      <div className="space-y-3">
        <button
          onClick={onRecordAgain}
          className="w-full bg-green-600 hover:bg-green-700 text-white font-bold py-4 px-6
                     rounded-xl shadow-lg transition-all duration-200 hover:scale-105
                     active:scale-95 font-tajawal text-lg flex items-center justify-center gap-2"
        >
          <Mic className="w-5 h-5" />
          تسجيل مرة أخرى
        </button>
        <button
          onClick={onViewProgress}
          className="w-full bg-white hover:bg-gray-50 text-navy font-bold py-4 px-6
                     rounded-xl shadow-md border-2 border-navy transition-all duration-200
                     hover:scale-105 active:scale-95 font-tajawal text-lg flex items-center justify-center gap-2"
        >
          <TrendingUp className="w-5 h-5" />
          عرض التقدم
        </button>
      </div>
    </div>
  );
};

// Helper Components

const InfoRow: React.FC<{ icon: React.ReactNode; label: string; value: string }> = ({
  icon,
  label,
  value,
}) => (
  <div className="flex items-center gap-3">
    <div className="p-2 bg-navy/10 rounded-lg text-navy">{icon}</div>
    <div className="flex-1">
      <p className="text-xs text-gray-500 font-tajawal">{label}</p>
      <p className="text-sm font-semibold text-gray-900 font-tajawal">{value}</p>
    </div>
  </div>
);

const ScoreBar: React.FC<{ label: string; score: number }> = ({ label, score }) => {
  const getScoreColor = (score: number): string => {
    if (score >= 90) return 'bg-green-600';
    if (score >= 80) return 'bg-green-700';
    if (score >= 70) return 'bg-yellow-600';
    if (score >= 60) return 'bg-yellow-500';
    return 'bg-red-600';
  };

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <span className="text-sm font-semibold text-gray-900 font-tajawal">{label}</span>
        <span className="text-sm font-bold text-gray-900 font-tajawal">{score.toFixed(0)}%</span>
      </div>
      <div className="w-full bg-gray-200 rounded-full h-2">
        <div
          className={`h-2 rounded-full transition-all duration-500 ${getScoreColor(score)}`}
          style={{ width: `${score}%` }}
        />
      </div>
    </div>
  );
};

const ErrorCard: React.FC<{ error: TajweedError }> = ({ error }) => {
  const [isExpanded, setIsExpanded] = useState(false);

  const getSeverityColor = (severity: string): string => {
    switch (severity) {
      case 'high':
        return 'border-red-500 bg-red-50';
      case 'medium':
        return 'border-yellow-500 bg-yellow-50';
      case 'low':
        return 'border-blue-500 bg-blue-50';
      default:
        return 'border-gray-300 bg-gray-50';
    }
  };

  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case 'high':
        return <AlertCircle className="w-5 h-5 text-red-600" />;
      case 'medium':
        return <AlertCircle className="w-5 h-5 text-yellow-600" />;
      case 'low':
        return <Info className="w-5 h-5 text-blue-600" />;
      default:
        return <Info className="w-5 h-5 text-gray-600" />;
    }
  };

  return (
    <div className={`border-2 rounded-xl overflow-hidden ${getSeverityColor(error.severity)}`}>
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="w-full p-4 flex items-start gap-3 hover:bg-black/5 transition-colors"
      >
        {getSeverityIcon(error.severity)}
        <div className="flex-1 text-right">
          <h4 className="font-bold text-gray-900 font-tajawal">{error.errorType}</h4>
          <p className="text-sm text-gray-600 font-tajawal mt-1">{error.description}</p>
          <p className="text-xs text-gray-500 font-tajawal mt-1">
            {formatTimestamp(error.timestamp)}
          </p>
        </div>
        {isExpanded ? <ChevronUp className="w-5 h-5" /> : <ChevronDown className="w-5 h-5" />}
      </button>
      {isExpanded && error.correction && (
        <div className="p-4 bg-green-50 border-t border-green-200">
          <div className="flex items-start gap-2 mb-3">
            <Lightbulb className="w-5 h-5 text-green-600 flex-shrink-0" />
            <div>
              <h5 className="font-bold text-green-900 font-tajawal mb-1">التصحيح</h5>
              <p className="text-sm text-gray-700 font-tajawal">{error.correction}</p>
            </div>
          </div>
          <button className="flex items-center gap-2 px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors font-tajawal text-sm">
            <Play className="w-4 h-4" />
            استماع للنطق الصحيح
          </button>
        </div>
      )}
    </div>
  );
};

const RecommendationCard: React.FC<{ recommendation: Recommendation }> = ({ recommendation }) => {
  const [isExpanded, setIsExpanded] = useState(false);

  return (
    <div className="border-2 border-gray-200 rounded-xl overflow-hidden bg-white">
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="w-full p-4 flex items-start gap-3 hover:bg-gray-50 transition-colors"
      >
        <Lightbulb className="w-5 h-5 text-gold flex-shrink-0" />
        <div className="flex-1 text-right">
          <h4 className="font-bold text-gray-900 font-tajawal">{recommendation.category}</h4>
          <p className="text-sm text-gray-600 font-tajawal mt-1">{recommendation.description}</p>
        </div>
        {isExpanded ? <ChevronUp className="w-5 h-5" /> : <ChevronDown className="w-5 h-5" />}
      </button>
      {isExpanded && (
        <div className="p-4 bg-gray-50 border-t border-gray-200 space-y-4">
          <div>
            <h5 className="font-bold text-gray-900 font-tajawal mb-2">نصيحة محددة</h5>
            <p className="text-sm text-gray-700 font-tajawal">{recommendation.specificAdvice}</p>
          </div>
          {recommendation.practiceExercises.length > 0 && (
            <div>
              <h5 className="font-bold text-gray-900 font-tajawal mb-2">تمارين مقترحة</h5>
              <ul className="space-y-2">
                {recommendation.practiceExercises.map((exercise, index) => (
                  <li key={index} className="flex items-start gap-2">
                    <span className="w-1.5 h-1.5 bg-green-600 rounded-full mt-2 flex-shrink-0" />
                    <span className="text-sm text-gray-700 font-tajawal">{exercise}</span>
                  </li>
                ))}
              </ul>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

// Helper Functions

const formatDuration = (ms: number): string => {
  const minutes = Math.floor(ms / 60000);
  const seconds = Math.floor((ms % 60000) / 1000);
  return `${minutes}:${seconds.toString().padStart(2, '0')}`;
};

const formatDate = (date: Date): string => {
  return new Date(date).toLocaleDateString('ar-SA');
};

const formatTimestamp = (seconds: number): string => {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
};

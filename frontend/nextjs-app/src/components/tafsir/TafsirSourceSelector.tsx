'use client';

import React, { useState } from 'react';
import type { TafsirSource, ScholarlyAuthentication, TafsirSourceType } from '@/types/tafsir';

interface TafsirSourceSelectorProps {
  sources: TafsirSource[];
  selectedSources: string[];
  onSourcesChange: (sourceIds: string[]) => void;
  onDownloadOffline?: () => void;
}

export const TafsirSourceSelector: React.FC<TafsirSourceSelectorProps> = ({
  sources,
  selectedSources,
  onSourcesChange,
  onDownloadOffline,
}) => {
  const [showAll, setShowAll] = useState(false);

  const getAuthenticationBadge = (auth: ScholarlyAuthentication) => {
    const badges = {
      highly_authenticated: { text: 'موثق بدرجة عالية', color: 'bg-green-100 text-green-800' },
      authenticated: { text: 'موثق', color: 'bg-blue-100 text-blue-800' },
      verified: { text: 'محقق', color: 'bg-yellow-100 text-yellow-800' },
      unverified: { text: 'غير محقق', color: 'bg-gray-100 text-gray-800' },
    };
    return badges[auth] || badges.unverified;
  };

  const getSourceTypeBadge = (type: TafsirSourceType) => {
    const badges = {
      classical: { text: 'كلاسيكي', icon: '📚' },
      contemporary: { text: 'معاصر', icon: '📖' },
      linguistic: { text: 'لغوي', icon: '🔤' },
      thematic: { text: 'موضوعي', icon: '🎯' },
      sectarian: { text: 'مذهبي', icon: '🕌' },
    };
    return badges[type] || badges.classical;
  };

  const getCredibilityColor = (score: number) => {
    if (score >= 9.0) return 'text-green-600';
    if (score >= 7.5) return 'text-blue-600';
    if (score >= 6.0) return 'text-yellow-600';
    return 'text-gray-600';
  };

  const toggleSource = (sourceId: string) => {
    if (selectedSources.includes(sourceId)) {
      onSourcesChange(selectedSources.filter((id) => id !== sourceId));
    } else {
      onSourcesChange([...selectedSources, sourceId]);
    }
  };

  const displayedSources = showAll ? sources : sources.slice(0, 5);

  return (
    <div className="bg-gray-50 rounded-lg p-4">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-bold text-[#1B365D] font-['Tajawal']">
          اختر مصادر التفسير
        </h3>
        <div className="flex gap-2">
          {onDownloadOffline && (
            <button
              onClick={onDownloadOffline}
              className="px-3 py-1 bg-[#2D5A27] text-white rounded-lg hover:bg-[#1A3318] transition-colors text-sm"
              title="تحميل للعمل دون اتصال"
            >
              ⬇️ تحميل
            </button>
          )}
          <span className="text-sm text-gray-600">
            {selectedSources.length} مصدر محدد
          </span>
        </div>
      </div>

      <div className="space-y-3">
        {displayedSources.map((source) => {
          const isSelected = selectedSources.includes(source.id);
          const authBadge = getAuthenticationBadge(source.scholarly_authentication);
          const typeBadge = getSourceTypeBadge(source.source_type);

          return (
            <div
              key={source.id}
              onClick={() => toggleSource(source.id)}
              className={`p-4 rounded-lg border-2 cursor-pointer transition-all ${
                isSelected
                  ? 'border-[#1B365D] bg-blue-50'
                  : 'border-gray-200 bg-white hover:border-gray-300'
              }`}
            >
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-2">
                    <input
                      type="checkbox"
                      checked={isSelected}
                      onChange={() => {}}
                      className="w-5 h-5 text-[#1B365D] rounded focus:ring-[#1B365D]"
                    />
                    <h4 className="font-bold text-[#1B365D] font-['Tajawal']" dir="rtl">
                      {source.name}
                    </h4>
                  </div>
                  
                  <p className="text-sm text-gray-600 mb-2" dir="rtl">
                    المؤلف: {source.author}
                  </p>

                  {source.description && (
                    <p className="text-sm text-gray-500 mb-2" dir="rtl">
                      {source.description}
                    </p>
                  )}

                  <div className="flex flex-wrap gap-2 items-center">
                    <span className={`px-2 py-1 rounded text-xs ${authBadge.color}`}>
                      {authBadge.text}
                    </span>
                    <span className="px-2 py-1 rounded text-xs bg-purple-100 text-purple-800">
                      {typeBadge.icon} {typeBadge.text}
                    </span>
                    <span className={`text-sm font-bold ${getCredibilityColor(source.credibility_score)}`}>
                      ⭐ {source.credibility_score.toFixed(1)}/10
                    </span>
                    {source.publication_year && (
                      <span className="text-xs text-gray-500">
                        📅 {source.publication_year}
                      </span>
                    )}
                  </div>
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {sources.length > 5 && (
        <button
          onClick={() => setShowAll(!showAll)}
          className="mt-4 w-full py-2 text-[#1B365D] hover:bg-gray-100 rounded-lg transition-colors text-sm font-medium"
        >
          {showAll ? '▲ عرض أقل' : `▼ عرض جميع المصادر (${sources.length})`}
        </button>
      )}
    </div>
  );
};

'use client';

import React, { useState, useEffect } from 'react';
import { TafsirService } from '@/lib/services/tafsir-service';
import type { TafsirComparisonResponse, ComparisonCriteria } from '@/types/tafsir';

interface TafsirComparisonProps {
  surahNumber: number;
  ayahNumber: number;
  selectedSources: string[];
}

export const TafsirComparison: React.FC<TafsirComparisonProps> = ({
  surahNumber,
  ayahNumber,
  selectedSources,
}) => {
  const [comparison, setComparison] = useState<TafsirComparisonResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [criteria, setCriteria] = useState<ComparisonCriteria[]>([
    ComparisonCriteria.Linguistic,
    ComparisonCriteria.Thematic,
  ]);

  useEffect(() => {
    if (selectedSources.length >= 2) {
      loadComparison();
    }
  }, [surahNumber, ayahNumber, selectedSources, criteria]);

  const loadComparison = async () => {
    try {
      setLoading(true);
      setError(null);

      const response = await TafsirService.compareTafsir({
        surah_number: surahNumber,
        ayah_number: ayahNumber,
        source_ids: selectedSources,
        comparison_criteria: criteria,
      });

      setComparison(response);
    } catch (err) {
      setError('فشل تحميل المقارنة');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const toggleCriteria = (criterion: ComparisonCriteria) => {
    if (criteria.includes(criterion)) {
      setCriteria(criteria.filter((c) => c !== criterion));
    } else {
      setCriteria([...criteria, criterion]);
    }
  };

  const getCriteriaLabel = (criterion: ComparisonCriteria) => {
    const labels = {
      [ComparisonCriteria.Linguistic]: '🔤 لغوي',
      [ComparisonCriteria.Thematic]: '🎯 موضوعي',
      [ComparisonCriteria.Historical]: '📜 تاريخي',
      [ComparisonCriteria.Jurisprudential]: '⚖️ فقهي',
      [ComparisonCriteria.Spiritual]: '✨ روحاني',
    };
    return labels[criterion];
  };

  const getSignificanceBadge = (significance: string) => {
    const badges = {
      major: { text: 'اختلاف كبير', color: 'bg-red-100 text-red-800' },
      moderate: { text: 'اختلاف متوسط', color: 'bg-yellow-100 text-yellow-800' },
      minor: { text: 'اختلاف طفيف', color: 'bg-green-100 text-green-800' },
    };
    return badges[significance as keyof typeof badges] || badges.minor;
  };

  if (selectedSources.length < 2) {
    return (
      <div className="text-center py-12 text-gray-500">
        <p className="text-lg">الرجاء اختيار مصدرين على الأقل للمقارنة</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Comparison Criteria Selector */}
      <div className="bg-gray-50 rounded-lg p-4">
        <h4 className="text-lg font-bold text-[#1B365D] mb-3 font-['Tajawal']" dir="rtl">
          معايير المقارنة
        </h4>
        <div className="flex flex-wrap gap-2">
          {Object.values(ComparisonCriteria).map((criterion) => (
            <button
              key={criterion}
              onClick={() => toggleCriteria(criterion)}
              className={`px-4 py-2 rounded-lg transition-colors ${
                criteria.includes(criterion)
                  ? 'bg-[#1B365D] text-white'
                  : 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-100'
              }`}
            >
              {getCriteriaLabel(criterion)}
            </button>
          ))}
        </div>
      </div>

      {loading && (
        <div className="flex items-center justify-center py-12">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-[#1B365D]"></div>
        </div>
      )}

      {error && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-4 text-red-700">
          {error}
        </div>
      )}

      {comparison && !loading && (
        <>
          {/* Summary */}
          <div className="bg-gradient-to-r from-blue-50 to-green-50 rounded-xl p-6 border-2 border-blue-200">
            <h4 className="text-xl font-bold text-[#1B365D] mb-4 font-['Tajawal']" dir="rtl">
              📊 ملخص المقارنة
            </h4>

            {/* Common Themes */}
            {comparison.summary.common_themes.length > 0 && (
              <div className="mb-4">
                <h5 className="text-sm font-bold text-gray-700 mb-2" dir="rtl">
                  🤝 المواضيع المشتركة:
                </h5>
                <div className="flex flex-wrap gap-2">
                  {comparison.summary.common_themes.map((theme, index) => (
                    <span
                      key={index}
                      className="px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm"
                    >
                      {theme}
                    </span>
                  ))}
                </div>
              </div>
            )}

            {/* Scholarly Consensus */}
            {comparison.summary.scholarly_consensus && (
              <div className="mb-4 p-4 bg-white rounded-lg">
                <h5 className="text-sm font-bold text-gray-700 mb-2" dir="rtl">
                  ✅ الإجماع العلمي:
                </h5>
                <p className="text-gray-800 font-['Tajawal']" dir="rtl">
                  {comparison.summary.scholarly_consensus}
                </p>
              </div>
            )}

            {/* Recommendations */}
            {comparison.recommendations.length > 0 && (
              <div>
                <h5 className="text-sm font-bold text-gray-700 mb-2" dir="rtl">
                  💡 التوصيات:
                </h5>
                <ul className="space-y-1">
                  {comparison.recommendations.map((rec, index) => (
                    <li key={index} className="text-sm text-gray-700 font-['Tajawal']" dir="rtl">
                      • {rec}
                    </li>
                  ))}
                </ul>
              </div>
            )}
          </div>

          {/* Divergent Views */}
          {comparison.summary.divergent_views.length > 0 && (
            <div className="space-y-4">
              <h4 className="text-xl font-bold text-[#1B365D] font-['Tajawal']" dir="rtl">
                🔍 الآراء المختلفة
              </h4>

              {comparison.summary.divergent_views.map((view, index) => {
                const badge = getSignificanceBadge(view.significance);
                return (
                  <div key={index} className="bg-white border-2 border-gray-200 rounded-xl p-6">
                    <div className="flex items-center justify-between mb-4">
                      <h5 className="text-lg font-bold text-[#1B365D] font-['Tajawal']" dir="rtl">
                        {view.topic}
                      </h5>
                      <span className={`px-3 py-1 rounded-full text-sm ${badge.color}`}>
                        {badge.text}
                      </span>
                    </div>

                    <div className="space-y-4">
                      {view.source_positions.map((position, posIndex) => (
                        <div key={posIndex} className="bg-gray-50 rounded-lg p-4">
                          <h6 className="font-bold text-gray-800 mb-2 font-['Tajawal']" dir="rtl">
                            📚 {position.source_name}
                          </h6>
                          <p className="text-gray-700 mb-2 font-['Tajawal']" dir="rtl">
                            {position.position}
                          </p>
                          {position.evidence.length > 0 && (
                            <div className="mt-2">
                              <p className="text-xs font-bold text-gray-600 mb-1" dir="rtl">
                                الأدلة:
                              </p>
                              <ul className="space-y-1">
                                {position.evidence.map((ev, evIndex) => (
                                  <li
                                    key={evIndex}
                                    className="text-sm text-gray-600 font-['Tajawal']"
                                    dir="rtl"
                                  >
                                    • {ev}
                                  </li>
                                ))}
                              </ul>
                            </div>
                          )}
                        </div>
                      ))}
                    </div>
                  </div>
                );
              })}
            </div>
          )}

          {/* Individual Comparisons */}
          <div className="space-y-4">
            <h4 className="text-xl font-bold text-[#1B365D] font-['Tajawal']" dir="rtl">
              📖 التفاصيل الكاملة
            </h4>

            {comparison.comparisons.map((comp, index) => (
              <div key={index} className="bg-white border-2 border-gray-200 rounded-xl p-6">
                <div className="flex items-center justify-between mb-4 pb-4 border-b border-gray-200">
                  <div>
                    <h5 className="text-lg font-bold text-[#1B365D] font-['Tajawal']" dir="rtl">
                      {comp.source.name}
                    </h5>
                    <p className="text-sm text-gray-600" dir="rtl">
                      {comp.source.author}
                    </p>
                  </div>
                  <div className="text-sm font-bold text-[#B8860B]">
                    ⭐ {comp.source.credibility_score.toFixed(1)}/10
                  </div>
                </div>

                {/* Key Points */}
                {comp.key_points.length > 0 && (
                  <div className="mb-4">
                    <h6 className="text-sm font-bold text-gray-700 mb-2" dir="rtl">
                      🔑 النقاط الرئيسية:
                    </h6>
                    <ul className="space-y-1">
                      {comp.key_points.map((point, pointIndex) => (
                        <li
                          key={pointIndex}
                          className="text-sm text-gray-700 font-['Tajawal']"
                          dir="rtl"
                        >
                          • {point}
                        </li>
                      ))}
                    </ul>
                  </div>
                )}

                {/* Unique Insights */}
                {comp.unique_insights.length > 0 && (
                  <div className="mb-4">
                    <h6 className="text-sm font-bold text-gray-700 mb-2" dir="rtl">
                      💎 الرؤى الفريدة:
                    </h6>
                    <ul className="space-y-1">
                      {comp.unique_insights.map((insight, insightIndex) => (
                        <li
                          key={insightIndex}
                          className="text-sm text-blue-700 font-['Tajawal']"
                          dir="rtl"
                        >
                          • {insight}
                        </li>
                      ))}
                    </ul>
                  </div>
                )}

                {/* Methodology Notes */}
                {comp.methodology_notes && (
                  <div className="bg-yellow-50 rounded-lg p-3">
                    <p className="text-sm text-gray-700 font-['Tajawal']" dir="rtl">
                      📝 {comp.methodology_notes}
                    </p>
                  </div>
                )}
              </div>
            ))}
          </div>
        </>
      )}
    </div>
  );
};

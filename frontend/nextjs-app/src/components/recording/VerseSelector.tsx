'use client';

import React, { useState, useEffect } from 'react';
import { VerseSelection, Surah } from '@/types/recording';
import { CheckCircle } from 'lucide-react';

interface VerseSelectorProps {
  initialSelection?: VerseSelection;
  onSelectionChanged: (selection: VerseSelection) => void;
}

// Sample surah data (in production, this would come from the API)
const SURAHS: Surah[] = [
  { number: 1, name: 'الفاتحة', ayahCount: 7, revelationType: 'meccan' },
  { number: 2, name: 'البقرة', ayahCount: 286, revelationType: 'medinan' },
  { number: 3, name: 'آل عمران', ayahCount: 200, revelationType: 'medinan' },
  { number: 4, name: 'النساء', ayahCount: 176, revelationType: 'medinan' },
  { number: 5, name: 'المائدة', ayahCount: 120, revelationType: 'medinan' },
  // Add more surahs as needed
];

export const VerseSelector: React.FC<VerseSelectorProps> = ({
  initialSelection,
  onSelectionChanged,
}) => {
  const [selectedSurah, setSelectedSurah] = useState<number | null>(
    initialSelection?.surahNumber ?? null
  );
  const [ayahStart, setAyahStart] = useState<number | null>(
    initialSelection?.ayahStart ?? null
  );
  const [ayahEnd, setAyahEnd] = useState<number | null>(
    initialSelection?.ayahEnd ?? null
  );

  const selectedSurahData = SURAHS.find((s) => s.number === selectedSurah);

  useEffect(() => {
    if (selectedSurah && ayahStart && ayahEnd && selectedSurahData) {
      onSelectionChanged({
        surahNumber: selectedSurah,
        surahName: selectedSurahData.name,
        ayahStart,
        ayahEnd,
        arabicText: '', // Would be fetched from API
      });
    }
  }, [selectedSurah, ayahStart, ayahEnd, selectedSurahData, onSelectionChanged]);

  const handleSurahChange = (surahNumber: number) => {
    setSelectedSurah(surahNumber);
    setAyahStart(null);
    setAyahEnd(null);
  };

  const handleAyahStartChange = (ayah: number) => {
    setAyahStart(ayah);
    if (ayahEnd && ayah > ayahEnd) {
      setAyahEnd(ayah);
    }
  };

  const getAyahOptions = (minAyah: number = 1) => {
    if (!selectedSurahData) return [];
    const options = [];
    for (let i = minAyah; i <= selectedSurahData.ayahCount; i++) {
      options.push(i);
    }
    return options;
  };

  return (
    <div className="bg-white rounded-2xl shadow-md p-6 space-y-6">
      <h2 className="text-xl font-bold text-navy text-center font-tajawal">
        اختر الآيات للتسجيل
      </h2>

      {/* Surah selector */}
      <div className="space-y-2">
        <label className="block text-sm font-semibold text-gray-600 font-tajawal">
          السورة
        </label>
        <select
          value={selectedSurah ?? ''}
          onChange={(e) => handleSurahChange(Number(e.target.value))}
          className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl
                     focus:outline-none focus:ring-2 focus:ring-navy/50 focus:border-navy
                     font-tajawal text-base"
        >
          <option value="">اختر السورة</option>
          {SURAHS.map((surah) => (
            <option key={surah.number} value={surah.number}>
              {surah.number}. {surah.name}
            </option>
          ))}
        </select>
      </div>

      {/* Ayah range selector */}
      {selectedSurah && (
        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-2">
            <label className="block text-sm font-semibold text-gray-600 font-tajawal">
              من الآية
            </label>
            <select
              value={ayahStart ?? ''}
              onChange={(e) => handleAyahStartChange(Number(e.target.value))}
              className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl
                         focus:outline-none focus:ring-2 focus:ring-navy/50 focus:border-navy
                         font-tajawal text-base"
            >
              <option value="">رقم الآية</option>
              {getAyahOptions().map((ayah) => (
                <option key={ayah} value={ayah}>
                  {ayah}
                </option>
              ))}
            </select>
          </div>

          <div className="space-y-2">
            <label className="block text-sm font-semibold text-gray-600 font-tajawal">
              إلى الآية
            </label>
            <select
              value={ayahEnd ?? ''}
              onChange={(e) => setAyahEnd(Number(e.target.value))}
              className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl
                         focus:outline-none focus:ring-2 focus:ring-navy/50 focus:border-navy
                         font-tajawal text-base"
              disabled={!ayahStart}
            >
              <option value="">رقم الآية</option>
              {getAyahOptions(ayahStart ?? 1).map((ayah) => (
                <option key={ayah} value={ayah}>
                  {ayah}
                </option>
              ))}
            </select>
          </div>
        </div>
      )}

      {/* Selection preview */}
      {selectedSurah && ayahStart && ayahEnd && selectedSurahData && (
        <div className="bg-navy/5 border border-navy/20 rounded-xl p-4 space-y-2">
          <div className="flex items-center justify-center gap-2">
            <CheckCircle className="w-5 h-5 text-green-600" />
            <span className="text-base font-bold text-navy font-tajawal">
              سورة {selectedSurahData.name}
            </span>
          </div>
          <p className="text-sm text-gray-600 text-center font-tajawal">
            من الآية {ayahStart} إلى الآية {ayahEnd}
          </p>
          <p className="text-xs text-gray-500 text-center font-tajawal">
            عدد الآيات: {ayahEnd - ayahStart + 1}
          </p>
        </div>
      )}
    </div>
  );
};

'use client';

import { useState } from 'react';
import type { HadithSearchFilters, HadithBook, HadithGrade } from '@/types/hadith';
import { getGradeArabicName, getGradeColor } from '@/types/hadith';

interface HadithFiltersProps {
  filters: HadithSearchFilters;
  books: HadithBook[];
  onFilterChange: (filters: HadithSearchFilters) => void;
}

export function HadithFilters({ filters, books, onFilterChange }: HadithFiltersProps) {
  const [localFilters, setLocalFilters] = useState<HadithSearchFilters>(filters);

  const searchTypes = [
    { value: 'text', label: 'نصي' },
    { value: 'semantic', label: 'دلالي' },
    { value: 'narrator', label: 'راوي' },
    { value: 'theme', label: 'موضوع' },
  ];

  const grades: HadithGrade[] = ['sahih', 'hasan', 'daif', 'mawdu'];

  const themes = [
    'عقيدة',
    'عبادة',
    'معاملات',
    'أسرة',
    'أخلاق',
    'تاريخ',
    'نبوءات',
    'فقه',
  ];

  const toggleBook = (bookName: string) => {
    const currentBooks = localFilters.books || [];
    const newBooks = currentBooks.includes(bookName)
      ? currentBooks.filter((b) => b !== bookName)
      : [...currentBooks, bookName];
    
    const newFilters = { ...localFilters, books: newBooks };
    setLocalFilters(newFilters);
    onFilterChange(newFilters);
  };

  const toggleGrade = (grade: HadithGrade) => {
    const currentGrades = localFilters.grades || [];
    const newGrades = currentGrades.includes(grade)
      ? currentGrades.filter((g) => g !== grade)
      : [...currentGrades, grade];
    
    const newFilters = { ...localFilters, grades: newGrades };
    setLocalFilters(newFilters);
    onFilterChange(newFilters);
  };

  const toggleTheme = (theme: string) => {
    const currentThemes = localFilters.themes || [];
    const newThemes = currentThemes.includes(theme)
      ? currentThemes.filter((t) => t !== theme)
      : [...currentThemes, theme];
    
    const newFilters = { ...localFilters, themes: newThemes };
    setLocalFilters(newFilters);
    onFilterChange(newFilters);
  };

  const setSearchType = (type: string) => {
    const newFilters = { ...localFilters, searchType: type as any };
    setLocalFilters(newFilters);
    onFilterChange(newFilters);
  };

  return (
    <div className="bg-white border border-gray-200 rounded-lg p-6 space-y-6">
      {/* Search Type */}
      <div>
        <h3 className="text-base font-bold text-gray-900 mb-3 font-tajawal">
          نوع البحث
        </h3>
        <div className="flex flex-wrap gap-2">
          {searchTypes.map((type) => (
            <button
              key={type.value}
              onClick={() => setSearchType(type.value)}
              className={`px-4 py-2 rounded-lg text-sm font-tajawal transition-colors ${
                localFilters.searchType === type.value
                  ? 'bg-[#1B365D] text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
            >
              {type.label}
            </button>
          ))}
        </div>
      </div>

      {/* Authenticity Grades */}
      <div>
        <h3 className="text-base font-bold text-gray-900 mb-3 font-tajawal">
          درجة الصحة
        </h3>
        <div className="flex flex-wrap gap-2">
          {grades.map((grade) => {
            const isSelected = localFilters.grades?.includes(grade);
            const color = getGradeColor(grade);
            
            return (
              <button
                key={grade}
                onClick={() => toggleGrade(grade)}
                className={`px-4 py-2 rounded-lg text-sm font-tajawal transition-colors border ${
                  isSelected
                    ? 'text-white border-transparent'
                    : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-50'
                }`}
                style={isSelected ? { backgroundColor: color } : {}}
              >
                {getGradeArabicName(grade)}
              </button>
            );
          })}
        </div>
      </div>

      {/* Books */}
      <div>
        <h3 className="text-base font-bold text-gray-900 mb-3 font-tajawal">
          المجموعات
        </h3>
        <div className="flex flex-wrap gap-2">
          {books.map((book) => {
            const isSelected = localFilters.books?.includes(book.name);
            
            return (
              <button
                key={book.id}
                onClick={() => toggleBook(book.name)}
                className={`px-4 py-2 rounded-lg text-sm font-tajawal transition-colors ${
                  isSelected
                    ? 'bg-[#2D5A27] text-white'
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
              >
                {book.arabic_name}
              </button>
            );
          })}
        </div>
      </div>

      {/* Themes */}
      <div>
        <h3 className="text-base font-bold text-gray-900 mb-3 font-tajawal">
          المواضيع
        </h3>
        <div className="flex flex-wrap gap-2">
          {themes.map((theme) => {
            const isSelected = localFilters.themes?.includes(theme);
            
            return (
              <button
                key={theme}
                onClick={() => toggleTheme(theme)}
                className={`px-4 py-2 rounded-lg text-sm font-tajawal transition-colors ${
                  isSelected
                    ? 'bg-[#B8860B] text-white'
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
              >
                {theme}
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
}

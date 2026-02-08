'use client';

/**
 * Smart Search Bar with Voice Search
 * Requirements: 8.1, 8.3
 */

import { useState, useRef, useEffect } from 'react';

interface SearchBarProps {
  value: string;
  onChange: (value: string) => void;
  onSearch: () => void;
  onVoiceSearch: (audioBlob: Blob) => void;
  onFilterClick: () => void;
  hasActiveFilters: boolean;
  isLoading: boolean;
}

export function SearchBar({
  value,
  onChange,
  onSearch,
  onVoiceSearch,
  onFilterClick,
  hasActiveFilters,
  isLoading,
}: SearchBarProps) {
  const [isListening, setIsListening] = useState(false);
  const [speechSupported, setSpeechSupported] = useState(false);
  const recognitionRef = useRef<any>(null);

  useEffect(() => {
    // Check if speech recognition is supported
    if (typeof window !== 'undefined') {
      const SpeechRecognition =
        (window as any).SpeechRecognition ||
        (window as any).webkitSpeechRecognition;
      
      if (SpeechRecognition) {
        setSpeechSupported(true);
        recognitionRef.current = new SpeechRecognition();
        recognitionRef.current.lang = 'ar-SA';
        recognitionRef.current.continuous = false;
        recognitionRef.current.interimResults = false;

        recognitionRef.current.onresult = (event: any) => {
          const transcript = event.results[0][0].transcript;
          onChange(transcript);
          setIsListening(false);
          // Auto-search after voice input
          setTimeout(() => onSearch(), 100);
        };

        recognitionRef.current.onerror = () => {
          setIsListening(false);
        };

        recognitionRef.current.onend = () => {
          setIsListening(false);
        };
      }
    }

    return () => {
      if (recognitionRef.current) {
        recognitionRef.current.stop();
      }
    };
  }, [onChange, onSearch]);

  const handleVoiceClick = () => {
    if (!speechSupported) {
      alert('البحث الصوتي غير مدعوم في هذا المتصفح');
      return;
    }

    if (isListening) {
      recognitionRef.current?.stop();
      setIsListening(false);
    } else {
      recognitionRef.current?.start();
      setIsListening(true);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      onSearch();
    }
  };

  return (
    <div className="relative">
      <div className="flex items-center gap-2 bg-gray-100 rounded-xl border border-gray-200 focus-within:border-[#1B365D] focus-within:ring-2 focus-within:ring-[#1B365D]/20 transition-all">
        {/* Search Icon */}
        <div className="pl-4">
          <svg
            className="w-5 h-5 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
        </div>

        {/* Input */}
        <input
          type="text"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="ابحث في القرآن والحديث والفتاوى..."
          className="flex-1 bg-transparent border-none outline-none py-3 text-gray-900 placeholder-gray-400"
          disabled={isLoading}
        />

        {/* Clear Button */}
        {value && (
          <button
            onClick={() => onChange('')}
            className="p-2 hover:bg-gray-200 rounded-lg transition-colors"
          >
            <svg
              className="w-5 h-5 text-gray-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        )}

        {/* Voice Search Button */}
        {speechSupported && (
          <button
            onClick={handleVoiceClick}
            className={`p-2 rounded-lg transition-colors ${
              isListening
                ? 'bg-red-100 text-red-600'
                : 'hover:bg-gray-200 text-[#1B365D]'
            }`}
            title="البحث الصوتي"
          >
            <svg
              className="w-5 h-5"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
              />
            </svg>
          </button>
        )}

        {/* Filter Button */}
        <button
          onClick={onFilterClick}
          className={`p-2 rounded-lg transition-colors relative ${
            hasActiveFilters
              ? 'bg-[#B8860B]/10 text-[#B8860B]'
              : 'hover:bg-gray-200 text-[#1B365D]'
          }`}
          title="الفلاتر"
        >
          <svg
            className="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"
            />
          </svg>
          {hasActiveFilters && (
            <span className="absolute top-1 right-1 w-2 h-2 bg-[#B8860B] rounded-full" />
          )}
        </button>

        {/* Search Button */}
        <button
          onClick={onSearch}
          disabled={isLoading || !value.trim()}
          className="bg-[#1B365D] text-white px-6 py-3 rounded-l-xl hover:bg-[#2E4A6B] disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-semibold"
        >
          {isLoading ? 'جاري البحث...' : 'بحث'}
        </button>
      </div>

      {/* Listening Indicator */}
      {isListening && (
        <div className="absolute top-full mt-2 left-1/2 transform -translate-x-1/2 bg-red-100 text-red-600 px-4 py-2 rounded-lg shadow-lg flex items-center gap-2">
          <div className="w-2 h-2 bg-red-600 rounded-full animate-pulse" />
          <span className="text-sm font-medium">جاري الاستماع...</span>
        </div>
      )}
    </div>
  );
}

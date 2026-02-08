/**
 * Quran Search Bar Component
 */
'use client';

interface QuranSearchBarProps {
  value: string;
  onChange: (value: string) => void;
}

export function QuranSearchBar({ value, onChange }: QuranSearchBarProps) {
  return (
    <div className="relative max-w-2xl mx-auto mb-6">
      <div className="relative">
        <input
          type="text"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder="ابحث في القرآن الكريم..."
          className="w-full px-12 py-4 bg-white text-gray-800 rounded-xl border-2 border-transparent focus:border-[#B8860B] focus:outline-none shadow-lg transition-all"
          dir="rtl"
        />
        
        {/* Search Icon */}
        <div className="absolute right-4 top-1/2 transform -translate-y-1/2">
          <svg
            className="w-6 h-6 text-[#1B365D]"
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

        {/* Clear Button */}
        {value && (
          <button
            onClick={() => onChange('')}
            className="absolute left-4 top-1/2 transform -translate-y-1/2 p-1 hover:bg-gray-100 rounded-full transition-colors"
          >
            <svg
              className="w-5 h-5 text-gray-500"
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
      </div>
    </div>
  );
}

'use client';

interface EmptyStateProps {
  onSuggestionClick: (suggestion: string) => void;
}

export function EmptyState({ onSuggestionClick }: EmptyStateProps) {
  const suggestions = [
    'ما حكم الصلاة في الطائرة؟',
    'كيف أحسب زكاة المال؟',
    'ما هي أركان الإسلام؟',
    'ما هو فضل قراءة سورة الكهف يوم الجمعة؟',
    'كيف أتوضأ بشكل صحيح؟',
    'ما هي شروط الحج؟',
  ];

  return (
    <div className="flex flex-col items-center justify-center min-h-[60vh] px-4">
      {/* Icon */}
      <div className="mb-8 relative">
        <div className="absolute inset-0 bg-gradient-to-br from-[#1B365D] to-[#2E4A6B] rounded-full blur-2xl opacity-20 animate-pulse" />
        <div className="relative bg-gradient-to-br from-[#1B365D] to-[#2E4A6B] p-8 rounded-full">
          <svg
            className="w-16 h-16 text-white"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"
            />
          </svg>
        </div>
      </div>

      {/* Welcome Text */}
      <h2 className="text-3xl font-bold text-gray-900 mb-3 text-center">
        مرحباً بك في المساعد الإسلامي الذكي
      </h2>
      <p className="text-gray-600 text-center max-w-md mb-8">
        اسأل أي سؤال عن الإسلام وسأجيبك بإذن الله مع المصادر الموثوقة من القرآن
        والسنة
      </p>

      {/* Features */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-12 max-w-3xl w-full">
        <div className="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
          <div className="bg-green-100 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
            <svg
              className="w-6 h-6 text-green-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
          </div>
          <h3 className="font-bold text-gray-900 mb-2">إجابات موثوقة</h3>
          <p className="text-sm text-gray-600">
            جميع الإجابات مدعومة بمصادر من القرآن والسنة
          </p>
        </div>

        <div className="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
          <div className="bg-blue-100 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
            <svg
              className="w-6 h-6 text-blue-600"
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
          </div>
          <h3 className="font-bold text-gray-900 mb-2">إدخال صوتي</h3>
          <p className="text-sm text-gray-600">
            اسأل بصوتك واحصل على إجابات فورية
          </p>
        </div>

        <div className="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
          <div className="bg-purple-100 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
            <svg
              className="w-6 h-6 text-purple-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M13 10V3L4 14h7v7l9-11h-7z"
              />
            </svg>
          </div>
          <h3 className="font-bold text-gray-900 mb-2">إجابات سريعة</h3>
          <p className="text-sm text-gray-600">
            احصل على إجابات فورية مع بث مباشر للنص
          </p>
        </div>
      </div>

      {/* Suggestions */}
      <div className="max-w-3xl w-full">
        <h3 className="text-lg font-bold text-gray-900 mb-4 text-center">
          أسئلة مقترحة
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
          {suggestions.map((suggestion, index) => (
            <button
              key={index}
              onClick={() => onSuggestionClick(suggestion)}
              className="text-right p-4 bg-white hover:bg-gray-50 border border-gray-200 rounded-xl transition-all hover:shadow-md hover:border-[#1B365D] group"
            >
              <div className="flex items-center gap-3">
                <div className="flex-shrink-0 w-8 h-8 bg-gradient-to-br from-[#1B365D] to-[#2E4A6B] rounded-lg flex items-center justify-center group-hover:scale-110 transition-transform">
                  <svg
                    className="w-4 h-4 text-white"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                  </svg>
                </div>
                <span className="text-sm text-gray-700 group-hover:text-[#1B365D] font-medium">
                  {suggestion}
                </span>
              </div>
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}

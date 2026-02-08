'use client';

import { useState } from 'react';
import Link from 'next/link';

interface AIAssistantHeaderProps {
  onClearChat: () => void;
}

export function AIAssistantHeader({ onClearChat }: AIAssistantHeaderProps) {
  const [showClearDialog, setShowClearDialog] = useState(false);

  const handleClearClick = () => {
    setShowClearDialog(true);
  };

  const handleConfirmClear = () => {
    onClearChat();
    setShowClearDialog(false);
  };

  return (
    <>
      <header className="bg-gradient-to-r from-[#1B365D] to-[#2E4A6B] text-white shadow-lg sticky top-0 z-50">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <Link
                href="/dashboard"
                className="p-2 hover:bg-white/10 rounded-lg transition-colors"
              >
                <svg
                  className="w-6 h-6"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M15 19l-7-7 7-7"
                  />
                </svg>
              </Link>

              <div className="flex items-center gap-3">
                <div className="p-3 bg-white/20 rounded-xl">
                  <svg
                    className="w-6 h-6"
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

                <div>
                  <h1 className="text-xl font-bold">المساعد الإسلامي الذكي</h1>
                  <p className="text-sm text-white/80">
                    اسأل عن أي شيء في الإسلام
                  </p>
                </div>
              </div>
            </div>

            <button
              onClick={handleClearClick}
              className="p-2 hover:bg-white/10 rounded-lg transition-colors"
              title="مسح المحادثة"
            >
              <svg
                className="w-6 h-6"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                />
              </svg>
            </button>
          </div>
        </div>
      </header>

      {/* Clear Chat Dialog */}
      {showClearDialog && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-2xl shadow-2xl max-w-md w-full p-6">
            <h3 className="text-xl font-bold text-gray-900 mb-2">
              مسح المحادثة
            </h3>
            <p className="text-gray-600 mb-6">
              هل تريد مسح جميع الرسائل؟ لا يمكن التراجع عن هذا الإجراء.
            </p>

            <div className="flex gap-3">
              <button
                onClick={() => setShowClearDialog(false)}
                className="flex-1 px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
              >
                إلغاء
              </button>
              <button
                onClick={handleConfirmClear}
                className="flex-1 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
              >
                مسح
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}

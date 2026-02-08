'use client';

import { useState } from 'react';
import { AIMessage } from '@/types/ai-assistant';
import { SourceCard } from './SourceCard';
import { TypingIndicator } from './TypingIndicator';

interface MessageBubbleProps {
  message: AIMessage;
}

export function MessageBubble({ message }: MessageBubbleProps) {
  const isUser = message.role === 'user';
  const [showSources, setShowSources] = useState(false);

  const formatTime = (date: Date) => {
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const minutes = Math.floor(diff / 60000);

    if (minutes < 1) return 'الآن';
    if (minutes < 60) return `${minutes} دقيقة`;
    if (minutes < 1440) return `${Math.floor(minutes / 60)} ساعة`;
    return date.toLocaleDateString('ar-SA');
  };

  return (
    <div
      className={`flex gap-3 ${isUser ? 'flex-row-reverse' : 'flex-row'}`}
    >
      {/* Avatar */}
      <div
        className={`flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center ${
          isUser
            ? 'bg-gradient-to-br from-[#1B365D] to-[#2E4A6B]'
            : 'bg-gradient-to-br from-[#B8860B] to-[#DAA520]'
        }`}
      >
        {isUser ? (
          <svg
            className="w-6 h-6 text-white"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
            />
          </svg>
        ) : (
          <svg
            className="w-6 h-6 text-white"
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
        )}
      </div>

      {/* Message Content */}
      <div className={`flex-1 max-w-3xl ${isUser ? 'items-end' : 'items-start'} flex flex-col`}>
        <div
          className={`rounded-2xl px-6 py-4 shadow-sm ${
            isUser
              ? 'bg-gradient-to-br from-[#1B365D] to-[#2E4A6B] text-white rounded-br-sm'
              : 'bg-white text-gray-900 rounded-bl-sm border border-gray-200'
          }`}
        >
          {/* Message Text */}
          <div className="prose prose-sm max-w-none">
            <p className="whitespace-pre-wrap leading-relaxed">
              {message.content}
            </p>
          </div>

          {/* Streaming Indicator */}
          {message.isStreaming && (
            <div className="mt-3">
              <TypingIndicator />
            </div>
          )}

          {/* Error Message */}
          {message.error && (
            <div className="mt-3 flex items-center gap-2 text-red-500 text-sm">
              <svg
                className="w-4 h-4"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              <span>{message.error}</span>
            </div>
          )}
        </div>

        {/* Sources Section */}
        {!isUser && message.sources && message.sources.length > 0 && (
          <div className="mt-4 w-full">
            <button
              onClick={() => setShowSources(!showSources)}
              className="flex items-center gap-2 text-sm font-semibold text-[#B8860B] hover:text-[#DAA520] transition-colors mb-3"
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
                  d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
                />
              </svg>
              <span>المصادر ({message.sources.length})</span>
              <svg
                className={`w-4 h-4 transition-transform ${
                  showSources ? 'rotate-180' : ''
                }`}
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M19 9l-7 7-7-7"
                />
              </svg>
            </button>

            {showSources && (
              <div className="space-y-3">
                {message.sources.map((source) => (
                  <SourceCard key={source.id} source={source} />
                ))}
              </div>
            )}
          </div>
        )}

        {/* Timestamp */}
        <div className="mt-2 text-xs text-gray-500">
          {formatTime(message.timestamp)}
        </div>
      </div>
    </div>
  );
}

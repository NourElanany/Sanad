'use client';

import { useEffect, useRef, useState } from 'react';
import { AIMessage } from '@/types/ai-assistant';
import { MessageBubble } from './MessageBubble';
import { ChatInput } from './ChatInput';
import { EmptyState } from './EmptyState';

interface AIAssistantChatProps {
  messages: AIMessage[];
  isStreaming: boolean;
  onSendMessage: (message: string) => void;
}

export function AIAssistantChat({
  messages,
  isStreaming,
  onSendMessage,
}: AIAssistantChatProps) {
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const [isAtBottom, setIsAtBottom] = useState(true);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    if (isAtBottom) {
      scrollToBottom();
    }
  }, [messages, isAtBottom]);

  const handleScroll = (e: React.UIEvent<HTMLDivElement>) => {
    const element = e.currentTarget;
    const isBottom =
      element.scrollHeight - element.scrollTop === element.clientHeight;
    setIsAtBottom(isBottom);
  };

  return (
    <div className="flex flex-col h-[calc(100vh-80px)]">
      {/* Messages Area */}
      <div
        className="flex-1 overflow-y-auto px-4 py-6"
        onScroll={handleScroll}
      >
        <div className="container mx-auto max-w-4xl">
          {messages.length === 0 ? (
            <EmptyState onSuggestionClick={onSendMessage} />
          ) : (
            <div className="space-y-6">
              {messages.map((message) => (
                <MessageBubble key={message.id} message={message} />
              ))}
              <div ref={messagesEndRef} />
            </div>
          )}
        </div>
      </div>

      {/* Input Area */}
      <div className="border-t border-gray-200 bg-white">
        <div className="container mx-auto max-w-4xl px-4 py-4">
          <ChatInput
            onSendMessage={onSendMessage}
            disabled={isStreaming}
          />
        </div>
      </div>

      {/* Scroll to Bottom Button */}
      {!isAtBottom && (
        <button
          onClick={scrollToBottom}
          className="fixed bottom-24 left-1/2 transform -translate-x-1/2 bg-white shadow-lg rounded-full p-3 hover:bg-gray-50 transition-colors border border-gray-200"
        >
          <svg
            className="w-6 h-6 text-gray-600"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M19 14l-7 7m0 0l-7-7m7 7V3"
            />
          </svg>
        </button>
      )}
    </div>
  );
}

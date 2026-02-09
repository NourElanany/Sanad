'use client';

import { useState } from 'react';
import { AIAssistantChat } from '@/components/ai-assistant/AIAssistantChat';
import { AIAssistantHeader } from '@/components/ai-assistant/AIAssistantHeader';
import { AIMessage, Source } from '@/types/ai-assistant';

export default function AIAssistantClient() {
  const [messages, setMessages] = useState<AIMessage[]>([]);
  const [isStreaming, setIsStreaming] = useState(false);
  const [sessionId] = useState(() => `session-${Date.now()}`);

  const handleSendMessage = async (content: string) => {
    // Add user message
    const userMessage: AIMessage = {
      id: `msg-${Date.now()}`,
      role: 'user',
      content,
      timestamp: new Date(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setIsStreaming(true);

    try {
      // Call API with streaming
      const response = await fetch('/api/ai-assistant/chat', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          message: content,
          session_id: sessionId,
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to send message');
      }

      // Handle streaming response
      const reader = response.body?.getReader();
      const decoder = new TextDecoder();
      let accumulatedContent = '';
      let currentMessageId = `msg-${Date.now()}-ai`;
      let sources: Source[] = [];

      if (reader) {
        while (true) {
          const { done, value } = await reader.read();
          if (done) break;

          const chunk = decoder.decode(value);
          const lines = chunk.split('\n');

          for (const line of lines) {
            if (!line.trim() || !line.startsWith('data: ')) continue;

            const data = line.substring(6);
            if (data === '[DONE]') continue;

            try {
              const json = JSON.parse(data);

              if (json.type === 'content') {
                accumulatedContent += json.content;

                // Update or add streaming message
                setMessages((prev) => {
                  const existingIndex = prev.findIndex(
                    (m) => m.id === currentMessageId
                  );

                  const streamingMessage: AIMessage = {
                    id: currentMessageId,
                    role: 'assistant',
                    content: accumulatedContent,
                    timestamp: new Date(),
                    isStreaming: true,
                  };

                  if (existingIndex >= 0) {
                    const newMessages = [...prev];
                    newMessages[existingIndex] = streamingMessage;
                    return newMessages;
                  } else {
                    return [...prev, streamingMessage];
                  }
                });
              } else if (json.type === 'sources') {
                sources = json.sources;

                // Update message with sources
                setMessages((prev) => {
                  const newMessages = [...prev];
                  const index = newMessages.findIndex(
                    (m) => m.id === currentMessageId
                  );

                  if (index >= 0) {
                    newMessages[index] = {
                      ...newMessages[index],
                      sources,
                      isStreaming: false,
                    };
                  }

                  return newMessages;
                });
              }
            } catch (e) {
              console.error('Error parsing SSE data:', e);
            }
          }
        }
      }
    } catch (error) {
      console.error('Error sending message:', error);

      // Add error message
      const errorMessage: AIMessage = {
        id: `msg-${Date.now()}-error`,
        role: 'assistant',
        content: 'عذراً، حدث خطأ أثناء معالجة رسالتك. يرجى المحاولة مرة أخرى.',
        timestamp: new Date(),
        error: error instanceof Error ? error.message : 'Unknown error',
      };

      setMessages((prev) => [...prev, errorMessage]);
    } finally {
      setIsStreaming(false);
    }
  };

  const handleClearChat = () => {
    setMessages([]);
  };

  return (
    <div className="min-h-screen bg-gray-50" dir="rtl">
      <AIAssistantHeader onClearChat={handleClearChat} />
      <AIAssistantChat
        messages={messages}
        isStreaming={isStreaming}
        onSendMessage={handleSendMessage}
      />
    </div>
  );
}

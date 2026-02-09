import { apiClient } from '../api/axios-client';

/**
 * AI Assistant Service for Web Application
 * Provides streaming responses, session management, and source verification
 * 
 * Requirements: 7.1, 7.2, 7.3, 7.4, 7.5
 */

// ============================================================================
// Types and Interfaces
// ============================================================================

export enum MessageRole {
  USER = 'user',
  ASSISTANT = 'assistant',
  SYSTEM = 'system',
}

export enum MessageStatus {
  SENDING = 'sending',
  SENT = 'sent',
  STREAMING = 'streaming',
  ERROR = 'error',
}

export interface SourceModel {
  id: string;
  title: string;
  type: 'quran' | 'hadith' | 'fatwa' | 'tafsir';
  reference: string;
  excerpt?: string;
  url?: string;
  confidence?: number;
}

export interface AIMessage {
  id: string;
  content: string;
  role: MessageRole;
  timestamp: Date;
  sources?: SourceModel[];
  status: MessageStatus;
  error?: string;
}

export interface ChatSession {
  id: string;
  createdAt: Date;
  updatedAt: Date;
  messages: AIMessage[];
}

export interface StreamChunk {
  type: 'content' | 'sources' | 'error' | 'done';
  content?: string;
  sources?: SourceModel[];
  error?: string;
}

// ============================================================================
// AI Assistant Service Class
// ============================================================================

class AIAssistantService {
  private readonly MAX_RETRIES = 3;
  private readonly RETRY_DELAY = 1000; // 1 second
  private activeSessions: Map<string, AbortController> = new Map();

  /**
   * Send a message and receive streaming response using Server-Sent Events
   * Requirement 7.5: Stream responses in real-time
   */
  async *sendMessageStream(
    message: string,
    sessionId: string
  ): AsyncGenerator<AIMessage, void, unknown> {
    const messageId = Date.now().toString();
    let accumulatedContent = '';
    let retryCount = 0;

    while (retryCount < this.MAX_RETRIES) {
      try {
        // Create abort controller for this session
        const abortController = new AbortController();
        this.activeSessions.set(sessionId, abortController);

        const response = await fetch(
          `${process.env.NEXT_PUBLIC_API_BASE_URL || 'https://api.sanad.app'}/api/ai-assistant/chat`,
          {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
              'Accept': 'text/event-stream',
              'Authorization': `Bearer ${this.getAccessToken()}`,
            },
            body: JSON.stringify({
              message,
              session_id: sessionId,
            }),
            signal: abortController.signal,
          }
        );

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        if (!response.body) {
          throw new Error('Response body is null');
        }

        // Process Server-Sent Events stream
        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        let buffer = '';

        while (true) {
          const { done, value } = await reader.read();

          if (done) {
            break;
          }

          buffer += decoder.decode(value, { stream: true });
          const lines = buffer.split('\n');
          buffer = lines.pop() || '';

          for (const line of lines) {
            const trimmedLine = line.trim();

            if (!trimmedLine || !trimmedLine.startsWith('data: ')) {
              continue;
            }

            const data = trimmedLine.substring(6); // Remove 'data: ' prefix

            if (data === '[DONE]') {
              // Stream complete
              this.activeSessions.delete(sessionId);
              return;
            }

            try {
              const chunk: StreamChunk = JSON.parse(data);

              if (chunk.type === 'content' && chunk.content) {
                accumulatedContent += chunk.content;

                yield {
                  id: messageId,
                  content: accumulatedContent,
                  role: MessageRole.ASSISTANT,
                  timestamp: new Date(),
                  status: MessageStatus.STREAMING,
                };
              } else if (chunk.type === 'sources' && chunk.sources) {
                // Final message with sources
                yield {
                  id: messageId,
                  content: accumulatedContent,
                  role: MessageRole.ASSISTANT,
                  timestamp: new Date(),
                  sources: chunk.sources,
                  status: MessageStatus.SENT,
                };
              } else if (chunk.type === 'error') {
                throw new Error(chunk.error || 'Unknown error occurred');
              }
            } catch (parseError) {
              console.error('Error parsing SSE data:', parseError);
            }
          }
        }

        // Clean up
        this.activeSessions.delete(sessionId);
        return;
      } catch (error: any) {
        retryCount++;

        if (error.name === 'AbortError') {
          // Request was cancelled
          yield {
            id: messageId,
            content: '',
            role: MessageRole.ASSISTANT,
            timestamp: new Date(),
            status: MessageStatus.ERROR,
            error: 'Request cancelled',
          };
          return;
        }

        if (retryCount >= this.MAX_RETRIES) {
          // Max retries reached
          yield {
            id: messageId,
            content: '',
            role: MessageRole.ASSISTANT,
            timestamp: new Date(),
            status: MessageStatus.ERROR,
            error: `Failed after ${this.MAX_RETRIES} attempts: ${error.message}`,
          };
          return;
        }

        // Wait before retry
        await this.delay(this.RETRY_DELAY * retryCount);
      }
    }
  }

  /**
   * Send a message and get complete response (non-streaming)
   * Requirement 7.1: Provide ChatGPT-like interface
   */
  async sendMessage(message: string, sessionId: string): Promise<AIMessage> {
    try {
      const response = await apiClient.post<{
        id: string;
        content: string;
        role: string;
        timestamp: string;
        sources?: SourceModel[];
      }>('/api/ai-assistant/chat', {
        message,
        session_id: sessionId,
        stream: false,
      });

      return {
        id: response.id,
        content: response.content,
        role: response.role as MessageRole,
        timestamp: new Date(response.timestamp),
        sources: response.sources,
        status: MessageStatus.SENT,
      };
    } catch (error: any) {
      throw new Error(`Failed to send message: ${error.message}`);
    }
  }

  /**
   * Get conversation history for a session
   * Requirement 7.1: Session management
   */
  async getHistory(sessionId: string): Promise<AIMessage[]> {
    try {
      const response = await apiClient.get<Array<{
        id: string;
        content: string;
        role: string;
        timestamp: string;
        sources?: SourceModel[];
      }>>(`/api/ai-assistant/chat/history/${sessionId}`);

      return response.map((msg) => ({
        id: msg.id,
        content: msg.content,
        role: msg.role as MessageRole,
        timestamp: new Date(msg.timestamp),
        sources: msg.sources,
        status: MessageStatus.SENT,
      }));
    } catch (error: any) {
      throw new Error(`Failed to get history: ${error.message}`);
    }
  }

  /**
   * Clear conversation for a session
   * Requirement 7.1: Session management
   */
  async clearConversation(sessionId: string): Promise<void> {
    try {
      await apiClient.delete(`/api/ai-assistant/chat/clear/${sessionId}`);
    } catch (error: any) {
      throw new Error(`Failed to clear conversation: ${error.message}`);
    }
  }

  /**
   * Get sources for verification
   * Requirement 7.4: Provide source verification links
   */
  async getSources(query: string): Promise<SourceModel[]> {
    try {
      const response = await apiClient.post<SourceModel[]>('/api/ai-assistant/sources', {
        query,
      });

      return response;
    } catch (error: any) {
      throw new Error(`Failed to get sources: ${error.message}`);
    }
  }

  /**
   * Verify a specific source
   * Requirement 7.4: Source verification
   */
  async verifySource(sourceId: string): Promise<{
    verified: boolean;
    details: string;
    confidence: number;
  }> {
    try {
      const response = await apiClient.get<{
        verified: boolean;
        details: string;
        confidence: number;
      }>(`/api/ai-assistant/sources/verify/${sourceId}`);

      return response;
    } catch (error: any) {
      throw new Error(`Failed to verify source: ${error.message}`);
    }
  }

  /**
   * Convert speech to text
   * Requirement 7.2: Support voice and text input
   */
  async speechToText(audioBlob: Blob): Promise<string> {
    try {
      const formData = new FormData();
      formData.append('audio', audioBlob, 'recording.wav');

      const response = await apiClient.post<{ text: string }>(
        '/api/ai-assistant/speech-to-text',
        formData,
        {
          headers: {
            'Content-Type': 'multipart/form-data',
          },
        }
      );

      return response.text;
    } catch (error: any) {
      throw new Error(`Failed to convert speech to text: ${error.message}`);
    }
  }

  /**
   * Create a new chat session
   * Requirement 7.1: Session management
   */
  async createSession(): Promise<string> {
    try {
      const response = await apiClient.post<{ session_id: string }>(
        '/api/ai-assistant/chat/session'
      );

      return response.session_id;
    } catch (error: any) {
      throw new Error(`Failed to create session: ${error.message}`);
    }
  }

  /**
   * Get all sessions for current user
   * Requirement 7.1: Session management
   */
  async getSessions(): Promise<ChatSession[]> {
    try {
      const response = await apiClient.get<Array<{
        id: string;
        created_at: string;
        updated_at: string;
        messages: Array<{
          id: string;
          content: string;
          role: string;
          timestamp: string;
          sources?: SourceModel[];
        }>;
      }>>('/api/ai-assistant/chat/sessions');

      return response.map((session) => ({
        id: session.id,
        createdAt: new Date(session.created_at),
        updatedAt: new Date(session.updated_at),
        messages: session.messages.map((msg) => ({
          id: msg.id,
          content: msg.content,
          role: msg.role as MessageRole,
          timestamp: new Date(msg.timestamp),
          sources: msg.sources,
          status: MessageStatus.SENT,
        })),
      }));
    } catch (error: any) {
      throw new Error(`Failed to get sessions: ${error.message}`);
    }
  }

  /**
   * Delete a session
   * Requirement 7.1: Session management
   */
  async deleteSession(sessionId: string): Promise<void> {
    try {
      // Cancel any active streaming for this session
      this.cancelStream(sessionId);

      await apiClient.delete(`/api/ai-assistant/chat/session/${sessionId}`);
    } catch (error: any) {
      throw new Error(`Failed to delete session: ${error.message}`);
    }
  }

  /**
   * Cancel an active streaming session
   */
  cancelStream(sessionId: string): void {
    const controller = this.activeSessions.get(sessionId);
    if (controller) {
      controller.abort();
      this.activeSessions.delete(sessionId);
    }
  }

  /**
   * Get citation details for a source
   * Requirement 7.3: Include citation cards with sources
   */
  async getCitationDetails(sourceId: string): Promise<{
    fullText: string;
    context: string;
    metadata: Record<string, any>;
  }> {
    try {
      const response = await apiClient.get<{
        full_text: string;
        context: string;
        metadata: Record<string, any>;
      }>(`/api/ai-assistant/sources/citation/${sourceId}`);

      return {
        fullText: response.full_text,
        context: response.context,
        metadata: response.metadata,
      };
    } catch (error: any) {
      throw new Error(`Failed to get citation details: ${error.message}`);
    }
  }

  /**
   * Report an issue with an AI response
   * Requirement 7.4: Source verification and quality control
   */
  async reportIssue(
    messageId: string,
    issueType: 'incorrect' | 'misleading' | 'inappropriate' | 'other',
    description: string
  ): Promise<void> {
    try {
      await apiClient.post('/api/ai-assistant/report', {
        message_id: messageId,
        issue_type: issueType,
        description,
      });
    } catch (error: any) {
      throw new Error(`Failed to report issue: ${error.message}`);
    }
  }

  /**
   * Get access token from auth service
   */
  private getAccessToken(): string {
    // This should integrate with your auth service
    if (typeof window !== 'undefined') {
      return localStorage.getItem('access_token') || '';
    }
    return '';
  }

  /**
   * Delay helper for retry logic
   */
  private delay(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  /**
   * Check if a session has an active stream
   */
  hasActiveStream(sessionId: string): boolean {
    return this.activeSessions.has(sessionId);
  }

  /**
   * Get the number of active streams
   */
  getActiveStreamCount(): number {
    return this.activeSessions.size;
  }
}

// Export singleton instance
export const aiAssistantService = new AIAssistantService();

// Export types
export type { AIMessage, ChatSession, SourceModel, StreamChunk };

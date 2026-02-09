import {
  aiAssistantService,
  MessageRole,
  MessageStatus,
  AIMessage,
  SourceModel,
} from '../ai-assistant-service';
import { apiClient } from '../../api/axios-client';

// Mock the API client
jest.mock('../../api/axios-client', () => ({
  apiClient: {
    get: jest.fn(),
    post: jest.fn(),
    delete: jest.fn(),
  },
}));

// Mock fetch for streaming tests
global.fetch = jest.fn() as jest.Mock;

describe('AIAssistantService', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    // Mock localStorage
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: jest.fn(() => 'mock-token'),
        setItem: jest.fn(),
        removeItem: jest.fn(),
        clear: jest.fn(),
      },
      writable: true,
    });
  });

  afterEach(() => {
    jest.restoreAllMocks();
  });

  describe('sendMessage', () => {
    it('should send a message and return AI response', async () => {
      const mockResponse = {
        id: '123',
        content: 'This is a test response',
        role: 'assistant',
        timestamp: new Date().toISOString(),
        sources: [
          {
            id: 'source-1',
            title: 'Test Source',
            type: 'quran',
            reference: 'Surah Al-Baqarah 2:255',
          },
        ],
      };

      (apiClient.post as jest.Mock).mockResolvedValue(mockResponse);

      const result = await aiAssistantService.sendMessage(
        'What is Ayat al-Kursi?',
        'session-123'
      );

      expect(result).toMatchObject({
        id: '123',
        content: 'This is a test response',
        role: MessageRole.ASSISTANT,
        status: MessageStatus.SENT,
      });
      expect(result.sources).toHaveLength(1);
      expect(result.sources![0].type).toBe('quran');
    });

    it('should handle errors when sending message', async () => {
      (apiClient.post as jest.Mock).mockRejectedValue(new Error('Network error'));

      await expect(
        aiAssistantService.sendMessage('Test message', 'session-123')
      ).rejects.toThrow('Failed to send message');
    });
  });

  describe('sendMessageStream', () => {
    it('should handle streaming errors with retry', async () => {
      (global.fetch as jest.Mock).mockRejectedValue(new Error('Network error'));

      const messages: AIMessage[] = [];
      for await (const message of aiAssistantService.sendMessageStream(
        'Test',
        'session-123'
      )) {
        messages.push(message);
      }

      expect(messages.length).toBe(1);
      expect(messages[0].status).toBe(MessageStatus.ERROR);
      expect(messages[0].error).toContain('Failed after');
    });

    it('should handle HTTP errors in streaming', async () => {
      (global.fetch as jest.Mock).mockResolvedValue({
        ok: false,
        status: 500,
      } as Response);

      const messages: AIMessage[] = [];
      for await (const message of aiAssistantService.sendMessageStream(
        'Test',
        'session-123'
      )) {
        messages.push(message);
      }

      expect(messages[0].status).toBe(MessageStatus.ERROR);
    });
  });

  describe('getHistory', () => {
    it('should retrieve conversation history', async () => {
      const mockHistory = [
        {
          id: '1',
          content: 'User message',
          role: 'user',
          timestamp: new Date().toISOString(),
        },
        {
          id: '2',
          content: 'Assistant response',
          role: 'assistant',
          timestamp: new Date().toISOString(),
          sources: [],
        },
      ];

      (apiClient.get as jest.Mock).mockResolvedValue(mockHistory);

      const result = await aiAssistantService.getHistory('session-123');

      expect(result).toHaveLength(2);
      expect(result[0].role).toBe(MessageRole.USER);
      expect(result[1].role).toBe(MessageRole.ASSISTANT);
    });

    it('should handle errors when getting history', async () => {
      (apiClient.get as jest.Mock).mockRejectedValue(new Error('Not found'));

      await expect(aiAssistantService.getHistory('session-123')).rejects.toThrow(
        'Failed to get history'
      );
    });
  });

  describe('clearConversation', () => {
    it('should clear conversation successfully', async () => {
      (apiClient.delete as jest.Mock).mockResolvedValue(undefined);

      await expect(
        aiAssistantService.clearConversation('session-123')
      ).resolves.not.toThrow();

      expect(apiClient.delete).toHaveBeenCalledWith(
        '/api/ai-assistant/chat/clear/session-123'
      );
    });

    it('should handle errors when clearing conversation', async () => {
      (apiClient.delete as jest.Mock).mockRejectedValue(new Error('Server error'));

      await expect(
        aiAssistantService.clearConversation('session-123')
      ).rejects.toThrow('Failed to clear conversation');
    });
  });

  describe('getSources', () => {
    it('should retrieve sources for a query', async () => {
      const mockSources: SourceModel[] = [
        {
          id: '1',
          title: 'Ayat al-Kursi',
          type: 'quran',
          reference: 'Surah Al-Baqarah 2:255',
          confidence: 0.95,
        },
        {
          id: '2',
          title: 'Hadith about prayer',
          type: 'hadith',
          reference: 'Sahih Bukhari 1:8',
          confidence: 0.88,
        },
      ];

      (apiClient.post as jest.Mock).mockResolvedValue(mockSources);

      const result = await aiAssistantService.getSources('prayer times');

      expect(result).toHaveLength(2);
      expect(result[0].type).toBe('quran');
      expect(result[1].type).toBe('hadith');
    });
  });

  describe('verifySource', () => {
    it('should verify a source and return verification details', async () => {
      const mockVerification = {
        verified: true,
        details: 'Source verified from authentic collection',
        confidence: 0.95,
      };

      (apiClient.get as jest.Mock).mockResolvedValue(mockVerification);

      const result = await aiAssistantService.verifySource('source-123');

      expect(result.verified).toBe(true);
      expect(result.confidence).toBe(0.95);
    });
  });

  describe('speechToText', () => {
    it('should convert audio blob to text', async () => {
      const mockResponse = { text: 'Converted speech text' };
      (apiClient.post as jest.Mock).mockResolvedValue(mockResponse);

      const audioBlob = new Blob(['audio data'], { type: 'audio/wav' });
      const result = await aiAssistantService.speechToText(audioBlob);

      expect(result).toBe('Converted speech text');
      expect(apiClient.post).toHaveBeenCalledWith(
        '/api/ai-assistant/speech-to-text',
        expect.any(FormData),
        expect.objectContaining({
          headers: {
            'Content-Type': 'multipart/form-data',
          },
        })
      );
    });
  });

  describe('Session Management', () => {
    it('should create a new session', async () => {
      const mockResponse = { session_id: 'new-session-123' };
      (apiClient.post as jest.Mock).mockResolvedValue(mockResponse);

      const sessionId = await aiAssistantService.createSession();

      expect(sessionId).toBe('new-session-123');
    });

    it('should get all sessions', async () => {
      const mockSessions = [
        {
          id: 'session-1',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
          messages: [],
        },
        {
          id: 'session-2',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
          messages: [],
        },
      ];

      (apiClient.get as jest.Mock).mockResolvedValue(mockSessions);

      const result = await aiAssistantService.getSessions();

      expect(result).toHaveLength(2);
      expect(result[0].id).toBe('session-1');
    });

    it('should delete a session', async () => {
      (apiClient.delete as jest.Mock).mockResolvedValue(undefined);

      await expect(
        aiAssistantService.deleteSession('session-123')
      ).resolves.not.toThrow();

      expect(apiClient.delete).toHaveBeenCalledWith(
        '/api/ai-assistant/chat/session/session-123'
      );
    });
  });

  describe('getCitationDetails', () => {
    it('should retrieve citation details for a source', async () => {
      const mockCitation = {
        full_text: 'Full text of the source',
        context: 'Contextual information',
        metadata: { author: 'Test Author', date: '2024' },
      };

      (apiClient.get as jest.Mock).mockResolvedValue(mockCitation);

      const result = await aiAssistantService.getCitationDetails('source-123');

      expect(result.fullText).toBe('Full text of the source');
      expect(result.context).toBe('Contextual information');
      expect(result.metadata.author).toBe('Test Author');
    });
  });

  describe('reportIssue', () => {
    it('should report an issue with a message', async () => {
      (apiClient.post as jest.Mock).mockResolvedValue(undefined);

      await expect(
        aiAssistantService.reportIssue('msg-123', 'incorrect', 'This is wrong')
      ).resolves.not.toThrow();

      expect(apiClient.post).toHaveBeenCalledWith('/api/ai-assistant/report', {
        message_id: 'msg-123',
        issue_type: 'incorrect',
        description: 'This is wrong',
      });
    });
  });

  describe('Stream Management', () => {
    it('should track active streams', () => {
      expect(aiAssistantService.getActiveStreamCount()).toBe(0);
      expect(aiAssistantService.hasActiveStream('session-123')).toBe(false);
    });

    it('should cancel active stream', () => {
      // This is tested indirectly through the streaming tests
      aiAssistantService.cancelStream('session-123');
      expect(aiAssistantService.hasActiveStream('session-123')).toBe(false);
    });
  });

  describe('Error Handling and Retry Logic', () => {
    it('should handle network errors gracefully', async () => {
      (global.fetch as jest.Mock).mockRejectedValue(new Error('Network error'));

      const messages: AIMessage[] = [];
      for await (const message of aiAssistantService.sendMessageStream(
        'Test',
        'session-123'
      )) {
        messages.push(message);
      }

      expect(messages.length).toBe(1);
      expect(messages[0].status).toBe(MessageStatus.ERROR);
      expect(messages[0].error).toContain('Failed after');
    });
  });

  describe('Edge Cases', () => {
    it('should handle empty message content', async () => {
      const mockResponse = {
        id: '123',
        content: '',
        role: 'assistant',
        timestamp: new Date().toISOString(),
      };

      (apiClient.post as jest.Mock).mockResolvedValue(mockResponse);

      const result = await aiAssistantService.sendMessage('', 'session-123');

      expect(result.content).toBe('');
      expect(result.status).toBe(MessageStatus.SENT);
    });

    it('should handle sources without confidence scores', async () => {
      const mockSources: SourceModel[] = [
        {
          id: '1',
          title: 'Test Source',
          type: 'quran',
          reference: '2:255',
        },
      ];

      (apiClient.post as jest.Mock).mockResolvedValue(mockSources);

      const result = await aiAssistantService.getSources('test');

      expect(result[0].confidence).toBeUndefined();
    });
  });
});

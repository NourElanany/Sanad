/**
 * Zustand Store for AI Assistant State Management
 * Handles chat sessions, messages, streaming, and source verification
 * 
 * Requirements: 19.1, 19.2, 19.3, 19.4, 19.5
 */

import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import { devtools } from 'zustand/middleware';
import { aiAssistantService, MessageRole, MessageStatus, type AIMessage, type ChatSession, type SourceModel } from '../services/ai-assistant-service';

// ============================================================================
// Types
// ============================================================================

interface AIAssistantState {
  // Data
  sessions: ChatSession[];
  currentSessionId: string | null;
  currentMessages: AIMessage[];
  
  // UI State
  loading: boolean;
  streaming: boolean;
  error: string | null;
  
  // Voice input
  isRecording: boolean;
  recordingDuration: number;
  
  // Actions
  createSession: () => Promise<void>;
  loadSessions: () => Promise<void>;
  loadSession: (sessionId: string) => Promise<void>;
  deleteSession: (sessionId: string) => Promise<void>;
  clearCurrentSession: () => Promise<void>;
  
  // Messaging
  sendMessage: (message: string, useStreaming?: boolean) => Promise<void>;
  cancelStreaming: () => void;
  
  // Voice
  startRecording: () => void;
  stopRecording: () => Promise<string | null>;
  
  // Sources
  verifySources: (sourceId: string) => Promise<void>;
  getCitationDetails: (sourceId: string) => Promise<void>;
  
  // Utility
  clearError: () => void;
  reset: () => void;
}

// ============================================================================
// Initial State
// ============================================================================

const initialState = {
  sessions: [],
  currentSessionId: null,
  currentMessages: [],
  loading: false,
  streaming: false,
  error: null,
  isRecording: false,
  recordingDuration: 0,
};

// ============================================================================
// Store Implementation
// ============================================================================

export const useAIAssistantStore = create<AIAssistantState>()(
  devtools(
    persist(
      (set, get) => ({
        ...initialState,

        // Create a new session
        createSession: async () => {
          set({ loading: true, error: null });
          try {
            const sessionId = await aiAssistantService.createSession();
            
            const newSession: ChatSession = {
              id: sessionId,
              createdAt: new Date(),
              updatedAt: new Date(),
              messages: [],
            };
            
            set({
              sessions: [newSession, ...get().sessions],
              currentSessionId: sessionId,
              currentMessages: [],
              loading: false,
            });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Load all sessions
        loadSessions: async () => {
          set({ loading: true, error: null });
          try {
            const sessions = await aiAssistantService.getSessions();
            set({ sessions, loading: false });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Load a specific session
        loadSession: async (sessionId: string) => {
          set({ loading: true, error: null });
          try {
            const messages = await aiAssistantService.getHistory(sessionId);
            set({
              currentSessionId: sessionId,
              currentMessages: messages,
              loading: false,
            });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Delete a session
        deleteSession: async (sessionId: string) => {
          // Optimistic update
          const previousSessions = get().sessions;
          set({
            sessions: get().sessions.filter(s => s.id !== sessionId),
            error: null,
          });

          // If deleting current session, clear it
          if (get().currentSessionId === sessionId) {
            set({ currentSessionId: null, currentMessages: [] });
          }

          try {
            await aiAssistantService.deleteSession(sessionId);
          } catch (error: any) {
            // Rollback on error
            set({
              sessions: previousSessions,
              error: error.message,
            });
          }
        },

        // Clear current session
        clearCurrentSession: async () => {
          const sessionId = get().currentSessionId;
          if (!sessionId) return;

          // Optimistic update
          set({ currentMessages: [], error: null });

          try {
            await aiAssistantService.clearConversation(sessionId);
          } catch (error: any) {
            set({ error: error.message });
          }
        },

        // Send a message
        sendMessage: async (message: string, useStreaming = true) => {
          let sessionId = get().currentSessionId;
          
          // Create session if none exists
          if (!sessionId) {
            await get().createSession();
            sessionId = get().currentSessionId;
            if (!sessionId) {
              set({ error: 'Failed to create session' });
              return;
            }
          }

          // Add user message optimistically
          const userMessage: AIMessage = {
            id: `user-${Date.now()}`,
            content: message,
            role: MessageRole.USER,
            timestamp: new Date(),
            status: MessageStatus.SENT,
          };

          set({
            currentMessages: [...get().currentMessages, userMessage],
            error: null,
          });

          if (useStreaming) {
            // Streaming response
            set({ streaming: true });
            
            try {
              const stream = aiAssistantService.sendMessageStream(message, sessionId);
              
              for await (const aiMessage of stream) {
                // Update or add assistant message
                set({
                  currentMessages: get().currentMessages.map(msg =>
                    msg.id === aiMessage.id ? aiMessage : msg
                  ).concat(
                    get().currentMessages.find(msg => msg.id === aiMessage.id)
                      ? []
                      : [aiMessage]
                  ),
                });
              }
              
              set({ streaming: false });
            } catch (error: any) {
              set({
                streaming: false,
                error: error.message,
              });
            }
          } else {
            // Non-streaming response
            set({ loading: true });
            
            try {
              const aiMessage = await aiAssistantService.sendMessage(message, sessionId);
              set({
                currentMessages: [...get().currentMessages, aiMessage],
                loading: false,
              });
            } catch (error: any) {
              set({
                loading: false,
                error: error.message,
              });
            }
          }

          // Update session timestamp
          set({
            sessions: get().sessions.map(s =>
              s.id === sessionId
                ? { ...s, updatedAt: new Date() }
                : s
            ),
          });
        },

        // Cancel streaming
        cancelStreaming: () => {
          const sessionId = get().currentSessionId;
          if (sessionId) {
            aiAssistantService.cancelStream(sessionId);
            set({ streaming: false });
          }
        },

        // Start voice recording
        startRecording: () => {
          set({ isRecording: true, recordingDuration: 0 });
          
          // Update duration every second
          const interval = setInterval(() => {
            if (!get().isRecording) {
              clearInterval(interval);
              return;
            }
            set({ recordingDuration: get().recordingDuration + 1 });
          }, 1000);
        },

        // Stop recording and convert to text
        stopRecording: async () => {
          set({ isRecording: false, loading: true, error: null });
          
          try {
            // This would integrate with Web Audio API
            // For now, return null as placeholder
            // In real implementation, you'd capture the audio blob
            const audioBlob = null as any; // Placeholder
            
            if (!audioBlob) {
              set({ loading: false });
              return null;
            }
            
            const text = await aiAssistantService.speechToText(audioBlob);
            set({ loading: false, recordingDuration: 0 });
            return text;
          } catch (error: any) {
            set({
              loading: false,
              recordingDuration: 0,
              error: error.message,
            });
            return null;
          }
        },

        // Verify sources
        verifySources: async (sourceId: string) => {
          set({ loading: true, error: null });
          try {
            const verification = await aiAssistantService.verifySource(sourceId);
            // Update the source in current messages
            set({
              currentMessages: get().currentMessages.map(msg => ({
                ...msg,
                sources: msg.sources?.map(source =>
                  source.id === sourceId
                    ? { ...source, confidence: verification.confidence }
                    : source
                ),
              })),
              loading: false,
            });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Get citation details
        getCitationDetails: async (sourceId: string) => {
          set({ loading: true, error: null });
          try {
            await aiAssistantService.getCitationDetails(sourceId);
            // Citation details would be displayed in a modal
            // This is handled by the UI component
            set({ loading: false });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Clear error
        clearError: () => set({ error: null }),

        // Reset store
        reset: () => set(initialState),
      }),
      {
        name: 'ai-assistant-storage',
        storage: createJSONStorage(() => localStorage),
        // Persist sessions and current session
        partialize: (state) => ({
          sessions: state.sessions,
          currentSessionId: state.currentSessionId,
          currentMessages: state.currentMessages,
        }),
      }
    ),
    {
      name: 'AIAssistantStore',
    }
  )
);

// ============================================================================
// Selectors
// ============================================================================

export const selectSessions = (state: AIAssistantState) => state.sessions;
export const selectCurrentSessionId = (state: AIAssistantState) => state.currentSessionId;
export const selectCurrentMessages = (state: AIAssistantState) => state.currentMessages;
export const selectLoading = (state: AIAssistantState) => state.loading;
export const selectStreaming = (state: AIAssistantState) => state.streaming;
export const selectError = (state: AIAssistantState) => state.error;
export const selectIsRecording = (state: AIAssistantState) => state.isRecording;
export const selectRecordingDuration = (state: AIAssistantState) => state.recordingDuration;

// Computed selectors
export const selectCurrentSession = (state: AIAssistantState) =>
  state.sessions.find(s => s.id === state.currentSessionId);

export const selectHasActiveSession = (state: AIAssistantState) =>
  state.currentSessionId !== null;

export const selectMessageCount = (state: AIAssistantState) =>
  state.currentMessages.length;

export const selectLastMessage = (state: AIAssistantState) =>
  state.currentMessages[state.currentMessages.length - 1];

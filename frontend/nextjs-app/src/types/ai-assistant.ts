export interface AIMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: Date;
  sources?: Source[];
  isStreaming?: boolean;
  error?: string;
}

export interface Source {
  id: string;
  title: string;
  type: 'quran' | 'hadith' | 'fatwa' | 'tafsir';
  reference: string;
  excerpt?: string;
  url?: string;
  confidence?: number;
}

export interface ChatSession {
  id: string;
  createdAt: Date;
  updatedAt: Date;
  messages: AIMessage[];
}

export interface VoiceRecording {
  blob: Blob;
  duration: number;
  url: string;
}

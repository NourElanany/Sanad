export enum AudioQuality {
  LOW = 'low',
  MEDIUM = 'medium',
  HIGH = 'high',
  ULTRA = 'ultra',
}

export const AudioQualitySettings = {
  [AudioQuality.LOW]: { sampleRate: 16000, bitRate: 32000, label: 'منخفضة' },
  [AudioQuality.MEDIUM]: { sampleRate: 22050, bitRate: 64000, label: 'متوسطة' },
  [AudioQuality.HIGH]: { sampleRate: 44100, bitRate: 128000, label: 'عالية' },
  [AudioQuality.ULTRA]: { sampleRate: 48000, bitRate: 192000, label: 'فائقة' },
};

export enum RecordingState {
  IDLE = 'idle',
  PREPARING = 'preparing',
  RECORDING = 'recording',
  PAUSED = 'paused',
  STOPPED = 'stopped',
  PROCESSING = 'processing',
  ERROR = 'error',
}

export interface WaveformData {
  amplitudes: number[];
  maxAmplitude: number;
  minAmplitude: number;
  duration: number; // in milliseconds
}

export interface RecordingMetadata {
  recordingId: string;
  surahNumber: number;
  ayahStart: number;
  ayahEnd: number;
  quality: AudioQuality;
  recordedAt: Date;
  duration: number; // in milliseconds
  fileSizeBytes: number;
  blob?: Blob;
}

export interface VerseSelection {
  surahNumber: number;
  surahName: string;
  ayahStart: number;
  ayahEnd: number;
  arabicText: string;
}

export interface RecitationAnalysis {
  recordingId: string;
  overallScore: number;
  detailedScores: DetailedScores;
  errors: TajweedError[];
  recommendations: Recommendation[];
  analyzedAt: Date;
}

export interface DetailedScores {
  pronunciationAccuracy: number;
  timingAccuracy: number;
  tajweedCompliance: number;
  fluency: number;
  clarity: number;
  rhythm: number;
}

export interface TajweedError {
  errorType: string;
  description: string;
  timestamp: number;
  severity: 'high' | 'medium' | 'low';
  correction?: string;
}

export interface Recommendation {
  category: 'pronunciation' | 'timing' | 'tajweed' | 'fluency' | 'general';
  priority: 'high' | 'medium' | 'low';
  description: string;
  specificAdvice: string;
  practiceExercises: string[];
}

export interface Surah {
  number: number;
  name: string;
  ayahCount: number;
  revelationType: 'meccan' | 'medinan';
}

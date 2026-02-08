import {
  AudioQuality,
  AudioQualitySettings,
  RecordingState,
  WaveformData,
  RecordingMetadata,
} from '@/types/recording';

export class RecordingService {
  private mediaRecorder: MediaRecorder | null = null;
  private audioContext: AudioContext | null = null;
  private analyser: AnalyserNode | null = null;
  private dataArray: Uint8Array | null = null;
  private stream: MediaStream | null = null;
  private chunks: Blob[] = [];
  private startTime: number = 0;
  private animationFrameId: number | null = null;

  private stateListeners: Set<(state: RecordingState)> = new Set();
  private waveformListeners: Set<(data: WaveformData)> = new Set();
  private durationListeners: Set<(duration: number)> = new Set();

  private currentState: RecordingState = RecordingState.IDLE;
  private amplitudes: number[] = [];

  constructor() {
    if (typeof window !== 'undefined') {
      this.audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
    }
  }

  // Event listeners
  onStateChange(listener: (state: RecordingState) => void): () => void {
    this.stateListeners.add(listener);
    return () => this.stateListeners.delete(listener);
  }

  onWaveformUpdate(listener: (data: WaveformData) => void): () => void {
    this.waveformListeners.add(listener);
    return () => this.waveformListeners.delete(listener);
  }

  onDurationUpdate(listener: (duration: number) => void): () => void {
    this.durationListeners.add(listener);
    return () => this.durationListeners.delete(listener);
  }

  private updateState(state: RecordingState): void {
    this.currentState = state;
    this.stateListeners.forEach(listener => listener(state));
  }

  private updateWaveform(): void {
    if (!this.analyser || !this.dataArray) return;

    this.analyser.getByteTimeDomainData(this.dataArray);

    // Convert to normalized amplitudes
    const amplitudes = Array.from(this.dataArray).map(
      value => Math.abs((value - 128) / 128)
    );

    // Store amplitudes
    this.amplitudes.push(...amplitudes);

    // Keep only recent amplitudes (last 1000 samples)
    if (this.amplitudes.length > 1000) {
      this.amplitudes = this.amplitudes.slice(-1000);
    }

    const maxAmplitude = Math.max(...this.amplitudes);
    const minAmplitude = Math.min(...this.amplitudes);
    const duration = Date.now() - this.startTime;

    const waveformData: WaveformData = {
      amplitudes: this.amplitudes,
      maxAmplitude,
      minAmplitude,
      duration,
    };

    this.waveformListeners.forEach(listener => listener(waveformData));
  }

  private updateDuration(): void {
    if (this.currentState === RecordingState.RECORDING) {
      const duration = Date.now() - this.startTime;
      this.durationListeners.forEach(listener => listener(duration));
    }
  }

  private startWaveformAnalysis(): void {
    const analyze = () => {
      if (this.currentState === RecordingState.RECORDING) {
        this.updateWaveform();
        this.updateDuration();
        this.animationFrameId = requestAnimationFrame(analyze);
      }
    };
    analyze();
  }

  private stopWaveformAnalysis(): void {
    if (this.animationFrameId !== null) {
      cancelAnimationFrame(this.animationFrameId);
      this.animationFrameId = null;
    }
  }

  async checkPermission(): Promise<boolean> {
    try {
      const result = await navigator.permissions.query({ name: 'microphone' as PermissionName });
      return result.state === 'granted';
    } catch {
      // Fallback: try to get user media
      try {
        const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
        stream.getTracks().forEach(track => track.stop());
        return true;
      } catch {
        return false;
      }
    }
  }

  async startRecording(quality: AudioQuality = AudioQuality.HIGH): Promise<void> {
    if (this.currentState === RecordingState.RECORDING) {
      throw new Error('Already recording');
    }

    this.updateState(RecordingState.PREPARING);

    try {
      // Get user media
      this.stream = await navigator.mediaDevices.getUserMedia({
        audio: {
          sampleRate: AudioQualitySettings[quality].sampleRate,
          channelCount: 1,
          echoCancellation: true,
          noiseSuppression: true,
          autoGainControl: true,
        },
      });

      // Setup audio context and analyser
      if (!this.audioContext) {
        this.audioContext = new AudioContext();
      }

      const source = this.audioContext.createMediaStreamSource(this.stream);
      this.analyser = this.audioContext.createAnalyser();
      this.analyser.fftSize = 2048;
      this.analyser.smoothingTimeConstant = 0.8;

      source.connect(this.analyser);

      const bufferLength = this.analyser.frequencyBinCount;
      this.dataArray = new Uint8Array(bufferLength);

      // Setup media recorder
      const mimeType = this.getSupportedMimeType();
      this.mediaRecorder = new MediaRecorder(this.stream, {
        mimeType,
        audioBitsPerSecond: AudioQualitySettings[quality].bitRate,
      });

      this.chunks = [];
      this.amplitudes = [];

      this.mediaRecorder.ondataavailable = (event) => {
        if (event.data.size > 0) {
          this.chunks.push(event.data);
        }
      };

      this.mediaRecorder.onstop = () => {
        this.stopWaveformAnalysis();
      };

      // Start recording
      this.mediaRecorder.start(100); // Collect data every 100ms
      this.startTime = Date.now();
      this.updateState(RecordingState.RECORDING);

      // Start waveform analysis
      this.startWaveformAnalysis();
    } catch (error) {
      this.updateState(RecordingState.ERROR);
      throw error;
    }
  }

  async pauseRecording(): Promise<void> {
    if (this.currentState !== RecordingState.RECORDING || !this.mediaRecorder) {
      return;
    }

    this.mediaRecorder.pause();
    this.stopWaveformAnalysis();
    this.updateState(RecordingState.PAUSED);
  }

  async resumeRecording(): Promise<void> {
    if (this.currentState !== RecordingState.PAUSED || !this.mediaRecorder) {
      return;
    }

    this.mediaRecorder.resume();
    this.startWaveformAnalysis();
    this.updateState(RecordingState.RECORDING);
  }

  async stopRecording(metadata: {
    surahNumber: number;
    ayahStart: number;
    ayahEnd: number;
    quality: AudioQuality;
  }): Promise<RecordingMetadata> {
    if (
      (this.currentState !== RecordingState.RECORDING &&
        this.currentState !== RecordingState.PAUSED) ||
      !this.mediaRecorder
    ) {
      throw new Error('Not recording');
    }

    this.updateState(RecordingState.PROCESSING);

    return new Promise((resolve, reject) => {
      if (!this.mediaRecorder) {
        reject(new Error('MediaRecorder not initialized'));
        return;
      }

      this.mediaRecorder.onstop = () => {
        const blob = new Blob(this.chunks, { type: this.getSupportedMimeType() });
        const duration = Date.now() - this.startTime;

        const recordingMetadata: RecordingMetadata = {
          recordingId: this.generateId(),
          surahNumber: metadata.surahNumber,
          ayahStart: metadata.ayahStart,
          ayahEnd: metadata.ayahEnd,
          quality: metadata.quality,
          recordedAt: new Date(),
          duration,
          fileSizeBytes: blob.size,
          blob,
        };

        this.cleanup();
        this.updateState(RecordingState.STOPPED);
        resolve(recordingMetadata);
      };

      this.mediaRecorder.stop();
    });
  }

  async cancelRecording(): Promise<void> {
    if (
      this.currentState === RecordingState.RECORDING ||
      this.currentState === RecordingState.PAUSED
    ) {
      if (this.mediaRecorder && this.mediaRecorder.state !== 'inactive') {
        this.mediaRecorder.stop();
      }
      this.cleanup();
      this.updateState(RecordingState.IDLE);
    }
  }

  private cleanup(): void {
    this.stopWaveformAnalysis();

    if (this.stream) {
      this.stream.getTracks().forEach(track => track.stop());
      this.stream = null;
    }

    this.mediaRecorder = null;
    this.analyser = null;
    this.dataArray = null;
    this.chunks = [];
    this.amplitudes = [];
  }

  private getSupportedMimeType(): string {
    const types = [
      'audio/webm;codecs=opus',
      'audio/webm',
      'audio/ogg;codecs=opus',
      'audio/ogg',
      'audio/wav',
    ];

    for (const type of types) {
      if (MediaRecorder.isTypeSupported(type)) {
        return type;
      }
    }

    return 'audio/webm'; // Fallback
  }

  private generateId(): string {
    return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  dispose(): void {
    this.cleanup();
    if (this.audioContext) {
      this.audioContext.close();
      this.audioContext = null;
    }
    this.stateListeners.clear();
    this.waveformListeners.clear();
    this.durationListeners.clear();
  }
}

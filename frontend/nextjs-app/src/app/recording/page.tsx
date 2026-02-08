'use client';

import React, { useState, useEffect, useRef } from 'react';
import { WaveformVisualizer, AnimatedWaveformBars } from '@/components/recording/WaveformVisualizer';
import {
  RecordingControls,
  RecordingDuration,
  AudioQualitySelector,
} from '@/components/recording/RecordingControls';
import { VerseSelector } from '@/components/recording/VerseSelector';
import { RecordingService } from '@/lib/services/recording-service';
import {
  RecordingState,
  AudioQuality,
  WaveformData,
  VerseSelection,
  RecordingMetadata,
} from '@/types/recording';
import { HelpCircle, AlertCircle } from 'lucide-react';

export default function RecordingPage() {
  const [recordingState, setRecordingState] = useState<RecordingState>(RecordingState.IDLE);
  const [waveformData, setWaveformData] = useState<WaveformData | undefined>();
  const [duration, setDuration] = useState<number>(0);
  const [selectedVerse, setSelectedVerse] = useState<VerseSelection | null>(null);
  const [audioQuality, setAudioQuality] = useState<AudioQuality>(AudioQuality.HIGH);
  const [recordingMetadata, setRecordingMetadata] = useState<RecordingMetadata | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [showHelp, setShowHelp] = useState(false);

  const recordingServiceRef = useRef<RecordingService | null>(null);

  useEffect(() => {
    // Initialize recording service
    recordingServiceRef.current = new RecordingService();

    const service = recordingServiceRef.current;

    // Setup listeners
    const unsubscribeState = service.onStateChange(setRecordingState);
    const unsubscribeWaveform = service.onWaveformUpdate(setWaveformData);
    const unsubscribeDuration = service.onDurationUpdate(setDuration);

    return () => {
      unsubscribeState();
      unsubscribeWaveform();
      unsubscribeDuration();
      service.dispose();
    };
  }, []);

  const handleStartRecording = async () => {
    if (!selectedVerse) {
      setError('الرجاء اختيار الآيات أولاً');
      return;
    }

    try {
      setError(null);
      await recordingServiceRef.current?.startRecording(audioQuality);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'فشل بدء التسجيل');
    }
  };

  const handlePauseRecording = async () => {
    try {
      await recordingServiceRef.current?.pauseRecording();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'فشل إيقاف التسجيل مؤقتاً');
    }
  };

  const handleResumeRecording = async () => {
    try {
      await recordingServiceRef.current?.resumeRecording();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'فشل استئناف التسجيل');
    }
  };

  const handleStopRecording = async () => {
    if (!selectedVerse) return;

    try {
      const metadata = await recordingServiceRef.current?.stopRecording({
        surahNumber: selectedVerse.surahNumber,
        ayahStart: selectedVerse.ayahStart,
        ayahEnd: selectedVerse.ayahEnd,
        quality: audioQuality,
      });

      if (metadata) {
        setRecordingMetadata(metadata);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'فشل إيقاف التسجيل');
    }
  };

  const handleCancelRecording = async () => {
    if (window.confirm('هل أنت متأكد من إلغاء التسجيل؟ سيتم حذف التسجيل الحالي.')) {
      try {
        await recordingServiceRef.current?.cancelRecording();
        setRecordingMetadata(null);
        setError(null);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'فشل إلغاء التسجيل');
      }
    }
  };

  const handleAnalyzeRecording = () => {
    // Navigate to analysis page or trigger analysis
    console.log('Analyzing recording:', recordingMetadata);
    // TODO: Implement analysis integration
  };

  return (
    <div className="min-h-screen bg-cream">
      {/* Header */}
      <header className="bg-navy text-white shadow-lg">
        <div className="container mx-auto px-4 py-6">
          <div className="flex items-center justify-between">
            <h1 className="text-2xl font-bold font-tajawal">تسجيل التلاوة</h1>
            <button
              onClick={() => setShowHelp(true)}
              className="p-2 hover:bg-white/10 rounded-lg transition-colors"
              title="مساعدة"
            >
              <HelpCircle className="w-6 h-6" />
            </button>
          </div>
        </div>
      </header>

      {/* Main content */}
      <main className="container mx-auto px-4 py-8 max-w-4xl">
        <div className="space-y-6">
          {/* Verse selector */}
          <VerseSelector
            initialSelection={selectedVerse ?? undefined}
            onSelectionChanged={setSelectedVerse}
          />

          {/* Audio quality selector */}
          {(recordingState === RecordingState.IDLE ||
            recordingState === RecordingState.STOPPED) && (
            <div className="bg-white rounded-2xl shadow-md p-6">
              <AudioQualitySelector
                selectedQuality={audioQuality}
                onQualityChanged={setAudioQuality}
                enabled={!!selectedVerse}
              />
            </div>
          )}

          {/* Waveform visualizer */}
          <div className="bg-white rounded-2xl shadow-md p-6 space-y-4">
            <WaveformVisualizer waveformData={waveformData} height={120} />

            {/* Recording indicator */}
            <div className="flex justify-center">
              <AnimatedWaveformBars
                isRecording={recordingState === RecordingState.RECORDING}
                height={40}
              />
            </div>

            {/* Duration display */}
            <div className="flex justify-center">
              <RecordingDuration
                duration={duration}
                maxDuration={5 * 60 * 1000} // 5 minutes
              />
            </div>
          </div>

          {/* Recording controls */}
          <div className="bg-white rounded-2xl shadow-md p-6">
            <RecordingControls
              state={recordingState}
              onRecord={handleStartRecording}
              onPause={handlePauseRecording}
              onResume={handleResumeRecording}
              onStop={handleStopRecording}
              onCancel={handleCancelRecording}
            />
          </div>

          {/* Error message */}
          {error && (
            <div className="bg-red-50 border border-red-200 rounded-xl p-4 flex items-start gap-3">
              <AlertCircle className="w-5 h-5 text-red-600 flex-shrink-0 mt-0.5" />
              <p className="text-red-800 font-tajawal">{error}</p>
            </div>
          )}

          {/* Analyze button */}
          {recordingMetadata && recordingState === RecordingState.STOPPED && (
            <button
              onClick={handleAnalyzeRecording}
              className="w-full bg-green-600 hover:bg-green-700 text-white font-bold py-4 px-6
                         rounded-xl shadow-lg transition-all duration-200 hover:scale-105
                         active:scale-95 font-tajawal text-lg"
            >
              تحليل التلاوة
            </button>
          )}
        </div>
      </main>

      {/* Help modal */}
      {showHelp && (
        <div
          className="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50"
          onClick={() => setShowHelp(false)}
        >
          <div
            className="bg-white rounded-2xl shadow-2xl max-w-md w-full p-6 space-y-4"
            onClick={(e) => e.stopPropagation()}
          >
            <h2 className="text-xl font-bold text-navy font-tajawal">
              كيفية استخدام مصحح التلاوة
            </h2>
            <div className="space-y-3 text-gray-700 font-tajawal">
              <p>1. اختر السورة والآيات التي تريد تسجيلها</p>
              <p>2. اختر جودة التسجيل المناسبة</p>
              <p>3. اضغط على زر التسجيل وابدأ القراءة</p>
              <p>4. راقب الموجات الصوتية أثناء التسجيل</p>
              <p>5. اضغط على زر الإيقاف عند الانتهاء</p>
              <p>6. اضغط على "تحليل التلاوة" للحصول على التقييم</p>
            </div>
            <button
              onClick={() => setShowHelp(false)}
              className="w-full bg-navy hover:bg-navy-dark text-white font-bold py-3 px-6
                         rounded-xl transition-colors font-tajawal"
            >
              فهمت
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

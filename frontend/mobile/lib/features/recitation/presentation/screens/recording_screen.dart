import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/providers/recording_provider.dart';
import '../../data/models/recording_models.dart';
import '../widgets/waveform_visualizer.dart';
import '../widgets/recording_controls.dart';
import '../widgets/verse_selector.dart';

/// Main recording screen for Quran recitation
class RecordingScreen extends ConsumerStatefulWidget {
  const RecordingScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<RecordingScreen> createState() => _RecordingScreenState();
}

class _RecordingScreenState extends ConsumerState<RecordingScreen> {
  @override
  void initState() {
    super.initState();
    // Initialize recording service
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(recordingControllerProvider.notifier).initialize();
    });
  }

  @override
  Widget build(BuildContext context) {
    final recordingState = ref.watch(recordingStateProvider);
    final waveformData = ref.watch(waveformDataProvider);
    final duration = ref.watch(recordingDurationProvider);
    final selectedVerse = ref.watch(selectedVerseProvider);
    final audioQuality = ref.watch(audioQualityProvider);
    final controllerState = ref.watch(recordingControllerProvider);

    return Scaffold(
      backgroundColor: const Color(0xFFFEFEFE),
      appBar: AppBar(
        title: const Text(
          'تسجيل التلاوة',
          style: TextStyle(
            fontFamily: 'Tajawal',
            fontWeight: FontWeight.bold,
          ),
        ),
        backgroundColor: const Color(0xFF1B365D),
        foregroundColor: Colors.white,
        elevation: 0,
        actions: [
          IconButton(
            icon: const Icon(Icons.help_outline),
            onPressed: () => _showHelpDialog(context),
            tooltip: 'مساعدة',
          ),
        ],
      ),
      body: SafeArea(
        child: SingleChildScrollView(
          padding: const EdgeInsets.all(20),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Verse selector
              VerseSelector(
                initialSelection: selectedVerse,
                onSelectionChanged: (selection) {
                  ref.read(selectedVerseProvider.notifier).state = selection;
                },
              ),
              const SizedBox(height: 24),

              // Audio quality selector (only when idle)
              if (recordingState.value == RecordingState.idle ||
                  recordingState.value == RecordingState.stopped)
                AudioQualitySelector(
                  selectedQuality: audioQuality,
                  onQualityChanged: (quality) {
                    ref.read(audioQualityProvider.notifier).state = quality;
                  },
                  enabled: selectedVerse != null,
                ),
              const SizedBox(height: 24),

              // Waveform visualizer
              waveformData.when(
                data: (data) => WaveformVisualizer(waveformData: data),
                loading: () => const WaveformVisualizer(),
                error: (_, __) => const WaveformVisualizer(),
              ),
              const SizedBox(height: 16),

              // Recording indicator
              recordingState.when(
                data: (state) => AnimatedWaveformBars(
                  isRecording: state == RecordingState.recording,
                ),
                loading: () => const SizedBox(height: 40),
                error: (_, __) => const SizedBox(height: 40),
              ),
              const SizedBox(height: 16),

              // Duration display
              duration.when(
                data: (dur) => Center(
                  child: RecordingDuration(
                    duration: dur,
                    maxDuration: const Duration(minutes: 5),
                  ),
                ),
                loading: () => const Center(
                  child: RecordingDuration(duration: Duration.zero),
                ),
                error: (_, __) => const Center(
                  child: RecordingDuration(duration: Duration.zero),
                ),
              ),
              const SizedBox(height: 32),

              // Recording controls
              recordingState.when(
                data: (state) => RecordingControls(
                  state: state,
                  onRecord: selectedVerse != null
                      ? () => _startRecording(selectedVerse, audioQuality)
                      : () {},
                  onPause: _pauseRecording,
                  onResume: _resumeRecording,
                  onStop: selectedVerse != null
                      ? () => _stopRecording(selectedVerse)
                      : () {},
                  onCancel: _cancelRecording,
                ),
                loading: () => const Center(
                  child: CircularProgressIndicator(),
                ),
                error: (error, _) => Center(
                  child: Text(
                    'خطأ: $error',
                    style: const TextStyle(color: Colors.red),
                  ),
                ),
              ),
              const SizedBox(height: 24),

              // Error message
              if (controllerState.error != null)
                Container(
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: Colors.red.withOpacity(0.1),
                    borderRadius: BorderRadius.circular(12),
                    border: Border.all(
                      color: Colors.red.withOpacity(0.3),
                      width: 1,
                    ),
                  ),
                  child: Row(
                    children: [
                      const Icon(Icons.error, color: Colors.red),
                      const SizedBox(width: 12),
                      Expanded(
                        child: Text(
                          controllerState.error!,
                          style: const TextStyle(
                            color: Colors.red,
                            fontFamily: 'Tajawal',
                          ),
                        ),
                      ),
                    ],
                  ),
                ),

              // Analyze button (when recording is stopped)
              if (controllerState.currentRecording != null &&
                  !controllerState.isAnalyzing)
                ElevatedButton.icon(
                  onPressed: () => _analyzeRecording(
                    controllerState.currentRecording!.recordingId,
                  ),
                  icon: const Icon(Icons.analytics),
                  label: const Text(
                    'تحليل التلاوة',
                    style: TextStyle(
                      fontFamily: 'Tajawal',
                      fontSize: 16,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: const Color(0xFF2D5A27),
                    foregroundColor: Colors.white,
                    padding: const EdgeInsets.symmetric(vertical: 16),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                  ),
                ),

              // Analyzing indicator
              if (controllerState.isAnalyzing)
                const Center(
                  child: Column(
                    children: [
                      CircularProgressIndicator(),
                      SizedBox(height: 16),
                      Text(
                        'جاري تحليل التلاوة...',
                        style: TextStyle(
                          fontFamily: 'Tajawal',
                          fontSize: 16,
                          color: Color(0xFF666666),
                        ),
                      ),
                    ],
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }

  void _startRecording(VerseSelection verse, AudioQuality quality) {
    ref.read(recordingControllerProvider.notifier).startRecording(
          surahNumber: verse.surahNumber,
          ayahStart: verse.ayahStart,
          ayahEnd: verse.ayahEnd,
          quality: quality,
        );
  }

  void _pauseRecording() {
    ref.read(recordingControllerProvider.notifier).pauseRecording();
  }

  void _resumeRecording() {
    ref.read(recordingControllerProvider.notifier).resumeRecording();
  }

  void _stopRecording(VerseSelection verse) {
    ref.read(recordingControllerProvider.notifier).stopRecording(
          surahNumber: verse.surahNumber,
          ayahStart: verse.ayahStart,
          ayahEnd: verse.ayahEnd,
        );
  }

  void _cancelRecording() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text(
          'إلغاء التسجيل',
          style: TextStyle(fontFamily: 'Tajawal'),
        ),
        content: const Text(
          'هل أنت متأكد من إلغاء التسجيل؟ سيتم حذف التسجيل الحالي.',
          style: TextStyle(fontFamily: 'Tajawal'),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text(
              'رجوع',
              style: TextStyle(fontFamily: 'Tajawal'),
            ),
          ),
          TextButton(
            onPressed: () {
              Navigator.pop(context);
              ref.read(recordingControllerProvider.notifier).cancelRecording();
            },
            child: const Text(
              'إلغاء التسجيل',
              style: TextStyle(
                fontFamily: 'Tajawal',
                color: Colors.red,
              ),
            ),
          ),
        ],
      ),
    );
  }

  void _analyzeRecording(String recordingId) {
    ref.read(recordingControllerProvider.notifier).analyzeRecording(recordingId);

    // Navigate to analysis results when complete
    // This would be handled by listening to the analysis state
  }

  void _showHelpDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text(
          'كيفية استخدام مصحح التلاوة',
          style: TextStyle(
            fontFamily: 'Tajawal',
            fontWeight: FontWeight.bold,
          ),
        ),
        content: const SingleChildScrollView(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                '1. اختر السورة والآيات التي تريد تسجيلها',
                style: TextStyle(fontFamily: 'Tajawal'),
              ),
              SizedBox(height: 8),
              Text(
                '2. اختر جودة التسجيل المناسبة',
                style: TextStyle(fontFamily: 'Tajawal'),
              ),
              SizedBox(height: 8),
              Text(
                '3. اضغط على زر التسجيل وابدأ القراءة',
                style: TextStyle(fontFamily: 'Tajawal'),
              ),
              SizedBox(height: 8),
              Text(
                '4. راقب الموجات الصوتية أثناء التسجيل',
                style: TextStyle(fontFamily: 'Tajawal'),
              ),
              SizedBox(height: 8),
              Text(
                '5. اضغط على زر الإيقاف عند الانتهاء',
                style: TextStyle(fontFamily: 'Tajawal'),
              ),
              SizedBox(height: 8),
              Text(
                '6. اضغط على "تحليل التلاوة" للحصول على التقييم',
                style: TextStyle(fontFamily: 'Tajawal'),
              ),
            ],
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text(
              'فهمت',
              style: TextStyle(fontFamily: 'Tajawal'),
            ),
          ),
        ],
      ),
    );
  }
}

import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../services/recording_service.dart';
import '../services/api_service.dart';
import '../../features/recitation/data/models/recording_models.dart';

/// Recording service provider
final recordingServiceProvider = Provider<RecordingService>((ref) {
  final service = RecordingService();
  ref.onDispose(() => service.dispose());
  return service;
});

/// Recording state provider
final recordingStateProvider = StreamProvider<RecordingState>((ref) {
  final service = ref.watch(recordingServiceProvider);
  return service.stateStream;
});

/// Waveform data provider
final waveformDataProvider = StreamProvider<WaveformData>((ref) {
  final service = ref.watch(recordingServiceProvider);
  return service.waveformStream;
});

/// Recording duration provider
final recordingDurationProvider = StreamProvider<Duration>((ref) {
  final service = ref.watch(recordingServiceProvider);
  return service.durationStream;
});

/// Selected verse provider
final selectedVerseProvider =
    StateProvider<VerseSelection?>((ref) => null);

/// Audio quality provider
final audioQualityProvider =
    StateProvider<AudioQuality>((ref) => AudioQuality.high);

/// Recording controller provider
final recordingControllerProvider =
    StateNotifierProvider<RecordingController, RecordingControllerState>(
  (ref) => RecordingController(
    ref.watch(recordingServiceProvider),
    ref.watch(apiServiceProvider),
  ),
);

/// Recording controller state
class RecordingControllerState {
  final bool isInitialized;
  final RecordingMetadata? currentRecording;
  final RecitationAnalysis? analysis;
  final bool isAnalyzing;
  final String? error;

  RecordingControllerState({
    this.isInitialized = false,
    this.currentRecording,
    this.analysis,
    this.isAnalyzing = false,
    this.error,
  });

  RecordingControllerState copyWith({
    bool? isInitialized,
    RecordingMetadata? currentRecording,
    RecitationAnalysis? analysis,
    bool? isAnalyzing,
    String? error,
  }) {
    return RecordingControllerState(
      isInitialized: isInitialized ?? this.isInitialized,
      currentRecording: currentRecording ?? this.currentRecording,
      analysis: analysis ?? this.analysis,
      isAnalyzing: isAnalyzing ?? this.isAnalyzing,
      error: error ?? this.error,
    );
  }
}

/// Recording controller
class RecordingController extends StateNotifier<RecordingControllerState> {
  final RecordingService _recordingService;
  final ApiService _apiService;

  RecordingController(this._recordingService, this._apiService)
      : super(RecordingControllerState());

  /// Initialize recording service
  Future<void> initialize() async {
    try {
      await _recordingService.initialize();
      state = state.copyWith(isInitialized: true, error: null);
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  /// Start recording
  Future<void> startRecording({
    required int surahNumber,
    required int ayahStart,
    required int ayahEnd,
    AudioQuality quality = AudioQuality.high,
  }) async {
    try {
      await _recordingService.startRecording(
        surahNumber: surahNumber,
        ayahStart: ayahStart,
        ayahEnd: ayahEnd,
        quality: quality,
      );
      state = state.copyWith(error: null);
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  /// Pause recording
  Future<void> pauseRecording() async {
    try {
      await _recordingService.pauseRecording();
      state = state.copyWith(error: null);
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  /// Resume recording
  Future<void> resumeRecording() async {
    try {
      await _recordingService.resumeRecording();
      state = state.copyWith(error: null);
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  /// Stop recording
  Future<void> stopRecording({
    required int surahNumber,
    required int ayahStart,
    required int ayahEnd,
  }) async {
    try {
      final metadata = await _recordingService.stopRecording(
        surahNumber: surahNumber,
        ayahStart: ayahStart,
        ayahEnd: ayahEnd,
      );
      state = state.copyWith(currentRecording: metadata, error: null);
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  /// Cancel recording
  Future<void> cancelRecording() async {
    try {
      await _recordingService.cancelRecording();
      state = state.copyWith(
        currentRecording: null,
        analysis: null,
        error: null,
      );
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  /// Analyze recording
  Future<void> analyzeRecording(String recordingId) async {
    try {
      state = state.copyWith(isAnalyzing: true, error: null);

      // Upload recording to backend
      final analysis = await _apiService.analyzeRecitation(
        recordingId: recordingId,
        filePath: state.currentRecording!.filePath!,
        surahNumber: state.currentRecording!.surahNumber,
        ayahStart: state.currentRecording!.ayahStart,
        ayahEnd: state.currentRecording!.ayahEnd,
      );

      state = state.copyWith(
        analysis: analysis,
        isAnalyzing: false,
        error: null,
      );
    } catch (e) {
      state = state.copyWith(
        isAnalyzing: false,
        error: e.toString(),
      );
    }
  }

  /// Clear analysis
  void clearAnalysis() {
    state = state.copyWith(
      analysis: null,
      currentRecording: null,
      error: null,
    );
  }
}

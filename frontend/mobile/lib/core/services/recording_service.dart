import 'dart:async';
import 'dart:io';
import 'dart:typed_data';
import 'package:flutter_sound/flutter_sound.dart';
import 'package:path_provider/path_provider.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:uuid/uuid.dart';
import '../features/recitation/data/models/recording_models.dart';

/// Service for audio recording with waveform visualization
class RecordingService {
  final FlutterSoundRecorder _recorder = FlutterSoundRecorder();
  final StreamController<WaveformData> _waveformController =
      StreamController<WaveformData>.broadcast();
  final StreamController<RecordingState> _stateController =
      StreamController<RecordingState>.broadcast();
  final StreamController<Duration> _durationController =
      StreamController<Duration>.broadcast();

  RecordingState _currentState = RecordingState.idle;
  String? _currentRecordingPath;
  DateTime? _recordingStartTime;
  Timer? _durationTimer;
  final List<double> _amplitudes = [];
  AudioQuality _currentQuality = AudioQuality.high;

  // Getters
  Stream<WaveformData> get waveformStream => _waveformController.stream;
  Stream<RecordingState> get stateStream => _stateController.stream;
  Stream<Duration> get durationStream => _durationController.stream;
  RecordingState get currentState => _currentState;
  String? get currentRecordingPath => _currentRecordingPath;

  /// Initialize the recording service
  Future<void> initialize() async {
    await _recorder.openRecorder();
    await _recorder.setSubscriptionDuration(const Duration(milliseconds: 100));
  }

  /// Check and request microphone permission
  Future<bool> checkPermission() async {
    final status = await Permission.microphone.status;
    if (status.isGranted) {
      return true;
    }

    final result = await Permission.microphone.request();
    return result.isGranted;
  }

  /// Start recording with specified quality
  Future<void> startRecording({
    required int surahNumber,
    required int ayahStart,
    required int ayahEnd,
    AudioQuality quality = AudioQuality.high,
  }) async {
    if (_currentState == RecordingState.recording) {
      throw Exception('Already recording');
    }

    // Check permission
    final hasPermission = await checkPermission();
    if (!hasPermission) {
      _updateState(RecordingState.error);
      throw Exception('Microphone permission not granted');
    }

    _updateState(RecordingState.preparing);
    _currentQuality = quality;
    _amplitudes.clear();

    // Generate file path
    final directory = await getApplicationDocumentsDirectory();
    final recordingId = const Uuid().v4();
    _currentRecordingPath =
        '${directory.path}/recordings/$recordingId.wav';

    // Create directory if it doesn't exist
    final recordingDir = Directory('${directory.path}/recordings');
    if (!await recordingDir.exists()) {
      await recordingDir.create(recursive: true);
    }

    // Start recording
    await _recorder.startRecorder(
      toFile: _currentRecordingPath,
      codec: Codec.pcm16WAV,
      sampleRate: quality.sampleRate,
      numChannels: 1,
    );

    _recordingStartTime = DateTime.now();
    _updateState(RecordingState.recording);

    // Start duration timer
    _startDurationTimer();

    // Listen to amplitude for waveform
    _recorder.onProgress!.listen((event) {
      if (event.decibels != null) {
        final amplitude = _decibelToAmplitude(event.decibels!);
        _amplitudes.add(amplitude);

        // Update waveform every 100ms
        if (_amplitudes.length % 10 == 0) {
          _updateWaveform();
        }
      }
    });
  }

  /// Pause recording
  Future<void> pauseRecording() async {
    if (_currentState != RecordingState.recording) {
      return;
    }

    await _recorder.pauseRecorder();
    _updateState(RecordingState.paused);
    _durationTimer?.cancel();
  }

  /// Resume recording
  Future<void> resumeRecording() async {
    if (_currentState != RecordingState.paused) {
      return;
    }

    await _recorder.resumeRecorder();
    _updateState(RecordingState.recording);
    _startDurationTimer();
  }

  /// Stop recording and return metadata
  Future<RecordingMetadata> stopRecording({
    required int surahNumber,
    required int ayahStart,
    required int ayahEnd,
  }) async {
    if (_currentState != RecordingState.recording &&
        _currentState != RecordingState.paused) {
      throw Exception('Not recording');
    }

    _updateState(RecordingState.processing);
    _durationTimer?.cancel();

    await _recorder.stopRecorder();

    final duration = DateTime.now().difference(_recordingStartTime!);
    final file = File(_currentRecordingPath!);
    final fileSize = await file.length();

    final metadata = RecordingMetadata(
      recordingId: const Uuid().v4(),
      surahNumber: surahNumber,
      ayahStart: ayahStart,
      ayahEnd: ayahEnd,
      quality: _currentQuality,
      recordedAt: _recordingStartTime!,
      duration: duration,
      fileSizeBytes: fileSize,
      filePath: _currentRecordingPath,
    );

    _updateState(RecordingState.stopped);
    _currentRecordingPath = null;
    _recordingStartTime = null;

    return metadata;
  }

  /// Cancel recording and delete file
  Future<void> cancelRecording() async {
    if (_currentState == RecordingState.recording ||
        _currentState == RecordingState.paused) {
      await _recorder.stopRecorder();
      _durationTimer?.cancel();

      // Delete the recording file
      if (_currentRecordingPath != null) {
        final file = File(_currentRecordingPath!);
        if (await file.exists()) {
          await file.delete();
        }
      }

      _currentRecordingPath = null;
      _recordingStartTime = null;
      _amplitudes.clear();
      _updateState(RecordingState.idle);
    }
  }

  /// Get current recording duration
  Duration getCurrentDuration() {
    if (_recordingStartTime == null) {
      return Duration.zero;
    }
    return DateTime.now().difference(_recordingStartTime!);
  }

  /// Convert decibels to normalized amplitude (0.0 to 1.0)
  double _decibelToAmplitude(double decibels) {
    // Normalize decibels (-160 to 0) to amplitude (0.0 to 1.0)
    const minDb = -60.0;
    const maxDb = 0.0;
    final normalized = (decibels - minDb) / (maxDb - minDb);
    return normalized.clamp(0.0, 1.0);
  }

  /// Update waveform data
  void _updateWaveform() {
    if (_amplitudes.isEmpty) return;

    final maxAmplitude = _amplitudes.reduce((a, b) => a > b ? a : b);
    final minAmplitude = _amplitudes.reduce((a, b) => a < b ? a : b);

    final waveformData = WaveformData(
      amplitudes: List.from(_amplitudes),
      maxAmplitude: maxAmplitude,
      minAmplitude: minAmplitude,
      duration: getCurrentDuration(),
    );

    _waveformController.add(waveformData);
  }

  /// Update recording state
  void _updateState(RecordingState state) {
    _currentState = state;
    _stateController.add(state);
  }

  /// Start duration timer
  void _startDurationTimer() {
    _durationTimer?.cancel();
    _durationTimer = Timer.periodic(const Duration(seconds: 1), (timer) {
      _durationController.add(getCurrentDuration());
    });
  }

  /// Dispose resources
  Future<void> dispose() async {
    _durationTimer?.cancel();
    await _recorder.closeRecorder();
    await _waveformController.close();
    await _stateController.close();
    await _durationController.close();
  }
}

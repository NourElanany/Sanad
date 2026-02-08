import 'package:freezed_annotation/freezed_annotation.dart';

part 'recording_models.freezed.dart';
part 'recording_models.g.dart';

/// Recording session state
@freezed
class RecordingSession with _$RecordingSession {
  const factory RecordingSession({
    required String sessionId,
    String? userId,
    required int surahNumber,
    required int ayahStart,
    required int ayahEnd,
    required DateTime startedAt,
    required int maxDurationSeconds,
    required bool isActive,
  }) = _RecordingSession;

  factory RecordingSession.fromJson(Map<String, dynamic> json) =>
      _$RecordingSessionFromJson(json);
}

/// Audio quality settings
enum AudioQuality {
  low(sampleRate: 16000, bitRate: 32000),
  medium(sampleRate: 22050, bitRate: 64000),
  high(sampleRate: 44100, bitRate: 128000),
  ultra(sampleRate: 48000, bitRate: 192000);

  const AudioQuality({
    required this.sampleRate,
    required this.bitRate,
  });

  final int sampleRate;
  final int bitRate;
}

/// Recording state
enum RecordingState {
  idle,
  preparing,
  recording,
  paused,
  stopped,
  processing,
  error,
}

/// Waveform data point
@freezed
class WaveformData with _$WaveformData {
  const factory WaveformData({
    required List<double> amplitudes,
    required double maxAmplitude,
    required double minAmplitude,
    required Duration duration,
  }) = _WaveformData;
}

/// Recording metadata
@freezed
class RecordingMetadata with _$RecordingMetadata {
  const factory RecordingMetadata({
    required String recordingId,
    required int surahNumber,
    required int ayahStart,
    required int ayahEnd,
    required AudioQuality quality,
    required DateTime recordedAt,
    required Duration duration,
    required int fileSizeBytes,
    String? filePath,
  }) = _RecordingMetadata;

  factory RecordingMetadata.fromJson(Map<String, dynamic> json) =>
      _$RecordingMetadataFromJson(json);
}

/// Analysis result
@freezed
class RecitationAnalysis with _$RecitationAnalysis {
  const factory RecitationAnalysis({
    required String recordingId,
    required double overallScore,
    required DetailedScores detailedScores,
    required List<TajweedError> errors,
    required List<Recommendation> recommendations,
    required DateTime analyzedAt,
  }) = _RecitationAnalysis;

  factory RecitationAnalysis.fromJson(Map<String, dynamic> json) =>
      _$RecitationAnalysisFromJson(json);
}

/// Detailed scores
@freezed
class DetailedScores with _$DetailedScores {
  const factory DetailedScores({
    required double pronunciationAccuracy,
    required double timingAccuracy,
    required double tajweedCompliance,
    required double fluency,
    required double clarity,
    required double rhythm,
  }) = _DetailedScores;

  factory DetailedScores.fromJson(Map<String, dynamic> json) =>
      _$DetailedScoresFromJson(json);
}

/// Tajweed error
@freezed
class TajweedError with _$TajweedError {
  const factory TajweedError({
    required String errorType,
    required String description,
    required double timestamp,
    required String severity,
    String? correction,
  }) = _TajweedError;

  factory TajweedError.fromJson(Map<String, dynamic> json) =>
      _$TajweedErrorFromJson(json);
}

/// Recommendation
@freezed
class Recommendation with _$Recommendation {
  const factory Recommendation({
    required String category,
    required String priority,
    required String description,
    required String specificAdvice,
    required List<String> practiceExercises,
  }) = _Recommendation;

  factory Recommendation.fromJson(Map<String, dynamic> json) =>
      _$RecommendationFromJson(json);
}

/// Verse selection for recording
@freezed
class VerseSelection with _$VerseSelection {
  const factory VerseSelection({
    required int surahNumber,
    required String surahName,
    required int ayahStart,
    required int ayahEnd,
    required String arabicText,
  }) = _VerseSelection;
}

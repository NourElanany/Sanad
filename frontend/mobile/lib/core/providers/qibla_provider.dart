import 'dart:async';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:geolocator/geolocator.dart';
import '../services/qibla_service.dart';
import '../../features/qibla/data/models/qibla_model.dart';

/// Provider for QiblaService
final qiblaServiceProvider = Provider<QiblaService>((ref) {
  final service = QiblaService();
  ref.onDispose(() => service.dispose());
  return service;
});

/// State for Qibla feature
class QiblaState {
  final QiblaModel? qiblaData;
  final CompassState compassState;
  final bool isLoading;
  final String? error;
  final bool isNightMode;

  const QiblaState({
    this.qiblaData,
    this.compassState = const CompassState.initial(),
    this.isLoading = false,
    this.error,
    this.isNightMode = false,
  });

  QiblaState copyWith({
    QiblaModel? qiblaData,
    CompassState? compassState,
    bool? isLoading,
    String? error,
    bool? isNightMode,
  }) {
    return QiblaState(
      qiblaData: qiblaData ?? this.qiblaData,
      compassState: compassState ?? this.compassState,
      isLoading: isLoading ?? this.isLoading,
      error: error,
      isNightMode: isNightMode ?? this.isNightMode,
    );
  }
}

/// Notifier for managing Qibla state
class QiblaNotifier extends StateNotifier<QiblaState> {
  final QiblaService _qiblaService;
  StreamSubscription<double>? _compassSubscription;
  StreamSubscription<CompassCalibration>? _calibrationSubscription;

  QiblaNotifier(this._qiblaService) : super(const QiblaState());

  /// Initialize Qibla compass
  Future<void> initialize() async {
    state = state.copyWith(isLoading: true, error: null);

    try {
      // Check if compass is available
      final isAvailable = await _qiblaService.isCompassAvailable();
      if (!isAvailable) {
        throw Exception('البوصلة غير متوفرة على هذا الجهاز');
      }

      // Get current location
      final position = await _qiblaService.getCurrentLocation();

      // Calculate Qibla direction
      final qiblaData = await _qiblaService.calculateQiblaDirection(position);

      state = state.copyWith(
        qiblaData: qiblaData,
        isLoading: false,
      );

      // Start listening to compass updates
      _startCompassUpdates();

      // Start listening to calibration updates
      _startCalibrationUpdates();
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  /// Start listening to compass heading updates
  void _startCompassUpdates() {
    _compassSubscription?.cancel();
    _compassSubscription = _qiblaService.getCompassHeading().listen(
      (heading) {
        if (state.qiblaData != null) {
          final qiblaDirection = state.qiblaData!.direction;
          final relativeDirection = _calculateRelativeDirection(
            heading,
            qiblaDirection,
          );

          state = state.copyWith(
            compassState: state.compassState.copyWith(
              heading: heading,
              qiblaDirection: qiblaDirection,
              relativeDirection: relativeDirection,
            ),
          );
        }
      },
      onError: (error) {
        state = state.copyWith(error: 'خطأ في قراءة البوصلة: $error');
      },
    );
  }

  /// Start listening to calibration status updates
  void _startCalibrationUpdates() {
    _calibrationSubscription?.cancel();
    _calibrationSubscription = _qiblaService.getCalibrationStatus().listen(
      (calibration) {
        state = state.copyWith(
          compassState: state.compassState.copyWith(
            calibration: calibration,
          ),
        );
      },
    );
  }

  /// Calculate relative direction from device heading to Qibla
  /// Returns angle in degrees (-180 to 180)
  double _calculateRelativeDirection(double heading, double qiblaDirection) {
    double diff = qiblaDirection - heading;

    // Normalize to -180 to 180 range
    if (diff > 180) {
      diff -= 360;
    } else if (diff < -180) {
      diff += 360;
    }

    return diff;
  }

  /// Refresh Qibla calculation with current location
  Future<void> refresh() async {
    await initialize();
  }

  /// Toggle night mode
  void toggleNightMode() {
    state = state.copyWith(isNightMode: !state.isNightMode);
  }

  /// Manually trigger compass calibration
  void startCalibration() {
    state = state.copyWith(
      compassState: state.compassState.copyWith(
        calibration: const CompassCalibration(
          isCalibrated: false,
          accuracy: 0.0,
          message: 'حرك الجهاز على شكل رقم 8 لمعايرة البوصلة',
        ),
      ),
    );
  }

  @override
  void dispose() {
    _compassSubscription?.cancel();
    _calibrationSubscription?.cancel();
    super.dispose();
  }
}

/// Provider for Qibla state
final qiblaProvider = StateNotifierProvider<QiblaNotifier, QiblaState>((ref) {
  final service = ref.watch(qiblaServiceProvider);
  return QiblaNotifier(service);
});

/// Provider to check if compass is available
final compassAvailableProvider = FutureProvider<bool>((ref) async {
  final service = ref.watch(qiblaServiceProvider);
  return await service.isCompassAvailable();
});

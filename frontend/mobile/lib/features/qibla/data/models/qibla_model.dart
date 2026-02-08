import 'package:equatable/equatable.dart';

/// Model representing Qibla direction information
class QiblaModel extends Equatable {
  final double direction; // Qibla direction in degrees (0-360)
  final double distance; // Distance to Mecca in kilometers
  final String locationName; // Current location name
  final double latitude; // Current latitude
  final double longitude; // Current longitude
  final DateTime calculatedAt; // When the calculation was performed

  const QiblaModel({
    required this.direction,
    required this.distance,
    required this.locationName,
    required this.latitude,
    required this.longitude,
    required this.calculatedAt,
  });

  factory QiblaModel.fromJson(Map<String, dynamic> json) {
    return QiblaModel(
      direction: (json['direction'] as num).toDouble(),
      distance: (json['distance'] as num).toDouble(),
      locationName: json['location_name'] as String,
      latitude: (json['latitude'] as num).toDouble(),
      longitude: (json['longitude'] as num).toDouble(),
      calculatedAt: DateTime.parse(json['calculated_at'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'direction': direction,
      'distance': distance,
      'location_name': locationName,
      'latitude': latitude,
      'longitude': longitude,
      'calculated_at': calculatedAt.toIso8601String(),
    };
  }

  @override
  List<Object?> get props => [
        direction,
        distance,
        locationName,
        latitude,
        longitude,
        calculatedAt,
      ];
}

/// Model representing compass calibration status
class CompassCalibration extends Equatable {
  final bool isCalibrated;
  final double accuracy; // 0.0 to 1.0, where 1.0 is perfect
  final String message; // User-friendly calibration message

  const CompassCalibration({
    required this.isCalibrated,
    required this.accuracy,
    required this.message,
  });

  const CompassCalibration.uncalibrated()
      : isCalibrated = false,
        accuracy = 0.0,
        message = 'يرجى معايرة البوصلة';

  const CompassCalibration.calibrated()
      : isCalibrated = true,
        accuracy = 1.0,
        message = 'البوصلة معايرة بشكل صحيح';

  CompassCalibration copyWith({
    bool? isCalibrated,
    double? accuracy,
    String? message,
  }) {
    return CompassCalibration(
      isCalibrated: isCalibrated ?? this.isCalibrated,
      accuracy: accuracy ?? this.accuracy,
      message: message ?? this.message,
    );
  }

  @override
  List<Object?> get props => [isCalibrated, accuracy, message];
}

/// Model representing the current compass state
class CompassState extends Equatable {
  final double heading; // Current device heading in degrees (0-360)
  final double qiblaDirection; // Qibla direction relative to north
  final double relativeDirection; // Qibla direction relative to device heading
  final CompassCalibration calibration;

  const CompassState({
    required this.heading,
    required this.qiblaDirection,
    required this.relativeDirection,
    required this.calibration,
  });

  const CompassState.initial()
      : heading = 0.0,
        qiblaDirection = 0.0,
        relativeDirection = 0.0,
        calibration = const CompassCalibration.uncalibrated();

  CompassState copyWith({
    double? heading,
    double? qiblaDirection,
    double? relativeDirection,
    CompassCalibration? calibration,
  }) {
    return CompassState(
      heading: heading ?? this.heading,
      qiblaDirection: qiblaDirection ?? this.qiblaDirection,
      relativeDirection: relativeDirection ?? this.relativeDirection,
      calibration: calibration ?? this.calibration,
    );
  }

  /// Check if the device is pointing towards Qibla (within tolerance)
  bool get isPointingToQibla {
    const tolerance = 5.0; // 5 degrees tolerance
    return (relativeDirection.abs() <= tolerance);
  }

  @override
  List<Object?> get props => [
        heading,
        qiblaDirection,
        relativeDirection,
        calibration,
      ];
}

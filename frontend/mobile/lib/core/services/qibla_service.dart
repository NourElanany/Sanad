import 'dart:async';
import 'dart:math' as math;
import 'package:flutter_compass/flutter_compass.dart';
import 'package:geolocator/geolocator.dart';
import 'package:sensors_plus/sensors_plus.dart';
import '../../../features/qibla/data/models/qibla_model.dart';

/// Service for calculating Qibla direction and managing compass functionality
class QiblaService {
  // Kaaba coordinates (Mecca, Saudi Arabia)
  static const double kaabaLatitude = 21.4225;
  static const double kaabaLongitude = 39.8262;

  StreamSubscription<CompassEvent>? _compassSubscription;
  StreamSubscription<MagnetometerEvent>? _magnetometerSubscription;

  /// Calculate Qibla direction from current location
  Future<QiblaModel> calculateQiblaDirection(Position position) async {
    final qiblaDirection = _calculateBearing(
      position.latitude,
      position.longitude,
      kaabaLatitude,
      kaabaLongitude,
    );

    final distance = _calculateDistance(
      position.latitude,
      position.longitude,
      kaabaLatitude,
      kaabaLongitude,
    );

    // Get location name (simplified - in production, use geocoding service)
    final locationName = 'Lat: ${position.latitude.toStringAsFixed(4)}, '
        'Lon: ${position.longitude.toStringAsFixed(4)}';

    return QiblaModel(
      direction: qiblaDirection,
      distance: distance,
      locationName: locationName,
      latitude: position.latitude,
      longitude: position.longitude,
      calculatedAt: DateTime.now(),
    );
  }

  /// Get current device location
  Future<Position> getCurrentLocation() async {
    bool serviceEnabled;
    LocationPermission permission;

    // Check if location services are enabled
    serviceEnabled = await Geolocator.isLocationServiceEnabled();
    if (!serviceEnabled) {
      throw Exception('خدمات الموقع غير مفعلة. يرجى تفعيل GPS.');
    }

    // Check location permissions
    permission = await Geolocator.checkPermission();
    if (permission == LocationPermission.denied) {
      permission = await Geolocator.requestPermission();
      if (permission == LocationPermission.denied) {
        throw Exception('تم رفض إذن الوصول إلى الموقع.');
      }
    }

    if (permission == LocationPermission.deniedForever) {
      throw Exception(
        'إذن الوصول إلى الموقع مرفوض بشكل دائم. '
        'يرجى تفعيله من إعدادات التطبيق.',
      );
    }

    // Get current position
    return await Geolocator.getCurrentPosition(
      desiredAccuracy: LocationAccuracy.high,
    );
  }

  /// Stream of compass heading updates
  Stream<double> getCompassHeading() {
    return FlutterCompass.events!.map((event) {
      // Normalize heading to 0-360 range
      double heading = event.heading ?? 0.0;
      if (heading < 0) heading += 360;
      return heading;
    });
  }

  /// Check if compass is available on device
  Future<bool> isCompassAvailable() async {
    final compassEvents = FlutterCompass.events;
    return compassEvents != null;
  }

  /// Calculate compass calibration status based on magnetometer data
  Stream<CompassCalibration> getCalibrationStatus() async* {
    await for (final event in magnetometerEvents) {
      // Calculate magnetic field strength
      final x = event.x;
      final y = event.y;
      final z = event.z;
      final magnitude = math.sqrt(x * x + y * y + z * z);

      // Typical Earth's magnetic field is 25-65 microteslas
      // If magnitude is too low or too high, compass needs calibration
      if (magnitude < 20 || magnitude > 70) {
        yield CompassCalibration(
          isCalibrated: false,
          accuracy: _calculateAccuracy(magnitude),
          message: 'يرجى معايرة البوصلة بتحريك الجهاز على شكل رقم 8',
        );
      } else {
        yield CompassCalibration(
          isCalibrated: true,
          accuracy: _calculateAccuracy(magnitude),
          message: 'البوصلة معايرة بشكل صحيح',
        );
      }
    }
  }

  /// Calculate bearing (direction) from one point to another
  /// Returns angle in degrees (0-360) where 0 is North
  double _calculateBearing(
    double lat1,
    double lon1,
    double lat2,
    double lon2,
  ) {
    final dLon = _toRadians(lon2 - lon1);
    final lat1Rad = _toRadians(lat1);
    final lat2Rad = _toRadians(lat2);

    final y = math.sin(dLon) * math.cos(lat2Rad);
    final x = math.cos(lat1Rad) * math.sin(lat2Rad) -
        math.sin(lat1Rad) * math.cos(lat2Rad) * math.cos(dLon);

    final bearing = math.atan2(y, x);
    final bearingDegrees = _toDegrees(bearing);

    // Normalize to 0-360
    return (bearingDegrees + 360) % 360;
  }

  /// Calculate distance between two points using Haversine formula
  /// Returns distance in kilometers
  double _calculateDistance(
    double lat1,
    double lon1,
    double lat2,
    double lon2,
  ) {
    const earthRadius = 6371.0; // Earth's radius in kilometers

    final dLat = _toRadians(lat2 - lat1);
    final dLon = _toRadians(lon2 - lon1);

    final a = math.sin(dLat / 2) * math.sin(dLat / 2) +
        math.cos(_toRadians(lat1)) *
            math.cos(_toRadians(lat2)) *
            math.sin(dLon / 2) *
            math.sin(dLon / 2);

    final c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a));

    return earthRadius * c;
  }

  /// Calculate accuracy score based on magnetic field magnitude
  double _calculateAccuracy(double magnitude) {
    // Ideal range is 25-65 microteslas
    const idealMin = 25.0;
    const idealMax = 65.0;
    const idealMid = (idealMin + idealMax) / 2;

    if (magnitude >= idealMin && magnitude <= idealMax) {
      // Within ideal range - calculate how close to center
      final deviation = (magnitude - idealMid).abs();
      final maxDeviation = (idealMax - idealMin) / 2;
      return 1.0 - (deviation / maxDeviation) * 0.3; // 0.7 to 1.0
    } else if (magnitude < idealMin) {
      // Below ideal range
      return (magnitude / idealMin).clamp(0.0, 0.7);
    } else {
      // Above ideal range
      final excess = magnitude - idealMax;
      return (1.0 - (excess / idealMax)).clamp(0.0, 0.7);
    }
  }

  double _toRadians(double degrees) => degrees * math.pi / 180.0;
  double _toDegrees(double radians) => radians * 180.0 / math.pi;

  /// Clean up resources
  void dispose() {
    _compassSubscription?.cancel();
    _magnetometerSubscription?.cancel();
  }
}

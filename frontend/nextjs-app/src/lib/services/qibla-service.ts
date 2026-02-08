import { QiblaData, GeolocationPosition, CompassCalibration } from '@/types/qibla';

/**
 * Service for calculating Qibla direction and managing compass functionality
 */
export class QiblaService {
  // Kaaba coordinates (Mecca, Saudi Arabia)
  private static readonly KAABA_LATITUDE = 21.4225;
  private static readonly KAABA_LONGITUDE = 39.8262;

  /**
   * Calculate Qibla direction from current location
   */
  static calculateQiblaDirection(position: GeolocationPosition): QiblaData {
    const qiblaDirection = this.calculateBearing(
      position.latitude,
      position.longitude,
      this.KAABA_LATITUDE,
      this.KAABA_LONGITUDE
    );

    const distance = this.calculateDistance(
      position.latitude,
      position.longitude,
      this.KAABA_LATITUDE,
      this.KAABA_LONGITUDE
    );

    const locationName = `Lat: ${position.latitude.toFixed(4)}, Lon: ${position.longitude.toFixed(4)}`;

    return {
      direction: qiblaDirection,
      distance,
      locationName,
      latitude: position.latitude,
      longitude: position.longitude,
      calculatedAt: new Date().toISOString(),
    };
  }

  /**
   * Get current device location using Geolocation API
   */
  static async getCurrentLocation(): Promise<GeolocationPosition> {
    return new Promise((resolve, reject) => {
      if (!navigator.geolocation) {
        reject(new Error('Geolocation is not supported by your browser'));
        return;
      }

      navigator.geolocation.getCurrentPosition(
        (position) => {
          resolve({
            latitude: position.coords.latitude,
            longitude: position.coords.longitude,
            accuracy: position.coords.accuracy,
          });
        },
        (error) => {
          let errorMessage = 'Unable to retrieve your location';
          switch (error.code) {
            case error.PERMISSION_DENIED:
              errorMessage = 'Location permission denied';
              break;
            case error.POSITION_UNAVAILABLE:
              errorMessage = 'Location information unavailable';
              break;
            case error.TIMEOUT:
              errorMessage = 'Location request timed out';
              break;
          }
          reject(new Error(errorMessage));
        },
        {
          enableHighAccuracy: true,
          timeout: 10000,
          maximumAge: 0,
        }
      );
    });
  }

  /**
   * Check if device orientation API is available
   */
  static isOrientationAvailable(): boolean {
    return 'DeviceOrientationEvent' in window;
  }

  /**
   * Request permission for device orientation (iOS 13+)
   */
  static async requestOrientationPermission(): Promise<boolean> {
    if (typeof (DeviceOrientationEvent as any).requestPermission === 'function') {
      try {
        const permission = await (DeviceOrientationEvent as any).requestPermission();
        return permission === 'granted';
      } catch (error) {
        console.error('Error requesting orientation permission:', error);
        return false;
      }
    }
    // Permission not needed on non-iOS devices
    return true;
  }

  /**
   * Calculate bearing (direction) from one point to another
   * Returns angle in degrees (0-360) where 0 is North
   */
  private static calculateBearing(
    lat1: number,
    lon1: number,
    lat2: number,
    lon2: number
  ): number {
    const dLon = this.toRadians(lon2 - lon1);
    const lat1Rad = this.toRadians(lat1);
    const lat2Rad = this.toRadians(lat2);

    const y = Math.sin(dLon) * Math.cos(lat2Rad);
    const x =
      Math.cos(lat1Rad) * Math.sin(lat2Rad) -
      Math.sin(lat1Rad) * Math.cos(lat2Rad) * Math.cos(dLon);

    const bearing = Math.atan2(y, x);
    const bearingDegrees = this.toDegrees(bearing);

    // Normalize to 0-360
    return (bearingDegrees + 360) % 360;
  }

  /**
   * Calculate distance between two points using Haversine formula
   * Returns distance in kilometers
   */
  private static calculateDistance(
    lat1: number,
    lon1: number,
    lat2: number,
    lon2: number
  ): number {
    const earthRadius = 6371.0; // Earth's radius in kilometers

    const dLat = this.toRadians(lat2 - lat1);
    const dLon = this.toRadians(lon2 - lon1);

    const a =
      Math.sin(dLat / 2) * Math.sin(dLat / 2) +
      Math.cos(this.toRadians(lat1)) *
        Math.cos(this.toRadians(lat2)) *
        Math.sin(dLon / 2) *
        Math.sin(dLon / 2);

    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

    return earthRadius * c;
  }

  /**
   * Calculate compass calibration status
   */
  static calculateCalibrationStatus(
    magneticField?: { x: number; y: number; z: number }
  ): CompassCalibration {
    if (!magneticField) {
      return {
        isCalibrated: false,
        accuracy: 0,
        message: 'Compass calibration needed',
      };
    }

    const magnitude = Math.sqrt(
      magneticField.x ** 2 + magneticField.y ** 2 + magneticField.z ** 2
    );

    // Typical Earth's magnetic field is 25-65 microteslas
    if (magnitude < 20 || magnitude > 70) {
      return {
        isCalibrated: false,
        accuracy: this.calculateAccuracy(magnitude),
        message: 'Please calibrate compass by moving device in figure-8 pattern',
      };
    }

    return {
      isCalibrated: true,
      accuracy: this.calculateAccuracy(magnitude),
      message: 'Compass calibrated correctly',
    };
  }

  /**
   * Calculate accuracy score based on magnetic field magnitude
   */
  private static calculateAccuracy(magnitude: number): number {
    const idealMin = 25.0;
    const idealMax = 65.0;
    const idealMid = (idealMin + idealMax) / 2;

    if (magnitude >= idealMin && magnitude <= idealMax) {
      const deviation = Math.abs(magnitude - idealMid);
      const maxDeviation = (idealMax - idealMin) / 2;
      return 1.0 - (deviation / maxDeviation) * 0.3;
    } else if (magnitude < idealMin) {
      return Math.min(magnitude / idealMin, 0.7);
    } else {
      const excess = magnitude - idealMax;
      return Math.max(1.0 - excess / idealMax, 0);
    }
  }

  /**
   * Normalize heading to 0-360 range
   */
  static normalizeHeading(heading: number): number {
    let normalized = heading % 360;
    if (normalized < 0) normalized += 360;
    return normalized;
  }

  /**
   * Calculate relative direction from device heading to Qibla
   */
  static calculateRelativeDirection(
    deviceHeading: number,
    qiblaDirection: number
  ): number {
    let diff = qiblaDirection - deviceHeading;

    // Normalize to -180 to 180 range
    if (diff > 180) {
      diff -= 360;
    } else if (diff < -180) {
      diff += 360;
    }

    return diff;
  }

  /**
   * Check if device is pointing towards Qibla
   */
  static isPointingToQibla(relativeDirection: number, tolerance: number = 5): boolean {
    return Math.abs(relativeDirection) <= tolerance;
  }

  /**
   * Get cardinal direction name in Arabic
   */
  static getCardinalDirection(heading: number): string {
    const normalized = this.normalizeHeading(heading);

    if (normalized >= 337.5 || normalized < 22.5) return 'شمال';
    if (normalized >= 22.5 && normalized < 67.5) return 'شمال شرق';
    if (normalized >= 67.5 && normalized < 112.5) return 'شرق';
    if (normalized >= 112.5 && normalized < 157.5) return 'جنوب شرق';
    if (normalized >= 157.5 && normalized < 202.5) return 'جنوب';
    if (normalized >= 202.5 && normalized < 247.5) return 'جنوب غرب';
    if (normalized >= 247.5 && normalized < 292.5) return 'غرب';
    return 'شمال غرب';
  }

  /**
   * Format distance for display
   */
  static formatDistance(distanceKm: number): string {
    if (distanceKm < 1) {
      return `${Math.round(distanceKm * 1000)} متر`;
    } else if (distanceKm < 100) {
      return `${distanceKm.toFixed(1)} كم`;
    } else {
      return `${Math.round(distanceKm)} كم`;
    }
  }

  private static toRadians(degrees: number): number {
    return (degrees * Math.PI) / 180;
  }

  private static toDegrees(radians: number): number {
    return (radians * 180) / Math.PI;
  }
}

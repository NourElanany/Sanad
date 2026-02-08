/**
 * Qibla direction and location information
 */
export interface QiblaData {
  direction: number; // Qibla direction in degrees (0-360)
  distance: number; // Distance to Mecca in kilometers
  locationName: string; // Current location name
  latitude: number; // Current latitude
  longitude: number; // Current longitude
  calculatedAt: string; // ISO timestamp
}

/**
 * Compass calibration status
 */
export interface CompassCalibration {
  isCalibrated: boolean;
  accuracy: number; // 0.0 to 1.0
  message: string;
}

/**
 * Current compass state
 */
export interface CompassState {
  heading: number; // Current device heading (0-360)
  qiblaDirection: number; // Qibla direction relative to north
  relativeDirection: number; // Qibla direction relative to device heading
  calibration: CompassCalibration;
}

/**
 * Geolocation position
 */
export interface GeolocationPosition {
  latitude: number;
  longitude: number;
  accuracy: number;
}

/**
 * Device orientation data
 */
export interface DeviceOrientation {
  alpha: number | null; // Compass heading (0-360)
  beta: number | null; // Front-to-back tilt
  gamma: number | null; // Left-to-right tilt
  absolute: boolean;
}

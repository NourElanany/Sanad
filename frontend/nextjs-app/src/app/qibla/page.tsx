'use client';

import { useState, useEffect, useCallback } from 'react';
import { QiblaService } from '@/lib/services/qibla-service';
import { QiblaData, CompassState, DeviceOrientation } from '@/types/qibla';
import CompassVisualization from '@/components/qibla/CompassVisualization';
import QiblaInfoPanel from '@/components/qibla/QiblaInfoPanel';
import CalibrationModal from '@/components/qibla/CalibrationModal';
import { IslamicButton } from '@/components/ui';

export default function QiblaCompassPage() {
  const [qiblaData, setQiblaData] = useState<QiblaData | null>(null);
  const [compassState, setCompassState] = useState<CompassState>({
    heading: 0,
    qiblaDirection: 0,
    relativeDirection: 0,
    calibration: {
      isCalibrated: false,
      accuracy: 0,
      message: 'Compass calibration needed',
    },
  });
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isNightMode, setIsNightMode] = useState(false);
  const [showCalibration, setShowCalibration] = useState(false);
  const [orientationPermissionGranted, setOrientationPermissionGranted] = useState(false);

  // Initialize Qibla compass
  const initialize = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      // Check if orientation API is available
      if (!QiblaService.isOrientationAvailable()) {
        throw new Error('Device orientation not supported on this browser');
      }

      // Request orientation permission (iOS 13+)
      const permissionGranted = await QiblaService.requestOrientationPermission();
      setOrientationPermissionGranted(permissionGranted);

      if (!permissionGranted) {
        throw new Error('Device orientation permission denied');
      }

      // Get current location
      const position = await QiblaService.getCurrentLocation();

      // Calculate Qibla direction
      const qibla = QiblaService.calculateQiblaDirection(position);
      setQiblaData(qibla);

      setIsLoading(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An error occurred');
      setIsLoading(false);
    }
  }, []);

  // Handle device orientation updates
  useEffect(() => {
    if (!qiblaData || !orientationPermissionGranted) return;

    const handleOrientation = (event: DeviceOrientationEvent) => {
      if (event.alpha === null) return;

      // Get compass heading (alpha is 0-360 where 0 is North)
      const heading = QiblaService.normalizeHeading(360 - event.alpha);

      // Calculate relative direction to Qibla
      const relativeDirection = QiblaService.calculateRelativeDirection(
        heading,
        qiblaData.direction
      );

      setCompassState((prev) => ({
        ...prev,
        heading,
        qiblaDirection: qiblaData.direction,
        relativeDirection,
      }));
    };

    window.addEventListener('deviceorientation', handleOrientation);

    return () => {
      window.removeEventListener('deviceorientation', handleOrientation);
    };
  }, [qiblaData, orientationPermissionGranted]);

  // Initialize on mount
  useEffect(() => {
    initialize();
  }, [initialize]);

  // Auto-detect night mode based on time
  useEffect(() => {
    const hour = new Date().getHours();
    setIsNightMode(hour >= 18 || hour < 6);
  }, []);

  const handleRefresh = () => {
    initialize();
  };

  const handleToggleNightMode = () => {
    setIsNightMode((prev) => !prev);
  };

  const handleShowCalibration = () => {
    setShowCalibration(true);
  };

  return (
    <div
      className={`min-h-screen ${
        isNightMode
          ? 'bg-gradient-to-b from-[#0F1F35] to-[#1B365D]'
          : 'bg-gradient-to-b from-[#FEFEFE] to-[#F8F9FA]'
      }`}
    >
      {/* Header */}
      <header
        className={`${
          isNightMode ? 'bg-[#1B365D]' : 'bg-[#1B365D]'
        } text-white shadow-lg`}
      >
        <div className="container mx-auto px-4 py-6">
          <div className="flex items-center justify-between">
            <h1 className="text-2xl md:text-3xl font-bold font-['Tajawal']">
              بوصلة القبلة
            </h1>
            <div className="flex gap-2">
              <button
                onClick={handleToggleNightMode}
                className="p-2 rounded-lg hover:bg-white/10 transition-colors"
                title={isNightMode ? 'الوضع النهاري' : 'الوضع الليلي'}
              >
                {isNightMode ? (
                  <svg
                    className="w-6 h-6"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                    />
                  </svg>
                ) : (
                  <svg
                    className="w-6 h-6"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
                    />
                  </svg>
                )}
              </button>
              <button
                onClick={handleRefresh}
                disabled={isLoading}
                className="p-2 rounded-lg hover:bg-white/10 transition-colors disabled:opacity-50"
                title="تحديث الموقع"
              >
                <svg
                  className={`w-6 h-6 ${isLoading ? 'animate-spin' : ''}`}
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                  />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-8">
        {isLoading && (
          <div className="flex flex-col items-center justify-center min-h-[400px]">
            <div className="animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-[#B8860B]"></div>
            <p
              className={`mt-4 text-lg font-['Tajawal'] ${
                isNightMode ? 'text-white/70' : 'text-gray-600'
              }`}
            >
              جاري تحديد موقعك...
            </p>
          </div>
        )}

        {error && (
          <div className="flex flex-col items-center justify-center min-h-[400px]">
            <div className="text-red-500 text-6xl mb-4">⚠️</div>
            <p
              className={`text-lg text-center mb-6 font-['Tajawal'] ${
                isNightMode ? 'text-white' : 'text-gray-800'
              }`}
            >
              {error}
            </p>
            <IslamicButton onClick={handleRefresh}>إعادة المحاولة</IslamicButton>
          </div>
        )}

        {!isLoading && !error && qiblaData && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
            {/* Compass Visualization */}
            <div>
              {!compassState.calibration.isCalibrated && (
                <div
                  className={`mb-4 p-4 rounded-lg ${
                    isNightMode
                      ? 'bg-orange-900/30 border border-orange-500/30'
                      : 'bg-orange-50 border border-orange-200'
                  }`}
                >
                  <div className="flex items-start gap-3">
                    <span className="text-2xl">⚠️</span>
                    <div>
                      <h3
                        className={`font-bold font-['Tajawal'] ${
                          isNightMode ? 'text-white' : 'text-gray-800'
                        }`}
                      >
                        تحتاج البوصلة إلى معايرة
                      </h3>
                      <p
                        className={`text-sm mt-1 font-['Tajawal'] ${
                          isNightMode ? 'text-white/70' : 'text-gray-600'
                        }`}
                      >
                        {compassState.calibration.message}
                      </p>
                    </div>
                  </div>
                </div>
              )}

              <CompassVisualization
                compassState={compassState}
                isNightMode={isNightMode}
              />

              <div className="mt-6">
                <IslamicButton
                  onClick={handleShowCalibration}
                  variant="secondary"
                  className="w-full"
                >
                  <svg
                    className="w-5 h-5 mr-2"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                    />
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                    />
                  </svg>
                  معايرة البوصلة
                </IslamicButton>
              </div>
            </div>

            {/* Qibla Information */}
            <div>
              <QiblaInfoPanel qiblaData={qiblaData} isNightMode={isNightMode} />
            </div>
          </div>
        )}
      </main>

      {/* Calibration Modal */}
      {showCalibration && (
        <CalibrationModal
          isOpen={showCalibration}
          onClose={() => setShowCalibration(false)}
        />
      )}
    </div>
  );
}

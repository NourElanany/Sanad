'use client';

import { QiblaData } from '@/types/qibla';
import { QiblaService } from '@/lib/services/qibla-service';
import { IslamicCard } from '@/components/ui';

interface QiblaInfoPanelProps {
  qiblaData: QiblaData;
  isNightMode?: boolean;
}

export default function QiblaInfoPanel({
  qiblaData,
  isNightMode = false,
}: QiblaInfoPanelProps) {
  const formatCoordinates = (lat: number, lon: number): string => {
    const latDir = lat >= 0 ? 'شمال' : 'جنوب';
    const lonDir = lon >= 0 ? 'شرق' : 'غرب';
    return `${Math.abs(lat).toFixed(4)}° ${latDir}, ${Math.abs(lon).toFixed(4)}° ${lonDir}`;
  };

  const formatTime = (isoString: string): string => {
    const date = new Date(isoString);
    return date.toLocaleTimeString('ar-SA', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  };

  const formatDate = (isoString: string): string => {
    const date = new Date(isoString);
    return date.toLocaleDateString('ar-SA', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
    });
  };

  const infoItems = [
    {
      icon: (
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
            d="M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7"
          />
        </svg>
      ),
      label: 'الاتجاه',
      value: `${qiblaData.direction.toFixed(1)}°`,
      subtitle: QiblaService.getCardinalDirection(qiblaData.direction),
    },
    {
      icon: (
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
            d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
          />
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>
      ),
      label: 'المسافة إلى مكة',
      value: QiblaService.formatDistance(qiblaData.distance),
      subtitle: 'خط مستقيم',
    },
    {
      icon: (
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
            d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
          />
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>
      ),
      label: 'موقعك الحالي',
      value: qiblaData.locationName,
      subtitle: formatCoordinates(qiblaData.latitude, qiblaData.longitude),
    },
    {
      icon: (
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
            d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
      ),
      label: 'آخر تحديث',
      value: formatTime(qiblaData.calculatedAt),
      subtitle: formatDate(qiblaData.calculatedAt),
    },
  ];

  return (
    <IslamicCard
      className={`p-6 ${
        isNightMode ? 'bg-[#1B365D]/30' : 'bg-white'
      }`}
    >
      <h2
        className={`text-2xl font-bold text-center mb-6 font-['Tajawal'] ${
          isNightMode ? 'text-[#B8860B]' : 'text-[#1B365D]'
        }`}
      >
        معلومات القبلة
      </h2>

      <div className="space-y-6">
        {infoItems.map((item, index) => (
          <div key={index}>
            <div className="flex items-start gap-4">
              <div
                className={`flex-shrink-0 w-12 h-12 rounded-xl flex items-center justify-center ${
                  isNightMode
                    ? 'bg-[#B8860B]/20 text-[#B8860B]'
                    : 'bg-[#1B365D]/10 text-[#1B365D]'
                }`}
              >
                {item.icon}
              </div>

              <div className="flex-1">
                <p
                  className={`text-sm font-['Tajawal'] ${
                    isNightMode ? 'text-white/70' : 'text-gray-600'
                  }`}
                >
                  {item.label}
                </p>
                <p
                  className={`text-lg font-bold mt-1 font-['Tajawal'] ${
                    isNightMode ? 'text-white' : 'text-gray-900'
                  }`}
                >
                  {item.value}
                </p>
                {item.subtitle && (
                  <p
                    className={`text-xs mt-1 font-['Tajawal'] ${
                      isNightMode ? 'text-white/60' : 'text-gray-500'
                    }`}
                  >
                    {item.subtitle}
                  </p>
                )}
              </div>
            </div>

            {index < infoItems.length - 1 && (
              <div
                className={`mt-6 border-t ${
                  isNightMode ? 'border-white/10' : 'border-gray-200'
                }`}
              />
            )}
          </div>
        ))}
      </div>

      {/* Map placeholder */}
      <div className="mt-6">
        <div
          className={`rounded-lg overflow-hidden border ${
            isNightMode ? 'border-white/10' : 'border-gray-200'
          }`}
        >
          <div
            className={`h-48 flex items-center justify-center ${
              isNightMode ? 'bg-[#0F1F35]' : 'bg-gray-100'
            }`}
          >
            <div className="text-center">
              <svg
                className={`w-16 h-16 mx-auto mb-2 ${
                  isNightMode ? 'text-white/30' : 'text-gray-400'
                }`}
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7"
                />
              </svg>
              <p
                className={`text-sm font-['Tajawal'] ${
                  isNightMode ? 'text-white/50' : 'text-gray-500'
                }`}
              >
                عرض الخريطة (قريباً)
              </p>
            </div>
          </div>
        </div>
      </div>
    </IslamicCard>
  );
}

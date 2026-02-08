'use client';

import { useEffect, useState } from 'react';

interface WeatherData {
  temperature: number;
  condition: string;
  conditionArabic: string;
  humidity: number;
  windSpeed: number;
  sunrise: Date;
  sunset: Date;
  location: string;
  icon: string;
}

interface WeatherWidgetProps {
  weatherData?: WeatherData;
  onTap?: () => void;
}

export function WeatherWidget({ weatherData, onTap }: WeatherWidgetProps) {
  const [weather, setWeather] = useState<WeatherData>({
    temperature: 28.5,
    condition: 'Clear',
    conditionArabic: 'صافي',
    humidity: 45,
    windSpeed: 12.5,
    sunrise: new Date(new Date().setHours(6, 15, 0)),
    sunset: new Date(new Date().setHours(18, 30, 0)),
    location: 'الرياض',
    icon: '☀️',
  });

  useEffect(() => {
    if (weatherData) {
      setWeather(weatherData);
    }
  }, [weatherData]);

  const isHotWeather = weather.temperature > 35;
  const isColdWeather = weather.temperature < 15;
  const isGoodForOutdoorPrayer = weather.temperature >= 15 && weather.temperature <= 35;

  const getWeatherColor = (): string => {
    if (isHotWeather) return '#FFC107';
    if (isColdWeather) return '#17A2B8';
    return '#28A745';
  };

  const getIslamicRecommendations = (): string[] => {
    const recommendations: string[] = [];

    if (isHotWeather) {
      recommendations.push('الطقس حار، احرص على شرب الماء بعد الإفطار');
      recommendations.push('يُستحب الصلاة في المسجد المكيف في الأوقات الحارة');
    } else if (isColdWeather) {
      recommendations.push('الطقس بارد، تذكر الوضوء بماء دافئ');
      recommendations.push('وقت مناسب للتهجد والقيام');
    } else {
      recommendations.push('طقس معتدل، وقت مناسب للصلاة في المسجد');
      recommendations.push('جو مناسب للمشي إلى المسجد');
    }

    const hour = new Date().getHours();
    if (hour >= 12 && hour < 15) {
      recommendations.push('وقت صلاة الظهر، احرص على أدائها في وقتها');
    }

    return recommendations;
  };

  const formatTime = (date: Date): string => {
    return date.toLocaleTimeString('ar-SA', {
      hour: '2-digit',
      minute: '2-digit',
      hour12: false,
    });
  };

  const weatherColor = getWeatherColor();
  const recommendations = getIslamicRecommendations();

  return (
    <div
      className="bg-white rounded-2xl shadow-lg border border-primary/10 p-6 cursor-pointer hover:shadow-xl transition-all"
      onClick={onTap}
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <div
            className="p-3 rounded-xl"
            style={{
              background: `linear-gradient(135deg, ${weatherColor}, ${weatherColor}B3)`,
            }}
          >
            <span className="text-2xl">{weather.icon}</span>
          </div>
          <div>
            <h3 className="text-xl font-bold text-primary">حالة الطقس</h3>
            <p className="text-sm text-gray-600">{weather.location}</p>
          </div>
        </div>
        <div className="text-right">
          <div className="flex items-start">
            <span className="text-4xl font-bold" style={{ color: weatherColor }}>
              {weather.temperature.toFixed(0)}
            </span>
            <span className="text-2xl" style={{ color: weatherColor }}>
              °
            </span>
          </div>
          <p className="text-sm text-gray-600">{weather.conditionArabic}</p>
        </div>
      </div>

      {/* Weather Details */}
      <div className="grid grid-cols-2 gap-3 mb-4">
        <div className="bg-blue-50 rounded-lg p-3">
          <div className="flex items-center gap-2 mb-1">
            <span className="text-lg">💧</span>
            <span className="text-xs text-gray-600">الرطوبة</span>
          </div>
          <p className="text-lg font-bold text-blue-600">{weather.humidity}%</p>
        </div>
        <div className="bg-green-50 rounded-lg p-3">
          <div className="flex items-center gap-2 mb-1">
            <span className="text-lg">💨</span>
            <span className="text-xs text-gray-600">الرياح</span>
          </div>
          <p className="text-lg font-bold text-green-600">
            {weather.windSpeed.toFixed(0)} كم/س
          </p>
        </div>
      </div>

      {/* Sunrise and Sunset */}
      <div className="grid grid-cols-2 gap-3 mb-4">
        <div className="bg-yellow-50 rounded-lg p-3 text-center">
          <span className="text-2xl mb-1 block">🌅</span>
          <p className="text-xs text-gray-600 mb-1">الشروق</p>
          <p className="text-base font-bold text-yellow-600 font-mono">
            {formatTime(weather.sunrise)}
          </p>
        </div>
        <div className="bg-orange-50 rounded-lg p-3 text-center">
          <span className="text-2xl mb-1 block">🌆</span>
          <p className="text-xs text-gray-600 mb-1">الغروب</p>
          <p className="text-base font-bold text-orange-600 font-mono">
            {formatTime(weather.sunset)}
          </p>
        </div>
      </div>

      {/* Islamic Recommendations */}
      <div className="bg-gradient-to-br from-secondary/10 to-primary/10 border border-primary/20 rounded-lg p-4">
        <div className="flex items-center gap-2 mb-3">
          <span className="text-lg">💡</span>
          <span className="text-sm font-bold text-primary">نصائح إسلامية</span>
        </div>
        <div className="space-y-2">
          {recommendations.map((recommendation, index) => (
            <div key={index} className="flex items-start gap-2">
              <span className="text-primary mt-1">•</span>
              <p className="text-sm text-gray-900 leading-relaxed flex-1">
                {recommendation}
              </p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

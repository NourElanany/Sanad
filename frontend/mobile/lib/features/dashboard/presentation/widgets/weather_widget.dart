import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Model for weather data
class WeatherData {
  final double temperature;
  final String condition;
  final String conditionArabic;
  final int humidity;
  final double windSpeed;
  final DateTime sunrise;
  final DateTime sunset;
  final String location;
  final String icon;

  WeatherData({
    required this.temperature,
    required this.condition,
    required this.conditionArabic,
    required this.humidity,
    required this.windSpeed,
    required this.sunrise,
    required this.sunset,
    required this.location,
    required this.icon,
  });

  factory WeatherData.fromJson(Map<String, dynamic> json) {
    return WeatherData(
      temperature: json['temperature']?.toDouble() ?? 0.0,
      condition: json['condition'] ?? '',
      conditionArabic: json['condition_arabic'] ?? '',
      humidity: json['humidity'] ?? 0,
      windSpeed: json['wind_speed']?.toDouble() ?? 0.0,
      sunrise: DateTime.parse(json['sunrise']),
      sunset: DateTime.parse(json['sunset']),
      location: json['location'] ?? '',
      icon: json['icon'] ?? '☀️',
    );
  }

  bool get isHotWeather => temperature > 35;
  bool get isColdWeather => temperature < 15;
  bool get isGoodForOutdoorPrayer => temperature >= 15 && temperature <= 35;
}

/// Provider for weather data
final weatherDataProvider = FutureProvider<WeatherData>((ref) async {
  // TODO: Fetch from backend API or weather service
  // For now, return mock data
  final now = DateTime.now();
  return WeatherData(
    temperature: 28.5,
    condition: 'Clear',
    conditionArabic: 'صافي',
    humidity: 45,
    windSpeed: 12.5,
    sunrise: DateTime(now.year, now.month, now.day, 6, 15),
    sunset: DateTime(now.year, now.month, now.day, 18, 30),
    location: 'الرياض',
    icon: '☀️',
  );
});

/// Interactive Weather Widget for fasting and prayer
class WeatherWidget extends ConsumerWidget {
  final WeatherData? weatherData;
  final VoidCallback? onTap;

  const WeatherWidget({
    Key? key,
    this.weatherData,
    this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final weatherAsync = weatherData != null
        ? AsyncValue.data(weatherData!)
        : ref.watch(weatherDataProvider);

    return weatherAsync.when(
      data: (weather) => _buildWidget(context, weather),
      loading: () => _buildLoadingWidget(),
      error: (error, stack) => _buildErrorWidget(error),
    );
  }

  Widget _buildWidget(BuildContext context, WeatherData weather) {
    return IslamicCard(
      onTap: onTap,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header
          Row(
            children: [
              Container(
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  gradient: LinearGradient(
                    colors: [
                      _getWeatherColor(weather),
                      _getWeatherColor(weather).withOpacity(0.7),
                    ],
                  ),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Text(
                  weather.icon,
                  style: const TextStyle(fontSize: 24),
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'حالة الطقس',
                      style: AppTextStyles.h6.copyWith(
                        color: AppColors.primary,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      weather.location,
                      style: AppTextStyles.bodySmall.copyWith(
                        color: AppColors.textSecondary,
                      ),
                    ),
                  ],
                ),
              ),
              // Temperature
              Column(
                crossAxisAlignment: CrossAxisAlignment.end,
                children: [
                  Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        weather.temperature.toStringAsFixed(0),
                        style: AppTextStyles.h3.copyWith(
                          color: _getWeatherColor(weather),
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      Text(
                        '°',
                        style: AppTextStyles.h5.copyWith(
                          color: _getWeatherColor(weather),
                        ),
                      ),
                    ],
                  ),
                  Text(
                    weather.conditionArabic,
                    style: AppTextStyles.caption.copyWith(
                      color: AppColors.textSecondary,
                    ),
                  ),
                ],
              ),
            ],
          ),
          const SizedBox(height: 16),

          // Weather details
          Row(
            children: [
              Expanded(
                child: _buildWeatherDetail(
                  icon: Icons.water_drop_outlined,
                  label: 'الرطوبة',
                  value: '${weather.humidity}%',
                  color: AppColors.info,
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: _buildWeatherDetail(
                  icon: Icons.air,
                  label: 'الرياح',
                  value: '${weather.windSpeed.toStringAsFixed(0)} كم/س',
                  color: AppColors.secondary,
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),

          // Sunrise and Sunset
          Row(
            children: [
              Expanded(
                child: _buildSunTimeCard(
                  icon: Icons.wb_sunny,
                  label: 'الشروق',
                  time: _formatTime(weather.sunrise),
                  color: AppColors.warning,
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: _buildSunTimeCard(
                  icon: Icons.wb_twilight,
                  label: 'الغروب',
                  time: _formatTime(weather.sunset),
                  color: AppColors.accent,
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),

          // Islamic recommendations
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              gradient: LinearGradient(
                colors: [
                  AppColors.secondary.withOpacity(0.1),
                  AppColors.primary.withOpacity(0.1),
                ],
              ),
              borderRadius: BorderRadius.circular(10),
              border: Border.all(
                color: AppColors.primary.withOpacity(0.2),
              ),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(
                      Icons.tips_and_updates_outlined,
                      size: 18,
                      color: AppColors.primary,
                    ),
                    const SizedBox(width: 6),
                    Text(
                      'نصائح إسلامية',
                      style: AppTextStyles.bodySmall.copyWith(
                        color: AppColors.primary,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 8),
                ..._getIslamicRecommendations(weather).map((recommendation) {
                  return Padding(
                    padding: const EdgeInsets.only(bottom: 6),
                    child: Row(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          '• ',
                          style: AppTextStyles.bodySmall.copyWith(
                            color: AppColors.textPrimary,
                          ),
                        ),
                        Expanded(
                          child: Text(
                            recommendation,
                            style: AppTextStyles.bodySmall.copyWith(
                              color: AppColors.textPrimary,
                              height: 1.5,
                            ),
                          ),
                        ),
                      ],
                    ),
                  );
                }).toList(),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildWeatherDetail({
    required IconData icon,
    required String label,
    required String value,
    required Color color,
  }) {
    return Container(
      padding: const EdgeInsets.all(10),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Row(
        children: [
          Icon(icon, size: 20, color: color),
          const SizedBox(width: 8),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  label,
                  style: AppTextStyles.caption.copyWith(
                    color: AppColors.textSecondary,
                  ),
                ),
                const SizedBox(height: 2),
                Text(
                  value,
                  style: AppTextStyles.bodySmall.copyWith(
                    color: color,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSunTimeCard({
    required IconData icon,
    required String label,
    required String time,
    required Color color,
  }) {
    return Container(
      padding: const EdgeInsets.all(10),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        children: [
          Icon(icon, size: 24, color: color),
          const SizedBox(height: 6),
          Text(
            label,
            style: AppTextStyles.caption.copyWith(
              color: AppColors.textSecondary,
            ),
          ),
          const SizedBox(height: 2),
          Text(
            time,
            style: AppTextStyles.bodyMedium.copyWith(
              color: color,
              fontWeight: FontWeight.bold,
              fontFeatures: [const FontFeature.tabularFigures()],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildLoadingWidget() {
    return IslamicCard(
      child: Center(
        child: Padding(
          padding: const EdgeInsets.all(40),
          child: CircularProgressIndicator(
            valueColor: AlwaysStoppedAnimation<Color>(AppColors.primary),
          ),
        ),
      ),
    );
  }

  Widget _buildErrorWidget(Object error) {
    return IslamicCard(
      child: Center(
        child: Padding(
          padding: const EdgeInsets.all(20),
          child: Text(
            'حدث خطأ في تحميل بيانات الطقس',
            style: AppTextStyles.bodyMedium.copyWith(
              color: AppColors.error,
            ),
          ),
        ),
      ),
    );
  }

  Color _getWeatherColor(WeatherData weather) {
    if (weather.isHotWeather) return AppColors.warning;
    if (weather.isColdWeather) return AppColors.info;
    return AppColors.success;
  }

  List<String> _getIslamicRecommendations(WeatherData weather) {
    final recommendations = <String>[];

    if (weather.isHotWeather) {
      recommendations.add('الطقس حار، احرص على شرب الماء بعد الإفطار');
      recommendations.add('يُستحب الصلاة في المسجد المكيف في الأوقات الحارة');
    } else if (weather.isColdWeather) {
      recommendations.add('الطقس بارد، تذكر الوضوء بماء دافئ');
      recommendations.add('وقت مناسب للتهجد والقيام');
    } else {
      recommendations.add('طقس معتدل، وقت مناسب للصلاة في المسجد');
      recommendations.add('جو مناسب للمشي إلى المسجد');
    }

    // Add general recommendations
    final now = DateTime.now();
    if (now.hour >= 12 && now.hour < 15) {
      recommendations.add('وقت صلاة الظهر، احرص على أدائها في وقتها');
    }

    return recommendations;
  }

  String _formatTime(DateTime time) {
    final hour = time.hour.toString().padLeft(2, '0');
    final minute = time.minute.toString().padLeft(2, '0');
    return '$hour:$minute';
  }
}

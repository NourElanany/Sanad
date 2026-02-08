import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import '../../data/models/qibla_model.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Card displaying Qibla information (direction, distance, location)
class QiblaInfoCard extends StatelessWidget {
  final QiblaModel qiblaData;
  final bool isNightMode;

  const QiblaInfoCard({
    super.key,
    required this.qiblaData,
    this.isNightMode = false,
  });

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      padding: const EdgeInsets.all(20),
      backgroundColor: isNightMode
          ? AppColors.primaryMain.withOpacity(0.3)
          : AppColors.backgroundPaper,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Title
          Text(
            'معلومات القبلة',
            style: TextStyle(
              fontFamily: 'Tajawal',
              fontSize: 20,
              fontWeight: FontWeight.bold,
              color: isNightMode
                  ? AppColors.accentGold
                  : AppColors.primaryMain,
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 20),

          // Direction
          _buildInfoRow(
            icon: Icons.explore,
            label: 'الاتجاه',
            value: '${qiblaData.direction.toStringAsFixed(1)}°',
            subtitle: _getDirectionDescription(qiblaData.direction),
          ),

          const Divider(height: 24),

          // Distance to Mecca
          _buildInfoRow(
            icon: Icons.place,
            label: 'المسافة إلى مكة',
            value: _formatDistance(qiblaData.distance),
            subtitle: 'خط مستقيم',
          ),

          const Divider(height: 24),

          // Current location
          _buildInfoRow(
            icon: Icons.location_on,
            label: 'موقعك الحالي',
            value: qiblaData.locationName,
            subtitle: _formatCoordinates(
              qiblaData.latitude,
              qiblaData.longitude,
            ),
          ),

          const Divider(height: 24),

          // Last updated
          _buildInfoRow(
            icon: Icons.access_time,
            label: 'آخر تحديث',
            value: _formatTime(qiblaData.calculatedAt),
            subtitle: _formatDate(qiblaData.calculatedAt),
          ),
        ],
      ),
    );
  }

  Widget _buildInfoRow({
    required IconData icon,
    required String label,
    required String value,
    String? subtitle,
  }) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Icon
        Container(
          width: 48,
          height: 48,
          decoration: BoxDecoration(
            color: isNightMode
                ? AppColors.accentGold.withOpacity(0.2)
                : AppColors.primaryMain.withOpacity(0.1),
            borderRadius: BorderRadius.circular(12),
          ),
          child: Icon(
            icon,
            color: isNightMode
                ? AppColors.accentGold
                : AppColors.primaryMain,
            size: 24,
          ),
        ),
        const SizedBox(width: 16),

        // Text content
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                label,
                style: TextStyle(
                  fontFamily: 'Tajawal',
                  fontSize: 14,
                  color: isNightMode
                      ? Colors.white70
                      : AppColors.textSecondary,
                ),
              ),
              const SizedBox(height: 4),
              Text(
                value,
                style: TextStyle(
                  fontFamily: 'Tajawal',
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  color: isNightMode ? Colors.white : AppColors.textPrimary,
                ),
              ),
              if (subtitle != null) ...[
                const SizedBox(height: 2),
                Text(
                  subtitle,
                  style: TextStyle(
                    fontFamily: 'Tajawal',
                    fontSize: 12,
                    color: isNightMode
                        ? Colors.white60
                        : AppColors.textSecondary,
                  ),
                ),
              ],
            ],
          ),
        ),
      ],
    );
  }

  String _getDirectionDescription(double direction) {
    if (direction >= 337.5 || direction < 22.5) return 'شمال';
    if (direction >= 22.5 && direction < 67.5) return 'شمال شرق';
    if (direction >= 67.5 && direction < 112.5) return 'شرق';
    if (direction >= 112.5 && direction < 157.5) return 'جنوب شرق';
    if (direction >= 157.5 && direction < 202.5) return 'جنوب';
    if (direction >= 202.5 && direction < 247.5) return 'جنوب غرب';
    if (direction >= 247.5 && direction < 292.5) return 'غرب';
    return 'شمال غرب';
  }

  String _formatDistance(double distanceKm) {
    if (distanceKm < 1) {
      return '${(distanceKm * 1000).toStringAsFixed(0)} متر';
    } else if (distanceKm < 100) {
      return '${distanceKm.toStringAsFixed(1)} كم';
    } else {
      return '${distanceKm.toStringAsFixed(0)} كم';
    }
  }

  String _formatCoordinates(double lat, double lon) {
    final latDir = lat >= 0 ? 'شمال' : 'جنوب';
    final lonDir = lon >= 0 ? 'شرق' : 'غرب';
    return '${lat.abs().toStringAsFixed(4)}° $latDir, '
        '${lon.abs().toStringAsFixed(4)}° $lonDir';
  }

  String _formatTime(DateTime dateTime) {
    return DateFormat('HH:mm:ss', 'ar').format(dateTime);
  }

  String _formatDate(DateTime dateTime) {
    return DateFormat('yyyy/MM/dd', 'ar').format(dateTime);
  }
}

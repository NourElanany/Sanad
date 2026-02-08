import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dart:async';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../../../core/services/prayer_times_service.dart';
import '../../../../core/services/notification_service.dart';

/// Interactive Prayer Times Widget with notification support
class PrayerTimesWidget extends ConsumerStatefulWidget {
  final PrayerTimes prayerTimes;
  final VoidCallback? onTap;

  const PrayerTimesWidget({
    Key? key,
    required this.prayerTimes,
    this.onTap,
  }) : super(key: key);

  @override
  ConsumerState<PrayerTimesWidget> createState() => _PrayerTimesWidgetState();
}

class _PrayerTimesWidgetState extends ConsumerState<PrayerTimesWidget> {
  Timer? _timer;
  Duration _timeRemaining = Duration.zero;
  bool _notificationsEnabled = true;
  bool _isExpanded = false;

  @override
  void initState() {
    super.initState();
    _startCountdown();
    _loadNotificationSettings();
  }

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  Future<void> _loadNotificationSettings() async {
    final notificationService = ref.read(notificationServiceProvider);
    final enabled = await notificationService.areNotificationsEnabled();
    if (mounted) {
      setState(() {
        _notificationsEnabled = enabled;
      });
    }
  }

  void _startCountdown() {
    _updateTimeRemaining();
    _timer = Timer.periodic(const Duration(seconds: 1), (_) {
      _updateTimeRemaining();
    });
  }

  void _updateTimeRemaining() {
    final nextPrayer = widget.prayerTimes.getNextPrayer();
    final nextPrayerTime = _parseTime(nextPrayer['time']!);
    final now = DateTime.now();

    if (mounted) {
      setState(() {
        _timeRemaining = nextPrayerTime.difference(now);
        if (_timeRemaining.isNegative) {
          _timeRemaining = Duration.zero;
        }
      });
    }
  }

  DateTime _parseTime(String time) {
    final parts = time.split(':');
    final now = DateTime.now();
    var prayerTime = DateTime(
      now.year,
      now.month,
      now.day,
      int.parse(parts[0]),
      int.parse(parts[1]),
    );

    if (prayerTime.isBefore(now)) {
      prayerTime = prayerTime.add(const Duration(days: 1));
    }

    return prayerTime;
  }

  String _formatDuration(Duration duration) {
    final hours = duration.inHours;
    final minutes = duration.inMinutes.remainder(60);
    final seconds = duration.inSeconds.remainder(60);
    return '${hours.toString().padLeft(2, '0')}:${minutes.toString().padLeft(2, '0')}:${seconds.toString().padLeft(2, '0')}';
  }

  Future<void> _toggleNotifications() async {
    final notificationService = ref.read(notificationServiceProvider);
    
    if (_notificationsEnabled) {
      await notificationService.disablePrayerNotifications();
    } else {
      await notificationService.enablePrayerNotifications(widget.prayerTimes);
    }

    setState(() {
      _notificationsEnabled = !_notificationsEnabled;
    });

    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            _notificationsEnabled
                ? 'تم تفعيل تنبيهات الصلاة'
                : 'تم إيقاف تنبيهات الصلاة',
          ),
          duration: const Duration(seconds: 2),
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    final nextPrayer = widget.prayerTimes.getNextPrayer();

    return IslamicCard(
      onTap: widget.onTap,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header with notification toggle
          Row(
            children: [
              Container(
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  gradient: LinearGradient(
                    colors: [
                      AppColors.primary,
                      AppColors.primary.withOpacity(0.7),
                    ],
                  ),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: const Icon(
                  Icons.mosque,
                  color: Colors.white,
                  size: 24,
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'الصلاة القادمة',
                      style: AppTextStyles.bodyMedium.copyWith(
                        color: AppColors.textSecondary,
                      ),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      nextPrayer['name']!,
                      style: AppTextStyles.h5.copyWith(
                        color: AppColors.primary,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ],
                ),
              ),
              // Notification toggle button
              IconButton(
                icon: Icon(
                  _notificationsEnabled
                      ? Icons.notifications_active
                      : Icons.notifications_off_outlined,
                  color: _notificationsEnabled
                      ? AppColors.accent
                      : AppColors.textSecondary,
                ),
                onPressed: _toggleNotifications,
                tooltip: _notificationsEnabled
                    ? 'إيقاف التنبيهات'
                    : 'تفعيل التنبيهات',
              ),
              Text(
                nextPrayer['time']!,
                style: AppTextStyles.h5.copyWith(
                  color: AppColors.accent,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),

          // Countdown with animation
          AnimatedContainer(
            duration: const Duration(milliseconds: 300),
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              gradient: LinearGradient(
                colors: [
                  AppColors.primary.withOpacity(0.1),
                  AppColors.secondary.withOpacity(0.1),
                ],
                begin: Alignment.topLeft,
                end: Alignment.bottomRight,
              ),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(
                color: AppColors.primary.withOpacity(0.2),
                width: 1,
              ),
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(
                  Icons.timer_outlined,
                  color: AppColors.primary,
                  size: 20,
                ),
                const SizedBox(width: 8),
                Text(
                  'باقي ',
                  style: AppTextStyles.bodyLarge.copyWith(
                    color: AppColors.textSecondary,
                  ),
                ),
                Text(
                  _formatDuration(_timeRemaining),
                  style: AppTextStyles.h4.copyWith(
                    color: AppColors.primary,
                    fontWeight: FontWeight.bold,
                    fontFeatures: [const FontFeature.tabularFigures()],
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(height: 16),

          // Expand/Collapse button
          InkWell(
            onTap: () {
              setState(() {
                _isExpanded = !_isExpanded;
              });
            },
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text(
                  _isExpanded ? 'إخفاء المواقيت' : 'عرض جميع المواقيت',
                  style: AppTextStyles.bodySmall.copyWith(
                    color: AppColors.primary,
                    fontWeight: FontWeight.w600,
                  ),
                ),
                Icon(
                  _isExpanded
                      ? Icons.keyboard_arrow_up
                      : Icons.keyboard_arrow_down,
                  color: AppColors.primary,
                  size: 20,
                ),
              ],
            ),
          ),

          // All prayer times (expandable)
          AnimatedCrossFade(
            firstChild: const SizedBox.shrink(),
            secondChild: Column(
              children: [
                const SizedBox(height: 16),
                _buildPrayerTimesList(),
                const SizedBox(height: 8),
              ],
            ),
            crossFadeState: _isExpanded
                ? CrossFadeState.showSecond
                : CrossFadeState.showFirst,
            duration: const Duration(milliseconds: 300),
          ),

          // Location
          Row(
            children: [
              Icon(
                Icons.location_on_outlined,
                size: 16,
                color: AppColors.textSecondary,
              ),
              const SizedBox(width: 4),
              Text(
                widget.prayerTimes.location,
                style: AppTextStyles.caption.copyWith(
                  color: AppColors.textSecondary,
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildPrayerTimesList() {
    final prayers = [
      {
        'name': 'الفجر',
        'time': widget.prayerTimes.fajr,
        'icon': Icons.wb_twilight
      },
      {
        'name': 'الشروق',
        'time': widget.prayerTimes.sunrise,
        'icon': Icons.wb_sunny
      },
      {
        'name': 'الظهر',
        'time': widget.prayerTimes.dhuhr,
        'icon': Icons.wb_sunny_outlined
      },
      {
        'name': 'العصر',
        'time': widget.prayerTimes.asr,
        'icon': Icons.wb_cloudy
      },
      {
        'name': 'المغرب',
        'time': widget.prayerTimes.maghrib,
        'icon': Icons.wb_twilight
      },
      {
        'name': 'العشاء',
        'time': widget.prayerTimes.isha,
        'icon': Icons.nightlight
      },
    ];

    return Column(
      children: prayers.map((prayer) {
        final isNext = prayer['name'] == widget.prayerTimes.getNextPrayer()['name'];
        return Container(
          margin: const EdgeInsets.only(bottom: 8),
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
          decoration: BoxDecoration(
            color: isNext
                ? AppColors.accent.withOpacity(0.1)
                : AppColors.backgroundSecondary,
            borderRadius: BorderRadius.circular(8),
            border: isNext
                ? Border.all(color: AppColors.accent.withOpacity(0.3))
                : null,
          ),
          child: Row(
            children: [
              Icon(
                prayer['icon'] as IconData,
                size: 20,
                color: isNext ? AppColors.accent : AppColors.textSecondary,
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Text(
                  prayer['name'] as String,
                  style: AppTextStyles.bodyMedium.copyWith(
                    color: isNext ? AppColors.accent : AppColors.textPrimary,
                    fontWeight: isNext ? FontWeight.bold : FontWeight.normal,
                  ),
                ),
              ),
              Text(
                prayer['time'] as String,
                style: AppTextStyles.bodyMedium.copyWith(
                  color: isNext ? AppColors.accent : AppColors.textPrimary,
                  fontWeight: FontWeight.w600,
                  fontFeatures: [const FontFeature.tabularFigures()],
                ),
              ),
            ],
          ),
        );
      }).toList(),
    );
  }
}

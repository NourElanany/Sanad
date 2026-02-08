import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:share_plus/share_plus.dart';
import 'dart:io';
import 'package:path_provider/path_provider.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/providers/prayer_calendar_provider.dart';
import '../widgets/calendar_grid_widget.dart';
import '../widgets/calendar_header_widget.dart';
import '../widgets/calendar_day_details_sheet.dart';
import '../widgets/calendar_legend_widget.dart';
import '../../data/models/calendar_day_model.dart';

class PrayerCalendarScreen extends ConsumerStatefulWidget {
  const PrayerCalendarScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<PrayerCalendarScreen> createState() =>
      _PrayerCalendarScreenState();
}

class _PrayerCalendarScreenState extends ConsumerState<PrayerCalendarScreen> {
  @override
  void initState() {
    super.initState();
    _initializeCalendar();
  }

  Future<void> _initializeCalendar() async {
    // Get current location (you should implement proper location service)
    // For now, using default location (Makkah)
    final notifier = ref.read(monthlyCalendarProvider.notifier);
    notifier.setLocation(21.4224779, 39.8251832);

    // Load current Hijri month
    final now = DateTime.now();
    // Approximate Hijri date calculation (should use proper conversion)
    final hijriYear = 1445;
    final hijriMonth = 7; // Rajab

    await notifier.loadMonthlyCalendar(hijriYear, hijriMonth);
  }

  @override
  Widget build(BuildContext context) {
    final calendarState = ref.watch(monthlyCalendarProvider);

    return Scaffold(
      backgroundColor: AppColors.backgroundPrimary,
      appBar: AppBar(
        backgroundColor: AppColors.primary,
        elevation: 0,
        title: Text(
          'تقويم المواقيت',
          style: AppTextStyles.h5.copyWith(
            color: Colors.white,
            fontWeight: FontWeight.bold,
          ),
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.share, color: Colors.white),
            onPressed: () => _shareCalendar(),
          ),
          IconButton(
            icon: const Icon(Icons.download, color: Colors.white),
            onPressed: () => _exportCalendar(),
          ),
          IconButton(
            icon: const Icon(Icons.settings, color: Colors.white),
            onPressed: () => _showSettings(),
          ),
        ],
      ),
      body: calendarState.isLoading
          ? _buildLoadingState()
          : calendarState.error != null
              ? _buildErrorState(calendarState.error!)
              : calendarState.calendar == null
                  ? _buildEmptyState()
                  : _buildCalendarContent(calendarState.calendar!),
    );
  }

  Widget _buildLoadingState() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          CircularProgressIndicator(
            valueColor: AlwaysStoppedAnimation<Color>(AppColors.primary),
          ),
          const SizedBox(height: 16),
          Text(
            'جاري تحميل التقويم...',
            style: AppTextStyles.bodyLarge.copyWith(
              color: AppColors.textSecondary,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildErrorState(String error) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.error_outline,
              size: 64,
              color: AppColors.error,
            ),
            const SizedBox(height: 16),
            Text(
              'حدث خطأ',
              style: AppTextStyles.h5.copyWith(
                color: AppColors.textPrimary,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 8),
            Text(
              error,
              style: AppTextStyles.bodyMedium.copyWith(
                color: AppColors.textSecondary,
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 24),
            ElevatedButton(
              onPressed: () => _initializeCalendar(),
              style: ElevatedButton.styleFrom(
                backgroundColor: AppColors.primary,
                padding: const EdgeInsets.symmetric(
                  horizontal: 32,
                  vertical: 16,
                ),
              ),
              child: Text(
                'إعادة المحاولة',
                style: AppTextStyles.bodyLarge.copyWith(
                  color: Colors.white,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildEmptyState() {
    return Center(
      child: Text(
        'لا توجد بيانات',
        style: AppTextStyles.bodyLarge.copyWith(
          color: AppColors.textSecondary,
        ),
      ),
    );
  }

  Widget _buildCalendarContent(MonthlyCalendarModel calendar) {
    return RefreshIndicator(
      onRefresh: () async {
        await ref.read(monthlyCalendarProvider.notifier).loadMonthlyCalendar(
              calendar.hijriYear,
              calendar.hijriMonth.monthNumber,
            );
      },
      child: SingleChildScrollView(
        physics: const AlwaysScrollableScrollPhysics(),
        child: Column(
          children: [
            // Calendar Header with month navigation
            CalendarHeaderWidget(
              calendar: calendar,
              onPreviousMonth: () {
                ref.read(monthlyCalendarProvider.notifier).previousMonth();
              },
              onNextMonth: () {
                ref.read(monthlyCalendarProvider.notifier).nextMonth();
              },
            ),

            // Calendar Legend
            const CalendarLegendWidget(),

            // Calendar Grid
            CalendarGridWidget(
              calendar: calendar,
              onDayTap: (day) => _showDayDetails(day),
            ),

            // Events Summary
            if (calendar.events.isNotEmpty) _buildEventsSummary(calendar),

            const SizedBox(height: 24),
          ],
        ),
      ),
    );
  }

  Widget _buildEventsSummary(MonthlyCalendarModel calendar) {
    return Container(
      margin: const EdgeInsets.all(16),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: AppColors.backgroundSecondary,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: AppColors.primary.withOpacity(0.1),
        ),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(
                Icons.event,
                color: AppColors.accent,
                size: 20,
              ),
              const SizedBox(width: 8),
              Text(
                'المناسبات الإسلامية هذا الشهر',
                style: AppTextStyles.h6.copyWith(
                  color: AppColors.textPrimary,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),
          ...calendar.events.map((event) => Padding(
                padding: const EdgeInsets.only(bottom: 8),
                child: Row(
                  children: [
                    Container(
                      width: 8,
                      height: 8,
                      decoration: BoxDecoration(
                        color: event.isEid
                            ? AppColors.success
                            : event.isHolyNight
                                ? AppColors.accent
                                : AppColors.primary,
                        shape: BoxShape.circle,
                      ),
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: Text(
                        event.nameArabic,
                        style: AppTextStyles.bodyMedium.copyWith(
                          color: AppColors.textPrimary,
                        ),
                      ),
                    ),
                  ],
                ),
              )),
        ],
      ),
    );
  }

  void _showDayDetails(CalendarDayModel day) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (context) => CalendarDayDetailsSheet(day: day),
    );
  }

  Future<void> _shareCalendar() async {
    try {
      final notifier = ref.read(monthlyCalendarProvider.notifier);
      final shareUrl = await notifier.getShareableLink();

      await Share.share(
        'تقويم المواقيت الشهري\n$shareUrl',
        subject: 'تقويم المواقيت',
      );
    } catch (e) {
      _showSnackBar('فشل مشاركة التقويم: $e');
    }
  }

  Future<void> _exportCalendar() async {
    try {
      final notifier = ref.read(monthlyCalendarProvider.notifier);
      final icalData = await notifier.exportToICal();

      // Save to file
      final directory = await getApplicationDocumentsDirectory();
      final file = File('${directory.path}/prayer_calendar.ics');
      await file.writeAsString(icalData);

      // Share the file
      await Share.shareXFiles(
        [XFile(file.path)],
        subject: 'تقويم المواقيت',
        text: 'تقويم المواقيت الشهري بصيغة iCal',
      );

      _showSnackBar('تم تصدير التقويم بنجاح');
    } catch (e) {
      _showSnackBar('فشل تصدير التقويم: $e');
    }
  }

  void _showSettings() {
    showModalBottomSheet(
      context: context,
      builder: (context) => Container(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'إعدادات التقويم',
              style: AppTextStyles.h5.copyWith(
                color: AppColors.textPrimary,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 24),
            ListTile(
              leading: Icon(Icons.calculate, color: AppColors.primary),
              title: const Text('طريقة الحساب'),
              subtitle: const Text('رابطة العالم الإسلامي'),
              trailing: const Icon(Icons.chevron_right),
              onTap: () {
                // Show calculation method selector
              },
            ),
            ListTile(
              leading: Icon(Icons.notifications, color: AppColors.primary),
              title: const Text('التنبيهات'),
              subtitle: const Text('إدارة تنبيهات المواقيت'),
              trailing: const Icon(Icons.chevron_right),
              onTap: () {
                // Navigate to notification settings
              },
            ),
            ListTile(
              leading: Icon(Icons.location_on, color: AppColors.primary),
              title: const Text('الموقع'),
              subtitle: const Text('مكة المكرمة، السعودية'),
              trailing: const Icon(Icons.chevron_right),
              onTap: () {
                // Show location selector
              },
            ),
          ],
        ),
      ),
    );
  }

  void _showSnackBar(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: AppColors.primary,
      ),
    );
  }
}

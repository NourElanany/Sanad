import 'package:flutter/material.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../data/models/calendar_day_model.dart';

class CalendarHeaderWidget extends StatelessWidget {
  final MonthlyCalendarModel calendar;
  final VoidCallback onPreviousMonth;
  final VoidCallback onNextMonth;

  const CalendarHeaderWidget({
    Key? key,
    required this.calendar,
    required this.onPreviousMonth,
    required this.onNextMonth,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: [
            AppColors.primary,
            AppColors.primary.withOpacity(0.8),
          ],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
      ),
      child: Column(
        children: [
          // Month navigation
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              IconButton(
                icon: const Icon(Icons.chevron_right, color: Colors.white),
                onPressed: onPreviousMonth,
              ),
              Expanded(
                child: Column(
                  children: [
                    Text(
                      calendar.hijriMonth.nameArabic,
                      style: AppTextStyles.h4.copyWith(
                        color: Colors.white,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      '${calendar.hijriYear} هـ',
                      style: AppTextStyles.bodyLarge.copyWith(
                        color: Colors.white.withOpacity(0.9),
                      ),
                    ),
                  ],
                ),
              ),
              IconButton(
                icon: const Icon(Icons.chevron_left, color: Colors.white),
                onPressed: onNextMonth,
              ),
            ],
          ),

          const SizedBox(height: 16),

          // Weekday headers
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              _buildWeekdayHeader('السبت'),
              _buildWeekdayHeader('الأحد'),
              _buildWeekdayHeader('الاثنين'),
              _buildWeekdayHeader('الثلاثاء'),
              _buildWeekdayHeader('الأربعاء'),
              _buildWeekdayHeader('الخميس'),
              _buildWeekdayHeader('الجمعة'),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildWeekdayHeader(String day) {
    return Expanded(
      child: Center(
        child: Text(
          day,
          style: AppTextStyles.caption.copyWith(
            color: Colors.white.withOpacity(0.9),
            fontWeight: FontWeight.w600,
          ),
        ),
      ),
    );
  }
}

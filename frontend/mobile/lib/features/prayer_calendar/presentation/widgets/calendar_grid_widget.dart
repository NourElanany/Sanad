import 'package:flutter/material.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../data/models/calendar_day_model.dart';

class CalendarGridWidget extends StatelessWidget {
  final MonthlyCalendarModel calendar;
  final Function(CalendarDayModel) onDayTap;

  const CalendarGridWidget({
    Key? key,
    required this.calendar,
    required this.onDayTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    // Group days by week
    final weeks = _groupDaysByWeek(calendar.days);

    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 8),
      child: Column(
        children: weeks.map((week) => _buildWeekRow(week)).toList(),
      ),
    );
  }

  List<List<CalendarDayModel?>> _groupDaysByWeek(List<CalendarDayModel> days) {
    final weeks = <List<CalendarDayModel?>>[];
    var currentWeek = <CalendarDayModel?>[];

    // Add empty cells for days before the first day of the month
    if (days.isNotEmpty) {
      final firstDayWeekday = days.first.gregorianDate.weekday;
      // Saturday = 6, Sunday = 7 in Dart, we want Saturday = 0
      final emptyCells = firstDayWeekday == 7 ? 1 : firstDayWeekday + 1;
      for (var i = 0; i < emptyCells; i++) {
        currentWeek.add(null);
      }
    }

    // Add all days
    for (var day in days) {
      currentWeek.add(day);
      if (currentWeek.length == 7) {
        weeks.add(List.from(currentWeek));
        currentWeek.clear();
      }
    }

    // Add empty cells for remaining days in the last week
    if (currentWeek.isNotEmpty) {
      while (currentWeek.length < 7) {
        currentWeek.add(null);
      }
      weeks.add(currentWeek);
    }

    return weeks;
  }

  Widget _buildWeekRow(List<CalendarDayModel?> week) {
    return Row(
      children: week.map((day) => _buildDayCell(day)).toList(),
    );
  }

  Widget _buildDayCell(CalendarDayModel? day) {
    if (day == null) {
      return Expanded(
        child: Container(
          height: 80,
          margin: const EdgeInsets.all(2),
        ),
      );
    }

    return Expanded(
      child: GestureDetector(
        onTap: () => onDayTap(day),
        child: Container(
          height: 80,
          margin: const EdgeInsets.all(2),
          decoration: BoxDecoration(
            color: _getDayBackgroundColor(day),
            borderRadius: BorderRadius.circular(8),
            border: Border.all(
              color: day.isToday
                  ? AppColors.accent
                  : AppColors.primary.withOpacity(0.1),
              width: day.isToday ? 2 : 1,
            ),
          ),
          child: Stack(
            children: [
              // Event indicator
              if (day.hasEvents)
                Positioned(
                  top: 4,
                  right: 4,
                  child: Container(
                    width: 6,
                    height: 6,
                    decoration: BoxDecoration(
                      color: day.events.any((e) => e.isEid)
                          ? AppColors.success
                          : AppColors.accent,
                      shape: BoxShape.circle,
                    ),
                  ),
                ),

              // Day content
              Padding(
                padding: const EdgeInsets.all(4),
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    // Hijri day
                    Text(
                      '${day.hijriDate.day}',
                      style: AppTextStyles.h6.copyWith(
                        color: _getDayTextColor(day),
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 2),
                    // Gregorian day
                    Text(
                      '${day.gregorianDate.day}',
                      style: AppTextStyles.caption.copyWith(
                        color: _getDayTextColor(day).withOpacity(0.7),
                        fontSize: 10,
                      ),
                    ),
                    // Prayer time indicator
                    if (day.prayerTimes.fajr.isNotEmpty)
                      Padding(
                        padding: const EdgeInsets.only(top: 2),
                        child: Icon(
                          Icons.mosque,
                          size: 12,
                          color: _getDayTextColor(day).withOpacity(0.5),
                        ),
                      ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Color _getDayBackgroundColor(CalendarDayModel day) {
    if (day.isToday) {
      return AppColors.primary.withOpacity(0.1);
    }
    if (day.isFriday) {
      return AppColors.secondary.withOpacity(0.1);
    }
    if (day.hasEvents && day.events.any((e) => e.isEid)) {
      return AppColors.success.withOpacity(0.1);
    }
    if (day.hasEvents) {
      return AppColors.accent.withOpacity(0.05);
    }
    return AppColors.backgroundPaper;
  }

  Color _getDayTextColor(CalendarDayModel day) {
    if (day.isToday) {
      return AppColors.primary;
    }
    if (day.isFriday) {
      return AppColors.secondary;
    }
    if (day.hasEvents && day.events.any((e) => e.isEid)) {
      return AppColors.success;
    }
    return AppColors.textPrimary;
  }
}

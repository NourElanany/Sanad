import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';
import '../../data/models/statistics_model.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Chart displaying daily reading minutes
class ReadingMinutesChart extends StatelessWidget {
  final ReadingStatistics readingStats;

  const ReadingMinutesChart({
    Key? key,
    required this.readingStats,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Summary Stats
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              _buildStatItem(
                'اليوم',
                '${readingStats.totalMinutesToday} د',
                Icons.today,
                AppColors.primaryMain,
              ),
              _buildStatItem(
                'هذا الأسبوع',
                '${readingStats.totalMinutesWeek} د',
                Icons.date_range,
                AppColors.secondaryMain,
              ),
              _buildStatItem(
                'المتوسط اليومي',
                '${readingStats.averageDailyMinutes.toStringAsFixed(0)} د',
                Icons.trending_up,
                AppColors.accentGold,
              ),
            ],
          ),
          const SizedBox(height: 24),

          // Line Chart
          if (readingStats.dailyReadingHistory.isNotEmpty) ...[
            Text(
              'آخر 7 أيام',
              style: AppTextStyles.subtitle2.copyWith(
                color: AppColors.textPrimary,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 16),
            SizedBox(
              height: 200,
              child: LineChart(
                LineChartData(
                  gridData: FlGridData(
                    show: true,
                    drawVerticalLine: false,
                    horizontalInterval: 20,
                    getDrawingHorizontalLine: (value) {
                      return FlLine(
                        color: AppColors.textDisabled.withOpacity(0.2),
                        strokeWidth: 1,
                      );
                    },
                  ),
                  titlesData: FlTitlesData(
                    show: true,
                    bottomTitles: AxisTitles(
                      sideTitles: SideTitles(
                        showTitles: true,
                        reservedSize: 30,
                        getTitlesWidget: (value, meta) {
                          if (value.toInt() >= 0 && 
                              value.toInt() < readingStats.dailyReadingHistory.length) {
                            final data = readingStats.dailyReadingHistory[value.toInt()];
                            final weekday = _getArabicWeekday(data.date.weekday);
                            return Padding(
                              padding: const EdgeInsets.only(top: 8),
                              child: Text(
                                weekday,
                                style: AppTextStyles.caption.copyWith(
                                  color: AppColors.textSecondary,
                                ),
                              ),
                            );
                          }
                          return const SizedBox.shrink();
                        },
                      ),
                    ),
                    leftTitles: AxisTitles(
                      sideTitles: SideTitles(
                        showTitles: true,
                        reservedSize: 40,
                        getTitlesWidget: (value, meta) {
                          return Text(
                            value.toInt().toString(),
                            style: AppTextStyles.caption.copyWith(
                              color: AppColors.textSecondary,
                            ),
                          );
                        },
                      ),
                    ),
                    topTitles: AxisTitles(
                      sideTitles: SideTitles(showTitles: false),
                    ),
                    rightTitles: AxisTitles(
                      sideTitles: SideTitles(showTitles: false),
                    ),
                  ),
                  borderData: FlBorderData(show: false),
                  minX: 0,
                  maxX: (readingStats.dailyReadingHistory.length - 1).toDouble(),
                  minY: 0,
                  maxY: readingStats.dailyReadingHistory
                      .map((e) => e.minutes.toDouble())
                      .reduce((a, b) => a > b ? a : b) * 1.2,
                  lineBarsData: [
                    LineChartBarData(
                      spots: readingStats.dailyReadingHistory
                          .asMap()
                          .entries
                          .map((entry) {
                        return FlSpot(
                          entry.key.toDouble(),
                          entry.value.minutes.toDouble(),
                        );
                      }).toList(),
                      isCurved: true,
                      color: AppColors.primaryMain,
                      barWidth: 3,
                      isStrokeCapRound: true,
                      dotData: FlDotData(
                        show: true,
                        getDotPainter: (spot, percent, barData, index) {
                          return FlDotCirclePainter(
                            radius: 4,
                            color: AppColors.primaryMain,
                            strokeWidth: 2,
                            strokeColor: Colors.white,
                          );
                        },
                      ),
                      belowBarData: BarAreaData(
                        show: true,
                        color: AppColors.primaryMain.withOpacity(0.1),
                      ),
                    ),
                  ],
                  lineTouchData: LineTouchData(
                    enabled: true,
                    touchTooltipData: LineTouchTooltipData(
                      tooltipBgColor: AppColors.primaryMain.withOpacity(0.9),
                      getTooltipItems: (touchedSpots) {
                        return touchedSpots.map((spot) {
                          final data = readingStats.dailyReadingHistory[spot.x.toInt()];
                          return LineTooltipItem(
                            '${data.minutes} دقيقة\n',
                            AppTextStyles.body2.copyWith(
                              color: Colors.white,
                              fontWeight: FontWeight.bold,
                            ),
                            children: [
                              TextSpan(
                                text: '${data.pagesRead} صفحة',
                                style: AppTextStyles.caption.copyWith(
                                  color: Colors.white70,
                                ),
                              ),
                            ],
                          );
                        }).toList();
                      },
                    ),
                  ),
                ),
              ),
            ),
          ] else ...[
            Center(
              child: Padding(
                padding: const EdgeInsets.all(32),
                child: Column(
                  children: [
                    Icon(
                      Icons.schedule_outlined,
                      size: 64,
                      color: AppColors.textDisabled,
                    ),
                    const SizedBox(height: 16),
                    Text(
                      'لا توجد بيانات قراءة بعد',
                      style: AppTextStyles.body1.copyWith(
                        color: AppColors.textSecondary,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildStatItem(String label, String value, IconData icon, Color color) {
    return Column(
      children: [
        Icon(icon, color: color, size: 28),
        const SizedBox(height: 8),
        Text(
          value,
          style: AppTextStyles.subtitle1.copyWith(
            color: AppColors.textPrimary,
            fontWeight: FontWeight.bold,
          ),
        ),
        Text(
          label,
          style: AppTextStyles.caption.copyWith(
            color: AppColors.textSecondary,
          ),
        ),
      ],
    );
  }

  String _getArabicWeekday(int weekday) {
    const weekdays = [
      'الإثنين',
      'الثلاثاء',
      'الأربعاء',
      'الخميس',
      'الجمعة',
      'السبت',
      'الأحد',
    ];
    return weekdays[weekday - 1];
  }
}

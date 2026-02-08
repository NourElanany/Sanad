import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';
import '../../data/models/statistics_model.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Chart displaying Khatma completion history
class KhatmaCompletionChart extends StatelessWidget {
  final KhatmaStatistics khatmaStats;

  const KhatmaCompletionChart({
    Key? key,
    required this.khatmaStats,
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
                'المكتملة',
                khatmaStats.totalCompleted.toString(),
                Icons.check_circle,
                AppColors.statusSuccess,
              ),
              _buildStatItem(
                'التقدم الحالي',
                '${khatmaStats.currentProgress}%',
                Icons.trending_up,
                AppColors.primaryMain,
              ),
              _buildStatItem(
                'أطول سلسلة',
                '${khatmaStats.longestStreak} يوم',
                Icons.local_fire_department,
                AppColors.accentGold,
              ),
            ],
          ),
          const SizedBox(height: 24),

          // Bar Chart
          if (khatmaStats.completionHistory.isNotEmpty) ...[
            Text(
              'سجل الإنجاز',
              style: AppTextStyles.subtitle2.copyWith(
                color: AppColors.textPrimary,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 16),
            SizedBox(
              height: 200,
              child: BarChart(
                BarChartData(
                  alignment: BarChartAlignment.spaceAround,
                  maxY: khatmaStats.completionHistory
                      .map((e) => e.durationDays.toDouble())
                      .reduce((a, b) => a > b ? a : b) * 1.2,
                  barTouchData: BarTouchData(
                    enabled: true,
                    touchTooltipData: BarTouchTooltipData(
                      tooltipBgColor: AppColors.primaryMain.withOpacity(0.9),
                      getTooltipItem: (group, groupIndex, rod, rodIndex) {
                        final completion = khatmaStats.completionHistory[groupIndex];
                        return BarTooltipItem(
                          '${completion.durationDays} يوم\n',
                          AppTextStyles.body2.copyWith(
                            color: Colors.white,
                            fontWeight: FontWeight.bold,
                          ),
                          children: [
                            TextSpan(
                              text: 'الاتساق: ${(completion.consistencyScore * 100).toStringAsFixed(0)}%',
                              style: AppTextStyles.caption.copyWith(
                                color: Colors.white70,
                              ),
                            ),
                          ],
                        );
                      },
                    ),
                  ),
                  titlesData: FlTitlesData(
                    show: true,
                    bottomTitles: AxisTitles(
                      sideTitles: SideTitles(
                        showTitles: true,
                        getTitlesWidget: (value, meta) {
                          if (value.toInt() >= 0 && 
                              value.toInt() < khatmaStats.completionHistory.length) {
                            final completion = khatmaStats.completionHistory[value.toInt()];
                            return Padding(
                              padding: const EdgeInsets.only(top: 8),
                              child: Text(
                                '${completion.completionDate.month}/${completion.completionDate.day}',
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
                  gridData: FlGridData(
                    show: true,
                    drawVerticalLine: false,
                    horizontalInterval: 10,
                    getDrawingHorizontalLine: (value) {
                      return FlLine(
                        color: AppColors.textDisabled.withOpacity(0.2),
                        strokeWidth: 1,
                      );
                    },
                  ),
                  borderData: FlBorderData(show: false),
                  barGroups: khatmaStats.completionHistory
                      .asMap()
                      .entries
                      .map((entry) {
                    return BarChartGroupData(
                      x: entry.key,
                      barRods: [
                        BarChartRodData(
                          toY: entry.value.durationDays.toDouble(),
                          color: _getBarColor(entry.value.consistencyScore),
                          width: 16,
                          borderRadius: const BorderRadius.vertical(
                            top: Radius.circular(4),
                          ),
                        ),
                      ],
                    );
                  }).toList(),
                ),
              ),
            ),
            const SizedBox(height: 16),
            _buildLegend(),
          ] else ...[
            Center(
              child: Padding(
                padding: const EdgeInsets.all(32),
                child: Column(
                  children: [
                    Icon(
                      Icons.book_outlined,
                      size: 64,
                      color: AppColors.textDisabled,
                    ),
                    const SizedBox(height: 16),
                    Text(
                      'لا توجد ختمات مكتملة بعد',
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
        Icon(icon, color: color, size: 32),
        const SizedBox(height: 8),
        Text(
          value,
          style: AppTextStyles.h6.copyWith(
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

  Widget _buildLegend() {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        _buildLegendItem('ممتاز', AppColors.statusSuccess),
        const SizedBox(width: 16),
        _buildLegendItem('جيد', AppColors.accentGold),
        const SizedBox(width: 16),
        _buildLegendItem('متوسط', AppColors.statusWarning),
      ],
    );
  }

  Widget _buildLegendItem(String label, Color color) {
    return Row(
      children: [
        Container(
          width: 12,
          height: 12,
          decoration: BoxDecoration(
            color: color,
            borderRadius: BorderRadius.circular(2),
          ),
        ),
        const SizedBox(width: 4),
        Text(
          label,
          style: AppTextStyles.caption.copyWith(
            color: AppColors.textSecondary,
          ),
        ),
      ],
    );
  }

  Color _getBarColor(double consistencyScore) {
    if (consistencyScore >= 0.8) {
      return AppColors.statusSuccess;
    } else if (consistencyScore >= 0.6) {
      return AppColors.accentGold;
    } else {
      return AppColors.statusWarning;
    }
  }
}

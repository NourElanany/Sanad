import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';
import '../../data/models/statistics_model.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Chart displaying recitation improvement over time
class RecitationImprovementChart extends StatelessWidget {
  final RecitationStatistics recitationStats;

  const RecitationImprovementChart({
    Key? key,
    required this.recitationStats,
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
                'النتيجة الحالية',
                '${recitationStats.currentScore.toStringAsFixed(1)}%',
                Icons.star,
                AppColors.accentGold,
              ),
              _buildStatItem(
                'التحسن',
                '${recitationStats.improvementPercentage >= 0 ? '+' : ''}${recitationStats.improvementPercentage.toStringAsFixed(1)}%',
                Icons.trending_up,
                recitationStats.improvementPercentage >= 0 
                    ? AppColors.statusSuccess 
                    : AppColors.statusError,
              ),
              _buildStatItem(
                'التلاوات',
                recitationStats.totalRecitations.toString(),
                Icons.mic,
                AppColors.primaryMain,
              ),
            ],
          ),
          const SizedBox(height: 24),

          // Score History Line Chart
          if (recitationStats.scoreHistory.isNotEmpty) ...[
            Text(
              'تطور الأداء',
              style: AppTextStyles.subtitle2.copyWith(
                color: AppColors.textPrimary,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 16),
            SizedBox(
              height: 180,
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
                              value.toInt() < recitationStats.scoreHistory.length) {
                            return Padding(
                              padding: const EdgeInsets.only(top: 8),
                              child: Text(
                                '${value.toInt() + 1}',
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
                            '${value.toInt()}%',
                            style: AppTextStyles.caption.copyWith(
                              color: AppColors.textSecondary,
                            ),
                          );
                        },
                      ),
                    ),
                    topTitles: AxisTitles(sideTitles: SideTitles(showTitles: false)),
                    rightTitles: AxisTitles(sideTitles: SideTitles(showTitles: false)),
                  ),
                  borderData: FlBorderData(show: false),
                  minX: 0,
                  maxX: (recitationStats.scoreHistory.length - 1).toDouble(),
                  minY: 0,
                  maxY: 100,
                  lineBarsData: [
                    LineChartBarData(
                      spots: recitationStats.scoreHistory
                          .asMap()
                          .entries
                          .map((entry) {
                        return FlSpot(
                          entry.key.toDouble(),
                          entry.value.score,
                        );
                      }).toList(),
                      isCurved: true,
                      color: AppColors.accentGold,
                      barWidth: 3,
                      isStrokeCapRound: true,
                      dotData: FlDotData(
                        show: true,
                        getDotPainter: (spot, percent, barData, index) {
                          return FlDotCirclePainter(
                            radius: 4,
                            color: AppColors.accentGold,
                            strokeWidth: 2,
                            strokeColor: Colors.white,
                          );
                        },
                      ),
                      belowBarData: BarAreaData(
                        show: true,
                        color: AppColors.accentGold.withOpacity(0.1),
                      ),
                    ),
                  ],
                  lineTouchData: LineTouchData(
                    enabled: true,
                    touchTooltipData: LineTouchTooltipData(
                      tooltipBgColor: AppColors.accentGold.withOpacity(0.9),
                      getTooltipItems: (touchedSpots) {
                        return touchedSpots.map((spot) {
                          final data = recitationStats.scoreHistory[spot.x.toInt()];
                          return LineTooltipItem(
                            '${data.score.toStringAsFixed(1)}%\n',
                            AppTextStyles.body2.copyWith(
                              color: Colors.white,
                              fontWeight: FontWeight.bold,
                            ),
                            children: [
                              TextSpan(
                                text: data.surahName,
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
            const SizedBox(height: 16),

            // Top Improvement Areas
            if (recitationStats.topImprovementAreas.isNotEmpty) ...[
              Text(
                'مجالات التحسين',
                style: AppTextStyles.subtitle2.copyWith(
                  color: AppColors.textPrimary,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 8),
              ...recitationStats.topImprovementAreas.take(3).map((area) {
                return Padding(
                  padding: const EdgeInsets.only(bottom: 8),
                  child: Row(
                    children: [
                      Icon(
                        Icons.arrow_forward,
                        size: 16,
                        color: AppColors.primaryMain,
                      ),
                      const SizedBox(width: 8),
                      Text(
                        area,
                        style: AppTextStyles.body2.copyWith(
                          color: AppColors.textSecondary,
                        ),
                      ),
                    ],
                  ),
                );
              }).toList(),
            ],
          ] else ...[
            Center(
              child: Padding(
                padding: const EdgeInsets.all(32),
                child: Column(
                  children: [
                    Icon(
                      Icons.mic_none,
                      size: 64,
                      color: AppColors.textDisabled,
                    ),
                    const SizedBox(height: 16),
                    Text(
                      'لا توجد تلاوات مسجلة بعد',
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
}

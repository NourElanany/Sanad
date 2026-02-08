import 'package:flutter/material.dart';

/// Progress chart widget for displaying improvement over time
/// This is a placeholder - full implementation would use a charting library
class ProgressChart extends StatelessWidget {
  final List<double> scores;
  final List<DateTime> dates;

  const ProgressChart({
    Key? key,
    required this.scores,
    required this.dates,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 200,
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: const Color(0xFF1B365D).withOpacity(0.1),
          width: 1,
        ),
      ),
      child: const Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.show_chart,
              size: 48,
              color: Color(0xFFB8860B),
            ),
            SizedBox(height: 8),
            Text(
              'رسم بياني للتقدم',
              style: TextStyle(
                fontSize: 16,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
                color: Color(0xFF1A1A1A),
              ),
            ),
            SizedBox(height: 4),
            Text(
              'سيتم عرض تقدمك عبر الوقت هنا',
              style: TextStyle(
                fontSize: 14,
                fontFamily: 'Tajawal',
                color: Color(0xFF666666),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

import 'package:flutter/material.dart';

/// Score card widget for displaying individual scores
class ScoreCard extends StatelessWidget {
  final String label;
  final double score;
  final IconData icon;

  const ScoreCard({
    Key? key,
    required this.label,
    required this.score,
    required this.icon,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: _getScoreColor(score).withOpacity(0.3),
          width: 1.5,
        ),
      ),
      child: Row(
        children: [
          // Icon
          Container(
            padding: const EdgeInsets.all(10),
            decoration: BoxDecoration(
              color: _getScoreColor(score).withOpacity(0.1),
              borderRadius: BorderRadius.circular(10),
            ),
            child: Icon(
              icon,
              color: _getScoreColor(score),
              size: 24,
            ),
          ),
          const SizedBox(width: 12),

          // Label and progress bar
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  label,
                  style: const TextStyle(
                    fontSize: 14,
                    fontWeight: FontWeight.w600,
                    fontFamily: 'Tajawal',
                    color: Color(0xFF1A1A1A),
                  ),
                ),
                const SizedBox(height: 8),
                ClipRRect(
                  borderRadius: BorderRadius.circular(4),
                  child: LinearProgressIndicator(
                    value: score / 100,
                    minHeight: 8,
                    backgroundColor: Colors.grey.shade200,
                    valueColor: AlwaysStoppedAnimation<Color>(
                      _getScoreColor(score),
                    ),
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(width: 12),

          // Score value
          Text(
            '${score.toStringAsFixed(0)}%',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
              color: _getScoreColor(score),
            ),
          ),
        ],
      ),
    );
  }

  Color _getScoreColor(double score) {
    if (score >= 90) return const Color(0xFF28A745); // Excellent
    if (score >= 80) return const Color(0xFF2D5A27); // Good
    if (score >= 70) return const Color(0xFFB8860B); // Fair
    if (score >= 60) return const Color(0xFFFFC107); // Needs Work
    return const Color(0xFFDC3545); // Poor
  }
}

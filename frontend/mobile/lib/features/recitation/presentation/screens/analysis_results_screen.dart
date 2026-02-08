import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../data/models/recording_models.dart';
import '../widgets/score_card.dart';
import '../widgets/error_list.dart';
import '../widgets/recommendation_card.dart';
import '../widgets/progress_chart.dart';

/// Analysis results display screen
class AnalysisResultsScreen extends ConsumerWidget {
  final RecitationAnalysis analysis;
  final RecordingMetadata metadata;

  const AnalysisResultsScreen({
    Key? key,
    required this.analysis,
    required this.metadata,
  }) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      backgroundColor: const Color(0xFFFEFEFE),
      appBar: AppBar(
        title: const Text(
          'نتائج التحليل',
          style: TextStyle(
            fontFamily: 'Tajawal',
            fontWeight: FontWeight.bold,
          ),
        ),
        backgroundColor: const Color(0xFF1B365D),
        foregroundColor: Colors.white,
        elevation: 0,
        actions: [
          IconButton(
            icon: const Icon(Icons.share),
            onPressed: () => _shareResults(context),
            tooltip: 'مشاركة النتائج',
          ),
        ],
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Overall score card
            _buildOverallScoreCard(),
            const SizedBox(height: 20),

            // Recording info
            _buildRecordingInfo(),
            const SizedBox(height: 20),

            // Detailed scores
            _buildDetailedScores(),
            const SizedBox(height: 20),

            // Errors section
            if (analysis.errors.isNotEmpty) ...[
              _buildSectionHeader('الأخطاء المكتشفة', analysis.errors.length),
              const SizedBox(height: 12),
              TajweedErrorList(errors: analysis.errors),
              const SizedBox(height: 20),
            ],

            // Recommendations section
            if (analysis.recommendations.isNotEmpty) ...[
              _buildSectionHeader('توصيات التحسين', analysis.recommendations.length),
              const SizedBox(height: 12),
              ...analysis.recommendations.map(
                (rec) => Padding(
                  padding: const EdgeInsets.only(bottom: 12),
                  child: RecommendationCard(recommendation: rec),
                ),
              ),
              const SizedBox(height: 20),
            ],

            // Action buttons
            _buildActionButtons(context),
          ],
        ),
      ),
    );
  }

  Widget _buildOverallScoreCard() {
    return Container(
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: [
            const Color(0xFF1B365D),
            const Color(0xFF2E4A6B),
          ],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
        borderRadius: BorderRadius.circular(20),
        boxShadow: [
          BoxShadow(
            color: const Color(0xFF1B365D).withOpacity(0.3),
            blurRadius: 16,
            offset: const Offset(0, 8),
          ),
        ],
      ),
      child: Column(
        children: [
          const Text(
            'النتيجة الإجمالية',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.w600,
              fontFamily: 'Tajawal',
              color: Colors.white,
            ),
          ),
          const SizedBox(height: 16),
          Stack(
            alignment: Alignment.center,
            children: [
              SizedBox(
                width: 140,
                height: 140,
                child: CircularProgressIndicator(
                  value: analysis.overallScore / 100,
                  strokeWidth: 12,
                  backgroundColor: Colors.white.withOpacity(0.2),
                  valueColor: AlwaysStoppedAnimation<Color>(
                    _getScoreColor(analysis.overallScore),
                  ),
                ),
              ),
              Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    '${analysis.overallScore.toStringAsFixed(1)}%',
                    style: const TextStyle(
                      fontSize: 36,
                      fontWeight: FontWeight.bold,
                      fontFamily: 'Tajawal',
                      color: Colors.white,
                    ),
                  ),
                  Text(
                    _getScoreLabel(analysis.overallScore),
                    style: TextStyle(
                      fontSize: 14,
                      fontFamily: 'Tajawal',
                      color: Colors.white.withOpacity(0.9),
                    ),
                  ),
                ],
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _buildRecordingInfo() {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: const Color(0xFF1B365D).withOpacity(0.1),
          width: 1,
        ),
      ),
      child: Column(
        children: [
          _buildInfoRow(
            Icons.book,
            'السورة',
            'سورة رقم ${metadata.surahNumber}',
          ),
          const Divider(height: 24),
          _buildInfoRow(
            Icons.format_list_numbered,
            'الآيات',
            'من ${metadata.ayahStart} إلى ${metadata.ayahEnd}',
          ),
          const Divider(height: 24),
          _buildInfoRow(
            Icons.access_time,
            'المدة',
            _formatDuration(metadata.duration),
          ),
          const Divider(height: 24),
          _buildInfoRow(
            Icons.calendar_today,
            'التاريخ',
            _formatDate(analysis.analyzedAt),
          ),
        ],
      ),
    );
  }

  Widget _buildInfoRow(IconData icon, String label, String value) {
    return Row(
      children: [
        Container(
          padding: const EdgeInsets.all(8),
          decoration: BoxDecoration(
            color: const Color(0xFF1B365D).withOpacity(0.1),
            borderRadius: BorderRadius.circular(8),
          ),
          child: Icon(
            icon,
            color: const Color(0xFF1B365D),
            size: 20,
          ),
        ),
        const SizedBox(width: 12),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                label,
                style: const TextStyle(
                  fontSize: 12,
                  fontFamily: 'Tajawal',
                  color: Color(0xFF666666),
                ),
              ),
              const SizedBox(height: 2),
              Text(
                value,
                style: const TextStyle(
                  fontSize: 14,
                  fontWeight: FontWeight.w600,
                  fontFamily: 'Tajawal',
                  color: Color(0xFF1A1A1A),
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildDetailedScores() {
    final scores = analysis.detailedScores;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'التقييم التفصيلي',
          style: TextStyle(
            fontSize: 18,
            fontWeight: FontWeight.bold,
            fontFamily: 'Tajawal',
            color: Color(0xFF1B365D),
          ),
        ),
        const SizedBox(height: 12),
        ScoreCard(
          label: 'دقة النطق',
          score: scores.pronunciationAccuracy,
          icon: Icons.record_voice_over,
        ),
        const SizedBox(height: 8),
        ScoreCard(
          label: 'دقة التوقيت',
          score: scores.timingAccuracy,
          icon: Icons.timer,
        ),
        const SizedBox(height: 8),
        ScoreCard(
          label: 'التزام التجويد',
          score: scores.tajweedCompliance,
          icon: Icons.check_circle,
        ),
        const SizedBox(height: 8),
        ScoreCard(
          label: 'الطلاقة',
          score: scores.fluency,
          icon: Icons.trending_up,
        ),
        const SizedBox(height: 8),
        ScoreCard(
          label: 'الوضوح',
          score: scores.clarity,
          icon: Icons.hearing,
        ),
        const SizedBox(height: 8),
        ScoreCard(
          label: 'الإيقاع',
          score: scores.rhythm,
          icon: Icons.music_note,
        ),
      ],
    );
  }

  Widget _buildSectionHeader(String title, int count) {
    return Row(
      children: [
        Text(
          title,
          style: const TextStyle(
            fontSize: 18,
            fontWeight: FontWeight.bold,
            fontFamily: 'Tajawal',
            color: Color(0xFF1B365D),
          ),
        ),
        const SizedBox(width: 8),
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
          decoration: BoxDecoration(
            color: const Color(0xFFB8860B).withOpacity(0.2),
            borderRadius: BorderRadius.circular(12),
          ),
          child: Text(
            count.toString(),
            style: const TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
              color: Color(0xFFB8860B),
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildActionButtons(BuildContext context) {
    return Column(
      children: [
        ElevatedButton.icon(
          onPressed: () => _recordAgain(context),
          icon: const Icon(Icons.mic),
          label: const Text(
            'تسجيل مرة أخرى',
            style: TextStyle(
              fontFamily: 'Tajawal',
              fontSize: 16,
              fontWeight: FontWeight.bold,
            ),
          ),
          style: ElevatedButton.styleFrom(
            backgroundColor: const Color(0xFF2D5A27),
            foregroundColor: Colors.white,
            padding: const EdgeInsets.symmetric(vertical: 16),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(12),
            ),
          ),
        ),
        const SizedBox(height: 12),
        OutlinedButton.icon(
          onPressed: () => _viewProgress(context),
          icon: const Icon(Icons.show_chart),
          label: const Text(
            'عرض التقدم',
            style: TextStyle(
              fontFamily: 'Tajawal',
              fontSize: 16,
              fontWeight: FontWeight.bold,
            ),
          ),
          style: OutlinedButton.styleFrom(
            foregroundColor: const Color(0xFF1B365D),
            side: const BorderSide(
              color: Color(0xFF1B365D),
              width: 2,
            ),
            padding: const EdgeInsets.symmetric(vertical: 16),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(12),
            ),
          ),
        ),
      ],
    );
  }

  Color _getScoreColor(double score) {
    if (score >= 90) return const Color(0xFF28A745); // Excellent - Green
    if (score >= 80) return const Color(0xFF2D5A27); // Good - Dark Green
    if (score >= 70) return const Color(0xFFB8860B); // Fair - Gold
    if (score >= 60) return const Color(0xFFFFC107); // Needs Work - Yellow
    return const Color(0xFFDC3545); // Poor - Red
  }

  String _getScoreLabel(double score) {
    if (score >= 90) return 'ممتاز';
    if (score >= 80) return 'جيد جداً';
    if (score >= 70) return 'جيد';
    if (score >= 60) return 'مقبول';
    return 'يحتاج تحسين';
  }

  String _formatDuration(Duration duration) {
    final minutes = duration.inMinutes;
    final seconds = duration.inSeconds % 60;
    return '$minutes:${seconds.toString().padLeft(2, '0')}';
  }

  String _formatDate(DateTime date) {
    return '${date.day}/${date.month}/${date.year}';
  }

  void _shareResults(BuildContext context) {
    // TODO: Implement share functionality
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('مشاركة النتائج...')),
    );
  }

  void _recordAgain(BuildContext context) {
    Navigator.pop(context);
  }

  void _viewProgress(BuildContext context) {
    // TODO: Navigate to progress screen
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('عرض التقدم...')),
    );
  }
}

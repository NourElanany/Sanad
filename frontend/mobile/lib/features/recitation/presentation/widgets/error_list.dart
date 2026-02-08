import 'package:flutter/material.dart';
import '../../data/models/recording_models.dart';

/// List widget for displaying Tajweed errors
class TajweedErrorList extends StatelessWidget {
  final List<TajweedError> errors;

  const TajweedErrorList({
    Key? key,
    required this.errors,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      children: errors.map((error) => TajweedErrorCard(error: error)).toList(),
    );
  }
}

/// Card widget for displaying a single Tajweed error
class TajweedErrorCard extends StatefulWidget {
  final TajweedError error;

  const TajweedErrorCard({
    Key? key,
    required this.error,
  }) : super(key: key);

  @override
  State<TajweedErrorCard> createState() => _TajweedErrorCardState();
}

class _TajweedErrorCardState extends State<TajweedErrorCard> {
  bool _isExpanded = false;

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(bottom: 12),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: _getSeverityColor(widget.error.severity).withOpacity(0.3),
          width: 2,
        ),
      ),
      child: Column(
        children: [
          InkWell(
            onTap: () => setState(() => _isExpanded = !_isExpanded),
            borderRadius: BorderRadius.circular(12),
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                children: [
                  // Error icon
                  Container(
                    padding: const EdgeInsets.all(8),
                    decoration: BoxDecoration(
                      color: _getSeverityColor(widget.error.severity).withOpacity(0.1),
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Icon(
                      _getSeverityIcon(widget.error.severity),
                      color: _getSeverityColor(widget.error.severity),
                      size: 24,
                    ),
                  ),
                  const SizedBox(width: 12),

                  // Error info
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            Expanded(
                              child: Text(
                                _getErrorTypeArabic(widget.error.errorType),
                                style: const TextStyle(
                                  fontSize: 16,
                                  fontWeight: FontWeight.bold,
                                  fontFamily: 'Tajawal',
                                  color: Color(0xFF1A1A1A),
                                ),
                              ),
                            ),
                            Container(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 8,
                                vertical: 4,
                              ),
                              decoration: BoxDecoration(
                                color: _getSeverityColor(widget.error.severity).withOpacity(0.2),
                                borderRadius: BorderRadius.circular(8),
                              ),
                              child: Text(
                                _getSeverityLabel(widget.error.severity),
                                style: TextStyle(
                                  fontSize: 11,
                                  fontWeight: FontWeight.bold,
                                  fontFamily: 'Tajawal',
                                  color: _getSeverityColor(widget.error.severity),
                                ),
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 4),
                        Text(
                          widget.error.description,
                          style: const TextStyle(
                            fontSize: 14,
                            fontFamily: 'Tajawal',
                            color: Color(0xFF666666),
                          ),
                        ),
                        const SizedBox(height: 4),
                        Row(
                          children: [
                            const Icon(
                              Icons.access_time,
                              size: 14,
                              color: Color(0xFF999999),
                            ),
                            const SizedBox(width: 4),
                            Text(
                              _formatTimestamp(widget.error.timestamp),
                              style: const TextStyle(
                                fontSize: 12,
                                fontFamily: 'Tajawal',
                                color: Color(0xFF999999),
                              ),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ),
                  const SizedBox(width: 8),

                  // Expand icon
                  Icon(
                    _isExpanded ? Icons.expand_less : Icons.expand_more,
                    color: const Color(0xFF666666),
                  ),
                ],
              ),
            ),
          ),

          // Expanded content
          if (_isExpanded && widget.error.correction != null)
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: const Color(0xFF2D5A27).withOpacity(0.05),
                borderRadius: const BorderRadius.only(
                  bottomLeft: Radius.circular(12),
                  bottomRight: Radius.circular(12),
                ),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      const Icon(
                        Icons.lightbulb,
                        color: Color(0xFF2D5A27),
                        size: 20,
                      ),
                      const SizedBox(width: 8),
                      const Text(
                        'التصحيح',
                        style: TextStyle(
                          fontSize: 14,
                          fontWeight: FontWeight.bold,
                          fontFamily: 'Tajawal',
                          color: Color(0xFF2D5A27),
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 8),
                  Text(
                    widget.error.correction!,
                    style: const TextStyle(
                      fontSize: 14,
                      fontFamily: 'Tajawal',
                      color: Color(0xFF1A1A1A),
                    ),
                  ),
                  const SizedBox(height: 12),
                  ElevatedButton.icon(
                    onPressed: () => _playCorrectPronunciation(),
                    icon: const Icon(Icons.play_arrow, size: 18),
                    label: const Text(
                      'استماع للنطق الصحيح',
                      style: TextStyle(
                        fontFamily: 'Tajawal',
                        fontSize: 13,
                      ),
                    ),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: const Color(0xFF2D5A27),
                      foregroundColor: Colors.white,
                      padding: const EdgeInsets.symmetric(
                        horizontal: 16,
                        vertical: 8,
                      ),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(8),
                      ),
                    ),
                  ),
                ],
              ),
            ),
        ],
      ),
    );
  }

  Color _getSeverityColor(String severity) {
    switch (severity.toLowerCase()) {
      case 'high':
        return const Color(0xFFDC3545); // Red
      case 'medium':
        return const Color(0xFFFFC107); // Yellow
      case 'low':
        return const Color(0xFF17A2B8); // Blue
      default:
        return const Color(0xFF666666); // Gray
    }
  }

  IconData _getSeverityIcon(String severity) {
    switch (severity.toLowerCase()) {
      case 'high':
        return Icons.error;
      case 'medium':
        return Icons.warning;
      case 'low':
        return Icons.info;
      default:
        return Icons.help;
    }
  }

  String _getSeverityLabel(String severity) {
    switch (severity.toLowerCase()) {
      case 'high':
        return 'عالي';
      case 'medium':
        return 'متوسط';
      case 'low':
        return 'منخفض';
      default:
        return 'غير محدد';
    }
  }

  String _getErrorTypeArabic(String errorType) {
    // Map English error types to Arabic
    final Map<String, String> errorTypes = {
      'Ikhfa': 'إخفاء',
      'Iqlab': 'إقلاب',
      'Idgham': 'إدغام',
      'Izhar': 'إظهار',
      'Madd': 'مد',
      'Ghunna': 'غنة',
      'Qalqalah': 'قلقلة',
      'Tafkheem': 'تفخيم',
      'Tarqeeq': 'ترقيق',
    };

    return errorTypes[errorType] ?? errorType;
  }

  String _formatTimestamp(double timestamp) {
    final minutes = (timestamp / 60).floor();
    final seconds = (timestamp % 60).floor();
    return '$minutes:${seconds.toString().padLeft(2, '0')}';
  }

  void _playCorrectPronunciation() {
    // TODO: Implement audio playback
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('تشغيل النطق الصحيح...')),
    );
  }
}

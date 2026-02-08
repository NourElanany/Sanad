import 'package:flutter/material.dart';
import 'dart:math' as math;
import '../../data/models/recording_models.dart';

/// Real-time waveform visualizer widget
class WaveformVisualizer extends StatelessWidget {
  final WaveformData? waveformData;
  final Color waveColor;
  final Color backgroundColor;
  final double height;
  final bool showGrid;

  const WaveformVisualizer({
    Key? key,
    this.waveformData,
    this.waveColor = const Color(0xFFB8860B), // Gold
    this.backgroundColor = const Color(0xFF1B365D), // Navy
    this.height = 120,
    this.showGrid = true,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      height: height,
      decoration: BoxDecoration(
        color: backgroundColor.withOpacity(0.1),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: backgroundColor.withOpacity(0.3),
          width: 1,
        ),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(12),
        child: CustomPaint(
          painter: WaveformPainter(
            waveformData: waveformData,
            waveColor: waveColor,
            showGrid: showGrid,
          ),
          size: Size.infinite,
        ),
      ),
    );
  }
}

/// Custom painter for waveform
class WaveformPainter extends CustomPainter {
  final WaveformData? waveformData;
  final Color waveColor;
  final bool showGrid;

  WaveformPainter({
    this.waveformData,
    required this.waveColor,
    this.showGrid = true,
  });

  @override
  void paint(Canvas canvas, Size size) {
    // Draw grid if enabled
    if (showGrid) {
      _drawGrid(canvas, size);
    }

    // Draw waveform if data is available
    if (waveformData != null && waveformData!.amplitudes.isNotEmpty) {
      _drawWaveform(canvas, size);
    } else {
      _drawPlaceholder(canvas, size);
    }
  }

  void _drawGrid(Canvas canvas, Size size) {
    final gridPaint = Paint()
      ..color = Colors.grey.withOpacity(0.2)
      ..strokeWidth = 0.5;

    // Horizontal lines
    for (int i = 0; i <= 4; i++) {
      final y = size.height * i / 4;
      canvas.drawLine(
        Offset(0, y),
        Offset(size.width, y),
        gridPaint,
      );
    }

    // Vertical lines
    for (int i = 0; i <= 10; i++) {
      final x = size.width * i / 10;
      canvas.drawLine(
        Offset(x, 0),
        Offset(x, size.height),
        gridPaint,
      );
    }
  }

  void _drawWaveform(Canvas canvas, Size size) {
    final amplitudes = waveformData!.amplitudes;
    final paint = Paint()
      ..color = waveColor
      ..strokeWidth = 2
      ..strokeCap = StrokeCap.round;

    final centerY = size.height / 2;
    final maxVisibleBars = 100;

    // Calculate how many amplitudes to show
    final startIndex = math.max(0, amplitudes.length - maxVisibleBars);
    final visibleAmplitudes = amplitudes.sublist(startIndex);

    // Calculate bar width
    final barWidth = size.width / maxVisibleBars;
    final barSpacing = barWidth * 0.3;
    final actualBarWidth = barWidth - barSpacing;

    // Draw bars
    for (int i = 0; i < visibleAmplitudes.length; i++) {
      final amplitude = visibleAmplitudes[i];
      final x = i * barWidth + barSpacing / 2;
      final barHeight = amplitude * (size.height / 2) * 0.8;

      // Draw bar from center
      final rect = RRect.fromRectAndRadius(
        Rect.fromCenter(
          center: Offset(x + actualBarWidth / 2, centerY),
          width: actualBarWidth,
          height: barHeight,
        ),
        const Radius.circular(2),
      );

      canvas.drawRRect(rect, paint);
    }
  }

  void _drawPlaceholder(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = Colors.grey.withOpacity(0.3)
      ..strokeWidth = 2;

    final centerY = size.height / 2;

    // Draw flat line
    canvas.drawLine(
      Offset(0, centerY),
      Offset(size.width, centerY),
      paint,
    );

    // Draw text
    final textPainter = TextPainter(
      text: const TextSpan(
        text: 'ابدأ التسجيل لرؤية الموجات الصوتية',
        style: TextStyle(
          color: Colors.grey,
          fontSize: 14,
          fontFamily: 'Tajawal',
        ),
      ),
      textDirection: TextDirection.rtl,
    );

    textPainter.layout();
    textPainter.paint(
      canvas,
      Offset(
        (size.width - textPainter.width) / 2,
        (size.height - textPainter.height) / 2,
      ),
    );
  }

  @override
  bool shouldRepaint(WaveformPainter oldDelegate) {
    return oldDelegate.waveformData != waveformData ||
        oldDelegate.waveColor != waveColor ||
        oldDelegate.showGrid != showGrid;
  }
}

/// Animated waveform bars for recording indicator
class AnimatedWaveformBars extends StatefulWidget {
  final bool isRecording;
  final Color color;
  final double height;

  const AnimatedWaveformBars({
    Key? key,
    required this.isRecording,
    this.color = const Color(0xFFB8860B),
    this.height = 40,
  }) : super(key: key);

  @override
  State<AnimatedWaveformBars> createState() => _AnimatedWaveformBarsState();
}

class _AnimatedWaveformBarsState extends State<AnimatedWaveformBars>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 800),
    )..repeat(reverse: true);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (!widget.isRecording) {
      return SizedBox(height: widget.height);
    }

    return SizedBox(
      height: widget.height,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: List.generate(5, (index) {
          return AnimatedBuilder(
            animation: _controller,
            builder: (context, child) {
              final delay = index * 0.2;
              final value = math.sin((_controller.value + delay) * math.pi * 2);
              final height = widget.height * (0.3 + value.abs() * 0.7);

              return Container(
                width: 4,
                height: height,
                margin: const EdgeInsets.symmetric(horizontal: 2),
                decoration: BoxDecoration(
                  color: widget.color,
                  borderRadius: BorderRadius.circular(2),
                ),
              );
            },
          );
        }),
      ),
    );
  }
}

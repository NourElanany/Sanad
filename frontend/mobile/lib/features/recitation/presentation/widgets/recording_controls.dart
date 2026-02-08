import 'package:flutter/material.dart';
import '../../data/models/recording_models.dart';

/// Recording control buttons
class RecordingControls extends StatelessWidget {
  final RecordingState state;
  final VoidCallback onRecord;
  final VoidCallback onPause;
  final VoidCallback onResume;
  final VoidCallback onStop;
  final VoidCallback onCancel;

  const RecordingControls({
    Key? key,
    required this.state,
    required this.onRecord,
    required this.onPause,
    required this.onResume,
    required this.onStop,
    required this.onCancel,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        // Cancel button (only when recording or paused)
        if (state == RecordingState.recording || state == RecordingState.paused)
          _buildControlButton(
            icon: Icons.close,
            label: 'إلغاء',
            color: Colors.red,
            onPressed: onCancel,
          ),

        const SizedBox(width: 16),

        // Main control button
        _buildMainControlButton(context),

        const SizedBox(width: 16),

        // Stop button (only when recording or paused)
        if (state == RecordingState.recording || state == RecordingState.paused)
          _buildControlButton(
            icon: Icons.stop,
            label: 'إيقاف',
            color: const Color(0xFF1B365D), // Navy
            onPressed: onStop,
          ),
      ],
    );
  }

  Widget _buildMainControlButton(BuildContext context) {
    IconData icon;
    String label;
    Color color;
    VoidCallback? onPressed;

    switch (state) {
      case RecordingState.idle:
      case RecordingState.stopped:
        icon = Icons.mic;
        label = 'تسجيل';
        color = const Color(0xFF2D5A27); // Green
        onPressed = onRecord;
        break;
      case RecordingState.recording:
        icon = Icons.pause;
        label = 'إيقاف مؤقت';
        color = const Color(0xFFFFC107); // Yellow
        onPressed = onPause;
        break;
      case RecordingState.paused:
        icon = Icons.play_arrow;
        label = 'استئناف';
        color = const Color(0xFF2D5A27); // Green
        onPressed = onResume;
        break;
      case RecordingState.preparing:
      case RecordingState.processing:
        icon = Icons.hourglass_empty;
        label = 'جاري المعالجة...';
        color = Colors.grey;
        onPressed = null;
        break;
      case RecordingState.error:
        icon = Icons.error;
        label: 'خطأ';
        color = Colors.red;
        onPressed = null;
        break;
    }

    return _buildControlButton(
      icon: icon,
      label: label,
      color: color,
      onPressed: onPressed,
      isMain: true,
    );
  }

  Widget _buildControlButton({
    required IconData icon,
    required String label,
    required Color color,
    VoidCallback? onPressed,
    bool isMain = false,
  }) {
    final size = isMain ? 72.0 : 56.0;
    final iconSize = isMain ? 32.0 : 24.0;

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: size,
          height: size,
          decoration: BoxDecoration(
            shape: BoxShape.circle,
            color: onPressed != null ? color : Colors.grey,
            boxShadow: onPressed != null
                ? [
                    BoxShadow(
                      color: color.withOpacity(0.3),
                      blurRadius: 12,
                      offset: const Offset(0, 4),
                    ),
                  ]
                : null,
          ),
          child: Material(
            color: Colors.transparent,
            child: InkWell(
              onTap: onPressed,
              customBorder: const CircleBorder(),
              child: Icon(
                icon,
                color: Colors.white,
                size: iconSize,
              ),
            ),
          ),
        ),
        const SizedBox(height: 8),
        Text(
          label,
          style: const TextStyle(
            fontSize: 14,
            fontWeight: FontWeight.w500,
            fontFamily: 'Tajawal',
            color: Color(0xFF1A1A1A),
          ),
        ),
      ],
    );
  }
}

/// Recording duration display
class RecordingDuration extends StatelessWidget {
  final Duration duration;
  final Duration? maxDuration;

  const RecordingDuration({
    Key? key,
    required this.duration,
    this.maxDuration,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final minutes = duration.inMinutes.toString().padLeft(2, '0');
    final seconds = (duration.inSeconds % 60).toString().padLeft(2, '0');

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
      decoration: BoxDecoration(
        color: const Color(0xFF1B365D).withOpacity(0.1),
        borderRadius: BorderRadius.circular(24),
        border: Border.all(
          color: const Color(0xFF1B365D).withOpacity(0.3),
          width: 1,
        ),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          const Icon(
            Icons.timer,
            color: Color(0xFFB8860B), // Gold
            size: 20,
          ),
          const SizedBox(width: 8),
          Text(
            '$minutes:$seconds',
            style: const TextStyle(
              fontSize: 24,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
              color: Color(0xFF1B365D),
            ),
          ),
          if (maxDuration != null) ...[
            Text(
              ' / ${maxDuration!.inMinutes}:${(maxDuration!.inSeconds % 60).toString().padLeft(2, '0')}',
              style: TextStyle(
                fontSize: 16,
                fontFamily: 'Tajawal',
                color: const Color(0xFF1B365D).withOpacity(0.6),
              ),
            ),
          ],
        ],
      ),
    );
  }
}

/// Audio quality selector
class AudioQualitySelector extends StatelessWidget {
  final AudioQuality selectedQuality;
  final ValueChanged<AudioQuality> onQualityChanged;
  final bool enabled;

  const AudioQualitySelector({
    Key? key,
    required this.selectedQuality,
    required this.onQualityChanged,
    this.enabled = true,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'جودة التسجيل',
          style: TextStyle(
            fontSize: 16,
            fontWeight: FontWeight.w600,
            fontFamily: 'Tajawal',
            color: Color(0xFF1A1A1A),
          ),
        ),
        const SizedBox(height: 12),
        Wrap(
          spacing: 8,
          runSpacing: 8,
          children: AudioQuality.values.map((quality) {
            final isSelected = quality == selectedQuality;
            return _buildQualityChip(
              quality: quality,
              isSelected: isSelected,
              onTap: enabled ? () => onQualityChanged(quality) : null,
            );
          }).toList(),
        ),
      ],
    );
  }

  Widget _buildQualityChip({
    required AudioQuality quality,
    required bool isSelected,
    VoidCallback? onTap,
  }) {
    String label;
    String subtitle;

    switch (quality) {
      case AudioQuality.low:
        label = 'منخفضة';
        subtitle = '16kHz';
        break;
      case AudioQuality.medium:
        label = 'متوسطة';
        subtitle = '22kHz';
        break;
      case AudioQuality.high:
        label = 'عالية';
        subtitle = '44kHz';
        break;
      case AudioQuality.ultra:
        label = 'فائقة';
        subtitle = '48kHz';
        break;
    }

    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(12),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        decoration: BoxDecoration(
          color: isSelected
              ? const Color(0xFF1B365D)
              : const Color(0xFFF8F9FA),
          borderRadius: BorderRadius.circular(12),
          border: Border.all(
            color: isSelected
                ? const Color(0xFF1B365D)
                : const Color(0xFFE0E0E0),
            width: 1.5,
          ),
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              label,
              style: TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.w600,
                fontFamily: 'Tajawal',
                color: isSelected ? Colors.white : const Color(0xFF1A1A1A),
              ),
            ),
            const SizedBox(height: 2),
            Text(
              subtitle,
              style: TextStyle(
                fontSize: 11,
                fontFamily: 'Tajawal',
                color: isSelected
                    ? Colors.white.withOpacity(0.8)
                    : const Color(0xFF666666),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

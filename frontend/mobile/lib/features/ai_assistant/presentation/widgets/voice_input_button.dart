import 'package:flutter/material.dart';
import '../../../../core/theme/app_theme.dart';

/// Voice input button with animation
class VoiceInputButton extends StatefulWidget {
  final bool isListening;
  final VoidCallback onStartListening;
  final VoidCallback onStopListening;

  const VoiceInputButton({
    Key? key,
    required this.isListening,
    required this.onStartListening,
    required this.onStopListening,
  }) : super(key: key);

  @override
  State<VoiceInputButton> createState() => _VoiceInputButtonState();
}

class _VoiceInputButtonState extends State<VoiceInputButton>
    with SingleTickerProviderStateMixin {
  late AnimationController _animationController;
  late Animation<double> _scaleAnimation;

  @override
  void initState() {
    super.initState();
    _animationController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1000),
    )..repeat(reverse: true);

    _scaleAnimation = Tween<double>(begin: 1.0, end: 1.2).animate(
      CurvedAnimation(
        parent: _animationController,
        curve: Curves.easeInOut,
      ),
    );
  }

  @override
  void dispose() {
    _animationController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: widget.isListening
          ? widget.onStopListening
          : widget.onStartListening,
      child: AnimatedBuilder(
        animation: _scaleAnimation,
        builder: (context, child) {
          return Transform.scale(
            scale: widget.isListening ? _scaleAnimation.value : 1.0,
            child: Container(
              width: 48,
              height: 48,
              decoration: BoxDecoration(
                gradient: widget.isListening
                    ? LinearGradient(
                        colors: [
                          Colors.red.shade400,
                          Colors.red.shade600,
                        ],
                      )
                    : LinearGradient(
                        colors: [
                          AppColors.accent,
                          AppColors.accent.withOpacity(0.8),
                        ],
                      ),
                shape: BoxShape.circle,
                boxShadow: [
                  BoxShadow(
                    color: (widget.isListening
                            ? Colors.red
                            : AppColors.accent)
                        .withOpacity(0.3),
                    blurRadius: widget.isListening ? 12 : 8,
                    spreadRadius: widget.isListening ? 2 : 0,
                  ),
                ],
              ),
              child: Icon(
                widget.isListening ? Icons.stop : Icons.mic,
                color: Colors.white,
                size: 24,
              ),
            ),
          );
        },
      ),
    );
  }
}

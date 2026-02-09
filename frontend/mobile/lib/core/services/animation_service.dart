import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';

/// Service for managing smooth animations throughout the app
/// Ensures 60fps animations with proper timing and easing
class AnimationService {
  static final AnimationService _instance = AnimationService._internal();
  factory AnimationService() => _instance;
  AnimationService._internal();

  // Standard animation durations
  static const Duration fastDuration = Duration(milliseconds: 150);
  static const Duration normalDuration = Duration(milliseconds: 300);
  static const Duration slowDuration = Duration(milliseconds: 500);
  
  // Standard curves
  static const Curve defaultCurve = Curves.easeInOut;
  static const Curve emphasizedCurve = Curves.easeOutCubic;
  static const Curve deceleratedCurve = Curves.decelerate;
  static const Curve acceleratedCurve = Curves.accelerate;

  /// Create a standard fade transition
  Widget createFadeTransition({
    required Widget child,
    required Animation<double> animation,
    Curve curve = defaultCurve,
  }) {
    return FadeTransition(
      opacity: CurvedAnimation(
        parent: animation,
        curve: curve,
      ),
      child: child,
    );
  }

  /// Create a slide transition
  Widget createSlideTransition({
    required Widget child,
    required Animation<double> animation,
    Offset begin = const Offset(1.0, 0.0),
    Offset end = Offset.zero,
    Curve curve = defaultCurve,
  }) {
    return SlideTransition(
      position: Tween<Offset>(
        begin: begin,
        end: end,
      ).animate(CurvedAnimation(
        parent: animation,
        curve: curve,
      )),
      child: child,
    );
  }

  /// Create a scale transition
  Widget createScaleTransition({
    required Widget child,
    required Animation<double> animation,
    double begin = 0.0,
    double end = 1.0,
    Curve curve = defaultCurve,
    Alignment alignment = Alignment.center,
  }) {
    return ScaleTransition(
      scale: Tween<double>(
        begin: begin,
        end: end,
      ).animate(CurvedAnimation(
        parent: animation,
        curve: curve,
      )),
      alignment: alignment,
      child: child,
    );
  }

  /// Create a rotation transition
  Widget createRotationTransition({
    required Widget child,
    required Animation<double> animation,
    double begin = 0.0,
    double end = 1.0,
    Curve curve = defaultCurve,
    Alignment alignment = Alignment.center,
  }) {
    return RotationTransition(
      turns: Tween<double>(
        begin: begin,
        end: end,
      ).animate(CurvedAnimation(
        parent: animation,
        curve: curve,
      )),
      alignment: alignment,
      child: child,
    );
  }

  /// Create a combined fade and slide transition
  Widget createFadeSlideTransition({
    required Widget child,
    required Animation<double> animation,
    Offset begin = const Offset(0.0, 0.3),
    Offset end = Offset.zero,
    Curve curve = emphasizedCurve,
  }) {
    return FadeTransition(
      opacity: CurvedAnimation(
        parent: animation,
        curve: curve,
      ),
      child: SlideTransition(
        position: Tween<Offset>(
          begin: begin,
          end: end,
        ).animate(CurvedAnimation(
          parent: animation,
          curve: curve,
        )),
        child: child,
      ),
    );
  }

  /// Create page route with custom transition
  PageRoute createPageRoute({
    required Widget page,
    RouteSettings? settings,
    PageTransitionType type = PageTransitionType.fadeSlide,
    Duration duration = normalDuration,
  }) {
    return PageRouteBuilder(
      settings: settings,
      pageBuilder: (context, animation, secondaryAnimation) => page,
      transitionDuration: duration,
      reverseTransitionDuration: duration,
      transitionsBuilder: (context, animation, secondaryAnimation, child) {
        switch (type) {
          case PageTransitionType.fade:
            return createFadeTransition(
              animation: animation,
              child: child,
            );
          case PageTransitionType.slide:
            return createSlideTransition(
              animation: animation,
              child: child,
            );
          case PageTransitionType.scale:
            return createScaleTransition(
              animation: animation,
              child: child,
            );
          case PageTransitionType.fadeSlide:
            return createFadeSlideTransition(
              animation: animation,
              child: child,
            );
        }
      },
    );
  }

  /// Animate a value with proper timing
  Future<void> animateValue({
    required TickerProvider vsync,
    required ValueChanged<double> onUpdate,
    double begin = 0.0,
    double end = 1.0,
    Duration duration = normalDuration,
    Curve curve = defaultCurve,
  }) async {
    final controller = AnimationController(
      duration: duration,
      vsync: vsync,
    );

    final animation = Tween<double>(
      begin: begin,
      end: end,
    ).animate(CurvedAnimation(
      parent: controller,
      curve: curve,
    ));

    animation.addListener(() {
      onUpdate(animation.value);
    });

    await controller.forward();
    controller.dispose();
  }

  /// Create staggered animation for list items
  Widget createStaggeredListItem({
    required Widget child,
    required int index,
    required Animation<double> animation,
    int maxStagger = 5,
    Duration staggerDelay = const Duration(milliseconds: 50),
  }) {
    final delay = (index < maxStagger) ? index * staggerDelay.inMilliseconds : 0;
    final delayedAnimation = Tween<double>(
      begin: 0.0,
      end: 1.0,
    ).animate(CurvedAnimation(
      parent: animation,
      curve: Interval(
        delay / normalDuration.inMilliseconds,
        1.0,
        curve: emphasizedCurve,
      ),
    ));

    return createFadeSlideTransition(
      animation: delayedAnimation,
      child: child,
    );
  }

  /// Create shimmer effect for loading states
  Widget createShimmerEffect({
    required Widget child,
    Color baseColor = const Color(0xFFE0E0E0),
    Color highlightColor = const Color(0xFFF5F5F5),
    Duration duration = const Duration(milliseconds: 1500),
  }) {
    return _ShimmerWidget(
      baseColor: baseColor,
      highlightColor: highlightColor,
      duration: duration,
      child: child,
    );
  }

  /// Check if animations should be reduced (accessibility)
  bool shouldReduceAnimations(BuildContext context) {
    return MediaQuery.of(context).disableAnimations ||
           timeDilation > 1.0;
  }

  /// Get adjusted duration based on accessibility settings
  Duration getAdjustedDuration(
    BuildContext context,
    Duration baseDuration,
  ) {
    if (shouldReduceAnimations(context)) {
      return Duration.zero;
    }
    return baseDuration;
  }
}

/// Page transition types
enum PageTransitionType {
  fade,
  slide,
  scale,
  fadeSlide,
}

/// Shimmer widget for loading states
class _ShimmerWidget extends StatefulWidget {
  final Widget child;
  final Color baseColor;
  final Color highlightColor;
  final Duration duration;

  const _ShimmerWidget({
    required this.child,
    required this.baseColor,
    required this.highlightColor,
    required this.duration,
  });

  @override
  State<_ShimmerWidget> createState() => _ShimmerWidgetState();
}

class _ShimmerWidgetState extends State<_ShimmerWidget>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      duration: widget.duration,
      vsync: this,
    )..repeat();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _controller,
      child: widget.child,
      builder: (context, child) {
        return ShaderMask(
          shaderCallback: (bounds) {
            return LinearGradient(
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
              colors: [
                widget.baseColor,
                widget.highlightColor,
                widget.baseColor,
              ],
              stops: [
                _controller.value - 0.3,
                _controller.value,
                _controller.value + 0.3,
              ].map((stop) => stop.clamp(0.0, 1.0)).toList(),
            ).createShader(bounds);
          },
          child: child,
        );
      },
    );
  }
}

/// Animated list item widget
class AnimatedListItem extends StatefulWidget {
  final Widget child;
  final int index;
  final Duration delay;
  final Duration duration;

  const AnimatedListItem({
    Key? key,
    required this.child,
    required this.index,
    this.delay = const Duration(milliseconds: 50),
    this.duration = AnimationService.normalDuration,
  }) : super(key: key);

  @override
  State<AnimatedListItem> createState() => _AnimatedListItemState();
}

class _AnimatedListItemState extends State<AnimatedListItem>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _fadeAnimation;
  late Animation<Offset> _slideAnimation;

  @override
  void initState() {
    super.initState();
    
    _controller = AnimationController(
      duration: widget.duration,
      vsync: this,
    );

    final delay = widget.index * widget.delay.inMilliseconds;
    final curve = Interval(
      delay / widget.duration.inMilliseconds,
      1.0,
      curve: AnimationService.emphasizedCurve,
    );

    _fadeAnimation = Tween<double>(
      begin: 0.0,
      end: 1.0,
    ).animate(CurvedAnimation(
      parent: _controller,
      curve: curve,
    ));

    _slideAnimation = Tween<Offset>(
      begin: const Offset(0.0, 0.3),
      end: Offset.zero,
    ).animate(CurvedAnimation(
      parent: _controller,
      curve: curve,
    ));

    _controller.forward();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return FadeTransition(
      opacity: _fadeAnimation,
      child: SlideTransition(
        position: _slideAnimation,
        child: widget.child,
      ),
    );
  }
}

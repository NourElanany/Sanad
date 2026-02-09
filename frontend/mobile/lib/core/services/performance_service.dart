import 'dart:async';
import 'dart:developer' as developer;
import 'package:flutter/foundation.dart';
import 'package:flutter/scheduler.dart';
import '../utils/logger.dart';

/// Service for monitoring and optimizing app performance
/// Ensures 60fps rendering and tracks performance metrics
class PerformanceService {
  static final PerformanceService _instance = PerformanceService._internal();
  factory PerformanceService() => _instance;
  PerformanceService._internal();

  final _logger = Logger('PerformanceService');
  
  // Performance metrics
  final List<double> _frameTimings = [];
  final List<PerformanceMetric> _metrics = [];
  Timer? _monitoringTimer;
  bool _isMonitoring = false;
  
  // Frame rate tracking
  int _frameCount = 0;
  DateTime? _lastFrameTime;
  double _currentFps = 60.0;
  
  // Performance thresholds
  static const double targetFps = 60.0;
  static const double minAcceptableFps = 55.0;
  static const int maxFrameTimings = 100;
  static const Duration monitoringInterval = Duration(seconds: 1);

  /// Initialize performance monitoring
  void initialize() {
    if (_isMonitoring) return;
    
    _isMonitoring = true;
    _logger.info('Performance monitoring initialized');
    
    // Start frame callback monitoring
    SchedulerBinding.instance.addTimingsCallback(_onFrameTimings);
    
    // Start periodic performance checks
    _startPeriodicMonitoring();
  }

  /// Stop performance monitoring
  void dispose() {
    _isMonitoring = false;
    _monitoringTimer?.cancel();
    _monitoringTimer = null;
    SchedulerBinding.instance.removeTimingsCallback(_onFrameTimings);
    _logger.info('Performance monitoring stopped');
  }

  /// Handle frame timing callbacks
  void _onFrameTimings(List<FrameTiming> timings) {
    if (!_isMonitoring) return;

    for (final timing in timings) {
      final frameTime = timing.totalSpan.inMicroseconds / 1000.0; // Convert to ms
      _frameTimings.add(frameTime);
      
      // Keep only recent timings
      if (_frameTimings.length > maxFrameTimings) {
        _frameTimings.removeAt(0);
      }
      
      // Track frame count for FPS calculation
      _frameCount++;
      
      // Log slow frames (> 16.67ms for 60fps)
      if (frameTime > 16.67) {
        _logger.warning('Slow frame detected: ${frameTime.toStringAsFixed(2)}ms');
      }
    }
  }

  /// Start periodic performance monitoring
  void _startPeriodicMonitoring() {
    _monitoringTimer = Timer.periodic(monitoringInterval, (_) {
      _calculateFps();
      _checkPerformance();
    });
  }

  /// Calculate current FPS
  void _calculateFps() {
    final now = DateTime.now();
    
    if (_lastFrameTime != null) {
      final elapsed = now.difference(_lastFrameTime!).inMilliseconds;
      if (elapsed > 0) {
        _currentFps = (_frameCount * 1000.0) / elapsed;
        _frameCount = 0;
      }
    }
    
    _lastFrameTime = now;
  }

  /// Check overall performance and log issues
  void _checkPerformance() {
    if (_frameTimings.isEmpty) return;

    final avgFrameTime = _frameTimings.reduce((a, b) => a + b) / _frameTimings.length;
    final maxFrameTime = _frameTimings.reduce((a, b) => a > b ? a : b);
    
    final metric = PerformanceMetric(
      timestamp: DateTime.now(),
      fps: _currentFps,
      avgFrameTime: avgFrameTime,
      maxFrameTime: maxFrameTime,
    );
    
    _metrics.add(metric);
    
    // Keep only recent metrics (last 60 seconds)
    if (_metrics.length > 60) {
      _metrics.removeAt(0);
    }

    // Log performance warnings
    if (_currentFps < minAcceptableFps) {
      _logger.warning(
        'Low FPS detected: ${_currentFps.toStringAsFixed(1)} fps '
        '(target: $targetFps fps)',
      );
    }

    if (kDebugMode) {
      developer.log(
        'Performance: ${_currentFps.toStringAsFixed(1)} fps, '
        'Avg frame: ${avgFrameTime.toStringAsFixed(2)}ms, '
        'Max frame: ${maxFrameTime.toStringAsFixed(2)}ms',
        name: 'PerformanceService',
      );
    }
  }

  /// Get current FPS
  double get currentFps => _currentFps;

  /// Get average frame time
  double get averageFrameTime {
    if (_frameTimings.isEmpty) return 0.0;
    return _frameTimings.reduce((a, b) => a + b) / _frameTimings.length;
  }

  /// Get performance metrics for the last period
  List<PerformanceMetric> get recentMetrics => List.unmodifiable(_metrics);

  /// Check if performance is acceptable
  bool get isPerformanceGood => _currentFps >= minAcceptableFps;

  /// Get performance summary
  PerformanceSummary getPerformanceSummary() {
    if (_metrics.isEmpty) {
      return PerformanceSummary(
        avgFps: targetFps,
        minFps: targetFps,
        maxFps: targetFps,
        avgFrameTime: 16.67,
        isHealthy: true,
      );
    }

    final fpsList = _metrics.map((m) => m.fps).toList();
    final frameTimeList = _metrics.map((m) => m.avgFrameTime).toList();

    return PerformanceSummary(
      avgFps: fpsList.reduce((a, b) => a + b) / fpsList.length,
      minFps: fpsList.reduce((a, b) => a < b ? a : b),
      maxFps: fpsList.reduce((a, b) => a > b ? a : b),
      avgFrameTime: frameTimeList.reduce((a, b) => a + b) / frameTimeList.length,
      isHealthy: fpsList.every((fps) => fps >= minAcceptableFps),
    );
  }

  /// Mark a performance-critical operation
  Future<T> measureOperation<T>(
    String operationName,
    Future<T> Function() operation,
  ) async {
    final stopwatch = Stopwatch()..start();
    
    try {
      final result = await operation();
      stopwatch.stop();
      
      final duration = stopwatch.elapsedMilliseconds;
      _logger.info('$operationName completed in ${duration}ms');
      
      if (duration > 100) {
        _logger.warning('$operationName took longer than expected: ${duration}ms');
      }
      
      return result;
    } catch (e) {
      stopwatch.stop();
      _logger.error('$operationName failed after ${stopwatch.elapsedMilliseconds}ms: $e');
      rethrow;
    }
  }
}

/// Performance metric data class
class PerformanceMetric {
  final DateTime timestamp;
  final double fps;
  final double avgFrameTime;
  final double maxFrameTime;

  PerformanceMetric({
    required this.timestamp,
    required this.fps,
    required this.avgFrameTime,
    required this.maxFrameTime,
  });
}

/// Performance summary data class
class PerformanceSummary {
  final double avgFps;
  final double minFps;
  final double maxFps;
  final double avgFrameTime;
  final bool isHealthy;

  PerformanceSummary({
    required this.avgFps,
    required this.minFps,
    required this.maxFps,
    required this.avgFrameTime,
    required this.isHealthy,
  });

  @override
  String toString() {
    return 'PerformanceSummary('
        'avgFps: ${avgFps.toStringAsFixed(1)}, '
        'minFps: ${minFps.toStringAsFixed(1)}, '
        'maxFps: ${maxFps.toStringAsFixed(1)}, '
        'avgFrameTime: ${avgFrameTime.toStringAsFixed(2)}ms, '
        'isHealthy: $isHealthy)';
  }
}

import 'package:flutter/material.dart';
import '../../services/performance_service.dart';
import '../../services/image_optimization_service.dart';
import '../../services/animation_service.dart';
import '../lazy_loading_list.dart';

/// Example widget demonstrating performance optimization features
/// This shows how to use the performance services in your app
class PerformanceOptimizationDemo extends StatefulWidget {
  const PerformanceOptimizationDemo({Key? key}) : super(key: key);

  @override
  State<PerformanceOptimizationDemo> createState() => _PerformanceOptimizationDemoState();
}

class _PerformanceOptimizationDemoState extends State<PerformanceOptimizationDemo>
    with SingleTickerProviderStateMixin {
  final _performanceService = PerformanceService();
  final _imageService = ImageOptimizationService();
  final _animationService = AnimationService();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'Performance Optimization Demo',
          style: TextStyle(fontFamily: 'Tajawal'),
        ),
        backgroundColor: const Color(0xFF1B365D),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Performance Metrics Card
            _buildPerformanceMetricsCard(),
            const SizedBox(height: 16),

            // Optimized Image Example
            _buildOptimizedImageExample(),
            const SizedBox(height: 16),

            // Animation Examples
            _buildAnimationExamples(),
            const SizedBox(height: 16),

            // Lazy Loading Example
            _buildLazyLoadingExample(),
          ],
        ),
      ),
    );
  }

  Widget _buildPerformanceMetricsCard() {
    final summary = _performanceService.getPerformanceSummary();

    return Card(
      elevation: 4,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Performance Metrics',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 12),
            _buildMetricRow('Current FPS', '${_performanceService.currentFps.toStringAsFixed(1)}'),
            _buildMetricRow('Average FPS', '${summary.avgFps.toStringAsFixed(1)}'),
            _buildMetricRow('Min FPS', '${summary.minFps.toStringAsFixed(1)}'),
            _buildMetricRow('Max FPS', '${summary.maxFps.toStringAsFixed(1)}'),
            _buildMetricRow('Avg Frame Time', '${summary.avgFrameTime.toStringAsFixed(2)}ms'),
            _buildMetricRow(
              'Status',
              summary.isHealthy ? '✅ Healthy' : '⚠️ Needs Attention',
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildMetricRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(
            label,
            style: const TextStyle(
              fontFamily: 'Tajawal',
              color: Color(0xFF666666),
            ),
          ),
          Text(
            value,
            style: const TextStyle(
              fontFamily: 'Tajawal',
              fontWeight: FontWeight.bold,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildOptimizedImageExample() {
    return Card(
      elevation: 4,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Optimized Image Loading',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 12),
            SizedBox(
              height: 200,
              child: _imageService.getOptimizedNetworkImage(
                imageUrl: 'https://via.placeholder.com/400x300',
                width: double.infinity,
                height: 200,
                fit: BoxFit.cover,
              ),
            ),
            const SizedBox(height: 8),
            const Text(
              'This image uses CachedNetworkImage with memory optimization',
              style: TextStyle(
                fontSize: 12,
                fontFamily: 'Tajawal',
                color: Color(0xFF666666),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildAnimationExamples() {
    return Card(
      elevation: 4,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Smooth Animations',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 12),
            AnimatedListItem(
              index: 0,
              child: Container(
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: const Color(0xFF1B365D).withOpacity(0.1),
                  borderRadius: BorderRadius.circular(8),
                ),
                child: const Text(
                  'This item animates in with fade and slide',
                  style: TextStyle(fontFamily: 'Tajawal'),
                ),
              ),
            ),
            const SizedBox(height: 8),
            AnimatedListItem(
              index: 1,
              child: Container(
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: const Color(0xFF2D5A27).withOpacity(0.1),
                  borderRadius: BorderRadius.circular(8),
                ),
                child: const Text(
                  'Staggered animation with delay',
                  style: TextStyle(fontFamily: 'Tajawal'),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildLazyLoadingExample() {
    return Card(
      elevation: 4,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Lazy Loading List',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 12),
            SizedBox(
              height: 300,
              child: LazyLoadingList<String>(
                onLoadMore: (page, pageSize) async {
                  // Simulate API call
                  await Future.delayed(const Duration(milliseconds: 500));
                  return List.generate(
                    pageSize,
                    (index) => 'Item ${page * pageSize + index + 1}',
                  );
                },
                itemBuilder: (context, item, index) {
                  return Container(
                    padding: const EdgeInsets.all(12),
                    margin: const EdgeInsets.only(bottom: 8),
                    decoration: BoxDecoration(
                      color: Colors.white,
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(
                        color: const Color(0xFF1B365D).withOpacity(0.2),
                      ),
                    ),
                    child: Text(
                      item,
                      style: const TextStyle(fontFamily: 'Tajawal'),
                    ),
                  );
                },
                pageSize: 10,
                enableAnimation: true,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

/// Example of using performance measurement for operations
class PerformanceMeasurementExample extends StatelessWidget {
  const PerformanceMeasurementExample({Key? key}) : super(key: key);

  Future<void> _performHeavyOperation() async {
    await PerformanceService().measureOperation(
      'Heavy Operation',
      () async {
        // Simulate heavy computation
        await Future.delayed(const Duration(milliseconds: 500));
        
        // Do some work
        var sum = 0;
        for (var i = 0; i < 1000000; i++) {
          sum += i;
        }
        
        return sum;
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: _performHeavyOperation,
      style: ElevatedButton.styleFrom(
        backgroundColor: const Color(0xFF1B365D),
        foregroundColor: Colors.white,
      ),
      child: const Text(
        'Measure Operation Performance',
        style: TextStyle(fontFamily: 'Tajawal'),
      ),
    );
  }
}

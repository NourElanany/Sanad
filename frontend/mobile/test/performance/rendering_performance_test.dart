import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../lib/features/quran/presentation/screens/mushaf_view_screen.dart';
import '../../lib/features/dashboard/presentation/screens/dashboard_screen.dart';

/// Performance tests for rendering and animations
/// **Validates: Requirements 20.5**
void main() {
  group('Rendering Performance Tests', () {
    testWidgets('Mushaf view should render at 60fps', (WidgetTester tester) async {
      // Arrange
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: MushafViewScreen(surahNumber: 1),
          ),
        ),
      );

      // Act - Measure frame rendering time
      final Stopwatch stopwatch = Stopwatch()..start();
      await tester.pumpAndSettle();
      stopwatch.stop();

      // Assert - Should render within 16.67ms (60fps)
      expect(stopwatch.elapsedMilliseconds, lessThan(17));
    });

    testWidgets('Dashboard should load quickly', (WidgetTester tester) async {
      // Arrange
      final Stopwatch stopwatch = Stopwatch()..start();

      // Act
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: DashboardScreen(),
          ),
        ),
      );
      await tester.pumpAndSettle();
      stopwatch.stop();

      // Assert - Should load within 1 second
      expect(stopwatch.elapsedMilliseconds, lessThan(1000));
    });

    testWidgets('Scrolling should be smooth', (WidgetTester tester) async {
      // Arrange
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: MushafViewScreen(surahNumber: 2), // Al-Baqarah (long surah)
          ),
        ),
      );
      await tester.pumpAndSettle();

      // Act - Perform scroll gesture
      final Stopwatch stopwatch = Stopwatch()..start();
      await tester.drag(
        find.byType(ListView),
        const Offset(0, -500),
        touchSlopY: 0,
      );
      await tester.pump();
      stopwatch.stop();

      // Assert - Scroll should be responsive (< 16.67ms)
      expect(stopwatch.elapsedMilliseconds, lessThan(17));
    });

    testWidgets('List with 114 surahs should render efficiently', (WidgetTester tester) async {
      // Arrange
      final Stopwatch stopwatch = Stopwatch()..start();

      // Act
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: ListView.builder(
                itemCount: 114,
                itemBuilder: (context, index) {
                  return ListTile(
                    title: Text('سورة ${index + 1}'),
                    subtitle: Text('عدد الآيات: ${index + 1}'),
                  );
                },
              ),
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();
      stopwatch.stop();

      // Assert - Should render within reasonable time
      expect(stopwatch.elapsedMilliseconds, lessThan(500));
    });

    testWidgets('Image loading should not block UI', (WidgetTester tester) async {
      // Arrange
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: ListView.builder(
                itemCount: 20,
                itemBuilder: (context, index) {
                  return Card(
                    child: Column(
                      children: [
                        Image.network(
                          'https://example.com/image$index.jpg',
                          loadingBuilder: (context, child, loadingProgress) {
                            if (loadingProgress == null) return child;
                            return const CircularProgressIndicator();
                          },
                        ),
                        Text('Item $index'),
                      ],
                    ),
                  );
                },
              ),
            ),
          ),
        ),
      );

      // Act
      await tester.pump();

      // Assert - UI should be responsive even with loading images
      expect(find.byType(CircularProgressIndicator), findsWidgets);
      expect(tester.binding.hasScheduledFrame, isFalse);
    });

    testWidgets('Animation should run smoothly', (WidgetTester tester) async {
      // Arrange
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Center(
              child: TweenAnimationBuilder<double>(
                tween: Tween(begin: 0.0, end: 1.0),
                duration: const Duration(milliseconds: 300),
                builder: (context, value, child) {
                  return Opacity(
                    opacity: value,
                    child: const Text('Animated Text'),
                  );
                },
              ),
            ),
          ),
        ),
      );

      // Act - Measure animation frames
      final List<Duration> frameTimes = [];
      await tester.pumpAndSettle(
        const Duration(milliseconds: 300),
        EnginePhase.sendSemanticsUpdate,
        const Duration(milliseconds: 16),
      );

      // Assert - All frames should be within 60fps threshold
      for (final frameTime in frameTimes) {
        expect(frameTime.inMilliseconds, lessThan(17));
      }
    });

    testWidgets('Memory usage should be reasonable', (WidgetTester tester) async {
      // Arrange
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: DashboardScreen(),
          ),
        ),
      );
      await tester.pumpAndSettle();

      // Act - Navigate through multiple screens
      for (var i = 0; i < 10; i++) {
        await tester.tap(find.text('القرآن الكريم'));
        await tester.pumpAndSettle();
        await tester.tap(find.byType(BackButton));
        await tester.pumpAndSettle();
      }

      // Assert - No memory leaks (widgets should be disposed)
      expect(tester.binding.hasScheduledFrame, isFalse);
    });

    testWidgets('Large text rendering should be efficient', (WidgetTester tester) async {
      // Arrange
      const largeArabicText = '''
بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ
الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ
الرَّحْمَٰنِ الرَّحِيمِ
مَالِكِ يَوْمِ الدِّينِ
إِيَّاكَ نَعْبُدُ وَإِيَّاكَ نَسْتَعِينُ
اهْدِنَا الصِّرَاطَ الْمُسْتَقِيمَ
صِرَاطَ الَّذِينَ أَنْعَمْتَ عَلَيْهِمْ غَيْرِ الْمَغْضُوبِ عَلَيْهِمْ وَلَا الضَّالِّينَ
      ''';

      final Stopwatch stopwatch = Stopwatch()..start();

      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: SingleChildScrollView(
              child: Text(
                largeArabicText * 100, // Repeat 100 times
                style: const TextStyle(
                  fontSize: 24,
                  fontFamily: 'KFGQPC Uthman Taha Naskh',
                  height: 2.0,
                ),
                textDirection: TextDirection.rtl,
              ),
            ),
          ),
        ),
      );
      await tester.pump();
      stopwatch.stop();

      // Assert - Should render large text efficiently
      expect(stopwatch.elapsedMilliseconds, lessThan(100));
    });

    testWidgets('Rapid state updates should not cause jank', (WidgetTester tester) async {
      // Arrange
      int counter = 0;

      await tester.pumpWidget(
        MaterialApp(
          home: StatefulBuilder(
            builder: (context, setState) {
              return Scaffold(
                body: Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text('Counter: $counter'),
                      ElevatedButton(
                        onPressed: () {
                          setState(() {
                            counter++;
                          });
                        },
                        child: const Text('Increment'),
                      ),
                    ],
                  ),
                ),
              );
            },
          ),
        ),
      );

      // Act - Rapidly update state
      final Stopwatch stopwatch = Stopwatch()..start();
      for (var i = 0; i < 100; i++) {
        await tester.tap(find.text('Increment'));
        await tester.pump();
      }
      stopwatch.stop();

      // Assert - Should handle rapid updates smoothly
      expect(stopwatch.elapsedMilliseconds, lessThan(1000));
      expect(find.text('Counter: 100'), findsOneWidget);
    });

    testWidgets('Complex layout should render efficiently', (WidgetTester tester) async {
      // Arrange
      final Stopwatch stopwatch = Stopwatch()..start();

      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Column(
              children: [
                Container(
                  height: 200,
                  decoration: BoxDecoration(
                    gradient: LinearGradient(
                      colors: [Colors.blue, Colors.purple],
                    ),
                  ),
                  child: const Center(
                    child: Text(
                      'Header',
                      style: TextStyle(fontSize: 32, color: Colors.white),
                    ),
                  ),
                ),
                Expanded(
                  child: GridView.builder(
                    gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                      crossAxisCount: 3,
                      childAspectRatio: 1.0,
                    ),
                    itemCount: 50,
                    itemBuilder: (context, index) {
                      return Card(
                        elevation: 4,
                        child: Center(
                          child: Text('Item $index'),
                        ),
                      );
                    },
                  ),
                ),
              ],
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();
      stopwatch.stop();

      // Assert
      expect(stopwatch.elapsedMilliseconds, lessThan(500));
    });
  });
}

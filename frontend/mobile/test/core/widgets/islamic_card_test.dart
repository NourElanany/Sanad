import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../../../lib/core/widgets/islamic_card.dart';

void main() {
  group('IslamicCard Widget Tests', () {
    testWidgets('should render child widget', (WidgetTester tester) async {
      // Arrange
      const childText = 'محتوى البطاقة';

      // Act
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: IslamicCard(
              child: Text(childText),
            ),
          ),
        ),
      );

      // Assert
      expect(find.text(childText), findsOneWidget);
    });

    testWidgets('should have elevation when elevated is true', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicCard(
              elevated: true,
              child: const Text('مرفوع'),
            ),
          ),
        ),
      );

      // Assert
      final container = tester.widget<Container>(
        find.descendant(
          of: find.byType(IslamicCard),
          matching: find.byType(Container),
        ).first,
      );

      final decoration = container.decoration as BoxDecoration;
      expect(decoration.boxShadow, isNotNull);
      expect(decoration.boxShadow!.isNotEmpty, isTrue);
    });

    testWidgets('should not have elevation when elevated is false', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicCard(
              elevated: false,
              child: const Text('مسطح'),
            ),
          ),
        ),
      );

      // Assert
      final container = tester.widget<Container>(
        find.descendant(
          of: find.byType(IslamicCard),
          matching: find.byType(Container),
        ).first,
      );

      final decoration = container.decoration as BoxDecoration;
      expect(decoration.boxShadow, isNull);
    });

    testWidgets('should call onTap when tapped', (WidgetTester tester) async {
      // Arrange
      var wasTapped = false;

      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicCard(
              onTap: () {
                wasTapped = true;
              },
              child: const Text('قابل للنقر'),
            ),
          ),
        ),
      );

      await tester.tap(find.byType(IslamicCard));
      await tester.pump();

      // Assert
      expect(wasTapped, isTrue);
    });

    testWidgets('should not be tappable when onTap is null', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: IslamicCard(
              child: Text('غير قابل للنقر'),
            ),
          ),
        ),
      );

      // Assert
      final inkWell = tester.widget<InkWell>(
        find.descendant(
          of: find.byType(IslamicCard),
          matching: find.byType(InkWell),
        ),
      );

      expect(inkWell.onTap, isNull);
    });

    testWidgets('should apply custom padding', (WidgetTester tester) async {
      // Arrange
      const customPadding = EdgeInsets.all(30);

      // Act
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: IslamicCard(
              padding: customPadding,
              child: Text('مع حشوة مخصصة'),
            ),
          ),
        ),
      );

      // Assert
      final padding = tester.widget<Padding>(
        find.descendant(
          of: find.byType(IslamicCard),
          matching: find.byType(Padding),
        ).first,
      );

      expect(padding.padding, equals(customPadding));
    });

    testWidgets('should have rounded corners', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: IslamicCard(
              child: Text('بطاقة'),
            ),
          ),
        ),
      );

      // Assert
      final container = tester.widget<Container>(
        find.descendant(
          of: find.byType(IslamicCard),
          matching: find.byType(Container),
        ).first,
      );

      final decoration = container.decoration as BoxDecoration;
      expect(decoration.borderRadius, isA<BorderRadius>());
    });

    testWidgets('should have border', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: IslamicCard(
              child: Text('بطاقة'),
            ),
          ),
        ),
      );

      // Assert
      final container = tester.widget<Container>(
        find.descendant(
          of: find.byType(IslamicCard),
          matching: find.byType(Container),
        ).first,
      );

      final decoration = container.decoration as BoxDecoration;
      expect(decoration.border, isNotNull);
    });

    testWidgets('should have correct background color', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: IslamicCard(
              child: Text('بطاقة'),
            ),
          ),
        ),
      );

      // Assert
      final container = tester.widget<Container>(
        find.descendant(
          of: find.byType(IslamicCard),
          matching: find.byType(Container),
        ).first,
      );

      final decoration = container.decoration as BoxDecoration;
      expect(decoration.color, isNotNull);
    });

    testWidgets('should show ripple effect on tap', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicCard(
              onTap: () {},
              child: const Text('بطاقة'),
            ),
          ),
        ),
      );

      // Assert
      expect(find.byType(InkWell), findsOneWidget);
    });

    testWidgets('should support nested widgets', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicCard(
              child: Column(
                children: const [
                  Text('عنوان'),
                  SizedBox(height: 8),
                  Text('محتوى'),
                ],
              ),
            ),
          ),
        ),
      );

      // Assert
      expect(find.text('عنوان'), findsOneWidget);
      expect(find.text('محتوى'), findsOneWidget);
    });
  });
}

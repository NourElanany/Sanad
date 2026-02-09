import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../../../lib/core/widgets/islamic_button.dart';

void main() {
  group('IslamicButton Widget Tests', () {
    testWidgets('should render button with text', (WidgetTester tester) async {
      // Arrange
      const buttonText = 'اضغط هنا';

      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicButton(
              text: buttonText,
              onPressed: () {},
            ),
          ),
        ),
      );

      // Assert
      expect(find.text(buttonText), findsOneWidget);
    });

    testWidgets('should call onPressed when tapped', (WidgetTester tester) async {
      // Arrange
      var wasPressed = false;

      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicButton(
              text: 'اضغط',
              onPressed: () {
                wasPressed = true;
              },
            ),
          ),
        ),
      );

      await tester.tap(find.byType(IslamicButton));
      await tester.pump();

      // Assert
      expect(wasPressed, isTrue);
    });

    testWidgets('should be disabled when onPressed is null', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        const MaterialApp(
          home: Scaffold(
            body: IslamicButton(
              text: 'معطل',
              onPressed: null,
            ),
          ),
        ),
      );

      // Assert
      final button = tester.widget<IslamicButton>(find.byType(IslamicButton));
      expect(button.onPressed, isNull);
    });

    testWidgets('should display icon when provided', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicButton(
              text: 'مع أيقونة',
              icon: Icons.mosque,
              onPressed: () {},
            ),
          ),
        ),
      );

      // Assert
      expect(find.byIcon(Icons.mosque), findsOneWidget);
    });

    testWidgets('should apply primary style correctly', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicButton(
              text: 'أساسي',
              type: IslamicButtonType.primary,
              onPressed: () {},
            ),
          ),
        ),
      );

      // Assert
      final container = tester.widget<Container>(
        find.descendant(
          of: find.byType(IslamicButton),
          matching: find.byType(Container),
        ).first,
      );

      expect(container.decoration, isA<BoxDecoration>());
    });

    testWidgets('should apply secondary style correctly', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicButton(
              text: 'ثانوي',
              type: IslamicButtonType.secondary,
              onPressed: () {},
            ),
          ),
        ),
      );

      // Assert
      expect(find.byType(IslamicButton), findsOneWidget);
    });

    testWidgets('should apply outline style correctly', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicButton(
              text: 'محدد',
              type: IslamicButtonType.outline,
              onPressed: () {},
            ),
          ),
        ),
      );

      // Assert
      expect(find.byType(IslamicButton), findsOneWidget);
    });

    testWidgets('should have correct padding', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicButton(
              text: 'زر',
              onPressed: () {},
            ),
          ),
        ),
      );

      // Assert
      final padding = tester.widget<Padding>(
        find.descendant(
          of: find.byType(IslamicButton),
          matching: find.byType(Padding),
        ).first,
      );

      expect(padding.padding, isA<EdgeInsets>());
    });

    testWidgets('should have rounded corners', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicButton(
              text: 'زر',
              onPressed: () {},
            ),
          ),
        ),
      );

      // Assert
      final container = tester.widget<Container>(
        find.descendant(
          of: find.byType(IslamicButton),
          matching: find.byType(Container),
        ).first,
      );

      final decoration = container.decoration as BoxDecoration;
      expect(decoration.borderRadius, isA<BorderRadius>());
    });

    testWidgets('should show loading indicator when loading', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicButton(
              text: 'جاري التحميل',
              isLoading: true,
              onPressed: () {},
            ),
          ),
        ),
      );

      // Assert
      expect(find.byType(CircularProgressIndicator), findsOneWidget);
    });

    testWidgets('should not call onPressed when loading', (WidgetTester tester) async {
      // Arrange
      var wasPressed = false;

      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicButton(
              text: 'جاري التحميل',
              isLoading: true,
              onPressed: () {
                wasPressed = true;
              },
            ),
          ),
        ),
      );

      await tester.tap(find.byType(IslamicButton));
      await tester.pump();

      // Assert
      expect(wasPressed, isFalse);
    });

    testWidgets('should have correct text direction for Arabic', (WidgetTester tester) async {
      // Act
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: IslamicButton(
              text: 'نص عربي',
              onPressed: () {},
            ),
          ),
        ),
      );

      // Assert
      final text = tester.widget<Text>(find.text('نص عربي'));
      expect(text.textDirection, equals(TextDirection.rtl));
    });
  });
}

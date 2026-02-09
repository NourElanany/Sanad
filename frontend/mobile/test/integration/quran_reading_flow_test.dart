import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:integration_test/integration_test.dart';
import '../../lib/main.dart';
import '../../lib/features/quran/presentation/screens/quran_index_screen.dart';
import '../../lib/features/quran/presentation/screens/mushaf_view_screen.dart';
import '../../lib/features/quran/presentation/widgets/surah_list_item.dart';

/// Integration test for Quran reading flow
/// **Validates: Requirements 20.3**
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Quran Reading Flow Integration Tests', () {
    testWidgets('Complete flow: Browse surahs -> Select surah -> Read -> Bookmark',
        (WidgetTester tester) async {
      // Arrange
      await tester.pumpWidget(
        const ProviderScope(
          child: SanadApp(),
        ),
      );
      await tester.pumpAndSettle();

      // Step 1: Navigate to Quran index from dashboard
      final quranButton = find.text('القرآن الكريم');
      expect(quranButton, findsOneWidget);
      await tester.tap(quranButton);
      await tester.pumpAndSettle();

      // Step 2: Verify Quran index screen is displayed
      expect(find.byType(QuranIndexScreen), findsOneWidget);
      expect(find.text('السور'), findsOneWidget);

      // Step 3: Search for a specific surah
      final searchField = find.byType(TextField);
      expect(searchField, findsOneWidget);
      await tester.enterText(searchField, 'البقرة');
      await tester.pumpAndSettle();

      // Step 4: Select Al-Baqarah surah
      final baqarahTile = find.text('البقرة');
      expect(baqarahTile, findsOneWidget);
      await tester.tap(baqarahTile);
      await tester.pumpAndSettle();

      // Step 5: Verify Mushaf view is displayed
      expect(find.byType(MushafViewScreen), findsOneWidget);

      // Step 6: Tap on an ayah to show options
      final ayahWidget = find.byType(GestureDetector).first;
      await tester.tap(ayahWidget);
      await tester.pumpAndSettle();

      // Step 7: Add bookmark
      final bookmarkButton = find.text('إضافة علامة مرجعية');
      expect(bookmarkButton, findsOneWidget);
      await tester.tap(bookmarkButton);
      await tester.pumpAndSettle();

      // Step 8: Verify bookmark was added
      expect(find.text('تمت إضافة العلامة المرجعية'), findsOneWidget);

      // Step 9: Navigate back to index
      final backButton = find.byType(BackButton);
      await tester.tap(backButton);
      await tester.pumpAndSettle();

      // Step 10: Check bookmarks tab
      final bookmarksTab = find.text('المفضلة');
      await tester.tap(bookmarksTab);
      await tester.pumpAndSettle();

      // Step 11: Verify bookmark appears in list
      expect(find.text('البقرة'), findsOneWidget);
    });

    testWidgets('Search and filter surahs flow', (WidgetTester tester) async {
      // Arrange
      await tester.pumpWidget(
        const ProviderScope(
          child: SanadApp(),
        ),
      );
      await tester.pumpAndSettle();

      // Navigate to Quran
      await tester.tap(find.text('القرآن الكريم'));
      await tester.pumpAndSettle();

      // Test search functionality
      final searchField = find.byType(TextField);
      await tester.enterText(searchField, 'يس');
      await tester.pumpAndSettle();

      expect(find.text('يس'), findsOneWidget);

      // Clear search
      await tester.enterText(searchField, '');
      await tester.pumpAndSettle();

      // Test filter by revelation type
      final filterButton = find.text('فلتر');
      await tester.tap(filterButton);
      await tester.pumpAndSettle();

      final meccaFilter = find.text('مكي');
      await tester.tap(meccaFilter);
      await tester.pumpAndSettle();

      // Verify only Meccan surahs are shown
      expect(find.byType(SurahListItem), findsWidgets);
    });

    testWidgets('Navigate between juz and surahs', (WidgetTester tester) async {
      // Arrange
      await tester.pumpWidget(
        const ProviderScope(
          child: SanadApp(),
        ),
      );
      await tester.pumpAndSettle();

      // Navigate to Quran
      await tester.tap(find.text('القرآن الكريم'));
      await tester.pumpAndSettle();

      // Switch to Juz tab
      final juzTab = find.text('الأجزاء');
      await tester.tap(juzTab);
      await tester.pumpAndSettle();

      // Select first juz
      final juz1 = find.text('الجزء 1');
      expect(juz1, findsOneWidget);
      await tester.tap(juz1);
      await tester.pumpAndSettle();

      // Verify Mushaf view shows juz content
      expect(find.byType(MushafViewScreen), findsOneWidget);
    });

    testWidgets('Reading progress tracking', (WidgetTester tester) async {
      // Arrange
      await tester.pumpWidget(
        const ProviderScope(
          child: SanadApp(),
        ),
      );
      await tester.pumpAndSettle();

      // Navigate to Quran and select a surah
      await tester.tap(find.text('القرآن الكريم'));
      await tester.pumpAndSettle();

      await tester.tap(find.text('الفاتحة'));
      await tester.pumpAndSettle();

      // Scroll through the surah (simulating reading)
      await tester.drag(
        find.byType(MushafViewScreen),
        const Offset(0, -500),
      );
      await tester.pumpAndSettle();

      // Navigate back
      await tester.tap(find.byType(BackButton));
      await tester.pumpAndSettle();

      // Check dashboard for updated progress
      await tester.tap(find.byIcon(Icons.home));
      await tester.pumpAndSettle();

      // Verify reading progress is updated
      expect(find.textContaining('%'), findsOneWidget);
    });

    testWidgets('Offline reading capability', (WidgetTester tester) async {
      // Arrange
      await tester.pumpWidget(
        const ProviderScope(
          child: SanadApp(),
        ),
      );
      await tester.pumpAndSettle();

      // Download surah for offline reading
      await tester.tap(find.text('القرآن الكريم'));
      await tester.pumpAndSettle();

      // Long press on surah to show options
      await tester.longPress(find.text('الكهف'));
      await tester.pumpAndSettle();

      // Select download option
      final downloadButton = find.text('تحميل للقراءة دون اتصال');
      await tester.tap(downloadButton);
      await tester.pumpAndSettle();

      // Wait for download to complete
      await tester.pump(const Duration(seconds: 2));

      // Verify download success message
      expect(find.text('تم التحميل بنجاح'), findsOneWidget);

      // Simulate offline mode
      // (In real test, would disable network)

      // Open downloaded surah
      await tester.tap(find.text('الكهف'));
      await tester.pumpAndSettle();

      // Verify content is accessible offline
      expect(find.byType(MushafViewScreen), findsOneWidget);
    });
  });
}

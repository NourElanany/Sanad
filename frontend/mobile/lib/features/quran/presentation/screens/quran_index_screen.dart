import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/providers/quran_provider.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../../../core/widgets/islamic_loading_indicator.dart';
import '../widgets/surah_list_item.dart';
import '../widgets/juz_list_item.dart';
import '../widgets/bookmark_list_item.dart';
import '../widgets/quran_search_bar.dart';
import '../widgets/quran_filter_sheet.dart';

/// Main Quran index screen with tabs for Surahs, Juzs, and Bookmarks
class QuranIndexScreen extends ConsumerStatefulWidget {
  const QuranIndexScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<QuranIndexScreen> createState() => _QuranIndexScreenState();
}

class _QuranIndexScreenState extends ConsumerState<QuranIndexScreen>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  final TextEditingController _searchController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 3, vsync: this);
    
    // Load data on init
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(quranIndexProvider.notifier).loadSurahs();
      ref.read(quranIndexProvider.notifier).loadJuzs();
      ref.read(quranIndexProvider.notifier).loadBookmarks();
    });
  }

  @override
  void dispose() {
    _tabController.dispose();
    _searchController.dispose();
    super.dispose();
  }

  void _showFilterSheet() {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (context) => const QuranFilterSheet(),
    );
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(quranIndexProvider);

    return Scaffold(
      backgroundColor: AppColors.backgroundPrimary,
      appBar: AppBar(
        backgroundColor: AppColors.primaryMain,
        elevation: 0,
        title: Text(
          'القرآن الكريم',
          style: AppTextStyles.h5.copyWith(
            color: Colors.white,
            fontWeight: FontWeight.bold,
          ),
        ),
        centerTitle: true,
        actions: [
          IconButton(
            icon: const Icon(Icons.filter_list, color: Colors.white),
            onPressed: _showFilterSheet,
            tooltip: 'فلاتر البحث',
          ),
          IconButton(
            icon: const Icon(Icons.bookmark, color: Colors.white),
            onPressed: () {
              _tabController.animateTo(2);
            },
            tooltip: 'المفضلة',
          ),
        ],
        bottom: PreferredSize(
          preferredSize: const Size.fromHeight(120),
          child: Column(
            children: [
              // Search bar
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                child: QuranSearchBar(
                  controller: _searchController,
                  onChanged: (query) {
                    ref.read(quranIndexProvider.notifier).setSearchQuery(query);
                  },
                ),
              ),
              // Tabs
              TabBar(
                controller: _tabController,
                indicatorColor: AppColors.accentGold,
                indicatorWeight: 3,
                labelColor: Colors.white,
                unselectedLabelColor: Colors.white70,
                labelStyle: AppTextStyles.subtitle1.copyWith(
                  fontWeight: FontWeight.bold,
                ),
                tabs: const [
                  Tab(text: 'السور'),
                  Tab(text: 'الأجزاء'),
                  Tab(text: 'المفضلة'),
                ],
              ),
            ],
          ),
        ),
      ),
      body: state.isLoading
          ? const Center(child: IslamicLoadingIndicator())
          : state.error != null
              ? _buildErrorWidget(state.error!)
              : TabBarView(
                  controller: _tabController,
                  children: [
                    _buildSurahsList(state),
                    _buildJuzsList(state),
                    _buildBookmarksList(state),
                  ],
                ),
    );
  }

  Widget _buildSurahsList(QuranIndexState state) {
    final surahs = state.filteredSurahs;

    if (surahs.isEmpty) {
      return _buildEmptyState('لا توجد سور');
    }

    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: surahs.length,
      itemBuilder: (context, index) {
        final surah = surahs[index];
        return Padding(
          padding: const EdgeInsets.only(bottom: 12),
          child: SurahListItem(
            surah: surah,
            onTap: () {
              // Navigate to Mushaf view
              // TODO: Implement navigation to reading screen
            },
          ),
        );
      },
    );
  }

  Widget _buildJuzsList(QuranIndexState state) {
    final juzs = state.juzs;

    if (juzs.isEmpty) {
      return _buildEmptyState('لا توجد أجزاء');
    }

    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: juzs.length,
      itemBuilder: (context, index) {
        final juz = juzs[index];
        return Padding(
          padding: const EdgeInsets.only(bottom: 12),
          child: JuzListItem(
            juz: juz,
            onTap: () {
              // Navigate to Juz view
              // TODO: Implement navigation to juz reading screen
            },
          ),
        );
      },
    );
  }

  Widget _buildBookmarksList(QuranIndexState state) {
    final bookmarks = state.bookmarks;

    if (bookmarks.isEmpty) {
      return _buildEmptyState('لا توجد علامات مرجعية');
    }

    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: bookmarks.length,
      itemBuilder: (context, index) {
        final bookmark = bookmarks[index];
        return Padding(
          padding: const EdgeInsets.only(bottom: 12),
          child: BookmarkListItem(
            bookmark: bookmark,
            onTap: () {
              // Navigate to bookmarked position
              // TODO: Implement navigation to bookmarked position
            },
            onDelete: () {
              ref.read(quranIndexProvider.notifier).deleteBookmark(bookmark.id);
            },
          ),
        );
      },
    );
  }

  Widget _buildEmptyState(String message) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            Icons.book_outlined,
            size: 80,
            color: AppColors.textDisabled,
          ),
          const SizedBox(height: 16),
          Text(
            message,
            style: AppTextStyles.h6.copyWith(
              color: AppColors.textSecondary,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildErrorWidget(String error) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.error_outline,
              size: 80,
              color: AppColors.statusError,
            ),
            const SizedBox(height: 16),
            Text(
              'حدث خطأ',
              style: AppTextStyles.h6.copyWith(
                color: AppColors.statusError,
              ),
            ),
            const SizedBox(height: 8),
            Text(
              error,
              style: AppTextStyles.body1.copyWith(
                color: AppColors.textSecondary,
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 24),
            ElevatedButton(
              onPressed: () {
                ref.read(quranIndexProvider.notifier).loadSurahs();
                ref.read(quranIndexProvider.notifier).loadJuzs();
                ref.read(quranIndexProvider.notifier).loadBookmarks();
              },
              style: ElevatedButton.styleFrom(
                backgroundColor: AppColors.primaryMain,
                padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 16),
              ),
              child: Text(
                'إعادة المحاولة',
                style: AppTextStyles.button.copyWith(color: Colors.white),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

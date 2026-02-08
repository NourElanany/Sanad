import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/providers/hadith_provider.dart';
import '../../../../core/theme/app_theme.dart';
import '../widgets/hadith_book_card.dart';
import '../widgets/hadith_search_bar.dart';
import '../widgets/hadith_filters_sheet.dart';

class HadithLibraryScreen extends ConsumerStatefulWidget {
  const HadithLibraryScreen({super.key});

  @override
  ConsumerState<HadithLibraryScreen> createState() => _HadithLibraryScreenState();
}

class _HadithLibraryScreenState extends ConsumerState<HadithLibraryScreen>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  final TextEditingController _searchController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 3, vsync: this);
  }

  @override
  void dispose() {
    _tabController.dispose();
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final hadithBooksAsync = ref.watch(hadithBooksProvider);
    final searchState = ref.watch(hadithSearchProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'مكتبة الأحاديث',
          style: TextStyle(
            fontFamily: 'Tajawal',
            fontWeight: FontWeight.bold,
          ),
        ),
        centerTitle: true,
        actions: [
          IconButton(
            icon: const Icon(Icons.filter_list),
            onPressed: () => _showFiltersSheet(context),
            tooltip: 'الفلاتر',
          ),
        ],
        bottom: TabBar(
          controller: _tabController,
          tabs: const [
            Tab(text: 'المجموعات'),
            Tab(text: 'المواضيع'),
            Tab(text: 'الرواة'),
          ],
        ),
      ),
      body: Column(
        children: [
          // Search Bar
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: HadithSearchBar(
              controller: _searchController,
              onSearch: (query) {
                ref.read(hadithSearchProvider.notifier).search(query);
              },
              onClear: () {
                _searchController.clear();
                ref.read(hadithSearchProvider.notifier).clearSearch();
              },
            ),
          ),

          // Active Filters Display
          if (searchState.filters.books.isNotEmpty ||
              searchState.filters.grades.isNotEmpty ||
              searchState.filters.themes.isNotEmpty)
            _buildActiveFilters(searchState),

          // Content
          Expanded(
            child: searchState.query.isNotEmpty
                ? _buildSearchResults(searchState)
                : TabBarView(
                    controller: _tabController,
                    children: [
                      _buildBooksTab(hadithBooksAsync),
                      _buildTopicsTab(),
                      _buildNarratorsTab(),
                    ],
                  ),
          ),
        ],
      ),
    );
  }

  Widget _buildActiveFilters(HadithSearchState searchState) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: Wrap(
        spacing: 8,
        runSpacing: 8,
        children: [
          ...searchState.filters.books.map((book) => _buildFilterChip(
                label: book,
                onDeleted: () => ref.read(hadithSearchProvider.notifier).toggleBook(book),
              )),
          ...searchState.filters.grades.map((grade) => _buildFilterChip(
                label: grade.arabicName,
                onDeleted: () => ref.read(hadithSearchProvider.notifier).toggleGrade(grade),
                color: _getGradeColor(grade),
              )),
          ...searchState.filters.themes.map((theme) => _buildFilterChip(
                label: theme,
                onDeleted: () => ref.read(hadithSearchProvider.notifier).toggleTheme(theme),
              )),
        ],
      ),
    );
  }

  Widget _buildFilterChip({
    required String label,
    required VoidCallback onDeleted,
    Color? color,
  }) {
    return Chip(
      label: Text(
        label,
        style: const TextStyle(fontSize: 12),
      ),
      backgroundColor: color ?? AppTheme.primaryColor.withOpacity(0.1),
      deleteIcon: const Icon(Icons.close, size: 16),
      onDeleted: onDeleted,
    );
  }

  Widget _buildSearchResults(HadithSearchState searchState) {
    if (searchState.isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (searchState.error != null) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.error_outline, size: 64, color: Colors.red),
            const SizedBox(height: 16),
            Text(
              'حدث خطأ في البحث',
              style: Theme.of(context).textTheme.titleLarge,
            ),
            const SizedBox(height: 8),
            Text(
              searchState.error!,
              textAlign: TextAlign.center,
              style: Theme.of(context).textTheme.bodyMedium,
            ),
          ],
        ),
      );
    }

    if (searchState.results.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.search_off, size: 64, color: Colors.grey),
            const SizedBox(height: 16),
            Text(
              'لا توجد نتائج',
              style: Theme.of(context).textTheme.titleLarge,
            ),
            const SizedBox(height: 8),
            Text(
              'جرب البحث بكلمات مختلفة',
              style: Theme.of(context).textTheme.bodyMedium,
            ),
          ],
        ),
      );
    }

    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: searchState.results.length,
      itemBuilder: (context, index) {
        final result = searchState.results[index];
        return _buildSearchResultCard(result);
      },
    );
  }

  Widget _buildSearchResultCard(HadithSearchResultModel result) {
    return Card(
      margin: const EdgeInsets.only(bottom: 16),
      child: InkWell(
        onTap: () => _navigateToHadithDetails(result.hadith.id),
        borderRadius: BorderRadius.circular(12),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              // Header with book and grade
              Row(
                children: [
                  Expanded(
                    child: Text(
                      result.book.arabicName,
                      style: const TextStyle(
                        fontWeight: FontWeight.bold,
                        fontSize: 14,
                      ),
                    ),
                  ),
                  _buildGradeBadge(result.hadith.grade),
                ],
              ),
              const SizedBox(height: 12),

              // Hadith text (highlighted)
              Text(
                result.highlightedText,
                style: const TextStyle(
                  fontSize: 16,
                  height: 1.8,
                  fontFamily: 'Amiri',
                ),
                textDirection: TextDirection.rtl,
                maxLines: 4,
                overflow: TextOverflow.ellipsis,
              ),
              const SizedBox(height: 12),

              // Metadata
              Row(
                children: [
                  Icon(Icons.person, size: 16, color: Colors.grey[600]),
                  const SizedBox(width: 4),
                  Expanded(
                    child: Text(
                      result.hadith.narrator,
                      style: TextStyle(
                        fontSize: 12,
                        color: Colors.grey[600],
                      ),
                    ),
                  ),
                  Text(
                    'رقم ${result.hadith.hadithNumber}',
                    style: TextStyle(
                      fontSize: 12,
                      color: Colors.grey[600],
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildBooksTab(AsyncValue<List<HadithBookModel>> hadithBooksAsync) {
    return hadithBooksAsync.when(
      data: (books) {
        if (books.isEmpty) {
          return const Center(
            child: Text('لا توجد مجموعات أحاديث'),
          );
        }

        return ListView.builder(
          padding: const EdgeInsets.all(16),
          itemCount: books.length,
          itemBuilder: (context, index) {
            final book = books[index];
            return HadithBookCard(
              book: book,
              onTap: () => _navigateToBookHadiths(book),
            );
          },
        );
      },
      loading: () => const Center(child: CircularProgressIndicator()),
      error: (error, stack) => Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.error_outline, size: 64, color: Colors.red),
            const SizedBox(height: 16),
            Text('حدث خطأ: $error'),
          ],
        ),
      ),
    );
  }

  Widget _buildTopicsTab() {
    final topics = [
      'عقيدة',
      'عبادة',
      'معاملات',
      'أسرة',
      'أخلاق',
      'تاريخ',
      'نبوءات',
      'فقه',
    ];

    return GridView.builder(
      padding: const EdgeInsets.all(16),
      gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: 2,
        crossAxisSpacing: 16,
        mainAxisSpacing: 16,
        childAspectRatio: 1.5,
      ),
      itemCount: topics.length,
      itemBuilder: (context, index) {
        final topic = topics[index];
        return _buildTopicCard(topic);
      },
    );
  }

  Widget _buildTopicCard(String topic) {
    return Card(
      child: InkWell(
        onTap: () => _navigateToTopicHadiths(topic),
        borderRadius: BorderRadius.circular(12),
        child: Center(
          child: Text(
            topic,
            style: const TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildNarratorsTab() {
    final narrators = [
      'أبو هريرة',
      'عائشة',
      'ابن عمر',
      'أنس بن مالك',
      'جابر بن عبد الله',
      'أبو سعيد الخدري',
    ];

    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: narrators.length,
      itemBuilder: (context, index) {
        final narrator = narrators[index];
        return Card(
          margin: const EdgeInsets.only(bottom: 12),
          child: ListTile(
            leading: const CircleAvatar(
              child: Icon(Icons.person),
            ),
            title: Text(
              narrator,
              style: const TextStyle(
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            trailing: const Icon(Icons.arrow_forward_ios, size: 16),
            onTap: () => _navigateToNarratorHadiths(narrator),
          ),
        );
      },
    );
  }

  Widget _buildGradeBadge(HadithGrade grade) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        color: _getGradeColor(grade),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Text(
        grade.arabicName,
        style: const TextStyle(
          color: Colors.white,
          fontSize: 12,
          fontWeight: FontWeight.bold,
        ),
      ),
    );
  }

  Color _getGradeColor(HadithGrade grade) {
    switch (grade) {
      case HadithGrade.sahih:
        return Colors.green;
      case HadithGrade.hasan:
        return Colors.amber;
      case HadithGrade.daif:
        return Colors.orange;
      case HadithGrade.mawdu:
        return Colors.red;
    }
  }

  void _showFiltersSheet(BuildContext context) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (context) => const HadithFiltersSheet(),
    );
  }

  void _navigateToHadithDetails(String hadithId) {
    Navigator.pushNamed(
      context,
      '/hadith-details',
      arguments: hadithId,
    );
  }

  void _navigateToBookHadiths(HadithBookModel book) {
    Navigator.pushNamed(
      context,
      '/book-hadiths',
      arguments: book,
    );
  }

  void _navigateToTopicHadiths(String topic) {
    Navigator.pushNamed(
      context,
      '/topic-hadiths',
      arguments: topic,
    );
  }

  void _navigateToNarratorHadiths(String narrator) {
    ref.read(hadithSearchProvider.notifier).search(narrator);
    ref.read(hadithSearchProvider.notifier).setSearchType('narrator');
  }
}

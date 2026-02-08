import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/providers/stories_provider.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../../../core/widgets/islamic_loading_indicator.dart';
import '../../data/models/story_model.dart';
import '../widgets/story_category_card.dart';
import '../widgets/story_list_item.dart';
import 'story_details_screen.dart';

/// Islamic Stories Library Screen
class StoriesLibraryScreen extends ConsumerStatefulWidget {
  const StoriesLibraryScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<StoriesLibraryScreen> createState() =>
      _StoriesLibraryScreenState();
}

class _StoriesLibraryScreenState extends ConsumerState<StoriesLibraryScreen>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  StoryCategory? _selectedCategory;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 3, vsync: this);
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'مكتبة القصص الإسلامية',
          style: TextStyle(
            fontFamily: 'Tajawal',
            fontWeight: FontWeight.bold,
          ),
        ),
        centerTitle: true,
        actions: [
          IconButton(
            icon: const Icon(Icons.search),
            onPressed: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => const StorySearchScreen(),
                ),
              );
            },
          ),
        ],
        bottom: TabBar(
          controller: _tabController,
          tabs: const [
            Tab(text: 'التصنيفات'),
            Tab(text: 'الشخصيات'),
            Tab(text: 'الدروس'),
          ],
        ),
      ),
      body: TabBarView(
        controller: _tabController,
        children: [
          _buildCategoriesTab(),
          _buildCharactersTab(),
          _buildLessonsTab(),
        ],
      ),
    );
  }

  Widget _buildCategoriesTab() {
    if (_selectedCategory != null) {
      return _buildCategoryStories(_selectedCategory!);
    }

    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'اختر تصنيفاً',
            style: TextStyle(
              fontSize: 20,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
            ),
          ),
          const SizedBox(height: 16),
          _buildCategoryGrid(),
          const SizedBox(height: 24),
          _buildStatisticsSection(),
        ],
      ),
    );
  }

  Widget _buildCategoryGrid() {
    final categories = StoryCategory.values;

    return GridView.builder(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: 2,
        crossAxisSpacing: 12,
        mainAxisSpacing: 12,
        childAspectRatio: 1.2,
      ),
      itemCount: categories.length,
      itemBuilder: (context, index) {
        final category = categories[index];
        return StoryCategoryCard(
          category: category,
          onTap: () {
            setState(() {
              _selectedCategory = category;
            });
          },
        );
      },
    );
  }

  Widget _buildStatisticsSection() {
    final statsAsync = ref.watch(categoryStatisticsProvider);

    return statsAsync.when(
      data: (stats) {
        return IslamicCard(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text(
                'إحصائيات المكتبة',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Tajawal',
                ),
              ),
              const SizedBox(height: 12),
              ...stats.entries.map((entry) {
                return Padding(
                  padding: const EdgeInsets.symmetric(vertical: 4),
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text(
                        _getCategoryArabicName(entry.key),
                        style: const TextStyle(fontFamily: 'Tajawal'),
                      ),
                      Text(
                        '${entry.value} قصة',
                        style: const TextStyle(
                          fontWeight: FontWeight.bold,
                          fontFamily: 'Tajawal',
                        ),
                      ),
                    ],
                  ),
                );
              }).toList(),
            ],
          ),
        );
      },
      loading: () => const Center(child: IslamicLoadingIndicator()),
      error: (error, stack) => Center(
        child: Text('خطأ في تحميل الإحصائيات: $error'),
      ),
    );
  }

  Widget _buildCategoryStories(StoryCategory category) {
    final storiesState = ref.watch(storiesByCategoryProvider(category));

    return Column(
      children: [
        Container(
          padding: const EdgeInsets.all(16),
          color: Theme.of(context).primaryColor.withOpacity(0.1),
          child: Row(
            children: [
              IconButton(
                icon: const Icon(Icons.arrow_back),
                onPressed: () {
                  setState(() {
                    _selectedCategory = null;
                  });
                },
              ),
              const SizedBox(width: 8),
              Text(
                category.arabicName,
                style: const TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Tajawal',
                ),
              ),
              const Spacer(),
              Text(
                category.icon,
                style: const TextStyle(fontSize: 32),
              ),
            ],
          ),
        ),
        Expanded(
          child: _buildStoriesList(storiesState, category),
        ),
      ],
    );
  }

  Widget _buildStoriesList(StoriesState state, StoryCategory category) {
    if (state.isLoading && state.stories.isEmpty) {
      return const Center(child: IslamicLoadingIndicator());
    }

    if (state.error != null && state.stories.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.error_outline, size: 64, color: Colors.red),
            const SizedBox(height: 16),
            Text(
              'خطأ في تحميل القصص',
              style: const TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 8),
            Text(
              state.error!,
              textAlign: TextAlign.center,
              style: const TextStyle(fontFamily: 'Tajawal'),
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: () {
                ref.read(storiesByCategoryProvider(category).notifier).refresh();
              },
              child: const Text('إعادة المحاولة'),
            ),
          ],
        ),
      );
    }

    if (state.stories.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.book_outlined, size: 64, color: Colors.grey),
            const SizedBox(height: 16),
            const Text(
              'لا توجد قصص في هذا التصنيف',
              style: TextStyle(
                fontSize: 18,
                fontFamily: 'Tajawal',
              ),
            ),
          ],
        ),
      );
    }

    return RefreshIndicator(
      onRefresh: () =>
          ref.read(storiesByCategoryProvider(category).notifier).refresh(),
      child: ListView.builder(
        padding: const EdgeInsets.all(16),
        itemCount: state.stories.length + (state.hasMore ? 1 : 0),
        itemBuilder: (context, index) {
          if (index == state.stories.length) {
            // Load more indicator
            if (state.isLoading) {
              return const Padding(
                padding: EdgeInsets.all(16),
                child: Center(child: IslamicLoadingIndicator()),
              );
            } else {
              // Trigger load more
              Future.microtask(() {
                ref.read(storiesByCategoryProvider(category).notifier).loadMore();
              });
              return const SizedBox.shrink();
            }
          }

          final story = state.stories[index];
          return StoryListItem(
            story: story,
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => StoryDetailsScreen(storyId: story.id),
                ),
              );
            },
          );
        },
      ),
    );
  }

  Widget _buildCharactersTab() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Icon(Icons.person_outline, size: 64, color: Colors.grey),
          const SizedBox(height: 16),
          const Text(
            'قريباً: تصفح القصص حسب الشخصيات',
            style: TextStyle(
              fontSize: 18,
              fontFamily: 'Tajawal',
            ),
          ),
          const SizedBox(height: 8),
          const Text(
            'ابحث عن قصص الأنبياء والصحابة والشخصيات التاريخية',
            textAlign: TextAlign.center,
            style: TextStyle(
              color: Colors.grey,
              fontFamily: 'Tajawal',
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildLessonsTab() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Icon(Icons.lightbulb_outline, size: 64, color: Colors.grey),
          const SizedBox(height: 16),
          const Text(
            'قريباً: تصفح القصص حسب الدروس المستفادة',
            style: TextStyle(
              fontSize: 18,
              fontFamily: 'Tajawal',
            ),
          ),
          const SizedBox(height: 8),
          const Text(
            'ابحث عن قصص تعلم الصبر، الشجاعة، الأمانة، وغيرها',
            textAlign: TextAlign.center,
            style: TextStyle(
              color: Colors.grey,
              fontFamily: 'Tajawal',
            ),
          ),
        ],
      ),
    );
  }

  String _getCategoryArabicName(String categoryKey) {
    try {
      final category = StoryCategory.values.firstWhere(
        (c) => c.name == categoryKey,
      );
      return category.arabicName;
    } catch (e) {
      return categoryKey;
    }
  }
}

/// Story Search Screen
class StorySearchScreen extends ConsumerStatefulWidget {
  const StorySearchScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<StorySearchScreen> createState() => _StorySearchScreenState();
}

class _StorySearchScreenState extends ConsumerState<StorySearchScreen> {
  final TextEditingController _searchController = TextEditingController();

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final searchState = ref.watch(storySearchProvider);

    return Scaffold(
      appBar: AppBar(
        title: TextField(
          controller: _searchController,
          autofocus: true,
          decoration: const InputDecoration(
            hintText: 'ابحث في القصص...',
            border: InputBorder.none,
            hintStyle: TextStyle(fontFamily: 'Tajawal'),
          ),
          style: const TextStyle(fontFamily: 'Tajawal'),
          textDirection: TextDirection.rtl,
          onSubmitted: (query) {
            ref.read(storySearchProvider.notifier).search(query);
          },
        ),
        actions: [
          if (_searchController.text.isNotEmpty)
            IconButton(
              icon: const Icon(Icons.clear),
              onPressed: () {
                _searchController.clear();
                ref.read(storySearchProvider.notifier).clear();
              },
            ),
        ],
      ),
      body: _buildSearchResults(searchState),
    );
  }

  Widget _buildSearchResults(StorySearchState state) {
    if (state.query.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: const [
            Icon(Icons.search, size: 64, color: Colors.grey),
            SizedBox(height: 16),
            Text(
              'ابحث عن قصة إسلامية',
              style: TextStyle(
                fontSize: 18,
                fontFamily: 'Tajawal',
              ),
            ),
          ],
        ),
      );
    }

    if (state.isLoading) {
      return const Center(child: IslamicLoadingIndicator());
    }

    if (state.error != null) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.error_outline, size: 64, color: Colors.red),
            const SizedBox(height: 16),
            Text(
              'خطأ في البحث',
              style: const TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 8),
            Text(
              state.error!,
              textAlign: TextAlign.center,
              style: const TextStyle(fontFamily: 'Tajawal'),
            ),
          ],
        ),
      );
    }

    if (state.results.isEmpty) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: const [
            Icon(Icons.search_off, size: 64, color: Colors.grey),
            SizedBox(height: 16),
            Text(
              'لم يتم العثور على نتائج',
              style: TextStyle(
                fontSize: 18,
                fontFamily: 'Tajawal',
              ),
            ),
          ],
        ),
      );
    }

    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: state.results.length,
      itemBuilder: (context, index) {
        final story = state.results[index];
        return StoryListItem(
          story: story,
          onTap: () {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => StoryDetailsScreen(storyId: story.id),
              ),
            );
          },
        );
      },
    );
  }
}

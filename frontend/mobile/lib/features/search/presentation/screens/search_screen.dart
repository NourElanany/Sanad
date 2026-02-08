/// Comprehensive search screen with smart search bar, filters, and voice search
/// Requirements: 8.1, 8.2, 8.3, 8.4, 8.5

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/providers/search_provider.dart';
import '../../../../core/theme/app_theme.dart';
import '../widgets/search_bar_widget.dart';
import '../widgets/search_filters_sheet.dart';
import '../widgets/search_result_card.dart';
import '../widgets/search_suggestions_list.dart';
import '../widgets/saved_searches_sheet.dart';
import '../../data/models/search_models.dart';

class SearchScreen extends ConsumerStatefulWidget {
  const SearchScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<SearchScreen> createState() => _SearchScreenState();
}

class _SearchScreenState extends ConsumerState<SearchScreen> {
  final TextEditingController _searchController = TextEditingController();
  final ScrollController _scrollController = ScrollController();
  bool _showSuggestions = false;

  @override
  void initState() {
    super.initState();
    _scrollController.addListener(_onScroll);
    _searchController.addListener(_onSearchTextChanged);
  }

  @override
  void dispose() {
    _searchController.dispose();
    _scrollController.dispose();
    super.dispose();
  }

  void _onScroll() {
    if (_scrollController.position.pixels >=
        _scrollController.position.maxScrollExtent * 0.9) {
      // Load more when scrolled to 90%
      ref.read(searchProvider.notifier).loadMore();
    }
  }

  void _onSearchTextChanged() {
    final query = _searchController.text;
    if (query.length >= 3) {
      ref.read(searchProvider.notifier).getSuggestions(query);
      setState(() => _showSuggestions = true);
    } else {
      setState(() => _showSuggestions = false);
    }
  }

  void _performSearch() {
    final query = _searchController.text.trim();
    if (query.isNotEmpty) {
      setState(() => _showSuggestions = false);
      ref.read(searchProvider.notifier).search(query);
      FocusScope.of(context).unfocus();
    }
  }

  void _showFiltersSheet() {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (context) => const SearchFiltersSheet(),
    );
  }

  void _showSavedSearchesSheet() {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (context) => const SavedSearchesSheet(),
    );
  }

  void _saveCurrentSearch() {
    final state = ref.read(searchProvider);
    if (state.currentQuery.isNotEmpty) {
      showDialog(
        context: context,
        builder: (context) => _SaveSearchDialog(
          query: state.currentQuery,
          filters: state.currentFilters,
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    final searchState = ref.watch(searchProvider);
    final theme = Theme.of(context);

    return Scaffold(
      backgroundColor: AppTheme.background.primary,
      body: SafeArea(
        child: Column(
          children: [
            // Header with search bar
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.white,
                boxShadow: [
                  BoxShadow(
                    color: AppTheme.primary.main.withOpacity(0.08),
                    blurRadius: 8,
                    offset: const Offset(0, 2),
                  ),
                ],
              ),
              child: Column(
                children: [
                  Row(
                    children: [
                      IconButton(
                        icon: const Icon(Icons.arrow_back),
                        onPressed: () => Navigator.pop(context),
                      ),
                      const SizedBox(width: 8),
                      Expanded(
                        child: Text(
                          'البحث الشامل',
                          style: theme.textTheme.headlineSmall?.copyWith(
                            fontWeight: FontWeight.bold,
                            color: AppTheme.primary.main,
                          ),
                        ),
                      ),
                      IconButton(
                        icon: const Icon(Icons.bookmark_border),
                        onPressed: _showSavedSearchesSheet,
                        tooltip: 'البحثات المحفوظة',
                      ),
                      if (searchState.currentQuery.isNotEmpty)
                        IconButton(
                          icon: const Icon(Icons.bookmark_add),
                          onPressed: _saveCurrentSearch,
                          tooltip: 'حفظ البحث',
                        ),
                    ],
                  ),
                  const SizedBox(height: 16),
                  SearchBarWidget(
                    controller: _searchController,
                    onSearch: _performSearch,
                    onFilterTap: _showFiltersSheet,
                    hasActiveFilters: searchState.currentFilters != null,
                  ),
                ],
              ),
            ),

            // Content area
            Expanded(
              child: _buildContent(searchState),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildContent(SearchState state) {
    if (_showSuggestions && state.suggestions.isNotEmpty) {
      return SearchSuggestionsList(
        suggestions: state.suggestions,
        onSuggestionTap: (suggestion) {
          _searchController.text = suggestion.suggestedQuery;
          _performSearch();
        },
      );
    }

    if (state.isLoading && state.response == null) {
      return const Center(child: CircularProgressIndicator());
    }

    if (state.error != null) {
      return _buildErrorState(state.error!);
    }

    if (state.response == null) {
      return _buildEmptyState();
    }

    return _buildResults(state.response!);
  }

  Widget _buildEmptyState() {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(32),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.search,
              size: 80,
              color: AppTheme.primary.main.withOpacity(0.3),
            ),
            const SizedBox(height: 24),
            Text(
              'ابحث في القرآن والحديث والفتاوى',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.w600,
                color: AppTheme.text.primary,
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 12),
            Text(
              'استخدم البحث الذكي للعثور على المحتوى الإسلامي',
              style: TextStyle(
                fontSize: 14,
                color: AppTheme.text.secondary,
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 32),
            _buildQuickSearchButtons(),
          ],
        ),
      ),
    );
  }

  Widget _buildQuickSearchButtons() {
    return Wrap(
      spacing: 12,
      runSpacing: 12,
      alignment: WrapAlignment.center,
      children: [
        _QuickSearchChip(
          label: 'القرآن',
          icon: Icons.menu_book,
          onTap: () {
            ref.read(searchProvider.notifier).updateFilters(
              const SearchFilters(contentTypes: [ContentType.quran]),
            );
          },
        ),
        _QuickSearchChip(
          label: 'الحديث',
          icon: Icons.article,
          onTap: () {
            ref.read(searchProvider.notifier).updateFilters(
              const SearchFilters(contentTypes: [
                ContentType.sahihHadith,
                ContentType.hasanHadith,
              ]),
            );
          },
        ),
        _QuickSearchChip(
          label: 'الفتاوى',
          icon: Icons.gavel,
          onTap: () {
            ref.read(searchProvider.notifier).updateFilters(
              const SearchFilters(contentTypes: [
                ContentType.fiqhRuling,
                ContentType.scholarOpinion,
              ]),
            );
          },
        ),
      ],
    );
  }

  Widget _buildErrorState(String error) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(32),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.error_outline,
              size: 64,
              color: AppTheme.status.error,
            ),
            const SizedBox(height: 16),
            Text(
              'حدث خطأ',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.w600,
                color: AppTheme.text.primary,
              ),
            ),
            const SizedBox(height: 8),
            Text(
              error,
              style: TextStyle(
                fontSize: 14,
                color: AppTheme.text.secondary,
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 24),
            ElevatedButton(
              onPressed: _performSearch,
              child: const Text('إعادة المحاولة'),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildResults(SearchResponse response) {
    if (response.results.isEmpty) {
      return Center(
        child: Padding(
          padding: const EdgeInsets.all(32),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(
                Icons.search_off,
                size: 64,
                color: AppTheme.text.secondary,
              ),
              const SizedBox(height: 16),
              Text(
                'لم يتم العثور على نتائج',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.w600,
                  color: AppTheme.text.primary,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                'جرب كلمات بحث مختلفة أو قم بتعديل الفلاتر',
                style: TextStyle(
                  fontSize: 14,
                  color: AppTheme.text.secondary,
                ),
                textAlign: TextAlign.center,
              ),
            ],
          ),
        ),
      );
    }

    return Column(
      children: [
        // Results header
        Container(
          padding: const EdgeInsets.all(16),
          child: Row(
            children: [
              Text(
                'النتائج: ${response.totalResults}',
                style: TextStyle(
                  fontSize: 14,
                  fontWeight: FontWeight.w600,
                  color: AppTheme.text.secondary,
                ),
              ),
              const Spacer(),
              if (response.fromCache)
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: AppTheme.accent.gold.withOpacity(0.1),
                    borderRadius: BorderRadius.circular(4),
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(
                        Icons.flash_on,
                        size: 14,
                        color: AppTheme.accent.gold,
                      ),
                      const SizedBox(width: 4),
                      Text(
                        'سريع',
                        style: TextStyle(
                          fontSize: 12,
                          color: AppTheme.accent.gold,
                        ),
                      ),
                    ],
                  ),
                ),
              const SizedBox(width: 8),
              Text(
                '${response.searchTimeMs}ms',
                style: TextStyle(
                  fontSize: 12,
                  color: AppTheme.text.secondary,
                ),
              ),
            ],
          ),
        ),

        // Results list
        Expanded(
          child: ListView.builder(
            controller: _scrollController,
            padding: const EdgeInsets.symmetric(horizontal: 16),
            itemCount: response.results.length + 1,
            itemBuilder: (context, index) {
              if (index == response.results.length) {
                // Loading indicator for pagination
                final pagination = response.pagination;
                if (pagination != null && pagination.hasNextPage) {
                  return const Padding(
                    padding: EdgeInsets.all(16),
                    child: Center(child: CircularProgressIndicator()),
                  );
                }
                return const SizedBox(height: 16);
              }

              final result = response.results[index];
              return SearchResultCard(
                result: result,
                onTap: () {
                  // Navigate to detail screen based on content type
                  _navigateToDetail(result);
                },
              );
            },
          ),
        ),
      ],
    );
  }

  void _navigateToDetail(SearchResult result) {
    // TODO: Implement navigation based on content type
    // For now, just show a snackbar
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('عرض تفاصيل: ${result.document.contentType}'),
      ),
    );
  }
}

class _QuickSearchChip extends StatelessWidget {
  final String label;
  final IconData icon;
  final VoidCallback onTap;

  const _QuickSearchChip({
    required this.label,
    required this.icon,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(20),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        decoration: BoxDecoration(
          color: AppTheme.primary.main.withOpacity(0.1),
          borderRadius: BorderRadius.circular(20),
          border: Border.all(
            color: AppTheme.primary.main.withOpacity(0.3),
          ),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(icon, size: 18, color: AppTheme.primary.main),
            const SizedBox(width: 8),
            Text(
              label,
              style: TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.w600,
                color: AppTheme.primary.main,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _SaveSearchDialog extends ConsumerStatefulWidget {
  final String query;
  final SearchFilters? filters;

  const _SaveSearchDialog({
    required this.query,
    this.filters,
  });

  @override
  ConsumerState<_SaveSearchDialog> createState() => _SaveSearchDialogState();
}

class _SaveSearchDialogState extends ConsumerState<_SaveSearchDialog> {
  final TextEditingController _nameController = TextEditingController();

  @override
  void dispose() {
    _nameController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('حفظ البحث'),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('البحث: ${widget.query}'),
          const SizedBox(height: 16),
          TextField(
            controller: _nameController,
            decoration: const InputDecoration(
              labelText: 'اسم البحث (اختياري)',
              border: OutlineInputBorder(),
            ),
          ),
        ],
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('إلغاء'),
        ),
        ElevatedButton(
          onPressed: () {
            ref.read(savedSearchesProvider.notifier).saveSearch(
              widget.query,
              widget.filters,
              name: _nameController.text.isEmpty ? null : _nameController.text,
            );
            Navigator.pop(context);
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(content: Text('تم حفظ البحث')),
            );
          },
          child: const Text('حفظ'),
        ),
      ],
    );
  }
}

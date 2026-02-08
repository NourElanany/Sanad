/// Saved searches bottom sheet
/// Requirements: 8.5

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/theme/app_theme.dart';
import '../../../../core/providers/search_provider.dart';
import 'package:intl/intl.dart' as intl;

class SavedSearchesSheet extends ConsumerStatefulWidget {
  const SavedSearchesSheet({Key? key}) : super(key: key);

  @override
  ConsumerState<SavedSearchesSheet> createState() => _SavedSearchesSheetState();
}

class _SavedSearchesSheetState extends ConsumerState<SavedSearchesSheet> {
  @override
  void initState() {
    super.initState();
    // Load saved searches when sheet opens
    Future.microtask(() {
      ref.read(savedSearchesProvider.notifier).loadSavedSearches();
    });
  }

  @override
  Widget build(BuildContext context) {
    final savedSearchesState = ref.watch(savedSearchesProvider);

    return Container(
      height: MediaQuery.of(context).size.height * 0.7,
      decoration: const BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      child: Column(
        children: [
          // Handle bar
          Container(
            margin: const EdgeInsets.only(top: 12),
            width: 40,
            height: 4,
            decoration: BoxDecoration(
              color: AppTheme.text.secondary.withOpacity(0.3),
              borderRadius: BorderRadius.circular(2),
            ),
          ),

          // Header
          Padding(
            padding: const EdgeInsets.all(20),
            child: Row(
              children: [
                Icon(
                  Icons.bookmark,
                  color: AppTheme.primary.main,
                  size: 24,
                ),
                const SizedBox(width: 12),
                Text(
                  'البحثات المحفوظة',
                  style: TextStyle(
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                    color: AppTheme.primary.main,
                  ),
                ),
                const Spacer(),
                if (savedSearchesState.searches.isNotEmpty)
                  Text(
                    '${savedSearchesState.searches.length}',
                    style: TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                      color: AppTheme.text.secondary,
                    ),
                  ),
              ],
            ),
          ),

          // Content
          Expanded(
            child: _buildContent(savedSearchesState),
          ),
        ],
      ),
    );
  }

  Widget _buildContent(SavedSearchesState state) {
    if (state.isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (state.error != null) {
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
                state.error!,
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

    if (state.searches.isEmpty) {
      return Center(
        child: Padding(
          padding: const EdgeInsets.all(32),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(
                Icons.bookmark_border,
                size: 80,
                color: AppTheme.text.secondary.withOpacity(0.5),
              ),
              const SizedBox(height: 24),
              Text(
                'لا توجد بحثات محفوظة',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.w600,
                  color: AppTheme.text.primary,
                ),
              ),
              const SizedBox(height: 12),
              Text(
                'احفظ بحثاتك المفضلة للوصول السريع',
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

    return ListView.builder(
      padding: const EdgeInsets.symmetric(horizontal: 20),
      itemCount: state.searches.length,
      itemBuilder: (context, index) {
        final search = state.searches[index];
        return _SavedSearchTile(
          search: search,
          onTap: () {
            // Execute the saved search
            ref.read(searchProvider.notifier).search(
              search.query,
              filters: search.filters,
            );
            Navigator.pop(context);
          },
          onDelete: () {
            ref.read(savedSearchesProvider.notifier).deleteSavedSearch(search.id);
          },
        );
      },
    );
  }
}

class _SavedSearchTile extends StatelessWidget {
  final dynamic search; // SavedSearch
  final VoidCallback onTap;
  final VoidCallback onDelete;

  const _SavedSearchTile({
    required this.search,
    required this.onTap,
    required this.onDelete,
  });

  @override
  Widget build(BuildContext context) {
    final dateFormat = intl.DateFormat('dd/MM/yyyy', 'ar');
    final formattedDate = dateFormat.format(search.createdAt);

    return Dismissible(
      key: Key(search.id),
      direction: DismissDirection.endToStart,
      background: Container(
        alignment: Alignment.centerLeft,
        padding: const EdgeInsets.only(left: 20),
        decoration: BoxDecoration(
          color: AppTheme.status.error,
          borderRadius: BorderRadius.circular(12),
        ),
        child: const Icon(
          Icons.delete,
          color: Colors.white,
        ),
      ),
      confirmDismiss: (direction) async {
        return await showDialog(
          context: context,
          builder: (context) => AlertDialog(
            title: const Text('حذف البحث'),
            content: const Text('هل تريد حذف هذا البحث المحفوظ؟'),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(context, false),
                child: const Text('إلغاء'),
              ),
              ElevatedButton(
                onPressed: () => Navigator.pop(context, true),
                style: ElevatedButton.styleFrom(
                  backgroundColor: AppTheme.status.error,
                ),
                child: const Text('حذف'),
              ),
            ],
          ),
        );
      },
      onDismissed: (_) => onDelete(),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(12),
        child: Container(
          padding: const EdgeInsets.all(16),
          margin: const EdgeInsets.only(bottom: 12),
          decoration: BoxDecoration(
            color: AppTheme.background.secondary,
            borderRadius: BorderRadius.circular(12),
            border: Border.all(
              color: AppTheme.primary.main.withOpacity(0.1),
            ),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                children: [
                  Expanded(
                    child: Text(
                      search.name ?? search.query,
                      style: TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.w600,
                        color: AppTheme.text.primary,
                      ),
                      maxLines: 2,
                      overflow: TextOverflow.ellipsis,
                      textDirection: TextDirection.rtl,
                    ),
                  ),
                  const SizedBox(width: 12),
                  Icon(
                    Icons.arrow_forward_ios,
                    size: 16,
                    color: AppTheme.text.secondary,
                  ),
                ],
              ),
              if (search.name != null) ...[
                const SizedBox(height: 8),
                Text(
                  search.query,
                  style: TextStyle(
                    fontSize: 14,
                    color: AppTheme.text.secondary,
                  ),
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  textDirection: TextDirection.rtl,
                ),
              ],
              const SizedBox(height: 12),
              Row(
                children: [
                  Icon(
                    Icons.calendar_today,
                    size: 14,
                    color: AppTheme.text.secondary,
                  ),
                  const SizedBox(width: 6),
                  Text(
                    formattedDate,
                    style: TextStyle(
                      fontSize: 12,
                      color: AppTheme.text.secondary,
                    ),
                  ),
                  if (search.filters != null) ...[
                    const SizedBox(width: 16),
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
                            Icons.filter_list,
                            size: 12,
                            color: AppTheme.accent.gold,
                          ),
                          const SizedBox(width: 4),
                          Text(
                            'مع فلاتر',
                            style: TextStyle(
                              fontSize: 11,
                              color: AppTheme.accent.gold,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                        ],
                      ),
                    ),
                  ],
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

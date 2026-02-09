import 'package:flutter/material.dart';
import '../services/animation_service.dart';

/// Lazy loading list widget for efficient content rendering
/// Implements pagination and smooth scrolling for large datasets
class LazyLoadingList<T> extends StatefulWidget {
  final Future<List<T>> Function(int page, int pageSize) onLoadMore;
  final Widget Function(BuildContext context, T item, int index) itemBuilder;
  final Widget? loadingWidget;
  final Widget? emptyWidget;
  final Widget? errorWidget;
  final int pageSize;
  final bool enableAnimation;
  final ScrollController? scrollController;
  final EdgeInsetsGeometry? padding;
  final double loadMoreThreshold;

  const LazyLoadingList({
    Key? key,
    required this.onLoadMore,
    required this.itemBuilder,
    this.loadingWidget,
    this.emptyWidget,
    this.errorWidget,
    this.pageSize = 20,
    this.enableAnimation = true,
    this.scrollController,
    this.padding,
    this.loadMoreThreshold = 200.0,
  }) : super(key: key);

  @override
  State<LazyLoadingList<T>> createState() => _LazyLoadingListState<T>();
}

class _LazyLoadingListState<T> extends State<LazyLoadingList<T>> {
  final List<T> _items = [];
  late ScrollController _scrollController;
  bool _isLoading = false;
  bool _hasMore = true;
  int _currentPage = 0;
  String? _error;

  @override
  void initState() {
    super.initState();
    _scrollController = widget.scrollController ?? ScrollController();
    _scrollController.addListener(_onScroll);
    _loadInitialData();
  }

  @override
  void dispose() {
    if (widget.scrollController == null) {
      _scrollController.dispose();
    }
    super.dispose();
  }

  void _onScroll() {
    if (_isLoading || !_hasMore) return;

    final maxScroll = _scrollController.position.maxScrollExtent;
    final currentScroll = _scrollController.position.pixels;
    
    if (maxScroll - currentScroll <= widget.loadMoreThreshold) {
      _loadMore();
    }
  }

  Future<void> _loadInitialData() async {
    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      final items = await widget.onLoadMore(0, widget.pageSize);
      
      if (mounted) {
        setState(() {
          _items.clear();
          _items.addAll(items);
          _currentPage = 0;
          _hasMore = items.length >= widget.pageSize;
          _isLoading = false;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _error = e.toString();
          _isLoading = false;
        });
      }
    }
  }

  Future<void> _loadMore() async {
    if (_isLoading || !_hasMore) return;

    setState(() {
      _isLoading = true;
    });

    try {
      final nextPage = _currentPage + 1;
      final items = await widget.onLoadMore(nextPage, widget.pageSize);
      
      if (mounted) {
        setState(() {
          _items.addAll(items);
          _currentPage = nextPage;
          _hasMore = items.length >= widget.pageSize;
          _isLoading = false;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isLoading = false;
        });
        
        // Show error snackbar
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('فشل تحميل المزيد: $e'),
            backgroundColor: const Color(0xFFDC3545),
          ),
        );
      }
    }
  }

  Future<void> refresh() async {
    await _loadInitialData();
  }

  @override
  Widget build(BuildContext context) {
    // Show error state
    if (_error != null && _items.isEmpty) {
      return widget.errorWidget ?? _buildErrorWidget();
    }

    // Show empty state
    if (!_isLoading && _items.isEmpty) {
      return widget.emptyWidget ?? _buildEmptyWidget();
    }

    // Show loading state for initial load
    if (_isLoading && _items.isEmpty) {
      return widget.loadingWidget ?? _buildLoadingWidget();
    }

    // Show list with items
    return RefreshIndicator(
      onRefresh: refresh,
      color: const Color(0xFF1B365D),
      child: ListView.builder(
        controller: _scrollController,
        padding: widget.padding ?? const EdgeInsets.all(16),
        itemCount: _items.length + (_hasMore ? 1 : 0),
        itemBuilder: (context, index) {
          // Show loading indicator at the end
          if (index >= _items.length) {
            return _buildLoadMoreIndicator();
          }

          final item = _items[index];
          
          // Wrap with animation if enabled
          if (widget.enableAnimation && index < 10) {
            return AnimatedListItem(
              index: index,
              child: widget.itemBuilder(context, item, index),
            );
          }
          
          return widget.itemBuilder(context, item, index);
        },
      ),
    );
  }

  Widget _buildLoadingWidget() {
    return const Center(
      child: CircularProgressIndicator(
        valueColor: AlwaysStoppedAnimation<Color>(Color(0xFF1B365D)),
      ),
    );
  }

  Widget _buildEmptyWidget() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            Icons.inbox_outlined,
            size: 64,
            color: Colors.grey[400],
          ),
          const SizedBox(height: 16),
          Text(
            'لا توجد عناصر',
            style: TextStyle(
              fontFamily: 'Tajawal',
              fontSize: 16,
              color: Colors.grey[600],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildErrorWidget() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Icon(
            Icons.error_outline,
            size: 64,
            color: Color(0xFFDC3545),
          ),
          const SizedBox(height: 16),
          Text(
            'حدث خطأ أثناء التحميل',
            style: TextStyle(
              fontFamily: 'Tajawal',
              fontSize: 16,
              color: Colors.grey[600],
            ),
          ),
          const SizedBox(height: 16),
          ElevatedButton(
            onPressed: _loadInitialData,
            style: ElevatedButton.styleFrom(
              backgroundColor: const Color(0xFF1B365D),
              foregroundColor: Colors.white,
            ),
            child: const Text(
              'إعادة المحاولة',
              style: TextStyle(fontFamily: 'Tajawal'),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildLoadMoreIndicator() {
    return Container(
      padding: const EdgeInsets.all(16),
      alignment: Alignment.center,
      child: const SizedBox(
        width: 24,
        height: 24,
        child: CircularProgressIndicator(
          strokeWidth: 2,
          valueColor: AlwaysStoppedAnimation<Color>(Color(0xFF1B365D)),
        ),
      ),
    );
  }
}

/// Lazy loading grid widget
class LazyLoadingGrid<T> extends StatefulWidget {
  final Future<List<T>> Function(int page, int pageSize) onLoadMore;
  final Widget Function(BuildContext context, T item, int index) itemBuilder;
  final int crossAxisCount;
  final double childAspectRatio;
  final double crossAxisSpacing;
  final double mainAxisSpacing;
  final Widget? loadingWidget;
  final Widget? emptyWidget;
  final Widget? errorWidget;
  final int pageSize;
  final EdgeInsetsGeometry? padding;

  const LazyLoadingGrid({
    Key? key,
    required this.onLoadMore,
    required this.itemBuilder,
    this.crossAxisCount = 2,
    this.childAspectRatio = 1.0,
    this.crossAxisSpacing = 16.0,
    this.mainAxisSpacing = 16.0,
    this.loadingWidget,
    this.emptyWidget,
    this.errorWidget,
    this.pageSize = 20,
    this.padding,
  }) : super(key: key);

  @override
  State<LazyLoadingGrid<T>> createState() => _LazyLoadingGridState<T>();
}

class _LazyLoadingGridState<T> extends State<LazyLoadingGrid<T>> {
  final List<T> _items = [];
  final ScrollController _scrollController = ScrollController();
  bool _isLoading = false;
  bool _hasMore = true;
  int _currentPage = 0;
  String? _error;

  @override
  void initState() {
    super.initState();
    _scrollController.addListener(_onScroll);
    _loadInitialData();
  }

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  void _onScroll() {
    if (_isLoading || !_hasMore) return;

    final maxScroll = _scrollController.position.maxScrollExtent;
    final currentScroll = _scrollController.position.pixels;
    
    if (maxScroll - currentScroll <= 200.0) {
      _loadMore();
    }
  }

  Future<void> _loadInitialData() async {
    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      final items = await widget.onLoadMore(0, widget.pageSize);
      
      if (mounted) {
        setState(() {
          _items.clear();
          _items.addAll(items);
          _currentPage = 0;
          _hasMore = items.length >= widget.pageSize;
          _isLoading = false;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _error = e.toString();
          _isLoading = false;
        });
      }
    }
  }

  Future<void> _loadMore() async {
    if (_isLoading || !_hasMore) return;

    setState(() {
      _isLoading = true;
    });

    try {
      final nextPage = _currentPage + 1;
      final items = await widget.onLoadMore(nextPage, widget.pageSize);
      
      if (mounted) {
        setState(() {
          _items.addAll(items);
          _currentPage = nextPage;
          _hasMore = items.length >= widget.pageSize;
          _isLoading = false;
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isLoading = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_error != null && _items.isEmpty) {
      return widget.errorWidget ?? const Center(child: Text('Error loading data'));
    }

    if (!_isLoading && _items.isEmpty) {
      return widget.emptyWidget ?? const Center(child: Text('No items'));
    }

    if (_isLoading && _items.isEmpty) {
      return widget.loadingWidget ?? const Center(child: CircularProgressIndicator());
    }

    return GridView.builder(
      controller: _scrollController,
      padding: widget.padding ?? const EdgeInsets.all(16),
      gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: widget.crossAxisCount,
        childAspectRatio: widget.childAspectRatio,
        crossAxisSpacing: widget.crossAxisSpacing,
        mainAxisSpacing: widget.mainAxisSpacing,
      ),
      itemCount: _items.length + (_hasMore && _isLoading ? widget.crossAxisCount : 0),
      itemBuilder: (context, index) {
        if (index >= _items.length) {
          return const Center(
            child: SizedBox(
              width: 24,
              height: 24,
              child: CircularProgressIndicator(strokeWidth: 2),
            ),
          );
        }

        final item = _items[index];
        return widget.itemBuilder(context, item, index);
      },
    );
  }
}

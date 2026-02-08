import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../data/models/ayah_model.dart';
import '../../../../core/providers/quran_provider.dart';
import '../../../../core/theme/app_theme.dart';
import '../widgets/ayah_widget.dart';
import '../widgets/mushaf_page_widget.dart';

/// Mushaf View Screen - High-quality Quran reading interface
/// 
/// Features:
/// - Page-based Quran display with high-quality typography
/// - Smooth page navigation with swipe gestures
/// - Zoom and pan functionality for text
/// - Verse highlighting on tap
/// - Automatic reading position saving
class MushafViewScreen extends ConsumerStatefulWidget {
  final int initialPage;
  final int? initialSurah;
  final int? initialAyah;

  const MushafViewScreen({
    Key? key,
    this.initialPage = 1,
    this.initialSurah,
    this.initialAyah,
  }) : super(key: key);

  @override
  ConsumerState<MushafViewScreen> createState() => _MushafViewScreenState();
}

class _MushafViewScreenState extends ConsumerState<MushafViewScreen> {
  late PageController _pageController;
  late TransformationController _transformationController;
  int _currentPage = 1;
  int? _selectedAyahNumber;
  bool _showControls = true;
  double _fontSize = 24.0;
  
  // Quran has 604 pages in standard Mushaf
  static const int totalPages = 604;

  @override
  void initState() {
    super.initState();
    _currentPage = widget.initialPage;
    _pageController = PageController(initialPage: widget.initialPage - 1);
    _transformationController = TransformationController();
    
    // Load initial page
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _loadPage(_currentPage);
    });
  }

  @override
  void dispose() {
    _pageController.dispose();
    _transformationController.dispose();
    super.dispose();
  }

  void _loadPage(int pageNumber) {
    ref.read(quranProvider.notifier).loadPage(pageNumber);
  }

  void _onPageChanged(int index) {
    setState(() {
      _currentPage = index + 1;
      _selectedAyahNumber = null;
      // Reset zoom when changing pages
      _transformationController.value = Matrix4.identity();
    });
    _loadPage(_currentPage);
    _saveReadingPosition();
  }

  void _saveReadingPosition() {
    // Save reading position automatically
    ref.read(quranProvider.notifier).updateReadingProgress(
      pageNumber: _currentPage,
    );
  }

  void _onAyahTap(AyahModel ayah) {
    setState(() {
      _selectedAyahNumber = ayah.number;
    });
    _showAyahOptions(ayah);
  }

  void _showAyahOptions(AyahModel ayah) {
    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      builder: (context) => Container(
        decoration: BoxDecoration(
          color: Theme.of(context).scaffoldBackgroundColor,
          borderRadius: const BorderRadius.vertical(top: Radius.circular(20)),
        ),
        padding: const EdgeInsets.all(20),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            // Handle bar
            Container(
              width: 40,
              height: 4,
              decoration: BoxDecoration(
                color: Colors.grey[300],
                borderRadius: BorderRadius.circular(2),
              ),
            ),
            const SizedBox(height: 20),
            
            // Ayah info
            Text(
              'سورة ${ayah.surahNumber} - آية ${ayah.numberInSurah}',
              style: const TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 20),
            
            // Options
            _buildOptionButton(
              icon: Icons.book,
              label: 'التفسير',
              onTap: () {
                Navigator.pop(context);
                _navigateToTafsir(ayah);
              },
            ),
            const SizedBox(height: 12),
            _buildOptionButton(
              icon: Icons.volume_up,
              label: 'استماع',
              onTap: () {
                Navigator.pop(context);
                _playAyahAudio(ayah);
              },
            ),
            const SizedBox(height: 12),
            _buildOptionButton(
              icon: Icons.mic,
              label: 'صحح تلاوتي',
              onTap: () {
                Navigator.pop(context);
                _navigateToRecitation(ayah);
              },
            ),
            const SizedBox(height: 12),
            _buildOptionButton(
              icon: Icons.bookmark_add,
              label: 'إضافة علامة',
              onTap: () {
                Navigator.pop(context);
                _addBookmark(ayah);
              },
            ),
            const SizedBox(height: 12),
            _buildOptionButton(
              icon: Icons.share,
              label: 'مشاركة',
              onTap: () {
                Navigator.pop(context);
                _shareAyah(ayah);
              },
            ),
            const SizedBox(height: 20),
          ],
        ),
      ),
    );
  }

  Widget _buildOptionButton({
    required IconData icon,
    required String label,
    required VoidCallback onTap,
  }) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(12),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        decoration: BoxDecoration(
          border: Border.all(color: AppTheme.primaryColor.withOpacity(0.2)),
          borderRadius: BorderRadius.circular(12),
        ),
        child: Row(
          children: [
            Icon(icon, color: AppTheme.primaryColor),
            const SizedBox(width: 12),
            Text(
              label,
              style: const TextStyle(
                fontSize: 16,
                fontFamily: 'Tajawal',
              ),
            ),
          ],
        ),
      ),
    );
  }

  void _navigateToTafsir(AyahModel ayah) {
    // TODO: Navigate to tafsir screen
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('التفسير قريباً')),
    );
  }

  void _playAyahAudio(AyahModel ayah) {
    // TODO: Play ayah audio
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('تشغيل الصوت قريباً')),
    );
  }

  void _navigateToRecitation(AyahModel ayah) {
    // TODO: Navigate to recitation screen
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('مصحح التلاوة قريباً')),
    );
  }

  void _addBookmark(AyahModel ayah) async {
    try {
      await ref.read(quranProvider.notifier).addBookmark(
        surahNumber: ayah.surahNumber,
        ayahNumber: ayah.numberInSurah,
        pageNumber: ayah.pageNumber,
      );
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('تمت إضافة العلامة المرجعية')),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('فشل إضافة العلامة: $e')),
        );
      }
    }
  }

  void _shareAyah(AyahModel ayah) {
    // TODO: Implement share functionality
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('المشاركة قريباً')),
    );
  }

  void _toggleControls() {
    setState(() {
      _showControls = !_showControls;
    });
  }

  void _showPageJumpDialog() {
    final controller = TextEditingController();
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text(
          'الانتقال إلى صفحة',
          style: TextStyle(fontFamily: 'Tajawal'),
        ),
        content: TextField(
          controller: controller,
          keyboardType: TextInputType.number,
          decoration: InputDecoration(
            hintText: 'رقم الصفحة (1-$totalPages)',
            border: const OutlineInputBorder(),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('إلغاء', style: TextStyle(fontFamily: 'Tajawal')),
          ),
          ElevatedButton(
            onPressed: () {
              final page = int.tryParse(controller.text);
              if (page != null && page >= 1 && page <= totalPages) {
                _pageController.jumpToPage(page - 1);
                Navigator.pop(context);
              }
            },
            child: const Text('انتقال', style: TextStyle(fontFamily: 'Tajawal')),
          ),
        ],
      ),
    );
  }

  void _adjustFontSize(double delta) {
    setState(() {
      _fontSize = (_fontSize + delta).clamp(16.0, 40.0);
    });
  }

  @override
  Widget build(BuildContext context) {
    final pageState = ref.watch(quranProvider);

    return Scaffold(
      backgroundColor: AppTheme.backgroundPrimary,
      body: SafeArea(
        child: Stack(
          children: [
            // Main content - PageView with zoom support
            GestureDetector(
              onTap: _toggleControls,
              child: PageView.builder(
                controller: _pageController,
                onPageChanged: _onPageChanged,
                itemCount: totalPages,
                itemBuilder: (context, index) {
                  final pageNumber = index + 1;
                  
                  return InteractiveViewer(
                    transformationController: _transformationController,
                    minScale: 1.0,
                    maxScale: 3.0,
                    child: Center(
                      child: pageState.when(
                        data: (data) {
                          if (data.currentPage?.pageNumber == pageNumber) {
                            return MushafPageWidget(
                              page: data.currentPage!,
                              selectedAyahNumber: _selectedAyahNumber,
                              fontSize: _fontSize,
                              onAyahTap: _onAyahTap,
                            );
                          }
                          return const CircularProgressIndicator();
                        },
                        loading: () => const Center(
                          child: CircularProgressIndicator(),
                        ),
                        error: (error, stack) => Center(
                          child: Column(
                            mainAxisAlignment: MainAxisAlignment.center,
                            children: [
                              const Icon(Icons.error_outline, size: 48, color: Colors.red),
                              const SizedBox(height: 16),
                              Text(
                                'خطأ في تحميل الصفحة',
                                style: const TextStyle(fontFamily: 'Tajawal'),
                              ),
                              const SizedBox(height: 8),
                              ElevatedButton(
                                onPressed: () => _loadPage(pageNumber),
                                child: const Text('إعادة المحاولة'),
                              ),
                            ],
                          ),
                        ),
                      ),
                    ),
                  );
                },
              ),
            ),
            
            // Top controls
            if (_showControls)
              Positioned(
                top: 0,
                left: 0,
                right: 0,
                child: Container(
                  decoration: BoxDecoration(
                    gradient: LinearGradient(
                      begin: Alignment.topCenter,
                      end: Alignment.bottomCenter,
                      colors: [
                        Colors.black.withOpacity(0.6),
                        Colors.transparent,
                      ],
                    ),
                  ),
                  padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                  child: Row(
                    children: [
                      IconButton(
                        icon: const Icon(Icons.arrow_back, color: Colors.white),
                        onPressed: () => Navigator.pop(context),
                      ),
                      const Spacer(),
                      Text(
                        'صفحة $_currentPage من $totalPages',
                        style: const TextStyle(
                          color: Colors.white,
                          fontSize: 16,
                          fontWeight: FontWeight.bold,
                          fontFamily: 'Tajawal',
                        ),
                      ),
                      const Spacer(),
                      IconButton(
                        icon: const Icon(Icons.bookmark, color: Colors.white),
                        onPressed: () {
                          // Show bookmarks
                        },
                      ),
                    ],
                  ),
                ),
              ),
            
            // Bottom controls
            if (_showControls)
              Positioned(
                bottom: 0,
                left: 0,
                right: 0,
                child: Container(
                  decoration: BoxDecoration(
                    gradient: LinearGradient(
                      begin: Alignment.bottomCenter,
                      end: Alignment.topCenter,
                      colors: [
                        Colors.black.withOpacity(0.6),
                        Colors.transparent,
                      ],
                    ),
                  ),
                  padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                    children: [
                      IconButton(
                        icon: const Icon(Icons.text_decrease, color: Colors.white),
                        onPressed: () => _adjustFontSize(-2),
                      ),
                      IconButton(
                        icon: const Icon(Icons.text_increase, color: Colors.white),
                        onPressed: () => _adjustFontSize(2),
                      ),
                      IconButton(
                        icon: const Icon(Icons.search, color: Colors.white),
                        onPressed: _showPageJumpDialog,
                      ),
                      IconButton(
                        icon: const Icon(Icons.settings, color: Colors.white),
                        onPressed: () {
                          // Show settings
                        },
                      ),
                    ],
                  ),
                ),
              ),
          ],
        ),
      ),
    );
  }
}

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../data/models/tafsir_model.dart';
import '../../../../core/providers/tafsir_provider.dart';
import '../widgets/tafsir_source_selector.dart';
import '../widgets/tafsir_content_widget.dart';
import '../widgets/tafsir_comparison_widget.dart';
import '../widgets/tafsir_search_widget.dart';

class TafsirViewerScreen extends ConsumerStatefulWidget {
  final int surahNumber;
  final int ayahNumber;
  final String arabicText;

  const TafsirViewerScreen({
    Key? key,
    required this.surahNumber,
    required this.ayahNumber,
    required this.arabicText,
  }) : super(key: key);

  @override
  ConsumerState<TafsirViewerScreen> createState() =>
      _TafsirViewerScreenState();
}

class _TafsirViewerScreenState extends ConsumerState<TafsirViewerScreen>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  TafsirDisplayPreferences _preferences = TafsirDisplayPreferences(
    selectedSources: [],
    layout: TafsirLayout.stacked,
    showCrossReferences: true,
    showThemes: true,
    fontSize: 16.0,
  );

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 3, vsync: this);
    
    // Load tafsir sources
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(tafsirSourcesProvider.notifier).loadSources();
    });
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  void _updatePreferences(TafsirDisplayPreferences newPreferences) {
    setState(() {
      _preferences = newPreferences;
    });

    // Load tafsir with new preferences
    if (newPreferences.selectedSources.isNotEmpty) {
      ref.read(tafsirProvider.notifier).loadTafsir(
            widget.surahNumber,
            widget.ayahNumber,
            newPreferences.selectedSources,
          );
    }
  }

  @override
  Widget build(BuildContext context) {
    final sourcesState = ref.watch(tafsirSourcesProvider);
    final tafsirState = ref.watch(tafsirProvider);

    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topCenter,
            end: Alignment.bottomCenter,
            colors: [Color(0xFF1B365D), Color(0xFF2D5A27)],
          ),
        ),
        child: SafeArea(
          child: Column(
            children: [
              // Header
              _buildHeader(context),

              // Content
              Expanded(
                child: Container(
                  decoration: const BoxDecoration(
                    color: Colors.white,
                    borderRadius: BorderRadius.only(
                      topLeft: Radius.circular(24),
                      topRight: Radius.circular(24),
                    ),
                  ),
                  child: Column(
                    children: [
                      // Tabs
                      _buildTabs(),

                      // Tab Content
                      Expanded(
                        child: TabBarView(
                          controller: _tabController,
                          children: [
                            // View Tab
                            _buildViewTab(sourcesState, tafsirState),

                            // Compare Tab
                            _buildCompareTab(),

                            // Search Tab
                            const TafsirSearchWidget(),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildHeader(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Title and Close Button
          Row(
            children: [
              IconButton(
                icon: const Icon(Icons.arrow_back, color: Colors.white),
                onPressed: () => Navigator.pop(context),
              ),
              Expanded(
                child: Text(
                  'التفسير - سورة ${widget.surahNumber} آية ${widget.ayahNumber}',
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                    fontFamily: 'Tajawal',
                  ),
                  textAlign: TextAlign.center,
                  textDirection: TextDirection.rtl,
                ),
              ),
              const SizedBox(width: 48), // Balance the back button
            ],
          ),

          const SizedBox(height: 16),

          // Arabic Text
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: Colors.white.withOpacity(0.1),
              borderRadius: BorderRadius.circular(12),
            ),
            child: Text(
              widget.arabicText,
              style: const TextStyle(
                color: Colors.white,
                fontSize: 24,
                height: 2.0,
                fontFamily: 'KFGQPC Uthman Taha Naskh',
              ),
              textAlign: TextAlign.center,
              textDirection: TextDirection.rtl,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildTabs() {
    return Container(
      margin: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.grey[200],
        borderRadius: BorderRadius.circular(12),
      ),
      child: TabBar(
        controller: _tabController,
        indicator: BoxDecoration(
          color: const Color(0xFF1B365D),
          borderRadius: BorderRadius.circular(12),
        ),
        labelColor: Colors.white,
        unselectedLabelColor: Colors.grey[700],
        tabs: const [
          Tab(text: 'عرض التفاسير'),
          Tab(text: 'مقارنة التفاسير'),
          Tab(text: 'البحث'),
        ],
      ),
    );
  }

  Widget _buildViewTab(
    AsyncValue<List<TafsirSource>> sourcesState,
    AsyncValue<List<TafsirWithSource>> tafsirState,
  ) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        children: [
          // Source Selector
          sourcesState.when(
            data: (sources) => TafsirSourceSelector(
              sources: sources,
              selectedSources: _preferences.selectedSources,
              onSourcesChanged: (sourceIds) {
                _updatePreferences(
                  _preferences.copyWith(selectedSources: sourceIds),
                );
              },
              onDownloadOffline: () async {
                try {
                  await ref.read(tafsirProvider.notifier).downloadForOffline(
                        widget.surahNumber,
                        _preferences.selectedSources,
                      );
                  if (mounted) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(
                        content: Text('تم تحميل التفاسير للعمل دون اتصال'),
                      ),
                    );
                  }
                } catch (e) {
                  if (mounted) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(content: Text('فشل التحميل: $e')),
                    );
                  }
                }
              },
            ),
            loading: () => const Center(child: CircularProgressIndicator()),
            error: (error, stack) => Center(
              child: Text('خطأ في تحميل المصادر: $error'),
            ),
          ),

          const SizedBox(height: 16),

          // Tafsir Content
          tafsirState.when(
            data: (tafsirs) => TafsirContentWidget(
              tafsirs: tafsirs,
              preferences: _preferences,
              onLayoutChanged: (layout) {
                _updatePreferences(_preferences.copyWith(layout: layout));
              },
            ),
            loading: () => const Center(
              child: Padding(
                padding: EdgeInsets.all(32.0),
                child: CircularProgressIndicator(),
              ),
            ),
            error: (error, stack) => Center(
              child: Padding(
                padding: const EdgeInsets.all(16.0),
                child: Text(
                  'خطأ في تحميل التفاسير: $error',
                  style: const TextStyle(color: Colors.red),
                  textAlign: TextAlign.center,
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildCompareTab() {
    if (_preferences.selectedSources.length < 2) {
      return const Center(
        child: Padding(
          padding: EdgeInsets.all(32.0),
          child: Text(
            'الرجاء اختيار مصدرين على الأقل للمقارنة',
            style: TextStyle(
              fontSize: 16,
              color: Colors.grey,
            ),
            textAlign: TextAlign.center,
            textDirection: TextDirection.rtl,
          ),
        ),
      );
    }

    return TafsirComparisonWidget(
      surahNumber: widget.surahNumber,
      ayahNumber: widget.ayahNumber,
      selectedSources: _preferences.selectedSources,
    );
  }
}

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Model for daily verse with tafsir
class DailyVerse {
  final String arabicText;
  final String translation;
  final String reference;
  final String briefTafsir;
  final String fullTafsir;
  final String tafsirSource;

  DailyVerse({
    required this.arabicText,
    required this.translation,
    required this.reference,
    required this.briefTafsir,
    required this.fullTafsir,
    required this.tafsirSource,
  });

  factory DailyVerse.fromJson(Map<String, dynamic> json) {
    return DailyVerse(
      arabicText: json['arabic_text'] ?? '',
      translation: json['translation'] ?? '',
      reference: json['reference'] ?? '',
      briefTafsir: json['brief_tafsir'] ?? '',
      fullTafsir: json['full_tafsir'] ?? '',
      tafsirSource: json['tafsir_source'] ?? '',
    );
  }
}

/// Provider for daily verse
final dailyVerseProvider = FutureProvider<DailyVerse>((ref) async {
  // TODO: Fetch from backend API
  // For now, return mock data
  return DailyVerse(
    arabicText: 'وَمَن يَتَّقِ اللَّهَ يَجْعَل لَّهُ مَخْرَجًا',
    translation: 'And whoever fears Allah - He will make for him a way out',
    reference: 'سورة الطلاق: 2',
    briefTafsir:
        'من يتق الله في جميع أموره، يجعل له مخرجًا من كل ضيق وكرب في الدنيا والآخرة.',
    fullTafsir:
        'من يتق الله في جميع أموره، يجعل له مخرجًا من كل ضيق وكرب في الدنيا والآخرة، ويرزقه من حيث لا يحتسب. وهذا وعد من الله تعالى لمن اتقاه بأن يجعل له فرجًا ومخرجًا من كل أمر يضيق عليه، وأن يرزقه من جهة لا تخطر بباله. والتقوى هي امتثال أوامر الله واجتناب نواهيه، وهي سبب كل خير في الدنيا والآخرة.',
    tafsirSource: 'تفسير السعدي',
  );
});

/// Interactive Daily Verse Widget with expandable tafsir
class DailyVerseWidget extends ConsumerStatefulWidget {
  final DailyVerse? dailyVerse;
  final VoidCallback? onTap;

  const DailyVerseWidget({
    Key? key,
    this.dailyVerse,
    this.onTap,
  }) : super(key: key);

  @override
  ConsumerState<DailyVerseWidget> createState() => _DailyVerseWidgetState();
}

class _DailyVerseWidgetState extends ConsumerState<DailyVerseWidget>
    with SingleTickerProviderStateMixin {
  bool _isExpanded = false;
  late AnimationController _animationController;
  late Animation<double> _rotationAnimation;

  @override
  void initState() {
    super.initState();
    _animationController = AnimationController(
      duration: const Duration(milliseconds: 300),
      vsync: this,
    );
    _rotationAnimation = Tween<double>(begin: 0, end: 0.5).animate(
      CurvedAnimation(
        parent: _animationController,
        curve: Curves.easeInOut,
      ),
    );
  }

  @override
  void dispose() {
    _animationController.dispose();
    super.dispose();
  }

  void _toggleExpanded() {
    setState(() {
      _isExpanded = !_isExpanded;
      if (_isExpanded) {
        _animationController.forward();
      } else {
        _animationController.reverse();
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    final verseAsync = widget.dailyVerse != null
        ? AsyncValue.data(widget.dailyVerse!)
        : ref.watch(dailyVerseProvider);

    return verseAsync.when(
      data: (verse) => _buildWidget(context, verse),
      loading: () => _buildLoadingWidget(),
      error: (error, stack) => _buildErrorWidget(error),
    );
  }

  Widget _buildWidget(BuildContext context, DailyVerse verse) {
    return IslamicCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Header
          Row(
            children: [
              Container(
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  gradient: LinearGradient(
                    colors: [
                      AppColors.accent,
                      AppColors.accent.withOpacity(0.7),
                    ],
                  ),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: const Icon(
                  Icons.auto_awesome,
                  color: Colors.white,
                  size: 24,
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'آية اليوم',
                      style: AppTextStyles.h6.copyWith(
                        color: AppColors.primary,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      verse.reference,
                      style: AppTextStyles.bodySmall.copyWith(
                        color: AppColors.textSecondary,
                      ),
                    ),
                  ],
                ),
              ),
              // Share button
              IconButton(
                icon: Icon(
                  Icons.share_outlined,
                  color: AppColors.primary,
                  size: 20,
                ),
                onPressed: () {
                  // TODO: Implement share functionality
                },
                tooltip: 'مشاركة',
              ),
            ],
          ),
          const SizedBox(height: 16),

          // Arabic text
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              gradient: LinearGradient(
                colors: [
                  AppColors.primary.withOpacity(0.05),
                  AppColors.accent.withOpacity(0.05),
                ],
                begin: Alignment.topRight,
                end: Alignment.bottomLeft,
              ),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(
                color: AppColors.primary.withOpacity(0.1),
              ),
            ),
            child: Text(
              verse.arabicText,
              style: AppTextStyles.h5.copyWith(
                fontFamily: 'Amiri',
                color: AppColors.primary,
                height: 2.0,
                fontWeight: FontWeight.w600,
              ),
              textAlign: TextAlign.center,
              textDirection: TextDirection.rtl,
            ),
          ),
          const SizedBox(height: 12),

          // Translation
          Text(
            verse.translation,
            style: AppTextStyles.bodyMedium.copyWith(
              color: AppColors.textSecondary,
              fontStyle: FontStyle.italic,
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 16),

          // Brief Tafsir
          Container(
            padding: const EdgeInsets.all(14),
            decoration: BoxDecoration(
              color: AppColors.secondary.withOpacity(0.1),
              borderRadius: BorderRadius.circular(10),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(
                      Icons.lightbulb_outline,
                      size: 18,
                      color: AppColors.secondary,
                    ),
                    const SizedBox(width: 6),
                    Text(
                      'التفسير المختصر',
                      style: AppTextStyles.bodySmall.copyWith(
                        color: AppColors.secondary,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 8),
                Text(
                  verse.briefTafsir,
                  style: AppTextStyles.bodyMedium.copyWith(
                    color: AppColors.textPrimary,
                    height: 1.6,
                  ),
                ),
              ],
            ),
          ),

          // Expandable full tafsir
          AnimatedCrossFade(
            firstChild: const SizedBox.shrink(),
            secondChild: Column(
              children: [
                const SizedBox(height: 12),
                Container(
                  padding: const EdgeInsets.all(14),
                  decoration: BoxDecoration(
                    color: AppColors.backgroundSecondary,
                    borderRadius: BorderRadius.circular(10),
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        children: [
                          Icon(
                            Icons.menu_book,
                            size: 18,
                            color: AppColors.primary,
                          ),
                          const SizedBox(width: 6),
                          Text(
                            'التفسير الكامل',
                            style: AppTextStyles.bodySmall.copyWith(
                              color: AppColors.primary,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 8),
                      Text(
                        verse.fullTafsir,
                        style: AppTextStyles.bodyMedium.copyWith(
                          color: AppColors.textPrimary,
                          height: 1.7,
                        ),
                      ),
                      const SizedBox(height: 8),
                      Row(
                        children: [
                          Icon(
                            Icons.source,
                            size: 14,
                            color: AppColors.textSecondary,
                          ),
                          const SizedBox(width: 4),
                          Text(
                            'المصدر: ${verse.tafsirSource}',
                            style: AppTextStyles.caption.copyWith(
                              color: AppColors.textSecondary,
                              fontStyle: FontStyle.italic,
                            ),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ],
            ),
            crossFadeState: _isExpanded
                ? CrossFadeState.showSecond
                : CrossFadeState.showFirst,
            duration: const Duration(milliseconds: 300),
          ),
          const SizedBox(height: 12),

          // Expand/Collapse button
          InkWell(
            onTap: _toggleExpanded,
            borderRadius: BorderRadius.circular(8),
            child: Container(
              padding: const EdgeInsets.symmetric(vertical: 10),
              decoration: BoxDecoration(
                border: Border.all(
                  color: AppColors.primary.withOpacity(0.2),
                ),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Text(
                    _isExpanded ? 'إخفاء التفسير الكامل' : 'عرض التفسير الكامل',
                    style: AppTextStyles.bodyMedium.copyWith(
                      color: AppColors.primary,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const SizedBox(width: 6),
                  RotationTransition(
                    turns: _rotationAnimation,
                    child: Icon(
                      Icons.keyboard_arrow_down,
                      color: AppColors.primary,
                      size: 20,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildLoadingWidget() {
    return IslamicCard(
      child: Center(
        child: Padding(
          padding: const EdgeInsets.all(40),
          child: CircularProgressIndicator(
            valueColor: AlwaysStoppedAnimation<Color>(AppColors.primary),
          ),
        ),
      ),
    );
  }

  Widget _buildErrorWidget(Object error) {
    return IslamicCard(
      child: Center(
        child: Padding(
          padding: const EdgeInsets.all(20),
          child: Text(
            'حدث خطأ في تحميل آية اليوم',
            style: AppTextStyles.bodyMedium.copyWith(
              color: AppColors.error,
            ),
          ),
        ),
      ),
    );
  }
}

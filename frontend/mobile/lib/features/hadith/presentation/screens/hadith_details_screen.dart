import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/providers/hadith_provider.dart';
import '../../../../core/theme/app_theme.dart';
import '../../data/models/hadith_model.dart';

class HadithDetailsScreen extends ConsumerWidget {
  final String hadithId;

  const HadithDetailsScreen({
    super.key,
    required this.hadithId,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final hadithDetailsAsync = ref.watch(
      hadithDetailsProvider(
        HadithDetailsParams(
          hadithId: hadithId,
          includeSanad: true,
          includeExplanations: true,
        ),
      ),
    );

    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'تفاصيل الحديث',
          style: TextStyle(
            fontFamily: 'Tajawal',
            fontWeight: FontWeight.bold,
          ),
        ),
        centerTitle: true,
        actions: [
          IconButton(
            icon: const Icon(Icons.share),
            onPressed: () => _shareHadith(context),
            tooltip: 'مشاركة',
          ),
          IconButton(
            icon: const Icon(Icons.bookmark_border),
            onPressed: () => _bookmarkHadith(context),
            tooltip: 'حفظ',
          ),
        ],
      ),
      body: hadithDetailsAsync.when(
        data: (hadithDetails) => _buildContent(context, hadithDetails),
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
      ),
    );
  }

  Widget _buildContent(BuildContext context, HadithWithDetailsModel hadithDetails) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Book and Grade Header
          _buildHeader(hadithDetails),
          const SizedBox(height: 24),

          // Hadith Text
          _buildHadithText(hadithDetails.hadith),
          const SizedBox(height: 24),

          // Narrator Info
          _buildNarratorInfo(hadithDetails.hadith),
          const SizedBox(height: 24),

          // Sanad (Chain of Narration)
          if (hadithDetails.sanad != null) ...[
            _buildSanad(hadithDetails.sanad!),
            const SizedBox(height: 24),
          ],

          // Chapter Info
          if (hadithDetails.chapter != null) ...[
            _buildChapterInfo(hadithDetails.chapter!),
            const SizedBox(height: 24),
          ],

          // Themes
          if (hadithDetails.hadith.themes.isNotEmpty) ...[
            _buildThemes(hadithDetails.hadith.themes),
            const SizedBox(height: 24),
          ],

          // Metadata
          _buildMetadata(hadithDetails),
        ],
      ),
    );
  }

  Widget _buildHeader(HadithWithDetailsModel hadithDetails) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: Text(
                    hadithDetails.book.arabicName,
                    style: const TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                      fontFamily: 'Tajawal',
                    ),
                    textDirection: TextDirection.rtl,
                  ),
                ),
                _buildGradeBadge(hadithDetails.hadith.grade),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              'المؤلف: ${hadithDetails.book.authorArabicName}',
              style: TextStyle(
                fontSize: 14,
                color: Colors.grey[700],
                fontFamily: 'Tajawal',
              ),
              textDirection: TextDirection.rtl,
            ),
            const SizedBox(height: 4),
            Text(
              'رقم الحديث: ${hadithDetails.hadith.hadithNumber}',
              style: TextStyle(
                fontSize: 14,
                color: Colors.grey[700],
                fontFamily: 'Tajawal',
              ),
              textDirection: TextDirection.rtl,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildHadithText(HadithModel hadith) {
    return Card(
      color: AppTheme.backgroundPaper,
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              'نص الحديث',
              style: TextStyle(
                fontSize: 16,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 16),
            Text(
              hadith.text,
              style: const TextStyle(
                fontSize: 20,
                height: 2.0,
                fontFamily: 'Amiri',
              ),
              textDirection: TextDirection.rtl,
              textAlign: TextAlign.justify,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildNarratorInfo(HadithModel hadith) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Row(
          children: [
            const CircleAvatar(
              backgroundColor: AppTheme.primaryColor,
              child: Icon(Icons.person, color: Colors.white),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    'الراوي',
                    style: TextStyle(
                      fontSize: 12,
                      color: Colors.grey,
                      fontFamily: 'Tajawal',
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    hadith.narrator,
                    style: const TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.bold,
                      fontFamily: 'Tajawal',
                    ),
                    textDirection: TextDirection.rtl,
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildSanad(SanadModel sanad) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Row(
              children: [
                const Text(
                  'السند',
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    fontFamily: 'Tajawal',
                  ),
                ),
                const Spacer(),
                _buildChainGradeBadge(sanad.chainGrade),
              ],
            ),
            const SizedBox(height: 16),

            // Chain Text
            Text(
              sanad.chainText,
              style: const TextStyle(
                fontSize: 16,
                height: 1.8,
                fontFamily: 'Amiri',
              ),
              textDirection: TextDirection.rtl,
            ),
            const SizedBox(height: 16),

            // Narrators List
            const Text(
              'سلسلة الرواة:',
              style: TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 8),
            ...sanad.narrators.asMap().entries.map((entry) {
              final index = entry.key;
              final narrator = entry.value;
              return Padding(
                padding: const EdgeInsets.only(bottom: 8),
                child: Row(
                  children: [
                    Container(
                      width: 24,
                      height: 24,
                      decoration: BoxDecoration(
                        color: AppTheme.primaryColor,
                        shape: BoxShape.circle,
                      ),
                      child: Center(
                        child: Text(
                          '${index + 1}',
                          style: const TextStyle(
                            color: Colors.white,
                            fontSize: 12,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: Text(
                        narrator,
                        style: const TextStyle(
                          fontSize: 14,
                          fontFamily: 'Tajawal',
                        ),
                        textDirection: TextDirection.rtl,
                      ),
                    ),
                  ],
                ),
              );
            }).toList(),

            // Chain Analysis
            if (sanad.chainAnalysis != null) ...[
              const SizedBox(height: 16),
              Container(
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: Colors.blue[50],
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    const Text(
                      'تحليل السند:',
                      style: TextStyle(
                        fontSize: 14,
                        fontWeight: FontWeight.bold,
                        fontFamily: 'Tajawal',
                      ),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      sanad.chainAnalysis!,
                      style: const TextStyle(
                        fontSize: 13,
                        height: 1.6,
                        fontFamily: 'Tajawal',
                      ),
                      textDirection: TextDirection.rtl,
                    ),
                  ],
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildChapterInfo(HadithChapterModel chapter) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              'الباب',
              style: TextStyle(
                fontSize: 16,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 12),
            Text(
              chapter.arabicTitle,
              style: const TextStyle(
                fontSize: 16,
                fontFamily: 'Tajawal',
              ),
              textDirection: TextDirection.rtl,
            ),
            if (chapter.description != null) ...[
              const SizedBox(height: 8),
              Text(
                chapter.description!,
                style: TextStyle(
                  fontSize: 14,
                  color: Colors.grey[700],
                  height: 1.6,
                ),
                textDirection: TextDirection.rtl,
              ),
            ],
            const SizedBox(height: 8),
            Text(
              'رقم الباب: ${chapter.chapterNumber} • عدد الأحاديث: ${chapter.hadithCount}',
              style: TextStyle(
                fontSize: 12,
                color: Colors.grey[600],
                fontFamily: 'Tajawal',
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildThemes(List<String> themes) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              'المواضيع',
              style: TextStyle(
                fontSize: 16,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 12),
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: themes.map((theme) {
                return Chip(
                  label: Text(
                    theme,
                    style: const TextStyle(
                      fontSize: 13,
                      fontFamily: 'Tajawal',
                    ),
                  ),
                  backgroundColor: AppTheme.accentGold.withOpacity(0.1),
                );
              }).toList(),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildMetadata(HadithWithDetailsModel hadithDetails) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            const Text(
              'معلومات إضافية',
              style: TextStyle(
                fontSize: 16,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 12),
            _buildMetadataRow('عدد الكلمات', '${hadithDetails.hadith.wordCount}'),
            _buildMetadataRow('المصدر', hadithDetails.hadith.source),
            _buildMetadataRow('اللغة', hadithDetails.hadith.language == 'ar' ? 'العربية' : hadithDetails.hadith.language),
          ],
        ),
      ),
    );
  }

  Widget _buildMetadataRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8),
      child: Row(
        children: [
          Text(
            '$label:',
            style: TextStyle(
              fontSize: 14,
              color: Colors.grey[600],
              fontFamily: 'Tajawal',
            ),
          ),
          const SizedBox(width: 8),
          Expanded(
            child: Text(
              value,
              style: const TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.w500,
                fontFamily: 'Tajawal',
              ),
              textDirection: TextDirection.rtl,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildGradeBadge(HadithGrade grade) {
    Color color;
    switch (grade) {
      case HadithGrade.sahih:
        color = Colors.green;
        break;
      case HadithGrade.hasan:
        color = Colors.amber;
        break;
      case HadithGrade.daif:
        color = Colors.orange;
        break;
      case HadithGrade.mawdu:
        color = Colors.red;
        break;
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        color: color,
        borderRadius: BorderRadius.circular(12),
      ),
      child: Text(
        grade.arabicName,
        style: const TextStyle(
          color: Colors.white,
          fontSize: 12,
          fontWeight: FontWeight.bold,
          fontFamily: 'Tajawal',
        ),
      ),
    );
  }

  Widget _buildChainGradeBadge(ChainGrade grade) {
    Color color;
    switch (grade) {
      case ChainGrade.sahih:
        color = Colors.green;
        break;
      case ChainGrade.hasan:
        color = Colors.amber;
        break;
      case ChainGrade.daif:
        color = Colors.orange;
        break;
      case ChainGrade.munqati:
      case ChainGrade.mursal:
        color = Colors.red;
        break;
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        color: color.withOpacity(0.2),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: color, width: 1),
      ),
      child: Text(
        grade.arabicName,
        style: TextStyle(
          color: color,
          fontSize: 12,
          fontWeight: FontWeight.bold,
          fontFamily: 'Tajawal',
        ),
      ),
    );
  }

  void _shareHadith(BuildContext context) {
    // TODO: Implement share functionality
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('سيتم إضافة ميزة المشاركة قريباً')),
    );
  }

  void _bookmarkHadith(BuildContext context) {
    // TODO: Implement bookmark functionality
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('تم حفظ الحديث')),
    );
  }
}

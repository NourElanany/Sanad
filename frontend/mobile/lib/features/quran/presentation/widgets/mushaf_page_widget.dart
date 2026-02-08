import 'package:flutter/material.dart';
import '../../data/models/ayah_model.dart';
import 'ayah_widget.dart';

/// Widget to display a single page of the Mushaf
class MushafPageWidget extends StatelessWidget {
  final QuranPageModel page;
  final int? selectedAyahNumber;
  final double fontSize;
  final Function(AyahModel) onAyahTap;

  const MushafPageWidget({
    Key? key,
    required this.page,
    this.selectedAyahNumber,
    required this.fontSize,
    required this.onAyahTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 20),
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(12),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.1),
            blurRadius: 10,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Page header with surah name and juz
          _buildPageHeader(),
          const SizedBox(height: 16),
          
          // Ayahs
          Expanded(
            child: SingleChildScrollView(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  // Check if this is the start of a surah (first ayah)
                  if (page.ayahs.isNotEmpty && page.ayahs.first.numberInSurah == 1)
                    _buildSurahHeader(),
                  
                  // Display all ayahs
                  ...page.ayahs.map((ayah) => AyahWidget(
                    ayah: ayah,
                    isSelected: ayah.number == selectedAyahNumber,
                    fontSize: fontSize,
                    onTap: () => onAyahTap(ayah),
                  )),
                ],
              ),
            ),
          ),
          
          // Page footer with page number
          const SizedBox(height: 16),
          _buildPageFooter(),
        ],
      ),
    );
  }

  Widget _buildPageHeader() {
    return Container(
      padding: const EdgeInsets.symmetric(vertical: 8),
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide(
            color: const Color(0xFF1B365D).withOpacity(0.2),
            width: 1,
          ),
        ),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(
            'جزء ${page.juzNumber}',
            style: const TextStyle(
              fontSize: 14,
              color: Color(0xFF1B365D),
              fontFamily: 'Tajawal',
              fontWeight: FontWeight.w500,
            ),
          ),
          Text(
            page.surahName,
            style: const TextStyle(
              fontSize: 16,
              color: Color(0xFF1B365D),
              fontFamily: 'Tajawal',
              fontWeight: FontWeight.bold,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSurahHeader() {
    return Container(
      margin: const EdgeInsets.only(bottom: 20),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF1B365D).withOpacity(0.05),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(
          color: const Color(0xFF1B365D).withOpacity(0.2),
          width: 1,
        ),
      ),
      child: Column(
        children: [
          // Bismillah (except for Surah At-Tawbah)
          if (page.surahNumber != 9 && page.ayahs.first.numberInSurah == 1)
            const Text(
              'بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ',
              textAlign: TextAlign.center,
              style: TextStyle(
                fontSize: 24,
                fontFamily: 'KFGQPC Uthman Taha Naskh',
                color: Color(0xFF1B365D),
                height: 2.0,
              ),
            ),
        ],
      ),
    );
  }

  Widget _buildPageFooter() {
    return Container(
      padding: const EdgeInsets.symmetric(vertical: 8),
      decoration: BoxDecoration(
        border: Border(
          top: BorderSide(
            color: const Color(0xFF1B365D).withOpacity(0.2),
            width: 1,
          ),
        ),
      ),
      child: Center(
        child: Container(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
          decoration: BoxDecoration(
            color: const Color(0xFFB8860B).withOpacity(0.1),
            borderRadius: BorderRadius.circular(20),
          ),
          child: Text(
            '${page.pageNumber}',
            style: const TextStyle(
              fontSize: 16,
              color: Color(0xFFB8860B),
              fontFamily: 'Tajawal',
              fontWeight: FontWeight.bold,
            ),
          ),
        ),
      ),
    );
  }
}

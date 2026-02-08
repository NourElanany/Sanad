import 'package:flutter/material.dart';
import '../../data/models/recording_models.dart';

/// Verse selection widget for recording
class VerseSelector extends StatefulWidget {
  final VerseSelection? initialSelection;
  final ValueChanged<VerseSelection> onSelectionChanged;

  const VerseSelector({
    Key? key,
    this.initialSelection,
    required this.onSelectionChanged,
  }) : super(key: key);

  @override
  State<VerseSelector> createState() => _VerseSelectorState();
}

class _VerseSelectorState extends State<VerseSelector> {
  int? _selectedSurah;
  int? _ayahStart;
  int? _ayahEnd;

  // Sample surah data (in production, this would come from the API)
  final List<Map<String, dynamic>> _surahs = [
    {'number': 1, 'name': 'الفاتحة', 'ayahCount': 7},
    {'number': 2, 'name': 'البقرة', 'ayahCount': 286},
    {'number': 3, 'name': 'آل عمران', 'ayahCount': 200},
    {'number': 4, 'name': 'النساء', 'ayahCount': 176},
    {'number': 5, 'name': 'المائدة', 'ayahCount': 120},
    // Add more surahs as needed
  ];

  @override
  void initState() {
    super.initState();
    if (widget.initialSelection != null) {
      _selectedSurah = widget.initialSelection!.surahNumber;
      _ayahStart = widget.initialSelection!.ayahStart;
      _ayahEnd = widget.initialSelection!.ayahEnd;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(16),
        boxShadow: [
          BoxShadow(
            color: const Color(0xFF1B365D).withOpacity(0.08),
            blurRadius: 16,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Text(
            'اختر الآيات للتسجيل',
            style: TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
              color: Color(0xFF1B365D),
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 20),

          // Surah selector
          _buildSurahSelector(),
          const SizedBox(height: 16),

          // Ayah range selector
          if (_selectedSurah != null) ...[
            Row(
              children: [
                Expanded(
                  child: _buildAyahSelector(
                    label: 'من الآية',
                    value: _ayahStart,
                    maxAyah: _getMaxAyahCount(),
                    onChanged: (value) {
                      setState(() {
                        _ayahStart = value;
                        if (_ayahEnd != null && value! > _ayahEnd!) {
                          _ayahEnd = value;
                        }
                        _notifySelection();
                      });
                    },
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: _buildAyahSelector(
                    label: 'إلى الآية',
                    value: _ayahEnd,
                    maxAyah: _getMaxAyahCount(),
                    minAyah: _ayahStart ?? 1,
                    onChanged: (value) {
                      setState(() {
                        _ayahEnd = value;
                        _notifySelection();
                      });
                    },
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),

            // Selected verse preview
            if (_ayahStart != null && _ayahEnd != null)
              _buildSelectionPreview(),
          ],
        ],
      ),
    );
  }

  Widget _buildSurahSelector() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text(
          'السورة',
          style: TextStyle(
            fontSize: 14,
            fontWeight: FontWeight.w600,
            fontFamily: 'Tajawal',
            color: Color(0xFF666666),
          ),
        ),
        const SizedBox(height: 8),
        Container(
          decoration: BoxDecoration(
            color: const Color(0xFFF8F9FA),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(
              color: const Color(0xFFE0E0E0),
              width: 1,
            ),
          ),
          child: DropdownButtonHideUnderline(
            child: DropdownButton<int>(
              value: _selectedSurah,
              isExpanded: true,
              hint: const Padding(
                padding: EdgeInsets.symmetric(horizontal: 16),
                child: Text(
                  'اختر السورة',
                  style: TextStyle(
                    fontFamily: 'Tajawal',
                    color: Color(0xFF999999),
                  ),
                ),
              ),
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
              borderRadius: BorderRadius.circular(12),
              items: _surahs.map((surah) {
                return DropdownMenuItem<int>(
                  value: surah['number'] as int,
                  child: Text(
                    '${surah['number']}. ${surah['name']}',
                    style: const TextStyle(
                      fontFamily: 'Tajawal',
                      fontSize: 16,
                    ),
                  ),
                );
              }).toList(),
              onChanged: (value) {
                setState(() {
                  _selectedSurah = value;
                  _ayahStart = null;
                  _ayahEnd = null;
                });
              },
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildAyahSelector({
    required String label,
    required int? value,
    required int maxAyah,
    int minAyah = 1,
    required ValueChanged<int?> onChanged,
  }) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          label,
          style: const TextStyle(
            fontSize: 14,
            fontWeight: FontWeight.w600,
            fontFamily: 'Tajawal',
            color: Color(0xFF666666),
          ),
        ),
        const SizedBox(height: 8),
        Container(
          decoration: BoxDecoration(
            color: const Color(0xFFF8F9FA),
            borderRadius: BorderRadius.circular(12),
            border: Border.all(
              color: const Color(0xFFE0E0E0),
              width: 1,
            ),
          ),
          child: DropdownButtonHideUnderline(
            child: DropdownButton<int>(
              value: value,
              isExpanded: true,
              hint: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16),
                child: Text(
                  'رقم الآية',
                  style: const TextStyle(
                    fontFamily: 'Tajawal',
                    color: Color(0xFF999999),
                  ),
                ),
              ),
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
              borderRadius: BorderRadius.circular(12),
              items: List.generate(
                maxAyah - minAyah + 1,
                (index) => minAyah + index,
              ).map((ayah) {
                return DropdownMenuItem<int>(
                  value: ayah,
                  child: Text(
                    ayah.toString(),
                    style: const TextStyle(
                      fontFamily: 'Tajawal',
                      fontSize: 16,
                    ),
                  ),
                );
              }).toList(),
              onChanged: onChanged,
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildSelectionPreview() {
    final surah = _surahs.firstWhere(
      (s) => s['number'] == _selectedSurah,
    );
    final ayahCount = _ayahEnd! - _ayahStart! + 1;

    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF1B365D).withOpacity(0.05),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: const Color(0xFF1B365D).withOpacity(0.2),
          width: 1,
        ),
      ),
      child: Column(
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const Icon(
                Icons.check_circle,
                color: Color(0xFF2D5A27),
                size: 20,
              ),
              const SizedBox(width: 8),
              Text(
                'سورة ${surah['name']}',
                style: const TextStyle(
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Tajawal',
                  color: Color(0xFF1B365D),
                ),
              ),
            ],
          ),
          const SizedBox(height: 8),
          Text(
            'من الآية $_ayahStart إلى الآية $_ayahEnd',
            style: const TextStyle(
              fontSize: 14,
              fontFamily: 'Tajawal',
              color: Color(0xFF666666),
            ),
          ),
          const SizedBox(height: 4),
          Text(
            'عدد الآيات: $ayahCount',
            style: const TextStyle(
              fontSize: 12,
              fontFamily: 'Tajawal',
              color: Color(0xFF999999),
            ),
          ),
        ],
      ),
    );
  }

  int _getMaxAyahCount() {
    if (_selectedSurah == null) return 0;
    final surah = _surahs.firstWhere(
      (s) => s['number'] == _selectedSurah,
    );
    return surah['ayahCount'] as int;
  }

  void _notifySelection() {
    if (_selectedSurah != null && _ayahStart != null && _ayahEnd != null) {
      final surah = _surahs.firstWhere(
        (s) => s['number'] == _selectedSurah,
      );

      widget.onSelectionChanged(
        VerseSelection(
          surahNumber: _selectedSurah!,
          surahName: surah['name'] as String,
          ayahStart: _ayahStart!,
          ayahEnd: _ayahEnd!,
          arabicText: '', // Would be fetched from API
        ),
      );
    }
  }
}

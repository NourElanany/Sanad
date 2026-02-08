/// Surah model representing a chapter of the Quran
class SurahModel {
  final int number;
  final String nameArabic;
  final String nameEnglish;
  final String nameTransliteration;
  final int ayahCount;
  final String revelationType; // 'Meccan' or 'Medinan'
  final int revelationOrder;
  final int juzStart;
  final int juzEnd;
  final int pageStart;
  final int pageEnd;

  const SurahModel({
    required this.number,
    required this.nameArabic,
    required this.nameEnglish,
    required this.nameTransliteration,
    required this.ayahCount,
    required this.revelationType,
    required this.revelationOrder,
    required this.juzStart,
    required this.juzEnd,
    required this.pageStart,
    required this.pageEnd,
  });

  factory SurahModel.fromJson(Map<String, dynamic> json) {
    return SurahModel(
      number: json['number'] as int,
      nameArabic: json['name_arabic'] as String,
      nameEnglish: json['name_english'] as String,
      nameTransliteration: json['name_transliteration'] as String,
      ayahCount: json['ayah_count'] as int,
      revelationType: json['revelation_type'] as String,
      revelationOrder: json['revelation_order'] as int,
      juzStart: json['juz_start'] as int,
      juzEnd: json['juz_end'] as int,
      pageStart: json['page_start'] as int,
      pageEnd: json['page_end'] as int,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'number': number,
      'name_arabic': nameArabic,
      'name_english': nameEnglish,
      'name_transliteration': nameTransliteration,
      'ayah_count': ayahCount,
      'revelation_type': revelationType,
      'revelation_order': revelationOrder,
      'juz_start': juzStart,
      'juz_end': juzEnd,
      'page_start': pageStart,
      'page_end': pageEnd,
    };
  }

  bool get isMeccan => revelationType.toLowerCase() == 'meccan';
  bool get isMedinan => revelationType.toLowerCase() == 'medinan';
}

/// Juz model representing a part of the Quran
class JuzModel {
  final int number;
  final int startSurah;
  final int startAyah;
  final int endSurah;
  final int endAyah;
  final int pageStart;
  final int pageEnd;

  const JuzModel({
    required this.number,
    required this.startSurah,
    required this.startAyah,
    required this.endSurah,
    required this.endAyah,
    required this.pageStart,
    required this.pageEnd,
  });

  factory JuzModel.fromJson(Map<String, dynamic> json) {
    return JuzModel(
      number: json['number'] as int,
      startSurah: json['start_surah'] as int,
      startAyah: json['start_ayah'] as int,
      endSurah: json['end_surah'] as int,
      endAyah: json['end_ayah'] as int,
      pageStart: json['page_start'] as int,
      pageEnd: json['page_end'] as int,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'number': number,
      'start_surah': startSurah,
      'start_ayah': startAyah,
      'end_surah': endSurah,
      'end_ayah': endAyah,
      'page_start': pageStart,
      'page_end': pageEnd,
    };
  }
}

/// Bookmark model for saved Quran positions
class QuranBookmark {
  final String id;
  final int surahNumber;
  final int ayahNumber;
  final int pageNumber;
  final String? note;
  final DateTime createdAt;

  const QuranBookmark({
    required this.id,
    required this.surahNumber,
    required this.ayahNumber,
    required this.pageNumber,
    this.note,
    required this.createdAt,
  });

  factory QuranBookmark.fromJson(Map<String, dynamic> json) {
    return QuranBookmark(
      id: json['id'] as String,
      surahNumber: json['surah_number'] as int,
      ayahNumber: json['ayah_number'] as int,
      pageNumber: json['page_number'] as int,
      note: json['note'] as String?,
      createdAt: DateTime.parse(json['created_at'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'surah_number': surahNumber,
      'ayah_number': ayahNumber,
      'page_number': pageNumber,
      'note': note,
      'created_at': createdAt.toIso8601String(),
    };
  }
}

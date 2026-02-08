/// Ayah (verse) model representing a verse of the Quran
class AyahModel {
  final int number;
  final int numberInSurah;
  final int surahNumber;
  final String textArabic;
  final String textUthmani;
  final int juzNumber;
  final int hizbNumber;
  final int pageNumber;
  final int manzilNumber;
  final int rukuNumber;

  const AyahModel({
    required this.number,
    required this.numberInSurah,
    required this.surahNumber,
    required this.textArabic,
    required this.textUthmani,
    required this.juzNumber,
    required this.hizbNumber,
    required this.pageNumber,
    required this.manzilNumber,
    required this.rukuNumber,
  });

  factory AyahModel.fromJson(Map<String, dynamic> json) {
    return AyahModel(
      number: json['number'] as int,
      numberInSurah: json['number_in_surah'] as int,
      surahNumber: json['surah_number'] as int,
      textArabic: json['text_arabic'] as String,
      textUthmani: json['text_uthmani'] as String,
      juzNumber: json['juz_number'] as int,
      hizbNumber: json['hizb_number'] as int,
      pageNumber: json['page_number'] as int,
      manzilNumber: json['manzil_number'] as int,
      rukuNumber: json['ruku_number'] as int,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'number': number,
      'number_in_surah': numberInSurah,
      'surah_number': surahNumber,
      'text_arabic': textArabic,
      'text_uthmani': textUthmani,
      'juz_number': juzNumber,
      'hizb_number': hizbNumber,
      'page_number': pageNumber,
      'manzil_number': manzilNumber,
      'ruku_number': rukuNumber,
    };
  }
}

/// Quran page model representing a page of the Mushaf
class QuranPageModel {
  final int pageNumber;
  final List<AyahModel> ayahs;
  final int juzNumber;
  final int surahNumber;
  final String surahName;

  const QuranPageModel({
    required this.pageNumber,
    required this.ayahs,
    required this.juzNumber,
    required this.surahNumber,
    required this.surahName,
  });

  factory QuranPageModel.fromJson(Map<String, dynamic> json) {
    return QuranPageModel(
      pageNumber: json['page_number'] as int,
      ayahs: (json['ayahs'] as List<dynamic>)
          .map((ayah) => AyahModel.fromJson(ayah as Map<String, dynamic>))
          .toList(),
      juzNumber: json['juz_number'] as int,
      surahNumber: json['surah_number'] as int,
      surahName: json['surah_name'] as String,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'page_number': pageNumber,
      'ayahs': ayahs.map((ayah) => ayah.toJson()).toList(),
      'juz_number': juzNumber,
      'surah_number': surahNumber,
      'surah_name': surahName,
    };
  }
}

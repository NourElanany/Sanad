import 'package:uuid/uuid.dart';

/// Hadith authenticity grades according to Islamic scholarship
enum HadithGrade {
  sahih('sahih', 'صحيح', 'Sahih'),
  hasan('hasan', 'حسن', 'Hasan'),
  daif('daif', 'ضعيف', 'Daif'),
  mawdu('mawdu', 'موضوع', 'Mawdu');

  final String value;
  final String arabicName;
  final String englishName;

  const HadithGrade(this.value, this.arabicName, this.englishName);

  static HadithGrade fromString(String value) {
    return HadithGrade.values.firstWhere(
      (grade) => grade.value == value,
      orElse: () => HadithGrade.daif,
    );
  }
}

/// Chain of narration grades for Sanad authenticity
enum ChainGrade {
  sahih('sahih', 'صحيح'),
  hasan('hasan', 'حسن'),
  daif('daif', 'ضعيف'),
  munqati('munqati', 'منقطع'),
  mursal('mursal', 'مرسل');

  final String value;
  final String arabicName;

  const ChainGrade(this.value, this.arabicName);

  static ChainGrade fromString(String value) {
    return ChainGrade.values.firstWhere(
      (grade) => grade.value == value,
      orElse: () => ChainGrade.daif,
    );
  }
}

/// Types of Hadith books
enum HadithBookType {
  sahih('sahih', 'صحيح'),
  sunan('sunan', 'سنن'),
  musnad('musnad', 'مسند'),
  mujam('mujam', 'معجم'),
  mustadrak('mustadrak', 'مستدرك'),
  jami('jami', 'جامع');

  final String value;
  final String arabicName;

  const HadithBookType(this.value, this.arabicName);

  static HadithBookType fromString(String value) {
    return HadithBookType.values.firstWhere(
      (type) => type.value == value,
      orElse: () => HadithBookType.jami,
    );
  }
}

/// Book authenticity levels
enum BookAuthenticityLevel {
  highest('highest', 'أعلى درجة'),
  high('high', 'عالية'),
  moderate('moderate', 'متوسطة'),
  variable('variable', 'متغيرة');

  final String value;
  final String arabicName;

  const BookAuthenticityLevel(this.value, this.arabicName);

  static BookAuthenticityLevel fromString(String value) {
    return BookAuthenticityLevel.values.firstWhere(
      (level) => level.value == value,
      orElse: () => BookAuthenticityLevel.variable,
    );
  }
}

/// Represents a Hadith (prophetic tradition)
class HadithModel {
  final String id;
  final String hadithNumber;
  final String text;
  final String textHash;
  final String narrator;
  final String book;
  final String chapter;
  final int? chapterNumber;
  final int? hadithNumberInChapter;
  final HadithGrade grade;
  final String source;
  final String language;
  final int wordCount;
  final List<String> themes;
  final List<String> keywords;
  final DateTime createdAt;
  final DateTime updatedAt;

  const HadithModel({
    required this.id,
    required this.hadithNumber,
    required this.text,
    required this.textHash,
    required this.narrator,
    required this.book,
    required this.chapter,
    this.chapterNumber,
    this.hadithNumberInChapter,
    required this.grade,
    required this.source,
    required this.language,
    required this.wordCount,
    required this.themes,
    required this.keywords,
    required this.createdAt,
    required this.updatedAt,
  });

  factory HadithModel.fromJson(Map<String, dynamic> json) {
    return HadithModel(
      id: json['id'] as String,
      hadithNumber: json['hadith_number'] as String,
      text: json['text'] as String,
      textHash: json['text_hash'] as String,
      narrator: json['narrator'] as String,
      book: json['book'] as String,
      chapter: json['chapter'] as String,
      chapterNumber: json['chapter_number'] as int?,
      hadithNumberInChapter: json['hadith_number_in_chapter'] as int?,
      grade: HadithGrade.fromString(json['grade'] as String),
      source: json['source'] as String,
      language: json['language'] as String,
      wordCount: json['word_count'] as int,
      themes: (json['themes'] as List<dynamic>?)
              ?.map((e) => e as String)
              .toList() ??
          [],
      keywords: (json['keywords'] as List<dynamic>?)
              ?.map((e) => e as String)
              .toList() ??
          [],
      createdAt: DateTime.parse(json['created_at'] as String),
      updatedAt: DateTime.parse(json['updated_at'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'hadith_number': hadithNumber,
      'text': text,
      'text_hash': textHash,
      'narrator': narrator,
      'book': book,
      'chapter': chapter,
      'chapter_number': chapterNumber,
      'hadith_number_in_chapter': hadithNumberInChapter,
      'grade': grade.value,
      'source': source,
      'language': language,
      'word_count': wordCount,
      'themes': themes,
      'keywords': keywords,
      'created_at': createdAt.toIso8601String(),
      'updated_at': updatedAt.toIso8601String(),
    };
  }

  bool get isAuthentic =>
      grade == HadithGrade.sahih || grade == HadithGrade.hasan;

  HadithModel copyWith({
    String? id,
    String? hadithNumber,
    String? text,
    String? textHash,
    String? narrator,
    String? book,
    String? chapter,
    int? chapterNumber,
    int? hadithNumberInChapter,
    HadithGrade? grade,
    String? source,
    String? language,
    int? wordCount,
    List<String>? themes,
    List<String>? keywords,
    DateTime? createdAt,
    DateTime? updatedAt,
  }) {
    return HadithModel(
      id: id ?? this.id,
      hadithNumber: hadithNumber ?? this.hadithNumber,
      text: text ?? this.text,
      textHash: textHash ?? this.textHash,
      narrator: narrator ?? this.narrator,
      book: book ?? this.book,
      chapter: chapter ?? this.chapter,
      chapterNumber: chapterNumber ?? this.chapterNumber,
      hadithNumberInChapter:
          hadithNumberInChapter ?? this.hadithNumberInChapter,
      grade: grade ?? this.grade,
      source: source ?? this.source,
      language: language ?? this.language,
      wordCount: wordCount ?? this.wordCount,
      themes: themes ?? this.themes,
      keywords: keywords ?? this.keywords,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
    );
  }
}

/// Represents the chain of narration (Sanad) for a Hadith
class SanadModel {
  final String id;
  final String hadithId;
  final String chainText;
  final String chainHash;
  final List<String> narrators;
  final ChainGrade chainGrade;
  final String? chainAnalysis;
  final DateTime createdAt;
  final DateTime updatedAt;

  const SanadModel({
    required this.id,
    required this.hadithId,
    required this.chainText,
    required this.chainHash,
    required this.narrators,
    required this.chainGrade,
    this.chainAnalysis,
    required this.createdAt,
    required this.updatedAt,
  });

  factory SanadModel.fromJson(Map<String, dynamic> json) {
    return SanadModel(
      id: json['id'] as String,
      hadithId: json['hadith_id'] as String,
      chainText: json['chain_text'] as String,
      chainHash: json['chain_hash'] as String,
      narrators: (json['narrators'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      chainGrade: ChainGrade.fromString(json['chain_grade'] as String),
      chainAnalysis: json['chain_analysis'] as String?,
      createdAt: DateTime.parse(json['created_at'] as String),
      updatedAt: DateTime.parse(json['updated_at'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'hadith_id': hadithId,
      'chain_text': chainText,
      'chain_hash': chainHash,
      'narrators': narrators,
      'chain_grade': chainGrade.value,
      'chain_analysis': chainAnalysis,
      'created_at': createdAt.toIso8601String(),
      'updated_at': updatedAt.toIso8601String(),
    };
  }

  int get narratorCount => narrators.length;

  bool get isContinuous =>
      chainGrade != ChainGrade.munqati && chainGrade != ChainGrade.mursal;
}

/// Represents a Hadith book/collection
class HadithBookModel {
  final String id;
  final String name;
  final String arabicName;
  final String author;
  final String authorArabicName;
  final String? description;
  final int? compilationYear;
  final int totalHadiths;
  final HadithBookType bookType;
  final BookAuthenticityLevel authenticityLevel;
  final String language;
  final DateTime createdAt;
  final DateTime updatedAt;

  const HadithBookModel({
    required this.id,
    required this.name,
    required this.arabicName,
    required this.author,
    required this.authorArabicName,
    this.description,
    this.compilationYear,
    required this.totalHadiths,
    required this.bookType,
    required this.authenticityLevel,
    required this.language,
    required this.createdAt,
    required this.updatedAt,
  });

  factory HadithBookModel.fromJson(Map<String, dynamic> json) {
    return HadithBookModel(
      id: json['id'] as String,
      name: json['name'] as String,
      arabicName: json['arabic_name'] as String,
      author: json['author'] as String,
      authorArabicName: json['author_arabic_name'] as String,
      description: json['description'] as String?,
      compilationYear: json['compilation_year'] as int?,
      totalHadiths: json['total_hadiths'] as int,
      bookType: HadithBookType.fromString(json['book_type'] as String),
      authenticityLevel: BookAuthenticityLevel.fromString(
          json['authenticity_level'] as String),
      language: json['language'] as String,
      createdAt: DateTime.parse(json['created_at'] as String),
      updatedAt: DateTime.parse(json['updated_at'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      'arabic_name': arabicName,
      'author': author,
      'author_arabic_name': authorArabicName,
      'description': description,
      'compilation_year': compilationYear,
      'total_hadiths': totalHadiths,
      'book_type': bookType.value,
      'authenticity_level': authenticityLevel.value,
      'language': language,
      'created_at': createdAt.toIso8601String(),
      'updated_at': updatedAt.toIso8601String(),
    };
  }

  bool get isMostAuthentic =>
      authenticityLevel == BookAuthenticityLevel.highest;
}

/// Represents a chapter within a Hadith book
class HadithChapterModel {
  final String id;
  final String bookId;
  final int chapterNumber;
  final String title;
  final String arabicTitle;
  final String? description;
  final int hadithCount;
  final List<String> themes;
  final DateTime createdAt;

  const HadithChapterModel({
    required this.id,
    required this.bookId,
    required this.chapterNumber,
    required this.title,
    required this.arabicTitle,
    this.description,
    required this.hadithCount,
    required this.themes,
    required this.createdAt,
  });

  factory HadithChapterModel.fromJson(Map<String, dynamic> json) {
    return HadithChapterModel(
      id: json['id'] as String,
      bookId: json['book_id'] as String,
      chapterNumber: json['chapter_number'] as int,
      title: json['title'] as String,
      arabicTitle: json['arabic_title'] as String,
      description: json['description'] as String?,
      hadithCount: json['hadith_count'] as int,
      themes: (json['themes'] as List<dynamic>?)
              ?.map((e) => e as String)
              .toList() ??
          [],
      createdAt: DateTime.parse(json['created_at'] as String),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'book_id': bookId,
      'chapter_number': chapterNumber,
      'title': title,
      'arabic_title': arabicTitle,
      'description': description,
      'hadith_count': hadithCount,
      'themes': themes,
      'created_at': createdAt.toIso8601String(),
    };
  }
}

/// Complete Hadith with all related information
class HadithWithDetailsModel {
  final HadithModel hadith;
  final HadithBookModel book;
  final HadithChapterModel? chapter;
  final SanadModel? sanad;

  const HadithWithDetailsModel({
    required this.hadith,
    required this.book,
    this.chapter,
    this.sanad,
  });

  factory HadithWithDetailsModel.fromJson(Map<String, dynamic> json) {
    return HadithWithDetailsModel(
      hadith: HadithModel.fromJson(json['hadith'] as Map<String, dynamic>),
      book: HadithBookModel.fromJson(json['book'] as Map<String, dynamic>),
      chapter: json['chapter'] != null
          ? HadithChapterModel.fromJson(json['chapter'] as Map<String, dynamic>)
          : null,
      sanad: json['sanad'] != null
          ? SanadModel.fromJson(json['sanad'] as Map<String, dynamic>)
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'hadith': hadith.toJson(),
      'book': book.toJson(),
      'chapter': chapter?.toJson(),
      'sanad': sanad?.toJson(),
    };
  }
}

/// Search result for Hadith content
class HadithSearchResultModel {
  final HadithModel hadith;
  final HadithBookModel book;
  final HadithChapterModel? chapter;
  final double relevanceScore;
  final String highlightedText;
  final List<String> matchingCriteria;

  const HadithSearchResultModel({
    required this.hadith,
    required this.book,
    this.chapter,
    required this.relevanceScore,
    required this.highlightedText,
    required this.matchingCriteria,
  });

  factory HadithSearchResultModel.fromJson(Map<String, dynamic> json) {
    return HadithSearchResultModel(
      hadith: HadithModel.fromJson(json['hadith'] as Map<String, dynamic>),
      book: HadithBookModel.fromJson(json['book'] as Map<String, dynamic>),
      chapter: json['chapter'] != null
          ? HadithChapterModel.fromJson(json['chapter'] as Map<String, dynamic>)
          : null,
      relevanceScore: (json['relevance_score'] as num).toDouble(),
      highlightedText: json['highlighted_text'] as String,
      matchingCriteria: (json['matching_criteria'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
    );
  }
}

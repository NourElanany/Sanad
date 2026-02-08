import 'package:json_annotation/json_annotation.dart';

part 'tafsir_model.g.dart';

enum ScholarlyAuthentication {
  @JsonValue('highly_authenticated')
  highlyAuthenticated,
  @JsonValue('authenticated')
  authenticated,
  @JsonValue('verified')
  verified,
  @JsonValue('unverified')
  unverified,
}

enum TafsirSourceType {
  @JsonValue('classical')
  classical,
  @JsonValue('contemporary')
  contemporary,
  @JsonValue('linguistic')
  linguistic,
  @JsonValue('thematic')
  thematic,
  @JsonValue('sectarian')
  sectarian,
}

@JsonSerializable()
class TafsirSource {
  final String id;
  final String name;
  final String author;
  final String language;
  final String? description;
  @JsonKey(name: 'credibility_score')
  final double credibilityScore;
  @JsonKey(name: 'scholarly_authentication')
  final ScholarlyAuthentication scholarlyAuthentication;
  @JsonKey(name: 'source_type')
  final TafsirSourceType sourceType;
  @JsonKey(name: 'publication_year')
  final int? publicationYear;
  final String? methodology;
  @JsonKey(name: 'created_at')
  final DateTime createdAt;
  @JsonKey(name: 'updated_at')
  final DateTime updatedAt;

  TafsirSource({
    required this.id,
    required this.name,
    required this.author,
    required this.language,
    this.description,
    required this.credibilityScore,
    required this.scholarlyAuthentication,
    required this.sourceType,
    this.publicationYear,
    this.methodology,
    required this.createdAt,
    required this.updatedAt,
  });

  factory TafsirSource.fromJson(Map<String, dynamic> json) =>
      _$TafsirSourceFromJson(json);

  Map<String, dynamic> toJson() => _$TafsirSourceToJson(this);

  bool get isHighlyCredible => credibilityScore >= 8.0;

  bool get isAuthenticated =>
      scholarlyAuthentication == ScholarlyAuthentication.highlyAuthenticated ||
      scholarlyAuthentication == ScholarlyAuthentication.authenticated;

  String get credibilityLevel {
    if (credibilityScore >= 9.0) return 'ممتاز';
    if (credibilityScore >= 7.5) return 'جيد جداً';
    if (credibilityScore >= 6.0) return 'جيد';
    if (credibilityScore >= 4.0) return 'مقبول';
    return 'ضعيف';
  }
}

@JsonSerializable()
class Tafsir {
  final String id;
  @JsonKey(name: 'surah_number')
  final int surahNumber;
  @JsonKey(name: 'ayah_number')
  final int ayahNumber;
  @JsonKey(name: 'source_id')
  final String sourceId;
  final String text;
  @JsonKey(name: 'text_hash')
  final String textHash;
  @JsonKey(name: 'word_count')
  final int wordCount;
  final List<String> themes;
  @JsonKey(name: 'cross_references')
  final List<String> crossReferences;
  @JsonKey(name: 'created_at')
  final DateTime createdAt;
  @JsonKey(name: 'updated_at')
  final DateTime updatedAt;

  Tafsir({
    required this.id,
    required this.surahNumber,
    required this.ayahNumber,
    required this.sourceId,
    required this.text,
    required this.textHash,
    required this.wordCount,
    required this.themes,
    required this.crossReferences,
    required this.createdAt,
    required this.updatedAt,
  });

  factory Tafsir.fromJson(Map<String, dynamic> json) =>
      _$TafsirFromJson(json);

  Map<String, dynamic> toJson() => _$TafsirToJson(this);

  bool get isComprehensive => wordCount > 100;

  int get estimatedReadingTime {
    if (wordCount == 0) return 0;
    return (wordCount / 200).ceil();
  }
}

@JsonSerializable()
class TafsirWithSource {
  final Tafsir tafsir;
  final TafsirSource source;

  TafsirWithSource({
    required this.tafsir,
    required this.source,
  });

  factory TafsirWithSource.fromJson(Map<String, dynamic> json) =>
      _$TafsirWithSourceFromJson(json);

  Map<String, dynamic> toJson() => _$TafsirWithSourceToJson(this);
}

enum ComparisonCriteria {
  @JsonValue('linguistic')
  linguistic,
  @JsonValue('thematic')
  thematic,
  @JsonValue('historical')
  historical,
  @JsonValue('jurisprudential')
  jurisprudential,
  @JsonValue('spiritual')
  spiritual,
}

enum ViewSignificance {
  @JsonValue('major')
  major,
  @JsonValue('moderate')
  moderate,
  @JsonValue('minor')
  minor,
}

@JsonSerializable()
class SourcePosition {
  @JsonKey(name: 'source_id')
  final String sourceId;
  @JsonKey(name: 'source_name')
  final String sourceName;
  final String position;
  final List<String> evidence;

  SourcePosition({
    required this.sourceId,
    required this.sourceName,
    required this.position,
    required this.evidence,
  });

  factory SourcePosition.fromJson(Map<String, dynamic> json) =>
      _$SourcePositionFromJson(json);

  Map<String, dynamic> toJson() => _$SourcePositionToJson(this);
}

@JsonSerializable()
class DivergentView {
  final String topic;
  @JsonKey(name: 'source_positions')
  final List<SourcePosition> sourcePositions;
  final ViewSignificance significance;

  DivergentView({
    required this.topic,
    required this.sourcePositions,
    required this.significance,
  });

  factory DivergentView.fromJson(Map<String, dynamic> json) =>
      _$DivergentViewFromJson(json);

  Map<String, dynamic> toJson() => _$DivergentViewToJson(this);
}

@JsonSerializable()
class ComparisonSummary {
  @JsonKey(name: 'common_themes')
  final List<String> commonThemes;
  @JsonKey(name: 'divergent_views')
  final List<DivergentView> divergentViews;
  @JsonKey(name: 'scholarly_consensus')
  final String? scholarlyConsensus;
  @JsonKey(name: 'recommended_reading_order')
  final List<String> recommendedReadingOrder;

  ComparisonSummary({
    required this.commonThemes,
    required this.divergentViews,
    this.scholarlyConsensus,
    required this.recommendedReadingOrder,
  });

  factory ComparisonSummary.fromJson(Map<String, dynamic> json) =>
      _$ComparisonSummaryFromJson(json);

  Map<String, dynamic> toJson() => _$ComparisonSummaryToJson(this);
}

@JsonSerializable()
class TafsirComparison {
  final TafsirSource source;
  final Tafsir tafsir;
  @JsonKey(name: 'key_points')
  final List<String> keyPoints;
  @JsonKey(name: 'unique_insights')
  final List<String> uniqueInsights;
  @JsonKey(name: 'methodology_notes')
  final String? methodologyNotes;

  TafsirComparison({
    required this.source,
    required this.tafsir,
    required this.keyPoints,
    required this.uniqueInsights,
    this.methodologyNotes,
  });

  factory TafsirComparison.fromJson(Map<String, dynamic> json) =>
      _$TafsirComparisonFromJson(json);

  Map<String, dynamic> toJson() => _$TafsirComparisonToJson(this);
}

@JsonSerializable()
class TafsirComparisonResponse {
  final dynamic ayah;
  final dynamic surah;
  final List<TafsirComparison> comparisons;
  final ComparisonSummary summary;
  final List<String> recommendations;

  TafsirComparisonResponse({
    required this.ayah,
    required this.surah,
    required this.comparisons,
    required this.summary,
    required this.recommendations,
  });

  factory TafsirComparisonResponse.fromJson(Map<String, dynamic> json) =>
      _$TafsirComparisonResponseFromJson(json);

  Map<String, dynamic> toJson() => _$TafsirComparisonResponseToJson(this);
}

enum TafsirLayout {
  stacked,
  sideBySide,
  tabbed,
}

class TafsirDisplayPreferences {
  final List<String> selectedSources;
  final TafsirLayout layout;
  final bool showCrossReferences;
  final bool showThemes;
  final double fontSize;

  TafsirDisplayPreferences({
    required this.selectedSources,
    this.layout = TafsirLayout.stacked,
    this.showCrossReferences = true,
    this.showThemes = true,
    this.fontSize = 16.0,
  });

  TafsirDisplayPreferences copyWith({
    List<String>? selectedSources,
    TafsirLayout? layout,
    bool? showCrossReferences,
    bool? showThemes,
    double? fontSize,
  }) {
    return TafsirDisplayPreferences(
      selectedSources: selectedSources ?? this.selectedSources,
      layout: layout ?? this.layout,
      showCrossReferences: showCrossReferences ?? this.showCrossReferences,
      showThemes: showThemes ?? this.showThemes,
      fontSize: fontSize ?? this.fontSize,
    );
  }
}

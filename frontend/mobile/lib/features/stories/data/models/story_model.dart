import 'package:json_annotation/json_annotation.dart';

part 'story_model.g.dart';

/// Represents an Islamic story with comprehensive metadata
@JsonSerializable()
class StoryModel {
  final String id;
  final String title;
  @JsonKey(name: 'arabic_title')
  final String arabicTitle;
  final String content;
  @JsonKey(name: 'content_hash')
  final String contentHash;
  final String? summary;
  final StoryCategory category;
  final String? subcategory;
  @JsonKey(name: 'time_period')
  final TimePeriod? timePeriod;
  final String? location;
  @JsonKey(name: 'word_count')
  final int wordCount;
  @JsonKey(name: 'estimated_reading_time')
  final int estimatedReadingTime;
  @JsonKey(name: 'age_group')
  final AgeGroup ageGroup;
  @JsonKey(name: 'moral_lessons')
  final List<String> moralLessons;
  final List<String> themes;
  final List<String> keywords;
  final String language;
  @JsonKey(name: 'authenticity_level')
  final AuthenticityLevel authenticityLevel;
  @JsonKey(name: 'scholarly_verification')
  final ScholarlyVerification scholarlyVerification;
  @JsonKey(name: 'created_at')
  final DateTime createdAt;
  @JsonKey(name: 'updated_at')
  final DateTime updatedAt;

  StoryModel({
    required this.id,
    required this.title,
    required this.arabicTitle,
    required this.content,
    required this.contentHash,
    this.summary,
    required this.category,
    this.subcategory,
    this.timePeriod,
    this.location,
    required this.wordCount,
    required this.estimatedReadingTime,
    required this.ageGroup,
    required this.moralLessons,
    required this.themes,
    required this.keywords,
    required this.language,
    required this.authenticityLevel,
    required this.scholarlyVerification,
    required this.createdAt,
    required this.updatedAt,
  });

  factory StoryModel.fromJson(Map<String, dynamic> json) =>
      _$StoryModelFromJson(json);

  Map<String, dynamic> toJson() => _$StoryModelToJson(this);

  String get categoryArabic => category.arabicName;
  String get ageGroupArabic => ageGroup.arabicName;
  String get authenticityArabic => authenticityLevel.arabicName;

  bool get isSuitableForChildren =>
      ageGroup == AgeGroup.children || ageGroup == AgeGroup.allAges;

  bool get isHistoricallyAuthentic =>
      authenticityLevel == AuthenticityLevel.authentic ||
      authenticityLevel == AuthenticityLevel.wellDocumented;
}

/// Story categories enum
@JsonEnum(fieldRename: FieldRename.snake)
enum StoryCategory {
  prophets,
  companions,
  @JsonValue('righteous_predecessors')
  righteousPredecessors,
  @JsonValue('historical_events')
  historicalEvents,
  @JsonValue('moral_lessons')
  moralLessons,
  miracles,
  battles,
  conversions,
  @JsonValue('women_in_islam')
  womenInIslam,
  @JsonValue('children_stories')
  childrenStories;

  String get arabicName {
    switch (this) {
      case StoryCategory.prophets:
        return 'قصص الأنبياء';
      case StoryCategory.companions:
        return 'قصص الصحابة';
      case StoryCategory.righteousPredecessors:
        return 'قصص السلف الصالح';
      case StoryCategory.historicalEvents:
        return 'الأحداث التاريخية';
      case StoryCategory.moralLessons:
        return 'العبر والمواعظ';
      case StoryCategory.miracles:
        return 'المعجزات';
      case StoryCategory.battles:
        return 'الغزوات والمعارك';
      case StoryCategory.conversions:
        return 'قصص الإسلام';
      case StoryCategory.womenInIslam:
        return 'نساء في الإسلام';
      case StoryCategory.childrenStories:
        return 'قصص الأطفال';
    }
  }

  String get icon {
    switch (this) {
      case StoryCategory.prophets:
        return '📖';
      case StoryCategory.companions:
        return '👥';
      case StoryCategory.righteousPredecessors:
        return '⭐';
      case StoryCategory.historicalEvents:
        return '🏛️';
      case StoryCategory.moralLessons:
        return '💡';
      case StoryCategory.miracles:
        return '✨';
      case StoryCategory.battles:
        return '⚔️';
      case StoryCategory.conversions:
        return '🌟';
      case StoryCategory.womenInIslam:
        return '👩';
      case StoryCategory.childrenStories:
        return '🧒';
    }
  }
}

/// Time periods in Islamic history
@JsonEnum(fieldRename: FieldRename.snake)
enum TimePeriod {
  @JsonValue('pre_islamic')
  preIslamic,
  @JsonValue('prophetic_era')
  propheticEra,
  @JsonValue('rightly_guided_caliphs')
  rightlyGuidedCaliphs,
  umayyad,
  abbasid,
  ottoman,
  modern,
  @JsonValue('ancient_prophets')
  ancientProphets;

  String get arabicName {
    switch (this) {
      case TimePeriod.preIslamic:
        return 'ما قبل الإسلام';
      case TimePeriod.propheticEra:
        return 'العهد النبوي';
      case TimePeriod.rightlyGuidedCaliphs:
        return 'عهد الخلفاء الراشدين';
      case TimePeriod.umayyad:
        return 'العهد الأموي';
      case TimePeriod.abbasid:
        return 'العهد العباسي';
      case TimePeriod.ottoman:
        return 'العهد العثماني';
      case TimePeriod.modern:
        return 'العصر الحديث';
      case TimePeriod.ancientProphets:
        return 'الأنبياء القدماء';
    }
  }
}

/// Age groups for story targeting
@JsonEnum(fieldRename: FieldRename.snake)
enum AgeGroup {
  children,
  teenagers,
  @JsonValue('young_adults')
  youngAdults,
  adults,
  @JsonValue('all_ages')
  allAges;

  String get arabicName {
    switch (this) {
      case AgeGroup.children:
        return 'الأطفال';
      case AgeGroup.teenagers:
        return 'المراهقون';
      case AgeGroup.youngAdults:
        return 'الشباب';
      case AgeGroup.adults:
        return 'البالغون';
      case AgeGroup.allAges:
        return 'جميع الأعمار';
    }
  }
}

/// Authenticity levels for Islamic stories
@JsonEnum(fieldRename: FieldRename.snake)
enum AuthenticityLevel {
  authentic,
  @JsonValue('well_documented')
  wellDocumented,
  probable,
  traditional,
  educational;

  String get arabicName {
    switch (this) {
      case AuthenticityLevel.authentic:
        return 'صحيح';
      case AuthenticityLevel.wellDocumented:
        return 'موثق جيداً';
      case AuthenticityLevel.probable:
        return 'محتمل';
      case AuthenticityLevel.traditional:
        return 'تراثي';
      case AuthenticityLevel.educational:
        return 'تعليمي';
    }
  }

  String get colorCode {
    switch (this) {
      case AuthenticityLevel.authentic:
        return '#28A745'; // Green
      case AuthenticityLevel.wellDocumented:
        return '#17A2B8'; // Blue
      case AuthenticityLevel.probable:
        return '#FFC107'; // Yellow
      case AuthenticityLevel.traditional:
        return '#FD7E14'; // Orange
      case AuthenticityLevel.educational:
        return '#6C757D'; // Gray
    }
  }
}

/// Scholarly verification status
@JsonEnum(fieldRename: FieldRename.snake)
enum ScholarlyVerification {
  verified,
  @JsonValue('under_review')
  underReview,
  pending,
  disputed;

  String get arabicName {
    switch (this) {
      case ScholarlyVerification.verified:
        return 'تم التحقق';
      case ScholarlyVerification.underReview:
        return 'قيد المراجعة';
      case ScholarlyVerification.pending:
        return 'في الانتظار';
      case ScholarlyVerification.disputed:
        return 'محل خلاف';
    }
  }
}

/// Character in Islamic stories
@JsonSerializable()
class CharacterModel {
  final String id;
  final String name;
  @JsonKey(name: 'arabic_name')
  final String arabicName;
  @JsonKey(name: 'character_type')
  final CharacterType characterType;
  final String? description;
  @JsonKey(name: 'historical_period')
  final TimePeriod? historicalPeriod;
  @JsonKey(name: 'birth_year')
  final int? birthYear;
  @JsonKey(name: 'death_year')
  final int? deathYear;
  final String? biography;
  final List<String> virtues;
  @JsonKey(name: 'role_significance')
  final String? roleSignificance;
  @JsonKey(name: 'related_stories_count')
  final int relatedStoriesCount;

  CharacterModel({
    required this.id,
    required this.name,
    required this.arabicName,
    required this.characterType,
    this.description,
    this.historicalPeriod,
    this.birthYear,
    this.deathYear,
    this.biography,
    required this.virtues,
    this.roleSignificance,
    required this.relatedStoriesCount,
  });

  factory CharacterModel.fromJson(Map<String, dynamic> json) =>
      _$CharacterModelFromJson(json);

  Map<String, dynamic> toJson() => _$CharacterModelToJson(this);

  String get characterTypeArabic => characterType.arabicName;

  bool get isProphet =>
      characterType == CharacterType.prophet ||
      characterType == CharacterType.messenger;
}

/// Types of characters in Islamic stories
@JsonEnum(fieldRename: FieldRename.snake)
enum CharacterType {
  prophet,
  messenger,
  companion,
  @JsonValue('righteous_person')
  righteousPerson,
  scholar,
  ruler,
  martyr,
  convert,
  @JsonValue('historical_figure')
  historicalFigure,
  antagonist;

  String get arabicName {
    switch (this) {
      case CharacterType.prophet:
        return 'نبي';
      case CharacterType.messenger:
        return 'رسول';
      case CharacterType.companion:
        return 'صحابي';
      case CharacterType.righteousPerson:
        return 'صالح';
      case CharacterType.scholar:
        return 'عالم';
      case CharacterType.ruler:
        return 'حاكم';
      case CharacterType.martyr:
        return 'شهيد';
      case CharacterType.convert:
        return 'مسلم جديد';
      case CharacterType.historicalFigure:
        return 'شخصية تاريخية';
      case CharacterType.antagonist:
        return 'معارض';
    }
  }
}

/// Lesson derived from Islamic stories
@JsonSerializable()
class LessonModel {
  final String id;
  final String title;
  @JsonKey(name: 'arabic_title')
  final String arabicTitle;
  final String description;
  @JsonKey(name: 'lesson_type')
  final LessonType lessonType;
  @JsonKey(name: 'moral_category')
  final MoralCategory moralCategory;
  @JsonKey(name: 'practical_application')
  final String? practicalApplication;
  @JsonKey(name: 'target_audience')
  final List<AgeGroup> targetAudience;
  @JsonKey(name: 'related_verses')
  final List<String> relatedVerses;
  @JsonKey(name: 'related_hadiths')
  final List<String> relatedHadiths;

  LessonModel({
    required this.id,
    required this.title,
    required this.arabicTitle,
    required this.description,
    required this.lessonType,
    required this.moralCategory,
    this.practicalApplication,
    required this.targetAudience,
    required this.relatedVerses,
    required this.relatedHadiths,
  });

  factory LessonModel.fromJson(Map<String, dynamic> json) =>
      _$LessonModelFromJson(json);

  Map<String, dynamic> toJson() => _$LessonModelToJson(this);

  String get lessonTypeArabic => lessonType.arabicName;
  String get moralCategoryArabic => moralCategory.arabicName;
}

/// Types of lessons
@JsonEnum(fieldRename: FieldRename.snake)
enum LessonType {
  moral,
  spiritual,
  practical,
  historical,
  theological,
  social;

  String get arabicName {
    switch (this) {
      case LessonType.moral:
        return 'أخلاقي';
      case LessonType.spiritual:
        return 'روحي';
      case LessonType.practical:
        return 'عملي';
      case LessonType.historical:
        return 'تاريخي';
      case LessonType.theological:
        return 'عقدي';
      case LessonType.social:
        return 'اجتماعي';
    }
  }
}

/// Moral categories
@JsonEnum(fieldRename: FieldRename.snake)
enum MoralCategory {
  patience,
  gratitude,
  justice,
  mercy,
  honesty,
  courage,
  humility,
  forgiveness,
  perseverance,
  faith;

  String get arabicName {
    switch (this) {
      case MoralCategory.patience:
        return 'الصبر';
      case MoralCategory.gratitude:
        return 'الشكر';
      case MoralCategory.justice:
        return 'العدل';
      case MoralCategory.mercy:
        return 'الرحمة';
      case MoralCategory.honesty:
        return 'الصدق';
      case MoralCategory.courage:
        return 'الشجاعة';
      case MoralCategory.humility:
        return 'التواضع';
      case MoralCategory.forgiveness:
        return 'المغفرة';
      case MoralCategory.perseverance:
        return 'المثابرة';
      case MoralCategory.faith:
        return 'الإيمان';
    }
  }
}

/// Story source for references
@JsonSerializable()
class StorySourceModel {
  final String id;
  @JsonKey(name: 'story_id')
  final String storyId;
  @JsonKey(name: 'source_type')
  final SourceType sourceType;
  @JsonKey(name: 'source_name')
  final String sourceName;
  @JsonKey(name: 'arabic_source_name')
  final String arabicSourceName;
  final String? author;
  final String reference;
  @JsonKey(name: 'authenticity_grade')
  final String? authenticityGrade;
  @JsonKey(name: 'credibility_score')
  final double credibilityScore;
  @JsonKey(name: 'verification_status')
  final VerificationStatus verificationStatus;
  final String? notes;

  StorySourceModel({
    required this.id,
    required this.storyId,
    required this.sourceType,
    required this.sourceName,
    required this.arabicSourceName,
    this.author,
    required this.reference,
    this.authenticityGrade,
    required this.credibilityScore,
    required this.verificationStatus,
    this.notes,
  });

  factory StorySourceModel.fromJson(Map<String, dynamic> json) =>
      _$StorySourceModelFromJson(json);

  Map<String, dynamic> toJson() => _$StorySourceModelToJson(this);

  String get sourceTypeArabic => sourceType.arabicName;

  bool get isPrimarySource =>
      sourceType == SourceType.quran || sourceType == SourceType.hadith;
}

/// Types of sources
@JsonEnum(fieldRename: FieldRename.snake)
enum SourceType {
  quran,
  hadith,
  @JsonValue('historical_book')
  historicalBook,
  biography,
  tafsir,
  @JsonValue('scholarly_work')
  scholarlyWork;

  String get arabicName {
    switch (this) {
      case SourceType.quran:
        return 'القرآن الكريم';
      case SourceType.hadith:
        return 'الحديث النبوي';
      case SourceType.historicalBook:
        return 'كتاب تاريخي';
      case SourceType.biography:
        return 'سيرة';
      case SourceType.tafsir:
        return 'تفسير';
      case SourceType.scholarlyWork:
        return 'عمل علمي';
    }
  }
}

/// Verification status
@JsonEnum(fieldRename: FieldRename.snake)
enum VerificationStatus {
  verified,
  unverified,
  questionable;

  String get arabicName {
    switch (this) {
      case VerificationStatus.verified:
        return 'تم التحقق';
      case VerificationStatus.unverified:
        return 'غير محقق';
      case VerificationStatus.questionable:
        return 'مشكوك فيه';
    }
  }
}

/// Complete story with details
@JsonSerializable()
class StoryWithDetailsModel {
  final StoryModel story;
  final List<CharacterInStoryModel> characters;
  final List<LessonInStoryModel> lessons;
  final List<StorySourceModel> sources;

  StoryWithDetailsModel({
    required this.story,
    required this.characters,
    required this.lessons,
    required this.sources,
  });

  factory StoryWithDetailsModel.fromJson(Map<String, dynamic> json) =>
      _$StoryWithDetailsModelFromJson(json);

  Map<String, dynamic> toJson() => _$StoryWithDetailsModelToJson(this);
}

/// Character with role in story
@JsonSerializable()
class CharacterInStoryModel {
  final CharacterModel character;
  @JsonKey(name: 'role_in_story')
  final String roleInStory;
  @JsonKey(name: 'importance_level')
  final String importanceLevel;
  @JsonKey(name: 'character_description_in_story')
  final String? characterDescriptionInStory;

  CharacterInStoryModel({
    required this.character,
    required this.roleInStory,
    required this.importanceLevel,
    this.characterDescriptionInStory,
  });

  factory CharacterInStoryModel.fromJson(Map<String, dynamic> json) =>
      _$CharacterInStoryModelFromJson(json);

  Map<String, dynamic> toJson() => _$CharacterInStoryModelToJson(this);
}

/// Lesson with relevance to story
@JsonSerializable()
class LessonInStoryModel {
  final LessonModel lesson;
  @JsonKey(name: 'relevance_score')
  final double relevanceScore;
  final String? explanation;

  LessonInStoryModel({
    required this.lesson,
    required this.relevanceScore,
    this.explanation,
  });

  factory LessonInStoryModel.fromJson(Map<String, dynamic> json) =>
      _$LessonInStoryModelFromJson(json);

  Map<String, dynamic> toJson() => _$LessonInStoryModelToJson(this);
}
